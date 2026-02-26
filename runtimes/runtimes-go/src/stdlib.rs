//! Go-specific standard library code (std::anvil, std::constants)

use poly_bench_runtime_traits::StdlibProvider;

const GO_ANVIL: &str = r#"
// std::anvil - Anvil RPC URL from poly-bench (managed by scheduler)
// The Anvil node is automatically started by poly-bench before benchmarks run.
var ANVIL_RPC_URL = os.Getenv("ANVIL_RPC_URL")
"#;

const GO_CONSTANTS: &str = r#"
// std::constants - Mathematical constants from poly-bench standard library
const std_PI float64 = 3.14159265358979323846
const std_E float64 = 2.71828182845904523536
"#;

pub struct GoStdlibProvider;

impl StdlibProvider for GoStdlibProvider {
    fn anvil_code(&self) -> Option<&'static str> {
        Some(GO_ANVIL)
    }

    fn anvil_imports(&self) -> Vec<&'static str> {
        vec!["\"os\""]
    }

    fn constants_code(&self) -> Option<&'static str> {
        Some(GO_CONSTANTS)
    }
}

pub static GO_STDLIB: GoStdlibProvider = GoStdlibProvider;
