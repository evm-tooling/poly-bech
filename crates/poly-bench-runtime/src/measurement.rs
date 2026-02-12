//! Unified measurement types for benchmark results

use serde::{Serialize, Deserialize};

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
}

impl Measurement {
    /// Create a new measurement from raw timing data
    pub fn from_samples(raw_samples: Vec<u64>, iterations: u64) -> Self {
        let total_nanos: u64 = raw_samples.iter().sum();
        let nanos_per_op = total_nanos as f64 / iterations as f64;
        let ops_per_sec = if nanos_per_op > 0.0 {
            1_000_000_000.0 / nanos_per_op
        } else {
            0.0
        };

        let mut sorted = raw_samples.clone();
        sorted.sort_unstable();

        let min_nanos = sorted.first().copied();
        let max_nanos = sorted.last().copied();
        let p50_nanos = percentile(&sorted, 50);
        let p75_nanos = percentile(&sorted, 75);
        let p99_nanos = percentile(&sorted, 99);
        let p995_nanos = percentile_f(&sorted, 99.5);
        
        // Calculate relative margin of error (RME)
        let rme_percent = if sorted.len() > 1 {
            let mean = nanos_per_op;
            let variance: f64 = sorted.iter()
                .map(|&x| (x as f64 - mean).powi(2))
                .sum::<f64>() / (sorted.len() - 1) as f64;
            let std_dev = variance.sqrt();
            let std_error = std_dev / (sorted.len() as f64).sqrt();
            // 95% confidence interval uses t-value ~1.96 for large samples
            Some((std_error / mean) * 100.0 * 1.96)
        } else {
            None
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
        }
    }

    /// Create a measurement from aggregate data (no samples)
    pub fn from_aggregate(iterations: u64, total_nanos: u64) -> Self {
        let nanos_per_op = total_nanos as f64 / iterations as f64;
        let ops_per_sec = if nanos_per_op > 0.0 {
            1_000_000_000.0 / nanos_per_op
        } else {
            0.0
        };

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

        Self {
            name,
            first,
            first_lang,
            second,
            second_lang,
            ratio,
            speedup,
            winner,
        }
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
