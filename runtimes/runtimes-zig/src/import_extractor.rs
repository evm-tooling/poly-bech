//! Zig import extraction from setup blocks.

use poly_bench_dsl::Lang;
use poly_bench_ir_traits::{ImportExtractor, ParsedSetup};

pub fn extract_zig_imports(setup: &str) -> ParsedSetup {
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

        if trimmed.contains("@import(") {
            imports.push(trimmed.to_string());
            continue;
        }

        body.push_str(line);
        body.push('\n');
    }

    ParsedSetup::new(imports, body.trim().to_string())
}

pub struct ZigImportExtractor;

impl ImportExtractor for ZigImportExtractor {
    fn lang(&self) -> Lang {
        Lang::Zig
    }

    fn extract(&self, setup: &str) -> ParsedSetup {
        extract_zig_imports(setup)
    }
}

pub static ZIG_IMPORT_EXTRACTOR: ZigImportExtractor = ZigImportExtractor;
