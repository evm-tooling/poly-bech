//! Poly-Bench Language Server
//!
//! This crate implements an LSP server for `.bench` files,
//! providing diagnostics, hover, completion, and semantic tokens.

pub mod backend;
pub mod completion;
pub mod diagnostics;
pub mod document;
pub mod embedded;
pub mod gopls_client;
pub mod hover;
pub mod lsp_client;
pub mod rust_analyzer_client;
pub mod semantic_tokens;
pub mod tsserver_client;
pub mod virtual_files;

pub use backend::Backend;
