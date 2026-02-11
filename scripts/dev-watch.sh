#!/bin/bash
# Watch for changes and auto-rebuild (debug mode)
# Requires: cargo install cargo-watch

set -e

echo "👀 Watching for changes..."
echo "   Will rebuild on file changes."
echo "   Remember to reload VS Code window after each build."
echo ""

cargo watch -x "build --bin poly-bench-lsp"
