#!/bin/bash
# Full prod build: grammar, release CLI, extension, VSIX
# Run from repo root.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

WASM_PATH="extensions/vscode/tree-sitter/tree-sitter-polybench.wasm"

# 1. Grammar (always build for prod to ensure consistency)
echo "==> 1. Building tree-sitter grammar + WASM..."
make grammar-wasm

# 2. CLI (release)
echo "==> 2. Building poly-bench (release, includes LSP)..."
cargo build --release --bin poly-bench

# 3. Extension
echo "==> 3. Compiling VS Code extension..."
cd extensions/vscode
npm ci
npm run compile
cd "$ROOT"

# 4. VSIX
echo "==> 4. Packaging VSIX..."
cd extensions/vscode
npx --yes @vscode/vsce package
cd "$ROOT"

echo ""
echo "Prod build complete!"
echo "  CLI:     target/release/poly-bench"
echo "  VSIX:    extensions/vscode/*.vsix"
