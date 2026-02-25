//! Error-tolerant diagnostics
//!
//! This module provides diagnostics from multiple sources:
//! 1. Syntax errors from Tree-sitter (always available)
//! 2. Semantic errors from partial AST validation
//! 3. Helper reference validation (function calls vs defined helpers)
//! 4. Embedded language errors (on save)

use crate::document::Document;
use poly_bench_syntax::{
    Lang, Node, PartialBenchmark, PartialFixture, PartialSuite, PropertyValue, StructuredSetup,
};
use std::collections::HashSet;
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

    let mut suite_has_iterations = false;
    let mut suite_has_target_time = false;
    let parsed_run_mode = suite.run_mode.clone();

    // Check for baseline language existence
    for prop in &suite.properties {
        if let Node::Valid(p) = prop {
            if matches!(p.name.as_str(), "suiteType" | "runMode" | "sameDataset") {
                diagnostics.push(Diagnostic {
                    range: doc.span_to_range(&p.span),
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: Some(NumberOrString::String("suite-semantics-in-header".to_string())),
                    source: Some("poly-bench".to_string()),
                    message:
                        "Suite semantics must be declared in header: declare suite <name> <performance|memory> <timeBased|iterationBased> sameDataset: <true|false>"
                            .to_string(),
                    ..Default::default()
                });
            }
            if p.name == "iterations" {
                suite_has_iterations = true;
            }
            if p.name == "targetTime" {
                suite_has_target_time = true;
            }
            if p.name == "mode" {
                diagnostics.push(Diagnostic {
                    range: doc.span_to_range(&p.span),
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: Some(NumberOrString::String("legacy-mode-removed".to_string())),
                    source: Some("poly-bench".to_string()),
                    message: "Property 'mode' is no longer supported. Use suite declaration run mode instead"
                        .to_string(),
                    ..Default::default()
                });
            }
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
                        // Require every benchmark to implement baseline
                        let lang_enum = lang_enum.unwrap();
                        for bench in &suite.benchmarks {
                            if let Node::Valid(bench_node) = bench {
                                if !bench_node.implementations.contains_key(&lang_enum) {
                                    diagnostics.push(Diagnostic {
                                        range: doc.span_to_range(&bench_node.span),
                                        severity: Some(DiagnosticSeverity::ERROR),
                                        code: Some(NumberOrString::String(
                                            "baseline-missing-in-benchmark".to_string(),
                                        )),
                                        source: Some("poly-bench".to_string()),
                                        message: format!(
                                            "Benchmark '{}' missing baseline language '{}'; baseline comparisons require every benchmark to implement the baseline",
                                            bench_node.name, lang
                                        ),
                                        ..Default::default()
                                    });
                                }
                            }
                        }
                    }
                }
            }
            if p.name == "fairness" {
                if let PropertyValue::String(mode) = &p.value {
                    if mode != "legacy" && mode != "strict" {
                        diagnostics.push(Diagnostic {
                            range: doc.span_to_range(&p.span),
                            severity: Some(DiagnosticSeverity::ERROR),
                            code: Some(NumberOrString::String("invalid-fairness-mode".to_string())),
                            source: Some("poly-bench".to_string()),
                            message: format!(
                                "Invalid fairness mode '{}'. Expected \"legacy\" or \"strict\"",
                                mode
                            ),
                            ..Default::default()
                        });
                    }
                }
            }
            if p.name == "asyncSamplingPolicy" {
                if let PropertyValue::String(policy) = &p.value {
                    if policy != "timeBudgeted" && policy != "fixedCap" {
                        diagnostics.push(Diagnostic {
                            range: doc.span_to_range(&p.span),
                            severity: Some(DiagnosticSeverity::ERROR),
                            code: Some(NumberOrString::String(
                                "invalid-async-sampling-policy".to_string(),
                            )),
                            source: Some("poly-bench".to_string()),
                            message: format!(
                                "Invalid asyncSamplingPolicy '{}'. Expected \"timeBudgeted\" or \"fixedCap\"",
                                policy
                            ),
                            ..Default::default()
                        });
                    }
                }
            }
            if matches!(p.name.as_str(), "fairnessSeed" | "asyncWarmupCap" | "asyncSampleCap") {
                if let PropertyValue::Number(value) = &p.value {
                    if *value < 0 {
                        diagnostics.push(Diagnostic {
                            range: doc.span_to_range(&p.span),
                            severity: Some(DiagnosticSeverity::ERROR),
                            code: Some(NumberOrString::String(
                                "negative-suite-property".to_string(),
                            )),
                            source: Some("poly-bench".to_string()),
                            message: format!("Property '{}' must be >= 0", p.name),
                            ..Default::default()
                        });
                    }
                }
            }
        }
    }

    match suite.suite_type.as_deref() {
        Some("memory") | Some("performance") => {}
        _ => {
            diagnostics.push(Diagnostic {
                range: doc.span_to_range(&suite.span),
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::String("missing-suite-type".to_string())),
                source: Some("poly-bench".to_string()),
                message:
                    "Missing/invalid suite type in declaration header (expected memory|performance)"
                        .to_string(),
                ..Default::default()
            });
        }
    }
    match suite.run_mode.as_deref() {
        Some("timeBased") | Some("iterationBased") => {}
        _ => {
            diagnostics.push(Diagnostic {
                range: doc.span_to_range(&suite.span),
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::String("missing-run-mode".to_string())),
                source: Some("poly-bench".to_string()),
                message:
                    "Missing/invalid run mode in declaration header (expected timeBased|iterationBased)"
                        .to_string(),
                ..Default::default()
            });
        }
    }
    if suite.same_dataset.is_none() {
        diagnostics.push(Diagnostic {
            range: doc.span_to_range(&suite.span),
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::String("missing-same-dataset".to_string())),
            source: Some("poly-bench".to_string()),
            message: "Missing sameDataset boolean in declaration header".to_string(),
            ..Default::default()
        });
    }
    if let Some(run_mode) = parsed_run_mode {
        if run_mode == "timeBased" && suite_has_iterations {
            diagnostics.push(Diagnostic {
                range: doc.span_to_range(&suite.span),
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::String("suite-iterations-invalid".to_string())),
                source: Some("poly-bench".to_string()),
                message: "iterations is invalid when run mode is timeBased".to_string(),
                ..Default::default()
            });
        }
        if run_mode == "iterationBased" && suite_has_target_time {
            diagnostics.push(Diagnostic {
                range: doc.span_to_range(&suite.span),
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::String("suite-target-time-invalid".to_string())),
                source: Some("poly-bench".to_string()),
                message: "targetTime is invalid when run mode is iterationBased".to_string(),
                ..Default::default()
            });
        }
    }

    // Line/bar charts are only valid for same-dataset suites.
    if let Some(Node::Valid(after_block)) = &suite.after_block {
        let has_line_or_bar = after_block.directives.iter().any(|d| {
            if let Node::Valid(chart) = d {
                chart.function == "drawLineChart" || chart.function == "drawBarChart"
            } else {
                false
            }
        });

        if has_line_or_bar {
            if suite.same_dataset != Some(true) {
                for directive in &after_block.directives {
                    if let Node::Valid(chart) = directive {
                        if chart.function == "drawLineChart" || chart.function == "drawBarChart" {
                            diagnostics.push(Diagnostic {
                                range: doc.span_to_range(&chart.span),
                                severity: Some(DiagnosticSeverity::ERROR),
                                code: Some(NumberOrString::String(
                                    "chart-requires-same-dataset".to_string(),
                                )),
                                source: Some("poly-bench".to_string()),
                                message:
                                    "drawLineChart/drawBarChart require suite declaration sameDataset: true"
                                        .to_string(),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
            if suite.benchmarks.len() < 2 {
                for directive in &after_block.directives {
                    if let Node::Valid(chart) = directive {
                        if chart.function == "drawLineChart" || chart.function == "drawBarChart" {
                            diagnostics.push(Diagnostic {
                                range: doc.span_to_range(&chart.span),
                                severity: Some(DiagnosticSeverity::ERROR),
                                code: Some(NumberOrString::String(
                                    "chart-requires-multiple-benchmarks".to_string(),
                                )),
                                source: Some("poly-bench".to_string()),
                                message: format!(
                                    "drawLineChart/drawBarChart require at least 2 benchmarks for meaningful comparison; suite has {}",
                                    suite.benchmarks.len()
                                ),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        }
    }

    // sameDataset: true consistency - fixture refs should match across benchmarks
    if suite.same_dataset == Some(true) && suite.benchmarks.len() >= 2 && !suite.fixtures.is_empty()
    {
        let fixture_names: Vec<String> = suite
            .fixtures
            .iter()
            .filter_map(|f| if let Node::Valid(fi) = f { Some(fi.name.clone()) } else { None })
            .collect();

        let mut bench_refs: Vec<(String, HashSet<String>)> = Vec::new();
        for bench in &suite.benchmarks {
            if let Node::Valid(b) = bench {
                let mut refs = HashSet::new();
                for impl_node in b.implementations.values() {
                    if let Node::Valid(code) = impl_node {
                        for name in &fixture_names {
                            if code.code.contains(name.as_str()) {
                                refs.insert(name.clone());
                            }
                        }
                    }
                }
                bench_refs.push((b.name.clone(), refs));
            }
        }

        if let Some((_, first_set)) = bench_refs.first() {
            for (bench_name, refs) in bench_refs.iter().skip(1) {
                if refs != first_set {
                    if let Some(Node::Valid(bench)) = suite.benchmarks.iter().find(|b| {
                        if let Node::Valid(b) = b {
                            b.name == *bench_name
                        } else {
                            false
                        }
                    }) {
                        diagnostics.push(Diagnostic {
                            range: doc.span_to_range(&bench.span),
                            severity: Some(DiagnosticSeverity::WARNING),
                            code: Some(NumberOrString::String(
                                "same-dataset-inconsistent-fixtures".to_string(),
                            )),
                            source: Some("poly-bench".to_string()),
                            message: format!(
                                "Benchmark '{}' may use different fixtures than other benchmarks; sameDataset: true expects all benchmarks to operate on the same dataset",
                                bench_name
                            ),
                            ..Default::default()
                        });
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
    if fixture.hex.is_none() &&
        fixture.data.is_none() &&
        fixture.implementations.is_empty() &&
        fixture.shape.is_none()
    {
        diagnostics.push(Diagnostic {
            range: doc.span_to_range(&fixture.span),
            severity: Some(DiagnosticSeverity::WARNING),
            code: Some(NumberOrString::String("empty-fixture".to_string())),
            source: Some("poly-bench".to_string()),
            message: format!("Fixture '{}' has no data or implementations", fixture.name),
            ..Default::default()
        });
    }

    if fixture.encoding.is_some() && fixture.data.is_none() {
        diagnostics.push(Diagnostic {
            range: doc.span_to_range(&fixture.span),
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::String("fixture-encoding-without-data".to_string())),
            source: Some("poly-bench".to_string()),
            message: format!(
                "Fixture '{}' sets encoding but has no data source. Add data: ...",
                fixture.name
            ),
            ..Default::default()
        });
    }

    if fixture.selector.is_some() && fixture.format.is_none() {
        diagnostics.push(Diagnostic {
            range: doc.span_to_range(&fixture.span),
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::String("fixture-selector-without-format".to_string())),
            source: Some("poly-bench".to_string()),
            message: format!(
                "Fixture '{}' sets selector without format. Add format: json|csv",
                fixture.name
            ),
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
    let suite_run_mode = suite.run_mode.as_deref();

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

    let mut has_benchmark_iterations = false;
    let mut has_benchmark_target_time = false;
    for prop in &benchmark.properties {
        if let Node::Valid(p) = prop {
            if p.name == "mode" {
                diagnostics.push(Diagnostic {
                    range: doc.span_to_range(&p.span),
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: Some(NumberOrString::String("legacy-mode-removed".to_string())),
                    source: Some("poly-bench".to_string()),
                    message: "Benchmark-level mode is no longer supported. Use suite declaration run mode"
                        .to_string(),
                    ..Default::default()
                });
            }
            if p.name == "iterations" {
                has_benchmark_iterations = true;
            }
            if p.name == "targetTime" {
                has_benchmark_target_time = true;
            }
        }
    }

    if suite_run_mode == Some("timeBased") && has_benchmark_iterations {
        diagnostics.push(Diagnostic {
            range: doc.span_to_range(&benchmark.span),
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::String("benchmark-iterations-invalid".to_string())),
            source: Some("poly-bench".to_string()),
            message: "Benchmark iterations is invalid when suite run mode is timeBased".to_string(),
            ..Default::default()
        });
    }
    if suite_run_mode == Some("iterationBased") && has_benchmark_target_time {
        diagnostics.push(Diagnostic {
            range: doc.span_to_range(&benchmark.span),
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::String("benchmark-target-time-invalid".to_string())),
            source: Some("poly-bench".to_string()),
            message: "Benchmark targetTime is invalid when suite run mode is iterationBased"
                .to_string(),
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

    // Collect fixture names for this suite (these are valid "function-like" references)
    let fixture_names: HashSet<String> =
        suite
            .fixtures
            .iter()
            .filter_map(|f| {
                if let Node::Valid(fixture) = f {
                    Some(fixture.name.clone())
                } else {
                    None
                }
            })
            .collect();

    // Validate function calls in implementations
    for (lang, impl_node) in &benchmark.implementations {
        if let Node::Valid(code) = impl_node {
            let code_text = &code.code;

            // Get defined helper functions for this language
            let helper_functions = if let Some(setup_node) = suite.setups.get(lang) {
                if let Node::Valid(setup) = setup_node {
                    extract_helper_functions(setup, lang)
                } else {
                    HashSet::new()
                }
            } else {
                HashSet::new()
            };

            // Extract function calls from the implementation
            let calls = extract_function_calls(code_text, lang);

            // Check each call against defined helpers and fixtures
            for call in calls {
                // Skip if it's a fixture reference
                if fixture_names.contains(&call.name) {
                    continue;
                }

                // Check if the function is defined in helpers
                if !helper_functions.contains(&call.name) {
                    // Calculate the position within the .bench file
                    let call_start = code.span.start + call.start_offset;
                    let call_end = code.span.start + call.end_offset;

                    let call_span = poly_bench_syntax::Span {
                        start: call_start,
                        end: call_end,
                        start_line: code.span.start_line,
                        start_col: code.span.start_col + call.start_offset,
                        end_line: code.span.start_line,
                        end_col: code.span.start_col + call.end_offset,
                    };

                    if helper_functions.is_empty() && !suite.setups.contains_key(lang) {
                        // No setup block at all - suggest adding one
                        diagnostics.push(Diagnostic {
                            range: doc.span_to_range(&call_span),
                            severity: Some(DiagnosticSeverity::ERROR),
                            code: Some(NumberOrString::String("undefined-function".to_string())),
                            source: Some("poly-bench".to_string()),
                            message: format!(
                                "Function '{}' is not defined. Add a 'setup {} {{ helpers {{ ... }} }}' block to define it.",
                                call.name,
                                lang.as_str()
                            ),
                            ..Default::default()
                        });
                    } else {
                        // Setup exists but function not found
                        diagnostics.push(Diagnostic {
                            range: doc.span_to_range(&call_span),
                            severity: Some(DiagnosticSeverity::ERROR),
                            code: Some(NumberOrString::String("undefined-function".to_string())),
                            source: Some("poly-bench".to_string()),
                            message: format!(
                                "Function '{}' is not defined in 'setup {} {{ helpers {{ ... }} }}'",
                                call.name,
                                lang.as_str()
                            ),
                            ..Default::default()
                        });
                    }
                }
            }
        }
    }
}

/// Extract function names defined in a helpers block for a given language
fn extract_helper_functions(setup: &StructuredSetup, lang: &Lang) -> HashSet<String> {
    let mut functions = HashSet::new();

    if let Some(helpers) = &setup.helpers {
        let code = &helpers.code;
        match lang {
            Lang::Go => extract_go_functions(code, &mut functions),
            Lang::TypeScript => extract_ts_functions(code, &mut functions),
            Lang::Rust => extract_rust_functions(code, &mut functions),
            Lang::Python => extract_python_functions(code, &mut functions),
        }
    }

    functions
}

/// Extract Go function names: `func funcName(`
fn extract_go_functions(code: &str, functions: &mut HashSet<String>) {
    for line in code.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("func ") {
            // Extract function name after "func "
            let rest = &trimmed[5..];
            // Skip receiver if present: func (r *Receiver) Name(
            let name_start = if rest.starts_with('(') {
                // Find closing paren of receiver
                if let Some(close_paren) = rest.find(')') {
                    close_paren + 1
                } else {
                    continue;
                }
            } else {
                0
            };

            let rest = rest[name_start..].trim_start();
            // Extract identifier until '('
            if let Some(paren_pos) = rest.find('(') {
                let name = rest[..paren_pos].trim();
                if !name.is_empty() && is_valid_identifier(name) {
                    functions.insert(name.to_string());
                }
            }
        }
    }
}

/// Extract TypeScript/JavaScript function names:
/// - `function funcName(`
/// - `const funcName =` (arrow functions)
/// - `let funcName =` (arrow functions)
/// - `var funcName =` (arrow functions)
fn extract_ts_functions(code: &str, functions: &mut HashSet<String>) {
    for line in code.lines() {
        let trimmed = line.trim();

        // function funcName(
        if trimmed.starts_with("function ") {
            let rest = &trimmed[9..];
            if let Some(paren_pos) = rest.find('(') {
                let name = rest[..paren_pos].trim();
                if !name.is_empty() && is_valid_identifier(name) {
                    functions.insert(name.to_string());
                }
            }
        }
        // const/let/var funcName = ... (likely arrow function or function expression)
        else if trimmed.starts_with("const ") ||
            trimmed.starts_with("let ") ||
            trimmed.starts_with("var ")
        {
            let keyword_len = if trimmed.starts_with("const ") {
                6
            } else if trimmed.starts_with("let ") {
                4
            } else {
                4
            };
            let rest = &trimmed[keyword_len..];

            // Look for pattern: name = ... where ... contains => or function
            if let Some(eq_pos) = rest.find('=') {
                let name = rest[..eq_pos].trim();
                let after_eq = rest[eq_pos + 1..].trim();

                // Check if it's a function (arrow or function expression)
                if (after_eq.contains("=>") || after_eq.starts_with("function")) &&
                    is_valid_identifier(name)
                {
                    functions.insert(name.to_string());
                }
            }
        }
        // async function funcName(
        else if trimmed.starts_with("async function ") {
            let rest = &trimmed[15..];
            if let Some(paren_pos) = rest.find('(') {
                let name = rest[..paren_pos].trim();
                if !name.is_empty() && is_valid_identifier(name) {
                    functions.insert(name.to_string());
                }
            }
        }
    }
}

/// Extract Rust function names: `fn func_name(`
fn extract_rust_functions(code: &str, functions: &mut HashSet<String>) {
    for line in code.lines() {
        let trimmed = line.trim();

        // Handle pub fn, async fn, pub async fn, etc.
        let fn_keyword_pos = trimmed.find("fn ");
        if let Some(pos) = fn_keyword_pos {
            // Make sure "fn" is at start or after pub/async/unsafe
            let before_fn = &trimmed[..pos];
            let valid_prefix = before_fn.is_empty() ||
                before_fn
                    .trim()
                    .split_whitespace()
                    .all(|word| matches!(word, "pub" | "async" | "unsafe" | "const" | "extern"));

            if valid_prefix {
                let rest = &trimmed[pos + 3..];
                // Extract until '(' or '<' (generics)
                let end_pos = rest.find(|c| c == '(' || c == '<').unwrap_or(rest.len());
                let name = rest[..end_pos].trim();
                if !name.is_empty() && is_valid_rust_identifier(name) {
                    functions.insert(name.to_string());
                }
            }
        }
    }
}

/// Extract Python function names: `def func_name(`
fn extract_python_functions(code: &str, functions: &mut HashSet<String>) {
    for line in code.lines() {
        let trimmed = line.trim();

        // def func_name( or async def func_name(
        let def_pos = if trimmed.starts_with("def ") {
            Some(4)
        } else if trimmed.starts_with("async def ") {
            Some(10)
        } else {
            None
        };

        if let Some(start) = def_pos {
            let rest = &trimmed[start..];
            if let Some(paren_pos) = rest.find('(') {
                let name = rest[..paren_pos].trim();
                if !name.is_empty() && is_valid_identifier(name) {
                    functions.insert(name.to_string());
                }
            }
        }
    }
}

/// Check if a string is a valid identifier (letters, digits, underscores, starting with
/// letter/underscore)
fn is_valid_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_alphanumeric() || c == '_')
}

/// Check if a string is a valid Rust identifier (allows snake_case)
fn is_valid_rust_identifier(s: &str) -> bool {
    is_valid_identifier(s)
}

/// A function call found in code, with its position
#[derive(Debug, Clone)]
struct FunctionCall {
    name: String,
    /// Byte offset within the code string where the function name starts
    start_offset: usize,
    /// Byte offset where the function name ends
    end_offset: usize,
}

/// Extract function calls from benchmark implementation code
/// Returns function names that look like custom helper calls (not builtins or method calls)
fn extract_function_calls(code: &str, lang: &Lang) -> Vec<FunctionCall> {
    let mut calls = Vec::new();

    // Language-specific builtins to ignore
    let builtins: HashSet<&str> = match lang {
        Lang::Go => [
            "len", "make", "append", "copy", "delete", "panic", "recover", "print", "println",
            "close", "cap", "new", "real", "imag", "complex", "error", "string", "int", "int8",
            "int16", "int32", "int64", "uint", "uint8", "uint16", "uint32", "uint64", "float32",
            "float64", "bool", "byte", "rune",
        ]
        .iter()
        .copied()
        .collect(),
        Lang::TypeScript => [
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
            "Date",
            "RegExp",
            "Error",
            "Map",
            "Set",
            "Promise",
            "Uint8Array",
            "Int32Array",
            "Float64Array",
            "ArrayBuffer",
            "DataView",
            "setTimeout",
            "setInterval",
            "clearTimeout",
            "clearInterval",
            "fetch",
            "require",
            "import",
        ]
        .iter()
        .copied()
        .collect(),
        Lang::Rust => [
            "vec",
            "println",
            "print",
            "format",
            "panic",
            "assert",
            "assert_eq",
            "assert_ne",
            "debug_assert",
            "todo",
            "unimplemented",
            "unreachable",
            "Some",
            "None",
            "Ok",
            "Err",
            "Box",
            "Rc",
            "Arc",
            "Vec",
            "String",
            "HashMap",
            "HashSet",
            "BTreeMap",
            "BTreeSet",
        ]
        .iter()
        .copied()
        .collect(),
        Lang::Python => [
            "print",
            "len",
            "range",
            "str",
            "int",
            "float",
            "bool",
            "list",
            "dict",
            "set",
            "tuple",
            "type",
            "isinstance",
            "issubclass",
            "hasattr",
            "getattr",
            "setattr",
            "delattr",
            "open",
            "input",
            "sorted",
            "reversed",
            "enumerate",
            "zip",
            "map",
            "filter",
            "sum",
            "min",
            "max",
            "abs",
            "round",
            "pow",
            "divmod",
            "hex",
            "oct",
            "bin",
            "ord",
            "chr",
            "repr",
            "format",
            "id",
            "hash",
            "callable",
            "iter",
            "next",
            "slice",
            "super",
            "staticmethod",
            "classmethod",
            "property",
        ]
        .iter()
        .copied()
        .collect(),
    };

    // Find function call patterns: identifier followed by (
    // We need to be careful to:
    // 1. Skip method calls (preceded by .)
    // 2. Skip type casts/constructors in some cases
    // 3. Handle Rust's macro calls (name!)

    let chars: Vec<char> = code.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Skip if we're inside a string literal
        if chars[i] == '"' || chars[i] == '\'' || chars[i] == '`' {
            let quote = chars[i];
            i += 1;
            while i < chars.len() {
                if chars[i] == '\\' && i + 1 < chars.len() {
                    i += 2; // Skip escaped character
                } else if chars[i] == quote {
                    i += 1;
                    break;
                } else {
                    i += 1;
                }
            }
            continue;
        }

        // Look for identifier start
        if chars[i].is_alphabetic() || chars[i] == '_' {
            let start = i;

            // Collect the identifier
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }

            let name: String = chars[start..i].iter().collect();

            // Skip whitespace
            while i < chars.len() && chars[i].is_whitespace() {
                i += 1;
            }

            // Check if followed by ( - this is a function call
            if i < chars.len() && chars[i] == '(' {
                // Check if this is NOT a method call (preceded by .)
                let is_method_call = start > 0 && {
                    let mut j = start - 1;
                    while j > 0 && chars[j].is_whitespace() {
                        j -= 1;
                    }
                    chars[j] == '.'
                };

                // Check if this is a Rust reference (&name)
                let is_rust_ref = *lang == Lang::Rust && start > 0 && chars[start - 1] == '&';

                if !is_method_call && !is_rust_ref && !builtins.contains(name.as_str()) {
                    // Calculate byte offsets
                    let byte_start: usize = chars[..start].iter().map(|c| c.len_utf8()).sum();
                    let byte_end: usize = chars[..i].iter().map(|c| c.len_utf8()).sum();

                    calls.push(FunctionCall {
                        name,
                        start_offset: byte_start,
                        end_offset: byte_end,
                    });
                }
            }
        } else {
            i += 1;
        }
    }

    calls
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
    #[ignore]
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

    fn has_code(diagnostics: &[Diagnostic], code: &str) -> bool {
        diagnostics.iter().any(|d| {
            d.code.as_ref().map_or(false, |c| match c {
                NumberOrString::String(s) => s == code,
                NumberOrString::Number(_) => false,
            })
        })
    }

    #[test]
    fn test_chart_requires_multiple_benchmarks() {
        let source = r#"
use std::charting

declare suite test performance timeBased sameDataset: true {
    targetTime: 2s
    bench foo {
        go: work()
        ts: work()
    }
    after {
        charting.drawLineChart(title: "Trend")
    }
}
"#;
        let doc = make_doc(source);
        let diagnostics = compute_diagnostics(&doc);
        assert!(has_code(&diagnostics, "chart-requires-multiple-benchmarks"));
    }

    #[test]
    fn test_baseline_missing_in_benchmark() {
        let source = r#"
declare suite test performance timeBased sameDataset: true {
    baseline: "go"
    targetTime: 2s
    bench foo {
        go: work()
    }
    bench bar {
        ts: work()
    }
}
"#;
        let doc = make_doc(source);
        let diagnostics = compute_diagnostics(&doc);
        assert!(has_code(&diagnostics, "baseline-missing-in-benchmark"));
    }

    #[test]
    fn test_same_dataset_inconsistent_fixtures() {
        let source = r#"
declare suite test performance timeBased sameDataset: true {
    targetTime: 2s
    fixture data1 {
        hex: "01020304"
    }
    fixture data2 {
        hex: "05060708"
    }
    bench foo {
        go: process(data1)
    }
    bench bar {
        go: process(data2)
    }
}
"#;
        let doc = make_doc(source);
        let diagnostics = compute_diagnostics(&doc);
        assert!(has_code(&diagnostics, "same-dataset-inconsistent-fixtures"));
    }
}
