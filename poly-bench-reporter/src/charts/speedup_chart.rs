//! Speedup chart generator - shows relative performance vs baseline language

use poly_bench_dsl::Lang;
use poly_bench_executor::comparison::BenchmarkResult;
use poly_bench_ir::ChartDirectiveIR;

use super::{
    escape_xml, filter_benchmarks, lang_color, sort_benchmarks, svg_header, svg_legend, svg_title,
    DEFAULT_MARGIN_BOTTOM, DEFAULT_MARGIN_LEFT, DEFAULT_MARGIN_RIGHT, DEFAULT_MARGIN_TOP, GO_COLOR,
    RUST_COLOR, TEXT_COLOR, TEXT_MUTED, TEXT_SECONDARY, TS_COLOR,
};

const DEFAULT_BAR_HEIGHT: i32 = 24;
const BAR_GAP: i32 = 8;

/// Generate a speedup chart showing relative performance vs baseline
pub fn generate(benchmarks: Vec<&BenchmarkResult>, directive: &ChartDirectiveIR) -> String {
    let mut filtered = filter_benchmarks(benchmarks, directive);
    sort_benchmarks(&mut filtered, directive);

    if filtered.is_empty() {
        return String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"400\" height=\"100\"><text x=\"200\" y=\"50\" text-anchor=\"middle\" font-family=\"sans-serif\">No benchmark data available</text></svg>");
    }

    // Determine baseline language (default to Go)
    let baseline_lang = directive
        .baseline_benchmark
        .as_ref()
        .and_then(|s| match s.to_lowercase().as_str() {
            "go" => Some(Lang::Go),
            "ts" | "typescript" => Some(Lang::TypeScript),
            "rust" => Some(Lang::Rust),
            _ => None,
        })
        .unwrap_or(Lang::Go);

    // Determine which languages have data (excluding baseline)
    let has_go = baseline_lang != Lang::Go &&
        filtered.iter().any(|b| b.measurements.contains_key(&Lang::Go));
    let has_ts = baseline_lang != Lang::TypeScript &&
        filtered.iter().any(|b| b.measurements.contains_key(&Lang::TypeScript));
    let has_rust = baseline_lang != Lang::Rust &&
        filtered.iter().any(|b| b.measurements.contains_key(&Lang::Rust));

    let comparison_langs: Vec<Lang> = [
        if has_go { Some(Lang::Go) } else { None },
        if has_ts { Some(Lang::TypeScript) } else { None },
        if has_rust { Some(Lang::Rust) } else { None },
    ]
    .into_iter()
    .flatten()
    .collect();

    if comparison_langs.is_empty() {
        return String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"400\" height=\"100\"><text x=\"200\" y=\"50\" text-anchor=\"middle\" font-family=\"sans-serif\">No comparison data available</text></svg>");
    }

    let num_benchmarks = filtered.len() as i32;
    let num_langs = comparison_langs.len() as i32;
    let rows_per_benchmark = num_langs;
    let total_rows = num_benchmarks * rows_per_benchmark;

    // Calculate dimensions
    let chart_height = directive.height.unwrap_or(
        DEFAULT_MARGIN_TOP +
            DEFAULT_MARGIN_BOTTOM +
            total_rows * (DEFAULT_BAR_HEIGHT + BAR_GAP) +
            (num_benchmarks - 1) * 20, // Extra gap between benchmark groups
    );
    let chart_width = directive.width.unwrap_or(700);
    let plot_width = chart_width - DEFAULT_MARGIN_LEFT - DEFAULT_MARGIN_RIGHT - 100; // Extra space for labels
    let plot_height = chart_height - DEFAULT_MARGIN_TOP - DEFAULT_MARGIN_BOTTOM;

    // Calculate speedups for all benchmarks
    let mut speedups: Vec<(String, Vec<(Lang, f64)>)> = Vec::new();
    let mut max_speedup: f64 = 2.0; // Minimum scale

    for bench in &filtered {
        let baseline_time = bench.measurements.get(&baseline_lang).map(|m| m.nanos_per_op);

        if let Some(base_ns) = baseline_time {
            let mut bench_speedups = Vec::new();
            for &lang in &comparison_langs {
                if let Some(m) = bench.measurements.get(&lang) {
                    let speedup = base_ns / m.nanos_per_op;
                    max_speedup = max_speedup.max(speedup);
                    bench_speedups.push((lang, speedup));
                }
            }
            if !bench_speedups.is_empty() {
                speedups.push((bench.name.clone(), bench_speedups));
            }
        }
    }

    // Round up max_speedup for nice scale
    max_speedup = (max_speedup * 1.1).ceil();

    let mut svg = svg_header(chart_width, chart_height);

    // Title
    let title = directive.title.as_deref().unwrap_or("Speedup vs Baseline");
    let baseline_name = match baseline_lang {
        Lang::Go => "Go",
        Lang::TypeScript => "TypeScript",
        Lang::Rust => "Rust",
        _ => "Baseline",
    };
    let subtitle = format!("Relative performance ({}x faster than {})", "N", baseline_name);
    svg.push_str(&svg_title(chart_width, title, Some(&subtitle)));

    // Draw plot area
    let plot_x = DEFAULT_MARGIN_LEFT + 80; // Extra space for benchmark names
    let plot_y = DEFAULT_MARGIN_TOP;

    // Draw vertical grid lines and x-axis
    let num_ticks = 5;
    for i in 0..=num_ticks {
        let value = (i as f64 / num_ticks as f64) * max_speedup;
        let x = plot_x + (i as f64 / num_ticks as f64 * plot_width as f64) as i32;

        // Grid line
        svg.push_str(&format!(
            "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-opacity=\"0.3\"/>\n",
            x, plot_y, x, plot_y + plot_height, TEXT_MUTED
        ));

        // Tick label
        let label = if value == 1.0 { "1x".to_string() } else { format!("{:.1}x", value) };
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"10\" fill=\"{}\">{}</text>\n",
            x, plot_y + plot_height + 15, TEXT_SECONDARY, label
        ));
    }

    // Draw 1x reference line (baseline)
    let x_1x = plot_x + (1.0 / max_speedup * plot_width as f64) as i32;
    svg.push_str(&format!(
        "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"2\" stroke-dasharray=\"4,4\"/>\n",
        x_1x, plot_y, x_1x, plot_y + plot_height, TEXT_SECONDARY
    ));

    // Draw bars
    let bar_height = DEFAULT_BAR_HEIGHT;
    let mut y = plot_y;

    for (bench_name, bench_speedups) in &speedups {
        // Benchmark name label
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" text-anchor=\"end\" font-family=\"sans-serif\" font-size=\"11\" font-weight=\"600\" fill=\"{}\">{}</text>\n",
            plot_x - 10,
            y + (bench_speedups.len() as i32 * (bar_height + BAR_GAP)) / 2,
            TEXT_COLOR,
            escape_xml(bench_name)
        ));

        for (lang, speedup) in bench_speedups {
            let color = lang_color(*lang);
            let bar_width = ((speedup / max_speedup) * plot_width as f64) as i32;

            // Determine bar color based on speedup
            let fill_color = if *speedup >= 1.0 {
                color // Faster than baseline
            } else {
                "#EF4444" // Slower than baseline (red)
            };

            // Draw bar
            svg.push_str(&format!(
                "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" rx=\"3\"/>\n",
                plot_x,
                y,
                bar_width.max(2),
                bar_height,
                fill_color
            ));

            // Language label and speedup value
            let lang_name = match lang {
                Lang::Go => "Go",
                Lang::TypeScript => "TS",
                Lang::Rust => "Rust",
                _ => "?",
            };
            let speedup_label = if *speedup >= 1.0 {
                format!("{}: {:.2}x faster", lang_name, speedup)
            } else {
                format!("{}: {:.2}x slower", lang_name, 1.0 / speedup)
            };

            svg.push_str(&format!(
                "  <text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"10\" fill=\"{}\">{}</text>\n",
                plot_x + bar_width + 5,
                y + bar_height / 2 + 4,
                TEXT_SECONDARY,
                speedup_label
            ));

            y += bar_height + BAR_GAP;
        }

        y += 12; // Extra gap between benchmark groups
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
    svg.push_str(&svg_legend(chart_width, chart_height - 25, &legend_items));

    svg.push_str("</svg>\n");
    svg
}
