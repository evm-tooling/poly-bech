; Injection queries for embedded languages in poly-bench
; These queries identify code blocks and inject the appropriate language parser

; ============================================================
; Go injections
; ============================================================

; Setup block with Go language
(setup_block
  language: (language_tag) @_lang
  (setup_body
    (import_section
      (paren_code_block
        (embedded_code) @injection.content)))
  (#eq? @_lang "go"))

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (declare_section
      (code_block
        (embedded_code) @injection.content)))
  (#eq? @_lang "go"))

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (init_section
      (code_block
        (embedded_code) @injection.content)))
  (#eq? @_lang "go"))

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (helpers_section
      (code_block
        (embedded_code) @injection.content)))
  (#eq? @_lang "go"))

; Language implementation with Go
(language_implementation
  language: (language_tag) @_lang
  (code_block
    (embedded_code) @injection.content)
  (#eq? @_lang "go"))

; Hook with Go
(hook_flat
  language: (language_tag) @_lang
  (code_block
    (embedded_code) @injection.content)
  (#eq? @_lang "go"))

; Set injection language for Go blocks
((embedded_code) @injection.content
  (#set! injection.language "go")
  (#match? @injection.content ""))

; ============================================================
; TypeScript injections
; ============================================================

; Setup block with TypeScript language
(setup_block
  language: (language_tag) @_lang
  (setup_body
    (import_section
      (code_block
        (embedded_code) @injection.content)))
  (#any-of? @_lang "ts" "typescript"))

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (declare_section
      (code_block
        (embedded_code) @injection.content)))
  (#any-of? @_lang "ts" "typescript"))

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (init_section
      (code_block
        (embedded_code) @injection.content)))
  (#any-of? @_lang "ts" "typescript"))

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (helpers_section
      (code_block
        (embedded_code) @injection.content)))
  (#any-of? @_lang "ts" "typescript"))

; Language implementation with TypeScript
(language_implementation
  language: (language_tag) @_lang
  (code_block
    (embedded_code) @injection.content)
  (#any-of? @_lang "ts" "typescript"))

; Hook with TypeScript
(hook_flat
  language: (language_tag) @_lang
  (code_block
    (embedded_code) @injection.content)
  (#any-of? @_lang "ts" "typescript"))

; ============================================================
; Rust injections
; ============================================================

; Setup block with Rust language
(setup_block
  language: (language_tag) @_lang
  (setup_body
    (import_section
      (code_block
        (embedded_code) @injection.content)))
  (#eq? @_lang "rust"))

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (declare_section
      (code_block
        (embedded_code) @injection.content)))
  (#eq? @_lang "rust"))

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (init_section
      (code_block
        (embedded_code) @injection.content)))
  (#eq? @_lang "rust"))

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (helpers_section
      (code_block
        (embedded_code) @injection.content)))
  (#eq? @_lang "rust"))

; Language implementation with Rust
(language_implementation
  language: (language_tag) @_lang
  (code_block
    (embedded_code) @injection.content)
  (#eq? @_lang "rust"))

; Hook with Rust
(hook_flat
  language: (language_tag) @_lang
  (code_block
    (embedded_code) @injection.content)
  (#eq? @_lang "rust"))

; ============================================================
; Python injections
; ============================================================

; Setup block with Python language
(setup_block
  language: (language_tag) @_lang
  (setup_body
    (import_section
      (code_block
        (embedded_code) @injection.content)))
  (#any-of? @_lang "python" "py")
  (#set! injection.language "python"))

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (declare_section
      (code_block
        (embedded_code) @injection.content)))
  (#any-of? @_lang "python" "py")
  (#set! injection.language "python"))

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (init_section
      (code_block
        (embedded_code) @injection.content)))
  (#any-of? @_lang "python" "py")
  (#set! injection.language "python"))

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (helpers_section
      (code_block
        (embedded_code) @injection.content)))
  (#any-of? @_lang "python" "py")
  (#set! injection.language "python"))

; Language implementation with Python
(language_implementation
  language: (language_tag) @_lang
  (code_block
    (embedded_code) @injection.content)
  (#any-of? @_lang "python" "py"))

; Hook with Python
(hook_flat
  language: (language_tag) @_lang
  (code_block
    (embedded_code) @injection.content)
  (#any-of? @_lang "python" "py"))

; ============================================================
; Inline code injections (single-line expressions)
; ============================================================

; Go inline code
(language_implementation
  language: (language_tag) @_lang
  (inline_code) @injection.content
  (#eq? @_lang "go")
  (#set! injection.language "go"))

; TypeScript inline code
(language_implementation
  language: (language_tag) @_lang
  (inline_code) @injection.content
  (#any-of? @_lang "ts" "typescript")
  (#set! injection.language "typescript"))

; Rust inline code
(language_implementation
  language: (language_tag) @_lang
  (inline_code) @injection.content
  (#eq? @_lang "rust")
  (#set! injection.language "rust"))

; Python inline code
(language_implementation
  language: (language_tag) @_lang
  (inline_code) @injection.content
  (#any-of? @_lang "python" "py")
  (#set! injection.language "python"))
