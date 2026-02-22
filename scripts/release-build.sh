#!/bin/bash
# Optimized release build (slower to compile, faster to run)
# Use this for final testing and distribution

set -e

echo "🔨 Building poly-bench (release, includes LSP v2 command)..."
cargo build --release --bin poly-bench

echo "✅ Done! Binary at: target/release/poly-bench"
