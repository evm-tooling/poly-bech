# LSP v2 Migration Guide

This document describes the completed migration to the Tree-sitter based LSP v2.

## Overview

LSP v2 is a complete rewrite of the language server that provides:

- **Error-tolerant parsing**: Always produces a syntax tree, even with incomplete code
- **Incremental parsing**: Fast re-parsing on edits using Tree-sitter
- **Full semantic token coverage**: All syntax elements are properly highlighted
- **Incremental formatting**: Small, targeted edits instead of whole-document replacement
- **Better diagnostics**: Syntax errors from Tree-sitter + semantic validation

## New Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    poly-bench-grammar                        │
│  Tree-sitter grammar with custom scanner for embedded code   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    poly-bench-syntax                         │
│  Error-tolerant CST-to-AST conversion with partial AST       │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    poly-bench-lsp-v2                         │
│  New LSP server with full semantic tokens and diagnostics    │
└─────────────────────────────────────────────────────────────┘
```

## LSP v2 Is Default

### Command Line

Use `lsp` (or `lsp-v2`, alias):

```bash
poly-bench lsp
```

The VS Code extension now launches v2 only. There is no feature flag for switching LSP versions.

## Known Differences

### Syntax Highlighting

LSP v2 provides more comprehensive highlighting:

- All keywords are highlighted (not just top-level)
- Property names have consistent styling
- Language tags are highlighted as types
- Comments are always highlighted correctly

### Formatting

LSP v2 formatting is more conservative:

- Only formats valid sections of the document
- Preserves user's incomplete code during typing
- Uses smaller edits for better undo behavior

### Diagnostics

LSP v2 provides more diagnostics:

- Syntax errors from Tree-sitter (always available)
- Semantic validation (empty suites, unused fixtures, etc.)
- Hints for missing setup blocks

## Troubleshooting

### LSP v2 Not Starting

1. Check that `poly-bench lsp` works from command line
2. Verify the binary is up to date: `poly-bench upgrade`
3. Check VSCode output panel for errors

### Highlighting Issues

1. Reload VSCode window (Cmd/Ctrl + Shift + P -> "Reload Window")
2. Check that semantic tokens are enabled in VSCode

### Formatting Not Working

1. LSP v2 skips formatting for documents with syntax errors
2. Fix any syntax errors first
3. Try saving the file to trigger format-on-save

## Reporting Issues

If you encounter issues with LSP v2:

1. Set `poly-bench.trace.server` to `verbose`
2. Reproduce the issue
3. Copy the output from "Poly-Bench LSP" output panel
4. Open an issue with the trace and a minimal reproduction

## Building the Grammar

If you need to rebuild the Tree-sitter grammar:

```bash
cd poly-bench-grammar
npm install
npm run generate
npm run build-wasm  # For WASM build
```

## Contributing

The LSP v2 codebase is organized as:

- `poly-bench-grammar/` - Tree-sitter grammar definition
- `poly-bench-syntax/` - Error-tolerant parsing layer
- `poly-bench-lsp-v2/` - LSP server implementation

See the plan document for detailed architecture information.
