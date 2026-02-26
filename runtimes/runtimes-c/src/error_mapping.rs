//! C error mapping - passthrough remapping.

use poly_bench_dsl::Lang;
use poly_bench_ir::SuiteIR;
use poly_bench_runtime_traits::{ErrorMapper, LineMappings};

pub struct CErrorMapper;

pub static C_ERROR_MAPPER: CErrorMapper = CErrorMapper;

impl ErrorMapper for CErrorMapper {
    fn lang(&self) -> Lang {
        Lang::C
    }

    fn build_mappings(&self, _suite: &SuiteIR, _generated_code: &str) -> LineMappings {
        LineMappings::default()
    }

    fn remap_error(&self, error: &str, _mappings: &LineMappings) -> String {
        error.to_string()
    }
}
