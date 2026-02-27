//! Display metadata for languages

/// Display metadata for a language (labels, colors, gradients).
#[derive(Debug, Clone, Copy)]
pub struct LangDisplayInfo {
    pub label: &'static str,
    pub full_name: &'static str,
    pub color: &'static str,
    pub gradient_id: &'static str,
    pub gradient_end: &'static str,
    pub terminal_color: &'static str,
    /// Optional emoji icon for reports (e.g. "🟢", "🔵")
    pub icon: Option<&'static str>,
}

impl LangDisplayInfo {
    pub const fn new(
        label: &'static str,
        full_name: &'static str,
        color: &'static str,
        gradient_id: &'static str,
        gradient_end: &'static str,
        terminal_color: &'static str,
    ) -> Self {
        Self { label, full_name, color, gradient_id, gradient_end, terminal_color, icon: None }
    }

    pub const fn new_with_icon(
        label: &'static str,
        full_name: &'static str,
        color: &'static str,
        gradient_id: &'static str,
        gradient_end: &'static str,
        terminal_color: &'static str,
        icon: &'static str,
    ) -> Self {
        Self {
            label,
            full_name,
            color,
            gradient_id,
            gradient_end,
            terminal_color,
            icon: Some(icon),
        }
    }
}
