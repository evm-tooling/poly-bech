//! Bar chart generator - horizontal bar chart with Go vs TS comparison
//! Each benchmark is displayed as a row with two horizontal bars (Go and TS)
//! Uses smart scaling to prevent extreme values from making other bars invisible

use miette::Result;
use poly_bench_dsl::Lang;
use poly_bench_executor::BenchmarkResults;
use poly_bench_ir::ChartDirectiveIR;
use poly_bench_runtime::measurement::ComparisonWinner;

use super::{
    calculate_geo_mean, count_wins, escape_xml, filter_benchmarks, format_duration_with_unit,
    format_ops_per_sec, sort_benchmarks, svg_header, svg_title, BORDER_COLOR,
    DEFAULT_MARGIN_BOTTOM, DEFAULT_MARGIN_TOP, GO_COLOR, TEXT_COLOR, TEXT_MUTED, TEXT_SECONDARY,
    TS_COLOR,
};

// Layout constants for horizontal bar chart
const BAR_HEIGHT: i32 = 20; // Height of each bar
const BAR_GAP: i32 = 4; // Gap between Go and TS bars within a benchmark
const ROW_HEIGHT: i32 = 70; // Total height per benchmark row (Go + TS bars + spacing)
const LABEL_WIDTH: i32 = 180; // Width reserved for benchmark name labels
const VALUE_LABEL_WIDTH: i32 = 120; // Width reserved for value labels on right
const STATS_BOX_HEIGHT: i32 = 80;
const LEGEND_HEIGHT: i32 = 40;
const MIN_CHART_WIDTH: i32 = 600; // Minimum width for the chart area

// Scaling constants to prevent tiny bars
const MIN_BAR_WIDTH_RATIO: f64 = 0.05; // Minimum bar width as ratio of max (5%)
const LOG_SCALE_THRESHOLD: f64 = 100.0; // Use log scaling if max/min ratio exceeds this

/// Suite configuration for display
pub struct SuiteConfig {
    pub iterations: Option<u64>,
    pub warmup: Option<u64>,
    pub timeout_ms: Option<u64>,
    pub order: Option<String>,
}

/// Generate a horizontal bar chart SVG with each benchmark as a row
/// Uses smart scaling to prevent extreme values from making other bars invisible
pub fn generate(
    results: &BenchmarkResults,
    directive: &ChartDirectiveIR,
    suite_config: Option<&SuiteConfig>,
) -> Result<String> {
    let mut svg = String::new();

    // Collect all benchmarks with comparisons
    let all_benchmarks: Vec<_> = results
        .suites
        .iter()
        .flat_map(|s| s.benchmarks.iter())
        .filter(|b| b.comparison.is_some())
        .collect();

    if all_benchmarks.is_empty() {
        return Ok("<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>".to_string());
    }

    // Apply filtering
    let mut filtered = filter_benchmarks(all_benchmarks, directive);

    // Apply sorting
    sort_benchmarks(&mut filtered, directive);

    if filtered.is_empty() {
        return Ok("<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>".to_string());
    }

    // Extract directive parameters with defaults
    let bar_height = directive.bar_height.unwrap_or(BAR_HEIGHT);
    let bar_gap = directive.bar_gap.unwrap_or(BAR_GAP);
    let time_unit = directive.time_unit.as_deref();
    let precision = directive.precision;
    let compact = directive.compact;

    // Calculate dynamic dimensions based on number of benchmarks
    let num_benchmarks = filtered.len() as i32;

    // Calculate width
    let margin_x = 20;
    let stats_box_needed =
        (directive.show_stats || directive.show_distribution || directive.show_geo_mean)
            && !compact;
    let width = directive
        .width
        .unwrap_or(MIN_CHART_WIDTH + margin_x * 2)
        .max(MIN_CHART_WIDTH);
    let bar_area_width = width - LABEL_WIDTH - VALUE_LABEL_WIDTH - margin_x * 2;

    // Calculate summary stats for legend
    let (go_wins, ts_wins, ties) = count_wins(&filtered);
    let geo_mean = calculate_geo_mean(&filtered);

    // Height calculation
    let legend_space = LEGEND_HEIGHT;
    let stats_box_space = if stats_box_needed {
        STATS_BOX_HEIGHT
    } else {
        0
    };
    let config_space = if directive.show_config && suite_config.is_some() && !compact {
        24
    } else {
        0
    };
    let chart_area_height = num_benchmarks * ROW_HEIGHT;
    let height = DEFAULT_MARGIN_TOP
        + chart_area_height
        + legend_space
        + stats_box_space
        + config_space
        + DEFAULT_MARGIN_BOTTOM;

    // Collect all values for smart scaling
    let all_values: Vec<f64> = filtered
        .iter()
        .filter_map(|b| b.comparison.as_ref())
        .flat_map(|c| {
            let first_val = c.first.median_across_runs.unwrap_or(c.first.nanos_per_op);
            let second_val = c.second.median_across_runs.unwrap_or(c.second.nanos_per_op);
            [first_val, second_val]
        })
        .collect();

    let max_value = all_values.iter().cloned().fold(1.0, f64::max);
    let min_value = all_values
        .iter()
        .cloned()
        .fold(f64::MAX, f64::min)
        .max(0.001);
    let value_ratio = max_value / min_value;

    // Determine scaling mode: use log scale if ratio is extreme
    let use_log_scale = value_ratio > LOG_SCALE_THRESHOLD;

    // Build subtitle
    let scale_note = if use_log_scale { " (log scale)" } else { "" };
    let subtitle = directive.description.clone().unwrap_or_else(|| {
        let winner_text = match results.summary.winner {
            Some(Lang::Go) => format!(
                "Go is {:.1}x faster overall",
                results.summary.geo_mean_speedup
            ),
            Some(Lang::TypeScript) => format!(
                "TypeScript is {:.1}x faster overall",
                1.0 / results.summary.geo_mean_speedup
            ),
            _ => "Similar performance".to_string(),
        };
        format!("{}{}", winner_text, scale_note)
    });

    let title = directive
        .title
        .clone()
        .unwrap_or_else(|| "Benchmark Results".to_string());

    // SVG header
    svg.push_str(&svg_header(width, height));

    // Title
    svg.push_str(&svg_title(width, &title, Some(&subtitle)));

    // Draw horizontal bar chart rows
    let start_y = DEFAULT_MARGIN_TOP + 20;

    for (i, bench) in filtered.iter().enumerate() {
        if let Some(ref cmp) = bench.comparison {
            let row_y = start_y + (i as i32 * ROW_HEIGHT);

            // Get values - use median when available (for count > 1), otherwise nanos_per_op
            let go_value = cmp
                .first
                .median_across_runs
                .unwrap_or(cmp.first.nanos_per_op);
            let ts_value = cmp
                .second
                .median_across_runs
                .unwrap_or(cmp.second.nanos_per_op);

            // Calculate bar widths with smart scaling
            let (go_bar_width, ts_bar_width) = if use_log_scale {
                // Log scale: ensures small values are still visible
                let log_max = max_value.ln();
                let log_min = min_value.ln();
                let log_range = log_max - log_min;

                let go_log = go_value.max(min_value).ln();
                let ts_log = ts_value.max(min_value).ln();

                let go_ratio = (go_log - log_min) / log_range;
                let ts_ratio = (ts_log - log_min) / log_range;

                // Map to bar width with minimum visible width
                let go_w = (go_ratio * bar_area_width as f64)
                    .max(bar_area_width as f64 * MIN_BAR_WIDTH_RATIO);
                let ts_w = (ts_ratio * bar_area_width as f64)
                    .max(bar_area_width as f64 * MIN_BAR_WIDTH_RATIO);
                (go_w, ts_w)
            } else {
                // Linear scale with minimum bar width
                let go_ratio = go_value / max_value;
                let ts_ratio = ts_value / max_value;

                let go_w = (go_ratio * bar_area_width as f64)
                    .max(bar_area_width as f64 * MIN_BAR_WIDTH_RATIO);
                let ts_w = (ts_ratio * bar_area_width as f64)
                    .max(bar_area_width as f64 * MIN_BAR_WIDTH_RATIO);
                (go_w, ts_w)
            };

            let go_winner = cmp.winner == ComparisonWinner::First;
            let ts_winner = cmp.winner == ComparisonWinner::Second;

            // Draw row group
            svg.push_str(&format!(
                "<g transform=\"translate({},{})\">\n",
                margin_x, row_y
            ));

            // Benchmark name label (left side)
            let name_display = if bench.name.len() > 20 {
                format!("{}...", &bench.name[..17])
            } else {
                bench.name.clone()
            };
            svg.push_str(&format!(
                "  <text x=\"{}\" y=\"{}\" text-anchor=\"end\" font-family=\"sans-serif\" font-size=\"11\" font-weight=\"500\" fill=\"{}\">{}</text>\n",
                LABEL_WIDTH - 10, bar_height + bar_gap / 2, TEXT_COLOR, escape_xml(&name_display)
            ));

            // Winner indicator (speedup) below name
            if go_winner || ts_winner {
                let winner_color = if go_winner { GO_COLOR } else { TS_COLOR };
                let speedup = if go_winner {
                    ts_value / go_value
                } else {
                    go_value / ts_value
                };
                let winner_label = if go_winner { "Go" } else { "TS" };
                svg.push_str(&format!(
                    "  <text x=\"{}\" y=\"{}\" text-anchor=\"end\" font-family=\"sans-serif\" font-size=\"9\" font-weight=\"600\" fill=\"{}\">{} {:.1}x faster</text>\n",
                    LABEL_WIDTH - 10, bar_height * 2 + bar_gap + 4, winner_color, winner_label, speedup
                ));
            }

            // Go bar (top)
            let go_bar_y = 0;
            svg.push_str(&format!(
                "  <rect x=\"{}\" y=\"{}\" width=\"{:.1}\" height=\"{}\" fill=\"url(#goGrad)\" rx=\"3\"/>\n",
                LABEL_WIDTH, go_bar_y, go_bar_width, bar_height
            ));

            // Go label on bar
            svg.push_str(&format!(
                "  <text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"9\" font-weight=\"500\" fill=\"white\">Go</text>\n",
                LABEL_WIDTH + 6, go_bar_y + bar_height - 5
            ));

            // Go value label (right of bar)
            let go_label = if let (Some(median), Some(ci_upper)) =
                (cmp.first.median_across_runs, cmp.first.ci_95_upper)
            {
                let ci_half = ci_upper - median;
                format!(
                    "{} ±{}",
                    format_duration_with_unit(median, time_unit, precision),
                    format_duration_with_unit(ci_half, time_unit, precision)
                )
            } else {
                format_duration_with_unit(go_value, time_unit, precision)
            };
            svg.push_str(&format!(
                "  <text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"9\" font-weight=\"{}\" fill=\"{}\">{}</text>\n",
                LABEL_WIDTH + bar_area_width + 8, go_bar_y + bar_height - 5,
                if go_winner { "600" } else { "400" },
                if go_winner { "#0E7490" } else { TEXT_MUTED },
                escape_xml(&go_label)
            ));

            // Go error bar (horizontal, at end of bar)
            if let (Some(ci_lower), Some(ci_upper)) = (cmp.first.ci_95_lower, cmp.first.ci_95_upper)
            {
                let ci_range = ci_upper - ci_lower;
                let error_bar_half_width = if use_log_scale {
                    // Scale error bar for log scale
                    ((ci_range / go_value) * go_bar_width / 2.0)
                        .max(3.0)
                        .min(20.0)
                } else {
                    (ci_range / max_value * bar_area_width as f64 / 2.0)
                        .max(3.0)
                        .min(20.0)
                };
                let error_bar_x = LABEL_WIDTH as f64 + go_bar_width;
                let error_bar_y = go_bar_y + bar_height / 2;
                let cap_height = 3;

                // Horizontal error bar line
                svg.push_str(&format!(
                    "  <line x1=\"{:.1}\" y1=\"{}\" x2=\"{:.1}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"1.5\" opacity=\"0.7\"/>\n",
                    error_bar_x - error_bar_half_width, error_bar_y,
                    error_bar_x + error_bar_half_width, error_bar_y,
                    GO_COLOR
                ));
                // Left cap
                svg.push_str(&format!(
                    "  <line x1=\"{:.1}\" y1=\"{}\" x2=\"{:.1}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"1.5\" opacity=\"0.7\"/>\n",
                    error_bar_x - error_bar_half_width, error_bar_y - cap_height,
                    error_bar_x - error_bar_half_width, error_bar_y + cap_height,
                    GO_COLOR
                ));
                // Right cap
                svg.push_str(&format!(
                    "  <line x1=\"{:.1}\" y1=\"{}\" x2=\"{:.1}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"1.5\" opacity=\"0.7\"/>\n",
                    error_bar_x + error_bar_half_width, error_bar_y - cap_height,
                    error_bar_x + error_bar_half_width, error_bar_y + cap_height,
                    GO_COLOR
                ));
            }

            // TypeScript bar (bottom)
            let ts_bar_y = bar_height + bar_gap;
            svg.push_str(&format!(
                "  <rect x=\"{}\" y=\"{}\" width=\"{:.1}\" height=\"{}\" fill=\"url(#tsGrad)\" rx=\"3\"/>\n",
                LABEL_WIDTH, ts_bar_y, ts_bar_width, bar_height
            ));

            // TS label on bar
            svg.push_str(&format!(
                "  <text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"9\" font-weight=\"500\" fill=\"white\">TS</text>\n",
                LABEL_WIDTH + 6, ts_bar_y + bar_height - 5
            ));

            // TS value label (right of bar)
            let ts_label = if let (Some(median), Some(ci_upper)) =
                (cmp.second.median_across_runs, cmp.second.ci_95_upper)
            {
                let ci_half = ci_upper - median;
                format!(
                    "{} ±{}",
                    format_duration_with_unit(median, time_unit, precision),
                    format_duration_with_unit(ci_half, time_unit, precision)
                )
            } else {
                format_duration_with_unit(ts_value, time_unit, precision)
            };
            svg.push_str(&format!(
                "  <text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"9\" font-weight=\"{}\" fill=\"{}\">{}</text>\n",
                LABEL_WIDTH + bar_area_width + 8, ts_bar_y + bar_height - 5,
                if ts_winner { "600" } else { "400" },
                if ts_winner { "#1E40AF" } else { TEXT_MUTED },
                escape_xml(&ts_label)
            ));

            // TS error bar (horizontal, at end of bar)
            if let (Some(ci_lower), Some(ci_upper)) =
                (cmp.second.ci_95_lower, cmp.second.ci_95_upper)
            {
                let ci_range = ci_upper - ci_lower;
                let error_bar_half_width = if use_log_scale {
                    ((ci_range / ts_value) * ts_bar_width / 2.0)
                        .max(3.0)
                        .min(20.0)
                } else {
                    (ci_range / max_value * bar_area_width as f64 / 2.0)
                        .max(3.0)
                        .min(20.0)
                };
                let error_bar_x = LABEL_WIDTH as f64 + ts_bar_width;
                let error_bar_y = ts_bar_y + bar_height / 2;
                let cap_height = 3;

                // Horizontal error bar line
                svg.push_str(&format!(
                    "  <line x1=\"{:.1}\" y1=\"{}\" x2=\"{:.1}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"1.5\" opacity=\"0.7\"/>\n",
                    error_bar_x - error_bar_half_width, error_bar_y,
                    error_bar_x + error_bar_half_width, error_bar_y,
                    TS_COLOR
                ));
                // Left cap
                svg.push_str(&format!(
                    "  <line x1=\"{:.1}\" y1=\"{}\" x2=\"{:.1}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"1.5\" opacity=\"0.7\"/>\n",
                    error_bar_x - error_bar_half_width, error_bar_y - cap_height,
                    error_bar_x - error_bar_half_width, error_bar_y + cap_height,
                    TS_COLOR
                ));
                // Right cap
                svg.push_str(&format!(
                    "  <line x1=\"{:.1}\" y1=\"{}\" x2=\"{:.1}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"1.5\" opacity=\"0.7\"/>\n",
                    error_bar_x + error_bar_half_width, error_bar_y - cap_height,
                    error_bar_x + error_bar_half_width, error_bar_y + cap_height,
                    TS_COLOR
                ));
            }

            // Separator line between benchmarks
            if i < filtered.len() - 1 {
                svg.push_str(&format!(
                    "  <line x1=\"0\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"0.5\" opacity=\"0.3\"/>\n",
                    ROW_HEIGHT - 5, width - margin_x * 2, ROW_HEIGHT - 5, BORDER_COLOR
                ));
            }

            svg.push_str("</g>\n");
        }
    }

    // Legend
    let legend_y = start_y + chart_area_height + 20;
    svg.push_str(&format!(
        "<g transform=\"translate({},{})\">\n",
        width / 2,
        legend_y
    ));

    // Go indicator
    let go_legend_label = if directive.show_win_counts {
        format!("Go ({} wins)", go_wins)
    } else {
        "Go".to_string()
    };
    svg.push_str(&format!(
        "  <rect x=\"-140\" width=\"14\" height=\"14\" fill=\"{}\" rx=\"3\"/>\
         <text x=\"-122\" y=\"11\" font-family=\"sans-serif\" font-size=\"11\" fill=\"{}\">{}</text>\n",
        GO_COLOR, TEXT_COLOR, escape_xml(&go_legend_label)
    ));

    // TS indicator
    let ts_legend_label = if directive.show_win_counts {
        format!("TypeScript ({} wins)", ts_wins)
    } else {
        "TypeScript".to_string()
    };
    svg.push_str(&format!(
        "  <rect x=\"30\" width=\"14\" height=\"14\" fill=\"{}\" rx=\"3\"/>\
         <text x=\"48\" y=\"11\" font-family=\"sans-serif\" font-size=\"11\" fill=\"{}\">{}</text>\n",
        TS_COLOR, TEXT_COLOR, escape_xml(&ts_legend_label)
    ));

    // Ties indicator
    if ties > 0 && directive.show_win_counts {
        svg.push_str(&format!(
            "  <text x=\"180\" y=\"11\" font-family=\"sans-serif\" font-size=\"11\" fill=\"{}\">Ties: {}</text>\n",
            TEXT_MUTED, ties
        ));
    }

    // Log scale indicator
    if use_log_scale {
        svg.push_str(&format!(
            "  <text x=\"250\" y=\"11\" font-family=\"sans-serif\" font-size=\"10\" font-style=\"italic\" fill=\"{}\">(logarithmic scale)</text>\n",
            TEXT_MUTED
        ));
    }

    svg.push_str("</g>\n");

    // Stats box below legend
    if stats_box_needed {
        let box_y = legend_y + LEGEND_HEIGHT;
        let box_margin = 40;
        let box_width = width - box_margin * 2;
        let box_x = box_margin;

        // Box background with border
        svg.push_str(&format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#F9FAFB\" stroke=\"{}\" rx=\"6\"/>\n",
            box_x, box_y, box_width, STATS_BOX_HEIGHT - 10, BORDER_COLOR
        ));

        // Stats content
        let stats_x = box_x + 12;
        let mut stats_y = box_y + 18;

        // Header row
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"10\" font-weight=\"600\" fill=\"{}\">SUMMARY STATISTICS</text>\n",
            stats_x, stats_y, TEXT_COLOR
        ));
        stats_y += 16;

        // Geo mean
        if directive.show_geo_mean {
            let geo_label = if geo_mean >= 1.0 {
                format!("Go is {:.2}x faster on average (geometric mean)", geo_mean)
            } else {
                format!(
                    "TypeScript is {:.2}x faster on average (geometric mean)",
                    1.0 / geo_mean
                )
            };
            svg.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"10\" fill=\"{}\">• {}</text>\n",
                stats_x, stats_y, TEXT_SECONDARY, escape_xml(&geo_label)
            ));
            stats_y += 14;
        }

        // Distribution stats (aggregate)
        if directive.show_distribution {
            let go_p50_avg: f64 = filtered
                .iter()
                .filter_map(|b| b.comparison.as_ref())
                .filter_map(|c| c.first.p50_nanos)
                .map(|n| n as f64)
                .sum::<f64>()
                / filtered.len().max(1) as f64;
            let go_p99_avg: f64 = filtered
                .iter()
                .filter_map(|b| b.comparison.as_ref())
                .filter_map(|c| c.first.p99_nanos)
                .map(|n| n as f64)
                .sum::<f64>()
                / filtered.len().max(1) as f64;
            let ts_p50_avg: f64 = filtered
                .iter()
                .filter_map(|b| b.comparison.as_ref())
                .filter_map(|c| c.second.p50_nanos)
                .map(|n| n as f64)
                .sum::<f64>()
                / filtered.len().max(1) as f64;
            let ts_p99_avg: f64 = filtered
                .iter()
                .filter_map(|b| b.comparison.as_ref())
                .filter_map(|c| c.second.p99_nanos)
                .map(|n| n as f64)
                .sum::<f64>()
                / filtered.len().max(1) as f64;

            if go_p50_avg > 0.0 || ts_p50_avg > 0.0 {
                let dist_str = format!(
                    "Avg p50: Go {} / TS {}  |  Avg p99: Go {} / TS {}",
                    format_duration_with_unit(go_p50_avg, time_unit, precision),
                    format_duration_with_unit(ts_p50_avg, time_unit, precision),
                    format_duration_with_unit(go_p99_avg, time_unit, precision),
                    format_duration_with_unit(ts_p99_avg, time_unit, precision)
                );
                svg.push_str(&format!(
                    "<text x=\"{}\" y=\"{}\" font-family=\"monospace\" font-size=\"9\" fill=\"{}\">• {}</text>\n",
                    stats_x, stats_y, TEXT_MUTED, escape_xml(&dist_str)
                ));
                stats_y += 14;
            }
        }

        // Ops/sec summary
        if directive.show_stats {
            let total_go_ops: f64 = filtered
                .iter()
                .filter_map(|b| b.comparison.as_ref())
                .map(|c| c.first.ops_per_sec)
                .sum();
            let total_ts_ops: f64 = filtered
                .iter()
                .filter_map(|b| b.comparison.as_ref())
                .map(|c| c.second.ops_per_sec)
                .sum();
            let avg_go_ops = total_go_ops / filtered.len().max(1) as f64;
            let avg_ts_ops = total_ts_ops / filtered.len().max(1) as f64;

            let ops_str = format!(
                "Avg ops/sec: Go {} / TS {}  |  Total benchmarks: {}",
                format_ops_per_sec(avg_go_ops),
                format_ops_per_sec(avg_ts_ops),
                filtered.len()
            );
            svg.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" font-family=\"monospace\" font-size=\"9\" fill=\"{}\">• {}</text>\n",
                stats_x, stats_y, TEXT_MUTED, escape_xml(&ops_str)
            ));
        }
    }

    // Config footer
    if directive.show_config && suite_config.is_some() && !compact {
        let config = suite_config.unwrap();
        let mut config_parts = Vec::new();

        if let Some(iter) = config.iterations {
            config_parts.push(format!("{} iterations", iter));
        }
        if let Some(warm) = config.warmup {
            config_parts.push(format!("{} warmup", warm));
        }
        if let Some(to) = config.timeout_ms {
            let timeout_str = if to >= 1000 {
                format!("{}s timeout", to / 1000)
            } else {
                format!("{}ms timeout", to)
            };
            config_parts.push(timeout_str);
        }
        if let Some(ref ord) = config.order {
            config_parts.push(ord.clone());
        }

        if !config_parts.is_empty() {
            let config_str = config_parts.join("  |  ");
            let config_y = height - 12;
            svg.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"monospace\" font-size=\"10\" fill=\"{}\">{}</text>\n",
                width / 2, config_y, TEXT_MUTED, escape_xml(&config_str)
            ));
        }
    }

    svg.push_str("</svg>");

    Ok(svg)
}

/// Generate a bar chart using the legacy signature (backwards compatibility)
pub fn generate_simple(
    results: &BenchmarkResults,
    title: &str,
    description: Option<&str>,
    _x_label: &str,
    _y_label: &str,
) -> Result<String> {
    use poly_bench_dsl::ChartType;

    let mut directive = ChartDirectiveIR::new(ChartType::BarChart, "bar-chart.svg".to_string());
    directive.title = Some(title.to_string());
    directive.description = description.map(|s| s.to_string());

    generate(results, &directive, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use poly_bench_dsl::ChartType;
    use poly_bench_executor::comparison::{BenchmarkResult, SuiteResults};
    use std::collections::HashMap;

    fn default_directive() -> ChartDirectiveIR {
        ChartDirectiveIR::new(ChartType::BarChart, "test.svg".to_string())
    }

    #[test]
    fn test_generate_empty() {
        let results = BenchmarkResults::new(vec![]);
        let svg = generate(&results, &default_directive(), None).unwrap();
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn test_generate_no_comparisons() {
        let benchmarks = vec![BenchmarkResult::new(
            "bench1".to_string(),
            "suite_bench1".to_string(),
            None,
            HashMap::new(),
        )];
        let suite = SuiteResults::new("suite".to_string(), None, benchmarks);
        let results = BenchmarkResults::new(vec![suite]);

        let svg = generate(&results, &default_directive(), None).unwrap();
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn test_generate_simple_backwards_compat() {
        let results = BenchmarkResults::new(vec![]);
        let svg = generate_simple(&results, "Test", None, "X", "Y").unwrap();
        assert!(svg.contains("<svg"));
    }
}
