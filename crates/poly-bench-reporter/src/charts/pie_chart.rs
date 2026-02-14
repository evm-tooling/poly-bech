//! Pie chart generator - individual pie charts per benchmark showing Go vs TS distribution

use miette::Result;
use poly_bench_dsl::Lang;
use poly_bench_executor::BenchmarkResults;
use poly_bench_ir::ChartDirectiveIR;

use super::{
    calculate_geo_mean, count_wins, escape_xml, filter_benchmarks, format_duration_with_unit,
    format_ops_per_sec, sort_benchmarks, svg_header, svg_title, BORDER_COLOR, GO_COLOR, TEXT_COLOR,
    TEXT_MUTED, TEXT_SECONDARY, TS_COLOR,
};

// Layout constants for individual pie charts
const PIE_RADIUS: f64 = 50.0;
const PIE_SPACING_X: i32 = 150; // Horizontal spacing between pie centers
const PIE_SPACING_Y: i32 = 180; // Vertical spacing between pie rows
const PIES_PER_ROW: i32 = 4; // Max pies per row before wrapping
const STATS_BOX_HEIGHT: i32 = 80;
const LEGEND_HEIGHT: i32 = 40;
const MIN_STATS_WIDTH: i32 = 520; // Minimum width to fit stats text

/// Generate a pie chart SVG showing individual Go vs TS pies per benchmark
pub fn generate(results: &BenchmarkResults, directive: &ChartDirectiveIR) -> Result<String> {
    let mut svg = String::new();

    // Collect all benchmarks
    let all_benchmarks: Vec<_> = results
        .suites
        .iter()
        .flat_map(|s| s.benchmarks.iter())
        .collect();

    // Apply filtering and sorting
    let mut filtered = filter_benchmarks(all_benchmarks, directive);
    sort_benchmarks(&mut filtered, directive);

    // Filter to only benchmarks with both Go and TS measurements
    let benchmarks_with_both: Vec<_> = filtered
        .iter()
        .filter(|b| {
            let has_go = b
                .measurements
                .get(&Lang::Go)
                .map(|m| m.total_nanos > 0)
                .unwrap_or(false);
            let has_ts = b
                .measurements
                .get(&Lang::TypeScript)
                .map(|m| m.total_nanos > 0)
                .unwrap_or(false);
            has_go || has_ts
        })
        .collect();

    if benchmarks_with_both.is_empty() {
        return Ok("<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>".to_string());
    }

    // Extract parameters
    let time_unit = directive.time_unit.as_deref();
    let precision = directive.precision;
    let compact = directive.compact;

    // Calculate dynamic dimensions based on number of benchmarks
    let num_benchmarks = benchmarks_with_both.len() as i32;
    let pies_per_row = directive
        .width
        .map(|w| (w / PIE_SPACING_X).max(1))
        .unwrap_or(PIES_PER_ROW);
    let num_rows = (num_benchmarks + pies_per_row - 1) / pies_per_row;

    // Calculate width and height
    let actual_pies_in_first_row = num_benchmarks.min(pies_per_row);
    let margin_x = 80;
    let content_width = (actual_pies_in_first_row * PIE_SPACING_X) + margin_x * 2;

    // Stats box needs minimum width to avoid overflow
    let stats_box_needed =
        (directive.show_stats || directive.show_distribution || directive.show_geo_mean)
            && !compact;
    let min_width = if stats_box_needed {
        MIN_STATS_WIDTH
    } else {
        300
    };
    let width = directive.width.unwrap_or(content_width).max(min_width);

    // Calculate summary statistics
    let (go_wins, ts_wins, ties) = count_wins(&filtered);
    let geo_mean = calculate_geo_mean(&filtered);

    // Height calculation
    let legend_space = LEGEND_HEIGHT;
    let stats_box_space =
        if (directive.show_stats || directive.show_distribution || directive.show_geo_mean)
            && !compact
        {
            STATS_BOX_HEIGHT
        } else {
            0
        };
    let pie_area_height = num_rows * PIE_SPACING_Y;
    let height = 80 + pie_area_height + legend_space + stats_box_space + 40;

    // SVG header
    svg.push_str(&svg_header(width, height));

    // Title
    let title = directive
        .title
        .clone()
        .unwrap_or_else(|| "Time Distribution".to_string());
    let subtitle = directive
        .description
        .clone()
        .unwrap_or_else(|| "Go vs TypeScript execution time per benchmark".to_string());
    svg.push_str(&svg_title(width, &title, Some(&subtitle)));

    // Draw individual pie charts for each benchmark
    let start_y = 100;

    for (i, bench) in benchmarks_with_both.iter().enumerate() {
        let row = i as i32 / pies_per_row;
        let col = i as i32 % pies_per_row;

        // Calculate center position for this pie
        // For single benchmark, center in the available width
        let center_x = if num_benchmarks == 1 {
            width / 2
        } else {
            margin_x + (col * PIE_SPACING_X) + PIE_SPACING_X / 2
        };
        let center_y = start_y + (row * PIE_SPACING_Y) + PIE_SPACING_Y / 2 - 20;

        // Get Go and TS times
        let go_time = bench
            .measurements
            .get(&Lang::Go)
            .map(|m| m.total_nanos as f64)
            .unwrap_or(0.0);
        let ts_time = bench
            .measurements
            .get(&Lang::TypeScript)
            .map(|m| m.total_nanos as f64)
            .unwrap_or(0.0);
        let total = go_time + ts_time;

        if total == 0.0 {
            continue;
        }

        // Calculate percentages
        let go_pct = go_time / total;
        let ts_pct = ts_time / total;

        // Determine winner
        let go_faster = go_time < ts_time;
        let ts_faster = ts_time < go_time;

        // Draw pie chart group
        svg.push_str(&format!(
            "<g transform=\"translate({},{})\">\n",
            center_x, center_y
        ));

        // Draw Go slice (starting from top, going clockwise)
        let go_angle = go_pct * 360.0;
        if go_time > 0.0 && go_pct < 1.0 {
            let start_rad = (-90.0_f64).to_radians();
            let end_rad = (-90.0 + go_angle).to_radians();

            let x1 = PIE_RADIUS * start_rad.cos();
            let y1 = PIE_RADIUS * start_rad.sin();
            let x2 = PIE_RADIUS * end_rad.cos();
            let y2 = PIE_RADIUS * end_rad.sin();

            let large_arc = if go_angle > 180.0 { 1 } else { 0 };

            svg.push_str(&format!(
                "  <path d=\"M0,0 L{:.1},{:.1} A{},{} 0 {} 1 {:.1},{:.1} Z\" fill=\"{}\" stroke=\"white\" stroke-width=\"2\"/>\n",
                x1, y1, PIE_RADIUS, PIE_RADIUS, large_arc, x2, y2, GO_COLOR
            ));
        } else if go_pct >= 1.0 {
            // Full circle for Go
            svg.push_str(&format!(
                "  <circle cx=\"0\" cy=\"0\" r=\"{}\" fill=\"{}\" stroke=\"white\" stroke-width=\"2\"/>\n",
                PIE_RADIUS, GO_COLOR
            ));
        }

        // Draw TS slice (continuing from Go slice)
        if ts_time > 0.0 && ts_pct < 1.0 {
            let start_rad = (-90.0 + go_angle).to_radians();
            let end_rad = (270.0_f64).to_radians(); // End at top (full circle)

            let x1 = PIE_RADIUS * start_rad.cos();
            let y1 = PIE_RADIUS * start_rad.sin();
            let x2 = PIE_RADIUS * end_rad.cos();
            let y2 = PIE_RADIUS * end_rad.sin();

            let ts_angle = ts_pct * 360.0;
            let large_arc = if ts_angle > 180.0 { 1 } else { 0 };

            svg.push_str(&format!(
                "  <path d=\"M0,0 L{:.1},{:.1} A{},{} 0 {} 1 {:.1},{:.1} Z\" fill=\"{}\" stroke=\"white\" stroke-width=\"2\"/>\n",
                x1, y1, PIE_RADIUS, PIE_RADIUS, large_arc, x2, y2, TS_COLOR
            ));
        } else if ts_pct >= 1.0 {
            // Full circle for TS
            svg.push_str(&format!(
                "  <circle cx=\"0\" cy=\"0\" r=\"{}\" fill=\"{}\" stroke=\"white\" stroke-width=\"2\"/>\n",
                PIE_RADIUS, TS_COLOR
            ));
        }

        // Winner indicator in center
        if !compact {
            let winner_color = if go_faster {
                GO_COLOR
            } else if ts_faster {
                TS_COLOR
            } else {
                TEXT_MUTED
            };
            let winner_label = if go_faster {
                "Go"
            } else if ts_faster {
                "TS"
            } else {
                "="
            };
            svg.push_str(&format!(
                "  <circle cx=\"0\" cy=\"0\" r=\"18\" fill=\"white\" stroke=\"{}\" stroke-width=\"2\"/>\n",
                winner_color
            ));
            svg.push_str(&format!(
                "  <text x=\"0\" y=\"4\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"11\" font-weight=\"600\" fill=\"{}\">{}</text>\n",
                winner_color, winner_label
            ));
        }

        // Benchmark name below pie
        svg.push_str(&format!(
            "  <text x=\"0\" y=\"{}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"10\" font-weight=\"500\" fill=\"{}\">{}</text>\n",
            PIE_RADIUS as i32 + 18, TEXT_COLOR, escape_xml(&bench.name)
        ));

        // Percentage labels
        if directive.show_stats && !compact {
            // Go percentage (left side)
            if go_pct > 0.05 {
                svg.push_str(&format!(
                    "  <text x=\"{}\" y=\"0\" text-anchor=\"end\" font-family=\"sans-serif\" font-size=\"9\" fill=\"{}\">{:.0}%</text>\n",
                    -(PIE_RADIUS as i32 + 8), GO_COLOR, go_pct * 100.0
                ));
            }
            // TS percentage (right side)
            if ts_pct > 0.05 {
                svg.push_str(&format!(
                    "  <text x=\"{}\" y=\"0\" text-anchor=\"start\" font-family=\"sans-serif\" font-size=\"9\" fill=\"{}\">{:.0}%</text>\n",
                    PIE_RADIUS as i32 + 8, TS_COLOR, ts_pct * 100.0
                ));
            }
        }

        // Timing info below name
        if directive.show_total_time && !compact {
            let go_str = format_duration_with_unit(go_time, time_unit, precision);
            let ts_str = format_duration_with_unit(ts_time, time_unit, precision);
            svg.push_str(&format!(
                "  <text x=\"0\" y=\"{}\" text-anchor=\"middle\" font-family=\"monospace\" font-size=\"8\" fill=\"{}\">Go: {} | TS: {}</text>\n",
                PIE_RADIUS as i32 + 32, TEXT_MUTED, go_str, ts_str
            ));
        }

        svg.push_str("</g>\n");
    }

    // Legend
    let legend_y = start_y + (num_rows * PIE_SPACING_Y) + 20;
    svg.push_str(&format!(
        "<g transform=\"translate({},{})\">\n",
        width / 2,
        legend_y
    ));

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

    // Stats box (similar to bar chart)
    if (directive.show_stats || directive.show_distribution || directive.show_geo_mean) && !compact
    {
        let box_y = legend_y + LEGEND_HEIGHT;
        let box_width = width - 80;
        let box_x = 40;
        let box_padding = 12;

        // Box background with border
        svg.push_str(&format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#F9FAFB\" stroke=\"{}\" rx=\"6\"/>\n",
            box_x, box_y, box_width, STATS_BOX_HEIGHT - 10, BORDER_COLOR
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
                format!(
                    "Go is {:.2}x faster on average (geometric mean)  |  Total benchmarks: {}",
                    geo_mean,
                    filtered.len()
                )
            } else {
                format!("TypeScript is {:.2}x faster on average (geometric mean)  |  Total benchmarks: {}", 1.0 / geo_mean, filtered.len())
            };
            svg.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"10\" fill=\"{}\">• {}</text>\n",
                stats_x, stats_y, TEXT_SECONDARY, escape_xml(&geo_label)
            ));
            stats_y += 14;
        }

        // Distribution stats (aggregate p50/p99)
        if directive.show_distribution {
            let go_p50_avg: f64 = filtered
                .iter()
                .filter_map(|b| b.comparison.as_ref())
                .filter_map(|c| c.first.p50_nanos)
                .map(|n| n as f64)
                .sum::<f64>()
                / filtered.len().max(1) as f64;
            let go_p99_avg: f64 = filtered
                .iter()
                .filter_map(|b| b.comparison.as_ref())
                .filter_map(|c| c.first.p99_nanos)
                .map(|n| n as f64)
                .sum::<f64>()
                / filtered.len().max(1) as f64;
            let ts_p50_avg: f64 = filtered
                .iter()
                .filter_map(|b| b.comparison.as_ref())
                .filter_map(|c| c.second.p50_nanos)
                .map(|n| n as f64)
                .sum::<f64>()
                / filtered.len().max(1) as f64;
            let ts_p99_avg: f64 = filtered
                .iter()
                .filter_map(|b| b.comparison.as_ref())
                .filter_map(|c| c.second.p99_nanos)
                .map(|n| n as f64)
                .sum::<f64>()
                / filtered.len().max(1) as f64;

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
            let total_go_ops: f64 = filtered
                .iter()
                .filter_map(|b| b.comparison.as_ref())
                .map(|c| c.first.ops_per_sec)
                .sum();
            let total_ts_ops: f64 = filtered
                .iter()
                .filter_map(|b| b.comparison.as_ref())
                .map(|c| c.second.ops_per_sec)
                .sum();
            let avg_go_ops = total_go_ops / filtered.len().max(1) as f64;
            let avg_ts_ops = total_ts_ops / filtered.len().max(1) as f64;

            let ops_str = format!(
                "Avg ops/sec: Go {} / TS {}",
                format_ops_per_sec(avg_go_ops),
                format_ops_per_sec(avg_ts_ops)
            );
            svg.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" font-family=\"monospace\" font-size=\"9\" fill=\"{}\">• {}</text>\n",
                stats_x, stats_y, TEXT_MUTED, escape_xml(&ops_str)
            ));
        }
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
    use poly_bench_dsl::ChartType;

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
