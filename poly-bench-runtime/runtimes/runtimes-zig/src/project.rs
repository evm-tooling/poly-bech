//! Zig project root detection.

use poly_bench_dsl::Lang;
use poly_bench_traits::ProjectRootDetector;

pub struct ZigDetector;

impl ProjectRootDetector for ZigDetector {
    fn lang(&self) -> Lang {
        Lang::Zig
    }

    fn marker_files(&self) -> &[&'static str] {
        &["build.zig", "build.zig.zon"]
    }
}

pub static ZIG_DETECTOR: ZigDetector = ZigDetector;
