//! Semantic tokens provider for the LSP
//!
//! This module provides semantic token highlighting that works
//! alongside TextMate grammar for enhanced syntax highlighting.

use once_cell::sync::Lazy;
use tower_lsp::lsp_types::{
    SemanticToken, SemanticTokenModifier, SemanticTokenType, SemanticTokensLegend,
};

use super::document::ParsedDocument;

/// Token types used by the semantic token provider
pub static TOKEN_TYPES: Lazy<Vec<SemanticTokenType>> = Lazy::new(|| {
    vec![
        SemanticTokenType::KEYWORD,    // 0
        SemanticTokenType::TYPE,       // 1 (language names)
        SemanticTokenType::FUNCTION,   // 2 (benchmark/fixture names)
        SemanticTokenType::VARIABLE,   // 3 (fixture references)
        SemanticTokenType::PROPERTY,   // 4 (description, iterations, etc.)
        SemanticTokenType::STRING,     // 5
        SemanticTokenType::NUMBER,     // 6
        SemanticTokenType::COMMENT,    // 7
        SemanticTokenType::NAMESPACE,  // 8 (suite names)
    ]
});

/// Token modifiers
pub static TOKEN_MODIFIERS: Lazy<Vec<SemanticTokenModifier>> = Lazy::new(|| {
    vec![
        SemanticTokenModifier::DECLARATION,  // 0
        SemanticTokenModifier::DEFINITION,   // 1
        SemanticTokenModifier::READONLY,     // 2
    ]
});

/// The semantic tokens legend
pub static LEGEND: Lazy<SemanticTokensLegend> = Lazy::new(|| SemanticTokensLegend {
    token_types: TOKEN_TYPES.clone(),
    token_modifiers: TOKEN_MODIFIERS.clone(),
});

/// Get semantic tokens for a document
pub fn get_semantic_tokens(doc: &ParsedDocument) -> Vec<SemanticToken> {
    let mut tokens = Vec::new();
    let mut prev_line = 0u32;
    let mut prev_char = 0u32;

    // If we have a parsed AST, use it for semantic tokens
    if let Some(ref ast) = doc.ast {
        for suite in &ast.suites {
            // Suite keyword and name
            add_keyword_tokens(doc, &mut tokens, &mut prev_line, &mut prev_char, &suite.span);

            // Setup blocks
            for (lang, setup) in &suite.setups {
                add_token(
                    doc,
                    &mut tokens,
                    &mut prev_line,
                    &mut prev_char,
                    &setup.span,
                    1, // TYPE for language
                    0,
                );
            }

            // Fixtures
            for fixture in &suite.fixtures {
                add_token(
                    doc,
                    &mut tokens,
                    &mut prev_line,
                    &mut prev_char,
                    &fixture.span,
                    2, // FUNCTION for fixture name
                    1, // DEFINITION modifier
                );
            }

            // Benchmarks
            for benchmark in &suite.benchmarks {
                add_token(
                    doc,
                    &mut tokens,
                    &mut prev_line,
                    &mut prev_char,
                    &benchmark.span,
                    2, // FUNCTION for benchmark name
                    1, // DEFINITION modifier
                );
            }
        }
    }

    // Fall back to simple lexical tokenization if needed
    if tokens.is_empty() {
        tokens = lexical_tokens(doc);
    }

    tokens
}

/// Add tokens based on lexical analysis (for when AST is not available)
fn lexical_tokens(doc: &ParsedDocument) -> Vec<SemanticToken> {
    let mut tokens = Vec::new();
    let mut prev_line = 0u32;
    let mut prev_char = 0u32;

    let keywords = [
        "suite", "bench", "setup", "fixture", "hex", "description", "iterations",
        "warmup", "declare", "init", "helpers", "import", "timeout", "tags",
        "skip", "validate", "before", "after", "each", "requires", "order",
        "compare", "baseline", "shape", "async",
    ];

    let lang_keywords = ["go", "ts", "typescript", "rust", "python"];

    let order_values = ["sequential", "parallel", "random"];

    // Simple tokenizer
    let mut line = 0u32;
    let mut char_pos = 0u32;
    let mut word_start = None;
    let mut current_word = String::new();

    for (i, c) in doc.source.chars().enumerate() {
        if c == '\n' {
            // Check if we have a word to emit
            if let Some(start_char) = word_start {
                emit_word_token(
                    &current_word,
                    &keywords,
                    &lang_keywords,
                    &order_values,
                    &mut tokens,
                    &mut prev_line,
                    &mut prev_char,
                    line,
                    start_char,
                );
                current_word.clear();
                word_start = None;
            }

            line += 1;
            char_pos = 0;
            continue;
        }

        if c.is_alphanumeric() || c == '_' {
            if word_start.is_none() {
                word_start = Some(char_pos);
            }
            current_word.push(c);
        } else {
            if let Some(start_char) = word_start {
                emit_word_token(
                    &current_word,
                    &keywords,
                    &lang_keywords,
                    &order_values,
                    &mut tokens,
                    &mut prev_line,
                    &mut prev_char,
                    line,
                    start_char,
                );
                current_word.clear();
                word_start = None;
            }

            // Handle comments
            if c == '#' {
                // Find end of line
                let remaining: String = doc.source[i..].chars().take_while(|&x| x != '\n').collect();
                tokens.push(SemanticToken {
                    delta_line: line - prev_line,
                    delta_start: if line == prev_line {
                        char_pos - prev_char
                    } else {
                        char_pos
                    },
                    length: remaining.len() as u32,
                    token_type: 7, // COMMENT
                    token_modifiers_bitset: 0,
                });
                prev_line = line;
                prev_char = char_pos;
            }
        }

        char_pos += 1;
    }

    // Handle final word
    if let Some(start_char) = word_start {
        emit_word_token(
            &current_word,
            &keywords,
            &lang_keywords,
            &order_values,
            &mut tokens,
            &mut prev_line,
            &mut prev_char,
            line,
            start_char,
        );
    }

    tokens
}

fn emit_word_token(
    word: &str,
    keywords: &[&str],
    lang_keywords: &[&str],
    order_values: &[&str],
    tokens: &mut Vec<SemanticToken>,
    prev_line: &mut u32,
    prev_char: &mut u32,
    line: u32,
    char_pos: u32,
) {
    let token_type = if keywords.contains(&word) {
        Some(0) // KEYWORD
    } else if lang_keywords.contains(&word) {
        Some(1) // TYPE
    } else if order_values.contains(&word) {
        Some(4) // PROPERTY
    } else if word.parse::<u64>().is_ok() {
        Some(6) // NUMBER
    } else if word == "true" || word == "false" {
        Some(0) // KEYWORD
    } else {
        None
    };

    if let Some(tt) = token_type {
        tokens.push(SemanticToken {
            delta_line: line - *prev_line,
            delta_start: if line == *prev_line {
                char_pos - *prev_char
            } else {
                char_pos
            },
            length: word.len() as u32,
            token_type: tt,
            token_modifiers_bitset: 0,
        });
        *prev_line = line;
        *prev_char = char_pos;
    }
}

fn add_keyword_tokens(
    doc: &ParsedDocument,
    tokens: &mut Vec<SemanticToken>,
    prev_line: &mut u32,
    prev_char: &mut u32,
    span: &poly_bench::dsl::Span,
) {
    add_token(doc, tokens, prev_line, prev_char, span, 0, 0);
}

fn add_token(
    doc: &ParsedDocument,
    tokens: &mut Vec<SemanticToken>,
    prev_line: &mut u32,
    prev_char: &mut u32,
    span: &poly_bench::dsl::Span,
    token_type: u32,
    modifiers: u32,
) {
    let pos = doc.offset_to_position(span.start);
    let length = (span.end - span.start) as u32;

    let delta_line = pos.line - *prev_line;
    let delta_start = if delta_line == 0 {
        pos.character - *prev_char
    } else {
        pos.character
    };

    tokens.push(SemanticToken {
        delta_line,
        delta_start,
        length,
        token_type,
        token_modifiers_bitset: modifiers,
    });

    *prev_line = pos.line;
    *prev_char = pos.character;
}
