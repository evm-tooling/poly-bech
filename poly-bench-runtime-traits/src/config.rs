//! Runtime configuration for project roots

use std::path::PathBuf;

/// Configuration passed to runtime factories when creating a runtime.
#[derive(Debug, Clone, Default)]
pub struct RuntimeConfig {
    /// Go module root (directory containing go.mod)
    pub go_root: Option<PathBuf>,
    /// Node.js project root (directory containing package.json or node_modules)
    pub node_root: Option<PathBuf>,
    /// Rust project root (directory containing Cargo.toml)
    pub rust_root: Option<PathBuf>,
    /// Python project root (directory containing requirements.txt or pyproject.toml)
    pub python_root: Option<PathBuf>,
    /// C# project root (directory containing .csproj/.sln)
    pub csharp_root: Option<PathBuf>,
}
