//! Rust runtime for executing benchmarks via cargo subprocess

pub mod codegen;
pub mod embedded_diagnostics;
pub mod error_mapping;
pub mod executor;
pub mod helper_extractor;
pub mod hover;
pub mod import_extractor;
pub mod plugin;
pub mod project;
pub mod rust_analyzer_client;
pub mod shared;
pub mod stdlib;
pub mod virtual_file;

pub use error_mapping::{RustErrorMapper, RUST_ERROR_MAPPER};
pub use executor::{RustRuntime, RustRuntimeFactory};
pub use import_extractor::RUST_IMPORT_EXTRACTOR;
pub use plugin::{RustPlugin, RUST_PLUGIN};
pub use project::RUST_DETECTOR;
pub use stdlib::{RustStdlibProvider, RUST_STDLIB};
pub use virtual_file::RUST_VIRTUAL_FILE_BUILDER;

use poly_bench_runtime_traits::LangDisplayInfo;

/// Rust language display info
pub fn rust_lang_display() -> LangDisplayInfo {
    LangDisplayInfo::new_with_icon("Rust", "Rust", "#DEA584", "rustGrad", "#B7410E", "yellow", "🟠")
}
