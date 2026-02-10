# Poly-bench examples Makefile
#
# Usage:
#   make setup     - Install Go and Node.js dependencies
#   make bench     - Run all benchmarks
#   make bench-abi - Run ABI benchmark only
#   make clean     - Remove generated files

POLY_BENCH := ../target/release/poly-bench
EXAMPLES_DIR := $(shell pwd)

.PHONY: all setup setup-go setup-ts bench bench-abi bench-hash bench-simple bench-minimal clean

all: setup bench

# Install all dependencies
setup: setup-go setup-ts

setup-go:
	@echo "==> Installing Go dependencies..."
	go mod tidy

setup-ts:
	@echo "==> Installing Node.js dependencies..."
	npm install

# Build poly-bench if needed
$(POLY_BENCH):
	@echo "==> Building poly-bench..."
	cd .. && cargo build --release

# Run all benchmarks
bench: $(POLY_BENCH)
	@echo "==> Running all benchmarks..."
	@echo ""
	@echo "--- ABI Benchmark ---"
	$(POLY_BENCH) run abi.bench --go-project $(EXAMPLES_DIR) --ts-project $(EXAMPLES_DIR)
	@echo ""
	@echo "--- Hash Benchmark ---"
	$(POLY_BENCH) run hash.bench --go-project $(EXAMPLES_DIR) --ts-project $(EXAMPLES_DIR)
	@echo ""
	@echo "--- Simple Benchmark ---"
	$(POLY_BENCH) run simple.bench --go-project $(EXAMPLES_DIR) --ts-project $(EXAMPLES_DIR)
	@echo ""
	@echo "--- Minimal Benchmark ---"
	$(POLY_BENCH) run minimal.bench --go-project $(EXAMPLES_DIR) --ts-project $(EXAMPLES_DIR)

# Individual benchmarks
bench-abi: $(POLY_BENCH)
	@echo "==> Running ABI benchmark..."
	$(POLY_BENCH) run abi.bench --go-project $(EXAMPLES_DIR) --ts-project $(EXAMPLES_DIR)

bench-hash: $(POLY_BENCH)
	@echo "==> Running Hash benchmark..."
	$(POLY_BENCH) run hash.bench --go-project $(EXAMPLES_DIR) --ts-project $(EXAMPLES_DIR)

bench-simple: $(POLY_BENCH)
	@echo "==> Running Simple benchmark..."
	$(POLY_BENCH) run simple.bench --go-project $(EXAMPLES_DIR) --ts-project $(EXAMPLES_DIR)

bench-minimal: $(POLY_BENCH)
	@echo "==> Running Minimal benchmark..."
	$(POLY_BENCH) run minimal.bench --go-project $(EXAMPLES_DIR) --ts-project $(EXAMPLES_DIR)

# Run with custom iterations
bench-fast: $(POLY_BENCH)
	@echo "==> Running quick benchmarks (100 iterations)..."
	$(POLY_BENCH) run abi.bench --go-project $(EXAMPLES_DIR) --ts-project $(EXAMPLES_DIR) --iterations 100

# Generate reports
report-json: $(POLY_BENCH)
	@mkdir -p reports
	$(POLY_BENCH) run abi.bench --go-project $(EXAMPLES_DIR) --ts-project $(EXAMPLES_DIR) --report json -o reports

report-markdown: $(POLY_BENCH)
	@mkdir -p reports
	$(POLY_BENCH) run abi.bench --go-project $(EXAMPLES_DIR) --ts-project $(EXAMPLES_DIR) --report markdown -o reports

# Clean generated/temporary files (keeps dependencies)
clean:
	rm -rf .polybench
	rm -rf reports

# Deep clean (removes dependencies too)
clean-all: clean
	rm -rf node_modules
	rm -f go.sum
