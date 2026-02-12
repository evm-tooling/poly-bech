//! Benchmark execution orchestration

mod anvil;
mod scheduler;
pub mod comparison;

use std::path::PathBuf;

pub use anvil::{AnvilService, AnvilConfig};

/// Project roots for different languages
#[derive(Debug, Clone, Default)]
pub struct ProjectRoots {
    /// Go module root (directory containing go.mod)
    pub go_root: Option<PathBuf>,
    /// Node.js project root (directory containing package.json or node_modules)
    pub node_root: Option<PathBuf>,
}

pub use scheduler::run;
pub use comparison::{BenchmarkResults, SuiteResults, BenchmarkResult};
