//! Embedded block types for virtual file building
//!
//! These types are shared between the LSP (extraction) and runtimes (building).

use poly_bench_syntax::{Lang, Span};

/// The type of embedded block (affects how code is wrapped in virtual files)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    /// Setup section: imports
    SetupImport,
    /// Setup section: declarations
    SetupDeclare,
    /// Setup section: init code
    SetupInit,
    /// Setup section: helper functions
    SetupHelpers,
    /// Fixture implementation
    Fixture,
    /// Benchmark implementation
    Benchmark,
    /// Lifecycle hook (before/after/each)
    Hook,
    /// Skip condition
    Skip,
    /// Validation expression
    Validate,
}

/// An embedded code block extracted from a .bench file
#[derive(Debug, Clone)]
pub struct EmbeddedBlock {
    /// The programming language
    pub lang: Lang,
    /// The type of block (affects wrapping)
    pub block_type: BlockType,
    /// The code content
    pub code: String,
    /// Source span in the .bench file
    pub span: Span,
    /// Name of the containing construct (for error messages)
    pub context_name: String,
}
