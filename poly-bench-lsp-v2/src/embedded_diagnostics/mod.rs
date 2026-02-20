//! Embedded language diagnostics
//!
//! This module provides diagnostics for embedded code blocks (Go, TypeScript, Rust)
//! by communicating with external language servers (gopls, tsserver, rust-analyzer).
//!
//! Architecture:
//! - Each language has its own provider implementing `EmbeddedDiagnosticProvider`
//! - Providers use virtual files to communicate with language servers
//! - Position mappings translate between .bench file and virtual file positions

pub mod go;
pub mod rust;
pub mod typescript;

use crate::{
    document::Document,
    embedded::{extract_embedded_blocks, EmbeddedBlock},
    gopls_client::init_gopls_client,
    rust_analyzer_client::init_rust_analyzer_client,
    tsserver_client::init_tsserver_client,
    virtual_files::{VirtualFile, VirtualGoFile, VirtualRustFile, VirtualTsFile},
};
use poly_bench_syntax::{Lang, Node};
use std::collections::{HashMap, HashSet};
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range};

/// Result of checking embedded code
#[derive(Debug, Clone)]
pub struct EmbeddedDiagnostic {
    /// The diagnostic message
    pub message: String,
    /// Severity level
    pub severity: DiagnosticSeverity,
    /// Position in the virtual file (line, character)
    pub virtual_line: u32,
    pub virtual_character: u32,
    /// Length of the diagnostic range
    pub length: u32,
    /// Diagnostic code (optional)
    pub code: Option<String>,
}

/// Trait for language-specific diagnostic providers
#[allow(dead_code)]
pub trait EmbeddedDiagnosticProvider: Send + Sync {
    /// Check embedded blocks and return diagnostics
    fn check_blocks(&self, blocks: &[EmbeddedBlock], bench_uri: &str) -> Vec<EmbeddedDiagnostic>;

    /// Get the language this provider handles
    fn language(&self) -> Lang;
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

    // Check Go blocks
    if let Some(go_blocks) = blocks_by_lang.get(&Lang::Go) {
        let go_mod_root = find_module_root(&bench_path, "go.mod")
            .or_else(|| find_polybench_runtime(&bench_path, "go"))
            .unwrap_or_else(|| bench_path.clone());

        // Initialize gopls client if not already done
        let _ = init_gopls_client(&go_mod_root);

        let virtual_file =
            VirtualGoFile::from_blocks(bench_uri, &bench_path, &go_mod_root, go_blocks, 1);
        let diagnostics = go::check_go_blocks(&virtual_file);
        let filtered = filter_fixture_diagnostics(diagnostics, &fixture_names);
        all_diagnostics.extend(map_diagnostics_to_bench(filtered, &virtual_file, doc, Lang::Go));
    }

    // Check TypeScript blocks
    if let Some(ts_blocks) = blocks_by_lang.get(&Lang::TypeScript) {
        let ts_module_root = find_module_root(&bench_path, "package.json")
            .or_else(|| find_polybench_runtime(&bench_path, "ts"))
            .unwrap_or_else(|| bench_path.clone());

        // Initialize tsserver client if not already done
        let _ = init_tsserver_client(&ts_module_root);

        let virtual_file =
            VirtualTsFile::from_blocks(bench_uri, &bench_path, &ts_module_root, ts_blocks, 1);
        let diagnostics = typescript::check_ts_blocks(&virtual_file);
        let filtered = filter_fixture_diagnostics(diagnostics, &fixture_names);
        all_diagnostics.extend(map_diagnostics_to_bench(
            filtered,
            &virtual_file,
            doc,
            Lang::TypeScript,
        ));
    }

    // Check Rust blocks
    if let Some(rust_blocks) = blocks_by_lang.get(&Lang::Rust) {
        let rust_project_root = find_module_root(&bench_path, "Cargo.toml")
            .or_else(|| find_polybench_runtime(&bench_path, "rust"))
            .unwrap_or_else(|| bench_path.clone());

        // Initialize rust-analyzer client if not already done
        let _ = init_rust_analyzer_client(&rust_project_root);

        let virtual_file = VirtualRustFile::from_blocks(
            bench_uri,
            &bench_path,
            &rust_project_root,
            rust_blocks,
            1,
        );
        let diagnostics = rust::check_rust_blocks(&virtual_file);
        let filtered = filter_fixture_diagnostics(diagnostics, &fixture_names);
        all_diagnostics.extend(map_diagnostics_to_bench(filtered, &virtual_file, doc, Lang::Rust));
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

/// Filter out diagnostics that are about fixture references (false positives)
fn filter_fixture_diagnostics(
    diagnostics: Vec<EmbeddedDiagnostic>,
    fixture_names: &HashSet<String>,
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
            true
        })
        .collect()
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

/// Find a module root by walking up from the given path looking for a marker file
fn find_module_root(start_path: &str, marker_file: &str) -> Option<String> {
    let mut current = std::path::Path::new(start_path);

    // If start_path is a file, start from its parent directory
    if current.is_file() {
        current = current.parent()?;
    }

    loop {
        if current.join(marker_file).exists() {
            return Some(current.to_string_lossy().to_string());
        }
        current = current.parent()?;
    }
}

/// Map embedded diagnostics back to .bench file positions
fn map_diagnostics_to_bench<V: VirtualFile>(
    diagnostics: Vec<EmbeddedDiagnostic>,
    virtual_file: &V,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offset_to_line_col() {
        let source = "line1\nline2\nline3";
        assert_eq!(offset_to_line_col(source, 0), (0, 0));
        assert_eq!(offset_to_line_col(source, 5), (0, 5));
        assert_eq!(offset_to_line_col(source, 6), (1, 0));
        assert_eq!(offset_to_line_col(source, 12), (2, 0));
    }
}
