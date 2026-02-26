//! TypeScript runtime for executing benchmarks via Node.js

pub mod builtins;
pub mod codegen;
pub mod embedded_diagnostics;
pub mod error_mapping;
pub mod helper_extractor;
pub mod hover;
pub mod import_extractor;
pub mod plugin;
pub mod project;
pub mod stdlib;
pub mod transpiler;
pub mod tsserver_client;
pub mod v8_runtime;
pub mod virtual_file;

pub use error_mapping::{TsErrorMapper, TS_ERROR_MAPPER};
pub use import_extractor::TS_IMPORT_EXTRACTOR;
pub use plugin::{TsPlugin, TS_PLUGIN};
pub use project::TS_DETECTOR;
pub use stdlib::{TsStdlibProvider, TS_STDLIB};
pub use v8_runtime::{
    extract_generated_snippet, extract_runtime_error_reason, JsRuntime, JsRuntimeFactory,
};
pub use virtual_file::TS_VIRTUAL_FILE_BUILDER;

use poly_bench_runtime_traits::LangDisplayInfo;

/// TypeScript language display info
pub fn ts_lang_display() -> LangDisplayInfo {
    LangDisplayInfo::new_with_icon("TS", "TypeScript", "#3178C6", "tsGrad", "#1D4ED8", "cyan", "🔵")
}
