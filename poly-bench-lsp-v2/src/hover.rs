//! Hover provider for the LSP v2
//!
//! This module provides hover information for keywords and identifiers
//! in poly-bench files, including embedded Go code via gopls,
//! TypeScript code via typescript-language-server, and Rust code via rust-analyzer.

use poly_bench_syntax::Lang;
use tower_lsp::lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind, Position, Url};

use crate::{
    document::Document,
    embedded::{
        blocks_for_language, extract_embedded_blocks, find_block_at_offset, EmbeddedConfig,
    },
    gopls_client::init_gopls_client,
    hover_cache::{cache_hover, get_cached_hover},
    rust_analyzer_client::init_rust_analyzer_client,
    tsserver_client::init_tsserver_client,
    virtual_files::{VirtualFile, VirtualFileManagers},
};

/// Get hover information at a position with full embedded language support
pub fn get_hover(
    doc: &Document,
    position: Position,
    config: &EmbeddedConfig,
    uri: &Url,
    managers: &VirtualFileManagers,
) -> Option<Hover> {
    let source = doc.source_text();
    let offset = doc.position_to_byte(position.line as usize, position.character as usize);

    // Extract embedded blocks from the partial AST
    let blocks = extract_embedded_blocks(&doc.partial_ast);

    // Check if we're in an embedded code block
    if let Some(block) = find_block_at_offset(&blocks, offset) {
        // Check cache first
        if let Some(cached) = get_cached_hover(uri, position) {
            return cached;
        }

        let hover = match block.lang {
            Lang::Go => {
                if let Some(go_mod_root) = &config.go_mod_root {
                    get_gopls_hover(doc, uri, offset, &blocks, go_mod_root, managers)
                } else {
                    None
                }
            }
            Lang::TypeScript => {
                if let Some(ts_module_root) = &config.ts_module_root {
                    get_tsserver_hover(doc, uri, offset, &blocks, ts_module_root, managers)
                } else {
                    None
                }
            }
            Lang::Rust => {
                if let Some(rust_project_root) = &config.rust_project_root {
                    get_rust_analyzer_hover(doc, uri, offset, &blocks, rust_project_root, managers)
                } else {
                    None
                }
            }
            Lang::Python => None,
        };

        // Cache the result
        cache_hover(uri, position, hover.clone());

        if hover.is_some() {
            return hover;
        }

        // Fallback to stdlib symbols
        if let Some(hover) = get_stdlib_symbol_hover(doc, position) {
            return Some(hover);
        }
    }

    // Try DSL hover (keywords, AST nodes)
    get_dsl_hover(doc, position, &source)
}

/// Get hover from gopls for Go code
fn get_gopls_hover(
    doc: &Document,
    bench_uri: &Url,
    bench_offset: usize,
    blocks: &[crate::embedded::EmbeddedBlock],
    go_mod_root: &str,
    managers: &VirtualFileManagers,
) -> Option<Hover> {
    let client = init_gopls_client(go_mod_root)?;

    let bench_uri_str = bench_uri.as_str();
    let bench_path = bench_uri.to_file_path().ok()?;
    let bench_path_str = bench_path.to_string_lossy();

    let go_blocks: Vec<_> = blocks_for_language(blocks, Lang::Go);

    let virtual_file = managers.go.get_or_create(
        bench_uri_str,
        &bench_path_str,
        go_mod_root,
        &go_blocks,
        doc.version,
    );

    let go_position = virtual_file.bench_to_go(bench_offset)?;

    if let Err(e) =
        client.did_change(virtual_file.uri(), virtual_file.content(), virtual_file.version())
    {
        tracing::warn!("Failed to sync virtual Go file: {}", e);
        return None;
    }

    match client.hover(virtual_file.uri(), go_position.line, go_position.character) {
        Ok(Some(mut hover)) => {
            if let Some(ref go_range) = hover.range {
                if let Some(bench_start_offset) =
                    virtual_file.go_to_bench(go_range.start.line, go_range.start.character)
                {
                    if let Some(bench_end_offset) =
                        virtual_file.go_to_bench(go_range.end.line, go_range.end.character)
                    {
                        let (start_line, start_col) = doc.byte_to_position(bench_start_offset);
                        let (end_line, end_col) = doc.byte_to_position(bench_end_offset);
                        hover.range = Some(tower_lsp::lsp_types::Range {
                            start: Position {
                                line: start_line as u32,
                                character: start_col as u32,
                            },
                            end: Position { line: end_line as u32, character: end_col as u32 },
                        });
                    }
                }
            }
            Some(hover)
        }
        Ok(None) => None,
        Err(e) => {
            tracing::warn!("gopls hover failed: {}", e);
            None
        }
    }
}

/// Get hover from tsserver for TypeScript code
fn get_tsserver_hover(
    doc: &Document,
    bench_uri: &Url,
    bench_offset: usize,
    blocks: &[crate::embedded::EmbeddedBlock],
    ts_module_root: &str,
    managers: &VirtualFileManagers,
) -> Option<Hover> {
    let client = init_tsserver_client(ts_module_root)?;

    let bench_uri_str = bench_uri.as_str();
    let bench_path = bench_uri.to_file_path().ok()?;
    let bench_path_str = bench_path.to_string_lossy();

    let ts_blocks: Vec<_> = blocks_for_language(blocks, Lang::TypeScript);

    let virtual_file = managers.ts.get_or_create(
        bench_uri_str,
        &bench_path_str,
        ts_module_root,
        &ts_blocks,
        doc.version,
    );

    let ts_position = virtual_file.bench_to_ts(bench_offset)?;

    if let Err(e) =
        client.did_change(virtual_file.uri(), virtual_file.content(), virtual_file.version())
    {
        tracing::warn!("Failed to sync virtual TS file: {}", e);
        return None;
    }

    match client.hover(virtual_file.uri(), ts_position.line, ts_position.character) {
        Ok(Some(mut hover)) => {
            if let Some(ref ts_range) = hover.range {
                if let Some(bench_start_offset) =
                    virtual_file.ts_to_bench(ts_range.start.line, ts_range.start.character)
                {
                    if let Some(bench_end_offset) =
                        virtual_file.ts_to_bench(ts_range.end.line, ts_range.end.character)
                    {
                        let (start_line, start_col) = doc.byte_to_position(bench_start_offset);
                        let (end_line, end_col) = doc.byte_to_position(bench_end_offset);
                        hover.range = Some(tower_lsp::lsp_types::Range {
                            start: Position {
                                line: start_line as u32,
                                character: start_col as u32,
                            },
                            end: Position { line: end_line as u32, character: end_col as u32 },
                        });
                    }
                }
            }
            Some(hover)
        }
        Ok(None) => None,
        Err(e) => {
            tracing::warn!("tsserver hover failed: {}", e);
            None
        }
    }
}

/// Get hover from rust-analyzer for Rust code
fn get_rust_analyzer_hover(
    doc: &Document,
    bench_uri: &Url,
    bench_offset: usize,
    blocks: &[crate::embedded::EmbeddedBlock],
    rust_project_root: &str,
    managers: &VirtualFileManagers,
) -> Option<Hover> {
    let client = init_rust_analyzer_client(rust_project_root)?;

    let bench_uri_str = bench_uri.as_str();
    let bench_path = bench_uri.to_file_path().ok()?;
    let bench_path_str = bench_path.to_string_lossy();

    let rust_blocks: Vec<_> = blocks_for_language(blocks, Lang::Rust);

    let virtual_file = managers.rust.get_or_create(
        bench_uri_str,
        &bench_path_str,
        rust_project_root,
        &rust_blocks,
        doc.version,
    );

    let rust_position = virtual_file.bench_to_rust(bench_offset)?;

    if let Err(e) =
        client.did_change(virtual_file.uri(), virtual_file.content(), virtual_file.version())
    {
        tracing::warn!("Failed to sync virtual Rust file: {}", e);
        return None;
    }

    match client.hover(virtual_file.uri(), rust_position.line, rust_position.character) {
        Ok(Some(mut hover)) => {
            hover.contents = enhance_rust_hover_content(hover.contents);

            if let Some(ref rust_range) = hover.range {
                if let Some(bench_start_offset) =
                    virtual_file.rust_to_bench(rust_range.start.line, rust_range.start.character)
                {
                    if let Some(bench_end_offset) =
                        virtual_file.rust_to_bench(rust_range.end.line, rust_range.end.character)
                    {
                        let (start_line, start_col) = doc.byte_to_position(bench_start_offset);
                        let (end_line, end_col) = doc.byte_to_position(bench_end_offset);
                        hover.range = Some(tower_lsp::lsp_types::Range {
                            start: Position {
                                line: start_line as u32,
                                character: start_col as u32,
                            },
                            end: Position { line: end_line as u32, character: end_col as u32 },
                        });
                    }
                }
            }
            Some(hover)
        }
        Ok(None) => None,
        Err(e) => {
            tracing::warn!("rust-analyzer hover failed: {}", e);
            None
        }
    }
}

/// Enhance Rust hover content with proper markdown formatting
fn enhance_rust_hover_content(contents: HoverContents) -> HoverContents {
    match contents {
        HoverContents::Markup(markup) => {
            let value = &markup.value;

            if value.contains("```rust") || value.contains("```rs") {
                return HoverContents::Markup(markup);
            }

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

                if trimmed.is_empty() {
                    if in_signature && !code_lines.is_empty() {
                        formatted_parts.push(format!("```rust\n{}\n```", code_lines.join("\n")));
                        code_lines.clear();
                        in_signature = false;
                        seen_signature = true;
                    }
                    continue;
                }

                if trimmed.starts_with("_lsp_virtual") || trimmed.starts_with("polybench_runner") {
                    continue;
                }

                if !seen_signature &&
                    !in_signature &&
                    !trimmed.contains(' ') &&
                    trimmed.chars().all(|c| c.is_alphanumeric() || c == '_' || c == ':')
                {
                    continue;
                }

                let is_signature_start = trimmed.starts_with("fn ") ||
                    trimmed.starts_with("pub fn ") ||
                    trimmed.starts_with("async fn ") ||
                    trimmed.starts_with("pub async fn ") ||
                    trimmed.starts_with("struct ") ||
                    trimmed.starts_with("pub struct ") ||
                    trimmed.starts_with("type ") ||
                    trimmed.starts_with("pub type ") ||
                    trimmed.starts_with("const ") ||
                    trimmed.starts_with("pub const ") ||
                    trimmed.starts_with("impl ") ||
                    trimmed.starts_with("impl<") ||
                    trimmed.starts_with("trait ") ||
                    trimmed.starts_with("pub trait ") ||
                    trimmed.starts_with("enum ") ||
                    trimmed.starts_with("pub enum ") ||
                    trimmed.starts_with("use ") ||
                    trimmed.starts_with("pub use ");

                if is_signature_start {
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
                    code_lines.push(*line);
                } else {
                    if in_signature && !code_lines.is_empty() {
                        formatted_parts.push(format!("```rust\n{}\n```", code_lines.join("\n")));
                        code_lines.clear();
                        in_signature = false;
                        seen_signature = true;
                    }
                    doc_lines.push(*line);
                }
            }

            if !code_lines.is_empty() {
                formatted_parts.push(format!("```rust\n{}\n```", code_lines.join("\n")));
            }

            if !doc_lines.is_empty() {
                let doc_text = doc_lines.join(" ");
                let cleaned: String = doc_text.split_whitespace().collect::<Vec<_>>().join(" ");

                let max_doc_len = 400;
                let truncated_doc = if cleaned.len() > max_doc_len {
                    let truncate_at = cleaned[..max_doc_len]
                        .rfind(". ")
                        .map(|i| i + 1)
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

            if formatted_parts.is_empty() && !value.trim().is_empty() {
                let trimmed_value = value.trim();
                if trimmed_value.contains("fn ") ||
                    trimmed_value.contains("struct ") ||
                    trimmed_value.contains("::")
                {
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
        other => other,
    }
}

/// Get hover for DSL keywords and AST nodes
fn get_dsl_hover(doc: &Document, position: Position, source: &str) -> Option<Hover> {
    let point = tree_sitter::Point::new(position.line as usize, position.character as usize);

    let node = doc.tree.root_node().descendant_for_point_range(point, point)?;
    let kind = node.kind();
    let text = node.utf8_text(source.as_bytes()).ok()?;

    // Check for specific node types first
    let content = match kind {
        "suite" | "suite_keyword" => {
            let name = if kind == "suite" {
                node.child_by_field_name("name")
                    .and_then(|n| n.utf8_text(source.as_bytes()).ok())
                    .unwrap_or("<unnamed>")
            } else {
                "<unnamed>"
            };
            format!("**Suite**: `{}`\n\n{}", name, keyword_docs("suite").unwrap_or(""))
        }
        "benchmark" | "bench_keyword" => {
            let name = if kind == "benchmark" {
                node.child_by_field_name("name")
                    .and_then(|n| n.utf8_text(source.as_bytes()).ok())
                    .unwrap_or("<unnamed>")
            } else {
                "<unnamed>"
            };
            format!("**Benchmark**: `{}`\n\n{}", name, keyword_docs("bench").unwrap_or(""))
        }
        "fixture" | "fixture_keyword" => {
            let name = if kind == "fixture" {
                node.child_by_field_name("name")
                    .and_then(|n| n.utf8_text(source.as_bytes()).ok())
                    .unwrap_or("<unnamed>")
            } else {
                "<unnamed>"
            };
            format!("**Fixture**: `{}`\n\n{}", name, keyword_docs("fixture").unwrap_or(""))
        }
        "property_name" => get_property_documentation(text),
        "chart_function_name" => get_chart_function_documentation(text),
        "language_tag" => format!("**Language**: `{}`\n\nEmbedded {} code block", text, text),
        "identifier" => {
            // Check if it's a keyword
            if let Some(docs) = keyword_docs(text) {
                docs.to_string()
            } else if let Some(docs) = stdlib_symbol_docs(text) {
                docs.to_string()
            } else if let Some(docs) = stdlib_module_docs(text) {
                docs.to_string()
            } else {
                return None;
            }
        }
        _ => {
            // Try keyword lookup for the text
            if let Some(docs) = keyword_docs(text) {
                docs.to_string()
            } else if let Some(docs) = stdlib_symbol_docs(text) {
                docs.to_string()
            } else {
                return None;
            }
        }
    };

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: content,
        }),
        range: None,
    })
}

/// Get hover for stdlib symbols in embedded code
fn get_stdlib_symbol_hover(doc: &Document, position: Position) -> Option<Hover> {
    let source = doc.source_text();
    let point = tree_sitter::Point::new(position.line as usize, position.character as usize);

    let node = doc.tree.root_node().descendant_for_point_range(point, point)?;
    let text = node.utf8_text(source.as_bytes()).ok()?;

    if let Some(docs) = stdlib_symbol_docs(text) {
        return Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: docs.to_string(),
            }),
            range: None,
        });
    }

    None
}

/// Get documentation for a property name
fn get_property_documentation(name: &str) -> String {
    match name {
        "description" => {
            "**description**: `string`\n\nA human-readable description of the suite or benchmark."
                .to_string()
        }
        "iterations" => {
            "**iterations**: `number`\n\nNumber of benchmark iterations to run.".to_string()
        }
        "warmup" => {
            "**warmup**: `number`\n\nNumber of warmup iterations before measurement.".to_string()
        }
        "timeout" => {
            "**timeout**: `duration`\n\nMaximum time allowed for the benchmark (e.g., `30s`, `5000ms`)."
                .to_string()
        }
        "order" => {
            "**order**: `sequential | random`\n\nOrder in which benchmarks are executed."
                .to_string()
        }
        "baseline" => {
            "**baseline**: `string`\n\nLanguage to use as the baseline for comparison.".to_string()
        }
        "mode" => {
            "**mode**: `auto | fixed | adaptive`\n\nBenchmark execution mode.".to_string()
        }
        "targetTime" => {
            "**targetTime**: `duration`\n\nTarget time for adaptive mode.".to_string()
        }
        "sink" => {
            "**sink**: `boolean`\n\nWhether to sink (consume) the result to prevent optimization."
                .to_string()
        }
        "memory" => {
            "**memory**: `boolean`\n\nEnable memory allocation profiling.".to_string()
        }
        "outlierDetection" => {
            "**outlierDetection**: `boolean`\n\nEnable IQR-based outlier detection and removal."
                .to_string()
        }
        "cvThreshold" => {
            "**cvThreshold**: `number`\n\nCoefficient of variation threshold (%) for stability warnings."
                .to_string()
        }
        "tags" => "**tags**: `string[]`\n\nLabels for filtering and grouping benchmarks.".to_string(),
        "requires" => {
            "**requires**: `string[]`\n\nLanguages that must have implementations.".to_string()
        }
        _ => format!("**{}**", name),
    }
}

/// Get documentation for a chart function
fn get_chart_function_documentation(name: &str) -> String {
    match name {
        "drawSpeedupChart" => {
            "**drawSpeedupChart**\n\nDraw a chart showing relative speedup compared to baseline."
                .to_string()
        }
        "drawTable" => "**drawTable**\n\nGenerate a table of benchmark results.".to_string(),
        _ => format!("**{}**", name),
    }
}

/// Get documentation for a keyword
fn keyword_docs(word: &str) -> Option<&'static str> {
    match word {
        "suite" => Some(
            "**suite** `name { ... }`\n\n\
            Top-level benchmark suite container.\n\n\
            Contains setup blocks, fixtures, and benchmark definitions.",
        ),
        "bench" => Some(
            "**bench** `name { ... }`\n\n\
            Benchmark definition with language implementations.\n\n\
            ```\nbench encode {\n    go: encodeData(input)\n    ts: encodeData(input)\n}\n```",
        ),
        "setup" => Some(
            "**setup** `<lang> { ... }`\n\n\
            Per-language setup block with sections:\n\
            - `import` - Import statements\n\
            - `declare` - Package-level declarations\n\
            - `init` - Initialization code (runs once)\n\
            - `helpers` - Reusable helper functions",
        ),
        "fixture" => Some(
            "**fixture** `name { ... }`\n\n\
            Shared test data fixture.\n\n\
            Can contain:\n\
            - `hex:` - Hex-encoded binary data\n\
            - `go:` / `ts:` - Language-specific implementations\n\
            - `shape:` - JSON-like shape annotation",
        ),
        "import" => Some(
            "**import** `{ ... }` or `( ... )`\n\n\
            Import statements for the language.\n\n\
            For Go, use grouped imports:\n\
            ```\nimport (\n    \"fmt\"\n    \"encoding/json\"\n)\n```",
        ),
        "declare" => Some(
            "**declare** `{ ... }`\n\n\
            Package-level declarations (vars, types, consts).\n\n\
            Placed at package scope, outside any function.",
        ),
        "init" => Some(
            "**init** `{ ... }`\n\n\
            Initialization code that runs once before benchmarks.\n\n\
            Use for expensive setup that shouldn't be timed.",
        ),
        "helpers" => Some(
            "**helpers** `{ ... }`\n\n\
            Reusable helper functions available to all benchmarks.\n\n\
            Define functions that can be called from benchmark implementations.",
        ),
        "async" => Some(
            "**async** `init { ... }`\n\n\
            Mark init block as async (TypeScript only).\n\n\
            Allows using `await` in initialization code.",
        ),
        "description" => Some(
            "**description:** `\"...\"`\n\n\
            Human-readable description for the suite, fixture, or benchmark.",
        ),
        "iterations" => Some(
            "**iterations:** `<number>`\n\n\
            Number of benchmark iterations.\n\n\
            Default: 1000",
        ),
        "warmup" => Some(
            "**warmup:** `<number>`\n\n\
            Number of warmup iterations before measuring.\n\n\
            Default: 100",
        ),
        "timeout" => Some(
            "**timeout:** `<duration>`\n\n\
            Maximum execution time per benchmark.\n\n\
            Examples: `30s`, `500ms`, `1m`",
        ),
        "tags" => Some(
            "**tags:** `[\"tag1\", \"tag2\"]`\n\n\
            Labels for filtering and grouping benchmarks.",
        ),
        "requires" => Some(
            "**requires:** `[\"go\", \"ts\"]`\n\n\
            Languages that must have implementations.\n\n\
            Benchmarks missing required languages will error.",
        ),
        "order" => Some(
            "**order:** `sequential | parallel | random`\n\n\
            Execution order for benchmarks in the suite.\n\n\
            - `sequential` - Run in definition order (default)\n\
            - `parallel` - Run concurrently where supported\n\
            - `random` - Randomize order",
        ),
        "baseline" => Some(
            "**baseline:** `\"go\" | \"ts\"`\n\n\
            Baseline language for comparison ratios.\n\n\
            Other languages are compared against this baseline.",
        ),
        "mode" => Some(
            "**mode:** `auto | fixed`\n\n\
            Execution mode for benchmarks:\n\n\
            - `auto` - Time-based calibration. Runs until `targetTime` is reached,\n  \
              automatically determining iteration count. *(Default)*\n\
            - `fixed` - Uses explicit `iterations` count.\n\n\
            Auto mode provides more accurate measurements for fast operations.",
        ),
        "targetTime" => Some(
            "**targetTime:** `<duration>`\n\n\
            Target duration for auto-calibration mode.\n\n\
            The benchmark will run approximately this long, automatically\n\
            scaling the iteration count.\n\n\
            Examples: `3000ms`, `10s`, `1m`\n\n\
            Default: `3000ms` (3 seconds)",
        ),
        "sink" => Some(
            "**sink:** `true | false`\n\n\
            Use sink/black-box pattern to prevent dead code elimination.\n\n\
            When enabled, the result of the benchmark expression is passed\n\
            to a sink function that prevents compiler optimizations from\n\
            eliminating the benchmarked code.\n\n\
            Default: `true`",
        ),
        "outlierDetection" => Some(
            "**outlierDetection:** `true | false`\n\n\
            Enable IQR-based outlier detection and removal.\n\n\
            When enabled, statistical outliers are identified using the\n\
            interquartile range (IQR) method and excluded from results.\n\
            This improves measurement stability.\n\n\
            Default: `true`",
        ),
        "cvThreshold" => Some(
            "**cvThreshold:** `<number>`\n\n\
            Coefficient of variation threshold (%) for stability warnings.\n\n\
            If the CV of measurements exceeds this threshold, a warning\n\
            is shown indicating the results may be unstable.\n\n\
            Default: `5` (5%)",
        ),
        "memory" => Some(
            "**memory:** `true | false`\n\n\
            Enable memory allocation profiling.\n\n\
            When enabled, tracks memory allocations during benchmark execution:\n\
            - **Go:** Uses `runtime.ReadMemStats` to measure bytes/allocs per op\n\
            - **TypeScript:** Uses `process.memoryUsage()` to track heap usage\n\n\
            Default: `false`",
        ),
        "skip" => Some(
            "**skip** `<lang>:` `<condition>`\n\n\
            Skip benchmark if condition is true.\n\n\
            ```\nskip go: runtime.GOOS == \"windows\"\n```",
        ),
        "validate" => Some(
            "**validate** `<lang>:` `<expression>`\n\n\
            Validate benchmark result.\n\n\
            Expression should return a boolean.",
        ),
        "before" => Some(
            "**before** `<lang>:` `{ ... }`\n\n\
            Hook that runs once before all iterations.",
        ),
        "after" => Some(
            "**after** `<lang>:` `{ ... }` or `{ ... }`\n\n\
            Hook that runs once after all iterations.\n\n\
            **Benchmark-level hook:**\n\
            ```\nbench test {\n    after go: { cleanup() }\n}\n```\n\n\
            **Suite-level charting block:**\n\
            ```\nafter {\n    charting.drawSpeedupChart(title: \"Results\")\n}\n```",
        ),
        "each" => Some(
            "**each** `<lang>:` `{ ... }`\n\n\
            Hook that runs before each iteration.\n\n\
            Executed outside timing measurement.",
        ),
        "hex" => Some(
            "**hex:** `\"0x...\"` or `@file(\"path\")`\n\n\
            Hex-encoded binary data.\n\n\
            Portable format that works across all languages.\n\n\
            **Inline:** `hex: \"deadbeef\"`\n\
            **From file:** `hex: @file(\"testdata/input.hex\")`",
        ),
        "@file" | "file" => Some(
            "**@file** `(\"path/to/file\")`\n\n\
            Load hex data from an external file.\n\n\
            The file should contain hex-encoded binary data.\n\n\
            **Example:**\n\
            ```\nfixture largeData {\n    hex: @file(\"testdata/large_input.hex\")\n}\n```\n\n\
            Paths are relative to the .bench file location.",
        ),
        "shape" => Some(
            "**shape:** `\"type\"`\n\n\
            JSON-like type annotation for documentation.\n\n\
            Example: `shape: \"{ id: number, name: string }\"`",
        ),
        "sequential" => Some(
            "**sequential**\n\n\
            Run benchmarks in definition order.\n\n\
            This is the default execution order.",
        ),
        "parallel" => Some(
            "**parallel**\n\n\
            Run benchmarks concurrently where supported.\n\n\
            May improve total execution time.",
        ),
        "random" => Some(
            "**random**\n\n\
            Randomize benchmark execution order.\n\n\
            Helps detect order-dependent issues.",
        ),
        "use" => Some(
            "**use** `std::module`\n\n\
            Import a module from the poly-bench standard library.\n\n\
            Available modules:\n\
            - `anvil` - Anvil node integration (ANVIL_RPC_URL)\n\
            - `charting` - Chart generation (drawSpeedupChart, drawTable)\n\
            - `constants` - Mathematical constants (std_PI, std_E)",
        ),
        "globalSetup" => Some(
            "**globalSetup** `{ ... }`\n\n\
            Global setup block for suite-level initialization.\n\n\
            Can be placed inside a suite or at file level (for all suites).\n\n\
            **Available functions (with std::anvil):**\n\
            - `anvil.spawnAnvil()` - Spawn a local Anvil Ethereum node\n\
            - `anvil.spawnAnvil(fork: \"url\")` - Spawn with chain forking",
        ),
        "go" => Some(
            "**Go** language\n\n\
            Implementations are compiled and executed via Go plugin system.",
        ),
        "ts" | "typescript" => Some(
            "**TypeScript** language\n\n\
            Implementations are transpiled and executed via embedded V8 runtime.",
        ),
        "rust" | "rs" => Some(
            "**Rust** language\n\n\
            Native Rust benchmark support.",
        ),
        "python" | "py" => Some(
            "**Python** language *(planned)*\n\n\
            Python benchmark support.",
        ),
        _ => None,
    }
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
            **Requirements:** Anvil must be installed (part of Foundry toolchain)",
        ),
        "charting" => Some(
            "**std::charting**\n\n\
            Chart generation from benchmark results.\n\n\
            Use in a suite-level `after { }` block to generate charts after benchmarks complete.\n\n\
            **Provided functions:**\n\
            - `charting.drawSpeedupChart()` - Generate a speedup comparison chart\n\
            - `charting.drawTable()` - Generate a data table",
        ),
        "constants" => Some(
            "**std::constants**\n\n\
            Mathematical constants from the poly-bench standard library.\n\n\
            **Provided constants:**\n\
            - `std_PI` - Pi (π ≈ 3.14159265358979323846)\n\
            - `std_E` - Euler's number (e ≈ 2.71828182845904523536)",
        ),
        "std" => Some(
            "**std**\n\n\
            Poly-bench standard library namespace.\n\n\
            Use `use std::module` to import a standard library module.\n\n\
            Available modules:\n\
            - `anvil` - Local Ethereum node management\n\
            - `charting` - Chart generation from benchmark results\n\
            - `constants` - Mathematical constants",
        ),
        _ => None,
    }
}

/// Get documentation for stdlib symbols
fn stdlib_symbol_docs(symbol: &str) -> Option<&'static str> {
    match symbol {
        "spawnAnvil" => Some(
            "**anvil.spawnAnvil** `()` or `(fork: \"url\")`\n\n\
            Spawn a local Anvil Ethereum node.\n\n\
            Anvil is started before benchmarks and stopped after.\n\
            The RPC URL is available as `anvil.ANVIL_RPC_URL` in benchmark code.\n\n\
            **Options:**\n\
            - `fork: \"url\"` - Fork from an existing chain",
        ),
        "ANVIL_RPC_URL" => Some(
            "```go\nvar ANVIL_RPC_URL string\n```\n\n\
            **anvil.ANVIL_RPC_URL** - Anvil RPC endpoint URL.\n\n\
            When `use std::anvil` is specified with `anvil.spawnAnvil()`,\n\
            poly-bench automatically starts an Anvil node and sets this variable\n\
            to its RPC URL (e.g., `http://127.0.0.1:8545`).\n\n\
            *From `std::anvil`*",
        ),
        "drawSpeedupChart" => Some(
            "**charting.drawSpeedupChart** `(...params)`\n\n\
            Generate a chart showing relative speedup compared to baseline.\n\n\
            **Basic Parameters:**\n\
            - `title` - Chart title (string)\n\
            - `baselineBenchmark` - Benchmark to use as baseline\n\
            - `rowCount` - Number of benchmark cards per row in combined charts\n\n\
            *From `std::charting`*",
        ),
        "drawTable" => Some(
            "**charting.drawTable** `(...params)`\n\n\
            Generate a table of benchmark results.\n\n\
            **Basic Parameters:**\n\
            - `title` - Table title (string)\n\n\
            *From `std::charting`*",
        ),
        "PI" | "std_PI" => Some(
            "```go\nconst PI float64 = 3.14159265358979323846\n```\n\n\
            **Pi (π)** - The ratio of a circle's circumference to its diameter.\n\n\
            *From `std::constants`*",
        ),
        "E" | "std_E" => Some(
            "```go\nconst E float64 = 2.71828182845904523536\n```\n\n\
            **Euler's number (e)** - The base of natural logarithms.\n\n\
            *From `std::constants`*",
        ),
        _ => None,
    }
}
