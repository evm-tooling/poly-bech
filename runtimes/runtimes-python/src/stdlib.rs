//! Python-specific standard library code (std::anvil, std::constants)

use poly_bench_runtime_traits::StdlibProvider;

const PYTHON_ANVIL: &str = r#"
# std::anvil - Anvil RPC URL from poly-bench (managed by scheduler)
# The Anvil node is automatically started by poly-bench before benchmarks run.
import os
ANVIL_RPC_URL = os.environ.get("ANVIL_RPC_URL", "")
"#;

const PYTHON_CONSTANTS: &str = r#"
# std::constants - Mathematical constants from poly-bench standard library
std_PI = 3.14159265358979323846
std_E = 2.71828182845904523536
"#;

pub struct PythonStdlibProvider;

impl StdlibProvider for PythonStdlibProvider {
    fn anvil_code(&self) -> Option<&'static str> {
        Some(PYTHON_ANVIL)
    }

    fn constants_code(&self) -> Option<&'static str> {
        Some(PYTHON_CONSTANTS)
    }
}

pub static PYTHON_STDLIB: PythonStdlibProvider = PythonStdlibProvider;
