//! Rust project root detection

use poly_bench_dsl::Lang;
use poly_bench_runtime_traits::ProjectRootDetector;

pub struct RustDetector;

impl ProjectRootDetector for RustDetector {
    fn lang(&self) -> Lang {
        Lang::Rust
    }
    fn marker_files(&self) -> &[&'static str] {
        &["Cargo.toml"]
    }
}

pub static RUST_DETECTOR: RustDetector = RustDetector;
