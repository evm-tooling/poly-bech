//! Tree-sitter wrapper for convenient tree traversal
//!
//! This module provides ergonomic wrappers around Tree-sitter's
//! tree traversal APIs.

use tree_sitter::{Node, Tree, TreeCursor};

/// A wrapper around Tree-sitter's Tree for convenient access
pub struct SyntaxTree {
    tree: Tree,
    source: String,
}

impl SyntaxTree {
    /// Create a new SyntaxTree from a Tree-sitter tree and source
    pub fn new(tree: Tree, source: String) -> Self {
        Self { tree, source }
    }

    /// Get the root node of the tree
    pub fn root(&self) -> Node<'_> {
        self.tree.root_node()
    }

    /// Get a cursor for traversing the tree
    pub fn cursor(&self) -> TreeCursor<'_> {
        self.tree.walk()
    }

    /// Get the source text for a node
    pub fn node_text(&self, node: Node<'_>) -> &str {
        node.utf8_text(self.source.as_bytes()).unwrap_or("<invalid utf8>")
    }

    /// Check if the tree has any errors
    pub fn has_errors(&self) -> bool {
        self.tree.root_node().has_error()
    }

    /// Get the underlying Tree-sitter tree
    pub fn inner(&self) -> &Tree {
        &self.tree
    }

    /// Get the source code
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Find all nodes of a specific kind
    pub fn find_nodes(&self, kind: &str) -> Vec<Node<'_>> {
        let mut nodes = Vec::new();
        let mut cursor = self.cursor();
        self.collect_nodes_of_kind(&mut cursor, kind, &mut nodes);
        nodes
    }

    fn collect_nodes_of_kind<'a>(
        &'a self,
        cursor: &mut TreeCursor<'a>,
        kind: &str,
        nodes: &mut Vec<Node<'a>>,
    ) {
        let node = cursor.node();
        if node.kind() == kind {
            nodes.push(node);
        }

        if cursor.goto_first_child() {
            loop {
                self.collect_nodes_of_kind(cursor, kind, nodes);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }
    }

    /// Find all error nodes in the tree
    pub fn find_errors(&self) -> Vec<Node<'_>> {
        self.find_nodes("ERROR")
    }

    /// Get the node at a specific byte offset
    pub fn node_at_offset(&self, offset: usize) -> Option<Node<'_>> {
        self.root().descendant_for_byte_range(offset, offset)
    }

    /// Get the node at a specific point (line, column)
    pub fn node_at_point(&self, line: usize, column: usize) -> Option<Node<'_>> {
        let point = tree_sitter::Point::new(line, column);
        self.root().descendant_for_point_range(point, point)
    }
}

/// Extension trait for Node to make text extraction easier
pub trait NodeExt<'a> {
    /// Get the text content of this node
    fn text(&self, source: &'a str) -> &'a str;

    /// Get a child node by field name
    fn field(&self, name: &str) -> Option<Node<'a>>;

    /// Get all children of a specific kind
    fn children_of_kind(&self, kind: &str) -> Vec<Node<'a>>;

    /// Check if this node is an error
    fn is_error(&self) -> bool;

    /// Check if this node is missing (expected but not found)
    fn is_missing(&self) -> bool;
}

impl<'a> NodeExt<'a> for Node<'a> {
    fn text(&self, source: &'a str) -> &'a str {
        self.utf8_text(source.as_bytes()).unwrap_or("<invalid utf8>")
    }

    fn field(&self, name: &str) -> Option<Node<'a>> {
        self.child_by_field_name(name)
    }

    fn children_of_kind(&self, kind: &str) -> Vec<Node<'a>> {
        let mut cursor = self.walk();
        self.children(&mut cursor).filter(|n| n.kind() == kind).collect()
    }

    fn is_error(&self) -> bool {
        self.kind() == "ERROR" || self.is_error()
    }

    fn is_missing(&self) -> bool {
        self.is_missing()
    }
}

/// Iterator over named children of a node
pub struct NamedChildren<'a> {
    cursor: TreeCursor<'a>,
    done: bool,
}

impl<'a> NamedChildren<'a> {
    pub fn new(node: Node<'a>) -> Self {
        let mut cursor = node.walk();
        let done = !cursor.goto_first_child();
        Self { cursor, done }
    }
}

impl<'a> Iterator for NamedChildren<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.done {
            let node = self.cursor.node();
            let is_named = node.is_named();

            if !self.cursor.goto_next_sibling() {
                self.done = true;
            }

            if is_named {
                return Some(node);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::IncrementalParser;

    #[test]
    fn test_syntax_tree() {
        let source = r#"
declare suite test performance timeBased sameDataset: true {
    bench foo {
        go: run()
    }
}
"#;
        let mut parser = IncrementalParser::new();
        let tree = parser.parse(source, None);
        let syntax_tree = SyntaxTree::new(tree, source.to_string());

        assert!(!syntax_tree.has_errors());
        assert_eq!(syntax_tree.root().kind(), "source_file");
    }

    #[test]
    fn test_find_nodes() {
        let source = r#"
declare suite test performance timeBased sameDataset: true {
    bench foo {
        go: run()
    }
    bench bar {
        go: test()
    }
}
"#;
        let mut parser = IncrementalParser::new();
        let tree = parser.parse(source, None);
        let syntax_tree = SyntaxTree::new(tree, source.to_string());

        let benchmarks = syntax_tree.find_nodes("benchmark");
        assert_eq!(benchmarks.len(), 2);
    }
}
