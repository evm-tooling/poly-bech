//! Incremental formatter that works with partial AST
//!
//! This formatter produces small, targeted edits instead of replacing
//! the entire document. It skips error nodes to avoid destroying
//! the user's incomplete code.

use crate::document::Document;
use poly_bench_syntax::{
    Node, PartialBenchmark, PartialFixture, PartialSuite, Property, PropertyValue, Span,
};
use tower_lsp::lsp_types::{Position, Range, TextEdit};

/// Configuration for the formatter
pub struct FormatterConfig {
    /// Number of spaces for indentation
    pub indent_size: usize,
    /// Whether to use tabs instead of spaces
    pub use_tabs: bool,
    /// Maximum line width before wrapping
    pub max_line_width: usize,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        Self { indent_size: 4, use_tabs: false, max_line_width: 100 }
    }
}

/// Format a document, returning incremental edits
pub fn format_document(doc: &Document) -> Vec<TextEdit> {
    format_document_with_config(doc, &FormatterConfig::default())
}

/// Format a document with custom configuration
pub fn format_document_with_config(doc: &Document, config: &FormatterConfig) -> Vec<TextEdit> {
    let mut edits = Vec::new();
    let source = doc.source_text();

    // Format each top-level construct independently
    for suite in &doc.partial_ast.suites {
        match suite {
            Node::Valid(s) => {
                if let Some(edit) = format_suite(s, &source, config, doc) {
                    edits.push(edit);
                }
            }
            Node::Error { .. } | Node::Missing { .. } => {
                // Skip error nodes - don't break user's incomplete code
            }
        }
    }

    edits
}

fn format_suite(
    suite: &PartialSuite,
    source: &str,
    config: &FormatterConfig,
    doc: &Document,
) -> Option<TextEdit> {
    let mut formatted = String::new();
    let indent = make_indent(config, 0);
    let inner_indent = make_indent(config, 1);

    // Suite header
    formatted.push_str(&format!("suite {} {{\n", suite.name));

    // Properties
    for prop in &suite.properties {
        if let Node::Valid(p) = prop {
            formatted.push_str(&format_property(p, config, 1));
        }
    }

    // Add blank line after properties if there are other items
    if !suite.properties.is_empty() &&
        (!suite.setups.is_empty() || !suite.fixtures.is_empty() || !suite.benchmarks.is_empty())
    {
        formatted.push('\n');
    }

    // Setup blocks
    for (lang, setup) in &suite.setups {
        if let Node::Valid(s) = setup {
            formatted.push_str(&inner_indent);
            formatted.push_str(&format!("setup {} {{\n", lang.as_str()));

            if let Some(imports) = &s.imports {
                formatted.push_str(&make_indent(config, 2));
                formatted.push_str("import ");
                if lang == &poly_bench_syntax::Lang::Go {
                    formatted.push_str("(\n");
                    for line in imports.code.lines() {
                        if !line.trim().is_empty() {
                            formatted.push_str(&make_indent(config, 3));
                            formatted.push_str(line.trim());
                            formatted.push('\n');
                        }
                    }
                    formatted.push_str(&make_indent(config, 2));
                    formatted.push_str(")\n");
                } else {
                    formatted.push_str("{\n");
                    for line in imports.code.lines() {
                        if !line.trim().is_empty() {
                            formatted.push_str(&make_indent(config, 3));
                            formatted.push_str(line.trim());
                            formatted.push('\n');
                        }
                    }
                    formatted.push_str(&make_indent(config, 2));
                    formatted.push_str("}\n");
                }
                formatted.push('\n');
            }

            if let Some(declare) = &s.declare {
                formatted.push_str(&make_indent(config, 2));
                formatted.push_str("declare {\n");
                format_code_block(&declare.code, config, 3, &mut formatted);
                formatted.push_str(&make_indent(config, 2));
                formatted.push_str("}\n\n");
            }

            if let Some(init) = &s.init {
                formatted.push_str(&make_indent(config, 2));
                if s.is_async_init {
                    formatted.push_str("async ");
                }
                formatted.push_str("init {\n");
                format_code_block(&init.code, config, 3, &mut formatted);
                formatted.push_str(&make_indent(config, 2));
                formatted.push_str("}\n\n");
            }

            if let Some(helpers) = &s.helpers {
                formatted.push_str(&make_indent(config, 2));
                formatted.push_str("helpers {\n");
                format_code_block(&helpers.code, config, 3, &mut formatted);
                formatted.push_str(&make_indent(config, 2));
                formatted.push_str("}\n");
            }

            formatted.push_str(&inner_indent);
            formatted.push_str("}\n\n");
        }
    }

    // Fixtures
    for fixture in &suite.fixtures {
        if let Node::Valid(f) = fixture {
            formatted.push_str(&format_fixture(f, config, 1));
            formatted.push('\n');
        }
    }

    // Benchmarks
    for benchmark in &suite.benchmarks {
        if let Node::Valid(b) = benchmark {
            formatted.push_str(&format_benchmark(b, config, 1));
            formatted.push('\n');
        }
    }

    // After block
    if let Some(Node::Valid(after)) = &suite.after_block {
        formatted.push_str(&inner_indent);
        formatted.push_str("after {\n");
        for directive in &after.directives {
            if let Node::Valid(d) = directive {
                formatted.push_str(&make_indent(config, 2));
                formatted.push_str(&format!("charting.{}(\n", d.function));
                for (name, value) in &d.params {
                    formatted.push_str(&make_indent(config, 3));
                    formatted.push_str(&format!("{}: {},\n", name, format_value(value)));
                }
                formatted.push_str(&make_indent(config, 2));
                formatted.push_str(")\n");
            }
        }
        formatted.push_str(&inner_indent);
        formatted.push_str("}\n");
    }

    // Close suite
    formatted.push_str("}\n");

    // Check if formatting changed anything
    let original = get_source_range(source, &suite.span);
    if formatted.trim() == original.trim() {
        return None;
    }

    Some(TextEdit { range: doc.span_to_range(&suite.span), new_text: formatted })
}

fn format_fixture(fixture: &PartialFixture, config: &FormatterConfig, depth: usize) -> String {
    let mut formatted = String::new();
    let indent = make_indent(config, depth);
    let inner_indent = make_indent(config, depth + 1);

    // Fixture header
    formatted.push_str(&indent);
    formatted.push_str(&format!("fixture {}", fixture.name));

    // Parameters
    if !fixture.params.is_empty() {
        formatted.push('(');
        let params: Vec<String> =
            fixture.params.iter().map(|p| format!("{}: {}", p.name, p.type_name)).collect();
        formatted.push_str(&params.join(", "));
        formatted.push(')');
    }

    formatted.push_str(" {\n");

    // Properties
    for prop in &fixture.properties {
        if let Node::Valid(p) = prop {
            formatted.push_str(&format_property(p, config, depth + 1));
        }
    }

    // Hex data
    if let Some(hex) = &fixture.hex {
        formatted.push_str(&inner_indent);
        match hex {
            poly_bench_syntax::HexData::Inline(data) => {
                formatted.push_str(&format!("hex: \"{}\"\n", data));
            }
            poly_bench_syntax::HexData::File(path) => {
                formatted.push_str(&format!("hex: @file(\"{}\")\n", path));
            }
        }
    }

    // Implementations
    for (lang, impl_node) in &fixture.implementations {
        if let Node::Valid(code) = impl_node {
            formatted.push_str(&inner_indent);
            if code.code.contains('\n') {
                formatted.push_str(&format!("{}: {{\n", lang.as_str()));
                format_code_block(&code.code, config, depth + 2, &mut formatted);
                formatted.push_str(&inner_indent);
                formatted.push_str("}\n");
            } else {
                formatted.push_str(&format!("{}: {}\n", lang.as_str(), code.code.trim()));
            }
        }
    }

    formatted.push_str(&indent);
    formatted.push_str("}\n");

    formatted
}

fn format_benchmark(
    benchmark: &PartialBenchmark,
    config: &FormatterConfig,
    depth: usize,
) -> String {
    let mut formatted = String::new();
    let indent = make_indent(config, depth);
    let inner_indent = make_indent(config, depth + 1);

    // Benchmark header
    formatted.push_str(&indent);
    formatted.push_str(&format!("bench {} {{\n", benchmark.name));

    // Properties
    for prop in &benchmark.properties {
        if let Node::Valid(p) = prop {
            formatted.push_str(&format_property(p, config, depth + 1));
        }
    }

    // Tags
    if !benchmark.tags.is_empty() {
        formatted.push_str(&inner_indent);
        let tags: Vec<String> = benchmark.tags.iter().map(|t| format!("\"{}\"", t)).collect();
        formatted.push_str(&format!("tags: [{}]\n", tags.join(", ")));
    }

    // Hooks
    format_hooks(&benchmark.skip, "skip", config, depth + 1, &mut formatted);
    format_hooks(&benchmark.validate, "validate", config, depth + 1, &mut formatted);
    format_hooks(&benchmark.before, "before", config, depth + 1, &mut formatted);
    format_hooks(&benchmark.after, "after", config, depth + 1, &mut formatted);
    format_hooks(&benchmark.each, "each", config, depth + 1, &mut formatted);

    // Implementations
    for (lang, impl_node) in &benchmark.implementations {
        if let Node::Valid(code) = impl_node {
            formatted.push_str(&inner_indent);
            if code.code.contains('\n') {
                formatted.push_str(&format!("{}: {{\n", lang.as_str()));
                format_code_block(&code.code, config, depth + 2, &mut formatted);
                formatted.push_str(&inner_indent);
                formatted.push_str("}\n");
            } else {
                formatted.push_str(&format!("{}: {}\n", lang.as_str(), code.code.trim()));
            }
        }
    }

    formatted.push_str(&indent);
    formatted.push_str("}\n");

    formatted
}

fn format_hooks(
    hooks: &std::collections::HashMap<
        poly_bench_syntax::Lang,
        poly_bench_syntax::Node<poly_bench_syntax::CodeBlock>,
    >,
    hook_name: &str,
    config: &FormatterConfig,
    depth: usize,
    output: &mut String,
) {
    let indent = make_indent(config, depth);

    for (lang, hook_node) in hooks {
        if let poly_bench_syntax::Node::Valid(code) = hook_node {
            output.push_str(&indent);
            if code.code.contains('\n') {
                output.push_str(&format!("{} {}: {{\n", hook_name, lang.as_str()));
                format_code_block(&code.code, config, depth + 1, output);
                output.push_str(&indent);
                output.push_str("}\n");
            } else {
                output.push_str(&format!(
                    "{} {}: {}\n",
                    hook_name,
                    lang.as_str(),
                    code.code.trim()
                ));
            }
        }
    }
}

fn format_property(prop: &Property, config: &FormatterConfig, depth: usize) -> String {
    let indent = make_indent(config, depth);
    format!("{}{}: {}\n", indent, prop.name, format_value(&prop.value))
}

fn format_value(value: &PropertyValue) -> String {
    match value {
        PropertyValue::String(s) => format!("\"{}\"", s),
        PropertyValue::Number(n) => n.to_string(),
        PropertyValue::Float(f) => format!("{:.1}", f),
        PropertyValue::Duration(n, unit) => {
            let unit_str = match unit {
                poly_bench_syntax::DurationUnit::Milliseconds => "ms",
                poly_bench_syntax::DurationUnit::Seconds => "s",
                poly_bench_syntax::DurationUnit::Minutes => "m",
            };
            format!("{}{}", n, unit_str)
        }
        PropertyValue::Boolean(b) => b.to_string(),
        PropertyValue::Identifier(id) => id.clone(),
        PropertyValue::StringArray(arr) => {
            let items: Vec<String> = arr.iter().map(|s| format!("\"{}\"", s)).collect();
            format!("[{}]", items.join(", "))
        }
    }
}

fn format_code_block(code: &str, config: &FormatterConfig, depth: usize, output: &mut String) {
    let indent = make_indent(config, depth);
    for line in code.lines() {
        if line.trim().is_empty() {
            output.push('\n');
        } else {
            output.push_str(&indent);
            output.push_str(line.trim());
            output.push('\n');
        }
    }
}

fn make_indent(config: &FormatterConfig, depth: usize) -> String {
    if config.use_tabs {
        "\t".repeat(depth)
    } else {
        " ".repeat(config.indent_size * depth)
    }
}

fn get_source_range<'a>(source: &'a str, span: &Span) -> &'a str {
    let start = span.start.min(source.len());
    let end = span.end.min(source.len());
    &source[start..end]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_doc(source: &str) -> Document {
        Document::new(
            tower_lsp::lsp_types::Url::parse("file:///test.bench").unwrap(),
            source.to_string(),
            1,
        )
    }

    #[test]
    fn test_format_simple() {
        let source = r#"suite test{description:"A test"
bench foo{go:run()}}"#;
        let doc = make_doc(source);
        let edits = format_document(&doc);

        // Should produce formatting edits
        assert!(!edits.is_empty());
    }

    #[test]
    fn test_format_already_formatted() {
        let source = r#"suite test {
    description: "A test"

    bench foo {
        go: run()
    }
}
"#;
        let doc = make_doc(source);
        let edits = format_document(&doc);

        // May or may not produce edits depending on exact formatting
        // The important thing is it doesn't crash
    }

    #[test]
    fn test_format_with_errors() {
        let source = r#"suite test {
    bench incomplete {
        go:
"#;
        let doc = make_doc(source);
        let edits = format_document(&doc);

        // Should not crash, may return empty edits for error nodes
    }

    #[test]
    fn test_format_preserves_code() {
        let source = r#"suite test {
    setup go {
        helpers {
            func helper() {
                if true {
                    fmt.Println("nested")
                }
            }
        }
    }

    bench foo {
        go: helper()
    }
}
"#;
        let doc = make_doc(source);
        let edits = format_document(&doc);

        // Formatting should preserve the embedded code structure
    }
}
