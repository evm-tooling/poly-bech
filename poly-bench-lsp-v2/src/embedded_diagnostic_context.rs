//! LSP implementation of EmbeddedDiagnosticContext
//!
//! Provides LSP clients and setup to diagnostic providers from runtimes.
//! Clients are obtained via the poly-bench runtime registry.

use std::{path::Path, sync::Arc};

use poly_bench_lsp_traits::{EmbeddedDiagnosticContext, EmbeddedLspClient};
use poly_bench_runtime::{get_embedded_lsp_client, init_embedded_lsp_client};
use poly_bench_syntax::Lang;

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
        init_embedded_lsp_client(poly_bench_dsl::Lang::Go, module_root)
            .or_else(|| get_embedded_lsp_client(poly_bench_dsl::Lang::Go))
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

    fn ensure_ready(&self, lang: Lang, module_root: &str) {
        if module_root != self.module_root {
            return;
        }
        let dsl_lang = poly_bench_lsp_traits::syntax_lang_to_dsl(lang);
        match lang {
            Lang::Go => {
                let _ = init_embedded_lsp_client(dsl_lang, module_root);
            }
            Lang::TypeScript => {
                self.ensure_tsconfig(module_root);
            }
            Lang::Rust => {
                let src_dir = Path::new(module_root).join("src");
                let _ = std::fs::create_dir_all(&src_dir);
            }
            Lang::Python => {
                let _ = init_embedded_lsp_client(dsl_lang, module_root);
            }
            _ => {}
        }
    }
}
