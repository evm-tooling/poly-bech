//! Poly-Bench Language Server
//!
//! This binary implements an LSP server for `.bench` files,
//! providing diagnostics, hover, completion, and semantic tokens.

use tower_lsp::{LspService, Server};

mod backend;
mod completion;
mod diagnostics;
mod document;
mod embedded;
mod gopls_client;
mod hover;
mod semantic_tokens;
mod virtual_files;

use backend::Backend;

#[tokio::main]
async fn main() {
    // Use stderr for logging since stdout is used for LSP communication
    eprintln!("Starting poly-bench-lsp...");

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(Backend::new);

    Server::new(stdin, stdout, socket).serve(service).await;
}
