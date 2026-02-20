//! TypeScript embedded code diagnostics via typescript-language-server
//!
//! This module provides diagnostics for embedded TypeScript code by communicating
//! with typescript-language-server (tsserver).

use crate::{
    tsserver_client::get_tsserver_client,
    virtual_files::{VirtualFile, VirtualTsFile},
};
use tower_lsp::lsp_types::DiagnosticSeverity;

use super::EmbeddedDiagnostic;

/// Check TypeScript blocks using tsserver
pub fn check_ts_blocks(virtual_file: &VirtualTsFile) -> Vec<EmbeddedDiagnostic> {
    let mut diagnostics = Vec::new();

    // Get the tsserver client
    let client = match get_tsserver_client() {
        Some(c) => c,
        None => {
            tracing::debug!("[ts-diagnostics] tsserver client not available");
            return diagnostics;
        }
    };

    // Ensure the virtual file is opened in tsserver
    let uri = virtual_file.uri();
    let content = virtual_file.content();
    let version = virtual_file.version();

    if let Err(e) = client.did_change(uri, content, version) {
        tracing::warn!("[ts-diagnostics] Failed to update file in tsserver: {}", e);
        return diagnostics;
    }

    // Request diagnostics from tsserver
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
            tracing::debug!("[ts-diagnostics] Failed to get diagnostics from tsserver: {}", e);
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
fn calculate_length(start_line: u32, start_char: u32, end_line: u32, end_char: u32) -> u32 {
    if start_line == end_line {
        end_char.saturating_sub(start_char).max(1)
    } else {
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
    }
}
