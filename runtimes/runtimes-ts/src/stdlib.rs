//! TypeScript-specific standard library code (std::anvil, std::constants)

use poly_bench_runtime_traits::StdlibProvider;

const TS_ANVIL: &str = r#"
// std::anvil - Anvil RPC URL from poly-bench (managed by scheduler)
// The Anvil node is automatically started by poly-bench before benchmarks run.
const ANVIL_RPC_URL: string = process.env.ANVIL_RPC_URL || "";
"#;

const TS_CONSTANTS: &str = r#"
// std::constants - Mathematical constants from poly-bench standard library
const std_PI: number = 3.14159265358979323846;
const std_E: number = 2.71828182845904523536;
"#;

pub struct TsStdlibProvider;

impl StdlibProvider for TsStdlibProvider {
    fn anvil_code(&self) -> Option<&'static str> {
        Some(TS_ANVIL)
    }

    fn constants_code(&self) -> Option<&'static str> {
        Some(TS_CONSTANTS)
    }
}

pub static TS_STDLIB: TsStdlibProvider = TsStdlibProvider;
