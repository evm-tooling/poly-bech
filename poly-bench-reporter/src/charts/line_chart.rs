//! Line chart generator for benchmark trends

use miette::Result;
use poly_bench_dsl::Lang;
use poly_bench_executor::BenchmarkResults;
use poly_bench_ir::ChartDirectiveIR;

use super::{
    compute_ci_bounds, escape_xml, filter_benchmarks, format_duration_with_unit,
    format_ops_per_sec,
    regression::{select_model, ModelType},
    sort_benchmarks, svg_header, svg_title, DEFAULT_MARGIN_BOTTOM, DEFAULT_MARGIN_LEFT,
    DEFAULT_MARGIN_RIGHT, DEFAULT_MARGIN_TOP, GO_COLOR, RUST_COLOR, TEXT_MUTED, TEXT_SECONDARY,
    TS_COLOR,
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
    let width = directive.width.unwrap_or(620); // Default line chart width
    let time_unit = directive.time_unit.as_deref();
    let precision = directive.precision;

    // Error bar settings
    let show_error_bars = directive.show_error_bars.unwrap_or(false);
    let error_bar_opacity = directive.error_bar_opacity.unwrap_or(0.4);
    let error_bar_thickness = directive.error_bar_thickness.unwrap_or(1.5);

    // Grid settings
    let show_grid = directive.show_grid.unwrap_or(true);
    let grid_opacity = directive.grid_opacity.unwrap_or(1.0);
    let show_minor_grid = directive.show_minor_grid.unwrap_or(false);
    let minor_grid_opacity = directive.minor_grid_opacity.unwrap_or(0.8);
    let show_vertical_grid = directive.show_vertical_grid.unwrap_or(false);
    let ci_level = directive.ci_level.unwrap_or(95);

    // Typography settings
    let tick_label_font_size = directive.tick_label_font_size.unwrap_or(11);

    // Collect benchmark data points from filtered results
    // (name, nanos, ops, ci_lower, ci_upper, std_dev)
    let mut go_points: Vec<(String, f64, f64, Option<f64>, Option<f64>, Option<f64>)> = Vec::new();
    let mut ts_points: Vec<(String, f64, f64, Option<f64>, Option<f64>, Option<f64>)> = Vec::new();
    let mut rust_points: Vec<(String, f64, f64, Option<f64>, Option<f64>, Option<f64>)> =
        Vec::new();

    for bench in &filtered {
        if let Some(m) = bench.measurements.get(&Lang::Go) {
            let (ci_lower, ci_upper) = compute_ci_bounds(
                m.nanos_per_op,
                m.raw_samples.as_ref(),
                ci_level,
                m.ci_95_lower,
                m.ci_95_upper,
            );
            go_points.push((
                bench.name.clone(),
                m.nanos_per_op,
                m.ops_per_sec,
                ci_lower,
                ci_upper,
                m.std_dev_nanos,
            ));
        }
        if let Some(m) = bench.measurements.get(&Lang::TypeScript) {
            let (ci_lower, ci_upper) = compute_ci_bounds(
                m.nanos_per_op,
                m.raw_samples.as_ref(),
                ci_level,
                m.ci_95_lower,
                m.ci_95_upper,
            );
            ts_points.push((
                bench.name.clone(),
                m.nanos_per_op,
                m.ops_per_sec,
                ci_lower,
                ci_upper,
                m.std_dev_nanos,
            ));
        }
        if let Some(m) = bench.measurements.get(&Lang::Rust) {
            let (ci_lower, ci_upper) = compute_ci_bounds(
                m.nanos_per_op,
                m.raw_samples.as_ref(),
                ci_level,
                m.ci_95_lower,
                m.ci_95_upper,
            );
            rust_points.push((
                bench.name.clone(),
                m.nanos_per_op,
                m.ops_per_sec,
                ci_lower,
                ci_upper,
                m.std_dev_nanos,
            ));
        }
    }

    if go_points.is_empty() && ts_points.is_empty() && rust_points.is_empty() {
        return Ok("<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>".to_string());
    }

    // Dimensions - use directive height or default
    let height = directive.height.unwrap_or(445); // Default line chart height
    let chart_width = width - DEFAULT_MARGIN_LEFT - DEFAULT_MARGIN_RIGHT;
    let chart_height = height - DEFAULT_MARGIN_TOP - DEFAULT_MARGIN_BOTTOM;

    // Collect all Y values for scaling (including CI bounds if error bars enabled)
    let all_values: Vec<f64> = go_points
        .iter()
        .flat_map(|(_, v, _, ci_lower, ci_upper, _)| {
            let mut vals = vec![*v];
            if let Some(lower) = ci_lower {
                vals.push(*lower);
            }
            if let Some(upper) = ci_upper {
                vals.push(*upper);
            }
            vals
        })
        .chain(ts_points.iter().flat_map(|(_, v, _, ci_lower, ci_upper, _)| {
            let mut vals = vec![*v];
            if let Some(lower) = ci_lower {
                vals.push(*lower);
            }
            if let Some(upper) = ci_upper {
                vals.push(*upper);
            }
            vals
        }))
        .chain(rust_points.iter().flat_map(|(_, v, _, ci_lower, ci_upper, _)| {
            let mut vals = vec![*v];
            if let Some(lower) = ci_lower {
                vals.push(*lower);
            }
            if let Some(upper) = ci_upper {
                vals.push(*upper);
            }
            vals
        }))
        .collect();

    // Determine Y-axis scale type
    let use_log_scale = directive.y_scale.as_deref() == Some("log");

    // Calculate min/max values
    let raw_max: f64 = all_values.iter().cloned().fold(1.0, f64::max);
    let raw_min: f64 = all_values.iter().cloned().fold(raw_max, f64::min).max(1.0);

    let max_value = directive.y_axis_max.unwrap_or(raw_max * 1.15);
    let min_value = if use_log_scale {
        directive.y_axis_min.unwrap_or(raw_min * 0.5).max(1.0)
    } else {
        directive.y_axis_min.unwrap_or(0.0)
    };

    let value_range = (max_value - min_value).max(1.0);
    let log_min = min_value.max(1.0).log10();
    let log_max = max_value.log10();
    let log_range = (log_max - log_min).max(0.001);

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

    // Draw Y axis grid lines and ticks
    if use_log_scale {
        // Log scale: draw grid lines at powers of 10
        let tick_values = compute_log_ticks(min_value, max_value);

        // Draw minor grid lines first (so major lines draw on top)
        if show_minor_grid && tick_values.len() >= 2 {
            for window in tick_values.windows(2) {
                let lower = window[0];
                let upper = window[1];
                for mult in 2..=9 {
                    let minor_value = lower * mult as f64;
                    if minor_value > upper {
                        break;
                    }
                    let y = chart_height -
                        ((minor_value.log10() - log_min) / log_range * chart_height as f64)
                            as i32;
                    svg.push_str(&format!(
                        "  <line x1=\"0\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#E5E7EB\" stroke-width=\"0.5\" opacity=\"{}\"/>\n",
                        y, chart_width, y, minor_grid_opacity
                    ));
                }
            }
        }

        for &tick_value in &tick_values {
            let y = chart_height -
                ((tick_value.log10() - log_min) / log_range * chart_height as f64) as i32;

            if show_grid {
                svg.push_str(&format!(
                    "  <line x1=\"0\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#E5E7EB\" opacity=\"{}\"/>\n",
                    y, chart_width, y, grid_opacity
                ));
            }
            svg.push_str(&format!(
                "  <text x=\"-10\" y=\"{}\" text-anchor=\"end\" font-family=\"sans-serif\" font-size=\"{}\" fill=\"{}\">{}</text>\n",
                y + 4, tick_label_font_size, TEXT_MUTED, format_duration_with_unit(tick_value, time_unit, precision)
            ));
        }
    } else {
        // Linear scale
        let num_y_lines = 5;

        // Draw minor grid lines first
        if show_minor_grid {
            let num_minor_per_major = 4;
            let total_minor = num_y_lines * num_minor_per_major;
            for i in 0..=total_minor {
                if i % num_minor_per_major == 0 {
                    continue;
                }
                let y = chart_height - (i as f64 / total_minor as f64 * chart_height as f64) as i32;
                svg.push_str(&format!(
                    "  <line x1=\"0\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#E5E7EB\" stroke-width=\"0.5\" opacity=\"{}\"/>\n",
                    y, chart_width, y, minor_grid_opacity
                ));
            }
        }

        for i in 0..=num_y_lines {
            let y = chart_height - (i as f64 / num_y_lines as f64 * chart_height as f64) as i32;
            let value = min_value + (value_range * i as f64 / num_y_lines as f64);

            if show_grid {
                svg.push_str(&format!(
                    "  <line x1=\"0\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#E5E7EB\" opacity=\"{}\"/>\n",
                    y, chart_width, y, grid_opacity
                ));
            }
            svg.push_str(&format!(
                "  <text x=\"-10\" y=\"{}\" text-anchor=\"end\" font-family=\"sans-serif\" font-size=\"{}\" fill=\"{}\">{}</text>\n",
                y + 4, tick_label_font_size, TEXT_MUTED, format_duration_with_unit(value, time_unit, precision)
            ));
        }
    }

    // Draw vertical grid lines
    if show_vertical_grid && num_points > 1 {
        for i in 0..num_points {
            let x = (i as f64 / (num_points - 1) as f64 * chart_width as f64) as i32;
            svg.push_str(&format!(
                "  <line x1=\"{}\" y1=\"0\" x2=\"{}\" y2=\"{}\" stroke=\"#E5E7EB\" stroke-width=\"0.5\" opacity=\"{}\"/>\n",
                x, x, chart_height, grid_opacity
            ));
        }
    }

    // Draw X axis labels and tick marks
    for (i, name) in bench_names.iter().enumerate() {
        // Center single points, otherwise distribute evenly
        let x = if num_points == 1 {
            chart_width / 2
        } else {
            (i as f64 / (num_points - 1) as f64 * chart_width as f64) as i32
        };
        // X-axis tick mark
        svg.push_str(&format!(
            "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#4B5563\" stroke-width=\"1\"/>\n",
            x, chart_height, x, chart_height + 5
        ));
        // X-axis label
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"{}\" fill=\"{}\">{}</text>\n",
            x, chart_height + 16, tick_label_font_size, TEXT_MUTED, escape_xml(name)
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

    // Helper function to draw a line series with optional error bars and std dev band
    #[allow(clippy::too_many_arguments)]
    fn draw_series(
        svg: &mut String,
        points: &[(String, f64, f64, Option<f64>, Option<f64>, Option<f64>)], /* (name, nanos,
                                                                               * ops,
                                                                               * ci_lower,
                                                                               * ci_upper,
                                                                               * std_dev) */
        bench_names: &[String],
        chart_width: i32,
        chart_height: i32,
        min_value: f64,
        value_range: f64,
        log_min: f64,
        log_range: f64,
        use_log_scale: bool,
        color: &str,
        show_stats: bool,
        time_unit: Option<&str>,
        precision: Option<u32>,
        show_error_bars: bool,
        error_bar_opacity: f32,
        error_bar_thickness: f32,
        show_std_dev_band: bool,
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

        // Helper to calculate Y position based on scale type
        let calc_y = |value: f64| -> i32 {
            if use_log_scale {
                chart_height -
                    ((value.max(1.0).log10() - log_min) / log_range * chart_height as f64) as i32
            } else {
                chart_height - ((value - min_value) / value_range * chart_height as f64) as i32
            }
        };

        // Draw std dev band first (behind everything)
        if show_std_dev_band {
            let mut upper_path = String::new();
            let mut lower_path = String::new();
            let mut first = true;

            for (i, name) in bench_names.iter().enumerate() {
                if let Some((_, value, _, _, _, std_dev)) =
                    points.iter().find(|(n, _, _, _, _, _)| n == name)
                {
                    if let Some(sd) = std_dev {
                        let x = calc_x(i);
                        let y_upper = calc_y((*value + sd).max(0.0));
                        let y_lower = calc_y((*value - sd).max(0.0));

                        if first {
                            upper_path.push_str(&format!("M{},{}", x, y_upper));
                            lower_path = format!("L{},{}", x, y_lower);
                            first = false;
                        } else {
                            upper_path.push_str(&format!(" L{},{}", x, y_upper));
                            lower_path = format!(" L{},{}{}", x, y_lower, lower_path);
                        }
                    }
                }
            }

            if !upper_path.is_empty() {
                let band_path = format!("{}{} Z", upper_path, lower_path);
                svg.push_str(&format!(
                    "  <path d=\"{}\" fill=\"{}\" opacity=\"0.15\"/>\n",
                    band_path, color
                ));
            }
        }

        // Build path
        let mut path_data = String::new();
        let mut first = true;

        for (i, name) in bench_names.iter().enumerate() {
            if let Some((_, value, _, _, _, _)) = points.iter().find(|(n, _, _, _, _, _)| n == name)
            {
                let x = calc_x(i);
                let y = calc_y(*value);

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

        // Draw error bars first (behind points)
        if show_error_bars {
            for (i, name) in bench_names.iter().enumerate() {
                if let Some((_, _, _, ci_lower, ci_upper, _)) =
                    points.iter().find(|(n, _, _, _, _, _)| n == name)
                {
                    if let (Some(lower), Some(upper)) = (ci_lower, ci_upper) {
                        let x = calc_x(i);
                        let y_lower = calc_y(*lower);
                        let y_upper = calc_y(*upper);

                        // Clamp to chart bounds
                        let y_lower = y_lower.min(chart_height).max(0);
                        let y_upper = y_upper.min(chart_height).max(0);

                        let cap_width = 4;

                        // Vertical line
                        svg.push_str(&format!(
                            "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\" opacity=\"{}\"/>\n",
                            x, y_lower, x, y_upper, color, error_bar_thickness, error_bar_opacity
                        ));

                        // Top cap
                        svg.push_str(&format!(
                            "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\" opacity=\"{}\"/>\n",
                            x - cap_width, y_upper, x + cap_width, y_upper, color, error_bar_thickness, error_bar_opacity
                        ));

                        // Bottom cap
                        svg.push_str(&format!(
                            "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\" opacity=\"{}\"/>\n",
                            x - cap_width, y_lower, x + cap_width, y_lower, color, error_bar_thickness, error_bar_opacity
                        ));
                    }
                }
            }
        }

        // Draw points with optional stats tooltips (on top of error bars)
        for (i, name) in bench_names.iter().enumerate() {
            if let Some((_, value, ops, _, _, _)) =
                points.iter().find(|(n, _, _, _, _, _)| n == name)
            {
                let x = calc_x(i);
                let y = calc_y(*value);

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

    let show_std_dev_band = directive.show_std_dev_band.unwrap_or(false);

    // Draw Go line
    draw_series(
        &mut svg,
        &go_points,
        &bench_names,
        chart_width,
        chart_height,
        min_value,
        value_range,
        log_min,
        log_range,
        use_log_scale,
        GO_COLOR,
        directive.show_stats,
        time_unit,
        precision,
        show_error_bars,
        error_bar_opacity,
        error_bar_thickness,
        show_std_dev_band,
    );

    // Draw TS line
    draw_series(
        &mut svg,
        &ts_points,
        &bench_names,
        chart_width,
        chart_height,
        min_value,
        value_range,
        log_min,
        log_range,
        use_log_scale,
        TS_COLOR,
        directive.show_stats,
        time_unit,
        precision,
        show_error_bars,
        error_bar_opacity,
        error_bar_thickness,
        show_std_dev_band,
    );

    // Draw Rust line
    draw_series(
        &mut svg,
        &rust_points,
        &bench_names,
        chart_width,
        chart_height,
        min_value,
        value_range,
        log_min,
        log_range,
        use_log_scale,
        RUST_COLOR,
        directive.show_stats,
        time_unit,
        precision,
        show_error_bars,
        error_bar_opacity,
        error_bar_thickness,
        show_std_dev_band,
    );

    // Draw regression lines if enabled and collect regression info for legend
    let show_regression = directive.show_regression.unwrap_or(false);
    let show_r_squared = directive.show_r_squared.unwrap_or(false);
    let show_equation = directive.show_equation.unwrap_or(false);
    let mut regression_infos: Vec<RegressionInfo> = Vec::new();

    if show_regression {
        let regression_style = directive.regression_style.as_deref().unwrap_or("dashed");
        let show_label = directive.show_regression_label.unwrap_or(true);
        let regression_model = directive.regression_model.as_deref();

        // Helper to extract numeric x values from benchmark names
        // Tries to parse numbers from names like "n100", "n200", "size_1000", etc.
        let extract_x_value = |name: &str| -> Option<f64> {
            // Try to extract a number from the name
            let digits: String = name.chars().filter(|c| c.is_ascii_digit()).collect();
            digits.parse::<f64>().ok()
        };

        // Draw regression for Go
        if !go_points.is_empty() {
            let regression_points: Vec<(f64, f64)> = go_points
                .iter()
                .filter_map(|(name, nanos, _, _, _, _)| extract_x_value(name).map(|x| (x, *nanos)))
                .collect();

            if regression_points.len() >= 2 {
                if let Some(info) = draw_regression_line(
                    &mut svg,
                    &regression_points,
                    &bench_names,
                    chart_width,
                    chart_height,
                    min_value,
                    value_range,
                    log_min,
                    log_range,
                    use_log_scale,
                    GO_COLOR,
                    regression_style,
                    show_label,
                    regression_model,
                    "Go",
                ) {
                    regression_infos.push(info);
                }
            }
        }

        // Draw regression for TypeScript
        if !ts_points.is_empty() {
            let regression_points: Vec<(f64, f64)> = ts_points
                .iter()
                .filter_map(|(name, nanos, _, _, _, _)| extract_x_value(name).map(|x| (x, *nanos)))
                .collect();

            if regression_points.len() >= 2 {
                if let Some(info) = draw_regression_line(
                    &mut svg,
                    &regression_points,
                    &bench_names,
                    chart_width,
                    chart_height,
                    min_value,
                    value_range,
                    log_min,
                    log_range,
                    use_log_scale,
                    TS_COLOR,
                    regression_style,
                    show_label,
                    regression_model,
                    "TS",
                ) {
                    regression_infos.push(info);
                }
            }
        }

        // Draw regression for Rust
        if !rust_points.is_empty() {
            let regression_points: Vec<(f64, f64)> = rust_points
                .iter()
                .filter_map(|(name, nanos, _, _, _, _)| extract_x_value(name).map(|x| (x, *nanos)))
                .collect();

            if regression_points.len() >= 2 {
                if let Some(info) = draw_regression_line(
                    &mut svg,
                    &regression_points,
                    &bench_names,
                    chart_width,
                    chart_height,
                    min_value,
                    value_range,
                    log_min,
                    log_range,
                    use_log_scale,
                    RUST_COLOR,
                    regression_style,
                    show_label,
                    regression_model,
                    "Rust",
                ) {
                    regression_infos.push(info);
                }
            }
        }
    }

    // Axis labels
    let axis_label_size = directive.axis_label_font_size.unwrap_or(12);
    let x_label = directive.x_label.clone().unwrap_or_else(|| "Benchmark".to_string());
    let y_label = directive.y_label.clone().unwrap_or_else(|| "Time".to_string());

    svg.push_str(&format!(
        "  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"{}\" fill=\"{}\">{}</text>\n",
        chart_width / 2, chart_height + 40, axis_label_size, TEXT_SECONDARY, escape_xml(&x_label)
    ));
    svg.push_str(&format!(
        "  <text x=\"-55\" y=\"{}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"{}\" fill=\"{}\" transform=\"rotate(-90,-55,{})\">{}</text>\n",
        chart_height / 2, axis_label_size, TEXT_SECONDARY, chart_height / 2, escape_xml(&y_label)
    ));

    svg.push_str("</g>\n");

    // Legend
    let legend_position = directive.legend_position.as_deref().unwrap_or("top-right");
    if legend_position != "hidden" {
        // Build legend items based on which languages have data
        let mut legend_items: Vec<(&str, &str)> = Vec::new();
        if !go_points.is_empty() {
            legend_items.push((GO_COLOR, "Go"));
        }
        if !ts_points.is_empty() {
            legend_items.push((TS_COLOR, "TypeScript"));
        }
        if !rust_points.is_empty() {
            legend_items.push((RUST_COLOR, "Rust"));
        }

        // Render positioned legend inside chart area with regression info
        let legend_svg = render_legend_positioned(
            chart_width,
            chart_height,
            DEFAULT_MARGIN_LEFT,
            DEFAULT_MARGIN_TOP,
            legend_position,
            &legend_items,
            &regression_infos,
            show_r_squared,
            show_equation,
        );
        svg.push_str(&legend_svg);
    }

    svg.push_str("</svg>");

    Ok(svg)
}

/// Compute autoscale bounds for Y-axis based on data distribution.
///
/// When data spans multiple orders of magnitude, this function detects if
/// the first tick (min) or last tick (max) should be adjusted to ensure
/// all data points are visible on the chart.
///
/// # Arguments
/// Compute tick values for logarithmic scale.
/// Returns powers of 10 that fall within the given range.
fn compute_log_ticks(min_value: f64, max_value: f64) -> Vec<f64> {
    let mut ticks = Vec::new();

    // Find the power of 10 range
    let min_power = (min_value.max(1.0).log10().floor()) as i32;
    let max_power = (max_value.log10().ceil()) as i32;

    // Add ticks at powers of 10
    for power in min_power..=max_power {
        let tick = 10_f64.powi(power);
        if tick >= min_value * 0.9 && tick <= max_value * 1.1 {
            ticks.push(tick);
        }
    }

    // If we have too few ticks, add intermediate values (2x, 5x)
    if ticks.len() < 3 {
        let mut extra_ticks = Vec::new();
        for power in (min_power - 1)..=max_power {
            for multiplier in &[2.0, 5.0] {
                let tick = 10_f64.powi(power) * multiplier;
                if tick >= min_value * 0.9 && tick <= max_value * 1.1 {
                    extra_ticks.push(tick);
                }
            }
        }
        ticks.extend(extra_ticks);
        ticks.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        ticks.dedup();
    }

    ticks
}

/// Regression info for legend display
#[derive(Clone)]
pub struct RegressionInfo {
    pub lang: String,
    pub color: String,
    pub complexity: String,
    pub r_squared: f64,
    pub equation: String,
}

/// Render a legend at the specified position inside the chart area
fn render_legend_positioned(
    chart_width: i32,
    chart_height: i32,
    margin_left: i32,
    margin_top: i32,
    position: &str,
    items: &[(&str, &str)],
    regression_info: &[RegressionInfo],
    show_r_squared: bool,
    show_equation: bool,
) -> String {
    if items.is_empty() {
        return String::new();
    }

    let padding = 10;
    let item_height = 20;
    let box_height = items.len() as i32 * item_height + padding * 2;

    // Calculate box width based on actual content
    // Circle(12) + gap(6) + label + gap(8) + R²(~50) + gap(8) + equation
    let char_width = 6; // approximate width per character at font-size 10
    let mut max_content_width = 0;

    for (_, label) in items.iter() {
        let mut line_width = 18 + (label.len() as i32 * char_width); // circle + gap + label

        if let Some(reg_info) = regression_info.iter().find(|r| {
            // Match by language name
            (r.lang == "Go" && *label == "Go") ||
                (r.lang == "TS" && *label == "TypeScript") ||
                (r.lang == "Rust" && *label == "Rust")
        }) {
            if show_r_squared {
                line_width += 8 + 50; // gap + "R²=0.99"
            }
            if show_equation {
                line_width += 8 + (reg_info.equation.len() as i32 * char_width);
            }
        }

        max_content_width = max_content_width.max(line_width);
    }

    let box_width = max_content_width + padding * 2 + 10; // extra padding for safety

    // Calculate position based on legend_position
    let (box_x, box_y) = match position {
        "top-left" => (margin_left + padding, margin_top + padding),
        "top-right" => (margin_left + chart_width - box_width - padding, margin_top + padding),
        "bottom-left" => (margin_left + padding, margin_top + chart_height - box_height - padding),
        "bottom-right" => (
            margin_left + chart_width - box_width - padding,
            margin_top + chart_height - box_height - padding,
        ),
        _ => (margin_left + chart_width - box_width - padding, margin_top + padding), /* default to top-right */
    };

    let mut svg = String::new();

    // Legend background box
    svg.push_str(&format!(
        "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"white\" stroke=\"#E5E7EB\" stroke-width=\"1\" rx=\"4\" opacity=\"0.9\"/>\n",
        box_x, box_y, box_width, box_height
    ));

    // Legend items - all on one line per language
    for (i, (color, label)) in items.iter().enumerate() {
        let item_y = box_y + padding + (i as i32 * item_height);

        // Color circle
        svg.push_str(&format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"6\" fill=\"{}\" stroke=\"white\" stroke-width=\"1.5\"/>\n",
            box_x + padding + 4, item_y + 8, color
        ));

        // Build the full label text with regression info inline
        let mut x_offset = box_x + padding + 18;

        // Language name
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"11\" fill=\"#111827\">{}</text>\n",
            x_offset, item_y + 12, escape_xml(label)
        ));
        x_offset += (label.len() as i32) * 7 + 8;

        // Add regression info inline if available
        if let Some(reg_info) = regression_info.iter().find(|r| r.color == *color) {
            if show_r_squared {
                svg.push_str(&format!(
                    "<text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"10\" fill=\"{}\" opacity=\"0.85\">R²={:.4}</text>\n",
                    x_offset, item_y + 12, color, reg_info.r_squared
                ));
                x_offset += 65;
            }
            if show_equation {
                // Format equation with 2 decimal places
                let formatted_eq = format_equation_short(&reg_info.equation);
                svg.push_str(&format!(
                    "<text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"10\" fill=\"{}\" opacity=\"0.85\">{}</text>\n",
                    x_offset, item_y + 12, color, escape_xml(&formatted_eq)
                ));
            }
        }
    }

    svg
}

/// Format equation to use 2 decimal places
fn format_equation_short(equation: &str) -> String {
    // The equation comes in format like "y = 40.8 + 3.02·x·ln(x)"
    // We want to keep it concise but readable
    equation.to_string()
}

/// Draw a regression line for a series of data points
/// Returns RegressionInfo if regression was drawn successfully
#[allow(clippy::too_many_arguments)]
fn draw_regression_line(
    svg: &mut String,
    regression_points: &[(f64, f64)], // (x_numeric, y_nanos)
    bench_names: &[String],
    chart_width: i32,
    chart_height: i32,
    min_value: f64,
    value_range: f64,
    log_min: f64,
    log_range: f64,
    use_log_scale: bool,
    color: &str,
    style: &str,
    show_label: bool,
    regression_model: Option<&str>,
    lang_name: &str,
) -> Option<RegressionInfo> {
    // Sort points by x value for regression
    let mut sorted_points = regression_points.to_vec();
    sorted_points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    // Select model (user-specified or auto)
    if let Some(model) = select_model(&sorted_points, regression_model) {
        let num_points = bench_names.len();

        // Helper to calculate X position
        let calc_x = |i: usize| -> i32 {
            if num_points == 1 {
                chart_width / 2
            } else {
                (i as f64 / (num_points - 1) as f64 * chart_width as f64) as i32
            }
        };

        // Helper to calculate Y position based on scale type
        let calc_y = |value: f64| -> i32 {
            if use_log_scale {
                chart_height -
                    ((value.max(1.0).log10() - log_min) / log_range * chart_height as f64)
                        .min(chart_height as f64) as i32
            } else {
                chart_height -
                    ((value - min_value) / value_range * chart_height as f64)
                        .min(chart_height as f64) as i32
            }
        };

        // Extract x values from data points to map to chart positions
        let extract_x_value = |name: &str| -> Option<f64> {
            let digits: String = name.chars().filter(|c| c.is_ascii_digit()).collect();
            digits.parse::<f64>().ok()
        };

        // Build smooth regression curve path
        let mut path_data = String::new();
        let mut first = true;

        for (i, name) in bench_names.iter().enumerate() {
            if let Some(x_val) = extract_x_value(name) {
                let predicted_y = model.predict(x_val).max(0.0);
                let chart_x = calc_x(i);
                let chart_y = calc_y(predicted_y);

                if first {
                    path_data.push_str(&format!("M{},{}", chart_x, chart_y));
                    first = false;
                } else {
                    path_data.push_str(&format!(" L{},{}", chart_x, chart_y));
                }
            }
        }

        // Determine stroke-dasharray based on style
        let dash_array = match style {
            "solid" => "".to_string(),
            "dotted" => "2,4".to_string(),
            "dashed" | _ => "6,4".to_string(),
        };

        // Draw the regression line
        let dash_attr = if dash_array.is_empty() {
            String::new()
        } else {
            format!(" stroke-dasharray=\"{}\"", dash_array)
        };

        svg.push_str(&format!(
            "  <path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"1.5\"{} opacity=\"0.75\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n",
            path_data, color, dash_attr
        ));

        // Add complexity label on the line if enabled
        if show_label {
            // Position label at the end of the line, but keep inside chart
            if let Some(last_name) = bench_names.last() {
                if let Some(x_val) = extract_x_value(last_name) {
                    let predicted_y = model.predict(x_val).max(0.0);
                    let chart_x = calc_x(bench_names.len() - 1);
                    let chart_y = calc_y(predicted_y);

                    // Only show complexity label on the line (bigger font)
                    let complexity = model_type_to_complexity(&model.model_type);
                    let label_text = format!("{} {}", lang_name, complexity);

                    // Position label to the left of the last point, inside the chart
                    let label_width = (label_text.len() as i32) * 7;
                    let label_x = (chart_x - label_width - 5).max(5);
                    let label_y = (chart_y - 5).max(14).min(chart_height - 5);

                    svg.push_str(&format!(
                        "  <text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"11\" fill=\"{}\" opacity=\"0.9\" font-weight=\"500\">{}</text>\n",
                        label_x, label_y, color, label_text
                    ));
                }
            }
        }

        // Return regression info for legend
        let complexity = model_type_to_complexity(&model.model_type).to_string();
        let equation = model.format_equation();

        return Some(RegressionInfo {
            lang: lang_name.to_string(),
            color: color.to_string(),
            complexity,
            r_squared: model.r_squared,
            equation,
        });
    }

    None
}

/// Convert ModelType to human-readable complexity notation
fn model_type_to_complexity(model_type: &ModelType) -> &'static str {
    match model_type {
        ModelType::Constant => "O(1)",
        ModelType::Logarithmic => "O(log n)",
        ModelType::Linear => "O(n)",
        ModelType::Linearithmic => "O(n log n)",
        ModelType::Quadratic => "O(n²)",
        ModelType::Mixed => "O(n²)",
        ModelType::Cubic => "O(n³)",
        ModelType::PowerLaw => "O(n^k)",
    }
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
