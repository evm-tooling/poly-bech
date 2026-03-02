//! T3-style init UI: │ ◇ prompts, blocky logo, green success block.

use console::style;
use dialoguer::theme::{ColorfulTheme, Theme};
use std::fmt::Write;

fn pipe_grey() -> console::StyledObject<&'static str> {
    style("│").dim()
}

fn diamond_grey() -> console::StyledObject<&'static str> {
    style("◇").dim()
}

fn diamond_green() -> console::StyledObject<&'static str> {
    style("◇").green()
}

fn diamond_red() -> console::StyledObject<&'static str> {
    style("◇").red()
}

fn pipe_red() -> console::StyledObject<&'static str> {
    style("│").red()
}

/// Theme that formats prompts with │ and ◇ (create-t3-app style).
pub struct T3StyleTheme(ColorfulTheme);

impl T3StyleTheme {
    pub fn new() -> Self {
        let mut inner = ColorfulTheme::default();
        inner.checked_item_prefix = style("● ".to_string()).green();
        inner.unchecked_item_prefix = style("○ ".to_string()).dim();
        inner.active_item_prefix = style("› ".to_string()).cyan();
        inner.inactive_item_prefix = style("  ".to_string());
        Self(inner)
    }
}

impl Default for T3StyleTheme {
    fn default() -> Self {
        Self::new()
    }
}

impl Theme for T3StyleTheme {
    fn format_prompt(&self, f: &mut dyn Write, prompt: &str) -> std::fmt::Result {
        self.0.format_prompt(f, prompt)
    }
    fn format_error(&self, f: &mut dyn Write, err: &str) -> std::fmt::Result {
        // T3-style error: pipe + diamond in red, error message in red
        write!(f, "{}  {}", pipe_red(), style(err).red())
    }
    fn format_confirm_prompt(
        &self,
        f: &mut dyn Write,
        prompt: &str,
        default: Option<bool>,
    ) -> std::fmt::Result {
        self.0.format_confirm_prompt(f, prompt, default)
    }
    fn format_confirm_prompt_selection(
        &self,
        f: &mut dyn Write,
        prompt: &str,
        selection: Option<bool>,
    ) -> std::fmt::Result {
        self.0.format_confirm_prompt_selection(f, prompt, selection)
    }
    fn format_input_prompt(
        &self,
        f: &mut dyn Write,
        prompt: &str,
        default: Option<&str>,
    ) -> std::fmt::Result {
        // Active step (pending): dim
        write!(f, "\n{}\n{}  {}", pipe_grey(), diamond_grey(), prompt)?;
        if let Some(d) = default {
            write!(f, " {}", style(d).dim())?;
        }
        writeln!(f)?;
        write!(f, "{}  ", pipe_grey())
    }
    fn format_input_prompt_selection(
        &self,
        f: &mut dyn Write,
        prompt: &str,
        sel: &str,
    ) -> std::fmt::Result {
        // Completed step: green
        write!(
            f,
            "\n{}\n{}  {}\n{}  {}\n{}",
            pipe_grey(),
            diamond_green(),
            style(prompt).green(),
            pipe_grey(),
            style(sel).green(),
            pipe_grey()
        )
    }
    fn format_password_prompt(&self, f: &mut dyn Write, prompt: &str) -> std::fmt::Result {
        self.0.format_password_prompt(f, prompt)
    }
    fn format_password_prompt_selection(
        &self,
        f: &mut dyn Write,
        prompt: &str,
    ) -> std::fmt::Result {
        self.0.format_password_prompt_selection(f, prompt)
    }
    fn format_select_prompt(&self, f: &mut dyn Write, prompt: &str) -> std::fmt::Result {
        write!(f, "{}  {}", diamond_grey(), prompt)
    }
    fn format_select_prompt_selection(
        &self,
        f: &mut dyn Write,
        _prompt: &str,
        sel: &str,
    ) -> std::fmt::Result {
        // Prompt is already on screen from format_select_prompt; only output selection to avoid
        // duplicate
        write!(f, "{}  {}\n{}\n", pipe_grey(), style(sel).green(), pipe_grey())
    }
    fn format_multi_select_prompt(&self, f: &mut dyn Write, prompt: &str) -> std::fmt::Result {
        // Active step (pending): dim
        write!(f, "{}  {}", diamond_grey(), prompt)
    }
    fn format_multi_select_prompt_selection(
        &self,
        f: &mut dyn Write,
        prompt: &str,
        selections: &[&str],
    ) -> std::fmt::Result {
        let sel_str = selections.join(", ");
        // Completed step: green. No leading \n or pipe (replaces menu in-place).
        write!(
            f,
            "{}  {}\n{}  {}\n{}\n",
            diamond_green(),
            style(prompt).green(),
            pipe_grey(),
            style(sel_str).green(),
            pipe_grey()
        )
    }
    fn format_sort_prompt(&self, f: &mut dyn Write, prompt: &str) -> std::fmt::Result {
        self.0.format_sort_prompt(f, prompt)
    }
    fn format_sort_prompt_selection(
        &self,
        f: &mut dyn Write,
        prompt: &str,
        selections: &[&str],
    ) -> std::fmt::Result {
        self.0.format_sort_prompt_selection(f, prompt, selections)
    }
    fn format_select_prompt_item(
        &self,
        f: &mut dyn Write,
        text: &str,
        active: bool,
    ) -> std::fmt::Result {
        self.0.format_select_prompt_item(f, text, active)
    }
    fn format_multi_select_prompt_item(
        &self,
        f: &mut dyn Write,
        text: &str,
        checked: bool,
        active: bool,
    ) -> std::fmt::Result {
        self.0.format_multi_select_prompt_item(f, text, checked, active)
    }
    fn format_sort_prompt_item(
        &self,
        f: &mut dyn Write,
        text: &str,
        picked: bool,
        active: bool,
    ) -> std::fmt::Result {
        self.0.format_sort_prompt_item(f, text, picked, active)
    }
}

/// Blocky ASCII logo for init flow (same as welcome: "Poly Bench" in yellow).
pub fn print_init_logo() {
    println!();
    crate::welcome::print_poly_bench_logo();
    println!();
}

/// Print T3-style scaffolding complete message (before build).
pub fn print_scaffold_complete(project_name: &str) {
    use colored::Colorize;
    println!();
    println!("{}", format!("✔ {} scaffolded successfully!", project_name).green().bold());
    println!();
    println!("Adding boilerplate...");
    println!("{}", "✔ Successfully setup boilerplate for polybench.toml".green());
    println!("{}", "✔ Successfully setup boilerplate for benchmarks/".green());
    println!("{}", "✔ Successfully setup boilerplate for .polybench/".green());
    println!();
}

/// Print T3-style next steps message (after build).
/// If `build_ran` is false, includes instruction to run build first.
pub fn print_next_steps(build_ran: bool) {
    println!();
    println!("Next steps:");
    if !build_ran {
        println!("  poly-bench build      # Install LSP servers and dependencies");
    }
    println!("  poly-bench run        # Run benchmarks");
    println!();
}

/// Init step labels for error display.
pub const INIT_STEP_1: &str = "What will your project be called?";
pub const INIT_STEP_2: &str = "Which languages to include?";
pub const INIT_STEP_3: &str = "Scaffolding project";
pub const INIT_STEP_4: &str = "Building project";

/// Print T3-style error block: step progress with failed step in red, then pretty error.
/// Uses Debug for miette::Report (graphical handler with syntax highlighting).
pub fn print_init_error_block<E: std::fmt::Debug>(
    completed_steps: &[(&str, &str)], // (prompt, value)
    failed_step: &str,
    error: &E,
) {
    use colored::Colorize;
    use std::io::Write;

    let stderr = std::io::stderr();
    let mut w = stderr.lock();

    let _ = writeln!(w);
    let _ = writeln!(w, "{}", pipe_grey());
    for (prompt, value) in completed_steps {
        let _ = writeln!(w, "{}  {}", diamond_green(), style(prompt).green());
        let _ = writeln!(w, "{}  {}", pipe_grey(), style(value).green());
        let _ = writeln!(w, "{}", pipe_grey());
    }
    // Failed step in red
    let _ = writeln!(w, "{}  {}", diamond_red(), style(failed_step).red().bold());
    let _ = writeln!(w, "{}", pipe_red());
    let _ = writeln!(w);

    // Debug format: miette::Report uses graphical handler (code-colored, snippets)
    let _ = writeln!(w, "{:?}", error);
    let _ = writeln!(w);
}

/// Print T3-style rejection when no runtimes remain (e.g. user skipped all installs).
/// Only prints the failed step + error (steps 1 and 2 are already on screen from language
/// selection).
pub fn print_init_runtime_rejection(
    failed_step_prompt: &str,
    failed_step_value: &str,
    error_msg: &str,
) {
    use colored::Colorize;
    use std::io::Write;

    let stderr = std::io::stderr();
    let mut w = stderr.lock();

    // Failed step in red (bullet + prompt, then pipe + value)
    let _ = writeln!(w, "{}  {}", diamond_red(), style(failed_step_prompt).red().bold());
    let _ = writeln!(w, "{}  {}", pipe_red(), style(failed_step_value).red());
    let _ = writeln!(w, "{}", pipe_red());
    let _ = writeln!(w);
    // Error in red
    let _ = writeln!(w, "{}", format!("Error: {}", error_msg).red().bold());
    let _ = writeln!(w);
}
