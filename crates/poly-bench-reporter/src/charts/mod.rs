//! Modular chart generators for benchmark visualization
//!
//! Provides bar charts, pie charts, and line charts for visualizing benchmark results.

pub mod bar_chart;
pub mod pie_chart;
pub mod line_chart;

use poly_bench_dsl::Lang;

// Default chart dimensions
pub const DEFAULT_WIDTH: i32 = 880;
pub const DEFAULT_MARGIN_TOP: i32 = 60;
pub const DEFAULT_MARGIN_BOTTOM: i32 = 40;
pub const DEFAULT_MARGIN_LEFT: i32 = 200;
pub const DEFAULT_MARGIN_RIGHT: i32 = 100;

// Default colors
pub const GO_COLOR: &str = "#00ADD8";
pub const TS_COLOR: &str = "#3178C6";
pub const TIE_COLOR: &str = "#9CA3AF";
pub const BG_COLOR: &str = "#FAFAFA";
pub const BORDER_COLOR: &str = "#E5E7EB";
pub const TEXT_COLOR: &str = "#111827";
pub const TEXT_SECONDARY: &str = "#6B7280";
pub const TEXT_TERTIARY: &str = "#4B5563";
pub const TEXT_MUTED: &str = "#9CA3AF";
pub const GO_GRADIENT_END: &str = "#0891B2";
pub const TS_GRADIENT_END: &str = "#1D4ED8";

// Color palette for pie charts
pub const PIE_COLORS: &[&str] = &[
    "#00ADD8", // Go blue
    "#3178C6", // TS blue
    "#10B981", // Emerald
    "#F59E0B", // Amber
    "#EF4444", // Red
    "#8B5CF6", // Violet
    "#EC4899", // Pink
    "#06B6D4", // Cyan
];

/// Escape XML special characters
pub fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Get color for a language
pub fn lang_color(lang: Lang) -> &'static str {
    match lang {
        Lang::Go => GO_COLOR,
        Lang::TypeScript => TS_COLOR,
        _ => TIE_COLOR,
    }
}

/// Format duration for display
pub fn format_duration(nanos: f64) -> String {
    if nanos >= 1_000_000_000.0 {
        format!("{:.2}s", nanos / 1_000_000_000.0)
    } else if nanos >= 1_000_000.0 {
        format!("{:.2}ms", nanos / 1_000_000.0)
    } else if nanos >= 1_000.0 {
        format!("{:.2}µs", nanos / 1_000.0)
    } else {
        format!("{:.0}ns", nanos)
    }
}

/// Generate SVG header with common styles
pub fn svg_header(width: i32, height: i32) -> String {
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n\
<defs>\n\
  <linearGradient id=\"goGrad\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">\n\
    <stop offset=\"0%\" stop-color=\"{}\" stop-opacity=\"0.95\"/>\n\
    <stop offset=\"100%\" stop-color=\"{}\" stop-opacity=\"0.85\"/>\n\
  </linearGradient>\n\
  <linearGradient id=\"tsGrad\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">\n\
    <stop offset=\"0%\" stop-color=\"{}\" stop-opacity=\"0.95\"/>\n\
    <stop offset=\"100%\" stop-color=\"{}\" stop-opacity=\"0.85\"/>\n\
  </linearGradient>\n\
</defs>\n\
<rect width=\"{}\" height=\"{}\" fill=\"{}\" rx=\"12\"/>\n\
<rect x=\".5\" y=\".5\" width=\"{}\" height=\"{}\" fill=\"none\" stroke=\"{}\" rx=\"12\"/>\n",
        width, height, width, height,
        GO_COLOR, GO_GRADIENT_END,
        TS_COLOR, TS_GRADIENT_END,
        width, height, BG_COLOR,
        width - 1, height - 1, BORDER_COLOR
    )
}

/// Generate title text
pub fn svg_title(width: i32, title: &str, subtitle: Option<&str>) -> String {
    let mut svg = format!(
        "<text x=\"{}\" y=\"30\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"16\" font-weight=\"700\" fill=\"{}\">{}</text>\n",
        width / 2,
        TEXT_COLOR,
        escape_xml(title)
    );

    if let Some(sub) = subtitle {
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"48\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"11\" fill=\"{}\">{}</text>\n",
            width / 2,
            TEXT_SECONDARY,
            escape_xml(sub)
        ));
    }

    svg
}

/// Generate a legend
pub fn svg_legend(width: i32, y: i32, items: &[(&str, &str)]) -> String {
    let mut svg = format!("<g transform=\"translate({},{})\">\n", width / 2 - (items.len() as i32 * 50), y);

    for (i, (color, label)) in items.iter().enumerate() {
        let x = i as i32 * 100;
        svg.push_str(&format!(
            "  <rect x=\"{}\" width=\"14\" height=\"14\" fill=\"{}\" rx=\"3\"/>\n\
  <text x=\"{}\" y=\"11\" font-family=\"sans-serif\" font-size=\"11\" fill=\"{}\">{}</text>\n",
            x, color, x + 18, TEXT_TERTIARY, escape_xml(label)
        ));
    }

    svg.push_str("</g>\n");
    svg
}
