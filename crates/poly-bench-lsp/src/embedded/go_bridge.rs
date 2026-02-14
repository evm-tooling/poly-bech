//! Go code checking for embedded blocks
//!
//! This module validates Go code by writing temporary files
//! and running `go build`, then parsing compiler errors.
//!
//! Key design: We compile all setup sections (import, declare, helpers, init)
//! together as a single unit to avoid false "unused" errors. Errors are then
//! mapped back to their original source locations.

use regex::Regex;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
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

/// Check a Go code block and return diagnostics
pub fn check_go_block(
    block: &EmbeddedBlock,
    context: &SetupContext,
    go_mod_root: Option<&str>,
) -> Vec<EmbeddedDiagnostic> {
    let mut diagnostics = Vec::new();
    let code = block.code.trim();

    if code.is_empty() {
        return diagnostics;
    }

    // Check if Go is available - check common locations if not in PATH
    let go_cmd = find_go_cmd();
    let go_cmd = match go_cmd {
        Some(cmd) => cmd,
        None => return diagnostics,
    };

    // Wrap code appropriately based on block type
    let (wrapped, header_lines) = wrap_go_code(code, block.block_type, context);

    // Run go build and get output
    let (error_output, is_in_mod_root) = match run_go_build(&go_cmd, &wrapped, go_mod_root) {
        Some(result) => result,
        None => return diagnostics,
    };

    // Check for "go.mod not found" type errors - skip silently when not in module
    if !is_in_mod_root
        && (error_output.contains("go.mod file not found")
            || error_output.contains("no required module provides package"))
    {
        return diagnostics;
    }

    // Parse error output
    let parsed = parse_go_errors(&error_output, block, header_lines);
    diagnostics.extend(parsed);

    diagnostics
}

/// Check all Go setup sections together and return diagnostics mapped to original blocks
pub fn check_go_setup_combined(
    blocks: &[&EmbeddedBlock],
    context: &SetupContext,
    go_mod_root: Option<&str>,
) -> Vec<EmbeddedDiagnostic> {
    let go_cmd = match find_go_cmd() {
        Some(cmd) => cmd,
        None => return Vec::new(),
    };

    // Build combined code with section mappings
    let (combined, mappings) = build_combined_setup(blocks, context);

    if combined.trim().is_empty() {
        return Vec::new();
    }

    // Run go build
    let (error_output, is_in_mod_root) = match run_go_build(&go_cmd, &combined, go_mod_root) {
        Some(result) => result,
        None => return Vec::new(),
    };

    // Skip module-not-found errors when not in module
    if !is_in_mod_root
        && (error_output.contains("go.mod file not found")
            || error_output.contains("no required module provides package"))
    {
        return Vec::new();
    }

    // Parse and map errors back to original blocks
    parse_combined_errors(&error_output, &mappings)
}

/// Find the Go compiler
fn find_go_cmd() -> Option<String> {
    if which::which("go").is_ok() {
        return Some("go".to_string());
    }

    let common_paths = [
        "/usr/local/go/bin/go",
        "/opt/homebrew/bin/go",
        "/usr/local/bin/go",
        &format!("{}/go/bin/go", std::env::var("HOME").unwrap_or_default()),
    ];

    common_paths
        .iter()
        .find(|p| Path::new(p).exists())
        .map(|s| s.to_string())
}

/// Run go build on the given code and return (error_output, is_in_mod_root)
fn run_go_build(go_cmd: &str, code: &str, go_mod_root: Option<&str>) -> Option<(String, bool)> {
    let use_mod_root = go_mod_root
        .map(|r| Path::new(r).join("go.mod").exists())
        .unwrap_or(false);

    let (_temp_dir_guard, work_dir, is_in_mod_root) = if use_mod_root {
        // Safe: use_mod_root is only true if go_mod_root is Some
        let Some(mod_root) = go_mod_root else {
            return None;
        };
        let temp_name = format!("_polybench_lint_{}", std::process::id());
        let temp_path = Path::new(mod_root).join(&temp_name);
        if fs::create_dir_all(&temp_path).is_err() {
            return None;
        }
        (None, temp_path, true)
    } else {
        match TempDir::new() {
            Ok(t) => {
                let path = t.path().to_path_buf();
                (Some(t), path, false)
            }
            Err(_) => return None,
        }
    };

    let check_file = work_dir.join("main.go");

    if let Err(_) = fs::File::create(&check_file).and_then(|mut f| f.write_all(code.as_bytes())) {
        if is_in_mod_root {
            cleanup_dir(&work_dir);
        }
        return None;
    }

    let output = Command::new(go_cmd)
        .args(["build", "-o", "/dev/null", "./main.go"])
        .current_dir(&work_dir)
        .output();

    if is_in_mod_root {
        cleanup_dir(&work_dir);
    }

    let output = output.ok()?;

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let error_output = if stderr.is_empty() {
        stdout.to_string()
    } else {
        stderr.to_string()
    };

    Some((error_output, is_in_mod_root))
}

/// Build combined setup code with mappings for error attribution
fn build_combined_setup(
    blocks: &[&EmbeddedBlock],
    context: &SetupContext,
) -> (String, Vec<SectionMapping>) {
    let mut combined = String::from("package main\n\n");
    let mut mappings = Vec::new();
    // "package main\n\n" = Line 1: "package main", Line 2: blank
    // Next content starts at line 3 (1-indexed)
    let mut current_line = 3;

    // Add imports from context
    if let Some(ref imports) = context.imports {
        let trimmed = imports.trim();
        if !trimmed.is_empty() {
            combined.push_str(trimmed);
            combined.push_str("\n\n");

            // Find the matching import block
            if let Some(block) = blocks
                .iter()
                .find(|b| b.block_type == BlockType::SetupImport)
            {
                let line_count = trimmed.lines().count();
                mappings.push(SectionMapping {
                    start_line: current_line,
                    line_count,
                    span_start: block.span.start,
                    code: block.code.clone(),
                });
                // After content + "\n\n": content takes `line_count` lines,
                // then "\n\n" adds 1 blank line. Next content starts at current + line_count + 1
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

            if let Some(block) = blocks
                .iter()
                .find(|b| b.block_type == BlockType::SetupDeclare)
            {
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

            if let Some(block) = blocks
                .iter()
                .find(|b| b.block_type == BlockType::SetupHelpers)
            {
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
            // Wrap init in a function if it's not already
            if !trimmed.contains("func ") {
                combined.push_str("func init() {\n");
                combined.push_str(trimmed);
                combined.push_str("\n}\n\n");

                let line_count = trimmed.lines().count();
                mappings.push(SectionMapping {
                    start_line: current_line + 1, // +1 for "func init() {"
                    line_count,
                    span_start: block.span.start,
                    code: block.code.clone(),
                });
                // func init() { (1 line) + content (line_count) + } (1 line) + blank (1 line)
                current_line += 1 + line_count + 1 + 1;
            } else {
                combined.push_str(trimmed);
                combined.push_str("\n\n");

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

    // Add a main function that uses everything to prevent "unused" errors
    // This is a trick: we add var _ = x for each top-level declaration
    combined.push_str("func main() {}\n");

    (combined, mappings)
}

/// Parse errors from combined build and map back to original blocks
fn parse_combined_errors(output: &str, mappings: &[SectionMapping]) -> Vec<EmbeddedDiagnostic> {
    let mut diagnostics = Vec::new();
    let error_re = Regex::new(r"(?m)^[^:\s]+\.go:(\d+):(\d+):\s*(.+)$").unwrap();

    eprintln!(
        "[go-combined-parse] Parsing errors, {} mappings:",
        mappings.len()
    );
    for (i, m) in mappings.iter().enumerate() {
        eprintln!(
            "[go-combined-parse]   [{}] start_line={}, line_count={}, span_start={}",
            i, m.start_line, m.line_count, m.span_start
        );
    }

    for cap in error_re.captures_iter(output) {
        // Use safe access for regex captures
        let Some(line_match) = cap.get(1) else {
            continue;
        };
        let Some(col_match) = cap.get(2) else {
            continue;
        };
        let Some(msg_match) = cap.get(3) else {
            continue;
        };

        let error_line: usize = line_match.as_str().parse().unwrap_or(1);
        let error_col: usize = col_match.as_str().parse().unwrap_or(1);
        let message = msg_match.as_str().to_string();

        eprintln!(
            "[go-combined-parse] Error at line {}, col {}: {}",
            error_line,
            error_col,
            &message[..message.len().min(60)]
        );

        // Find which section this error belongs to
        if let Some(mapping) = find_section_for_line(error_line, mappings) {
            // Calculate line within the section
            let line_in_section = error_line.saturating_sub(mapping.start_line);

            eprintln!(
                "[go-combined-parse]   -> Found in section at line {}, line_in_section={}",
                mapping.start_line, line_in_section
            );

            // Get the line content from original code
            let lines: Vec<&str> = mapping.code.lines().collect();
            if line_in_section < lines.len() {
                let line_content = lines[line_in_section];

                // Calculate offset in original code
                let mut offset = 0;
                for i in 0..line_in_section {
                    offset += lines[i].len() + 1;
                }

                let col = (error_col.saturating_sub(1)).min(line_content.len());
                let start_offset = mapping.span_start + offset + col;
                let end_offset = mapping.span_start + offset + line_content.len().max(col + 1);

                eprintln!(
                    "[go-combined-parse]   -> Mapped to offset {}..{}",
                    start_offset, end_offset
                );

                if let Some(sanitized) = sanitize_go_message(&message) {
                    diagnostics.push(EmbeddedDiagnostic {
                        start_offset,
                        end_offset,
                        message: sanitized,
                        severity: EmbeddedSeverity::Error,
                    });
                }
            } else {
                eprintln!(
                    "[go-combined-parse]   -> line_in_section {} >= lines.len() {}",
                    line_in_section,
                    lines.len()
                );
            }
        } else {
            eprintln!(
                "[go-combined-parse]   -> No mapping found for line {}",
                error_line
            );
        }
    }

    diagnostics
}

/// Find which section a line number belongs to
fn find_section_for_line(line: usize, mappings: &[SectionMapping]) -> Option<&SectionMapping> {
    mappings
        .iter()
        .find(|m| line >= m.start_line && line < m.start_line + m.line_count)
}

/// Build the context prefix (imports + declarations) for wrapping
fn build_context_prefix(context: &SetupContext, include_decls: bool) -> (String, usize) {
    let mut prefix = String::from("package main\n\n");
    let mut lines = 2; // "package main\n\n"

    if let Some(ref imports) = context.imports {
        prefix.push_str(imports.trim());
        prefix.push_str("\n\n");
        lines += imports.lines().count() + 2;
    }

    // Add stdlib code (constants, etc.) after imports
    if let Some(ref stdlib) = context.stdlib_code {
        prefix.push_str(stdlib.trim());
        prefix.push_str("\n\n");
        lines += stdlib.lines().count() + 2;
    }

    if include_decls {
        if let Some(ref decls) = context.declarations {
            prefix.push_str(decls.trim());
            prefix.push_str("\n\n");
            lines += decls.lines().count() + 2;
        }
        if let Some(ref helpers) = context.helpers {
            prefix.push_str(helpers.trim());
            prefix.push_str("\n\n");
            lines += helpers.lines().count() + 2;
        }
    }

    (prefix, lines)
}

/// Wrap Go code based on block type, including necessary context
fn wrap_go_code(code: &str, block_type: BlockType, context: &SetupContext) -> (String, usize) {
    let has_package = code.contains("package ");
    let has_func = code.contains("func ");

    match block_type {
        BlockType::SetupImport => {
            // Import statements - wrap in package with blank import to prevent unused errors
            let wrapped = format!("package main\n\n{}\n\nvar _ = func() {{}}", code);
            (wrapped, 2)
        }
        BlockType::SetupDeclare => {
            // Declarations need imports for type resolution
            let (prefix, prefix_lines) = build_context_prefix(context, false);
            let wrapped = format!("{}{}", prefix, code);
            (wrapped, prefix_lines)
        }
        BlockType::SetupHelpers => {
            // Helper functions need imports + declarations
            let (prefix, prefix_lines) = build_context_prefix(context, true);
            if has_package && has_func {
                (code.to_string(), 0)
            } else if has_func {
                let wrapped = format!("{}{}", prefix, code);
                (wrapped, prefix_lines)
            } else {
                let wrapped = format!("{}func __helpers() {{\n{}\n}}", prefix, code);
                (wrapped, prefix_lines + 1)
            }
        }
        BlockType::SetupInit | BlockType::Hook => {
            // Init code and hooks need full context
            let (prefix, prefix_lines) = build_context_prefix(context, true);
            if has_func {
                let wrapped = format!("{}{}", prefix, code);
                (wrapped, prefix_lines)
            } else {
                let wrapped = format!("{}func __init_or_hook() {{\n{}\n}}", prefix, code);
                (wrapped, prefix_lines + 1)
            }
        }
        BlockType::Fixture => {
            // Fixtures need full context
            let (prefix, prefix_lines) = build_context_prefix(context, true);
            let wrapped = format!("{}func __fixture() []byte {{\n{}\n}}", prefix, code);
            (wrapped, prefix_lines + 1)
        }
        BlockType::Benchmark | BlockType::Skip | BlockType::Validate => {
            // Benchmark code needs full context
            let (prefix, prefix_lines) = build_context_prefix(context, true);
            if has_package && has_func {
                (code.to_string(), 0)
            } else if has_func {
                let wrapped = format!("{}{}", prefix, code);
                (wrapped, prefix_lines)
            } else {
                let wrapped = format!("{}func main() {{\n{}\n}}", prefix, code);
                (wrapped, prefix_lines + 1)
            }
        }
    }
}

/// Parse Go compiler error output
fn parse_go_errors(
    output: &str,
    block: &EmbeddedBlock,
    header_lines: usize,
) -> Vec<EmbeddedDiagnostic> {
    let mut diagnostics = Vec::new();

    // Regex to match Go compiler errors: file.go:line:col: message
    // Use multiline mode (?m) so ^ matches line start, and capture message to end of line
    let error_re = Regex::new(r"(?m)^[^:\s]+\.go:(\d+):(\d+):\s*(.+)$").unwrap();
    let lines: Vec<&str> = block.code.split('\n').collect();

    eprintln!(
        "[go-parse] Looking for errors in {} bytes, header_lines={}",
        output.len(),
        header_lines
    );

    for cap in error_re.captures_iter(output) {
        // Use safe access for regex captures
        let Some(line_match) = cap.get(1) else {
            continue;
        };
        let Some(col_match) = cap.get(2) else {
            continue;
        };
        let Some(msg_match) = cap.get(3) else {
            continue;
        };

        let line_num: usize = line_match.as_str().parse().unwrap_or(1);
        let col_num: usize = col_match.as_str().parse().unwrap_or(1);
        let message = msg_match.as_str().to_string();

        eprintln!(
            "[go-parse] Found error: line={}, col={}, msg={}",
            line_num,
            col_num,
            &message[..message.len().min(80)]
        );

        // Adjust line number to account for header
        let adjusted_line = if line_num > header_lines {
            line_num - header_lines - 1
        } else {
            continue; // Error in header, skip
        };

        if adjusted_line >= lines.len() {
            continue;
        }

        // Calculate offset in original code
        let mut offset = 0;
        for i in 0..adjusted_line {
            offset += lines[i].len() + 1; // +1 for newline
        }

        let line_content = lines.get(adjusted_line).unwrap_or(&"");
        let col = (col_num.saturating_sub(1)).min(line_content.len());
        let start_offset = block.span.start + offset + col;
        let end_offset = block.span.start + offset + line_content.len().max(col + 1);

        // Sanitize message
        if let Some(sanitized) = sanitize_go_message(&message) {
            diagnostics.push(EmbeddedDiagnostic {
                start_offset,
                end_offset,
                message: sanitized,
                severity: EmbeddedSeverity::Error,
            });
        }
    }

    // If no line:col errors found but build failed, report on first line
    if diagnostics.is_empty() && !output.trim().is_empty() && output.contains("error") {
        let first_line = lines.first().unwrap_or(&"");
        let summary = output
            .lines()
            .find(|l| !l.trim().is_empty())
            .unwrap_or(output)
            .trim();

        if let Some(sanitized) = sanitize_go_message(summary) {
            // Use char_indices for safe string truncation (handles multi-byte chars)
            let truncated = if sanitized.len() > 120 {
                let truncate_at = sanitized
                    .char_indices()
                    .take(117)
                    .last()
                    .map(|(i, c)| i + c.len_utf8())
                    .unwrap_or(sanitized.len());
                format!("{}...", &sanitized[..truncate_at])
            } else {
                sanitized
            };

            diagnostics.push(EmbeddedDiagnostic {
                start_offset: block.span.start,
                end_offset: block.span.start + first_line.len().max(1),
                message: truncated,
                severity: EmbeddedSeverity::Error,
            });
        }
    }

    diagnostics
}

/// Remove noise from Go compiler messages
fn sanitize_go_message(msg: &str) -> Option<String> {
    let cleaned = msg
        .trim()
        .replace("# command-line-arguments", "")
        .replace("package command-line-arguments:", "")
        .trim()
        .to_string();

    if cleaned.is_empty() || cleaned == "#" {
        None
    } else {
        Some(cleaned)
    }
}

fn cleanup_dir(dir: &Path) {
    let _ = fs::remove_dir_all(dir);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_go_code_benchmark() {
        let code = "fmt.Println(\"hello\")";
        let ctx = SetupContext::default();
        let (wrapped, header) = wrap_go_code(code, BlockType::Benchmark, &ctx);
        assert!(wrapped.contains("package main"));
        assert!(wrapped.contains("func main()"));
        assert_eq!(header, 3);
    }

    #[test]
    fn test_wrap_go_code_fixture() {
        let code = "return []byte{1, 2, 3}";
        let ctx = SetupContext::default();
        let (wrapped, header) = wrap_go_code(code, BlockType::Fixture, &ctx);
        assert!(wrapped.contains("func __fixture() []byte"));
        assert_eq!(header, 3);
    }
}
