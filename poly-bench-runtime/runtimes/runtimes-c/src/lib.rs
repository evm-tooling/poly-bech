//! C runtime for executing benchmarks via clang.

pub mod clangd_client;
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

pub use error_mapping::{CErrorMapper, C_ERROR_MAPPER};
pub use executor::{CRuntime, CRuntimeFactory, C_RUNTIME_FACTORY};
pub use import_extractor::C_IMPORT_EXTRACTOR;
pub use plugin::{CPlugin, C_PLUGIN};
pub use project::C_DETECTOR;
pub use stdlib::{CStdlibProvider, C_STDLIB};
pub use virtual_file::C_VIRTUAL_FILE_BUILDER;

use poly_bench_traits::LangDisplayInfo;

pub fn c_lang_display() -> LangDisplayInfo {
    LangDisplayInfo::new_with_icon("C", "C", "#2563EB", "cGrad", "#3B82F6", "blue", "🔵")
}
