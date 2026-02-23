#!/bin/bash
# Fast debug build for local development
# Run this after making changes, then reload VS Code window

set -e

echo "🔨 Building poly-bench (debug, includes LSP command)..."
cargo build --bin poly-bench

echo "✅ Done! Binary at: target/debug/poly-bench"
echo ""
echo "To apply changes in VS Code:"
echo "  Cmd+Shift+P → 'Developer: Reload Window'"
