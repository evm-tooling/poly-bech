//! Terminal output utilities for poly-bench CLI
//!
//! Provides spinners, progress indicators, and clean output formatting
//! to mask underlying subprocess noise (go get, npm install, etc.)

use colored::Colorize;
use console::{measure_text_width, Term};
use indicatif::{HumanBytes, ProgressBar, ProgressStyle};
use std::{
    process::{Command, Output, Stdio},
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant},
};

/// Minimum display time for spinners (500ms) so users can follow progress
const MIN_DISPLAY_MS: u64 = 500;
/// Minimum interval between progress bar redraws (preserve last frame until new one ready)
const PROGRESS_REDRAW_MS: u64 = 80;
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

/// Print an indented info message
pub fn info_indented(msg: &str) {
    println!("  {} {}", "·".dimmed(), msg);
}

/// Custom download progress that builds each frame in memory and does a single atomic write.
/// Preserves last frame until new one is ready to eliminate flicker.
pub struct DownloadProgress {
    msg: String,
    total: Option<u64>,
    pos: AtomicU64,
    started: Instant,
    last_draw: std::sync::Mutex<Instant>,
    term: Term,
    spinner_idx: std::sync::Mutex<usize>,
}

const SPINNER: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
const BAR_WIDTH: usize = 24;

impl DownloadProgress {
    fn draw(&self, force: bool) {
        let pos = self.pos.load(Ordering::Relaxed);
        let now = Instant::now();
        if !force {
            let mut last = self.last_draw.lock().unwrap();
            if now.duration_since(*last).as_millis() < PROGRESS_REDRAW_MS as u128 {
                return;
            }
            *last = now;
        }

        let line = if let Some(total) = self.total {
            let pct = if total > 0 { (pos as f64 / total as f64 * 100.0).min(100.0) } else { 0.0 };
            let filled = (pct / 100.0 * BAR_WIDTH as f64) as usize;
            let bar: String = (0..BAR_WIDTH).map(|i| if i < filled { "█" } else { "░" }).collect();
            format!(
                "  {} [\x1b[32m{}\x1b[0m] {}/{} ({:.0}%)",
                self.msg,
                bar,
                HumanBytes(pos),
                HumanBytes(total),
                pct
            )
        } else {
            let mut idx = self.spinner_idx.lock().unwrap();
            let s = SPINNER[*idx % SPINNER.len()];
            *idx = idx.wrapping_add(1);
            format!("  \x1b[32m{}\x1b[0m {} {}", s, self.msg, HumanBytes(pos))
        };

        let (_, cols) = self.term.size();
        let width = cols as usize;
        let visible_len = measure_text_width(&line);
        let pad = width.saturating_sub(visible_len);
        let full = format!("\r{}{}", line, " ".repeat(pad));

        let _ = self.term.write_str(&full);
        let _ = self.term.flush();
    }

    pub fn inc(&self, delta: u64) {
        self.pos.fetch_add(delta, Ordering::Relaxed);
        self.draw(false);
    }

    pub fn elapsed(&self) -> Duration {
        self.started.elapsed()
    }

    pub fn finish_and_clear(&self) {
        self.draw(true);
        let (_, cols) = self.term.size();
        let _ = self.term.write_str(&format!("\r{}\r", " ".repeat(cols as usize)));
        let _ = self.term.flush();
    }
}

/// Create a progress bar for downloads with known size (shows bytes and percent).
/// Builds each frame in memory and does a single atomic write to prevent flicker.
pub fn download_progress_bar(total: u64, msg: &str) -> DownloadProgress {
    let dp = DownloadProgress {
        msg: msg.to_string(),
        total: Some(total),
        pos: AtomicU64::new(0),
        started: Instant::now(),
        last_draw: std::sync::Mutex::new(Instant::now()),
        term: Term::stderr(),
        spinner_idx: std::sync::Mutex::new(0),
    };
    dp.draw(true);
    dp
}

/// Create a spinner for downloads with unknown size. Call pb.inc(n) as bytes are read.
/// Builds each frame in memory and does a single atomic write to prevent flicker.
pub fn download_spinner(msg: &str) -> DownloadProgress {
    let dp = DownloadProgress {
        msg: msg.to_string(),
        total: None,
        pos: AtomicU64::new(0),
        started: Instant::now(),
        last_draw: std::sync::Mutex::new(Instant::now()),
        term: Term::stderr(),
        spinner_idx: std::sync::Mutex::new(0),
    };
    dp.draw(true);
    dp
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
