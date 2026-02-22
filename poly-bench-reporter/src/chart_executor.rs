//! Chart directive executor
//!
//! Executes chart directives after benchmarks complete, generating SVG charts
//! from the benchmark results.

use miette::{miette, Result};
use poly_bench_dsl::ChartType;
use poly_bench_executor::BenchmarkResults;
use poly_bench_ir::ChartDirectiveIR;
use std::path::Path;

use crate::charts::{speedup_chart, table};

/// Information about a generated chart
#[derive(Debug, Clone)]
pub struct GeneratedChart {
    /// Output file path
    pub path: String,
    /// Chart type
    pub chart_type: ChartType,
    /// Chart title
    pub title: String,
}

/// Execute all chart directives, generating SVG charts
pub fn execute_chart_directives(
    directives: &[ChartDirectiveIR],
    results: &BenchmarkResults,
    output_dir: &Path,
) -> Result<Vec<GeneratedChart>> {
    let mut generated = Vec::new();

    for directive in directives {
        let chart = execute_single_directive(directive, results, output_dir)?;
        generated.push(chart);
    }

    Ok(generated)
}

/// Execute a single chart directive
fn execute_single_directive(
    directive: &ChartDirectiveIR,
    results: &BenchmarkResults,
    output_dir: &Path,
) -> Result<GeneratedChart> {
    // Filter results to the relevant suite if specified
    let filtered_results = if let Some(ref suite_name) = directive.suite_name {
        filter_results_by_suite(results, suite_name)
    } else {
        results.clone()
    };

    // Generate the SVG content based on chart type
    let svg_content = match directive.chart_type {
        ChartType::SpeedupChart => generate_speedup_chart(directive, &filtered_results)?,
        ChartType::Table => generate_table(directive, &filtered_results)?,
    };

    // Ensure output directory exists
    std::fs::create_dir_all(output_dir)
        .map_err(|e| miette!("Failed to create output directory: {}", e))?;

    // Write the SVG file
    let output_path = output_dir.join(&directive.output_file);
    std::fs::write(&output_path, &svg_content)
        .map_err(|e| miette!("Failed to write chart file: {}", e))?;

    Ok(GeneratedChart {
        path: output_path.to_string_lossy().to_string(),
        chart_type: directive.chart_type,
        title: directive.get_title(),
    })
}

/// Filter results to only include benchmarks from a specific suite
fn filter_results_by_suite(results: &BenchmarkResults, suite_name: &str) -> BenchmarkResults {
    let filtered_suites: Vec<_> =
        results.suites.iter().filter(|s| s.name == suite_name).cloned().collect();

    BenchmarkResults::new(filtered_suites)
}

/// Generate a speedup chart SVG
fn generate_speedup_chart(
    directive: &ChartDirectiveIR,
    results: &BenchmarkResults,
) -> Result<String> {
    let benchmarks: Vec<_> = results.suites.iter().flat_map(|s| s.benchmarks.iter()).collect();
    Ok(speedup_chart::generate(benchmarks, directive))
}

/// Generate a data table SVG
fn generate_table(directive: &ChartDirectiveIR, results: &BenchmarkResults) -> Result<String> {
    let benchmarks: Vec<_> = results.suites.iter().flat_map(|s| s.benchmarks.iter()).collect();
    Ok(table::generate(benchmarks, directive))
}

#[cfg(test)]
mod tests {
    use super::*;
    use poly_bench_executor::comparison::{BenchmarkResult, SuiteResults};
    use std::collections::HashMap;

    fn make_test_results() -> BenchmarkResults {
        let benchmarks = vec![BenchmarkResult::new(
            "bench1".to_string(),
            "suite_bench1".to_string(),
            None,
            HashMap::new(),
        )];
        let suite = SuiteResults::new("suite".to_string(), None, benchmarks);
        BenchmarkResults::new(vec![suite])
    }

    #[test]
    fn test_filter_results_by_suite() {
        let results = make_test_results();
        let filtered = filter_results_by_suite(&results, "suite");
        assert_eq!(filtered.suites.len(), 1);

        let filtered_empty = filter_results_by_suite(&results, "nonexistent");
        assert_eq!(filtered_empty.suites.len(), 0);
    }
}
