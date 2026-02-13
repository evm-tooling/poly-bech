//! Document management for the LSP
//!
//! This module handles parsing and storing documents,
//! providing utilities for position/offset conversion.

use poly_bench_dsl::{self as dsl, File, Span};
use ropey::Rope;

/// A parsed document with its source and AST
#[derive(Clone)]
pub struct ParsedDocument {
    /// The original source text
    pub source: String,
    /// Rope for efficient position/offset conversion
    pub rope: Rope,
    /// The parsed AST (None if parsing failed)
    pub ast: Option<File>,
    /// Parse error message if parsing failed
    pub parse_error: Option<ParseErrorInfo>,
    /// The filename for this document
    pub filename: String,
    /// Document version from the editor
    pub version: i32,
}

/// Information about a parse error
#[derive(Clone)]
pub struct ParseErrorInfo {
    pub message: String,
    pub span: Option<Span>,
}

impl ParsedDocument {
    /// Parse source text into a document
    pub fn parse(source: &str, filename: &str, version: i32) -> Self {
        let rope = Rope::from_str(source);

        match dsl::parse(source, filename) {
            Ok(ast) => Self {
                source: source.to_string(),
                rope,
                ast: Some(ast),
                parse_error: None,
                filename: filename.to_string(),
                version,
            },
            Err(err) => {
                // Extract span from miette error if available
                let span = extract_span_from_error(&err);
                Self {
                    source: source.to_string(),
                    rope,
                    ast: None,
                    parse_error: Some(ParseErrorInfo {
                        message: format!("{}", err),
                        span,
                    }),
                    filename: filename.to_string(),
                    version,
                }
            }
        }
    }

    /// Convert a byte offset to an LSP Position (line, character)
    pub fn offset_to_position(&self, offset: usize) -> tower_lsp::lsp_types::Position {
        let offset = offset.min(self.rope.len_bytes());
        let line = self.rope.byte_to_line(offset);
        let line_start = self.rope.line_to_byte(line);
        let character = offset - line_start;

        tower_lsp::lsp_types::Position {
            line: line as u32,
            character: character as u32,
        }
    }

    /// Convert an LSP Position to a byte offset
    pub fn position_to_offset(
        &self,
        position: tower_lsp::lsp_types::Position,
    ) -> Option<usize> {
        let line = position.line as usize;
        if line >= self.rope.len_lines() {
            return None;
        }

        let line_start = self.rope.line_to_byte(line);
        let line_end = if line + 1 < self.rope.len_lines() {
            self.rope.line_to_byte(line + 1)
        } else {
            self.rope.len_bytes()
        };

        let char_offset = position.character as usize;
        let offset = line_start + char_offset;

        if offset <= line_end {
            Some(offset)
        } else {
            Some(line_end)
        }
    }

    /// Convert a Span to an LSP Range
    pub fn span_to_range(&self, span: &Span) -> tower_lsp::lsp_types::Range {
        tower_lsp::lsp_types::Range {
            start: self.offset_to_position(span.start),
            end: self.offset_to_position(span.end),
        }
    }

    /// Get the word at a given position (for hover/completion context)
    pub fn word_at_position(
        &self,
        position: tower_lsp::lsp_types::Position,
    ) -> Option<(String, tower_lsp::lsp_types::Range)> {
        let offset = self.position_to_offset(position)?;

        // Find word boundaries
        let bytes = self.source.as_bytes();
        let mut start = offset;
        let mut end = offset;

        // Scan backwards for word start
        while start > 0 && is_word_char(bytes[start - 1]) {
            start -= 1;
        }

        // Scan forwards for word end
        while end < bytes.len() && is_word_char(bytes[end]) {
            end += 1;
        }

        if start == end {
            return None;
        }

        let word = String::from_utf8_lossy(&bytes[start..end]).to_string();
        let range = tower_lsp::lsp_types::Range {
            start: self.offset_to_position(start),
            end: self.offset_to_position(end),
        };

        Some((word, range))
    }

    /// Get the line content at a given position
    pub fn line_at_position(&self, position: tower_lsp::lsp_types::Position) -> Option<String> {
        let line = position.line as usize;
        if line >= self.rope.len_lines() {
            return None;
        }

        Some(self.rope.line(line).to_string())
    }

    /// Get text before the cursor on the current line (for completion context)
    pub fn text_before_position(
        &self,
        position: tower_lsp::lsp_types::Position,
    ) -> Option<String> {
        let line = position.line as usize;
        if line >= self.rope.len_lines() {
            return None;
        }

        let line_content = self.rope.line(line).to_string();
        let char_idx = (position.character as usize).min(line_content.len());

        Some(line_content[..char_idx].to_string())
    }
}

/// Check if a byte is a word character (alphanumeric or underscore)
fn is_word_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Try to extract a Span from a miette error
fn extract_span_from_error(err: &miette::Report) -> Option<Span> {
    // Try to downcast to our ParseError type
    if let Some(parse_err) = err.downcast_ref::<dsl::ParseError>() {
        match parse_err {
            dsl::ParseError::UnexpectedChar { span, .. } => Some(span.clone()),
            dsl::ParseError::UnterminatedString { span } => Some(span.clone()),
            dsl::ParseError::InvalidEscape { span, .. } => Some(span.clone()),
            dsl::ParseError::InvalidNumber { span } => Some(span.clone()),
            dsl::ParseError::ExpectedToken { span, .. } => Some(span.clone()),
            dsl::ParseError::ExpectedIdentifier { span } => Some(span.clone()),
            dsl::ParseError::UnknownLang { span, .. } => Some(span.clone()),
            dsl::ParseError::Duplicate { span, .. } => Some(span.clone()),
            dsl::ParseError::UnclosedBrace { span } => Some(span.clone()),
            dsl::ParseError::UnexpectedEof { span } => Some(span.clone()),
            dsl::ParseError::InvalidProperty { span, .. } => Some(span.clone()),
        }
    } else {
        None
    }
}
