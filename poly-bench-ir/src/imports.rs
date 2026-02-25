//! Import extraction for setup blocks
//!
//! This module provides language-specific import extraction from setup blocks.
//! Imports are separated from the body so generators can place them at the
//! correct location in generated code (e.g., top of file for Go/TypeScript).
//!
//! Extractors are registered at startup via `set_import_extractors()` (called by
//! poly-bench-runtime). If unset, `extract_imports` falls back to passthrough.

use poly_bench_dsl::Lang;
use poly_bench_ir_traits::{ImportExtractor, ParsedSetup};
use std::sync::Mutex;

static EXTRACTORS: Mutex<Option<Vec<&'static dyn ImportExtractor>>> = Mutex::new(None);

/// Register import extractors from runtime crates.
/// Called by poly-bench-runtime at application startup.
pub fn set_import_extractors(extractors: Vec<&'static dyn ImportExtractor>) {
    *EXTRACTORS.lock().unwrap() = Some(extractors);
}

/// Extract imports from setup block code for the given language.
/// Uses the registry if set; otherwise falls back to passthrough.
pub fn extract_imports(lang: Lang, setup: &str) -> ParsedSetup {
    match EXTRACTORS.lock().unwrap().as_ref() {
        Some(extractors) => extractors
            .iter()
            .find(|e| e.lang() == lang)
            .map(|e| e.extract(setup))
            .unwrap_or_else(|| ParsedSetup::passthrough(setup)),
        None => ParsedSetup::passthrough(setup),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passthrough_when_unset() {
        let setup = "some code";
        let parsed = extract_imports(Lang::Go, setup);
        assert_eq!(parsed.imports, vec![] as Vec<String>);
        assert_eq!(parsed.body, "some code");
    }
}
