//! Line chart generator for benchmark trends

use poly_bench_dsl::Lang;
use poly_bench_executor::BenchmarkResults;
use miette::Result;

use super::{
    escape_xml, svg_header, svg_title, svg_legend, format_duration,
    DEFAULT_WIDTH, DEFAULT_MARGIN_TOP, DEFAULT_MARGIN_BOTTOM,
    DEFAULT_MARGIN_LEFT, DEFAULT_MARGIN_RIGHT,
    GO_COLOR, TS_COLOR,
};

/// Generate a line chart SVG showing benchmark trends
pub fn generate(
    results: &BenchmarkResults,
    title: &str,
    description: Option<&str>,
    x_label: &str,
    y_label: &str,
) -> Result<String> {
    let mut svg = String::new();

    // Collect benchmark data points
    let mut go_points: Vec<(String, f64)> = Vec::new();
    let mut ts_points: Vec<(String, f64)> = Vec::new();
    
    for suite in &results.suites {
        for bench in &suite.benchmarks {
            if let Some(m) = bench.measurements.get(&Lang::Go) {
                go_points.push((bench.name.clone(), m.nanos_per_op));
            }
            if let Some(m) = bench.measurements.get(&Lang::TypeScript) {
                ts_points.push((bench.name.clone(), m.nanos_per_op));
            }
        }
    }

    if go_points.is_empty() && ts_points.is_empty() {
        return Ok("<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>".to_string());
    }

    // Dimensions
    let width = DEFAULT_WIDTH;
    let height = 400;
    let chart_width = width - DEFAULT_MARGIN_LEFT - DEFAULT_MARGIN_RIGHT;
    let chart_height = height - DEFAULT_MARGIN_TOP - DEFAULT_MARGIN_BOTTOM - 40;

    // Find max value for Y scale
    let max_value: f64 = go_points.iter().map(|(_, v)| *v)
        .chain(ts_points.iter().map(|(_, v)| *v))
        .fold(0.0, f64::max);

    // Collect unique benchmark names for X axis
    let bench_names: Vec<String> = go_points.iter()
        .map(|(n, _)| n.clone())
        .chain(ts_points.iter().map(|(n, _)| n.clone()))
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let num_points = bench_names.len();
    if num_points == 0 {
        return Ok("<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>".to_string());
    }

    // SVG header
    svg.push_str(&svg_header(width, height));

    // Title
    let subtitle = description.unwrap_or("Performance comparison across benchmarks");
    svg.push_str(&svg_title(width, title, Some(subtitle)));

    // Chart group
    svg.push_str(&format!("<g transform=\"translate({},{})\">\n", DEFAULT_MARGIN_LEFT, DEFAULT_MARGIN_TOP));

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
            "  <text x=\"-10\" y=\"{}\" text-anchor=\"end\" font-family=\"sans-serif\" font-size=\"10\" fill=\"#9CA3AF\">{}</text>\n",
            y + 4, format_duration(value)
        ));
    }

    // Draw X axis labels
    for (i, name) in bench_names.iter().enumerate() {
        let x = (i as f64 / (num_points - 1).max(1) as f64 * chart_width as f64) as i32;
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"10\" fill=\"#9CA3AF\">{}</text>\n",
            x, chart_height + 16, escape_xml(name)
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
        points: &[(String, f64)],
        bench_names: &[String],
        chart_width: i32,
        chart_height: i32,
        max_value: f64,
        color: &str,
    ) {
        if points.is_empty() {
            return;
        }

        let num_points = bench_names.len();
        
        // Build path
        let mut path_data = String::new();
        let mut first = true;
        
        for (i, name) in bench_names.iter().enumerate() {
            if let Some((_, value)) = points.iter().find(|(n, _)| n == name) {
                let x = (i as f64 / (num_points - 1).max(1) as f64 * chart_width as f64) as i32;
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

        // Draw points
        for (i, name) in bench_names.iter().enumerate() {
            if let Some((_, value)) = points.iter().find(|(n, _)| n == name) {
                let x = (i as f64 / (num_points - 1).max(1) as f64 * chart_width as f64) as i32;
                let y = chart_height - (value / max_value * chart_height as f64) as i32;
                
                svg.push_str(&format!(
                    "  <circle cx=\"{}\" cy=\"{}\" r=\"4\" fill=\"{}\" stroke=\"white\" stroke-width=\"2\"/>\n",
                    x, y, color
                ));
            }
        }
    }

    // Draw Go line
    draw_series(&mut svg, &go_points, &bench_names, chart_width, chart_height, max_value, GO_COLOR);
    
    // Draw TS line
    draw_series(&mut svg, &ts_points, &bench_names, chart_width, chart_height, max_value, TS_COLOR);

    // Axis labels
    svg.push_str(&format!(
        "  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"11\" fill=\"#4B5563\">{}</text>\n",
        chart_width / 2, chart_height + 35, escape_xml(x_label)
    ));
    svg.push_str(&format!(
        "  <text x=\"-40\" y=\"{}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"11\" fill=\"#4B5563\" transform=\"rotate(-90,-40,{})\">{}</text>\n",
        chart_height / 2, chart_height / 2, escape_xml(y_label)
    ));

    svg.push_str("</g>\n");

    // Legend
    let legend_y = height - 24;
    svg.push_str(&svg_legend(width, legend_y, &[
        (GO_COLOR, "Go"),
        (TS_COLOR, "TypeScript"),
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
}
