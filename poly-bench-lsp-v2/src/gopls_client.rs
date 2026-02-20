//! Gopls client for embedded Go code
//!
//! This module manages a gopls subprocess and communicates with it
//! via the Language Server Protocol over stdin/stdout.

use std::sync::Arc;

use once_cell::sync::OnceCell;
use serde_json::{json, Value};

use crate::lsp_client::{LspClient, LspConfig};

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
    // Try which first
    if let Ok(path) = which::which("gopls") {
        return Some(path.to_string_lossy().to_string());
    }

    // Check common Go bin locations
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
        // This test just checks that find_gopls doesn't panic
        let _ = find_gopls();
    }
}
