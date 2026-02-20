; Locals queries for poly-bench
; These queries define scopes and variable definitions/references

; ============================================================
; Scopes
; ============================================================

; File is the root scope
(source_file) @local.scope

; Suite creates a new scope
(suite) @local.scope

; Setup block creates a scope
(setup_block) @local.scope

; Benchmark creates a scope
(benchmark) @local.scope

; Fixture creates a scope
(fixture) @local.scope

; Code blocks create scopes (for embedded language)
(code_block) @local.scope

; ============================================================
; Definitions
; ============================================================

; Suite name is a definition
(suite
  name: (identifier) @local.definition.type)

; Benchmark name is a definition
(benchmark
  name: (identifier) @local.definition.function)

; Fixture name is a definition
(fixture
  name: (identifier) @local.definition.variable)

; Fixture parameters are definitions
(fixture_param
  name: (identifier) @local.definition.parameter)

; ============================================================
; References
; ============================================================

; Identifiers in code are potential references
; (The actual resolution happens in the embedded language)

; Baseline reference to a language
(property
  name: (property_name) @_name
  value: (string) @local.reference
  (#eq? @_name "baseline"))

; Requires array references languages
(property
  name: (property_name) @_name
  value: (string_array
    (string) @local.reference)
  (#eq? @_name "requires"))
