//! Hover provider for the LSP
//!
//! This module provides hover information for keywords and identifiers
//! in poly-bench files, including embedded Go code via gopls and
//! TypeScript code via typescript-language-server.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use dashmap::DashMap;
use once_cell::sync::Lazy;
use poly_bench_dsl::Lang;
use tower_lsp::lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind, Position, Url};

use super::document::ParsedDocument;
use super::embedded::{extract_embedded_blocks, EmbeddedBlock, EmbeddedConfig};
use super::gopls_client::init_gopls_client;
use super::tsserver_client::init_tsserver_client;
use super::virtual_files::{VirtualFileManager, VirtualTsFileManager};

/// Cache TTL for embedded language hover results (in milliseconds)
const HOVER_CACHE_TTL_MS: u64 = 500;

/// Cache key for hover requests
#[derive(Clone, PartialEq, Eq, Hash)]
struct HoverCacheKey {
    uri: String,
    line: u32,
    character: u32,
}

/// Cached hover result
struct CachedHover {
    hover: Option<Hover>,
    timestamp: Instant,
}

/// Global hover cache for embedded language results
static HOVER_CACHE: Lazy<DashMap<HoverCacheKey, CachedHover>> = Lazy::new(DashMap::new);

/// Counter for cache cleanup (run cleanup every N requests)
static CACHE_CLEANUP_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Check if a cached hover result is still valid
fn get_cached_hover(uri: &Url, position: Position) -> Option<Option<Hover>> {
    let key = HoverCacheKey {
        uri: uri.to_string(),
        line: position.line,
        character: position.character,
    };

    if let Some(cached) = HOVER_CACHE.get(&key) {
        if cached.timestamp.elapsed() < Duration::from_millis(HOVER_CACHE_TTL_MS) {
            eprintln!("[hover-cache] Cache hit for {:?} at {}:{}", uri.path(), position.line, position.character);
            return Some(cached.hover.clone());
        }
    }
    None
}

/// Store a hover result in the cache
fn cache_hover(uri: &Url, position: Position, hover: Option<Hover>) {
    let key = HoverCacheKey {
        uri: uri.to_string(),
        line: position.line,
        character: position.character,
    };

    HOVER_CACHE.insert(
        key,
        CachedHover {
            hover,
            timestamp: Instant::now(),
        },
    );

    // Periodically clean up old entries (every 100 requests)
    let count = CACHE_CLEANUP_COUNTER.fetch_add(1, Ordering::Relaxed);
    if count % 100 == 0 {
        cleanup_expired_cache();
    }
}

/// Remove expired entries from the cache
fn cleanup_expired_cache() {
    let ttl = Duration::from_millis(HOVER_CACHE_TTL_MS * 2); // Use 2x TTL for cleanup
    HOVER_CACHE.retain(|_, v| v.timestamp.elapsed() < ttl);
}

/// Get hover information at a position (with embedded language support)
///
/// This function first checks if the position is within a Go or TypeScript code block.
/// If so, it delegates to the appropriate language server for rich type information.
/// Otherwise, it falls back to keyword and AST hover.
pub fn get_hover_with_gopls(
    doc: &ParsedDocument,
    position: Position,
    config: &EmbeddedConfig,
    uri: &Url,
    virtual_file_manager: &VirtualFileManager,
    virtual_ts_file_manager: &VirtualTsFileManager,
) -> Option<Hover> {
    // Convert position to offset
    let offset = match doc.position_to_offset(position) {
        Some(o) => o,
        None => {
            eprintln!("[hover] Failed to convert position {:?} to offset", position);
            return get_hover(doc, position);
        }
    };
    
    eprintln!("[hover] Position {:?} -> offset {}", position, offset);
    
    // Extract embedded blocks
    let blocks = extract_embedded_blocks(doc);
    
    // Separate Go and TypeScript blocks
    let go_blocks: Vec<&EmbeddedBlock> = blocks.iter()
        .filter(|b| b.lang == Lang::Go)
        .collect();
    let ts_blocks: Vec<&EmbeddedBlock> = blocks.iter()
        .filter(|b| b.lang == Lang::TypeScript)
        .collect();
    
    eprintln!("[hover] Found {} Go blocks, {} TS blocks", go_blocks.len(), ts_blocks.len());
    
    // Check if the offset is within a Go code block
    if let Some(go_block) = find_block_at_offset(&go_blocks, offset) {
        eprintln!("[hover] Offset {} is in Go block {:?} (span {}..{})", 
            offset, go_block.block_type, go_block.span.start, go_block.span.end);
        
        // Check cache first for embedded language hover
        if let Some(cached) = get_cached_hover(uri, position) {
            return cached;
        }
        
        // Try to get hover from gopls
        if let Some(go_mod_root) = &config.go_mod_root {
            eprintln!("[hover] go_mod_root: {}", go_mod_root);
            let hover = get_gopls_hover(
                doc,
                uri,
                offset,
                &go_blocks,
                go_mod_root,
                virtual_file_manager,
            );
            
            // Cache the result (even if None)
            cache_hover(uri, position, hover.clone());
            
            if hover.is_some() {
                return hover;
            }
        } else {
            eprintln!("[hover] No go_mod_root configured");
        }
        
        // Fallback: check for stdlib symbols when gopls returns None
        if let Some(hover) = get_stdlib_symbol_hover(doc, position) {
            return Some(hover);
        }
    }
    
    // Check if the offset is within a TypeScript code block
    if let Some(ts_block) = find_block_at_offset(&ts_blocks, offset) {
        eprintln!("[hover] Offset {} is in TS block {:?} (span {}..{})", 
            offset, ts_block.block_type, ts_block.span.start, ts_block.span.end);
        
        // Check cache first for embedded language hover
        if let Some(cached) = get_cached_hover(uri, position) {
            return cached;
        }
        
        // Try to get hover from tsserver
        if let Some(ts_module_root) = &config.ts_module_root {
            eprintln!("[hover] ts_module_root: {}", ts_module_root);
            let hover = get_tsserver_hover(
                doc,
                uri,
                offset,
                &ts_blocks,
                ts_module_root,
                virtual_ts_file_manager,
            );
            
            // Cache the result (even if None)
            cache_hover(uri, position, hover.clone());
            
            if hover.is_some() {
                return hover;
            }
        } else {
            eprintln!("[hover] No ts_module_root configured");
        }
        
        // Fallback: check for stdlib symbols when tsserver returns None
        if let Some(hover) = get_stdlib_symbol_hover(doc, position) {
            return Some(hover);
        }
    }
    
    if go_blocks.is_empty() && ts_blocks.is_empty() {
        eprintln!("[hover] No embedded language blocks found");
    } else {
        eprintln!("[hover] Offset {} is NOT in any embedded block", offset);
    }
    
    // Fall back to standard hover
    get_hover(doc, position)
}

/// Find the block containing the given offset
fn find_block_at_offset<'a>(blocks: &[&'a EmbeddedBlock], offset: usize) -> Option<&'a EmbeddedBlock> {
    blocks.iter()
        .find(|b| offset >= b.span.start && offset < b.span.end)
        .copied()
}

/// Get hover information from gopls for embedded Go code
fn get_gopls_hover(
    doc: &ParsedDocument,
    bench_uri: &Url,
    bench_offset: usize,
    go_blocks: &[&EmbeddedBlock],
    go_mod_root: &str,
    virtual_file_manager: &VirtualFileManager,
) -> Option<Hover> {
    eprintln!("[gopls] get_gopls_hover called for offset {} in {}", bench_offset, bench_uri);
    
    // Initialize gopls client if needed
    let client = match init_gopls_client(go_mod_root) {
        Some(c) => c,
        None => {
            eprintln!("[gopls] Failed to initialize gopls client");
            return None;
        }
    };
    
    // Get or create the virtual file
    let bench_uri_str = bench_uri.as_str();
    let bench_path = bench_uri.to_file_path().ok()?;
    let bench_path_str = bench_path.to_string_lossy();
    
    eprintln!("[gopls] Creating virtual file from {} Go blocks", go_blocks.len());
    
    let virtual_file = virtual_file_manager.get_or_create(
        bench_uri_str,
        &bench_path_str,
        go_mod_root,
        go_blocks,
        doc.version,
    );
    
    eprintln!("[gopls] Virtual file: {}", virtual_file.path);
    
    // Translate position from .bench to virtual Go file
    let go_position = match virtual_file.bench_to_go(bench_offset) {
        Some(pos) => {
            eprintln!("[gopls] Translated bench offset {} to Go position {}:{}", 
                bench_offset, pos.line, pos.character);
            pos
        }
        None => {
            eprintln!("[gopls] Failed to translate bench offset {} to Go position", bench_offset);
            return None;
        }
    };
    
    // Ensure the virtual file is synced with gopls
    if let Err(e) = client.did_change(&virtual_file.uri, &virtual_file.content, virtual_file.version) {
        eprintln!("[gopls] Failed to sync virtual file: {}", e);
        return None;
    }
    
    eprintln!("[gopls] Requesting hover at {}:{}", go_position.line, go_position.character);
    
    // Request hover from gopls
    match client.hover(&virtual_file.uri, go_position.line, go_position.character) {
        Ok(Some(mut hover)) => {
            eprintln!("[gopls] Got hover response!");
            // Translate the range back to .bench file if present
            if let Some(ref go_range) = hover.range {
                if let Some(bench_start_offset) = virtual_file.go_to_bench(
                    go_range.start.line,
                    go_range.start.character,
                ) {
                    if let Some(bench_end_offset) = virtual_file.go_to_bench(
                        go_range.end.line,
                        go_range.end.character,
                    ) {
                        hover.range = Some(tower_lsp::lsp_types::Range {
                            start: doc.offset_to_position(bench_start_offset),
                            end: doc.offset_to_position(bench_end_offset),
                        });
                    }
                }
            }
            Some(hover)
        }
        Ok(None) => {
            eprintln!("[gopls] Hover returned None");
            None
        }
        Err(e) => {
            eprintln!("[gopls] Hover request failed: {}", e);
            None
        }
    }
}

/// Get hover information from tsserver for embedded TypeScript code
fn get_tsserver_hover(
    doc: &ParsedDocument,
    bench_uri: &Url,
    bench_offset: usize,
    ts_blocks: &[&EmbeddedBlock],
    ts_module_root: &str,
    virtual_ts_file_manager: &VirtualTsFileManager,
) -> Option<Hover> {
    eprintln!("[tsserver] get_tsserver_hover called for offset {} in {}", bench_offset, bench_uri);
    
    // Initialize tsserver client if needed
    let client = match init_tsserver_client(ts_module_root) {
        Some(c) => c,
        None => {
            eprintln!("[tsserver] Failed to initialize tsserver client");
            return None;
        }
    };
    
    // Get or create the virtual file
    let bench_uri_str = bench_uri.as_str();
    let bench_path = bench_uri.to_file_path().ok()?;
    let bench_path_str = bench_path.to_string_lossy();
    
    let virtual_file = virtual_ts_file_manager.get_or_create(
        bench_uri_str,
        &bench_path_str,
        ts_module_root,
        ts_blocks,
        doc.version,
    );
    
    eprintln!("[tsserver] Virtual file: {}", virtual_file.path);
    
    // Translate position from .bench to virtual TS file
    let ts_position = match virtual_file.bench_to_ts(bench_offset) {
        Some(pos) => {
            eprintln!("[tsserver] Translated bench offset {} to TS position {}:{}", 
                bench_offset, pos.line, pos.character);
            pos
        }
        None => {
            eprintln!("[tsserver] Failed to translate bench offset {} to TS position", bench_offset);
            return None;
        }
    };
    
    // Ensure the virtual file is synced with tsserver
    if let Err(e) = client.did_change(&virtual_file.uri, &virtual_file.content, virtual_file.version) {
        eprintln!("[tsserver] Failed to sync virtual file: {}", e);
        return None;
    }
    
    eprintln!("[tsserver] Requesting hover at {}:{}", ts_position.line, ts_position.character);
    
    // Request hover from tsserver
    match client.hover(&virtual_file.uri, ts_position.line, ts_position.character) {
        Ok(Some(mut hover)) => {
            eprintln!("[tsserver] Got hover response!");
            // Translate the range back to .bench file if present
            if let Some(ref ts_range) = hover.range {
                if let Some(bench_start_offset) = virtual_file.ts_to_bench(
                    ts_range.start.line,
                    ts_range.start.character,
                ) {
                    if let Some(bench_end_offset) = virtual_file.ts_to_bench(
                        ts_range.end.line,
                        ts_range.end.character,
                    ) {
                        hover.range = Some(tower_lsp::lsp_types::Range {
                            start: doc.offset_to_position(bench_start_offset),
                            end: doc.offset_to_position(bench_end_offset),
                        });
                    }
                }
            }
            Some(hover)
        }
        Ok(None) => {
            eprintln!("[tsserver] Hover returned None");
            None
        }
        Err(e) => {
            eprintln!("[tsserver] Hover request failed: {}", e);
            None
        }
    }
}

/// Get hover information at a position (DSL keywords and AST only)
pub fn get_hover(doc: &ParsedDocument, position: Position) -> Option<Hover> {
    let (word, range) = doc.word_at_position(position)?;
    
    // Convert position to offset for span-based matching
    let offset = doc.position_to_offset(position);

    // Check for UseStd statements first (position-aware)
    if let Some(ref ast) = doc.ast {
        if let Some(offset) = offset {
            for use_std in &ast.use_stds {
                // Check if hovering over 'use' keyword
                if offset >= use_std.use_span.start && offset < use_std.use_span.end {
                    return Some(Hover {
                        contents: HoverContents::Markup(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: keyword_docs("use").unwrap_or("").to_string(),
                        }),
                        range: Some(range),
                    });
                }
                
                // Check if hovering over 'std' namespace
                if offset >= use_std.std_span.start && offset < use_std.std_span.end {
                    return Some(Hover {
                        contents: HoverContents::Markup(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: std_namespace_docs().to_string(),
                        }),
                        range: Some(range),
                    });
                }
                
                // Check if hovering over module name
                if offset >= use_std.module_span.start && offset < use_std.module_span.end {
                    if let Some(docs) = stdlib_module_docs(&use_std.module) {
                        return Some(Hover {
                            contents: HoverContents::Markup(MarkupContent {
                                kind: MarkupKind::Markdown,
                                value: docs.to_string(),
                            }),
                            range: Some(range),
                        });
                    }
                }
            }
        }
    }

    // Look up keyword documentation
    if let Some(docs) = keyword_docs(&word) {
        return Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: docs.to_string(),
            }),
            range: Some(range),
        });
    }

    // Check if it's a stdlib module name (anvil, constants) used in code
    if let Some(docs) = stdlib_module_docs(&word) {
        return Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: docs.to_string(),
            }),
            range: Some(range),
        });
    }
    
    // Check if it's a stdlib symbol (spawnAnvil, ANVIL_RPC_URL, PI, E)
    if let Some(docs) = stdlib_symbol_docs(&word) {
        return Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: docs.to_string(),
            }),
            range: Some(range),
        });
    }

    // Check if it's a language identifier
    if let Some(docs) = lang_docs(&word) {
        return Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: docs.to_string(),
            }),
            range: Some(range),
        });
    }

    // Try to find the identifier in the AST (fixture, benchmark, etc.)
    if let Some(ref ast) = doc.ast {
        for suite in &ast.suites {
            // Check if hovering over suite name
            if suite.name == word {
                let mut content = format!("**Suite** `{}`", suite.name);
                if let Some(ref desc) = suite.description {
                    content.push_str(&format!("\n\n{}", desc));
                }
                return Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: content,
                    }),
                    range: Some(range),
                });
            }

            // Check fixtures
            for fixture in &suite.fixtures {
                if fixture.name == word {
                    let mut content = format!("**Fixture** `{}`", fixture.name);
                    if let Some(ref desc) = fixture.description {
                        content.push_str(&format!("\n\n{}", desc));
                    }
                    if let Some(ref shape) = fixture.shape {
                        content.push_str(&format!("\n\n**Shape:** `{}`", shape));
                    }
                    if fixture.hex_data.is_some() {
                        content.push_str("\n\n*Has hex data*");
                    }
                    return Some(Hover {
                        contents: HoverContents::Markup(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: content,
                        }),
                        range: Some(range),
                    });
                }
            }

            // Check benchmarks
            for benchmark in &suite.benchmarks {
                if benchmark.name == word {
                    let mut content = format!("**Benchmark** `{}`", benchmark.name);
                    if let Some(ref desc) = benchmark.description {
                        content.push_str(&format!("\n\n{}", desc));
                    }
                    if let Some(iters) = benchmark.iterations {
                        content.push_str(&format!("\n\n**Iterations:** {}", iters));
                    }
                    if !benchmark.tags.is_empty() {
                        content.push_str(&format!("\n\n**Tags:** {}", benchmark.tags.join(", ")));
                    }
                    let langs: Vec<_> = benchmark.implementations.keys().map(|l| l.as_str()).collect();
                    if !langs.is_empty() {
                        content.push_str(&format!("\n\n**Implementations:** {}", langs.join(", ")));
                    }
                    return Some(Hover {
                        contents: HoverContents::Markup(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: content,
                        }),
                        range: Some(range),
                    });
                }
            }
        }
    }

    None
}

/// Get documentation for a keyword
fn keyword_docs(word: &str) -> Option<&'static str> {
    match word {
        // Core keywords
        "suite" => Some(
            "**suite** `name { ... }`\n\n\
            Top-level benchmark suite container.\n\n\
            Contains setup blocks, fixtures, and benchmark definitions."
        ),
        "bench" => Some(
            "**bench** `name { ... }`\n\n\
            Benchmark definition with language implementations.\n\n\
            ```\nbench encode {\n    go: encodeData(input)\n    ts: encodeData(input)\n}\n```"
        ),
        "setup" => Some(
            "**setup** `<lang> { ... }`\n\n\
            Per-language setup block with sections:\n\
            - `import` - Import statements\n\
            - `declare` - Package-level declarations\n\
            - `init` - Initialization code (runs once)\n\
            - `helpers` - Reusable helper functions"
        ),
        "fixture" => Some(
            "**fixture** `name { ... }`\n\n\
            Shared test data fixture.\n\n\
            Can contain:\n\
            - `hex:` - Hex-encoded binary data\n\
            - `go:` / `ts:` - Language-specific implementations\n\
            - `shape:` - JSON-like shape annotation"
        ),

        // Setup sections
        "import" => Some(
            "**import** `{ ... }` or `( ... )`\n\n\
            Import statements for the language.\n\n\
            For Go, use grouped imports:\n\
            ```\nimport (\n    \"fmt\"\n    \"encoding/json\"\n)\n```"
        ),
        "declare" => Some(
            "**declare** `{ ... }`\n\n\
            Package-level declarations (vars, types, consts).\n\n\
            Placed at package scope, outside any function."
        ),
        "init" => Some(
            "**init** `{ ... }`\n\n\
            Initialization code that runs once before benchmarks.\n\n\
            Use for expensive setup that shouldn't be timed."
        ),
        "helpers" => Some(
            "**helpers** `{ ... }`\n\n\
            Reusable helper functions available to all benchmarks.\n\n\
            Define functions that can be called from benchmark implementations."
        ),
        "async" => Some(
            "**async** `init { ... }`\n\n\
            Mark init block as async (TypeScript only).\n\n\
            Allows using `await` in initialization code."
        ),

        // Configuration
        "description" => Some(
            "**description:** `\"...\"`\n\n\
            Human-readable description for the suite, fixture, or benchmark."
        ),
        "iterations" => Some(
            "**iterations:** `<number>`\n\n\
            Number of benchmark iterations.\n\n\
            Default: 1000"
        ),
        "warmup" => Some(
            "**warmup:** `<number>`\n\n\
            Number of warmup iterations before measuring.\n\n\
            Default: 100"
        ),
        "timeout" => Some(
            "**timeout:** `<duration>`\n\n\
            Maximum execution time per benchmark.\n\n\
            Examples: `30s`, `500ms`, `1m`"
        ),
        "tags" => Some(
            "**tags:** `[\"tag1\", \"tag2\"]`\n\n\
            Labels for filtering and grouping benchmarks."
        ),
        "requires" => Some(
            "**requires:** `[\"go\", \"ts\"]`\n\n\
            Languages that must have implementations.\n\n\
            Benchmarks missing required languages will error."
        ),
        "order" => Some(
            "**order:** `sequential | parallel | random`\n\n\
            Execution order for benchmarks in the suite.\n\n\
            - `sequential` - Run in definition order (default)\n\
            - `parallel` - Run concurrently where supported\n\
            - `random` - Randomize order"
        ),
        "compare" => Some(
            "**compare:** `true | false`\n\n\
            Enable comparison tables in output.\n\n\
            Shows relative performance between languages."
        ),
        "baseline" => Some(
            "**baseline:** `\"go\" | \"ts\"`\n\n\
            Baseline language for comparison ratios.\n\n\
            Other languages are compared against this baseline."
        ),

        // Lifecycle hooks
        "skip" => Some(
            "**skip** `<lang>:` `<condition>`\n\n\
            Skip benchmark if condition is true.\n\n\
            ```\nskip go: runtime.GOOS == \"windows\"\n```"
        ),
        "validate" => Some(
            "**validate** `<lang>:` `<expression>`\n\n\
            Validate benchmark result.\n\n\
            Expression should return a boolean."
        ),
        "before" => Some(
            "**before** `<lang>:` `{ ... }`\n\n\
            Hook that runs once before all iterations."
        ),
        "after" => Some(
            "**after** `<lang>:` `{ ... }` or `{ ... }`\n\n\
            Hook that runs once after all iterations.\n\n\
            **Benchmark-level hook:**\n\
            ```\nbench test {\n    after go: { cleanup() }\n}\n```\n\n\
            **Suite-level charting block:**\n\
            ```\nafter {\n    charting.drawBarChart(title: \"Results\")\n}\n```"
        ),
        "each" => Some(
            "**each** `<lang>:` `{ ... }`\n\n\
            Hook that runs before each iteration.\n\n\
            Executed outside timing measurement."
        ),

        // Fixture keywords
        "hex" => Some(
            "**hex:** `\"0x...\"`\n\n\
            Hex-encoded binary data.\n\n\
            Portable format that works across all languages."
        ),
        "shape" => Some(
            "**shape:** `\"type\"`\n\n\
            JSON-like type annotation for documentation.\n\n\
            Example: `shape: \"{ id: number, name: string }\"`"
        ),

        // Order values
        "sequential" => Some(
            "**sequential**\n\n\
            Run benchmarks in definition order.\n\n\
            This is the default execution order."
        ),
        "parallel" => Some(
            "**parallel**\n\n\
            Run benchmarks concurrently where supported.\n\n\
            May improve total execution time."
        ),
        "random" => Some(
            "**random**\n\n\
            Randomize benchmark execution order.\n\n\
            Helps detect order-dependent issues."
        ),

        // Standard library import
        "use" => Some(
            "**use** `std::module`\n\n\
            Import a module from the poly-bench standard library.\n\n\
            Available modules:\n\
            - `anvil` - Anvil node integration (ANVIL_RPC_URL)\n\
            - `charting` - Chart generation (drawBarChart, drawPieChart, drawLineChart)\n\
            - `constants` - Mathematical constants (std_PI, std_E)"
        ),

        // Global setup
        "globalSetup" => Some(
            "**globalSetup** `{ ... }`\n\n\
            Global setup block for suite-level initialization.\n\n\
            Can be placed inside a suite or at file level (for all suites).\n\n\
            **Available functions (with std::anvil):**\n\
            - `anvil.spawnAnvil()` - Spawn a local Anvil Ethereum node\n\
            - `anvil.spawnAnvil(fork: \"url\")` - Spawn with chain forking\n\n\
            **Example (inside suite):**\n\
            ```\nsuite evmBench {\n    globalSetup {\n        anvil.spawnAnvil()\n    }\n    \n    bench test {\n        go: callRpc(anvil.ANVIL_RPC_URL)\n    }\n}\n```"
        ),
        "spawnAnvil" => Some(
            "**spawnAnvil** `()` or `(fork: \"url\")`\n\n\
            Spawn a local Anvil Ethereum node.\n\n\
            Anvil is started before benchmarks and stopped after.\n\
            The RPC URL is available as `ANVIL_RPC_URL` in benchmark code.\n\n\
            **Options:**\n\
            - `fork: \"url\"` - Fork from an existing chain"
        ),

        _ => None,
    }
}

/// Get documentation for the stdlib namespace
fn std_namespace_docs() -> &'static str {
    "**std**\n\n\
    Poly-bench standard library namespace.\n\n\
    Use `use std::module` to import a standard library module.\n\n\
    Available modules:\n\
    - `anvil` - Local Ethereum node management\n\
    - `charting` - Chart generation from benchmark results\n\
    - `constants` - Mathematical constants"
}

/// Get documentation for a stdlib module
fn stdlib_module_docs(module: &str) -> Option<&'static str> {
    match module {
        "anvil" => Some(
            "**std::anvil**\n\n\
            Anvil Ethereum node integration from the poly-bench standard library.\n\n\
            When imported, poly-bench automatically spawns a local Anvil node before\n\
            running benchmarks and makes the RPC URL available via `ANVIL_RPC_URL`.\n\n\
            **Provided variables:**\n\
            - `ANVIL_RPC_URL` - The RPC endpoint URL (e.g., http://127.0.0.1:8545)\n\n\
            **Lifecycle:**\n\
            - Anvil is started automatically when benchmarks begin\n\
            - Anvil is stopped automatically when benchmarks complete\n\n\
            **Requirements:** Anvil must be installed (part of Foundry toolchain)"
        ),
        "charting" => Some(
            "**std::charting**\n\n\
            Chart generation from benchmark results.\n\n\
            Use in a suite-level `after { }` block to generate charts after benchmarks complete.\n\n\
            **Provided functions:**\n\
            - `charting.drawBarChart()` - Generate a bar chart comparing benchmark times\n\
            - `charting.drawPieChart()` - Generate a pie chart showing time distribution\n\
            - `charting.drawLineChart()` - Generate a line chart for trend visualization\n\n\
            **Example:**\n\
            ```\nafter {\n    charting.drawBarChart(\n        title: \"Performance Comparison\",\n        xlabel: \"Benchmark\",\n        ylabel: \"Time (ns)\"\n    )\n}\n```"
        ),
        "constants" => Some(
            "**std::constants**\n\n\
            Mathematical constants from the poly-bench standard library.\n\n\
            **Provided constants:**\n\
            - `std_PI` - Pi (π ≈ 3.14159265358979323846)\n\
            - `std_E` - Euler's number (e ≈ 2.71828182845904523536)"
        ),
        "math" => Some(
            "**std::math** *(planned)*\n\n\
            Mathematical utility functions."
        ),
        _ => None,
    }
}

/// Get documentation for stdlib symbols (constants, functions, etc.)
/// Supports both legacy (ANVIL_RPC_URL, std_PI) and namespaced (anvil.ANVIL_RPC_URL, constants.PI)
fn stdlib_symbol_docs(symbol: &str) -> Option<&'static str> {
    match symbol {
        // Namespaced std::anvil symbols (new preferred style)
        "spawnAnvil" => Some(
            "**anvil.spawnAnvil** `()` or `(fork: \"url\")`\n\n\
            Spawn a local Anvil Ethereum node.\n\n\
            Anvil is started before benchmarks and stopped after.\n\
            The RPC URL is available as `anvil.ANVIL_RPC_URL` in benchmark code.\n\n\
            **Options:**\n\
            - `fork: \"url\"` - Fork from an existing chain\n\n\
            **Example:**\n\
            ```\nglobalSetup {\n    anvil.spawnAnvil()\n}\n```"
        ),
        // Legacy std::anvil symbols (still supported)
        "ANVIL_RPC_URL" => Some(
            "```go\nvar ANVIL_RPC_URL string\n```\n\n\
            **anvil.ANVIL_RPC_URL** - Anvil RPC endpoint URL.\n\n\
            When `use std::anvil` is specified with `anvil.spawnAnvil()`,\n\
            poly-bench automatically starts an Anvil node and sets this variable\n\
            to its RPC URL (e.g., `http://127.0.0.1:8545`).\n\n\
            **Example:**\n\
            ```go\nhttp.Post(anvil.ANVIL_RPC_URL, \"application/json\", body)\n```\n\n\
            *From `std::anvil`*"
        ),
        // std::charting symbols
        "drawBarChart" => Some(
            "**charting.drawBarChart** `(title?, description?, xlabel?, ylabel?, output?)`\n\n\
            Generate a bar chart comparing benchmark execution times.\n\n\
            **Parameters:**\n\
            - `title` - Chart title\n\
            - `description` - Chart description\n\
            - `xlabel` - X-axis label\n\
            - `ylabel` - Y-axis label\n\
            - `output` - Output filename (default: bar-chart.svg)\n\n\
            **Example:**\n\
            ```\nafter {\n    charting.drawBarChart(\n        title: \"Performance Comparison\",\n        xlabel: \"Benchmark\",\n        ylabel: \"Time (ns)\"\n    )\n}\n```\n\n\
            *From `std::charting`*"
        ),
        "drawPieChart" => Some(
            "**charting.drawPieChart** `(title?, description?, output?)`\n\n\
            Generate a pie chart showing time distribution across benchmarks.\n\n\
            **Parameters:**\n\
            - `title` - Chart title\n\
            - `description` - Chart description\n\
            - `output` - Output filename (default: pie-chart.svg)\n\n\
            **Example:**\n\
            ```\nafter {\n    charting.drawPieChart(\n        title: \"Time Distribution\"\n    )\n}\n```\n\n\
            *From `std::charting`*"
        ),
        "drawLineChart" => Some(
            "**charting.drawLineChart** `(title?, description?, xlabel?, ylabel?, output?)`\n\n\
            Generate a line chart for trend visualization.\n\n\
            **Parameters:**\n\
            - `title` - Chart title\n\
            - `description` - Chart description\n\
            - `xlabel` - X-axis label\n\
            - `ylabel` - Y-axis label\n\
            - `output` - Output filename (default: line-chart.svg)\n\n\
            **Example:**\n\
            ```\nafter {\n    charting.drawLineChart(\n        title: \"Performance Trends\"\n    )\n}\n```\n\n\
            *From `std::charting`*"
        ),
        // Namespaced std::constants symbols (new preferred style)
        "PI" => Some(
            "```go\nconst constants.PI float64 = 3.14159265358979323846\n```\n\n\
            **constants.PI** - Pi (π), the ratio of a circle's circumference to its diameter.\n\n\
            **Example:**\n\
            ```go\narea := constants.PI * radius * radius\n```\n\n\
            *From `std::constants`*"
        ),
        "E" => Some(
            "```go\nconst constants.E float64 = 2.71828182845904523536\n```\n\n\
            **constants.E** - Euler's number (e), the base of natural logarithms.\n\n\
            **Example:**\n\
            ```go\nresult := math.Pow(constants.E, x)\n```\n\n\
            *From `std::constants`*"
        ),
        // Legacy std::constants symbols (still supported)
        "std_PI" => Some(
            "```go\nconst std_PI float64 = 3.14159265358979323846\n```\n\n\
            **Pi (π)** - The ratio of a circle's circumference to its diameter.\n\n\
            *Legacy: consider using `constants.PI` instead.*\n\n\
            *From `std::constants`*"
        ),
        "std_E" => Some(
            "```go\nconst std_E float64 = 2.71828182845904523536\n```\n\n\
            **Euler's number (e)** - The base of natural logarithms.\n\n\
            *Legacy: consider using `constants.E` instead.*\n\n\
            *From `std::constants`*"
        ),
        _ => None,
    }
}

/// Get hover information for stdlib symbols in embedded code blocks
fn get_stdlib_symbol_hover(doc: &ParsedDocument, position: Position) -> Option<Hover> {
    let (word, range) = doc.word_at_position(position)?;
    
    if let Some(docs) = stdlib_symbol_docs(&word) {
        eprintln!("[hover] Found stdlib symbol: {}", word);
        return Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: docs.to_string(),
            }),
            range: Some(range),
        });
    }
    
    None
}

/// Get documentation for a language identifier
fn lang_docs(word: &str) -> Option<&'static str> {
    match word.to_lowercase().as_str() {
        "go" => Some(
            "**Go** language\n\n\
            Implementations are compiled and executed via Go plugin system."
        ),
        "ts" | "typescript" => Some(
            "**TypeScript** language\n\n\
            Implementations are transpiled and executed via embedded V8 runtime."
        ),
        "rust" | "rs" => Some(
            "**Rust** language *(planned)*\n\n\
            Native Rust benchmark support."
        ),
        "python" | "py" => Some(
            "**Python** language *(planned)*\n\n\
            Python benchmark support."
        ),
        _ => None,
    }
}
