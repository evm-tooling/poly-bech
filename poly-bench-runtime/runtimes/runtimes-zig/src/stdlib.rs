//! Zig-specific standard library code (std::anvil, std::constants)

use poly_bench_traits::StdlibProvider;

const ZIG_ANVIL: &str = r#"
// std::anvil - Anvil RPC URL from poly-bench (managed by scheduler)
const ANVIL_RPC_URL = std.posix.getenv("ANVIL_RPC_URL");
"#;

const ZIG_CONSTANTS: &str = r#"
// std::constants - Mathematical constants from poly-bench standard library
const std_PI: f64 = 3.14159265358979323846;
const std_E: f64 = 2.71828182845904523536;
"#;

pub struct ZigStdlibProvider;

impl StdlibProvider for ZigStdlibProvider {
    fn anvil_code(&self) -> Option<&'static str> {
        Some(ZIG_ANVIL)
    }

    fn anvil_imports(&self) -> Vec<&'static str> {
        vec!["const std = @import(\"std\");"]
    }

    fn constants_code(&self) -> Option<&'static str> {
        Some(ZIG_CONSTANTS)
    }
}

pub static ZIG_STDLIB: ZigStdlibProvider = ZigStdlibProvider;
