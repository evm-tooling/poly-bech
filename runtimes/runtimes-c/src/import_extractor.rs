//! C include extraction from setup blocks.

use poly_bench_dsl::Lang;
use poly_bench_ir_traits::{ImportExtractor, ParsedSetup};

pub fn extract_c_imports(setup: &str) -> ParsedSetup {
    let mut imports = Vec::new();
    let mut body = String::new();

    for line in setup.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            if !body.is_empty() || !imports.is_empty() {
                body.push('\n');
            }
            continue;
        }

        if trimmed.starts_with("#include ") {
            imports.push(trimmed.to_string());
            continue;
        }

        body.push_str(line);
        body.push('\n');
    }

    ParsedSetup::new(imports, body.trim().to_string())
}

pub struct CImportExtractor;

impl ImportExtractor for CImportExtractor {
    fn lang(&self) -> Lang {
        Lang::C
    }

    fn extract(&self, setup: &str) -> ParsedSetup {
        extract_c_imports(setup)
    }
}

pub static C_IMPORT_EXTRACTOR: CImportExtractor = CImportExtractor;
