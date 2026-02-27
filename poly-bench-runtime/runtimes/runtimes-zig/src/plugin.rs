//! Zig runtime plugin.

use std::sync::Arc;

use linkme::distributed_slice;
use poly_bench_traits::{
    ErrorMapper, LangDisplayInfo, ProjectRootDetector, RuntimeFactory, RuntimePlugin,
};

use crate::{
    embedded_diagnostics::{ZIG_EMBEDDED_DIAGNOSTIC_PROVIDER, ZIG_EMBEDDED_DIAGNOSTIC_SETUP},
    error_mapping::ZIG_ERROR_MAPPER,
    executor::ZIG_RUNTIME_FACTORY,
    helper_extractor::ZIG_HELPER_EXTRACTOR,
    hover::ZIG_EMBEDDED_HOVER_PROVIDER,
    import_extractor::ZIG_IMPORT_EXTRACTOR,
    project::ZIG_DETECTOR,
    virtual_file::ZIG_VIRTUAL_FILE_BUILDER,
    zig_lang_display, zls_client,
};

pub struct ZigPlugin;

impl RuntimePlugin for ZigPlugin {
    fn lang(&self) -> poly_bench_dsl::Lang {
        poly_bench_dsl::Lang::Zig
    }

    fn runtime_factory(&self) -> &'static dyn RuntimeFactory {
        &ZIG_RUNTIME_FACTORY
    }

    fn error_mapper(&self) -> &'static dyn ErrorMapper {
        &ZIG_ERROR_MAPPER
    }

    fn lang_display(&self) -> LangDisplayInfo {
        zig_lang_display()
    }

    fn project_root_detector(&self) -> Option<&'static dyn ProjectRootDetector> {
        Some(&ZIG_DETECTOR)
    }

    fn import_extractor(&self) -> Option<&'static dyn poly_bench_ir_traits::ImportExtractor> {
        Some(&ZIG_IMPORT_EXTRACTOR)
    }

    fn virtual_file_builder(&self) -> Option<&'static dyn poly_bench_traits::VirtualFileBuilder> {
        Some(&ZIG_VIRTUAL_FILE_BUILDER)
    }

    fn embedded_diagnostic_provider(
        &self,
    ) -> Option<&'static dyn poly_bench_traits::EmbeddedDiagnosticProvider> {
        Some(&ZIG_EMBEDDED_DIAGNOSTIC_PROVIDER)
    }

    fn embedded_diagnostic_setup(
        &self,
    ) -> Option<&'static dyn poly_bench_traits::EmbeddedDiagnosticSetup> {
        Some(&ZIG_EMBEDDED_DIAGNOSTIC_SETUP)
    }

    fn embedded_hover_provider(
        &self,
    ) -> Option<&'static dyn poly_bench_traits::EmbeddedHoverProvider> {
        Some(&ZIG_EMBEDDED_HOVER_PROVIDER)
    }

    fn helper_function_extractor(
        &self,
    ) -> Option<&'static dyn poly_bench_traits::HelperFunctionExtractor> {
        Some(&ZIG_HELPER_EXTRACTOR)
    }

    fn embedded_lsp_client_init(
        &self,
        workspace_root: &str,
    ) -> Option<Arc<dyn poly_bench_traits::EmbeddedLspClient>> {
        zls_client::init_zls_client(workspace_root)
            .map(|c| c as Arc<dyn poly_bench_traits::EmbeddedLspClient>)
    }

    fn embedded_lsp_client_get(&self) -> Option<Arc<dyn poly_bench_traits::EmbeddedLspClient>> {
        zls_client::get_zls_client().map(|c| c as Arc<dyn poly_bench_traits::EmbeddedLspClient>)
    }
}

pub static ZIG_PLUGIN: ZigPlugin = ZigPlugin;

#[distributed_slice(poly_bench_traits::PLUGINS)]
static _ZIG: &dyn RuntimePlugin = &ZIG_PLUGIN;
