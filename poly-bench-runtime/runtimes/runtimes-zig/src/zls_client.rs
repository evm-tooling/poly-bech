//! ZLS client for embedded Zig code.

use std::sync::Arc;

use once_cell::sync::OnceCell;
use poly_bench_traits::{LspClient, LspConfig};
use serde_json::{json, Value};

static ZLS_CLIENT: OnceCell<Arc<ZlsClient>> = OnceCell::new();

pub fn init_zls_client(workspace_root: &str) -> Option<Arc<ZlsClient>> {
    ZLS_CLIENT.get_or_try_init(|| ZlsClient::new(workspace_root).map(Arc::new)).ok().cloned()
}

pub fn get_zls_client() -> Option<Arc<ZlsClient>> {
    ZLS_CLIENT.get().cloned()
}

pub struct ZlsConfig;

impl LspConfig for ZlsConfig {
    const SERVER_NAME: &'static str = "zls";
    const LANGUAGE_ID: &'static str = "zig";

    fn find_executable() -> Option<String> {
        which::which("zls").ok().map(|p| p.to_string_lossy().to_string())
    }

    fn find_executable_in_workspace(workspace_root: &str) -> Option<String> {
        let bin_path = std::path::Path::new(workspace_root).join("bin").join("zls");
        if bin_path.exists() {
            bin_path.to_str().map(String::from)
        } else {
            None
        }
    }

    fn server_args() -> Vec<String> {
        vec![]
    }

    fn request_timeout_ms() -> u64 {
        4_000
    }

    fn current_dir(workspace_root: &str) -> Option<std::path::PathBuf> {
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

pub type ZlsClient = LspClient<ZlsConfig>;
