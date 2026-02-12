//! Bar chart generator for benchmark results

use poly_bench_dsl::Lang;
use poly_bench_executor::BenchmarkResults;
use poly_bench_runtime::measurement::ComparisonWinner;
use miette::Result;

use super::{
    escape_xml, svg_header, svg_title, svg_legend,
    DEFAULT_WIDTH, DEFAULT_MARGIN_TOP, DEFAULT_MARGIN_BOTTOM,
    DEFAULT_MARGIN_LEFT, DEFAULT_MARGIN_RIGHT,
    GO_COLOR, TS_COLOR,
};

const BAR_HEIGHT: i32 = 26;
const BAR_GAP: i32 = 5;

/// Generate a bar chart SVG from benchmark results
pub fn generate(
    results: &BenchmarkResults,
    title: &str,
    description: Option<&str>,
    _x_label: &str,
    _y_label: &str,
) -> Result<String> {
    let mut svg = String::new();

    // Collect all benchmarks with comparisons
    let benchmarks: Vec<_> = results.suites.iter()
        .flat_map(|s| s.benchmarks.iter())
        .filter(|b| b.comparison.is_some())
        .collect();

    if benchmarks.is_empty() {
        return Ok("<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>".to_string());
    }

    // Dimensions
    let width = DEFAULT_WIDTH;
    let content_height = (benchmarks.len() as i32) * (BAR_HEIGHT + BAR_GAP);
    let height = content_height + DEFAULT_MARGIN_TOP + DEFAULT_MARGIN_BOTTOM;
    let chart_width = width - DEFAULT_MARGIN_LEFT - DEFAULT_MARGIN_RIGHT;

    // Find max speedup for scale
    let max_speedup: f64 = benchmarks.iter()
        .filter_map(|b| b.comparison.as_ref())
        .map(|c| c.speedup)
        .fold(1.0, f64::max);

    let log_ceil = (max_speedup * 1.2).log10().ceil() as i32;
    let log_ceil = log_ceil.max(1);

    // Build subtitle
    let subtitle = description.map(|d| d.to_string()).unwrap_or_else(|| {
        match results.summary.winner {
            Some(Lang::Go) => format!("Go is {:.1}x faster overall", results.summary.geo_mean_speedup),
            Some(Lang::TypeScript) => format!("TypeScript is {:.1}x faster overall", 1.0 / results.summary.geo_mean_speedup),
            _ => "Similar performance".to_string(),
        }
    });

    // SVG header
    svg.push_str(&svg_header(width, height));

    // Title
    svg.push_str(&svg_title(width, title, Some(&subtitle)));

    // Chart group
    svg.push_str(&format!("<g transform=\"translate({},{})\">\n", DEFAULT_MARGIN_LEFT, DEFAULT_MARGIN_TOP));

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
            x, content_height + 16, val as i32
        ));
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
                ComparisonWinner::First => "url(#goGrad)",
                ComparisonWinner::Second => "url(#tsGrad)",
                ComparisonWinner::Tie => "#9CA3AF",
            };

            // Label
            svg.push_str(&format!(
                "  <text x=\"-10\" y=\"{}\" text-anchor=\"end\" font-family=\"sans-serif\" font-size=\"11\" fill=\"#4B5563\">{}</text>\n",
                y + BAR_HEIGHT / 2 + 4, escape_xml(&bench.name)
            ));

            // Bar
            svg.push_str(&format!(
                "  <rect x=\"0\" y=\"{}\" width=\"{:.1}\" height=\"{}\" fill=\"{}\" rx=\"4\"/>\n",
                y, bar_width, BAR_HEIGHT, fill
            ));

            // Value
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
                bar_width + 8.0, y + BAR_HEIGHT / 2 + 4, val_color, val_str
            ));

            y += BAR_HEIGHT + BAR_GAP;
        }
    }

    svg.push_str("</g>\n");

    // Legend
    let legend_y = height - 24;
    svg.push_str(&svg_legend(width, legend_y, &[
        (GO_COLOR, "Go faster"),
        (TS_COLOR, "TS faster"),
    ]));

    svg.push_str("</svg>");

    Ok(svg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use poly_bench_executor::comparison::{SuiteResults, BenchmarkResult};
    use std::collections::HashMap;

    #[test]
    fn test_generate_empty() {
        let results = BenchmarkResults::new(vec![]);
        let svg = generate(&results, "Test", None, "X", "Y").unwrap();
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
        
        let svg = generate(&results, "Test", None, "X", "Y").unwrap();
        assert!(svg.contains("<svg"));
    }
}
