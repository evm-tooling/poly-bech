//! Runtime plugin trait
//!
//! Bundles all runtime interfaces (RuntimeFactory, ErrorMapper, LangDisplayInfo,
//! ProjectRootDetector, ImportExtractor) for registration-based discovery.

use poly_bench_dsl::Lang;
use poly_bench_ir_traits::ImportExtractor;
use poly_bench_lsp_traits::{
    EmbeddedDiagnosticProvider, EmbeddedDiagnosticSetup, EmbeddedHoverProvider,
    HelperFunctionExtractor, VirtualFileBuilder,
};

use crate::{ErrorMapper, LangDisplayInfo, ProjectRootDetector, RuntimeFactory};

/// A runtime plugin bundles all interfaces for a language.
/// Implement this trait to register a new runtime with poly-bench.
pub trait RuntimePlugin: Send + Sync {
    /// The language this plugin provides
    fn lang(&self) -> Lang;

    /// The runtime factory for creating runtime instances
    fn runtime_factory(&self) -> &'static dyn RuntimeFactory;

    /// The error mapper for remapping compiler errors to .bench lines
    fn error_mapper(&self) -> &'static dyn ErrorMapper;

    /// Display metadata (labels, colors, gradients)
    fn lang_display(&self) -> LangDisplayInfo;

    /// Optional project root detector
    fn project_root_detector(&self) -> Option<&'static dyn ProjectRootDetector> {
        None
    }

    /// Optional import extractor for setup blocks
    fn import_extractor(&self) -> Option<&'static dyn ImportExtractor> {
        None
    }

    /// Optional virtual file builder for LSP
    fn virtual_file_builder(&self) -> Option<&'static dyn VirtualFileBuilder> {
        None
    }

    /// Optional embedded diagnostic provider for LSP
    fn embedded_diagnostic_provider(&self) -> Option<&'static dyn EmbeddedDiagnosticProvider> {
        None
    }

    /// Optional embedded diagnostic setup for LSP
    fn embedded_diagnostic_setup(&self) -> Option<&'static dyn EmbeddedDiagnosticSetup> {
        None
    }

    /// Optional embedded hover provider for LSP
    fn embedded_hover_provider(&self) -> Option<&'static dyn EmbeddedHoverProvider> {
        None
    }

    /// Optional helper function extractor for LSP undefined-function diagnostics
    fn helper_function_extractor(&self) -> Option<&'static dyn HelperFunctionExtractor> {
        None
    }
}
