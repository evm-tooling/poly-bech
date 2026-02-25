//! Rust import extraction from setup blocks

use poly_bench_dsl::Lang;
use poly_bench_ir_traits::{ImportExtractor, ParsedSetup};

/// Extract imports from a Rust setup block.
///
/// Handles:
/// - `use std::collections::HashMap;`
/// - `use std::{io::Read, fs::File};`
/// - `use crate::module::*;`
/// - `use super::something;`
/// - Multi-line use statements with { ... }
/// - `extern crate` statements
pub fn extract_rust_imports(setup: &str) -> ParsedSetup {
    let mut imports = Vec::new();
    let mut body = String::new();
    let mut in_multiline_use = false;
    let mut current_use = String::new();
    let mut brace_depth = 0;

    for line in setup.lines() {
        let trimmed = line.trim();

        if in_multiline_use {
            current_use.push_str(line);
            current_use.push('\n');

            for ch in line.chars() {
                match ch {
                    '{' => brace_depth += 1,
                    '}' => brace_depth -= 1,
                    _ => {}
                }
            }

            if brace_depth == 0 && trimmed.ends_with(';') {
                imports.push(current_use.trim().to_string());
                current_use.clear();
                in_multiline_use = false;
            }
            continue;
        }

        if trimmed.is_empty() || trimmed.starts_with("//") {
            if !body.is_empty() || !imports.is_empty() {
                body.push_str(line);
                body.push('\n');
            }
            continue;
        }

        if trimmed.starts_with("use ") || trimmed.starts_with("use\t") {
            brace_depth = 0;
            for ch in trimmed.chars() {
                match ch {
                    '{' => brace_depth += 1,
                    '}' => brace_depth -= 1,
                    _ => {}
                }
            }

            if brace_depth == 0 && trimmed.ends_with(';') {
                imports.push(trimmed.to_string());
            } else {
                in_multiline_use = true;
                current_use = line.to_string();
                current_use.push('\n');
            }
            continue;
        }

        if trimmed.starts_with("extern crate ") {
            imports.push(trimmed.to_string());
            continue;
        }

        body.push_str(line);
        body.push('\n');
    }

    if !current_use.is_empty() {
        imports.push(current_use.trim().to_string());
    }

    ParsedSetup::new(imports, body)
}

pub struct RustImportExtractor;

impl ImportExtractor for RustImportExtractor {
    fn lang(&self) -> Lang {
        Lang::Rust
    }
    fn extract(&self, setup: &str) -> ParsedSetup {
        extract_rust_imports(setup)
    }
}

pub static RUST_IMPORT_EXTRACTOR: RustImportExtractor = RustImportExtractor;
