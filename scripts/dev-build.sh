#!/bin/bash
# Full dev build: grammar, CLI, extension, VSIX
# Skips grammar rebuild when sources unchanged. Use --force to rebuild everything.
# Run from repo root.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

FORCE=false
[[ "${1:-}" == "--force" ]] && FORCE=true

WASM_PATH="extensions/vscode/tree-sitter/tree-sitter-polybench.wasm"

# 1. Grammar (if needed)
grammar_needed() {
  [[ "$FORCE" == true ]] && return 0
  [[ ! -f "$WASM_PATH" ]] && return 0
  for f in poly-bench-grammar/grammar.js poly-bench-grammar/src/scanner.c poly-bench-grammar/package.json; do
    if [[ -f "$f" && "$f" -nt "$WASM_PATH" ]]; then
      return 0
    fi
  done
  return 1
}

if grammar_needed; then
  echo "==> 1. Building tree-sitter grammar + WASM..."
  make grammar-wasm
else
  echo "==> 1. Grammar: skip (unchanged)"
fi

# 2. CLI (includes LSP)
echo "==> 2. Building poly-bench (debug, includes LSP)..."
cargo build --bin poly-bench

# 3. Extension
echo "==> 3. Compiling VS Code extension..."
cd extensions/vscode
npm install
npm run compile
cd "$ROOT"

# 4. VSIX
echo "==> 4. Packaging VSIX..."
cd extensions/vscode
npx --yes @vscode/vsce package
cd "$ROOT"

# 5. Install extension in VS Code
VSIX=$(ls -t extensions/vscode/*.vsix 2>/dev/null | head -1)
if [[ -n "$VSIX" ]]; then
  echo "==> 5. Installing extension in VS Code..."
  if command -v code >/dev/null 2>&1; then
    code --install-extension "$VSIX" --force
    echo "  Installed: $VSIX"
  else
    echo "  code CLI not found. Install manually: code --install-extension $VSIX"
  fi
fi

echo ""
echo "Dev build complete!"
echo "  CLI:     target/debug/poly-bench"
echo "  VSIX:    extensions/vscode/*.vsix"
echo ""
echo "Reload VS Code: Cmd+Shift+P -> Developer: Reload Window"
