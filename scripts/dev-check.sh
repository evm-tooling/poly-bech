#!/bin/bash
# Fastest way to check for compile errors (no binary output)
# Use this while coding to quickly catch errors

set -e

echo "🔍 Checking for compile errors..."
cargo check --bin poly-bench

echo "✅ No errors!"
