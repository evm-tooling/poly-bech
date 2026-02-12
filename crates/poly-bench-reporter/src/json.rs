//! JSON report generator

use poly_bench_executor::BenchmarkResults;
use miette::Result;

/// Generate JSON report
pub fn report(results: &BenchmarkResults) -> Result<String> {
    serde_json::to_string_pretty(results)
        .map_err(|e| miette::miette!("Failed to serialize results: {}", e))
}
