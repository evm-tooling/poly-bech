//! Benchmark execution orchestration

mod anvil;
pub mod comparison;
pub mod compile_cache;
mod scheduler;
mod validation;
pub mod workspace;

use poly_bench_dsl::Lang;
use std::{collections::HashMap, path::PathBuf};

pub use anvil::{AnvilConfig, AnvilService};
pub use compile_cache::{CacheStats, CompileCache};
pub use workspace::{format_size, CompileWorkspace};

/// Project roots for different languages
#[derive(Debug, Clone, Default)]
pub struct ProjectRoots {
    /// Project roots per language
    pub roots: HashMap<Lang, Option<PathBuf>>,
}

impl ProjectRoots {
    /// Get the project root for a language
    pub fn get_root(&self, lang: Lang) -> Option<PathBuf> {
        self.roots.get(&lang).and_then(|o| o.clone())
    }

    /// Set the project root for a language
    pub fn set_root(&mut self, lang: Lang, path: Option<PathBuf>) {
        self.roots.insert(lang, path);
    }
}

pub use comparison::{BenchmarkResult, BenchmarkResults, SuiteResults};
pub use scheduler::{run, RunOptions};
pub use validation::{
    validate_benchmarks, validate_benchmarks_with_cache, CompileError, ValidationStats,
};
