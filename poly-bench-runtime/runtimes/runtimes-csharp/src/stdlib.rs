//! C#-specific standard library code (std::anvil, std::constants)

use poly_bench_traits::StdlibProvider;

const CSHARP_ANVIL: &str = r#"
// std::anvil - Anvil RPC URL from poly-bench (managed by scheduler)
// The Anvil node is automatically started by poly-bench before benchmarks run.
var ANVIL_RPC_URL = Environment.GetEnvironmentVariable("ANVIL_RPC_URL") ?? string.Empty;
"#;

const CSHARP_CONSTANTS: &str = r#"
// std::constants - Mathematical constants from poly-bench standard library
const double std_PI = 3.14159265358979323846;
const double std_E = 2.71828182845904523536;
"#;

pub struct CSharpStdlibProvider;

impl StdlibProvider for CSharpStdlibProvider {
    fn anvil_code(&self) -> Option<&'static str> {
        Some(CSHARP_ANVIL)
    }

    fn constants_code(&self) -> Option<&'static str> {
        Some(CSHARP_CONSTANTS)
    }
}

pub static CSHARP_STDLIB: CSharpStdlibProvider = CSharpStdlibProvider;
