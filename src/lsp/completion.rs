//! Completion provider for the LSP
//!
//! This module provides context-aware code completions
//! for poly-bench files.

use poly_bench::stdlib::{self, StdlibSymbolKind};
use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, InsertTextFormat, Position,
};

use super::document::ParsedDocument;

/// Get completions at a position
/// 
/// `trigger_char` is the character that triggered completion (e.g., "." or ":"),
/// provided by the LSP client when available.
pub fn get_completions(doc: &ParsedDocument, position: Position, trigger_char: Option<&str>) -> Vec<CompletionItem> {
    let mut items = Vec::new();

    // Get the text before the cursor to determine context
    let line_text = match doc.text_before_position(position) {
        Some(text) => text,
        None => return items,
    };

    let trimmed = line_text.trim();
    
    // If triggered by ".", check for module dot access immediately
    // This handles the case where VS Code sends completion before document is fully updated
    if trigger_char == Some(".") {
        // Check if the word before the dot is a known stdlib module
        if let Some(module_name) = extract_module_name_before_trigger(trimmed) {
            let stdlib_imports = detect_stdlib_imports(doc);
            if stdlib_imports.contains(&module_name) {
                // Return only module member completions
                return stdlib_module_member_completions(&module_name, &stdlib_imports);
            }
        }
    }
    
    // If triggered by ":", check for use statement pattern immediately
    if trigger_char == Some(":") {
        // Check if this looks like a use statement
        if trimmed.starts_with("use") {
            // Return stdlib module completions
            return stdlib_module_completions();
        }
    }

    // Determine context and provide relevant completions
    let context = determine_context(doc, position, trimmed);

    // Detect stdlib imports for stdlib-aware completions
    let stdlib_imports = detect_stdlib_imports(doc);

    match context {
        Context::TopLevel => {
            items.extend(top_level_completions());
        }
        Context::InsideSuite => {
            items.extend(suite_body_completions());
            // Add stdlib module names for autocomplete (e.g., typing "anvil" to get "anvil.")
            items.extend(stdlib_module_name_completions(&stdlib_imports));
        }
        Context::InsideSetup => {
            items.extend(setup_section_completions());
            // Add stdlib module names for autocomplete
            items.extend(stdlib_module_name_completions(&stdlib_imports));
        }
        Context::InsideBench => {
            items.extend(bench_body_completions());
            // Add stdlib module names for autocomplete
            items.extend(stdlib_module_name_completions(&stdlib_imports));
        }
        Context::InsideFixture => {
            items.extend(fixture_body_completions());
            // Add stdlib module names for autocomplete
            items.extend(stdlib_module_name_completions(&stdlib_imports));
        }
        Context::AfterColon(keyword) => {
            items.extend(after_colon_completions(&keyword));
            // Also add stdlib module names for expressions
            items.extend(stdlib_module_name_completions(&stdlib_imports));
        }
        Context::UseStdModule => {
            items.extend(stdlib_module_completions());
        }
        Context::InsideGlobalSetup => {
            items.extend(global_setup_completions(&stdlib_imports));
        }
        Context::ModuleDotAccess(module_name) => {
            // User typed "anvil." - show all symbols from that module
            items.extend(stdlib_module_member_completions(&module_name, &stdlib_imports));
        }
        Context::Unknown => {
            // Provide all keywords as fallback
            items.extend(all_keyword_completions());
            // Also add stdlib module names
            items.extend(stdlib_module_name_completions(&stdlib_imports));
        }
    }

    items
}

/// Detect which stdlib modules are imported in the document
fn detect_stdlib_imports(doc: &ParsedDocument) -> Vec<String> {
    let mut imports = Vec::new();
    
    // First, try to get imports from the AST if available
    if let Some(ref ast) = doc.ast {
        for use_std in &ast.use_stds {
            if !imports.contains(&use_std.module) {
                imports.push(use_std.module.clone());
            }
        }
    }
    
    // Also scan the source text for use statements (in case AST is incomplete)
    // This ensures completions work even when the file has parse errors
    for line in doc.source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("use std::") {
            // Extract module name: "use std::anvil" -> "anvil"
            if let Some(module) = trimmed.strip_prefix("use std::") {
                let module = module.trim();
                if !module.is_empty() && !imports.contains(&module.to_string()) {
                    imports.push(module.to_string());
                }
            }
        }
    }
    
    imports
}

/// Get completions for stdlib module names (e.g., "anvil", "constants")
/// These show up when the user might want to access module members
fn stdlib_module_name_completions(imports: &[String]) -> Vec<CompletionItem> {
    let mut items = Vec::new();
    
    for module in imports {
        let symbols = stdlib::get_module_symbols(module);
        if !symbols.is_empty() {
            // Add the module name as a completion - when selected, user can type "."
            items.push(CompletionItem {
                label: module.clone(),
                kind: Some(CompletionItemKind::MODULE),
                detail: Some(format!("std::{} module", module)),
                documentation: Some(tower_lsp::lsp_types::Documentation::MarkupContent(
                    tower_lsp::lsp_types::MarkupContent {
                        kind: tower_lsp::lsp_types::MarkupKind::Markdown,
                        value: format!(
                            "**{}** module\n\nType `{}.` to see available members:\n{}",
                            module,
                            module,
                            symbols.iter()
                                .map(|s| format!("- `{}.{}` - {}", module, s.name, s.description))
                                .collect::<Vec<_>>()
                                .join("\n")
                        ),
                    }
                )),
                insert_text: Some(module.clone()),
                ..Default::default()
            });
        }
    }
    
    items
}

/// Get completions for members of a specific stdlib module (after typing "module.")
fn stdlib_module_member_completions(module_name: &str, imports: &[String]) -> Vec<CompletionItem> {
    let mut items = Vec::new();
    
    // Only provide completions if the module is imported
    if !imports.contains(&module_name.to_string()) {
        return items;
    }
    
    let symbols = stdlib::get_module_symbols(module_name);
    
    for symbol in symbols {
        let kind = match symbol.kind {
            StdlibSymbolKind::Function => CompletionItemKind::FUNCTION,
            StdlibSymbolKind::Constant => CompletionItemKind::CONSTANT,
            StdlibSymbolKind::Variable => CompletionItemKind::VARIABLE,
        };
        
        let insert_text = if symbol.kind == StdlibSymbolKind::Function {
            // For functions, add parentheses
            format!("{}()", symbol.name)
        } else {
            symbol.name.to_string()
        };
        
        items.push(CompletionItem {
            label: symbol.name.to_string(),
            kind: Some(kind),
            detail: Some(format!("{}.{} - {}", module_name, symbol.name, symbol.description)),
            documentation: Some(tower_lsp::lsp_types::Documentation::MarkupContent(
                tower_lsp::lsp_types::MarkupContent {
                    kind: tower_lsp::lsp_types::MarkupKind::Markdown,
                    value: symbol.documentation.to_string(),
                }
            )),
            insert_text: Some(insert_text),
            insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
            ..Default::default()
        });
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
    /// After typing a module name followed by "." (e.g., "anvil.")
    /// Contains the module name
    ModuleDotAccess(String),
    Unknown,
}

/// Determine the completion context based on cursor position
fn determine_context(doc: &ParsedDocument, position: Position, line_text: &str) -> Context {
    // Check for "use std::" pattern - suggesting stdlib modules
    // Handle various patterns: "use std::", "use::", "use:", "use ::", etc.
    let trimmed = line_text.trim();
    
    // Check for use statement completion patterns
    if is_use_statement_pattern(trimmed) {
        return Context::UseStdModule;
    }
    
    // Check for module dot access pattern (e.g., "anvil.", "constants.")
    // This enables autocomplete for imported module members
    if let Some(module_name) = extract_module_before_dot(trimmed) {
        // Verify this module is imported
        let stdlib_imports = detect_stdlib_imports(doc);
        if stdlib_imports.contains(&module_name) {
            return Context::ModuleDotAccess(module_name);
        }
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
            // Inside a nested block (setup, bench, fixture, or globalSetup inside suite)
            match last_keyword.as_deref() {
                Some("setup") => Context::InsideSetup,
                Some("bench") => Context::InsideBench,
                Some("fixture") => Context::InsideFixture,
                Some("globalSetup") => Context::InsideGlobalSetup,
                Some("go") | Some("ts") => Context::InsideSetup,
                _ => Context::InsideSuite,
            }
        }
        _ => Context::Unknown,
    }
}

/// Extract the module name when a trigger character was typed
/// For example, if line_text is "    anvil" and trigger is ".", return Some("anvil")
fn extract_module_name_before_trigger(line_text: &str) -> Option<String> {
    let trimmed = line_text.trim();
    
    // Known stdlib module names
    let known_modules = ["anvil", "constants"];
    
    // Get the last word on the line
    let words: Vec<&str> = trimmed.split_whitespace().collect();
    if let Some(last_word) = words.last() {
        let word = last_word.trim();
        // Check if it's a known module (without the dot, since trigger char comes separately)
        if known_modules.contains(&word) {
            return Some(word.to_string());
        }
        // Also check if it ends with a known module followed by dot
        // (for cases where the doc was already updated)
        for module in known_modules {
            if word == module || word.ends_with(&format!("{}.", module)) || word == format!("{}.", module) {
                return Some(module.to_string());
            }
        }
    }
    
    None
}

/// Check if the line is a use statement pattern where we should suggest stdlib modules
/// Handles: "use:", "use::", "use std:", "use std::", "use ::", etc.
fn is_use_statement_pattern(line_text: &str) -> bool {
    let trimmed = line_text.trim();
    
    // Pattern: "use std::" or "use std::mod"
    if trimmed.starts_with("use std::") {
        return true;
    }
    
    // Pattern: "use std:" (incomplete)
    if trimmed == "use std:" {
        return true;
    }
    
    // Pattern: ends with "use" followed by colon variants at end of line
    // e.g., user typed "use" and is about to type "::"
    if trimmed == "use" {
        return false;  // Don't trigger yet, wait for colons
    }
    
    false
}

/// Extract module name if the line contains or ends with "module." pattern
/// Returns the module name if the cursor is positioned after "module."
fn extract_module_before_dot(_line_text: &str) -> Option<String> {
    let line_text = _line_text.trim();

    // Check if line contains a dot
    if !line_text.contains('.') {
        return None;
    }
    
    // Known stdlib module names
    let known_modules = ["anvil", "constants"];
    
    // Check for pattern: "module." at the end (user just typed the dot)
    if line_text.ends_with('.') {
        let without_dot = line_text.trim_end_matches('.');
        let words: Vec<&str> = without_dot.split_whitespace().collect();
        
        if let Some(last_word) = words.last() {
            let word = last_word.trim();
            if known_modules.contains(&word) {
                return Some(word.to_string());
            }
        }
    }
    
    // Check for pattern: "module.partial" where user is typing after the dot
    // e.g., "anvil.spawn" while typing "spawnAnvil"
    for module in known_modules {
        let pattern = format!("{}.", module);
        if line_text.contains(&pattern) {
            // Check if the pattern is at a word boundary
            // e.g., "  anvil." or "anvil.sp"
            let trimmed = line_text.trim();
            
            // Find where module. appears
            if let Some(pos) = trimmed.rfind(&pattern) {
                // Check that before the module name is either start of line or whitespace
                if pos == 0 || trimmed[..pos].ends_with(char::is_whitespace) {
                    return Some(module.to_string());
                }
            }
        }
    }
    
    None
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

fn global_setup_completions(stdlib_imports: &[String]) -> Vec<CompletionItem> {
    let mut items = Vec::new();
    
    // If anvil module is imported, suggest anvil.spawnAnvil()
    if stdlib_imports.contains(&"anvil".to_string()) {
        items.push(completion_item(
            "anvil.spawnAnvil",
            "anvil.spawnAnvil()",
            "Spawn a local Anvil Ethereum node",
            CompletionItemKind::FUNCTION,
        ));
        items.push(completion_item(
            "anvil.spawnAnvil with fork",
            "anvil.spawnAnvil(fork: \"${1:https://eth-mainnet.g.alchemy.com/v2/...}\")",
            "Spawn Anvil with chain forking",
            CompletionItemKind::FUNCTION,
        ));
    }
    
    // Also add module names for dot access
    items.extend(stdlib_module_name_completions(stdlib_imports));
    
    items
}

fn suite_body_completions() -> Vec<CompletionItem> {
    vec![
        // globalSetup block (inside suite)
        completion_item(
            "globalSetup",
            "globalSetup {\n    $0\n}",
            "Global setup block for suite-level initialization",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "globalSetup with anvil",
            "globalSetup {\n    anvil.spawnAnvil()$0\n}",
            "Global setup with Anvil node",
            CompletionItemKind::KEYWORD,
        ),
        
        // Setup blocks
        completion_item(
            "setup",
            "setup ${1|go,ts|} {\n    $0\n}",
            "Language-specific setup block",
            CompletionItemKind::KEYWORD,
        ),
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
        
        // Fixture and bench
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
        
        // Suite-level lifecycle hooks
        completion_item(
            "before",
            "before ${1|go,ts|}: {\n    $0\n}",
            "Suite-level before hook",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "after",
            "after ${1|go,ts|}: {\n    $0\n}",
            "Suite-level after hook",
            CompletionItemKind::KEYWORD,
        ),
        
        // Configuration properties
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
        completion_item(
            "tags",
            "tags: [\"$0\"]",
            "Suite-level tags",
            CompletionItemKind::PROPERTY,
        ),
    ]
}

fn setup_section_completions() -> Vec<CompletionItem> {
    vec![
        // Core setup section keywords
        completion_item(
            "import",
            "import {\n    $0\n}",
            "Import statements block",
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
            "Initialization code block",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "async",
            "async $0",
            "Async modifier (for TypeScript init)",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "async init",
            "async init {\n    $0\n}",
            "Async initialization (TypeScript only)",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "helpers",
            "helpers {\n    $0\n}",
            "Helper function definitions",
            CompletionItemKind::KEYWORD,
        ),
        
        // Go-specific import syntax
        completion_item(
            "import (Go)",
            "import (\n    \"$0\"\n)",
            "Go import with parentheses",
            CompletionItemKind::SNIPPET,
        ),
        
        // TypeScript-specific import syntax
        completion_item(
            "import (TypeScript)",
            "import {\n    $1 from \"$0\"\n}",
            "TypeScript import block",
            CompletionItemKind::SNIPPET,
        ),
    ]
}

fn bench_body_completions() -> Vec<CompletionItem> {
    vec![
        // Language implementations
        completion_item(
            "go:",
            "go: $0",
            "Go implementation",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "go: (block)",
            "go: {\n    $0\n}",
            "Go implementation (multi-line)",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "ts:",
            "ts: $0",
            "TypeScript implementation",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "ts: (block)",
            "ts: {\n    $0\n}",
            "TypeScript implementation (multi-line)",
            CompletionItemKind::PROPERTY,
        ),
        
        // Configuration properties
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
            "Benchmark tags for filtering",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "validate",
            "validate: $0",
            "Validation expression",
            CompletionItemKind::PROPERTY,
        ),
        
        // Skip conditions
        completion_item(
            "skip",
            "skip ${1|go,ts|}: $0",
            "Skip condition for a language",
            CompletionItemKind::KEYWORD,
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
        
        // Lifecycle hooks - Go
        completion_item(
            "before",
            "before ${1|go,ts|}: {\n    $0\n}",
            "Before hook (runs once before benchmark)",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "before go",
            "before go: {\n    $0\n}",
            "Before hook for Go",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "after",
            "after ${1|go,ts|}: {\n    $0\n}",
            "After hook (runs once after benchmark)",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "after go",
            "after go: {\n    $0\n}",
            "After hook for Go",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "each",
            "each ${1|go,ts|}: {\n    $0\n}",
            "Each hook (runs per iteration)",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "each go",
            "each go: {\n    $0\n}",
            "Per-iteration hook for Go",
            CompletionItemKind::KEYWORD,
        ),
        
        // Lifecycle hooks - TypeScript
        completion_item(
            "before ts",
            "before ts: {\n    $0\n}",
            "Before hook for TypeScript",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "after ts",
            "after ts: {\n    $0\n}",
            "After hook for TypeScript",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "each ts",
            "each ts: {\n    $0\n}",
            "Per-iteration hook for TypeScript",
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
    
    // Top-level keywords
    items.extend(top_level_completions());
    
    // Suite body completions
    items.extend(suite_body_completions());
    
    // Setup section completions
    items.extend(setup_section_completions());
    
    // Bench body completions
    items.extend(bench_body_completions());
    
    // Fixture body completions
    items.extend(fixture_body_completions());
    
    // Global setup completions (passing empty imports since this is generic fallback)
    items.extend(global_setup_completions(&[]));
    
    // Add individual keyword completions that might be missing from context-specific functions
    items.extend(vec![
        // Core structure keywords
        completion_item(
            "suite",
            "suite ${1:name} {\n    $0\n}",
            "Top-level benchmark suite",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "bench",
            "bench ${1:name} {\n    go: $2\n    ts: $0\n}",
            "Benchmark definition",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "setup",
            "setup ${1|go,ts|} {\n    $0\n}",
            "Language-specific setup block",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "fixture",
            "fixture ${1:name} {\n    $0\n}",
            "Shared test data fixture",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "globalSetup",
            "globalSetup {\n    $0\n}",
            "Global setup block (runs once before all benchmarks)",
            CompletionItemKind::KEYWORD,
        ),
        
        // Setup section keywords
        completion_item(
            "init",
            "init {\n    $0\n}",
            "Initialization code block",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "declare",
            "declare {\n    $0\n}",
            "Package-level declarations",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "helpers",
            "helpers {\n    $0\n}",
            "Helper function definitions",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "import",
            "import {\n    $0\n}",
            "Import statements",
            CompletionItemKind::KEYWORD,
        ),
        
        // Language keywords
        completion_item(
            "go",
            "go: $0",
            "Go language implementation",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "ts",
            "ts: $0",
            "TypeScript language implementation",
            CompletionItemKind::KEYWORD,
        ),
        
        // Configuration properties
        completion_item(
            "description",
            "description: \"$0\"",
            "Description text",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "iterations",
            "iterations: ${1:1000}",
            "Number of benchmark iterations",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "warmup",
            "warmup: ${1:100}",
            "Number of warmup iterations",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "timeout",
            "timeout: ${1:30s}",
            "Benchmark timeout duration",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "tags",
            "tags: [\"$0\"]",
            "Benchmark tags for filtering",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "skip",
            "skip ${1|go,ts|}: $0",
            "Skip condition for a language",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "validate",
            "validate: $0",
            "Validation expression",
            CompletionItemKind::PROPERTY,
        ),
        
        // Lifecycle hooks
        completion_item(
            "before",
            "before ${1|go,ts|}: {\n    $0\n}",
            "Before hook (runs once before benchmark)",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "after",
            "after ${1|go,ts|}: {\n    $0\n}",
            "After hook (runs once after benchmark)",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "each",
            "each ${1|go,ts|}: {\n    $0\n}",
            "Each hook (runs per iteration)",
            CompletionItemKind::KEYWORD,
        ),
        
        // Suite configuration
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
            "compare: ${1|true,false|}",
            "Enable cross-language comparison",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "baseline",
            "baseline: \"${1|go,ts|}\"",
            "Baseline language for comparison",
            CompletionItemKind::PROPERTY,
        ),
        
        // Fixture properties
        completion_item(
            "shape",
            "shape: \"$0\"",
            "Type shape annotation for fixture",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "hex",
            "hex: \"$0\"",
            "Hex-encoded data literal",
            CompletionItemKind::PROPERTY,
        ),
        
        // Async keyword
        completion_item(
            "async",
            "async $0",
            "Async modifier (for TypeScript)",
            CompletionItemKind::KEYWORD,
        ),
        
        // Use statement
        completion_item(
            "use",
            "use std::${1|constants,anvil|}",
            "Import from standard library",
            CompletionItemKind::KEYWORD,
        ),
    ]);
    
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
