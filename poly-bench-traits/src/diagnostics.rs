//! Embedded diagnostic types and traits

use std::sync::Arc;
use tower_lsp::lsp_types::{DiagnosticSeverity, Hover};

use crate::virtual_file::VirtualFile;

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

/// Trait for LSP client used by diagnostic and hover providers (Go uses gopls)
pub trait EmbeddedLspClient: Send + Sync {
    fn did_change(&self, uri: &str, content: &str, version: i32) -> Result<(), String>;
    fn request_diagnostics(&self, uri: &str) -> Result<Vec<LspDiagnostic>, String> {
        let _ = (uri,);
        Ok(Vec::new())
    }
    fn hover(&self, uri: &str, line: u32, character: u32) -> Result<Option<Hover>, String>;
}

/// Diagnostic from LSP client (simplified)
#[derive(Debug, Clone)]
pub struct LspDiagnostic {
    pub message: String,
    pub severity: u32,
    pub start_line: u32,
    pub start_character: u32,
    pub end_line: u32,
    pub end_character: u32,
    pub code: Option<String>,
}

/// Context injected by LSP for diagnostic providers that need LSP clients
pub trait EmbeddedDiagnosticContext: Send + Sync {
    /// Get the LSP client for a language (init if needed). Uses registry; works for any registered
    /// runtime.
    fn get_lsp_client(
        &self,
        lang: poly_bench_dsl::Lang,
        module_root: &str,
    ) -> Option<Arc<dyn EmbeddedLspClient>>;
    fn ensure_tsconfig(&self, module_root: &str);
    /// Called before checking; e.g. init gopls, ensure tsconfig, etc.
    fn ensure_ready(&self, lang: poly_bench_syntax::Lang, module_root: &str);
}

/// Trait for language-specific diagnostic providers
pub trait EmbeddedDiagnosticProvider: Send + Sync {
    fn check_blocks(
        &self,
        virtual_file: &dyn VirtualFile,
        ctx: &dyn EmbeddedDiagnosticContext,
    ) -> Vec<EmbeddedDiagnostic>;
    fn language(&self) -> poly_bench_syntax::Lang;
}

/// Trait for language-specific setup before running diagnostics
pub trait EmbeddedDiagnosticSetup: Send + Sync {
    fn lang(&self) -> poly_bench_syntax::Lang;
    fn prepare(&self, module_root: &str, ctx: &dyn EmbeddedDiagnosticContext);
    /// Language-specific environment setup (tsconfig, src dir, etc.). Default: no-op.
    fn prepare_environment(&self, _module_root: &str) {}
}
