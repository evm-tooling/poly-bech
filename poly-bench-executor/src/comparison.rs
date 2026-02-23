//! Cross-language comparison types and logic

use poly_bench_dsl::{BenchmarkKind, Lang};
use poly_bench_runtime::measurement::{Comparison, Measurement};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Results from running all benchmarks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResults {
    /// Results organized by suite
    pub suites: Vec<SuiteResults>,
    /// Overall statistics
    pub summary: OverallSummary,
}

impl BenchmarkResults {
    pub fn new(suites: Vec<SuiteResults>) -> Self {
        let summary = OverallSummary::calculate(&suites);
        Self { suites, summary }
    }
}

/// Results for a single suite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteResults {
    /// Suite name
    pub name: String,
    /// Suite description
    pub description: Option<String>,
    /// Individual benchmark results
    pub benchmarks: Vec<BenchmarkResult>,
    /// Suite-level summary
    pub summary: SuiteSummary,
}

impl SuiteResults {
    pub fn new(
        name: String,
        description: Option<String>,
        benchmarks: Vec<BenchmarkResult>,
    ) -> Self {
        let summary = SuiteSummary::calculate(&benchmarks);
        Self { name, description, benchmarks, summary }
    }
}

/// Result for a single benchmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// Benchmark name
    pub name: String,
    /// Full qualified name
    pub full_name: String,
    /// Benchmark kind (sync vs async-sequential)
    pub kind: BenchmarkKind,
    /// Description
    pub description: Option<String>,
    /// Measurements by language
    pub measurements: HashMap<Lang, Measurement>,
    /// Comparison (if multiple languages)
    pub comparison: Option<Comparison>,
    /// Extra metadata for async benchmarks (benchAsync)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub async_details: Option<AsyncBenchmarkDetails>,
}

/// Extra output included for async benchmarks in `results.json`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsyncBenchmarkDetails {
    /// Execution semantics for async benchmarks
    pub mode: String,
    /// Internal warmup cap used by runtimes
    pub warmup_cap: u64,
    /// Internal sample cap used by runtimes
    pub sample_cap: u64,
    /// Actual iterations executed per language
    pub actual_iterations: HashMap<Lang, u64>,
    /// Actual samples captured per language
    pub actual_samples: HashMap<Lang, u64>,
}

impl BenchmarkResult {
    pub fn new(
        name: String,
        full_name: String,
        kind: BenchmarkKind,
        description: Option<String>,
        measurements: HashMap<Lang, Measurement>,
    ) -> Self {
        let comparison = Self::calculate_comparison(&measurements);
        let async_details = Self::build_async_details(kind, &measurements);
        Self { name, full_name, kind, description, measurements, comparison, async_details }
    }

    fn calculate_comparison(measurements: &HashMap<Lang, Measurement>) -> Option<Comparison> {
        // Compare Go vs TypeScript if both are present
        let go_measurement = measurements.get(&Lang::Go)?;
        let ts_measurement = measurements.get(&Lang::TypeScript)?;

        Some(Comparison::new(
            String::new(), // Will be set by caller
            go_measurement.clone(),
            "Go".to_string(),
            ts_measurement.clone(),
            "TypeScript".to_string(),
        ))
    }

    fn build_async_details(
        kind: BenchmarkKind,
        measurements: &HashMap<Lang, Measurement>,
    ) -> Option<AsyncBenchmarkDetails> {
        if kind != BenchmarkKind::Async {
            return None;
        }

        let mut actual_iterations = HashMap::new();
        let mut actual_samples = HashMap::new();
        for (lang, measurement) in measurements {
            actual_iterations.insert(*lang, measurement.iterations);
            actual_samples.insert(*lang, measurement.samples.unwrap_or(0));
        }

        Some(AsyncBenchmarkDetails {
            mode: "async-sequential".to_string(),
            warmup_cap: 5,
            sample_cap: 50,
            actual_iterations,
            actual_samples,
        })
    }
}

/// Summary statistics for a suite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteSummary {
    /// Total benchmarks
    pub total_benchmarks: usize,
    /// Go wins
    pub go_wins: usize,
    /// TypeScript wins
    pub ts_wins: usize,
    /// Rust wins
    pub rust_wins: usize,
    /// Ties
    pub ties: usize,
    /// Geometric mean speedup (>1 means Go is faster)
    pub geo_mean_speedup: f64,
    /// Winner
    pub winner: Option<Lang>,
    /// Number of benchmarks with unstable results (CV > threshold)
    pub unstable_count: usize,
    /// Total outliers removed across all measurements
    pub total_outliers_removed: u64,
}

impl SuiteSummary {
    fn calculate(benchmarks: &[BenchmarkResult]) -> Self {
        let mut go_wins = 0;
        let mut ts_wins = 0;
        let mut rust_wins = 0;
        let mut ties = 0;
        let mut log_speedups = Vec::new();
        let mut unstable_count = 0;
        let mut total_outliers_removed = 0u64;

        for bench in benchmarks {
            // Count stability issues across all measurements
            for measurement in bench.measurements.values() {
                if let Some(false) = measurement.is_stable {
                    unstable_count += 1;
                }
                if let Some(outliers) = measurement.outliers_removed {
                    total_outliers_removed += outliers;
                }
            }

            // Determine winner across all available languages
            let go_ns = bench.measurements.get(&Lang::Go).map(|m| m.nanos_per_op);
            let ts_ns = bench.measurements.get(&Lang::TypeScript).map(|m| m.nanos_per_op);
            let rust_ns = bench.measurements.get(&Lang::Rust).map(|m| m.nanos_per_op);

            // Find the fastest language among those present
            let mut times: Vec<(Lang, f64)> = vec![];
            if let Some(ns) = go_ns {
                times.push((Lang::Go, ns));
            }
            if let Some(ns) = ts_ns {
                times.push((Lang::TypeScript, ns));
            }
            if let Some(ns) = rust_ns {
                times.push((Lang::Rust, ns));
            }

            if times.len() >= 2 {
                // Sort by time (fastest first)
                times.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
                let (fastest_lang, fastest_time) = times[0];
                let (_, second_time) = times[1];

                // Check for tie (within 5%)
                let speedup = second_time / fastest_time;
                if speedup < 1.05 {
                    ties += 1;
                } else {
                    match fastest_lang {
                        Lang::Go => go_wins += 1,
                        Lang::TypeScript => ts_wins += 1,
                        Lang::Rust => rust_wins += 1,
                        _ => {}
                    }
                }

                // For geometric mean: use Go vs TS comparison if both present (for backwards
                // compatibility)
                if let (Some(go), Some(ts)) = (go_ns, ts_ns) {
                    if go > 0.0 && ts > 0.0 {
                        log_speedups.push((ts / go).ln());
                    }
                }
            }
        }

        let geo_mean_speedup = if !log_speedups.is_empty() {
            let avg_log = log_speedups.iter().sum::<f64>() / log_speedups.len() as f64;
            avg_log.exp()
        } else {
            1.0
        };

        // Determine overall winner by most wins
        let winner = if go_wins == ts_wins && go_wins == rust_wins {
            None
        } else if go_wins >= ts_wins && go_wins >= rust_wins {
            Some(Lang::Go)
        } else if ts_wins >= go_wins && ts_wins >= rust_wins {
            Some(Lang::TypeScript)
        } else {
            Some(Lang::Rust)
        };

        Self {
            total_benchmarks: benchmarks.len(),
            go_wins,
            ts_wins,
            rust_wins,
            ties,
            geo_mean_speedup,
            winner,
            unstable_count,
            total_outliers_removed,
        }
    }
}

/// Overall summary across all suites
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverallSummary {
    /// Total suites
    pub total_suites: usize,
    /// Total benchmarks
    pub total_benchmarks: usize,
    /// Go wins
    pub go_wins: usize,
    /// TypeScript wins
    pub ts_wins: usize,
    /// Rust wins
    pub rust_wins: usize,
    /// Ties
    pub ties: usize,
    /// Geometric mean speedup across all benchmarks
    pub geo_mean_speedup: f64,
    /// Overall winner
    pub winner: Option<Lang>,
    /// Winner description
    pub winner_description: String,
    /// Number of measurements with unstable results (CV > threshold)
    pub unstable_count: usize,
    /// Total outliers removed across all measurements
    pub total_outliers_removed: u64,
}

impl OverallSummary {
    fn calculate(suites: &[SuiteResults]) -> Self {
        let total_suites = suites.len();
        let mut total_benchmarks = 0;
        let mut go_wins = 0;
        let mut ts_wins = 0;
        let mut rust_wins = 0;
        let mut ties = 0;
        let mut unstable_count = 0;
        let mut total_outliers_removed = 0u64;
        let mut log_speedups = Vec::new();

        for suite in suites {
            total_benchmarks += suite.benchmarks.len();
            go_wins += suite.summary.go_wins;
            ts_wins += suite.summary.ts_wins;
            rust_wins += suite.summary.rust_wins;
            ties += suite.summary.ties;
            unstable_count += suite.summary.unstable_count;
            total_outliers_removed += suite.summary.total_outliers_removed;

            // For geometric mean, use Go vs TS comparison if both present
            for bench in &suite.benchmarks {
                let go_ns = bench.measurements.get(&Lang::Go).map(|m| m.nanos_per_op);
                let ts_ns = bench.measurements.get(&Lang::TypeScript).map(|m| m.nanos_per_op);
                if let (Some(go), Some(ts)) = (go_ns, ts_ns) {
                    if go > 0.0 && ts > 0.0 {
                        log_speedups.push((ts / go).ln());
                    }
                }
            }
        }

        let geo_mean_speedup = if !log_speedups.is_empty() {
            let avg_log = log_speedups.iter().sum::<f64>() / log_speedups.len() as f64;
            avg_log.exp()
        } else {
            1.0
        };

        // Determine winner by most wins
        let (winner, winner_description) =
            if go_wins == ts_wins && go_wins == rust_wins && go_wins == 0 {
                (None, "No benchmark results".to_string())
            } else if go_wins == ts_wins && go_wins == rust_wins {
                (None, "Similar performance".to_string())
            } else if go_wins >= ts_wins && go_wins >= rust_wins {
                (Some(Lang::Go), format!("Go is {:.2}x faster overall", geo_mean_speedup))
            } else if ts_wins >= go_wins && ts_wins >= rust_wins {
                (
                    Some(Lang::TypeScript),
                    format!("TypeScript is {:.2}x faster overall", 1.0 / geo_mean_speedup),
                )
            } else {
                // Rust wins - calculate Rust's speedup vs the average of Go/TS
                let rust_desc = format!("Rust wins {} benchmarks", rust_wins);
                (Some(Lang::Rust), rust_desc)
            };

        Self {
            total_suites,
            total_benchmarks,
            go_wins,
            ts_wins,
            rust_wins,
            ties,
            geo_mean_speedup,
            winner,
            winner_description,
            unstable_count,
            total_outliers_removed,
        }
    }
}
