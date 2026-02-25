use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub fn section(title: &str) {
    println!();
    println!("{}", "─".repeat(78).dimmed());
    println!("{} {}", "■".cyan(), title.bold());
    println!("{}", "─".repeat(78).dimmed());
}

pub fn subsection(title: &str) {
    println!();
    println!("{} {}", "▸".cyan(), title.bold());
}

pub fn kv(label: &str, value: impl AsRef<str>) {
    println!("  {:<18} {}", format!("{}:", label).dimmed(), value.as_ref());
}

pub fn info(message: impl AsRef<str>) {
    println!("  {} {}", "ℹ".blue(), message.as_ref());
}

pub fn success(message: impl AsRef<str>) {
    println!("  {} {}", "✓".green().bold(), message.as_ref());
}

pub fn failure(message: impl AsRef<str>) {
    eprintln!("  {} {}", "✗".red().bold(), message.as_ref());
}

pub fn indented_line(message: impl AsRef<str>) {
    println!("    {}", message.as_ref());
}

pub fn spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .expect("valid spinner template")
            .tick_strings(SPINNER_FRAMES),
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}
