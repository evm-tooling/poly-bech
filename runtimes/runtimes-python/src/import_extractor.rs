//! Python import extraction from setup blocks

use poly_bench_dsl::Lang;
use poly_bench_ir_traits::{ImportExtractor, ParsedSetup};

/// Extract imports from a Python setup block.
///
/// Handles:
/// - `import foo`
/// - `import foo as bar`
/// - `from foo import bar`
/// - `from foo import bar, baz`
pub fn extract_python_imports(setup: &str) -> ParsedSetup {
    let mut imports = Vec::new();
    let mut body = String::new();

    for line in setup.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            if !body.is_empty() || !imports.is_empty() {
                body.push_str(line);
                body.push('\n');
            }
            continue;
        }

        if trimmed.starts_with("import ") || trimmed.starts_with("from ") {
            imports.push(trimmed.to_string());
            continue;
        }

        body.push_str(line);
        body.push('\n');
    }

    ParsedSetup::new(imports, body.trim().to_string())
}

pub struct PythonImportExtractor;

impl ImportExtractor for PythonImportExtractor {
    fn lang(&self) -> Lang {
        Lang::Python
    }
    fn extract(&self, setup: &str) -> ParsedSetup {
        extract_python_imports(setup)
    }
}

pub static PYTHON_IMPORT_EXTRACTOR: PythonImportExtractor = PythonImportExtractor;
