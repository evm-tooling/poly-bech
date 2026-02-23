# Contributing to poly-bench

Thanks for helping improve `poly-bench`! This project includes:

- A Rust CLI for running multi-language benchmarks
- A custom `.bench` DSL, parser, and compiler/runtime
- LSP support (`poly-bench lsp`) and editor tooling
- Documentation and examples

If you are planning a significant change, open an issue first so we can align on approach before implementation.

## Ground Rules

1. Keep pull requests focused and reviewable.
2. Include tests when changing behavior.
3. Update docs/examples when changing user-facing behavior.
4. Be respectful and collaborative in discussions and code review.

## Local Setup

### 1) Clone

```bash
git clone git@github.com:evm-tooling/poly-bech.git
cd poly-bech
```

### 2) Install prerequisites

- Rust toolchain (stable)
- Nightly Rust (used for formatting)
- Optional: Node.js 18+ (for VS Code extension and TypeScript-related workflows)
- Optional: Go 1.21+ (for running Go benchmarks in sample projects)

### 3) Quick sanity check

```bash
make check
```

This runs formatting checks, lints, and tests.

## Common Commands

Use either `make` or `just` (both are supported).

### Quality and tests

```bash
make fmt
make fmt-check
make lint
make test
make check
```

```bash
just dev fmt
just dev lint
just dev test
just dev test check
```

### Build and run

```bash
make cli            # debug binary
make cli-release    # optimized release binary
make run ARGS='run examples/simple'
```

```bash
just dev build debug
just prod build
```

### Hooks (recommended)

```bash
make install-hooks
```

## Project Areas

When changing behavior in one area, please verify related areas:

- `src/` - CLI, parser/compiler, runtime, and LSP implementation
- `poly-bench-lsp-v2/` - LSP-specific components
- `extensions/vscode/` - VS Code extension integration
- `docs/` - documentation site and examples

## Pull Request Process

1. Create a branch from `main`.
2. Make your change and keep commits focused.
3. Run checks locally (`make check` at minimum).
4. Add or update tests for behavior changes.
5. Update docs/examples for user-facing changes.
6. Open a PR targeting `main`.

### Commit message guidance

- Use imperative style (`Add ...`, `Fix ...`, `Refactor ...`)
- Keep the first line concise
- Reference issues when relevant (`Fixes #123`)

## Testing Expectations

Before opening a PR, validate the paths you touched:

- Core Rust changes: `make check`
- CLI behavior changes: run representative `poly-bench` commands
- LSP/editor changes: run the extension compile/lint flow if relevant
- Docs changes: verify docs render correctly

## Reporting Bugs or Requesting Features

Please use the issue templates in this repository:

- Bug report
- Feature request
- Documentation issue
- Question

Security reports should be submitted privately through GitHub Security Advisories.
