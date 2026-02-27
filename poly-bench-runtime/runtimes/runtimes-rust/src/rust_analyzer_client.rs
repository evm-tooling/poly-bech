//! Rust-analyzer client for embedded Rust code
//!
//! This module manages a rust-analyzer subprocess and communicates with it
//! via the Language Server Protocol over stdin/stdout.

use std::sync::Arc;

use once_cell::sync::OnceCell;
use poly_bench_traits::{LspClient, LspConfig};
use serde_json::{json, Value};

/// Global rust-analyzer client instance (lazy initialized)
static RUST_ANALYZER_CLIENT: OnceCell<Arc<RustAnalyzerClient>> = OnceCell::new();

/// Initialize the global rust-analyzer client with a workspace root
pub fn init_rust_analyzer_client(workspace_root: &str) -> Option<Arc<RustAnalyzerClient>> {
    RUST_ANALYZER_CLIENT
        .get_or_try_init(|| RustAnalyzerClient::new(workspace_root).map(Arc::new))
        .ok()
        .cloned()
}

/// Get the global rust-analyzer client if initialized
pub fn get_rust_analyzer_client() -> Option<Arc<RustAnalyzerClient>> {
    RUST_ANALYZER_CLIENT.get().cloned()
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
            "experimental": {
                "serverStatusNotification": true
            }
        })
    }
}

/// Type alias for the rust-analyzer client
pub type RustAnalyzerClient = LspClient<RustAnalyzerConfig>;

/// Find rust-analyzer in PATH or common locations
fn find_rust_analyzer() -> Option<String> {
    if let Ok(path) = which::which("rust-analyzer") {
        return Some(path.to_string_lossy().to_string());
    }

    let home = std::env::var("HOME").ok()?;
    let candidates = [
        format!("{}/.cargo/bin/rust-analyzer", home),
        format!("{}/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rust-analyzer", home),
        format!("{}/.rustup/toolchains/stable-aarch64-apple-darwin/bin/rust-analyzer", home),
        format!("{}/.local/bin/rust-analyzer", home),
        "/usr/local/bin/rust-analyzer".to_string(),
        "/opt/homebrew/bin/rust-analyzer".to_string(),
    ];

    for path in &candidates {
        if std::path::Path::new(path).exists() {
            return Some(path.clone());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_rust_analyzer() {
        let _ = find_rust_analyzer();
    }
}
