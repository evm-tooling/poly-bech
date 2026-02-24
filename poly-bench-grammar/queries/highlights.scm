; Syntax highlighting queries for poly-bench
; These queries map Tree-sitter nodes to highlight capture names

; ============================================================
; Keywords
; ============================================================

; Top-level keywords
"declare" @keyword
"suite" @keyword
"use" @keyword.import
"globalSetup" @keyword
"sameDataset" @property
"performance" @type
"memory" @type
"timeBased" @keyword
"iterationBased" @keyword

; Setup keywords
"setup" @keyword
"import" @keyword.import
"declare" @keyword
"init" @keyword
"helpers" @keyword
"async" @keyword.coroutine

; Benchmark keywords
"bench" @keyword
"fixture" @keyword

; Hook keywords
"before" @keyword
"after" @keyword
"each" @keyword
"skip" @keyword
"validate" @keyword

; Charting
"charting" @module

; ============================================================
; Property names
; ============================================================

(property_name) @property

(chart_param_name) @property

; ============================================================
; Identifiers in context
; ============================================================

; Suite name
(suite
  name: (identifier) @type.definition)

; Benchmark name
(benchmark
  name: (identifier) @function.definition)

; Fixture name
(fixture
  name: (identifier) @variable.definition)

; Fixture parameter
(fixture_param
  name: (identifier) @variable.parameter
  type: (identifier) @type)

; Module in use statement
(use_statement
  module: (identifier) @module)

; Chart function name
(chart_directive
  function: (chart_function_name) @function.builtin)

; Anvil call
(anvil_call
  "anvil" @module
  "spawnAnvil" @function.builtin)

; ============================================================
; Language tags
; ============================================================

(language_tag) @type

; ============================================================
; Literals
; ============================================================

(string) @string
(string_content) @string
(single_string_content) @string
(escape_sequence) @string.escape

(number) @number
(float) @number.float

(duration
  (number) @number
  (duration_unit) @keyword)

(boolean) @constant.builtin

; ============================================================
; Operators and punctuation
; ============================================================

"::" @punctuation.delimiter
":" @punctuation.delimiter
"." @punctuation.delimiter
"," @punctuation.delimiter

"{" @punctuation.bracket
"}" @punctuation.bracket
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket

; ============================================================
; Comments
; ============================================================

(comment) @comment

; ============================================================
; Special constructs
; ============================================================

; File reference
(file_ref
  "@file" @function.builtin)

; Embedded code (will be overridden by injections)
(embedded_code) @string.special

; Inline code
(inline_code) @string.special

; ============================================================
; Error nodes (for visual feedback)
; ============================================================

(ERROR) @error
