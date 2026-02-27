//! TypeScript language server client for embedded TypeScript code
//!
//! This module manages a typescript-language-server subprocess and communicates
//! with it via the Language Server Protocol over stdin/stdout.

use std::sync::Arc;

use once_cell::sync::OnceCell;
use poly_bench_traits::{LspClient, LspConfig};
use serde_json::{json, Value};

/// Global tsserver client instance (lazy initialized)
static TSSERVER_CLIENT: OnceCell<Arc<TsServerClient>> = OnceCell::new();

/// Initialize the global tsserver client with a workspace root
pub fn init_tsserver_client(workspace_root: &str) -> Option<Arc<TsServerClient>> {
    TSSERVER_CLIENT
        .get_or_try_init(|| TsServerClient::new(workspace_root).map(Arc::new))
        .ok()
        .cloned()
}

/// Get the global tsserver client if initialized
pub fn get_tsserver_client() -> Option<Arc<TsServerClient>> {
    TSSERVER_CLIENT.get().cloned()
}

/// Configuration for typescript-language-server
pub struct TsServerConfig;

impl LspConfig for TsServerConfig {
    const SERVER_NAME: &'static str = "typescript-language-server";
    const LANGUAGE_ID: &'static str = "typescript";

    fn find_executable() -> Option<String> {
        find_tsserver()
    }

    fn find_executable_in_workspace(workspace_root: &str) -> Option<String> {
        let bin_dir = std::path::Path::new(workspace_root).join("node_modules").join(".bin");
        let candidates: &[&str] = if cfg!(windows) {
            &["typescript-language-server.cmd", "typescript-language-server"]
        } else {
            &["typescript-language-server"]
        };
        for name in candidates {
            let bin_path = bin_dir.join(name);
            if bin_path.exists() {
                return bin_path.to_str().map(String::from);
            }
        }
        None
    }

    fn server_args() -> Vec<String> {
        vec!["--stdio".to_string()]
    }

    fn additional_capabilities() -> Value {
        json!({})
    }
}

/// Type alias for the TypeScript server client
pub type TsServerClient = LspClient<TsServerConfig>;

/// Find typescript-language-server in PATH or common locations
fn find_tsserver() -> Option<String> {
    if let Ok(path) = which::which("typescript-language-server") {
        return Some(path.to_string_lossy().to_string());
    }

    let home = std::env::var("HOME").ok()?;
    let candidates = [
        format!("{}/.npm-global/bin/typescript-language-server", home),
        format!("{}/.local/bin/typescript-language-server", home),
        "/usr/local/bin/typescript-language-server".to_string(),
        "/opt/homebrew/bin/typescript-language-server".to_string(),
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
    fn test_find_tsserver() {
        let _ = find_tsserver();
    }
}
