//! Rust runtime for executing Rust benchmarks via subprocess

pub mod codegen;
pub mod executor;
pub mod shared;

pub use executor::RustRuntime;
