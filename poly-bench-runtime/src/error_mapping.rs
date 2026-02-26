//! Error line mapping utilities
//!
//! Maps compiler error line numbers from generated code back to original .bench file locations.

use poly_bench_dsl::Lang;
use poly_bench_runtime_traits::ErrorMapper;
use runtimes_c::C_PLUGIN;
use runtimes_csharp::CSHARP_PLUGIN;
use runtimes_go::GO_PLUGIN;
use runtimes_python::PYTHON_PLUGIN;
use runtimes_rust::RUST_PLUGIN;
use runtimes_ts::TS_PLUGIN;

static PLUGINS: &[&dyn poly_bench_runtime_traits::RuntimePlugin] =
    &[&GO_PLUGIN, &TS_PLUGIN, &RUST_PLUGIN, &PYTHON_PLUGIN, &C_PLUGIN, &CSHARP_PLUGIN];

/// Get the error mapper for a language
pub fn get_error_mapper(lang: Lang) -> Option<&'static dyn ErrorMapper> {
    PLUGINS.iter().find(|p| p.lang() == lang).map(|p| p.error_mapper())
}
