//! TypeScript import extraction from setup blocks

use poly_bench_dsl::Lang;
use poly_bench_ir_traits::{ImportExtractor, ParsedSetup};

/// Extract imports from a TypeScript/JavaScript setup block.
///
/// Handles:
/// - `import { a, b } from 'pkg'`
/// - `import * as x from 'pkg'`
/// - `import x from 'pkg'`
/// - `import 'pkg'` (side-effect import)
/// - Multi-line imports with { ... }
pub fn extract_ts_imports(setup: &str) -> ParsedSetup {
    let mut imports = Vec::new();
    let mut body = String::new();
    let mut in_multiline_import = false;
    let mut current_import = String::new();

    for line in setup.lines() {
        let trimmed = line.trim();

        if in_multiline_import {
            current_import.push_str(line);
            current_import.push('\n');

            if is_import_complete(&current_import) {
                imports.push(current_import.trim().to_string());
                current_import.clear();
                in_multiline_import = false;
            }
            continue;
        }

        if trimmed.starts_with("import ") || trimmed.starts_with("import{") {
            if is_import_complete(trimmed) {
                imports.push(trimmed.to_string());
            } else {
                in_multiline_import = true;
                current_import = line.to_string();
                current_import.push('\n');
            }
            continue;
        }

        body.push_str(line);
        body.push('\n');
    }

    if !current_import.is_empty() {
        imports.push(current_import.trim().to_string());
    }

    ParsedSetup::new(imports, body)
}

fn is_import_complete(import_text: &str) -> bool {
    let open_braces = import_text.matches('{').count();
    let close_braces = import_text.matches('}').count();

    if open_braces != close_braces {
        return false;
    }

    let has_from_clause = import_text.contains(" from ") &&
        (count_quotes(import_text, '\'') >= 2 || count_quotes(import_text, '"') >= 2);

    let is_side_effect = !import_text.contains(" from ") &&
        (count_quotes(import_text, '\'') >= 2 || count_quotes(import_text, '"') >= 2);

    has_from_clause || is_side_effect
}

fn count_quotes(s: &str, quote: char) -> usize {
    s.chars().filter(|&c| c == quote).count()
}

pub struct TsImportExtractor;

impl ImportExtractor for TsImportExtractor {
    fn lang(&self) -> Lang {
        Lang::TypeScript
    }
    fn extract(&self, setup: &str) -> ParsedSetup {
        extract_ts_imports(setup)
    }
}

pub static TS_IMPORT_EXTRACTOR: TsImportExtractor = TsImportExtractor;
