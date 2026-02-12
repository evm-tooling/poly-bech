//! Diagnostics generation for the LSP
//!
//! This module converts parse errors and validation warnings/errors
//! into LSP diagnostics, including embedded Go/TypeScript code checking.

use poly_bench_dsl as dsl;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

use super::document::ParsedDocument;
use super::embedded::{check_embedded_blocks, extract_embedded_blocks, EmbeddedCheckResult, EmbeddedConfig};

/// Compute all diagnostics for a document
#[allow(dead_code)]
pub fn compute_diagnostics(doc: &ParsedDocument) -> Vec<Diagnostic> {
    compute_diagnostics_with_config(doc, &EmbeddedConfig::default()).diagnostics
}

/// Result from diagnostics computation
pub struct DiagnosticsResult {
    pub diagnostics: Vec<Diagnostic>,
    pub embedded_debug: Option<EmbeddedCheckResult>,
}

/// Compute diagnostics with embedded language configuration
pub fn compute_diagnostics_with_config(
    doc: &ParsedDocument,
    config: &EmbeddedConfig,
) -> DiagnosticsResult {
    let mut diagnostics = Vec::new();

    // Add parse error if parsing failed
    if let Some(ref error) = doc.parse_error {
        let range = if let Some(ref span) = error.span {
            doc.span_to_range(span)
        } else {
            // Default to start of file
            Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 1,
                },
            }
        };

        diagnostics.push(Diagnostic {
            range,
            severity: Some(DiagnosticSeverity::ERROR),
            code: None,
            code_description: None,
            source: Some("poly-bench".to_string()),
            message: error.message.clone(),
            related_information: None,
            tags: None,
            data: None,
        });

        return DiagnosticsResult {
            diagnostics,
            embedded_debug: None,
        };
    }

    // Run validation on each suite
    if let Some(ref ast) = doc.ast {
        for suite in &ast.suites {
            let result = dsl::validate_suite(suite);

            // Add validation errors
            for error in result.errors {
                let range = location_to_range(doc, &error.location, suite);
                diagnostics.push(Diagnostic {
                    range,
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: None,
                    code_description: None,
                    source: Some("poly-bench".to_string()),
                    message: error.message,
                    related_information: None,
                    tags: None,
                    data: None,
                });
            }

            // Add validation warnings
            for warning in result.warnings {
                let range = location_to_range(doc, &warning.location, suite);
                diagnostics.push(Diagnostic {
                    range,
                    severity: Some(DiagnosticSeverity::WARNING),
                    code: None,
                    code_description: None,
                    source: Some("poly-bench".to_string()),
                    message: warning.message,
                    related_information: None,
                    tags: None,
                    data: None,
                });
            }
        }
    }

    // Check embedded Go/TypeScript code
    let blocks = extract_embedded_blocks(doc);
    let embedded_result = check_embedded_blocks(doc, &blocks, config);
    diagnostics.extend(embedded_result.diagnostics.clone());

    DiagnosticsResult {
        diagnostics,
        embedded_debug: Some(embedded_result),
    }
}

/// Convert a validation location string to an LSP Range
fn location_to_range(
    doc: &ParsedDocument,
    location: &Option<String>,
    suite: &dsl::Suite,
) -> Range {
    if let Some(ref loc) = location {
        // Parse location format: "suite.name.bench.benchmark_name"
        let parts: Vec<&str> = loc.split('.').collect();

        if parts.len() >= 4 && parts[2] == "bench" {
            let bench_name = parts[3];
            if let Some(bench) = suite.benchmarks.iter().find(|b| b.name == bench_name) {
                return doc.span_to_range(&bench.span);
            }
        }

        if parts.len() >= 4 && parts[2] == "fixture" {
            let fixture_name = parts[3];
            if let Some(fixture) = suite.fixtures.iter().find(|f| f.name == fixture_name) {
                return doc.span_to_range(&fixture.span);
            }
        }

        if parts.len() >= 4 && parts[2] == "setup" {
            let lang_str = parts[3];
            if let Some(lang) = dsl::Lang::from_str(lang_str) {
                if let Some(setup) = suite.setups.get(&lang) {
                    return doc.span_to_range(&setup.span);
                }
            }
        }

        // Fall back to suite span
        return doc.span_to_range(&suite.span);
    }

    // Default to suite span
    doc.span_to_range(&suite.span)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_diagnostic() {
        let source = r#"suite test { unclosed"#;
        let doc = ParsedDocument::parse(source, "test.bench", 1);
        let diagnostics = compute_diagnostics(&doc);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].severity, Some(DiagnosticSeverity::ERROR));
    }

    #[test]
    fn test_validation_error_diagnostic() {
        let source = r#"
suite test {
    bench empty {
    }
}
"#;
        let doc = ParsedDocument::parse(source, "test.bench", 1);
        let diagnostics = compute_diagnostics(&doc);

        assert!(diagnostics.iter().any(|d| d.message.contains("no language implementations")));
    }
}
