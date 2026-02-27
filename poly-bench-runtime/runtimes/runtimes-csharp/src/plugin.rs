//! C# runtime plugin.

use std::sync::Arc;

use linkme::distributed_slice;
use poly_bench_traits::{
    ErrorMapper, LangDisplayInfo, ProjectRootDetector, RuntimeFactory, RuntimePlugin,
};

use crate::{
    csharp_lang_display,
    embedded_diagnostics::{CSHARP_EMBEDDED_DIAGNOSTIC_PROVIDER, CSHARP_EMBEDDED_DIAGNOSTIC_SETUP},
    error_mapping::CSHARP_ERROR_MAPPER,
    executor::CSHARP_RUNTIME_FACTORY,
    helper_extractor::CSHARP_HELPER_EXTRACTOR,
    hover::CSHARP_EMBEDDED_HOVER_PROVIDER,
    import_extractor::CSHARP_IMPORT_EXTRACTOR,
    omnisharp_client,
    project::CSHARP_DETECTOR,
    virtual_file::CSHARP_VIRTUAL_FILE_BUILDER,
};

pub struct CSharpPlugin;

impl RuntimePlugin for CSharpPlugin {
    fn lang(&self) -> poly_bench_dsl::Lang {
        poly_bench_dsl::Lang::CSharp
    }

    fn runtime_factory(&self) -> &'static dyn RuntimeFactory {
        &CSHARP_RUNTIME_FACTORY
    }

    fn error_mapper(&self) -> &'static dyn ErrorMapper {
        &CSHARP_ERROR_MAPPER
    }

    fn lang_display(&self) -> LangDisplayInfo {
        csharp_lang_display()
    }

    fn project_root_detector(&self) -> Option<&'static dyn ProjectRootDetector> {
        Some(&CSHARP_DETECTOR)
    }

    fn import_extractor(&self) -> Option<&'static dyn poly_bench_ir_traits::ImportExtractor> {
        Some(&CSHARP_IMPORT_EXTRACTOR)
    }

    fn virtual_file_builder(&self) -> Option<&'static dyn poly_bench_traits::VirtualFileBuilder> {
        Some(&CSHARP_VIRTUAL_FILE_BUILDER)
    }

    fn embedded_diagnostic_provider(
        &self,
    ) -> Option<&'static dyn poly_bench_traits::EmbeddedDiagnosticProvider> {
        Some(&CSHARP_EMBEDDED_DIAGNOSTIC_PROVIDER)
    }

    fn embedded_diagnostic_setup(
        &self,
    ) -> Option<&'static dyn poly_bench_traits::EmbeddedDiagnosticSetup> {
        Some(&CSHARP_EMBEDDED_DIAGNOSTIC_SETUP)
    }

    fn embedded_hover_provider(
        &self,
    ) -> Option<&'static dyn poly_bench_traits::EmbeddedHoverProvider> {
        Some(&CSHARP_EMBEDDED_HOVER_PROVIDER)
    }

    fn helper_function_extractor(
        &self,
    ) -> Option<&'static dyn poly_bench_traits::HelperFunctionExtractor> {
        Some(&CSHARP_HELPER_EXTRACTOR)
    }

    fn embedded_lsp_client_init(
        &self,
        workspace_root: &str,
    ) -> Option<Arc<dyn poly_bench_traits::EmbeddedLspClient>> {
        omnisharp_client::init_omnisharp_client(workspace_root)
            .map(|c| c as Arc<dyn poly_bench_traits::EmbeddedLspClient>)
    }

    fn embedded_lsp_client_get(&self) -> Option<Arc<dyn poly_bench_traits::EmbeddedLspClient>> {
        omnisharp_client::get_omnisharp_client()
            .map(|c| c as Arc<dyn poly_bench_traits::EmbeddedLspClient>)
    }
}

pub static CSHARP_PLUGIN: CSharpPlugin = CSharpPlugin;

#[distributed_slice(poly_bench_traits::PLUGINS)]
static _CSHARP: &dyn RuntimePlugin = &CSHARP_PLUGIN;
