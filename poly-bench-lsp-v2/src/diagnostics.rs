//! Error-tolerant diagnostics
//!
//! This module provides diagnostics from multiple sources:
//! 1. Syntax errors from Tree-sitter (always available)
//! 2. Semantic errors from partial AST validation
//! 3. Embedded language errors (on save)

use crate::document::Document;
use poly_bench_syntax::{
    Lang, Node, PartialBenchmark, PartialFixture, PartialSuite, PropertyValue,
};
use tower_lsp::lsp_types::*;

/// Compute diagnostics for a document
pub fn compute_diagnostics(doc: &Document) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // 1. Syntax errors from Tree-sitter
    collect_syntax_errors(&doc.tree, &doc.source_text(), &mut diagnostics);

    // 2. Semantic errors from partial AST
    validate_partial_ast(doc, &mut diagnostics);

    diagnostics
}

/// Collect syntax errors from the Tree-sitter tree
fn collect_syntax_errors(
    tree: &tree_sitter::Tree,
    source: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut cursor = tree.walk();
    collect_errors_recursive(&mut cursor, source, diagnostics);
}

fn collect_errors_recursive(
    cursor: &mut tree_sitter::TreeCursor,
    source: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let node = cursor.node();

    if node.is_error() {
        let start = node.start_position();
        let end = node.end_position();

        let text = node.utf8_text(source.as_bytes()).unwrap_or("<unknown>");
        let message = if text.len() > 30 {
            format!("Syntax error near: {}...", &text[..30])
        } else {
            format!("Syntax error near: {}", text)
        };

        diagnostics.push(Diagnostic {
            range: Range {
                start: Position { line: start.row as u32, character: start.column as u32 },
                end: Position { line: end.row as u32, character: end.column as u32 },
            },
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::String("syntax-error".to_string())),
            source: Some("poly-bench".to_string()),
            message,
            ..Default::default()
        });
    } else if node.is_missing() {
        let start = node.start_position();

        diagnostics.push(Diagnostic {
            range: Range {
                start: Position { line: start.row as u32, character: start.column as u32 },
                end: Position { line: start.row as u32, character: start.column as u32 + 1 },
            },
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::String("missing-token".to_string())),
            source: Some("poly-bench".to_string()),
            message: format!("Missing {}", node.kind()),
            ..Default::default()
        });
    }

    // Recurse into children
    if cursor.goto_first_child() {
        loop {
            collect_errors_recursive(cursor, source, diagnostics);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
}

/// Validate the partial AST for semantic errors
fn validate_partial_ast(doc: &Document, diagnostics: &mut Vec<Diagnostic>) {
    let ast = &doc.partial_ast;

    // Validate suites
    for suite in &ast.suites {
        if let Node::Valid(s) = suite {
            validate_suite(s, doc, diagnostics);
        } else if let Node::Error { span, message } = suite {
            diagnostics.push(Diagnostic {
                range: doc.span_to_range(span),
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::String("parse-error".to_string())),
                source: Some("poly-bench".to_string()),
                message: message.clone(),
                ..Default::default()
            });
        }
    }
}

fn validate_suite(suite: &PartialSuite, doc: &Document, diagnostics: &mut Vec<Diagnostic>) {
    // Check for empty suite
    if suite.benchmarks.is_empty() && suite.fixtures.is_empty() {
        diagnostics.push(Diagnostic {
            range: doc.span_to_range(&suite.span),
            severity: Some(DiagnosticSeverity::WARNING),
            code: Some(NumberOrString::String("empty-suite".to_string())),
            source: Some("poly-bench".to_string()),
            message: format!("Suite '{}' has no benchmarks or fixtures", suite.name),
            ..Default::default()
        });
    }

    // Validate fixtures
    for fixture in &suite.fixtures {
        if let Node::Valid(f) = fixture {
            validate_fixture(f, suite, doc, diagnostics);
        }
    }

    // Validate benchmarks
    for benchmark in &suite.benchmarks {
        if let Node::Valid(b) = benchmark {
            validate_benchmark(b, suite, doc, diagnostics);
        }
    }

    // Check for baseline language existence
    for prop in &suite.properties {
        if let Node::Valid(p) = prop {
            if p.name == "baseline" {
                if let PropertyValue::String(lang) = &p.value {
                    let lang_enum = Lang::from_str(lang.as_str());
                    if lang_enum.is_none() {
                        diagnostics.push(Diagnostic {
                            range: doc.span_to_range(&p.span),
                            severity: Some(DiagnosticSeverity::ERROR),
                            code: Some(NumberOrString::String("invalid-baseline".to_string())),
                            source: Some("poly-bench".to_string()),
                            message: format!("Unknown baseline language: '{}'", lang),
                            ..Default::default()
                        });
                    } else {
                        // Check if any benchmark implements this language
                        let lang_enum = lang_enum.unwrap();
                        let has_impl = suite.benchmarks.iter().any(|b| {
                            if let Node::Valid(bench) = b {
                                bench.implementations.contains_key(&lang_enum)
                            } else {
                                false
                            }
                        });

                        if !has_impl {
                            diagnostics.push(Diagnostic {
                                range: doc.span_to_range(&p.span),
                                severity: Some(DiagnosticSeverity::WARNING),
                                code: Some(NumberOrString::String("unused-baseline".to_string())),
                                source: Some("poly-bench".to_string()),
                                message: format!(
                                    "Baseline language '{}' is not implemented by any benchmark",
                                    lang
                                ),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        }
    }
}

fn validate_fixture(
    fixture: &PartialFixture,
    suite: &PartialSuite,
    doc: &Document,
    diagnostics: &mut Vec<Diagnostic>,
) {
    // Check for empty fixture
    if fixture.hex.is_none() && fixture.implementations.is_empty() && fixture.shape.is_none() {
        diagnostics.push(Diagnostic {
            range: doc.span_to_range(&fixture.span),
            severity: Some(DiagnosticSeverity::WARNING),
            code: Some(NumberOrString::String("empty-fixture".to_string())),
            source: Some("poly-bench".to_string()),
            message: format!("Fixture '{}' has no data or implementations", fixture.name),
            ..Default::default()
        });
    }

    // Check for unused fixture
    let fixture_used = suite.benchmarks.iter().any(|b| {
        if let Node::Valid(bench) = b {
            // Check if fixture name appears in any implementation code
            bench.implementations.values().any(|impl_node| {
                if let Node::Valid(code) = impl_node {
                    code.code.contains(&fixture.name)
                } else {
                    false
                }
            })
        } else {
            false
        }
    });

    if !fixture_used {
        diagnostics.push(Diagnostic {
            range: doc.span_to_range(&fixture.span),
            severity: Some(DiagnosticSeverity::HINT),
            code: Some(NumberOrString::String("unused-fixture".to_string())),
            source: Some("poly-bench".to_string()),
            message: format!("Fixture '{}' appears to be unused", fixture.name),
            tags: Some(vec![DiagnosticTag::UNNECESSARY]),
            ..Default::default()
        });
    }
}

fn validate_benchmark(
    benchmark: &PartialBenchmark,
    suite: &PartialSuite,
    doc: &Document,
    diagnostics: &mut Vec<Diagnostic>,
) {
    // Check for empty benchmark
    if benchmark.implementations.is_empty() {
        diagnostics.push(Diagnostic {
            range: doc.span_to_range(&benchmark.span),
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::String("empty-benchmark".to_string())),
            source: Some("poly-bench".to_string()),
            message: format!("Benchmark '{}' has no implementations", benchmark.name),
            ..Default::default()
        });
    }

    // Check that hooks have corresponding implementations
    for (lang, _) in &benchmark.before {
        if !benchmark.implementations.contains_key(lang) {
            diagnostics.push(Diagnostic {
                range: doc.span_to_range(&benchmark.span),
                severity: Some(DiagnosticSeverity::WARNING),
                code: Some(NumberOrString::String("hook-without-impl".to_string())),
                source: Some("poly-bench".to_string()),
                message: format!(
                    "Benchmark '{}' has a 'before' hook for {} but no {} implementation",
                    benchmark.name,
                    lang.as_str(),
                    lang.as_str()
                ),
                ..Default::default()
            });
        }
    }

    for (lang, _) in &benchmark.after {
        if !benchmark.implementations.contains_key(lang) {
            diagnostics.push(Diagnostic {
                range: doc.span_to_range(&benchmark.span),
                severity: Some(DiagnosticSeverity::WARNING),
                code: Some(NumberOrString::String("hook-without-impl".to_string())),
                source: Some("poly-bench".to_string()),
                message: format!(
                    "Benchmark '{}' has an 'after' hook for {} but no {} implementation",
                    benchmark.name,
                    lang.as_str(),
                    lang.as_str()
                ),
                ..Default::default()
            });
        }
    }

    // Check for missing setup when using helpers
    for (lang, impl_node) in &benchmark.implementations {
        if let Node::Valid(code) = impl_node {
            // Simple heuristic: if code calls a function that's not a builtin,
            // it might need a setup block
            if !suite.setups.contains_key(lang) {
                // Check if code looks like it's calling custom functions
                let code_text = &code.code;
                if looks_like_custom_function_call(code_text) {
                    diagnostics.push(Diagnostic {
                        range: doc.span_to_range(&code.span),
                        severity: Some(DiagnosticSeverity::HINT),
                        code: Some(NumberOrString::String("missing-setup".to_string())),
                        source: Some("poly-bench".to_string()),
                        message: format!(
                            "Consider adding a 'setup {}' block if '{}' uses custom helpers",
                            lang.as_str(),
                            benchmark.name
                        ),
                        ..Default::default()
                    });
                }
            }
        }
    }
}

fn looks_like_custom_function_call(code: &str) -> bool {
    // Simple heuristic: lowercase function calls that aren't common builtins
    let builtins = [
        "len",
        "make",
        "append",
        "copy",
        "delete",
        "panic",
        "recover",
        "print",
        "println",
        "close",
        "cap",
        "new",
        "real",
        "imag",
        "complex",
        // TypeScript/JS
        "console",
        "Math",
        "JSON",
        "Array",
        "Object",
        "String",
        "Number",
        "Boolean",
        "parseInt",
        "parseFloat",
        "isNaN",
        "isFinite",
    ];

    // Look for function call patterns
    let has_call = code.contains('(');
    if !has_call {
        return false;
    }

    // Check if it's not just builtin calls
    for builtin in &builtins {
        if code.contains(builtin) {
            return false;
        }
    }

    // If it has a call and no builtins, it might be custom
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_doc(source: &str) -> Document {
        Document::new(Url::parse("file:///test.bench").unwrap(), source.to_string(), 1)
    }

    #[test]
    fn test_syntax_error_diagnostics() {
        let source = r#"suite test {
    bench incomplete {
        go:
"#;
        let doc = make_doc(source);
        let diagnostics = compute_diagnostics(&doc);

        // Should have syntax errors
        assert!(diagnostics.iter().any(|d| d.severity == Some(DiagnosticSeverity::ERROR)));
    }

    #[test]
    fn test_empty_suite_warning() {
        let source = r#"suite empty {
    description: "Empty suite"
}"#;
        let doc = make_doc(source);
        let diagnostics = compute_diagnostics(&doc);

        // Should have warning about empty suite
        assert!(diagnostics.iter().any(|d| {
            d.severity == Some(DiagnosticSeverity::WARNING) && d.message.contains("no benchmarks")
        }));
    }

    #[test]
    fn test_empty_benchmark_error() {
        let source = r#"suite test {
    bench empty {
    }
}"#;
        let doc = make_doc(source);
        let diagnostics = compute_diagnostics(&doc);

        // Should have error about empty benchmark
        assert!(diagnostics.iter().any(|d| {
            d.severity == Some(DiagnosticSeverity::ERROR) &&
                d.message.contains("no implementations")
        }));
    }

    #[test]
    fn test_no_diagnostics_for_valid() {
        let source = r#"suite test {
    description: "Valid suite"
    
    bench foo {
        go: doSomething()
    }
}"#;
        let doc = make_doc(source);
        let diagnostics = compute_diagnostics(&doc);

        // Should have no errors (may have hints)
        assert!(!diagnostics.iter().any(|d| d.severity == Some(DiagnosticSeverity::ERROR)));
    }
}
