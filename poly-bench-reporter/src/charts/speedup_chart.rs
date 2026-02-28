//! Speedup chart generator - shows relative performance vs baseline language
//!
//! Generates bar charts showing relative performance vs baseline with support for
//! light and dark themes.

use poly_bench_dsl::{BenchmarkKind, Lang};
use poly_bench_executor::comparison::BenchmarkResult;
use poly_bench_ir::ChartDirectiveIR;
use poly_bench_runtime::{lang_color, lang_full_name, lang_gradient_id, lang_label};

use super::{escape_xml, filter_benchmarks, format_duration, sort_benchmarks, svg_gradient_defs};

// Bar dimensions
const DEFAULT_BAR_HEIGHT: i32 = 48;
const BAR_GAP: i32 = 8;
/// Multiplier for bar width (1.8 = bars 1.8x wider by default)
const SPEEDUP_BAR_WIDTH_FACTOR: f64 = 1.8;

// Margins and spacing
const MARGIN_TOP: i32 = 100;
const MARGIN_BOTTOM: i32 = 100; // Reduced to favour wider aspect ratio (more width vs height)
const MARGIN_LEFT: i32 = 50;
const MARGIN_RIGHT: i32 = 40;
const PLOT_PADDING: i32 = 12;
const GRID_GAP_X: i32 = 18;
const GRID_GAP_Y: i32 = 22;
const CARD_TITLE_HEIGHT: i32 = 22;
const MIN_CARD_WIDTH: i32 = 468; // 260 * 1.8 for wider bars
const MAX_GRID_COLUMNS: i32 = 4; // Allow more columns → fewer rows → taller aspect ratio
const TARGET_COMBINED_GRID_HEIGHT: i32 = 420;

// Accent colors (same for both themes)
const ACCENT_COLOR: &str = "#FF8A00";
const ACCENT_GLOW: &str = "#FFBA07";

// Font
const FONT_FAMILY: &str = "system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif";
const FONT_SIZE_TITLE: i32 = 34;
const FONT_SIZE_SUBTITLE: i32 = 24;
const FONT_SIZE_AXIS: i32 = 20;
const FONT_SIZE_LEGEND: i32 = 18;
const FONT_SIZE_DETAIL: i32 = 17;
const FONT_SIZE_BAR_LABEL: i32 = 17;
const FONT_SIZE_FOOTER: i32 = 15;
const STATS_HEADER_HEIGHT: i32 = 34;
const STATS_ROW_HEIGHT: i32 = 34;

/// Theme colors for chart rendering
struct ThemeColors {
    bg_color: &'static str,
    text_primary: &'static str,
    text_secondary: &'static str,
    text_muted: &'static str,
    text_dim: &'static str,
    bar_outline: &'static str,
    container_stroke: &'static str,
    plot_bg: &'static str,
    detail_box_fill: &'static str,
    detail_box_stroke: &'static str,
    row_border: &'static str,
}

impl ThemeColors {
    fn dark() -> Self {
        Self {
            bg_color: "#1E1E20",
            text_primary: "#FFFFFF",
            text_secondary: "rgba(255,255,255,0.7)",
            text_muted: "rgba(255,255,255,0.45)",
            text_dim: "rgba(255,255,255,0.35)",
            bar_outline: "rgba(255,255,255,0.15)",
            container_stroke: "rgba(255,255,255,0.12)",
            plot_bg: "rgba(255,255,255,0.02)",
            detail_box_fill: "rgba(255,255,255,0.03)",
            detail_box_stroke: "rgba(255,255,255,0.14)",
            row_border: "rgba(255,255,255,0.23)",
        }
    }

    fn light() -> Self {
        Self {
            bg_color: "#FFFFFF",
            text_primary: "#1A1A1A",
            text_secondary: "rgba(0,0,0,0.7)",
            text_muted: "rgba(0,0,0,0.5)",
            text_dim: "rgba(0,0,0,0.35)",
            bar_outline: "rgba(0,0,0,0.1)",
            container_stroke: "rgba(0,0,0,0.08)",
            plot_bg: "rgba(0,0,0,0.02)",
            detail_box_fill: "rgba(0,0,0,0.03)",
            detail_box_stroke: "rgba(0,0,0,0.14)",
            row_border: "rgba(0,0,0,0.22)",
        }
    }

    fn from_name(name: Option<&str>) -> Self {
        match name.map(|s| s.to_lowercase()).as_deref() {
            Some("light") => Self::light(),
            _ => Self::dark(), // Default to dark theme
        }
    }
}

/// Speedup data for a single language measurement
struct LangSpeedup {
    lang: Lang,
    total_nanos: f64,
    /// Primary value for ratio (nanos for performance, bytes for memory - lower is better)
    primary_value: f64,
    iterations: u64,
    #[allow(dead_code)]
    run_count: Option<u64>,
}

struct LangSummary {
    lang: Lang,
    total_nanos: f64,
    total_iterations: u64,
    total_speedup: f64,
    is_baseline: bool,
    /// Number of benchmarks that contributed to this summary (for averaging)
    bench_count: usize,
    /// Sum of bytes_per_op for memory mode; count in bytes_bench_count (None for performance)
    bytes_per_op_sum: Option<f64>,
    bytes_bench_count: usize,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum BenchMetricMode {
    Fixed,
    Auto,
    Memory,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum OverallMetricMode {
    Fixed,
    Auto,
    Memory,
    Mixed,
}

struct GridLayout {
    columns: i32,
    card_width: i32,
    card_height: i32,
    grid_height: i32,
}

fn calculate_grid_layout(
    num_benchmarks: i32,
    num_langs: i32,
    plot_width: i32,
    forced_columns: Option<i32>,
) -> GridLayout {
    let bar_stack_height = num_langs * (DEFAULT_BAR_HEIGHT + BAR_GAP) - BAR_GAP;
    let card_height = CARD_TITLE_HEIGHT + PLOT_PADDING * 2 + bar_stack_height + 6;
    let auto_max_columns =
        ((plot_width + GRID_GAP_X) / (MIN_CARD_WIDTH + GRID_GAP_X)).max(1).min(MAX_GRID_COLUMNS);
    let max_columns_from_width = forced_columns.unwrap_or(auto_max_columns).max(1);
    let mut best = GridLayout {
        columns: 1,
        card_width: plot_width.max(MIN_CARD_WIDTH),
        card_height,
        grid_height: num_benchmarks.max(1) * card_height + (num_benchmarks.max(1) - 1) * GRID_GAP_Y,
    };

    for candidate_columns in 1..=max_columns_from_width.max(1) {
        let rows = (num_benchmarks + candidate_columns - 1) / candidate_columns;
        let width_without_gaps = plot_width - (candidate_columns - 1) * GRID_GAP_X;
        let card_width = (width_without_gaps / candidate_columns).max(MIN_CARD_WIDTH);
        let grid_height = rows * card_height + (rows - 1) * GRID_GAP_Y;
        let candidate =
            GridLayout { columns: candidate_columns, card_width, card_height, grid_height };

        best = candidate;
        if grid_height <= TARGET_COMBINED_GRID_HEIGHT {
            break;
        }
    }

    // If no candidate meets the height target, retain the widest feasible grid from the loop.
    if best.grid_height > TARGET_COMBINED_GRID_HEIGHT && max_columns_from_width > 1 {
        let columns = max_columns_from_width;
        let rows = (num_benchmarks + columns - 1) / columns;
        let width_without_gaps = plot_width - (columns - 1) * GRID_GAP_X;
        let card_width = (width_without_gaps / columns).max(MIN_CARD_WIDTH);
        let grid_height = rows * card_height + (rows - 1) * GRID_GAP_Y;
        return GridLayout { columns, card_width, card_height, grid_height };
    }

    best
}

fn row_item_count(row: i32, columns: i32, total_items: i32) -> i32 {
    let start = row * columns;
    let remaining = total_items - start;
    remaining.max(0).min(columns).max(1)
}

/// Generate a speedup chart showing relative performance vs baseline (or memory usage for memory
/// suites)
pub fn generate(
    benchmarks: Vec<&BenchmarkResult>,
    directive: &ChartDirectiveIR,
    suite_type: poly_bench_dsl::SuiteType,
) -> String {
    let mut filtered = filter_benchmarks(benchmarks, directive);
    sort_benchmarks(&mut filtered, directive);

    if filtered.is_empty() {
        return empty_chart(
            "No benchmark data available",
            &ThemeColors::from_name(directive.theme.as_deref()),
        );
    }

    // Get theme colors
    let theme = ThemeColors::from_name(directive.theme.as_deref());

    // Determine baseline language (default to Go)
    let baseline_lang =
        directive.baseline_benchmark.as_ref().and_then(|s| Lang::from_str(s)).unwrap_or(Lang::Go);

    // Determine which languages have data (use supported_languages for consistency)
    let all_langs: Vec<Lang> = poly_bench_runtime::supported_languages()
        .iter()
        .copied()
        .filter(|&lang| filtered.iter().any(|b| b.measurements.contains_key(&lang)))
        .collect();

    if all_langs.is_empty() {
        return empty_chart("No comparison data available", &theme);
    }

    // Calculate speedups with timing data
    let mut speedups: Vec<(String, BenchMetricMode, Vec<LangSpeedup>)> = Vec::new();
    let mut max_run_count: Option<u64> = None;
    let mut fixed_mode_count = 0usize;
    let mut auto_mode_count = 0usize;

    let is_memory = suite_type == poly_bench_dsl::SuiteType::Memory;
    for bench in &filtered {
        if bench.measurements.contains_key(&baseline_lang) {
            let mut bench_speedups = Vec::new();
            for &lang in &all_langs {
                if let Some(m) = bench.measurements.get(&lang) {
                    // Track max run_count across all measurements
                    if let Some(rc) = m.run_count {
                        max_run_count = Some(max_run_count.map_or(rc, |c| c.max(rc)));
                    }

                    let primary_value = if is_memory {
                        m.bytes_per_op.map(|b| b as f64).unwrap_or(f64::MAX)
                    } else {
                        m.nanos_per_op
                    };

                    bench_speedups.push(LangSpeedup {
                        lang,
                        total_nanos: m.nanos_per_op * m.iterations as f64,
                        primary_value,
                        iterations: m.iterations,
                        run_count: m.run_count,
                    });
                }
            }
            if !bench_speedups.is_empty() {
                let bench_mode = infer_benchmark_mode(&bench_speedups, is_memory);
                match bench_mode {
                    BenchMetricMode::Fixed => fixed_mode_count += 1,
                    BenchMetricMode::Auto => auto_mode_count += 1,
                    BenchMetricMode::Memory => {}
                }
                // Sort bars by metric value descending (highest first)
                bench_speedups.sort_by(|a, b| {
                    let va = bench_metric_value(bench_mode, a);
                    let vb = bench_metric_value(bench_mode, b);
                    vb.partial_cmp(&va).unwrap_or(std::cmp::Ordering::Equal)
                });
                speedups.push((bench.name.clone(), bench_mode, bench_speedups));
            }
        }
    }

    if speedups.is_empty() {
        return empty_chart("No valid speedup data", &theme);
    }

    let primary_from_m = |m: &poly_bench_runtime::measurement::Measurement| -> f64 {
        if is_memory {
            m.bytes_per_op.map(|b| b as f64).unwrap_or(f64::MAX)
        } else {
            m.nanos_per_op
        }
    };

    let mut language_summaries = Vec::new();
    for &lang in &all_langs {
        let mut lang_total_nanos = 0.0;
        let mut total_iterations = 0u64;
        let mut speedup_log_sum = 0.0;
        let mut speedup_samples = 0usize;
        let mut bench_count = 0usize;
        let mut bytes_per_op_sum = 0.0;
        let mut bytes_bench_count = 0usize;

        for bench in &filtered {
            let baseline = bench.measurements.get(&baseline_lang);
            let current = bench.measurements.get(&lang);

            if let (Some(base), Some(cur)) = (baseline, current) {
                bench_count += 1;
                lang_total_nanos += cur.nanos_per_op * cur.iterations as f64;
                total_iterations += cur.iterations;
                if is_memory {
                    if let Some(b) = cur.bytes_per_op {
                        bytes_per_op_sum += b as f64;
                        bytes_bench_count += 1;
                    }
                }
                let base_val = primary_from_m(base);
                let cur_val = primary_from_m(cur);
                if cur_val > 0.0 && cur_val < f64::MAX {
                    speedup_log_sum += (base_val / cur_val).ln();
                    speedup_samples += 1;
                }
            }
        }

        if lang_total_nanos > 0.0 && speedup_samples > 0 {
            language_summaries.push(LangSummary {
                lang,
                total_nanos: lang_total_nanos,
                total_iterations,
                total_speedup: (speedup_log_sum / speedup_samples as f64).exp(),
                is_baseline: lang == baseline_lang,
                bench_count,
                bytes_per_op_sum: if is_memory && bytes_bench_count > 0 {
                    Some(bytes_per_op_sum)
                } else {
                    None
                },
                bytes_bench_count,
            });
        }
    }

    let overall_mode = if is_memory {
        OverallMetricMode::Memory
    } else {
        match (fixed_mode_count > 0, auto_mode_count > 0) {
            (true, true) => OverallMetricMode::Mixed,
            (false, true) => OverallMetricMode::Auto,
            _ => OverallMetricMode::Fixed,
        }
    };

    // Replace combined grid with single averaged chart when multiple benchmarks
    if speedups.len() > 1 {
        let agg_mode = match overall_mode {
            OverallMetricMode::Fixed => BenchMetricMode::Fixed,
            OverallMetricMode::Auto => BenchMetricMode::Auto,
            OverallMetricMode::Memory => BenchMetricMode::Memory,
            OverallMetricMode::Mixed => BenchMetricMode::Auto, // Use iterations for mixed
        };
        let mut agg_speedups: Vec<LangSpeedup> = language_summaries
            .iter()
            .map(|s| {
                let n = s.bench_count.max(1);
                let avg_total_nanos = s.total_nanos / n as f64;
                let avg_iterations = s.total_iterations / n as u64;
                let primary_value = match agg_mode {
                    BenchMetricMode::Fixed => avg_total_nanos,
                    BenchMetricMode::Auto => avg_iterations as f64,
                    BenchMetricMode::Memory => s
                        .bytes_per_op_sum
                        .and_then(|sum| {
                            let bc = s.bytes_bench_count.max(1);
                            Some(sum / bc as f64)
                        })
                        .unwrap_or(f64::MAX),
                };
                LangSpeedup {
                    lang: s.lang,
                    total_nanos: avg_total_nanos,
                    primary_value,
                    iterations: avg_iterations,
                    run_count: max_run_count,
                }
            })
            .collect();
        agg_speedups.sort_by(|a, b| {
            let va = bench_metric_value(agg_mode, a);
            let vb = bench_metric_value(agg_mode, b);
            vb.partial_cmp(&va).unwrap_or(std::cmp::Ordering::Equal)
        });
        speedups = vec![("Average".to_string(), agg_mode, agg_speedups)];
    }

    let num_benchmarks = speedups.len() as i32;
    let num_langs = all_langs.len() as i32;
    let forced_row_count =
        directive.row_count.map(|c| (c as i32).max(1).min(num_benchmarks.max(1)));

    let default_columns = forced_row_count.unwrap_or(MAX_GRID_COLUMNS).max(1);
    let combined_min_width = 40 +
        (default_columns * MIN_CARD_WIDTH) +
        ((default_columns - 1) * GRID_GAP_X) +
        MARGIN_RIGHT;
    let default_single_width = (720.0 * SPEEDUP_BAR_WIDTH_FACTOR) as i32;
    let chart_width = if num_benchmarks > 1 {
        directive.width.unwrap_or(combined_min_width).max(combined_min_width)
    } else {
        directive.width.unwrap_or(default_single_width)
    };
    let plot_y = MARGIN_TOP;
    let details_height =
        STATS_HEADER_HEIGHT + (language_summaries.len() as i32).max(1) * STATS_ROW_HEIGHT + 12;
    let combined_summary_height =
        STATS_HEADER_HEIGHT + (language_summaries.len() as i32).max(1) * STATS_ROW_HEIGHT + 18;
    let is_combined_chart = num_benchmarks > 1;

    let (
        plot_x,
        plot_width,
        plot_height,
        summary_y,
        x_axis_label_y,
        legend_y,
        details_y,
        chart_height,
    ) = if is_combined_chart {
        // Grid layout for combined chart: auto columns from width with max-height target.
        let plot_x = 40;
        let plot_width = chart_width - plot_x - MARGIN_RIGHT;
        let grid_layout =
            calculate_grid_layout(num_benchmarks, num_langs, plot_width, forced_row_count);
        let plot_height = grid_layout.grid_height;
        let summary_y = plot_y + plot_height + 12;
        let x_axis_label_y = summary_y + combined_summary_height + 18;
        let legend_y = x_axis_label_y + 28;
        let details_y = legend_y + 26;
        let chart_height = directive.height.unwrap_or(details_y + details_height + 24);
        (
            plot_x,
            plot_width,
            plot_height,
            summary_y,
            x_axis_label_y,
            legend_y,
            details_y,
            chart_height,
        )
    } else {
        // Preserve pre-grid look and spacing for single-benchmark charts.
        let plot_x = MARGIN_LEFT;
        let plot_width = chart_width - plot_x - MARGIN_RIGHT;
        let bar_area_height = num_langs * (DEFAULT_BAR_HEIGHT + BAR_GAP) - BAR_GAP;
        let plot_height = bar_area_height + PLOT_PADDING * 2;
        let summary_y = plot_y + plot_height;
        let x_axis_label_y = plot_y + plot_height + 20;
        let legend_y = x_axis_label_y + 28;
        let details_y = legend_y + 26;
        let chart_height = directive.height.unwrap_or(
            (details_y + details_height + 24).max(MARGIN_TOP + plot_height + MARGIN_BOTTOM),
        );
        (
            plot_x,
            plot_width,
            plot_height,
            summary_y,
            x_axis_label_y,
            legend_y,
            details_y,
            chart_height,
        )
    };

    let grid_layout = if is_combined_chart {
        Some(calculate_grid_layout(num_benchmarks, num_langs, plot_width, forced_row_count))
    } else {
        None
    };

    let mut svg = svg_header(chart_width, chart_height, &theme);

    // Title
    let title = directive.title.as_deref().unwrap_or("Speedup vs Baseline");
    let baseline_name = lang_full_name(baseline_lang);
    svg.push_str(&svg_title(chart_width, title, baseline_name, overall_mode, &theme));

    // Plot area background
    svg.push_str(&format!(
        "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"8\" fill=\"{}\"/>\n",
        plot_x, plot_y, plot_width, plot_height, theme.plot_bg
    ));

    // Keep accent bar on single charts; remove it for combined chart grid.
    if !is_combined_chart {
        svg.push_str(&format!(
            "  <rect x=\"{}\" y=\"{}\" width=\"6\" height=\"{}\" rx=\"3\" fill=\"url(#accentGrad)\" fill-opacity=\"0.96\" filter=\"url(#accentGlow)\"/>\n",
            plot_x - 6,
            plot_y,
            plot_height
        ));
    }

    // Vertical axis label.
    let y_axis_label = "runtime";
    let y_axis_x = if is_combined_chart { plot_x - 30 } else { plot_x - 24 };
    svg.push_str(&format!(
        "  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" transform=\"rotate(-90 {} {})\" font-family=\"{}\" font-size=\"{}\" font-weight=\"600\" fill=\"{}\">{}</text>\n",
        y_axis_x,
        plot_y + plot_height / 2,
        y_axis_x,
        plot_y + plot_height / 2,
        FONT_FAMILY,
        FONT_SIZE_AXIS,
        theme.text_muted,
        y_axis_label
    ));

    let bar_height = DEFAULT_BAR_HEIGHT;
    if let Some(grid_layout) = &grid_layout {
        // Draw benchmark cards in a centered small-multiples grid.
        for (idx, (bench_name, bench_mode, bench_speedups)) in speedups.iter().enumerate() {
            let card_idx = idx as i32;
            let row = card_idx / grid_layout.columns;
            let col = card_idx % grid_layout.columns;
            let row_items = row_item_count(row, grid_layout.columns, num_benchmarks);
            let row_width = row_items * grid_layout.card_width + (row_items - 1) * GRID_GAP_X;
            let row_start_x = plot_x + (plot_width - row_width) / 2;
            let card_x = row_start_x + col * (grid_layout.card_width + GRID_GAP_X);
            let card_y = plot_y + row * (grid_layout.card_height + GRID_GAP_Y);

            svg.push_str(&format!(
                "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"10\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>\n",
                card_x,
                card_y,
                grid_layout.card_width,
                grid_layout.card_height,
                theme.plot_bg,
                theme.container_stroke
            ));

            svg.push_str(&format!(
                "  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"{}\" font-size=\"{}\" font-weight=\"600\" fill=\"{}\">{}</text>\n",
                card_x + grid_layout.card_width / 2,
                card_y + 15,
                FONT_FAMILY,
                FONT_SIZE_BAR_LABEL,
                theme.text_primary,
                escape_xml(bench_name)
            ));

            let card_plot_x = card_x + 8;
            let card_plot_y = card_y + CARD_TITLE_HEIGHT;
            let card_plot_width = (grid_layout.card_width - 16).max(40);
            let bench_max_metric = bench_speedups
                .iter()
                .map(|ls| bench_metric_value(*bench_mode, ls))
                .fold(0.0_f64, f64::max)
                .max(1.0);

            let mut y = card_plot_y + PLOT_PADDING;
            for ls in bench_speedups {
                let row_x = card_plot_x + 4;
                let row_y = y;
                let row_width = (card_plot_width - 8).max(20);
                let row_height = bar_height;
                let inset = 6;
                let inner_x = row_x + inset;
                let inner_y = row_y + inset;
                let inner_height = (row_height - inset * 2).max(8);
                let max_inner_width = (row_width - inset * 2).max(12);
                let metric_value = bench_metric_value(*bench_mode, ls);
                let bar_width = ((metric_value / bench_max_metric) * max_inner_width as f64) as i32;
                let gradient_id = lang_gradient_id(ls.lang);
                let color = lang_color(ls.lang);

                svg.push_str(&format!(
                    "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"8\" fill=\"none\" stroke=\"{}\" stroke-width=\"1.2\"/>\n",
                    row_x,
                    row_y,
                    row_width,
                    row_height,
                    theme.row_border
                ));
                svg.push_str(&format!(
                    "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"8\" fill=\"url(#{})\" stroke=\"{}\" stroke-width=\"1.5\" filter=\"url(#barShadow)\"/>\n",
                    inner_x,
                    inner_y,
                    bar_width.max(6),
                    inner_height,
                    gradient_id,
                    theme.bar_outline
                ));

                let lang_name = lang_label(ls.lang);
                let metric_label = format_primary_metric(*bench_mode, ls);
                let speedup_label = format!("{} · {}", lang_name, metric_label);
                let bar_end = inner_x + bar_width.max(6);
                let label_inside = bar_width > (row_width / 2);
                let (label_x, label_anchor, label_color) = if label_inside {
                    (bar_end - 8, "end", theme.text_primary)
                } else {
                    ((bar_end + 8).min(row_x + row_width - 8), "start", theme.text_secondary)
                };

                svg.push_str(&format!(
                    "  <text x=\"{}\" y=\"{}\" text-anchor=\"{}\" font-family=\"{}\" font-size=\"{}\" font-weight=\"500\" fill=\"{}\">{}</text>\n",
                    label_x,
                    y + bar_height / 2 + 4,
                    label_anchor,
                    FONT_FAMILY,
                    FONT_SIZE_BAR_LABEL,
                    label_color,
                    escape_xml(&speedup_label)
                ));
                svg.push_str(&format!(
                    "  <circle cx=\"{}\" cy=\"{}\" r=\"3\" fill=\"{}\"/>\n",
                    row_x - 8,
                    y + bar_height / 2,
                    color
                ));

                y += bar_height + BAR_GAP;
            }
        }
    } else {
        // Keep single-benchmark charts visually unchanged from pre-grid behavior.
        let mut y = plot_y + PLOT_PADDING;
        for (_bench_name, bench_mode, bench_speedups) in &speedups {
            let bench_max_metric = bench_speedups
                .iter()
                .map(|ls| bench_metric_value(*bench_mode, ls))
                .fold(0.0_f64, f64::max)
                .max(1.0);

            for ls in bench_speedups {
                let inner_x = plot_x + 8;
                let inner_y = y + 6;
                let inner_height = (bar_height - 12).max(8);
                let max_inner_width = (plot_width - 20).max(12);
                let row_x = plot_x + 4;
                let row_y = y;
                let row_width = (plot_width - 8).max(20);
                let row_height = bar_height;
                let metric_value = bench_metric_value(*bench_mode, ls);
                let bar_width = ((metric_value / bench_max_metric) * max_inner_width as f64) as i32;
                let gradient_id = lang_gradient_id(ls.lang);
                let color = lang_color(ls.lang);

                svg.push_str(&format!(
                    "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"8\" fill=\"none\" stroke=\"{}\" stroke-width=\"1.2\"/>\n",
                    row_x,
                    row_y,
                    row_width,
                    row_height,
                    theme.row_border
                ));
                svg.push_str(&format!(
                    "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"8\" fill=\"url(#{})\" stroke=\"{}\" stroke-width=\"1.5\" filter=\"url(#barShadow)\"/>\n",
                    inner_x,
                    inner_y,
                    bar_width.max(6),
                    inner_height,
                    gradient_id,
                    theme.bar_outline
                ));

                let lang_name = lang_label(ls.lang);
                let metric_label = format_primary_metric(*bench_mode, ls);
                let speedup_label = format!("{} · {}", lang_name, metric_label);
                let bar_end = inner_x + bar_width.max(6);
                let label_inside = bar_width > 220;
                let (label_x, label_anchor, label_color) = if label_inside {
                    (bar_end - 10, "end", theme.text_primary)
                } else {
                    (bar_end + 10, "start", theme.text_secondary)
                };

                svg.push_str(&format!(
                    "  <text x=\"{}\" y=\"{}\" text-anchor=\"{}\" font-family=\"{}\" font-size=\"{}\" font-weight=\"500\" fill=\"{}\">{}</text>\n",
                    label_x,
                    y + bar_height / 2 + 4,
                    label_anchor,
                    FONT_FAMILY,
                    FONT_SIZE_BAR_LABEL,
                    label_color,
                    escape_xml(&speedup_label)
                ));
                svg.push_str(&format!(
                    "  <circle cx=\"{}\" cy=\"{}\" r=\"3\" fill=\"{}\"/>\n",
                    plot_x - 12,
                    y + bar_height / 2,
                    color
                ));

                y += bar_height + BAR_GAP;
            }
        }
    }

    if is_combined_chart {
        let summary_max_width = grid_layout
            .as_ref()
            .map(|layout| (3 * layout.card_width) + (2 * GRID_GAP_X))
            .unwrap_or(plot_width);
        svg.push_str(&svg_combined_summary(
            plot_x,
            plot_width,
            summary_max_width,
            summary_y,
            combined_summary_height,
            &language_summaries,
            baseline_lang,
            overall_mode,
            &theme,
        ));
    }

    // X-axis label (mode aware)
    let x_axis_label = match overall_mode {
        OverallMetricMode::Fixed => "time (ms)",
        OverallMetricMode::Auto => "iterations",
        OverallMetricMode::Memory => "bytes/op",
        OverallMetricMode::Mixed => "time / iterations by mode",
    };
    svg.push_str(&format!(
        "  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"{}\" font-size=\"{}\" font-weight=\"600\" fill=\"{}\">{}</text>\n",
        plot_x + plot_width / 2,
        x_axis_label_y,
        FONT_FAMILY,
        FONT_SIZE_AXIS,
        theme.text_muted,
        x_axis_label
    ));

    // Legend and detail stats below axis label.
    svg.push_str(&svg_legend(plot_x, plot_width, legend_y, &all_langs, baseline_lang, &theme));
    svg.push_str(&svg_detail_legend(
        plot_x,
        plot_width,
        details_y,
        details_height,
        &language_summaries,
        baseline_lang,
        overall_mode,
        &theme,
    ));

    // Footer info - show run count and async metadata if applicable
    let has_async = filtered.iter().any(|b| b.kind == BenchmarkKind::Async);
    let mut footer_lines: Vec<String> = Vec::new();

    if let Some(run_count) = max_run_count {
        if run_count > 1 {
            footer_lines.push(format!("averaged over {} runs", run_count));
        }
    }
    if has_async {
        let mut warmup_cap = None;
        let mut sample_cap = None;
        let mut sampling_policy = None;
        for bench in &filtered {
            if let Some(details) = &bench.async_details {
                warmup_cap.get_or_insert(details.warmup_cap);
                sample_cap.get_or_insert(details.sample_cap);
                sampling_policy.get_or_insert(details.sampling_policy.as_str());
            }
        }
        let warmup_text = warmup_cap.unwrap_or(5);
        let sample_text = sample_cap.unwrap_or(50);
        let policy_text = sampling_policy.unwrap_or("timeBudgeted");
        footer_lines.push(format!(
            "contains async-sequential benchmarks (policy: {}, warmup<={}, samples<={})",
            policy_text, warmup_text, sample_text
        ));
    }
    for (idx, line) in footer_lines.iter().enumerate() {
        let y = chart_height - 8 - ((footer_lines.len() as i32 - 1 - idx as i32) * 12);
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"{}\" font-size=\"{}\" fill=\"{}\">{}</text>\n",
            chart_width / 2,
            y,
            FONT_FAMILY,
            FONT_SIZE_FOOTER,
            theme.text_dim,
            escape_xml(line)
        ));
    }

    svg.push_str("</svg>\n");
    svg
}

/// Format iterations count compactly
fn format_iterations_short(iterations: u64) -> String {
    if iterations >= 1_000_000 {
        format!("{:.1}M", iterations as f64 / 1_000_000.0)
    } else if iterations >= 1_000 {
        format!("{:.1}K", iterations as f64 / 1_000.0)
    } else {
        format!("{}", iterations)
    }
}

/// Generate an empty chart with a message
fn empty_chart(message: &str, theme: &ThemeColors) -> String {
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"400\" height=\"100\">\
        <rect width=\"400\" height=\"100\" fill=\"{}\" rx=\"8\"/>\
        <text x=\"200\" y=\"55\" text-anchor=\"middle\" font-family=\"{}\" font-size=\"{}\" fill=\"{}\">{}</text>\
        </svg>",
        theme.bg_color, FONT_FAMILY, FONT_SIZE_DETAIL, theme.text_muted, message
    )
}

/// SVG header with gradients and filters (gradients built from registered runtimes)
fn svg_header(width: i32, height: i32, theme: &ThemeColors) -> String {
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\" fill=\"none\">\n\
<rect width=\"{}\" height=\"{}\" fill=\"{}\"/>\n\
<defs>\n\
  <radialGradient id=\"accentGrad\" cx=\"0.5\" cy=\"0.5\" r=\"0.8\">\n\
    <stop stop-color=\"{}\"/>\n\
    <stop offset=\"1\" stop-color=\"{}\"/>\n\
  </radialGradient>\n\
{}\
  <filter id=\"barShadow\" x=\"-5%\" y=\"-15%\" width=\"110%\" height=\"140%\">\n\
    <feDropShadow dx=\"0\" dy=\"2\" stdDeviation=\"2\" flood-opacity=\"0.25\"/>\n\
  </filter>\n\
  <filter id=\"accentGlow\" x=\"-150%\" y=\"-10%\" width=\"400%\" height=\"120%\">\n\
    <feDropShadow dx=\"0\" dy=\"0\" stdDeviation=\"4\" flood-color=\"{}\" flood-opacity=\"0.7\"/>\n\
  </filter>\n\
</defs>\n",
        width, height, width, height,
        width, height, theme.bg_color,
        ACCENT_COLOR, ACCENT_GLOW,
        svg_gradient_defs(),
        ACCENT_COLOR
    )
}

/// Title section
fn svg_title(
    width: i32,
    title: &str,
    baseline_name: &str,
    mode: OverallMetricMode,
    theme: &ThemeColors,
) -> String {
    let subtitle = match mode {
        OverallMetricMode::Fixed => {
            format!("Fixed mode · bars show total runtime · speedup details vs {} in legend", baseline_name)
        }
        OverallMetricMode::Auto => {
            format!(
                "Auto mode · bars show total iterations completed · speedup details vs {} in legend",
                baseline_name
            )
        }
        OverallMetricMode::Memory => {
            format!(
                "Memory mode · bars show bytes/op · memory comparison vs {} in legend",
                baseline_name
            )
        }
        OverallMetricMode::Mixed => format!(
            "Mixed modes · auto bars show iterations, fixed bars show runtime · speedup details vs {} in legend",
            baseline_name
        ),
    };
    format!(
        "<text x=\"{}\" y=\"44\" text-anchor=\"middle\" font-family=\"{}\" font-size=\"{}\" font-weight=\"700\" fill=\"{}\">{}</text>\n\
<text x=\"{}\" y=\"78\" text-anchor=\"middle\" font-family=\"{}\" font-size=\"{}\" fill=\"{}\">{}</text>\n",
        width / 2, FONT_FAMILY, FONT_SIZE_TITLE, theme.text_primary, escape_xml(title),
        width / 2, FONT_FAMILY, FONT_SIZE_SUBTITLE, theme.text_muted, subtitle
    )
}

/// Legend section - spans plot width, items distributed evenly
fn svg_legend(
    plot_x: i32,
    plot_width: i32,
    y: i32,
    all_langs: &[Lang],
    baseline_lang: Lang,
    theme: &ThemeColors,
) -> String {
    let mut items: Vec<(&str, &str, bool)> = Vec::new();

    for &lang in all_langs {
        items.push((lang_color(lang), lang_full_name(lang), baseline_lang == lang));
    }

    let n = items.len().max(1) as i32;
    let item_width = plot_width / n;

    let mut svg = format!("<g transform=\"translate({},{})\">\n", plot_x, y);

    for (i, (color, label, is_baseline)) in items.iter().enumerate() {
        let x = i as i32 * item_width;
        let display_label = if *is_baseline { format!("{} ★", label) } else { label.to_string() };

        svg.push_str(&format!(
            "  <circle cx=\"{}\" cy=\"0\" r=\"5\" fill=\"{}\"/>\n",
            x + 5,
            color
        ));

        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"5\" font-family=\"{}\" font-size=\"{}\" fill=\"{}\">{}</text>\n",
            x + 18,
            FONT_FAMILY,
            FONT_SIZE_LEGEND,
            theme.text_secondary,
            display_label
        ));
    }

    svg.push_str("</g>\n");
    svg
}

fn svg_detail_legend(
    plot_x: i32,
    plot_width: i32,
    y: i32,
    _height: i32,
    summaries: &[LangSummary],
    baseline_lang: Lang,
    mode: OverallMetricMode,
    theme: &ThemeColors,
) -> String {
    let mut svg = String::new();

    if summaries.is_empty() {
        return svg;
    }

    let pad_x = 10;
    let table_x = plot_x + pad_x;
    let table_w = (plot_width - pad_x * 2).max(280);
    let table_h = STATS_HEADER_HEIGHT + summaries.len() as i32 * STATS_ROW_HEIGHT;

    // Column widths: Language | Iterations | Runtime | Speedup
    let lang_w = table_w * 18 / 100;
    let iter_w = table_w * 22 / 100;
    let runtime_w = table_w * 22 / 100;
    let speedup_w = table_w - lang_w - iter_w - runtime_w;
    let cols = [lang_w, iter_w, runtime_w, speedup_w];
    let headers = ["Language", "Iterations", "Runtime", "Speedup"];
    let header_y = y + STATS_HEADER_HEIGHT;

    svg.push_str(&format!(
        "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"6\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>\n",
        table_x, y, table_w, table_h, theme.detail_box_fill, theme.detail_box_stroke
    ));
    svg.push_str(&format!(
        "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>\n",
        table_x,
        header_y,
        table_x + table_w,
        header_y,
        theme.detail_box_stroke
    ));

    let mut col_x = table_x;
    for (idx, col_w) in cols.iter().enumerate() {
        if idx < headers.len() {
            svg.push_str(&format!(
                "  <text x=\"{}\" y=\"{}\" font-family=\"{}\" font-size=\"{}\" font-weight=\"700\" fill=\"{}\">{}</text>\n",
                col_x + 8,
                y + 20,
                FONT_FAMILY,
                FONT_SIZE_DETAIL,
                theme.text_secondary,
                headers[idx]
            ));
        }
        col_x += *col_w;
        if idx < cols.len() - 1 {
            svg.push_str(&format!(
                "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>\n",
                col_x, y, col_x, y + table_h, theme.detail_box_stroke
            ));
        }
    }

    for (idx, summary) in summaries.iter().enumerate() {
        let row_top = header_y + idx as i32 * STATS_ROW_HEIGHT;
        if idx % 2 == 1 {
            svg.push_str(&format!(
                "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" fill-opacity=\"0.35\"/>\n",
                table_x, row_top, table_w, STATS_ROW_HEIGHT, theme.detail_box_fill
            ));
        }
        if idx < summaries.len() - 1 {
            svg.push_str(&format!(
                "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>\n",
                table_x, row_top + STATS_ROW_HEIGHT, table_x + table_w, row_top + STATS_ROW_HEIGHT,
                theme.detail_box_stroke
            ));
        }

        let total = format_duration(summary.total_nanos);
        let iters = format_iterations_short(summary.total_iterations);
        let speedup = format_speedup_phrase(
            summary.total_speedup,
            summary.is_baseline,
            baseline_lang,
            mode == OverallMetricMode::Memory,
        );
        let lang_name = lang_full_name(summary.lang);
        let baseline_y = row_top + 20;

        let mut x_cursor = table_x;
        svg.push_str(&format!(
            "  <circle cx=\"{}\" cy=\"{}\" r=\"4\" fill=\"{}\"/>\n",
            x_cursor + 9,
            baseline_y - 3,
            lang_color(summary.lang)
        ));
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" font-family=\"{}\" font-size=\"{}\" font-weight=\"600\" fill=\"{}\">{}</text>\n",
            x_cursor + 17, baseline_y, FONT_FAMILY, FONT_SIZE_DETAIL, theme.text_primary, lang_name
        ));
        x_cursor += cols[0];
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" font-family=\"{}\" font-size=\"{}\" fill=\"{}\">{}</text>\n",
            x_cursor + 8,
            baseline_y,
            FONT_FAMILY,
            FONT_SIZE_DETAIL,
            theme.text_muted,
            iters
        ));
        x_cursor += cols[1];
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" font-family=\"{}\" font-size=\"{}\" fill=\"{}\">{}</text>\n",
            x_cursor + 8,
            baseline_y,
            FONT_FAMILY,
            FONT_SIZE_DETAIL,
            theme.text_muted,
            total
        ));
        x_cursor += cols[2];
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" font-family=\"{}\" font-size=\"{}\" fill=\"{}\">{}</text>\n",
            x_cursor + 8,
            baseline_y,
            FONT_FAMILY,
            FONT_SIZE_DETAIL,
            theme.text_muted,
            escape_xml(&speedup)
        ));
    }

    svg
}

fn svg_combined_summary(
    plot_x: i32,
    plot_width: i32,
    _summary_max_width: i32,
    y: i32,
    _height: i32,
    summaries: &[LangSummary],
    baseline_lang: Lang,
    mode: OverallMetricMode,
    theme: &ThemeColors,
) -> String {
    let mut svg = String::new();

    if summaries.is_empty() {
        return svg;
    }

    let pad_x = 10;
    let table_x = plot_x + pad_x;
    let table_w = (plot_width - pad_x * 2).max(280);
    let table_h = STATS_HEADER_HEIGHT + summaries.len() as i32 * STATS_ROW_HEIGHT;

    let lang_w = table_w * 18 / 100;
    let iter_w = table_w * 22 / 100;
    let runtime_w = table_w * 22 / 100;
    let speedup_w = table_w - lang_w - iter_w - runtime_w;
    let cols = [lang_w, iter_w, runtime_w, speedup_w];
    let headers = ["Language", "Iterations", "Runtime", "Avg Speedup"];
    let header_y = y + STATS_HEADER_HEIGHT;

    svg.push_str(&format!(
        "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"6\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>\n",
        table_x, y, table_w, table_h, theme.detail_box_fill, theme.detail_box_stroke
    ));
    svg.push_str(&format!(
        "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>\n",
        table_x,
        header_y,
        table_x + table_w,
        header_y,
        theme.detail_box_stroke
    ));

    let mut col_x = table_x;
    for (idx, col_w) in cols.iter().enumerate() {
        if idx < headers.len() {
            svg.push_str(&format!(
                "  <text x=\"{}\" y=\"{}\" font-family=\"{}\" font-size=\"{}\" font-weight=\"700\" fill=\"{}\">{}</text>\n",
                col_x + 8,
                y + 20,
                FONT_FAMILY,
                FONT_SIZE_DETAIL,
                theme.text_secondary,
                headers[idx]
            ));
        }
        col_x += *col_w;
        if idx < cols.len() - 1 {
            svg.push_str(&format!(
                "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>\n",
                col_x, y, col_x, y + table_h, theme.detail_box_stroke
            ));
        }
    }

    for (idx, summary) in summaries.iter().enumerate() {
        let row_top = header_y + idx as i32 * STATS_ROW_HEIGHT;
        if idx % 2 == 1 {
            svg.push_str(&format!(
                "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" fill-opacity=\"0.35\"/>\n",
                table_x, row_top, table_w, STATS_ROW_HEIGHT, theme.detail_box_fill
            ));
        }
        if idx < summaries.len() - 1 {
            svg.push_str(&format!(
                "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>\n",
                table_x, row_top + STATS_ROW_HEIGHT, table_x + table_w, row_top + STATS_ROW_HEIGHT,
                theme.detail_box_stroke
            ));
        }

        let total = format_duration(summary.total_nanos);
        let iters = format_iterations_short(summary.total_iterations);
        let speedup = format_speedup_phrase(
            summary.total_speedup,
            summary.is_baseline,
            baseline_lang,
            mode == OverallMetricMode::Memory,
        );
        let lang_name = lang_full_name(summary.lang);
        let baseline_y = row_top + 20;

        let mut x_cursor = table_x;
        svg.push_str(&format!(
            "  <circle cx=\"{}\" cy=\"{}\" r=\"4\" fill=\"{}\"/>\n",
            x_cursor + 9,
            baseline_y - 3,
            lang_color(summary.lang)
        ));
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" font-family=\"{}\" font-size=\"{}\" font-weight=\"600\" fill=\"{}\">{}</text>\n",
            x_cursor + 17, baseline_y, FONT_FAMILY, FONT_SIZE_DETAIL, theme.text_primary, lang_name
        ));
        x_cursor += cols[0];
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" font-family=\"{}\" font-size=\"{}\" fill=\"{}\">{}</text>\n",
            x_cursor + 8,
            baseline_y,
            FONT_FAMILY,
            FONT_SIZE_DETAIL,
            theme.text_muted,
            iters
        ));
        x_cursor += cols[1];
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" font-family=\"{}\" font-size=\"{}\" fill=\"{}\">{}</text>\n",
            x_cursor + 8,
            baseline_y,
            FONT_FAMILY,
            FONT_SIZE_DETAIL,
            theme.text_muted,
            total
        ));
        x_cursor += cols[2];
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" font-family=\"{}\" font-size=\"{}\" fill=\"{}\">{}</text>\n",
            x_cursor + 8,
            baseline_y,
            FONT_FAMILY,
            FONT_SIZE_DETAIL,
            theme.text_muted,
            escape_xml(&speedup)
        ));
    }

    svg
}

fn format_speedup_phrase(
    speedup: f64,
    is_baseline: bool,
    baseline_lang: Lang,
    is_memory: bool,
) -> String {
    if is_baseline {
        return "baseline".to_string();
    }

    let baseline_name = lang_full_name(baseline_lang);
    if is_memory {
        if speedup >= 1.0 {
            format!("{:.2}x less memory vs {}", speedup, baseline_name)
        } else {
            format!("{:.2}x more memory vs {}", 1.0 / speedup, baseline_name)
        }
    } else if speedup >= 1.0 {
        format!("{:.2}x faster vs {}", speedup, baseline_name)
    } else {
        format!("{:.2}x slower vs {}", 1.0 / speedup, baseline_name)
    }
}

fn infer_benchmark_mode(bench_speedups: &[LangSpeedup], is_memory: bool) -> BenchMetricMode {
    if is_memory {
        return BenchMetricMode::Memory;
    }
    if bench_speedups.len() < 2 {
        return BenchMetricMode::Fixed;
    }

    let first_iters = bench_speedups[0].iterations;
    if bench_speedups.iter().all(|ls| ls.iterations == first_iters) {
        BenchMetricMode::Fixed
    } else {
        BenchMetricMode::Auto
    }
}

fn bench_metric_value(mode: BenchMetricMode, ls: &LangSpeedup) -> f64 {
    match mode {
        BenchMetricMode::Fixed => ls.total_nanos,
        BenchMetricMode::Auto => ls.iterations as f64,
        BenchMetricMode::Memory => ls.primary_value.max(1.0),
    }
}

fn format_primary_metric(mode: BenchMetricMode, ls: &LangSpeedup) -> String {
    match mode {
        BenchMetricMode::Fixed => format_duration(ls.total_nanos),
        BenchMetricMode::Auto => format!("{} iter", format_iterations_short(ls.iterations)),
        BenchMetricMode::Memory => {
            poly_bench_runtime::measurement::Measurement::format_bytes(ls.primary_value as u64)
        }
    }
}
