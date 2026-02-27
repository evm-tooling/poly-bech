//! C# runtime for executing benchmarks via dotnet

pub mod codegen;
pub mod embedded_diagnostics;
pub mod error_mapping;
pub mod executor;
pub mod helper_extractor;
pub mod hover;
pub mod import_extractor;
pub mod omnisharp_client;
pub mod plugin;
pub mod project;
pub mod stdlib;
pub mod virtual_file;

pub use error_mapping::{CSharpErrorMapper, CSHARP_ERROR_MAPPER};
pub use executor::{CSharpRuntime, CSharpRuntimeFactory, CSHARP_RUNTIME_FACTORY};
pub use import_extractor::CSHARP_IMPORT_EXTRACTOR;
pub use plugin::{CSharpPlugin, CSHARP_PLUGIN};
pub use project::CSHARP_DETECTOR;
pub use stdlib::{CSharpStdlibProvider, CSHARP_STDLIB};
pub use virtual_file::CSHARP_VIRTUAL_FILE_BUILDER;

use poly_bench_traits::LangDisplayInfo;

pub fn csharp_lang_display() -> LangDisplayInfo {
    LangDisplayInfo::new_with_icon(
        "C#",
        "CSharp",
        "#512BD4",
        "csharpGrad",
        "#7C3AED",
        "magenta",
        "🟣",
    )
}
