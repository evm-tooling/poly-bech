//! Cross-language comparison types and logic

use poly_bench_dsl::Lang;
use poly_bench_runtime::measurement::{Measurement, Comparison, ComparisonWinner};
use serde::{Serialize, Deserialize};
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
    pub fn new(name: String, description: Option<String>, benchmarks: Vec<BenchmarkResult>) -> Self {
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
    /// Description
    pub description: Option<String>,
    /// Measurements by language
    pub measurements: HashMap<Lang, Measurement>,
    /// Comparison (if multiple languages)
    pub comparison: Option<Comparison>,
}

impl BenchmarkResult {
    pub fn new(
        name: String,
        full_name: String,
        description: Option<String>,
        measurements: HashMap<Lang, Measurement>,
    ) -> Self {
        let comparison = Self::calculate_comparison(&measurements);
        Self { name, full_name, description, measurements, comparison }
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
            
            if let Some(ref comparison) = bench.comparison {
                match comparison.winner {
                    ComparisonWinner::First => go_wins += 1,
                    ComparisonWinner::Second => ts_wins += 1,
                    ComparisonWinner::Tie => ties += 1,
                }

                // For geometric mean: log of (ts_time / go_time)
                // > 0 means Go is faster
                let go_ns = comparison.first.nanos_per_op;
                let ts_ns = comparison.second.nanos_per_op;
                if go_ns > 0.0 && ts_ns > 0.0 {
                    log_speedups.push((ts_ns / go_ns).ln());
                }
            }
        }

        let geo_mean_speedup = if !log_speedups.is_empty() {
            let avg_log = log_speedups.iter().sum::<f64>() / log_speedups.len() as f64;
            avg_log.exp()
        } else {
            1.0
        };

        let winner = if (geo_mean_speedup - 1.0).abs() < 0.05 {
            None
        } else if geo_mean_speedup > 1.0 {
            Some(Lang::Go)
        } else {
            Some(Lang::TypeScript)
        };

        Self {
            total_benchmarks: benchmarks.len(),
            go_wins,
            ts_wins,
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
        let mut ties = 0;
        let mut unstable_count = 0;
        let mut total_outliers_removed = 0u64;
        let mut log_speedups = Vec::new();

        for suite in suites {
            total_benchmarks += suite.benchmarks.len();
            go_wins += suite.summary.go_wins;
            ts_wins += suite.summary.ts_wins;
            ties += suite.summary.ties;
            unstable_count += suite.summary.unstable_count;
            total_outliers_removed += suite.summary.total_outliers_removed;

            for bench in &suite.benchmarks {
                if let Some(ref comparison) = bench.comparison {
                    let go_ns = comparison.first.nanos_per_op;
                    let ts_ns = comparison.second.nanos_per_op;
                    if go_ns > 0.0 && ts_ns > 0.0 {
                        log_speedups.push((ts_ns / go_ns).ln());
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

        let (winner, winner_description) = if (geo_mean_speedup - 1.0).abs() < 0.05 {
            (None, "Similar performance".to_string())
        } else if geo_mean_speedup > 1.0 {
            (Some(Lang::Go), format!("Go is {:.2}x faster overall", geo_mean_speedup))
        } else {
            (Some(Lang::TypeScript), format!("TypeScript is {:.2}x faster overall", 1.0 / geo_mean_speedup))
        };

        Self {
            total_suites,
            total_benchmarks,
            go_wins,
            ts_wins,
            ties,
            geo_mean_speedup,
            winner,
            winner_description,
            unstable_count,
            total_outliers_removed,
        }
    }
}
