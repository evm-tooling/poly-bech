//! TypeScript project root detection

use poly_bench_dsl::Lang;
use poly_bench_traits::ProjectRootDetector;

pub struct TsDetector;

impl ProjectRootDetector for TsDetector {
    fn lang(&self) -> Lang {
        Lang::TypeScript
    }
    fn marker_files(&self) -> &[&'static str] {
        &["package.json", "node_modules"]
    }
}

pub static TS_DETECTOR: TsDetector = TsDetector;
