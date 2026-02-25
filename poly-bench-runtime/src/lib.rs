//! Runtime execution modules
//!
//! Provides language-specific runtimes for executing benchmarks.

pub mod config;
pub mod error_mapping;
pub mod go;
pub mod js;
pub mod lang_display;
pub mod measurement;
pub mod python;
pub mod registry;
pub mod rust;
pub mod traits;

pub use config::RuntimeConfig;
pub use error_mapping::{
    build_go_mappings, build_rust_mappings, build_typescript_mappings, get_error_mapper,
    remap_go_error, remap_rust_error, remap_typescript_error, ErrorMapper, LineMappings,
};
pub use js::{extract_generated_snippet, extract_runtime_error_reason};
pub use lang_display::{
    lang_color, lang_display, lang_full_name, lang_gradient_id, lang_label, LangDisplayInfo,
};
pub use measurement::Measurement;
pub use registry::{
    create_runtime, create_runtimes, create_runtimes_arc, supported_languages, RuntimeFactory,
};
pub use traits::Runtime;
