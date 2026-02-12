# Poly-Bench Development Makefile
# ================================
# Quick commands for local development

.PHONY: help check build watch release clean install-tools reload cli run

# Default target
help:
	@echo "Poly-Bench Development Commands"
	@echo "================================"
	@echo ""
	@echo "Development (LSP):"
	@echo "  make check    - Fast compile check (no binary)"
	@echo "  make build    - Debug build (~30s)"
	@echo "  make cb       - Check + Build combined"
	@echo "  make watch    - Auto-rebuild on changes"
	@echo ""
	@echo "CLI:"
	@echo "  make cli      - Build the poly-bench CLI (debug)"
	@echo "  make run      - Run poly-bench CLI (use ARGS for arguments)"
	@echo "                  Example: make run ARGS='run examples/simple'"
	@echo ""
	@echo "Release:"
	@echo "  make release  - Optimized release build (both binaries)"
	@echo ""
	@echo "Utilities:"
	@echo "  make clean    - Clean build artifacts"
	@echo "  make install-tools - Install cargo-watch"
	@echo ""
	@echo "After building LSP, reload VS Code:"
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

# Build the poly-bench CLI (debug)
cli:
	@echo "🔨 Building poly-bench CLI (debug)..."
	@cargo build --bin poly-bench
	@echo "✅ Done! Binary at: target/debug/poly-bench"

# Run the poly-bench CLI with arguments
# Usage: make run ARGS='run examples/simple'
run: cli
	@./target/debug/poly-bench $(ARGS)

# Watch for changes and auto-rebuild
watch:
	@echo "👀 Watching for changes..."
	@cargo watch -x "build --bin poly-bench-lsp"

# Optimized release build (both binaries)
release:
	@echo "🔨 Building poly-bench + poly-bench-lsp (release)..."
	@cargo build --release
	@echo "✅ Done!"
	@echo "  CLI: target/release/poly-bench"
	@echo "  LSP: target/release/poly-bench-lsp"

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

# ============================================================================
# Quick PR Workflow
# ============================================================================
# Creates a PR from staged files, squash-merges it, and pulls changes.
# Your editor stays on main the entire time.
#
# Usage: make pr TITLE="your-pr-title"
#
# Requirements:
# - Must be on main branch
# - Must have staged files (git add <files> first)
# - gh CLI must be installed and authenticated
# ============================================================================

.PHONY: pr

pr:
ifndef TITLE
	$(error Usage: make pr TITLE="your-pr-title")
endif
	@./scripts/quick-pr.sh "$(TITLE)"
