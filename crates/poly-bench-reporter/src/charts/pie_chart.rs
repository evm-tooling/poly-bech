//! Pie chart generator for benchmark time distribution

use poly_bench_dsl::Lang;
use poly_bench_executor::BenchmarkResults;
use poly_bench_ir::ChartDirectiveIR;
use miette::Result;

use super::{
    escape_xml, svg_header, svg_title,
    filter_benchmarks, sort_benchmarks,
    format_duration_with_unit,
    DEFAULT_WIDTH, PIE_COLORS, TEXT_COLOR, TEXT_SECONDARY,
};

/// Generate a pie chart SVG showing time distribution
pub fn generate(
    results: &BenchmarkResults,
    directive: &ChartDirectiveIR,
) -> Result<String> {
    let mut svg = String::new();

    // Collect all benchmarks
    let all_benchmarks: Vec<_> = results.suites.iter()
        .flat_map(|s| s.benchmarks.iter())
        .collect();

    // Apply filtering and sorting
    let mut filtered = filter_benchmarks(all_benchmarks, directive);
    sort_benchmarks(&mut filtered, directive);

    // Collect benchmark times (using Go times, or TS if Go not available)
    let benchmark_times: Vec<(&str, f64)> = filtered.iter()
        .filter_map(|bench| {
            let time = bench.measurements.get(&Lang::Go)
                .or_else(|| bench.measurements.get(&Lang::TypeScript))
                .map(|m| m.nanos_per_op)
                .unwrap_or(0.0);
            
            if time > 0.0 {
                Some((bench.name.as_str(), time))
            } else {
                None
            }
        })
        .collect();

    if benchmark_times.is_empty() {
        return Ok("<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>".to_string());
    }

    // Extract parameters
    let width = directive.width.unwrap_or(DEFAULT_WIDTH);
    let time_unit = directive.time_unit.as_deref();
    let precision = directive.precision;
    let compact = directive.compact;

    // Dimensions
    let height = 500;
    let center_x = width / 2;
    let center_y = 250;
    let radius = 150.0;

    // Calculate total and percentages
    let total: f64 = benchmark_times.iter().map(|(_, t)| t).sum();

    // SVG header
    svg.push_str(&svg_header(width, height));

    // Title
    let title = directive.title.clone().unwrap_or_else(|| "Time Distribution".to_string());
    let subtitle = directive.description.clone()
        .unwrap_or_else(|| "Time distribution across benchmarks".to_string());
    svg.push_str(&svg_title(width, &title, Some(&subtitle)));

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
        
        // Enhanced legend with timing info if showStats is enabled
        let label = if directive.show_stats && !compact {
            let time_str = format_duration_with_unit(*time, time_unit, precision);
            format!("{} ({:.1}%, {})", escape_xml(name), percentage, time_str)
        } else {
            format!("{} ({:.1}%)", escape_xml(name), percentage)
        };
        
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"10\" fill=\"{}\">{}</text>\n",
            x + 16, y + 10, TEXT_COLOR, label
        ));
    }
    
    svg.push_str("</g>\n");

    // Show total time if enabled
    if directive.show_total_time && !compact {
        let total_str = format_duration_with_unit(total, time_unit, precision);
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"11\" fill=\"{}\">Total: {}</text>\n",
            center_x, center_y + radius as i32 + 30, TEXT_SECONDARY, total_str
        ));
    }

    svg.push_str("</svg>");

    Ok(svg)
}

/// Generate a pie chart using the legacy signature (backwards compatibility)
pub fn generate_simple(
    results: &BenchmarkResults,
    title: &str,
    description: Option<&str>,
) -> Result<String> {
    use poly_bench_dsl::ChartType;
    
    let mut directive = ChartDirectiveIR::new(ChartType::PieChart, "pie-chart.svg".to_string());
    directive.title = Some(title.to_string());
    directive.description = description.map(|s| s.to_string());
    
    generate(results, &directive)
}

#[cfg(test)]
mod tests {
    use super::*;
    use poly_bench_executor::comparison::{SuiteResults, BenchmarkResult};
    use poly_bench_dsl::ChartType;
    use std::collections::HashMap;

    fn default_directive() -> ChartDirectiveIR {
        ChartDirectiveIR::new(ChartType::PieChart, "test.svg".to_string())
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
        let svg = generate_simple(&results, "Test", None).unwrap();
        assert!(svg.contains("<svg"));
    }
}
