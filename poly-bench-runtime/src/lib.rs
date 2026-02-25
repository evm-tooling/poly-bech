//! Runtime execution modules
//!
//! Provides language-specific runtimes for executing benchmarks.

pub mod config;
pub mod error_mapping;
/// Re-export Go runtime from runtimes-go for backward compatibility
pub mod go {
    pub use runtimes_go::{codegen, GoRuntime};
}
/// Re-export TypeScript runtime from runtimes-ts for backward compatibility
/// Re-export Rust runtime from runtimes-rust for backward compatibility
pub mod rust {
    pub use runtimes_rust::{codegen, RustRuntime};
}
pub mod js {
    pub use runtimes_ts::{
        codegen, extract_generated_snippet, extract_runtime_error_reason, JsRuntime,
    };
}
pub mod lang_display;
pub mod measurement;
/// Re-export Python runtime from runtimes-python for backward compatibility
pub mod python {
    pub use runtimes_python::PythonRuntime;
}
pub mod registry;
pub mod traits;

pub use config::RuntimeConfig;
pub use error_mapping::get_error_mapper;
pub use js::{extract_generated_snippet, extract_runtime_error_reason};
pub use lang_display::{
    lang_color, lang_display, lang_full_name, lang_gradient_id, lang_icon, lang_label,
};
pub use measurement::Measurement;
pub use poly_bench_runtime_traits::{
    detect_from_markers, ErrorMapper, LangDisplayInfo, LineMapping, LineMappings,
    ProjectRootDetector, RuntimeFactory,
};
pub use registry::{
    create_runtime, create_runtimes, create_runtimes_arc, get_detector,
    get_embedded_diagnostic_provider, get_embedded_diagnostic_setup, get_embedded_hover_provider,
    get_embedded_lsp_client, get_helper_function_extractor, get_virtual_file_builder,
    init_embedded_lsp_client, init_import_extractors, supported_languages,
};
pub use traits::Runtime;
