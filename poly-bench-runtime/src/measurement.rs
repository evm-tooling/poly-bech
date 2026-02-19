//! Unified measurement types for benchmark results

use serde::{Deserialize, Serialize};

/// A single benchmark measurement result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Measurement {
    /// Number of iterations executed
    pub iterations: u64,
    /// Total time in nanoseconds
    pub total_nanos: u64,
    /// Nanoseconds per operation
    pub nanos_per_op: f64,
    /// Operations per second
    pub ops_per_sec: f64,
    /// Minimum time per operation (nanoseconds)
    pub min_nanos: Option<u64>,
    /// Maximum time per operation (nanoseconds)
    pub max_nanos: Option<u64>,
    /// Median (p50) time per operation (nanoseconds)
    pub p50_nanos: Option<u64>,
    /// 75th percentile time per operation (nanoseconds)
    pub p75_nanos: Option<u64>,
    /// 99th percentile time per operation (nanoseconds)
    pub p99_nanos: Option<u64>,
    /// 99.5th percentile time per operation (nanoseconds)
    pub p995_nanos: Option<u64>,
    /// Relative margin of error (percentage)
    pub rme_percent: Option<f64>,
    /// Number of samples collected
    pub samples: Option<u64>,
    /// Bytes allocated per operation (Go-specific)
    pub bytes_per_op: Option<u64>,
    /// Allocations per operation (Go-specific)
    pub allocs_per_op: Option<u64>,
    /// Raw sample times in nanoseconds (for detailed analysis)
    pub raw_samples: Option<Vec<u64>>,
    /// Coefficient of variation (std_dev / mean * 100) - measures result stability
    pub cv_percent: Option<f64>,
    /// Number of outliers removed via IQR method
    pub outliers_removed: Option<u64>,
    /// Whether the benchmark result is considered stable (CV < threshold)
    pub is_stable: Option<bool>,

    // Multi-run aggregation fields (for count > 1)
    /// Number of benchmark runs aggregated (from count directive)
    pub run_count: Option<u64>,
    /// Median nanos_per_op across multiple runs
    pub median_across_runs: Option<f64>,
    /// 95% CI lower bound (nanos)
    pub ci_95_lower: Option<f64>,
    /// 95% CI upper bound (nanos)
    pub ci_95_upper: Option<f64>,
    /// Standard deviation of sample times (nanos)
    pub std_dev_nanos: Option<f64>,
}

/// Default CV threshold percentage (5%) - results with CV above this are considered unstable
pub const DEFAULT_CV_THRESHOLD: f64 = 5.0;

impl Measurement {
    /// Create a new measurement from raw timing data with outlier detection
    pub fn from_samples(raw_samples: Vec<u64>, iterations: u64) -> Self {
        Self::from_samples_with_options(raw_samples, iterations, true, DEFAULT_CV_THRESHOLD)
    }

    /// Create a new measurement from raw timing data with configurable outlier detection
    pub fn from_samples_with_options(
        raw_samples: Vec<u64>,
        iterations: u64,
        remove_outliers: bool,
        cv_threshold: f64,
    ) -> Self {
        let original_count = raw_samples.len();

        // Sort samples first
        let mut sorted = raw_samples.clone();
        sorted.sort_unstable();

        // Optionally remove outliers using IQR method
        let (filtered_samples, outliers_removed) = if remove_outliers && sorted.len() >= 4 {
            let filtered = remove_outliers_iqr(&sorted);
            let removed = original_count.saturating_sub(filtered.len()) as u64;
            (filtered, removed)
        } else {
            (sorted.clone(), 0)
        };

        // Use filtered samples for statistics if available, otherwise original
        let samples_for_stats =
            if filtered_samples.is_empty() { &sorted } else { &filtered_samples };

        // Calculate totals from filtered samples
        let total_nanos: u64 = samples_for_stats.iter().sum();
        let effective_iterations = samples_for_stats.len() as u64;
        let nanos_per_op = if effective_iterations > 0 {
            total_nanos as f64 / effective_iterations as f64
        } else {
            0.0
        };
        let ops_per_sec = if nanos_per_op > 0.0 { 1_000_000_000.0 / nanos_per_op } else { 0.0 };

        let min_nanos = samples_for_stats.first().copied();
        let max_nanos = samples_for_stats.last().copied();
        let p50_nanos = percentile(samples_for_stats, 50);
        let p75_nanos = percentile(samples_for_stats, 75);
        let p99_nanos = percentile(samples_for_stats, 99);
        let p995_nanos = percentile_f(samples_for_stats, 99.5);

        // Calculate standard deviation and CV
        let (rme_percent, cv_percent, is_stable, std_dev_nanos) = if samples_for_stats.len() > 1 {
            let mean = nanos_per_op;
            let variance: f64 =
                samples_for_stats.iter().map(|&x| (x as f64 - mean).powi(2)).sum::<f64>() /
                    (samples_for_stats.len() - 1) as f64;
            let std_dev = variance.sqrt();
            let std_error = std_dev / (samples_for_stats.len() as f64).sqrt();

            // 95% confidence interval uses t-value ~1.96 for large samples
            let rme = (std_error / mean) * 100.0 * 1.96;

            // Coefficient of variation: (std_dev / mean) * 100
            let cv = if mean > 0.0 { (std_dev / mean) * 100.0 } else { 0.0 };

            // Stability check: CV below threshold
            let stable = cv <= cv_threshold;

            (Some(rme), Some(cv), Some(stable), Some(std_dev))
        } else {
            (None, None, None, None)
        };

        let sample_count = raw_samples.len() as u64;

        Self {
            iterations,
            total_nanos,
            nanos_per_op,
            ops_per_sec,
            min_nanos,
            max_nanos,
            p50_nanos,
            p75_nanos,
            p99_nanos,
            p995_nanos,
            rme_percent,
            samples: Some(sample_count),
            bytes_per_op: None,
            allocs_per_op: None,
            raw_samples: Some(raw_samples),
            cv_percent,
            outliers_removed: Some(outliers_removed),
            is_stable,
            // Multi-run fields (None for single run)
            run_count: None,
            median_across_runs: None,
            ci_95_lower: None,
            ci_95_upper: None,
            std_dev_nanos,
        }
    }

    /// Create a measurement from aggregate data (no samples)
    pub fn from_aggregate(iterations: u64, total_nanos: u64) -> Self {
        let nanos_per_op = total_nanos as f64 / iterations as f64;
        let ops_per_sec = if nanos_per_op > 0.0 { 1_000_000_000.0 / nanos_per_op } else { 0.0 };

        Self {
            iterations,
            total_nanos,
            nanos_per_op,
            ops_per_sec,
            min_nanos: None,
            max_nanos: None,
            p50_nanos: None,
            p75_nanos: None,
            p99_nanos: None,
            p995_nanos: None,
            rme_percent: None,
            samples: Some(iterations),
            bytes_per_op: None,
            allocs_per_op: None,
            raw_samples: None,
            cv_percent: None,
            outliers_removed: None,
            is_stable: None,
            // Multi-run fields (None for single run)
            run_count: None,
            median_across_runs: None,
            ci_95_lower: None,
            ci_95_upper: None,
            std_dev_nanos: None,
        }
    }

    /// Set memory allocation data (for Go)
    pub fn with_allocs(mut self, bytes_per_op: u64, allocs_per_op: u64) -> Self {
        self.bytes_per_op = Some(bytes_per_op);
        self.allocs_per_op = Some(allocs_per_op);
        self
    }

    /// Format duration for display
    pub fn format_duration(nanos: f64) -> String {
        if nanos < 1_000.0 {
            format!("{:.2} ns", nanos)
        } else if nanos < 1_000_000.0 {
            format!("{:.2} µs", nanos / 1_000.0)
        } else if nanos < 1_000_000_000.0 {
            format!("{:.2} ms", nanos / 1_000_000.0)
        } else {
            format!("{:.3} s", nanos / 1_000_000_000.0)
        }
    }

    /// Format ops/sec for display
    pub fn format_ops_per_sec(ops: f64) -> String {
        if ops >= 1_000_000_000.0 {
            format!("{:.2}B ops/s", ops / 1_000_000_000.0)
        } else if ops >= 1_000_000.0 {
            format!("{:.2}M ops/s", ops / 1_000_000.0)
        } else if ops >= 1_000.0 {
            format!("{:.2}K ops/s", ops / 1_000.0)
        } else {
            format!("{:.2} ops/s", ops)
        }
    }

    /// Aggregate multiple run measurements into a single representative measurement.
    /// Uses median for the primary value and calculates 95% confidence interval.
    /// This matches Go's benchstat approach for statistical consistency.
    pub fn aggregate_runs(runs: Vec<Measurement>) -> Self {
        if runs.is_empty() {
            return Measurement::from_aggregate(0, 0);
        }
        if runs.len() == 1 {
            let mut single = runs.into_iter().next().unwrap();
            single.run_count = Some(1);
            return single;
        }

        let run_count = runs.len();

        // Collect nanos_per_op from each run
        let mut nanos_values: Vec<f64> = runs.iter().map(|r| r.nanos_per_op).collect();
        nanos_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        // Calculate median
        let median = if run_count % 2 == 0 {
            (nanos_values[run_count / 2 - 1] + nanos_values[run_count / 2]) / 2.0
        } else {
            nanos_values[run_count / 2]
        };

        // Calculate mean and std deviation for CI
        let mean: f64 = nanos_values.iter().sum::<f64>() / run_count as f64;
        let variance: f64 =
            nanos_values.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / (run_count - 1) as f64;
        let std_dev = variance.sqrt();
        let std_error = std_dev / (run_count as f64).sqrt();

        // 95% CI using t-distribution (approximate with z=1.96 for larger samples)
        let t_value = 1.96;
        let ci_half_width = t_value * std_error;
        let ci_lower = (median - ci_half_width).max(0.0);
        let ci_upper = median + ci_half_width;

        // Aggregate totals across runs
        let total_iterations: u64 = runs.iter().map(|r| r.iterations).sum();
        let total_nanos: u64 = runs.iter().map(|r| r.total_nanos).sum();

        // Use min/max across all runs
        let min_nanos = runs.iter().filter_map(|r| r.min_nanos).min();
        let max_nanos = runs.iter().filter_map(|r| r.max_nanos).max();

        // Aggregate percentiles using median of percentiles
        let p50_nanos = median_of_options(runs.iter().filter_map(|r| r.p50_nanos).collect());
        let p75_nanos = median_of_options(runs.iter().filter_map(|r| r.p75_nanos).collect());
        let p99_nanos = median_of_options(runs.iter().filter_map(|r| r.p99_nanos).collect());
        let p995_nanos = median_of_options(runs.iter().filter_map(|r| r.p995_nanos).collect());

        // Aggregate memory stats (average)
        let bytes_per_op = {
            let values: Vec<u64> = runs.iter().filter_map(|r| r.bytes_per_op).collect();
            if values.is_empty() {
                None
            } else {
                Some(values.iter().sum::<u64>() / values.len() as u64)
            }
        };
        let allocs_per_op = {
            let values: Vec<u64> = runs.iter().filter_map(|r| r.allocs_per_op).collect();
            if values.is_empty() {
                None
            } else {
                Some(values.iter().sum::<u64>() / values.len() as u64)
            }
        };

        // Total samples across all runs
        let total_samples: u64 = runs.iter().filter_map(|r| r.samples).sum();
        let total_outliers: u64 = runs.iter().filter_map(|r| r.outliers_removed).sum();

        // Calculate cross-run CV and RME
        let cv_percent = if mean > 0.0 { Some((std_dev / mean) * 100.0) } else { None };
        let rme_percent =
            if median > 0.0 { Some((std_error / median) * 100.0 * 1.96) } else { None };
        let is_stable = cv_percent.map(|cv| cv <= DEFAULT_CV_THRESHOLD);

        Self {
            iterations: total_iterations,
            total_nanos,
            nanos_per_op: median, // Use median as primary value
            ops_per_sec: if median > 0.0 { 1_000_000_000.0 / median } else { 0.0 },
            min_nanos,
            max_nanos,
            p50_nanos,
            p75_nanos,
            p99_nanos,
            p995_nanos,
            rme_percent,
            samples: Some(total_samples),
            bytes_per_op,
            allocs_per_op,
            raw_samples: None, // Don't combine raw samples (too large)
            cv_percent,
            outliers_removed: Some(total_outliers),
            is_stable,
            // Multi-run aggregation fields
            run_count: Some(run_count as u64),
            median_across_runs: Some(median),
            ci_95_lower: Some(ci_lower),
            ci_95_upper: Some(ci_upper),
            std_dev_nanos: Some(std_dev),
        }
    }
}

/// Calculate median of a vector of u64 values
fn median_of_options(mut values: Vec<u64>) -> Option<u64> {
    if values.is_empty() {
        return None;
    }
    values.sort_unstable();
    let len = values.len();
    if len % 2 == 0 {
        Some((values[len / 2 - 1] + values[len / 2]) / 2)
    } else {
        Some(values[len / 2])
    }
}

/// Calculate percentile from sorted samples (integer percentile)
fn percentile(sorted: &[u64], p: usize) -> Option<u64> {
    if sorted.is_empty() {
        return None;
    }
    let idx = (sorted.len() * p / 100).min(sorted.len() - 1);
    Some(sorted[idx])
}

/// Calculate fractional percentile from sorted samples
fn percentile_f(sorted: &[u64], p: f64) -> Option<u64> {
    if sorted.is_empty() {
        return None;
    }
    let idx = ((sorted.len() as f64 * p / 100.0) as usize).min(sorted.len() - 1);
    Some(sorted[idx])
}

/// Remove outliers using the IQR (Interquartile Range) method
///
/// Outliers are defined as values outside [Q1 - 1.5*IQR, Q3 + 1.5*IQR]
/// where IQR = Q3 - Q1.
fn remove_outliers_iqr(sorted: &[u64]) -> Vec<u64> {
    if sorted.len() < 4 {
        return sorted.to_vec();
    }

    // Calculate Q1 (25th percentile) and Q3 (75th percentile)
    let q1_idx = sorted.len() / 4;
    let q3_idx = (sorted.len() * 3) / 4;

    let q1 = sorted[q1_idx] as f64;
    let q3 = sorted[q3_idx] as f64;
    let iqr = q3 - q1;

    // Define bounds for outlier detection
    let lower_bound = (q1 - 1.5 * iqr).max(0.0) as u64;
    let upper_bound = (q3 + 1.5 * iqr) as u64;

    // Filter out outliers
    sorted.iter().copied().filter(|&s| s >= lower_bound && s <= upper_bound).collect()
}

/// Comparison between two measurements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comparison {
    /// The benchmark name
    pub name: String,
    /// First measurement (e.g., Go)
    pub first: Measurement,
    /// First language name
    pub first_lang: String,
    /// Second measurement (e.g., TypeScript)
    pub second: Measurement,
    /// Second language name
    pub second_lang: String,
    /// Ratio of first/second (>1 means second is faster)
    pub ratio: f64,
    /// Speedup factor (always >= 1)
    pub speedup: f64,
    /// Which one is faster
    pub winner: ComparisonWinner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparisonWinner {
    First,
    Second,
    Tie,
}

impl Comparison {
    pub fn new(
        name: String,
        first: Measurement,
        first_lang: String,
        second: Measurement,
        second_lang: String,
    ) -> Self {
        let ratio = first.nanos_per_op / second.nanos_per_op;

        let (winner, speedup) = if (ratio - 1.0).abs() < 0.05 {
            (ComparisonWinner::Tie, 1.0)
        } else if ratio > 1.0 {
            (ComparisonWinner::Second, ratio)
        } else {
            (ComparisonWinner::First, 1.0 / ratio)
        };

        Self { name, first, first_lang, second, second_lang, ratio, speedup, winner }
    }

    /// Get a description of the speedup
    pub fn speedup_description(&self) -> String {
        match self.winner {
            ComparisonWinner::First => format!("{} {:.2}x faster", self.first_lang, self.speedup),
            ComparisonWinner::Second => format!("{} {:.2}x faster", self.second_lang, self.speedup),
            ComparisonWinner::Tie => "Similar performance".to_string(),
        }
    }
}
