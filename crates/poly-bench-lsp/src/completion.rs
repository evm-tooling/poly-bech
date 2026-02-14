//! Completion provider for the LSP
//!
//! This module provides context-aware code completions
//! for poly-bench files.

use poly_bench_dsl::Lang;
use poly_bench_stdlib::{self as stdlib, StdlibSymbolKind};
use regex::Regex;
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, InsertTextFormat, Position};

use super::document::ParsedDocument;

/// Get completions at a position
///
/// `trigger_char` is the character that triggered completion (e.g., "." or ":"),
/// provided by the LSP client when available.
pub fn get_completions(
    doc: &ParsedDocument,
    position: Position,
    trigger_char: Option<&str>,
) -> Vec<CompletionItem> {
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
            // Add setup-declared symbols (functions and variables from init/helpers)
            items.extend(extract_setup_symbols(doc));
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
        // Embedded code contexts - only show setup symbols, no DSL keywords
        Context::InsideEmbeddedInit |
        Context::InsideEmbeddedHelpers |
        Context::InsideEmbeddedDeclarations => {
            // Inside embedded Go/TypeScript code blocks
            // Only provide setup-declared symbols for reference
            items.extend(extract_setup_symbols(doc));
            // Also provide stdlib module constants that might be used (like ANVIL_RPC_URL)
            items.extend(stdlib_module_name_completions(&stdlib_imports));
        }
        Context::InsideEmbeddedBenchCode => {
            // Inside go: or ts: code in a bench block
            // Only provide setup-declared symbols (functions/variables from init/helpers)
            items.extend(extract_setup_symbols(doc));
            // Also provide stdlib module constants
            items.extend(stdlib_module_name_completions(&stdlib_imports));
        }
        Context::InsideAfterBlock => {
            // Inside suite-level after { } block - only charting is allowed
            // Suggest "charting" module if imported
            if stdlib_imports.contains(&"charting".to_string()) {
                items.push(CompletionItem {
    label: "charting".to_string(),
    kind: Some(CompletionItemKind::MODULE),
    detail: Some("Chart generation module".to_string()),
    documentation: Some(tower_lsp::lsp_types::Documentation::MarkupContent(
        tower_lsp::lsp_types::MarkupContent {
            kind: tower_lsp::lsp_types::MarkupKind::Markdown,
            value: "**std::charting**\n\nType `charting.` to access chart functions:\n- `charting.drawBarChart()` - Bar chart\n- `charting.drawPieChart()` - Pie chart\n- `charting.drawLineChart()` - Line chart".to_string(),
        }
    )),
    insert_text: Some("charting".to_string()),
    ..Default::default()
});
            }
        }
        Context::ChartingDotAccess => {
            // After "charting." - show chart function completions
            items.extend(charting_function_completions());
        }
        Context::InsideChartingFunctionArgs => {
            // Inside charting function arguments - show parameter completions
            items.extend(charting_function_param_completions());
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
                            symbols
                                .iter()
                                .map(|s| format!("- `{}.{}` - {}", module, s.name, s.description))
                                .collect::<Vec<_>>()
                                .join("\n")
                        ),
                    },
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
                },
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
    /// Inside init{} block - embedded code, no DSL completions
    InsideEmbeddedInit,
    /// Inside helpers{} block - embedded code, no DSL completions
    InsideEmbeddedHelpers,
    /// Inside go: or ts: code in bench block - embedded code
    InsideEmbeddedBenchCode,
    /// Inside import{} or declare{} block - embedded code
    InsideEmbeddedDeclarations,
    /// Inside suite-level after { } block for charting directives
    InsideAfterBlock,
    /// After typing "charting." - suggests chart functions
    ChartingDotAccess,
    /// Inside charting function arguments (e.g., "charting.drawBarChart(")
    InsideChartingFunctionArgs,
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

    // Check for charting function argument context (e.g., "charting.drawBarChart(")
    if is_inside_chart_function_args(trimmed) {
        return Context::InsideChartingFunctionArgs;
    }

    // Check for charting dot access specifically (takes precedence over generic module dot access)
    if let Some(module_name) = extract_module_before_dot(trimmed) {
        if module_name == "charting" {
            // Verify charting is imported
            let stdlib_imports = detect_stdlib_imports(doc);
            if stdlib_imports.contains(&"charting".to_string()) {
                return Context::ChartingDotAccess;
            }
        }
        // Check for other module dot access
        let stdlib_imports = detect_stdlib_imports(doc);
        if stdlib_imports.contains(&module_name) {
            return Context::ModuleDotAccess(module_name);
        }
    }

    // Check if we're after a colon (but not for go: or ts: in bench blocks)
    if line_text.ends_with(':') || line_text.contains(": ") {
        // Check if this is a go: or ts: line in a bench block first
        if !trimmed.starts_with("go:") && !trimmed.starts_with("ts:") {
            if let Some(keyword) = extract_keyword_before_colon(line_text) {
                // Don't return AfterColon for go/ts - we'll handle those below
                if keyword != "go" && keyword != "ts" {
                    return Context::AfterColon(keyword);
                }
            }
        }
    }

    // Check if we're on a go: or ts: line (embedded bench code)
    if trimmed.starts_with("go:") || trimmed.starts_with("ts:") {
        return Context::InsideEmbeddedBenchCode;
    }

    // Simple heuristic: count braces and track block hierarchy
    let offset = match doc.position_to_offset(position) {
        Some(o) => o,
        None => return Context::Unknown,
    };

    let text_before = &doc.source[..offset];

    // Track block hierarchy: (keyword, depth when entered)
    let mut block_stack: Vec<(String, i32)> = Vec::new();
    let mut depth = 0;
    let mut current_word = String::new();
    let mut last_word = String::new(); // Keep track of the last complete word

    // Simple scanner for context detection
    let mut chars = text_before.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '{' => {
                depth += 1;
                // Use current_word if not empty, otherwise use last_word
                let word_to_push =
                    if !current_word.is_empty() { current_word.clone() } else { last_word.clone() };
                if !word_to_push.is_empty() {
                    block_stack.push((word_to_push, depth));
                }
                current_word.clear();
                last_word.clear();
            }
            '}' => {
                // Pop blocks that were opened at this depth
                while let Some((_, block_depth)) = block_stack.last() {
                    if *block_depth == depth {
                        block_stack.pop();
                    } else {
                        break;
                    }
                }
                depth -= 1;
                current_word.clear();
                last_word.clear();
            }
            c if c.is_alphanumeric() || c == '_' => {
                current_word.push(c);
            }
            _ => {
                // Save the word before clearing
                if !current_word.is_empty() {
                    last_word = current_word.clone();
                }
                current_word.clear();
            }
        }
    }

    // Analyze the block stack to determine context
    // Check if we're inside an embedded code block (init, helpers, import, declare)
    // or inside a suite-level after block (for charting)
    for (keyword, block_depth) in block_stack.iter().rev() {
        match keyword.as_str() {
            "init" => return Context::InsideEmbeddedInit,
            "helpers" => return Context::InsideEmbeddedHelpers,
            "import" | "declare" => return Context::InsideEmbeddedDeclarations,
            // Suite-level "after" block (not "after go:" or "after ts:" which are hooks)
            // This is detected by being at depth 2 inside a suite
            "after" => {
                // Check if this is a suite-level after block (depth 2, inside suite)
                // by checking if the block before it is "suite"
                let in_suite = block_stack.iter().any(|(kw, _)| kw == "suite");
                // Also check that this after block is at the right depth (directly inside suite)
                // For suite-level after, the after block is at depth 2 (suite is at depth 1)
                if in_suite && *block_depth == 2 {
                    return Context::InsideAfterBlock;
                }
            }
            _ => {}
        }
    }

    // Get the most recent structural keyword (suite, setup, bench, fixture, globalSetup, after)
    let last_structural = block_stack
        .iter()
        .rev()
        .find(|(kw, _)| {
            matches!(
                kw.as_str(),
                "suite" | "setup" | "bench" | "fixture" | "globalSetup" | "go" | "ts" | "after"
            )
        })
        .map(|(kw, _)| kw.as_str());

    match depth {
        0 => Context::TopLevel,
        1 => {
            // Inside a top-level block (suite or globalSetup)
            match last_structural {
                Some("suite") => Context::InsideSuite,
                Some("globalSetup") => Context::InsideGlobalSetup,
                _ => Context::InsideSuite,
            }
        }
        2 => {
            // Inside a nested block (setup, bench, fixture, after)
            match last_structural {
                Some("setup") | Some("go") | Some("ts") => Context::InsideSetup,
                Some("bench") => Context::InsideBench,
                Some("fixture") => Context::InsideFixture,
                Some("globalSetup") => Context::InsideGlobalSetup,
                Some("after") => {
                    // Suite-level after block - only charting allowed
                    Context::InsideAfterBlock
                }
                _ => Context::InsideSuite,
            }
        }
        _ => {
            // Deeper nesting - likely inside embedded code
            // Check what structural block we're in
            match last_structural {
                Some("setup") | Some("go") | Some("ts") => {
                    // We're deep inside a setup block - probably in init/helpers
                    // But we already checked for those above, so this is unknown
                    Context::InsideSetup
                }
                Some("bench") => Context::InsideBench,
                Some("after") => {
                    // Still inside an after block (maybe with charting function args)
                    Context::InsideAfterBlock
                }
                _ => Context::Unknown,
            }
        }
    }
}

/// Extract the module name when a trigger character was typed
/// For example, if line_text is "    anvil" and trigger is ".", return Some("anvil")
fn extract_module_name_before_trigger(line_text: &str) -> Option<String> {
    let trimmed = line_text.trim();

    // Known stdlib module names
    let known_modules = ["anvil", "charting", "constants"];

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
            if word == module ||
                word.ends_with(&format!("{}.", module)) ||
                word == format!("{}.", module)
            {
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
        return false; // Don't trigger yet, wait for colons
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
    let known_modules = ["anvil", "charting", "constants"];

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

    // Use safe access with .first() instead of parts[0]
    let before_colon = parts.first()?.trim();
    let words: Vec<&str> = before_colon.split_whitespace().collect();

    words.last().map(|s| s.to_string())
}

/// Check if cursor is inside charting function arguments
/// e.g., "charting.drawBarChart(" or "charting.drawBarChart(title:"
fn is_inside_chart_function_args(text: &str) -> bool {
    // Check for unclosed charting.drawX( pattern
    let draw_pattern = Regex::new(r"charting\.draw\w+\([^)]*$").ok();
    if let Some(pattern) = draw_pattern {
        return pattern.is_match(text);
    }
    false
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

// Auto-calibration settings
completion_item(
    "mode",
    "mode: ${1|auto,fixed|}",
    "Execution mode: auto (time-based) or fixed (iteration count)",
    CompletionItemKind::PROPERTY,
),
completion_item(
    "targetTime",
    "targetTime: ${1:3000ms}",
    "Target duration for auto-calibration mode",
    CompletionItemKind::PROPERTY,
),
completion_item(
    "minIterations",
    "minIterations: ${1:100}",
    "Minimum iterations for auto-calibration",
    CompletionItemKind::PROPERTY,
),
completion_item(
    "maxIterations",
    "maxIterations: ${1:1000000}",
    "Maximum iterations for auto-calibration",
    CompletionItemKind::PROPERTY,
),

// Performance settings
completion_item(
    "sink",
    "sink: ${1|true,false|}",
    "Use sink/black-box pattern to prevent dead code elimination",
    CompletionItemKind::PROPERTY,
),

// Statistical settings
completion_item(
    "outlierDetection",
    "outlierDetection: ${1|true,false|}",
    "Enable IQR-based outlier detection and removal",
    CompletionItemKind::PROPERTY,
),
completion_item(
    "cvThreshold",
    "cvThreshold: ${1:5}",
    "Coefficient of variation threshold (%) for stability warnings",
    CompletionItemKind::PROPERTY,
),

// Statistical consistency settings
completion_item(
    "count",
    "count: ${1:10}",
    "Number of times to run each benchmark for statistical consistency (results use median)",
    CompletionItemKind::PROPERTY,
),

// Observability settings (Phase 2B)
completion_item(
    "memory",
    "memory: ${1|true,false|}",
    "Enable memory allocation profiling",
    CompletionItemKind::PROPERTY,
),
completion_item(
    "concurrency",
    "concurrency: ${1:1}",
    "Number of concurrent goroutines/workers for parallel execution",
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
        completion_item("go:", "go: $0", "Go implementation", CompletionItemKind::PROPERTY),
        completion_item(
            "go: (block)",
            "go: {\n    $0\n}",
            "Go implementation (multi-line)",
            CompletionItemKind::PROPERTY,
        ),
        completion_item("ts:", "ts: $0", "TypeScript implementation", CompletionItemKind::PROPERTY),
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
        // Auto-calibration overrides
        completion_item(
            "mode",
            "mode: ${1|auto,fixed|}",
            "Override execution mode for this benchmark",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "targetTime",
            "targetTime: ${1:3000ms}",
            "Override target duration for auto-calibration",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "minIterations",
            "minIterations: ${1:100}",
            "Override minimum iterations for auto-calibration",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "maxIterations",
            "maxIterations: ${1:1000000}",
            "Override maximum iterations for auto-calibration",
            CompletionItemKind::PROPERTY,
        ),
        // Performance overrides
        completion_item(
            "sink",
            "sink: ${1|true,false|}",
            "Override sink/black-box pattern for this benchmark",
            CompletionItemKind::PROPERTY,
        ),
        // Statistical overrides
        completion_item(
            "outlierDetection",
            "outlierDetection: ${1|true,false|}",
            "Override outlier detection for this benchmark",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "cvThreshold",
            "cvThreshold: ${1:5}",
            "Override CV threshold for this benchmark",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "count",
            "count: ${1:10}",
            "Override: number of times to run this benchmark for statistical consistency",
            CompletionItemKind::PROPERTY,
        ),
        // Observability overrides (Phase 2B)
        completion_item(
            "memory",
            "memory: ${1|true,false|}",
            "Enable memory profiling for this benchmark",
            CompletionItemKind::PROPERTY,
        ),
        completion_item(
            "concurrency",
            "concurrency: ${1:4}",
            "Number of concurrent goroutines/workers",
            CompletionItemKind::PROPERTY,
        ),
    ]
}

fn fixture_body_completions() -> Vec<CompletionItem> {
    vec![
        completion_item("hex", "hex: \"$0\"", "Hex-encoded data", CompletionItemKind::PROPERTY),
        completion_item(
            "hex: @file",
            "hex: @file(\"${1:path/to/file.hex}\")",
            "Load hex data from external file",
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
        "compare" | "sink" | "outlierDetection" | "memory" => vec![
            simple_completion("true", CompletionItemKind::ENUM_MEMBER),
            simple_completion("false", CompletionItemKind::ENUM_MEMBER),
        ],
        "mode" => vec![
            simple_completion("auto", CompletionItemKind::ENUM_MEMBER),
            simple_completion("fixed", CompletionItemKind::ENUM_MEMBER),
        ],
        "timeUnit" => vec![
            simple_completion("\"auto\"", CompletionItemKind::ENUM_MEMBER),
            simple_completion("\"ns\"", CompletionItemKind::ENUM_MEMBER),
            simple_completion("\"us\"", CompletionItemKind::ENUM_MEMBER),
            simple_completion("\"ms\"", CompletionItemKind::ENUM_MEMBER),
            simple_completion("\"s\"", CompletionItemKind::ENUM_MEMBER),
        ],
        "sortBy" => vec![
            simple_completion("\"speedup\"", CompletionItemKind::ENUM_MEMBER),
            simple_completion("\"name\"", CompletionItemKind::ENUM_MEMBER),
            simple_completion("\"time\"", CompletionItemKind::ENUM_MEMBER),
            simple_completion("\"ops\"", CompletionItemKind::ENUM_MEMBER),
        ],
        "sortOrder" => vec![
            simple_completion("\"asc\"", CompletionItemKind::ENUM_MEMBER),
            simple_completion("\"desc\"", CompletionItemKind::ENUM_MEMBER),
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
        completion_item("go", "go: $0", "Go language implementation", CompletionItemKind::KEYWORD),
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
    label: "charting".to_string(),
    kind: Some(CompletionItemKind::MODULE),
    detail: Some("Chart generation (drawBarChart, drawPieChart, drawLineChart)".to_string()),
    documentation: Some(tower_lsp::lsp_types::Documentation::MarkupContent(
tower_lsp::lsp_types::MarkupContent {
    kind: tower_lsp::lsp_types::MarkupKind::Markdown,
    value: "**std::charting**\n\nGenerate charts from benchmark results:\n- `charting.drawBarChart()` - Bar chart comparison\n- `charting.drawPieChart()` - Pie chart distribution\n- `charting.drawLineChart()` - Line chart trends\n\nUse in suite-level `after { }` block.".to_string(),
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

/// Completions for charting functions after "charting."
fn charting_function_completions() -> Vec<CompletionItem> {
    vec![
CompletionItem {
    label: "drawBarChart".to_string(),
    kind: Some(CompletionItemKind::FUNCTION),
    detail: Some("Draw a bar chart of benchmark results".to_string()),
    documentation: Some(tower_lsp::lsp_types::Documentation::MarkupContent(
tower_lsp::lsp_types::MarkupContent {
    kind: tower_lsp::lsp_types::MarkupKind::Markdown,
    value: "**charting.drawBarChart** `(title?, description?, xlabel?, ylabel?, output?)`\n\nGenerates a bar chart comparing benchmark execution times.\n\n**Parameters:**\n- `title` - Chart title\n- `description` - Chart description\n- `xlabel` - X-axis label\n- `ylabel` - Y-axis label\n- `output` - Output filename (default: bar-chart.svg)".to_string(),
}
    )),
    insert_text: Some("drawBarChart($0)".to_string()),
    insert_text_format: Some(InsertTextFormat::SNIPPET),
    ..Default::default()
},
CompletionItem {
    label: "drawPieChart".to_string(),
    kind: Some(CompletionItemKind::FUNCTION),
    detail: Some("Draw a pie chart of time distribution".to_string()),
    documentation: Some(tower_lsp::lsp_types::Documentation::MarkupContent(
tower_lsp::lsp_types::MarkupContent {
    kind: tower_lsp::lsp_types::MarkupKind::Markdown,
    value: "**charting.drawPieChart** `(title?, description?, output?)`\n\nGenerates a pie chart showing time distribution across benchmarks.\n\n**Parameters:**\n- `title` - Chart title\n- `description` - Chart description\n- `output` - Output filename (default: pie-chart.svg)".to_string(),
}
    )),
    insert_text: Some("drawPieChart($0)".to_string()),
    insert_text_format: Some(InsertTextFormat::SNIPPET),
    ..Default::default()
},
CompletionItem {
    label: "drawLineChart".to_string(),
    kind: Some(CompletionItemKind::FUNCTION),
    detail: Some("Draw a line chart for trend visualization".to_string()),
    documentation: Some(tower_lsp::lsp_types::Documentation::MarkupContent(
tower_lsp::lsp_types::MarkupContent {
    kind: tower_lsp::lsp_types::MarkupKind::Markdown,
    value: "**charting.drawLineChart** `(title?, description?, xlabel?, ylabel?, output?)`\n\nGenerates a line chart for visualizing benchmark trends.\n\n**Parameters:**\n- `title` - Chart title\n- `description` - Chart description\n- `xlabel` - X-axis label\n- `ylabel` - Y-axis label\n- `output` - Output filename (default: line-chart.svg)".to_string(),
}
    )),
    insert_text: Some("drawLineChart($0)".to_string()),
    insert_text_format: Some(InsertTextFormat::SNIPPET),
    ..Default::default()
},
    ]
}

/// Completions for charting function parameters
fn charting_function_param_completions() -> Vec<CompletionItem> {
    let mut items = vec![
// String parameters
chart_param_completion("title", "string", "Chart title", "The title displayed at the top of the chart.", "title: \"$0\""),
chart_param_completion("description", "string", "Chart description", "A description shown below the chart title.", "description: \"$0\""),
chart_param_completion("xlabel", "string", "X-axis label", "Label for the X-axis.", "xlabel: \"$0\""),
chart_param_completion("ylabel", "string", "Y-axis label", "Label for the Y-axis.", "ylabel: \"$0\""),
chart_param_completion("output", "string", "Output filename", "The output filename for the generated chart SVG.\n\nDefault: `bar-chart.svg`, `pie-chart.svg`, or `line-chart.svg` depending on chart type.", "output: \"$0\""),

// Display toggle parameters (boolean)
chart_param_completion("showStats", "bool", "Show statistics", "Show ops/sec and time per op for each benchmark.\n\nDefault: `true`", "showStats: ${1|true,false|}"),
chart_param_completion("showConfig", "bool", "Show config", "Show benchmark configuration (iterations, warmup, timeout) in chart footer.\n\nDefault: `true`", "showConfig: ${1|true,false|}"),
chart_param_completion("showWinCounts", "bool", "Show win counts", "Show win counts in legend (e.g., 'Go faster (5 wins)').\n\nDefault: `true`", "showWinCounts: ${1|true,false|}"),
chart_param_completion("showGeoMean", "bool", "Show geometric mean", "Show geometric mean speedup in legend.\n\nDefault: `true`", "showGeoMean: ${1|true,false|}"),
chart_param_completion("showDistribution", "bool", "Show distribution", "Show min/max/p50/p99 percentile distribution.\n\nDefault: `false`", "showDistribution: ${1|true,false|}"),
chart_param_completion("showMemory", "bool", "Show memory stats", "Show bytes/allocs memory statistics (if available).\n\nDefault: `false`", "showMemory: ${1|true,false|}"),
chart_param_completion("showTotalTime", "bool", "Show total time", "Show total execution time.\n\nDefault: `false`", "showTotalTime: ${1|true,false|}"),
chart_param_completion("compact", "bool", "Compact mode", "Minimal chart mode without extra statistics.\n\nDefault: `false`", "compact: ${1|true,false|}"),

// Filtering parameters
chart_param_completion("minSpeedup", "number", "Minimum speedup", "Only show benchmarks with speedup >= N.\n\nExample: `minSpeedup: 2.0` shows only benchmarks where one language is at least 2x faster.", "minSpeedup: $0"),
chart_param_completion("filterWinner", "string", "Filter by winner", "Filter benchmarks by winner: `\"go\"`, `\"ts\"`, or `\"all\"`.", "filterWinner: \"${1|go,ts,all|}\""),
chart_param_completion("includeBenchmarks", "array", "Include benchmarks", "Only include these benchmark names (case-insensitive substring match).\n\nExample: `includeBenchmarks: [\"hash\", \"sort\"]`", "includeBenchmarks: [\"$0\"]"),
chart_param_completion("excludeBenchmarks", "array", "Exclude benchmarks", "Exclude these benchmark names (case-insensitive substring match).\n\nExample: `excludeBenchmarks: [\"slow\", \"legacy\"]`", "excludeBenchmarks: [\"$0\"]"),
chart_param_completion("limit", "number", "Limit results", "Maximum number of benchmarks to show.\n\nExample: `limit: 10` shows only top 10 benchmarks.", "limit: $0"),

// Sorting parameters
chart_param_completion("sortBy", "string", "Sort by", "Sort benchmarks by: `\"speedup\"`, `\"name\"`, `\"time\"`, or `\"ops\"`.\n\nDefault: `\"name\"`", "sortBy: \"${1|speedup,name,time,ops|}\""),
chart_param_completion("sortOrder", "string", "Sort order", "Sort order: `\"asc\"` (ascending) or `\"desc\"` (descending).\n\nDefault: `\"asc\"`", "sortOrder: \"${1|asc,desc|}\""),

// Layout parameters
chart_param_completion("width", "number", "Chart width", "Chart width in pixels.\n\nDefault: `880`", "width: $0"),
chart_param_completion("barHeight", "number", "Bar height", "Height of each bar in pixels.\n\nDefault: `26`", "barHeight: $0"),
chart_param_completion("barGap", "number", "Bar gap", "Gap between bars in pixels.\n\nDefault: `5`", "barGap: $0"),
chart_param_completion("marginLeft", "number", "Left margin", "Left margin for benchmark labels in pixels.\n\nDefault: `200`", "marginLeft: $0"),

// Data display parameters
chart_param_completion("precision", "number", "Decimal precision", "Number of decimal places for numbers.\n\nDefault: `2`", "precision: $0"),
chart_param_completion("timeUnit", "string", "Time unit", "Time unit for display: `\"auto\"`, `\"ns\"`, `\"us\"`, `\"ms\"`, or `\"s\"`.\n\nDefault: `\"auto\"` (chooses appropriate unit)", "timeUnit: \"${1|auto,ns,us,ms,s|}\""),
    ];

    items
}

/// Helper to create a chart parameter completion item
fn chart_param_completion(
    name: &str,
    param_type: &str,
    detail: &str,
    doc: &str,
    insert_text: &str,
) -> CompletionItem {
    CompletionItem {
        label: name.to_string(),
        kind: Some(CompletionItemKind::PROPERTY),
        detail: Some(format!("{} ({})", detail, param_type)),
        documentation: Some(tower_lsp::lsp_types::Documentation::MarkupContent(
            tower_lsp::lsp_types::MarkupContent {
                kind: tower_lsp::lsp_types::MarkupKind::Markdown,
                value: format!("**{}** `{}`\n\n{}", name, param_type, doc),
            },
        )),
        insert_text: Some(insert_text.to_string()),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        ..Default::default()
    }
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
    CompletionItem { label: label.to_string(), kind: Some(kind), ..Default::default() }
}

/// Extract symbols (functions and variables) declared in setup init/helpers blocks
fn extract_setup_symbols(doc: &ParsedDocument) -> Vec<CompletionItem> {
    let mut items = Vec::new();
    let Some(ref ast) = doc.ast else {
        return items;
    };

    for suite in &ast.suites {
        for (lang, setup) in &suite.setups {
            // Extract from helpers block
            if let Some(ref helpers) = setup.helpers {
                items.extend(extract_symbols_from_code(&helpers.code, *lang, "helper"));
            }
            // Extract from init block
            if let Some(ref init) = setup.init {
                items.extend(extract_symbols_from_code(&init.code, *lang, "init"));
            }
            // Extract from declarations block
            if let Some(ref decls) = setup.declarations {
                items.extend(extract_symbols_from_code(&decls.code, *lang, "declaration"));
            }
        }
    }

    items
}

/// Extract function and variable names from code using regex patterns
fn extract_symbols_from_code(code: &str, lang: Lang, source: &str) -> Vec<CompletionItem> {
    let mut items = Vec::new();
    let mut seen = std::collections::HashSet::new();

    match lang {
        Lang::Go => {
            // Go function: func name(
            if let Ok(re) = Regex::new(r"func\s+(\w+)\s*\(") {
                for cap in re.captures_iter(code) {
                    if let Some(name) = cap.get(1) {
                        let name_str = name.as_str();
                        if seen.insert(name_str.to_string()) {
                            items.push(CompletionItem {
                                label: format!("{}()", name_str),
                                kind: Some(CompletionItemKind::FUNCTION),
                                detail: Some(format!("Go {} function", source)),
                                insert_text: Some(format!("{}($0)", name_str)),
                                insert_text_format: Some(InsertTextFormat::SNIPPET),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
            // Go variable: var name or name :=
            if let Ok(re) = Regex::new(r"(?:var\s+(\w+)|(\w+)\s*:=)") {
                for cap in re.captures_iter(code) {
                    let name = cap.get(1).or_else(|| cap.get(2));
                    if let Some(name) = name {
                        let name_str = name.as_str();
                        // Skip common Go keywords/patterns
                        if !["err", "ok", "_", "nil"].contains(&name_str) &&
                            seen.insert(name_str.to_string())
                        {
                            items.push(CompletionItem {
                                label: name_str.to_string(),
                                kind: Some(CompletionItemKind::VARIABLE),
                                detail: Some(format!("Go {} variable", source)),
                                insert_text: Some(name_str.to_string()),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        }
        Lang::TypeScript => {
            // TypeScript function: function name( or async function name(
            if let Ok(re) = Regex::new(r"(?:async\s+)?function\s+(\w+)\s*\(") {
                for cap in re.captures_iter(code) {
                    if let Some(name) = cap.get(1) {
                        let name_str = name.as_str();
                        if seen.insert(name_str.to_string()) {
                            items.push(CompletionItem {
                                label: format!("{}()", name_str),
                                kind: Some(CompletionItemKind::FUNCTION),
                                detail: Some(format!("TypeScript {} function", source)),
                                insert_text: Some(format!("{}($0)", name_str)),
                                insert_text_format: Some(InsertTextFormat::SNIPPET),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
            // TypeScript const/let: const name or let name
            if let Ok(re) = Regex::new(r"(?:const|let)\s+(\w+)") {
                for cap in re.captures_iter(code) {
                    if let Some(name) = cap.get(1) {
                        let name_str = name.as_str();
                        if seen.insert(name_str.to_string()) {
                            items.push(CompletionItem {
                                label: name_str.to_string(),
                                kind: Some(CompletionItemKind::VARIABLE),
                                detail: Some(format!("TypeScript {} variable", source)),
                                insert_text: Some(name_str.to_string()),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
            // Arrow functions: const name = ( or const name = async (
            if let Ok(re) = Regex::new(r"const\s+(\w+)\s*=\s*(?:async\s*)?\(") {
                for cap in re.captures_iter(code) {
                    if let Some(name) = cap.get(1) {
                        let name_str = name.as_str();
                        if seen.insert(format!("{}()", name_str)) {
                            items.push(CompletionItem {
                                label: format!("{}()", name_str),
                                kind: Some(CompletionItemKind::FUNCTION),
                                detail: Some(format!("TypeScript {} arrow function", source)),
                                insert_text: Some(format!("{}($0)", name_str)),
                                insert_text_format: Some(InsertTextFormat::SNIPPET),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        }
        _ => {}
    }

    items
}
