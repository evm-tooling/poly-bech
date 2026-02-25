//! JavaScript/TypeScript runtime for executing benchmarks via V8

pub mod builtins;
pub mod codegen;
pub mod transpiler;
pub mod v8_runtime;

pub use v8_runtime::{extract_generated_snippet, extract_runtime_error_reason, JsRuntime};
