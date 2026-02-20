//! Hover provider for the LSP
//!
//! This module provides hover information for keywords and identifiers
//! in poly-bench files, including embedded Go code via gopls,
//! TypeScript code via typescript-language-server, and Rust code via rust-analyzer.

use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant},
};

use dashmap::DashMap;
use once_cell::sync::Lazy;
use poly_bench_dsl::Lang;
use tower_lsp::lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind, Position, Url};

use super::{
    document::ParsedDocument,
    embedded::{extract_embedded_blocks, EmbeddedBlock, EmbeddedConfig},
    gopls_client::init_gopls_client,
    rust_analyzer_client::init_rust_analyzer_client,
    tsserver_client::init_tsserver_client,
    virtual_files::{
        VirtualFile, VirtualFileManager, VirtualRustFileManager, VirtualTsFileManager,
    },
};

/// Cache TTL for embedded language hover results (in milliseconds)
/// Increased from 500ms to 3000ms (3 seconds) to reduce re-computation overhead
const HOVER_CACHE_TTL_MS: u64 = 3000;

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
    let key =
        HoverCacheKey { uri: uri.to_string(), line: position.line, character: position.character };

    if let Some(cached) = HOVER_CACHE.get(&key) {
        if cached.timestamp.elapsed() < Duration::from_millis(HOVER_CACHE_TTL_MS) {
            eprintln!(
                "[hover-cache] Cache hit for {:?} at {}:{}",
                uri.path(),
                position.line,
                position.character
            );
            return Some(cached.hover.clone());
        }
    }
    None
}

/// Store a hover result in the cache
fn cache_hover(uri: &Url, position: Position, hover: Option<Hover>) {
    let key =
        HoverCacheKey { uri: uri.to_string(), line: position.line, character: position.character };

    HOVER_CACHE.insert(key, CachedHover { hover, timestamp: Instant::now() });

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
/// This function first checks if the position is within a Go, TypeScript, or Rust code block.
/// If so, it delegates to the appropriate language server for rich type information.
/// Otherwise, it falls back to keyword and AST hover.
pub fn get_hover_with_embedded(
    doc: &ParsedDocument,
    position: Position,
    config: &EmbeddedConfig,
    uri: &Url,
    virtual_file_manager: &VirtualFileManager,
    virtual_ts_file_manager: &VirtualTsFileManager,
    virtual_rust_file_manager: &VirtualRustFileManager,
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

    // Separate Go, TypeScript, and Rust blocks
    let go_blocks: Vec<&EmbeddedBlock> = blocks.iter().filter(|b| b.lang == Lang::Go).collect();
    let ts_blocks: Vec<&EmbeddedBlock> =
        blocks.iter().filter(|b| b.lang == Lang::TypeScript).collect();
    let rust_blocks: Vec<&EmbeddedBlock> = blocks.iter().filter(|b| b.lang == Lang::Rust).collect();

    eprintln!(
        "[hover] Found {} Go blocks, {} TS blocks, {} Rust blocks",
        go_blocks.len(),
        ts_blocks.len(),
        rust_blocks.len()
    );

    // Check if the offset is within a Go code block
    if let Some(go_block) = find_block_at_offset(&go_blocks, offset) {
        eprintln!(
            "[hover] Offset {} is in Go block {:?} (span {}..{})",
            offset, go_block.block_type, go_block.span.start, go_block.span.end
        );

        // Check cache first for embedded language hover
        if let Some(cached) = get_cached_hover(uri, position) {
            return cached;
        }

        // Try to get hover from gopls
        if let Some(go_mod_root) = &config.go_mod_root {
            eprintln!("[hover] go_mod_root: {}", go_mod_root);
            let hover =
                get_gopls_hover(doc, uri, offset, &go_blocks, go_mod_root, virtual_file_manager);

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
        eprintln!(
            "[hover] Offset {} is in TS block {:?} (span {}..{})",
            offset, ts_block.block_type, ts_block.span.start, ts_block.span.end
        );

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

    // Check if the offset is within a Rust code block
    if let Some(rust_block) = find_block_at_offset(&rust_blocks, offset) {
        eprintln!(
            "[hover] Offset {} is in Rust block {:?} (span {}..{})",
            offset, rust_block.block_type, rust_block.span.start, rust_block.span.end
        );

        // Check cache first for embedded language hover
        if let Some(cached) = get_cached_hover(uri, position) {
            return cached;
        }

        // Try to get hover from rust-analyzer
        if let Some(rust_project_root) = &config.rust_project_root {
            eprintln!("[hover] rust_project_root: {}", rust_project_root);
            let hover = get_rust_analyzer_hover(
                doc,
                uri,
                offset,
                &rust_blocks,
                rust_project_root,
                virtual_rust_file_manager,
            );

            // Cache the result (even if None)
            cache_hover(uri, position, hover.clone());

            if hover.is_some() {
                return hover;
            }
        } else {
            eprintln!("[hover] No rust_project_root configured");
        }

        // Fallback: check for stdlib symbols when rust-analyzer returns None
        if let Some(hover) = get_stdlib_symbol_hover(doc, position) {
            return Some(hover);
        }
    }

    if go_blocks.is_empty() && ts_blocks.is_empty() && rust_blocks.is_empty() {
        eprintln!("[hover] No embedded language blocks found");
    } else {
        eprintln!("[hover] Offset {} is NOT in any embedded block", offset);
    }

    // Fall back to standard hover
    get_hover(doc, position)
}

/// Find the block containing the given offset
fn find_block_at_offset<'a>(
    blocks: &[&'a EmbeddedBlock],
    offset: usize,
) -> Option<&'a EmbeddedBlock> {
    blocks.iter().find(|b| offset >= b.span.start && offset < b.span.end).copied()
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

    eprintln!("[gopls] Virtual file: {}", virtual_file.path());

    // Translate position from .bench to virtual Go file
    let go_position = match virtual_file.bench_to_go(bench_offset) {
        Some(pos) => {
            eprintln!(
                "[gopls] Translated bench offset {} to Go position {}:{}",
                bench_offset, pos.line, pos.character
            );
            pos
        }
        None => {
            eprintln!("[gopls] Failed to translate bench offset {} to Go position", bench_offset);
            return None;
        }
    };

    // Ensure the virtual file is synced with gopls
    if let Err(e) =
        client.did_change(virtual_file.uri(), virtual_file.content(), virtual_file.version())
    {
        eprintln!("[gopls] Failed to sync virtual file: {}", e);
        return None;
    }

    eprintln!("[gopls] Requesting hover at {}:{}", go_position.line, go_position.character);

    // Request hover from gopls
    match client.hover(virtual_file.uri(), go_position.line, go_position.character) {
        Ok(Some(mut hover)) => {
            eprintln!("[gopls] Got hover response!");
            // Translate the range back to .bench file if present
            if let Some(ref go_range) = hover.range {
                if let Some(bench_start_offset) =
                    virtual_file.go_to_bench(go_range.start.line, go_range.start.character)
                {
                    if let Some(bench_end_offset) =
                        virtual_file.go_to_bench(go_range.end.line, go_range.end.character)
                    {
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

    eprintln!("[tsserver] Virtual file: {}", virtual_file.path());

    // Translate position from .bench to virtual TS file
    let ts_position = match virtual_file.bench_to_ts(bench_offset) {
        Some(pos) => {
            eprintln!(
                "[tsserver] Translated bench offset {} to TS position {}:{}",
                bench_offset, pos.line, pos.character
            );
            pos
        }
        None => {
            eprintln!(
                "[tsserver] Failed to translate bench offset {} to TS position",
                bench_offset
            );
            return None;
        }
    };

    // Ensure the virtual file is synced with tsserver
    if let Err(e) =
        client.did_change(virtual_file.uri(), virtual_file.content(), virtual_file.version())
    {
        eprintln!("[tsserver] Failed to sync virtual file: {}", e);
        return None;
    }

    eprintln!("[tsserver] Requesting hover at {}:{}", ts_position.line, ts_position.character);

    // Request hover from tsserver
    match client.hover(virtual_file.uri(), ts_position.line, ts_position.character) {
        Ok(Some(mut hover)) => {
            eprintln!("[tsserver] Got hover response!");
            // Translate the range back to .bench file if present
            if let Some(ref ts_range) = hover.range {
                if let Some(bench_start_offset) =
                    virtual_file.ts_to_bench(ts_range.start.line, ts_range.start.character)
                {
                    if let Some(bench_end_offset) =
                        virtual_file.ts_to_bench(ts_range.end.line, ts_range.end.character)
                    {
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

/// Get hover information from rust-analyzer for embedded Rust code
fn get_rust_analyzer_hover(
    doc: &ParsedDocument,
    bench_uri: &Url,
    bench_offset: usize,
    rust_blocks: &[&EmbeddedBlock],
    rust_project_root: &str,
    virtual_rust_file_manager: &VirtualRustFileManager,
) -> Option<Hover> {
    eprintln!(
        "[rust-analyzer] get_rust_analyzer_hover called for offset {} in {}",
        bench_offset, bench_uri
    );

    // Initialize rust-analyzer client if needed
    let client = match init_rust_analyzer_client(rust_project_root) {
        Some(c) => c,
        None => {
            eprintln!("[rust-analyzer] Failed to initialize rust-analyzer client");
            return None;
        }
    };

    // Get or create the virtual file
    let bench_uri_str = bench_uri.as_str();
    let bench_path = bench_uri.to_file_path().ok()?;
    let bench_path_str = bench_path.to_string_lossy();

    let virtual_file = virtual_rust_file_manager.get_or_create(
        bench_uri_str,
        &bench_path_str,
        rust_project_root,
        rust_blocks,
        doc.version,
    );

    eprintln!("[rust-analyzer] Virtual file: {}", virtual_file.path());

    // Translate position from .bench to virtual Rust file
    let rust_position = match virtual_file.bench_to_rust(bench_offset) {
        Some(pos) => {
            eprintln!(
                "[rust-analyzer] Translated bench offset {} to Rust position {}:{}",
                bench_offset, pos.line, pos.character
            );
            pos
        }
        None => {
            eprintln!(
                "[rust-analyzer] Failed to translate bench offset {} to Rust position",
                bench_offset
            );
            return None;
        }
    };

    // Ensure the virtual file is synced with rust-analyzer
    if let Err(e) =
        client.did_change(virtual_file.uri(), virtual_file.content(), virtual_file.version())
    {
        eprintln!("[rust-analyzer] Failed to sync virtual file: {}", e);
        return None;
    }

    eprintln!(
        "[rust-analyzer] Requesting hover at {}:{}",
        rust_position.line, rust_position.character
    );

    // Request hover from rust-analyzer
    match client.hover(virtual_file.uri(), rust_position.line, rust_position.character) {
        Ok(Some(mut hover)) => {
            eprintln!("[rust-analyzer] Got hover response: {:?}", hover.contents);

            // Enhance the hover content with proper markdown formatting
            hover.contents = enhance_rust_hover_content(hover.contents);

            // Translate the range back to .bench file if present
            if let Some(ref rust_range) = hover.range {
                if let Some(bench_start_offset) =
                    virtual_file.rust_to_bench(rust_range.start.line, rust_range.start.character)
                {
                    if let Some(bench_end_offset) =
                        virtual_file.rust_to_bench(rust_range.end.line, rust_range.end.character)
                    {
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
            eprintln!("[rust-analyzer] Hover returned None");
            None
        }
        Err(e) => {
            eprintln!("[rust-analyzer] Hover request failed: {}", e);
            None
        }
    }
}

/// Enhance Rust hover content with proper markdown formatting
fn enhance_rust_hover_content(contents: HoverContents) -> HoverContents {
    match contents {
        HoverContents::Markup(markup) => {
            let value = &markup.value;

            // Check if already properly formatted as markdown with code blocks
            if value.contains("```rust") || value.contains("```rs") {
                return HoverContents::Markup(markup);
            }

            // Split into lines for processing
            let lines: Vec<&str> = value.lines().collect();
            if lines.is_empty() {
                return HoverContents::Markup(markup);
            }

            let mut formatted_parts = Vec::new();
            let mut code_lines = Vec::new();
            let mut doc_lines = Vec::new();
            let mut in_signature = false;
            let mut seen_signature = false;

            for line in &lines {
                let trimmed = line.trim();

                // Skip empty lines
                if trimmed.is_empty() {
                    // If we were in a signature, end it
                    if in_signature && !code_lines.is_empty() {
                        formatted_parts.push(format!("```rust\n{}\n```", code_lines.join("\n")));
                        code_lines.clear();
                        in_signature = false;
                        seen_signature = true;
                    }
                    continue;
                }

                // Skip virtual file module names and internal module paths
                if trimmed.starts_with("_lsp_virtual") || trimmed.starts_with("polybench_runner") {
                    continue;
                }

                // Check if this looks like a module path (single identifier, no spaces)
                // These appear at the start before signatures
                if !seen_signature &&
                    !in_signature &&
                    !trimmed.contains(' ') &&
                    trimmed.chars().all(|c| c.is_alphanumeric() || c == '_' || c == ':')
                {
                    // Skip module paths like "tiny_keccak::keccak::Keccak"
                    continue;
                }

                // Detect function/struct/type signatures
                let is_signature_start = trimmed.starts_with("fn ") ||
                    trimmed.starts_with("pub fn ") ||
                    trimmed.starts_with("async fn ") ||
                    trimmed.starts_with("pub async fn ") ||
                    trimmed.starts_with("unsafe fn ") ||
                    trimmed.starts_with("pub unsafe fn ") ||
                    trimmed.starts_with("struct ") ||
                    trimmed.starts_with("pub struct ") ||
                    trimmed.starts_with("type ") ||
                    trimmed.starts_with("pub type ") ||
                    trimmed.starts_with("const ") ||
                    trimmed.starts_with("pub const ") ||
                    trimmed.starts_with("static ") ||
                    trimmed.starts_with("pub static ") ||
                    trimmed.starts_with("impl ") ||
                    trimmed.starts_with("impl<") ||
                    trimmed.starts_with("trait ") ||
                    trimmed.starts_with("pub trait ") ||
                    trimmed.starts_with("enum ") ||
                    trimmed.starts_with("pub enum ") ||
                    trimmed.starts_with("mod ") ||
                    trimmed.starts_with("pub mod ") ||
                    trimmed.starts_with("use ") ||
                    trimmed.starts_with("pub use ") ||
                    trimmed.starts_with("extern crate ");

                if is_signature_start {
                    // Flush any previous signature
                    if !code_lines.is_empty() {
                        formatted_parts.push(format!("```rust\n{}\n```", code_lines.join("\n")));
                        code_lines.clear();
                    }
                    in_signature = true;
                    code_lines.push(*line);
                } else if in_signature &&
                    (trimmed.starts_with("where") ||
                        trimmed.ends_with(',') ||
                        trimmed.ends_with('{') ||
                        trimmed.ends_with('>'))
                {
                    // Continue collecting signature lines
                    code_lines.push(*line);
                } else {
                    // End signature mode and collect as documentation
                    if in_signature && !code_lines.is_empty() {
                        formatted_parts.push(format!("```rust\n{}\n```", code_lines.join("\n")));
                        code_lines.clear();
                        in_signature = false;
                        seen_signature = true;
                    }
                    doc_lines.push(*line);
                }
            }

            // Flush remaining code lines
            if !code_lines.is_empty() {
                formatted_parts.push(format!("```rust\n{}\n```", code_lines.join("\n")));
            }

            // Format documentation - truncate if too long and clean up
            if !doc_lines.is_empty() {
                let doc_text = doc_lines.join(" ");
                // Clean up the text - remove excessive whitespace
                let cleaned: String = doc_text.split_whitespace().collect::<Vec<_>>().join(" ");

                // Truncate very long documentation
                let max_doc_len = 400;
                let truncated_doc = if cleaned.len() > max_doc_len {
                    // Find a good break point (end of sentence or word)
                    let truncate_at = cleaned[..max_doc_len]
                        .rfind(". ")
                        .map(|i| i + 1) // Include the period
                        .or_else(|| cleaned[..max_doc_len].rfind(' '))
                        .unwrap_or(max_doc_len);
                    format!("{}...", &cleaned[..truncate_at])
                } else {
                    cleaned
                };

                if !truncated_doc.is_empty() {
                    formatted_parts.push(truncated_doc);
                }
            }

            // If we couldn't parse anything meaningful, just format it simply
            if formatted_parts.is_empty() && !value.trim().is_empty() {
                let trimmed_value = value.trim();
                // If it looks like code, wrap in code block
                if trimmed_value.contains("fn ") ||
                    trimmed_value.contains("struct ") ||
                    trimmed_value.contains("::")
                {
                    // Truncate if needed
                    let display_value = if trimmed_value.len() > 500 {
                        format!("{}...", &trimmed_value[..500])
                    } else {
                        trimmed_value.to_string()
                    };
                    formatted_parts.push(format!("```rust\n{}\n```", display_value));
                } else {
                    formatted_parts.push(trimmed_value.to_string());
                }
            }

            HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: formatted_parts.join("\n\n"),
            })
        }
        // For other content types, return as-is
        other => other,
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
                    let langs: Vec<_> =
                        benchmark.implementations.keys().map(|l| l.as_str()).collect();
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

        // Auto-calibration settings
        "mode" => Some(
            "**mode:** `auto | fixed`\n\n\
            Execution mode for benchmarks:\n\n\
            - `auto` - Time-based calibration. Runs until `targetTime` is reached,\n  \
              automatically determining iteration count. *(Default)*\n\
            - `fixed` - Uses explicit `iterations` count.\n\n\
            Auto mode provides more accurate measurements for fast operations."
        ),
        "targetTime" => Some(
            "**targetTime:** `<duration>`\n\n\
            Target duration for auto-calibration mode.\n\n\
            The benchmark will run approximately this long, automatically\n\
            scaling the iteration count.\n\n\
            Examples: `3000ms`, `10s`, `1m`\n\n\
            Default: `3000ms` (3 seconds)"
        ),
        "minIterations" => Some(
            "**minIterations:** `<number>`\n\n\
            Minimum iterations for auto-calibration mode.\n\n\
            Even if targetTime is reached quickly, at least this many\n\
            iterations will be run.\n\n\
            Default: `10`"
        ),
        "maxIterations" => Some(
            "**maxIterations:** `<number>`\n\n\
            Maximum iterations for auto-calibration mode.\n\n\
            The benchmark will not exceed this iteration count,\n\
            even if targetTime hasn't been reached.\n\n\
            Default: `1000000`"
        ),
        "sink" => Some(
            "**sink:** `true | false`\n\n\
            Use sink/black-box pattern to prevent dead code elimination.\n\n\
            When enabled, the result of the benchmark expression is passed\n\
            to a sink function that prevents compiler optimizations from\n\
            eliminating the benchmarked code.\n\n\
            Default: `true`"
        ),
        "outlierDetection" => Some(
            "**outlierDetection:** `true | false`\n\n\
            Enable IQR-based outlier detection and removal.\n\n\
            When enabled, statistical outliers are identified using the\n\
            interquartile range (IQR) method and excluded from results.\n\
            This improves measurement stability.\n\n\
            Default: `true`"
        ),
        "cvThreshold" => Some(
            "**cvThreshold:** `<number>`\n\n\
            Coefficient of variation threshold (%) for stability warnings.\n\n\
            If the CV of measurements exceeds this threshold, a warning\n\
            is shown indicating the results may be unstable.\n\n\
            Default: `5` (5%)"
        ),
        "memory" => Some(
            "**memory:** `true | false`\n\n\
            Enable memory allocation profiling.\n\n\
            When enabled, tracks memory allocations during benchmark execution:\n\
            - **Go:** Uses `runtime.ReadMemStats` to measure bytes/allocs per op\n\
            - **TypeScript:** Uses `process.memoryUsage()` to track heap usage\n\n\
            Results appear in output when `showMemory: true` is set on charts.\n\n\
            Default: `false`"
        ),
        "concurrency" => Some(
            "**concurrency:** `<number>`\n\n\
            Number of concurrent goroutines/workers for parallel execution.\n\n\
            When set > 1, the benchmark runs with multiple parallel workers,\n\
            measuring throughput instead of single-threaded latency.\n\n\
            - **Go:** Uses goroutines with sync.WaitGroup\n\
            - **TypeScript:** Not yet supported\n\n\
            Default: `1` (single-threaded)"
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
            "**hex:** `\"0x...\"` or `@file(\"path\")`\n\n\
            Hex-encoded binary data.\n\n\
            Portable format that works across all languages.\n\n\
            **Inline:** `hex: \"deadbeef\"`\n\
            **From file:** `hex: @file(\"testdata/input.hex\")`"
        ),
        "@file" | "file" => Some(
            "**@file** `(\"path/to/file\")`\n\n\
            Load hex data from an external file.\n\n\
            The file should contain hex-encoded binary data.\n\n\
            **Example:**\n\
            ```\nfixture largeData {\n    hex: @file(\"testdata/large_input.hex\")\n}\n```\n\n\
            Paths are relative to the .bench file location."
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
            "**charting.drawBarChart** `(...params)`\n\n\
            Generate a bar chart comparing benchmark execution times.\n\n\
            **Basic Parameters:**\n\
            - `title` - Chart title (string)\n\
            - `description` - Chart description (string)\n\
            - `xlabel`, `ylabel` - Axis labels (string)\n\
            - `output` - Output filename (default: bar-chart.svg)\n\n\
            **Display Toggles:** (default: true unless noted)\n\
            - `showStats` - Show ops/sec and time per op\n\
            - `showConfig` - Show iterations/warmup/timeout\n\
            - `showWinCounts` - Show win counts in legend\n\
            - `showGeoMean` - Show geometric mean speedup\n\
            - `showDistribution` - Show p50/p99 (default: false)\n\
            - `compact` - Minimal chart mode (default: false)\n\n\
            **Filtering:**\n\
            - `minSpeedup` - Only show benchmarks with speedup >= N (number)\n\
            - `filterWinner` - Filter by winner: \"go\", \"ts\", \"all\"\n\
            - `includeBenchmarks`, `excludeBenchmarks` - Filter by name (array)\n\
            - `limit` - Max benchmarks to show (number)\n\n\
            **Sorting:**\n\
            - `sortBy` - \"speedup\", \"name\", \"time\", \"ops\"\n\
            - `sortOrder` - \"asc\" or \"desc\"\n\n\
            **Layout:**\n\
            - `width`, `barHeight`, `barGap`, `marginLeft` (pixels)\n\n\
            **Data Display:**\n\
            - `precision` - Decimal places (default: 2)\n\
            - `timeUnit` - \"auto\", \"ns\", \"us\", \"ms\", \"s\"\n\n\
            **Example:**\n\
            ```\nafter {\n    charting.drawBarChart(\n        title: \"Performance Comparison\",\n        sortBy: \"speedup\",\n        sortOrder: \"desc\",\n        limit: 10\n    )\n}\n```\n\n\
            *From `std::charting`*"
        ),
        "drawPieChart" => Some(
            "**charting.drawPieChart** `(...params)`\n\n\
            Generate a pie chart showing time distribution across benchmarks.\n\n\
            **Basic Parameters:**\n\
            - `title` - Chart title (string)\n\
            - `description` - Chart description (string)\n\
            - `output` - Output filename (default: pie-chart.svg)\n\n\
            **Display Toggles:**\n\
            - `showStats` - Show timing info in legend (default: true)\n\
            - `showTotalTime` - Show total time (default: false)\n\
            - `compact` - Minimal mode (default: false)\n\n\
            **Filtering:** Same as drawBarChart\n\n\
            **Example:**\n\
            ```\nafter {\n    charting.drawPieChart(\n        title: \"Time Distribution\",\n        showStats: true\n    )\n}\n```\n\n\
            *From `std::charting`*"
        ),
        "drawLineChart" => Some(
            "**charting.drawLineChart** `(...params)`\n\n\
            Generate a line chart for trend visualization.\n\n\
            **Basic Parameters:**\n\
            - `title` - Chart title (string)\n\
            - `description` - Chart description (string)\n\
            - `xlabel`, `ylabel` - Axis labels (string)\n\
            - `output` - Output filename (default: line-chart.svg)\n\n\
            **Display Toggles:**\n\
            - `showStats` - Show timing tooltips on hover (default: true)\n\
            - `compact` - Minimal mode (default: false)\n\n\
            **Filtering & Sorting:** Same as drawBarChart\n\n\
            **Data Display:**\n\
            - `precision` - Decimal places (default: 2)\n\
            - `timeUnit` - \"auto\", \"ns\", \"us\", \"ms\", \"s\"\n\n\
            **Example:**\n\
            ```\nafter {\n    charting.drawLineChart(\n        title: \"Performance Trends\",\n        sortBy: \"name\"\n    )\n}\n```\n\n\
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
            Implementations are compiled and executed via Go plugin system.",
        ),
        "ts" | "typescript" => Some(
            "**TypeScript** language\n\n\
            Implementations are transpiled and executed via embedded V8 runtime.",
        ),
        "rust" | "rs" => Some(
            "**Rust** language *(planned)*\n\n\
            Native Rust benchmark support.",
        ),
        "python" | "py" => Some(
            "**Python** language *(planned)*\n\n\
            Python benchmark support.",
        ),
        _ => None,
    }
}
