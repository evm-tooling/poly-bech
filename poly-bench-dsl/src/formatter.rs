//! AST-based formatter for the poly-bench DSL
//!
//! Source-preserving formatter that maintains embedded code blocks exactly as written.
//! Only DSL structure (keywords, properties) is reformatted with consistent indentation.
//!
//! The formatter preserves:
//! - `use std::module` import statements
//! - File-level `globalSetup` blocks
//! - Suite-level `globalSetup` blocks
//! - Comments (when using `format_file_with_source`)
//! - Original embedded code formatting (Go, TypeScript, etc.)

use crate::{
    ast::{HookStyle, *},
    ChartType,
};
use std::fmt::Write;

const INDENT: &str = "    ";

/// Reformat embedded code with proper indentation based on brace counting.
/// This handles Go, TypeScript, and Rust code by tracking `{` and `}` to determine indent level.
fn reformat_embedded_code(code: &str, base_indent: &str) -> Vec<String> {
    let lines: Vec<&str> = code.lines().collect();
    if lines.is_empty() {
        return Vec::new();
    }

    // First, strip all lines to their trimmed content
    let trimmed_lines: Vec<&str> = lines.iter().map(|l| l.trim()).collect();

    // Track brace depth and format each line
    let mut result = Vec::new();
    let mut depth: i32 = 0;

    for line in &trimmed_lines {
        if line.is_empty() {
            result.push(String::new());
            continue;
        }

        // Count braces to determine indent adjustment
        // Lines starting with } should be dedented before we print them
        let starts_with_close = line.starts_with('}') || line.starts_with(')');

        // Calculate indent for this line
        let line_depth = if starts_with_close { (depth - 1).max(0) } else { depth };
        let indent = format!("{}{}", base_indent, INDENT.repeat(line_depth as usize));

        result.push(format!("{}{}", indent, line));

        // Update depth based on braces in this line (excluding those in strings)
        let net_braces = count_net_braces(line);
        depth = (depth + net_braces).max(0);
    }

    result
}

/// Count net brace change in a line, attempting to skip braces inside string literals.
fn count_net_braces(line: &str) -> i32 {
    let mut net = 0i32;
    let mut in_string = false;
    let mut string_char = '"';
    let mut prev_char = '\0';

    for ch in line.chars() {
        if in_string {
            if ch == string_char && prev_char != '\\' {
                in_string = false;
            }
        } else {
            match ch {
                '"' | '\'' | '`' => {
                    in_string = true;
                    string_char = ch;
                }
                '{' => net += 1,
                '}' => net -= 1,
                _ => {}
            }
        }
        prev_char = ch;
    }

    net
}

/// Preserve embedded code with minimal normalization.
/// Only strips common leading indent to align to the block's indent level.
/// Does NOT attempt to reformat or re-indent based on brace counting.
#[allow(dead_code)]
fn preserve_embedded_code(code: &str, base_indent: &str) -> Vec<String> {
    let lines: Vec<&str> = code.lines().collect();
    if lines.is_empty() {
        return Vec::new();
    }

    // Find minimum indent among non-empty lines
    let min_indent = lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min()
        .unwrap_or(0);

    // Strip the minimum indent and apply base_indent
    let mut result = Vec::new();
    for line in &lines {
        if line.trim().is_empty() {
            result.push(String::new());
        } else {
            let stripped =
                if line.len() >= min_indent { &line[min_indent..] } else { line.trim_start() };
            result.push(format!("{}{}", base_indent, stripped));
        }
    }

    result
}

/// Format an AST into a string with consistent indentation and style.
pub fn format_file(file: &File) -> String {
    let mut out = String::new();

    // Format use statements first
    for use_std in &file.use_stds {
        writeln!(out, "use std::{}", use_std.module).unwrap();
    }

    // Add blank line after use statements if there are any
    if !file.use_stds.is_empty() {
        out.push('\n');
    }

    // Format file-level globalSetup if present (and not inherited by suites)
    // Only output file-level globalSetup if it exists and is different from suite-level
    if let Some(ref global_setup) = file.global_setup {
        // Check if any suite has its own globalSetup - if so, don't output file-level
        let any_suite_has_own = file.suites.iter().any(|s| {
            if let Some(ref suite_gs) = s.global_setup {
                // Suite has its own if it's different from file-level
                suite_gs.span.start != global_setup.span.start
            } else {
                false
            }
        });

        // Only output file-level if no suite overrides it
        if !any_suite_has_own {
            format_global_setup(&mut out, global_setup, 0);
            out.push('\n');
        }
    }

    // Format suites
    for (i, suite) in file.suites.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        format_suite(&mut out, suite, 0);
    }
    if !out.is_empty() && !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

/// Format an AST into a string, preserving comments from the original source.
///
/// This function extracts comments from the original source and places them
/// appropriately in the formatted output.
pub fn format_file_with_source(file: &File, original_source: &str) -> String {
    let mut out = String::new();

    // Extract leading comments (before first use statement or suite)
    let leading_comments = extract_leading_comments(original_source);
    if !leading_comments.is_empty() {
        out.push_str(&leading_comments);
        if !leading_comments.ends_with('\n') {
            out.push('\n');
        }
    }

    // Format use statements first
    for use_std in &file.use_stds {
        writeln!(out, "use std::{}", use_std.module).unwrap();
    }

    // Add blank line after use statements if there are any
    if !file.use_stds.is_empty() {
        out.push('\n');
    }

    // Format file-level globalSetup if present
    if let Some(ref global_setup) = file.global_setup {
        // Check if any suite has its own globalSetup
        let any_suite_has_own = file.suites.iter().any(|s| {
            if let Some(ref suite_gs) = s.global_setup {
                suite_gs.span.start != global_setup.span.start
            } else {
                false
            }
        });

        if !any_suite_has_own {
            format_global_setup(&mut out, global_setup, 0);
            out.push('\n');
        }
    }

    // Format suites with comment preservation
    for (i, suite) in file.suites.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        format_suite_with_source(&mut out, suite, 0, original_source);
    }

    if !out.is_empty() && !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

/// Format a suite, preserving comments from the original source
fn format_suite_with_source(
    out: &mut String,
    suite: &Suite,
    indent_level: usize,
    original_source: &str,
) {
    let pad = INDENT.repeat(indent_level);
    let inner = INDENT.repeat(indent_level + 1);

    write!(out, "{}suite {} {{\n", pad, suite.name).unwrap();

    // Suite properties in canonical order
    if let Some(ref desc) = suite.description {
        write!(out, "{}description: \"{}\"\n", inner, escape_string(desc)).unwrap();
    }
    if let Some(n) = suite.iterations {
        write!(out, "{}iterations: {}\n", inner, n).unwrap();
    }
    if let Some(n) = suite.warmup {
        write!(out, "{}warmup: {}\n", inner, n).unwrap();
    }
    if let Some(n) = suite.timeout {
        write!(out, "{}timeout: {}\n", inner, n).unwrap();
    }
    if !suite.requires.is_empty() {
        let langs: Vec<_> = suite.requires.iter().map(|l| l.as_str()).collect();
        write!(out, "{}requires: [{}]\n", inner, langs.join(", ")).unwrap();
    }
    if let Some(order) = suite.order {
        let s = match order {
            ExecutionOrder::Sequential => "sequential",
            ExecutionOrder::Parallel => "parallel",
            ExecutionOrder::Random => "random",
        };
        write!(out, "{}order: {}\n", inner, s).unwrap();
    }
    if let Some(baseline) = suite.baseline {
        write!(out, "{}baseline: \"{}\"\n", inner, baseline.as_str()).unwrap();
    }

    // Benchmark accuracy settings - only output non-default values
    if let Some(mode) = suite.mode {
        write!(out, "{}mode: \"{}\"\n", inner, mode.as_str()).unwrap();
    }
    if let Some(target_time) = suite.target_time_ms {
        write!(out, "{}targetTime: {}ms\n", inner, target_time).unwrap();
    }
    if !suite.sink {
        write!(out, "{}sink: false\n", inner).unwrap();
    }
    if !suite.outlier_detection {
        write!(out, "{}outlierDetection: false\n", inner).unwrap();
    }
    if let Some(cv_threshold) = suite.cv_threshold {
        write!(out, "{}cvThreshold: {}\n", inner, cv_threshold).unwrap();
    }
    if let Some(count) = suite.count {
        write!(out, "{}count: {}\n", inner, count).unwrap();
    }
    if suite.memory {
        write!(out, "{}memory: true\n", inner).unwrap();
    }

    // Add blank line after properties
    let has_content = suite.global_setup.is_some() ||
        !suite.setups.is_empty() ||
        !suite.fixtures.is_empty() ||
        !suite.benchmarks.is_empty();
    if has_content {
        out.push('\n');
    }

    // Suite-level globalSetup
    if let Some(ref global_setup) = suite.global_setup {
        format_global_setup(out, global_setup, indent_level + 1);
        out.push('\n');
    }

    // Setups in canonical order
    let lang_order = [Lang::Go, Lang::TypeScript, Lang::Rust, Lang::Python];
    for lang in &lang_order {
        if let Some(setup) = suite.setups.get(lang) {
            format_setup(out, lang, setup, indent_level + 1);
        }
    }

    // Fixtures
    for fixture in &suite.fixtures {
        format_fixture(out, fixture, indent_level + 1);
    }

    // Benchmarks with comment preservation
    let mut last_bench_end = 0usize;
    for bench in &suite.benchmarks {
        // Extract comments between last benchmark and this one
        let comments_before =
            extract_comments_between(original_source, last_bench_end, bench.span.start);
        if !comments_before.is_empty() {
            // Output preserved comments
            for comment_line in comments_before.lines() {
                if !comment_line.trim().is_empty() {
                    writeln!(out, "{}{}", inner, comment_line.trim()).unwrap();
                } else {
                    out.push('\n');
                }
            }
        }

        format_benchmark(out, bench, indent_level + 1);
        last_bench_end = bench.span.end;
    }

    // Chart directives
    if !suite.chart_directives.is_empty() {
        format_chart_directives(out, &suite.chart_directives, indent_level + 1);
    }

    write!(out, "{}}}\n", pad).unwrap();
}

/// Extract comment lines between two positions in the source
fn extract_comments_between(source: &str, start: usize, end: usize) -> String {
    if start >= end || start >= source.len() {
        return String::new();
    }

    let slice = &source[start..end.min(source.len())];
    let mut comments = String::new();

    for line in slice.lines() {
        let trimmed = line.trim();
        // Only preserve lines that are pure comments (start with #)
        if trimmed.starts_with('#') {
            comments.push_str(trimmed);
            comments.push('\n');
        } else if trimmed.is_empty() && !comments.is_empty() {
            // Keep blank lines within comment blocks
            comments.push('\n');
        }
    }

    comments
}

/// Extract leading comments from source (comments before any code)
fn extract_leading_comments(source: &str) -> String {
    let mut comments = String::new();

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            // Keep blank lines in the comment block
            comments.push('\n');
        } else if trimmed.starts_with('#') {
            // This is a comment line
            comments.push_str(line);
            comments.push('\n');
        } else {
            // First non-comment, non-blank line - stop
            break;
        }
    }

    // Trim trailing newlines but keep one
    let trimmed = comments.trim_end();
    if trimmed.is_empty() {
        String::new()
    } else {
        format!("{}\n", trimmed)
    }
}

/// Format a globalSetup block
fn format_global_setup(out: &mut String, global_setup: &GlobalSetup, indent_level: usize) {
    let pad = INDENT.repeat(indent_level);
    let inner = INDENT.repeat(indent_level + 1);

    writeln!(out, "{}globalSetup {{", pad).unwrap();

    if let Some(ref anvil_config) = global_setup.anvil_config {
        if let Some(ref fork_url) = anvil_config.fork_url {
            writeln!(out, "{}anvil.spawnAnvil(fork: \"{}\")", inner, escape_string(fork_url))
                .unwrap();
        } else {
            writeln!(out, "{}anvil.spawnAnvil()", inner).unwrap();
        }
    }

    writeln!(out, "{}}}", pad).unwrap();
}

fn format_suite(out: &mut String, suite: &Suite, indent_level: usize) {
    let pad = INDENT.repeat(indent_level);
    let inner = INDENT.repeat(indent_level + 1);

    write!(out, "{}suite {} {{\n", pad, suite.name).unwrap();

    // Suite properties in canonical order
    if let Some(ref desc) = suite.description {
        write!(out, "{}description: \"{}\"\n", inner, escape_string(desc)).unwrap();
    }
    if let Some(n) = suite.iterations {
        write!(out, "{}iterations: {}\n", inner, n).unwrap();
    }
    if let Some(n) = suite.warmup {
        write!(out, "{}warmup: {}\n", inner, n).unwrap();
    }
    if let Some(n) = suite.timeout {
        write!(out, "{}timeout: {}\n", inner, n).unwrap();
    }
    if !suite.requires.is_empty() {
        let langs: Vec<_> = suite.requires.iter().map(|l| l.as_str()).collect();
        write!(out, "{}requires: [{}]\n", inner, langs.join(", ")).unwrap();
    }
    if let Some(order) = suite.order {
        let s = match order {
            ExecutionOrder::Sequential => "sequential",
            ExecutionOrder::Parallel => "parallel",
            ExecutionOrder::Random => "random",
        };
        write!(out, "{}order: {}\n", inner, s).unwrap();
    }
    if let Some(baseline) = suite.baseline {
        write!(out, "{}baseline: \"{}\"\n", inner, baseline.as_str()).unwrap();
    }

    // Benchmark accuracy settings - only output non-default values
    if let Some(mode) = suite.mode {
        write!(out, "{}mode: \"{}\"\n", inner, mode.as_str()).unwrap();
    }
    if let Some(target_time) = suite.target_time_ms {
        write!(out, "{}targetTime: {}ms\n", inner, target_time).unwrap();
    }
    if !suite.sink {
        // Only output sink: false since true is the default
        write!(out, "{}sink: false\n", inner).unwrap();
    }

    // Statistical analysis settings - only output non-default values
    if !suite.outlier_detection {
        // Only output outlierDetection: false since true is the default
        write!(out, "{}outlierDetection: false\n", inner).unwrap();
    }
    if let Some(cv_threshold) = suite.cv_threshold {
        write!(out, "{}cvThreshold: {}\n", inner, cv_threshold).unwrap();
    }
    if let Some(count) = suite.count {
        write!(out, "{}count: {}\n", inner, count).unwrap();
    }

    // Observability settings (Phase 2B) - only output non-default values
    if suite.memory {
        // Only output memory: true since false is the default
        write!(out, "{}memory: true\n", inner).unwrap();
    }

    // Add blank line after properties if there are any setups, fixtures, or benchmarks
    let has_content = suite.global_setup.is_some() ||
        !suite.setups.is_empty() ||
        !suite.fixtures.is_empty() ||
        !suite.benchmarks.is_empty();
    if has_content {
        out.push('\n');
    }

    // Suite-level globalSetup (if present and not inherited from file-level)
    if let Some(ref global_setup) = suite.global_setup {
        format_global_setup(out, global_setup, indent_level + 1);
        out.push('\n');
    }

    // Setups in canonical order: go, ts, rust, python
    let lang_order = [Lang::Go, Lang::TypeScript, Lang::Rust, Lang::Python];
    for lang in &lang_order {
        if let Some(setup) = suite.setups.get(lang) {
            format_setup(out, lang, setup, indent_level + 1);
        }
    }

    // Fixtures
    for fixture in &suite.fixtures {
        format_fixture(out, fixture, indent_level + 1);
    }

    // Benchmarks
    for bench in &suite.benchmarks {
        format_benchmark(out, bench, indent_level + 1);
    }

    // Chart directives (in after { } block)
    if !suite.chart_directives.is_empty() {
        format_chart_directives(out, &suite.chart_directives, indent_level + 1);
    }

    write!(out, "{}}}\n", pad).unwrap();
}

fn format_setup(out: &mut String, lang: &Lang, setup: &StructuredSetup, indent_level: usize) {
    let pad = INDENT.repeat(indent_level);
    let inner = INDENT.repeat(indent_level + 1);

    write!(out, "{}setup {} {{\n", pad, lang.as_str()).unwrap();

    let mut wrote_section = false;

    // Sections in canonical order: import, declare, init, helpers
    // Import: Go stores full "import ( ... )", TS stores inner content for "import { ... }"
    if let Some(ref imports) = setup.imports {
        let code = trim_code_block(&imports.code);
        if !code.is_empty() {
            if code.starts_with("import (") {
                // Go style: full "import ( ... )" - emit as-is with indent
                let preserved = preserve_embedded_code(&code, &inner);
                for line in &preserved {
                    writeln!(out, "{}", line).unwrap();
                }
            } else {
                // TS style: wrap inner content in "import { ... }"
                let inner2 = format!("{}{}", inner, INDENT);
                writeln!(out, "{}import {{", inner).unwrap();
                let preserved = preserve_embedded_code(&code, &inner2);
                for line in &preserved {
                    writeln!(out, "{}", line).unwrap();
                }
                writeln!(out, "{}}}", inner).unwrap();
            }
            wrote_section = true;
        }
    }
    if let Some(ref decl) = setup.declarations {
        if wrote_section {
            out.push('\n');
        }
        write_code_block(out, "declare", decl, &inner);
        wrote_section = true;
    }
    if let Some(ref init) = setup.init {
        if wrote_section {
            out.push('\n');
        }
        let kw = if setup.async_init { "async init" } else { "init" };
        write_code_block(out, kw, init, &inner);
        wrote_section = true;
    }
    if let Some(ref helpers) = setup.helpers {
        if wrote_section {
            out.push('\n');
        }
        write_code_block(out, "helpers", helpers, &inner);
    }

    write!(out, "{}}}\n\n", pad).unwrap();
}

fn write_code_block(out: &mut String, keyword: &str, block: &CodeBlock, inner: &str) {
    // Use trim_lines to preserve internal indentation structure
    let code = trim_code_block(&block.code);
    let content_indent = format!("{}{}", inner, INDENT); // One more level for content inside block

    // Check if the trimmed code is empty (whitespace-only blocks should be single-line)
    if code.is_empty() {
        // Empty blocks should be single-line: `init {}`
        writeln!(out, "{}{} {{}}", inner, keyword).unwrap();
    } else if code.contains('\n') {
        // Multi-line code block
        writeln!(out, "{}{} {{", inner, keyword).unwrap();
        let reformatted = reformat_embedded_code(&code, &content_indent);
        for line in &reformatted {
            if line.trim().is_empty() {
                out.push('\n');
            } else {
                writeln!(out, "{}", line).unwrap();
            }
        }
        writeln!(out, "{}}}", inner).unwrap();
    } else {
        // Single-line code - keep on same line with braces
        writeln!(out, "{}{} {{ {} }}", inner, keyword, code.trim()).unwrap();
    }
}

/// Trim a code block by removing leading/trailing empty lines
/// but preserving the internal indentation structure
fn trim_code_block(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();

    // Find first non-empty line
    let start = lines.iter().position(|l| !l.trim().is_empty()).unwrap_or(0);
    // Find last non-empty line
    let end =
        lines.iter().rposition(|l| !l.trim().is_empty()).map(|i| i + 1).unwrap_or(lines.len());

    if start >= end {
        return String::new();
    }

    lines[start..end].join("\n")
}

fn format_fixture(out: &mut String, fixture: &Fixture, indent_level: usize) {
    let pad = INDENT.repeat(indent_level);
    let inner = INDENT.repeat(indent_level + 1);

    write!(out, "{}fixture ", pad).unwrap();
    if fixture.params.is_empty() {
        write!(out, "{} {{\n", fixture.name).unwrap();
    } else {
        let params: Vec<_> =
            fixture.params.iter().map(|p| format!("{}: {}", p.name, p.param_type)).collect();
        write!(out, "{}({}) {{\n", fixture.name, params.join(", ")).unwrap();
    }

    if let Some(ref desc) = fixture.description {
        write!(out, "{}description: \"{}\"\n", inner, escape_string(desc)).unwrap();
    }
    if let Some(ref hex) = fixture.hex_data {
        write!(out, "{}hex: \"{}\"\n", inner, escape_string(hex)).unwrap();
    }
    if let Some(ref path) = fixture.hex_file {
        write!(out, "{}hex: @file(\"{}\")\n", inner, escape_string(path)).unwrap();
    }
    if let Some(ref shape) = fixture.shape {
        let shape = shape.trim();
        if shape.contains('\n') {
            write!(out, "{}shape {{\n", inner).unwrap();
            for line in shape.lines() {
                write!(out, "{}{}\n", inner, line).unwrap();
            }
            write!(out, "{}}}\n", inner).unwrap();
        } else {
            write!(out, "{}shape: {}\n", inner, shape).unwrap();
        }
    }

    // Use original definition order for implementations
    for lang in &fixture.impl_order {
        if let Some(code) = fixture.implementations.get(lang) {
            write!(out, "{}{}: ", inner, lang.as_str()).unwrap();
            format_code_block_inline_with_indent(out, code, &inner);
            out.push('\n');
        }
    }

    write!(out, "{}}}\n\n", pad).unwrap();
}

fn format_benchmark(out: &mut String, bench: &Benchmark, indent_level: usize) {
    let pad = INDENT.repeat(indent_level);
    let inner = INDENT.repeat(indent_level + 1);
    let lang_order = [Lang::Go, Lang::TypeScript, Lang::Rust, Lang::Python];

    write!(out, "{}bench {} {{\n", pad, bench.name).unwrap();

    if let Some(ref desc) = bench.description {
        write!(out, "{}description: \"{}\"\n", inner, escape_string(desc)).unwrap();
    }
    if let Some(n) = bench.iterations {
        write!(out, "{}iterations: {}\n", inner, n).unwrap();
    }
    if let Some(n) = bench.warmup {
        write!(out, "{}warmup: {}\n", inner, n).unwrap();
    }
    if let Some(n) = bench.timeout {
        write!(out, "{}timeout: {}\n", inner, n).unwrap();
    }
    if !bench.tags.is_empty() {
        let tags: Vec<_> = bench.tags.iter().map(|t| format!("\"{}\"", escape_string(t))).collect();
        write!(out, "{}tags: [{}]\n", inner, tags.join(", ")).unwrap();
    }

    // Benchmark accuracy overrides - only output if set (Option::Some)
    if let Some(mode) = bench.mode {
        write!(out, "{}mode: \"{}\"\n", inner, mode.as_str()).unwrap();
    }
    if let Some(target_time) = bench.target_time_ms {
        write!(out, "{}targetTime: {}ms\n", inner, target_time).unwrap();
    }
    if let Some(sink) = bench.sink {
        // Only output if explicitly set to override suite default
        write!(out, "{}sink: {}\n", inner, if sink { "true" } else { "false" }).unwrap();
    }
    if let Some(outlier_detection) = bench.outlier_detection {
        // Only output if explicitly set to override suite default
        write!(
            out,
            "{}outlierDetection: {}\n",
            inner,
            if outlier_detection { "true" } else { "false" }
        )
        .unwrap();
    }
    if let Some(cv_threshold) = bench.cv_threshold {
        write!(out, "{}cvThreshold: {}\n", inner, cv_threshold).unwrap();
    }
    if let Some(count) = bench.count {
        write!(out, "{}count: {}\n", inner, count).unwrap();
    }

    // Observability settings (Phase 2B) - only output if explicitly set
    if let Some(memory) = bench.memory {
        write!(out, "{}memory: {}\n", inner, if memory { "true" } else { "false" }).unwrap();
    }

    // Skip and validate hooks (always flat syntax)
    for lang in &lang_order {
        if let Some(code) = bench.skip.get(lang) {
            write!(out, "{}skip {}: ", inner, lang.as_str()).unwrap();
            format_code_block_inline_with_indent(out, code, &inner);
            out.push('\n');
        }
    }
    for lang in &lang_order {
        if let Some(code) = bench.validate.get(lang) {
            write!(out, "{}validate {}: ", inner, lang.as_str()).unwrap();
            format_code_block_inline_with_indent(out, code, &inner);
            out.push('\n');
        }
    }

    // Lifecycle hooks - respect original style (grouped vs flat)
    match bench.hook_style {
        HookStyle::Grouped => {
            // Grouped syntax: before: { go: ... ts: ... }
            format_grouped_hooks(out, "before", &bench.before, &lang_order, &inner);
            format_grouped_hooks(out, "each", &bench.each, &lang_order, &inner);
        }
        HookStyle::Flat => {
            // Flat syntax: before go: ... \n before ts: ...
            for lang in &lang_order {
                if let Some(code) = bench.before.get(lang) {
                    write!(out, "{}before {}: ", inner, lang.as_str()).unwrap();
                    format_code_block_inline_with_indent(out, code, &inner);
                    out.push('\n');
                }
            }
            for lang in &lang_order {
                if let Some(code) = bench.each.get(lang) {
                    write!(out, "{}each {}: ", inner, lang.as_str()).unwrap();
                    format_code_block_inline_with_indent(out, code, &inner);
                    out.push('\n');
                }
            }
        }
    }

    // Language implementations (preserve original order)
    for lang in &bench.impl_order {
        if let Some(code) = bench.implementations.get(lang) {
            write!(out, "{}{}: ", inner, lang.as_str()).unwrap();
            format_code_block_inline_with_indent(out, code, &inner);
            out.push('\n');
        }
    }

    // After hooks - respect original style (grouped vs flat)
    match bench.hook_style {
        HookStyle::Grouped => {
            format_grouped_hooks(out, "after", &bench.after, &lang_order, &inner);
        }
        HookStyle::Flat => {
            for lang in &lang_order {
                if let Some(code) = bench.after.get(lang) {
                    write!(out, "{}after {}: ", inner, lang.as_str()).unwrap();
                    format_code_block_inline_with_indent(out, code, &inner);
                    out.push('\n');
                }
            }
        }
    }

    write!(out, "{}}}\n\n", pad).unwrap();
}

/// Format lifecycle hooks in grouped syntax: hook_name: { go: CODE ts: CODE }
fn format_grouped_hooks(
    out: &mut String,
    hook_name: &str,
    hooks: &std::collections::HashMap<Lang, CodeBlock>,
    lang_order: &[Lang],
    inner: &str,
) {
    if hooks.is_empty() {
        return;
    }

    let inner2 = format!("{}{}", inner, INDENT);

    out.push('\n');
    writeln!(out, "{}{}: {{", inner, hook_name).unwrap();
    for lang in lang_order {
        if let Some(code) = hooks.get(lang) {
            write!(out, "{}{}: ", inner2, lang.as_str()).unwrap();
            format_code_block_inline_with_indent(out, code, &inner2);
            out.push('\n');
        }
    }
    writeln!(out, "{}}}", inner).unwrap();
}

/// Format an inline code block (e.g. `go: { ... }` or `ts: expr`).
/// `line_indent` is the indent of the current line (e.g. "        " for benchmark props);
/// block content inside braces will be indented with line_indent + one more level.
fn format_code_block_inline_with_indent(out: &mut String, block: &CodeBlock, line_indent: &str) {
    let code = trim_code_block(&block.code);
    if block.is_multiline || code.contains('\n') {
        writeln!(out, "{{").unwrap();
        let block_content_indent = format!("{}{}", line_indent, INDENT);
        let preserved = preserve_embedded_code(&code, &block_content_indent);
        for line in &preserved {
            if line.trim().is_empty() {
                out.push('\n');
            } else {
                writeln!(out, "{}", line).unwrap();
            }
        }
        write!(out, "{}}}", line_indent).unwrap();
    } else {
        write!(out, "{}", code.trim()).unwrap();
    }
}

/// Format chart directives inside an after { } block
fn format_chart_directives(out: &mut String, directives: &[ChartDirective], indent_level: usize) {
    let pad = INDENT.repeat(indent_level);
    let inner = INDENT.repeat(indent_level + 1);
    let inner2 = INDENT.repeat(indent_level + 2);

    writeln!(out, "{}after {{", pad).unwrap();

    for (i, directive) in directives.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }

        // Determine function name based on chart type
        let func_name = match directive.chart_type {
            ChartType::SpeedupChart => "drawSpeedupChart",
            ChartType::Table => "drawTable",
        };

        // Collect all the parameters to output, preserving original order if available
        let params: Vec<String> = if !directive.param_order.is_empty() {
            // Use the original parameter order from parsing
            format_params_in_order(directive, &inner2)
        } else {
            // Fall back to default order for backwards compatibility
            format_params_default_order(directive, &inner2)
        };

        // Output the directive
        if params.is_empty() {
            writeln!(out, "{}charting.{}()", inner, func_name).unwrap();
        } else {
            writeln!(out, "{}charting.{}(", inner, func_name).unwrap();
            for (j, param) in params.iter().enumerate() {
                if j < params.len() - 1 {
                    writeln!(out, "{},", param).unwrap();
                } else {
                    writeln!(out, "{}", param).unwrap();
                }
            }
            writeln!(out, "{})", inner).unwrap();
        }
    }

    writeln!(out, "{}}}", pad).unwrap();
    out.push('\n');
}

/// Format parameters in the order they appeared in the source
fn format_params_in_order(directive: &ChartDirective, indent: &str) -> Vec<String> {
    let mut params = Vec::new();

    for param_name in &directive.param_order {
        if let Some(formatted) = format_single_param(directive, param_name, indent) {
            params.push(formatted);
        }
    }

    params
}

/// Format a single parameter by name
fn format_single_param(directive: &ChartDirective, name: &str, indent: &str) -> Option<String> {
    match name {
        // String parameters
        "title" => {
            directive.title.as_ref().map(|v| format!("{}title: \"{}\"", indent, escape_string(v)))
        }
        "description" => directive
            .description
            .as_ref()
            .map(|v| format!("{}description: \"{}\"", indent, escape_string(v))),
        "output" => directive
            .output_file
            .as_ref()
            .map(|v| format!("{}output: \"{}\"", indent, escape_string(v))),
        "filterWinner" => directive
            .filter_winner
            .as_ref()
            .map(|v| format!("{}filterWinner: \"{}\"", indent, escape_string(v))),
        "sortBy" => directive
            .sort_by
            .as_ref()
            .map(|v| format!("{}sortBy: \"{}\"", indent, escape_string(v))),
        "sortOrder" => directive
            .sort_order
            .as_ref()
            .map(|v| format!("{}sortOrder: \"{}\"", indent, escape_string(v))),
        "baseline" | "baselineBenchmark" => directive
            .baseline_benchmark
            .as_ref()
            .map(|v| format!("{}baselineBenchmark: \"{}\"", indent, escape_string(v))),
        "theme" => {
            directive.theme.as_ref().map(|v| format!("{}theme: \"{}\"", indent, escape_string(v)))
        }

        // Integer parameters
        "limit" => directive.limit.map(|v| format!("{}limit: {}", indent, v)),
        "width" => directive.width.map(|v| format!("{}width: {}", indent, v)),
        "rowCount" => directive.row_count.map(|v| format!("{}rowCount: {}", indent, v)),
        "height" => directive.height.map(|v| format!("{}height: {}", indent, v)),

        // Float parameters
        "minSpeedup" => directive.min_speedup.map(|v| format!("{}minSpeedup: {}", indent, v)),

        // Array parameters
        "includeBenchmarks" => {
            if !directive.include_benchmarks.is_empty() {
                let items: Vec<_> = directive
                    .include_benchmarks
                    .iter()
                    .map(|s| format!("\"{}\"", escape_string(s)))
                    .collect();
                Some(format!("{}includeBenchmarks: [{}]", indent, items.join(", ")))
            } else {
                None
            }
        }
        "excludeBenchmarks" => {
            if !directive.exclude_benchmarks.is_empty() {
                let items: Vec<_> = directive
                    .exclude_benchmarks
                    .iter()
                    .map(|s| format!("\"{}\"", escape_string(s)))
                    .collect();
                Some(format!("{}excludeBenchmarks: [{}]", indent, items.join(", ")))
            } else {
                None
            }
        }

        _ => None,
    }
}

/// Format parameters in default order (for backwards compatibility)
fn format_params_default_order(directive: &ChartDirective, inner2: &str) -> Vec<String> {
    let mut params: Vec<String> = Vec::new();

    // Title and description
    if let Some(ref title) = directive.title {
        params.push(format!("{}title: \"{}\"", inner2, escape_string(title)));
    }
    if let Some(ref desc) = directive.description {
        params.push(format!("{}description: \"{}\"", inner2, escape_string(desc)));
    }

    // Output file
    if let Some(ref output) = directive.output_file {
        params.push(format!("{}output: \"{}\"", inner2, escape_string(output)));
    }

    // Filtering
    if let Some(min_speedup) = directive.min_speedup {
        params.push(format!("{}minSpeedup: {}", inner2, min_speedup));
    }
    if let Some(ref filter_winner) = directive.filter_winner {
        params.push(format!("{}filterWinner: \"{}\"", inner2, escape_string(filter_winner)));
    }
    if !directive.include_benchmarks.is_empty() {
        let items: Vec<_> = directive
            .include_benchmarks
            .iter()
            .map(|s| format!("\"{}\"", escape_string(s)))
            .collect();
        params.push(format!("{}includeBenchmarks: [{}]", inner2, items.join(", ")));
    }
    if !directive.exclude_benchmarks.is_empty() {
        let items: Vec<_> = directive
            .exclude_benchmarks
            .iter()
            .map(|s| format!("\"{}\"", escape_string(s)))
            .collect();
        params.push(format!("{}excludeBenchmarks: [{}]", inner2, items.join(", ")));
    }
    if let Some(limit) = directive.limit {
        params.push(format!("{}limit: {}", inner2, limit));
    }

    // Sorting
    if let Some(ref sort_by) = directive.sort_by {
        params.push(format!("{}sortBy: \"{}\"", inner2, escape_string(sort_by)));
    }
    if let Some(ref sort_order) = directive.sort_order {
        params.push(format!("{}sortOrder: \"{}\"", inner2, escape_string(sort_order)));
    }

    // Layout
    if let Some(width) = directive.width {
        params.push(format!("{}width: {}", inner2, width));
    }
    if let Some(row_count) = directive.row_count {
        params.push(format!("{}rowCount: {}", inner2, row_count));
    }

    // Dimensions
    if let Some(height) = directive.height {
        params.push(format!("{}height: {}", inner2, height));
    }

    if let Some(ref baseline_benchmark) = directive.baseline_benchmark {
        params.push(format!(
            "{}baselineBenchmark: \"{}\"",
            inner2,
            escape_string(baseline_benchmark)
        ));
    }
    if let Some(ref theme) = directive.theme {
        params.push(format!("{}theme: \"{}\"", inner2, escape_string(theme)));
    }

    params
}

fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    #[test]
    fn test_format_chart_directives() {
        let input = r#"use std::charting

suite example {
    bench sha256Bench {
        go: sha256SumGo(data)
    }

    after {
        charting.drawSpeedupChart(
            title: "Speedup Comparison",
            sortBy: "speedup"
        )
        
        charting.drawTable(
            title: "Results Table",
            theme: "light"
        )
    }
}"#;
        let ast = parse(input, "test.bench").unwrap();
        assert_eq!(ast.suites.len(), 1);
        assert_eq!(ast.suites[0].chart_directives.len(), 2, "Expected 2 chart directives");

        let formatted = format_file(&ast);
        assert!(formatted.contains("after {"), "Expected 'after {{' block in output");
        assert!(formatted.contains("charting.drawSpeedupChart"), "Expected drawSpeedupChart call");
        assert!(formatted.contains("charting.drawTable"), "Expected drawTable call in output");
        // Verify non-default values are preserved
        assert!(formatted.contains("sortBy: \"speedup\""), "Expected sortBy parameter");
        assert!(formatted.contains("theme: \"light\""), "Expected theme parameter");
    }

    #[test]
    fn test_format_roundtrip() {
        let input = r#"suite example {
    description: "Example benchmark"
    iterations: 50
    warmup: 100
    setup go {
        import (
            "crypto/sha256"
        )
        init {}
        helpers {
            func sha256SumGo(data []byte) [32]byte {
                return sha256.Sum256(data)
            }
        }
    }
    fixture data {
        hex: "68656c6c6f20776f726c64"
    }
    bench sha256Bench {
        go: sha256SumGo(data)
        ts: sha256SumTs(data)
    }
}"#;
        let ast = parse(input, "test.bench").unwrap();
        let formatted = format_file(&ast);
        let roundtrip = parse(&formatted, "test.bench").unwrap();
        assert_eq!(ast.suites.len(), roundtrip.suites.len());
        assert_eq!(ast.suites[0].name, roundtrip.suites[0].name);
    }

    #[test]
    fn test_format_preserves_use_statements() {
        let input = r#"use std::anvil

suite example {
    bench test {
        go: doSomething()
    }
}"#;
        let ast = parse(input, "test.bench").unwrap();
        let formatted = format_file(&ast);

        // Verify use statement is preserved
        assert!(formatted.contains("use std::anvil"), "use statement should be preserved");
        assert!(formatted.starts_with("use std::anvil"), "use statement should be at the start");
    }

    #[test]
    fn test_format_preserves_comments_with_source() {
        let input = r#"# This is a comment about the benchmark
# Another comment line

use std::anvil

suite example {
    bench test {
        go: doSomething()
    }
}"#;
        let ast = parse(input, "test.bench").unwrap();
        let formatted = format_file_with_source(&ast, input);

        // Verify comments are preserved
        assert!(
            formatted.contains("# This is a comment about the benchmark"),
            "first comment should be preserved"
        );
        assert!(formatted.contains("# Another comment line"), "second comment should be preserved");
        // Verify use statement is preserved
        assert!(formatted.contains("use std::anvil"), "use statement should be preserved");
    }

    #[test]
    fn test_format_suite_with_global_setup() {
        let input = r#"use std::anvil

suite example {
    globalSetup {
        anvil.spawnAnvil()
    }
    
    bench test {
        go: doSomething()
    }
}"#;
        let ast = parse(input, "test.bench").unwrap();
        let formatted = format_file(&ast);

        // Verify use statement is preserved
        assert!(formatted.contains("use std::anvil"), "use statement should be preserved");
        // Verify globalSetup is inside suite
        assert!(formatted.contains("globalSetup {"), "globalSetup should be present");
        assert!(formatted.contains("anvil.spawnAnvil()"), "spawnAnvil call should be present");
    }

    #[test]
    fn test_extract_leading_comments() {
        let source = "# Comment 1\n# Comment 2\n\nuse std::anvil\nsuite test {}";
        let comments = extract_leading_comments(source);
        assert!(comments.contains("# Comment 1"));
        assert!(comments.contains("# Comment 2"));
        assert!(!comments.contains("use std::anvil"));
    }
}
