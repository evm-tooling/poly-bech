//! Error types for DSL parsing

use crate::ast::Span;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

/// Parse error with source location
#[derive(Error, Debug, Diagnostic)]
pub enum ParseError {
    #[error("Unexpected character '{char}'")]
    #[diagnostic(code(poly_bench::parse::unexpected_char))]
    UnexpectedChar {
        char: char,
        #[label("unexpected character here")]
        span: Span,
    },

    #[error("Unterminated string literal")]
    #[diagnostic(code(poly_bench::parse::unterminated_string))]
    UnterminatedString {
        #[label("string starts here but never ends")]
        span: Span,
    },

    #[error("Invalid escape sequence '\\{char}'")]
    #[diagnostic(code(poly_bench::parse::invalid_escape))]
    InvalidEscape {
        char: char,
        #[label("invalid escape here")]
        span: Span,
    },

    #[error("Invalid number literal")]
    #[diagnostic(code(poly_bench::parse::invalid_number))]
    InvalidNumber {
        #[label("invalid number here")]
        span: Span,
    },

    #[error("Expected {expected}, found {found}")]
    #[diagnostic(code(poly_bench::parse::expected_token))]
    ExpectedToken {
        expected: String,
        found: String,
        #[label("expected {expected} here")]
        span: Span,
    },

    #[error("Expected identifier")]
    #[diagnostic(code(poly_bench::parse::expected_identifier))]
    ExpectedIdentifier {
        #[label("expected identifier here")]
        span: Span,
    },

    #[error("Unknown language '{lang}'")]
    #[diagnostic(
        code(poly_bench::parse::unknown_lang),
        help("supported languages: go, ts, typescript, rust, python, c, csharp")
    )]
    UnknownLang {
        lang: String,
        #[label("unknown language")]
        span: Span,
    },

    #[error("Duplicate {item} '{name}'")]
    #[diagnostic(code(poly_bench::parse::duplicate))]
    Duplicate {
        item: String,
        name: String,
        #[label("duplicate {item} here")]
        span: Span,
    },

    #[error("Unclosed brace")]
    #[diagnostic(code(poly_bench::parse::unclosed_brace))]
    UnclosedBrace {
        #[label("opening brace here")]
        span: Span,
    },

    #[error("Unexpected end of file")]
    #[diagnostic(code(poly_bench::parse::unexpected_eof))]
    UnexpectedEof {
        #[label("file ends unexpectedly here")]
        span: Span,
    },

    #[error("Invalid property '{name}'")]
    #[diagnostic(
        code(poly_bench::parse::invalid_property),
        help("valid properties: description, iterations, warmup, hex")
    )]
    InvalidProperty {
        name: String,
        #[label("invalid property")]
        span: Span,
    },
}

// Implement From<Span> for SourceSpan to work with miette
impl From<Span> for SourceSpan {
    fn from(span: Span) -> Self {
        SourceSpan::new(span.start.into(), (span.end - span.start).into())
    }
}

/// A wrapper for source code that implements miette::SourceCode
#[derive(Debug)]
pub struct NamedSource {
    name: String,
    source: String,
}

impl NamedSource {
    pub fn new(name: impl Into<String>, source: impl Into<String>) -> Self {
        Self { name: name.into(), source: source.into() }
    }
}

impl miette::SourceCode for NamedSource {
    fn read_span<'a>(
        &'a self,
        span: &SourceSpan,
        context_lines_before: usize,
        context_lines_after: usize,
    ) -> Result<Box<dyn miette::SpanContents<'a> + 'a>, miette::MietteError> {
        let contents = self.source.as_str();

        let start = span.offset();
        let len = span.len();
        let end = start + len;

        // Find line boundaries for context
        let mut line_start = start;
        let mut lines_before = 0;
        for (i, c) in contents[..start].char_indices().rev() {
            if c == '\n' {
                lines_before += 1;
                if lines_before > context_lines_before {
                    line_start = i + 1;
                    break;
                }
            }
            line_start = i;
        }

        let mut line_end = end;
        let mut lines_after = 0;
        for (i, c) in contents[end..].char_indices() {
            if c == '\n' {
                lines_after += 1;
                if lines_after > context_lines_after {
                    break;
                }
            }
            line_end = end + i + 1;
        }
        line_end = line_end.min(contents.len());

        // Count line number
        let line = contents[..start].chars().filter(|&c| c == '\n').count();
        let column = start - contents[..start].rfind('\n').map(|p| p + 1).unwrap_or(0);

        Ok(Box::new(miette::MietteSpanContents::new_named(
            self.name.clone(),
            contents[line_start..line_end].as_bytes(),
            SourceSpan::new((start - line_start).into(), len.into()),
            line,
            column,
            lines_before + 1 + lines_after,
        )))
    }
}
