//! Embedded hover provider trait

use tower_lsp::lsp_types::Hover;

use crate::context::EmbeddedHoverContext;

/// Trait for language-specific hover providers
pub trait EmbeddedHoverProvider: Send + Sync {
    fn lang(&self) -> poly_bench_syntax::Lang;
    fn get_hover(&self, ctx: &dyn EmbeddedHoverContext) -> Option<Hover>;
}
