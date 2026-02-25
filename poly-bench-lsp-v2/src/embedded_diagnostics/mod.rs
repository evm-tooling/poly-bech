//! Embedded language diagnostics
//!
//! This module provides diagnostics for embedded code blocks (Go, TypeScript, Rust, Python)
//! via runtime-registered providers. Each runtime implements EmbeddedDiagnosticProvider
//! and EmbeddedDiagnosticSetup; position mappings translate between .bench file and virtual file.

use crate::{
    document::Document,
    embedded::{extract_embedded_blocks, EmbeddedBlock},
    embedded_diagnostic_context::LspEmbeddedDiagnosticContext,
};
use poly_bench_lsp_traits::{
    syntax_lang_to_dsl, EmbeddedDiagnosticContext, EmbeddedDiagnosticProvider,
    EmbeddedDiagnosticSetup, VirtualFile, VirtualFileParams,
};
use poly_bench_project::get_detector;
use poly_bench_runtime::{
    get_embedded_diagnostic_provider as get_registry_provider,
    get_embedded_diagnostic_setup as get_registry_setup, get_virtual_file_builder,
};
use poly_bench_syntax::{Lang, Node};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range};

/// Result of checking embedded code (re-export from traits)
pub use poly_bench_lsp_traits::EmbeddedDiagnostic;

fn get_embedded_diagnostic_provider(
    dsl_lang: poly_bench_dsl::Lang,
) -> Option<&'static dyn EmbeddedDiagnosticProvider> {
    get_registry_provider(dsl_lang)
}

fn get_embedded_diagnostic_setup(
    dsl_lang: poly_bench_dsl::Lang,
) -> Option<&'static dyn EmbeddedDiagnosticSetup> {
    get_registry_setup(dsl_lang)
}

/// Check all embedded code in a document and return diagnostics
pub fn check_embedded_code(doc: &Document) -> Vec<Diagnostic> {
    let mut all_diagnostics = Vec::new();

    // Extract embedded blocks
    let blocks = extract_embedded_blocks(&doc.partial_ast);
    if blocks.is_empty() {
        return all_diagnostics;
    }

    // Group blocks by language
    let mut blocks_by_lang: HashMap<Lang, Vec<&EmbeddedBlock>> = HashMap::new();
    for block in &blocks {
        blocks_by_lang.entry(block.lang).or_default().push(block);
    }

    let bench_uri = doc.uri.as_str();
    let bench_path =
        doc.uri.to_file_path().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();

    // Collect fixture names from the AST to filter out false positives
    let fixture_names = collect_fixture_names(&doc.partial_ast);
    // `std::anvil` injects ANVIL_RPC_URL at runtime; virtual lint files need this exception.
    let has_anvil_stdlib = has_stdlib_import(&doc.partial_ast, "anvil");

    // Convert fixture names to Vec<String> for passing to virtual file builders
    let fixture_names_vec: Vec<String> = fixture_names.iter().cloned().collect();

    // Check each language via registry
    for (lang, blocks) in &blocks_by_lang {
        let dsl_lang = syntax_lang_to_dsl(*lang);
        let Some(builder) = get_virtual_file_builder(dsl_lang) else {
            continue;
        };
        let Some(provider) = get_embedded_diagnostic_provider(dsl_lang) else {
            continue;
        };

        let module_root = find_module_root_for_lang(*lang, &bench_path);

        let ctx = LspEmbeddedDiagnosticContext::new(&module_root);

        // Language-specific setup via registry or context
        if let Some(setup) = get_embedded_diagnostic_setup(dsl_lang) {
            setup.prepare(&module_root, &ctx);
        } else {
            ctx.ensure_ready(*lang, &module_root);
        }

        let params = VirtualFileParams {
            bench_uri,
            bench_path: &bench_path,
            module_root: &module_root,
            blocks,
            fixture_names: &fixture_names_vec,
            version: 1,
        };

        let virtual_file = builder.build(params);
        write_virtual_file_to_disk(virtual_file.path(), virtual_file.content());

        let diagnostics = provider.check_blocks(virtual_file.as_ref(), &ctx);
        let filtered = filter_fixture_diagnostics(diagnostics, &fixture_names, has_anvil_stdlib);
        all_diagnostics.extend(map_diagnostics_to_bench(
            filtered,
            virtual_file.as_ref(),
            doc,
            *lang,
        ));
    }

    all_diagnostics
}

/// Collect all fixture names from the partial AST
fn collect_fixture_names(ast: &poly_bench_syntax::PartialFile) -> HashSet<String> {
    let mut names = HashSet::new();

    for suite_node in &ast.suites {
        if let Node::Valid(suite) = suite_node {
            for fixture_node in &suite.fixtures {
                if let Node::Valid(fixture) = fixture_node {
                    names.insert(fixture.name.clone());
                }
            }
        }
    }

    names
}

/// Check whether a stdlib module is imported via `use std::<module>`
fn has_stdlib_import(ast: &poly_bench_syntax::PartialFile, module: &str) -> bool {
    ast.use_stds.iter().any(|use_node| match use_node {
        Node::Valid(use_std) => use_std.module == module,
        _ => false,
    })
}

/// Filter out diagnostics that are about fixture references (false positives)
fn filter_fixture_diagnostics(
    diagnostics: Vec<EmbeddedDiagnostic>,
    fixture_names: &HashSet<String>,
    has_anvil_stdlib: bool,
) -> Vec<EmbeddedDiagnostic> {
    diagnostics
        .into_iter()
        .filter(|diag| {
            // Check if the diagnostic message mentions a fixture name as undefined
            // Common patterns from different language servers:
            // - Go (gopls): "undefined: name", "undeclared name: name"
            // - TypeScript: "Cannot find name 'name'"
            // - Rust: "cannot find value `name`", "not found in this scope"
            let msg = &diag.message;

            for fixture in fixture_names {
                // Go patterns
                if msg.contains(&format!("undefined: {}", fixture))
                    || msg.contains(&format!("undeclared name: {}", fixture))
                    // TypeScript patterns
                    || msg.contains(&format!("Cannot find name '{}'", fixture))
                    || msg.contains(&format!("Cannot find name \"{}\"", fixture))
                    // Rust patterns
                    || msg.contains(&format!("cannot find value `{}`", fixture))
                    || (msg.contains("not found in this scope") && msg.contains(fixture))
                    // Generic patterns - check if message is about the fixture being undefined
                    || (msg.to_lowercase().contains("undefined") && msg.contains(fixture))
                    || (msg.to_lowercase().contains("undeclared") && msg.contains(fixture))
                    || (msg.to_lowercase().contains("not defined") && msg.contains(fixture))
                {
                    tracing::debug!(
                        "[embedded-diagnostics] Filtering out fixture reference diagnostic: {}",
                        msg
                    );
                    return false;
                }
            }

            // `ANVIL_RPC_URL` is injected by std::anvil at runtime, but virtual lint files
            // don't contain runtime injection scaffolding.
            if has_anvil_stdlib &&
                (msg.contains("undefined: ANVIL_RPC_URL") ||
                    msg.contains("undeclared name: ANVIL_RPC_URL") ||
                    msg.contains("Cannot find name 'ANVIL_RPC_URL'") ||
                    msg.contains("Cannot find name \"ANVIL_RPC_URL\"") ||
                    msg.contains("cannot find value `ANVIL_RPC_URL`"))
            {
                tracing::debug!(
                    "[embedded-diagnostics] Filtering std::anvil ANVIL_RPC_URL diagnostic: {}",
                    msg
                );
                return false;
            }
            true
        })
        .collect()
}

/// Write a virtual file to disk, creating parent directories as needed
fn write_virtual_file_to_disk(path: &str, content: &str) {
    let path = Path::new(path);

    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            tracing::warn!(
                "[embedded-diagnostics] Failed to create directory {}: {}",
                parent.display(),
                e
            );
            return;
        }
    }

    // Write the file
    if let Err(e) = std::fs::write(path, content) {
        tracing::warn!(
            "[embedded-diagnostics] Failed to write virtual file {}: {}",
            path.display(),
            e
        );
    } else {
        tracing::debug!("[embedded-diagnostics] Wrote virtual file: {}", path.display());
    }
}

/// Ensure tsconfig.json exists (used by LspEmbeddedDiagnosticContext)
#[allow(dead_code)]
fn ensure_tsconfig(ts_module_root: &str) {
    let tsconfig_path = Path::new(ts_module_root).join("tsconfig.json");
    if !tsconfig_path.exists() {
        let tsconfig_content = r#"{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "moduleResolution": "node",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "noEmit": true
  }
}
"#;
        if let Err(e) = std::fs::write(&tsconfig_path, tsconfig_content) {
            tracing::warn!("[embedded-diagnostics] Failed to create tsconfig.json: {}", e);
        } else {
            tracing::debug!(
                "[embedded-diagnostics] Created tsconfig.json at {}",
                tsconfig_path.display()
            );
        }
    }
}

/// Find module root for a language (used by embedded diagnostics)
/// Uses poly-bench runtime layout first, then ProjectRootDetector
fn find_module_root_for_lang(lang: Lang, bench_path: &str) -> String {
    let dsl_lang = syntax_lang_to_dsl(lang);
    find_polybench_runtime(bench_path, lang.as_str())
        .or_else(|| find_module_root_via_detector(dsl_lang, bench_path))
        .unwrap_or_else(|| bench_path.to_string())
}

/// Find module root via ProjectRootDetector
fn find_module_root_via_detector(lang: poly_bench_dsl::Lang, start_path: &str) -> Option<String> {
    let detector = get_detector(lang)?;
    detector.detect(Path::new(start_path)).map(|p| p.to_string_lossy().to_string())
}

/// Find .polybench/runtime-env/{lang}/ directory
fn find_polybench_runtime(start_path: &str, lang: &str) -> Option<String> {
    let mut current = std::path::Path::new(start_path);

    if current.is_file() {
        current = current.parent()?;
    }

    loop {
        let runtime_dir = current.join(format!(".polybench/runtime-env/{}", lang));
        if runtime_dir.exists() {
            return Some(runtime_dir.to_string_lossy().to_string());
        }
        current = current.parent()?;
    }
}

/// Map embedded diagnostics back to .bench file positions
fn map_diagnostics_to_bench(
    diagnostics: Vec<EmbeddedDiagnostic>,
    virtual_file: &dyn VirtualFile,
    doc: &Document,
    lang: Lang,
) -> Vec<Diagnostic> {
    let mut result = Vec::new();

    for diag in diagnostics {
        // Try to map the position back to the .bench file
        if let Some(bench_offset) =
            virtual_file.virtual_to_bench(diag.virtual_line, diag.virtual_character)
        {
            // Convert byte offset to line/column in the .bench file
            let source = doc.source_text();
            let (line, col) = offset_to_line_col(&source, bench_offset);

            // Calculate end position
            let end_offset = bench_offset + diag.length as usize;
            let (end_line, end_col) = offset_to_line_col(&source, end_offset);

            result.push(Diagnostic {
                range: Range {
                    start: Position { line, character: col },
                    end: Position { line: end_line, character: end_col },
                },
                severity: Some(diag.severity),
                code: diag.code.map(NumberOrString::String),
                source: Some(format!("poly-bench/{}", lang.as_str())),
                message: diag.message,
                ..Default::default()
            });
        }
    }

    result
}

/// Convert a byte offset to line and column numbers
fn offset_to_line_col(source: &str, offset: usize) -> (u32, u32) {
    let mut line = 0u32;
    let mut col = 0u32;
    let mut current_offset = 0usize;

    for ch in source.chars() {
        if current_offset >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
        current_offset += ch.len_utf8();
    }

    (line, col)
}

/// Result of validating embedded code for pre-run checks
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// The language of the code with the error
    pub lang: Lang,
    /// Line number in the .bench file (1-indexed for display)
    pub line: u32,
    /// Column number in the .bench file (1-indexed for display)
    pub column: u32,
    /// The error message
    pub message: String,
    /// Severity of the error
    pub severity: DiagnosticSeverity,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] line {}:{}: {}", self.lang.as_str(), self.line, self.column, self.message)
    }
}

/// Validate all embedded code in a document and return any errors found
/// This is useful for pre-run validation to catch errors before benchmark execution
pub fn validate_all_embedded_code(doc: &Document) -> Vec<ValidationError> {
    let diagnostics = check_embedded_code(doc);

    diagnostics
        .into_iter()
        .filter(|d| d.severity == Some(DiagnosticSeverity::ERROR))
        .map(|d| {
            let lang = d
                .source
                .as_ref()
                .and_then(|s| s.strip_prefix("poly-bench/"))
                .and_then(|suffix| Lang::from_str(suffix))
                .unwrap_or(Lang::Go);

            ValidationError {
                lang,
                line: d.range.start.line + 1,
                column: d.range.start.character + 1,
                message: d.message,
                severity: d.severity.unwrap_or(DiagnosticSeverity::ERROR),
            }
        })
        .collect()
}

/// Check if a document has any embedded code errors
/// Returns true if there are errors that would cause runtime failures
pub fn has_embedded_errors(doc: &Document) -> bool {
    let errors = validate_all_embedded_code(doc);
    !errors.is_empty()
}

/// Format validation errors for display
pub fn format_validation_errors(errors: &[ValidationError]) -> String {
    if errors.is_empty() {
        return String::from("No embedded code errors found.");
    }

    let mut output = format!("Found {} embedded code error(s):\n", errors.len());

    for (i, error) in errors.iter().enumerate() {
        output.push_str(&format!("  {}. {}\n", i + 1, error));
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_offset_to_line_col() {
        let source = "line1\nline2\nline3";
        assert_eq!(offset_to_line_col(source, 0), (0, 0));
        assert_eq!(offset_to_line_col(source, 5), (0, 5));
        assert_eq!(offset_to_line_col(source, 6), (1, 0));
        assert_eq!(offset_to_line_col(source, 12), (2, 0));
    }

    #[test]
    fn test_filter_anvil_rpc_url_when_std_anvil_enabled() {
        let diagnostics = vec![EmbeddedDiagnostic {
            message: "undefined: ANVIL_RPC_URL".to_string(),
            severity: DiagnosticSeverity::ERROR,
            virtual_line: 0,
            virtual_character: 0,
            length: 10,
            code: Some("UndeclaredName".to_string()),
        }];
        let fixture_names = HashSet::new();

        let filtered = filter_fixture_diagnostics(diagnostics, &fixture_names, true);
        assert!(filtered.is_empty());
    }

    #[test]
    fn test_keep_anvil_rpc_url_without_std_anvil() {
        let diagnostics = vec![EmbeddedDiagnostic {
            message: "undefined: ANVIL_RPC_URL".to_string(),
            severity: DiagnosticSeverity::ERROR,
            virtual_line: 0,
            virtual_character: 0,
            length: 10,
            code: Some("UndeclaredName".to_string()),
        }];
        let fixture_names = HashSet::new();

        let filtered = filter_fixture_diagnostics(diagnostics, &fixture_names, false);
        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn test_filter_rust_anvil_rpc_url_when_std_anvil_enabled() {
        let diagnostics = vec![EmbeddedDiagnostic {
            message: "cannot find value `ANVIL_RPC_URL` in this scope".to_string(),
            severity: DiagnosticSeverity::ERROR,
            virtual_line: 0,
            virtual_character: 0,
            length: 10,
            code: Some("E0425".to_string()),
        }];
        let fixture_names = HashSet::new();

        let filtered = filter_fixture_diagnostics(diagnostics, &fixture_names, true);
        assert!(filtered.is_empty());
    }
}
