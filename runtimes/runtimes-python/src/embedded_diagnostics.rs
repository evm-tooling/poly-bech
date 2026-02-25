//! Python embedded code diagnostics via py_compile or pyright
//!
//! Uses `python -m py_compile` for basic syntax checking (always available with Python).
//! Falls back to pyright if available for more comprehensive diagnostics.

use std::process::Command;

use poly_bench_lsp_traits::{
    EmbeddedDiagnostic, EmbeddedDiagnosticContext, EmbeddedDiagnosticProvider,
    EmbeddedDiagnosticSetup, VirtualFile,
};
use poly_bench_syntax::Lang;
use regex::Regex;
use tower_lsp::lsp_types::DiagnosticSeverity;

/// Python embedded diagnostic provider
pub(crate) struct PythonEmbeddedDiagnosticProvider;

impl EmbeddedDiagnosticProvider for PythonEmbeddedDiagnosticProvider {
    fn check_blocks(
        &self,
        virtual_file: &dyn VirtualFile,
        _ctx: &dyn EmbeddedDiagnosticContext,
    ) -> Vec<EmbeddedDiagnostic> {
        check_python_blocks(virtual_file)
    }

    fn language(&self) -> Lang {
        Lang::Python
    }
}

pub(crate) static PYTHON_EMBEDDED_DIAGNOSTIC_PROVIDER: PythonEmbeddedDiagnosticProvider =
    PythonEmbeddedDiagnosticProvider;

/// Python embedded diagnostic setup - delegates to context
pub(crate) struct PythonEmbeddedDiagnosticSetup;

impl EmbeddedDiagnosticSetup for PythonEmbeddedDiagnosticSetup {
    fn lang(&self) -> Lang {
        Lang::Python
    }
    fn prepare(&self, module_root: &str, ctx: &dyn EmbeddedDiagnosticContext) {
        ctx.ensure_ready(Lang::Python, module_root);
    }
}

pub(crate) static PYTHON_EMBEDDED_DIAGNOSTIC_SETUP: PythonEmbeddedDiagnosticSetup =
    PythonEmbeddedDiagnosticSetup;

/// Check Python blocks by running py_compile or pyright
fn check_python_blocks(virtual_file: &dyn VirtualFile) -> Vec<EmbeddedDiagnostic> {
    let content = virtual_file.content();
    let path = virtual_file.path();

    if content.trim().is_empty() {
        return Vec::new();
    }

    // Try pyright first for better diagnostics, then py_compile
    if let Some(diags) = run_pyright(path, content) {
        return diags;
    }
    if let Some(diags) = run_py_compile(path, content) {
        return diags;
    }

    Vec::new()
}

/// Run pyright and parse output
fn run_pyright(file_path: &str, content: &str) -> Option<Vec<EmbeddedDiagnostic>> {
    which::which("pyright").ok()?;
    std::fs::write(file_path, content).ok()?;

    let output = Command::new("pyright").arg("--outputjson").arg(file_path).output().ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).ok()?;
    let summary = json.get("summary")?;
    let error_count = summary.get("errorCount")?.as_u64().unwrap_or(0);
    let warning_count = summary.get("warningCount")?.as_u64().unwrap_or(0);

    if error_count == 0 && warning_count == 0 {
        return Some(Vec::new());
    }

    let mut diags = Vec::new();
    let diagnostics = json.get("generalDiagnostics")?.as_array()?;
    for d in diagnostics {
        let range = d.get("range")?;
        let start = range.get("start")?;
        let line = start.get("line")?.as_u64().unwrap_or(0) as u32;
        let character = start.get("character")?.as_u64().unwrap_or(0) as u32;
        let end = range.get("end")?;
        let end_line = end.get("line")?.as_u64().unwrap_or(0) as u32;
        let end_char = end.get("character")?.as_u64().unwrap_or(0) as u32;
        let message = d.get("message")?.as_str().unwrap_or("").to_string();
        let rule = d.get("rule").and_then(|r| r.as_str()).map(String::from);
        let severity = match d.get("severity")?.as_str() {
            Some("error") => DiagnosticSeverity::ERROR,
            Some("warning") => DiagnosticSeverity::WARNING,
            _ => DiagnosticSeverity::INFORMATION,
        };
        let length = if line == end_line { end_char.saturating_sub(character) } else { 10 };
        diags.push(EmbeddedDiagnostic {
            message,
            severity,
            virtual_line: line,
            virtual_character: character,
            length: length.max(1),
            code: rule,
        });
    }
    Some(diags)
}

/// Run py_compile and parse stderr for syntax errors
fn run_py_compile(file_path: &str, content: &str) -> Option<Vec<EmbeddedDiagnostic>> {
    let python = which::which("python3").or_else(|_| which::which("python")).ok()?;
    std::fs::write(file_path, content).ok()?;

    let output = Command::new(&python).arg("-m").arg("py_compile").arg(file_path).output().ok()?;

    if output.status.success() {
        return Some(Vec::new());
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    parse_py_compile_errors(&stderr)
}

/// Parse py_compile stderr: "  File \"...\", line N" and "ErrorType: message"
fn parse_py_compile_errors(stderr: &str) -> Option<Vec<EmbeddedDiagnostic>> {
    // Pattern: "  File \"path\", line N" or "  File \"path\", line N, in ..."
    let line_re = Regex::new(r#"^\s*File "[^"]+", line (\d+)"#).ok()?;
    let mut diags = Vec::new();
    let mut current_line = 0u32;

    for line in stderr.lines() {
        if let Some(cap) = line_re.captures(line) {
            current_line = cap.get(1)?.as_str().parse().unwrap_or(1);
            // 1-indexed in Python, we use 0-indexed
            current_line = current_line.saturating_sub(1);
        } else if !line.trim().is_empty() && !line.starts_with("  File") {
            let line_str = line.trim();
            if !line_str.starts_with("File ") {
                let length = line_str.len().min(100) as u32;
                diags.push(EmbeddedDiagnostic {
                    message: line_str.to_string(),
                    severity: DiagnosticSeverity::ERROR,
                    virtual_line: current_line,
                    virtual_character: 0,
                    length: length.max(1),
                    code: None,
                });
            }
        }
    }
    Some(diags)
}
