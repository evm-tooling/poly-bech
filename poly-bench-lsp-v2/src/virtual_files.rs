//! Virtual file generation for language server integration
//!
//! This module creates virtual Go/TypeScript/Rust files from embedded code blocks
//! in .bench files, and provides position translation between the
//! .bench file and the virtual files.
//!
//! Virtual file builders are provided by runtimes via the registry.

use std::{path::Path, sync::Arc};

use dashmap::DashMap;
use poly_bench_dsl::Lang as DslLang;
use poly_bench_lsp_traits::{
    syntax_lang_to_dsl, VirtualFile, VirtualFileBuilder, VirtualFileParams,
};
use poly_bench_runtime::{
    get_embedded_diagnostic_setup as get_registry_setup,
    get_virtual_file_builder as get_registry_builder,
};
use poly_bench_syntax::Lang;

use crate::{embedded::EmbeddedBlock, embedded_diagnostic_context::LspEmbeddedDiagnosticContext};

// Re-export for compatibility
pub use poly_bench_lsp_traits::SectionMapping;

/// Get the virtual file builder for a language (syntax Lang)
pub fn get_virtual_file_builder(lang: Lang) -> Option<&'static dyn VirtualFileBuilder> {
    get_registry_builder(syntax_lang_to_dsl(lang))
}

fn cache_key(bench_uri: &str, lang: DslLang) -> String {
    format!("{}\0{}", bench_uri, format!("{:?}", lang))
}

fn setup_key(module_root: &str, dsl_lang: DslLang) -> String {
    format!("{}\0{:?}", module_root, dsl_lang)
}

/// Registry-based virtual file managers
pub struct VirtualFileManagers {
    files: DashMap<String, Arc<dyn VirtualFile>>,
    setup_initialized: DashMap<String, ()>,
}

impl VirtualFileManagers {
    pub fn new() -> Self {
        Self { files: DashMap::new(), setup_initialized: DashMap::new() }
    }

    /// Get or create a virtual file for the given language
    pub fn get_or_create(
        &self,
        bench_uri: &str,
        bench_path: &str,
        lang: Lang,
        blocks: &[&EmbeddedBlock],
        fixture_names: &[String],
        version: i32,
        module_root: &str,
    ) -> Option<Arc<dyn VirtualFile>> {
        let dsl_lang = syntax_lang_to_dsl(lang);
        let builder = get_registry_builder(dsl_lang)?;

        // Language-specific setup via registry
        if let Some(setup) = get_registry_setup(dsl_lang) {
            let key = setup_key(module_root, dsl_lang);
            if !self.setup_initialized.contains_key(&key) {
                let ctx = LspEmbeddedDiagnosticContext::new(module_root);
                setup.prepare(module_root, &ctx);
                self.setup_initialized.insert(key, ());
            }
        }

        let key = cache_key(bench_uri, dsl_lang);
        if let Some(existing) = self.files.get(&key) {
            if existing.version() >= version {
                return Some(Arc::clone(&existing));
            }
        }

        let params = VirtualFileParams {
            bench_uri,
            bench_path,
            module_root,
            blocks,
            fixture_names,
            version,
        };

        let virtual_file = builder.build(params);

        if let Some(parent) = Path::new(virtual_file.path()).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Err(e) = std::fs::write(virtual_file.path(), virtual_file.content()) {
            tracing::warn!("Failed to write virtual file: {}", e);
        }

        let vf: Arc<dyn VirtualFile> = Arc::from(virtual_file);
        self.files.insert(key, Arc::clone(&vf));
        Some(vf)
    }

    pub fn remove_all(&self, bench_uri: &str) {
        let keys: Vec<_> = self
            .files
            .iter()
            .filter(|r| r.key().starts_with(&format!("{}\0", bench_uri)))
            .map(|r| r.key().clone())
            .collect();
        for key in keys {
            if let Some((_, vf)) = self.files.remove(&key) {
                let _ = std::fs::remove_file(vf.path());
            }
        }
    }

    pub fn clear_all_caches(&self) {
        for entry in self.files.iter() {
            let _ = std::fs::remove_file(entry.value().path());
        }
        self.files.clear();
        self.setup_initialized.clear();
    }

    /// Clear caches for a specific language (e.g. when go.mod changes)
    pub fn clear_caches_for_lang(&self, lang: DslLang) {
        let suffix = format!("\0{:?}", lang);
        let keys: Vec<_> = self
            .files
            .iter()
            .filter(|r| r.key().ends_with(&suffix))
            .map(|r| r.key().clone())
            .collect();
        for key in keys {
            if let Some((_, vf)) = self.files.remove(&key) {
                let _ = std::fs::remove_file(vf.path());
            }
        }
        // Clear setup tracking for this lang (any module root)
        let setup_keys: Vec<_> = self
            .setup_initialized
            .iter()
            .filter(|r| r.key().ends_with(&suffix))
            .map(|r| r.key().clone())
            .collect();
        for key in setup_keys {
            self.setup_initialized.remove(&key);
        }
    }

    // Backward-compatible accessors for per-language operations
    pub fn go(&self) -> VirtualLangManager<'_> {
        VirtualLangManager { managers: self, lang: Lang::Go }
    }
    pub fn ts(&self) -> VirtualLangManager<'_> {
        VirtualLangManager { managers: self, lang: Lang::TypeScript }
    }
    pub fn rust(&self) -> VirtualLangManager<'_> {
        VirtualLangManager { managers: self, lang: Lang::Rust }
    }
    pub fn python(&self) -> VirtualLangManager<'_> {
        VirtualLangManager { managers: self, lang: Lang::Python }
    }
}

impl Default for VirtualFileManagers {
    fn default() -> Self {
        Self::new()
    }
}

/// Per-language manager for backward compatibility with existing hover code
pub struct VirtualLangManager<'a> {
    managers: &'a VirtualFileManagers,
    lang: Lang,
}

impl VirtualLangManager<'_> {
    pub fn get_or_create(
        &self,
        bench_uri: &str,
        bench_path: &str,
        module_root: &str,
        blocks: &[&EmbeddedBlock],
        version: i32,
    ) -> Option<Arc<dyn VirtualFile>> {
        self.managers.get_or_create(
            bench_uri,
            bench_path,
            self.lang,
            blocks,
            &[],
            version,
            module_root,
        )
    }

    pub fn remove(&self, bench_uri: &str) {
        let dsl_lang = syntax_lang_to_dsl(self.lang);
        let key = cache_key(bench_uri, dsl_lang);
        if let Some((_, vf)) = self.managers.files.remove(&key) {
            let _ = std::fs::remove_file(vf.path());
        }
    }

    pub fn clear_caches(&self) {
        self.managers.clear_caches_for_lang(syntax_lang_to_dsl(self.lang));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poly_bench_lsp_traits::BlockType;
    use poly_bench_syntax::Span;

    fn make_block(code: &str, block_type: BlockType, start: usize) -> EmbeddedBlock {
        let end = start + code.len();
        EmbeddedBlock {
            lang: Lang::Go,
            block_type,
            code: code.to_string(),
            span: Span::new(start, end, 1, 0, 1, code.len()),
            context_name: "test".to_string(),
        }
    }

    #[test]
    fn test_virtual_file_generation() {
        let managers = VirtualFileManagers::new();
        let import_block = make_block("\"fmt\"", BlockType::SetupImport, 100);
        let bench_block = make_block("fmt.Println(\"hello\")", BlockType::Benchmark, 200);
        let blocks: Vec<&EmbeddedBlock> = vec![&import_block, &bench_block];

        let vf =
            managers.go().get_or_create("file:///test.bench", "/test.bench", "/tmp/go", &blocks, 1);

        assert!(vf.is_some());
        let vf = vf.unwrap();
        assert!(vf.content().contains("package main"));
        assert!(vf.content().contains("\"fmt\""));
        assert!(vf.content().contains("fmt.Println"));
        assert_eq!(vf.section_mappings().len(), 2);
    }

    #[test]
    fn test_position_translation() {
        let managers = VirtualFileManagers::new();
        let code = "fmt.Println(\"hello\")";
        let block = make_block(code, BlockType::Benchmark, 100);
        let blocks: Vec<&EmbeddedBlock> = vec![&block];

        let vf = managers
            .go()
            .get_or_create("file:///test.bench", "/test.bench", "/tmp/go", &blocks, 1)
            .unwrap();

        let pos = vf.bench_to_virtual(100);
        assert!(pos.is_some());
        let pos = pos.unwrap();
        assert_eq!(pos.character, 0);

        let offset = vf.virtual_to_bench(pos.line, pos.character);
        assert!(offset.is_some());
        assert_eq!(offset.unwrap(), 100);
    }
}
