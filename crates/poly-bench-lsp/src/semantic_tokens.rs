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
        SemanticTokenType::KEYWORD,   // 0
        SemanticTokenType::TYPE,      // 1 (language names)
        SemanticTokenType::FUNCTION,  // 2 (benchmark/fixture names)
        SemanticTokenType::VARIABLE,  // 3 (fixture references)
        SemanticTokenType::PROPERTY,  // 4 (description, iterations, etc.)
        SemanticTokenType::STRING,    // 5
        SemanticTokenType::NUMBER,    // 6
        SemanticTokenType::COMMENT,   // 7
        SemanticTokenType::NAMESPACE, // 8 (suite names)
    ]
});

/// Token modifiers
pub static TOKEN_MODIFIERS: Lazy<Vec<SemanticTokenModifier>> = Lazy::new(|| {
    vec![
        SemanticTokenModifier::DECLARATION, // 0
        SemanticTokenModifier::DEFINITION,  // 1
        SemanticTokenModifier::READONLY,    // 2
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
        // Handle use std::module statements
        for use_std in &ast.use_stds {
            // 'use' keyword
            add_token(
                doc,
                &mut tokens,
                &mut prev_line,
                &mut prev_char,
                &use_std.use_span,
                0, // KEYWORD
                0,
            );
            // 'std' identifier
            add_token(
                doc,
                &mut tokens,
                &mut prev_line,
                &mut prev_char,
                &use_std.std_span,
                8, // NAMESPACE
                0,
            );
            // module name
            add_token(
                doc,
                &mut tokens,
                &mut prev_line,
                &mut prev_char,
                &use_std.module_span,
                8, // NAMESPACE
                0,
            );
        }

        for suite in &ast.suites {
            // Suite keyword and name
            add_keyword_tokens(
                doc,
                &mut tokens,
                &mut prev_line,
                &mut prev_char,
                &suite.span,
            );

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
        "suite",
        "bench",
        "setup",
        "fixture",
        "hex",
        "description",
        "iterations",
        "warmup",
        "declare",
        "init",
        "helpers",
        "import",
        "timeout",
        "tags",
        "skip",
        "validate",
        "before",
        "after",
        "each",
        "requires",
        "order",
        "compare",
        "baseline",
        "shape",
        "async",
        "use",
        "globalSetup",
        // Auto-calibration keywords
        "mode",
        "targetTime",
        "minIterations",
        "maxIterations",
        // Performance keywords
        "sink",
        // Statistical keywords
        "outlierDetection",
        "cvThreshold",
        // Observability keywords (Phase 2B)
        "memory",
        "concurrency",
        // Charting keywords (used as parameters in charting calls)
        "title",
        "xlabel",
        "ylabel",
        "sortBy",
        "sortOrder",
        "timeUnit",
        "showTotalTime",
        "showLegend",
        "showGrid",
    ];

    // std and stdlib module names get NAMESPACE highlighting
    let namespace_keywords = ["std", "constants", "math", "charting", "anvil"];

    let lang_keywords = ["go", "ts", "typescript", "rust", "python"];

    let order_values = ["sequential", "parallel", "random", "auto", "fixed"];

    // Charting library methods get FUNCTION highlighting
    let charting_methods = [
        "drawBarChart",
        "drawLineChart",
        "drawPieChart",
        "drawScatterPlot",
        "drawHistogram",
        "drawHeatmap",
        "drawBoxPlot",
        "drawAreaChart",
    ];

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
                    &namespace_keywords,
                    &charting_methods,
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
                    &namespace_keywords,
                    &charting_methods,
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
                // Skip out-of-order tokens to prevent underflow
                if line < prev_line || (line == prev_line && char_pos < prev_char) {
                    // Skip this comment token
                } else {
                    // Find end of line
                    #[allow(clippy::all)]
                    let remaining: String =
                        doc.source[i..].chars().take_while(|&x| x != '\n').collect();

                    // Use saturating_sub as a safety net
                    let delta_line = line.saturating_sub(prev_line);
                    let delta_start = if delta_line == 0 {
                        char_pos.saturating_sub(prev_char)
                    } else {
                        char_pos
                    };

                    tokens.push(SemanticToken {
                        delta_line,
                        delta_start,
                        length: remaining.len() as u32,
                        token_type: 7, // COMMENT
                        token_modifiers_bitset: 0,
                    });
                    prev_line = line;
                    prev_char = char_pos;
                }
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
            &namespace_keywords,
            &charting_methods,
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
    namespace_keywords: &[&str],
    charting_methods: &[&str],
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
    } else if namespace_keywords.contains(&word) {
        Some(8) // NAMESPACE
    } else if charting_methods.contains(&word) {
        Some(2) // FUNCTION for charting methods
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
        // Skip out-of-order tokens to prevent underflow
        if line < *prev_line || (line == *prev_line && char_pos < *prev_char) {
            return;
        }

        // Use saturating_sub as a safety net
        let delta_line = line.saturating_sub(*prev_line);
        let delta_start = if delta_line == 0 {
            char_pos.saturating_sub(*prev_char)
        } else {
            char_pos
        };

        tokens.push(SemanticToken {
            delta_line,
            delta_start,
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
    span: &poly_bench_dsl::Span,
) {
    add_token(doc, tokens, prev_line, prev_char, span, 0, 0);
}

fn add_token(
    doc: &ParsedDocument,
    tokens: &mut Vec<SemanticToken>,
    prev_line: &mut u32,
    prev_char: &mut u32,
    span: &poly_bench_dsl::Span,
    token_type: u32,
    modifiers: u32,
) {
    // Validate span bounds to prevent overflow
    if span.end < span.start {
        return;
    }

    let pos = doc.offset_to_position(span.start);
    let length = (span.end - span.start) as u32;

    // Skip out-of-order tokens to prevent underflow
    // Tokens must be in document order for LSP semantic tokens protocol
    if pos.line < *prev_line || (pos.line == *prev_line && pos.character < *prev_char) {
        return;
    }

    // Use saturating_sub as a safety net even though we've validated above
    let delta_line = pos.line.saturating_sub(*prev_line);
    let delta_start = if delta_line == 0 {
        pos.character.saturating_sub(*prev_char)
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
