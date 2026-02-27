# test-init-bubble3

A poly-bench project for cross-language benchmarking.

## Getting Started

1. Install dependencies:

```bash
poly-bench install
```

2. Run benchmarks:

```bash
poly-bench run
```

## Project Structure

```
test-init-bubble3/
├── polybench.toml       # Project configuration
├── benchmarks/          # Benchmark files (.bench)
│   └── example.bench    # Example benchmark
├── out/                 # Results and charts (gitignored)
│   ├── results.json     # Benchmark results
│   └── *.svg            # Generated charts
└── .polybench/          # Runtime environment (gitignored)
    └── runtime-env/      # Per-runtime deps and harness
        └── go/           # go.mod, go.sum, generated bench code
        └── ts/           # package.json, node_modules, generated bench code
        └── c/            # Makefile, bench.c, generated bench code
```

## Adding Dependencies

### Go

```bash
poly-bench add --go "github.com/ethereum/go-ethereum@v1.13.0"
```

### TypeScript

```bash
poly-bench add --ts "viem@^2.0.0"
```

### C

```bash
poly-bench add --c "openssl@3.2"
```

## Creating New Benchmarks

```bash
poly-bench new my-benchmark
```

This creates `benchmarks/my-benchmark.bench` with a template.

## DSL Reference

```bench
suite my_suite {
    iterations: 1000
    warmup: 100
    description: "My benchmarks"

    setup go {
        import "my/package"
    }

    setup ts {
        import { myFunc } from 'my-package';
    }

    fixture data {
        hex: "68656c6c6f"  // Binary data as hex
    }

    bench my_benchmark {
        go: myFunc(data)
        ts: myFunc(data)
    }
}
```
