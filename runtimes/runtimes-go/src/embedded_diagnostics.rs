//! Go embedded code diagnostics via gopls

use std::path::Path;

use poly_bench_lsp_traits::{
    EmbeddedDiagnostic, EmbeddedDiagnosticContext, EmbeddedDiagnosticProvider,
    EmbeddedDiagnosticSetup, VirtualFile,
};
use poly_bench_syntax::Lang;
use tower_lsp::lsp_types::DiagnosticSeverity;

/// Go embedded diagnostic provider
pub(crate) struct GoEmbeddedDiagnosticProvider;

impl EmbeddedDiagnosticProvider for GoEmbeddedDiagnosticProvider {
    fn check_blocks(
        &self,
        virtual_file: &dyn VirtualFile,
        ctx: &dyn EmbeddedDiagnosticContext,
    ) -> Vec<EmbeddedDiagnostic> {
        let mut diagnostics = Vec::new();

        // Derive module root from virtual file path: {module_root}/.lsp_virtual/virtual_xxx.go
        let module_root = Path::new(virtual_file.path())
            .parent() // .lsp_virtual
            .and_then(|p| p.parent()) // module root
            .map(|p| p.to_string_lossy().to_string());

        let module_root = match module_root {
            Some(r) => r,
            None => {
                tracing::debug!("[go-diagnostics] Could not derive module root from path");
                return diagnostics;
            }
        };

        let client = match ctx.get_lsp_client(poly_bench_dsl::Lang::Go, &module_root) {
            Some(c) => c,
            None => {
                tracing::debug!("[go-diagnostics] gopls client not available");
                return diagnostics;
            }
        };

        let uri = virtual_file.uri();
        let content = virtual_file.content();
        let version = virtual_file.version();

        if let Err(e) = client.did_change(uri, content, version) {
            tracing::warn!("[go-diagnostics] Failed to update file in gopls: {}", e);
            return diagnostics;
        }

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

    fn language(&self) -> Lang {
        Lang::Go
    }
}

fn severity_from_lsp(severity: u32) -> DiagnosticSeverity {
    match severity {
        1 => DiagnosticSeverity::ERROR,
        2 => DiagnosticSeverity::WARNING,
        3 => DiagnosticSeverity::INFORMATION,
        4 => DiagnosticSeverity::HINT,
        _ => DiagnosticSeverity::ERROR,
    }
}

fn calculate_length(start_line: u32, start_char: u32, end_line: u32, end_char: u32) -> u32 {
    if start_line == end_line {
        end_char.saturating_sub(start_char).max(1)
    } else {
        10
    }
}

pub(crate) static GO_EMBEDDED_DIAGNOSTIC_PROVIDER: GoEmbeddedDiagnosticProvider =
    GoEmbeddedDiagnosticProvider;

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
        assert_eq!(calculate_length(0, 0, 1, 5), 10);
    }
}

/// Go embedded diagnostic setup - delegates to context
pub(crate) struct GoEmbeddedDiagnosticSetup;

impl EmbeddedDiagnosticSetup for GoEmbeddedDiagnosticSetup {
    fn lang(&self) -> Lang {
        Lang::Go
    }
    fn prepare(&self, module_root: &str, ctx: &dyn EmbeddedDiagnosticContext) {
        ctx.ensure_ready(Lang::Go, module_root);
    }
}

pub(crate) static GO_EMBEDDED_DIAGNOSTIC_SETUP: GoEmbeddedDiagnosticSetup =
    GoEmbeddedDiagnosticSetup;
