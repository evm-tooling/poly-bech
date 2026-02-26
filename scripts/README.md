# Language Codegen Scripts

## Overview

The `add-lang` script reduces boilerplate when adding new language runtimes. It uses `scripts/languages.toml` as the single source of truth and regenerates language-specific code across the codebase.

## Usage

```bash
./scripts/add-lang              # Regenerate all languages from languages.toml
./scripts/add-lang csharp       # Regenerate for a specific language only
python scripts/add-lang.py --dry-run   # Preview without making changes
```

## Source of Truth

- **`scripts/languages.toml`** – Defines all supported languages with:
  - `aliases` – CLI/grammar names (e.g. `["csharp", "cs"]`)
  - `rust_enum` – Rust enum variant (e.g. `CSharp`)
  - `token_kinds` – Parser token kinds (e.g. `["CSharp"]`)
  - `vscode_scope` – VS Code scope (e.g. `cs` for C#)
  - `tree_sitter_injection` – Tree-sitter scope (e.g. `c_sharp`)
  - `config_field` – Optional override for RuntimeConfig (e.g. `node_root` for TypeScript)
  - `embedded_lang` – Optional VS Code embedded language ID (e.g. `typescript` for ts)
  - `import_style` – Optional: `"paren"` for Go import blocks

## Regenerated Files

| Category | Files |
|----------|-------|
| DSL | `poly-bench-dsl/src/ast.rs`, `tokens.rs`, `parser.rs` |
| Syntax | `poly-bench-syntax/src/partial_ast.rs` |
| Config | `poly-bench-runtime-traits/src/config.rs` |
| Grammar | `poly-bench-grammar/grammar.js`, `queries/injections.scm` |
| Project | `poly-bench-project/src/lib.rs` |
| Executor | `poly-bench-executor/src/lib.rs`, `validation.rs`, `scheduler.rs` |
| VS Code | `extensions/vscode/syntaxes/polybench.tmLanguage.json`, `package.json` |
| Templates | C# template insertion in `manifest.rs`, `build.rs`, `deps.rs`, `templates.rs` |

## Validation Workflow

When adding a new language (e.g. after reverting C# and re-adding):

1. **Run script**: `./scripts/add-lang csharp`
2. **Create runtime crate**: `runtimes/runtimes-csharp/` with plugin implementation (manual)
3. **Register plugin**: Add `&CSHARP_PLUGIN` to `poly-bench-runtime/src/registry.rs`
4. **Add to workspace**: Add `"runtimes/runtimes-csharp"` to root `Cargo.toml` members
5. **Add runtime dep**: Add `runtimes-csharp = { path = "..." }` to `poly-bench-runtime/Cargo.toml`
6. **Regenerate grammar**: `cd poly-bench-grammar && npx tree-sitter generate`
7. **Build**: `cargo build`
8. **Test**: `poly-bench run --lang csharp examples/demo-basic/benchmarks/csharp_ftest.bench`

## Manual Steps (Not Codegened)

- Runtime crate implementation (`runtimes/runtimes-<lang>/`)
- Stdlib constants (e.g. `CSHARP_CONSTANTS`, `CSHARP_ANVIL`)
- Plugin registration in `poly-bench-runtime/src/registry.rs`
- Workspace and Cargo.toml entries
- CLI `main.rs` – language args, filters, project roots (Path 3 refactor optional)

## C# Templates

Template files in `scripts/templates/csharp/` are used when adding C# to a codebase that doesn't have it. The script inserts them into `manifest.rs`, `build.rs`, `deps.rs`, and `templates.rs` if the content is not already present.
