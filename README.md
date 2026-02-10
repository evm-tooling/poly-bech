# poly-bench

A high-performance multi-language benchmarking framework with a custom DSL.

## Overview

poly-bench allows you to define benchmarks once and run them across multiple programming languages (Go, TypeScript) with unified output and comparison. It's designed for:

- **Cross-language library comparisons** - Compare your Go library against its TypeScript equivalent
- **Performance regression testing** - Track performance across languages over time
- **Fair benchmarking** - Same data, same iterations, unified measurement

## Features

- **Custom DSL** - Purpose-built syntax for defining benchmarks
- **Portable fixtures** - Share test data across languages via hex encoding
- **Unified measurements** - ns/op, ops/sec, percentiles across all languages
- **Multiple runtimes** - Go (via subprocess), TypeScript (via Node.js)
- **Rich output** - Console, Markdown, JSON, and SVG charts

## Installation

```bash
# From source
cargo install --path .

# Or build directly
cargo build --release
```

### Requirements

- **Rust** 1.70+ (for building)
- **Go** 1.21+ (for Go benchmarks)
- **Node.js** 18+ (for TypeScript benchmarks)

## Quick Start

### 1. Create a benchmark file

```
# hash.bench
suite hash {
    description: "Hash function benchmarks"
    iterations: 5000
    
    fixture data {
        hex: "68656c6c6f20776f726c64"  # "hello world"
    }
    
    bench keccak256 {
        go:  keccak256(data)
        ts:  keccak256(data_hex)
    }
}
```

### 2. Validate the syntax

```bash
poly-bench check hash.bench
```

### 3. Run benchmarks

```bash
# Run all languages
poly-bench run hash.bench

# Run only Go
poly-bench run hash.bench --lang go

# Run with custom iterations
poly-bench run hash.bench --iterations 10000

# Generate markdown report
poly-bench run hash.bench --report markdown --output results/
```

## DSL Syntax

### Suite

A suite is the top-level container for benchmarks:

```
suite <name> {
    description: "Optional description"
    iterations: 5000    # Default iterations
    warmup: 100         # Warmup iterations
    
    # ... setup, fixtures, benchmarks
}
```

### Setup

Language-specific initialization code:

```
setup go {
    import "crypto/sha256"
    
    func hash(data []byte) []byte {
        h := sha256.Sum256(data)
        return h[:]
    }
}

setup ts {
    import { sha256 } from 'viem'
}
```

### Fixtures

Shared test data with portable hex format:

```
# Simple hex fixture
fixture short_data {
    hex: "deadbeef"
}

# Fixture from file
fixture long_data {
    hex: @file("fixtures/1kb.hex")
}

# Language-specific fixtures (for complex data)
fixture complex {
    go: buildComplexStruct()
    ts: buildComplexObject()
}
```

### Benchmarks

Individual benchmark definitions:

```
bench <name> {
    description: "Optional description"
    iterations: 10000   # Override suite default
    
    go:  hash(data)
    ts:  hash(data_hex)
}
```

## CLI Reference

### `poly-bench check <file>`

Parse and validate a benchmark file.

```bash
poly-bench check hash.bench --show-ast
```

### `poly-bench run <file>`

Execute benchmarks.

```bash
poly-bench run hash.bench [options]

Options:
  --lang <go|ts>        Run only specified language
  --iterations <N>      Override iteration count
  --report <format>     Output format: console, markdown, json
  --output <dir>        Output directory for reports
```

### `poly-bench codegen <file>`

Generate code without running.

```bash
poly-bench codegen hash.bench --lang go --output generated/
```

## Output Formats

### Console (default)

```
═══════════════════════════════════════════════════════════════════
  BENCHMARK RESULTS
═══════════════════════════════════════════════════════════════════

OVERALL SUMMARY

  🏆 Go is 2.5x faster overall

  Total Benchmarks:    10
  Go Wins:             8 (80%)
  TypeScript Wins:     2 (20%)
```

### Markdown

Generates a complete report with tables and summary statistics.

### JSON

Structured output for programmatic processing.

### SVG Charts

Visual speedup comparison charts.

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        poly-bench CLI                           │
├─────────────────────────────────────────────────────────────────┤
│  DSL Parser (custom lexer + parser)                             │
│         ↓                                                       │
│  IR (Normalized benchmark specifications)                       │
│         ↓                                                       │
│  ┌─────────────────┐    ┌─────────────────┐                    │
│  │   Go Runtime    │    │   JS Runtime    │                    │
│  │  (subprocess)   │    │   (Node.js)     │                    │
│  └─────────────────┘    └─────────────────┘                    │
│         ↓                       ↓                               │
│  Unified Measurement Collection                                 │
│         ↓                                                       │
│  Reporters (Console, Markdown, JSON, SVG)                       │
└─────────────────────────────────────────────────────────────────┘
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT License - see [LICENSE](LICENSE) for details.
