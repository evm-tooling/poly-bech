//! C runtime plugin.

use std::sync::Arc;

use linkme::distributed_slice;
use poly_bench_traits::{
    ErrorMapper, LangDisplayInfo, ProjectRootDetector, RuntimeFactory, RuntimePlugin,
};

use crate::{
    c_lang_display, clangd_client,
    embedded_diagnostics::{C_EMBEDDED_DIAGNOSTIC_PROVIDER, C_EMBEDDED_DIAGNOSTIC_SETUP},
    error_mapping::C_ERROR_MAPPER,
    executor::C_RUNTIME_FACTORY,
    helper_extractor::C_HELPER_EXTRACTOR,
    hover::C_EMBEDDED_HOVER_PROVIDER,
    import_extractor::C_IMPORT_EXTRACTOR,
    project::C_DETECTOR,
    virtual_file::C_VIRTUAL_FILE_BUILDER,
};

pub struct CPlugin;

impl RuntimePlugin for CPlugin {
    fn lang(&self) -> poly_bench_dsl::Lang {
        poly_bench_dsl::Lang::C
    }

    fn runtime_factory(&self) -> &'static dyn RuntimeFactory {
        &C_RUNTIME_FACTORY
    }

    fn error_mapper(&self) -> &'static dyn ErrorMapper {
        &C_ERROR_MAPPER
    }

    fn lang_display(&self) -> LangDisplayInfo {
        c_lang_display()
    }

    fn project_root_detector(&self) -> Option<&'static dyn ProjectRootDetector> {
        Some(&C_DETECTOR)
    }

    fn import_extractor(&self) -> Option<&'static dyn poly_bench_ir_traits::ImportExtractor> {
        Some(&C_IMPORT_EXTRACTOR)
    }

    fn virtual_file_builder(&self) -> Option<&'static dyn poly_bench_traits::VirtualFileBuilder> {
        Some(&C_VIRTUAL_FILE_BUILDER)
    }

    fn embedded_diagnostic_provider(
        &self,
    ) -> Option<&'static dyn poly_bench_traits::EmbeddedDiagnosticProvider> {
        Some(&C_EMBEDDED_DIAGNOSTIC_PROVIDER)
    }

    fn embedded_diagnostic_setup(
        &self,
    ) -> Option<&'static dyn poly_bench_traits::EmbeddedDiagnosticSetup> {
        Some(&C_EMBEDDED_DIAGNOSTIC_SETUP)
    }

    fn embedded_hover_provider(
        &self,
    ) -> Option<&'static dyn poly_bench_traits::EmbeddedHoverProvider> {
        Some(&C_EMBEDDED_HOVER_PROVIDER)
    }

    fn helper_function_extractor(
        &self,
    ) -> Option<&'static dyn poly_bench_traits::HelperFunctionExtractor> {
        Some(&C_HELPER_EXTRACTOR)
    }

    fn embedded_lsp_client_init(
        &self,
        workspace_root: &str,
    ) -> Option<Arc<dyn poly_bench_traits::EmbeddedLspClient>> {
        clangd_client::init_clangd_client(workspace_root)
            .map(|c| c as Arc<dyn poly_bench_traits::EmbeddedLspClient>)
    }

    fn embedded_lsp_client_get(&self) -> Option<Arc<dyn poly_bench_traits::EmbeddedLspClient>> {
        clangd_client::get_clangd_client()
            .map(|c| c as Arc<dyn poly_bench_traits::EmbeddedLspClient>)
    }
}

pub static C_PLUGIN: CPlugin = CPlugin;

#[distributed_slice(poly_bench_traits::PLUGINS)]
static _C: &dyn RuntimePlugin = &C_PLUGIN;
