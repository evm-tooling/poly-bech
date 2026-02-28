//! LSP implementation of EmbeddedHoverContext
//!
//! Provides LSP clients and virtual files to hover providers from runtimes.
//! Clients are obtained via the poly-bench runtime registry.

use std::sync::Arc;

use poly_bench_runtime::{get_embedded_lsp_client, init_embedded_lsp_client};
use poly_bench_syntax::Lang;
use poly_bench_traits::{EmbeddedHoverContext, EmbeddedLspClient, VirtualFile};

use crate::{
    document::Document, embedded::blocks_for_language, virtual_files::VirtualFileManagers,
};

/// LSP implementation of EmbeddedHoverContext
pub struct LspEmbeddedHoverContext<'a> {
    pub doc: &'a Document,
    pub bench_uri: &'a str,
    pub bench_path: &'a str,
    pub blocks: &'a [crate::embedded::EmbeddedBlock],
    pub module_root: &'a str,
    pub bench_offset: usize,
    pub managers: &'a VirtualFileManagers,
}

impl EmbeddedHoverContext for LspEmbeddedHoverContext<'_> {
    fn get_virtual_file(&self, lang: Lang) -> Option<Arc<dyn VirtualFile>> {
        let lang_blocks: Vec<_> = blocks_for_language(self.blocks, lang);
        let block_refs: Vec<_> = lang_blocks.iter().map(|b| *b).collect();
        self.managers.get_or_create(
            self.bench_uri,
            self.bench_path,
            lang,
            &block_refs,
            &[],
            self.doc.version,
            self.module_root,
        )
    }

    fn get_lsp_client(
        &self,
        lang: poly_bench_dsl::Lang,
        module_root: &str,
    ) -> Option<Arc<dyn EmbeddedLspClient>> {
        if crate::embedded_diagnostic_context::should_skip_embedded_lsp_for_lang(
            std::path::Path::new(self.bench_path),
            lang,
        ) {
            return None;
        }
        if module_root != self.module_root {
            return None;
        }
        init_embedded_lsp_client(lang, module_root).or_else(|| get_embedded_lsp_client(lang))
    }

    fn byte_to_position(&self, offset: usize) -> (u32, u32) {
        let (line, col) = self.doc.byte_to_position(offset);
        (line as u32, col as u32)
    }

    fn bench_offset(&self) -> usize {
        self.bench_offset
    }

    fn module_root(&self) -> &str {
        self.module_root
    }
}
