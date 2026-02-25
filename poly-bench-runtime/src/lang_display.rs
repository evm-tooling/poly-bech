//! Central display metadata for supported languages.
//!
//! This module is the single source of truth for labels, colors, and gradients
//! used across the executor, reporter, and CLI. Adding a new runtime requires
//! registering display info here so all integrations stay consistent.

use poly_bench_dsl::Lang;

/// Display metadata for a language (labels, colors, gradients).
/// Used by executor, reporter, and CLI for consistent presentation.
#[derive(Debug, Clone, Copy)]
pub struct LangDisplayInfo {
    /// Short label for console output (e.g. "Go", "TS", "Rust", "Python")
    pub label: &'static str,
    /// Full name for reports (e.g. "Go", "TypeScript", "Rust", "Python")
    pub full_name: &'static str,
    /// Hex color for charts (e.g. "#00ADD8")
    pub color: &'static str,
    /// SVG gradient ID for charts
    pub gradient_id: &'static str,
    /// End color for gradient definition
    pub gradient_end: &'static str,
    /// Terminal color name for colored crate ("green", "cyan", "yellow", "bright_blue")
    pub terminal_color: &'static str,
}

impl LangDisplayInfo {
    const fn new(
        label: &'static str,
        full_name: &'static str,
        color: &'static str,
        gradient_id: &'static str,
        gradient_end: &'static str,
        terminal_color: &'static str,
    ) -> Self {
        Self { label, full_name, color, gradient_id, gradient_end, terminal_color }
    }
}

/// Fallback when a language has no display info (e.g. future languages)
const FALLBACK: LangDisplayInfo = LangDisplayInfo::new(
    "Unknown", "Unknown", "#9CA3AF", // TIE_COLOR
    "goGrad", "#0891B2", "white",
);

/// Get display info for a language.
/// Returns fallback for unsupported languages.
pub fn lang_display(lang: Lang) -> LangDisplayInfo {
    match lang {
        Lang::Go => LangDisplayInfo::new("Go", "Go", "#00ADD8", "goGrad", "#0891B2", "green"),
        Lang::TypeScript => {
            LangDisplayInfo::new("TS", "TypeScript", "#3178C6", "tsGrad", "#1D4ED8", "cyan")
        }
        Lang::Rust => {
            LangDisplayInfo::new("Rust", "Rust", "#DEA584", "rustGrad", "#B7410E", "yellow")
        }
        Lang::Python => LangDisplayInfo::new(
            "Python",
            "Python",
            "#3776AB",
            "pythonGrad",
            "#FFD43B",
            "bright_blue",
        ),
        _ => FALLBACK,
    }
}

/// Short label for console (alias for lang_display(lang).label)
pub fn lang_label(lang: Lang) -> &'static str {
    lang_display(lang).label
}

/// Full name for reports
pub fn lang_full_name(lang: Lang) -> &'static str {
    lang_display(lang).full_name
}

/// Hex color for charts
pub fn lang_color(lang: Lang) -> &'static str {
    lang_display(lang).color
}

/// SVG gradient ID
pub fn lang_gradient_id(lang: Lang) -> &'static str {
    lang_display(lang).gradient_id
}
