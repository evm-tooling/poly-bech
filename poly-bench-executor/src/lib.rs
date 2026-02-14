//! Benchmark execution orchestration

mod anvil;
pub mod comparison;
mod scheduler;

use std::path::PathBuf;

pub use anvil::{AnvilConfig, AnvilService};

/// Project roots for different languages
#[derive(Debug, Clone, Default)]
pub struct ProjectRoots {
    /// Go module root (directory containing go.mod)
    pub go_root: Option<PathBuf>,
    /// Node.js project root (directory containing package.json or node_modules)
    pub node_root: Option<PathBuf>,
    /// Rust project root (directory containing Cargo.toml)
    pub rust_root: Option<PathBuf>,
}

pub use comparison::{BenchmarkResult, BenchmarkResults, SuiteResults};
pub use scheduler::run;
