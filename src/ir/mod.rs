//! Intermediate Representation (IR) for benchmarks
//!
//! The IR is a normalized, validated representation of benchmarks
//! that is easier to work with for code generation and execution.

mod types;
mod lower;
mod fixtures;
pub mod imports;

pub use types::*;
pub use lower::lower;
pub use fixtures::*;
pub use imports::{ParsedSetup, extract_go_imports, extract_ts_imports};
