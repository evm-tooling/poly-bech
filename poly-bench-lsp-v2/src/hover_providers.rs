//! Hover providers for embedded languages
//!
//! Registry-based hover support for Go, TypeScript, Rust, and Python.

use poly_bench_syntax::Lang;
use tower_lsp::lsp_types::{Hover, Url};

use crate::{document::Document, embedded::EmbeddedBlock, virtual_files::VirtualFileManagers};

/// Trait for language-specific hover providers
pub trait EmbeddedHoverProvider: Send + Sync {
    fn lang(&self) -> Lang;
    fn get_hover(
        &self,
        doc: &Document,
        uri: &Url,
        offset: usize,
        blocks: &[EmbeddedBlock],
        module_root: &str,
        managers: &VirtualFileManagers,
    ) -> Option<Hover>;
}

mod impls {
    use super::*;

    pub struct GoHoverProvider;
    impl super::EmbeddedHoverProvider for GoHoverProvider {
        fn lang(&self) -> Lang {
            Lang::Go
        }
        fn get_hover(
            &self,
            doc: &Document,
            uri: &Url,
            offset: usize,
            blocks: &[EmbeddedBlock],
            module_root: &str,
            managers: &VirtualFileManagers,
        ) -> Option<Hover> {
            crate::hover::get_gopls_hover_impl(doc, uri, offset, blocks, module_root, managers)
        }
    }

    pub struct TsHoverProvider;
    impl super::EmbeddedHoverProvider for TsHoverProvider {
        fn lang(&self) -> Lang {
            Lang::TypeScript
        }
        fn get_hover(
            &self,
            doc: &Document,
            uri: &Url,
            offset: usize,
            blocks: &[EmbeddedBlock],
            module_root: &str,
            managers: &VirtualFileManagers,
        ) -> Option<Hover> {
            crate::hover::get_tsserver_hover_impl(doc, uri, offset, blocks, module_root, managers)
        }
    }

    pub struct RustHoverProvider;
    impl super::EmbeddedHoverProvider for RustHoverProvider {
        fn lang(&self) -> Lang {
            Lang::Rust
        }
        fn get_hover(
            &self,
            doc: &Document,
            uri: &Url,
            offset: usize,
            blocks: &[EmbeddedBlock],
            module_root: &str,
            managers: &VirtualFileManagers,
        ) -> Option<Hover> {
            crate::hover::get_rust_analyzer_hover_impl(
                doc,
                uri,
                offset,
                blocks,
                module_root,
                managers,
            )
        }
    }

    /// Python hover provider via pyright or pylsp LSP
    pub struct PythonHoverProvider;
    impl super::EmbeddedHoverProvider for PythonHoverProvider {
        fn lang(&self) -> Lang {
            Lang::Python
        }
        fn get_hover(
            &self,
            doc: &Document,
            uri: &Url,
            offset: usize,
            blocks: &[EmbeddedBlock],
            module_root: &str,
            managers: &VirtualFileManagers,
        ) -> Option<Hover> {
            crate::hover::get_pyright_hover_impl(doc, uri, offset, blocks, module_root, managers)
        }
    }
}

/// Get the embedded hover provider for a language
pub fn get_embedded_hover_provider(lang: Lang) -> Option<&'static dyn EmbeddedHoverProvider> {
    match lang {
        Lang::Go => Some(&impls::GoHoverProvider),
        Lang::TypeScript => Some(&impls::TsHoverProvider),
        Lang::Rust => Some(&impls::RustHoverProvider),
        Lang::Python => Some(&impls::PythonHoverProvider),
        _ => None,
    }
}
