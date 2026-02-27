//! Central display metadata for supported languages.
//!
//! Dispatches to runtime plugins for language-specific display info.

use poly_bench_dsl::Lang;
use poly_bench_traits::LangDisplayInfo;
use runtimes_c::C_PLUGIN;
use runtimes_csharp::CSHARP_PLUGIN;
use runtimes_go::GO_PLUGIN;
use runtimes_python::PYTHON_PLUGIN;
use runtimes_rust::RUST_PLUGIN;
use runtimes_ts::TS_PLUGIN;
use runtimes_zig::ZIG_PLUGIN;

static PLUGINS: &[&dyn poly_bench_traits::RuntimePlugin] =
    &[&GO_PLUGIN, &TS_PLUGIN, &RUST_PLUGIN, &PYTHON_PLUGIN, &C_PLUGIN, &CSHARP_PLUGIN, &ZIG_PLUGIN];

const FALLBACK: LangDisplayInfo =
    LangDisplayInfo::new("Unknown", "Unknown", "#9CA3AF", "goGrad", "#0891B2", "white");

/// Get display icon for a language (emoji), or "⚪" if not set
pub fn lang_icon(lang: Lang) -> &'static str {
    lang_display(lang).icon.unwrap_or("⚪")
}

/// Get display info for a language.
/// Returns fallback for unsupported languages.
pub fn lang_display(lang: Lang) -> LangDisplayInfo {
    PLUGINS.iter().find(|p| p.lang() == lang).map(|p| p.lang_display()).unwrap_or(FALLBACK)
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

/// End color for SVG gradient (used when building gradient defs dynamically)
pub fn lang_gradient_end(lang: Lang) -> &'static str {
    lang_display(lang).gradient_end
}
