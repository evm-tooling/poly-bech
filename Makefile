# Poly-Bench Development Makefile
# ================================
# Quick commands for local development

.PHONY: help check build watch release clean install-tools reload cli run \
        cli-release init add install pb-build pb-run

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
	@echo "CLI (Debug):"
	@echo "  make cli      - Build the poly-bench CLI (debug)"
	@echo "  make run      - Run poly-bench CLI (use ARGS for arguments)"
	@echo "                  Example: make run ARGS='run examples/simple'"
	@echo ""
	@echo "CLI (Release - Production):"
	@echo "  make cli-release - Build optimized poly-bench CLI"
	@echo "  make init        - poly-bench init (NAME=<name> [DIR=<parent>])"
	@echo "  make add         - poly-bench add (requires PROJECT=<dir>)"
	@echo "  make install     - poly-bench install (requires PROJECT=<dir>)"
	@echo "  make pb-build    - poly-bench build (requires PROJECT=<dir>)"
	@echo "  make pb-run      - poly-bench run (requires PROJECT=<dir>)"
	@echo ""
	@echo "  Example workflow:"
	@echo "    make init NAME=my-bench"
	@echo "    make init NAME=demo DIR=examples"
	@echo "    make pb-build PROJECT=examples/demo"
	@echo "    make pb-run PROJECT=examples/demo"
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

# Run the poly-bench CLI with arguments (debug)
# Usage: make run ARGS='run examples/simple'
run: cli
	@./target/debug/poly-bench $(ARGS)

# ============================================================================
# Production CLI Commands (Release Build)
# ============================================================================
# These use the optimized release binary for actual benchmarking work.
# The release build is much faster and produces accurate timing results.
#
# Use PROJECT=<dir> to specify which project to operate on:
#   make pb-build PROJECT=my-project
#   make pb-run PROJECT=my-project
# ============================================================================

# Build the poly-bench CLI (release/production)
# Uses RUSTFLAGS to suppress warnings for clean user-facing output
cli-release:
	@printf "\033[36m[±]\033[0m Building poly-bench (release)...\r" && \
	RUSTFLAGS="-A warnings" cargo build --release --bin poly-bench --quiet && \
	printf "\033[32m[✓]\033[0m Built poly-bench (release)        \n"

# poly-bench init
# Usage: make init NAME=my-project
# Usage: make init NAME=my-project DIR=examples
# Usage: make init NAME=. DIR=examples/demo ARGS='-l go'
init: cli-release
ifndef NAME
	$(error Usage: make init NAME=<project-name> [DIR=<parent-dir>] [ARGS='...'])
endif
ifdef DIR
	@cd $(DIR) && $(CURDIR)/target/release/poly-bench init $(NAME) $(ARGS)
else
	@./target/release/poly-bench init $(NAME) $(ARGS)
endif

# poly-bench add
# Usage: make add PROJECT=my-project ARGS='--go github.com/pkg/errors@v0.9.1'
# Usage: make add PROJECT=my-project ARGS='--ts viem@^2.0.0'
add: cli-release
ifndef PROJECT
	$(error Usage: make add PROJECT=<project-dir> ARGS='--go pkg@version')
endif
	@cd $(PROJECT) && $(CURDIR)/target/release/poly-bench add $(ARGS)

# poly-bench install
# Usage: make install PROJECT=my-project
install: cli-release
ifndef PROJECT
	$(error Usage: make install PROJECT=<project-dir>)
endif
	@cd $(PROJECT) && $(CURDIR)/target/release/poly-bench install

# poly-bench build
# Usage: make pb-build PROJECT=my-project
pb-build: cli-release
ifndef PROJECT
	$(error Usage: make pb-build PROJECT=<project-dir>)
endif
	@cd $(PROJECT) && $(CURDIR)/target/release/poly-bench build $(ARGS)

# poly-bench run
# Usage: make pb-run PROJECT=my-project
# Usage: make pb-run PROJECT=my-project ARGS='--lang go'
pb-run: cli-release
ifndef PROJECT
	$(error Usage: make pb-run PROJECT=<project-dir>)
endif
	@cd $(PROJECT) && $(CURDIR)/target/release/poly-bench run $(ARGS)

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
