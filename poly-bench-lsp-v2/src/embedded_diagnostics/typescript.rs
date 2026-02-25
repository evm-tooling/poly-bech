//! TypeScript embedded code diagnostics via direct tsc execution
//!
//! This module provides diagnostics for embedded TypeScript code by running
//! the TypeScript compiler (tsc) directly and parsing its output.
//! This approach is more reliable than LSP protocol communication.

use std::{fs, io::Write, path::Path, process::Command};

use regex::Regex;
use tempfile::TempDir;
use tower_lsp::lsp_types::DiagnosticSeverity;

use crate::virtual_files::VirtualFile;

use super::EmbeddedDiagnostic;

/// Check TypeScript blocks by running tsc directly
pub fn check_ts_blocks(virtual_file: &dyn VirtualFile) -> Vec<EmbeddedDiagnostic> {
    let content = virtual_file.content();
    let path = virtual_file.path();

    if content.trim().is_empty() {
        return Vec::new();
    }

    // Find tsc command
    let (tsc_cmd, use_npx) = match find_tsc_cmd() {
        Some(cmd) => cmd,
        None => {
            tracing::debug!("[ts-diagnostics] tsc/npx not found");
            return Vec::new();
        }
    };

    // Get the module root from the virtual file path
    let ts_module_root = Path::new(path).parent().map(|p| p.to_string_lossy().to_string());

    // Run tsc and get output
    let error_output = match run_tsc(&tsc_cmd, use_npx, content, ts_module_root.as_deref()) {
        Some(output) => output,
        None => return Vec::new(),
    };

    tracing::debug!(
        "[ts-diagnostics] tsc output ({} bytes): {}",
        error_output.len(),
        &error_output[..error_output.len().min(500)]
    );

    // Parse error output and map to virtual file positions
    parse_ts_errors(&error_output, virtual_file)
}

/// Find tsc command - returns (command, use_npx)
fn find_tsc_cmd() -> Option<(String, bool)> {
    if which::which("tsc").is_ok() {
        return Some(("tsc".to_string(), false));
    }
    if which::which("npx").is_ok() {
        return Some(("npx".to_string(), true));
    }

    let common_npx_paths = ["/opt/homebrew/bin/npx", "/usr/local/bin/npx"];

    for path in &common_npx_paths {
        if Path::new(path).exists() {
            return Some((path.to_string(), true));
        }
    }

    // Check for nvm installed node
    if let Ok(home) = std::env::var("HOME") {
        let nvm_base = format!("{}/.nvm/versions/node", home);
        if let Ok(entries) = std::fs::read_dir(&nvm_base) {
            for entry in entries.flatten() {
                let npx_path = entry.path().join("bin/npx");
                if npx_path.exists() {
                    return Some((npx_path.to_string_lossy().to_string(), true));
                }
            }
        }
    }

    None
}

/// Run tsc on the given code and return error output
fn run_tsc(
    tsc_cmd: &str,
    use_npx: bool,
    code: &str,
    ts_module_root: Option<&str>,
) -> Option<String> {
    // If we have a module root, write the check file there so tsc can resolve node_modules
    // Otherwise fall back to a temp directory
    let (check_file, _temp_dir) = if let Some(root) = ts_module_root {
        let root_path = Path::new(root);
        if root_path.exists() {
            // Write directly to the module root so node_modules resolution works
            let check_path = root_path.join("__polybench_check__.ts");
            (check_path, None)
        } else {
            let temp = TempDir::new().ok()?;
            let path = temp.path().join("check.ts");
            (path, Some(temp))
        }
    } else {
        let temp = TempDir::new().ok()?;
        let path = temp.path().join("check.ts");
        (path, Some(temp))
    };

    fs::File::create(&check_file).and_then(|mut f| f.write_all(code.as_bytes())).ok()?;

    // First, try to use local tsc from node_modules if we have a module root
    let (actual_cmd, actual_use_npx) = if let Some(root) = ts_module_root {
        let local_tsc = Path::new(root).join("node_modules/.bin/tsc");
        if local_tsc.exists() {
            tracing::debug!("[ts-diagnostics] Using local tsc: {:?}", local_tsc);
            (local_tsc.to_string_lossy().to_string(), false)
        } else {
            // Check parent directories for node_modules
            let mut current = Path::new(root);
            let mut found_tsc = None;
            for _ in 0..5 {
                if let Some(parent) = current.parent() {
                    let parent_tsc = parent.join("node_modules/.bin/tsc");
                    if parent_tsc.exists() {
                        found_tsc = Some(parent_tsc);
                        break;
                    }
                    current = parent;
                } else {
                    break;
                }
            }
            if let Some(tsc_path) = found_tsc {
                tracing::debug!("[ts-diagnostics] Using parent tsc: {:?}", tsc_path);
                (tsc_path.to_string_lossy().to_string(), false)
            } else {
                (tsc_cmd.to_string(), use_npx)
            }
        }
    } else {
        (tsc_cmd.to_string(), use_npx)
    };

    let mut cmd = if actual_use_npx {
        let mut c = Command::new(&actual_cmd);
        c.arg("tsc");
        c
    } else {
        Command::new(&actual_cmd)
    };

    cmd.args([
        "--noEmit",
        "--skipLibCheck",
        "--target",
        "ES2020",
        "--module",
        "ESNext",
        "--moduleResolution",
        "bundler",
        "--strict",
        "false",
        "--noImplicitAny",
        "false",
    ]);

    // If we have a module root, use it for resolution
    if let Some(root) = ts_module_root {
        if Path::new(root).exists() {
            cmd.current_dir(root);
        }
    }

    cmd.arg(check_file.to_string_lossy().as_ref());

    tracing::debug!("[ts-diagnostics] Running: {:?}", cmd);

    let output = cmd.output().ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    tracing::debug!(
        "[ts-diagnostics] exit={:?}, stdout={} bytes, stderr={} bytes",
        output.status.code(),
        stdout.len(),
        stderr.len()
    );

    // tsc outputs errors to stdout, but some errors go to stderr
    let error_output = if !stdout.is_empty() { stdout.to_string() } else { stderr.to_string() };

    // Clean up the check file if we wrote it to the module root
    if _temp_dir.is_none() {
        let _ = fs::remove_file(&check_file);
    }

    Some(error_output)
}

/// Parse TypeScript compiler error output and map to virtual file positions
fn parse_ts_errors(output: &str, virtual_file: &dyn VirtualFile) -> Vec<EmbeddedDiagnostic> {
    let mut diagnostics = Vec::new();

    // TSC error formats:
    // check.ts(line,col): error TS####: message
    // check.ts:line:col - error TS####: message
    // ../../path/to/check.ts(line,col): error TS####: message (relative paths)
    let error_re = Regex::new(
        r"[^\s]*(?:check|snippet|__polybench_check__|polybench_virtual_[a-f0-9]+)\.ts[:\(](\d+)[,:](\d+)\)?[:\s\-]+(?:error|warning)\s+TS(\d+):\s*(.+)",
    )
    .unwrap();

    for cap in error_re.captures_iter(output) {
        let Some(line_match) = cap.get(1) else {
            continue;
        };
        let Some(col_match) = cap.get(2) else {
            continue;
        };
        let Some(code_match) = cap.get(3) else {
            continue;
        };
        let Some(msg_match) = cap.get(4) else {
            continue;
        };

        let line_num: u32 = line_match.as_str().parse().unwrap_or(1);
        let col_num: u32 = col_match.as_str().parse().unwrap_or(1);
        let error_code = code_match.as_str().to_string();
        let message = msg_match.as_str().to_string();

        // Skip common false positives
        if should_skip_ts_error(&message) {
            tracing::debug!(
                "[ts-diagnostics] Skipping error (filtered): {}",
                &message[..message.len().min(60)]
            );
            continue;
        }

        // tsc uses 1-indexed lines, convert to 0-indexed for LSP
        let virtual_line = line_num.saturating_sub(1);
        let virtual_character = col_num.saturating_sub(1);

        // Calculate length based on the error (default to reasonable length)
        let length = calculate_error_length(&message, virtual_file.content(), virtual_line);

        tracing::debug!(
            "[ts-diagnostics] Error at line {}, col {}: TS{}: {}",
            virtual_line,
            virtual_character,
            error_code,
            &message[..message.len().min(80)]
        );

        diagnostics.push(EmbeddedDiagnostic {
            message,
            severity: DiagnosticSeverity::ERROR,
            virtual_line,
            virtual_character,
            length,
            code: Some(format!("TS{}", error_code)),
        });
    }

    diagnostics
}

/// Calculate the length of an error based on the message and source
fn calculate_error_length(message: &str, content: &str, line: u32) -> u32 {
    // Try to extract the identifier from common error patterns
    let identifier_patterns = [
        r"Cannot find name '([^']+)'",
        r"Cannot find module '([^']+)'",
        r"Property '([^']+)' does not exist",
        r"'([^']+)' is not defined",
        r"Type '([^']+)' is not assignable",
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

/// Check if a TypeScript error should be skipped
fn should_skip_ts_error(message: &str) -> bool {
    // Skip "Cannot find module 'node:*'" errors - common in Node.js code
    if message.contains("Cannot find module 'node:") {
        return true;
    }

    // Skip "Cannot find name" for common globals that may not be typed
    if message.contains("Cannot find name 'console'") {
        return true;
    }

    // Skip errors about __polybench internal identifiers
    if message.contains("__polybench") || message.contains("__fixture") {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_skip_node_module() {
        assert!(should_skip_ts_error("Cannot find module 'node:crypto'"));
        assert!(!should_skip_ts_error("Property 'foo' does not exist"));
    }

    #[test]
    fn test_should_skip_internal() {
        assert!(should_skip_ts_error("Cannot find name '__polybench_init'"));
        assert!(should_skip_ts_error("Cannot find name '__fixture'"));
    }

    #[test]
    fn test_calculate_error_length() {
        assert_eq!(calculate_error_length("Cannot find name 'myVariable'", "", 0), 10);
        assert_eq!(calculate_error_length("Property 'foo' does not exist", "", 0), 3);
    }
}
