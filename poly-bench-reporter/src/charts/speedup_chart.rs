//! Speedup chart generator - shows relative performance vs baseline language
//!
//! Generates bar charts showing relative performance vs baseline with support for
//! light and dark themes.

use poly_bench_dsl::{BenchmarkKind, Lang};
use poly_bench_executor::comparison::BenchmarkResult;
use poly_bench_ir::ChartDirectiveIR;

use super::{escape_xml, filter_benchmarks, format_duration, sort_benchmarks};

// Bar dimensions
const DEFAULT_BAR_HEIGHT: i32 = 48;
const BAR_GAP: i32 = 8;

// Margins and spacing
const MARGIN_TOP: i32 = 72;
const MARGIN_BOTTOM: i32 = 156;
const MARGIN_LEFT: i32 = 50;
const MARGIN_RIGHT: i32 = 40;
const PLOT_PADDING: i32 = 12;
const GRID_GAP_X: i32 = 18;
const GRID_GAP_Y: i32 = 22;
const CARD_TITLE_HEIGHT: i32 = 22;
const MIN_CARD_WIDTH: i32 = 260;
const MAX_GRID_COLUMNS: i32 = 4;
const TARGET_COMBINED_GRID_HEIGHT: i32 = 420;

// Language colors (same for both themes)
const GO_COLOR: &str = "#00ADD8";
const TS_COLOR: &str = "#3178C6";
const RUST_COLOR: &str = "#DEA584";

// Accent colors (same for both themes)
const ACCENT_COLOR: &str = "#FF8A00";
const ACCENT_GLOW: &str = "#FFBA07";

// Font
const FONT_FAMILY: &str = "system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif";

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
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum BenchMetricMode {
    Fixed,
    Auto,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum OverallMetricMode {
    Fixed,
    Auto,
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

/// Generate a speedup chart showing relative performance vs baseline
pub fn generate(benchmarks: Vec<&BenchmarkResult>, directive: &ChartDirectiveIR) -> String {
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

    // Determine which languages have data
    let has_go = filtered.iter().any(|b| b.measurements.contains_key(&Lang::Go));
    let has_ts = filtered.iter().any(|b| b.measurements.contains_key(&Lang::TypeScript));
    let has_rust = filtered.iter().any(|b| b.measurements.contains_key(&Lang::Rust));

    let all_langs: Vec<Lang> = [
        if has_go { Some(Lang::Go) } else { None },
        if has_ts { Some(Lang::TypeScript) } else { None },
        if has_rust { Some(Lang::Rust) } else { None },
    ]
    .into_iter()
    .flatten()
    .collect();

    if all_langs.is_empty() {
        return empty_chart("No comparison data available", &theme);
    }

    // Calculate speedups with timing data
    let mut speedups: Vec<(String, BenchMetricMode, Vec<LangSpeedup>)> = Vec::new();
    let mut max_run_count: Option<u64> = None;
    let mut fixed_mode_count = 0usize;
    let mut auto_mode_count = 0usize;

    for bench in &filtered {
        if bench.measurements.contains_key(&baseline_lang) {
            let mut bench_speedups = Vec::new();
            for &lang in &all_langs {
                if let Some(m) = bench.measurements.get(&lang) {
                    // Track max run_count across all measurements
                    if let Some(rc) = m.run_count {
                        max_run_count = Some(max_run_count.map_or(rc, |c| c.max(rc)));
                    }

                    bench_speedups.push(LangSpeedup {
                        lang,
                        total_nanos: m.nanos_per_op * m.iterations as f64,
                        iterations: m.iterations,
                        run_count: m.run_count,
                    });
                }
            }
            if !bench_speedups.is_empty() {
                let bench_mode = infer_benchmark_mode(&bench_speedups);
                match bench_mode {
                    BenchMetricMode::Fixed => fixed_mode_count += 1,
                    BenchMetricMode::Auto => auto_mode_count += 1,
                }
                speedups.push((bench.name.clone(), bench_mode, bench_speedups));
            }
        }
    }

    if speedups.is_empty() {
        return empty_chart("No valid speedup data", &theme);
    }

    let mut language_summaries = Vec::new();
    for &lang in &all_langs {
        let mut lang_total_nanos = 0.0;
        let mut total_iterations = 0u64;
        let mut speedup_log_sum = 0.0;
        let mut speedup_samples = 0usize;

        for bench in &filtered {
            let baseline = bench.measurements.get(&baseline_lang);
            let current = bench.measurements.get(&lang);

            if let (Some(base), Some(cur)) = (baseline, current) {
                lang_total_nanos += cur.nanos_per_op * cur.iterations as f64;
                total_iterations += cur.iterations;
                speedup_log_sum += (base.nanos_per_op / cur.nanos_per_op).ln();
                speedup_samples += 1;
            }
        }

        if lang_total_nanos > 0.0 && speedup_samples > 0 {
            language_summaries.push(LangSummary {
                lang,
                total_nanos: lang_total_nanos,
                total_iterations,
                total_speedup: (speedup_log_sum / speedup_samples as f64).exp(),
                is_baseline: lang == baseline_lang,
            });
        }
    }

    let overall_mode = match (fixed_mode_count > 0, auto_mode_count > 0) {
        (true, true) => OverallMetricMode::Mixed,
        (false, true) => OverallMetricMode::Auto,
        _ => OverallMetricMode::Fixed,
    };

    let num_benchmarks = speedups.len() as i32;
    let num_langs = all_langs.len() as i32;
    let forced_row_count =
        directive.row_count.map(|c| (c as i32).max(1).min(num_benchmarks.max(1)));

    let default_columns = forced_row_count.unwrap_or(MAX_GRID_COLUMNS).max(1);
    let combined_min_width = 40 +
        (default_columns * MIN_CARD_WIDTH) +
        ((default_columns - 1) * GRID_GAP_X) +
        MARGIN_RIGHT;
    let chart_width = if num_benchmarks > 1 {
        directive.width.unwrap_or(combined_min_width).max(combined_min_width)
    } else {
        directive.width.unwrap_or(720)
    };
    let plot_y = MARGIN_TOP;
    let details_height = ((language_summaries.len() as i32).max(1) * 16) + 18;
    let combined_summary_height = ((language_summaries.len() as i32).max(1) * 24) + 30;
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
        let chart_height = directive.height.unwrap_or(MARGIN_TOP + plot_height + MARGIN_BOTTOM);
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
    let baseline_name = match baseline_lang {
        Lang::Go => "Go",
        Lang::TypeScript => "TypeScript",
        Lang::Rust => "Rust",
        _ => "Baseline",
    };
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
        "  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" transform=\"rotate(-90 {} {})\" font-family=\"{}\" font-size=\"10\" font-weight=\"600\" fill=\"{}\">{}</text>\n",
        y_axis_x,
        plot_y + plot_height / 2,
        y_axis_x,
        plot_y + plot_height / 2,
        FONT_FAMILY,
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
                "  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"{}\" font-size=\"11\" font-weight=\"600\" fill=\"{}\">{}</text>\n",
                card_x + grid_layout.card_width / 2,
                card_y + 15,
                FONT_FAMILY,
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
                let lang_color = get_lang_color(ls.lang);

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

                let lang_name = short_lang_name(ls.lang);
                let metric_label =
                    format_primary_metric(*bench_mode, ls.total_nanos, ls.iterations);
                let speedup_label = format!("{} · {}", lang_name, metric_label);
                let bar_end = inner_x + bar_width.max(6);
                let label_inside = bar_width > (row_width / 2);
                let (label_x, label_anchor, label_color) = if label_inside {
                    (bar_end - 8, "end", theme.text_primary)
                } else {
                    ((bar_end + 8).min(row_x + row_width - 8), "start", theme.text_secondary)
                };

                svg.push_str(&format!(
                    "  <text x=\"{}\" y=\"{}\" text-anchor=\"{}\" font-family=\"{}\" font-size=\"10\" font-weight=\"500\" fill=\"{}\">{}</text>\n",
                    label_x,
                    y + bar_height / 2 + 4,
                    label_anchor,
                    FONT_FAMILY,
                    label_color,
                    escape_xml(&speedup_label)
                ));
                svg.push_str(&format!(
                    "  <circle cx=\"{}\" cy=\"{}\" r=\"3\" fill=\"{}\"/>\n",
                    row_x - 8,
                    y + bar_height / 2,
                    lang_color
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
                let lang_color = get_lang_color(ls.lang);

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

                let lang_name = short_lang_name(ls.lang);
                let metric_label =
                    format_primary_metric(*bench_mode, ls.total_nanos, ls.iterations);
                let speedup_label = format!("{} · {}", lang_name, metric_label);
                let bar_end = inner_x + bar_width.max(6);
                let label_inside = bar_width > 220;
                let (label_x, label_anchor, label_color) = if label_inside {
                    (bar_end - 10, "end", theme.text_primary)
                } else {
                    (bar_end + 10, "start", theme.text_secondary)
                };

                svg.push_str(&format!(
                    "  <text x=\"{}\" y=\"{}\" text-anchor=\"{}\" font-family=\"{}\" font-size=\"11\" font-weight=\"500\" fill=\"{}\">{}</text>\n",
                    label_x,
                    y + bar_height / 2 + 4,
                    label_anchor,
                    FONT_FAMILY,
                    label_color,
                    escape_xml(&speedup_label)
                ));
                svg.push_str(&format!(
                    "  <circle cx=\"{}\" cy=\"{}\" r=\"3\" fill=\"{}\"/>\n",
                    plot_x - 12,
                    y + bar_height / 2,
                    lang_color
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
            &theme,
        ));
    }

    // X-axis label (mode aware)
    let x_axis_label = match overall_mode {
        OverallMetricMode::Fixed => "time (ms)",
        OverallMetricMode::Auto => "iterations",
        OverallMetricMode::Mixed => "time / iterations by mode",
    };
    svg.push_str(&format!(
        "  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"{}\" font-size=\"10\" font-weight=\"600\" fill=\"{}\">{}</text>\n",
        plot_x + plot_width / 2,
        x_axis_label_y,
        FONT_FAMILY,
        theme.text_muted,
        x_axis_label
    ));

    // Legend and detail stats below axis label.
    svg.push_str(&svg_legend(
        chart_width,
        legend_y,
        has_go,
        has_ts,
        has_rust,
        baseline_lang,
        &theme,
    ));
    svg.push_str(&svg_detail_legend(
        chart_width,
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
        footer_lines.push(
            "contains async-sequential benchmarks (internal caps: warmup<=5, samples<=50)"
                .to_string(),
        );
    }
    for (idx, line) in footer_lines.iter().enumerate() {
        let y = chart_height - 8 - ((footer_lines.len() as i32 - 1 - idx as i32) * 12);
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"{}\" font-size=\"9\" fill=\"{}\">{}</text>\n",
            chart_width / 2,
            y,
            FONT_FAMILY,
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
        <text x=\"200\" y=\"55\" text-anchor=\"middle\" font-family=\"{}\" font-size=\"13\" fill=\"{}\">{}</text>\
        </svg>",
        theme.bg_color, FONT_FAMILY, theme.text_muted, message
    )
}

/// SVG header with gradients and filters
fn svg_header(width: i32, height: i32, theme: &ThemeColors) -> String {
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\" fill=\"none\">\n\
<rect width=\"{}\" height=\"{}\" fill=\"{}\"/>\n\
<defs>\n\
  <radialGradient id=\"accentGrad\" cx=\"0.5\" cy=\"0.5\" r=\"0.8\">\n\
    <stop stop-color=\"{}\"/>\n\
    <stop offset=\"1\" stop-color=\"{}\"/>\n\
  </radialGradient>\n\
  <linearGradient id=\"goGrad\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">\n\
    <stop offset=\"0%\" stop-color=\"{}\" stop-opacity=\"0.95\"/>\n\
    <stop offset=\"100%\" stop-color=\"#0891B2\" stop-opacity=\"0.8\"/>\n\
  </linearGradient>\n\
  <linearGradient id=\"tsGrad\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">\n\
    <stop offset=\"0%\" stop-color=\"{}\" stop-opacity=\"0.95\"/>\n\
    <stop offset=\"100%\" stop-color=\"#1D4ED8\" stop-opacity=\"0.8\"/>\n\
  </linearGradient>\n\
  <linearGradient id=\"rustGrad\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">\n\
    <stop offset=\"0%\" stop-color=\"{}\" stop-opacity=\"0.95\"/>\n\
    <stop offset=\"100%\" stop-color=\"#B7410E\" stop-opacity=\"0.8\"/>\n\
  </linearGradient>\n\
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
        GO_COLOR, TS_COLOR, RUST_COLOR,
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
        OverallMetricMode::Mixed => format!(
            "Mixed modes · auto bars show iterations, fixed bars show runtime · speedup details vs {} in legend",
            baseline_name
        ),
    };
    format!(
        "<text x=\"{}\" y=\"28\" text-anchor=\"middle\" font-family=\"{}\" font-size=\"18\" font-weight=\"700\" fill=\"{}\">{}</text>\n\
<text x=\"{}\" y=\"48\" text-anchor=\"middle\" font-family=\"{}\" font-size=\"11\" fill=\"{}\">{}</text>\n",
        width / 2, FONT_FAMILY, theme.text_primary, escape_xml(title),
        width / 2, FONT_FAMILY, theme.text_muted, subtitle
    )
}

/// Legend section
fn svg_legend(
    width: i32,
    y: i32,
    has_go: bool,
    has_ts: bool,
    has_rust: bool,
    baseline_lang: Lang,
    theme: &ThemeColors,
) -> String {
    let mut items: Vec<(&str, &str, bool)> = Vec::new();

    if has_go {
        items.push((GO_COLOR, "Go", baseline_lang == Lang::Go));
    }
    if has_ts {
        items.push((TS_COLOR, "TypeScript", baseline_lang == Lang::TypeScript));
    }
    if has_rust {
        items.push((RUST_COLOR, "Rust", baseline_lang == Lang::Rust));
    }

    let item_width = 130;
    let total_width = items.len() as i32 * item_width;
    let start_x = (width - total_width) / 2;

    let mut svg = format!("<g transform=\"translate({},{})\">\n", start_x, y);

    for (i, (color, label, is_baseline)) in items.iter().enumerate() {
        let x = i as i32 * item_width;
        let display_label = if *is_baseline { format!("{} ★", label) } else { label.to_string() };

        svg.push_str(&format!(
            "  <circle cx=\"{}\" cy=\"0\" r=\"4\" fill=\"{}\"/>\n",
            x + 4,
            color
        ));

        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"4\" font-family=\"{}\" font-size=\"11\" fill=\"{}\">{}</text>\n",
            x + 14,
            FONT_FAMILY,
            theme.text_secondary,
            display_label
        ));
    }

    svg.push_str("</g>\n");
    svg
}

fn svg_detail_legend(
    width: i32,
    y: i32,
    height: i32,
    summaries: &[LangSummary],
    baseline_lang: Lang,
    mode: OverallMetricMode,
    theme: &ThemeColors,
) -> String {
    let box_x = 40;
    let box_width = (width - 80).max(220);
    let mut svg = String::new();

    svg.push_str(&format!(
        "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"8\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>\n",
        box_x,
        y,
        box_width,
        height,
        theme.detail_box_fill,
        theme.detail_box_stroke
    ));

    let mut text_y = y + 14;
    for summary in summaries {
        let lang_name = full_lang_name(summary.lang);
        let total = format_duration(summary.total_nanos);
        let iters = format_iterations_short(summary.total_iterations);
        let speedup =
            format_speedup_phrase(summary.total_speedup, summary.is_baseline, baseline_lang);
        let line = match mode {
            OverallMetricMode::Fixed => {
                format!("{lang_name}: total {total} · {iters} iter · {speedup}")
            }
            OverallMetricMode::Auto => {
                format!("{lang_name}: total {iters} iter · {total} runtime · {speedup}")
            }
            OverallMetricMode::Mixed => {
                format!("{lang_name}: total {iters} iter · {total} runtime · {speedup}")
            }
        };
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"{}\" font-size=\"10\" fill=\"{}\">{}</text>\n",
            box_x + (box_width / 2),
            text_y,
            FONT_FAMILY,
            theme.text_muted,
            escape_xml(&line)
        ));
        text_y += 16;
    }

    svg
}

fn svg_combined_summary(
    plot_x: i32,
    plot_width: i32,
    summary_max_width: i32,
    y: i32,
    height: i32,
    summaries: &[LangSummary],
    baseline_lang: Lang,
    theme: &ThemeColors,
) -> String {
    let box_width = plot_width.min(summary_max_width).max(300);
    let box_x = plot_x + (plot_width - box_width) / 2;
    let mut svg = String::new();
    let row_spacing = 24;
    let track_height = 15;
    let max_speedup = summaries.iter().map(|s| s.total_speedup).fold(1.0_f64, f64::max).max(1.0);

    svg.push_str(&format!(
        "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"8\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>\n",
        box_x, y, box_width, height, theme.detail_box_fill, theme.detail_box_stroke
    ));
    svg.push_str(&format!(
        "  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"{}\" font-size=\"10\" font-weight=\"600\" fill=\"{}\">Average gain across all benchmarks (geometric mean)</text>\n",
        box_x + box_width / 2,
        y + 14,
        FONT_FAMILY,
        theme.text_secondary
    ));

    let track_x = box_x + 170;
    let track_width = (box_width - 230).max(120);
    let baseline_x = track_x + ((track_width as f64) * (1.0 / max_speedup)) as i32;

    for (idx, summary) in summaries.iter().enumerate() {
        let row_y = y + 26 + (idx as i32 * row_spacing);
        let speedup = summary.total_speedup.max(0.0);
        let ratio = (speedup / max_speedup).clamp(0.0, 1.0);
        let fill_width = ((track_width as f64) * ratio) as i32;
        let label = if summary.is_baseline {
            "baseline".to_string()
        } else if speedup >= 1.0 {
            format!("{:.2}x faster", speedup)
        } else {
            format!("{:.2}x slower", 1.0 / speedup.max(1e-9))
        };

        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" font-family=\"{}\" font-size=\"10\" fill=\"{}\">{} · {}</text>\n",
            box_x + 12,
            row_y + 10,
            FONT_FAMILY,
            theme.text_muted,
            full_lang_name(summary.lang),
            label
        ));
        svg.push_str(&format!(
            "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"5\" fill=\"{}\"/>\n",
            track_x, row_y, track_width, track_height, theme.plot_bg
        ));
        svg.push_str(&format!(
            "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"5\" fill=\"{}\"/>\n",
            track_x,
            row_y,
            fill_width.max(4),
            track_height,
            get_lang_color(summary.lang)
        ));

        if summary.lang != baseline_lang {
            svg.push_str(&format!(
                "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"1\" stroke-opacity=\"0.65\"/>\n",
                baseline_x,
                row_y - 1,
                baseline_x,
                row_y + track_height + 1,
                theme.text_secondary
            ));
        }
    }

    svg
}

/// Get gradient ID for a language
fn lang_gradient_id(lang: Lang) -> &'static str {
    match lang {
        Lang::Go => "goGrad",
        Lang::TypeScript => "tsGrad",
        Lang::Rust => "rustGrad",
        _ => "goGrad",
    }
}

/// Get solid color for a language
fn get_lang_color(lang: Lang) -> &'static str {
    match lang {
        Lang::Go => GO_COLOR,
        Lang::TypeScript => TS_COLOR,
        Lang::Rust => RUST_COLOR,
        _ => GO_COLOR,
    }
}

fn short_lang_name(lang: Lang) -> &'static str {
    match lang {
        Lang::Go => "Go",
        Lang::TypeScript => "TS",
        Lang::Rust => "Rust",
        _ => "?",
    }
}

fn full_lang_name(lang: Lang) -> &'static str {
    match lang {
        Lang::Go => "Go",
        Lang::TypeScript => "TypeScript",
        Lang::Rust => "Rust",
        _ => "Unknown",
    }
}

fn format_speedup_phrase(speedup: f64, is_baseline: bool, baseline_lang: Lang) -> String {
    if is_baseline {
        return "baseline".to_string();
    }

    let baseline_name = full_lang_name(baseline_lang);
    if speedup >= 1.0 {
        format!("{:.2}x faster vs {}", speedup, baseline_name)
    } else {
        format!("{:.2}x slower vs {}", 1.0 / speedup, baseline_name)
    }
}

fn infer_benchmark_mode(bench_speedups: &[LangSpeedup]) -> BenchMetricMode {
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
    }
}

fn format_primary_metric(mode: BenchMetricMode, total_nanos: f64, iterations: u64) -> String {
    match mode {
        BenchMetricMode::Fixed => format_duration(total_nanos),
        BenchMetricMode::Auto => format!("{} iter", format_iterations_short(iterations)),
    }
}
