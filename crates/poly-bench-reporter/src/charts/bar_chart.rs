//! Bar chart generator - individual vertical bar chart per benchmark with Go vs TS comparison

use poly_bench_dsl::Lang;
use poly_bench_executor::BenchmarkResults;
use poly_bench_ir::ChartDirectiveIR;
use poly_bench_runtime::measurement::ComparisonWinner;
use miette::Result;

use super::{
    escape_xml, svg_header, svg_title,
    filter_benchmarks, sort_benchmarks, count_wins, calculate_geo_mean,
    format_duration_with_unit, format_ops_per_sec,
    DEFAULT_MARGIN_TOP, DEFAULT_MARGIN_BOTTOM,
    GO_COLOR, TS_COLOR, TEXT_COLOR, TEXT_SECONDARY, TEXT_MUTED, BORDER_COLOR,
};

// Layout constants for individual bar chart groups
const BAR_WIDTH: i32 = 40;
const BAR_GAP: i32 = 8;           // Gap between Go and TS bars within a benchmark
const CHART_HEIGHT: i32 = 180;    // Height of each mini chart area
const CHART_SPACING_X: i32 = 160; // Horizontal spacing between chart centers
const CHARTS_PER_ROW: i32 = 4;    // Max charts per row before wrapping
const CHART_SPACING_Y: i32 = 220; // Vertical spacing between chart rows
const STATS_BOX_HEIGHT: i32 = 80;
const LEGEND_HEIGHT: i32 = 40;
const MIN_STATS_WIDTH: i32 = 520; // Minimum width to fit stats text

/// Suite configuration for display
pub struct SuiteConfig {
    pub iterations: Option<u64>,
    pub warmup: Option<u64>,
    pub timeout_ms: Option<u64>,
    pub order: Option<String>,
}

/// Generate a bar chart SVG with individual charts per benchmark
pub fn generate(
    results: &BenchmarkResults,
    directive: &ChartDirectiveIR,
    suite_config: Option<&SuiteConfig>,
) -> Result<String> {
    let mut svg = String::new();

    // Collect all benchmarks with comparisons
    let all_benchmarks: Vec<_> = results.suites.iter()
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
    let bar_width = directive.bar_height.unwrap_or(BAR_WIDTH);
    let bar_gap = directive.bar_gap.unwrap_or(BAR_GAP);
    let time_unit = directive.time_unit.as_deref();
    let precision = directive.precision;
    let compact = directive.compact;
    
    // Calculate dynamic dimensions based on number of benchmarks
    let num_benchmarks = filtered.len() as i32;
    let charts_per_row = directive.width.map(|w| (w / CHART_SPACING_X).max(1)).unwrap_or(CHARTS_PER_ROW);
    let num_rows = (num_benchmarks + charts_per_row - 1) / charts_per_row;
    
    // Calculate width - ensure it's wide enough for stats
    let actual_charts_in_first_row = num_benchmarks.min(charts_per_row);
    let margin_x = 60;
    let content_width = actual_charts_in_first_row * CHART_SPACING_X;
    
    // Stats box needs minimum width to avoid overflow
    let stats_box_needed = (directive.show_stats || directive.show_distribution || directive.show_geo_mean) && !compact;
    let min_width = if stats_box_needed { MIN_STATS_WIDTH } else { 300 };
    let width = directive.width.unwrap_or(content_width + margin_x * 2).max(min_width);
    
    // Calculate summary stats for legend
    let (go_wins, ts_wins, ties) = count_wins(&filtered);
    let geo_mean = calculate_geo_mean(&filtered);
    
    // Height calculation
    let legend_space = LEGEND_HEIGHT;
    let stats_box_space = if stats_box_needed { STATS_BOX_HEIGHT } else { 0 };
    let config_space = if directive.show_config && suite_config.is_some() && !compact { 24 } else { 0 };
    let chart_area_height = num_rows * CHART_SPACING_Y;
    let height = DEFAULT_MARGIN_TOP + chart_area_height + legend_space + stats_box_space + config_space + DEFAULT_MARGIN_BOTTOM;

    // Find max total time for scale (using total_nanos from measurements)
    let max_total_nanos: f64 = filtered.iter()
        .filter_map(|b| b.comparison.as_ref())
        .flat_map(|c| [c.first.total_nanos as f64, c.second.total_nanos as f64])
        .fold(1.0, f64::max);

    // Build subtitle
    let subtitle = directive.description.clone().unwrap_or_else(|| {
        match results.summary.winner {
            Some(Lang::Go) => format!("Go is {:.1}x faster overall", results.summary.geo_mean_speedup),
            Some(Lang::TypeScript) => format!("TypeScript is {:.1}x faster overall", 1.0 / results.summary.geo_mean_speedup),
            _ => "Similar performance".to_string(),
        }
    });

    let title = directive.title.clone().unwrap_or_else(|| "Benchmark Results".to_string());

    // SVG header
    svg.push_str(&svg_header(width, height));

    // Title
    svg.push_str(&svg_title(width, &title, Some(&subtitle)));

    // Draw individual bar chart for each benchmark
    let start_y = DEFAULT_MARGIN_TOP + 20;
    
    for (i, bench) in filtered.iter().enumerate() {
        if let Some(ref cmp) = bench.comparison {
            let row = i as i32 / charts_per_row;
            let col = i as i32 % charts_per_row;
            
            // Calculate center position for this chart
            // For single benchmark, center in the available width
            let center_x = if num_benchmarks == 1 {
                width / 2
            } else {
                margin_x + (col * CHART_SPACING_X) + CHART_SPACING_X / 2
            };
            let chart_bottom_y = start_y + (row * CHART_SPACING_Y) + CHART_HEIGHT;
            
            // Calculate bar heights based on total time (linear scale)
            let go_total_nanos = cmp.first.total_nanos as f64;
            let ts_total_nanos = cmp.second.total_nanos as f64;
            
            let go_bar_height = (go_total_nanos / max_total_nanos * CHART_HEIGHT as f64).max(3.0);
            let ts_bar_height = (ts_total_nanos / max_total_nanos * CHART_HEIGHT as f64).max(3.0);

            let go_winner = cmp.winner == ComparisonWinner::First;
            let ts_winner = cmp.winner == ComparisonWinner::Second;
            
            // Width of the bar group
            let group_width = bar_width * 2 + bar_gap;
            let group_start_x = center_x - group_width / 2;
            
            // Draw chart group
            svg.push_str(&format!("<g transform=\"translate({},{})\">\n", group_start_x, chart_bottom_y));
            
            // Baseline
            svg.push_str(&format!(
                "  <line x1=\"-10\" y1=\"0\" x2=\"{}\" y2=\"0\" stroke=\"{}\" stroke-width=\"1\"/>\n",
                group_width + 10, BORDER_COLOR
            ));
            
            // Go bar
            svg.push_str(&format!(
                "  <rect x=\"0\" y=\"{:.1}\" width=\"{}\" height=\"{:.1}\" fill=\"url(#goGrad)\" rx=\"3\"/>\n",
                -go_bar_height, bar_width, go_bar_height
            ));
            
            // Go value label (above bar)
            let go_total_str = format_duration_with_unit(go_total_nanos, time_unit, precision);
            if go_bar_height > 15.0 || num_benchmarks == 1 {
                svg.push_str(&format!(
                    "  <text x=\"{}\" y=\"{:.1}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"9\" font-weight=\"{}\" fill=\"{}\">{}</text>\n",
                    bar_width / 2, -go_bar_height - 4.0,
                    if go_winner { "600" } else { "400" },
                    if go_winner { "#0E7490" } else { TEXT_MUTED },
                    go_total_str
                ));
            }
            
            // Go label below bar
            svg.push_str(&format!(
                "  <text x=\"{}\" y=\"16\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"9\" font-weight=\"500\" fill=\"{}\">Go</text>\n",
                bar_width / 2, GO_COLOR
            ));

            // TypeScript bar
            let ts_x = bar_width + bar_gap;
            svg.push_str(&format!(
                "  <rect x=\"{}\" y=\"{:.1}\" width=\"{}\" height=\"{:.1}\" fill=\"url(#tsGrad)\" rx=\"3\"/>\n",
                ts_x, -ts_bar_height, bar_width, ts_bar_height
            ));
            
            // TS value label (above bar)
            let ts_total_str = format_duration_with_unit(ts_total_nanos, time_unit, precision);
            if ts_bar_height > 15.0 || num_benchmarks == 1 {
                svg.push_str(&format!(
                    "  <text x=\"{}\" y=\"{:.1}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"9\" font-weight=\"{}\" fill=\"{}\">{}</text>\n",
                    ts_x + bar_width / 2, -ts_bar_height - 4.0,
                    if ts_winner { "600" } else { "400" },
                    if ts_winner { "#1E40AF" } else { TEXT_MUTED },
                    ts_total_str
                ));
            }
            
            // TS label below bar
            svg.push_str(&format!(
                "  <text x=\"{}\" y=\"16\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"9\" font-weight=\"500\" fill=\"{}\">TS</text>\n",
                ts_x + bar_width / 2, TS_COLOR
            ));

            // Benchmark name below (horizontal, centered)
            svg.push_str(&format!(
                "  <text x=\"{}\" y=\"34\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"10\" font-weight=\"500\" fill=\"{}\">{}</text>\n",
                group_width / 2, TEXT_COLOR, escape_xml(&bench.name)
            ));

            // Winner indicator (speedup text below benchmark name)
            if !compact && (go_winner || ts_winner) {
                let winner_color = if go_winner { GO_COLOR } else { TS_COLOR };
                let speedup = if go_winner {
                    ts_total_nanos / go_total_nanos
                } else {
                    go_total_nanos / ts_total_nanos
                };
                svg.push_str(&format!(
                    "  <text x=\"{}\" y=\"48\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"8\" font-weight=\"600\" fill=\"{}\">{:.1}x faster</text>\n",
                    group_width / 2, winner_color, speedup
                ));
            }
            
            svg.push_str("</g>\n");
        }
    }

    // Legend
    let legend_y = start_y + (num_rows * CHART_SPACING_Y) + 20;
    svg.push_str(&format!("<g transform=\"translate({},{})\">\n", width / 2, legend_y));
    
    // Go indicator
    let go_label = if directive.show_win_counts {
        format!("Go ({} wins)", go_wins)
    } else {
        "Go".to_string()
    };
    svg.push_str(&format!(
        "  <rect x=\"-140\" width=\"14\" height=\"14\" fill=\"{}\" rx=\"3\"/>\
         <text x=\"-122\" y=\"11\" font-family=\"sans-serif\" font-size=\"11\" fill=\"{}\">{}</text>\n",
        GO_COLOR, TEXT_COLOR, escape_xml(&go_label)
    ));
    
    // TS indicator
    let ts_label = if directive.show_win_counts {
        format!("TypeScript ({} wins)", ts_wins)
    } else {
        "TypeScript".to_string()
    };
    svg.push_str(&format!(
        "  <rect x=\"30\" width=\"14\" height=\"14\" fill=\"{}\" rx=\"3\"/>\
         <text x=\"48\" y=\"11\" font-family=\"sans-serif\" font-size=\"11\" fill=\"{}\">{}</text>\n",
        TS_COLOR, TEXT_COLOR, escape_xml(&ts_label)
    ));
    
    // Ties indicator
    if ties > 0 && directive.show_win_counts {
        svg.push_str(&format!(
            "  <text x=\"180\" y=\"11\" font-family=\"sans-serif\" font-size=\"11\" fill=\"{}\">Ties: {}</text>\n",
            TEXT_MUTED, ties
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
                format!("TypeScript is {:.2}x faster on average (geometric mean)", 1.0 / geo_mean)
            };
            svg.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"10\" fill=\"{}\">• {}</text>\n",
                stats_x, stats_y, TEXT_SECONDARY, escape_xml(&geo_label)
            ));
            stats_y += 14;
        }
        
        // Distribution stats (aggregate)
        if directive.show_distribution {
            let go_p50_avg: f64 = filtered.iter()
                .filter_map(|b| b.comparison.as_ref())
                .filter_map(|c| c.first.p50_nanos)
                .map(|n| n as f64)
                .sum::<f64>() / filtered.len().max(1) as f64;
            let go_p99_avg: f64 = filtered.iter()
                .filter_map(|b| b.comparison.as_ref())
                .filter_map(|c| c.first.p99_nanos)
                .map(|n| n as f64)
                .sum::<f64>() / filtered.len().max(1) as f64;
            let ts_p50_avg: f64 = filtered.iter()
                .filter_map(|b| b.comparison.as_ref())
                .filter_map(|c| c.second.p50_nanos)
                .map(|n| n as f64)
                .sum::<f64>() / filtered.len().max(1) as f64;
            let ts_p99_avg: f64 = filtered.iter()
                .filter_map(|b| b.comparison.as_ref())
                .filter_map(|c| c.second.p99_nanos)
                .map(|n| n as f64)
                .sum::<f64>() / filtered.len().max(1) as f64;
            
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
            let total_go_ops: f64 = filtered.iter()
                .filter_map(|b| b.comparison.as_ref())
                .map(|c| c.first.ops_per_sec)
                .sum();
            let total_ts_ops: f64 = filtered.iter()
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
    use poly_bench_executor::comparison::{SuiteResults, BenchmarkResult};
    use poly_bench_dsl::ChartType;
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
        let benchmarks = vec![
            BenchmarkResult::new(
                "bench1".to_string(),
                "suite_bench1".to_string(),
                None,
                HashMap::new(),
            ),
        ];
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
