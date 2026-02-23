//! Completion provider for the LSP
//!
//! This module provides context-aware code completions
//! for poly-bench files.

use poly_bench_dsl::Lang;
use poly_bench_stdlib::{self as stdlib, StdlibSymbolKind};
use regex::Regex;
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, InsertTextFormat, Position};

use super::document::ParsedDocument;

/// Minimum number of characters required before showing completions
const MIN_PREFIX_CHARS: usize = 2;

/// Extract the current word/prefix being typed from the line text
/// Returns the partial word at the end of the line that the user is typing
fn extract_current_prefix(line_text: &str) -> String {
    let trimmed = line_text.trim_end();
    // Split on non-alphanumeric characters (except underscore) and get the last word
    trimmed.rsplit(|c: char| !c.is_alphanumeric() && c != '_').next().unwrap_or("").to_string()
}

/// Filter completions based on the prefix the user has typed
/// Only returns completions that match the prefix and only if prefix meets minimum length
fn filter_completions_by_prefix(
    items: Vec<CompletionItem>,
    prefix: &str,
    min_chars: usize,
) -> Vec<CompletionItem> {
    // If prefix is too short, return empty (no completions yet)
    if prefix.len() < min_chars {
        return vec![];
    }

    let prefix_lower = prefix.to_lowercase();
    items
        .into_iter()
        .filter(|item| {
            let label_lower = item.label.to_lowercase();
            // Match if label starts with prefix or contains it as a word
            label_lower.starts_with(&prefix_lower)
        })
        .collect()
}

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
        Context::AfterColon(ref keyword) => {
            items.extend(after_colon_completions(keyword));
            // Also add stdlib module names for expressions
            items.extend(stdlib_module_name_completions(&stdlib_imports));
        }
        Context::UseStdModule => {
            items.extend(stdlib_module_completions());
        }
        Context::InsideGlobalSetup => {
            items.extend(global_setup_completions(&stdlib_imports));
        }
        Context::ModuleDotAccess(ref module_name) => {
            // User typed "anvil." - show all symbols from that module
            items.extend(stdlib_module_member_completions(module_name, &stdlib_imports));
        }
        Context::Unknown => {
            // In unknown context, don't provide completions to avoid noise
            // The user needs to be in a recognized context for completions
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
            value: "**std::charting**\n\nType `charting.` to access chart functions:\n- `charting.drawSpeedupChart()` - Speedup chart\n- `charting.drawTable()` - Data table".to_string(),
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

    // Apply prefix filtering for non-trigger-character completions
    // This ensures completions only appear after user types 2+ characters
    let prefix = extract_current_prefix(trimmed);

    // Don't filter if:
    // 1. Triggered by a special character (., :, {, space)
    // 2. Context is AfterColon (showing enum values)
    // 3. Context is UseStdModule (showing module names after "use std::")
    // 4. Context is ModuleDotAccess or ChartingDotAccess (showing members after "module.")
    let should_skip_filtering = trigger_char.is_some() ||
        matches!(
            context,
            Context::AfterColon(_) |
                Context::UseStdModule |
                Context::ModuleDotAccess(_) |
                Context::ChartingDotAccess
        );

    if !should_skip_filtering && !prefix.is_empty() {
        items = filter_completions_by_prefix(items, &prefix, MIN_PREFIX_CHARS);
    } else if !should_skip_filtering && prefix.is_empty() {
        // No prefix typed and no trigger character - don't show completions
        items.clear();
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
    /// Inside charting function arguments (e.g., "charting.drawSpeedupChart(")
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

    // Check for charting function argument context (e.g., "charting.drawSpeedupChart(")
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
        // Check if this is a go:, ts:, or rust: line in a bench block first
        if !trimmed.starts_with("go:") &&
            !trimmed.starts_with("ts:") &&
            !trimmed.starts_with("rust:")
        {
            if let Some(keyword) = extract_keyword_before_colon(line_text) {
                // Don't return AfterColon for go/ts/rust - we'll handle those below
                if keyword != "go" && keyword != "ts" && keyword != "rust" {
                    return Context::AfterColon(keyword);
                }
            }
        }
    }

    // Check if we're on a go:, ts:, or rust: line (embedded bench code)
    if trimmed.starts_with("go:") || trimmed.starts_with("ts:") || trimmed.starts_with("rust:") {
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
/// e.g., "charting.drawSpeedupChart(" or "charting.drawSpeedupChart(title:"
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
    "setup ${1|go,ts,rust|} {\n    $0\n}",
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
completion_item(
    "setup rust",
    "setup rust {\n    import {\n        use $1;\n    }\n    init {\n        $0\n    }\n}",
    "Rust setup block",
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
        completion_item("rust:", "rust: $0", "Rust implementation", CompletionItemKind::PROPERTY),
        completion_item(
            "rust: (block)",
            "rust: {\n    $0\n}",
            "Rust implementation (multi-line)",
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
            "skip ${1|go,ts,rust|}: $0",
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
        completion_item(
            "skip rust",
            "skip rust: $0",
            "Skip condition for Rust",
            CompletionItemKind::KEYWORD,
        ),
        // Lifecycle hooks - Go
        completion_item(
            "before",
            "before ${1|go,ts,rust|}: {\n    $0\n}",
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
            "after ${1|go,ts,rust|}: {\n    $0\n}",
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
            "each ${1|go,ts,rust|}: {\n    $0\n}",
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
        // Lifecycle hooks - Rust
        completion_item(
            "before rust",
            "before rust: {\n    $0\n}",
            "Before hook for Rust",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "after rust",
            "after rust: {\n    $0\n}",
            "After hook for Rust",
            CompletionItemKind::KEYWORD,
        ),
        completion_item(
            "each rust",
            "each rust: {\n    $0\n}",
            "Per-iteration hook for Rust",
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
            "rust:",
            "rust: {\n    $0\n}",
            "Rust fixture implementation",
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
            simple_completion("\"rust\"", CompletionItemKind::ENUM_MEMBER),
        ],
        "sink" | "outlierDetection" | "memory" => vec![
            simple_completion("true", CompletionItemKind::ENUM_MEMBER),
            simple_completion("false", CompletionItemKind::ENUM_MEMBER),
        ],
        "mode" => vec![
            simple_completion("auto", CompletionItemKind::ENUM_MEMBER),
            simple_completion("fixed", CompletionItemKind::ENUM_MEMBER),
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
    detail: Some("Chart generation (drawSpeedupChart, drawTable)".to_string()),
    documentation: Some(tower_lsp::lsp_types::Documentation::MarkupContent(
tower_lsp::lsp_types::MarkupContent {
    kind: tower_lsp::lsp_types::MarkupKind::Markdown,
    value: "**std::charting**\n\nGenerate charts from benchmark results:\n- `charting.drawSpeedupChart()` - Speedup comparison\n- `charting.drawTable()` - Data table\n\nUse in suite-level `after { }` block.".to_string(),
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
    label: "drawSpeedupChart".to_string(),
    kind: Some(CompletionItemKind::FUNCTION),
    detail: Some("Draw a speedup chart for benchmark comparisons".to_string()),
    documentation: Some(tower_lsp::lsp_types::Documentation::MarkupContent(
tower_lsp::lsp_types::MarkupContent {
    kind: tower_lsp::lsp_types::MarkupKind::Markdown,
    value: "**charting.drawSpeedupChart** `(title?, description?, baseline?, output?)`\n\nGenerates a speedup chart showing relative performance against a baseline.".to_string(),
}
    )),
    insert_text: Some("drawSpeedupChart($0)".to_string()),
    insert_text_format: Some(InsertTextFormat::SNIPPET),
    ..Default::default()
},
CompletionItem {
    label: "drawTable".to_string(),
    kind: Some(CompletionItemKind::FUNCTION),
    detail: Some("Draw a table of benchmark results".to_string()),
    documentation: Some(tower_lsp::lsp_types::Documentation::MarkupContent(
tower_lsp::lsp_types::MarkupContent {
    kind: tower_lsp::lsp_types::MarkupKind::Markdown,
    value: "**charting.drawTable** `(title?, description?, output?)`\n\nGenerates a results table from benchmark data.".to_string(),
}
    )),
    insert_text: Some("drawTable($0)".to_string()),
    insert_text_format: Some(InsertTextFormat::SNIPPET),
    ..Default::default()
},
    ]
}

/// Completions for charting function parameters
fn charting_function_param_completions() -> Vec<CompletionItem> {
    vec![
        chart_param_completion("title", "string", "Chart title", "The title displayed at the top of the chart.", "title: \"$0\""),
        chart_param_completion("description", "string", "Chart description", "A description shown below the chart title.", "description: \"$0\""),
        chart_param_completion("output", "string", "Output filename", "Output filename for the generated chart SVG.", "output: \"$0\""),
        chart_param_completion("minSpeedup", "number", "Minimum speedup", "Only show benchmarks with speedup >= N.", "minSpeedup: $0"),
        chart_param_completion("filterWinner", "string", "Filter by winner", "Filter benchmarks by winner: `\"go\"`, `\"ts\"`, or `\"all\"`.", "filterWinner: \"${1|go,ts,all|}\""),
        chart_param_completion("includeBenchmarks", "array", "Include benchmarks", "Only include these benchmark names (case-insensitive substring match).", "includeBenchmarks: [\"$0\"]"),
        chart_param_completion("excludeBenchmarks", "array", "Exclude benchmarks", "Exclude these benchmark names (case-insensitive substring match).", "excludeBenchmarks: [\"$0\"]"),
        chart_param_completion("limit", "number", "Limit results", "Maximum number of benchmarks to show.", "limit: $0"),
        chart_param_completion("sortBy", "string", "Sort by", "Sort benchmarks by: `\"speedup\"`, `\"name\"`, `\"time\"`, `\"ops\"`, or `\"natural\"`.", "sortBy: \"${1|speedup,name,time,ops,natural|}\""),
        chart_param_completion("sortOrder", "string", "Sort order", "Sort order: `\"asc\"` (ascending) or `\"desc\"` (descending).", "sortOrder: \"${1|asc,desc|}\""),
        chart_param_completion("width", "number", "Chart width", "Chart width in pixels.", "width: $0"),
        chart_param_completion("height", "number", "Chart height", "Chart height in pixels.", "height: $0"),
        chart_param_completion("baselineBenchmark", "string", "Baseline benchmark", "Baseline benchmark/language used for comparisons.", "baselineBenchmark: \"$0\""),
        chart_param_completion("theme", "string", "Color theme", "Color theme: `\"dark\"` or `\"light\"`.", "theme: \"${1|dark,light|}\""),
    ]
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
        Lang::Rust => {
            // Rust function: fn name(
            if let Ok(re) = Regex::new(r"fn\s+(\w+)\s*\(") {
                for cap in re.captures_iter(code) {
                    if let Some(name) = cap.get(1) {
                        let name_str = name.as_str();
                        if seen.insert(name_str.to_string()) {
                            items.push(CompletionItem {
                                label: format!("{}()", name_str),
                                kind: Some(CompletionItemKind::FUNCTION),
                                detail: Some(format!("Rust {} function", source)),
                                insert_text: Some(format!("{}($0)", name_str)),
                                insert_text_format: Some(InsertTextFormat::SNIPPET),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
            // Rust let binding: let name or let mut name
            if let Ok(re) = Regex::new(r"let\s+(?:mut\s+)?(\w+)") {
                for cap in re.captures_iter(code) {
                    if let Some(name) = cap.get(1) {
                        let name_str = name.as_str();
                        // Skip common Rust patterns
                        if !["_", "mut"].contains(&name_str) && seen.insert(name_str.to_string()) {
                            items.push(CompletionItem {
                                label: name_str.to_string(),
                                kind: Some(CompletionItemKind::VARIABLE),
                                detail: Some(format!("Rust {} variable", source)),
                                insert_text: Some(name_str.to_string()),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
            // Rust const: const NAME
            if let Ok(re) = Regex::new(r"const\s+(\w+)") {
                for cap in re.captures_iter(code) {
                    if let Some(name) = cap.get(1) {
                        let name_str = name.as_str();
                        if seen.insert(name_str.to_string()) {
                            items.push(CompletionItem {
                                label: name_str.to_string(),
                                kind: Some(CompletionItemKind::CONSTANT),
                                detail: Some(format!("Rust {} constant", source)),
                                insert_text: Some(name_str.to_string()),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
            // Rust static: static NAME
            if let Ok(re) = Regex::new(r"static\s+(?:mut\s+)?(\w+)") {
                for cap in re.captures_iter(code) {
                    if let Some(name) = cap.get(1) {
                        let name_str = name.as_str();
                        if seen.insert(name_str.to_string()) {
                            items.push(CompletionItem {
                                label: name_str.to_string(),
                                kind: Some(CompletionItemKind::VARIABLE),
                                detail: Some(format!("Rust {} static", source)),
                                insert_text: Some(name_str.to_string()),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        }
        // Unsupported languages - no symbol extraction
        _ => {}
    }

    items
}
