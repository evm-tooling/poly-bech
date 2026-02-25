//! Traits and types for poly-bench LSP integration
//!
//! This crate defines the interface for virtual file building, embedded diagnostics,
//! and hover providers. Runtimes implement these traits; the LSP injects context.

pub mod context;
pub mod diagnostics;
pub mod embedded;
pub mod helper_extractor;
pub mod hover;
pub mod lsp_client;
pub mod position;
pub mod virtual_file;
pub mod virtual_file_core;

pub use context::EmbeddedHoverContext;
pub use diagnostics::{
    EmbeddedDiagnostic, EmbeddedDiagnosticContext, EmbeddedDiagnosticProvider,
    EmbeddedDiagnosticSetup, EmbeddedLspClient, LspDiagnostic,
};
pub use embedded::{BlockType, EmbeddedBlock};
pub use helper_extractor::HelperFunctionExtractor;
pub use hover::EmbeddedHoverProvider;
pub use lsp_client::{LspClient, LspConfig};
pub use position::LspPosition;
pub use virtual_file::{SectionMapping, VirtualFile, VirtualFileBuilder, VirtualFileParams};
pub use virtual_file_core::{VirtualFileBuilderCore, VirtualFileData};

/// Convert syntax Lang to dsl Lang for registry lookups
pub fn syntax_lang_to_dsl(l: poly_bench_syntax::Lang) -> poly_bench_dsl::Lang {
    match l {
        poly_bench_syntax::Lang::Go => poly_bench_dsl::Lang::Go,
        poly_bench_syntax::Lang::TypeScript => poly_bench_dsl::Lang::TypeScript,
        poly_bench_syntax::Lang::Rust => poly_bench_dsl::Lang::Rust,
        poly_bench_syntax::Lang::Python => poly_bench_dsl::Lang::Python,
    }
}
