# poly-bench

A high-performance multi-language benchmarking framework with a custom DSL and full LSP support.

## Overview

poly-bench lets you define benchmarks once and run them across multiple languages (Go, TypeScript, Rust) with unified output and comparison. It's designed for:

- **Cross-language library comparisons** — Compare your Go library against its TypeScript equivalent
- **Performance regression testing** — Track performance across languages over time
- **Fair benchmarking** — Same data, same iterations, unified measurement

## Quick Start

```bash
# Install
curl -L https://raw.githubusercontent.com/evm-tooling/poly-bench/main/scripts/install.sh | bash

# Initialize a project
poly-bench init my-benchmarks

# Run benchmarks
poly-bench run
```

## Language Features

### Suite Structure

```
use std::charting

suite hash {
    description: "Hash function benchmarks"
    iterations: 5000
    warmup: 100
    baseline: "go"
    
    setup go {
        import "crypto/sha256"
        
        helpers {
            func hash(data []byte) []byte {
                h := sha256.Sum256(data)
                return h[:]
            }
        }
    }
    
    setup ts {
        import { keccak256 } from 'viem'
    }
    
    fixture data {
        hex: "68656c6c6f20776f726c64"  # "hello world"
    }
    
    bench sha256 {
        go:  hash(data)
        ts:  keccak256(data_hex)
    }
}
```

### DSL Keywords

| Keyword | Description |
|---------|-------------|
| `suite` | Top-level container for benchmarks |
| `setup <lang>` | Language-specific initialization (imports, helpers, init code) |
| `fixture` | Shared test data with portable hex encoding |
| `bench` | Individual benchmark definition |
| `globalSetup` | File-level initialization (e.g., spawning Anvil) |
| `use std::*` | Import standard library modules |

### Suite Configuration

```
suite example {
    description: "Suite description"
    iterations: 5000           # Default iteration count
    warmup: 100                # Warmup iterations before timing
    baseline: "go"             # Baseline language for comparisons
    mode: "auto"               # "auto" (calibrate to targetTime) or "fixed"
    targetTime: 3000ms         # Target time for auto mode
    minIterations: 100         # Min iterations (auto mode)
    maxIterations: 1000000     # Max iterations (auto mode)
    count: 3                   # Runs per benchmark for statistics
    memory: true               # Enable memory allocation profiling
    concurrency: 4             # Parallel workers/goroutines
    outlierDetection: true     # IQR-based outlier removal
    cvThreshold: 5.0           # Coefficient of variation target (%)
    timeout: 30000             # Suite timeout (ms)
    compare: true              # Enable comparison tables
    sink: true                 # Prevent dead code elimination
}
```

### Setup Blocks

Setup blocks contain language-specific initialization with four sections:

```
setup go {
    import "crypto/sha256"
    import "encoding/hex"
    
    declare {
        var globalCounter int
        type Config struct { Size int }
    }
    
    init {
        globalCounter = 0
    }
    
    helpers {
        func hash(data []byte) []byte {
            h := sha256.Sum256(data)
            return h[:]
        }
    }
}
```

### Fixtures

Portable test data shared across languages:

```
# Hex-encoded data
fixture short_data {
    hex: "deadbeef"
}

# Load from file
fixture long_data {
    hex: @file("fixtures/1kb.hex")
}

# Language-specific implementations
fixture complex {
    go:   buildComplexStruct()
    ts:   buildComplexObject()
    rust: build_complex_struct()
}
```

### Benchmarks

```
bench keccak256 {
    description: "Keccak-256 hash"
    iterations: 10000          # Override suite default
    warmup: 500
    timeout: 5000
    tags: ["crypto", "hash"]
    
    before {
        go:   prepareData()
        ts:   prepareData()
    }
    
    go:   keccak256(data)
    ts:   keccak256(data_hex)
    rust: keccak256(&data)
    
    after {
        charting.drawBarChart(title: "Keccak256 Performance")
    }
    
    validate {
        go:   len(result) == 32
        ts:   result.length === 66
    }
}
```

### Lifecycle Hooks

- `before` — Runs once before benchmark iterations
- `after` — Runs once after iterations (useful for charting)
- `each` — Runs before each iteration (outside timing)

### Standard Library

```
use std::anvil      # Ethereum node spawning
use std::charting   # Chart generation
use std::constants  # Mathematical constants

globalSetup {
    anvil.spawnAnvil(fork: "https://eth-mainnet.g.alchemy.com/v2/...")
}

bench example {
    go: doSomething(anvil.ANVIL_RPC_URL)
    
    after {
        charting.drawBarChart(title: "Results", horizontal: true)
        charting.drawPieChart(title: "Distribution")
    }
}
```

## LSP Support

poly-bench includes a full-featured language server for editor integration.

### Features

| Feature | Description |
|---------|-------------|
| **Diagnostics** | Parse errors, validation warnings, embedded language type checking |
| **Formatting** | Document formatting with proper indentation |
| **Completions** | Context-aware completions for keywords, stdlib, and user symbols |
| **Hover** | Documentation for DSL keywords and stdlib; delegates to gopls/tsserver/rust-analyzer for embedded code |
| **Semantic Highlighting** | Full syntax highlighting via semantic tokens |
| **Embedded Language Support** | Go, TypeScript, and Rust code blocks are checked by their respective language servers |

### Editor Extensions

- **VS Code / Cursor**: Install from `extensions/vscode` or the marketplace
- More editors coming soon

### Starting the LSP

```bash
poly-bench lsp  # Starts language server on stdio
```

## CLI Reference

### Core Commands

```bash
poly-bench check <file>           # Parse and validate
poly-bench run [<file>]           # Execute benchmarks
poly-bench codegen <file>         # Generate code without running
poly-bench fmt [<files>...]       # Format .bench files
```

### Project Management

```bash
poly-bench init [<name>]          # Initialize new project
poly-bench new <name>             # Create benchmark template
poly-bench add --go <pkg>         # Add Go dependency
poly-bench add --ts <pkg>         # Add npm dependency  
poly-bench add --rs <crate>       # Add Rust crate
poly-bench install                # Install dependencies
poly-bench build                  # Build runtime environment
```

### Run Options

```bash
poly-bench run hash.bench \
    --lang go \                   # Run only Go
    --iterations 100000 \         # Override iterations
    --report markdown \           # Output format: console, markdown, json
    --output results/             # Output directory
```

## Output Formats

### Console (default)

```
═══════════════════════════════════════════════════════════════════
  BENCHMARK RESULTS
═══════════════════════════════════════════════════════════════════

OVERALL SUMMARY

  Go is 2.5x faster overall

  Total Benchmarks:    10
  Go Wins:             8 (80%)
  TypeScript Wins:     2 (20%)
```

### Markdown

Generates `benchmark-report.md` with tables and statistics.

### JSON

Structured output to `benchmark-results.json` for CI/automation.

### SVG Charts

Visual comparisons via `charting.*` directives in `after` blocks.

## Supported Languages

| Language | Runtime | Memory Profiling | Concurrency |
|----------|---------|------------------|-------------|
| Go | Subprocess + plugin | `runtime.ReadMemStats` | Goroutines |
| TypeScript | Node.js | `process.memoryUsage()` | — |
| Rust | Cargo subprocess | Supported | — |

## Requirements

- **Go** 1.21+ (for Go benchmarks)
- **Node.js** 18+ (for TypeScript benchmarks)
- **Rust** 1.70+ (for Rust benchmarks, or building from source)

## Installation

**One-liner:**

```bash
curl -L https://raw.githubusercontent.com/evm-tooling/poly-bench/main/scripts/install.sh | bash
```

**From source:**

```bash
cargo install --path .
```

**Upgrade:**

```bash
poly-bench upgrade
```

## Release

Releases are created via the Makefile:

```bash
make release VERSION=v0.1.0
```

This bumps versions in `Cargo.toml` and `extensions/vscode/package.json`, creates a git tag, and triggers the GitHub release workflow.

## License

MIT License — see [LICENSE](LICENSE) for details.
