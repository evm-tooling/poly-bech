//! Rust embedded code diagnostics via cargo check
//!
//! This module provides diagnostics for embedded Rust code by running
//! `cargo check` and parsing its JSON output. This allows proper dependency
//! resolution from Cargo.toml, unlike running rustc directly.

use std::{path::Path, process::Command};

use poly_bench_lsp_traits::{
    EmbeddedDiagnostic, EmbeddedDiagnosticContext, EmbeddedDiagnosticProvider,
    EmbeddedDiagnosticSetup, VirtualFile,
};
use poly_bench_syntax::Lang;
use regex::Regex;
use serde::Deserialize;
use tower_lsp::lsp_types::DiagnosticSeverity;

/// Cargo JSON message for compiler diagnostics
#[derive(Debug, Deserialize)]
struct CargoMessage {
    reason: String,
    #[serde(default)]
    message: Option<CompilerMessage>,
}

/// Compiler message from cargo check
#[derive(Debug, Deserialize)]
struct CompilerMessage {
    #[serde(default)]
    code: Option<DiagnosticCode>,
    message: String,
    level: String,
    #[serde(default)]
    spans: Vec<DiagnosticSpan>,
}

/// Diagnostic code (e.g., E0432)
#[derive(Debug, Deserialize)]
struct DiagnosticCode {
    code: String,
}

/// Span information for a diagnostic
#[derive(Debug, Deserialize)]
struct DiagnosticSpan {
    file_name: String,
    line_start: u32,
    column_start: u32,
    line_end: u32,
    column_end: u32,
    is_primary: bool,
}

/// Rust embedded diagnostic provider
pub(crate) struct RustEmbeddedDiagnosticProvider;

impl EmbeddedDiagnosticProvider for RustEmbeddedDiagnosticProvider {
    fn check_blocks(
        &self,
        virtual_file: &dyn VirtualFile,
        _ctx: &dyn EmbeddedDiagnosticContext,
    ) -> Vec<EmbeddedDiagnostic> {
        check_rust_blocks(virtual_file)
    }

    fn language(&self) -> Lang {
        Lang::Rust
    }
}

pub(crate) static RUST_EMBEDDED_DIAGNOSTIC_PROVIDER: RustEmbeddedDiagnosticProvider =
    RustEmbeddedDiagnosticProvider;

/// Rust embedded diagnostic setup - delegates to context
pub(crate) struct RustEmbeddedDiagnosticSetup;

impl EmbeddedDiagnosticSetup for RustEmbeddedDiagnosticSetup {
    fn lang(&self) -> Lang {
        Lang::Rust
    }
    fn prepare(&self, module_root: &str, ctx: &dyn EmbeddedDiagnosticContext) {
        ctx.ensure_ready(Lang::Rust, module_root);
    }
    fn prepare_environment(&self, module_root: &str) {
        let src_dir = Path::new(module_root).join("src");
        let _ = std::fs::create_dir_all(&src_dir);
    }
}

pub(crate) static RUST_EMBEDDED_DIAGNOSTIC_SETUP: RustEmbeddedDiagnosticSetup =
    RustEmbeddedDiagnosticSetup;

/// Check Rust blocks by running cargo check
fn check_rust_blocks(virtual_file: &dyn VirtualFile) -> Vec<EmbeddedDiagnostic> {
    let content = virtual_file.content();
    let virtual_path = virtual_file.path();

    if content.trim().is_empty() {
        return Vec::new();
    }

    // Find cargo command
    let cargo_cmd = match find_cargo_cmd() {
        Some(cmd) => cmd,
        None => {
            tracing::debug!("[rust-diagnostics] cargo not found");
            return Vec::new();
        }
    };

    // Derive project root from virtual file path (virtual file is at
    // {root}/src/bin/_lsp_virtual_xxx.rs)
    let project_root = Path::new(virtual_path)
        .parent() // src/bin
        .and_then(|p| p.parent()) // src
        .and_then(|p| p.parent()); // project root

    let project_root = match project_root {
        Some(root) if root.join("Cargo.toml").exists() => root,
        _ => {
            tracing::debug!(
                "[rust-diagnostics] Could not find Cargo.toml for virtual file: {}",
                virtual_path
            );
            return Vec::new();
        }
    };

    tracing::debug!(
        "[rust-diagnostics] Running cargo check in project root: {}",
        project_root.display()
    );

    // Run cargo check and get JSON output
    let json_output = match run_cargo_check(&cargo_cmd, project_root) {
        Some(output) => output,
        None => return Vec::new(),
    };

    tracing::debug!("[rust-diagnostics] cargo check output ({} bytes)", json_output.len());

    // Parse JSON output and filter to virtual file only
    parse_cargo_json(&json_output, virtual_file)
}

/// Find the cargo command
fn find_cargo_cmd() -> Option<String> {
    if which::which("cargo").is_ok() {
        return Some("cargo".to_string());
    }

    // Check common paths
    let home = std::env::var("HOME").unwrap_or_default();
    let common_paths = [
        format!("{}/.cargo/bin/cargo", home),
        "/usr/local/bin/cargo".to_string(),
        "/opt/homebrew/bin/cargo".to_string(),
    ];

    for path in &common_paths {
        if Path::new(path).exists() {
            return Some(path.clone());
        }
    }

    // Check rustup toolchain
    if let Ok(output) = Command::new("rustup").args(["which", "cargo"]).output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() && Path::new(&path).exists() {
                return Some(path);
            }
        }
    }

    None
}

/// Run cargo check and return JSON output
fn run_cargo_check(cargo_cmd: &str, project_root: &Path) -> Option<String> {
    tracing::debug!(
        "[rust-diagnostics] Running: {} check --message-format=json in {:?}",
        cargo_cmd,
        project_root
    );

    let output = Command::new(cargo_cmd)
        .args(["check", "--message-format=json"])
        .current_dir(project_root)
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    tracing::debug!(
        "[rust-diagnostics] exit={:?}, stdout={} bytes, stderr={} bytes",
        output.status.code(),
        stdout.len(),
        stderr.len()
    );

    // cargo check outputs JSON to stdout, human-readable errors to stderr
    // We want the JSON output
    Some(stdout.to_string())
}

/// Parse cargo check JSON output and filter to virtual file diagnostics
fn parse_cargo_json(json_output: &str, virtual_file: &dyn VirtualFile) -> Vec<EmbeddedDiagnostic> {
    let mut diagnostics = Vec::new();
    let virtual_path = virtual_file.path();
    let content = virtual_file.content();

    // Extract just the filename from the virtual path for matching
    let virtual_filename =
        Path::new(virtual_path).file_name().and_then(|n| n.to_str()).unwrap_or("");

    for line in json_output.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let msg: CargoMessage = match serde_json::from_str(line) {
            Ok(m) => m,
            Err(_) => continue,
        };

        // Only process compiler-message entries
        if msg.reason != "compiler-message" {
            continue;
        }

        let compiler_msg = match msg.message {
            Some(m) => m,
            None => continue,
        };

        // Skip certain errors that are artifacts of our wrapping
        if should_skip_rust_error(&compiler_msg.message) {
            tracing::debug!(
                "[rust-diagnostics] Skipping error (filtered): {}",
                &compiler_msg.message[..compiler_msg.message.len().min(60)]
            );
            continue;
        }

        // Find the primary span that belongs to our virtual file
        let primary_span = compiler_msg
            .spans
            .iter()
            .find(|span| span.is_primary && span.file_name.contains(virtual_filename));

        let span = match primary_span {
            Some(s) => s,
            None => {
                // Try any span in our virtual file
                match compiler_msg.spans.iter().find(|s| s.file_name.contains(virtual_filename)) {
                    Some(s) => s,
                    None => continue, // Skip diagnostics not in our virtual file
                }
            }
        };

        let severity = match compiler_msg.level.as_str() {
            "warning" => DiagnosticSeverity::WARNING,
            "note" | "help" => DiagnosticSeverity::HINT,
            _ => DiagnosticSeverity::ERROR,
        };

        // Cargo uses 1-indexed lines, convert to 0-indexed for LSP
        let virtual_line = span.line_start.saturating_sub(1);
        let virtual_character = span.column_start.saturating_sub(1);

        // Calculate length from span or message
        let length = if span.line_start == span.line_end {
            span.column_end.saturating_sub(span.column_start).max(1)
        } else {
            calculate_error_length(&compiler_msg.message, content, virtual_line)
        };

        let error_code = compiler_msg.code.map(|c| c.code);

        tracing::debug!(
            "[rust-diagnostics] Error at line {}, col {}: {:?}: {}",
            virtual_line,
            virtual_character,
            error_code,
            &compiler_msg.message[..compiler_msg.message.len().min(80)]
        );

        diagnostics.push(EmbeddedDiagnostic {
            message: compiler_msg.message,
            severity,
            virtual_line,
            virtual_character,
            length,
            code: error_code,
        });
    }

    diagnostics
}

/// Calculate the length of an error based on the message and source
fn calculate_error_length(message: &str, content: &str, line: u32) -> u32 {
    // Try to extract the identifier from common error patterns
    let identifier_patterns = [
        r"cannot find value `([^`]+)`",
        r"cannot find type `([^`]+)`",
        r"cannot find macro `([^`]+)`",
        r"cannot find crate `([^`]+)`",
        r"cannot find trait `([^`]+)`",
        r"cannot find function `([^`]+)`",
        r"not found in this scope",
        r"unresolved import `([^`]+)`",
    ];

    for pattern in &identifier_patterns {
        if let Ok(re) = Regex::new(pattern) {
            if let Some(cap) = re.captures(message) {
                if let Some(m) = cap.get(1) {
                    return m.as_str().len() as u32;
                }
            }
        }
    }

    // Fall back to finding the token at the error position
    if let Some(line_content) = content.lines().nth(line as usize) {
        // Use a reasonable default based on line length
        return (line_content.trim().len() as u32).min(20).max(1);
    }

    10
}

/// Check if a Rust error should be skipped
fn should_skip_rust_error(message: &str) -> bool {
    // Skip errors about our internal identifiers
    if message.contains("__polybench") {
        return true;
    }

    // Skip "aborting due to previous error" messages
    if message.contains("aborting due to") {
        return true;
    }

    // Skip "For more information about this error" messages
    if message.contains("For more information about this error") {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_skip_internal() {
        assert!(should_skip_rust_error("cannot find value `__polybench_init`"));
        assert!(should_skip_rust_error("aborting due to previous error"));
        assert!(!should_skip_rust_error("cannot find value `foo`"));
    }

    #[test]
    fn test_calculate_error_length() {
        assert_eq!(
            calculate_error_length("cannot find value `myVariable` in this scope", "", 0),
            10
        );
        assert_eq!(calculate_error_length("cannot find type `Foo` in this scope", "", 0), 3);
    }

    #[test]
    fn test_find_cargo_cmd() {
        // This test may fail if cargo is not installed, which is fine
        let cmd = find_cargo_cmd();
        // Just check it doesn't panic
        assert!(cmd.is_none() || cmd.is_some());
    }

    #[test]
    fn test_parse_cargo_json_message() {
        // Test that we can deserialize cargo JSON messages
        let json = r#"{"reason":"compiler-message","message":{"code":{"code":"E0432"},"message":"unresolved import `foo`","level":"error","spans":[{"file_name":"src/bin/_lsp_virtual_test.rs","line_start":5,"column_start":5,"line_end":5,"column_end":8,"is_primary":true}]}}"#;
        let msg: CargoMessage = serde_json::from_str(json).unwrap();
        assert_eq!(msg.reason, "compiler-message");
        let compiler_msg = msg.message.unwrap();
        assert_eq!(compiler_msg.message, "unresolved import `foo`");
        assert_eq!(compiler_msg.code.unwrap().code, "E0432");
    }
}
