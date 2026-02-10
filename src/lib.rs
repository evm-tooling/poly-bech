//! poly-bench: A high-performance multi-language benchmarking framework
//!
//! This library provides:
//! - A custom DSL for defining cross-language benchmarks
//! - Embedded V8 runtime for TypeScript/JavaScript execution
//! - Go plugin-based execution for Go benchmarks
//! - Unified measurement and comparison across languages

pub mod dsl;
pub mod ir;
pub mod runtime;
pub mod executor;
pub mod reporter;
