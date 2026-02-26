//! TypeScript runtime plugin

use std::sync::Arc;

use linkme::distributed_slice;
use poly_bench_runtime_traits::{
    ErrorMapper, LangDisplayInfo, ProjectRootDetector, RuntimeFactory, RuntimePlugin,
};

use crate::{
    embedded_diagnostics::{TS_EMBEDDED_DIAGNOSTIC_PROVIDER, TS_EMBEDDED_DIAGNOSTIC_SETUP},
    error_mapping::TS_ERROR_MAPPER,
    helper_extractor::TS_HELPER_EXTRACTOR,
    hover::TS_EMBEDDED_HOVER_PROVIDER,
    import_extractor::TS_IMPORT_EXTRACTOR,
    project::TS_DETECTOR,
    ts_lang_display, tsserver_client,
    v8_runtime::JS_RUNTIME_FACTORY,
    virtual_file::TS_VIRTUAL_FILE_BUILDER,
};

pub struct TsPlugin;

impl RuntimePlugin for TsPlugin {
    fn lang(&self) -> poly_bench_dsl::Lang {
        poly_bench_dsl::Lang::TypeScript
    }

    fn runtime_factory(&self) -> &'static dyn RuntimeFactory {
        &JS_RUNTIME_FACTORY
    }

    fn error_mapper(&self) -> &'static dyn ErrorMapper {
        &TS_ERROR_MAPPER
    }

    fn lang_display(&self) -> LangDisplayInfo {
        ts_lang_display()
    }

    fn project_root_detector(&self) -> Option<&'static dyn ProjectRootDetector> {
        Some(&TS_DETECTOR)
    }

    fn import_extractor(&self) -> Option<&'static dyn poly_bench_ir_traits::ImportExtractor> {
        Some(&TS_IMPORT_EXTRACTOR)
    }

    fn virtual_file_builder(
        &self,
    ) -> Option<&'static dyn poly_bench_lsp_traits::VirtualFileBuilder> {
        Some(&TS_VIRTUAL_FILE_BUILDER)
    }

    fn embedded_diagnostic_provider(
        &self,
    ) -> Option<&'static dyn poly_bench_lsp_traits::EmbeddedDiagnosticProvider> {
        Some(&TS_EMBEDDED_DIAGNOSTIC_PROVIDER)
    }

    fn embedded_diagnostic_setup(
        &self,
    ) -> Option<&'static dyn poly_bench_lsp_traits::EmbeddedDiagnosticSetup> {
        Some(&TS_EMBEDDED_DIAGNOSTIC_SETUP)
    }

    fn embedded_hover_provider(
        &self,
    ) -> Option<&'static dyn poly_bench_lsp_traits::EmbeddedHoverProvider> {
        Some(&TS_EMBEDDED_HOVER_PROVIDER)
    }

    fn helper_function_extractor(
        &self,
    ) -> Option<&'static dyn poly_bench_lsp_traits::HelperFunctionExtractor> {
        Some(&TS_HELPER_EXTRACTOR)
    }

    fn embedded_lsp_client_init(
        &self,
        workspace_root: &str,
    ) -> Option<Arc<dyn poly_bench_lsp_traits::EmbeddedLspClient>> {
        tsserver_client::init_tsserver_client(workspace_root)
            .map(|c| c as Arc<dyn poly_bench_lsp_traits::EmbeddedLspClient>)
    }

    fn embedded_lsp_client_get(&self) -> Option<Arc<dyn poly_bench_lsp_traits::EmbeddedLspClient>> {
        tsserver_client::get_tsserver_client()
            .map(|c| c as Arc<dyn poly_bench_lsp_traits::EmbeddedLspClient>)
    }
}

pub static TS_PLUGIN: TsPlugin = TsPlugin;

#[distributed_slice(poly_bench_runtime_traits::PLUGINS)]
static _TS: &dyn RuntimePlugin = &TS_PLUGIN;
