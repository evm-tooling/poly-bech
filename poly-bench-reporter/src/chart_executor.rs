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

use crate::charts::{bar_chart, line_chart, speedup_chart, table};

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
        // Generate the main combined chart (and wide SVG when bar chart is wide)
        let charts = execute_single_directive(directive, results, output_dir)?;
        generated.extend(charts);

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
) -> Result<Vec<GeneratedChart>> {
    // Filter results to the relevant suite if specified
    let filtered_results = if let Some(ref suite_name) = directive.suite_name {
        filter_results_by_suite(results, suite_name)
    } else {
        results.clone()
    };

    // Generate the chart content and determine output path
    let (content, output_file, wide_svg) = match directive.chart_type {
        ChartType::SpeedupChart => {
            let c = generate_speedup_chart(directive, &filtered_results)?;
            (c, directive.output_file.clone(), None)
        }
        ChartType::Table => {
            let c = generate_table(directive, &filtered_results)?;
            (c, directive.output_file.clone(), None)
        }
        ChartType::LineChart => {
            let c = generate_line_chart(directive, &filtered_results)?;
            (c, directive.output_file.clone(), None)
        }
        ChartType::BarChart => {
            let out = generate_bar_chart(directive, &filtered_results)?;
            let output_file = if out.is_html {
                directive.output_file.replace(".svg", ".html")
            } else {
                directive.output_file.clone()
            };
            (out.content, output_file, out.wide_svg)
        }
    };

    // Ensure output directory exists
    std::fs::create_dir_all(output_dir)
        .map_err(|e| miette!("Failed to create output directory: {}", e))?;

    // Write the chart file
    let output_path = output_dir.join(&output_file);
    std::fs::write(&output_path, &content)
        .map_err(|e| miette!("Failed to write chart file: {}", e))?;

    let mut charts = vec![GeneratedChart {
        path: output_path.to_string_lossy().to_string(),
        chart_type: directive.chart_type,
        title: directive.get_title(),
    }];

    // When bar chart is wide, also write full-width SVG for users who prefer it
    if let Some(svg) = wide_svg {
        let base = output_file.strip_suffix(".html").unwrap_or(&output_file);
        let wide_path = format!("{}-wide.svg", base);
        let wide_full = output_dir.join(&wide_path);
        std::fs::write(&wide_full, &svg).map_err(|e| miette!("Failed to write wide SVG: {}", e))?;
        charts.push(GeneratedChart {
            path: wide_full.to_string_lossy().to_string(),
            chart_type: directive.chart_type,
            title: format!("{} (wide)", directive.get_title()),
        });
    }

    Ok(charts)
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

        // Use wider default for single-benchmark charts (matches SPEEDUP_BAR_WIDTH_FACTOR)
        single_directive.width = single_directive.width.or(Some(936));

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
                suite.suite_type,
                vec![single_bench],
            );
            return BenchmarkResults::new(vec![single_suite]);
        }
    }

    // Fallback: create a generic suite
    let single_bench = bench.clone();
    let single_suite = SuiteResults::new(
        "benchmark".to_string(),
        None,
        poly_bench_dsl::SuiteType::Performance,
        vec![single_bench],
    );
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

/// Get suite type from results (for memory vs performance chart mode)
fn suite_type_from_results(results: &BenchmarkResults) -> poly_bench_dsl::SuiteType {
    results.suites.first().map(|s| s.suite_type).unwrap_or(poly_bench_dsl::SuiteType::Performance)
}

/// Generate a speedup chart SVG
fn generate_speedup_chart(
    directive: &ChartDirectiveIR,
    results: &BenchmarkResults,
) -> Result<String> {
    let benchmarks: Vec<_> = results.suites.iter().flat_map(|s| s.benchmarks.iter()).collect();
    let suite_type = suite_type_from_results(results);
    Ok(speedup_chart::generate(benchmarks, directive, suite_type))
}

/// Generate a data table SVG
fn generate_table(directive: &ChartDirectiveIR, results: &BenchmarkResults) -> Result<String> {
    let benchmarks: Vec<_> = results.suites.iter().flat_map(|s| s.benchmarks.iter()).collect();
    let suite_type = suite_type_from_results(results);
    Ok(table::generate(benchmarks, directive, suite_type))
}

/// Generate a line chart SVG
fn generate_line_chart(directive: &ChartDirectiveIR, results: &BenchmarkResults) -> Result<String> {
    let benchmarks: Vec<_> = results.suites.iter().flat_map(|s| s.benchmarks.iter()).collect();
    let suite_type = suite_type_from_results(results);
    Ok(line_chart::generate(benchmarks, directive, suite_type))
}

/// Generate a bar chart (SVG or HTML with scrollable wrapper when wide)
fn generate_bar_chart(
    directive: &ChartDirectiveIR,
    results: &BenchmarkResults,
) -> Result<bar_chart::BarChartOutput> {
    let benchmarks: Vec<_> = results.suites.iter().flat_map(|s| s.benchmarks.iter()).collect();
    let suite_type = suite_type_from_results(results);
    Ok(bar_chart::generate(benchmarks, directive, suite_type))
}

#[cfg(test)]
mod tests {
    use super::*;
    use poly_bench_dsl::BenchmarkKind;
    use poly_bench_executor::comparison::BenchmarkResult;
    use poly_bench_ir::ChartDirectiveIR;
    use poly_bench_runtime::measurement::Measurement;
    use std::collections::HashMap;

    fn make_test_results() -> BenchmarkResults {
        let mut measurements = HashMap::new();
        measurements.insert(
            poly_bench_dsl::Lang::Go,
            Measurement {
                iterations: 100,
                total_nanos: 100_000,
                warmup_nanos: None,
                spawn_nanos: None,
                nanos_per_op: 1000.0,
                ops_per_sec: 1_000_000.0,
                min_nanos: None,
                max_nanos: None,
                p50_nanos: None,
                p75_nanos: None,
                p99_nanos: None,
                p995_nanos: None,
                rme_percent: None,
                samples: None,
                bytes_per_op: None,
                allocs_per_op: None,
                raw_samples: Some(vec![900, 1000, 1100]),
                raw_result: None,
                successful_results: None,
                async_success_count: None,
                async_error_count: None,
                async_error_samples: None,
                cv_percent: None,
                outliers_removed: None,
                is_stable: None,
                run_count: Some(3),
                median_across_runs: None,
                ci_95_lower: Some(900.0),
                ci_95_upper: Some(1100.0),
                std_dev_nanos: Some(100.0),
                estimator_source: None,
                raw_nanos_per_op: None,
                filtered_nanos_per_op: None,
                timed_out: None,
                run_nanos_per_op: None,
            },
        );
        measurements.insert(
            poly_bench_dsl::Lang::TypeScript,
            Measurement {
                iterations: 100,
                total_nanos: 130_000,
                warmup_nanos: None,
                spawn_nanos: None,
                nanos_per_op: 1300.0,
                ops_per_sec: 769_230.0,
                min_nanos: None,
                max_nanos: None,
                p50_nanos: None,
                p75_nanos: None,
                p99_nanos: None,
                p995_nanos: None,
                rme_percent: None,
                samples: None,
                bytes_per_op: None,
                allocs_per_op: None,
                raw_samples: Some(vec![1200, 1300, 1400]),
                raw_result: None,
                successful_results: None,
                async_success_count: None,
                async_error_count: None,
                async_error_samples: None,
                cv_percent: None,
                outliers_removed: None,
                is_stable: None,
                run_count: Some(3),
                median_across_runs: None,
                ci_95_lower: Some(1200.0),
                ci_95_upper: Some(1400.0),
                std_dev_nanos: Some(100.0),
                estimator_source: None,
                raw_nanos_per_op: None,
                filtered_nanos_per_op: None,
                timed_out: None,
                run_nanos_per_op: None,
            },
        );

        let benchmarks = vec![BenchmarkResult::new(
            "bench1".to_string(),
            "suite_bench1".to_string(),
            BenchmarkKind::Sync,
            None,
            measurements,
            poly_bench_dsl::SuiteType::Performance,
            "legacy".to_string(),
            None,
            None,
            None,
            None,
        )];
        let suite = SuiteResults::new(
            "suite".to_string(),
            None,
            poly_bench_dsl::SuiteType::Performance,
            benchmarks,
        );
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

    #[test]
    fn test_execute_line_chart_directive() {
        let results = make_test_results();
        let mut directive =
            ChartDirectiveIR::new(ChartType::LineChart, "line-test.svg".to_string());
        directive.title = Some("Line".to_string());
        let out_dir = std::env::temp_dir().join("polybench_chart_executor_line");
        let generated = execute_chart_directives(&[directive], &results, &out_dir).unwrap();
        assert_eq!(generated.len(), 1);
        assert_eq!(generated[0].chart_type, ChartType::LineChart);
    }

    #[test]
    fn test_execute_bar_chart_directive() {
        let results = make_test_results();
        let mut directive = ChartDirectiveIR::new(ChartType::BarChart, "bar-test.svg".to_string());
        directive.title = Some("Bar".to_string());
        directive.width = Some(600); // Narrow chart → SVG only, no wide export
        let out_dir = std::env::temp_dir().join("polybench_chart_executor_bar");
        let generated = execute_chart_directives(&[directive], &results, &out_dir).unwrap();
        assert_eq!(generated.len(), 1);
        assert_eq!(generated[0].chart_type, ChartType::BarChart);
    }

    #[test]
    fn test_execute_wide_bar_chart_outputs_html() {
        let results = make_wide_bar_chart_results();
        let directive = ChartDirectiveIR::new(ChartType::BarChart, "wide-bar.svg".to_string());
        let out_dir = std::env::temp_dir().join("polybench_chart_executor_wide_bar");
        let generated = execute_chart_directives(&[directive], &results, &out_dir).unwrap();
        assert_eq!(generated.len(), 2, "wide bar chart should output HTML + wide SVG");
        let html_chart = generated.iter().find(|c| c.path.ends_with(".html")).unwrap();
        let wide_chart = generated.iter().find(|c| c.path.ends_with("-wide.svg")).unwrap();
        let content = std::fs::read_to_string(&html_chart.path).unwrap();
        assert!(content.contains("<!DOCTYPE html"));
        assert!(content.contains("chart-scroll"));
        let svg_content = std::fs::read_to_string(&wide_chart.path).unwrap();
        assert!(svg_content.starts_with("<svg"), "wide SVG should be raw SVG");
    }

    fn make_wide_bar_chart_results() -> BenchmarkResults {
        let mut benchmarks = Vec::new();
        for i in 1..=15 {
            let mut measurements = HashMap::new();
            measurements.insert(
                poly_bench_dsl::Lang::Go,
                Measurement {
                    iterations: 100,
                    total_nanos: (i as u64) * 100_000,
                    warmup_nanos: None,
                    spawn_nanos: None,
                    nanos_per_op: (i as f64) * 1000.0,
                    ops_per_sec: 1_000_000.0 / i as f64,
                    min_nanos: None,
                    max_nanos: None,
                    p50_nanos: None,
                    p75_nanos: None,
                    p99_nanos: None,
                    p995_nanos: None,
                    rme_percent: None,
                    samples: None,
                    bytes_per_op: None,
                    allocs_per_op: None,
                    raw_samples: Some(vec![900, 1000, 1100]),
                    raw_result: None,
                    successful_results: None,
                    async_success_count: None,
                    async_error_count: None,
                    async_error_samples: None,
                    cv_percent: None,
                    outliers_removed: None,
                    is_stable: None,
                    run_count: Some(3),
                    median_across_runs: None,
                    ci_95_lower: Some(900.0),
                    ci_95_upper: Some(1100.0),
                    std_dev_nanos: Some(100.0),
                    estimator_source: None,
                    raw_nanos_per_op: None,
                    filtered_nanos_per_op: None,
                    timed_out: None,
                    run_nanos_per_op: None,
                },
            );
            measurements.insert(
                poly_bench_dsl::Lang::TypeScript,
                Measurement {
                    iterations: 100,
                    total_nanos: (i as u64) * 130_000,
                    warmup_nanos: None,
                    spawn_nanos: None,
                    nanos_per_op: (i as f64) * 1300.0,
                    ops_per_sec: 769_230.0 / i as f64,
                    min_nanos: None,
                    max_nanos: None,
                    p50_nanos: None,
                    p75_nanos: None,
                    p99_nanos: None,
                    p995_nanos: None,
                    rme_percent: None,
                    samples: None,
                    bytes_per_op: None,
                    allocs_per_op: None,
                    raw_samples: Some(vec![1200, 1300, 1400]),
                    raw_result: None,
                    successful_results: None,
                    async_success_count: None,
                    async_error_count: None,
                    async_error_samples: None,
                    cv_percent: None,
                    outliers_removed: None,
                    is_stable: None,
                    run_count: Some(3),
                    median_across_runs: None,
                    ci_95_lower: Some(1200.0),
                    ci_95_upper: Some(1400.0),
                    std_dev_nanos: Some(100.0),
                    estimator_source: None,
                    raw_nanos_per_op: None,
                    filtered_nanos_per_op: None,
                    timed_out: None,
                    run_nanos_per_op: None,
                },
            );
            benchmarks.push(BenchmarkResult::new(
                format!("bench{}", i * 100),
                format!("suite_bench{}", i * 100),
                poly_bench_dsl::BenchmarkKind::Sync,
                None,
                measurements,
                poly_bench_dsl::SuiteType::Performance,
                "legacy".to_string(),
                None,
                None,
                None,
                None,
            ));
        }
        let suite = SuiteResults::new(
            "suite".to_string(),
            None,
            poly_bench_dsl::SuiteType::Performance,
            benchmarks,
        );
        BenchmarkResults::new(vec![suite])
    }
}
