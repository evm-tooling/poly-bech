//! Python error mapping - passthrough (no line remapping for generated Python)

use poly_bench_dsl::Lang;
use poly_bench_ir::SuiteIR;
use poly_bench_runtime_traits::{ErrorMapper, LineMappings};

pub struct PythonErrorMapper;

pub static PYTHON_ERROR_MAPPER: PythonErrorMapper = PythonErrorMapper;

impl ErrorMapper for PythonErrorMapper {
    fn lang(&self) -> Lang {
        Lang::Python
    }
    fn build_mappings(&self, _suite: &SuiteIR, _generated_code: &str) -> LineMappings {
        LineMappings::default()
    }
    fn remap_error(&self, error: &str, _mappings: &LineMappings) -> String {
        error.to_string()
    }
}
