//! LSP implementation of EmbeddedHoverContext

use std::sync::Arc;

use poly_bench_lsp_traits::{EmbeddedHoverContext, EmbeddedLspClient, VirtualFile};
use poly_bench_syntax::Lang;

use crate::{
    document::Document, embedded::blocks_for_language, gopls_client, pyright_client,
    rust_analyzer_client, tsserver_client, virtual_files::VirtualFileManagers,
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

    fn get_go_client(&self, module_root: &str) -> Option<Arc<dyn EmbeddedLspClient>> {
        if module_root != self.module_root {
            return None;
        }
        let _ = gopls_client::init_gopls_client(module_root);
        gopls_client::get_gopls_client().map(|c| {
            Arc::new(crate::embedded_diagnostic_context::GoplsClientAdapter(c))
                as Arc<dyn EmbeddedLspClient>
        })
    }

    fn get_ts_client(&self, module_root: &str) -> Option<Arc<dyn EmbeddedLspClient>> {
        if module_root != self.module_root {
            return None;
        }
        let _ = tsserver_client::init_tsserver_client(module_root);
        tsserver_client::get_tsserver_client().map(|c| {
            Arc::new(crate::embedded_diagnostic_context::TsserverClientAdapter(c))
                as Arc<dyn EmbeddedLspClient>
        })
    }

    fn get_rust_client(&self, module_root: &str) -> Option<Arc<dyn EmbeddedLspClient>> {
        if module_root != self.module_root {
            return None;
        }
        let _ = rust_analyzer_client::init_rust_analyzer_client(module_root);
        rust_analyzer_client::get_rust_analyzer_client().map(|c| {
            Arc::new(crate::embedded_diagnostic_context::RustAnalyzerClientAdapter(c))
                as Arc<dyn EmbeddedLspClient>
        })
    }

    fn get_pyright_client(&self, module_root: &str) -> Option<Arc<dyn EmbeddedLspClient>> {
        if module_root != self.module_root {
            return None;
        }
        let _ = pyright_client::init_pyright_client(module_root);
        pyright_client::get_pyright_client().map(|c| {
            Arc::new(crate::embedded_diagnostic_context::PyrightClientAdapter(c))
                as Arc<dyn EmbeddedLspClient>
        })
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
