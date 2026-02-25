//! Go error line mapping

use poly_bench_dsl::Lang;
use poly_bench_ir::SuiteIR;
use poly_bench_runtime_traits::{ErrorMapper, LineMappings};
use regex::Regex;
use std::sync::LazyLock;

static GO_LINE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^[^:]+:(\d+):\d+").unwrap());

/// Go error mapper
pub struct GoErrorMapper;

/// Static instance for get_error_mapper dispatch
pub static GO_ERROR_MAPPER: GoErrorMapper = GoErrorMapper;

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

fn find_code_start(generated: &str, snippet: &str) -> Option<usize> {
    let first_non_empty = snippet.lines().find(|line| !line.trim().is_empty())?;
    let first_line = first_non_empty.trim();

    if first_line.is_empty() {
        return None;
    }

    for (i, line) in generated.lines().enumerate() {
        if line.trim() == first_line || line.contains(first_line) {
            return Some(i + 1);
        }
    }
    None
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
