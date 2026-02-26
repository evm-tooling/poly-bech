//! Rust-specific standard library code (std::anvil, std::constants)

use poly_bench_runtime_traits::StdlibProvider;

const RUST_ANVIL: &str = r##"
// std::anvil - Anvil RPC URL from poly-bench (managed by scheduler)
// The Anvil node is automatically started by poly-bench before benchmarks run.
lazy_static::lazy_static! {
    static ref ANVIL_RPC_URL: String = std::env::var("ANVIL_RPC_URL").unwrap_or_default();
}
"##;

const RUST_CONSTANTS: &str = r#"
// std::constants - Mathematical constants from poly-bench standard library
const STD_PI: f64 = 3.14159265358979323846;
const STD_E: f64 = 2.71828182845904523536;
"#;

pub struct RustStdlibProvider;

impl StdlibProvider for RustStdlibProvider {
    fn anvil_code(&self) -> Option<&'static str> {
        Some(RUST_ANVIL)
    }

    fn constants_code(&self) -> Option<&'static str> {
        Some(RUST_CONSTANTS)
    }
}

pub static RUST_STDLIB: RustStdlibProvider = RustStdlibProvider;
