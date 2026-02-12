//! Report generation modules

pub mod console;
pub mod markdown;
pub mod json;
pub mod svg;
pub mod charts;
pub mod chart_executor;pub use chart_executor::{execute_chart_directives, GeneratedChart};
