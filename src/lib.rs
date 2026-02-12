//! poly-bench: A high-performance multi-language benchmarking framework
//!
//! This library provides:
//! - A custom DSL for defining cross-language benchmarks
//! - Embedded V8 runtime for TypeScript/JavaScript execution
//! - Go plugin-based execution for Go benchmarks
//! - Unified measurement and comparison across languages
//! - Project scaffolding and dependency management

// Re-export from crates for backwards compatibility
pub use poly_bench_dsl as dsl;
pub use poly_bench_stdlib as stdlib;
pub use poly_bench_ir as ir;

pub mod runtime;
pub mod executor;
pub mod reporter;
pub mod project;