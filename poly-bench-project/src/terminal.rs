//! Terminal output utilities for poly-bench CLI
//!
//! Provides spinners, progress indicators, and clean output formatting
//! to mask underlying subprocess noise (go get, npm install, etc.)

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    process::{Command, Output, Stdio},
    time::Duration,
};

/// Minimum display time for spinners (500ms) so users can follow progress
const MIN_DISPLAY_MS: u64 = 500;
const ERROR_EXCERPT_MAX_LINES: usize = 24;

/// Create a spinner for a step with the [±] style prefix
pub fn step_spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.set_message(msg.to_string());
    pb.enable_steady_tick(Duration::from_millis(120));
    pb
}

/// Create an indented spinner (for sub-steps)
pub fn indented_spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("  {spinner:.cyan} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.set_message(msg.to_string());
    pb.enable_steady_tick(Duration::from_millis(120));
    pb
}

/// Ensure minimum display time for a spinner before finishing
pub fn ensure_min_display(elapsed: Duration) {
    if elapsed < Duration::from_millis(MIN_DISPLAY_MS) {
        std::thread::sleep(Duration::from_millis(MIN_DISPLAY_MS) - elapsed);
    }
}

/// Finish a spinner with a success message
pub fn finish_success(pb: &ProgressBar, msg: &str) {
    ensure_min_display(pb.elapsed());
    pb.finish_and_clear();
    println!("{} {}", "✓".green().bold(), msg);
}

/// Finish a spinner with a success message (indented)
pub fn finish_success_indented(pb: &ProgressBar, msg: &str) {
    ensure_min_display(pb.elapsed());
    pb.finish_and_clear();
    println!("  {} {}", "✓".green(), msg);
}

/// Finish a spinner with a failure message
pub fn finish_failure(pb: &ProgressBar, msg: &str) {
    ensure_min_display(pb.elapsed());
    pb.finish_and_clear();
    eprintln!("{} {}", "✗".red().bold(), msg);
}

/// Finish a spinner with a failure message (indented)
pub fn finish_failure_indented(pb: &ProgressBar, msg: &str) {
    ensure_min_display(pb.elapsed());
    pb.finish_and_clear();
    eprintln!("  {} {}", "✗".red(), msg);
}

/// Finish a spinner with a warning message (indented)
pub fn finish_warning_indented(pb: &ProgressBar, msg: &str) {
    ensure_min_display(pb.elapsed());
    pb.finish_and_clear();
    eprintln!("  {} {}", "⚠".yellow(), msg);
}

/// Print a success message (no spinner)
pub fn success(msg: &str) {
    println!("{} {}", "✓".green().bold(), msg);
}

/// Print an indented success message (no spinner)
pub fn success_indented(msg: &str) {
    println!("  {} {}", "✓".green(), msg);
}

/// Print a failure message (no spinner)
pub fn failure(msg: &str) {
    eprintln!("{} {}", "✗".red().bold(), msg);
}

/// Print an indented failure message (no spinner)
pub fn failure_indented(msg: &str) {
    eprintln!("  {} {}", "✗".red(), msg);
}

/// Print an indented info message
pub fn info_indented(msg: &str) {
    println!("  {} {}", "·".dimmed(), msg);
}

/// Create a progress bar for downloads with known size (shows bytes and percent).
pub fn download_progress_bar(total: u64, msg: &str) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("  {msg} [{wide_bar:.green}] {bytes}/{total_bytes} ({percent}%)")
            .unwrap()
            .progress_chars("█░"),
    );
    pb.set_message(msg.to_string());
    pb
}

/// Create a spinner for downloads with unknown size. Call pb.inc(n) as bytes are read.
pub fn download_spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("  {spinner:.green} {msg} {bytes}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.set_message(msg.to_string());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

/// Print install step progress (e.g. "[1/4] Downloading...")
pub fn install_step(current: u32, total: u32, msg: &str) {
    println!("  {} [{}/{}] {}", "·".dimmed(), current, total, msg);
}

/// Print a section header
pub fn section(msg: &str) {
    println!();
    println!("{}", "─".repeat(72).dimmed());
    println!("{} {}", "■".cyan(), msg.bold());
    println!("{}", "─".repeat(72).dimmed());
}

/// Run a command with output suppressed, showing a spinner instead
/// Returns the command output for checking status
pub fn run_command_with_spinner(
    spinner: &ProgressBar,
    cmd: &mut Command,
) -> std::io::Result<Output> {
    let output = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).output();

    ensure_min_display(spinner.elapsed());
    output
}

/// Helper to extract the first line of stderr for error messages
pub fn first_error_line(stderr: &[u8]) -> String {
    let text = String::from_utf8_lossy(stderr);
    text.lines().map(|l| l.trim()).find(|l| !l.is_empty()).unwrap_or("Unknown error").to_string()
}

/// Extract a compact but useful stderr excerpt.
/// Keeps up to `max_lines` non-empty lines and appends a truncation marker.
pub fn stderr_excerpt(stderr: &[u8], max_lines: usize) -> String {
    let text = String::from_utf8_lossy(stderr);
    let lines: Vec<&str> = text.lines().map(|l| l.trim_end()).filter(|l| !l.is_empty()).collect();
    if lines.is_empty() {
        return "Unknown error".to_string();
    }

    let cap = max_lines.min(ERROR_EXCERPT_MAX_LINES).max(1);
    let mut excerpt = lines.iter().take(cap).cloned().collect::<Vec<_>>().join("\n");
    if lines.len() > cap {
        excerpt.push_str("\n... (truncated)");
    }
    excerpt
}

/// Render stderr excerpt as indented diagnostic lines.
pub fn print_stderr_excerpt(stderr: &[u8], max_lines: usize) {
    for line in stderr_excerpt(stderr, max_lines).lines() {
        eprintln!("    {}", line.dimmed());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_error_line() {
        let stderr = b"go: module not found\nsome other line";
        assert_eq!(first_error_line(stderr), "go: module not found");
    }

    #[test]
    fn test_first_error_line_empty() {
        let stderr = b"";
        assert_eq!(first_error_line(stderr), "Unknown error");
    }

    #[test]
    fn test_stderr_excerpt_truncates() {
        let stderr = b"line1\nline2\nline3\nline4";
        assert_eq!(stderr_excerpt(stderr, 2), "line1\nline2\n... (truncated)");
    }
}
