---
name: Modular Runtime Architecture
overview: Refactor poly-bench to use a modular runtime architecture with traits and registries, enabling new runtimes (e.g. Python) to be added by implementing a small set of interfaces and registering once. Migration is incremental with no parallel code paths.
todos: []
isProject: false
---

# Modular Runtime Architecture Refactor

## Architecture Overview

```mermaid
flowchart TB
    subgraph CLI [CLI / Executor]
        ProjectRoots
        resolve_project_roots
    end
    
    subgraph Runtime [poly-bench-runtime]
        RuntimeConfig
        RuntimeFactory
        Registry
        Runtime
    end
    
    subgraph IR [poly-bench-ir]
        ImportExtractor
        lower
    end
    
    subgraph LSP [poly-bench-lsp-v2]
        VirtualFileBuilder
        EmbeddedDiagnosticProvider
    end
    
    ProjectRoots -->|"convert"| RuntimeConfig
    RuntimeConfig --> Registry
    Registry -->|"create"| Runtime
    ImportExtractor --> lower
    VirtualFileBuilder --> EmbeddedDiagnosticProvider
```



## Phase 1: RuntimeFactory and RuntimeConfig

**Goal:** Replace hardcoded runtime creation in validation and scheduler with registry lookup.

### 1.1 Add RuntimeConfig (poly-bench-runtime)

Create [poly-bench-runtime/src/config.rs](poly-bench-runtime/src/config.rs):

```rust
use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct RuntimeConfig {
    pub go_root: Option<PathBuf>,
    pub node_root: Option<PathBuf>,
    pub rust_root: Option<PathBuf>,
}
```

Add `pub mod config` and `pub use config::RuntimeConfig` to [poly-bench-runtime/src/lib.rs](poly-bench-runtime/src/lib.rs).

### 1.2 Add RuntimeFactory trait and registry (poly-bench-runtime)

Create [poly-bench-runtime/src/registry.rs](poly-bench-runtime/src/registry.rs):

- `RuntimeFactory` trait with `lang()`, `name()`, `create(&self, config: &RuntimeConfig) -> Result<Box<dyn Runtime>>`
- Static `FACTORIES: &[&dyn RuntimeFactory]` with Go, Js, Rust factories
- `create_runtime(lang: Lang, config: &RuntimeConfig) -> Result<Box<dyn Runtime>>`
- `supported_languages() -> &'static [Lang]`

### 1.3 Implement factories for each runtime

Each runtime module exposes a factory:

- [poly-bench-runtime/src/go/mod.rs](poly-bench-runtime/src/go/mod.rs): `GoRuntimeFactory` that creates `GoRuntime`, calls `set_module_root(config.go_root.clone())`
- [poly-bench-runtime/src/js/mod.rs](poly-bench-runtime/src/js/mod.rs): `JsRuntimeFactory` for `JsRuntime`, `set_project_root(config.node_root.clone())`
- [poly-bench-runtime/src/rust/mod.rs](poly-bench-runtime/src/rust/mod.rs): `RustRuntimeFactory` for `RustRuntime`, `set_project_root(config.rust_root.clone())`

### 1.4 Update executor to use registry

In [poly-bench-executor/src/validation.rs](poly-bench-executor/src/validation.rs) (lines 139-163) and the cached validation path (lines 450-470):

- Add conversion: `let config = RuntimeConfig { go_root: project_roots.go_root.clone(), node_root: project_roots.node_root.clone(), rust_root: project_roots.rust_root.clone() };`
- Replace `if langs.contains(&Lang::Go) { let mut rt = GoRuntime::new(); ... }` with a loop over `langs` calling `runtime::create_runtime(lang, &config)` and storing in `HashMap<Lang, Arc<dyn Runtime>>`
- Update compile_check calls to use the map: `runtimes.get(&Lang::Go).and_then(|rt| rt.compile_check(...))`

In [poly-bench-executor/src/scheduler.rs](poly-bench-executor/src/scheduler.rs) (lines 448-485):

- Same pattern: build `HashMap<Lang, Box<dyn Runtime>>` via registry instead of separate `go_runtime`, `js_runtime`, `rust_runtime` variables
- Update all `go_runtime.as_mut()`, `js_runtime.as_mut()`, etc. to `runtimes.get_mut(&lang)`

**Verification:** `cargo run -- run`, `cargo run -- compile`, `cargo run -- compile --lang go` produce identical behavior. Run `poly-bench run` in [examples/demo-basic](examples/demo-basic).

---

## Phase 2: ImportExtractor Trait

**Goal:** Make import extraction extensible via a trait.

### 2.1 Add ImportExtractor trait (poly-bench-ir)

In [poly-bench-ir/src/imports.rs](poly-bench-ir/src/imports.rs):

- Add `ImportExtractor` trait: `fn lang(&self) -> Lang`, `fn extract(&self, setup: &str) -> ParsedSetup`
- Create `GoImportExtractor`, `TsImportExtractor`, `RustImportExtractor` structs that wrap `extract_go_imports`, `extract_ts_imports`, `extract_rust_imports`

### 2.2 Add dispatch function

```rust
pub fn extract_imports(lang: Lang, setup: &str) -> ParsedSetup {
    match lang {
        Lang::Go => GoImportExtractor.extract(setup),
        Lang::TypeScript => TsImportExtractor.extract(setup),
        Lang::Rust => RustImportExtractor.extract(setup),
        _ => ParsedSetup::passthrough(setup),
    }
}
```

### 2.3 Update lower.rs

In [poly-bench-ir/src/lower.rs](poly-bench-ir/src/lower.rs) (lines 124-128), replace:

```rust
let parsed = match lang {
    Lang::Go => extract_go_imports(&import_block.code),
    Lang::TypeScript => extract_ts_imports(&import_block.code),
    Lang::Rust => extract_rust_imports(&import_block.code),
    _ => ParsedSetup::passthrough(&import_block.code),
};
```

with `extract_imports(*lang, &import_block.code)`.

**Verification:** IR output unchanged. Run `poly-bench check` and compare AST/IR; existing tests in `lower.rs` pass.

---

## Phase 3: ProjectRootDetector Trait

**Goal:** Centralize project root detection per language.

### 3.1 Add trait and implementations (poly-bench-project)

Create [poly-bench-project/src/detectors.rs](poly-bench-project/src/detectors.rs):

- `ProjectRootDetector` trait: `fn lang(&self) -> Lang`, `fn marker_files(&self) -> &[&'static str]`, `fn detect(&self, start: &Path) -> Option<PathBuf>`
- Generic `detect_from_markers(start, markers)` that walks up directories
- `GoDetector` (markers: `["go.mod"]`), `TsDetector` (`["package.json", "node_modules"]`), `RustDetector` (`["Cargo.toml"]`)

### 3.2 Refactor resolve_project_roots (CLI)

In [cli/main.rs](cli/main.rs) `resolve_project_roots` (lines 648-711):

- For each detector, if explicit root not provided, use `detector.detect(start_dir)` instead of inline logic
- Preserve existing behavior: polybench.toml runtime-env paths, fallback to classic layout

**Verification:** Same roots for existing projects. Test with `--go-project`, `--ts-project` overrides.

---

## Phase 4: ErrorMapper Trait

**Goal:** Make error mapping language-agnostic.

### 4.1 Add ErrorMapper trait (poly-bench-runtime)

In [poly-bench-runtime/src/error_mapping.rs](poly-bench-runtime/src/error_mapping.rs):

- Add `ErrorMapper` trait: `fn lang(&self) -> Lang`, `fn build_mappings(&self, suite: &SuiteIR, generated: &str) -> LineMappings`, `fn remap_error(&self, error: &str, mappings: &LineMappings) -> String`
- Implement for Go, TypeScript, Rust (wrap existing `build_*_mappings`, `remap_*_error`)

### 4.2 Use in compile_check path

Each runtime's `compile_check` already uses error mapping internally (Go compiler, TS transpiler, Rust cargo). The error mapping is called when converting stderr to user-facing messages. Trace where `remap_*_error` is used and ensure it goes through the trait if applicable. If error mapping is fully internal to each runtime, this phase may only add the trait for future use; confirm usage in [poly-bench-runtime/src/go/executor.rs](poly-bench-runtime/src/go/executor.rs) and similar.

**Verification:** Compile errors still map correctly to .bench file lines.

---

## Phase 5: LSP VirtualFileBuilder and EmbeddedDiagnosticProvider Registry

**Goal:** LSP diagnostics via registry instead of hardcoded branches.

### 5.1 Add VirtualFileBuilder trait (poly-bench-lsp-v2)

In [poly-bench-lsp-v2/src/virtual_files.rs](poly-bench-lsp-v2/src/virtual_files.rs):

- `VirtualFileBuilder` trait: `fn lang(&self) -> Lang`, `fn build(&self, params: VirtualFileParams) -> Box<dyn VirtualFile>`
- `VirtualFileParams` struct (bench_uri, bench_path, module_root, blocks, fixture_names, version)
- Implement for `VirtualGoFile`, `VirtualTsFile`, `VirtualRustFile` (wrap existing `from_blocks_with_fixtures`)

### 5.2 Refactor check_embedded_code (poly-bench-lsp-v2)

In [poly-bench-lsp-v2/src/embedded_diagnostics/mod.rs](poly-bench-lsp-v2/src/embedded_diagnostics/mod.rs) (lines 82-173):

- Replace the three `if let Some(x_blocks) = blocks_by_lang.get(&Lang::X)` blocks with a loop over a registry of `(VirtualFileBuilder, EmbeddedDiagnosticProvider)` per language
- Each provider: build virtual file, write to disk, run check (go::check_go_blocks, typescript::check_ts_blocks, rust::check_rust_blocks), map diagnostics

**Verification:** LSP diagnostics for Go, TS, Rust in .bench files unchanged. Test with `poly-bench lsp` and editor.

---

## Phase 6: Python Runtime (Validation of New Architecture)

**Goal:** Add Python as the first runtime using the new modular system.

- Implement `Runtime` for Python in `poly-bench-runtime/src/python/`
- Implement `ImportExtractor` for Python (`extract_python_imports`)
- Implement `ProjectRootDetector` for Python (`requirements.txt`, `pyproject.toml`)
- Implement `ErrorMapper` for Python
- Implement `VirtualFileBuilder` and `EmbeddedDiagnosticProvider` for Python (pylsp or pyright)
- Register in all registries
- Add CLI support (`--lang python`, `--python-project`, codegen, add/remove deps)
- Add `Lang::Python` branches where needed (already in DSL)

---

## File Change Summary


| Phase | Files to Create                                    | Files to Modify                                                                                                                                              |
| ----- | -------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 1     | `runtime/src/config.rs`, `runtime/src/registry.rs` | `runtime/src/lib.rs`, `runtime/src/go/mod.rs`, `runtime/src/js/mod.rs`, `runtime/src/rust/mod.rs`, `executor/src/validation.rs`, `executor/src/scheduler.rs` |
| 2     | -                                                  | `ir/src/imports.rs`, `ir/src/lower.rs`                                                                                                                       |
| 3     | `project/src/detectors.rs`                         | `project/src/lib.rs`, `cli/main.rs`                                                                                                                          |
| 4     | -                                                  | `runtime/src/error_mapping.rs`                                                                                                                               |
| 5     | -                                                  | `lsp-v2/src/virtual_files.rs`, `lsp-v2/src/embedded_diagnostics/mod.rs`                                                                                      |
| 6     | `runtime/src/python/`*                             | Multiple (registry, imports, detectors, CLI, LSP)                                                                                                            |


---

## Migration Approach

**Incremental refactor (no parallel path):** Each phase is a single PR/commit. After each phase, run full test suite and manual smoke tests. No feature flags or dual code paths.

**Order rationale:** Phase 1 has the highest impact and unblocks the registry pattern. Phases 2-4 are independent and can be reordered. Phase 5 touches LSP (higher risk). Phase 6 validates the design.

---

## Success Criteria

- All existing benchmarks (Go, TS, Rust) run identically before and after
- Adding a new runtime requires: implement N traits, add N registry entries, minimal edits to core crates
- No circular dependencies introduced
- Test coverage maintained or improved

