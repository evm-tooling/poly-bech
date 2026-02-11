#!/bin/bash
# Optimized release build (slower to compile, faster to run)
# Use this for final testing and distribution

set -e

echo "🔨 Building poly-bench-lsp (release)..."
cargo build --release --bin poly-bench-lsp

echo "✅ Done! Binary at: target/release/poly-bench-lsp"
