//! Lexer for the poly-bench DSL
//!
//! Converts source text into a stream of tokens.

use crate::{
    ast::Span,
    error::ParseError,
    tokens::{keyword_from_str, Token, TokenKind},
};

/// Lexer state
pub struct Lexer<'a> {
    source: &'a str,
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
    current_pos: usize,
    line: usize,
    col: usize,
    line_start: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.char_indices().peekable(),
            current_pos: 0,
            line: 1,
            col: 1,
            line_start: 0,
        }
    }

    /// Tokenize the entire source
    pub fn tokenize(&mut self) -> Result<Vec<Token>, ParseError> {
        let mut tokens = Vec::new();

        loop {
            match self.next_token() {
                Ok(token) => {
                    let is_eof = token.kind == TokenKind::Eof;
                    tokens.push(token);
                    if is_eof {
                        break;
                    }
                }
                Err(e) => return Err(e),
            }
        }

        Ok(tokens)
    }

    /// Get the next token
    pub fn next_token(&mut self) -> Result<Token, ParseError> {
        self.skip_whitespace_and_comments();

        let start_pos = self.current_pos;
        let start_line = self.line;
        let start_col = self.col;

        let Some((pos, ch)) = self.advance() else {
            return Ok(Token::new(
                TokenKind::Eof,
                Span::new(start_pos, start_pos, start_line, start_col),
                String::new(),
            ));
        };

        let (kind, lexeme) = match ch {
            '{' => (TokenKind::LBrace, "{".to_string()),
            '}' => (TokenKind::RBrace, "}".to_string()),
            '(' => (TokenKind::LParen, "(".to_string()),
            ')' => (TokenKind::RParen, ")".to_string()),
            '[' => (TokenKind::LBracket, "[".to_string()),
            ']' => (TokenKind::RBracket, "]".to_string()),
            ':' => {
                // Check for ::
                if self.peek() == Some(':') {
                    self.advance();
                    (TokenKind::DoubleColon, "::".to_string())
                } else {
                    (TokenKind::Colon, ":".to_string())
                }
            }
            ',' => (TokenKind::Comma, ",".to_string()),
            '@' => {
                // Check for @file
                if self.peek_str("file") {
                    self.advance_n(4);
                    (TokenKind::FileRef, "@file".to_string())
                } else {
                    (TokenKind::At, "@".to_string())
                }
            }
            '.' => (TokenKind::Dot, ".".to_string()),
            '"' => self.scan_string()?,
            '\'' => self.scan_single_quote_string()?,
            c if c.is_ascii_digit() => self.scan_number_or_duration(pos)?,
            c if c.is_ascii_alphabetic() || c == '_' => self.scan_identifier(pos)?,
            // Code characters - treat as identifiers for inline code
            '+' | '-' | '*' | '/' | '%' | '=' | '<' | '>' | '!' | '&' | '|' | ';' | '?' | '^'
            | '~' | '`' => {
                // Scan the rest as a code expression
                self.scan_code_expr(pos, ch)?
            }
            _ => {
                return Err(ParseError::UnexpectedChar {
                    char: ch,
                    span: Span::new(start_pos, self.current_pos, start_line, start_col),
                });
            }
        };

        Ok(Token::new(kind, Span::new(start_pos, self.current_pos, start_line, start_col), lexeme))
    }

    /// Advance and return the next character
    fn advance(&mut self) -> Option<(usize, char)> {
        let result = self.chars.next();
        if let Some((pos, ch)) = result {
            self.current_pos = pos + ch.len_utf8();
            if ch == '\n' {
                self.line += 1;
                self.col = 1;
                self.line_start = self.current_pos;
            } else {
                self.col += 1;
            }
        }
        result
    }

    /// Advance n characters
    fn advance_n(&mut self, n: usize) {
        for _ in 0..n {
            self.advance();
        }
    }

    /// Peek at the next character without consuming
    fn peek(&mut self) -> Option<char> {
        self.chars.peek().map(|(_, ch)| *ch)
    }

    /// Peek at the character after next
    fn peek_next(&self) -> Option<char> {
        self.source[self.current_pos..].chars().nth(1)
    }

    /// Check if the next characters match a string
    fn peek_str(&self, s: &str) -> bool {
        self.source[self.current_pos..].starts_with(s)
    }

    /// Skip whitespace and comments
    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.peek() {
                Some(c) if c.is_whitespace() => {
                    self.advance();
                }
                Some('#') => {
                    // Skip until end of line
                    while let Some(c) = self.peek() {
                        if c == '\n' {
                            break;
                        }
                        self.advance();
                    }
                }
                _ => break,
            }
        }
    }

    /// Scan a string literal
    fn scan_string(&mut self) -> Result<(TokenKind, String), ParseError> {
        let start_line = self.line;
        let start_col = self.col - 1; // Account for opening quote
        let start_pos = self.current_pos - 1;

        let mut value = String::new();

        loop {
            match self.advance() {
                Some((_, '"')) => break,
                Some((_, '\\')) => {
                    // Handle escape sequences
                    match self.advance() {
                        Some((_, 'n')) => value.push('\n'),
                        Some((_, 't')) => value.push('\t'),
                        Some((_, 'r')) => value.push('\r'),
                        Some((_, '\\')) => value.push('\\'),
                        Some((_, '"')) => value.push('"'),
                        Some((_, c)) => {
                            return Err(ParseError::InvalidEscape {
                                char: c,
                                span: Span::new(start_pos, self.current_pos, start_line, start_col),
                            });
                        }
                        None => {
                            return Err(ParseError::UnterminatedString {
                                span: Span::new(start_pos, self.current_pos, start_line, start_col),
                            });
                        }
                    }
                }
                Some((_, c)) => value.push(c),
                None => {
                    return Err(ParseError::UnterminatedString {
                        span: Span::new(start_pos, self.current_pos, start_line, start_col),
                    });
                }
            }
        }

        Ok((TokenKind::String(value.clone()), format!("\"{}\"", value)))
    }

    /// Scan a number literal or duration (e.g., 30s, 500ms, 1m)
    fn scan_number_or_duration(&mut self, start: usize) -> Result<(TokenKind, String), ParseError> {
        // Scan digits
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        // Check for decimal point (floating point number)
        let is_float = if self.peek() == Some('.') {
            // Peek ahead to make sure it's followed by a digit (not a method call)
            if let Some(next) = self.peek_next() {
                if next.is_ascii_digit() {
                    self.advance(); // consume '.'
                                    // Scan fractional part
                    while let Some(c) = self.peek() {
                        if c.is_ascii_digit() || c == '_' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };

        // Check for duration suffix (s, ms, m) - only for integers
        let has_suffix = if !is_float {
            if let Some(c) = self.peek() {
                if c == 'm' {
                    self.advance();
                    // Check for 'ms' vs just 'm' (minutes)
                    if self.peek() == Some('s') {
                        self.advance();
                        true
                    } else {
                        true // 'm' for minutes
                    }
                } else if c == 's' {
                    self.advance();
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };

        let lexeme = &self.source[start..self.current_pos];

        if has_suffix {
            // Parse as duration
            let ms = Self::parse_duration_to_ms(lexeme).map_err(|_| ParseError::InvalidNumber {
                span: Span::new(start, self.current_pos, self.line, self.col),
            })?;
            Ok((TokenKind::Duration(ms), lexeme.to_string()))
        } else if is_float {
            // Parse as floating point number
            let clean = lexeme.replace('_', "");
            let value = clean.parse::<f64>().map_err(|_| ParseError::InvalidNumber {
                span: Span::new(start, self.current_pos, self.line, self.col),
            })?;
            Ok((TokenKind::Float(value), lexeme.to_string()))
        } else {
            // Parse as regular integer
            let clean = lexeme.replace('_', "");
            let value = clean.parse::<u64>().map_err(|_| ParseError::InvalidNumber {
                span: Span::new(start, self.current_pos, self.line, self.col),
            })?;
            Ok((TokenKind::Number(value), lexeme.to_string()))
        }
    }

    /// Parse a duration string to milliseconds
    fn parse_duration_to_ms(s: &str) -> Result<u64, ()> {
        let s = s.replace('_', "");
        if let Some(num) = s.strip_suffix("ms") {
            num.parse::<u64>().map_err(|_| ())
        } else if let Some(num) = s.strip_suffix('s') {
            num.parse::<u64>().map(|n| n * 1000).map_err(|_| ())
        } else if let Some(num) = s.strip_suffix('m') {
            num.parse::<u64>().map(|n| n * 60 * 1000).map_err(|_| ())
        } else {
            Err(())
        }
    }

    /// Scan an identifier or keyword
    fn scan_identifier(&mut self, start: usize) -> Result<(TokenKind, String), ParseError> {
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let lexeme = &self.source[start..self.current_pos];

        // Check if it's a keyword
        let kind =
            keyword_from_str(lexeme).unwrap_or_else(|| TokenKind::Identifier(lexeme.to_string()));

        Ok((kind, lexeme.to_string()))
    }

    /// Scan a single-quoted string (for JS imports)
    fn scan_single_quote_string(&mut self) -> Result<(TokenKind, String), ParseError> {
        let start_line = self.line;
        let start_col = self.col - 1;
        let start_pos = self.current_pos - 1;

        let mut value = String::new();

        loop {
            match self.advance() {
                Some((_, '\'')) => break,
                Some((_, '\\')) => match self.advance() {
                    Some((_, 'n')) => value.push('\n'),
                    Some((_, 't')) => value.push('\t'),
                    Some((_, 'r')) => value.push('\r'),
                    Some((_, '\\')) => value.push('\\'),
                    Some((_, '\'')) => value.push('\''),
                    Some((_, c)) => {
                        return Err(ParseError::InvalidEscape {
                            char: c,
                            span: Span::new(start_pos, self.current_pos, start_line, start_col),
                        });
                    }
                    None => {
                        return Err(ParseError::UnterminatedString {
                            span: Span::new(start_pos, self.current_pos, start_line, start_col),
                        });
                    }
                },
                Some((_, c)) => value.push(c),
                None => {
                    return Err(ParseError::UnterminatedString {
                        span: Span::new(start_pos, self.current_pos, start_line, start_col),
                    });
                }
            }
        }

        Ok((TokenKind::String(value.clone()), format!("'{}'", value)))
    }

    /// Scan a code expression (for inline code)
    fn scan_code_expr(
        &mut self,
        start: usize,
        first_char: char,
    ) -> Result<(TokenKind, String), ParseError> {
        let mut expr = first_char.to_string();

        // Continue until we hit a delimiter
        while let Some(c) = self.peek() {
            // Stop at structural delimiters
            if matches!(c, '{' | '}' | '\n' | '#') {
                break;
            }

            // Include most characters in the code expression
            expr.push(c);
            self.advance();
        }

        let trimmed = expr.trim().to_string();
        Ok((TokenKind::Identifier(trimmed.clone()), trimmed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let source = "suite hash { }";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 5); // suite, hash, {, }, EOF
        assert_eq!(tokens[0].kind, TokenKind::Suite);
        assert!(matches!(tokens[1].kind, TokenKind::Identifier(_)));
        assert_eq!(tokens[2].kind, TokenKind::LBrace);
        assert_eq!(tokens[3].kind, TokenKind::RBrace);
        assert_eq!(tokens[4].kind, TokenKind::Eof);
    }

    #[test]
    fn test_string_literal() {
        let source = r#""hello world""#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();

        assert!(matches!(&tokens[0].kind, TokenKind::String(s) if s == "hello world"));
    }

    #[test]
    fn test_number_literal() {
        let source = "12345";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();

        assert!(matches!(tokens[0].kind, TokenKind::Number(12345)));
    }

    #[test]
    fn test_comment() {
        let source = "suite # this is a comment\nhash";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 3); // suite, hash, EOF
        assert_eq!(tokens[0].kind, TokenKind::Suite);
        assert!(matches!(tokens[1].kind, TokenKind::Identifier(_)));
    }

    #[test]
    fn test_file_ref() {
        let source = r#"@file("path.hex")"#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::FileRef);
        assert_eq!(tokens[1].kind, TokenKind::LParen);
        assert!(matches!(&tokens[2].kind, TokenKind::String(s) if s == "path.hex"));
        assert_eq!(tokens[3].kind, TokenKind::RParen);
    }

    #[test]
    fn test_brackets_and_comma() {
        let source = r#"["foo", "bar"]"#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::LBracket);
        assert!(matches!(&tokens[1].kind, TokenKind::String(s) if s == "foo"));
        assert_eq!(tokens[2].kind, TokenKind::Comma);
        assert!(matches!(&tokens[3].kind, TokenKind::String(s) if s == "bar"));
        assert_eq!(tokens[4].kind, TokenKind::RBracket);
    }

    #[test]
    fn test_duration_seconds() {
        let source = "30s";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();

        assert!(matches!(tokens[0].kind, TokenKind::Duration(30000))); // 30 seconds = 30000ms
    }

    #[test]
    fn test_duration_milliseconds() {
        let source = "500ms";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();

        assert!(matches!(tokens[0].kind, TokenKind::Duration(500)));
    }

    #[test]
    fn test_duration_minutes() {
        let source = "2m";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();

        assert!(matches!(tokens[0].kind, TokenKind::Duration(120000))); // 2 minutes = 120000ms
    }

    #[test]
    fn test_new_keywords() {
        let source = "declare init helpers import timeout tags skip validate before after each requires order compare baseline shape async";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::Declare);
        assert_eq!(tokens[1].kind, TokenKind::Init);
        assert_eq!(tokens[2].kind, TokenKind::Helpers);
        assert_eq!(tokens[3].kind, TokenKind::Import);
        assert_eq!(tokens[4].kind, TokenKind::Timeout);
        assert_eq!(tokens[5].kind, TokenKind::Tags);
        assert_eq!(tokens[6].kind, TokenKind::Skip);
        assert_eq!(tokens[7].kind, TokenKind::Validate);
        assert_eq!(tokens[8].kind, TokenKind::Before);
        assert_eq!(tokens[9].kind, TokenKind::After);
        assert_eq!(tokens[10].kind, TokenKind::Each);
        assert_eq!(tokens[11].kind, TokenKind::Requires);
        assert_eq!(tokens[12].kind, TokenKind::Order);
        assert_eq!(tokens[13].kind, TokenKind::Compare);
        assert_eq!(tokens[14].kind, TokenKind::Baseline);
        assert_eq!(tokens[15].kind, TokenKind::Shape);
        assert_eq!(tokens[16].kind, TokenKind::Async);
    }

    #[test]
    fn test_use_keyword() {
        let source = "use";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::Use);
    }

    #[test]
    fn test_double_colon() {
        let source = "std::constants";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();

        assert!(matches!(tokens[0].kind, TokenKind::Identifier(ref s) if s == "std"));
        assert_eq!(tokens[1].kind, TokenKind::DoubleColon);
        assert!(matches!(tokens[2].kind, TokenKind::Identifier(ref s) if s == "constants"));
    }

    #[test]
    fn test_use_std_statement() {
        let source = "use std::constants";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::Use);
        assert!(matches!(tokens[1].kind, TokenKind::Identifier(ref s) if s == "std"));
        assert_eq!(tokens[2].kind, TokenKind::DoubleColon);
        assert!(matches!(tokens[3].kind, TokenKind::Identifier(ref s) if s == "constants"));
        assert_eq!(tokens[4].kind, TokenKind::Eof);
    }

    #[test]
    fn test_colon_vs_double_colon() {
        let source = "foo: bar::baz";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();

        assert!(matches!(tokens[0].kind, TokenKind::Identifier(ref s) if s == "foo"));
        assert_eq!(tokens[1].kind, TokenKind::Colon);
        assert!(matches!(tokens[2].kind, TokenKind::Identifier(ref s) if s == "bar"));
        assert_eq!(tokens[3].kind, TokenKind::DoubleColon);
        assert!(matches!(tokens[4].kind, TokenKind::Identifier(ref s) if s == "baz"));
    }

    #[test]
    fn test_dot_token() {
        let source = "anvil.spawnAnvil";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();

        assert!(matches!(tokens[0].kind, TokenKind::Identifier(ref s) if s == "anvil"));
        assert_eq!(tokens[1].kind, TokenKind::Dot);
        assert!(matches!(tokens[2].kind, TokenKind::Identifier(ref s) if s == "spawnAnvil"));
    }

    #[test]
    fn test_namespaced_function_call() {
        let source = "anvil.spawnAnvil()";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();

        assert!(matches!(tokens[0].kind, TokenKind::Identifier(ref s) if s == "anvil"));
        assert_eq!(tokens[1].kind, TokenKind::Dot);
        assert!(matches!(tokens[2].kind, TokenKind::Identifier(ref s) if s == "spawnAnvil"));
        assert_eq!(tokens[3].kind, TokenKind::LParen);
        assert_eq!(tokens[4].kind, TokenKind::RParen);
    }
}
