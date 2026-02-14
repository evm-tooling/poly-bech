//! Go runtime for executing Go benchmarks via plugin system

pub mod codegen;
pub mod compiler;
pub mod executor;
pub mod shared;

pub use executor::GoRuntime;
