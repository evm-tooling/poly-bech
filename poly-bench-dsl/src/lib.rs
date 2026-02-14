//! DSL parsing module for poly-bench
//!
//! This crate provides the lexer, parser, and AST types for the poly-bench DSL.

mod ast;
mod error;
mod formatter;
mod lexer;
mod parser;
mod tokens;
pub mod validate;

pub use ast::*;
pub use error::{NamedSource, ParseError};
pub use formatter::{format_file, format_file_with_source};
pub use validate::{
    validate_file, validate_suite, ValidationError, ValidationResult, ValidationWarning,
};

use miette::Result;

/// Parse a poly-bench DSL source string into an AST
pub fn parse(source: &str, filename: &str) -> Result<File> {
    parser::parse(source, filename)
}
