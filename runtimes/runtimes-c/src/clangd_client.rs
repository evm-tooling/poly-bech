//! clangd client for embedded C code.

use std::sync::Arc;

use once_cell::sync::OnceCell;
use poly_bench_lsp_traits::{LspClient, LspConfig};
use serde_json::{json, Value};

static CLANGD_CLIENT: OnceCell<Arc<ClangdClient>> = OnceCell::new();

pub fn init_clangd_client(workspace_root: &str) -> Option<Arc<ClangdClient>> {
    CLANGD_CLIENT.get_or_try_init(|| ClangdClient::new(workspace_root).map(Arc::new)).ok().cloned()
}

pub fn get_clangd_client() -> Option<Arc<ClangdClient>> {
    CLANGD_CLIENT.get().cloned()
}

pub struct ClangdConfig;

impl LspConfig for ClangdConfig {
    const SERVER_NAME: &'static str = "clangd";
    const LANGUAGE_ID: &'static str = "c";

    fn find_executable() -> Option<String> {
        which::which("clangd").ok().map(|p| p.to_string_lossy().to_string())
    }

    fn server_args() -> Vec<String> {
        vec![
            "--background-index".to_string(),
            "--clang-tidy".to_string(),
            "--header-insertion=never".to_string(),
        ]
    }

    fn request_timeout_ms() -> u64 {
        4_000
    }

    fn additional_capabilities() -> Value {
        json!({})
    }
}

pub type ClangdClient = LspClient<ClangdConfig>;
