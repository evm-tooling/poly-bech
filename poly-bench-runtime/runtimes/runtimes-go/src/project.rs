//! Go project root detection

use poly_bench_dsl::Lang;
use poly_bench_traits::ProjectRootDetector;

pub struct GoDetector;

impl ProjectRootDetector for GoDetector {
    fn lang(&self) -> Lang {
        Lang::Go
    }
    fn marker_files(&self) -> &[&'static str] {
        &["go.mod"]
    }
}

pub static GO_DETECTOR: GoDetector = GoDetector;
