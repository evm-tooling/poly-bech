//! Full semantic token coverage using Tree-sitter CST
//!
//! This module provides comprehensive syntax highlighting by walking
//! the Tree-sitter concrete syntax tree and emitting semantic tokens
//! for all relevant nodes.

use crate::document::Document;
use once_cell::sync::Lazy;
use tower_lsp::lsp_types::*;
use tree_sitter::{Node, TreeCursor};

/// Semantic token types we support
pub const TOKEN_TYPES: &[SemanticTokenType] = &[
    SemanticTokenType::KEYWORD,
    SemanticTokenType::TYPE,
    SemanticTokenType::FUNCTION,
    SemanticTokenType::VARIABLE,
    SemanticTokenType::STRING,
    SemanticTokenType::NUMBER,
    SemanticTokenType::COMMENT,
    SemanticTokenType::PROPERTY,
    SemanticTokenType::NAMESPACE,
    SemanticTokenType::PARAMETER,
    SemanticTokenType::OPERATOR,
];

/// Semantic token modifiers we support
pub const TOKEN_MODIFIERS: &[SemanticTokenModifier] = &[
    SemanticTokenModifier::DEFINITION,
    SemanticTokenModifier::DECLARATION,
    SemanticTokenModifier::READONLY,
    SemanticTokenModifier::STATIC,
];

/// The semantic tokens legend
pub static LEGEND: Lazy<SemanticTokensLegend> = Lazy::new(|| SemanticTokensLegend {
    token_types: TOKEN_TYPES.to_vec(),
    token_modifiers: TOKEN_MODIFIERS.to_vec(),
});

/// Token type indices
const KEYWORD: u32 = 0;
const TYPE: u32 = 1;
const FUNCTION: u32 = 2;
const VARIABLE: u32 = 3;
const STRING: u32 = 4;
const NUMBER: u32 = 5;
const COMMENT: u32 = 6;
const PROPERTY: u32 = 7;
const NAMESPACE: u32 = 8;
const PARAMETER: u32 = 9;
const OPERATOR: u32 = 10;

/// Token modifier bits
const DEFINITION: u32 = 1 << 0;
const DECLARATION: u32 = 1 << 1;

/// A semantic token being built
struct TokenBuilder {
    tokens: Vec<SemanticToken>,
    prev_line: u32,
    prev_start: u32,
}

impl TokenBuilder {
    fn new() -> Self {
        Self { tokens: Vec::new(), prev_line: 0, prev_start: 0 }
    }

    fn push(&mut self, line: u32, start: u32, length: u32, token_type: u32, modifiers: u32) {
        let delta_line = line - self.prev_line;
        let delta_start = if delta_line == 0 { start - self.prev_start } else { start };

        self.tokens.push(SemanticToken {
            delta_line,
            delta_start,
            length,
            token_type,
            token_modifiers_bitset: modifiers,
        });

        self.prev_line = line;
        self.prev_start = start;
    }

    fn finish(self) -> Vec<SemanticToken> {
        self.tokens
    }
}

/// Get semantic tokens for a document
pub fn get_semantic_tokens(doc: &Document) -> Vec<SemanticToken> {
    let mut builder = TokenBuilder::new();
    let source = doc.source_text();

    let mut cursor = doc.tree.walk();
    visit_node(&mut cursor, &source, &mut builder);

    builder.finish()
}

fn visit_node(cursor: &mut TreeCursor, source: &str, builder: &mut TokenBuilder) {
    let node = cursor.node();

    // Emit token based on node type
    emit_token_for_node(node, source, builder);

    // Recurse into children
    if cursor.goto_first_child() {
        loop {
            visit_node(cursor, source, builder);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
}

fn emit_token_for_node(node: Node, source: &str, builder: &mut TokenBuilder) {
    let kind = node.kind();
    let start = node.start_position();
    let end = node.end_position();

    // Skip multi-line tokens for now (they need special handling)
    if start.row != end.row {
        return;
    }

    let line = start.row as u32;
    let start_col = start.column as u32;
    let length = (end.column - start.column) as u32;

    if length == 0 {
        return;
    }

    match kind {
        // Keywords
        "suite" | "bench" | "benchAsync" | "fixture" | "setup" | "after" | "before" | "each" |
        "globalSetup" | "import" | "declare" | "init" | "helpers" | "skip" | "validate" |
        "async" | "performance" | "memory" | "timeBased" | "iterationBased" | "sameDataset" => {
            // Only emit for the actual keyword token, not the whole construct
            if node.child_count() == 0 || is_keyword_text(node, source) {
                builder.push(line, start_col, length, KEYWORD, 0);
            }
        }

        // Use statement keyword
        "use" => {
            builder.push(line, start_col, length, KEYWORD, 0);
        }

        // Identifiers - context-dependent
        "identifier" => {
            emit_identifier(node, source, builder);
        }

        // Property names
        "property_name" | "chart_param_name" => {
            builder.push(line, start_col, length, PROPERTY, 0);
        }

        // Language tags
        "language_tag" => {
            builder.push(line, start_col, length, TYPE, 0);
        }

        // Charting module
        "charting" => {
            builder.push(line, start_col, length, NAMESPACE, 0);
        }

        // Chart function names
        "chart_function_name" => {
            builder.push(line, start_col, length, FUNCTION, 0);
        }

        // Literals
        "string" | "string_content" | "single_string_content" => {
            // Only emit for leaf string nodes to avoid double-counting
            if node.child_count() == 0 || kind == "string" {
                let text = node.utf8_text(source.as_bytes()).unwrap_or("");
                // Handle multi-line strings
                for (i, line_text) in text.lines().enumerate() {
                    let line_num = line + i as u32;
                    let col = if i == 0 { start_col } else { 0 };
                    let len = line_text.len() as u32;
                    if len > 0 {
                        builder.push(line_num, col, len, STRING, 0);
                    }
                }
            }
        }

        "number" | "float" => {
            builder.push(line, start_col, length, NUMBER, 0);
        }

        "boolean" => {
            builder.push(line, start_col, length, KEYWORD, 0);
        }

        "duration_unit" => {
            builder.push(line, start_col, length, KEYWORD, 0);
        }

        // Comments
        "comment" => {
            builder.push(line, start_col, length, COMMENT, 0);
        }

        // Operators and punctuation
        "::" => {
            builder.push(line, start_col, length, OPERATOR, 0);
        }

        // Special constructs
        "@file" => {
            builder.push(line, start_col, length, FUNCTION, 0);
        }

        // Escape sequences
        "escape_sequence" => {
            builder.push(line, start_col, length, STRING, 0);
        }

        _ => {
            // Don't emit tokens for structural nodes
        }
    }
}

fn is_keyword_text(node: Node, source: &str) -> bool {
    let text = node.utf8_text(source.as_bytes()).unwrap_or("");
    matches!(
        text,
        "suite" |
            "performance" |
            "memory" |
            "timeBased" |
            "iterationBased" |
            "sameDataset" |
            "bench" |
            "benchAsync" |
            "fixture" |
            "setup" |
            "after" |
            "before" |
            "each" |
            "globalSetup" |
            "import" |
            "declare" |
            "init" |
            "helpers" |
            "skip" |
            "validate" |
            "async" |
            "legacy" |
            "strict" |
            "timeBudgeted" |
            "fixedCap" |
            "use"
    )
}

fn emit_identifier(node: Node, _source: &str, builder: &mut TokenBuilder) {
    let start = node.start_position();
    let end = node.end_position();

    if start.row != end.row {
        return;
    }

    let line = start.row as u32;
    let start_col = start.column as u32;
    let length = (end.column - start.column) as u32;

    if length == 0 {
        return;
    }

    // Determine context from parent
    let parent = node.parent();
    let parent_kind = parent.map(|p| p.kind()).unwrap_or("");

    // Check if this is a field
    let field_name = parent.and_then(|p| {
        let mut cursor = p.walk();
        for child in p.children(&mut cursor) {
            if child.id() == node.id() {
                return p.field_name_for_child(child.id() as u32);
            }
        }
        None
    });

    let (token_type, modifiers) = match (parent_kind, field_name) {
        // Suite name
        (_, Some("name")) if parent_kind == "suite" => (TYPE, DEFINITION),

        // Benchmark name
        (_, Some("name")) if parent_kind == "benchmark" => (FUNCTION, DEFINITION),

        // Fixture name
        (_, Some("name")) if parent_kind == "fixture" => (VARIABLE, DEFINITION),

        // Fixture parameter name
        (_, Some("name")) if parent_kind == "fixture_param" => (PARAMETER, DECLARATION),

        // Fixture parameter type
        (_, Some("type")) if parent_kind == "fixture_param" => (TYPE, 0),

        // Module in use statement
        (_, Some("module")) if parent_kind == "use_statement" => (NAMESPACE, 0),

        // Anvil module
        ("anvil_call", _) => (NAMESPACE, 0),

        // Default - treat as variable
        _ => (VARIABLE, 0),
    };

    builder.push(line, start_col, length, token_type, modifiers);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_doc(source: &str) -> Document {
        Document::new(
            tower_lsp::lsp_types::Url::parse("file:///test.bench").unwrap(),
            source.to_string(),
            1,
        )
    }

    #[test]
    fn test_semantic_tokens_simple() {
        let source = r#"suite test {
    description: "A test"
    bench foo {
        go: run()
    }
}"#;
        let doc = make_doc(source);
        let tokens = get_semantic_tokens(&doc);

        // Should have tokens for: suite, test, description, "A test", bench, foo, go, run
        assert!(!tokens.is_empty());
    }

    #[test]
    fn test_semantic_tokens_with_comments() {
        let source = r#"# This is a comment
suite test {
    # Another comment
    bench foo {
        go: run()
    }
}"#;
        let doc = make_doc(source);
        let tokens = get_semantic_tokens(&doc);

        // Should include comment tokens
        assert!(!tokens.is_empty());
    }

    #[test]
    fn test_semantic_tokens_empty() {
        let doc = make_doc("");
        let tokens = get_semantic_tokens(&doc);
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_semantic_tokens_with_errors() {
        let source = r#"suite test {
    bench incomplete {
        go:
"#;
        let doc = make_doc(source);
        let tokens = get_semantic_tokens(&doc);

        // Should still produce tokens for valid parts
        assert!(!tokens.is_empty());
    }
}
