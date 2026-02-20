/**
 * Custom scanner for poly-bench Tree-sitter grammar
 *
 * This scanner handles embedded code blocks by performing proper brace-counting.
 * Unlike the TextMate grammar which uses fragile indentation-based matching,
 * this scanner correctly tracks nested braces to find the true end of code blocks.
 */

#include "tree_sitter/parser.h"
#include <wctype.h>
#include <stdbool.h>

enum TokenType {
  EMBEDDED_CODE,
  EMBEDDED_CODE_START,
};

void *tree_sitter_polybench_external_scanner_create(void) {
  return NULL;
}

void tree_sitter_polybench_external_scanner_destroy(void *payload) {
  (void)payload;
}

unsigned tree_sitter_polybench_external_scanner_serialize(void *payload, char *buffer) {
  (void)payload;
  (void)buffer;
  return 0;
}

void tree_sitter_polybench_external_scanner_deserialize(void *payload, const char *buffer, unsigned length) {
  (void)payload;
  (void)buffer;
  (void)length;
}

/**
 * Scan embedded code handling both {} and () delimiters
 * Returns when either } or ) is found at depth 0
 */
static bool scan_embedded_code(TSLexer *lexer) {
  int brace_depth = 0;
  int paren_depth = 0;
  bool in_string = false;
  char string_char = 0;
  bool in_line_comment = false;
  bool in_block_comment = false;
  bool has_content = false;

  // Mark the start position
  lexer->mark_end(lexer);

  while (lexer->lookahead != 0) {
    char c = (char)lexer->lookahead;

    // Check for closing delimiters at depth 0 FIRST
    if (!in_string && !in_line_comment && !in_block_comment) {
      if (c == '}' && brace_depth == 0) {
        lexer->result_symbol = EMBEDDED_CODE;
        return has_content;
      }
      if (c == ')' && paren_depth == 0) {
        lexer->result_symbol = EMBEDDED_CODE;
        return has_content;
      }
    }

    // Handle line comments
    if (!in_string && !in_block_comment && !in_line_comment) {
      if (c == '/') {
        lexer->advance(lexer, false);
        lexer->mark_end(lexer);
        has_content = true;
        if (lexer->lookahead == '/') {
          in_line_comment = true;
          lexer->advance(lexer, false);
          lexer->mark_end(lexer);
          continue;
        } else if (lexer->lookahead == '*') {
          in_block_comment = true;
          lexer->advance(lexer, false);
          lexer->mark_end(lexer);
          continue;
        }
        continue;
      }
    }

    // Handle end of line comment
    if (in_line_comment) {
      if (c == '\n' || c == '\r') {
        in_line_comment = false;
      }
      has_content = true;
      lexer->advance(lexer, false);
      lexer->mark_end(lexer);
      continue;
    }

    // Handle block comment content
    if (in_block_comment) {
      if (c == '*') {
        lexer->advance(lexer, false);
        lexer->mark_end(lexer);
        has_content = true;
        if (lexer->lookahead == '/') {
          in_block_comment = false;
          lexer->advance(lexer, false);
          lexer->mark_end(lexer);
        }
        continue;
      }
      has_content = true;
      lexer->advance(lexer, false);
      lexer->mark_end(lexer);
      continue;
    }

    // Handle string literals
    if (!in_string) {
      if (c == '"' || c == '\'' || c == '`') {
        in_string = true;
        string_char = c;
        has_content = true;
        lexer->advance(lexer, false);
        lexer->mark_end(lexer);
        continue;
      }
    } else {
      // Inside string
      if (c == '\\') {
        // Escape sequence - skip next character
        has_content = true;
        lexer->advance(lexer, false);
        lexer->mark_end(lexer);
        if (lexer->lookahead != 0) {
          lexer->advance(lexer, false);
          lexer->mark_end(lexer);
        }
        continue;
      }
      if (c == string_char) {
        in_string = false;
        string_char = 0;
      }
      has_content = true;
      lexer->advance(lexer, false);
      lexer->mark_end(lexer);
      continue;
    }

    // Handle braces (track depth)
    if (c == '{') {
      brace_depth++;
    } else if (c == '}') {
      brace_depth--;
    } else if (c == '(') {
      paren_depth++;
    } else if (c == ')') {
      paren_depth--;
    }

    // Advance and mark as content
    has_content = true;
    lexer->advance(lexer, false);
    lexer->mark_end(lexer);
  }

  // EOF - return what we have
  if (has_content) {
    lexer->result_symbol = EMBEDDED_CODE;
    return true;
  }

  return false;
}

/**
 * Main scanner entry point
 */
bool tree_sitter_polybench_external_scanner_scan(
  void *payload,
  TSLexer *lexer,
  const bool *valid_symbols
) {
  (void)payload;

  if (valid_symbols[EMBEDDED_CODE]) {
    return scan_embedded_code(lexer);
  }

  return false;
}
