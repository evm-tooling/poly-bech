//! Intermediate Representation (IR) for benchmarks
//!
//! The IR is a normalized, validated representation of benchmarks
//! that is easier to work with for code generation and execution.

mod fixtures;
pub mod imports;
mod lower;
mod types;

pub use fixtures::*;
pub use imports::{
    extract_go_imports, extract_imports, extract_python_imports, extract_rust_imports,
    extract_ts_imports, ImportExtractor, ParsedSetup,
};
pub use lower::lower;
pub use types::*;
