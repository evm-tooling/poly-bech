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
  (#eq? @_lang "go")
)

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (declare_section
      (code_block
        (embedded_code) @injection.content)))
  (#eq? @_lang "go")

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (init_section
      (code_block
        (embedded_code) @injection.content)))
  (#eq? @_lang "go")

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (helpers_section
      (code_block
        (embedded_code) @injection.content)))
  (#eq? @_lang "go")
)

; Language implementation with Go
(language_implementation
  language: (language_tag) @_lang
  (code_block
    (embedded_code) @injection.content)
  (#eq? @_lang "go")
)

; Hook with Go
(hook_flat
  language: (language_tag) @_lang
  (code_block
    (embedded_code) @injection.content)
  (#eq? @_lang "go")
)
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
  (#any-of? @_lang "ts" "typescript")
    (#set! injection.language "typescript")
)

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (declare_section
      (code_block
        (embedded_code) @injection.content)))
  (#any-of? @_lang "ts" "typescript")
    (#set! injection.language "typescript")

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (init_section
      (code_block
        (embedded_code) @injection.content)))
  (#any-of? @_lang "ts" "typescript")
    (#set! injection.language "typescript")

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (helpers_section
      (code_block
        (embedded_code) @injection.content)))
  (#any-of? @_lang "ts" "typescript")
    (#set! injection.language "typescript")
)

; Language implementation with TypeScript
(language_implementation
  language: (language_tag) @_lang
  (code_block
    (embedded_code) @injection.content)
  (#any-of? @_lang "ts" "typescript")
    (#set! injection.language "typescript")
)

; Hook with TypeScript
(hook_flat
  language: (language_tag) @_lang
  (code_block
    (embedded_code) @injection.content)
  (#any-of? @_lang "ts" "typescript")
    (#set! injection.language "typescript")
)
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
  (#any-of? @_lang "rust" "rs")
)

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (declare_section
      (code_block
        (embedded_code) @injection.content)))
  (#any-of? @_lang "rust" "rs")

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (init_section
      (code_block
        (embedded_code) @injection.content)))
  (#any-of? @_lang "rust" "rs")

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (helpers_section
      (code_block
        (embedded_code) @injection.content)))
  (#any-of? @_lang "rust" "rs")
)

; Language implementation with Rust
(language_implementation
  language: (language_tag) @_lang
  (code_block
    (embedded_code) @injection.content)
  (#any-of? @_lang "rust" "rs")
)

; Hook with Rust
(hook_flat
  language: (language_tag) @_lang
  (code_block
    (embedded_code) @injection.content)
  (#any-of? @_lang "rust" "rs")
)
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
)

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (declare_section
      (code_block
        (embedded_code) @injection.content)))
  (#any-of? @_lang "python" "py")

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (init_section
      (code_block
        (embedded_code) @injection.content)))
  (#any-of? @_lang "python" "py")

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (helpers_section
      (code_block
        (embedded_code) @injection.content)))
  (#any-of? @_lang "python" "py")
)

; Language implementation with Python
(language_implementation
  language: (language_tag) @_lang
  (code_block
    (embedded_code) @injection.content)
  (#any-of? @_lang "python" "py")
)

; Hook with Python
(hook_flat
  language: (language_tag) @_lang
  (code_block
    (embedded_code) @injection.content)
  (#any-of? @_lang "python" "py")
)
; ============================================================
; CSharp injections
; ============================================================

; Setup block with CSharp language
(setup_block
  language: (language_tag) @_lang
  (setup_body
    (import_section
      (code_block
        (embedded_code) @injection.content)))
  (#any-of? @_lang "csharp" "cs")
    (#set! injection.language "c_sharp")
)

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (declare_section
      (code_block
        (embedded_code) @injection.content)))
  (#any-of? @_lang "csharp" "cs")
    (#set! injection.language "c_sharp")

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (init_section
      (code_block
        (embedded_code) @injection.content)))
  (#any-of? @_lang "csharp" "cs")
    (#set! injection.language "c_sharp")

(setup_block
  language: (language_tag) @_lang
  (setup_body
    (helpers_section
      (code_block
        (embedded_code) @injection.content)))
  (#any-of? @_lang "csharp" "cs")
    (#set! injection.language "c_sharp")
)

; Language implementation with CSharp
(language_implementation
  language: (language_tag) @_lang
  (code_block
    (embedded_code) @injection.content)
  (#any-of? @_lang "csharp" "cs")
    (#set! injection.language "c_sharp")
)

; Hook with CSharp
(hook_flat
  language: (language_tag) @_lang
  (code_block
    (embedded_code) @injection.content)
  (#any-of? @_lang "csharp" "cs")
    (#set! injection.language "c_sharp")
)
; ============================================================
; Inline code injections (single-line expressions)
; ============================================================

; Go inline code
(language_implementation
  language: (language_tag) @_lang
  (inline_code) @injection.content)
  (#eq? @_lang "go")
  (#set! injection.language "go")
)
; TypeScript inline code
(language_implementation
  language: (language_tag) @_lang
  (inline_code) @injection.content)
  (#any-of? @_lang "ts" "typescript")
  (#set! injection.language "typescript")
)
; Rust inline code
(language_implementation
  language: (language_tag) @_lang
  (inline_code) @injection.content)
  (#any-of? @_lang "rust" "rs")
  (#set! injection.language "rust")
)
; Python inline code
(language_implementation
  language: (language_tag) @_lang
  (inline_code) @injection.content)
  (#any-of? @_lang "python" "py")
  (#set! injection.language "python")
)
; CSharp inline code
(language_implementation
  language: (language_tag) @_lang
  (inline_code) @injection.content)
  (#any-of? @_lang "csharp" "cs")
  (#set! injection.language "c_sharp")
)