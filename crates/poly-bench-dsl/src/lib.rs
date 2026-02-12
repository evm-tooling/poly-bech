//! DSL parsing module for poly-bench
//!
//! This crate provides the lexer, parser, and AST types for the poly-bench DSL.

mod tokens;
mod lexer;
mod parser;
mod ast;
mod error;
mod formatter;
pub mod validate;

pub use ast::*;
pub use formatter::{format_file, format_file_with_source};
pub use error::{ParseError, NamedSource};
pub use validate::{ValidationResult, ValidationError, ValidationWarning, validate_file, validate_suite};

use miette::Result;

/// Parse a poly-bench DSL source string into an AST
pub fn parse(source: &str, filename: &str) -> Result<File> {
    parser::parse(source, filename)
}
