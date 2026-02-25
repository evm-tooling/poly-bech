//! Project management module for poly-bench
//!
//! This module handles:
//! - Project initialization (`poly-bench init`)
//! - Project discovery (finding `polybench.toml`)
//! - Dependency management (`poly-bench add`, `poly-bench install`)
//! - Benchmark file scaffolding (`poly-bench new`)

pub mod build;
pub mod deps;
pub mod detectors;
pub mod error;
pub mod init;
pub mod manifest;
pub mod templates;
pub mod terminal;

pub use detectors::{detect_from_markers, get_detector, ProjectRootDetector};

use miette::Result;
use std::path::{Path, PathBuf};

pub use manifest::Manifest;

/// The manifest filename
pub const MANIFEST_FILENAME: &str = "polybench.toml";

/// The default benchmarks directory name
pub const BENCHMARKS_DIR: &str = "benchmarks";

/// Directory under .polybench for per-runtime env (go.mod, package.json, deps, harness)
pub const RUNTIME_ENV_DIR: &str = ".polybench/runtime-env";
/// Go runtime env subdir (go.mod, go.sum, generated bench code)
pub const RUNTIME_ENV_GO: &str = "go";
/// TypeScript/Node runtime env subdir (package.json, node_modules, generated bench code)
pub const RUNTIME_ENV_TS: &str = "ts";
/// Rust runtime env subdir (Cargo.toml, Cargo.lock, generated bench code)
pub const RUNTIME_ENV_RUST: &str = "rust";
pub const RUNTIME_ENV_PYTHON: &str = "python";

/// Path to the Go runtime env for a project (where go.mod and deps live)
pub fn runtime_env_go(project_root: &Path) -> PathBuf {
    project_root.join(RUNTIME_ENV_DIR).join(RUNTIME_ENV_GO)
}

/// Path to the TypeScript runtime env for a project
pub fn runtime_env_ts(project_root: &Path) -> PathBuf {
    project_root.join(RUNTIME_ENV_DIR).join(RUNTIME_ENV_TS)
}

/// Path to the Rust runtime env for a project
pub fn runtime_env_rust(project_root: &Path) -> PathBuf {
    project_root.join(RUNTIME_ENV_DIR).join(RUNTIME_ENV_RUST)
}

/// Path to the Python runtime env for a project
pub fn runtime_env_python(project_root: &Path) -> PathBuf {
    project_root.join(RUNTIME_ENV_DIR).join(RUNTIME_ENV_PYTHON)
}

/// True if path looks like a runtime-env root (e.g. .../runtime-env/go)
pub fn is_runtime_env_root(path: &Path) -> bool {
    path.as_os_str().to_string_lossy().contains("runtime-env")
}

/// Find the project root by searching for polybench.toml
///
/// Starts from `start_path` and walks up the directory tree until
/// a `polybench.toml` file is found.
pub fn find_project_root(start_path: &Path) -> Option<PathBuf> {
    let start = if start_path.is_file() { start_path.parent()? } else { start_path };

    let mut current = start.canonicalize().ok()?;

    loop {
        if current.join(MANIFEST_FILENAME).exists() {
            return Some(current);
        }

        if !current.pop() {
            return None;
        }
    }
}

/// Check if we're inside a poly-bench project
pub fn is_inside_project(path: &Path) -> bool {
    find_project_root(path).is_some()
}

/// Load the manifest from a project root
pub fn load_manifest(project_root: &Path) -> Result<Manifest> {
    let manifest_path = project_root.join(MANIFEST_FILENAME);
    manifest::load(&manifest_path)
}

/// Save the manifest to a project root
pub fn save_manifest(project_root: &Path, manifest: &Manifest) -> Result<()> {
    let manifest_path = project_root.join(MANIFEST_FILENAME);
    manifest::save(&manifest_path, manifest)
}

/// Get the benchmarks directory for a project
pub fn benchmarks_dir(project_root: &Path) -> PathBuf {
    project_root.join(BENCHMARKS_DIR)
}

/// Find all .bench files in a project's benchmarks directory
pub fn find_bench_files(project_root: &Path) -> Result<Vec<PathBuf>> {
    let bench_dir = benchmarks_dir(project_root);

    if !bench_dir.exists() {
        return Ok(vec![]);
    }

    let mut files = Vec::new();

    for entry in std::fs::read_dir(&bench_dir)
        .map_err(|e| miette::miette!("Failed to read benchmarks directory: {}", e))?
    {
        let entry = entry.map_err(|e| miette::miette!("Failed to read directory entry: {}", e))?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "bench" {
                    files.push(path);
                }
            }
        }
    }

    // Sort for consistent ordering
    files.sort();

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_find_project_root() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create a nested directory structure
        let nested = root.join("a").join("b").join("c");
        std::fs::create_dir_all(&nested).unwrap();

        // No manifest yet
        assert!(find_project_root(&nested).is_none());

        // Create manifest at root
        std::fs::write(root.join(MANIFEST_FILENAME), "").unwrap();

        // Now should find it
        let found = find_project_root(&nested).unwrap();
        assert_eq!(found, root.canonicalize().unwrap());
    }
}
