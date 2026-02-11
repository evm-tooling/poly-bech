//! TypeScript code checking for embedded blocks
//!
//! This module validates TypeScript code by writing temporary files
//! and running `tsc`, then parsing compiler errors.
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
    /// Block type for this section (used for debugging)
    #[allow(dead_code)]
    block_type: BlockType,
    /// Original span start in .bench file
    span_start: usize,
    /// Original code for line lookups
    code: String,
}

/// Check a TypeScript code block and return diagnostics
pub fn check_ts_block(
    block: &EmbeddedBlock,
    context: &SetupContext,
    ts_module_root: Option<&str>,
) -> Vec<EmbeddedDiagnostic> {
    let code = block.code.trim();

    if code.is_empty() {
        return Vec::new();
    }

    // Find tsc command
    let (tsc_cmd, use_npx) = match find_tsc_cmd() {
        Some(cmd) => cmd,
        None => {
            eprintln!("[ts-check] tsc/npx not found");
            return Vec::new();
        }
    };

    // Wrap code appropriately based on block type
    let (wrapped, prefix_lines) = wrap_ts_code(code, block.block_type, context);
    
    eprintln!("[ts-check] Checking {:?} block, prefix_lines={}", block.block_type, prefix_lines);
    eprintln!("[ts-check] Wrapped code ({} lines):\n{}", wrapped.lines().count(), &wrapped[..wrapped.len().min(500)]);

    // Run tsc and get output
    let error_output = match run_tsc(&tsc_cmd, use_npx, &wrapped, ts_module_root) {
        Some(output) => output,
        None => return Vec::new(),
    };

    eprintln!("[ts-check] tsc output ({} bytes): {}", error_output.len(), &error_output[..error_output.len().min(500)]);

    // Parse error output
    let parsed = parse_ts_errors(&error_output, block, prefix_lines);
    eprintln!("[ts-check] parsed {} diagnostics", parsed.len());
    
    parsed
}

/// Check all TypeScript setup sections together and return diagnostics mapped to original blocks
pub fn check_ts_setup_combined(
    blocks: &[&EmbeddedBlock],
    context: &SetupContext,
    ts_module_root: Option<&str>,
) -> Vec<EmbeddedDiagnostic> {
    let (tsc_cmd, use_npx) = match find_tsc_cmd() {
        Some(cmd) => cmd,
        None => {
            eprintln!("[ts-check] tsc/npx not found for combined check");
            return Vec::new();
        }
    };

    // Build combined code with section mappings
    let (combined, mappings) = build_combined_setup(blocks, context);

    if combined.trim().is_empty() {
        return Vec::new();
    }

    eprintln!("[ts-check] Combined setup code ({} lines, {} sections):\n{}", 
        combined.lines().count(), mappings.len(), &combined[..combined.len().min(800)]);

    // Run tsc
    let error_output = match run_tsc(&tsc_cmd, use_npx, &combined, ts_module_root) {
        Some(output) => output,
        None => return Vec::new(),
    };

    eprintln!("[ts-check] Combined tsc output ({} bytes): {}", error_output.len(), &error_output[..error_output.len().min(500)]);

    // Parse and map errors back to original blocks
    let diagnostics = parse_combined_errors(&error_output, &mappings);
    eprintln!("[ts-check] Parsed {} combined diagnostics", diagnostics.len());
    
    diagnostics
}

/// Find tsc command - returns (command, use_npx)
fn find_tsc_cmd() -> Option<(String, bool)> {
    if which::which("tsc").is_ok() {
        return Some(("tsc".to_string(), false));
    }
    if which::which("npx").is_ok() {
        return Some(("npx".to_string(), true));
    }
    
    let common_npx_paths = [
        "/opt/homebrew/bin/npx",
        "/usr/local/bin/npx",
    ];
    
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
fn run_tsc(tsc_cmd: &str, use_npx: bool, code: &str, ts_module_root: Option<&str>) -> Option<String> {
    let temp_dir = TempDir::new().ok()?;
    let check_file = temp_dir.path().join("check.ts");

    fs::File::create(&check_file)
        .and_then(|mut f| f.write_all(code.as_bytes()))
        .ok()?;

    // First, try to use local tsc from node_modules if we have a module root
    let (actual_cmd, actual_use_npx) = if let Some(root) = ts_module_root {
        let local_tsc = Path::new(root).join("node_modules/.bin/tsc");
        if local_tsc.exists() {
            eprintln!("[ts-check] Using local tsc: {:?}", local_tsc);
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
                eprintln!("[ts-check] Using parent tsc: {:?}", tsc_path);
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
        "--target", "ES2020",
        "--module", "ESNext",
        "--moduleResolution", "bundler",
        "--strict", "false",
        "--noImplicitAny", "false",
    ]);

    // If we have a module root, use it for resolution
    if let Some(root) = ts_module_root {
        if Path::new(root).exists() {
            cmd.current_dir(root);
        }
    }

    cmd.arg(check_file.to_string_lossy().as_ref());

    eprintln!("[ts-check] Running: {:?}", cmd);

    let output = cmd.output().ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    eprintln!("[ts-check] exit={:?}, stdout={} bytes, stderr={} bytes", 
        output.status.code(), stdout.len(), stderr.len());
    
    // tsc outputs errors to stdout, but some errors go to stderr
    let error_output = if !stdout.is_empty() {
        stdout.to_string()
    } else {
        stderr.to_string()
    };

    Some(error_output)
}

/// Build combined setup code with mappings for error attribution
fn build_combined_setup(blocks: &[&EmbeddedBlock], context: &SetupContext) -> (String, Vec<SectionMapping>) {
    let mut combined = String::new();
    let mut mappings = Vec::new();
    let mut current_line = 1; // TypeScript has no package header, starts at line 1

    // Add imports from context
    if let Some(ref imports) = context.imports {
        let trimmed = imports.trim();
        if !trimmed.is_empty() {
            combined.push_str(trimmed);
            combined.push_str("\n\n");
            
            if let Some(block) = blocks.iter().find(|b| b.block_type == BlockType::SetupImport) {
                let line_count = trimmed.lines().count();
                mappings.push(SectionMapping {
                    start_line: current_line,
                    line_count,
                    block_type: BlockType::SetupImport,
                    span_start: block.span.start,
                    code: block.code.clone(),
                });
                // After content + "\n\n": 1 blank line added
                current_line += line_count + 1;
            }
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
                    block_type: BlockType::SetupDeclare,
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
                    block_type: BlockType::SetupHelpers,
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
            // Wrap init in an IIFE
            combined.push_str("(async () => {\n");
            combined.push_str(trimmed);
            combined.push_str("\n})();\n\n");
            
            let line_count = trimmed.lines().count();
            mappings.push(SectionMapping {
                start_line: current_line + 1, // +1 for "(async () => {"
                line_count,
                block_type: BlockType::SetupInit,
                span_start: block.span.start,
                code: block.code.clone(),
            });
            // (async () => { (1) + content (line_count) + })(); (1) + blank (1)
            current_line += 1 + line_count + 1 + 1;
        }
    }

    // Add a usage statement to prevent "unused" errors
    combined.push_str("export {};\n");

    (combined, mappings)
}

/// Parse errors from combined build and map back to original blocks
fn parse_combined_errors(output: &str, mappings: &[SectionMapping]) -> Vec<EmbeddedDiagnostic> {
    let mut diagnostics = Vec::new();
    
    // TSC error formats:
    // check.ts(line,col): error TS####: message
    // check.ts:line:col - error TS####: message
    // ../../path/to/check.ts(line,col): error TS####: message (relative paths)
    // The key is matching any path ending in check.ts or snippet.ts
    let error_re = Regex::new(r"[^\s]*(?:check|snippet)\.ts[:\(](\d+)[,:](\d+)\)?[:\s\-]+(?:error|warning)\s+TS\d+:\s*(.+)").unwrap();

    eprintln!("[ts-combined-parse] Parsing errors, {} mappings:", mappings.len());
    for (i, m) in mappings.iter().enumerate() {
        eprintln!("[ts-combined-parse]   [{}] {:?} start_line={}, line_count={}, span_start={}", 
            i, m.block_type, m.start_line, m.line_count, m.span_start);
    }

    for cap in error_re.captures_iter(output) {
        let error_line: usize = cap[1].parse().unwrap_or(1);
        let error_col: usize = cap[2].parse().unwrap_or(1);
        let message = cap[3].to_string();

        // Skip common false positives
        if should_skip_ts_error(&message) {
            eprintln!("[ts-combined-parse] Skipping error (filtered): {}", &message[..message.len().min(60)]);
            continue;
        }

        eprintln!("[ts-combined-parse] Error at line {}, col {}: {}", error_line, error_col, &message[..message.len().min(80)]);

        // Find which section this error belongs to
        if let Some(mapping) = find_section_for_line(error_line, mappings) {
            let line_in_section = error_line.saturating_sub(mapping.start_line);
            
            eprintln!("[ts-combined-parse]   -> Found in {:?}, line_in_section={}", mapping.block_type, line_in_section);
            
            let lines: Vec<&str> = mapping.code.lines().collect();
            if line_in_section < lines.len() {
                let line_content = lines[line_in_section];
                
                let mut offset = 0;
                for i in 0..line_in_section {
                    offset += lines[i].len() + 1;
                }
                
                let col = (error_col.saturating_sub(1)).min(line_content.len());
                let start_offset = mapping.span_start + offset + col;
                let end_offset = mapping.span_start + offset + line_content.len().max(col + 1);

                eprintln!("[ts-combined-parse]   -> Mapped to offset {}..{}", start_offset, end_offset);

                diagnostics.push(EmbeddedDiagnostic {
                    start_offset,
                    end_offset,
                    message,
                    severity: EmbeddedSeverity::Error,
                });
            } else {
                eprintln!("[ts-combined-parse]   -> line_in_section {} >= lines.len() {}", line_in_section, lines.len());
            }
        } else {
            eprintln!("[ts-combined-parse]   -> No mapping found for line {}", error_line);
        }
    }

    diagnostics
}

/// Find which section a line number belongs to
fn find_section_for_line(line: usize, mappings: &[SectionMapping]) -> Option<&SectionMapping> {
    mappings.iter().find(|m| {
        line >= m.start_line && line < m.start_line + m.line_count
    })
}

/// Build the context prefix (imports + declarations) for TypeScript
fn build_ts_context_prefix(context: &SetupContext, include_decls: bool) -> (String, usize) {
    let mut prefix = String::new();
    let mut lines = 0;

    if let Some(ref imports) = context.imports {
        prefix.push_str(imports.trim());
        prefix.push_str("\n\n");
        lines += imports.lines().count() + 2;
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

/// Wrap TypeScript code based on block type, including necessary context
fn wrap_ts_code(code: &str, block_type: BlockType, context: &SetupContext) -> (String, usize) {
    match block_type {
        BlockType::SetupImport => {
            // Import statements - use as-is
            (code.to_string(), 0)
        }
        BlockType::SetupDeclare => {
            // Declarations need imports for type resolution
            let (prefix, prefix_lines) = build_ts_context_prefix(context, false);
            let wrapped = format!("{}{}", prefix, code);
            (wrapped, prefix_lines)
        }
        BlockType::SetupHelpers => {
            // Helper functions need imports + declarations
            let (prefix, prefix_lines) = build_ts_context_prefix(context, true);
            let wrapped = format!("{}{}", prefix, code);
            (wrapped, prefix_lines)
        }
        BlockType::SetupInit | BlockType::Hook => {
            // Init code and hooks need full context
            let (prefix, prefix_lines) = build_ts_context_prefix(context, true);
            let has_await = code.contains("await ");
            if has_await {
                let wrapped = format!("{}(async () => {{\n{}\n}})();", prefix, code);
                (wrapped, prefix_lines + 1)
            } else {
                let wrapped = format!("{}(() => {{\n{}\n}})();", prefix, code);
                (wrapped, prefix_lines + 1)
            }
        }
        BlockType::Fixture => {
            // Fixtures need full context
            let (prefix, prefix_lines) = build_ts_context_prefix(context, true);
            let wrapped = format!("{}function __fixture(): unknown {{\n{}\n}}", prefix, code);
            (wrapped, prefix_lines + 1)
        }
        BlockType::Benchmark | BlockType::Skip | BlockType::Validate => {
            // Benchmark code needs full context
            let (prefix, prefix_lines) = build_ts_context_prefix(context, true);
            let wrapped = format!("{}const __result = (() => {{\n  return {};\n}})();", prefix, code);
            (wrapped, prefix_lines + 1)
        }
    }
}

/// Parse TypeScript compiler error output
fn parse_ts_errors(
    output: &str,
    block: &EmbeddedBlock,
    prefix_lines: usize,
) -> Vec<EmbeddedDiagnostic> {
    let mut diagnostics = Vec::new();

    // TSC error format: file.ts(line,col): error TS####: message
    // Also handles: file.ts:line:col - error TS####: message
    // Also handles relative paths like ../../check.ts(line,col): ...
    let error_re = Regex::new(r"[^\s]*(?:check|snippet)\.ts[:\(](\d+)[,:](\d+)\)?[:\s\-]+(?:error|warning)\s+TS\d+:\s*(.+)").unwrap();
    let lines: Vec<&str> = block.code.split('\n').collect();

    for cap in error_re.captures_iter(output) {
        let line_num: usize = cap[1].parse().unwrap_or(1);
        let col_num: usize = cap[2].parse().unwrap_or(1);
        let message = cap[3].to_string();

        // Skip common false positives
        if should_skip_ts_error(&message) {
            continue;
        }

        // Adjust line number to account for prefix
        let adjusted_line = if line_num > prefix_lines {
            line_num - prefix_lines - 1
        } else {
            continue; // Error in prefix, skip
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

        diagnostics.push(EmbeddedDiagnostic {
            start_offset,
            end_offset,
            message,
            severity: EmbeddedSeverity::Error,
        });
    }

    diagnostics
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

    // Note: We no longer skip "Cannot find module" errors - those are legitimate
    // and indicate the user has a typo in their import path

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_ts_code_fixture() {
        let code = "return { foo: 'bar' }";
        let ctx = SetupContext::default();
        let (wrapped, prefix) = wrap_ts_code(code, BlockType::Fixture, &ctx);
        assert!(wrapped.contains("function __fixture()"));
        assert_eq!(prefix, 1);
    }

    #[test]
    fn test_wrap_ts_code_benchmark() {
        let code = "someFunction()";
        let ctx = SetupContext::default();
        let (wrapped, prefix) = wrap_ts_code(code, BlockType::Benchmark, &ctx);
        assert!(wrapped.contains("const __result"));
        assert_eq!(prefix, 1);
    }

    #[test]
    fn test_should_skip_node_module() {
        assert!(should_skip_ts_error("Cannot find module 'node:crypto'"));
        assert!(!should_skip_ts_error("Property 'foo' does not exist"));
    }
}
