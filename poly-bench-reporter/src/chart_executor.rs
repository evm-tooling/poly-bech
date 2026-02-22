//! Chart directive executor
//!
//! Executes chart directives after benchmarks complete, generating SVG charts
//! from the benchmark results.

use miette::{miette, Result};
use poly_bench_dsl::ChartType;
use poly_bench_executor::{
    comparison::{BenchmarkResult, SuiteResults},
    BenchmarkResults,
};
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
        // Generate the main combined chart
        let chart = execute_single_directive(directive, results, output_dir)?;
        generated.push(chart);

        // For SpeedupChart, also generate per-benchmark individual charts
        if directive.chart_type == ChartType::SpeedupChart {
            let per_bench_charts = execute_per_benchmark_charts(directive, results, output_dir)?;
            generated.extend(per_bench_charts);
        }
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

/// Generate individual speedup charts for each benchmark in the suite
/// Output to a subdirectory named after the output file (minus extension)
fn execute_per_benchmark_charts(
    directive: &ChartDirectiveIR,
    results: &BenchmarkResults,
    output_dir: &Path,
) -> Result<Vec<GeneratedChart>> {
    let mut generated = Vec::new();

    // Filter results to the relevant suite if specified
    let filtered_results = if let Some(ref suite_name) = directive.suite_name {
        filter_results_by_suite(results, suite_name)
    } else {
        results.clone()
    };

    // Derive subdirectory name from output file (e.g., "bubble-line.svg" -> "bubble-line")
    let subdir_name = directive.output_file.strip_suffix(".svg").unwrap_or(&directive.output_file);

    let per_bench_dir = output_dir.join(subdir_name);

    // Create the subdirectory
    std::fs::create_dir_all(&per_bench_dir)
        .map_err(|e| miette!("Failed to create per-benchmark directory: {}", e))?;

    // Collect all benchmarks
    let all_benchmarks: Vec<&BenchmarkResult> =
        filtered_results.suites.iter().flat_map(|s| s.benchmarks.iter()).collect();

    // Generate a chart for each benchmark
    for bench in &all_benchmarks {
        // Create a modified directive for this single benchmark
        let mut single_directive = directive.clone();

        // Set title to benchmark name if not overridden
        single_directive.title = Some(bench.name.clone());

        // Output file for this benchmark
        let safe_name = sanitize_filename(&bench.name);
        single_directive.output_file = format!("{}.svg", safe_name);

        // Use compact width for single-benchmark charts, let height auto-calculate
        single_directive.width = single_directive.width.or(Some(520));

        // Create results with just this benchmark
        let single_results = create_single_benchmark_results(&filtered_results, bench);

        // Generate the SVG content
        let svg_content = generate_speedup_chart(&single_directive, &single_results)?;

        // Write the SVG file
        let output_path = per_bench_dir.join(&single_directive.output_file);
        std::fs::write(&output_path, &svg_content)
            .map_err(|e| miette!("Failed to write per-benchmark chart: {}", e))?;

        generated.push(GeneratedChart {
            path: output_path.to_string_lossy().to_string(),
            chart_type: ChartType::SpeedupChart,
            title: bench.name.clone(),
        });
    }

    Ok(generated)
}

/// Create BenchmarkResults containing only a single benchmark
fn create_single_benchmark_results(
    original: &BenchmarkResults,
    bench: &BenchmarkResult,
) -> BenchmarkResults {
    // Find the suite this benchmark belongs to and create a copy with just this benchmark
    for suite in &original.suites {
        if suite.benchmarks.iter().any(|b| b.full_name == bench.full_name) {
            let single_bench = bench.clone();
            let single_suite = SuiteResults::new(
                suite.name.clone(),
                suite.description.clone(),
                vec![single_bench],
            );
            return BenchmarkResults::new(vec![single_suite]);
        }
    }

    // Fallback: create a generic suite
    let single_bench = bench.clone();
    let single_suite = SuiteResults::new("benchmark".to_string(), None, vec![single_bench]);
    BenchmarkResults::new(vec![single_suite])
}

/// Sanitize a benchmark name for use as a filename
fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect()
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
    use poly_bench_executor::comparison::BenchmarkResult;
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

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("n100"), "n100");
        assert_eq!(sanitize_filename("bench-test"), "bench-test");
        assert_eq!(sanitize_filename("bench test"), "bench_test");
        assert_eq!(sanitize_filename("a/b\\c:d"), "a_b_c_d");
    }

    #[test]
    fn test_create_single_benchmark_results() {
        let results = make_test_results();
        let bench = &results.suites[0].benchmarks[0];
        let single = create_single_benchmark_results(&results, bench);
        assert_eq!(single.suites.len(), 1);
        assert_eq!(single.suites[0].benchmarks.len(), 1);
        assert_eq!(single.suites[0].benchmarks[0].name, "bench1");
    }
}
