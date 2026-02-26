//! C project root detection.

use poly_bench_dsl::Lang;
use poly_bench_runtime_traits::ProjectRootDetector;

pub struct CDetector;

impl ProjectRootDetector for CDetector {
    fn lang(&self) -> Lang {
        Lang::C
    }

    fn marker_files(&self) -> &[&'static str] {
        &["compile_commands.json", "CMakeLists.txt", "Makefile", ".clangd", ".clang-format"]
    }
}

pub static C_DETECTOR: CDetector = CDetector;
