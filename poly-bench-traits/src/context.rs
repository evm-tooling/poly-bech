//! Context traits for LSP to inject clients and resources into runtime providers

use std::sync::Arc;

use poly_bench_syntax::Lang;

use crate::{diagnostics::EmbeddedLspClient, virtual_file::VirtualFile};

/// Context injected by LSP for hover providers
/// Provides virtual file access and LSP clients
pub trait EmbeddedHoverContext: Send + Sync {
    /// Get or create virtual file for the given language
    fn get_virtual_file(&self, lang: Lang) -> Option<Arc<dyn VirtualFile>>;
    /// Get the LSP client for a language (init if needed). Uses registry; works for any registered
    /// runtime.
    fn get_lsp_client(
        &self,
        lang: poly_bench_dsl::Lang,
        module_root: &str,
    ) -> Option<Arc<dyn EmbeddedLspClient>>;
    fn byte_to_position(&self, offset: usize) -> (u32, u32);
    fn bench_offset(&self) -> usize;
    fn module_root(&self) -> &str;
}
