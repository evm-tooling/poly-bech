#!/bin/bash
# Fast debug build for local development
# Run this after making changes, then reload VS Code window

set -e

echo "🔨 Building poly-bench-lsp (debug)..."
cargo build --bin poly-bench-lsp

echo "✅ Done! Binary at: target/debug/poly-bench-lsp"
echo ""
echo "To apply changes in VS Code:"
echo "  Cmd+Shift+P → 'Developer: Reload Window'"
