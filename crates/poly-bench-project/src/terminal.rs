//! Terminal output utilities for poly-bench CLI
//!
//! Provides spinners, progress indicators, and clean output formatting
//! to mask underlying subprocess noise (go get, npm install, etc.)

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::process::{Command, Output, Stdio};
use std::time::Duration;

/// Minimum display time for spinners (500ms) so users can follow progress
const MIN_DISPLAY_MS: u64 = 500;

/// Create a spinner for a step with the [±] style prefix
pub fn step_spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap()
            .tick_strings(&["[±]", "[∓]", "[±]", "[∓]"]),
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
            .tick_strings(&["[±]", "[∓]", "[±]", "[∓]"]),
    );
    pb.set_message(msg.to_string());
    pb.enable_steady_tick(Duration::from_millis(120));
    pb
}

/// Ensure minimum display time for a spinner before finishing
pub fn ensure_min_display(pb: &ProgressBar) {
    let elapsed = pb.elapsed();
    if elapsed < Duration::from_millis(MIN_DISPLAY_MS) {
        std::thread::sleep(Duration::from_millis(MIN_DISPLAY_MS) - elapsed);
    }
}

/// Finish a spinner with a success message
pub fn finish_success(pb: &ProgressBar, msg: &str) {
    ensure_min_display(pb);
    pb.finish_and_clear();
    println!("{} {}", "[✓]".green().bold(), msg);
}

/// Finish a spinner with a success message (indented)
pub fn finish_success_indented(pb: &ProgressBar, msg: &str) {
    ensure_min_display(pb);
    pb.finish_and_clear();
    println!("  {} {}", "[✓]".green(), msg);
}

/// Finish a spinner with a failure message
pub fn finish_failure(pb: &ProgressBar, msg: &str) {
    ensure_min_display(pb);
    pb.finish_and_clear();
    eprintln!("{} {}", "[✗]".red().bold(), msg);
}

/// Finish a spinner with a failure message (indented)
pub fn finish_failure_indented(pb: &ProgressBar, msg: &str) {
    ensure_min_display(pb);
    pb.finish_and_clear();
    eprintln!("  {} {}", "[✗]".red(), msg);
}

/// Finish a spinner with a warning message (indented)
pub fn finish_warning_indented(pb: &ProgressBar, msg: &str) {
    ensure_min_display(pb);
    pb.finish_and_clear();
    eprintln!("  {} {}", "[⚠]".yellow(), msg);
}

/// Print a success message (no spinner)
pub fn success(msg: &str) {
    println!("{} {}", "[✓]".green().bold(), msg);
}

/// Print an indented success message (no spinner)
pub fn success_indented(msg: &str) {
    println!("  {} {}", "[✓]".green(), msg);
}

/// Print a failure message (no spinner)
pub fn failure(msg: &str) {
    eprintln!("{} {}", "[✗]".red().bold(), msg);
}

/// Print an indented info message
pub fn info_indented(msg: &str) {
    println!("  {} {}", "[·]".dimmed(), msg);
}

/// Print a section header
pub fn section(msg: &str) {
    println!("\n{}", msg.bold());
}

/// Run a command with output suppressed, showing a spinner instead
/// Returns the command output for checking status
pub fn run_command_with_spinner(
    spinner: &ProgressBar,
    cmd: &mut Command,
) -> std::io::Result<Output> {
    let output = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).output();

    ensure_min_display(spinner);
    output
}

/// Helper to extract the first line of stderr for error messages
pub fn first_error_line(stderr: &[u8]) -> String {
    let text = String::from_utf8_lossy(stderr);
    text.lines()
        .next()
        .unwrap_or("Unknown error")
        .trim()
        .to_string()
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
}
