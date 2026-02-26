//! C embedded code diagnostics via clangd.

use std::path::Path;

use poly_bench_lsp_traits::{
    EmbeddedDiagnostic, EmbeddedDiagnosticContext, EmbeddedDiagnosticProvider,
    EmbeddedDiagnosticSetup, VirtualFile,
};
use poly_bench_syntax::Lang;
use tower_lsp::lsp_types::DiagnosticSeverity;

pub(crate) struct CEmbeddedDiagnosticProvider;

impl EmbeddedDiagnosticProvider for CEmbeddedDiagnosticProvider {
    fn check_blocks(
        &self,
        virtual_file: &dyn VirtualFile,
        ctx: &dyn EmbeddedDiagnosticContext,
    ) -> Vec<EmbeddedDiagnostic> {
        let mut diagnostics = Vec::new();

        let module_root = Path::new(virtual_file.path())
            .parent()
            .and_then(|p| p.parent())
            .map(|p| p.to_string_lossy().to_string());

        let module_root = match module_root {
            Some(r) => r,
            None => return diagnostics,
        };

        let client = match ctx.get_lsp_client(poly_bench_dsl::Lang::C, &module_root) {
            Some(c) => c,
            None => return diagnostics,
        };

        if let Err(e) =
            client.did_change(virtual_file.uri(), virtual_file.content(), virtual_file.version())
        {
            tracing::warn!("[c-diagnostics] failed to sync virtual file: {}", e);
            return diagnostics;
        }

        match client.request_diagnostics(virtual_file.uri()) {
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
                tracing::debug!("[c-diagnostics] request failed: {}", e);
            }
        }

        diagnostics
    }

    fn language(&self) -> Lang {
        Lang::C
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

pub(crate) static C_EMBEDDED_DIAGNOSTIC_PROVIDER: CEmbeddedDiagnosticProvider =
    CEmbeddedDiagnosticProvider;

pub(crate) struct CEmbeddedDiagnosticSetup;

impl EmbeddedDiagnosticSetup for CEmbeddedDiagnosticSetup {
    fn lang(&self) -> Lang {
        Lang::C
    }

    fn prepare(&self, module_root: &str, ctx: &dyn EmbeddedDiagnosticContext) {
        ctx.ensure_ready(Lang::C, module_root);
    }
}

pub(crate) static C_EMBEDDED_DIAGNOSTIC_SETUP: CEmbeddedDiagnosticSetup = CEmbeddedDiagnosticSetup;
