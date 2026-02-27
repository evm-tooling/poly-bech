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
        self.0.format_error(f, err)
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
        write!(
            f,
            "\n{}\n{}  {}\n{}  {}\n{}\n",
            pipe_grey(),
            diamond_grey(),
            prompt,
            pipe_grey(),
            sel,
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
        self.0.format_select_prompt(f, prompt)
    }
    fn format_select_prompt_selection(
        &self,
        f: &mut dyn Write,
        prompt: &str,
        sel: &str,
    ) -> std::fmt::Result {
        self.0.format_select_prompt_selection(f, prompt, sel)
    }
    fn format_multi_select_prompt(&self, f: &mut dyn Write, prompt: &str) -> std::fmt::Result {
        write!(f, "\n{}\n{}  {}\n", pipe_grey(), diamond_grey(), prompt)
    }
    fn format_multi_select_prompt_selection(
        &self,
        f: &mut dyn Write,
        prompt: &str,
        selections: &[&str],
    ) -> std::fmt::Result {
        let sel_str = selections.join(", ");
        write!(
            f,
            "\n{}\n{}  {}\n{}  {}\n{}\n",
            pipe_grey(),
            diamond_grey(),
            prompt,
            pipe_grey(),
            sel_str,
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

/// Print T3-style success block after scaffold (✔ lines in green).
pub fn print_init_success_block(project_name: &str) {
    use colored::Colorize;
    println!();
    println!("{}", format!("✔ {} scaffolded successfully!", project_name).green().bold());
    println!();
    println!("Adding boilerplate...");
    println!("{}", "✔ Successfully setup boilerplate for polybench.toml".green());
    println!("{}", "✔ Successfully setup boilerplate for benchmarks/".green());
    println!("{}", "✔ Successfully setup boilerplate for .polybench/".green());
    println!();
    println!("Next steps:");
    println!("  {}", format!("cd {}", project_name).cyan());
    println!("  poly-bench run        # Run benchmarks");
    println!();
}

/// Print T3-style success block for init in current directory (no cd needed).
pub fn print_init_success_block_current_dir(project_name: &str) {
    use colored::Colorize;
    println!();
    println!("{}", format!("✔ {} scaffolded successfully!", project_name).green().bold());
    println!();
    println!("Adding boilerplate...");
    println!("{}", "✔ Successfully setup boilerplate for polybench.toml".green());
    println!("{}", "✔ Successfully setup boilerplate for benchmarks/".green());
    println!("{}", "✔ Successfully setup boilerplate for .polybench/".green());
    println!();
    println!("Next steps:");
    println!("  poly-bench run        # Run benchmarks");
    println!();
}
