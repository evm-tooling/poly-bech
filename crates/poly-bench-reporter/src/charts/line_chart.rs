//! Line chart generator for benchmark trends

use miette::Result;
use poly_bench_dsl::Lang;
use poly_bench_executor::BenchmarkResults;
use poly_bench_ir::ChartDirectiveIR;

use super::{
    calculate_geo_mean, count_wins, escape_xml, filter_benchmarks, format_duration_with_unit,
    format_ops_per_sec, sort_benchmarks, svg_header, svg_legend, svg_title, BORDER_COLOR,
    DEFAULT_MARGIN_BOTTOM, DEFAULT_MARGIN_LEFT, DEFAULT_MARGIN_RIGHT, DEFAULT_MARGIN_TOP,
    DEFAULT_WIDTH, GO_COLOR, TEXT_COLOR, TEXT_MUTED, TEXT_SECONDARY, TS_COLOR,
};

/// Generate a line chart SVG showing benchmark trends
pub fn generate(results: &BenchmarkResults, directive: &ChartDirectiveIR) -> Result<String> {
    let mut svg = String::new();

    // Collect all benchmarks
    let all_benchmarks: Vec<_> = results.suites.iter().flat_map(|s| s.benchmarks.iter()).collect();

    // Apply filtering and sorting
    let mut filtered = filter_benchmarks(all_benchmarks, directive);
    sort_benchmarks(&mut filtered, directive);

    // Extract parameters
    let width = directive.width.unwrap_or(DEFAULT_WIDTH);
    let time_unit = directive.time_unit.as_deref();
    let precision = directive.precision;

    // Collect benchmark data points from filtered results
    let mut go_points: Vec<(String, f64, f64)> = Vec::new(); // (name, nanos, ops)
    let mut ts_points: Vec<(String, f64, f64)> = Vec::new();

    for bench in &filtered {
        if let Some(m) = bench.measurements.get(&Lang::Go) {
            go_points.push((bench.name.clone(), m.nanos_per_op, m.ops_per_sec));
        }
        if let Some(m) = bench.measurements.get(&Lang::TypeScript) {
            ts_points.push((bench.name.clone(), m.nanos_per_op, m.ops_per_sec));
        }
    }

    if go_points.is_empty() && ts_points.is_empty() {
        return Ok("<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>".to_string());
    }

    // Calculate space for stats box
    let stats_box_height =
        if directive.show_stats || directive.show_distribution || directive.show_geo_mean {
            80
        } else {
            0
        };

    // Dimensions
    let height = 400 + stats_box_height;
    let chart_width = width - DEFAULT_MARGIN_LEFT - DEFAULT_MARGIN_RIGHT;
    let chart_height = 400 - DEFAULT_MARGIN_TOP - DEFAULT_MARGIN_BOTTOM - 40;

    // Find max value for Y scale
    let max_value: f64 = go_points
        .iter()
        .map(|(_, v, _)| *v)
        .chain(ts_points.iter().map(|(_, v, _)| *v))
        .fold(0.0, f64::max);

    // Collect unique benchmark names for X axis (preserve order from sorted results)
    let bench_names: Vec<String> = filtered.iter().map(|b| b.name.clone()).collect();

    let num_points = bench_names.len();
    if num_points == 0 {
        return Ok("<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>".to_string());
    }

    // SVG header
    svg.push_str(&svg_header(width, height));

    // Title
    let title = directive.title.clone().unwrap_or_else(|| "Performance Trend".to_string());
    let subtitle = directive
        .description
        .clone()
        .unwrap_or_else(|| "Performance comparison across benchmarks".to_string());
    svg.push_str(&svg_title(width, &title, Some(&subtitle)));

    // Chart group
    svg.push_str(&format!(
        "<g transform=\"translate({},{})\">\n",
        DEFAULT_MARGIN_LEFT, DEFAULT_MARGIN_TOP
    ));

    // Draw Y axis grid lines
    let num_y_lines = 5;
    for i in 0..=num_y_lines {
        let y = chart_height - (i as f64 / num_y_lines as f64 * chart_height as f64) as i32;
        let value = max_value * i as f64 / num_y_lines as f64;

        svg.push_str(&format!(
            "  <line x1=\"0\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#E5E7EB\"/>\n",
            y, chart_width, y
        ));
        svg.push_str(&format!(
            "  <text x=\"-10\" y=\"{}\" text-anchor=\"end\" font-family=\"sans-serif\" font-size=\"10\" fill=\"{}\">{}</text>\n",
            y + 4, TEXT_MUTED, format_duration_with_unit(value, time_unit, precision)
        ));
    }

    // Draw X axis labels
    for (i, name) in bench_names.iter().enumerate() {
        // Center single points, otherwise distribute evenly
        let x = if num_points == 1 {
            chart_width / 2
        } else {
            (i as f64 / (num_points - 1) as f64 * chart_width as f64) as i32
        };
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"10\" fill=\"{}\">{}</text>\n",
            x, chart_height + 16, TEXT_MUTED, escape_xml(name)
        ));
    }

    // Draw axes
    svg.push_str(&format!(
        "  <line x1=\"0\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#4B5563\" stroke-width=\"1\"/>\n",
        chart_height, chart_width, chart_height
    ));
    svg.push_str(&format!(
        "  <line x1=\"0\" y1=\"0\" x2=\"0\" y2=\"{}\" stroke=\"#4B5563\" stroke-width=\"1\"/>\n",
        chart_height
    ));

    // Helper function to draw a line series
    fn draw_series(
        svg: &mut String,
        points: &[(String, f64, f64)],
        bench_names: &[String],
        chart_width: i32,
        chart_height: i32,
        max_value: f64,
        color: &str,
        show_stats: bool,
        time_unit: Option<&str>,
        precision: Option<u32>,
    ) {
        if points.is_empty() {
            return;
        }

        let num_points = bench_names.len();

        // Helper to calculate X position - centers single points
        let calc_x = |i: usize| -> i32 {
            if num_points == 1 {
                chart_width / 2
            } else {
                (i as f64 / (num_points - 1) as f64 * chart_width as f64) as i32
            }
        };

        // Build path
        let mut path_data = String::new();
        let mut first = true;

        for (i, name) in bench_names.iter().enumerate() {
            if let Some((_, value, _)) = points.iter().find(|(n, _, _)| n == name) {
                let x = calc_x(i);
                let y = chart_height - (value / max_value * chart_height as f64) as i32;

                if first {
                    path_data.push_str(&format!("M{},{}", x, y));
                    first = false;
                } else {
                    path_data.push_str(&format!(" L{},{}", x, y));
                }
            }
        }

        // Draw line
        svg.push_str(&format!(
            "  <path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"2.5\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n",
            path_data, color
        ));

        // Draw points with optional stats tooltips
        for (i, name) in bench_names.iter().enumerate() {
            if let Some((_, value, ops)) = points.iter().find(|(n, _, _)| n == name) {
                let x = calc_x(i);
                let y = chart_height - (value / max_value * chart_height as f64) as i32;

                // Add title (tooltip) with stats if enabled
                if show_stats {
                    let time_str = format_duration_with_unit(*value, time_unit, precision);
                    let ops_str = format_ops_per_sec(*ops);
                    svg.push_str(&format!(
                        "  <circle cx=\"{}\" cy=\"{}\" r=\"5\" fill=\"{}\" stroke=\"white\" stroke-width=\"2\">\n    <title>{}: {} ({} ops/s)</title>\n  </circle>\n",
                        x, y, color, escape_xml(name), time_str, ops_str
                    ));
                } else {
                    svg.push_str(&format!(
                        "  <circle cx=\"{}\" cy=\"{}\" r=\"4\" fill=\"{}\" stroke=\"white\" stroke-width=\"2\"/>\n",
                        x, y, color
                    ));
                }
            }
        }
    }

    // Draw Go line
    draw_series(
        &mut svg,
        &go_points,
        &bench_names,
        chart_width,
        chart_height,
        max_value,
        GO_COLOR,
        directive.show_stats,
        time_unit,
        precision,
    );

    // Draw TS line
    draw_series(
        &mut svg,
        &ts_points,
        &bench_names,
        chart_width,
        chart_height,
        max_value,
        TS_COLOR,
        directive.show_stats,
        time_unit,
        precision,
    );

    // Axis labels
    let x_label = directive.x_label.clone().unwrap_or_else(|| "Benchmark".to_string());
    let y_label = directive.y_label.clone().unwrap_or_else(|| "Time".to_string());

    svg.push_str(&format!(
        "  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"11\" fill=\"{}\">{}</text>\n",
        chart_width / 2, chart_height + 35, TEXT_SECONDARY, escape_xml(&x_label)
    ));
    svg.push_str(&format!(
        "  <text x=\"-40\" y=\"{}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"11\" fill=\"{}\" transform=\"rotate(-90,-40,{})\">{}</text>\n",
        chart_height / 2, TEXT_SECONDARY, chart_height / 2, escape_xml(&y_label)
    ));

    svg.push_str("</g>\n");

    // Legend
    let legend_y = DEFAULT_MARGIN_TOP + chart_height + 50;
    svg.push_str(&svg_legend(width, legend_y, &[(GO_COLOR, "Go"), (TS_COLOR, "TypeScript")]));

    // Stats box below legend
    if directive.show_stats || directive.show_distribution || directive.show_geo_mean {
        let box_y = legend_y + 30;
        let box_width = width - 80;
        let box_x = 40;
        let box_padding = 12;

        // Calculate stats
        let (go_wins, ts_wins, _rust_wins, ties) = count_wins(&filtered);
        let geo_mean = calculate_geo_mean(&filtered);

        // Box background with border
        svg.push_str(&format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#F9FAFB\" stroke=\"{}\" rx=\"6\"/>\n",
            box_x, box_y, box_width, 70, BORDER_COLOR
        ));

        // Stats content
        let stats_x = box_x + box_padding;
        let mut stats_y = box_y + 18;

        // Header row
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"10\" font-weight=\"600\" fill=\"{}\">SUMMARY STATISTICS</text>\n",
            stats_x, stats_y, TEXT_COLOR
        ));
        stats_y += 16;

        // Geo mean and win counts
        if directive.show_geo_mean {
            let geo_label = if geo_mean >= 1.0 {
                format!("Go is {:.2}x faster on average (geometric mean)  |  Go: {} wins  |  TS: {} wins  |  Ties: {}", geo_mean, go_wins, ts_wins, ties)
            } else {
                format!("TypeScript is {:.2}x faster on average (geometric mean)  |  Go: {} wins  |  TS: {} wins  |  Ties: {}", 1.0 / geo_mean, go_wins, ts_wins, ties)
            };
            svg.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"10\" fill=\"{}\">• {}</text>\n",
                stats_x, stats_y, TEXT_SECONDARY, escape_xml(&geo_label)
            ));
            stats_y += 14;
        }

        // Distribution stats
        if directive.show_distribution {
            let go_p50_avg: f64 = filtered
                .iter()
                .filter_map(|b| b.comparison.as_ref())
                .filter_map(|c| c.first.p50_nanos)
                .map(|n| n as f64)
                .sum::<f64>() /
                filtered.len().max(1) as f64;
            let go_p99_avg: f64 = filtered
                .iter()
                .filter_map(|b| b.comparison.as_ref())
                .filter_map(|c| c.first.p99_nanos)
                .map(|n| n as f64)
                .sum::<f64>() /
                filtered.len().max(1) as f64;
            let ts_p50_avg: f64 = filtered
                .iter()
                .filter_map(|b| b.comparison.as_ref())
                .filter_map(|c| c.second.p50_nanos)
                .map(|n| n as f64)
                .sum::<f64>() /
                filtered.len().max(1) as f64;
            let ts_p99_avg: f64 = filtered
                .iter()
                .filter_map(|b| b.comparison.as_ref())
                .filter_map(|c| c.second.p99_nanos)
                .map(|n| n as f64)
                .sum::<f64>() /
                filtered.len().max(1) as f64;

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
            }
        }
    }

    svg.push_str("</svg>");

    Ok(svg)
}

/// Generate a line chart using the legacy signature (backwards compatibility)
pub fn generate_simple(
    results: &BenchmarkResults,
    title: &str,
    description: Option<&str>,
    x_label: &str,
    y_label: &str,
) -> Result<String> {
    use poly_bench_dsl::ChartType;

    let mut directive = ChartDirectiveIR::new(ChartType::LineChart, "line-chart.svg".to_string());
    directive.title = Some(title.to_string());
    directive.description = description.map(|s| s.to_string());
    directive.x_label = Some(x_label.to_string());
    directive.y_label = Some(y_label.to_string());

    generate(results, &directive)
}

#[cfg(test)]
mod tests {
    use super::*;
    use poly_bench_dsl::ChartType;
    use poly_bench_executor::comparison::{BenchmarkResult, SuiteResults};
    use std::collections::HashMap;

    fn default_directive() -> ChartDirectiveIR {
        ChartDirectiveIR::new(ChartType::LineChart, "test.svg".to_string())
    }

    #[test]
    fn test_generate_empty() {
        let results = BenchmarkResults::new(vec![]);
        let svg = generate(&results, &default_directive()).unwrap();
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn test_generate_simple_backwards_compat() {
        let results = BenchmarkResults::new(vec![]);
        let svg = generate_simple(&results, "Test", None, "X", "Y").unwrap();
        assert!(svg.contains("<svg"));
    }
}
