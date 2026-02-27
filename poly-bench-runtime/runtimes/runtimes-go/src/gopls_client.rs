//! Gopls client for embedded Go code
//!
//! This module manages a gopls subprocess and communicates with it
//! via the Language Server Protocol over stdin/stdout.

use std::sync::Arc;

use once_cell::sync::OnceCell;
use poly_bench_traits::{LspClient, LspConfig};
use serde_json::{json, Value};

/// Global gopls client instance (lazy initialized)
static GOPLS_CLIENT: OnceCell<Arc<GoplsClient>> = OnceCell::new();

/// Initialize the global gopls client with a workspace root
pub fn init_gopls_client(workspace_root: &str) -> Option<Arc<GoplsClient>> {
    GOPLS_CLIENT.get_or_try_init(|| GoplsClient::new(workspace_root).map(Arc::new)).ok().cloned()
}

/// Get the global gopls client if initialized
pub fn get_gopls_client() -> Option<Arc<GoplsClient>> {
    GOPLS_CLIENT.get().cloned()
}

/// Configuration for gopls
pub struct GoplsConfig;

impl LspConfig for GoplsConfig {
    const SERVER_NAME: &'static str = "gopls";
    const LANGUAGE_ID: &'static str = "go";

    fn find_executable() -> Option<String> {
        find_gopls()
    }

    fn find_executable_in_workspace(workspace_root: &str) -> Option<String> {
        let bin_name = if cfg!(windows) { "gopls.exe" } else { "gopls" };
        let bin_path = std::path::Path::new(workspace_root).join("bin").join(bin_name);
        if bin_path.exists() {
            bin_path.to_str().map(String::from)
        } else {
            None
        }
    }

    fn server_args() -> Vec<String> {
        vec!["serve".to_string()]
    }

    fn additional_capabilities() -> Value {
        json!({})
    }
}

/// Type alias for the gopls client
pub type GoplsClient = LspClient<GoplsConfig>;

/// Find gopls in PATH or common locations
fn find_gopls() -> Option<String> {
    if let Ok(path) = which::which("gopls") {
        return Some(path.to_string_lossy().to_string());
    }

    let home = std::env::var("HOME").ok()?;
    let candidates = [
        format!("{}/go/bin/gopls", home),
        format!("{}/.local/bin/gopls", home),
        "/usr/local/go/bin/gopls".to_string(),
        "/opt/homebrew/bin/gopls".to_string(),
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
    fn test_find_gopls() {
        let _ = find_gopls();
    }
}
