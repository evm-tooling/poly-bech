//! Runtime execution modules
//!
//! Provides language-specific runtimes for executing benchmarks.

pub mod error_mapping;
pub mod go;
pub mod js;
pub mod measurement;
pub mod rust;
pub mod traits;

pub use error_mapping::{
    build_go_mappings, build_rust_mappings, build_typescript_mappings, remap_go_error,
    remap_rust_error, remap_typescript_error, LineMappings,
};
pub use measurement::Measurement;
pub use traits::Runtime;
