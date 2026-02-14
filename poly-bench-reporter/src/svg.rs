//! SVG chart generator

use miette::Result;
use poly_bench_dsl::Lang;
use poly_bench_executor::BenchmarkResults;

/// Generate SVG speedup chart
pub fn generate_speedup_chart(results: &BenchmarkResults) -> Result<String> {
    let mut svg = String::new();

    // Collect all benchmarks with comparisons
    let benchmarks: Vec<_> = results
        .suites
        .iter()
        .flat_map(|s| s.benchmarks.iter())
        .filter(|b| b.comparison.is_some())
        .collect();

    if benchmarks.is_empty() {
        return Ok("<svg></svg>".to_string());
    }

    // Dimensions
    let bar_height: i32 = 26;
    let bar_gap: i32 = 5;
    let margin_top: i32 = 60;
    let margin_bottom: i32 = 40;
    let margin_left: i32 = 200;
    let margin_right: i32 = 100;
    let width: i32 = 880;

    let content_height: i32 = (benchmarks.len() as i32) * (bar_height + bar_gap);
    let height: i32 = content_height + margin_top + margin_bottom;
    let chart_width: i32 = width - margin_left - margin_right;

    // Colors
    let go_color = "#00ADD8";
    let ts_color = "#3178C6";

    // Find max speedup for scale
    let max_speedup: f64 = benchmarks
        .iter()
        .filter_map(|b| b.comparison.as_ref())
        .map(|c| c.speedup)
        .fold(1.0, f64::max);

    let log_ceil = (max_speedup * 1.2).log10().ceil() as i32;
    let log_ceil = log_ceil.max(1); // Ensure at least 1

    // SVG header
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n",
        width, height, width, height));

    // Gradients
    svg.push_str("<defs>\n");
    svg.push_str(&format!(
        "  <linearGradient id=\"goGrad\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">\n"
    ));
    svg.push_str(&format!(
        "    <stop offset=\"0%\" stop-color=\"{}\" stop-opacity=\"0.95\"/>\n",
        go_color
    ));
    svg.push_str("    <stop offset=\"100%\" stop-color=\"#0891B2\" stop-opacity=\"0.85\"/>\n");
    svg.push_str("  </linearGradient>\n");
    svg.push_str(&format!(
        "  <linearGradient id=\"tsGrad\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">\n"
    ));
    svg.push_str(&format!(
        "    <stop offset=\"0%\" stop-color=\"{}\" stop-opacity=\"0.95\"/>\n",
        ts_color
    ));
    svg.push_str("    <stop offset=\"100%\" stop-color=\"#1D4ED8\" stop-opacity=\"0.85\"/>\n");
    svg.push_str("  </linearGradient>\n");
    svg.push_str("</defs>\n");

    // Background
    svg.push_str(&format!(
        "<rect width=\"{}\" height=\"{}\" fill=\"#FAFAFA\" rx=\"12\"/>\n",
        width, height
    ));
    svg.push_str(&format!(
        "<rect x=\".5\" y=\".5\" width=\"{}\" height=\"{}\" fill=\"none\" stroke=\"#E5E7EB\" rx=\"12\"/>\n",
        width - 1, height - 1));

    // Title
    let summary = &results.summary;
    let subtitle = match summary.winner {
        Some(Lang::Go) => format!("Go is {:.1}x faster overall", summary.geo_mean_speedup),
        Some(Lang::TypeScript) => {
            format!("TypeScript is {:.1}x faster overall", 1.0 / summary.geo_mean_speedup)
        }
        Some(Lang::Rust) => format!("Rust is {:.1}x faster overall", summary.geo_mean_speedup),
        _ => "Similar performance".to_string(),
    };

    svg.push_str(&format!(
        "<text x=\"{}\" y=\"30\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"16\" font-weight=\"700\" fill=\"#111827\">Benchmark Speedup Comparison</text>\n",
        width / 2));
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"48\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"11\" fill=\"#6B7280\">{} (log scale)</text>\n",
        width / 2, escape_xml(&subtitle)));

    // Chart group
    svg.push_str(&format!("<g transform=\"translate({},{})\">\n", margin_left, margin_top));

    // Grid lines
    for exp in 0..=log_ceil {
        let val = 10.0_f64.powi(exp);
        let x = (exp as f64 / log_ceil as f64) * chart_width as f64;

        svg.push_str(&format!(
            "  <line x1=\"{:.1}\" y1=\"0\" x2=\"{:.1}\" y2=\"{}\" stroke=\"#E5E7EB\"/>\n",
            x, x, content_height
        ));
        svg.push_str(&format!(
            "  <text x=\"{:.1}\" y=\"{}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"10\" fill=\"#9CA3AF\">{}x</text>\n",
            x, content_height + 16, val as i32));
    }

    // Bars
    let mut y: i32 = 0;
    for bench in &benchmarks {
        if let Some(ref cmp) = bench.comparison {
            let bar_width = if cmp.speedup <= 1.0 {
                3.0
            } else {
                (cmp.speedup.log10() / log_ceil as f64) * chart_width as f64
            };
            let bar_width = bar_width.max(3.0);

            let fill = match cmp.winner {
                poly_bench_runtime::measurement::ComparisonWinner::First => "url(#goGrad)",
                poly_bench_runtime::measurement::ComparisonWinner::Second => "url(#tsGrad)",
                poly_bench_runtime::measurement::ComparisonWinner::Tie => "#9CA3AF",
            };

            // Label
            svg.push_str(&format!(
                "  <text x=\"-10\" y=\"{}\" text-anchor=\"end\" font-family=\"sans-serif\" font-size=\"11\" fill=\"#4B5563\">{}</text>\n",
                y + bar_height / 2 + 4, escape_xml(&bench.name)));

            // Bar
            svg.push_str(&format!(
                "  <rect x=\"0\" y=\"{}\" width=\"{:.1}\" height=\"{}\" fill=\"{}\" rx=\"4\"/>\n",
                y, bar_width, bar_height, fill
            ));

            // Value
            let val_str = if cmp.speedup >= 10.0 {
                format!("{:.0}x", cmp.speedup)
            } else {
                format!("{:.1}x", cmp.speedup)
            };

            let val_color = match cmp.winner {
                poly_bench_runtime::measurement::ComparisonWinner::First => "#0E7490",
                poly_bench_runtime::measurement::ComparisonWinner::Second => "#1E40AF",
                poly_bench_runtime::measurement::ComparisonWinner::Tie => "#6B7280",
            };

            svg.push_str(&format!(
                "  <text x=\"{:.1}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"11\" font-weight=\"600\" fill=\"{}\">{}</text>\n",
                bar_width + 8.0, y + bar_height / 2 + 4, val_color, val_str));

            y += bar_height + bar_gap;
        }
    }

    svg.push_str("</g>\n");

    // Legend
    let legend_y = height - 24;
    svg.push_str(&format!("<g transform=\"translate({},{})\">\n", width / 2 - 100, legend_y));
    svg.push_str(&format!("  <rect width=\"14\" height=\"14\" fill=\"{}\" rx=\"3\"/>\n", go_color));
    svg.push_str("  <text x=\"18\" y=\"11\" font-family=\"sans-serif\" font-size=\"11\" fill=\"#4B5563\">Go faster</text>\n");
    svg.push_str(&format!(
        "  <rect x=\"100\" width=\"14\" height=\"14\" fill=\"{}\" rx=\"3\"/>\n",
        ts_color
    ));
    svg.push_str("  <text x=\"118\" y=\"11\" font-family=\"sans-serif\" font-size=\"11\" fill=\"#4B5563\">TS faster</text>\n");
    svg.push_str("</g>\n");

    svg.push_str("</svg>");

    Ok(svg)
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}
