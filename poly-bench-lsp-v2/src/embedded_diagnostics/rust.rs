//! Rust embedded code diagnostics via rust-analyzer
//!
//! This module provides diagnostics for embedded Rust code by communicating
//! with rust-analyzer.

use crate::{
    rust_analyzer_client::get_rust_analyzer_client,
    virtual_files::{VirtualFile, VirtualRustFile},
};
use tower_lsp::lsp_types::DiagnosticSeverity;

use super::EmbeddedDiagnostic;

/// Check Rust blocks using rust-analyzer
pub fn check_rust_blocks(virtual_file: &VirtualRustFile) -> Vec<EmbeddedDiagnostic> {
    let mut diagnostics = Vec::new();

    // Get the rust-analyzer client
    let client = match get_rust_analyzer_client() {
        Some(c) => c,
        None => {
            tracing::debug!("[rust-diagnostics] rust-analyzer client not available");
            return diagnostics;
        }
    };

    let uri = virtual_file.uri();
    let content = virtual_file.content();
    let version = virtual_file.version();

    tracing::debug!("[rust-diagnostics] Checking Rust file: {} (version {})", uri, version);

    // Open/update the file in rust-analyzer
    if let Err(e) = client.did_change(uri, content, version) {
        tracing::warn!("[rust-diagnostics] Failed to update file in rust-analyzer: {}", e);
        return diagnostics;
    }

    // Give rust-analyzer a moment to analyze the file
    // rust-analyzer needs more time than tsserver due to cargo/rustc integration
    std::thread::sleep(std::time::Duration::from_millis(200));

    // Request diagnostics from rust-analyzer
    match client.request_diagnostics(uri) {
        Ok(lsp_diags) => {
            tracing::debug!(
                "[rust-diagnostics] Received {} diagnostics from rust-analyzer",
                lsp_diags.len()
            );
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
            tracing::debug!(
                "[rust-diagnostics] Failed to get diagnostics from rust-analyzer: {}",
                e
            );
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
