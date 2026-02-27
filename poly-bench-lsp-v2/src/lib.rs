//! Poly-bench LSP Server v2
//!
//! This is a rewritten LSP server that provides robust language features
//! using Tree-sitter for error-tolerant parsing.
//!
//! # Key Improvements over v1
//!
//! - **Error-tolerant parsing**: Always produces a syntax tree, even with errors
//! - **Incremental parsing**: Fast re-parsing on edits
//! - **Full semantic token coverage**: All syntax elements are highlighted
//! - **Incremental formatting**: Small edits instead of whole-document replacement
//! - **Better diagnostics**: Syntax errors from Tree-sitter + semantic validation
//! - **Embedded language hover**: Full hover support for Go, TypeScript, Rust, and Python code

pub mod diagnostics;
pub mod document;
pub mod embedded;
pub mod embedded_diagnostic_context;
pub mod embedded_diagnostics;
pub mod embedded_hover_context;
pub mod formatter;
pub mod hover;
pub mod hover_cache;
pub mod semantic_tokens;
pub mod server;
pub mod virtual_files;

pub use server::PolyBenchLanguageServer;

use tower_lsp::{LspService, Server};

/// Run the LSP server on stdin/stdout
pub async fn run_server() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("info".parse().unwrap())
                .add_directive("poly_bench_lsp_v2=debug".parse().unwrap())
                .add_directive("runtimes_csharp=debug".parse().unwrap())
                .add_directive("poly_bench_traits=debug".parse().unwrap()),
        )
        .with_writer(std::io::stderr)
        .init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| PolyBenchLanguageServer::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}
