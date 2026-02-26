//! Runtime trait definition

use crate::{Measurement, RuntimeConfig};
use async_trait::async_trait;
use miette::Result;
use poly_bench_dsl::Lang;
use poly_bench_ir::{BenchmarkSpec, SuiteIR};

/// Factory for creating runtime instances
pub trait RuntimeFactory: Send + Sync {
    /// Get the language this factory creates runtimes for
    fn lang(&self) -> Lang;

    /// Get the display name of this runtime
    fn name(&self) -> &'static str;

    /// Create a new runtime instance with the given configuration
    fn create(&self, config: &RuntimeConfig) -> Result<Box<dyn Runtime>>;
}

/// A runtime capable of executing benchmarks
#[async_trait]
pub trait Runtime: Send + Sync {
    /// Get the name of this runtime
    fn name(&self) -> &'static str;

    /// Get the language this runtime executes
    fn lang(&self) -> Lang;

    /// Set the Anvil RPC URL when std::anvil is used (default: no-op)
    fn set_anvil_rpc_url(&mut self, _url: String) {}

    /// Pre-compile a benchmark before timed runs (e.g. for fairness). Default: no-op.
    async fn precompile(&mut self, _spec: &BenchmarkSpec, _suite: &SuiteIR) -> Result<()> {
        Ok(())
    }

    /// Duration of the last precompile in nanoseconds, if the runtime measured it.
    /// Used for accurate precompile timing when the runtime does significant work (e.g. cargo
    /// build).
    fn last_precompile_nanos(&self) -> Option<u64> {
        None
    }

    /// Initialize the runtime with suite-level setup
    async fn initialize(&mut self, suite: &SuiteIR) -> Result<()>;

    /// Generate the source code that would be compiled for a benchmark.
    /// This is used for content-based caching of compilation results.
    fn generate_check_source(&self, spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<String>;

    /// Compile-check a benchmark without running it.
    /// Returns Ok(()) if compilation succeeds, or an error with compiler output.
    /// This is used for pre-run validation to catch errors before executing benchmarks.
    async fn compile_check(&self, spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<()>;

    /// Run a single benchmark and return measurements
    async fn run_benchmark(&mut self, spec: &BenchmarkSpec, suite: &SuiteIR)
        -> Result<Measurement>;

    /// Cleanup the runtime
    async fn shutdown(&mut self) -> Result<()>;
}
