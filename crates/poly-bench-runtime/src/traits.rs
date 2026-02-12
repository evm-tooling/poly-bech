//! Runtime trait definition

use poly_bench_dsl::Lang;
use poly_bench_ir::{BenchmarkSpec, SuiteIR};
use crate::Measurement;
use async_trait::async_trait;
use miette::Result;

/// A runtime capable of executing benchmarks
#[async_trait]
pub trait Runtime: Send + Sync {
    /// Get the name of this runtime
    fn name(&self) -> &'static str;

    /// Get the language this runtime executes
    fn lang(&self) -> Lang;

    /// Initialize the runtime with suite-level setup
    async fn initialize(&mut self, suite: &SuiteIR) -> Result<()>;

    /// Run a single benchmark and return measurements
    async fn run_benchmark(&mut self, spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<Measurement>;

    /// Cleanup the runtime
    async fn shutdown(&mut self) -> Result<()>;
}
