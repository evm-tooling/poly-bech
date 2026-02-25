# Poly-bench Runtimes

This workspace contains language-specific runtime crates for poly-bench. Each runtime implements the plugin interface defined in `poly-bench-runtime-traits`.

## Structure

```
runtimes/
├── runtimes-go/      # Go (plugin + subprocess)
├── runtimes-ts/      # TypeScript/JavaScript (Node.js)
├── runtimes-rust/    # Rust (cargo subprocess)
└── runtimes-python/  # Python (subprocess)
```

## Plugin Interface

Each runtime crate must provide:

1. **Runtime** – Implements `poly_bench_runtime_traits::Runtime`:
   - `compile_check`, `run_benchmark`, `initialize`, `shutdown`
   - `generate_check_source` for validation

2. **RuntimeFactory** – Implements `poly_bench_runtime_traits::RuntimeFactory`:
   - `create(config) -> Box<dyn Runtime>`
   - Registered in `poly-bench-runtime` registry

3. **ErrorMapper** – Implements `poly_bench_runtime_traits::ErrorMapper`:
   - `build_mappings(suite, generated_code)` – map generated lines to .bench lines
   - `remap_error(error, mappings)` – rewrite compiler errors with .bench line refs
   - Export static `*_ERROR_MAPPER` for `get_error_mapper()` dispatch

4. **lang_display** – Export `fn <lang>_lang_display() -> LangDisplayInfo`:
   - Labels, colors, gradients for UI (executor, reporter, CLI)

## Adding a New Runtime

1. Copy `runtimes-rust` as a template (it has the full structure)
2. Update `Cargo.toml` (name, dependencies)
3. Implement `Runtime`, `RuntimeFactory`, `ErrorMapper`, `*_lang_display()`
4. Add crate to root workspace `Cargo.toml`
5. Register in `poly-bench-runtime/src/registry.rs`
6. Add to `get_error_mapper()` and `lang_display()` in poly-bench-runtime
7. Add `Lang` variant in poly-bench-dsl
8. Implement `ImportExtractor` in poly-bench-ir
9. Implement `ProjectRootDetector` in poly-bench-project
10. Add LSP support (VirtualFileBuilder, EmbeddedDiagnosticProvider, EmbeddedHoverProvider)

See [docs/ADDING_A_RUNTIME.md](../docs/ADDING_A_RUNTIME.md) for the full checklist.

## Dependencies

- **poly-bench-runtime-traits** – `Runtime`, `RuntimeFactory`, `ErrorMapper`, `LangDisplayInfo`, `Measurement`, `RuntimeConfig`
- **poly-bench-dsl** – `Lang`, `BenchMode`, etc.
- **poly-bench-ir** – `SuiteIR`, `BenchmarkSpec`, `FixtureIR`, etc.
- **poly-bench-stdlib** – stdlib imports/code for benchmarks

Runtime crates do **not** depend on `poly-bench-runtime`; the registry in poly-bench-runtime depends on them to avoid circular dependencies.
