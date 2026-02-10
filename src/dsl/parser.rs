//! Parser for the poly-bench DSL
//!
//! Parses a stream of tokens into an AST.

use crate::dsl::ast::*;
use crate::dsl::error::{ParseError, NamedSource};
use crate::dsl::lexer::Lexer;
use crate::dsl::tokens::{Token, TokenKind};
use miette::{Report, Result};
use std::collections::HashMap;

/// Parser state
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    filename: String,
    source: String,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, filename: String, source: String) -> Self {
        Self {
            tokens,
            current: 0,
            filename,
            source,
        }
    }

    /// Parse the entire file
    pub fn parse_file(&mut self) -> Result<File> {
        let mut suites = Vec::new();

        while !self.is_at_end() {
            let suite = self.parse_suite()?;
            suites.push(suite);
        }

        Ok(File::new(suites))
    }

    /// Parse a suite definition
    fn parse_suite(&mut self) -> Result<Suite> {
        // Expect 'suite' keyword
        self.expect_keyword(TokenKind::Suite)?;

        // Get suite name
        let name_token = self.expect_identifier()?;
        let name = match &name_token.kind {
            TokenKind::Identifier(s) => s.clone(),
            _ => unreachable!(),
        };

        // Expect opening brace
        self.expect(TokenKind::LBrace)?;

        let mut suite = Suite::new(name, name_token.span.clone());

        // Parse suite body
        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            self.parse_suite_item(&mut suite)?;
        }

        // Expect closing brace
        self.expect(TokenKind::RBrace)?;

        Ok(suite)
    }

    /// Parse a single item within a suite
    fn parse_suite_item(&mut self, suite: &mut Suite) -> Result<()> {
        let token = self.peek().clone();

        match &token.kind {
            TokenKind::Description => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_string()?;
                suite.description = Some(value);
            }
            TokenKind::Iterations => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_number()?;
                suite.iterations = Some(value);
            }
            TokenKind::Warmup => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_number()?;
                suite.warmup = Some(value);
            }
            TokenKind::Setup => {
                let (lang, code) = self.parse_setup()?;
                suite.setups.insert(lang, code);
            }
            TokenKind::Fixture => {
                let fixture = self.parse_fixture()?;
                suite.fixtures.push(fixture);
            }
            TokenKind::Bench => {
                let benchmark = self.parse_benchmark()?;
                suite.benchmarks.push(benchmark);
            }
            TokenKind::Identifier(_) => {
                // Could be a property or language implementation
                // Check if it's a known property
                let ident = match &token.kind {
                    TokenKind::Identifier(s) => s.clone(),
                    _ => unreachable!(),
                };
                
                return Err(self.make_error(ParseError::InvalidProperty {
                    name: ident,
                    span: token.span.clone(),
                }));
            }
            _ => {
                return Err(self.make_error(ParseError::ExpectedToken {
                    expected: "suite item (setup, fixture, bench, or property)".to_string(),
                    found: format!("{:?}", token.kind),
                    span: token.span.clone(),
                }));
            }
        }

        Ok(())
    }

    /// Parse a setup block
    fn parse_setup(&mut self) -> Result<(Lang, CodeBlock)> {
        self.expect_keyword(TokenKind::Setup)?;

        let lang = self.expect_lang()?;
        let code = self.parse_code_block()?;

        Ok((lang, code))
    }

    /// Parse a fixture definition
    fn parse_fixture(&mut self) -> Result<Fixture> {
        self.expect_keyword(TokenKind::Fixture)?;

        let name_token = self.expect_identifier()?;
        let name = match &name_token.kind {
            TokenKind::Identifier(s) => s.clone(),
            _ => unreachable!(),
        };

        self.expect(TokenKind::LBrace)?;

        let mut fixture = Fixture::new(name, name_token.span.clone());

        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            self.parse_fixture_item(&mut fixture)?;
        }

        self.expect(TokenKind::RBrace)?;

        Ok(fixture)
    }

    /// Parse a single item within a fixture
    fn parse_fixture_item(&mut self, fixture: &mut Fixture) -> Result<()> {
        let token = self.peek().clone();

        match &token.kind {
            TokenKind::Description => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_string()?;
                fixture.description = Some(value);
            }
            TokenKind::Hex => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                
                // Check for @file reference or string literal
                if self.check(TokenKind::FileRef) {
                    self.advance();
                    self.expect(TokenKind::LParen)?;
                    let path = self.expect_string()?;
                    self.expect(TokenKind::RParen)?;
                    fixture.hex_file = Some(path);
                } else {
                    let value = self.expect_string()?;
                    fixture.hex_data = Some(value);
                }
            }
            // Language-specific implementation
            TokenKind::Go | TokenKind::Ts | TokenKind::TypeScript | TokenKind::Rust | TokenKind::Python => {
                let lang = self.expect_lang()?;
                self.expect(TokenKind::Colon)?;
                let code = self.parse_inline_or_block_code()?;
                fixture.implementations.insert(lang, code);
            }
            TokenKind::Identifier(s) => {
                // Check if it's a language
                if let Some(lang) = Lang::from_str(s) {
                    self.advance();
                    self.expect(TokenKind::Colon)?;
                    let code = self.parse_inline_or_block_code()?;
                    fixture.implementations.insert(lang, code);
                } else {
                    return Err(self.make_error(ParseError::InvalidProperty {
                        name: s.clone(),
                        span: token.span.clone(),
                    }));
                }
            }
            _ => {
                return Err(self.make_error(ParseError::ExpectedToken {
                    expected: "fixture property (hex, description) or language".to_string(),
                    found: format!("{:?}", token.kind),
                    span: token.span.clone(),
                }));
            }
        }

        Ok(())
    }

    /// Parse a benchmark definition
    fn parse_benchmark(&mut self) -> Result<Benchmark> {
        self.expect_keyword(TokenKind::Bench)?;

        let name_token = self.expect_identifier()?;
        let name = match &name_token.kind {
            TokenKind::Identifier(s) => s.clone(),
            _ => unreachable!(),
        };

        self.expect(TokenKind::LBrace)?;

        let mut benchmark = Benchmark::new(name, name_token.span.clone());

        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            self.parse_benchmark_item(&mut benchmark)?;
        }

        self.expect(TokenKind::RBrace)?;

        Ok(benchmark)
    }

    /// Parse a single item within a benchmark
    fn parse_benchmark_item(&mut self, benchmark: &mut Benchmark) -> Result<()> {
        let token = self.peek().clone();

        match &token.kind {
            TokenKind::Description => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_string()?;
                benchmark.description = Some(value);
            }
            TokenKind::Iterations => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_number()?;
                benchmark.iterations = Some(value);
            }
            // Language-specific implementation
            TokenKind::Go | TokenKind::Ts | TokenKind::TypeScript | TokenKind::Rust | TokenKind::Python => {
                let lang = self.expect_lang()?;
                self.expect(TokenKind::Colon)?;
                let code = self.parse_inline_or_block_code()?;
                benchmark.implementations.insert(lang, code);
            }
            TokenKind::Identifier(s) => {
                // Check if it's a language
                if let Some(lang) = Lang::from_str(s) {
                    self.advance();
                    self.expect(TokenKind::Colon)?;
                    let code = self.parse_inline_or_block_code()?;
                    benchmark.implementations.insert(lang, code);
                } else {
                    return Err(self.make_error(ParseError::InvalidProperty {
                        name: s.clone(),
                        span: token.span.clone(),
                    }));
                }
            }
            _ => {
                return Err(self.make_error(ParseError::ExpectedToken {
                    expected: "benchmark property (iterations, description) or language implementation".to_string(),
                    found: format!("{:?}", token.kind),
                    span: token.span.clone(),
                }));
            }
        }

        Ok(())
    }

    /// Parse a code block (braces required)
    fn parse_code_block(&mut self) -> Result<CodeBlock> {
        let open_brace = self.expect(TokenKind::LBrace)?;
        let start_pos = self.current;
        
        // Find matching closing brace with brace counting
        let mut depth = 1;
        let mut code_tokens = Vec::new();
        
        while depth > 0 && !self.is_at_end() {
            let token = self.advance().clone();
            match token.kind {
                TokenKind::LBrace => depth += 1,
                TokenKind::RBrace => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                _ => {}
            }
            if depth > 0 {
                code_tokens.push(token);
            }
        }

        if depth > 0 {
            return Err(self.make_error(ParseError::UnclosedBrace {
                span: open_brace.span.clone(),
            }));
        }

        // Reconstruct code from tokens (this is a simplification - in a real impl
        // we'd preserve the original source)
        let code = self.reconstruct_code(&code_tokens);

        Ok(CodeBlock::new(code, true, open_brace.span))
    }

    /// Parse inline code (single line) or block code
    fn parse_inline_or_block_code(&mut self) -> Result<CodeBlock> {
        if self.check(TokenKind::LBrace) {
            self.parse_code_block()
        } else {
            // Single line - collect tokens until we hit a newline-ish boundary
            // For simplicity, we'll collect until we see a language keyword, 
            // closing brace, or known property keyword
            let start_token = self.peek().clone();
            let mut code_tokens = Vec::new();

            while !self.is_at_end() {
                let token = self.peek();
                
                // Stop conditions for inline code
                if matches!(token.kind, 
                    TokenKind::RBrace |
                    TokenKind::Go |
                    TokenKind::Ts |
                    TokenKind::TypeScript |
                    TokenKind::Rust |
                    TokenKind::Python |
                    TokenKind::Description |
                    TokenKind::Iterations |
                    TokenKind::Warmup |
                    TokenKind::Hex |
                    TokenKind::Bench |
                    TokenKind::Setup |
                    TokenKind::Fixture
                ) {
                    break;
                }

                // Check if identifier might be a language
                if let TokenKind::Identifier(s) = &token.kind {
                    if Lang::from_str(s).is_some() {
                        break;
                    }
                }

                code_tokens.push(self.advance().clone());
            }

            let code = self.reconstruct_code(&code_tokens);

            Ok(CodeBlock::new(code.trim().to_string(), false, start_token.span))
        }
    }

    /// Reconstruct code string from tokens by extracting from original source
    fn reconstruct_code(&self, tokens: &[Token]) -> String {
        if tokens.is_empty() {
            return String::new();
        }

        // Find the span covering all tokens
        let first_span = &tokens.first().unwrap().span;
        let last_span = &tokens.last().unwrap().span;

        // Extract the original source text between these spans
        let start = first_span.start;
        let end = last_span.end;

        if end <= self.source.len() {
            self.source[start..end].to_string()
        } else {
            // Fallback to token-based reconstruction if spans are invalid
            tokens.iter()
                .map(|t| t.lexeme.as_str())
                .collect::<Vec<_>>()
                .join(" ")
        }
    }

    // ========== Helper methods ==========

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or_else(|| {
            self.tokens.last().expect("tokens should have at least EOF")
        })
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current.saturating_sub(1)]
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Eof)
    }

    fn check(&self, kind: TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(&kind)
    }

    fn expect(&mut self, expected: TokenKind) -> Result<Token> {
        if self.check(expected.clone()) {
            Ok(self.advance().clone())
        } else {
            let token = self.peek().clone();
            Err(self.make_error(ParseError::ExpectedToken {
                expected: format!("{:?}", expected),
                found: format!("{:?}", token.kind),
                span: token.span,
            }))
        }
    }

    fn expect_keyword(&mut self, expected: TokenKind) -> Result<Token> {
        self.expect(expected)
    }

    fn expect_identifier(&mut self) -> Result<Token> {
        let token = self.peek().clone();
        if matches!(token.kind, TokenKind::Identifier(_)) {
            Ok(self.advance().clone())
        } else {
            Err(self.make_error(ParseError::ExpectedIdentifier {
                span: token.span,
            }))
        }
    }

    fn expect_string(&mut self) -> Result<String> {
        let token = self.peek().clone();
        if let TokenKind::String(s) = &token.kind {
            let s = s.clone();
            self.advance();
            Ok(s)
        } else {
            Err(self.make_error(ParseError::ExpectedToken {
                expected: "string".to_string(),
                found: format!("{:?}", token.kind),
                span: token.span,
            }))
        }
    }

    fn expect_number(&mut self) -> Result<u64> {
        let token = self.peek().clone();
        if let TokenKind::Number(n) = token.kind {
            self.advance();
            Ok(n)
        } else {
            Err(self.make_error(ParseError::ExpectedToken {
                expected: "number".to_string(),
                found: format!("{:?}", token.kind),
                span: token.span,
            }))
        }
    }

    fn expect_lang(&mut self) -> Result<Lang> {
        let token = self.peek().clone();
        let lang = match &token.kind {
            TokenKind::Go => Some(Lang::Go),
            TokenKind::Ts | TokenKind::TypeScript => Some(Lang::TypeScript),
            TokenKind::Rust => Some(Lang::Rust),
            TokenKind::Python => Some(Lang::Python),
            TokenKind::Identifier(s) => Lang::from_str(s),
            _ => None,
        };

        match lang {
            Some(l) => {
                self.advance();
                Ok(l)
            }
            None => {
                Err(self.make_error(ParseError::UnknownLang {
                    lang: token.lexeme.clone(),
                    span: token.span,
                }))
            }
        }
    }

    fn make_error(&self, error: ParseError) -> Report {
        Report::new(error)
            .with_source_code(NamedSource::new(
                self.filename.clone(),
                self.source.clone(),
            ))
    }
}

/// Parse source code into an AST
pub fn parse(source: &str, filename: &str) -> Result<File> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().map_err(|e| {
        Report::new(e).with_source_code(NamedSource::new(filename, source.to_string()))
    })?;

    let mut parser = Parser::new(tokens, filename.to_string(), source.to_string());
    parser.parse_file()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_suite() {
        let source = r#"
suite hash {
    description: "Hash benchmarks"
    iterations: 5000
    
    bench keccak256 {
        go: hash.Keccak256(data)
        ts: keccak256(data)
    }
}
"#;
        let result = parse(source, "test.bench");
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());
        
        let file = result.unwrap();
        assert_eq!(file.suites.len(), 1);
        
        let suite = &file.suites[0];
        assert_eq!(suite.name, "hash");
        assert_eq!(suite.description, Some("Hash benchmarks".to_string()));
        assert_eq!(suite.iterations, Some(5000));
        assert_eq!(suite.benchmarks.len(), 1);
        
        let bench = &suite.benchmarks[0];
        assert_eq!(bench.name, "keccak256");
        assert!(bench.implementations.contains_key(&Lang::Go));
        assert!(bench.implementations.contains_key(&Lang::TypeScript));
    }

    #[test]
    fn test_parse_fixture() {
        let source = r#"
suite test {
    fixture data {
        hex: "deadbeef"
    }
    
    bench foo {
        go: test(data)
    }
}
"#;
        let result = parse(source, "test.bench");
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());
        
        let file = result.unwrap();
        let suite = &file.suites[0];
        assert_eq!(suite.fixtures.len(), 1);
        assert_eq!(suite.fixtures[0].name, "data");
        assert_eq!(suite.fixtures[0].hex_data, Some("deadbeef".to_string()));
    }

    #[test]
    fn test_parse_setup() {
        let source = r#"
suite test {
    setup go {
        import "testing"
    }
    
    setup ts {
        import { foo } from 'bar'
    }
    
    bench foo {
        go: test()
    }
}
"#;
        let result = parse(source, "test.bench");
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());
        
        let file = result.unwrap();
        let suite = &file.suites[0];
        assert!(suite.setups.contains_key(&Lang::Go));
        assert!(suite.setups.contains_key(&Lang::TypeScript));
    }
}
