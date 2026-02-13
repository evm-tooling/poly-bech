//! AST-based formatter for the poly-bench DSL
//!
//! Produces consistently indented output. Embedded code blocks (Go, TypeScript)
//! are normalized (min indent stripped) then re-indented.
//!
//! The formatter preserves:
//! - `use std::module` import statements
//! - File-level `globalSetup` blocks
//! - Suite-level `globalSetup` blocks
//! - Comments (when using `format_file_preserving_comments`)

use crate::ast::*;
use crate::ChartType;
use std::fmt::Write;

const INDENT: &str = "    ";

/// Normalize embedded code: strip minimum leading indent, then re-normalize remaining
/// relative indentation using smart heuristics to fix broken source indentation.
fn normalize_embedded_code(code: &str) -> Vec<String> {
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
    
    // First pass: calculate raw indent levels for each line
    let mut raw_levels: Vec<(usize, &str)> = Vec::new();
    for l in &lines {
        if l.trim().is_empty() {
            raw_levels.push((0, ""));
        } else {
            let leading = l.len() - l.trim_start().len();
            let extra_spaces = leading.saturating_sub(min_indent);
            // Use any 4+ spaces as one indent level
            let level = extra_spaces / 4;
            raw_levels.push((level, l.trim_start()));
        }
    }
    
    // Second pass: apply smart normalization based on code structure
    // Use closing brace detection to determine indent level
    let mut result = Vec::new();
    let mut current_depth = 0usize;
    
    for (_, content) in raw_levels.iter() {
        if content.is_empty() {
            result.push(String::new());
            continue;
        }
        
        // Check if this line starts with a closing brace/bracket
        let starts_with_close = content.starts_with('}') || 
                                 content.starts_with(')') || 
                                 content.starts_with(']');
        
        // Decrease depth before this line if it starts with closing brace
        if starts_with_close && current_depth > 0 {
            current_depth -= 1;
        }
        
        let pad = INDENT.repeat(current_depth);
        result.push(format!("{}{}", pad, content));
        
        // Count braces to adjust depth for next line
        let opens = content.chars().filter(|c| *c == '{' || *c == '(' || *c == '[').count();
        let closes = content.chars().filter(|c| *c == '}' || *c == ')' || *c == ']').count();
        
        // Adjust depth for next line (not counting the starting close we already handled)
        let effective_closes = if starts_with_close { closes - 1 } else { closes };
        if opens > effective_closes {
            current_depth += opens - effective_closes;
        } else if effective_closes > opens && current_depth >= (effective_closes - opens) {
            current_depth -= effective_closes - opens;
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
            writeln!(out, "{}anvil.spawnAnvil(fork: \"{}\")", inner, escape_string(fork_url)).unwrap();
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
    if suite.compare {
        write!(out, "{}compare: true\n", inner).unwrap();
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
    if let Some(min_iters) = suite.min_iterations {
        write!(out, "{}minIterations: {}\n", inner, min_iters).unwrap();
    }
    if let Some(max_iters) = suite.max_iterations {
        write!(out, "{}maxIterations: {}\n", inner, max_iters).unwrap();
    }
    if !suite.sink {
        // Only output sink: false since true is the default
        write!(out, "{}sink: false\n", inner).unwrap();
    }

    // Add blank line after properties if there are any setups, fixtures, or benchmarks
    let has_content = suite.global_setup.is_some() || !suite.setups.is_empty() || !suite.fixtures.is_empty() || !suite.benchmarks.is_empty();
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
        let code = imports.code.trim();
        if !code.is_empty() {
            if code.starts_with("import (") {
                // Go style: full "import ( ... )" - emit as-is with indent
                let normalized = normalize_embedded_code(code);
                for line in &normalized {
                    write!(out, "{}{}\n", inner, line).unwrap();
                }
            } else {
                // TS style: wrap inner content in "import { ... }"
                let inner2 = format!("{}{}", inner, INDENT);
                write!(out, "{}import {{\n", inner).unwrap();
                let normalized = normalize_embedded_code(code);
                for line in &normalized {
                    write!(out, "{}{}\n", inner2, line).unwrap();
                }
                write!(out, "{}}}\n", inner).unwrap();
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
    let code = block.code.trim();
    let content_indent = format!("{}{}", inner, INDENT); // One more level for content inside block
    if code.is_empty() {
        write!(out, "{}{} {{\n{}}}\n", inner, keyword, inner).unwrap();
    } else if block.is_multiline || code.contains('\n') {
        write!(out, "{}{} {{\n", inner, keyword).unwrap();
        let normalized = normalize_embedded_code(code);
        for line in &normalized {
            if line.is_empty() {
                out.push('\n');
            } else {
                write!(out, "{}{}\n", content_indent, line).unwrap();
            }
        }
        write!(out, "{}}}\n", inner).unwrap();
    } else {
        write!(out, "{}{} {}\n", inner, keyword, code).unwrap();
    }
}

fn format_fixture(out: &mut String, fixture: &Fixture, indent_level: usize) {
    let pad = INDENT.repeat(indent_level);
    let inner = INDENT.repeat(indent_level + 1);

    write!(out, "{}fixture ", pad).unwrap();
    if fixture.params.is_empty() {
        write!(out, "{} {{\n", fixture.name).unwrap();
    } else {
        let params: Vec<_> = fixture
            .params
            .iter()
            .map(|p| format!("{}: {}", p.name, p.param_type))
            .collect();
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

    let lang_order = [Lang::Go, Lang::TypeScript, Lang::Rust, Lang::Python];
    for lang in &lang_order {
        if let Some(code) = fixture.implementations.get(lang) {
            write!(out, "{}{}: ", inner, lang.as_str()).unwrap();
            format_code_block_inline(out, code);
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
    if let Some(min_iters) = bench.min_iterations {
        write!(out, "{}minIterations: {}\n", inner, min_iters).unwrap();
    }
    if let Some(max_iters) = bench.max_iterations {
        write!(out, "{}maxIterations: {}\n", inner, max_iters).unwrap();
    }
    if let Some(sink) = bench.sink {
        // Only output if explicitly set to override suite default
        write!(out, "{}sink: {}\n", inner, if sink { "true" } else { "false" }).unwrap();
    }

    for lang in &lang_order {
        if let Some(code) = bench.skip.get(lang) {
            write!(out, "{}skip {}: ", inner, lang.as_str()).unwrap();
            format_code_block_inline(out, code);
            out.push('\n');
        }
    }
    for lang in &lang_order {
        if let Some(code) = bench.validate.get(lang) {
            write!(out, "{}validate {}: ", inner, lang.as_str()).unwrap();
            format_code_block_inline(out, code);
            out.push('\n');
        }
    }
    for lang in &lang_order {
        if let Some(code) = bench.before.get(lang) {
            write!(out, "{}before {}: ", inner, lang.as_str()).unwrap();
            format_code_block_inline(out, code);
            out.push('\n');
        }
    }
    for lang in &lang_order {
        if let Some(code) = bench.after.get(lang) {
            write!(out, "{}after {}: ", inner, lang.as_str()).unwrap();
            format_code_block_inline(out, code);
            out.push('\n');
        }
    }
    for lang in &lang_order {
        if let Some(code) = bench.each.get(lang) {
            write!(out, "{}each {}: ", inner, lang.as_str()).unwrap();
            format_code_block_inline(out, code);
            out.push('\n');
        }
    }

    for lang in &lang_order {
        if let Some(code) = bench.implementations.get(lang) {
            write!(out, "{}{}: ", inner, lang.as_str()).unwrap();
            format_code_block_inline(out, code);
            out.push('\n');
        }
    }

    write!(out, "{}}}\n\n", pad).unwrap();
}

fn format_code_block_inline(out: &mut String, block: &CodeBlock) {
    let code = block.code.trim();
    if block.is_multiline || code.contains('\n') {
        write!(out, "{{\n").unwrap();
        let normalized = normalize_embedded_code(code);
        for line in &normalized {
            write!(out, "{}{}\n", INDENT, line).unwrap();
        }
        out.push('}');
    } else {
        write!(out, "{}", code).unwrap();
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
            ChartType::BarChart => "drawBarChart",
            ChartType::PieChart => "drawPieChart",
            ChartType::LineChart => "drawLineChart",
        };
        
        // Collect all the parameters to output
        let mut params: Vec<String> = Vec::new();
        
        // Title and description
        if let Some(ref title) = directive.title {
            params.push(format!("{}title: \"{}\"", inner2, escape_string(title)));
        }
        if let Some(ref desc) = directive.description {
            params.push(format!("{}description: \"{}\"", inner2, escape_string(desc)));
        }
        
        // Axis labels (for line/bar charts)
        if let Some(ref xlabel) = directive.x_label {
            params.push(format!("{}xlabel: \"{}\"", inner2, escape_string(xlabel)));
        }
        if let Some(ref ylabel) = directive.y_label {
            params.push(format!("{}ylabel: \"{}\"", inner2, escape_string(ylabel)));
        }
        
        // Output file
        if let Some(ref output) = directive.output_file {
            params.push(format!("{}output: \"{}\"", inner2, escape_string(output)));
        }
        
        // Display toggles - only output non-default values
        // Defaults: showStats=true, showConfig=true, showWinCounts=true, showGeoMean=true,
        //           showDistribution=true, showMemory=false, showTotalTime=false, compact=false
        if !directive.show_stats {
            params.push(format!("{}showStats: false", inner2));
        }
        if !directive.show_config {
            params.push(format!("{}showConfig: false", inner2));
        }
        if !directive.show_win_counts {
            params.push(format!("{}showWinCounts: false", inner2));
        }
        if !directive.show_geo_mean {
            params.push(format!("{}showGeoMean: false", inner2));
        }
        if !directive.show_distribution {
            params.push(format!("{}showDistribution: false", inner2));
        }
        if directive.show_memory {
            params.push(format!("{}showMemory: true", inner2));
        }
        if directive.show_total_time {
            params.push(format!("{}showTotalTime: true", inner2));
        }
        if directive.compact {
            params.push(format!("{}compact: true", inner2));
        }
        
        // Filtering
        if let Some(min_speedup) = directive.min_speedup {
            params.push(format!("{}minSpeedup: {}", inner2, min_speedup));
        }
        if let Some(ref filter_winner) = directive.filter_winner {
            params.push(format!("{}filterWinner: \"{}\"", inner2, escape_string(filter_winner)));
        }
        if !directive.include_benchmarks.is_empty() {
            let items: Vec<_> = directive.include_benchmarks.iter()
                .map(|s| format!("\"{}\"", escape_string(s)))
                .collect();
            params.push(format!("{}includeBenchmarks: [{}]", inner2, items.join(", ")));
        }
        if !directive.exclude_benchmarks.is_empty() {
            let items: Vec<_> = directive.exclude_benchmarks.iter()
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
        if let Some(bar_height) = directive.bar_height {
            params.push(format!("{}barHeight: {}", inner2, bar_height));
        }
        if let Some(bar_gap) = directive.bar_gap {
            params.push(format!("{}barGap: {}", inner2, bar_gap));
        }
        if let Some(margin_left) = directive.margin_left {
            params.push(format!("{}marginLeft: {}", inner2, margin_left));
        }
        
        // Data display
        if let Some(precision) = directive.precision {
            params.push(format!("{}precision: {}", inner2, precision));
        }
        if let Some(ref time_unit) = directive.time_unit {
            params.push(format!("{}timeUnit: \"{}\"", inner2, escape_string(time_unit)));
        }
        
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
        charting.drawBarChart(
            title: "Performance Comparison",
            sortBy: "speedup"
        )
        
        charting.drawLineChart(
            title: "Performance Trends"
        )
        
        charting.drawPieChart(
            title: "Time Distribution",
            showTotalTime: true
        )
    }
}"#;
        let ast = parse(input, "test.bench").unwrap();
        assert_eq!(ast.suites.len(), 1);
        assert_eq!(ast.suites[0].chart_directives.len(), 3, "Expected 3 chart directives");
        
        let formatted = format_file(&ast);
        assert!(formatted.contains("after {"), "Expected 'after {{' block in output");
        assert!(formatted.contains("charting.drawBarChart"), "Expected drawBarChart call in output");
        assert!(formatted.contains("charting.drawLineChart"), "Expected drawLineChart call in output");
        assert!(formatted.contains("charting.drawPieChart"), "Expected drawPieChart call in output");
        // Verify non-default values are preserved
        assert!(formatted.contains("sortBy: \"speedup\""), "Expected sortBy parameter");
        assert!(formatted.contains("showTotalTime: true"), "Expected showTotalTime parameter");
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
        assert!(formatted.contains("# This is a comment about the benchmark"), "first comment should be preserved");
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
