//! Python runtime for executing benchmarks via subprocess

pub mod embedded_diagnostics;
pub mod error_mapping;
pub mod executor;
pub mod helper_extractor;
pub mod hover;
pub mod import_extractor;
pub mod plugin;
pub mod project;
pub mod virtual_file;

pub use error_mapping::{PythonErrorMapper, PYTHON_ERROR_MAPPER};
pub use executor::{PythonRuntime, PythonRuntimeFactory, PYTHON_RUNTIME_FACTORY};
pub use import_extractor::PYTHON_IMPORT_EXTRACTOR;
pub use plugin::{PythonPlugin, PYTHON_PLUGIN};
pub use project::PYTHON_DETECTOR;
pub use virtual_file::PYTHON_VIRTUAL_FILE_BUILDER;

use poly_bench_runtime_traits::LangDisplayInfo;

/// Python language display info
pub fn python_lang_display() -> LangDisplayInfo {
    LangDisplayInfo::new_with_icon(
        "Python",
        "Python",
        "#3776AB",
        "pythonGrad",
        "#FFD43B",
        "bright_blue",
        "🐍",
    )
}
