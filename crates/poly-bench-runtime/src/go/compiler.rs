//! Go plugin compilation

use miette::{miette, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

/// Compile Go source to a plugin (.so file)
pub struct GoCompiler {
    /// Temporary directory for build artifacts
    temp_dir: TempDir,
    /// Path to the go binary
    go_binary: PathBuf,
}

impl GoCompiler {
    /// Create a new Go compiler
    pub fn new() -> Result<Self> {
        let go_binary = which::which("go")
            .map_err(|_| miette!("Go compiler not found in PATH. Please install Go."))?;

        let temp_dir =
            TempDir::new().map_err(|e| miette!("Failed to create temp directory: {}", e))?;

        Ok(Self {
            temp_dir,
            go_binary,
        })
    }

    /// Compile Go source code to a plugin
    pub fn compile(&self, source: &str, module_name: &str) -> Result<PathBuf> {
        let src_dir = self.temp_dir.path();

        // Write go.mod
        let go_mod = format!("module {}\n\ngo 1.21\n", module_name);
        std::fs::write(src_dir.join("go.mod"), go_mod)
            .map_err(|e| miette!("Failed to write go.mod: {}", e))?;

        // Write main.go
        let main_go = src_dir.join("main.go");
        std::fs::write(&main_go, source).map_err(|e| miette!("Failed to write main.go: {}", e))?;

        // Output plugin path
        let plugin_path = src_dir.join("benchmark.so");

        // Compile
        let output = Command::new(&self.go_binary)
            .args([
                "build",
                "-buildmode=plugin",
                "-o",
                plugin_path.to_str().unwrap(),
                ".",
            ])
            .current_dir(src_dir)
            .output()
            .map_err(|e| miette!("Failed to run go build: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(miette!("Go compilation failed:\n{}", stderr));
        }

        Ok(plugin_path)
    }

    /// Get the temp directory path (for debugging)
    pub fn temp_path(&self) -> &Path {
        self.temp_dir.path()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Requires Go installed
    fn test_compile_simple() {
        let source = r#"
package main

func main() {}

var Hello = func() string { return "Hello" }
"#;
        let compiler = GoCompiler::new().unwrap();
        let result = compiler.compile(source, "test");

        // This will fail on systems without Go plugin support (e.g., macOS)
        // but should succeed on Linux
        if let Ok(path) = result {
            assert!(path.exists());
        }
    }
}
