//! Bar chart generator - vertical grouped bar chart with Go vs TS vs Rust comparison
//! Each benchmark group is displayed as vertical bars side-by-side for each language
//! All benchmarks in a suite are shown on one chart

use miette::Result;
use poly_bench_dsl::Lang;
use poly_bench_executor::BenchmarkResults;
use poly_bench_ir::ChartDirectiveIR;

use super::{
    compute_ci_bounds, escape_xml, extract_numeric_value, filter_benchmarks,
    format_duration_with_unit,
    regression::{select_model, ModelType},
    sort_benchmarks, svg_header, DEFAULT_MARGIN_TOP, GO_COLOR, RUST_COLOR, TEXT_COLOR, TEXT_MUTED,
    TEXT_SECONDARY, TS_COLOR,
};

// Layout constants for vertical bar chart
const DEFAULT_BAR_WIDTH: i32 = 20; // Width of each bar
const DEFAULT_BAR_GROUP_GAP: i32 = 20; // Gap between benchmark groups
const DEFAULT_BAR_WITHIN_GAP: i32 = 2; // Gap between bars within a group
const DEFAULT_CHART_HEIGHT: i32 = 445; // Default chart height
const MARGIN_LEFT: i32 = 80; // Left margin for Y-axis labels
const MARGIN_RIGHT: i32 = 40; // Right margin
const MARGIN_BOTTOM: i32 = 70; // Bottom margin for X-axis labels

/// Generate a vertical grouped bar chart SVG
/// Each benchmark is a group with bars for Go, TS, Rust side-by-side
pub fn generate(results: &BenchmarkResults, directive: &ChartDirectiveIR) -> Result<String> {
    let mut svg = String::new();

    // Collect all benchmarks with at least one measurement
    let all_benchmarks: Vec<_> = results
        .suites
        .iter()
        .flat_map(|s| s.benchmarks.iter())
        .filter(|b| !b.measurements.is_empty())
        .collect();

    if all_benchmarks.is_empty() {
        return Ok("<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>".to_string());
    }

    // Apply filtering
    let mut filtered = filter_benchmarks(all_benchmarks, directive);

    // Apply sorting
    sort_benchmarks(&mut filtered, directive);

    if filtered.is_empty() {
        return Ok("<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>".to_string());
    }

    // Extract directive parameters with defaults
    let time_unit = directive.time_unit.as_deref();
    let precision = directive.precision;
    let bar_group_gap = directive.bar_group_gap.unwrap_or(DEFAULT_BAR_GROUP_GAP);
    let bar_within_gap = directive.bar_within_group_gap.unwrap_or(DEFAULT_BAR_WITHIN_GAP);
    let round_ticks = directive.round_ticks.unwrap_or(false);

    // Typography settings
    let title_font_size = directive.title_font_size.unwrap_or(16);
    let subtitle_font_size = directive.subtitle_font_size.unwrap_or(12);
    let axis_label_font_size = directive.axis_label_font_size.unwrap_or(12);
    let tick_label_font_size = directive.tick_label_font_size.unwrap_or(11);

    // Grid settings
    let show_grid = directive.show_grid.unwrap_or(true);
    let grid_opacity = directive.grid_opacity.unwrap_or(1.0);
    let show_minor_grid = directive.show_minor_grid.unwrap_or(false);
    let minor_grid_opacity = directive.minor_grid_opacity.unwrap_or(0.8);
    let show_vertical_grid = directive.show_vertical_grid.unwrap_or(false);

    // Axis settings
    let axis_thickness = directive.axis_thickness.unwrap_or(1.5);

    // Error bar settings
    let show_error_bars = directive.show_error_bars.unwrap_or(false);
    let error_bar_opacity = directive.error_bar_opacity.unwrap_or(0.6);
    let error_bar_thickness = directive.error_bar_thickness.unwrap_or(1.5);
    let ci_level = directive.ci_level.unwrap_or(95);

    // Regression settings
    let show_regression = directive.show_regression.unwrap_or(false);
    let regression_style = directive.regression_style.as_deref().unwrap_or("dashed");
    let show_regression_label = directive.show_regression_label.unwrap_or(true);
    let show_r_squared = directive.show_r_squared.unwrap_or(false);

    // Determine which languages have data
    let has_go = filtered.iter().any(|b| b.measurements.contains_key(&Lang::Go));
    let has_ts = filtered.iter().any(|b| b.measurements.contains_key(&Lang::TypeScript));
    let has_rust = filtered.iter().any(|b| b.measurements.contains_key(&Lang::Rust));

    let num_langs = [has_go, has_ts, has_rust].iter().filter(|&&x| x).count() as i32;
    let num_benchmarks = filtered.len() as i32;

    // Calculate dimensions
    let bar_width = directive.bar_width.unwrap_or(DEFAULT_BAR_WIDTH);
    let group_width = num_langs * bar_width + (num_langs - 1).max(0) * bar_within_gap;
    let total_groups_width =
        num_benchmarks * group_width + (num_benchmarks - 1).max(0) * bar_group_gap;

    // Chart dimensions - calculate width needed to fit all bars
    // Width = MARGIN_LEFT + total_groups_width + MARGIN_RIGHT + padding
    let dynamic_width = total_groups_width + MARGIN_LEFT + MARGIN_RIGHT + 60;

    // Use user-specified width if provided (ensuring it's at least the dynamic minimum),
    // otherwise use the dynamically calculated width based on bar count
    let chart_width = directive
        .width
        .map(|w| w.max(dynamic_width)) // If user specified width, ensure it's at least min
        .unwrap_or(dynamic_width); // Otherwise use dynamic width based on bar count

    let chart_height = directive.height.unwrap_or(DEFAULT_CHART_HEIGHT);
    let plot_height = chart_height - DEFAULT_MARGIN_TOP - MARGIN_BOTTOM;
    let plot_width = chart_width - MARGIN_LEFT - MARGIN_RIGHT;

    // Collect all values for Y scale
    let all_values: Vec<f64> = filtered
        .iter()
        .flat_map(|b| {
            let mut values = Vec::new();
            if let Some(m) = b.measurements.get(&Lang::Go) {
                values.push(m.nanos_per_op);
                // Include CI upper bound if error bars are enabled
                if show_error_bars {
                    if let Some(ci_upper) = m.ci_95_upper {
                        values.push(ci_upper);
                    }
                }
            }
            if let Some(m) = b.measurements.get(&Lang::TypeScript) {
                values.push(m.nanos_per_op);
                if show_error_bars {
                    if let Some(ci_upper) = m.ci_95_upper {
                        values.push(ci_upper);
                    }
                }
            }
            if let Some(m) = b.measurements.get(&Lang::Rust) {
                values.push(m.nanos_per_op);
                if show_error_bars {
                    if let Some(ci_upper) = m.ci_95_upper {
                        values.push(ci_upper);
                    }
                }
            }
            values
        })
        .collect();

    // Determine Y-axis scale type
    let use_log_scale = directive.y_scale.as_deref() == Some("log");

    // Calculate min/max values
    let raw_max = all_values.iter().cloned().fold(1.0, f64::max);
    let raw_min = all_values.iter().cloned().fold(raw_max, f64::min).max(1.0); // Clamp to 1.0 for log scale safety

    let max_value = directive.y_axis_max.unwrap_or(raw_max * 1.15);
    let min_value = if use_log_scale {
        // For log scale, use the minimum positive value (or 1.0)
        directive.y_axis_min.unwrap_or(raw_min * 0.5).max(1.0)
    } else {
        directive.y_axis_min.unwrap_or(0.0)
    };

    // For linear scale, use value_range; for log scale, use log_range
    let value_range = (max_value - min_value).max(1.0);
    let log_min = min_value.max(1.0).log10();
    let log_max = max_value.log10();
    let log_range = (log_max - log_min).max(0.001);

    // Build title and subtitle
    let title = directive.title.clone().unwrap_or_else(|| "Benchmark Results".to_string());
    let subtitle = directive.description.clone();

    // SVG header
    svg.push_str(&svg_header(chart_width, chart_height));

    // Title
    svg.push_str(&svg_title_custom(
        chart_width,
        &title,
        subtitle.as_deref(),
        title_font_size,
        subtitle_font_size,
    ));

    // Chart group - translate to plot area
    let plot_x = MARGIN_LEFT;
    let plot_y = DEFAULT_MARGIN_TOP;
    svg.push_str(&format!("<g transform=\"translate({},{})\">\n", plot_x, plot_y));

    // Calculate bar positions - start at origin (left edge of plot area)
    let start_x = 0;

    // Calculate the actual X-axis end position (end of last bar group)
    let x_axis_end = if num_benchmarks > 0 {
        start_x + (num_benchmarks - 1) * (group_width + bar_group_gap) + group_width
    } else {
        plot_width
    };

    // Draw grid lines and Y-axis ticks
    if use_log_scale {
        // Log scale: draw grid lines at powers of 10
        let tick_values = compute_log_ticks(min_value, max_value);

        // Draw minor grid lines first (so major lines draw on top)
        if show_minor_grid && tick_values.len() >= 2 {
            for window in tick_values.windows(2) {
                let lower = window[0];
                let upper = window[1];
                // Draw minor lines at 2, 3, 4, 5, 6, 7, 8, 9 times the lower power of 10
                for mult in 2..=9 {
                    let minor_value = lower * mult as f64;
                    if minor_value > upper {
                        break;
                    }
                    let y = plot_height -
                        ((minor_value.log10() - log_min) / log_range * plot_height as f64) as i32;
                    svg.push_str(&format!(
                        "  <line x1=\"0\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#E5E7EB\" stroke-width=\"0.5\" opacity=\"{}\"/>\n",
                        y, x_axis_end, y, minor_grid_opacity
                    ));
                }
            }
        }

        for &tick_value in &tick_values {
            let y = plot_height -
                ((tick_value.log10() - log_min) / log_range * plot_height as f64) as i32;

            // Grid line
            if show_grid {
                svg.push_str(&format!(
                    "  <line x1=\"0\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#E5E7EB\" stroke-width=\"1\" opacity=\"{}\"/>\n",
                    y, x_axis_end, y, grid_opacity
                ));
            }

            // Tick mark
            svg.push_str(&format!(
                "  <line x1=\"-5\" y1=\"{}\" x2=\"0\" y2=\"{}\" stroke=\"#4B5563\" stroke-width=\"{}\"/>\n",
                y, y, axis_thickness
            ));

            // Label
            let label = if round_ticks {
                format_duration_rounded(tick_value, time_unit)
            } else {
                format_duration_with_unit(tick_value, time_unit, precision)
            };
            svg.push_str(&format!(
                "  <text x=\"-10\" y=\"{}\" text-anchor=\"end\" font-family=\"sans-serif\" font-size=\"{}\" fill=\"{}\">{}</text>\n",
                y + 4, tick_label_font_size, TEXT_MUTED, label
            ));
        }
    } else {
        // Linear scale: draw grid lines at regular intervals
        let num_grid_lines = 5;

        // Draw minor grid lines first (between major lines)
        if show_minor_grid {
            let num_minor_per_major = 4; // 4 minor lines between each major line
            let total_minor = num_grid_lines * num_minor_per_major;
            for i in 0..=total_minor {
                if i % num_minor_per_major == 0 {
                    continue; // Skip positions where major lines will be
                }
                let y = plot_height - (i as f64 / total_minor as f64 * plot_height as f64) as i32;
                svg.push_str(&format!(
                    "  <line x1=\"0\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#E5E7EB\" stroke-width=\"0.5\" opacity=\"{}\"/>\n",
                    y, x_axis_end, y, minor_grid_opacity
                ));
            }
        }

        // Draw major grid lines
        if show_grid {
            for i in 0..=num_grid_lines {
                let y =
                    plot_height - (i as f64 / num_grid_lines as f64 * plot_height as f64) as i32;
                svg.push_str(&format!(
                    "  <line x1=\"0\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#E5E7EB\" stroke-width=\"1\" opacity=\"{}\"/>\n",
                    y, x_axis_end, y, grid_opacity
                ));
            }
        }

        // Draw Y-axis with tick marks and labels
        let num_y_ticks = 5;
        for i in 0..=num_y_ticks {
            let y = plot_height - (i as f64 / num_y_ticks as f64 * plot_height as f64) as i32;
            let value = min_value + (value_range * i as f64 / num_y_ticks as f64);

            // Tick mark
            svg.push_str(&format!(
                "  <line x1=\"-5\" y1=\"{}\" x2=\"0\" y2=\"{}\" stroke=\"#4B5563\" stroke-width=\"{}\"/>\n",
                y, y, axis_thickness
            ));

            // Label - optionally round to whole numbers
            let label = if round_ticks {
                format_duration_rounded(value, time_unit)
            } else {
                format_duration_with_unit(value, time_unit, precision)
            };
            svg.push_str(&format!(
                "  <text x=\"-10\" y=\"{}\" text-anchor=\"end\" font-family=\"sans-serif\" font-size=\"{}\" fill=\"{}\">{}</text>\n",
                y + 4, tick_label_font_size, TEXT_MUTED, label
            ));
        }
    }

    // Draw vertical grid lines at each benchmark position
    if show_vertical_grid {
        for i in 0..num_benchmarks {
            let x = start_x + i as i32 * (group_width + bar_group_gap) + group_width / 2;
            svg.push_str(&format!(
                "  <line x1=\"{}\" y1=\"0\" x2=\"{}\" y2=\"{}\" stroke=\"#E5E7EB\" stroke-width=\"0.5\" opacity=\"{}\"/>\n",
                x, x, plot_height, grid_opacity
            ));
        }
    }

    // Draw axes
    svg.push_str(&format!(
        "  <line x1=\"0\" y1=\"0\" x2=\"0\" y2=\"{}\" stroke=\"#4B5563\" stroke-width=\"{}\"/>\n",
        plot_height, axis_thickness
    ));
    svg.push_str(&format!(
        "  <line x1=\"0\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#4B5563\" stroke-width=\"{}\"/>\n",
        plot_height, x_axis_end, plot_height, axis_thickness
    ));

    // Draw X-axis tick marks for each benchmark
    for i in 0..num_benchmarks {
        let tick_x = start_x + i as i32 * (group_width + bar_group_gap) + group_width / 2;
        svg.push_str(&format!(
            "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#4B5563\" stroke-width=\"{}\"/>\n",
            tick_x, plot_height, tick_x, plot_height + 5, axis_thickness
        ));
    }

    // Collect data for regression lines
    let mut go_data: Vec<(f64, f64)> = Vec::new();
    let mut ts_data: Vec<(f64, f64)> = Vec::new();
    let mut rust_data: Vec<(f64, f64)> = Vec::new();

    // Draw bars for each benchmark
    for (i, bench) in filtered.iter().enumerate() {
        let group_x = start_x + i as i32 * (group_width + bar_group_gap);
        let mut bar_offset = 0;

        // Extract numeric x value for regression
        let x_val = extract_numeric_value(&bench.name).unwrap_or(i as i64) as f64;

        // Go bar
        if has_go {
            if let Some(m) = bench.measurements.get(&Lang::Go) {
                let value = m.nanos_per_op;
                let bar_height = if use_log_scale {
                    ((value.max(1.0).log10() - log_min) / log_range * plot_height as f64).max(1.0)
                        as i32
                } else {
                    ((value - min_value) / value_range * plot_height as f64).max(1.0) as i32
                };
                let bar_y = plot_height - bar_height;
                let bar_x = group_x + bar_offset;

                svg.push_str(&format!(
                    "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#00ADD8\" rx=\"2\">\n",
                    bar_x, bar_y, bar_width, bar_height
                ));
                let time_str = format_duration_with_unit(value, time_unit, precision);
                svg.push_str(&format!("    <title>Go: {}</title>\n  </rect>\n", time_str));

                // Draw error bar if enabled
                if show_error_bars {
                    let (ci_lower, ci_upper) = compute_ci_bounds(
                        value,
                        m.raw_samples.as_ref(),
                        ci_level,
                        m.ci_95_lower,
                        m.ci_95_upper,
                    );
                    if let (Some(lower), Some(upper)) = (ci_lower, ci_upper) {
                        draw_error_bar_scaled(
                            &mut svg,
                            bar_x + bar_width / 2,
                            plot_height,
                            lower,
                            upper,
                            min_value,
                            value_range,
                            log_min,
                            log_range,
                            use_log_scale,
                            "#006080", // Darker Go color for error bars
                            error_bar_opacity,
                            error_bar_thickness,
                        );
                    }
                }

                go_data.push((x_val, value));
            }
            bar_offset += bar_width + bar_within_gap;
        }

        // TypeScript bar
        if has_ts {
            if let Some(m) = bench.measurements.get(&Lang::TypeScript) {
                let value = m.nanos_per_op;
                let bar_height = if use_log_scale {
                    ((value.max(1.0).log10() - log_min) / log_range * plot_height as f64).max(1.0)
                        as i32
                } else {
                    ((value - min_value) / value_range * plot_height as f64).max(1.0) as i32
                };
                let bar_y = plot_height - bar_height;
                let bar_x = group_x + bar_offset;

                svg.push_str(&format!(
                    "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#3178C6\" rx=\"2\">\n",
                    bar_x, bar_y, bar_width, bar_height
                ));
                let time_str = format_duration_with_unit(value, time_unit, precision);
                svg.push_str(&format!("    <title>TypeScript: {}</title>\n  </rect>\n", time_str));

                if show_error_bars {
                    let (ci_lower, ci_upper) = compute_ci_bounds(
                        value,
                        m.raw_samples.as_ref(),
                        ci_level,
                        m.ci_95_lower,
                        m.ci_95_upper,
                    );
                    if let (Some(lower), Some(upper)) = (ci_lower, ci_upper) {
                        draw_error_bar_scaled(
                            &mut svg,
                            bar_x + bar_width / 2,
                            plot_height,
                            lower,
                            upper,
                            min_value,
                            value_range,
                            log_min,
                            log_range,
                            use_log_scale,
                            "#1a4a80", // Darker TS color
                            error_bar_opacity,
                            error_bar_thickness,
                        );
                    }
                }

                ts_data.push((x_val, value));
            }
            bar_offset += bar_width + bar_within_gap;
        }

        // Rust bar
        if has_rust {
            if let Some(m) = bench.measurements.get(&Lang::Rust) {
                let value = m.nanos_per_op;
                let bar_height = if use_log_scale {
                    ((value.max(1.0).log10() - log_min) / log_range * plot_height as f64).max(1.0)
                        as i32
                } else {
                    ((value - min_value) / value_range * plot_height as f64).max(1.0) as i32
                };
                let bar_y = plot_height - bar_height;
                let bar_x = group_x + bar_offset;

                svg.push_str(&format!(
                    "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#DEA584\" rx=\"2\">\n",
                    bar_x, bar_y, bar_width, bar_height
                ));
                let time_str = format_duration_with_unit(value, time_unit, precision);
                svg.push_str(&format!("    <title>Rust: {}</title>\n  </rect>\n", time_str));

                if show_error_bars {
                    let (ci_lower, ci_upper) = compute_ci_bounds(
                        value,
                        m.raw_samples.as_ref(),
                        ci_level,
                        m.ci_95_lower,
                        m.ci_95_upper,
                    );
                    if let (Some(lower), Some(upper)) = (ci_lower, ci_upper) {
                        draw_error_bar_scaled(
                            &mut svg,
                            bar_x + bar_width / 2,
                            plot_height,
                            lower,
                            upper,
                            min_value,
                            value_range,
                            log_min,
                            log_range,
                            use_log_scale,
                            "#8B4513", // Darker Rust color
                            error_bar_opacity,
                            error_bar_thickness,
                        );
                    }
                }

                rust_data.push((x_val, value));
            }
        }

        // X-axis label (benchmark name) - positioned below bars
        let label_x = group_x + group_width / 2;
        let name_display = &bench.name;
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" text-anchor=\"end\" font-family=\"sans-serif\" font-size=\"{}\" fill=\"{}\" transform=\"rotate(-45,{},{})\">{}</text>\n",
            label_x, plot_height + 15, tick_label_font_size, TEXT_MUTED, label_x, plot_height + 15, escape_xml(name_display)
        ));
    }

    // Draw regression lines if enabled and collect regression info for legend
    let show_equation = directive.show_equation.unwrap_or(false);
    let mut regression_infos: Vec<RegressionInfo> = Vec::new();

    if show_regression && num_benchmarks >= 2 {
        let dash_array = match regression_style {
            "solid" => "",
            "dotted" => "2,2",
            _ => "6,4", // dashed default
        };
        let regression_model = directive.regression_model.as_deref();

        // Draw regression for each language
        if go_data.len() >= 2 {
            if let Some(info) = draw_regression_line_bar(
                &mut svg,
                &go_data,
                &filtered,
                start_x,
                group_width,
                bar_group_gap,
                plot_height,
                min_value,
                value_range,
                GO_COLOR,
                dash_array,
                show_regression_label,
                regression_model,
                "Go",
                plot_width,
            ) {
                regression_infos.push(info);
            }
        }

        if ts_data.len() >= 2 {
            if let Some(info) = draw_regression_line_bar(
                &mut svg,
                &ts_data,
                &filtered,
                start_x,
                group_width,
                bar_group_gap,
                plot_height,
                min_value,
                value_range,
                TS_COLOR,
                dash_array,
                show_regression_label,
                regression_model,
                "TS",
                plot_width,
            ) {
                regression_infos.push(info);
            }
        }

        if rust_data.len() >= 2 {
            if let Some(info) = draw_regression_line_bar(
                &mut svg,
                &rust_data,
                &filtered,
                start_x,
                group_width,
                bar_group_gap,
                plot_height,
                min_value,
                value_range,
                RUST_COLOR,
                dash_array,
                show_regression_label,
                regression_model,
                "Rust",
                plot_width,
            ) {
                regression_infos.push(info);
            }
        }
    }

    // Y-axis label
    let y_label = directive.y_label.clone().unwrap_or_else(|| "Time (ns/op)".to_string());
    svg.push_str(&format!(
        "  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"{}\" fill=\"{}\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        -55, plot_height / 2, axis_label_font_size, TEXT_SECONDARY, -55, plot_height / 2, escape_xml(&y_label)
    ));

    // X-axis label - centered on actual data area
    let x_label = directive.x_label.clone().unwrap_or_else(|| "Benchmark".to_string());
    svg.push_str(&format!(
        "  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"{}\" fill=\"{}\">{}</text>\n",
        x_axis_end / 2, plot_height + 55, axis_label_font_size, TEXT_SECONDARY, escape_xml(&x_label)
    ));

    svg.push_str("</g>\n");

    // Legend
    let legend_position = directive.legend_position.as_deref().unwrap_or("top-left");
    if legend_position != "hidden" {
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

        let legend_svg = render_legend_positioned(
            plot_width,
            plot_height,
            plot_x,
            plot_y,
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

/// Draw regression line for bar chart
/// Returns RegressionInfo if regression was drawn successfully
#[allow(clippy::too_many_arguments)]
fn draw_regression_line_bar(
    svg: &mut String,
    data: &[(f64, f64)],
    filtered: &[&poly_bench_executor::comparison::BenchmarkResult],
    start_x: i32,
    group_width: i32,
    bar_group_gap: i32,
    plot_height: i32,
    min_value: f64,
    value_range: f64,
    color: &str,
    dash_array: &str,
    show_label: bool,
    regression_model: Option<&str>,
    lang_name: &str,
    plot_width: i32,
) -> Option<RegressionInfo> {
    if data.len() < 2 {
        return None;
    }

    // Fit regression model (user-specified or auto)
    let model = match select_model(data, regression_model) {
        Some(m) => m,
        None => return None,
    };

    // Generate points along the regression curve at each bar position
    let mut path_points = Vec::new();

    for (i, _bench) in filtered.iter().enumerate() {
        let x_screen = start_x + i as i32 * (group_width + bar_group_gap) + group_width / 2;
        let x_val = extract_numeric_value(&filtered[i].name).unwrap_or(i as i64) as f64;
        let y_pred = model.predict(x_val);
        let y_screen =
            plot_height - ((y_pred - min_value) / value_range * plot_height as f64) as i32;
        let y_screen = y_screen.max(0).min(plot_height);
        path_points.push((x_screen, y_screen));
    }

    if path_points.is_empty() {
        return None;
    }

    // Build path
    let mut path = format!("M{},{}", path_points[0].0, path_points[0].1);
    for (x, y) in path_points.iter().skip(1) {
        path.push_str(&format!(" L{},{}", x, y));
    }

    // Draw the regression line
    let dash_attr = if dash_array.is_empty() {
        String::new()
    } else {
        format!(" stroke-dasharray=\"{}\"", dash_array)
    };

    svg.push_str(&format!(
        "  <path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"2\"{} opacity=\"0.7\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n",
        path, color, dash_attr
    ));

    // Add complexity label on the line if enabled (bigger font, no R² or equation)
    if show_label {
        let last_point = path_points.last().unwrap();

        // Position label to the left of the last point, inside the chart
        let complexity = model_type_to_complexity(&model.model_type);
        let label_text = format!("{} {}", lang_name, complexity);
        let label_width = (label_text.len() as i32) * 7;
        let label_x = (last_point.0 - label_width - 5).min(plot_width - label_width - 10);
        let label_y = (last_point.1 - 8).max(16);

        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"11\" fill=\"{}\" opacity=\"0.9\" font-weight=\"500\">{}</text>\n",
            label_x, label_y, color, label_text
        ));
    }

    // Return regression info for legend
    let complexity = model_type_to_complexity(&model.model_type).to_string();
    let equation = model.format_equation();

    Some(RegressionInfo {
        lang: lang_name.to_string(),
        color: color.to_string(),
        complexity,
        r_squared: model.r_squared,
        equation,
    })
}

/// Convert model type to complexity notation
fn model_type_to_complexity(model_type: &ModelType) -> &'static str {
    match model_type {
        ModelType::Constant => "O(1)",
        ModelType::Logarithmic => "O(log n)",
        ModelType::Linear => "O(n)",
        ModelType::Linearithmic => "O(n log n)",
        ModelType::Quadratic => "O(n²)",
        ModelType::Mixed => "O(n²)",
        ModelType::Cubic => "O(n³)",
        ModelType::PowerLaw => "O(nᵏ)",
    }
}

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

/// Draw an error bar with support for both linear and log scale
#[allow(clippy::too_many_arguments)]
fn draw_error_bar_scaled(
    svg: &mut String,
    x: i32,
    plot_height: i32,
    ci_lower: f64,
    ci_upper: f64,
    min_value: f64,
    value_range: f64,
    log_min: f64,
    log_range: f64,
    use_log_scale: bool,
    color: &str,
    opacity: f32,
    thickness: f32,
) {
    let (y_lower, y_upper) = if use_log_scale {
        let y_lower = plot_height -
            ((ci_lower.max(1.0).log10() - log_min) / log_range * plot_height as f64) as i32;
        let y_upper = plot_height -
            ((ci_upper.max(1.0).log10() - log_min) / log_range * plot_height as f64) as i32;
        (y_lower, y_upper)
    } else {
        let y_lower =
            plot_height - ((ci_lower - min_value) / value_range * plot_height as f64) as i32;
        let y_upper =
            plot_height - ((ci_upper - min_value) / value_range * plot_height as f64) as i32;
        (y_lower, y_upper)
    };

    // Clamp to plot area
    let y_lower = y_lower.min(plot_height).max(0);
    let y_upper = y_upper.min(plot_height).max(0);

    let cap_width = 5;

    // Vertical line
    svg.push_str(&format!(
        "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\" opacity=\"{}\"/>\n",
        x, y_lower, x, y_upper, color, thickness, opacity
    ));

    // Top cap
    svg.push_str(&format!(
        "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\" opacity=\"{}\"/>\n",
        x - cap_width, y_upper, x + cap_width, y_upper, color, thickness, opacity
    ));

    // Bottom cap
    svg.push_str(&format!(
        "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\" opacity=\"{}\"/>\n",
        x - cap_width, y_lower, x + cap_width, y_lower, color, thickness, opacity
    ));
}

/// Format duration with rounding to whole numbers
fn format_duration_rounded(nanos: f64, time_unit: Option<&str>) -> String {
    let (value, unit) = match time_unit {
        Some("ns") => (nanos, "ns"),
        Some("us") | Some("µs") => (nanos / 1_000.0, "µs"),
        Some("ms") => (nanos / 1_000_000.0, "ms"),
        Some("s") => (nanos / 1_000_000_000.0, "s"),
        _ => {
            // Auto-select unit
            if nanos >= 1_000_000_000.0 {
                (nanos / 1_000_000_000.0, "s")
            } else if nanos >= 1_000_000.0 {
                (nanos / 1_000_000.0, "ms")
            } else if nanos >= 1_000.0 {
                (nanos / 1_000.0, "µs")
            } else {
                (nanos, "ns")
            }
        }
    };

    // Round to whole number
    format!("{}{}", value.round() as i64, unit)
}

/// Custom title rendering with configurable font sizes
fn svg_title_custom(
    width: i32,
    title: &str,
    subtitle: Option<&str>,
    title_size: i32,
    subtitle_size: i32,
) -> String {
    let mut svg = format!(
        "<text x=\"{}\" y=\"30\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"{}\" font-weight=\"700\" fill=\"{}\">{}</text>\n",
        width / 2,
        title_size,
        TEXT_COLOR,
        escape_xml(title)
    );

    if let Some(sub) = subtitle {
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"{}\" fill=\"{}\">{}</text>\n",
            width / 2,
            30 + title_size,
            subtitle_size,
            TEXT_SECONDARY,
            escape_xml(sub)
        ));
    }

    svg
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

    let padding = 8;
    let item_height = 18;
    let box_height = items.len() as i32 * item_height + padding * 2;

    // Calculate box width based on actual content
    // Square(12) + gap(6) + label + gap(8) + R²(~50) + gap(8) + equation
    let char_width = 6; // approximate width per character at font-size 10
    let mut max_content_width = 0;

    for (_, label) in items.iter() {
        let mut line_width = 18 + (label.len() as i32 * char_width); // square + gap + label

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
        "top-left" => (margin_left + padding + 5, margin_top + padding),
        "top-right" => (margin_left + chart_width - box_width - padding, margin_top + padding),
        "bottom-left" => {
            (margin_left + padding + 5, margin_top + chart_height - box_height - padding - 20)
        }
        "bottom-right" => (
            margin_left + chart_width - box_width - padding,
            margin_top + chart_height - box_height - padding - 20,
        ),
        _ => (margin_left + padding + 5, margin_top + padding), // default to top-left
    };

    let mut svg = String::new();

    // Legend background box
    svg.push_str(&format!(
        "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"white\" stroke=\"#E5E7EB\" stroke-width=\"1\" rx=\"4\" opacity=\"0.95\"/>\n",
        box_x, box_y, box_width, box_height
    ));

    // Legend items - all on one line per language
    for (i, (color, label)) in items.iter().enumerate() {
        let item_y = box_y + padding + (i as i32 * item_height);

        // Color square
        svg.push_str(&format!(
            "<rect x=\"{}\" y=\"{}\" width=\"12\" height=\"12\" fill=\"{}\" rx=\"2\"/>\n",
            box_x + padding,
            item_y,
            color
        ));

        // Build the full label text with regression info inline
        let mut x_offset = box_x + padding + 18;

        // Language name
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"10\" fill=\"{}\">{}</text>\n",
            x_offset, item_y + 10, TEXT_COLOR, escape_xml(label)
        ));
        x_offset += (label.len() as i32) * 7 + 8;

        // Add regression info inline if available
        if let Some(reg_info) = regression_info.iter().find(|r| r.color == *color) {
            if show_r_squared {
                svg.push_str(&format!(
                    "<text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"10\" fill=\"{}\" opacity=\"0.85\">R²={:.4}</text>\n",
                    x_offset, item_y + 10, color, reg_info.r_squared
                ));
                x_offset += 65;
            }
            if show_equation {
                svg.push_str(&format!(
                    "<text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"10\" fill=\"{}\" opacity=\"0.85\">{}</text>\n",
                    x_offset, item_y + 10, color, escape_xml(&reg_info.equation)
                ));
            }
        }
    }

    svg
}

/// Generate a bar chart using the legacy signature (backwards compatibility)
pub fn generate_simple(
    results: &BenchmarkResults,
    title: &str,
    description: Option<&str>,
    _x_label: &str,
    _y_label: &str,
) -> Result<String> {
    use poly_bench_dsl::ChartType;

    let mut directive = ChartDirectiveIR::new(ChartType::BarChart, "bar-chart.svg".to_string());
    directive.title = Some(title.to_string());
    directive.description = description.map(|s| s.to_string());

    generate(results, &directive)
}

#[cfg(test)]
mod tests {
    use super::*;
    use poly_bench_dsl::ChartType;
    use poly_bench_executor::comparison::{BenchmarkResult, SuiteResults};
    use std::collections::HashMap;

    fn default_directive() -> ChartDirectiveIR {
        ChartDirectiveIR::new(ChartType::BarChart, "test.svg".to_string())
    }

    #[test]
    fn test_generate_empty() {
        let results = BenchmarkResults::new(vec![]);
        let svg = generate(&results, &default_directive()).unwrap();
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn test_generate_no_comparisons() {
        let benchmarks = vec![BenchmarkResult::new(
            "bench1".to_string(),
            "suite_bench1".to_string(),
            None,
            HashMap::new(),
        )];
        let suite = SuiteResults::new("suite".to_string(), None, benchmarks);
        let results = BenchmarkResults::new(vec![suite]);

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
