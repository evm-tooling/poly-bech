//! C# error mapping - passthrough remapping.

use poly_bench_dsl::Lang;
use poly_bench_ir::SuiteIR;
use poly_bench_traits::{ErrorMapper, LineMappings};

pub struct CSharpErrorMapper;

pub static CSHARP_ERROR_MAPPER: CSharpErrorMapper = CSharpErrorMapper;

impl ErrorMapper for CSharpErrorMapper {
    fn lang(&self) -> Lang {
        Lang::CSharp
    }

    fn build_mappings(&self, _suite: &SuiteIR, _generated_code: &str) -> LineMappings {
        LineMappings::default()
    }

    fn remap_error(&self, error: &str, _mappings: &LineMappings) -> String {
        error.to_string()
    }
}
