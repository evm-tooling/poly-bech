//! Console output reporter with vitest/tinybench-style distribution stats

use colored::Colorize;
use miette::Result;
use poly_bench_dsl::{BenchmarkKind, Lang};
use poly_bench_executor::{comparison::BenchmarkResult, BenchmarkResults, SuiteResults};
use poly_bench_runtime::measurement::Measurement;

/// Benchmark configuration for display
#[derive(Debug, Clone, Default)]
pub struct BenchConfig {
    pub iterations: Option<u64>,
    pub warmup: Option<u64>,
    pub timeout_ms: Option<u64>,
    pub order: Option<String>,
}

/// Report options for enhanced output
#[derive(Debug, Clone)]
pub struct ReportOptions {
    /// Show ops/sec for each benchmark
    pub show_ops_per_sec: bool,
    /// Show config section
    pub show_config: bool,
    /// Show distribution stats (min, max, mean, p50, p75, p99) - default: true
    pub show_distribution: bool,
    /// Show memory stats (if available)
    pub show_memory: bool,
    /// Benchmark configuration
    pub config: BenchConfig,
}

impl Default for ReportOptions {
    fn default() -> Self {
        Self {
            show_ops_per_sec: true,
            show_config: true,
            show_distribution: true, // Show distribution by default
            show_memory: false,
            config: BenchConfig::default(),
        }
    }
}

/// Format ops/sec for display (hz style like vitest)
fn format_hz(ops: f64) -> String {
    if ops >= 1_000_000_000.0 {
        format!("{:>10.2}G", ops / 1_000_000_000.0)
    } else if ops >= 1_000_000.0 {
        format!("{:>10.2}M", ops / 1_000_000.0)
    } else if ops >= 1_000.0 {
        // Rust doesn't support comma separators directly, use thousands manually
        let formatted = format!("{:.2}", ops);
        format!("{:>12}", formatted)
    } else {
        format!("{:>10.2}", ops)
    }
}

/// Format duration in ms for display (vitest style)
fn format_ms(nanos: f64) -> String {
    let ms = nanos / 1_000_000.0;
    if ms >= 1.0 {
        format!("{:.4}", ms)
    } else if ms >= 0.001 {
        format!("{:.4}", ms)
    } else {
        // Very fast - show as us
        let us = nanos / 1_000.0;
        format!("{:.4}", us)
    }
}

/// Format ops/sec for display (legacy)
fn format_ops_per_sec(ops: f64) -> String {
    if ops >= 1_000_000_000.0 {
        format!("{:.1}G", ops / 1_000_000_000.0)
    } else if ops >= 1_000_000.0 {
        format!("{:.1}M", ops / 1_000_000.0)
    } else if ops >= 1_000.0 {
        format!("{:.1}K", ops / 1_000.0)
    } else {
        format!("{:.0}", ops)
    }
}

const ASYNC_WARN_MIN_SUCCESS_RATIO: f64 = 0.95;
const ASYNC_WARN_MAX_RATIO_SPREAD: f64 = 0.05;

fn lang_short_name(lang: Lang) -> &'static str {
    match lang {
        Lang::Go => "Go",
        Lang::TypeScript => "TS",
        Lang::Rust => "Rust",
        _ => "Unknown",
    }
}

fn async_reliability_warnings(bench: &BenchmarkResult) -> Vec<String> {
    if bench.kind != BenchmarkKind::Async {
        return Vec::new();
    }
    let Some(details) = &bench.async_details else {
        return Vec::new();
    };

    let mut warnings = Vec::new();
    let mut ratios = Vec::new();
    for (lang, ratio) in &details.success_ratio {
        ratios.push((*lang, *ratio));
        if *ratio < ASYNC_WARN_MIN_SUCCESS_RATIO {
            warnings.push(format!(
                "{} success ratio is {:.1}% (< {:.0}%)",
                lang_short_name(*lang),
                ratio * 100.0,
                ASYNC_WARN_MIN_SUCCESS_RATIO * 100.0
            ));
        }
    }

    if ratios.len() >= 2 {
        let min_ratio = ratios.iter().map(|(_, r)| *r).fold(f64::INFINITY, f64::min);
        let max_ratio = ratios.iter().map(|(_, r)| *r).fold(f64::NEG_INFINITY, f64::max);
        let spread = max_ratio - min_ratio;
        if spread > ASYNC_WARN_MAX_RATIO_SPREAD {
            warnings.push(format!(
                "cross-language success-ratio spread is {:.1}% (> {:.0}%); speedup may be low-confidence",
                spread * 100.0,
                ASYNC_WARN_MAX_RATIO_SPREAD * 100.0
            ));
        }
    }

    warnings
}

/// Generate console report (simple version)
pub fn report(results: &BenchmarkResults) -> Result<()> {
    report_with_options(results, &ReportOptions::default())
}

/// Generate console report with options
pub fn report_with_options(results: &BenchmarkResults, options: &ReportOptions) -> Result<()> {
    println!("\n{}", "═".repeat(80));
    println!("{}", "  BENCHMARK RESULTS".bold());
    println!("{}\n", "═".repeat(80));

    // Overall summary
    println!("{}", "OVERALL SUMMARY".bold().underline());
    println!();

    let summary = &results.summary;

    // Winner banner
    match summary.winner {
        Some(Lang::Go) => {
            println!("  {} {}", "🏆".green(), summary.winner_description.green().bold());
        }
        Some(Lang::TypeScript) => {
            println!("  {} {}", "🏆".cyan(), summary.winner_description.cyan().bold());
        }
        Some(Lang::Rust) => {
            println!("  {} {}", "🏆".yellow(), summary.winner_description.yellow().bold());
        }
        _ => {
            println!("  {} {}", "🤝", summary.winner_description.dimmed());
        }
    }
    println!();

    // Stats table
    println!("  {:<20} {}", "Total Suites:", summary.total_suites);
    println!("  {:<20} {}", "Total Benchmarks:", summary.total_benchmarks);
    println!(
        "  {:<20} {} ({}%)",
        "Go Wins:",
        summary.go_wins,
        (summary.go_wins * 100) / summary.total_benchmarks.max(1)
    );
    println!(
        "  {:<20} {} ({}%)",
        "TypeScript Wins:",
        summary.ts_wins,
        (summary.ts_wins * 100) / summary.total_benchmarks.max(1)
    );
    println!(
        "  {:<20} {} ({}%)",
        "Rust Wins:",
        summary.rust_wins,
        (summary.rust_wins * 100) / summary.total_benchmarks.max(1)
    );
    println!(
        "  {:<20} {} ({}%)",
        "Ties:",
        summary.ties,
        (summary.ties * 100) / summary.total_benchmarks.max(1)
    );
    println!("  {:<20} {:.2}x", "Geometric Mean:", summary.geo_mean_speedup);

    // Statistical quality indicators
    if summary.total_outliers_removed > 0 {
        println!("  {:<20} {}", "Outliers Removed:", summary.total_outliers_removed);
    }
    if summary.unstable_count > 0 {
        println!(
            "  {:<20} {} {}",
            "Unstable Results:",
            format!("{}", summary.unstable_count).yellow(),
            "(CV > threshold)".dimmed()
        );
    }
    println!();

    // Config section (if enabled and config provided)
    if options.show_config {
        let config = &options.config;
        let has_config = config.iterations.is_some() ||
            config.warmup.is_some() ||
            config.timeout_ms.is_some() ||
            config.order.is_some();

        if has_config {
            println!("{}", "CONFIG".bold().underline());
            println!();

            if let Some(iter) = config.iterations {
                println!("  {:<20} {}", "Iterations:", iter);
            }
            if let Some(warm) = config.warmup {
                println!("  {:<20} {}", "Warmup:", warm);
            }
            if let Some(to) = config.timeout_ms {
                let timeout_str =
                    if to >= 1000 { format!("{}s", to / 1000) } else { format!("{}ms", to) };
                println!("  {:<20} {}", "Timeout:", timeout_str);
            }
            if let Some(ref ord) = config.order {
                println!("  {:<20} {}", "Execution Order:", ord);
            }
            println!();
        }
    }

    // Suite details
    println!("{}", "SUITE RESULTS".bold().underline());
    println!();

    for suite in &results.suites {
        print_suite_with_options(suite, options);
    }

    // Legend
    let has_memory_suite =
        results.suites.iter().any(|s| s.suite_type == poly_bench_dsl::SuiteType::Memory);
    println!("{}", "─".repeat(110));
    println!("{}", "LEGEND".dimmed());
    println!(
        "  {} = Go result  |  {} = TypeScript result  |  {} = Rust result",
        "go".green(),
        "ts".cyan(),
        "rust".yellow()
    );
    println!("  {} = operations per second (higher is better)", "hz".dimmed());
    if has_memory_suite {
        println!(
            "  {} = bytes per operation (lower is better)  |  {} = allocations per operation",
            "bytes/op".dimmed(),
            "allocs/op".dimmed()
        );
    }
    println!(
        "  {} = minimum latency  |  {} = maximum latency  |  {} = mean latency (all in ms)",
        "min".dimmed(),
        "max".dimmed(),
        "mean".dimmed()
    );
    println!(
        "  {} = 75th percentile  |  {} = 99th percentile  |  {} = 99.5th percentile",
        "p75".dimmed(),
        "p99".dimmed(),
        "p995".dimmed()
    );
    println!("  {} = relative margin of error  |  {} = coefficient of variation (stability)  |  {} = number of samples",
        "rme".dimmed(),
        "cv".dimmed(),
        "samples".dimmed()
    );
    println!("  {} = CV above threshold (results may be unstable)", "yellow cv".yellow());
    println!();

    Ok(())
}

fn print_suite_with_options(suite: &SuiteResults, options: &ReportOptions) {
    let icon = match suite.summary.winner {
        Some(Lang::Go) => "✓",
        Some(Lang::TypeScript) => "✓",
        _ => "✓",
    };

    // Suite header
    println!(" {} {}", icon.green(), suite.name.bold());

    if let Some(ref desc) = suite.description {
        println!("   {}", desc.dimmed());
    }

    // Distribution stats table (vitest/tinybench style)
    if options.show_distribution {
        print_distribution_table(&suite.benchmarks, suite.suite_type, options);
    } else {
        // Legacy compact table
        print_compact_table(&suite.benchmarks, suite.suite_type, options);
    }

    // Suite summary footer
    let go_wins = suite.summary.go_wins;
    let ts_wins = suite.summary.ts_wins;
    let rust_wins = suite.summary.rust_wins;
    let ties = suite.summary.ties;

    println!();
    println!(
        "   {} Go: {} wins | TS: {} wins | Rust: {} wins | Ties: {} | Geo mean: {:.2}x",
        "Summary:".dimmed(),
        format!("{}", go_wins).green(),
        format!("{}", ts_wins).cyan(),
        format!("{}", rust_wins).yellow(),
        format!("{}", ties).dimmed(),
        suite.summary.geo_mean_speedup
    );

    println!();

    let has_async = suite.benchmarks.iter().any(|b| b.kind == BenchmarkKind::Async);
    if has_async {
        let mut warmup_cap = None;
        let mut sample_cap = None;
        let mut sampling_policy = None;
        for bench in &suite.benchmarks {
            if let Some(details) = &bench.async_details {
                warmup_cap.get_or_insert(details.warmup_cap);
                sample_cap.get_or_insert(details.sample_cap);
                sampling_policy.get_or_insert(details.sampling_policy.as_str());
            }
        }
        println!(
            "   {}",
            format!(
                "Async-sequential mode detected (policy: {}, warmup<={}, samples<={})",
                sampling_policy.unwrap_or("timeBudgeted"),
                warmup_cap.unwrap_or(5),
                sample_cap.unwrap_or(50)
            )
            .dimmed()
        );
        println!();
    }
}

/// Print the vitest/tinybench style distribution table
fn print_distribution_table(
    benchmarks: &[BenchmarkResult],
    suite_type: poly_bench_dsl::SuiteType,
    _options: &ReportOptions,
) {
    let is_memory = suite_type == poly_bench_dsl::SuiteType::Memory;

    // Check if any measurement has multiple runs
    let has_multi_run = benchmarks
        .iter()
        .flat_map(|b| b.measurements.values())
        .any(|m| m.run_count.unwrap_or(1) > 1);

    // Table header - memory suite shows bytes/op, allocs/op; performance shows latency
    if is_memory {
        println!(
            "   {:<40} {:>12} {:>10} {:>12} {:>8} {:>8}",
            "name".dimmed(),
            "bytes/op".dimmed(),
            "allocs/op".dimmed(),
            "hz".dimmed(),
            "mean".dimmed(),
            "samples".dimmed()
        );
    } else if has_multi_run {
        println!(
            "   {:<40} {:>12} {:>10} {:>12} {:>8} {:>8} {:>8} {:>8} {:>9} {:>7} {:>6}",
            "name".dimmed(),
            "hz".dimmed(),
            "median".dimmed(),
            "95% CI".dimmed(),
            "min".dimmed(),
            "max".dimmed(),
            "p75".dimmed(),
            "p99".dimmed(),
            "rme".dimmed(),
            "cv".dimmed(),
            "runs".dimmed()
        );
    } else {
        println!(
            "   {:<40} {:>12} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>9} {:>7} {:>8}",
            "name".dimmed(),
            "hz".dimmed(),
            "min".dimmed(),
            "max".dimmed(),
            "mean".dimmed(),
            "p75".dimmed(),
            "p99".dimmed(),
            "p995".dimmed(),
            "rme".dimmed(),
            "cv".dimmed(),
            "samples".dimmed()
        );
    }

    // Determine fastest/slowest (or lowest/highest memory) for each language
    let metric_val = |m: &Measurement| -> f64 {
        if is_memory {
            m.bytes_per_op.map(|b| b as f64).unwrap_or(0.0)
        } else {
            m.ops_per_sec
        }
    };
    let better = |a: f64, b: f64| {
        if is_memory {
            a < b // lower bytes is better
        } else {
            a > b // higher ops/sec is better
        }
    };

    let go_best: Option<&str> = benchmarks
        .iter()
        .filter_map(|b| b.measurements.get(&Lang::Go).map(|m| (b.name.as_str(), metric_val(m))))
        .max_by(|a, b| {
            if better(a.1, b.1) {
                std::cmp::Ordering::Greater
            } else if better(b.1, a.1) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .map(|(name, _)| name);

    let go_worst: Option<&str> = benchmarks
        .iter()
        .filter_map(|b| b.measurements.get(&Lang::Go).map(|m| (b.name.as_str(), metric_val(m))))
        .min_by(|a, b| {
            if better(a.1, b.1) {
                std::cmp::Ordering::Greater
            } else if better(b.1, a.1) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .map(|(name, _)| name);

    let ts_best: Option<&str> = benchmarks
        .iter()
        .filter_map(|b| {
            b.measurements.get(&Lang::TypeScript).map(|m| (b.name.as_str(), metric_val(m)))
        })
        .max_by(|a, b| {
            if better(a.1, b.1) {
                std::cmp::Ordering::Greater
            } else if better(b.1, a.1) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .map(|(name, _)| name);

    let ts_worst: Option<&str> = benchmarks
        .iter()
        .filter_map(|b| {
            b.measurements.get(&Lang::TypeScript).map(|m| (b.name.as_str(), metric_val(m)))
        })
        .min_by(|a, b| {
            if better(a.1, b.1) {
                std::cmp::Ordering::Greater
            } else if better(b.1, a.1) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .map(|(name, _)| name);

    let rust_best: Option<&str> = benchmarks
        .iter()
        .filter_map(|b| b.measurements.get(&Lang::Rust).map(|m| (b.name.as_str(), metric_val(m))))
        .max_by(|a, b| {
            if better(a.1, b.1) {
                std::cmp::Ordering::Greater
            } else if better(b.1, a.1) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .map(|(name, _)| name);

    let rust_worst: Option<&str> = benchmarks
        .iter()
        .filter_map(|b| b.measurements.get(&Lang::Rust).map(|m| (b.name.as_str(), metric_val(m))))
        .min_by(|a, b| {
            if better(a.1, b.1) {
                std::cmp::Ordering::Greater
            } else if better(b.1, a.1) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .map(|(name, _)| name);

    let (best_label, worst_label) =
        if is_memory { (" lowest", " highest") } else { (" fastest", " slowest") };

    for bench in benchmarks {
        // Go row
        if let Some(m) = bench.measurements.get(&Lang::Go) {
            let badge = if Some(bench.name.as_str()) == go_best && benchmarks.len() > 1 {
                best_label.green().to_string()
            } else if Some(bench.name.as_str()) == go_worst && benchmarks.len() > 1 {
                worst_label.yellow().to_string()
            } else {
                String::new()
            };

            let name = format!("· go: {}", bench.name);

            if is_memory {
                let bytes = m
                    .bytes_per_op
                    .map(Measurement::format_bytes)
                    .unwrap_or_else(|| "-".to_string());
                let allocs =
                    m.allocs_per_op.map(|a| a.to_string()).unwrap_or_else(|| "-".to_string());
                let mean_ns = m.nanos_per_op;
                let samples = m.samples.unwrap_or(1000);
                println!(
                    "   {:<40} {:>12} {:>10} {:>12} {:>8} {:>8}{}",
                    name.green(),
                    bytes,
                    allocs,
                    format_hz(m.ops_per_sec),
                    format_ms(mean_ns),
                    samples,
                    badge
                );
            } else if has_multi_run && m.run_count.unwrap_or(1) > 1 {
                // Multi-run format: show median and 95% CI
                let median_ns = m.median_across_runs.unwrap_or(m.nanos_per_op);
                let ci_str = if let (Some(lower), Some(upper)) = (m.ci_95_lower, m.ci_95_upper) {
                    format!("±{}", format_ms((upper - lower) / 2.0))
                } else {
                    "-".to_string()
                };
                let min_ns = m.min_nanos.unwrap_or(m.nanos_per_op as u64) as f64;
                let max_ns = m.max_nanos.unwrap_or(m.nanos_per_op as u64) as f64;
                let p75_ns = m.p75_nanos.unwrap_or(m.nanos_per_op as u64) as f64;
                let p99_ns = m.p99_nanos.unwrap_or(m.nanos_per_op as u64) as f64;
                let rme = m.rme_percent.unwrap_or(0.0);
                let cv = m.cv_percent.unwrap_or(0.0);
                let runs = m.run_count.unwrap_or(1);

                let cv_str = if m.is_stable == Some(false) {
                    format!("{:.1}%", cv).yellow().to_string()
                } else {
                    format!("{:.1}%", cv)
                };

                println!(
                    "   {:<40} {:>12} {:>10} {:>12} {:>8} {:>8} {:>8} {:>8} {:>8}% {:>7} {:>6}{}",
                    name.green(),
                    format_hz(m.ops_per_sec),
                    format_ms(median_ns),
                    ci_str,
                    format_ms(min_ns),
                    format_ms(max_ns),
                    format_ms(p75_ns),
                    format_ms(p99_ns),
                    format!("±{:.2}", rme),
                    cv_str,
                    runs,
                    badge
                );
            } else {
                // Single-run format (existing)
                let min_ns = m.min_nanos.unwrap_or(m.nanos_per_op as u64) as f64;
                let max_ns = m.max_nanos.unwrap_or(m.nanos_per_op as u64) as f64;
                let mean_ns = m.nanos_per_op;
                let p75_ns = m.p75_nanos.unwrap_or(m.nanos_per_op as u64) as f64;
                let p99_ns = m.p99_nanos.unwrap_or(m.nanos_per_op as u64) as f64;
                let p995_ns =
                    m.p995_nanos.unwrap_or(m.p99_nanos.unwrap_or(m.nanos_per_op as u64)) as f64;
                let rme = m.rme_percent.unwrap_or(0.0);
                let cv = m.cv_percent.unwrap_or(0.0);
                let samples = m.samples.unwrap_or(1000);

                let cv_str = if m.is_stable == Some(false) {
                    format!("{:.1}%", cv).yellow().to_string()
                } else {
                    format!("{:.1}%", cv)
                };

                println!(
                    "   {:<40} {:>12} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8}% {:>7} {:>8}{}",
                    name.green(),
                    format_hz(m.ops_per_sec),
                    format_ms(min_ns),
                    format_ms(max_ns),
                    format_ms(mean_ns),
                    format_ms(p75_ns),
                    format_ms(p99_ns),
                    format_ms(p995_ns),
                    format!("±{:.2}", rme),
                    cv_str,
                    samples,
                    badge
                );
            }
        }

        // TypeScript row
        if let Some(m) = bench.measurements.get(&Lang::TypeScript) {
            let badge = if Some(bench.name.as_str()) == ts_best && benchmarks.len() > 1 {
                best_label.green().to_string()
            } else if Some(bench.name.as_str()) == ts_worst && benchmarks.len() > 1 {
                worst_label.yellow().to_string()
            } else {
                String::new()
            };

            let name = format!("· ts: {}", bench.name);

            if is_memory {
                let bytes = m
                    .bytes_per_op
                    .map(Measurement::format_bytes)
                    .unwrap_or_else(|| "-".to_string());
                let allocs =
                    m.allocs_per_op.map(|a| a.to_string()).unwrap_or_else(|| "-".to_string());
                let mean_ns = m.nanos_per_op;
                let samples = m.samples.unwrap_or(1000);
                println!(
                    "   {:<40} {:>12} {:>10} {:>12} {:>8} {:>8}{}",
                    name.cyan(),
                    bytes,
                    allocs,
                    format_hz(m.ops_per_sec),
                    format_ms(mean_ns),
                    samples,
                    badge
                );
            } else if has_multi_run && m.run_count.unwrap_or(1) > 1 {
                // Multi-run format: show median and 95% CI
                let median_ns = m.median_across_runs.unwrap_or(m.nanos_per_op);
                let ci_str = if let (Some(lower), Some(upper)) = (m.ci_95_lower, m.ci_95_upper) {
                    format!("±{}", format_ms((upper - lower) / 2.0))
                } else {
                    "-".to_string()
                };
                let min_ns = m.min_nanos.unwrap_or(m.nanos_per_op as u64) as f64;
                let max_ns = m.max_nanos.unwrap_or(m.nanos_per_op as u64) as f64;
                let p75_ns = m.p75_nanos.unwrap_or(m.nanos_per_op as u64) as f64;
                let p99_ns = m.p99_nanos.unwrap_or(m.nanos_per_op as u64) as f64;
                let rme = m.rme_percent.unwrap_or(0.0);
                let cv = m.cv_percent.unwrap_or(0.0);
                let runs = m.run_count.unwrap_or(1);

                let cv_str = if m.is_stable == Some(false) {
                    format!("{:.1}%", cv).yellow().to_string()
                } else {
                    format!("{:.1}%", cv)
                };

                println!(
                    "   {:<40} {:>12} {:>10} {:>12} {:>8} {:>8} {:>8} {:>8} {:>8}% {:>7} {:>6}{}",
                    name.cyan(),
                    format_hz(m.ops_per_sec),
                    format_ms(median_ns),
                    ci_str,
                    format_ms(min_ns),
                    format_ms(max_ns),
                    format_ms(p75_ns),
                    format_ms(p99_ns),
                    format!("±{:.2}", rme),
                    cv_str,
                    runs,
                    badge
                );
            } else {
                // Single-run format (existing)
                let min_ns = m.min_nanos.unwrap_or(m.nanos_per_op as u64) as f64;
                let max_ns = m.max_nanos.unwrap_or(m.nanos_per_op as u64) as f64;
                let mean_ns = m.nanos_per_op;
                let p75_ns = m.p75_nanos.unwrap_or(m.nanos_per_op as u64) as f64;
                let p99_ns = m.p99_nanos.unwrap_or(m.nanos_per_op as u64) as f64;
                let p995_ns =
                    m.p995_nanos.unwrap_or(m.p99_nanos.unwrap_or(m.nanos_per_op as u64)) as f64;
                let rme = m.rme_percent.unwrap_or(0.0);
                let cv = m.cv_percent.unwrap_or(0.0);
                let samples = m.samples.unwrap_or(1000);

                let cv_str = if m.is_stable == Some(false) {
                    format!("{:.1}%", cv).yellow().to_string()
                } else {
                    format!("{:.1}%", cv)
                };

                println!(
                    "   {:<40} {:>12} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8}% {:>7} {:>8}{}",
                    name.cyan(),
                    format_hz(m.ops_per_sec),
                    format_ms(min_ns),
                    format_ms(max_ns),
                    format_ms(mean_ns),
                    format_ms(p75_ns),
                    format_ms(p99_ns),
                    format_ms(p995_ns),
                    format!("±{:.2}", rme),
                    cv_str,
                    samples,
                    badge
                );
            }
        }

        // Rust row
        if let Some(m) = bench.measurements.get(&Lang::Rust) {
            let badge = if Some(bench.name.as_str()) == rust_best && benchmarks.len() > 1 {
                best_label.green().to_string()
            } else if Some(bench.name.as_str()) == rust_worst && benchmarks.len() > 1 {
                worst_label.yellow().to_string()
            } else {
                String::new()
            };

            let name = format!("· rust: {}", bench.name);

            if is_memory {
                let bytes = m
                    .bytes_per_op
                    .map(Measurement::format_bytes)
                    .unwrap_or_else(|| "-".to_string());
                let allocs =
                    m.allocs_per_op.map(|a| a.to_string()).unwrap_or_else(|| "-".to_string());
                let mean_ns = m.nanos_per_op;
                let samples = m.samples.unwrap_or(1000);
                println!(
                    "   {:<40} {:>12} {:>10} {:>12} {:>8} {:>8}{}",
                    name.yellow(),
                    bytes,
                    allocs,
                    format_hz(m.ops_per_sec),
                    format_ms(mean_ns),
                    samples,
                    badge
                );
            } else if has_multi_run && m.run_count.unwrap_or(1) > 1 {
                // Multi-run format: show median and 95% CI
                let median_ns = m.median_across_runs.unwrap_or(m.nanos_per_op);
                let ci_str = if let (Some(lower), Some(upper)) = (m.ci_95_lower, m.ci_95_upper) {
                    format!("±{}", format_ms((upper - lower) / 2.0))
                } else {
                    "-".to_string()
                };
                let min_ns = m.min_nanos.unwrap_or(m.nanos_per_op as u64) as f64;
                let max_ns = m.max_nanos.unwrap_or(m.nanos_per_op as u64) as f64;
                let p75_ns = m.p75_nanos.unwrap_or(m.nanos_per_op as u64) as f64;
                let p99_ns = m.p99_nanos.unwrap_or(m.nanos_per_op as u64) as f64;
                let rme = m.rme_percent.unwrap_or(0.0);
                let cv = m.cv_percent.unwrap_or(0.0);
                let runs = m.run_count.unwrap_or(1);

                let cv_str = if m.is_stable == Some(false) {
                    format!("{:.1}%", cv).yellow().to_string()
                } else {
                    format!("{:.1}%", cv)
                };

                println!(
                    "   {:<40} {:>12} {:>10} {:>12} {:>8} {:>8} {:>8} {:>8} {:>8}% {:>7} {:>6}{}",
                    name.yellow(),
                    format_hz(m.ops_per_sec),
                    format_ms(median_ns),
                    ci_str,
                    format_ms(min_ns),
                    format_ms(max_ns),
                    format_ms(p75_ns),
                    format_ms(p99_ns),
                    format!("±{:.2}", rme),
                    cv_str,
                    runs,
                    badge
                );
            } else {
                // Single-run format (existing)
                let min_ns = m.min_nanos.unwrap_or(m.nanos_per_op as u64) as f64;
                let max_ns = m.max_nanos.unwrap_or(m.nanos_per_op as u64) as f64;
                let mean_ns = m.nanos_per_op;
                let p75_ns = m.p75_nanos.unwrap_or(m.nanos_per_op as u64) as f64;
                let p99_ns = m.p99_nanos.unwrap_or(m.nanos_per_op as u64) as f64;
                let p995_ns =
                    m.p995_nanos.unwrap_or(m.p99_nanos.unwrap_or(m.nanos_per_op as u64)) as f64;
                let rme = m.rme_percent.unwrap_or(0.0);
                let cv = m.cv_percent.unwrap_or(0.0);
                let samples = m.samples.unwrap_or(1000);

                let cv_str = if m.is_stable == Some(false) {
                    format!("{:.1}%", cv).yellow().to_string()
                } else {
                    format!("{:.1}%", cv)
                };

                println!(
                    "   {:<40} {:>12} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8}% {:>7} {:>8}{}",
                    name.yellow(),
                    format_hz(m.ops_per_sec),
                    format_ms(min_ns),
                    format_ms(max_ns),
                    format_ms(mean_ns),
                    format_ms(p75_ns),
                    format_ms(p99_ns),
                    format_ms(p995_ns),
                    format!("±{:.2}", rme),
                    cv_str,
                    samples,
                    badge
                );
            }
        }

        // Multi-language comparison row
        if bench.kind == BenchmarkKind::Async {
            println!("   {}", "  mode: async-sequential".dimmed());
        }

        let primary = |m: &Measurement| -> f64 {
            if is_memory {
                m.bytes_per_op.map(|b| b as f64).unwrap_or(f64::MAX)
            } else {
                m.nanos_per_op
            }
        };
        let go_val = bench.measurements.get(&Lang::Go).map(primary);
        let ts_val = bench.measurements.get(&Lang::TypeScript).map(primary);
        let rust_val = bench.measurements.get(&Lang::Rust).map(primary);

        // Find winner across all languages present (lower is better for both time and memory)
        let mut times: Vec<(&str, f64)> = vec![];
        if let Some(v) = go_val {
            times.push(("Go", v));
        }
        if let Some(v) = ts_val {
            times.push(("TS", v));
        }
        if let Some(v) = rust_val {
            times.push(("Rust", v));
        }

        if times.len() >= 2 {
            times.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            let (fastest_name, fastest_time) = times[0];
            let (_, second_time) = times[1];
            let speedup = second_time / fastest_time;

            let winner_str = if speedup < 1.05 {
                "  → Similar performance".dimmed().to_string()
            } else {
                let color_fn = match fastest_name {
                    "Go" => |s: String| s.green().to_string(),
                    "TS" => |s: String| s.cyan().to_string(),
                    "Rust" => |s: String| s.yellow().to_string(),
                    _ => |s: String| s,
                };
                let msg = if is_memory {
                    format!("  → {} uses {:.2}x less memory", fastest_name, speedup)
                } else {
                    format!("  → {} is {:.2}x faster", fastest_name, speedup)
                };
                color_fn(msg)
            };
            println!("{}", winner_str);
        }

        for warning in async_reliability_warnings(bench) {
            println!("   {}", format!("  ! async reliability: {}", warning).yellow());
        }

        // Add visual separation between benchmarks
        println!();
    }
}

/// Print compact table (legacy format)
fn print_compact_table(
    benchmarks: &[BenchmarkResult],
    _suite_type: poly_bench_dsl::SuiteType,
    options: &ReportOptions,
) {
    // Table header
    if options.show_ops_per_sec {
        println!(
            "    {:<25} {:>12} {:>12} {:>18} {:>12}",
            "Benchmark".underline(),
            "Go".underline(),
            "TypeScript".underline(),
            "Result".underline(),
            "ops/s".underline()
        );
    } else {
        println!(
            "    {:<30} {:>15} {:>15} {:>20}",
            "Benchmark".underline(),
            "Go".underline(),
            "TypeScript".underline(),
            "Result".underline()
        );
    }

    for bench in benchmarks {
        let go_measurement = bench.measurements.get(&Lang::Go);
        let ts_measurement = bench.measurements.get(&Lang::TypeScript);

        let go_str = go_measurement
            .map(|m| Measurement::format_duration(m.nanos_per_op))
            .unwrap_or_else(|| "-".to_string());

        let ts_str = ts_measurement
            .map(|m| Measurement::format_duration(m.nanos_per_op))
            .unwrap_or_else(|| "-".to_string());

        let result_str = if let Some(ref cmp) = bench.comparison {
            cmp.speedup_description()
        } else {
            "-".to_string()
        };

        let result_colored = if let Some(ref cmp) = bench.comparison {
            match cmp.winner {
                poly_bench_runtime::measurement::ComparisonWinner::First => {
                    result_str.green().to_string()
                }
                poly_bench_runtime::measurement::ComparisonWinner::Second => {
                    result_str.cyan().to_string()
                }
                poly_bench_runtime::measurement::ComparisonWinner::Tie => {
                    result_str.dimmed().to_string()
                }
            }
        } else {
            result_str.dimmed().to_string()
        };

        let ops_str = if let Some(ref cmp) = bench.comparison {
            let go_ops = cmp.first.ops_per_sec;
            let ts_ops = cmp.second.ops_per_sec;
            format!(
                "{} / {}",
                format_ops_per_sec(go_ops).green(),
                format_ops_per_sec(ts_ops).cyan()
            )
        } else if let Some(m) = go_measurement {
            format_ops_per_sec(m.ops_per_sec).green().to_string()
        } else if let Some(m) = ts_measurement {
            format_ops_per_sec(m.ops_per_sec).cyan().to_string()
        } else {
            "-".to_string()
        };

        if options.show_ops_per_sec {
            println!(
                "    {:<25} {:>12} {:>12} {:>18} {:>12}",
                bench.name,
                go_str.green(),
                ts_str.cyan(),
                result_colored,
                ops_str
            );
        } else {
            println!(
                "    {:<30} {:>15} {:>15} {:>20}",
                bench.name,
                go_str.green(),
                ts_str.cyan(),
                result_colored
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::async_reliability_warnings;
    use poly_bench_dsl::{BenchmarkKind, Lang};
    use poly_bench_executor::comparison::BenchmarkResult;
    use poly_bench_runtime::measurement::Measurement;
    use std::collections::HashMap;

    #[test]
    fn test_async_reliability_warnings_trigger_for_low_success_ratio_and_spread() {
        let mut measurements = HashMap::new();
        let mut go = Measurement::from_aggregate(100, 100_000);
        go.async_success_count = Some(80);
        go.async_error_count = Some(20);
        measurements.insert(Lang::Go, go);

        let mut ts = Measurement::from_aggregate(100, 90_000);
        ts.async_success_count = Some(99);
        ts.async_error_count = Some(1);
        measurements.insert(Lang::TypeScript, ts);

        let bench = BenchmarkResult::new(
            "rpc".to_string(),
            "suite_rpc".to_string(),
            BenchmarkKind::Async,
            None,
            measurements,
            poly_bench_dsl::SuiteType::Performance,
            "strict".to_string(),
            None,
            Some(5),
            Some(50),
            Some("timeBudgeted".to_string()),
        );

        let warnings = async_reliability_warnings(&bench);
        assert!(warnings.iter().any(|w| w.contains("Go success ratio")));
        assert!(warnings.iter().any(|w| w.contains("spread")));
    }

    #[test]
    fn test_async_reliability_warnings_empty_for_sync_benchmarks() {
        let bench = BenchmarkResult::new(
            "hash".to_string(),
            "suite_hash".to_string(),
            BenchmarkKind::Sync,
            None,
            HashMap::new(),
            poly_bench_dsl::SuiteType::Performance,
            "strict".to_string(),
            None,
            None,
            None,
            None,
        );

        assert!(async_reliability_warnings(&bench).is_empty());
    }
}
