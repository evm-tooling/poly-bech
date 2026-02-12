//! Pie chart generator for benchmark time distribution

use poly_bench_dsl::Lang;
use poly_bench_executor::BenchmarkResults;
use miette::Result;

use super::{
    escape_xml, svg_header, svg_title,
    DEFAULT_WIDTH, PIE_COLORS,
};

/// Generate a pie chart SVG showing time distribution
pub fn generate(
    results: &BenchmarkResults,
    title: &str,
    description: Option<&str>,
) -> Result<String> {
    let mut svg = String::new();

    // Collect benchmark times (using Go times, or TS if Go not available)
    let mut benchmark_times: Vec<(&str, f64)> = Vec::new();
    
    for suite in &results.suites {
        for bench in &suite.benchmarks {
            let time = bench.measurements.get(&Lang::Go)
                .or_else(|| bench.measurements.get(&Lang::TypeScript))
                .map(|m| m.nanos_per_op)
                .unwrap_or(0.0);
            
            if time > 0.0 {
                benchmark_times.push((&bench.name, time));
            }
        }
    }

    if benchmark_times.is_empty() {
        return Ok("<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>".to_string());
    }

    // Dimensions
    let width = DEFAULT_WIDTH;
    let height = 500;
    let center_x = width / 2;
    let center_y = 250;
    let radius = 150.0;

    // Calculate total and percentages
    let total: f64 = benchmark_times.iter().map(|(_, t)| t).sum();

    // SVG header
    svg.push_str(&svg_header(width, height));

    // Title
    let subtitle = description.unwrap_or("Time distribution across benchmarks");
    svg.push_str(&svg_title(width, title, Some(subtitle)));

    // Draw pie slices
    let mut current_angle = -90.0_f64; // Start at top

    for (i, (name, time)) in benchmark_times.iter().enumerate() {
        let percentage = time / total;
        let angle_extent = percentage * 360.0;
        
        let color = PIE_COLORS[i % PIE_COLORS.len()];
        
        // Calculate arc
        let start_rad = current_angle.to_radians();
        let end_rad = (current_angle + angle_extent).to_radians();
        
        let x1 = center_x as f64 + radius * start_rad.cos();
        let y1 = center_y as f64 + radius * start_rad.sin();
        let x2 = center_x as f64 + radius * end_rad.cos();
        let y2 = center_y as f64 + radius * end_rad.sin();
        
        let large_arc = if angle_extent > 180.0 { 1 } else { 0 };
        
        // Draw slice
        svg.push_str(&format!(
            "<path d=\"M{},{} L{:.1},{:.1} A{},{} 0 {} 1 {:.1},{:.1} Z\" fill=\"{}\" stroke=\"white\" stroke-width=\"2\"/>\n",
            center_x, center_y,
            x1, y1,
            radius, radius,
            large_arc,
            x2, y2,
            color
        ));

        // Draw label if slice is big enough
        if percentage >= 0.05 {
            let label_angle = (current_angle + angle_extent / 2.0).to_radians();
            let label_radius = radius * 0.7;
            let label_x = center_x as f64 + label_radius * label_angle.cos();
            let label_y = center_y as f64 + label_radius * label_angle.sin();
            
            svg.push_str(&format!(
                "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"10\" font-weight=\"600\" fill=\"white\">{:.0}%</text>\n",
                label_x, label_y,
                percentage * 100.0
            ));
        }

        current_angle += angle_extent;
    }

    // Legend
    let legend_start_y = height - 80;
    let legend_cols = 3;
    let legend_item_width = (width - 100) / legend_cols;

    svg.push_str(&format!("<g transform=\"translate(50,{})\">\n", legend_start_y));
    
    for (i, (name, time)) in benchmark_times.iter().enumerate() {
        let col = i as i32 % legend_cols;
        let row = i as i32 / legend_cols;
        let x = col * legend_item_width;
        let y = row * 20;
        let color = PIE_COLORS[i % PIE_COLORS.len()];
        let percentage = time / total * 100.0;
        
        svg.push_str(&format!(
            "  <rect x=\"{}\" y=\"{}\" width=\"12\" height=\"12\" fill=\"{}\" rx=\"2\"/>\n",
            x, y, color
        ));
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"10\" fill=\"#4B5563\">{} ({:.1}%)</text>\n",
            x + 16, y + 10, escape_xml(name), percentage
        ));
    }
    
    svg.push_str("</g>\n");

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
        let svg = generate(&results, "Test", None).unwrap();
        assert!(svg.contains("<svg"));
    }
}
