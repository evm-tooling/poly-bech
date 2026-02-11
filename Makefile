# Poly-Bench Development Makefile
# ================================
# Quick commands for local development

.PHONY: help check build watch release clean install-tools reload

# Default target
help:
	@echo "Poly-Bench Development Commands"
	@echo "================================"
	@echo ""
	@echo "Development:"
	@echo "  make check    - Fast compile check (no binary)"
	@echo "  make build    - Debug build (~30s)"
	@echo "  make cb       - Check + Build combined"
	@echo "  make watch    - Auto-rebuild on changes"
	@echo ""
	@echo "Release:"
	@echo "  make release  - Optimized release build"
	@echo ""
	@echo "Utilities:"
	@echo "  make clean    - Clean build artifacts"
	@echo "  make install-tools - Install cargo-watch"
	@echo ""
	@echo "After building, reload VS Code:"
	@echo "  Cmd+Shift+P → 'Developer: Reload Window'"

# Fast compile check (no binary output)
check:
	@echo "🔍 Checking for compile errors..."
	@cargo check --bin poly-bench-lsp
	@echo "✅ No errors!"

# Debug build (fast, unoptimized)
build:
	@echo "🔨 Building poly-bench-lsp (debug)..."
	@cargo build --bin poly-bench-lsp
	@echo "✅ Done! Binary at: target/debug/poly-bench-lsp"
	@echo ""
	@echo "Reload VS Code: Cmd+Shift+P → 'Developer: Reload Window'"

# Check then build (common workflow)
cb: check build

# Watch for changes and auto-rebuild
watch:
	@echo "👀 Watching for changes..."
	@cargo watch -x "build --bin poly-bench-lsp"

# Optimized release build
release:
	@echo "🔨 Building poly-bench-lsp (release)..."
	@cargo build --release --bin poly-bench-lsp
	@echo "✅ Done! Binary at: target/release/poly-bench-lsp"

# Clean build artifacts
clean:
	@echo "🧹 Cleaning..."
	@cargo clean
	@echo "✅ Clean!"

# Install development tools
install-tools:
	@echo "📦 Installing cargo-watch..."
	@cargo install cargo-watch
	@echo "✅ Done!"

# Build and show size comparison
size: build release
	@echo ""
	@echo "📊 Binary sizes:"
	@ls -lh target/debug/poly-bench-lsp | awk '{print "  Debug:   " $$5}'
	@ls -lh target/release/poly-bench-lsp | awk '{print "  Release: " $$5}'
