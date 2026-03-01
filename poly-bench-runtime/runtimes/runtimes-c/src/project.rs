//! C project root detection.

use poly_bench_dsl::Lang;
use poly_bench_traits::ProjectRootDetector;

pub struct CDetector;

impl ProjectRootDetector for CDetector {
    fn lang(&self) -> Lang {
        Lang::C
    }

    fn marker_files(&self) -> &[&'static str] {
        // Keep markers conservative to avoid false positives (e.g. workspace-level Makefiles).
        &["compile_commands.json", "CMakeLists.txt", "vcpkg.json", ".clangd"]
    }
}

pub static C_DETECTOR: CDetector = CDetector;
