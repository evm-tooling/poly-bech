#!/bin/bash
# Bootstrap poly-bench dev environment: grammar, WASM, cargo build, extension compile
# Run from repo root. Works with or without Docker.

set -e

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "==> 1. Generating tree-sitter grammar..."
cd poly-bench-grammar && npm install && npm run generate
cd "$ROOT"

echo "==> 2. Building tree-sitter WASM..."
cd poly-bench-grammar && npm run build-wasm
cd "$ROOT"
mkdir -p extensions/vscode/tree-sitter
cp poly-bench-grammar/tree-sitter-polybench.wasm extensions/vscode/tree-sitter/

echo "==> 3. Building poly-bench (includes LSP)..."
cargo build --bin poly-bench

echo "==> 4. Compiling VS Code extension..."
cd extensions/vscode && npm install && npm run compile
cd "$ROOT"

echo ""
echo "Bootstrap complete!"
echo "  - poly-bench: target/debug/poly-bench"
echo "  - Extension: extensions/vscode/out/"
echo ""
echo "To run interactively in Docker: docker compose run dev bash"
echo "To reload VS Code after changes: Cmd+Shift+P -> Developer: Reload Window"
