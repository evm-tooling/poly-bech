//! Benchmark execution orchestration

mod anvil;
pub mod comparison;
pub mod compile_cache;
mod scheduler;
mod validation;
pub mod workspace;

use std::path::PathBuf;

pub use anvil::{AnvilConfig, AnvilService};
pub use compile_cache::{CacheStats, CompileCache};
pub use workspace::{format_size, CompileWorkspace};

/// Project roots for different languages
#[derive(Debug, Clone, Default)]
pub struct ProjectRoots {
    // BEGIN-GENERATED: ProjectRoots fields (do not edit)
    /// Go module root (directory containing go.mod)
    pub go_root: Option<PathBuf>,
    /// Node.js project root (directory containing package.json or node_modules)
    pub node_root: Option<PathBuf>,
    /// Rust project root (directory containing Cargo.toml)
    pub rust_root: Option<PathBuf>,
    /// Python project root (directory containing requirements.txt or pyproject.toml)
    pub python_root: Option<PathBuf>,
    /// C# project root (directory containing .csproj/.sln)
    pub csharp_root: Option<PathBuf>,
    // END-GENERATED: ProjectRoots fields
}

pub use comparison::{BenchmarkResult, BenchmarkResults, SuiteResults};
pub use scheduler::{run, RunOptions};
pub use validation::{
    validate_benchmarks, validate_benchmarks_with_cache, CompileError, ValidationStats,
};
