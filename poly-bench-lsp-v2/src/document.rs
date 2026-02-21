//! Document management with Tree-sitter
//!
//! This module handles document state, including:
//! - Source text storage using Rope for efficient edits
//! - Tree-sitter syntax tree
//! - Partial AST for semantic analysis

use poly_bench_syntax::{convert_file, IncrementalParser, PartialFile, Span, Tree};
use ropey::Rope;
use tower_lsp::lsp_types::*;

/// A document being edited
pub struct Document {
    /// The document URI
    pub uri: Url,
    /// The source text as a rope for efficient edits
    pub source: Rope,
    /// The Tree-sitter syntax tree
    pub tree: Tree,
    /// The partial AST (always available, even with errors)
    pub partial_ast: PartialFile,
    /// Document version
    pub version: i32,
    /// Incremental parser
    parser: IncrementalParser,
}

impl Document {
    /// Create a new document from source text
    pub fn new(uri: Url, source: String, version: i32) -> Self {
        let mut parser = IncrementalParser::new();
        let tree = parser.parse(&source, None);
        let partial_ast = convert_file(&tree, &source);
        let rope = Rope::from_str(&source);

        Self { uri, source: rope, tree, partial_ast, version, parser }
    }

    /// Apply a text edit to the document
    pub fn apply_edit(&mut self, edit: &TextDocumentContentChangeEvent, version: i32) {
        self.version = version;

        match &edit.range {
            Some(range) => {
                // Incremental edit
                let start_line = range.start.line as usize;
                let start_col = range.start.character as usize;
                let end_line = range.end.line as usize;
                let end_col = range.end.character as usize;

                // Calculate byte offsets
                let start_byte = self.position_to_byte(start_line, start_col);
                let end_byte = self.position_to_byte(end_line, end_col);

                // Apply edit to rope
                let start_char = self.source.byte_to_char(start_byte);
                let end_char = self.source.byte_to_char(end_byte);
                self.source.remove(start_char..end_char);
                self.source.insert(start_char, &edit.text);

                // Calculate new end position
                let new_end_byte = start_byte + edit.text.len();
                let (new_end_line, new_end_col) = self.byte_to_position(new_end_byte);

                // Create Tree-sitter input edit
                let input_edit = tree_sitter::InputEdit {
                    start_byte,
                    old_end_byte: end_byte,
                    new_end_byte,
                    start_position: tree_sitter::Point::new(start_line, start_col),
                    old_end_position: tree_sitter::Point::new(end_line, end_col),
                    new_end_position: tree_sitter::Point::new(new_end_line, new_end_col),
                };

                // Incremental re-parse
                let source_str = self.source.to_string();
                self.tree = self.parser.parse(&source_str, Some(&input_edit));
                self.partial_ast = convert_file(&self.tree, &source_str);
            }
            None => {
                // Full document replacement
                self.source = Rope::from_str(&edit.text);
                self.parser.reset();
                self.tree = self.parser.parse(&edit.text, None);
                self.partial_ast = convert_file(&self.tree, &edit.text);
            }
        }
    }

    /// Get the source text as a string
    pub fn source_text(&self) -> String {
        self.source.to_string()
    }

    /// Get a slice of the source text
    pub fn source_slice(&self, span: &Span) -> String {
        let start_char = self.source.byte_to_char(span.start);
        let end_char = self.source.byte_to_char(span.end.min(self.source.len_bytes()));
        self.source.slice(start_char..end_char).to_string()
    }

    /// Convert a byte offset to (line, column)
    pub fn byte_to_position(&self, byte: usize) -> (usize, usize) {
        let byte = byte.min(self.source.len_bytes());
        let char_idx = self.source.byte_to_char(byte);
        let line = self.source.char_to_line(char_idx);
        let line_start = self.source.line_to_char(line);
        let col = char_idx - line_start;
        (line, col)
    }

    /// Convert (line, column) to byte offset
    pub fn position_to_byte(&self, line: usize, col: usize) -> usize {
        let line = line.min(self.source.len_lines().saturating_sub(1));
        let line_start = self.source.line_to_char(line);
        let line_len = self.source.line(line).len_chars();
        let col = col.min(line_len);
        let char_idx = line_start + col;
        self.source.char_to_byte(char_idx)
    }

    /// Convert a Span to an LSP Range
    pub fn span_to_range(&self, span: &Span) -> Range {
        Range {
            start: Position { line: span.start_line as u32, character: span.start_col as u32 },
            end: Position { line: span.end_line as u32, character: span.end_col as u32 },
        }
    }

    /// Check if the document has any syntax errors
    pub fn has_errors(&self) -> bool {
        self.tree.root_node().has_error() || self.partial_ast.has_errors()
    }

    /// Get the number of lines in the document
    pub fn line_count(&self) -> usize {
        self.source.len_lines()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_new() {
        let source = r#"
suite test {
    bench foo {
        go: run()
    }
}
"#;
        let doc = Document::new(Url::parse("file:///test.bench").unwrap(), source.to_string(), 1);

        assert!(!doc.has_errors());
        assert_eq!(doc.partial_ast.suites.len(), 1);
    }

    #[test]
    fn test_document_incremental_edit() {
        let source = r#"suite test {
    bench foo {
        go: run()
    }
}"#;
        let mut doc =
            Document::new(Url::parse("file:///test.bench").unwrap(), source.to_string(), 1);

        // Change "foo" to "bar"
        let edit = TextDocumentContentChangeEvent {
            range: Some(Range {
                start: Position { line: 1, character: 10 },
                end: Position { line: 1, character: 13 },
            }),
            range_length: None,
            text: "bar".to_string(),
        };

        doc.apply_edit(&edit, 2);

        assert!(!doc.has_errors());
        let suite = doc.partial_ast.suites[0].as_valid().unwrap();
        let bench = suite.benchmarks[0].as_valid().unwrap();
        assert_eq!(bench.name, "bar");
    }

    #[test]
    #[ignore]
    fn test_document_full_replacement() {
        let source1 = "suite test { bench foo { go: run() } }";
        let mut doc =
            Document::new(Url::parse("file:///test.bench").unwrap(), source1.to_string(), 1);

        let source2 = "suite other { bench bar { ts: test() } }";
        let edit = TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text: source2.to_string(),
        };

        doc.apply_edit(&edit, 2);

        let suite = doc.partial_ast.suites[0].as_valid().unwrap();
        assert_eq!(suite.name, "other");
    }

    #[test]
    fn test_byte_position_conversion() {
        let source = "line1\nline2\nline3";
        let doc = Document::new(Url::parse("file:///test.bench").unwrap(), source.to_string(), 1);

        assert_eq!(doc.byte_to_position(0), (0, 0));
        assert_eq!(doc.byte_to_position(6), (1, 0));
        assert_eq!(doc.byte_to_position(12), (2, 0));

        assert_eq!(doc.position_to_byte(0, 0), 0);
        assert_eq!(doc.position_to_byte(1, 0), 6);
        assert_eq!(doc.position_to_byte(2, 0), 12);
    }
}
