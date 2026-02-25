//! Error line mapping utilities
//!
//! Maps compiler error line numbers from generated code back to original .bench file locations.

use poly_bench_dsl::Lang;
use poly_bench_ir::SuiteIR;
use regex::Regex;
use std::sync::LazyLock;

/// Trait for language-specific error line remapping
pub trait ErrorMapper: Send + Sync {
    /// The language this mapper handles
    fn lang(&self) -> Lang;

    /// Build line mappings from suite and generated code
    fn build_mappings(&self, suite: &SuiteIR, generated_code: &str) -> LineMappings;

    /// Remap compiler error output to reference .bench file lines
    fn remap_error(&self, error: &str, mappings: &LineMappings) -> String;
}

struct GoErrorMapper;
impl ErrorMapper for GoErrorMapper {
    fn lang(&self) -> Lang {
        Lang::Go
    }
    fn build_mappings(&self, suite: &SuiteIR, generated_code: &str) -> LineMappings {
        build_go_mappings(suite, generated_code)
    }
    fn remap_error(&self, error: &str, mappings: &LineMappings) -> String {
        remap_go_error(error, mappings)
    }
}

struct TsErrorMapper;
impl ErrorMapper for TsErrorMapper {
    fn lang(&self) -> Lang {
        Lang::TypeScript
    }
    fn build_mappings(&self, suite: &SuiteIR, generated_code: &str) -> LineMappings {
        build_typescript_mappings(suite, generated_code)
    }
    fn remap_error(&self, error: &str, mappings: &LineMappings) -> String {
        remap_typescript_error(error, mappings)
    }
}

struct RustErrorMapper;
impl ErrorMapper for RustErrorMapper {
    fn lang(&self) -> Lang {
        Lang::Rust
    }
    fn build_mappings(&self, suite: &SuiteIR, generated_code: &str) -> LineMappings {
        build_rust_mappings(suite, generated_code)
    }
    fn remap_error(&self, error: &str, mappings: &LineMappings) -> String {
        remap_rust_error(error, mappings)
    }
}

/// Python error mapper - passthrough (no line remapping for generated Python)
struct PythonErrorMapper;
impl ErrorMapper for PythonErrorMapper {
    fn lang(&self) -> Lang {
        Lang::Python
    }
    fn build_mappings(&self, _suite: &SuiteIR, _generated_code: &str) -> LineMappings {
        LineMappings::default()
    }
    fn remap_error(&self, error: &str, _mappings: &LineMappings) -> String {
        error.to_string()
    }
}

/// Get the error mapper for a language
pub fn get_error_mapper(lang: Lang) -> Option<&'static dyn ErrorMapper> {
    match lang {
        Lang::Go => Some(&GoErrorMapper),
        Lang::TypeScript => Some(&TsErrorMapper),
        Lang::Rust => Some(&RustErrorMapper),
        Lang::Python => Some(&PythonErrorMapper),
        _ => None,
    }
}

/// A mapping entry from generated code line to .bench file line
#[derive(Debug, Clone)]
pub struct LineMapping {
    /// Start line in generated code (1-indexed)
    pub gen_start: usize,
    /// End line in generated code (1-indexed)
    pub gen_end: usize,
    /// Start line in .bench file (1-indexed)
    pub bench_line: usize,
    /// Description of what this section contains
    pub section: String,
}

/// Line mappings for a generated file
#[derive(Debug, Clone, Default)]
pub struct LineMappings {
    pub mappings: Vec<LineMapping>,
}

impl LineMappings {
    pub fn new() -> Self {
        Self { mappings: Vec::new() }
    }

    pub fn add(&mut self, gen_start: usize, gen_end: usize, bench_line: usize, section: &str) {
        self.mappings.push(LineMapping {
            gen_start,
            gen_end,
            bench_line,
            section: section.to_string(),
        });
    }

    /// Find the .bench file line for a generated code line
    pub fn find_bench_line(&self, gen_line: usize) -> Option<(usize, &str)> {
        for mapping in &self.mappings {
            if gen_line >= mapping.gen_start && gen_line <= mapping.gen_end {
                let offset = gen_line - mapping.gen_start;
                return Some((mapping.bench_line + offset, &mapping.section));
            }
        }
        None
    }
}

/// Build line mappings for Rust generated code
pub fn build_rust_mappings(suite: &SuiteIR, generated_code: &str) -> LineMappings {
    let mut mappings = LineMappings::new();

    // Find helpers section in generated code
    if let Some(helpers) = suite.helpers.get(&Lang::Rust) {
        if let Some(source) = suite.helpers_source.get(&Lang::Rust) {
            if let Some(gen_line) = find_code_start(generated_code, helpers) {
                let line_count = helpers.lines().count();
                // The span points to the opening brace line, but the code starts on the next line
                // Add 1 to account for this offset
                let bench_start = source.bench_file_line + 1;
                mappings.add(gen_line, gen_line + line_count - 1, bench_start, "helpers");
            }
        }
    }

    // Find declarations section
    if let Some(decls) = suite.declarations.get(&Lang::Rust) {
        if let Some(source) = suite.declarations_source.get(&Lang::Rust) {
            if let Some(gen_line) = find_code_start(generated_code, decls) {
                let line_count = decls.lines().count();
                let bench_start = source.bench_file_line + 1;
                mappings.add(gen_line, gen_line + line_count - 1, bench_start, "declarations");
            }
        }
    }

    mappings
}

/// Build line mappings for TypeScript generated code
pub fn build_typescript_mappings(suite: &SuiteIR, generated_code: &str) -> LineMappings {
    let mut mappings = LineMappings::new();

    if let Some(helpers) = suite.helpers.get(&Lang::TypeScript) {
        if let Some(source) = suite.helpers_source.get(&Lang::TypeScript) {
            if let Some(gen_line) = find_code_start(generated_code, helpers) {
                let line_count = helpers.lines().count();
                let bench_start = source.bench_file_line + 1;
                mappings.add(gen_line, gen_line + line_count - 1, bench_start, "helpers");
            }
        }
    }

    if let Some(decls) = suite.declarations.get(&Lang::TypeScript) {
        if let Some(source) = suite.declarations_source.get(&Lang::TypeScript) {
            if let Some(gen_line) = find_code_start(generated_code, decls) {
                let line_count = decls.lines().count();
                let bench_start = source.bench_file_line + 1;
                mappings.add(gen_line, gen_line + line_count - 1, bench_start, "declarations");
            }
        }
    }

    mappings
}

/// Build line mappings for Go generated code
pub fn build_go_mappings(suite: &SuiteIR, generated_code: &str) -> LineMappings {
    let mut mappings = LineMappings::new();

    if let Some(helpers) = suite.helpers.get(&Lang::Go) {
        if let Some(source) = suite.helpers_source.get(&Lang::Go) {
            if let Some(gen_line) = find_code_start(generated_code, helpers) {
                let line_count = helpers.lines().count();
                let bench_start = source.bench_file_line + 1;
                mappings.add(gen_line, gen_line + line_count - 1, bench_start, "helpers");
            }
        }
    }

    if let Some(decls) = suite.declarations.get(&Lang::Go) {
        if let Some(source) = suite.declarations_source.get(&Lang::Go) {
            if let Some(gen_line) = find_code_start(generated_code, decls) {
                let line_count = decls.lines().count();
                let bench_start = source.bench_file_line + 1;
                mappings.add(gen_line, gen_line + line_count - 1, bench_start, "declarations");
            }
        }
    }

    mappings
}

/// Find the line number where a code snippet starts in generated code
fn find_code_start(generated: &str, snippet: &str) -> Option<usize> {
    // Find the first non-empty line in the snippet
    let first_non_empty = snippet.lines().find(|line| !line.trim().is_empty())?;
    let first_line = first_non_empty.trim();

    if first_line.is_empty() {
        return None;
    }

    // Search for this line in the generated code
    for (i, line) in generated.lines().enumerate() {
        // Use exact match after trimming to avoid false positives
        if line.trim() == first_line || line.contains(first_line) {
            return Some(i + 1); // 1-indexed
        }
    }
    None
}

// Regex patterns for extracting line numbers from compiler errors
static RUST_LINE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^\s*-->\s*[^:]+:(\d+):\d+").unwrap());

static TS_LINE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^[^(]+\((\d+),\d+\)").unwrap());

static GO_LINE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^[^:]+:(\d+):\d+").unwrap());

/// Remap error message line numbers from generated code to .bench file
pub fn remap_rust_error(error: &str, mappings: &LineMappings) -> String {
    remap_error_lines(error, mappings, &RUST_LINE_REGEX, |_line, bench_line, section| {
        format!("  --> .bench file line {} (in {})", bench_line, section)
    })
}

/// Remap TypeScript error message line numbers
pub fn remap_typescript_error(error: &str, mappings: &LineMappings) -> String {
    remap_error_lines(error, mappings, &TS_LINE_REGEX, |_line, bench_line, section| {
        format!("  at .bench file line {} (in {})", bench_line, section)
    })
}

/// Remap Go error message line numbers  
pub fn remap_go_error(error: &str, mappings: &LineMappings) -> String {
    remap_error_lines(error, mappings, &GO_LINE_REGEX, |_line, bench_line, section| {
        format!("  at .bench file line {} (in {})", bench_line, section)
    })
}

fn remap_error_lines<F>(error: &str, mappings: &LineMappings, regex: &Regex, format_fn: F) -> String
where
    F: Fn(usize, usize, &str) -> String,
{
    let mut result = String::new();
    let mut added_mapping = false;

    for line in error.lines() {
        result.push_str(line);
        result.push('\n');

        // Check if this line contains a line number reference
        if let Some(caps) = regex.captures(line) {
            if let Some(line_match) = caps.get(1) {
                if let Ok(gen_line) = line_match.as_str().parse::<usize>() {
                    if let Some((bench_line, section)) = mappings.find_bench_line(gen_line) {
                        if !added_mapping {
                            result.push_str(&format_fn(gen_line, bench_line, section));
                            result.push('\n');
                            added_mapping = true;
                        }
                    }
                }
            }
        }
    }

    result
}
