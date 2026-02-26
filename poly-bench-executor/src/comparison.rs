//! Cross-language comparison types and logic

use poly_bench_dsl::{BenchmarkKind, Lang, SuiteType};
use poly_bench_runtime::{
    measurement::{Comparison, Measurement},
    supported_languages,
};
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
    /// Suite type (memory vs performance) - affects comparison and display
    pub suite_type: SuiteType,
    /// Individual benchmark results
    pub benchmarks: Vec<BenchmarkResult>,
    /// Suite-level summary
    pub summary: SuiteSummary,
}

impl SuiteResults {
    pub fn new(
        name: String,
        description: Option<String>,
        suite_type: SuiteType,
        benchmarks: Vec<BenchmarkResult>,
    ) -> Self {
        let summary = SuiteSummary::calculate(&benchmarks, suite_type);
        Self { name, description, suite_type, benchmarks, summary }
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
    /// Comparison/statistics mode used for this benchmark result
    pub comparison_mode: String,
    /// Optional fairness seed used for randomized/interleaved execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fairness_seed: Option<u64>,
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
    /// Async sampling policy used by runtimes
    pub sampling_policy: String,
    /// Actual iterations executed per language
    pub actual_iterations: HashMap<Lang, u64>,
    /// Actual samples captured per language
    pub actual_samples: HashMap<Lang, u64>,
    /// Successful async iterations captured per language
    pub successful_iterations: HashMap<Lang, u64>,
    /// Failed async iterations captured per language
    pub error_iterations: HashMap<Lang, u64>,
    /// Success ratio (successful / total attempts) per language
    pub success_ratio: HashMap<Lang, f64>,
    /// Error ratio (failed / total attempts) per language
    pub error_ratio: HashMap<Lang, f64>,
}

impl BenchmarkResult {
    pub fn new(
        name: String,
        full_name: String,
        kind: BenchmarkKind,
        description: Option<String>,
        measurements: HashMap<Lang, Measurement>,
        suite_type: SuiteType,
        comparison_mode: String,
        fairness_seed: Option<u64>,
        async_warmup_cap: Option<u64>,
        async_sample_cap: Option<u64>,
        async_sampling_policy: Option<String>,
    ) -> Self {
        let comparison = Self::calculate_comparison(&measurements, suite_type);
        let async_details = Self::build_async_details(
            kind,
            &measurements,
            async_warmup_cap,
            async_sample_cap,
            async_sampling_policy,
        );
        Self {
            name,
            full_name,
            kind,
            description,
            measurements,
            comparison,
            async_details,
            comparison_mode,
            fairness_seed,
        }
    }

    fn calculate_comparison(
        measurements: &HashMap<Lang, Measurement>,
        suite_type: SuiteType,
    ) -> Option<Comparison> {
        // Use first two languages from supported_languages() that have measurements
        let (first_lang, second_lang) = {
            let langs: Vec<Lang> = supported_languages()
                .iter()
                .copied()
                .filter(|l| measurements.contains_key(l))
                .take(2)
                .collect();
            if langs.len() < 2 {
                return None;
            }
            (langs[0], langs[1])
        };

        let first_measurement = measurements.get(&first_lang)?;
        let second_measurement = measurements.get(&second_lang)?;

        let use_memory = suite_type == SuiteType::Memory;
        let (first_val, second_val) = if use_memory {
            match (
                first_measurement.bytes_per_op,
                second_measurement.bytes_per_op,
            ) {
                (Some(a), Some(b)) => (a as f64, b as f64),
                _ => (
                    first_measurement.nanos_per_op,
                    second_measurement.nanos_per_op,
                ),
            }
        } else {
            (
                first_measurement.nanos_per_op,
                second_measurement.nanos_per_op,
            )
        };

        let ratio_ci_95 = if !use_memory {
            match (
                &first_measurement.run_nanos_per_op,
                &second_measurement.run_nanos_per_op,
            ) {
                (Some(a), Some(b)) => {
                    Measurement::paired_ratio_ci(a, b).map(|(_, lo, hi)| (lo, hi))
                }
                _ => None,
            }
        } else {
            None
        };

        let first_name = poly_bench_runtime::lang_full_name(first_lang).to_string();
        let second_name = poly_bench_runtime::lang_full_name(second_lang).to_string();

        Some(Comparison::new_with_metric(
            String::new(),
            first_measurement.clone(),
            first_name,
            second_measurement.clone(),
            second_name,
            first_val,
            second_val,
            ratio_ci_95,
        ))
    }

    fn build_async_details(
        kind: BenchmarkKind,
        measurements: &HashMap<Lang, Measurement>,
        async_warmup_cap: Option<u64>,
        async_sample_cap: Option<u64>,
        async_sampling_policy: Option<String>,
    ) -> Option<AsyncBenchmarkDetails> {
        if kind != BenchmarkKind::Async {
            return None;
        }

        let mut actual_iterations = HashMap::new();
        let mut actual_samples = HashMap::new();
        let mut successful_iterations = HashMap::new();
        let mut error_iterations = HashMap::new();
        let mut success_ratio = HashMap::new();
        let mut error_ratio = HashMap::new();
        for (lang, measurement) in measurements {
            actual_iterations.insert(*lang, measurement.iterations);
            actual_samples.insert(*lang, measurement.samples.unwrap_or(0));

            let mut success = measurement
                .async_success_count
                .or_else(|| measurement.successful_results.as_ref().map(|v| v.len() as u64))
                .unwrap_or(0);
            let mut errors = measurement.async_error_count.unwrap_or(0);
            if measurement.timed_out == Some(true) {
                errors = errors.saturating_add(1);
            }

            let total = success.saturating_add(errors);
            if total == 0 {
                if measurement.iterations > 0 {
                    success = measurement.iterations;
                } else if measurement.timed_out == Some(true) {
                    errors = 1;
                }
            }

            let total = success.saturating_add(errors);
            let success_pct = if total > 0 { success as f64 / total as f64 } else { 0.0 };
            let error_pct = if total > 0 { errors as f64 / total as f64 } else { 0.0 };
            successful_iterations.insert(*lang, success);
            error_iterations.insert(*lang, errors);
            success_ratio.insert(*lang, success_pct);
            error_ratio.insert(*lang, error_pct);
        }

        Some(AsyncBenchmarkDetails {
            mode: "async-sequential".to_string(),
            warmup_cap: async_warmup_cap.unwrap_or(5),
            sample_cap: async_sample_cap.unwrap_or(50),
            sampling_policy: async_sampling_policy.unwrap_or_else(|| "timeBudgeted".to_string()),
            actual_iterations,
            actual_samples,
            successful_iterations,
            error_iterations,
            success_ratio,
            error_ratio,
        })
    }
}

/// Summary statistics for a suite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteSummary {
    /// Total benchmarks
    pub total_benchmarks: usize,
    /// Wins per language
    pub lang_wins: HashMap<Lang, u32>,
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
    fn calculate(benchmarks: &[BenchmarkResult], suite_type: SuiteType) -> Self {
        let mut lang_wins: HashMap<Lang, u32> =
            supported_languages().iter().map(|&l| (l, 0)).collect();
        let mut ties = 0;
        let mut log_speedups = Vec::new();
        let mut unstable_count = 0;
        let mut total_outliers_removed = 0u64;
        let use_memory = suite_type == SuiteType::Memory;

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

            // Get primary metric: bytes_per_op for memory suite, nanos_per_op for performance
            let metric = |m: &Measurement| -> Option<f64> {
                if use_memory {
                    m.bytes_per_op.map(|b| b as f64)
                } else {
                    Some(m.nanos_per_op)
                }
            };

            // Collect values from all measurements (dynamic - no hardcoded langs)
            let mut values: Vec<(Lang, f64)> = bench
                .measurements
                .iter()
                .filter_map(|(lang, m)| metric(m).map(|v| (*lang, v)))
                .collect();

            if values.len() >= 2 {
                // Sort by value (lowest first)
                values.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
                let (best_lang, best_val) = values[0];
                let (_, second_val) = values[1];

                // Check for tie (within 5%)
                let speedup = second_val / best_val;
                if speedup < 1.05 {
                    ties += 1;
                } else {
                    lang_wins.entry(best_lang).and_modify(|c| *c += 1).or_insert(1);
                }

                // For geometric mean: use best vs second comparison
                if best_val > 0.0 && second_val > 0.0 {
                    log_speedups.push((second_val / best_val).ln());
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
        let winner = {
            let max_wins = lang_wins.values().max().copied().unwrap_or(0);
            let winners: Vec<Lang> =
                lang_wins.iter().filter(|(_, &w)| w == max_wins).map(|(&l, _)| l).collect();
            if winners.len() == 1 && max_wins > 0 {
                Some(winners[0])
            } else {
                None
            }
        };

        Self {
            total_benchmarks: benchmarks.len(),
            lang_wins,
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
    /// Wins per language
    pub lang_wins: HashMap<Lang, u32>,
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
        let mut lang_wins: HashMap<Lang, u32> =
            supported_languages().iter().map(|&l| (l, 0)).collect();
        let mut ties = 0;
        let mut unstable_count = 0;
        let mut total_outliers_removed = 0u64;
        let mut log_speedups = Vec::new();

        for suite in suites {
            total_benchmarks += suite.benchmarks.len();
            for (lang, wins) in &suite.summary.lang_wins {
                lang_wins.entry(*lang).and_modify(|c| *c += *wins).or_insert(*wins);
            }
            ties += suite.summary.ties;
            unstable_count += suite.summary.unstable_count;
            total_outliers_removed += suite.summary.total_outliers_removed;

            // For geometric mean, use best vs second comparison per benchmark
            let use_memory = suite.suite_type == SuiteType::Memory;
            let metric = |m: &Measurement| -> Option<f64> {
                if use_memory {
                    m.bytes_per_op.map(|b| b as f64)
                } else {
                    Some(m.nanos_per_op)
                }
            };
            for bench in &suite.benchmarks {
                let mut vals: Vec<f64> = bench
                    .measurements
                    .values()
                    .filter_map(metric)
                    .filter(|&v| v > 0.0)
                    .collect();
                vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                if vals.len() >= 2 {
                    let best = vals[0];
                    let second = vals[1];
                    if best > 0.0 && second > 0.0 {
                        log_speedups.push((second / best).ln());
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
        let (winner, winner_description) = {
            let max_wins = lang_wins.values().max().copied().unwrap_or(0);
            let winners: Vec<Lang> =
                lang_wins.iter().filter(|(_, &w)| w == max_wins).map(|(&l, _)| l).collect();
            if total_benchmarks == 0 {
                (None, "No benchmark results".to_string())
            } else if winners.len() > 1 || max_wins == 0 {
                (None, "Similar performance".to_string())
            } else {
                let win_lang = winners[0];
                let wins = lang_wins.get(&win_lang).copied().unwrap_or(0);
                let desc = format!(
                    "{} is {:.2}x faster overall ({} wins)",
                    poly_bench_runtime::lang_full_name(win_lang),
                    geo_mean_speedup,
                    wins
                );
                (Some(win_lang), desc)
            }
        };

        Self {
            total_suites,
            total_benchmarks,
            lang_wins,
            ties,
            geo_mean_speedup,
            winner,
            winner_description,
            unstable_count,
            total_outliers_removed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poly_bench_dsl::SuiteType;
    use poly_bench_runtime::measurement::Measurement;

    #[test]
    fn test_benchmark_result_metadata_preserved() {
        let result = BenchmarkResult::new(
            "bench".to_string(),
            "suite_bench".to_string(),
            BenchmarkKind::Sync,
            None,
            HashMap::new(),
            SuiteType::Performance,
            "strict".to_string(),
            Some(123),
            None,
            None,
            None,
        );
        assert_eq!(result.comparison_mode, "strict");
        assert_eq!(result.fairness_seed, Some(123));
    }

    #[test]
    fn test_benchmark_result_serialization_includes_metadata() {
        let result = BenchmarkResult::new(
            "bench".to_string(),
            "suite_bench".to_string(),
            BenchmarkKind::Sync,
            None,
            HashMap::<Lang, Measurement>::new(),
            SuiteType::Performance,
            "legacy".to_string(),
            None,
            None,
            None,
            None,
        );

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"comparison_mode\":\"legacy\""));
        assert!(!json.contains("\"fairness_seed\""));
    }

    #[test]
    fn test_async_details_include_success_error_ratios() {
        let mut measurements = HashMap::<Lang, Measurement>::new();
        let mut go = Measurement::from_aggregate(10, 1000);
        go.async_success_count = Some(8);
        go.async_error_count = Some(2);
        measurements.insert(Lang::Go, go);

        let result = BenchmarkResult::new(
            "rpc".to_string(),
            "suite_rpc".to_string(),
            BenchmarkKind::Async,
            None,
            measurements,
            SuiteType::Performance,
            "strict".to_string(),
            None,
            Some(5),
            Some(50),
            Some("timeBudgeted".to_string()),
        );

        let details = result.async_details.expect("async details should be present");
        assert_eq!(details.successful_iterations.get(&Lang::Go), Some(&8));
        assert_eq!(details.error_iterations.get(&Lang::Go), Some(&2));
        assert_eq!(details.success_ratio.get(&Lang::Go), Some(&0.8));
        assert_eq!(details.error_ratio.get(&Lang::Go), Some(&0.2));
    }

    #[test]
    fn test_async_details_counts_timeout_as_error() {
        let mut measurements = HashMap::<Lang, Measurement>::new();
        let mut ts = Measurement::timeout_marker();
        ts.async_success_count = Some(4);
        ts.async_error_count = Some(1);
        measurements.insert(Lang::TypeScript, ts);

        let result = BenchmarkResult::new(
            "bench".to_string(),
            "suite_bench".to_string(),
            BenchmarkKind::Async,
            None,
            measurements,
            SuiteType::Performance,
            "strict".to_string(),
            None,
            None,
            None,
            None,
        );

        let details = result.async_details.expect("async details should be present");
        assert_eq!(details.successful_iterations.get(&Lang::TypeScript), Some(&4));
        // base error_count(1) + timeout(1) according to policy
        assert_eq!(details.error_iterations.get(&Lang::TypeScript), Some(&2));
        assert_eq!(details.success_ratio.get(&Lang::TypeScript), Some(&(4.0 / 6.0)));
    }
}
