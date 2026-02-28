//! LSP implementation of EmbeddedDiagnosticContext
//!
//! Provides LSP clients and setup to diagnostic providers from runtimes.
//! Clients are obtained via the poly-bench runtime registry.

use std::{path::Path, sync::Arc};

use poly_bench_project::{find_project_root, load_manifest};
use poly_bench_runtime::{
    get_embedded_diagnostic_setup, get_embedded_lsp_client, init_embedded_lsp_client,
};
use poly_bench_syntax::Lang;
use poly_bench_traits::{EmbeddedDiagnosticContext, EmbeddedLspClient};

/// Returns true if we should skip starting the LSP for this language.
/// Only skips when we're in a poly-bench project AND the manifest doesn't include this language.
/// Standalone .bench files (no project) are never skipped.
pub fn should_skip_embedded_lsp_for_lang(
    bench_path: &Path,
    dsl_lang: poly_bench_dsl::Lang,
) -> bool {
    let start = bench_path.parent().unwrap_or(bench_path);
    let Some(project_root) = find_project_root(start) else {
        return false; // Not in a project - don't skip
    };
    let Ok(manifest) = load_manifest(&project_root) else {
        return false; // Can't load manifest - don't skip
    };
    !manifest.has_runtime(dsl_lang)
}

/// LSP implementation of EmbeddedDiagnosticContext
pub struct LspEmbeddedDiagnosticContext {
    module_root: String,
    bench_path: String,
}

impl LspEmbeddedDiagnosticContext {
    pub fn new(module_root: &str, bench_path: &str) -> Self {
        Self { module_root: module_root.to_string(), bench_path: bench_path.to_string() }
    }
}

impl EmbeddedDiagnosticContext for LspEmbeddedDiagnosticContext {
    fn get_lsp_client(
        &self,
        lang: poly_bench_dsl::Lang,
        module_root: &str,
    ) -> Option<Arc<dyn EmbeddedLspClient>> {
        if should_skip_embedded_lsp_for_lang(Path::new(&self.bench_path), lang) {
            return None;
        }
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
        if should_skip_embedded_lsp_for_lang(Path::new(&self.bench_path), dsl_lang) {
            return;
        }
        let _ = init_embedded_lsp_client(dsl_lang, module_root);
        if let Some(setup) = get_embedded_diagnostic_setup(dsl_lang) {
            setup.prepare_environment(module_root);
        }
    }
}
