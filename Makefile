# Poly-Bench Development Makefile
# ================================
# Quick commands for local development and CI

.PHONY: help check check-compile build build-debug watch release release-build release-both clean install-tools reload cli run \
        cli-release init add install pb-build pb-run fmt fmt-check lint test test-cover

# Default target
help:
	@echo "Poly-Bench Development Commands"
	@echo "================================"
	@echo ""
	@echo "CI / quality:"
	@echo "  make fmt        - Format code"
	@echo "  make fmt-check  - Check formatting (CI)"
	@echo "  make lint       - Clippy (CI)"
	@echo "  make test       - Run tests"
	@echo "  make test-cover - Run tests with coverage"
	@echo "  make check      - fmt-check + lint + test"
	@echo "  make build      - Release build of poly-bench (single binary)"
	@echo ""
	@echo "Development:"
	@echo "  make check-compile - Fast compile check (no binary)"
	@echo "  make cli        - Debug build of poly-bench"
	@echo "  make cb         - Check + debug build"
	@echo "  make watch      - Auto-rebuild on changes"
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
	@echo "  make release       - Tag, prerelease, open PR to production (VERSION=v0.0.1)"
	@echo "  make release-build - Build single release binary (poly-bench)"
	@echo "  make release-both  - Build poly-bench + poly-bench-lsp (legacy)"
	@echo ""
	@echo "Utilities:"
	@echo "  make clean    - Clean build artifacts"
	@echo "  make install-tools - Install cargo-watch"
	@echo ""
	@echo "After building, reload VS Code for LSP: Cmd+Shift+P → 'Developer: Reload Window'"

# ============================================================================
# CI / quality targets
# ============================================================================

# Format all workspace Rust (examples/ has no crates, so not formatted)
fmt:
	@cargo +nightly fmt --all

fmt-check:
	@cargo +nightly fmt --all -- --check

lint:
	@cargo clippy --all-targets -- -D warnings

test:
	@cargo test --all

test-cover:
	@cargo test --all --no-run 2>/dev/null || true
	@cargo test --all

# Full check: formatting + lint + test (CI)
check: fmt-check lint test
	@echo "==> All checks passed!"

# Release build: single binary (poly-bench includes LSP via 'poly-bench lsp')
build: cli-release

# Fast compile check (no binary output)
check-compile:
	@echo "🔍 Checking for compile errors..."
	@cargo check --bin poly-bench
	@echo "✅ No errors!"

# Debug build (fast, unoptimized)
build-debug:
	@echo "🔨 Building poly-bench (debug)..."
	@cargo build --bin poly-bench
	@echo "✅ Done! Binary at: target/debug/poly-bench"

# Check then build (common workflow)
cb: check-compile build-debug

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
	@cargo watch -x "build --bin poly-bench"

# Optimized release build: single binary (poly-bench; use 'poly-bench lsp' for LSP)
release-build: cli-release
	@echo "✅ Release binary: target/release/poly-bench"

# Build both binaries (legacy: poly-bench + poly-bench-lsp)
release-both:
	@echo "🔨 Building poly-bench + poly-bench-lsp (release)..."
	@cargo build --release
	@echo "✅ Done!"
	@echo "  CLI: target/release/poly-bench"
	@echo "  LSP: target/release/poly-bench-lsp"

# Release automation: bump versions, tag, prerelease, open PR to production
# Usage: make release VERSION=v0.0.1
# Requires: gh CLI authenticated, on main branch
release:
ifndef VERSION
	$(error VERSION is required. Usage: make release VERSION=v0.0.1)
endif
	@echo "==> Checking gh CLI authentication..."
	@gh auth status || (echo "Error: Please run 'gh auth login' first" && exit 1)
	@echo "==> Ensuring we're on main branch..."
	@git checkout main
	@git pull origin main
	@echo "==> Bumping versions to $(VERSION)..."
	@VER=$$(echo $(VERSION) | sed 's/^v//'); \
	sed -i.bak "s/^version = \".*\"/version = \"$$VER\"/" Cargo.toml && rm -f Cargo.toml.bak; \
	sed -i.bak "s/\"version\": \"[^\"]*\"/\"version\": \"$$VER\"/" extensions/vscode/package.json && rm -f extensions/vscode/package.json.bak; \
	git add Cargo.toml extensions/vscode/package.json && \
	git diff --staged --quiet && echo "==> No version changes (already at $(VERSION)?)" || (git commit -m "chore: release $(VERSION)" && git push origin main)
	@echo "==> Creating tag $(VERSION)..."
	@git tag -a $(VERSION) -m "Release $(VERSION)"
	@git push origin $(VERSION)
	@echo "==> Creating prerelease on GitHub..."
	@gh release create $(VERSION) \
		--title "$(VERSION)" \
		--generate-notes \
		--prerelease
	@echo "==> Creating PR from main to production..."
	@gh pr create \
		--base production \
		--head main \
		--title "Release $(VERSION)" \
		--body "## Release $(VERSION)$$( echo )$$( echo )This PR releases $(VERSION) to production.$$( echo )$$( echo )When merged, comment \`/release\` on this PR to promote the prerelease to the latest release."
	@echo "==> Done! Review and merge the PR to publish the release."

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
size: build-debug release-build
	@echo ""
	@echo "📊 Binary sizes:"
	@ls -lh target/debug/poly-bench 2>/dev/null | awk '{print "  Debug:   " $$5}' || true
	@ls -lh target/release/poly-bench 2>/dev/null | awk '{print "  Release: " $$5}' || true

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
