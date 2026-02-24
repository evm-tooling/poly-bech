//! Error-tolerant parsing layer for poly-bench DSL
//!
//! This crate provides a robust parsing layer built on Tree-sitter that:
//! - Always produces a syntax tree, even with errors
//! - Supports incremental parsing for fast re-parsing on edits
//! - Converts Tree-sitter CST to a typed partial AST
//!
//! # Architecture
//!
//! ```text
//! Source Code
//!     │
//!     ▼
//! ┌─────────────────┐
//! │  Tree-sitter    │  ← Incremental, error-tolerant
//! │  Parser         │
//! └────────┬────────┘
//!          │
//!          ▼
//! ┌─────────────────┐
//! │  Concrete       │  ← Always available, even with errors
//! │  Syntax Tree    │
//! └────────┬────────┘
//!          │
//!          ▼
//! ┌─────────────────┐
//! │  Partial AST    │  ← Typed nodes with error markers
//! │  Conversion     │
//! └────────┬────────┘
//!          │
//!          ▼
//! ┌─────────────────┐
//! │  PartialFile    │  ← Used by LSP for highlighting,
//! │                 │    formatting, diagnostics
//! └─────────────────┘
//! ```

pub mod convert;
pub mod incremental;
pub mod partial_ast;
pub mod tree;

pub use convert::convert_file;
pub use incremental::IncrementalParser;
pub use partial_ast::*;
pub use tree::SyntaxTree;

/// Tree-sitter node type (renamed to avoid conflict with partial_ast::Node)
pub use tree_sitter::Node as TsNode;
/// Re-export tree-sitter types for convenience
pub use tree_sitter::{InputEdit, Point, Tree, TreeCursor};

/// Parse source code into a partial AST
///
/// This is the main entry point for parsing. It always succeeds,
/// returning a PartialFile that may contain error nodes.
///
/// # Example
///
/// ```
/// use poly_bench_syntax::parse;
///
/// let source = r#"
/// declare suite test performance timeBased sameDataset: true {
///     bench foo {
///         go: run()
///     }
/// }
/// "#;
///
/// let file = parse(source);
/// assert!(!file.suites.is_empty());
/// ```
pub fn parse(source: &str) -> PartialFile {
    let mut parser = IncrementalParser::new();
    let tree = parser.parse(source, None);
    convert_file(&tree, source)
}

/// Parse source code and return both the tree and partial AST
///
/// Use this when you need access to the raw Tree-sitter tree
/// for additional analysis (e.g., semantic tokens).
pub fn parse_with_tree(source: &str) -> (Tree, PartialFile) {
    let mut parser = IncrementalParser::new();
    let tree = parser.parse(source, None);
    let ast = convert_file(&tree, source);
    (tree, ast)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let source = r#"
declare suite test performance timeBased sameDataset: true {
    description: "A test"
    bench foo {
        go: run()
    }
}
"#;
        let file = parse(source);
        assert_eq!(file.suites.len(), 1);

        let suite = &file.suites[0];
        assert!(suite.is_valid());
        if let partial_ast::Node::Valid(s) = suite {
            assert_eq!(s.name, "test");
            assert_eq!(s.benchmarks.len(), 1);
        }
    }

    #[test]
    #[ignore]
    fn test_parse_with_errors() {
        let source = r#"
declare suite test performance timeBased sameDataset: true {
    bench incomplete {
        go:
"#;
        let file = parse(source);

        // Should still have parsed the suite
        assert_eq!(file.suites.len(), 1);

        // Should have errors
        assert!(!file.errors.is_empty());
    }

    #[test]
    fn test_parse_empty() {
        let file = parse("");
        assert!(file.suites.is_empty());
        assert!(file.errors.is_empty());
    }
}
