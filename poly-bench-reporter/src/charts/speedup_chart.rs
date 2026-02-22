//! Speedup chart generator - shows relative performance vs baseline language
//!
//! Generates bar charts showing relative performance vs baseline with support for
//! light and dark themes.

use poly_bench_dsl::Lang;
use poly_bench_executor::comparison::BenchmarkResult;
use poly_bench_ir::ChartDirectiveIR;

use super::{escape_xml, filter_benchmarks, format_duration, sort_benchmarks};

// Bar dimensions
const DEFAULT_BAR_HEIGHT: i32 = 32;
const BAR_GAP: i32 = 8;
const GROUP_GAP: i32 = 20;

// Margins and spacing
const MARGIN_TOP: i32 = 72;
const MARGIN_BOTTOM: i32 = 70;
const MARGIN_LEFT: i32 = 20;
const MARGIN_RIGHT: i32 = 20;
const PLOT_PADDING: i32 = 12;

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
    grid_color: &'static str,
    grid_color_major: &'static str,
    axis_color: &'static str,
    bar_outline: &'static str,
    container_stroke: &'static str,
    plot_bg: &'static str,
}

impl ThemeColors {
    fn dark() -> Self {
        Self {
            bg_color: "#1E1E20",
            text_primary: "#FFFFFF",
            text_secondary: "rgba(255,255,255,0.7)",
            text_muted: "rgba(255,255,255,0.45)",
            text_dim: "rgba(255,255,255,0.35)",
            grid_color: "rgba(255,255,255,0.12)",
            grid_color_major: "rgba(255,255,255,0.4)",
            axis_color: "rgba(255,255,255,0.2)",
            bar_outline: "rgba(255,255,255,0.15)",
            container_stroke: "rgba(255,255,255,0.12)",
            plot_bg: "rgba(255,255,255,0.02)",
        }
    }

    fn light() -> Self {
        Self {
            bg_color: "#FFFFFF",
            text_primary: "#1A1A1A",
            text_secondary: "rgba(0,0,0,0.7)",
            text_muted: "rgba(0,0,0,0.5)",
            text_dim: "rgba(0,0,0,0.35)",
            grid_color: "rgba(0,0,0,0.08)",
            grid_color_major: "rgba(0,0,0,0.25)",
            axis_color: "rgba(0,0,0,0.15)",
            bar_outline: "rgba(0,0,0,0.1)",
            container_stroke: "rgba(0,0,0,0.08)",
            plot_bg: "rgba(0,0,0,0.02)",
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
    speedup: f64,
    is_baseline: bool,
    nanos_per_op: f64,
    iterations: u64,
    #[allow(dead_code)]
    run_count: Option<u64>,
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
    let mut speedups: Vec<(String, Vec<LangSpeedup>)> = Vec::new();
    let mut max_speedup: f64 = 0.0;
    let mut max_run_count: Option<u64> = None;

    for bench in &filtered {
        let baseline_time = bench.measurements.get(&baseline_lang).map(|m| m.nanos_per_op);

        if let Some(base_ns) = baseline_time {
            let mut bench_speedups = Vec::new();
            for &lang in &all_langs {
                if let Some(m) = bench.measurements.get(&lang) {
                    let speedup = base_ns / m.nanos_per_op;
                    let is_baseline = lang == baseline_lang;
                    max_speedup = max_speedup.max(speedup);

                    // Track max run_count across all measurements
                    if let Some(rc) = m.run_count {
                        max_run_count = Some(max_run_count.map_or(rc, |c| c.max(rc)));
                    }

                    bench_speedups.push(LangSpeedup {
                        lang,
                        speedup,
                        is_baseline,
                        nanos_per_op: m.nanos_per_op,
                        iterations: m.iterations,
                        run_count: m.run_count,
                    });
                }
            }
            if !bench_speedups.is_empty() {
                speedups.push((bench.name.clone(), bench_speedups));
            }
        }
    }

    if speedups.is_empty() {
        return empty_chart("No valid speedup data", &theme);
    }

    // Calculate tight scale
    max_speedup = max_speedup * 1.12;

    // Calculate dynamic name width
    let max_name_len = speedups.iter().map(|(name, _)| name.len()).max().unwrap_or(5);
    let name_width = (max_name_len as i32 * 8).min(90).max(40);

    let num_benchmarks = speedups.len() as i32;
    let num_langs = all_langs.len() as i32;
    let total_bars = num_benchmarks * num_langs;

    // Calculate bar area height
    let bar_area_height =
        total_bars * (DEFAULT_BAR_HEIGHT + BAR_GAP) + (num_benchmarks - 1) * GROUP_GAP - BAR_GAP;

    // Plot area dimensions
    let plot_height = bar_area_height + PLOT_PADDING * 2;

    // Total chart height - calculate dynamically
    let chart_height = directive.height.unwrap_or(MARGIN_TOP + plot_height + MARGIN_BOTTOM);
    let chart_width = directive.width.unwrap_or(720);

    // Plot area positioning
    let plot_x = MARGIN_LEFT + name_width + 12;
    let plot_y = MARGIN_TOP;
    let plot_width = chart_width - plot_x - MARGIN_RIGHT;

    let mut svg = svg_header(chart_width, chart_height, &theme);

    // Title
    let title = directive.title.as_deref().unwrap_or("Speedup vs Baseline");
    let baseline_name = match baseline_lang {
        Lang::Go => "Go",
        Lang::TypeScript => "TypeScript",
        Lang::Rust => "Rust",
        _ => "Baseline",
    };
    svg.push_str(&svg_title(chart_width, title, baseline_name, &theme));

    // Plot area container with rounded border
    svg.push_str(&format!(
        "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"12\" fill=\"none\" stroke=\"{}\" stroke-width=\"1.5\"/>\n",
        plot_x - 6,
        plot_y - 6,
        plot_width + 12,
        plot_height + 12,
        theme.container_stroke
    ));

    // Plot area background
    svg.push_str(&format!(
        "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"8\" fill=\"{}\"/>\n",
        plot_x, plot_y, plot_width, plot_height, theme.plot_bg
    ));

    // Gold accent bar beside Y-axis (left edge of plot)
    svg.push_str(&format!(
        "  <rect x=\"{}\" y=\"{}\" width=\"6\" height=\"{}\" rx=\"3\" fill=\"url(#accentGrad)\" fill-opacity=\"0.96\" filter=\"url(#accentGlow)\"/>\n",
        plot_x - 6,
        plot_y,
        plot_height
    ));

    // Calculate smart tick values
    let ticks = calculate_smart_ticks(max_speedup);

    // X-axis line
    svg.push_str(&format!(
        "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>\n",
        plot_x,
        plot_y + plot_height,
        plot_x + plot_width,
        plot_y + plot_height,
        theme.axis_color
    ));

    // Draw bars first (so grid lines render on top)
    let bar_height = DEFAULT_BAR_HEIGHT;
    let mut y = plot_y + PLOT_PADDING;

    for (bench_name, bench_speedups) in &speedups {
        // Only show benchmark name label if there are multiple benchmarks
        if num_benchmarks > 1 {
            let label_y =
                y + (bench_speedups.len() as i32 * (bar_height + BAR_GAP) - BAR_GAP) / 2 + 5;
            svg.push_str(&format!(
                "  <text x=\"{}\" y=\"{}\" text-anchor=\"end\" font-family=\"{}\" font-size=\"12\" font-weight=\"600\" fill=\"{}\">{}</text>\n",
                plot_x - 12,
                label_y,
                FONT_FAMILY,
                theme.text_primary,
                escape_xml(bench_name)
            ));
        }

        for ls in bench_speedups {
            let bar_width = ((ls.speedup / max_speedup) * plot_width as f64) as i32;
            let gradient_id = lang_gradient_id(ls.lang);
            let lang_color = get_lang_color(ls.lang);

            // Bar with gradient fill and subtle outline
            svg.push_str(&format!(
                "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"6\" fill=\"url(#{})\" stroke=\"{}\" stroke-width=\"1\" filter=\"url(#barShadow)\"/>\n",
                plot_x,
                y,
                bar_width.max(6),
                bar_height,
                gradient_id,
                theme.bar_outline
            ));

            // Language label with timing and iterations info
            let lang_name = match ls.lang {
                Lang::Go => "Go",
                Lang::TypeScript => "TS",
                Lang::Rust => "Rust",
                _ => "?",
            };

            let timing_str = format_duration(ls.nanos_per_op);
            let iter_str = format_iterations_short(ls.iterations);

            let speedup_label = if ls.is_baseline {
                format!("{} · {} · {} iter (baseline)", lang_name, timing_str, iter_str)
            } else if ls.speedup >= 1.0 {
                format!(
                    "{} · {} · {} iter · {:.2}x faster",
                    lang_name, timing_str, iter_str, ls.speedup
                )
            } else {
                format!(
                    "{} · {} · {} iter · {:.2}x slower",
                    lang_name,
                    timing_str,
                    iter_str,
                    1.0 / ls.speedup
                )
            };

            // Position label: inside if wide enough, otherwise outside
            let label_inside = bar_width > 280;
            let (label_x, label_anchor, label_color) = if label_inside {
                (plot_x + bar_width - 10, "end", theme.text_primary)
            } else {
                (plot_x + bar_width + 10, "start", theme.text_secondary)
            };

            svg.push_str(&format!(
                "  <text x=\"{}\" y=\"{}\" text-anchor=\"{}\" font-family=\"{}\" font-size=\"11\" font-weight=\"500\" fill=\"{}\">{}</text>\n",
                label_x,
                y + bar_height / 2 + 4,
                label_anchor,
                FONT_FAMILY,
                label_color,
                speedup_label
            ));

            // Language color indicator dot (positioned left of the gold accent bar)
            svg.push_str(&format!(
                "  <circle cx=\"{}\" cy=\"{}\" r=\"3\" fill=\"{}\"/>\n",
                plot_x - 18,
                y + bar_height / 2,
                lang_color
            ));

            y += bar_height + BAR_GAP;
        }

        y += GROUP_GAP - BAR_GAP;
    }

    // Draw grid lines ON TOP of bars (brighter and more visible)
    for &tick_value in &ticks {
        if tick_value > max_speedup || tick_value == 0.0 {
            continue;
        }
        let x = plot_x + ((tick_value / max_speedup) * plot_width as f64) as i32;

        // Use stronger styling for "round" values (0.5, 1.0, 1.5, 2.0, etc)
        let is_major = (tick_value * 2.0).fract().abs() < 0.001;
        let (grid_stroke, stroke_width) =
            if is_major { (theme.grid_color_major, "1.5") } else { (theme.grid_color, "1") };

        // Vertical grid line
        svg.push_str(&format!(
            "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\" stroke-dasharray=\"4,4\"/>\n",
            x,
            plot_y,
            x,
            plot_y + plot_height,
            grid_stroke,
            stroke_width
        ));

        // Tick label below plot
        let label = format_tick_label(tick_value);
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"{}\" font-size=\"10\" fill=\"{}\">{}</text>\n",
            x,
            plot_y + plot_height + 18,
            FONT_FAMILY,
            theme.text_muted,
            label
        ));
    }

    // Legend at bottom with extra gap
    let legend_y = chart_height - 38;
    svg.push_str(&svg_legend(
        chart_width,
        legend_y,
        has_go,
        has_ts,
        has_rust,
        baseline_lang,
        &theme,
    ));

    // Footer info - show run count if applicable
    if let Some(run_count) = max_run_count {
        if run_count > 1 {
            svg.push_str(&format!(
                "  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"{}\" font-size=\"9\" fill=\"{}\">averaged over {} runs</text>\n",
                chart_width / 2,
                chart_height - 8,
                FONT_FAMILY,
                theme.text_dim,
                run_count
            ));
        }
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
fn svg_title(width: i32, title: &str, baseline_name: &str, theme: &ThemeColors) -> String {
    let subtitle = format!("Performance relative to {} baseline", baseline_name);
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

    let item_width = 100;
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

/// Calculate smart tick values that fit the data
fn calculate_smart_ticks(max_value: f64) -> Vec<f64> {
    let rough_step = max_value / 4.0;
    let magnitude = 10_f64.powf(rough_step.log10().floor());
    let normalized = rough_step / magnitude;

    let nice_step = if normalized <= 1.0 {
        magnitude
    } else if normalized <= 2.0 {
        2.0 * magnitude
    } else if normalized <= 5.0 {
        5.0 * magnitude
    } else {
        10.0 * magnitude
    };

    let step = if max_value <= 2.0 {
        0.5
    } else if max_value <= 3.0 {
        0.5
    } else if max_value <= 5.0 {
        1.0
    } else {
        nice_step.max(1.0)
    };

    let mut ticks = Vec::new();
    let mut tick = 0.0;
    while tick <= max_value + 0.001 {
        ticks.push(tick);
        tick += step;
    }

    // Always include 1.0 (baseline) if not already present
    if !ticks.iter().any(|&t| (t - 1.0).abs() < 0.001) {
        ticks.push(1.0);
        ticks.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    ticks
}

/// Format tick label nicely
fn format_tick_label(value: f64) -> String {
    if value == 0.0 {
        "0".to_string()
    } else if value == 1.0 {
        "1x".to_string()
    } else if value.fract() == 0.0 {
        format!("{}x", value as i32)
    } else {
        format!("{:.1}x", value)
    }
}
