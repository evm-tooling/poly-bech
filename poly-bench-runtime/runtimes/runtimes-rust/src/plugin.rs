//! Rust runtime plugin

use std::sync::Arc;

use linkme::distributed_slice;
use poly_bench_traits::{
    ErrorMapper, LangDisplayInfo, ProjectRootDetector, RuntimeFactory, RuntimePlugin,
};

use crate::{
    embedded_diagnostics::{RUST_EMBEDDED_DIAGNOSTIC_PROVIDER, RUST_EMBEDDED_DIAGNOSTIC_SETUP},
    error_mapping::RUST_ERROR_MAPPER,
    executor::RUST_RUNTIME_FACTORY,
    helper_extractor::RUST_HELPER_EXTRACTOR,
    hover::RUST_EMBEDDED_HOVER_PROVIDER,
    import_extractor::RUST_IMPORT_EXTRACTOR,
    project::RUST_DETECTOR,
    rust_analyzer_client, rust_lang_display,
    virtual_file::RUST_VIRTUAL_FILE_BUILDER,
};

pub struct RustPlugin;

impl RuntimePlugin for RustPlugin {
    fn lang(&self) -> poly_bench_dsl::Lang {
        poly_bench_dsl::Lang::Rust
    }

    fn runtime_factory(&self) -> &'static dyn RuntimeFactory {
        &RUST_RUNTIME_FACTORY
    }

    fn error_mapper(&self) -> &'static dyn ErrorMapper {
        &RUST_ERROR_MAPPER
    }

    fn lang_display(&self) -> LangDisplayInfo {
        rust_lang_display()
    }

    fn project_root_detector(&self) -> Option<&'static dyn ProjectRootDetector> {
        Some(&RUST_DETECTOR)
    }

    fn import_extractor(&self) -> Option<&'static dyn poly_bench_ir_traits::ImportExtractor> {
        Some(&RUST_IMPORT_EXTRACTOR)
    }

    fn virtual_file_builder(&self) -> Option<&'static dyn poly_bench_traits::VirtualFileBuilder> {
        Some(&RUST_VIRTUAL_FILE_BUILDER)
    }

    fn embedded_diagnostic_provider(
        &self,
    ) -> Option<&'static dyn poly_bench_traits::EmbeddedDiagnosticProvider> {
        Some(&RUST_EMBEDDED_DIAGNOSTIC_PROVIDER)
    }

    fn embedded_diagnostic_setup(
        &self,
    ) -> Option<&'static dyn poly_bench_traits::EmbeddedDiagnosticSetup> {
        Some(&RUST_EMBEDDED_DIAGNOSTIC_SETUP)
    }

    fn embedded_hover_provider(
        &self,
    ) -> Option<&'static dyn poly_bench_traits::EmbeddedHoverProvider> {
        Some(&RUST_EMBEDDED_HOVER_PROVIDER)
    }

    fn helper_function_extractor(
        &self,
    ) -> Option<&'static dyn poly_bench_traits::HelperFunctionExtractor> {
        Some(&RUST_HELPER_EXTRACTOR)
    }

    fn embedded_lsp_client_init(
        &self,
        workspace_root: &str,
    ) -> Option<Arc<dyn poly_bench_traits::EmbeddedLspClient>> {
        rust_analyzer_client::init_rust_analyzer_client(workspace_root)
            .map(|c| c as Arc<dyn poly_bench_traits::EmbeddedLspClient>)
    }

    fn embedded_lsp_client_get(&self) -> Option<Arc<dyn poly_bench_traits::EmbeddedLspClient>> {
        rust_analyzer_client::get_rust_analyzer_client()
            .map(|c| c as Arc<dyn poly_bench_traits::EmbeddedLspClient>)
    }
}

pub static RUST_PLUGIN: RustPlugin = RustPlugin;

#[distributed_slice(poly_bench_traits::PLUGINS)]
static _RUST: &dyn RuntimePlugin = &RUST_PLUGIN;
