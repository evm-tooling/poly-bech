//! Go runtime plugin

use poly_bench_runtime_traits::{
    ErrorMapper, LangDisplayInfo, ProjectRootDetector, RuntimeFactory, RuntimePlugin,
};

use crate::{
    embedded_diagnostics::{GO_EMBEDDED_DIAGNOSTIC_PROVIDER, GO_EMBEDDED_DIAGNOSTIC_SETUP},
    error_mapping::GO_ERROR_MAPPER,
    executor::GO_RUNTIME_FACTORY,
    go_lang_display,
    helper_extractor::GO_HELPER_EXTRACTOR,
    hover::GO_EMBEDDED_HOVER_PROVIDER,
    import_extractor::GO_IMPORT_EXTRACTOR,
    project::GO_DETECTOR,
    virtual_file::GO_VIRTUAL_FILE_BUILDER,
};

pub struct GoPlugin;

impl RuntimePlugin for GoPlugin {
    fn lang(&self) -> poly_bench_dsl::Lang {
        poly_bench_dsl::Lang::Go
    }

    fn runtime_factory(&self) -> &'static dyn RuntimeFactory {
        &GO_RUNTIME_FACTORY
    }

    fn error_mapper(&self) -> &'static dyn ErrorMapper {
        &GO_ERROR_MAPPER
    }

    fn lang_display(&self) -> LangDisplayInfo {
        go_lang_display()
    }

    fn project_root_detector(&self) -> Option<&'static dyn ProjectRootDetector> {
        Some(&GO_DETECTOR)
    }

    fn import_extractor(&self) -> Option<&'static dyn poly_bench_ir_traits::ImportExtractor> {
        Some(&GO_IMPORT_EXTRACTOR)
    }

    fn virtual_file_builder(
        &self,
    ) -> Option<&'static dyn poly_bench_lsp_traits::VirtualFileBuilder> {
        Some(&GO_VIRTUAL_FILE_BUILDER)
    }

    fn embedded_diagnostic_provider(
        &self,
    ) -> Option<&'static dyn poly_bench_lsp_traits::EmbeddedDiagnosticProvider> {
        Some(&GO_EMBEDDED_DIAGNOSTIC_PROVIDER)
    }

    fn embedded_diagnostic_setup(
        &self,
    ) -> Option<&'static dyn poly_bench_lsp_traits::EmbeddedDiagnosticSetup> {
        Some(&GO_EMBEDDED_DIAGNOSTIC_SETUP)
    }

    fn embedded_hover_provider(
        &self,
    ) -> Option<&'static dyn poly_bench_lsp_traits::EmbeddedHoverProvider> {
        Some(&GO_EMBEDDED_HOVER_PROVIDER)
    }

    fn helper_function_extractor(
        &self,
    ) -> Option<&'static dyn poly_bench_lsp_traits::HelperFunctionExtractor> {
        Some(&GO_HELPER_EXTRACTOR)
    }
}

pub static GO_PLUGIN: GoPlugin = GoPlugin;
