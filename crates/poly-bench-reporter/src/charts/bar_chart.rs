//! Bar chart generator for benchmark results

use poly_bench_dsl::Lang;
use poly_bench_executor::BenchmarkResults;
use poly_bench_ir::ChartDirectiveIR;
use poly_bench_runtime::measurement::ComparisonWinner;
use miette::Result;

use super::{
    escape_xml, svg_header, svg_title,
    filter_benchmarks, sort_benchmarks, count_wins, calculate_geo_mean,
    format_duration_with_unit, format_ops_per_sec,
    DEFAULT_WIDTH, DEFAULT_MARGIN_TOP, DEFAULT_MARGIN_BOTTOM,
    DEFAULT_MARGIN_LEFT, DEFAULT_MARGIN_RIGHT,
    GO_COLOR, TS_COLOR, TEXT_COLOR, TEXT_SECONDARY, TEXT_MUTED, BORDER_COLOR,
};

const BAR_HEIGHT: i32 = 26;
const BAR_GAP: i32 = 5;
const STATS_PANEL_HEIGHT: i32 = 14;
const LEGEND_HEIGHT: i32 = 30;
const CONFIG_HEIGHT: i32 = 24;
const STATS_BOX_HEIGHT: i32 = 80;
const STATS_BOX_PADDING: i32 = 12;

/// Suite configuration for display
pub struct SuiteConfig {
    pub iterations: Option<u64>,
    pub warmup: Option<u64>,
    pub timeout_ms: Option<u64>,
    pub order: Option<String>,
}

/// Generate a bar chart SVG from benchmark results
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
    let width = directive.width.unwrap_or(DEFAULT_WIDTH);
    let bar_height = directive.bar_height.unwrap_or(BAR_HEIGHT);
    let bar_gap = directive.bar_gap.unwrap_or(BAR_GAP);
    let margin_left = directive.margin_left.unwrap_or(DEFAULT_MARGIN_LEFT);
    let time_unit = directive.time_unit.as_deref();
    let precision = directive.precision;
    let compact = directive.compact;
    
    // Calculate dimensions
    let row_height = if directive.show_stats && !compact {
        bar_height + STATS_PANEL_HEIGHT + bar_gap
    } else {
        bar_height + bar_gap
    };
    let content_height = (filtered.len() as i32) * row_height;
    
    // Additional height for legend, stats box, and config
    let legend_space = if directive.show_win_counts || directive.show_geo_mean { LEGEND_HEIGHT } else { 0 };
    let config_space = if directive.show_config && suite_config.is_some() && !compact { CONFIG_HEIGHT } else { 0 };
    let stats_box_space = if (directive.show_stats || directive.show_distribution || directive.show_geo_mean) && !compact { STATS_BOX_HEIGHT } else { 0 };
    
    let height = content_height + DEFAULT_MARGIN_TOP + DEFAULT_MARGIN_BOTTOM + legend_space + stats_box_space + config_space;
    let chart_width = width - margin_left - DEFAULT_MARGIN_RIGHT;

    // Find max speedup for scale
    let max_speedup: f64 = filtered.iter()
        .filter_map(|b| b.comparison.as_ref())
        .map(|c| c.speedup)
        .fold(1.0, f64::max);

    let log_ceil = (max_speedup * 1.2).log10().ceil() as i32;
    let log_ceil = log_ceil.max(1);

    // Calculate summary stats for legend
    let (go_wins, ts_wins, ties) = count_wins(&filtered);
    let geo_mean = calculate_geo_mean(&filtered);

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

    // Chart group
    svg.push_str(&format!("<g transform=\"translate({},{})\">\n", margin_left, DEFAULT_MARGIN_TOP));

    // Grid lines
    for exp in 0..=log_ceil {
        let val = 10.0_f64.powi(exp);
        let x = (exp as f64 / log_ceil as f64) * chart_width as f64;
        
        svg.push_str(&format!(
            "  <line x1=\"{:.1}\" y1=\"0\" x2=\"{:.1}\" y2=\"{}\" stroke=\"#E5E7EB\"/>\n",
            x, x, content_height
        ));
        svg.push_str(&format!(
            "  <text x=\"{:.1}\" y=\"{}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"10\" fill=\"{}\">{}x</text>\n",
            x, content_height + 16, TEXT_MUTED, val as i32
        ));
    }

    // Bars and stats
    let mut y: i32 = 0;
    for bench in &filtered {
        if let Some(ref cmp) = bench.comparison {
            let bar_width = if cmp.speedup <= 1.0 {
                3.0
            } else {
                (cmp.speedup.log10() / log_ceil as f64) * chart_width as f64
            };
            let bar_width = bar_width.max(3.0);

            let fill = match cmp.winner {
                ComparisonWinner::First => "url(#goGrad)",
                ComparisonWinner::Second => "url(#tsGrad)",
                ComparisonWinner::Tie => "#9CA3AF",
            };

            // Benchmark name label
            svg.push_str(&format!(
                "  <text x=\"-10\" y=\"{}\" text-anchor=\"end\" font-family=\"sans-serif\" font-size=\"11\" fill=\"{}\">{}</text>\n",
                y + bar_height / 2 + 4, TEXT_COLOR, escape_xml(&bench.name)
            ));

            // Bar
            svg.push_str(&format!(
                "  <rect x=\"0\" y=\"{}\" width=\"{:.1}\" height=\"{}\" fill=\"{}\" rx=\"4\"/>\n",
                y, bar_width, bar_height, fill
            ));

            // Speedup value
            let val_str = if cmp.speedup >= 10.0 {
                format!("{:.0}x", cmp.speedup)
            } else {
                format!("{:.1}x", cmp.speedup)
            };
            
            let val_color = match cmp.winner {
                ComparisonWinner::First => "#0E7490",
                ComparisonWinner::Second => "#1E40AF",
                ComparisonWinner::Tie => "#6B7280",
            };

            svg.push_str(&format!(
                "  <text x=\"{:.1}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"11\" font-weight=\"600\" fill=\"{}\">{}</text>\n",
                bar_width + 8.0, y + bar_height / 2 + 4, val_color, val_str
            ));

            // Stats panel (below bar) - show timing and ops/sec info
            if directive.show_stats && !compact {
                let go_time = format_duration_with_unit(cmp.first.nanos_per_op, time_unit, precision);
                let ts_time = format_duration_with_unit(cmp.second.nanos_per_op, time_unit, precision);
                let go_ops = format_ops_per_sec(cmp.first.ops_per_sec);
                let ts_ops = format_ops_per_sec(cmp.second.ops_per_sec);
                
                let stats_str = format!(
                    "Go: {} ({} ops/s)  |  TS: {} ({} ops/s)",
                    go_time, go_ops, ts_time, ts_ops
                );
                
                svg.push_str(&format!(
                    "  <text x=\"0\" y=\"{}\" font-family=\"monospace\" font-size=\"9\" fill=\"{}\">{}</text>\n",
                    y + bar_height + STATS_PANEL_HEIGHT, TEXT_SECONDARY, escape_xml(&stats_str)
                ));
                
                // Show distribution (p50/p99) if enabled
                if directive.show_distribution {
                    let p50_go = cmp.first.p50_nanos.map(|n| format_duration_with_unit(n as f64, time_unit, precision)).unwrap_or("-".to_string());
                    let p99_go = cmp.first.p99_nanos.map(|n| format_duration_with_unit(n as f64, time_unit, precision)).unwrap_or("-".to_string());
                    let p50_ts = cmp.second.p50_nanos.map(|n| format_duration_with_unit(n as f64, time_unit, precision)).unwrap_or("-".to_string());
                    let p99_ts = cmp.second.p99_nanos.map(|n| format_duration_with_unit(n as f64, time_unit, precision)).unwrap_or("-".to_string());
                    
                    // Add distribution info on new line
                    y += STATS_PANEL_HEIGHT;
                    let dist_str = format!(
                        "p50: {} / {}  |  p99: {} / {}",
                        p50_go, p50_ts, p99_go, p99_ts
                    );
                    svg.push_str(&format!(
                        "  <text x=\"0\" y=\"{}\" font-family=\"monospace\" font-size=\"9\" fill=\"{}\">{}</text>\n",
                        y + bar_height + STATS_PANEL_HEIGHT, TEXT_MUTED, escape_xml(&dist_str)
                    ));
                }
            }

            y += row_height;
        }
    }

    svg.push_str("</g>\n");

    // Enhanced Legend with win counts
    let legend_y = DEFAULT_MARGIN_TOP + content_height + 30;
    svg.push_str(&format!("<g transform=\"translate({},{})\">\n", width / 2, legend_y));
    
    // Go wins indicator
    let go_label = if directive.show_win_counts {
        format!("Go faster ({} wins)", go_wins)
    } else {
        "Go faster".to_string()
    };
    svg.push_str(&format!(
        "  <rect x=\"-180\" width=\"14\" height=\"14\" fill=\"{}\" rx=\"3\"/>\
         <text x=\"-162\" y=\"11\" font-family=\"sans-serif\" font-size=\"11\" fill=\"{}\">{}</text>\n",
        GO_COLOR, TEXT_COLOR, escape_xml(&go_label)
    ));
    
    // TS wins indicator
    let ts_label = if directive.show_win_counts {
        format!("TS faster ({} wins)", ts_wins)
    } else {
        "TS faster".to_string()
    };
    svg.push_str(&format!(
        "  <rect x=\"10\" width=\"14\" height=\"14\" fill=\"{}\" rx=\"3\"/>\
         <text x=\"28\" y=\"11\" font-family=\"sans-serif\" font-size=\"11\" fill=\"{}\">{}</text>\n",
        TS_COLOR, TEXT_COLOR, escape_xml(&ts_label)
    ));
    
    // Ties indicator
    if ties > 0 && directive.show_win_counts {
        svg.push_str(&format!(
            "  <text x=\"200\" y=\"11\" font-family=\"sans-serif\" font-size=\"11\" fill=\"{}\">Ties: {}</text>\n",
            TEXT_MUTED, ties
        ));
    }
    
    svg.push_str("</g>\n");

    // Stats box below legend
    if (directive.show_stats || directive.show_distribution || directive.show_geo_mean) && !compact {
        let box_y = legend_y + LEGEND_HEIGHT;
        let box_width = width - 80;
        let box_x = 40;
        
        // Box background with border
        svg.push_str(&format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#F9FAFB\" stroke=\"{}\" rx=\"6\"/>\n",
            box_x, box_y, box_width, STATS_BOX_HEIGHT - 10, BORDER_COLOR
        ));
        
        // Stats content
        let stats_x = box_x + STATS_BOX_PADDING;
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
            // Calculate aggregate p50/p99 across all benchmarks
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
