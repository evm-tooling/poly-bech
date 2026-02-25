//! Context traits for LSP to inject clients and resources into runtime providers

use std::sync::Arc;

use poly_bench_syntax::Lang;

use crate::{diagnostics::EmbeddedLspClient, virtual_file::VirtualFile};

/// Context injected by LSP for hover providers
/// Provides virtual file access and LSP clients
pub trait EmbeddedHoverContext: Send + Sync {
    /// Get or create virtual file for the given language
    fn get_virtual_file(&self, lang: Lang) -> Option<Arc<dyn VirtualFile>>;
    fn get_go_client(&self, module_root: &str) -> Option<Arc<dyn EmbeddedLspClient>>;
    fn get_ts_client(&self, module_root: &str) -> Option<Arc<dyn EmbeddedLspClient>>;
    fn get_rust_client(&self, module_root: &str) -> Option<Arc<dyn EmbeddedLspClient>>;
    fn get_pyright_client(&self, module_root: &str) -> Option<Arc<dyn EmbeddedLspClient>>;
    fn byte_to_position(&self, offset: usize) -> (u32, u32);
    fn bench_offset(&self) -> usize;
    fn module_root(&self) -> &str;
}
