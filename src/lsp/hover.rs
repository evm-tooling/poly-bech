//! Hover provider for the LSP
//!
//! This module provides hover information for keywords and identifiers
//! in poly-bench files.

use tower_lsp::lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind, Position};

use super::document::ParsedDocument;

/// Get hover information at a position
pub fn get_hover(doc: &ParsedDocument, position: Position) -> Option<Hover> {
    let (word, range) = doc.word_at_position(position)?;

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
            "**after** `<lang>:` `{ ... }`\n\n\
            Hook that runs once after all iterations."
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

        _ => None,
    }
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
