//! LSP implementation of EmbeddedDiagnosticContext
//!
//! Provides LSP clients and setup to diagnostic providers from runtimes.
//! Clients are obtained via the poly-bench runtime registry.

use std::sync::Arc;

use poly_bench_runtime::{
    get_embedded_diagnostic_setup, get_embedded_lsp_client, init_embedded_lsp_client,
};
use poly_bench_syntax::Lang;
use poly_bench_traits::{EmbeddedDiagnosticContext, EmbeddedLspClient};

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
    fn get_lsp_client(
        &self,
        lang: poly_bench_dsl::Lang,
        module_root: &str,
    ) -> Option<Arc<dyn EmbeddedLspClient>> {
        if module_root != self.module_root {
            return None;
        }
        init_embedded_lsp_client(lang, module_root).or_else(|| get_embedded_lsp_client(lang))
    }

    fn ensure_tsconfig(&self, module_root: &str) {
        let tsconfig_path = std::path::Path::new(module_root).join("tsconfig.json");
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
        let dsl_lang = poly_bench_traits::syntax_lang_to_dsl(lang);
        let _ = init_embedded_lsp_client(dsl_lang, module_root);
        if let Some(setup) = get_embedded_diagnostic_setup(dsl_lang) {
            setup.prepare_environment(module_root);
        }
    }
}
