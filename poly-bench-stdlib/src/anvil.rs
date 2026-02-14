//! Anvil module - Anvil RPC URL accessor for EVM benchmarks
//!
//! When `use std::anvil` is specified along with `globalSetup { spawnAnvil() }`,
//! poly-bench spawns a local Anvil Ethereum node and makes the RPC URL available
//! via the `ANVIL_RPC_URL` variable.
//!
//! ## Available Variables
//!
//! - `ANVIL_RPC_URL` - The RPC endpoint URL (e.g., "http://127.0.0.1:8545")
//!
//! ## Usage
//!
//! ```bench
//! use std::anvil
//!
//! globalSetup {
//!     spawnAnvil()                           // Basic spawn
//!     // spawnAnvil(fork: "https://...")     // With chain forking
//! }
//!
//! suite evmBench {
//!     setup go {
//!         import ("net/http")
//!         
//!         helpers {
//!             func callRpc() {
//!                 http.Post(ANVIL_RPC_URL, "application/json", ...)
//!             }
//!         }
//!     }
//!     
//!     bench rpcTest {
//!         go: callRpc()
//!     }
//! }
//! ```

use poly_bench_dsl::Lang;

/// Get the language-specific imports for the anvil module
pub fn get_imports(lang: Lang) -> Vec<&'static str> {
    match lang {
        Lang::Go => vec!["\"os\""], // Go needs "os" to read ANVIL_RPC_URL env var
        _ => vec![],
    }
}

/// Get the language-specific code for the anvil module
pub fn get_code(lang: Lang) -> String {
    match lang {
        Lang::Go => GO_ANVIL.to_string(),
        Lang::TypeScript => TS_ANVIL.to_string(),
        Lang::Rust => RUST_ANVIL.to_string(),
        Lang::Python => PYTHON_ANVIL.to_string(),
    }
}

const GO_ANVIL: &str = r#"
// std::anvil - Anvil RPC URL from poly-bench (managed by scheduler)
// The Anvil node is automatically started by poly-bench before benchmarks run.
var ANVIL_RPC_URL = os.Getenv("ANVIL_RPC_URL")
"#;

const TS_ANVIL: &str = r#"
// std::anvil - Anvil RPC URL from poly-bench (managed by scheduler)
// The Anvil node is automatically started by poly-bench before benchmarks run.
const ANVIL_RPC_URL: string = process.env.ANVIL_RPC_URL || "";
"#;

const RUST_ANVIL: &str = r##"
// std::anvil - Anvil RPC URL from poly-bench (managed by scheduler)
// The Anvil node is automatically started by poly-bench before benchmarks run.
lazy_static::lazy_static! {
    static ref ANVIL_RPC_URL: String = std::env::var("ANVIL_RPC_URL").unwrap_or_default();
}
"##;

const PYTHON_ANVIL: &str = r#"
# std::anvil - Anvil RPC URL from poly-bench (managed by scheduler)
# The Anvil node is automatically started by poly-bench before benchmarks run.
import os
ANVIL_RPC_URL = os.environ.get("ANVIL_RPC_URL", "")
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_go_anvil_contains_rpc_url() {
        let code = get_code(Lang::Go);
        assert!(code.contains("ANVIL_RPC_URL"));
        assert!(code.contains("os.Getenv"));
    }

    #[test]
    fn test_ts_anvil_contains_rpc_url() {
        let code = get_code(Lang::TypeScript);
        assert!(code.contains("ANVIL_RPC_URL"));
        assert!(code.contains("process.env"));
    }

    #[test]
    fn test_rust_anvil_contains_rpc_url() {
        let code = get_code(Lang::Rust);
        assert!(code.contains("ANVIL_RPC_URL"));
        assert!(code.contains("std::env::var"));
    }

    #[test]
    fn test_python_anvil_contains_rpc_url() {
        let code = get_code(Lang::Python);
        assert!(code.contains("ANVIL_RPC_URL"));
        assert!(code.contains("os.environ"));
    }
}
