//! Error line mapping trait and types
//!
//! Maps compiler error line numbers from generated code back to original .bench file locations.

use poly_bench_dsl::Lang;
use poly_bench_ir::SuiteIR;

/// Trait for language-specific error line remapping
pub trait ErrorMapper: Send + Sync {
    /// The language this mapper handles
    fn lang(&self) -> Lang;

    /// Build line mappings from suite and generated code
    fn build_mappings(&self, suite: &SuiteIR, generated_code: &str) -> LineMappings;

    /// Remap compiler error output to reference .bench file lines
    fn remap_error(&self, error: &str, mappings: &LineMappings) -> String;
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
