//! Roslyn Language Server client for embedded C# code.
//!
//! Uses Microsoft's official roslyn-language-server (from dotnet/roslyn) via `dotnet tool install`.
//! Avoids csharp-ls version/SDK compatibility issues.

use std::{env, path::Path, sync::Arc};

use once_cell::sync::OnceCell;
use poly_bench_traits::{LspClient, LspConfig};
use serde_json::{json, Value};

static OMNISHARP_CLIENT: OnceCell<Arc<OmniSharpClient>> = OnceCell::new();

pub fn init_omnisharp_client(workspace_root: &str) -> Option<Arc<OmniSharpClient>> {
    match OMNISHARP_CLIENT.get_or_try_init(|| OmniSharpClient::new(workspace_root).map(Arc::new)) {
        Ok(client) => Some(client.clone()),
        Err(e) => {
            eprintln!(
                "[poly-bench:csharp-lsp] failed to initialize Roslyn Language Server at workspace '{}': {}",
                workspace_root, e
            );
            tracing::error!(
                "[csharp-lsp] failed to initialize Roslyn Language Server at workspace '{}': {}",
                workspace_root,
                e
            );
            None
        }
    }
}

pub fn get_omnisharp_client() -> Option<Arc<OmniSharpClient>> {
    OMNISHARP_CLIENT.get().cloned()
}

pub struct CSharpLsConfig;

impl LspConfig for CSharpLsConfig {
    const SERVER_NAME: &'static str = "C# LSP";
    const LANGUAGE_ID: &'static str = "csharp";

    fn find_executable() -> Option<String> {
        if let Ok(path) = env::var("POLYBENCH_CSHARP_LSP_BIN") {
            let trimmed = path.trim();
            if !trimmed.is_empty() && Path::new(trimmed).exists() {
                return Some(trimmed.to_string());
            }
        }
        which::which("roslyn-language-server")
            .ok()
            .or_else(|| which::which("csharp-ls").ok())
            .map(|p| p.to_string_lossy().to_string())
    }

    fn find_executable_in_workspace(workspace_root: &str) -> Option<String> {
        let workspace = Path::new(workspace_root);
        let dot_csharp_ls = workspace.join(".csharp-ls");

        // 1. Prefer roslyn-language-server in .csharp-ls (tool-path install)
        if dot_csharp_ls.is_dir() {
            let candidates = [
                dot_csharp_ls.join("roslyn-language-server"),
                dot_csharp_ls.join("roslyn-language-server.exe"),
                dot_csharp_ls.join("roslyn-language-server.cmd"),
            ];
            if let Some(p) = candidates.iter().find(|p| p.exists()) {
                return Some(p.to_string_lossy().to_string());
            }
        }

        // 2. Prefer dotnet tool run when dotnet-tools.json has csharp-ls (local install, correct paths)
        // dotnet new tool-manifest creates .config/dotnet-tools.json, not dotnet-tools.json in cwd
        let dotnet_tools = workspace.join(".config").join("dotnet-tools.json");
        let dotnet_tools = if dotnet_tools.exists() {
            Some(dotnet_tools)
        } else {
            let fallback = workspace.join("dotnet-tools.json");
            if fallback.exists() {
                Some(fallback)
            } else {
                None
            }
        };
        if let Some(dotnet_tools) = dotnet_tools {
            if let Ok(content) = std::fs::read_to_string(&dotnet_tools) {
                if content.contains("csharp-ls") {
                    if let Some(dotnet) = which::which("dotnet").ok() {
                        return Some(dotnet.to_string_lossy().to_string());
                    }
                }
            }
        }

        None
    }

    fn server_args() -> Vec<String> {
        vec!["--stdio".to_string(), "--autoLoadProjects".to_string()]
    }

    fn server_args_for_path(path: &str) -> Option<Vec<String>> {
        if path.contains("roslyn-language-server") {
            Some(vec!["--stdio".to_string(), "--autoLoadProjects".to_string()])
        } else if path.ends_with("dotnet") || path.ends_with("dotnet.exe") {
            // dotnet tool run csharp-ls (from dotnet-tools.json)
            Some(vec!["tool".to_string(), "run".to_string(), "csharp-ls".to_string(), "--".to_string()])
        } else {
            Some(vec![])
        }
    }

    fn current_dir(workspace_root: &str) -> Option<std::path::PathBuf> {
        let p = Path::new(workspace_root);
        if p.exists() && p.is_dir() {
            Some(p.to_path_buf())
        } else {
            None
        }
    }

    fn request_timeout_ms() -> u64 {
        // Keep hover/diagnostic UX responsive when csharp-ls is unavailable/misconfigured.
        4_000
    }

    fn additional_capabilities() -> Value {
        json!({})
    }
}

pub type OmniSharpClient = LspClient<CSharpLsConfig>;

#[cfg(test)]
mod tests {
    use super::*;
    use poly_bench_traits::lsp_client::parse_hover_response;
    use tower_lsp::lsp_types::HoverContents;

    #[test]
    fn test_csharp_lsp_args() {
        let _ = CSharpLsConfig::find_executable();
        assert_eq!(
            CSharpLsConfig::server_args_for_path("/tmp/.csharp-ls/roslyn-language-server"),
            Some(vec!["--stdio".to_string(), "--autoLoadProjects".to_string()])
        );
        assert_eq!(
            CSharpLsConfig::server_args_for_path("/tmp/.csharp-ls/csharp-ls"),
            Some(vec![])
        );
    }

    #[test]
    fn test_workspace_lookup() {
        let unique = format!(
            "polybench-csharp-lsp-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        );
        let base = std::env::temp_dir().join(unique);
        let tool_dir = base.join(".csharp-ls");
        std::fs::create_dir_all(&tool_dir).expect("create test tool dir");
        let tool_path = tool_dir.join("roslyn-language-server");
        std::fs::write(&tool_path, b"#!/bin/sh\nexit 0\n").expect("write fake roslyn-language-server");

        let found = CSharpLsConfig::find_executable_in_workspace(base.to_string_lossy().as_ref());
        assert_eq!(found, Some(tool_path.to_string_lossy().to_string()));

        let _ = std::fs::remove_dir_all(base);
    }

    #[test]
    fn test_parse_csharp_hover_response() {
        let payload = serde_json::json!({
            "contents": {
                "kind": "markdown",
                "value": "```csharp\nint SumBytes(byte[] data)\n```"
            },
            "range": {
                "start": { "line": 10, "character": 4 },
                "end": { "line": 10, "character": 12 }
            }
        });

        let hover = parse_hover_response(&payload)
            .expect("hover parse should succeed")
            .expect("hover should be present");

        match hover.contents {
            HoverContents::Markup(markup) => {
                assert!(markup.value.contains("SumBytes"));
            }
            _ => panic!("expected markup hover content"),
        }

        let range = hover.range.expect("expected hover range");
        assert_eq!(range.start.line, 10);
        assert_eq!(range.start.character, 4);
        assert_eq!(range.end.line, 10);
        assert_eq!(range.end.character, 12);
    }
}
