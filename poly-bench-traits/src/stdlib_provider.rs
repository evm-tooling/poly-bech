//! Stdlib provider trait for language-specific standard library code
//!
//! Each runtime implements this trait to provide its own anvil and constants
//! code. This allows poly-bench-stdlib to be language-agnostic—new runtimes
//! register their stdlib code via this trait without modifying the stdlib crate.

/// Provider of language-specific standard library code (anvil, constants).
///
/// Each runtime implements this trait with its own code. The stdlib crate
/// calls these methods when injecting code into generated benchmarks.
pub trait StdlibProvider: Send + Sync {
    /// Code for std::anvil (ANVIL_RPC_URL). Returns None if this runtime
    /// does not support the anvil module.
    fn anvil_code(&self) -> Option<&'static str> {
        None
    }

    /// Additional imports required for std::anvil (e.g. Go needs "os").
    fn anvil_imports(&self) -> Vec<&'static str> {
        Vec::new()
    }

    /// Code for std::constants (PI, E). Returns None if this runtime
    /// does not support the constants module.
    fn constants_code(&self) -> Option<&'static str> {
        None
    }
}
