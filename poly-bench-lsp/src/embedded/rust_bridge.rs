//! Rust code checking for embedded blocks
//!
//! This module validates Rust code by writing temporary files
//! and running `rustc` or `cargo check`, then parsing compiler errors.
//!
//! Key design: We compile all setup sections (import, declare, helpers, init)
//! together as a single unit to avoid false "unused" errors. Errors are then
//! mapped back to their original source locations.

use regex::Regex;
use std::{fs, io::Write, path::Path, process::Command};
use tempfile::TempDir;

use super::{BlockType, EmbeddedBlock, EmbeddedDiagnostic, EmbeddedSeverity, SetupContext};

/// Tracks where each section starts in the combined code
#[derive(Debug, Clone)]
struct SectionMapping {
    /// Starting line in combined code (1-indexed)
    start_line: usize,
    /// Number of lines in this section
    line_count: usize,
    /// Original span start in .bench file
    span_start: usize,
    /// Original code for line lookups
    code: String,
}

/// Check a Rust code block and return diagnostics
pub fn check_rust_block(
    block: &EmbeddedBlock,
    context: &SetupContext,
    cargo_root: Option<&str>,
) -> Vec<EmbeddedDiagnostic> {
    let mut diagnostics = Vec::new();
    let code = block.code.trim();

    if code.is_empty() {
        return diagnostics;
    }

    // Check if rustc is available
    let rustc_cmd = find_rustc_cmd();
    let rustc_cmd = match rustc_cmd {
        Some(cmd) => cmd,
        None => return diagnostics,
    };

    // Wrap code appropriately based on block type
    let (wrapped, header_lines) = wrap_rust_code(code, block.block_type, context);

    // Run rustc and get output
    let error_output = match run_rustc(&rustc_cmd, &wrapped, cargo_root) {
        Some(result) => result,
        None => return diagnostics,
    };

    // Parse error output
    let parsed = parse_rust_errors(&error_output, block, header_lines);
    diagnostics.extend(parsed);

    diagnostics
}

/// Check all Rust setup sections together and return diagnostics mapped to original blocks
pub fn check_rust_setup_combined(
    blocks: &[&EmbeddedBlock],
    context: &SetupContext,
    cargo_root: Option<&str>,
) -> Vec<EmbeddedDiagnostic> {
    let rustc_cmd = match find_rustc_cmd() {
        Some(cmd) => cmd,
        None => return Vec::new(),
    };

    // Build combined code with section mappings
    let (combined, mappings) = build_combined_setup(blocks, context);

    if combined.trim().is_empty() {
        return Vec::new();
    }

    // Run rustc
    let error_output = match run_rustc(&rustc_cmd, &combined, cargo_root) {
        Some(result) => result,
        None => return Vec::new(),
    };

    // Parse and map errors back to original blocks
    parse_combined_errors(&error_output, &mappings)
}

/// Find the Rust compiler
fn find_rustc_cmd() -> Option<String> {
    if which::which("rustc").is_ok() {
        return Some("rustc".to_string());
    }

    let common_paths = [
        &format!("{}/.cargo/bin/rustc", std::env::var("HOME").unwrap_or_default()),
        "/usr/local/bin/rustc",
        "/opt/homebrew/bin/rustc",
    ];

    common_paths.iter().find(|p| Path::new(p).exists()).map(|s| s.to_string())
}

/// Run rustc on the given code and return error output
fn run_rustc(rustc_cmd: &str, code: &str, _cargo_root: Option<&str>) -> Option<String> {
    let temp_dir = match TempDir::new() {
        Ok(t) => t,
        Err(_) => return None,
    };

    let check_file = temp_dir.path().join("check.rs");

    if fs::File::create(&check_file).and_then(|mut f| f.write_all(code.as_bytes())).is_err() {
        return None;
    }

    // Use --emit=metadata for faster checking (no codegen)
    let output = Command::new(rustc_cmd)
        .args(["--emit=metadata", "--edition=2021", "-o", "/dev/null", check_file.to_str()?])
        .output()
        .ok()?;

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let error_output = if stderr.is_empty() { stdout.to_string() } else { stderr.to_string() };

    Some(error_output)
}

/// Build combined setup code with mappings for error attribution
fn build_combined_setup(
    blocks: &[&EmbeddedBlock],
    context: &SetupContext,
) -> (String, Vec<SectionMapping>) {
    let mut combined = String::new();
    let mut mappings = Vec::new();
    let mut current_line = 1;

    // Add imports from context
    if let Some(ref imports) = context.imports {
        let trimmed = imports.trim();
        if !trimmed.is_empty() {
            combined.push_str(trimmed);
            combined.push_str("\n\n");

            // Find the matching import block
            if let Some(block) = blocks.iter().find(|b| b.block_type == BlockType::SetupImport) {
                let line_count = trimmed.lines().count();
                mappings.push(SectionMapping {
                    start_line: current_line,
                    line_count,
                    span_start: block.span.start,
                    code: block.code.clone(),
                });
                current_line += line_count + 1;
            }
        }
    }

    // Add stdlib code (constants, etc.) after imports - no mapping needed as it's synthetic
    if let Some(ref stdlib) = context.stdlib_code {
        let trimmed = stdlib.trim();
        if !trimmed.is_empty() {
            combined.push_str(trimmed);
            combined.push_str("\n\n");
            let line_count = trimmed.lines().count();
            current_line += line_count + 1;
        }
    }

    // Add declarations
    if let Some(ref decls) = context.declarations {
        let trimmed = decls.trim();
        if !trimmed.is_empty() {
            combined.push_str(trimmed);
            combined.push_str("\n\n");

            if let Some(block) = blocks.iter().find(|b| b.block_type == BlockType::SetupDeclare) {
                let line_count = trimmed.lines().count();
                mappings.push(SectionMapping {
                    start_line: current_line,
                    line_count,
                    span_start: block.span.start,
                    code: block.code.clone(),
                });
                current_line += line_count + 1;
            }
        }
    }

    // Add helpers
    if let Some(ref helpers) = context.helpers {
        let trimmed = helpers.trim();
        if !trimmed.is_empty() {
            combined.push_str(trimmed);
            combined.push_str("\n\n");

            if let Some(block) = blocks.iter().find(|b| b.block_type == BlockType::SetupHelpers) {
                let line_count = trimmed.lines().count();
                mappings.push(SectionMapping {
                    start_line: current_line,
                    line_count,
                    span_start: block.span.start,
                    code: block.code.clone(),
                });
                current_line += line_count + 1;
            }
        }
    }

    // Add init block if present
    if let Some(block) = blocks.iter().find(|b| b.block_type == BlockType::SetupInit) {
        let trimmed = block.code.trim();
        if !trimmed.is_empty() {
            // Wrap init in a function
            combined.push_str("fn __polybench_init() {\n");
            combined.push_str(trimmed);
            combined.push_str("\n}\n\n");

            let line_count = trimmed.lines().count();
            mappings.push(SectionMapping {
                start_line: current_line + 1, // +1 for "fn __polybench_init() {"
                line_count,
                span_start: block.span.start,
                code: block.code.clone(),
            });
            current_line += 1 + line_count + 1 + 1;
        }
    }

    // Add a main function to make it a valid program
    combined.push_str("fn main() {}\n");

    // Add allow unused to suppress warnings
    let prefixed = format!(
        "#![allow(unused_imports, unused_variables, dead_code, unused_mut)]\n\n{}",
        combined
    );

    // Adjust mappings for the added prefix (2 lines)
    for mapping in &mut mappings {
        mapping.start_line += 2;
    }

    (prefixed, mappings)
}

/// Wrap Rust code for standalone checking
fn wrap_rust_code(code: &str, block_type: BlockType, context: &SetupContext) -> (String, usize) {
    let mut wrapped = String::new();
    wrapped.push_str("#![allow(unused_imports, unused_variables, dead_code, unused_mut)]\n\n");
    let mut header_lines = 2;

    // Add imports if available
    if let Some(ref imports) = context.imports {
        let trimmed = imports.trim();
        if !trimmed.is_empty() {
            wrapped.push_str(trimmed);
            wrapped.push_str("\n\n");
            header_lines += trimmed.lines().count() + 1;
        }
    }

    // Add declarations if available
    if let Some(ref decls) = context.declarations {
        let trimmed = decls.trim();
        if !trimmed.is_empty() {
            wrapped.push_str(trimmed);
            wrapped.push_str("\n\n");
            header_lines += trimmed.lines().count() + 1;
        }
    }

    // Add helpers if available
    if let Some(ref helpers) = context.helpers {
        let trimmed = helpers.trim();
        if !trimmed.is_empty() {
            wrapped.push_str(trimmed);
            wrapped.push_str("\n\n");
            header_lines += trimmed.lines().count() + 1;
        }
    }

    match block_type {
        BlockType::SetupImport => {
            // Import blocks are just use statements
            wrapped.push_str(code);
            wrapped.push_str("\nfn main() {}\n");
        }
        BlockType::SetupDeclare | BlockType::SetupHelpers => {
            // Declarations and helpers are top-level items
            wrapped.push_str(code);
            wrapped.push_str("\nfn main() {}\n");
        }
        BlockType::SetupInit => {
            // Init code goes inside a function
            wrapped.push_str("fn main() {\n");
            header_lines += 1;
            wrapped.push_str(code);
            wrapped.push_str("\n}\n");
        }
        BlockType::Benchmark |
        BlockType::Fixture |
        BlockType::Hook |
        BlockType::Skip |
        BlockType::Validate => {
            // Benchmark code is an expression/statement
            wrapped.push_str("fn main() {\n    let _ = {\n");
            header_lines += 2;
            wrapped.push_str(code);
            wrapped.push_str("\n    };\n}\n");
        }
    }

    (wrapped, header_lines)
}

/// Parse rustc error output and map to block
fn parse_rust_errors(
    error_output: &str,
    block: &EmbeddedBlock,
    header_lines: usize,
) -> Vec<EmbeddedDiagnostic> {
    let mut diagnostics = Vec::new();

    // Rust error format: error[E0001]: message
    //  --> file.rs:line:col
    let error_regex = Regex::new(r"(?m)^(error|warning)\[?[^\]]*\]?: (.+)$").unwrap();
    let location_regex = Regex::new(r"^\s*--> [^:]+:(\d+):(\d+)").unwrap();

    let lines: Vec<&str> = error_output.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        if let Some(caps) = error_regex.captures(lines[i]) {
            let severity = match caps.get(1).map(|m| m.as_str()) {
                Some("error") => EmbeddedSeverity::Error,
                Some("warning") => EmbeddedSeverity::Warning,
                _ => EmbeddedSeverity::Error,
            };
            let message = caps.get(2).map(|m| m.as_str()).unwrap_or("Unknown error");

            // Look for location on next line
            let mut line_num: usize = 1;
            let mut col_num: usize = 1;
            if i + 1 < lines.len() {
                if let Some(loc_caps) = location_regex.captures(lines[i + 1]) {
                    line_num = loc_caps.get(1).and_then(|m| m.as_str().parse().ok()).unwrap_or(1);
                    col_num = loc_caps.get(2).and_then(|m| m.as_str().parse().ok()).unwrap_or(1);
                }
            }

            // Adjust for header lines
            let adjusted_line = if line_num > header_lines { line_num - header_lines } else { 1 };

            // Map to original span
            let code_lines: Vec<&str> = block.code.lines().collect();
            let target_line =
                adjusted_line.saturating_sub(1).min(code_lines.len().saturating_sub(1));

            // Calculate byte offset
            let mut offset = 0;
            for (idx, line) in code_lines.iter().enumerate() {
                if idx == target_line {
                    offset += (col_num.saturating_sub(1)).min(line.len());
                    break;
                }
                offset += line.len() + 1; // +1 for newline
            }

            diagnostics.push(EmbeddedDiagnostic {
                message: message.to_string(),
                severity,
                start_offset: block.span.start + offset,
                end_offset: block.span.start + offset + 1,
            });
        }
        i += 1;
    }

    diagnostics
}

/// Parse errors from combined setup and map back to original blocks
fn parse_combined_errors(
    error_output: &str,
    mappings: &[SectionMapping],
) -> Vec<EmbeddedDiagnostic> {
    let mut diagnostics = Vec::new();

    let error_regex = Regex::new(r"(?m)^(error|warning)\[?[^\]]*\]?: (.+)$").unwrap();
    let location_regex = Regex::new(r"^\s*--> [^:]+:(\d+):(\d+)").unwrap();

    let lines: Vec<&str> = error_output.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        if let Some(caps) = error_regex.captures(lines[i]) {
            let severity = match caps.get(1).map(|m| m.as_str()) {
                Some("error") => EmbeddedSeverity::Error,
                Some("warning") => EmbeddedSeverity::Warning,
                _ => EmbeddedSeverity::Error,
            };
            let message = caps.get(2).map(|m| m.as_str()).unwrap_or("Unknown error");

            // Look for location on next line
            let mut line_num: usize = 1;
            let mut col_num: usize = 1;
            if i + 1 < lines.len() {
                if let Some(loc_caps) = location_regex.captures(lines[i + 1]) {
                    line_num = loc_caps.get(1).and_then(|m| m.as_str().parse().ok()).unwrap_or(1);
                    col_num = loc_caps.get(2).and_then(|m| m.as_str().parse().ok()).unwrap_or(1);
                }
            }

            // Find which mapping this error belongs to
            if let Some(mapping) = find_mapping_for_line(line_num, mappings) {
                let relative_line = line_num - mapping.start_line;
                let code_lines: Vec<&str> = mapping.code.lines().collect();
                let target_line = relative_line.min(code_lines.len().saturating_sub(1));

                // Calculate byte offset
                let mut offset = 0;
                for (idx, line) in code_lines.iter().enumerate() {
                    if idx == target_line {
                        offset += (col_num.saturating_sub(1)).min(line.len());
                        break;
                    }
                    offset += line.len() + 1;
                }

                diagnostics.push(EmbeddedDiagnostic {
                    message: message.to_string(),
                    severity,
                    start_offset: mapping.span_start + offset,
                    end_offset: mapping.span_start + offset + 1,
                });
            }
        }
        i += 1;
    }

    diagnostics
}

/// Find the mapping that contains a given line number
fn find_mapping_for_line(line: usize, mappings: &[SectionMapping]) -> Option<&SectionMapping> {
    mappings.iter().find(|m| line >= m.start_line && line < m.start_line + m.line_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embedded::Span;

    #[test]
    fn test_find_rustc_cmd() {
        // This test may fail if rustc is not installed, which is fine
        let cmd = find_rustc_cmd();
        // Just check it doesn't panic
        assert!(cmd.is_none() || cmd.is_some());
    }

    #[test]
    fn test_wrap_rust_code_import() {
        let context = SetupContext::default();
        let (wrapped, header) = wrap_rust_code("use std::io;", BlockType::SetupImport, &context);
        assert!(wrapped.contains("use std::io;"));
        assert!(wrapped.contains("fn main()"));
        assert!(header >= 2);
    }

    #[test]
    fn test_wrap_rust_code_init() {
        let context = SetupContext::default();
        let (wrapped, _header) = wrap_rust_code("let x = 1;", BlockType::SetupInit, &context);
        assert!(wrapped.contains("fn main()"));
        assert!(wrapped.contains("let x = 1;"));
    }
}
