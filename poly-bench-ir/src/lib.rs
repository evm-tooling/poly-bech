//! Intermediate Representation (IR) for benchmarks
//!
//! The IR is a normalized, validated representation of benchmarks
//! that is easier to work with for code generation and execution.

mod fixtures;
pub mod imports;
mod lower;
mod types;

pub use fixtures::*;
pub use imports::{extract_imports, set_import_extractors};
pub use lower::lower;
pub use poly_bench_ir_traits::{ImportExtractor, ParsedSetup};
pub use types::*;
