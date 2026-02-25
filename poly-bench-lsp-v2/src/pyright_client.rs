//! Pyright/pylsp client for embedded Python code
//!
//! This module manages a pyright or pylsp subprocess and communicates with it
//! via the Language Server Protocol over stdin/stdout.
//!
//! Tries pyright first (pip install pyright / npm install pyright), then falls
//! back to pylsp (pip install python-lsp-server) if pyright is not available.

use std::sync::Arc;

use once_cell::sync::OnceCell;
use serde_json::{json, Value};

use crate::lsp_client::{LspClient, LspConfig};

/// Cache for (executable path, server args) - pyright uses --langserver, pylsp uses []
static PYTHON_LSP_CACHE: OnceCell<(String, Vec<String>)> = OnceCell::new();

/// Global pyright/pylsp client instance (lazy initialized)
static PYRIGHT_CLIENT: OnceCell<Arc<PyrightClient>> = OnceCell::new();

/// Initialize the global Python LSP client with a workspace root
pub fn init_pyright_client(workspace_root: &str) -> Option<Arc<PyrightClient>> {
    PYRIGHT_CLIENT
        .get_or_try_init(|| PyrightClient::new(workspace_root).map(Arc::new))
        .ok()
        .cloned()
}

/// Get the global Python LSP client if initialized
pub fn get_pyright_client() -> Option<Arc<PyrightClient>> {
    PYRIGHT_CLIENT.get().cloned()
}

/// Configuration for pyright or pylsp (tries pyright first, then pylsp)
pub struct PyrightConfig;

impl LspConfig for PyrightConfig {
    const SERVER_NAME: &'static str = "pyright";
    const LANGUAGE_ID: &'static str = "python";

    fn find_executable() -> Option<String> {
        find_python_lsp().map(|(path, _)| path.clone())
    }

    fn find_executable_in_workspace(workspace_root: &str) -> Option<String> {
        // Prefer pyright-langserver from the poly-bench venv (pip pyright package).
        // The pip "pyright" package provides pyright-langserver for LSP; "pyright" is CLI-only.
        let venv_bin = std::path::Path::new(workspace_root).join(".venv").join("bin");
        let langserver = venv_bin.join("pyright-langserver");
        if langserver.exists() {
            return Some(langserver.to_string_lossy().to_string());
        }
        // Fallback to pyright (npm version uses --langserver)
        let pyright = venv_bin.join("pyright");
        if pyright.exists() {
            return Some(pyright.to_string_lossy().to_string());
        }
        None
    }

    fn server_args() -> Vec<String> {
        find_python_lsp().map(|(_, args)| args.clone()).unwrap_or_default()
    }

    fn server_args_for_path(path: &str) -> Option<Vec<String>> {
        // pip pyright package: pyright-langserver uses --stdio (not --langserver)
        if path.contains("pyright-langserver") {
            return Some(vec!["--stdio".to_string()]);
        }
        None
    }

    fn request_timeout_ms() -> u64 {
        // Pyright can take 10–15s to initialize on first run (indexing, venv resolution).
        15_000
    }

    fn current_dir(workspace_root: &str) -> Option<std::path::PathBuf> {
        // Pyright must run from the python module root (e.g. .polybench/runtime-env/python)
        // so it finds the venv, pyrightconfig.json, and resolves imports correctly.
        let p = std::path::Path::new(workspace_root);
        if p.exists() && p.is_dir() {
            Some(p.to_path_buf())
        } else {
            None
        }
    }

    fn additional_capabilities() -> Value {
        json!({})
    }
}

/// Type alias for the Python LSP client (pyright or pylsp)
pub type PyrightClient = LspClient<PyrightConfig>;

/// Find pyright or pylsp in PATH or common locations.
/// Returns (path, args) - pyright needs ["--langserver"], pylsp uses [].
fn find_python_lsp() -> Option<&'static (String, Vec<String>)> {
    PYTHON_LSP_CACHE
        .get_or_try_init(|| {
            // Try pyright first (pip install pyright or npm install pyright)
            if let Ok(path) = which::which("pyright") {
                return Ok((path.to_string_lossy().to_string(), vec!["--langserver".to_string()]));
            }

            // Fallback to pylsp (pip install python-lsp-server)
            if let Ok(path) = which::which("pylsp") {
                return Ok((path.to_string_lossy().to_string(), vec![]));
            }

            // Check common locations
            let home = std::env::var("HOME").map_err(|_| ())?;
            let candidates: [(String, Vec<String>); 4] = [
                (format!("{}/.local/bin/pyright", home), vec!["--langserver".to_string()]),
                (format!("{}/.local/bin/pylsp", home), vec![]),
                (format!("{}/.npm-global/bin/pyright", home), vec!["--langserver".to_string()]),
                (format!("{}/.nvm/current/bin/pyright", home), vec!["--langserver".to_string()]),
            ];

            for (path, args) in candidates {
                if std::path::Path::new(&path).exists() {
                    return Ok((path, args));
                }
            }

            Err(())
        })
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_python_lsp() {
        // This test just checks that find_python_lsp doesn't panic
        let _ = find_python_lsp();
    }
}
