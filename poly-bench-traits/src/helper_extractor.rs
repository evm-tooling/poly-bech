//! Helper function extraction for undefined-function diagnostics
//!
//! Runtimes implement this trait to provide language-specific logic for
//! extracting function names from helpers blocks and builtin identifiers to ignore.

use std::collections::HashSet;

/// Trait for extracting function names from helpers code and builtin identifiers.
/// Used by LSP diagnostics for "undefined function" checks.
pub trait HelperFunctionExtractor: Send + Sync {
    /// Extract function names defined in a helpers block
    fn extract_functions(&self, code: &str) -> HashSet<String>;

    /// Builtin identifiers to ignore when checking for undefined function calls
    fn builtins(&self) -> &'static [&'static str];
}
