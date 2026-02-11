//! DSL parsing module
//!
//! This module provides the lexer, parser, and AST types for the poly-bench DSL.

mod tokens;
mod lexer;
mod parser;
mod ast;
mod error;
pub mod validate;

pub use ast::*;
pub use error::ParseError;
pub use validate::{validate_suite, ValidationResult, ValidationError, ValidationWarning};

use miette::Result;

/// Parse a poly-bench DSL source string into an AST
pub fn parse(source: &str, filename: &str) -> Result<File> {
    parser::parse(source, filename)
}
