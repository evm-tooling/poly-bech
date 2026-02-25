//! Traits and types for poly-bench IR
//!
//! Minimal crate for ImportExtractor and ParsedSetup, used by poly-bench-ir
//! and runtime crates without creating circular dependencies.

use poly_bench_dsl::Lang;

/// Trait for language-specific import extraction from setup blocks
pub trait ImportExtractor: Send + Sync {
    /// The language this extractor handles
    fn lang(&self) -> Lang;

    /// Extract imports from setup block code, returning imports and remaining body
    fn extract(&self, setup: &str) -> ParsedSetup;
}

/// Parsed setup block with imports separated from body
#[derive(Debug, Clone)]
pub struct ParsedSetup {
    /// Extracted import statements (formatted for output)
    pub imports: Vec<String>,
    /// Remaining body code (non-import declarations)
    pub body: String,
}

impl ParsedSetup {
    pub fn new(imports: Vec<String>, body: String) -> Self {
        Self { imports, body }
    }

    /// Create a ParsedSetup with no imports (passthrough)
    pub fn passthrough(code: &str) -> Self {
        Self { imports: Vec::new(), body: code.to_string() }
    }
}
