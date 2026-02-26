//! Runtime registry for pluggable language runtimes
//!
//! Plugins register themselves via the `PLUGINS` distributed slice in
//! poly-bench-runtime-traits. No manual registration needed here.

use crate::config::RuntimeConfig;
use miette::{miette, Result};
use poly_bench_dsl::Lang;
use poly_bench_lsp_traits::{
    EmbeddedDiagnosticProvider, EmbeddedDiagnosticSetup, EmbeddedHoverProvider, EmbeddedLspClient,
    HelperFunctionExtractor, VirtualFileBuilder,
};
use poly_bench_runtime_traits::{ProjectRootDetector, Runtime, PLUGINS};
use std::{collections::HashMap, sync::Arc};

/// Create a runtime for the given language
pub fn create_runtime(lang: Lang, config: &RuntimeConfig) -> Result<Box<dyn Runtime>> {
    for plugin in PLUGINS {
        if plugin.lang() == lang {
            return plugin.runtime_factory().create(config);
        }
    }
    Err(miette!("No runtime registered for language: {}", lang))
}

/// Get the project root detector for a language
pub fn get_detector(lang: Lang) -> Option<&'static dyn ProjectRootDetector> {
    PLUGINS.iter().find(|p| p.lang() == lang).and_then(|p| p.project_root_detector())
}

/// Get the virtual file builder for a language
pub fn get_virtual_file_builder(lang: Lang) -> Option<&'static dyn VirtualFileBuilder> {
    PLUGINS.iter().find(|p| p.lang() == lang).and_then(|p| p.virtual_file_builder())
}

/// Get the embedded diagnostic provider for a language
pub fn get_embedded_diagnostic_provider(
    lang: Lang,
) -> Option<&'static dyn EmbeddedDiagnosticProvider> {
    PLUGINS.iter().find(|p| p.lang() == lang).and_then(|p| p.embedded_diagnostic_provider())
}

/// Get the embedded diagnostic setup for a language
pub fn get_embedded_diagnostic_setup(lang: Lang) -> Option<&'static dyn EmbeddedDiagnosticSetup> {
    PLUGINS.iter().find(|p| p.lang() == lang).and_then(|p| p.embedded_diagnostic_setup())
}

/// Get the embedded hover provider for a language
pub fn get_embedded_hover_provider(lang: Lang) -> Option<&'static dyn EmbeddedHoverProvider> {
    PLUGINS.iter().find(|p| p.lang() == lang).and_then(|p| p.embedded_hover_provider())
}

/// Get the helper function extractor for a language
pub fn get_helper_function_extractor(lang: Lang) -> Option<&'static dyn HelperFunctionExtractor> {
    PLUGINS.iter().find(|p| p.lang() == lang).and_then(|p| p.helper_function_extractor())
}

/// Initialize the embedded LSP client for a language and return it if available
pub fn init_embedded_lsp_client(
    lang: Lang,
    workspace_root: &str,
) -> Option<Arc<dyn EmbeddedLspClient>> {
    PLUGINS
        .iter()
        .find(|p| p.lang() == lang)
        .and_then(|p| p.embedded_lsp_client_init(workspace_root))
}

/// Get the embedded LSP client for a language if already initialized
pub fn get_embedded_lsp_client(lang: Lang) -> Option<Arc<dyn EmbeddedLspClient>> {
    PLUGINS.iter().find(|p| p.lang() == lang).and_then(|p| p.embedded_lsp_client_get())
}

/// Initialize import extractors for poly-bench-ir.
/// Must be called at application startup before any ir::lower.
pub fn init_import_extractors() {
    let extractors: Vec<_> = PLUGINS.iter().filter_map(|p| p.import_extractor()).collect();
    poly_bench_ir::set_import_extractors(extractors);
}

/// Get all supported languages (derived from registered plugins)
pub fn supported_languages() -> &'static [Lang] {
    use once_cell::sync::Lazy;
    static LANGS: Lazy<Vec<Lang>> = Lazy::new(|| PLUGINS.iter().map(|p| p.lang()).collect());
    LANGS.as_slice()
}

/// Build a map of runtimes for the requested languages (owned, for scheduler).
pub fn create_runtimes(
    langs: &[Lang],
    config: &RuntimeConfig,
) -> Result<HashMap<Lang, Box<dyn Runtime>>> {
    let mut runtimes = HashMap::new();
    for lang in langs {
        let rt = create_runtime(*lang, config)?;
        runtimes.insert(*lang, rt);
    }
    Ok(runtimes)
}

/// Build a map of Arc-wrapped runtimes for validation.
pub fn create_runtimes_arc(
    langs: &[Lang],
    config: &RuntimeConfig,
) -> HashMap<Lang, Arc<dyn Runtime>> {
    let mut runtimes = HashMap::new();
    for lang in langs {
        if let Ok(rt) = create_runtime(*lang, config) {
            runtimes.insert(*lang, Arc::from(rt));
        }
    }
    runtimes
}
