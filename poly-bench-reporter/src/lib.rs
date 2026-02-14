//! Report generation modules

pub mod chart_executor;
pub mod charts;
pub mod console;
pub mod json;
pub mod markdown;
pub mod svg;
pub use chart_executor::{execute_chart_directives, GeneratedChart};
