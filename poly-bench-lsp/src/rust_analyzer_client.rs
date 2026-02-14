//! rust-analyzer client for embedded Rust code
//!
//! This module manages a rust-analyzer subprocess and communicates with it
//! via the Language Server Protocol over stdin/stdout.

use std::sync::Arc;

use once_cell::sync::OnceCell;
use serde_json::{json, Value};

use super::lsp_client::{LspClient, LspConfig};

/// Global rust-analyzer client instance (lazy initialized)
static RUST_ANALYZER_CLIENT: OnceCell<Arc<RustAnalyzerClient>> = OnceCell::new();

/// Initialize the global rust-analyzer client with a workspace root
pub fn init_rust_analyzer_client(workspace_root: &str) -> Option<Arc<RustAnalyzerClient>> {
    RUST_ANALYZER_CLIENT
        .get_or_try_init(|| RustAnalyzerClient::new(workspace_root).map(Arc::new))
        .ok()
        .cloned()
}

/// Configuration for rust-analyzer
pub struct RustAnalyzerConfig;

impl LspConfig for RustAnalyzerConfig {
    const SERVER_NAME: &'static str = "rust-analyzer";
    const LANGUAGE_ID: &'static str = "rust";

    fn find_executable() -> Option<String> {
        find_rust_analyzer()
    }

    fn server_args() -> Vec<String> {
        vec![]
    }

    fn additional_capabilities() -> Value {
        json!({
            "textDocument": {
                "synchronization": {
                    "didOpen": true,
                    "didChange": true,
                    "didClose": true
                }
            },
            "workspace": {
                "workspaceFolders": true
            }
        })
    }
}

/// Type alias for the rust-analyzer client
pub type RustAnalyzerClient = LspClient<RustAnalyzerConfig>;

/// Find rust-analyzer in PATH or common locations
fn find_rust_analyzer() -> Option<String> {
    let home = std::env::var("HOME").ok()?;

    // First, check rustup toolchain directories directly (these are the real binaries, not shims)
    // This avoids issues with rustup shims requiring a default toolchain to be configured
    let rustup_home = std::env::var("RUSTUP_HOME").unwrap_or_else(|_| format!("{}/.rustup", home));
    let toolchains_dir = std::path::Path::new(&rustup_home).join("toolchains");

    if let Ok(entries) = std::fs::read_dir(&toolchains_dir) {
        // Prefer stable toolchain
        let mut toolchains: Vec<_> = entries.flatten().collect();
        toolchains.sort_by(|a, b| {
            let a_name = a.file_name();
            let b_name = b.file_name();
            let a_str = a_name.to_string_lossy();
            let b_str = b_name.to_string_lossy();
            // Sort stable first, then by name
            let a_stable = a_str.starts_with("stable");
            let b_stable = b_str.starts_with("stable");
            b_stable.cmp(&a_stable).then_with(|| b_str.cmp(&a_str))
        });

        for entry in toolchains {
            let ra_path = entry.path().join("bin/rust-analyzer");
            if ra_path.exists() {
                eprintln!("[rust-analyzer] Found in rustup toolchain: {}", ra_path.display());
                return Some(ra_path.to_string_lossy().to_string());
            }
        }
    }

    // Try which (but this might return the rustup shim which can fail)
    if let Ok(path) = which::which("rust-analyzer") {
        // Skip if it's the rustup shim (we already tried toolchains above)
        let path_str = path.to_string_lossy();
        if !path_str.contains(".cargo/bin") {
            return Some(path_str.to_string());
        }
    }

    // Check other common locations
    let candidates = [
        format!("{}/.local/bin/rust-analyzer", home),
        "/usr/local/bin/rust-analyzer".to_string(),
        "/opt/homebrew/bin/rust-analyzer".to_string(),
    ];

    for path in &candidates {
        if std::path::Path::new(path).exists() {
            return Some(path.clone());
        }
    }

    // Check VS Code / Cursor extension directories
    let extension_dirs =
        [format!("{}/.vscode/extensions", home), format!("{}/.cursor/extensions", home)];

    for ext_dir in &extension_dirs {
        if let Ok(entries) = std::fs::read_dir(ext_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.starts_with("rust-lang.rust-analyzer-") {
                    let ra_path = entry.path().join("server/rust-analyzer");
                    if ra_path.exists() {
                        return Some(ra_path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_rust_analyzer() {
        // This test just checks that find_rust_analyzer doesn't panic
        let _ = find_rust_analyzer();
    }
}
