//! Python project root detection

use poly_bench_dsl::Lang;
use poly_bench_traits::ProjectRootDetector;

pub struct PythonDetector;

impl ProjectRootDetector for PythonDetector {
    fn lang(&self) -> Lang {
        Lang::Python
    }
    fn marker_files(&self) -> &[&'static str] {
        &["requirements.txt", "pyproject.toml"]
    }
}

pub static PYTHON_DETECTOR: PythonDetector = PythonDetector;
