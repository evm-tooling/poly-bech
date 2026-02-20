//! Tree-sitter grammar for the poly-bench DSL
//!
//! This crate provides Rust bindings for the poly-bench Tree-sitter grammar.
//! It enables robust, incremental parsing with proper error recovery.

use tree_sitter::Language;

extern "C" {
    fn tree_sitter_polybench() -> Language;
}

/// Returns the tree-sitter Language for poly-bench.
///
/// # Example
///
/// ```
/// let language = tree_sitter_polybench::language();
/// let mut parser = tree_sitter::Parser::new();
/// parser.set_language(&language).expect("Error loading polybench grammar");
/// ```
pub fn language() -> Language {
    unsafe { tree_sitter_polybench() }
}

/// The syntax highlighting query for poly-bench.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../queries/highlights.scm");

/// The injection query for embedded languages.
pub const INJECTIONS_QUERY: &str = include_str!("../../queries/injections.scm");

/// The locals query for scope tracking.
pub const LOCALS_QUERY: &str = include_str!("../../queries/locals.scm");

/// The content of the [`node-types.json`][] file for this grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES: &str = include_str!("../../src/node-types.json");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&language()).expect("Error loading polybench grammar");
    }

    #[test]
    fn test_parse_simple_suite() {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&language()).expect("Error loading polybench grammar");

        let source = r#"
suite test {
    description: "A test suite"
    iterations: 100

    bench example {
        go: doSomething()
    }
}
"#;

        let tree = parser.parse(source, None).expect("Failed to parse");
        let root = tree.root_node();

        assert_eq!(root.kind(), "source_file");
        assert!(!root.has_error());
    }

    #[test]
    fn test_parse_with_embedded_code() {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&language()).expect("Error loading polybench grammar");

        let source = r#"
suite test {
    setup go {
        import (
            "fmt"
            "testing"
        )

        helpers {
            func helper() {
                if true {
                    fmt.Println("nested braces")
                }
            }
        }
    }

    bench example {
        go: helper()
    }
}
"#;

        let tree = parser.parse(source, None).expect("Failed to parse");
        let root = tree.root_node();

        assert_eq!(root.kind(), "source_file");
        // The grammar should handle nested braces correctly
    }

    #[test]
    fn test_error_recovery() {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&language()).expect("Error loading polybench grammar");

        // Incomplete input - should still parse what it can
        let source = r#"
suite test {
    bench incomplete {
        go:
"#;

        let tree = parser.parse(source, None).expect("Failed to parse");
        let root = tree.root_node();

        // Should have parsed something, even with errors
        assert_eq!(root.kind(), "source_file");
        // Tree-sitter should mark errors but still produce a tree
        assert!(root.has_error());
    }
}
