//! Completion provider for the LSP
//!
//! This module provides context-aware code completions
//! for poly-bench files.

use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, InsertTextFormat, Position,
};

use super::document::ParsedDocument;

/// Get completions at a position
pub fn get_completions(doc: &ParsedDocument, position: Position) -> Vec<CompletionItem> {
    let mut items = Vec::new();

    // Get the text before the cursor to determine context
    let line_text = match doc.text_before_position(position) {
        Some(text) => text,
        None => return items,
    };

    let trimmed = line_text.trim();

    // Determine context and provide relevant completions
    let context = determine_context(doc, position, trimmed);

    match context {
        Context::TopLevel => {
            items.extend(top_level_completions());
        }
        Context::InsideSuite => {
            items.extend(suite_body_completions());
        }
        Context::InsideSetup => {
            items.extend(setup_section_completions());
        }
        Context::InsideBench => {
            items.extend(bench_body_completions());
        }
        Context::InsideFixture => {
            items.extend(fixture_body_completions());
        }
        Context::AfterColon(keyword) => {
            items.extend(after_colon_completions(&keyword));
        }
        Context::UseStdModule => {
            items.extend(stdlib_module_completions());
        }
        Context::InsideGlobalSetup => {
            items.extend(global_setup_completions());
        }
        Context::Unknown => {
            // Provide all keywords as fallback
            items.extend(all_keyword_completions());
        }
    }

    items
}

#[derive(Debug)]
enum Context {
    TopLevel,
    InsideSuite,
    InsideSetup,
    InsideBench,
    InsideFixture,
    InsideGlobalSetup,
    AfterColon(String),
    /// After typing "use std::" - suggests stdlib modules
    UseStdModule,
    Unknown,
}

/// Determine the completion context based on cursor position
fn determine_context(doc: &ParsedDocument, position: Position, line_text: &str) -> Context {
    // Check for "use std::" pattern - suggesting stdlib modules
    let trimmed = line_text.trim();
    if trimmed == "use std::" || trimmed.starts_with("use std::") {
        return Context::UseStdModule;
    }
    
    // Check if we're after a colon
    if line_text.ends_with(':') || line_text.contains(": ") {
        if let Some(keyword) = extract_keyword_before_colon(line_text) {
            return Context::AfterColon(keyword);
        }
    }

    // Simple heuristic: count braces to determine nesting
    let offset = match doc.position_to_offset(position) {
        Some(o) => o,
        None => return Context::Unknown,
    };

    let text_before = &doc.source[..offset];
    let mut depth = 0;
    let mut last_keyword = None;

    // Simple scanner for context detection
    let mut chars = text_before.chars().peekable();
    let mut current_word = String::new();

    while let Some(c) = chars.next() {
        match c {
            '{' => {
                depth += 1;
                if !current_word.is_empty() {
                    last_keyword = Some(current_word.clone());
                }
                current_word.clear();
            }
            '}' => {
                depth -= 1;
                current_word.clear();
            }
            c if c.is_alphanumeric() || c == '_' => {
                current_word.push(c);
            }
            _ => {
                current_word.clear();
            }
        }
    }

    match depth {
        0 => Context::TopLevel,
        1 => {
            // Inside a top-level block (suite or globalSetup)
            match last_keyword.as_deref() {
                Some("suite") => Context::InsideSuite,
                Some("globalSetup") => Context::InsideGlobalSetup,
                _ => Context::InsideSuite,
            }
        }
        2 => {
            // Inside a nested block (setup, bench, fixture)
            match last_keyword.as_deref() {
                Some("setup") => Context::InsideSetup,
                Some("bench") => Context::InsideBench,
                Some("fixture") => Context::InsideFixture,
                Some("go") | Some("ts") => Context::InsideSetup,
                _ => Context::InsideSuite,
            }
        }
        _ => Context::Unknown,
    }
}

fn extract_keyword_before_colon(line_text: &str) -> Option<String> {
    let parts: Vec<&str> = line_text.split(':').collect();
    if parts.is_empty() {
        return None;
    }

    let before_colon = parts[0].trim();
    let words: Vec<&str> = before_colon.split_whitespace().collect();

    words.last().map(|s| s.to_string())
}

fn top_level_completions() -> Vec<CompletionItem> {
    vec![
        completion_item(
            "suite",
            "suite ${1:name} {\n    $0\n}",
            "Top-level benchmark suite",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "use std::",
            "use std::${1|constants,anvil|}",
            "Import from standard library",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "globalSetup",
            "globalSetup {\n    spawnAnvil()$0\n}",
            "Global setup block for spawning Anvil",
            CompletionItemKind::KEYWORD,
        ),
    ]
}

fn global_setup_completions() -> Vec<CompletionItem> {
    vec![
        completion_item(
            "spawnAnvil",
            "spawnAnvil()",
            "Spawn a local Anvil Ethereum node",
            CompletionItemKind::FUNCTION,
        ),
        completion_item(
            "spawnAnvil with fork",
            "spawnAnvil(fork: \"${1:https://eth-mainnet.g.alchemy.com/v2/...}\")",
            "Spawn Anvil with chain forking",
            CompletionItemKind::FUNCTION,
        ),
    ]
}

fn suite_body_completions() -> Vec<CompletionItem> {
    vec![
        completion_item(
            "setup go",
            "setup go {\n    import ($1)\n    init {\n        $0\n    }\n}",
            "Go setup block",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "setup ts",
            "setup ts {\n    import {\n        $1\n    }\n    init {\n        $0\n    }\n}",
            "TypeScript setup block",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "fixture",
            "fixture ${1:name} {\n    $0\n}",
            "Shared test data fixture",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "bench",
            "bench ${1:name} {\n    go: $2\n    ts: $0\n}",
            "Benchmark definition",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "description",
            "description: \"$0\"",
            "Suite description",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "iterations",
            "iterations: ${1:1000}",
            "Default iterations",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "warmup",
            "warmup: ${1:100}",
            "Warmup iterations",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "timeout",
            "timeout: ${1:30s}",
            "Suite timeout",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "requires",
            "requires: [\"${1:go}\", \"${2:ts}\"]",
            "Required language implementations",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "order",
            "order: ${1|sequential,parallel,random|}",
            "Benchmark execution order",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "compare",
            "compare: true",
            "Enable comparison tables",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "baseline",
            "baseline: \"${1|go,ts|}\"",
            "Baseline language for comparison",
            CompletionItemKind::PROPERTY,
        ),
    ]
}

fn setup_section_completions() -> Vec<CompletionItem> {
    vec![
        completion_item(
            "import",
            "import {\n    $0\n}",
            "Import statements",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "declare",
            "declare {\n    $0\n}",
            "Package-level declarations",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "init",
            "init {\n    $0\n}",
            "Initialization code",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "async init",
            "async init {\n    $0\n}",
            "Async initialization (TypeScript)",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "helpers",
            "helpers {\n    $0\n}",
            "Helper functions",
            CompletionItemKind::KEYWORD,
        ),
    ]
}

fn bench_body_completions() -> Vec<CompletionItem> {
    vec![
        completion_item(
            "go:",
            "go: $0",
            "Go implementation",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "ts:",
            "ts: $0",
            "TypeScript implementation",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "description",
            "description: \"$0\"",
            "Benchmark description",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "iterations",
            "iterations: ${1:1000}",
            "Override iterations",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "warmup",
            "warmup: ${1:100}",
            "Override warmup",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "timeout",
            "timeout: ${1:30s}",
            "Benchmark timeout",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "tags",
            "tags: [\"$0\"]",
            "Benchmark tags",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "skip go",
            "skip go: $0",
            "Skip condition for Go",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "skip ts",
            "skip ts: $0",
            "Skip condition for TypeScript",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "before go",
            "before go: {\n    $0\n}",
            "Before hook for Go",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "after go",
            "after go: {\n    $0\n}",
            "After hook for Go",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "each go",
            "each go: {\n    $0\n}",
            "Per-iteration hook for Go",
            CompletionItemKind::KEYWORD,
        ),
    ]
}

fn fixture_body_completions() -> Vec<CompletionItem> {
    vec![
        completion_item(
            "hex",
            "hex: \"$0\"",
            "Hex-encoded data",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "go:",
            "go: {\n    $0\n}",
            "Go fixture implementation",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "ts:",
            "ts: {\n    $0\n}",
            "TypeScript fixture implementation",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "description",
            "description: \"$0\"",
            "Fixture description",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "shape",
            "shape: \"$0\"",
            "Type shape annotation",
            CompletionItemKind::PROPERTY,
        ),
    ]
}

fn after_colon_completions(keyword: &str) -> Vec<CompletionItem> {
    match keyword {
        "order" => vec![
            simple_completion("sequential", CompletionItemKind::ENUM_MEMBER),
            simple_completion("parallel", CompletionItemKind::ENUM_MEMBER),
            simple_completion("random", CompletionItemKind::ENUM_MEMBER),
        ],
        "baseline" => vec![
            simple_completion("\"go\"", CompletionItemKind::ENUM_MEMBER),
            simple_completion("\"ts\"", CompletionItemKind::ENUM_MEMBER),
        ],
        "compare" => vec![
            simple_completion("true", CompletionItemKind::ENUM_MEMBER),
            simple_completion("false", CompletionItemKind::ENUM_MEMBER),
        ],
        _ => vec![],
    }
}

fn all_keyword_completions() -> Vec<CompletionItem> {
    let mut items = Vec::new();
    items.extend(top_level_completions());
    items.extend(suite_body_completions());
    items.extend(setup_section_completions());
    items.extend(bench_body_completions());
    items
}

/// Completions for stdlib module names after "use std::"
fn stdlib_module_completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "anvil".to_string(),
            kind: Some(CompletionItemKind::MODULE),
            detail: Some("Anvil Ethereum node (ANVIL_RPC_URL)".to_string()),
            documentation: Some(tower_lsp::lsp_types::Documentation::MarkupContent(
                tower_lsp::lsp_types::MarkupContent {
                    kind: tower_lsp::lsp_types::MarkupKind::Markdown,
                    value: "**std::anvil**\n\nAutomatically spawns a local Anvil node:\n- `ANVIL_RPC_URL` - RPC endpoint URL\n\nAnvil starts when benchmarks begin and stops when they complete.".to_string(),
                }
            )),
            ..Default::default()
        },
        CompletionItem {
            label: "constants".to_string(),
            kind: Some(CompletionItemKind::MODULE),
            detail: Some("Mathematical constants (std_PI, std_E)".to_string()),
            documentation: Some(tower_lsp::lsp_types::Documentation::MarkupContent(
                tower_lsp::lsp_types::MarkupContent {
                    kind: tower_lsp::lsp_types::MarkupKind::Markdown,
                    value: "**std::constants**\n\nProvides mathematical constants:\n- `std_PI` - Pi (π ≈ 3.14159)\n- `std_E` - Euler's number (e ≈ 2.71828)".to_string(),
                }
            )),
            ..Default::default()
        },
    ]
}

fn completion_item(
    label: &str,
    insert_text: &str,
    detail: &str,
    kind: CompletionItemKind,
) -> CompletionItem {
    CompletionItem {
        label: label.to_string(),
        kind: Some(kind),
        detail: Some(detail.to_string()),
        insert_text: Some(insert_text.to_string()),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        ..Default::default()
    }
}

fn simple_completion(label: &str, kind: CompletionItemKind) -> CompletionItem {
    CompletionItem {
        label: label.to_string(),
        kind: Some(kind),
        ..Default::default()
    }
}
