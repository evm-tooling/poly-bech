//! TypeScript language server client for embedded TypeScript code
//!
//! This module manages a typescript-language-server subprocess and communicates
//! with it via the Language Server Protocol over stdin/stdout.

use std::sync::Arc;
use std::process::Command;

use once_cell::sync::OnceCell;
use serde_json::{json, Value};

use super::lsp_client::{LspClient, LspConfig};

/// Global tsserver client instance (lazy initialized)
static TSSERVER_CLIENT: OnceCell<Arc<TsServerClient>> = OnceCell::new();

/// Initialize the global tsserver client with a workspace root
pub fn init_tsserver_client(workspace_root: &str) -> Option<Arc<TsServerClient>> {
    TSSERVER_CLIENT.get_or_try_init(|| {
        TsServerClient::new(workspace_root).map(Arc::new)
    }).ok().cloned()
}

/// Configuration for typescript-language-server
pub struct TsServerConfig;

impl LspConfig for TsServerConfig {
    const SERVER_NAME: &'static str = "typescript-language-server";
    const LANGUAGE_ID: &'static str = "typescript";
    
    fn find_executable() -> Option<String> {
        find_ts_language_server()
    }
    
    fn server_args() -> Vec<String> {
        vec!["--stdio".to_string()]
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

/// Type alias for the TypeScript server client
pub type TsServerClient = LspClient<TsServerConfig>;

/// Find typescript-language-server executable
fn find_ts_language_server() -> Option<String> {
    // Check common locations
    let candidates = [
        "typescript-language-server",
        "/usr/local/bin/typescript-language-server",
        "/opt/homebrew/bin/typescript-language-server",
    ];

    for candidate in candidates {
        if let Ok(output) = Command::new("which").arg(candidate).output() {
            if output.status.success() {
                let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path.is_empty() {
                    return Some(path);
                }
            }
        }
    }

    // Check if available via npx
    if let Ok(output) = Command::new("npx")
        .args(["--no", "typescript-language-server", "--version"])
        .output()
    {
        if output.status.success() {
            return Some("npx".to_string());
        }
    }

    // Check home directory npm bin
    if let Ok(home) = std::env::var("HOME") {
        let npm_bin = format!("{}/.npm-global/bin/typescript-language-server", home);
        if std::path::Path::new(&npm_bin).exists() {
            return Some(npm_bin);
        }

        // Check nvm locations
        let nvm_dir = format!("{}/.nvm/versions/node", home);
        if let Ok(entries) = std::fs::read_dir(&nvm_dir) {
            for entry in entries.flatten() {
                let ts_server = entry.path().join("bin/typescript-language-server");
                if ts_server.exists() {
                    return Some(ts_server.to_string_lossy().to_string());
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
    fn test_find_ts_language_server() {
        // This test just checks that find_ts_language_server doesn't panic
        let _ = find_ts_language_server();
    }
}
