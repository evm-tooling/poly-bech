//! AST-based formatter for the poly-bench DSL
//!
//! Produces consistently indented output. Embedded code blocks (Go, TypeScript)
//! are normalized (min indent stripped) then re-indented.

use crate::dsl::ast::*;
use std::fmt::Write;

const INDENT: &str = "    ";

/// Normalize embedded code: strip minimum leading indent, then convert remaining
/// relative indent to consistent 4-space units (each 4 spaces = 1 level).
fn normalize_embedded_code(code: &str) -> Vec<String> {
    let lines: Vec<&str> = code.lines().collect();
    if lines.is_empty() {
        return Vec::new();
    }
    let min_indent = lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min()
        .unwrap_or(0);
    let indent_unit = 4; // assume 4-space indent in source
    lines
        .iter()
        .map(|l| {
            let leading = l.chars().take_while(|c| c.is_whitespace()).count();
            let trim_len = min_indent.min(leading);
            let rest: String = l.chars().skip(trim_len).collect();
            let extra_spaces = leading.saturating_sub(trim_len);
            let levels = extra_spaces / indent_unit;
            let pad = INDENT.repeat(levels);
            format!("{}{}", pad, rest.trim_start())
        })
        .collect()
}

/// Format an AST into a string with consistent indentation and style.
pub fn format_file(file: &File) -> String {
    let mut out = String::new();
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

    write!(out, "{}}}\n", pad).unwrap();
}

fn format_setup(out: &mut String, lang: &Lang, setup: &StructuredSetup, indent_level: usize) {
    let pad = INDENT.repeat(indent_level);
    let inner = INDENT.repeat(indent_level + 1);

    write!(out, "{}setup {} {{\n", pad, lang.as_str()).unwrap();

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
        }
    }
    if let Some(ref decl) = setup.declarations {
        write_code_block(out, "declare", decl, &inner);
    }
    if let Some(ref init) = setup.init {
        let kw = if setup.async_init { "async init" } else { "init" };
        write_code_block(out, kw, init, &inner);
    }
    if let Some(ref helpers) = setup.helpers {
        write_code_block(out, "helpers", helpers, &inner);
    }

    write!(out, "{}}}\n\n", pad).unwrap();
}

fn write_code_block(out: &mut String, keyword: &str, block: &CodeBlock, inner: &str) {
    let code = block.code.trim();
    if code.is_empty() {
        write!(out, "{}{} {{\n{}}}\n", inner, keyword, inner).unwrap();
    } else if block.is_multiline || code.contains('\n') {
        write!(out, "{}{} {{\n", inner, keyword).unwrap();
        let normalized = normalize_embedded_code(code);
        for line in &normalized {
            write!(out, "{}{}\n", inner, line).unwrap();
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
    use crate::dsl::parse;

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
}
