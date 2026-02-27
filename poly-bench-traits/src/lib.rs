//! Traits and types for poly-bench
//!
//! This crate defines interfaces for LSP integration (virtual file building,
//! embedded diagnostics, hover providers) and runtimes. Runtimes implement
//! these traits; the LSP injects context.

// LSP / virtual file modules
pub mod context;
pub mod diagnostics;
pub mod embedded;
pub mod helper_extractor;
pub mod hover;
pub mod lsp_client;
pub mod position;
pub mod virtual_file;
pub mod virtual_file_core;

// Runtime modules
pub mod config;
pub mod error_mapping;
pub mod lang_display;
pub mod measurement;
pub mod plugin;
pub mod project;
pub mod stdlib_provider;
pub mod traits;

// LSP re-exports
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

// Runtime re-exports
pub use config::RuntimeConfig;
pub use error_mapping::{ErrorMapper, LineMapping, LineMappings};
pub use lang_display::LangDisplayInfo;
pub use measurement::{Comparison, ComparisonWinner, Measurement, DEFAULT_CV_THRESHOLD};
pub use plugin::{RuntimePlugin, PLUGINS};
pub use project::{detect_from_markers, ProjectRootDetector};
pub use stdlib_provider::StdlibProvider;
pub use traits::{Runtime, RuntimeFactory};

/// Convert syntax Lang to dsl Lang for registry lookups
pub fn syntax_lang_to_dsl(l: poly_bench_syntax::Lang) -> poly_bench_dsl::Lang {
    poly_bench_dsl::Lang::from_str(l.as_str()).expect("syntax Lang and dsl Lang must stay in sync")
}
