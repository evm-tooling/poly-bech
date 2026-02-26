//! Zig error mapping - passthrough remapping.

use poly_bench_dsl::Lang;
use poly_bench_ir::SuiteIR;
use poly_bench_runtime_traits::{ErrorMapper, LineMappings};

pub struct ZigErrorMapper;

pub static ZIG_ERROR_MAPPER: ZigErrorMapper = ZigErrorMapper;

impl ErrorMapper for ZigErrorMapper {
    fn lang(&self) -> Lang {
        Lang::Zig
    }

    fn build_mappings(&self, _suite: &SuiteIR, _generated_code: &str) -> LineMappings {
        LineMappings::default()
    }

    fn remap_error(&self, error: &str, _mappings: &LineMappings) -> String {
        error.to_string()
    }
}
