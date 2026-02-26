//! Zig runtime for executing benchmarks via zig.

pub mod embedded_diagnostics;
pub mod error_mapping;
pub mod executor;
pub mod helper_extractor;
pub mod hover;
pub mod import_extractor;
pub mod plugin;
pub mod project;
pub mod stdlib;
pub mod virtual_file;
pub mod zls_client;

pub use error_mapping::{ZigErrorMapper, ZIG_ERROR_MAPPER};
pub use executor::{ZigRuntime, ZigRuntimeFactory, ZIG_RUNTIME_FACTORY};
pub use import_extractor::ZIG_IMPORT_EXTRACTOR;
pub use plugin::{ZigPlugin, ZIG_PLUGIN};
pub use project::ZIG_DETECTOR;
pub use stdlib::{ZigStdlibProvider, ZIG_STDLIB};
pub use virtual_file::ZIG_VIRTUAL_FILE_BUILDER;

use poly_bench_runtime_traits::LangDisplayInfo;

pub fn zig_lang_display() -> LangDisplayInfo {
    LangDisplayInfo::new_with_icon("Zig", "Zig", "#F7A41D", "zigGrad", "#F7A41D", "yellow", "⚡")
}
