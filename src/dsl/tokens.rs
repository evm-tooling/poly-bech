//! Token types for the poly-bench DSL lexer

use crate::dsl::ast::Span;

/// Token types for the DSL
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Suite,
    Bench,
    Setup,
    Fixture,
    Hex,
    Description,
    Iterations,
    Warmup,

    // Language keywords
    Go,
    Ts,
    TypeScript,
    Rust,
    Python,

    // Literals
    Identifier(String),
    String(String),
    Number(u64),
    HexLiteral(String),

    // Punctuation
    LBrace,      // {
    RBrace,      // }
    LParen,      // (
    RParen,      // )
    Colon,       // :
    At,          // @

    // Special
    FileRef,     // @file
    Comment(String),
    
    // End of file
    Eof,
}

impl TokenKind {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            TokenKind::Suite
                | TokenKind::Bench
                | TokenKind::Setup
                | TokenKind::Fixture
                | TokenKind::Hex
                | TokenKind::Description
                | TokenKind::Iterations
                | TokenKind::Warmup
        )
    }

    pub fn is_lang(&self) -> bool {
        matches!(
            self,
            TokenKind::Go | TokenKind::Ts | TokenKind::TypeScript | TokenKind::Rust | TokenKind::Python
        )
    }
}

/// A token with its span information
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub lexeme: String,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span, lexeme: String) -> Self {
        Self { kind, span, lexeme }
    }
}

/// Map string to keyword token kind
pub fn keyword_from_str(s: &str) -> Option<TokenKind> {
    match s {
        "suite" => Some(TokenKind::Suite),
        "bench" => Some(TokenKind::Bench),
        "setup" => Some(TokenKind::Setup),
        "fixture" => Some(TokenKind::Fixture),
        "hex" => Some(TokenKind::Hex),
        "description" => Some(TokenKind::Description),
        "iterations" => Some(TokenKind::Iterations),
        "warmup" => Some(TokenKind::Warmup),
        "go" => Some(TokenKind::Go),
        "ts" => Some(TokenKind::Ts),
        "typescript" => Some(TokenKind::TypeScript),
        "rust" => Some(TokenKind::Rust),
        "python" => Some(TokenKind::Python),
        _ => None,
    }
}
