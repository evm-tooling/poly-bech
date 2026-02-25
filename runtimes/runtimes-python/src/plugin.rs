//! Python runtime plugin

use std::sync::Arc;

use poly_bench_runtime_traits::{
    ErrorMapper, LangDisplayInfo, ProjectRootDetector, RuntimeFactory, RuntimePlugin,
};

use crate::{
    embedded_diagnostics::{PYTHON_EMBEDDED_DIAGNOSTIC_PROVIDER, PYTHON_EMBEDDED_DIAGNOSTIC_SETUP},
    error_mapping::PYTHON_ERROR_MAPPER,
    executor::PYTHON_RUNTIME_FACTORY,
    helper_extractor::PYTHON_HELPER_EXTRACTOR,
    hover::PYTHON_EMBEDDED_HOVER_PROVIDER,
    import_extractor::PYTHON_IMPORT_EXTRACTOR,
    project::PYTHON_DETECTOR,
    pyright_client, python_lang_display,
    virtual_file::PYTHON_VIRTUAL_FILE_BUILDER,
};

pub struct PythonPlugin;

impl RuntimePlugin for PythonPlugin {
    fn lang(&self) -> poly_bench_dsl::Lang {
        poly_bench_dsl::Lang::Python
    }

    fn runtime_factory(&self) -> &'static dyn RuntimeFactory {
        &PYTHON_RUNTIME_FACTORY
    }

    fn error_mapper(&self) -> &'static dyn ErrorMapper {
        &PYTHON_ERROR_MAPPER
    }

    fn lang_display(&self) -> LangDisplayInfo {
        python_lang_display()
    }

    fn project_root_detector(&self) -> Option<&'static dyn ProjectRootDetector> {
        Some(&PYTHON_DETECTOR)
    }

    fn import_extractor(&self) -> Option<&'static dyn poly_bench_ir_traits::ImportExtractor> {
        Some(&PYTHON_IMPORT_EXTRACTOR)
    }

    fn virtual_file_builder(
        &self,
    ) -> Option<&'static dyn poly_bench_lsp_traits::VirtualFileBuilder> {
        Some(&PYTHON_VIRTUAL_FILE_BUILDER)
    }

    fn embedded_diagnostic_provider(
        &self,
    ) -> Option<&'static dyn poly_bench_lsp_traits::EmbeddedDiagnosticProvider> {
        Some(&PYTHON_EMBEDDED_DIAGNOSTIC_PROVIDER)
    }

    fn embedded_diagnostic_setup(
        &self,
    ) -> Option<&'static dyn poly_bench_lsp_traits::EmbeddedDiagnosticSetup> {
        Some(&PYTHON_EMBEDDED_DIAGNOSTIC_SETUP)
    }

    fn embedded_hover_provider(
        &self,
    ) -> Option<&'static dyn poly_bench_lsp_traits::EmbeddedHoverProvider> {
        Some(&PYTHON_EMBEDDED_HOVER_PROVIDER)
    }

    fn helper_function_extractor(
        &self,
    ) -> Option<&'static dyn poly_bench_lsp_traits::HelperFunctionExtractor> {
        Some(&PYTHON_HELPER_EXTRACTOR)
    }

    fn embedded_lsp_client_init(
        &self,
        workspace_root: &str,
    ) -> Option<Arc<dyn poly_bench_lsp_traits::EmbeddedLspClient>> {
        pyright_client::init_pyright_client(workspace_root)
            .map(|c| c as Arc<dyn poly_bench_lsp_traits::EmbeddedLspClient>)
    }

    fn embedded_lsp_client_get(&self) -> Option<Arc<dyn poly_bench_lsp_traits::EmbeddedLspClient>> {
        pyright_client::get_pyright_client()
            .map(|c| c as Arc<dyn poly_bench_lsp_traits::EmbeddedLspClient>)
    }
}

pub static PYTHON_PLUGIN: PythonPlugin = PythonPlugin;
