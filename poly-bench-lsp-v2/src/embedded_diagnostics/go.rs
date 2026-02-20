//! Go embedded code diagnostics via gopls
//!
//! This module provides diagnostics for embedded Go code by communicating
//! with gopls (the Go language server).

use crate::{
    gopls_client::get_gopls_client,
    virtual_files::{VirtualFile, VirtualGoFile},
};
use tower_lsp::lsp_types::DiagnosticSeverity;

use super::EmbeddedDiagnostic;

/// Check Go blocks using gopls
pub fn check_go_blocks(virtual_file: &VirtualGoFile) -> Vec<EmbeddedDiagnostic> {
    let mut diagnostics = Vec::new();

    // Get the gopls client
    let client = match get_gopls_client() {
        Some(c) => c,
        None => {
            tracing::debug!("[go-diagnostics] gopls client not available");
            return diagnostics;
        }
    };

    // Ensure the virtual file is opened in gopls
    let uri = virtual_file.uri();
    let content = virtual_file.content();
    let version = virtual_file.version();

    if let Err(e) = client.did_change(uri, content, version) {
        tracing::warn!("[go-diagnostics] Failed to update file in gopls: {}", e);
        return diagnostics;
    }

    // Request diagnostics from gopls
    match client.request_diagnostics(uri) {
        Ok(lsp_diags) => {
            for diag in lsp_diags {
                diagnostics.push(EmbeddedDiagnostic {
                    message: diag.message,
                    severity: severity_from_lsp(diag.severity),
                    virtual_line: diag.start_line,
                    virtual_character: diag.start_character,
                    length: calculate_length(
                        diag.start_line,
                        diag.start_character,
                        diag.end_line,
                        diag.end_character,
                    ),
                    code: diag.code,
                });
            }
        }
        Err(e) => {
            tracing::debug!("[go-diagnostics] Failed to get diagnostics from gopls: {}", e);
        }
    }

    diagnostics
}

/// Convert LSP severity number to DiagnosticSeverity
fn severity_from_lsp(severity: u32) -> DiagnosticSeverity {
    match severity {
        1 => DiagnosticSeverity::ERROR,
        2 => DiagnosticSeverity::WARNING,
        3 => DiagnosticSeverity::INFORMATION,
        4 => DiagnosticSeverity::HINT,
        _ => DiagnosticSeverity::ERROR,
    }
}

/// Calculate the length of a diagnostic range
/// For simplicity, if multi-line, just use the end character
fn calculate_length(start_line: u32, start_char: u32, end_line: u32, end_char: u32) -> u32 {
    if start_line == end_line {
        end_char.saturating_sub(start_char).max(1)
    } else {
        // Multi-line diagnostic - just highlight to end of first line
        // This is a simplification; could be improved
        10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_from_lsp() {
        assert_eq!(severity_from_lsp(1), DiagnosticSeverity::ERROR);
        assert_eq!(severity_from_lsp(2), DiagnosticSeverity::WARNING);
        assert_eq!(severity_from_lsp(3), DiagnosticSeverity::INFORMATION);
        assert_eq!(severity_from_lsp(4), DiagnosticSeverity::HINT);
    }

    #[test]
    fn test_calculate_length() {
        assert_eq!(calculate_length(0, 0, 0, 5), 5);
        assert_eq!(calculate_length(0, 5, 0, 10), 5);
        assert_eq!(calculate_length(0, 0, 1, 5), 10); // Multi-line
    }
}
