//! Project root detection for language runtimes
//!
//! Walks up the directory tree to find project roots by marker files
//! (e.g. go.mod, package.json, Cargo.toml).

use poly_bench_dsl::Lang;
use std::path::{Path, PathBuf};

/// Detector for finding a language's project root
pub trait ProjectRootDetector: Send + Sync {
    /// The language this detector handles
    fn lang(&self) -> Lang;

    /// Marker files that indicate a project root (first found wins)
    fn marker_files(&self) -> &[&'static str];

    /// Walk up from start until a directory containing any marker file is found
    fn detect(&self, start: &Path) -> Option<PathBuf> {
        detect_from_markers(start, self.marker_files())
    }
}

/// Walk up from start until a directory containing any of the marker files is found
pub fn detect_from_markers(start: &Path, marker_files: &[&'static str]) -> Option<PathBuf> {
    let mut current = if start.is_file() { start.parent()? } else { start }.canonicalize().ok()?;

    loop {
        for marker in marker_files {
            if current.join(marker).exists() {
                return Some(current);
            }
        }
        if !current.pop() {
            return None;
        }
    }
}
