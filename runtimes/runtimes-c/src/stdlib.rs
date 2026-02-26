//! C-specific standard library code (std::anvil, std::constants)

use poly_bench_runtime_traits::StdlibProvider;

const C_ANVIL: &str = r#"
// std::anvil - Anvil RPC URL from poly-bench (managed by scheduler)
const char* ANVIL_RPC_URL = getenv("ANVIL_RPC_URL");
"#;

const C_CONSTANTS: &str = r#"
// std::constants - Mathematical constants from poly-bench standard library
const double std_PI = 3.14159265358979323846;
const double std_E = 2.71828182845904523536;
"#;

pub struct CStdlibProvider;

impl StdlibProvider for CStdlibProvider {
    fn anvil_code(&self) -> Option<&'static str> {
        Some(C_ANVIL)
    }

    fn anvil_imports(&self) -> Vec<&'static str> {
        vec!["#include <stdlib.h>"]
    }

    fn constants_code(&self) -> Option<&'static str> {
        Some(C_CONSTANTS)
    }
}

pub static C_STDLIB: CStdlibProvider = CStdlibProvider;
