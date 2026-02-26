//! Traits and types for poly-bench runtimes
//!
//! This crate defines the interface that each language runtime must implement.
//! It has no dependencies on runtime implementations, allowing runtimes to
//! depend on it without circular dependencies.

pub mod config;
pub mod error_mapping;
pub mod lang_display;
pub mod measurement;
pub mod plugin;
pub mod project;
pub mod stdlib_provider;
pub mod traits;

pub use config::RuntimeConfig;
pub use error_mapping::{ErrorMapper, LineMapping, LineMappings};
pub use lang_display::LangDisplayInfo;
pub use measurement::{Comparison, ComparisonWinner, Measurement, DEFAULT_CV_THRESHOLD};
pub use plugin::{RuntimePlugin, PLUGINS};
pub use project::{detect_from_markers, ProjectRootDetector};
pub use stdlib_provider::StdlibProvider;
pub use traits::{Runtime, RuntimeFactory};
