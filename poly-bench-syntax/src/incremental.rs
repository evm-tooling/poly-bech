//! Incremental parsing support
//!
//! This module provides incremental parsing capabilities using Tree-sitter.
//! Incremental parsing allows for fast re-parsing when only small edits
//! are made to the source code.

use tree_sitter::{InputEdit, Parser, Tree};

/// An incremental parser that reuses previous parse results
///
/// This parser maintains state between parses, allowing Tree-sitter
/// to reuse unchanged portions of the syntax tree.
pub struct IncrementalParser {
    parser: Parser,
    old_tree: Option<Tree>,
}

impl IncrementalParser {
    /// Create a new incremental parser
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_polybench::language())
            .expect("Error loading polybench grammar");

        Self { parser, old_tree: None }
    }

    /// Parse source code, optionally using an edit for incremental parsing
    ///
    /// If `edit` is provided, the parser will use the previous tree and
    /// the edit information to perform incremental parsing.
    ///
    /// # Arguments
    ///
    /// * `source` - The source code to parse
    /// * `edit` - Optional edit information for incremental parsing
    ///
    /// # Returns
    ///
    /// A Tree-sitter Tree representing the parsed syntax
    pub fn parse(&mut self, source: &str, edit: Option<&InputEdit>) -> Tree {
        // Apply edit to old tree if provided
        if let (Some(old), Some(edit)) = (&mut self.old_tree, edit) {
            old.edit(edit);
        }

        // Parse with or without the old tree
        let tree = self
            .parser
            .parse(source, self.old_tree.as_ref())
            .expect("Parser should always produce a tree");

        // Store the new tree for next incremental parse
        self.old_tree = Some(tree.clone());

        tree
    }

    /// Parse source code without incremental parsing
    ///
    /// This clears any cached state and performs a fresh parse.
    pub fn parse_fresh(&mut self, source: &str) -> Tree {
        self.old_tree = None;
        self.parse(source, None)
    }

    /// Reset the parser state
    ///
    /// Call this when the document has changed significantly and
    /// incremental parsing would not be beneficial.
    pub fn reset(&mut self) {
        self.old_tree = None;
    }

    /// Check if the parser has a cached tree
    pub fn has_cached_tree(&self) -> bool {
        self.old_tree.is_some()
    }
}

impl Default for IncrementalParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate an InputEdit from a text change
///
/// This is a helper function to create Tree-sitter InputEdit
/// from typical LSP-style change events.
///
/// # Arguments
///
/// * `old_source` - The source before the edit
/// * `start_byte` - The byte offset where the edit starts
/// * `old_end_byte` - The byte offset where the old text ends
/// * `new_end_byte` - The byte offset where the new text ends
/// * `start_point` - The (row, column) where the edit starts
/// * `old_end_point` - The (row, column) where the old text ends
/// * `new_end_point` - The (row, column) where the new text ends
pub fn create_input_edit(
    start_byte: usize,
    old_end_byte: usize,
    new_end_byte: usize,
    start_point: (usize, usize),
    old_end_point: (usize, usize),
    new_end_point: (usize, usize),
) -> InputEdit {
    InputEdit {
        start_byte,
        old_end_byte,
        new_end_byte,
        start_position: tree_sitter::Point::new(start_point.0, start_point.1),
        old_end_position: tree_sitter::Point::new(old_end_point.0, old_end_point.1),
        new_end_position: tree_sitter::Point::new(new_end_point.0, new_end_point.1),
    }
}

/// Calculate line and column from byte offset
pub fn byte_to_point(source: &str, byte_offset: usize) -> (usize, usize) {
    let mut line = 0;
    let mut col = 0;

    for (i, c) in source.char_indices() {
        if i >= byte_offset {
            break;
        }
        if c == '\n' {
            line += 1;
            col = 0;
        } else {
            col += c.len_utf8();
        }
    }

    (line, col)
}

/// Calculate byte offset from line and column
pub fn point_to_byte(source: &str, line: usize, column: usize) -> usize {
    let mut current_line = 0;
    let mut current_col = 0;

    for (i, c) in source.char_indices() {
        if current_line == line && current_col == column {
            return i;
        }
        if c == '\n' {
            if current_line == line {
                return i;
            }
            current_line += 1;
            current_col = 0;
        } else {
            current_col += c.len_utf8();
        }
    }

    source.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incremental_parser() {
        let mut parser = IncrementalParser::new();

        let source1 = r#"
declare suite test performance timeBased sameDataset: true {
    bench foo {
        go: run()
    }
}
"#;
        let tree1 = parser.parse(source1, None);
        assert!(!tree1.root_node().has_error());
        assert!(parser.has_cached_tree());

        // Make a small edit - change "foo" to "bar"
        let source2 = r#"
declare suite test performance timeBased sameDataset: true {
    bench bar {
        go: run()
    }
}
"#;
        let edit = create_input_edit(26, 29, 29, (2, 10), (2, 13), (2, 13));
        let tree2 = parser.parse(source2, Some(&edit));
        assert!(!tree2.root_node().has_error());
    }

    #[test]
    fn test_parse_fresh() {
        let mut parser = IncrementalParser::new();

        let source =
            "declare suite test performance timeBased sameDataset: true { bench foo { go: run() } }";
        let _ = parser.parse(source, None);
        assert!(parser.has_cached_tree());

        parser.reset();
        assert!(!parser.has_cached_tree());
    }

    #[test]
    fn test_byte_to_point() {
        let source = "line1\nline2\nline3";
        assert_eq!(byte_to_point(source, 0), (0, 0));
        assert_eq!(byte_to_point(source, 5), (0, 5));
        assert_eq!(byte_to_point(source, 6), (1, 0));
        assert_eq!(byte_to_point(source, 11), (1, 5));
        assert_eq!(byte_to_point(source, 12), (2, 0));
    }

    #[test]
    fn test_point_to_byte() {
        let source = "line1\nline2\nline3";
        assert_eq!(point_to_byte(source, 0, 0), 0);
        assert_eq!(point_to_byte(source, 0, 5), 5);
        assert_eq!(point_to_byte(source, 1, 0), 6);
        assert_eq!(point_to_byte(source, 1, 5), 11);
        assert_eq!(point_to_byte(source, 2, 0), 12);
    }
}
