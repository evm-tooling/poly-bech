//! Modular chart generators for benchmark visualization
//!
//! Provides bar charts, pie charts, and line charts for visualizing benchmark results.

pub mod bar_chart;
pub mod line_chart;
pub mod pie_chart;
pub mod regression;

use poly_bench_dsl::Lang;
use poly_bench_executor::comparison::BenchmarkResult;
use poly_bench_ir::ChartDirectiveIR;
use poly_bench_runtime::measurement::ComparisonWinner;

// Default chart dimensions
pub const DEFAULT_WIDTH: i32 = 880;
pub const DEFAULT_MARGIN_TOP: i32 = 60;
pub const DEFAULT_MARGIN_BOTTOM: i32 = 50;
pub const DEFAULT_MARGIN_LEFT: i32 = 70;
pub const DEFAULT_MARGIN_RIGHT: i32 = 30;

// Default colors
pub const GO_COLOR: &str = "#00ADD8";
pub const TS_COLOR: &str = "#3178C6";
pub const RUST_COLOR: &str = "#DEA584"; // Rust's official logo color (orange-ish)
pub const TIE_COLOR: &str = "#9CA3AF";
pub const BG_COLOR: &str = "#FAFAFA";
pub const BORDER_COLOR: &str = "#E5E7EB";
pub const TEXT_COLOR: &str = "#111827";
pub const TEXT_SECONDARY: &str = "#6B7280";
pub const TEXT_TERTIARY: &str = "#4B5563";
pub const TEXT_MUTED: &str = "#9CA3AF";
pub const GO_GRADIENT_END: &str = "#0891B2";
pub const TS_GRADIENT_END: &str = "#1D4ED8";
pub const RUST_GRADIENT_END: &str = "#B7410E"; // Darker rust color

// Color palette for pie charts
pub const PIE_COLORS: &[&str] = &[
    "#00ADD8", // Go blue
    "#3178C6", // TS blue
    "#DEA584", // Rust orange
    "#10B981", // Emerald
    "#F59E0B", // Amber
    "#EF4444", // Red
    "#8B5CF6", // Violet
    "#EC4899", // Pink
    "#06B6D4", // Cyan
];

/// Escape XML special characters
pub fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// Get color for a language
pub fn lang_color(lang: Lang) -> &'static str {
    match lang {
        Lang::Go => GO_COLOR,
        Lang::TypeScript => TS_COLOR,
        Lang::Rust => RUST_COLOR,
        _ => TIE_COLOR,
    }
}

/// Format duration for display
pub fn format_duration(nanos: f64) -> String {
    if nanos >= 1_000_000_000.0 {
        format!("{:.2}s", nanos / 1_000_000_000.0)
    } else if nanos >= 1_000_000.0 {
        format!("{:.2}ms", nanos / 1_000_000.0)
    } else if nanos >= 1_000.0 {
        format!("{:.2}µs", nanos / 1_000.0)
    } else {
        format!("{:.0}ns", nanos)
    }
}

/// Generate SVG header with common styles
pub fn svg_header(width: i32, height: i32) -> String {
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n\
<defs>\n\
  <linearGradient id=\"goGrad\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">\n\
    <stop offset=\"0%\" stop-color=\"{}\" stop-opacity=\"0.95\"/>\n\
    <stop offset=\"100%\" stop-color=\"{}\" stop-opacity=\"0.85\"/>\n\
  </linearGradient>\n\
  <linearGradient id=\"tsGrad\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">\n\
    <stop offset=\"0%\" stop-color=\"{}\" stop-opacity=\"0.95\"/>\n\
    <stop offset=\"100%\" stop-color=\"{}\" stop-opacity=\"0.85\"/>\n\
  </linearGradient>\n\
  <linearGradient id=\"rustGrad\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">\n\
    <stop offset=\"0%\" stop-color=\"{}\" stop-opacity=\"0.95\"/>\n\
    <stop offset=\"100%\" stop-color=\"{}\" stop-opacity=\"0.85\"/>\n\
  </linearGradient>\n\
</defs>\n\
<rect width=\"{}\" height=\"{}\" fill=\"{}\" rx=\"12\"/>\n\
<rect x=\".5\" y=\".5\" width=\"{}\" height=\"{}\" fill=\"none\" stroke=\"{}\" rx=\"12\"/>\n",
        width, height, width, height,
        GO_COLOR, GO_GRADIENT_END,
        TS_COLOR, TS_GRADIENT_END,
        RUST_COLOR, RUST_GRADIENT_END,
        width, height, BG_COLOR,
        width - 1, height - 1, BORDER_COLOR
    )
}

/// Generate title text
pub fn svg_title(width: i32, title: &str, subtitle: Option<&str>) -> String {
    let mut svg = format!(
        "<text x=\"{}\" y=\"30\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"16\" font-weight=\"700\" fill=\"{}\">{}</text>\n",
        width / 2,
        TEXT_COLOR,
        escape_xml(title)
    );

    if let Some(sub) = subtitle {
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"48\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"11\" fill=\"{}\">{}</text>\n",
            width / 2,
            TEXT_SECONDARY,
            escape_xml(sub)
        ));
    }

    svg
}

/// Generate a legend
pub fn svg_legend(width: i32, y: i32, items: &[(&str, &str)]) -> String {
    let mut svg =
        format!("<g transform=\"translate({},{})\">\n", width / 2 - (items.len() as i32 * 50), y);

    for (i, (color, label)) in items.iter().enumerate() {
        let x = i as i32 * 100;
        svg.push_str(&format!(
            "  <rect x=\"{}\" width=\"14\" height=\"14\" fill=\"{}\" rx=\"3\"/>\n\
  <text x=\"{}\" y=\"11\" font-family=\"sans-serif\" font-size=\"11\" fill=\"{}\">{}</text>\n",
            x,
            color,
            x + 18,
            TEXT_TERTIARY,
            escape_xml(label)
        ));
    }

    svg.push_str("</g>\n");
    svg
}

// ============================================================================
// Filter and Sort Utilities
// ============================================================================

/// Filter benchmarks based on directive parameters
pub fn filter_benchmarks<'a>(
    benchmarks: Vec<&'a BenchmarkResult>,
    directive: &ChartDirectiveIR,
) -> Vec<&'a BenchmarkResult> {
    let mut filtered: Vec<&BenchmarkResult> = benchmarks
        .into_iter()
        .filter(|bench| {
            // Apply include filter
            if !directive.include_benchmarks.is_empty() {
                if !directive
                    .include_benchmarks
                    .iter()
                    .any(|name: &String| bench.name.to_lowercase().contains(&name.to_lowercase()))
                {
                    return false;
                }
            }

            // Apply exclude filter
            if directive
                .exclude_benchmarks
                .iter()
                .any(|name: &String| bench.name.to_lowercase().contains(&name.to_lowercase()))
            {
                return false;
            }

            // Apply min_speedup filter
            if let Some(min_speedup) = directive.min_speedup {
                if let Some(ref comparison) = bench.comparison {
                    let speedup = if comparison.first.nanos_per_op > 0.0 {
                        comparison.second.nanos_per_op / comparison.first.nanos_per_op
                    } else {
                        1.0
                    };
                    if speedup.abs() < min_speedup {
                        return false;
                    }
                }
            }

            // Apply filter_winner filter
            // Note: Currently filter_winner is designed for 2-language comparisons
            // For multi-language scenarios, the comparison module would need to be updated
            if let Some(ref winner_filter) = directive.filter_winner {
                if let Some(ref comparison) = bench.comparison {
                    let wf = winner_filter.to_lowercase();
                    match wf.as_str() {
                        "go" => {
                            if comparison.winner != ComparisonWinner::First {
                                return false;
                            }
                        }
                        "ts" | "typescript" => {
                            if comparison.winner != ComparisonWinner::Second {
                                return false;
                            }
                        }
                        // Rust filtering would require multi-language comparison support
                        "rust" => {
                            // For now, Rust wins would need to be added to the comparison module
                            // This is a placeholder for future multi-language support
                        }
                        "all" | _ => {} // No filter
                    }
                }
            }

            true
        })
        .collect();

    // Apply limit
    if let Some(limit) = directive.limit {
        if limit > 0 {
            filtered.truncate(limit as usize);
        }
    }

    filtered
}

/// Extract numeric value from a benchmark name for natural sorting
/// e.g., "n100" -> 100, "size1000" -> 1000, "bench_42_test" -> 42
pub fn extract_numeric_value(name: &str) -> Option<i64> {
    // Find all contiguous digit sequences and return the first one
    let mut num_str = String::new();
    let mut found_digit = false;

    for ch in name.chars() {
        if ch.is_ascii_digit() {
            num_str.push(ch);
            found_digit = true;
        } else if found_digit {
            // Stop at first non-digit after finding digits
            break;
        }
    }

    if num_str.is_empty() {
        None
    } else {
        num_str.parse().ok()
    }
}

/// Sort benchmarks based on directive parameters
pub fn sort_benchmarks(benchmarks: &mut [&BenchmarkResult], directive: &ChartDirectiveIR) {
    // Default to "natural" sorting which handles numeric values in names correctly
    let sort_by = directive.sort_by.as_deref().unwrap_or("natural");
    let sort_desc = directive.sort_order.as_deref().unwrap_or("asc") == "desc";

    benchmarks.sort_by(|a, b| {
        let cmp = match sort_by {
            "speedup" => {
                let speedup_a = a
                    .comparison
                    .as_ref()
                    .map(|c| {
                        if c.first.nanos_per_op > 0.0 {
                            c.second.nanos_per_op / c.first.nanos_per_op
                        } else {
                            1.0
                        }
                    })
                    .unwrap_or(1.0);
                let speedup_b = b
                    .comparison
                    .as_ref()
                    .map(|c| {
                        if c.first.nanos_per_op > 0.0 {
                            c.second.nanos_per_op / c.first.nanos_per_op
                        } else {
                            1.0
                        }
                    })
                    .unwrap_or(1.0);
                speedup_a.partial_cmp(&speedup_b).unwrap_or(std::cmp::Ordering::Equal)
            }
            "time" => {
                let time_a = a
                    .comparison
                    .as_ref()
                    .map(|c| c.first.nanos_per_op.min(c.second.nanos_per_op))
                    .unwrap_or(f64::MAX);
                let time_b = b
                    .comparison
                    .as_ref()
                    .map(|c| c.first.nanos_per_op.min(c.second.nanos_per_op))
                    .unwrap_or(f64::MAX);
                time_a.partial_cmp(&time_b).unwrap_or(std::cmp::Ordering::Equal)
            }
            "ops" => {
                let ops_a = a
                    .comparison
                    .as_ref()
                    .map(|c| c.first.ops_per_sec.max(c.second.ops_per_sec))
                    .unwrap_or(0.0);
                let ops_b = b
                    .comparison
                    .as_ref()
                    .map(|c| c.first.ops_per_sec.max(c.second.ops_per_sec))
                    .unwrap_or(0.0);
                ops_a.partial_cmp(&ops_b).unwrap_or(std::cmp::Ordering::Equal)
            }
            "name" => a.name.cmp(&b.name),
            // "natural" is the default - sorts by numeric value in name, then alphabetically
            "natural" | _ => {
                let num_a = extract_numeric_value(&a.name);
                let num_b = extract_numeric_value(&b.name);

                match (num_a, num_b) {
                    // Both have numeric values - sort by number
                    (Some(na), Some(nb)) => na.cmp(&nb),
                    // Only one has numeric - numeric comes first
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    // Neither has numeric - fall back to alphabetical
                    (None, None) => a.name.cmp(&b.name),
                }
            }
        };

        if sort_desc {
            cmp.reverse()
        } else {
            cmp
        }
    });
}

/// Format duration with custom time unit
pub fn format_duration_with_unit(
    nanos: f64,
    time_unit: Option<&str>,
    precision: Option<u32>,
) -> String {
    let precision = precision.unwrap_or(2) as usize;

    match time_unit {
        Some("ns") => format!("{:.precision$}ns", nanos),
        Some("us") => format!("{:.precision$}µs", nanos / 1_000.0),
        Some("ms") => format!("{:.precision$}ms", nanos / 1_000_000.0),
        Some("s") => format!("{:.precision$}s", nanos / 1_000_000_000.0),
        Some("auto") | None => format_duration(nanos),
        Some(_) => format_duration(nanos), // Unknown unit, use auto
    }
}

/// Format ops/sec for display
pub fn format_ops_per_sec(ops: f64) -> String {
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

/// Format config footer for charts
pub fn format_config_footer(
    iterations: Option<u64>,
    warmup: Option<u64>,
    timeout: Option<u64>,
    order: Option<&str>,
) -> String {
    let mut parts = Vec::new();

    if let Some(iter) = iterations {
        parts.push(format!("{} iterations", iter));
    }
    if let Some(warm) = warmup {
        parts.push(format!("{} warmup", warm));
    }
    if let Some(to) = timeout {
        parts.push(format!("{}ms timeout", to));
    }
    if let Some(ord) = order {
        parts.push(ord.to_string());
    }

    parts.join(" | ")
}

/// Format stats label for a measurement
pub fn format_stats_label(
    time_nanos: f64,
    ops_per_sec: f64,
    time_unit: Option<&str>,
    precision: Option<u32>,
) -> String {
    let time_str = format_duration_with_unit(time_nanos, time_unit, precision);
    let ops_str = format_ops_per_sec(ops_per_sec);
    format!("{} ({} ops/s)", time_str, ops_str)
}

/// Calculate geometric mean of speedups
pub fn calculate_geo_mean(benchmarks: &[&BenchmarkResult]) -> f64 {
    let log_speedups: Vec<f64> = benchmarks
        .iter()
        .filter_map(|b| {
            b.comparison.as_ref().and_then(|c| {
                if c.first.nanos_per_op > 0.0 && c.second.nanos_per_op > 0.0 {
                    Some((c.second.nanos_per_op / c.first.nanos_per_op).ln())
                } else {
                    None
                }
            })
        })
        .collect();

    if log_speedups.is_empty() {
        1.0
    } else {
        let avg_log = log_speedups.iter().sum::<f64>() / log_speedups.len() as f64;
        avg_log.exp()
    }
}

/// Count wins by language (returns go_wins, ts_wins, rust_wins, ties)
pub fn count_wins(benchmarks: &[&BenchmarkResult]) -> (usize, usize, usize, usize) {
    let mut go_wins = 0;
    let mut ts_wins = 0;
    let mut rust_wins = 0;
    let mut ties = 0;

    for bench in benchmarks {
        // Determine winner across all available languages
        let go_ns = bench.measurements.get(&Lang::Go).map(|m| m.nanos_per_op);
        let ts_ns = bench.measurements.get(&Lang::TypeScript).map(|m| m.nanos_per_op);
        let rust_ns = bench.measurements.get(&Lang::Rust).map(|m| m.nanos_per_op);

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
            times.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            let (fastest_lang, fastest_time) = times[0];
            let (_, second_time) = times[1];

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
        }
    }

    (go_wins, ts_wins, rust_wins, ties)
}
