//! LSP implementation of EmbeddedDiagnosticContext
//!
//! Provides LSP clients and setup to diagnostic providers from runtimes.

use std::{path::Path, sync::Arc};

use poly_bench_lsp_traits::{EmbeddedDiagnosticContext, EmbeddedLspClient, LspDiagnostic};

use crate::{gopls_client, pyright_client, rust_analyzer_client, tsserver_client};

/// Adapter to make GoplsClient implement EmbeddedLspClient
pub struct GoplsClientAdapter(pub Arc<gopls_client::GoplsClient>);

impl EmbeddedLspClient for GoplsClientAdapter {
    fn did_change(&self, uri: &str, content: &str, version: i32) -> Result<(), String> {
        self.0.did_change(uri, content, version)
    }

    fn request_diagnostics(&self, uri: &str) -> Result<Vec<LspDiagnostic>, String> {
        self.0.request_diagnostics(uri).map(|diags| {
            diags
                .into_iter()
                .map(|d| LspDiagnostic {
                    message: d.message,
                    severity: d.severity,
                    start_line: d.start_line,
                    start_character: d.start_character,
                    end_line: d.end_line,
                    end_character: d.end_character,
                    code: d.code,
                })
                .collect()
        })
    }

    fn hover(
        &self,
        uri: &str,
        line: u32,
        character: u32,
    ) -> Result<Option<tower_lsp::lsp_types::Hover>, String> {
        self.0.hover(uri, line, character)
    }
}

/// Adapter to make TsServerClient implement EmbeddedLspClient
pub struct TsserverClientAdapter(pub Arc<tsserver_client::TsServerClient>);

impl EmbeddedLspClient for TsserverClientAdapter {
    fn did_change(&self, uri: &str, content: &str, version: i32) -> Result<(), String> {
        self.0.did_change(uri, content, version)
    }

    fn request_diagnostics(&self, uri: &str) -> Result<Vec<LspDiagnostic>, String> {
        self.0.request_diagnostics(uri).map(|diags| {
            diags
                .into_iter()
                .map(|d| LspDiagnostic {
                    message: d.message,
                    severity: d.severity,
                    start_line: d.start_line,
                    start_character: d.start_character,
                    end_line: d.end_line,
                    end_character: d.end_character,
                    code: d.code,
                })
                .collect()
        })
    }

    fn hover(
        &self,
        uri: &str,
        line: u32,
        character: u32,
    ) -> Result<Option<tower_lsp::lsp_types::Hover>, String> {
        self.0.hover(uri, line, character)
    }
}

/// Adapter to make RustAnalyzerClient implement EmbeddedLspClient
pub struct RustAnalyzerClientAdapter(pub Arc<rust_analyzer_client::RustAnalyzerClient>);

impl EmbeddedLspClient for RustAnalyzerClientAdapter {
    fn did_change(&self, uri: &str, content: &str, version: i32) -> Result<(), String> {
        self.0.did_change(uri, content, version)
    }

    fn request_diagnostics(&self, uri: &str) -> Result<Vec<LspDiagnostic>, String> {
        self.0.request_diagnostics(uri).map(|diags| {
            diags
                .into_iter()
                .map(|d| LspDiagnostic {
                    message: d.message,
                    severity: d.severity,
                    start_line: d.start_line,
                    start_character: d.start_character,
                    end_line: d.end_line,
                    end_character: d.end_character,
                    code: d.code,
                })
                .collect()
        })
    }

    fn hover(
        &self,
        uri: &str,
        line: u32,
        character: u32,
    ) -> Result<Option<tower_lsp::lsp_types::Hover>, String> {
        self.0.hover(uri, line, character)
    }
}

/// Adapter to make PyrightClient implement EmbeddedLspClient
pub struct PyrightClientAdapter(pub Arc<pyright_client::PyrightClient>);

impl EmbeddedLspClient for PyrightClientAdapter {
    fn did_change(&self, uri: &str, content: &str, version: i32) -> Result<(), String> {
        self.0.did_change(uri, content, version)
    }

    fn request_diagnostics(&self, uri: &str) -> Result<Vec<LspDiagnostic>, String> {
        self.0.request_diagnostics(uri).map(|diags| {
            diags
                .into_iter()
                .map(|d| LspDiagnostic {
                    message: d.message,
                    severity: d.severity,
                    start_line: d.start_line,
                    start_character: d.start_character,
                    end_line: d.end_line,
                    end_character: d.end_character,
                    code: d.code,
                })
                .collect()
        })
    }

    fn hover(
        &self,
        uri: &str,
        line: u32,
        character: u32,
    ) -> Result<Option<tower_lsp::lsp_types::Hover>, String> {
        self.0.hover(uri, line, character)
    }
}

/// LSP implementation of EmbeddedDiagnosticContext
pub struct LspEmbeddedDiagnosticContext {
    module_root: String,
}

impl LspEmbeddedDiagnosticContext {
    pub fn new(module_root: &str) -> Self {
        Self { module_root: module_root.to_string() }
    }
}

impl EmbeddedDiagnosticContext for LspEmbeddedDiagnosticContext {
    fn get_go_client(&self, module_root: &str) -> Option<Arc<dyn EmbeddedLspClient>> {
        if module_root != self.module_root {
            return None;
        }
        let _ = gopls_client::init_gopls_client(module_root);
        gopls_client::get_gopls_client()
            .map(|c| Arc::new(GoplsClientAdapter(c)) as Arc<dyn EmbeddedLspClient>)
    }

    fn ensure_tsconfig(&self, module_root: &str) {
        let tsconfig_path = Path::new(module_root).join("tsconfig.json");
        if !tsconfig_path.exists() {
            let tsconfig_content = r#"{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "moduleResolution": "node",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "noEmit": true
  }
}
"#;
            let _ = std::fs::write(&tsconfig_path, tsconfig_content);
        }
    }

    fn ensure_ready(&self, lang: poly_bench_syntax::Lang, module_root: &str) {
        if module_root != self.module_root {
            return;
        }
        match lang {
            poly_bench_syntax::Lang::Go => {
                let _ = gopls_client::init_gopls_client(module_root);
            }
            poly_bench_syntax::Lang::TypeScript => {
                self.ensure_tsconfig(module_root);
            }
            poly_bench_syntax::Lang::Rust => {
                let src_dir = Path::new(module_root).join("src");
                let _ = std::fs::create_dir_all(&src_dir);
            }
            poly_bench_syntax::Lang::Python => {
                let _ = pyright_client::init_pyright_client(module_root);
            }
            _ => {}
        }
    }
}
