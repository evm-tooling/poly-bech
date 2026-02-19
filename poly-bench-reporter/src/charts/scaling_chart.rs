//! Scaling efficiency chart generator - shows how performance scales with input size

use poly_bench_dsl::Lang;
use poly_bench_executor::comparison::BenchmarkResult;
use poly_bench_ir::ChartDirectiveIR;

use super::{
    extract_numeric_value, filter_benchmarks, sort_benchmarks, svg_header, svg_legend, svg_title,
    DEFAULT_MARGIN_BOTTOM, DEFAULT_MARGIN_LEFT, DEFAULT_MARGIN_RIGHT, DEFAULT_MARGIN_TOP, GO_COLOR,
    RUST_COLOR, TEXT_MUTED, TEXT_SECONDARY, TS_COLOR,
};

const DEFAULT_CHART_HEIGHT: i32 = 400;
const DEFAULT_CHART_WIDTH: i32 = 700;

/// Generate a scaling efficiency chart with ideal line overlay
pub fn generate(benchmarks: Vec<&BenchmarkResult>, directive: &ChartDirectiveIR) -> String {
    let mut filtered = filter_benchmarks(benchmarks, directive);
    sort_benchmarks(&mut filtered, directive);

    if filtered.is_empty() {
        return String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"400\" height=\"100\"><text x=\"200\" y=\"50\" text-anchor=\"middle\" font-family=\"sans-serif\">No benchmark data available</text></svg>");
    }

    // Extract numeric values from benchmark names for x-axis
    let points_with_n: Vec<(i64, &BenchmarkResult)> =
        filtered.iter().filter_map(|b| extract_numeric_value(&b.name).map(|n| (n, *b))).collect();

    if points_with_n.is_empty() {
        return String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"400\" height=\"100\"><text x=\"200\" y=\"50\" text-anchor=\"middle\" font-family=\"sans-serif\">No numeric values found in benchmark names</text></svg>");
    }

    // Determine which languages have data
    let has_go = filtered.iter().any(|b| b.measurements.contains_key(&Lang::Go));
    let has_ts = filtered.iter().any(|b| b.measurements.contains_key(&Lang::TypeScript));
    let has_rust = filtered.iter().any(|b| b.measurements.contains_key(&Lang::Rust));

    // Calculate dimensions
    let chart_width = directive.width.unwrap_or(DEFAULT_CHART_WIDTH);
    let chart_height = directive.height.unwrap_or(DEFAULT_CHART_HEIGHT);
    let plot_width = chart_width - DEFAULT_MARGIN_LEFT - DEFAULT_MARGIN_RIGHT;
    let plot_height = chart_height - DEFAULT_MARGIN_TOP - DEFAULT_MARGIN_BOTTOM - 30;
    let plot_x = DEFAULT_MARGIN_LEFT;
    let plot_y = DEFAULT_MARGIN_TOP;

    // Collect data points by language
    let mut go_points: Vec<(i64, f64)> = Vec::new();
    let mut ts_points: Vec<(i64, f64)> = Vec::new();
    let mut rust_points: Vec<(i64, f64)> = Vec::new();

    for (n, bench) in &points_with_n {
        if let Some(m) = bench.measurements.get(&Lang::Go) {
            go_points.push((*n, m.nanos_per_op));
        }
        if let Some(m) = bench.measurements.get(&Lang::TypeScript) {
            ts_points.push((*n, m.nanos_per_op));
        }
        if let Some(m) = bench.measurements.get(&Lang::Rust) {
            rust_points.push((*n, m.nanos_per_op));
        }
    }

    // Calculate scaling efficiency relative to smallest input
    // Efficiency = (time_n / time_1) / (n / 1) for O(n) expected scaling
    // Values > 1 mean worse than linear, < 1 means better than linear
    fn calculate_efficiency(points: &[(i64, f64)]) -> Vec<(i64, f64)> {
        if points.is_empty() {
            return Vec::new();
        }
        let (base_n, base_time) = points[0];
        points
            .iter()
            .map(|(n, time)| {
                let n_ratio = *n as f64 / base_n as f64;
                let time_ratio = time / base_time;
                let efficiency = if n_ratio > 0.0 { time_ratio / n_ratio } else { 1.0 };
                (*n, efficiency)
            })
            .collect()
    }

    let go_efficiency = calculate_efficiency(&go_points);
    let ts_efficiency = calculate_efficiency(&ts_points);
    let rust_efficiency = calculate_efficiency(&rust_points);

    // Find min/max for scales
    let all_n: Vec<i64> = points_with_n.iter().map(|(n, _)| *n).collect();
    let min_n = *all_n.iter().min().unwrap_or(&1) as f64;
    let max_n = *all_n.iter().max().unwrap_or(&100) as f64;

    let all_efficiency: Vec<f64> = go_efficiency
        .iter()
        .chain(ts_efficiency.iter())
        .chain(rust_efficiency.iter())
        .map(|(_, e)| *e)
        .collect();
    let max_efficiency = all_efficiency.iter().cloned().fold(2.0_f64, f64::max).max(2.0);

    // Use log scale for x-axis if range is large
    let use_log_x = max_n / min_n > 100.0;

    let x_scale = |n: f64| -> i32 {
        if use_log_x {
            let log_min = min_n.ln();
            let log_max = max_n.ln();
            let log_n = n.ln();
            plot_x + ((log_n - log_min) / (log_max - log_min) * plot_width as f64) as i32
        } else {
            plot_x + ((n - min_n) / (max_n - min_n) * plot_width as f64) as i32
        }
    };

    let y_scale = |e: f64| -> i32 {
        plot_y + plot_height - ((e / max_efficiency) * plot_height as f64) as i32
    };

    let mut svg = svg_header(chart_width, chart_height);

    // Title
    let title = directive.title.as_deref().unwrap_or("Scaling Efficiency");
    svg.push_str(&svg_title(
        chart_width,
        title,
        Some("Efficiency = 1.0 means perfect O(n) scaling"),
    ));

    // Draw grid
    let show_grid = directive.show_grid.unwrap_or(true);
    if show_grid {
        // Horizontal grid lines
        for i in 0..=4 {
            let e = (i as f64 / 4.0) * max_efficiency;
            let y = y_scale(e);
            svg.push_str(&format!(
                "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-opacity=\"0.2\"/>\n",
                plot_x, y, plot_x + plot_width, y, TEXT_MUTED
            ));
            svg.push_str(&format!(
                "  <text x=\"{}\" y=\"{}\" text-anchor=\"end\" font-family=\"sans-serif\" font-size=\"10\" fill=\"{}\">{:.1}</text>\n",
                plot_x - 5, y + 4, TEXT_SECONDARY, e
            ));
        }
    }

    // Draw ideal efficiency line at y=1.0
    let y_ideal = y_scale(1.0);
    svg.push_str(&format!(
        "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#10B981\" stroke-width=\"2\" stroke-dasharray=\"6,4\"/>\n",
        plot_x, y_ideal, plot_x + plot_width, y_ideal
    ));
    svg.push_str(&format!(
        "  <text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"10\" fill=\"#10B981\">Ideal (O(n))</text>\n",
        plot_x + plot_width + 5, y_ideal + 4
    ));

    // Draw x-axis labels
    let x_ticks: Vec<i64> = if use_log_x {
        // Generate log-spaced ticks
        let mut ticks = Vec::new();
        let mut tick = min_n as i64;
        while tick <= max_n as i64 {
            ticks.push(tick);
            tick *= 10;
        }
        ticks
    } else {
        // Linear ticks
        (0..=4).map(|i| (min_n + (i as f64 / 4.0) * (max_n - min_n)) as i64).collect()
    };

    for n in x_ticks {
        let x = x_scale(n as f64);
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"10\" fill=\"{}\">{}</text>\n",
            x, plot_y + plot_height + 15, TEXT_SECONDARY, n
        ));
    }

    // X-axis label
    svg.push_str(&format!(
        "  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"11\" fill=\"{}\">Input Size (n)</text>\n",
        plot_x + plot_width / 2, chart_height - 10, TEXT_SECONDARY
    ));

    // Y-axis label
    svg.push_str(&format!(
        "  <text x=\"15\" y=\"{}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"11\" fill=\"{}\" transform=\"rotate(-90, 15, {})\">Scaling Efficiency</text>\n",
        plot_y + plot_height / 2, TEXT_SECONDARY, plot_y + plot_height / 2
    ));

    // Draw data lines
    fn draw_line(
        svg: &mut String,
        points: &[(i64, f64)],
        color: &str,
        x_scale: impl Fn(f64) -> i32,
        y_scale: impl Fn(f64) -> i32,
    ) {
        if points.len() < 2 {
            return;
        }

        let mut path = String::new();
        for (i, (n, e)) in points.iter().enumerate() {
            let x = x_scale(*n as f64);
            let y = y_scale(*e);
            if i == 0 {
                path.push_str(&format!("M{},{}", x, y));
            } else {
                path.push_str(&format!(" L{},{}", x, y));
            }
        }

        svg.push_str(&format!(
            "  <path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"2\"/>\n",
            path, color
        ));

        // Draw points
        for (n, e) in points {
            let x = x_scale(*n as f64);
            let y = y_scale(*e);
            svg.push_str(&format!(
                "  <circle cx=\"{}\" cy=\"{}\" r=\"4\" fill=\"{}\" stroke=\"white\" stroke-width=\"1\"/>\n",
                x, y, color
            ));
        }
    }

    if has_go && !go_efficiency.is_empty() {
        draw_line(&mut svg, &go_efficiency, GO_COLOR, &x_scale, &y_scale);
    }
    if has_ts && !ts_efficiency.is_empty() {
        draw_line(&mut svg, &ts_efficiency, TS_COLOR, &x_scale, &y_scale);
    }
    if has_rust && !rust_efficiency.is_empty() {
        draw_line(&mut svg, &rust_efficiency, RUST_COLOR, &x_scale, &y_scale);
    }

    // Legend
    let mut legend_items: Vec<(&str, &str)> = Vec::new();
    if has_go {
        legend_items.push((GO_COLOR, "Go"));
    }
    if has_ts {
        legend_items.push((TS_COLOR, "TypeScript"));
    }
    if has_rust {
        legend_items.push((RUST_COLOR, "Rust"));
    }
    legend_items.push(("#10B981", "Ideal O(n)"));
    svg.push_str(&svg_legend(chart_width, chart_height - 25, &legend_items));

    svg.push_str("</svg>\n");
    svg
}
