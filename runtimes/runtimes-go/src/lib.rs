//! Go runtime for executing Go benchmarks via plugin system

pub mod codegen;
pub mod compiler;
pub mod embedded_diagnostics;
pub mod error_mapping;
pub mod executor;
pub mod gopls_client;
pub mod helper_extractor;
pub mod hover;
pub mod import_extractor;
pub mod plugin;
pub mod project;
pub mod shared;
pub mod stdlib;
pub mod virtual_file;

pub use error_mapping::{GoErrorMapper, GO_ERROR_MAPPER};
pub use executor::{GoRuntime, GoRuntimeFactory};
pub use import_extractor::GO_IMPORT_EXTRACTOR;
pub use plugin::{GoPlugin, GO_PLUGIN};
pub use project::GO_DETECTOR;
pub use stdlib::{GoStdlibProvider, GO_STDLIB};
pub use virtual_file::GO_VIRTUAL_FILE_BUILDER;

use poly_bench_runtime_traits::LangDisplayInfo;

/// Go language display info
pub fn go_lang_display() -> LangDisplayInfo {
    LangDisplayInfo::new_with_icon("Go", "Go", "#00ADD8", "goGrad", "#0891B2", "green", "🟢")
}
