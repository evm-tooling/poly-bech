//! Runtime registry for pluggable language runtimes

use crate::{config::RuntimeConfig, go, js, python, rust, traits::Runtime};
use miette::{miette, Result};
use poly_bench_dsl::Lang;
use std::{collections::HashMap, sync::Arc};

/// Factory for creating runtime instances
pub trait RuntimeFactory: Send + Sync {
    /// Get the language this factory creates runtimes for
    fn lang(&self) -> Lang;

    /// Get the display name of this runtime
    fn name(&self) -> &'static str;

    /// Create a new runtime instance with the given configuration
    fn create(&self, config: &RuntimeConfig) -> Result<Box<dyn Runtime>>;
}

struct GoRuntimeFactory;
impl RuntimeFactory for GoRuntimeFactory {
    fn lang(&self) -> Lang {
        Lang::Go
    }
    fn name(&self) -> &'static str {
        "Go Plugin Runtime"
    }
    fn create(&self, config: &RuntimeConfig) -> Result<Box<dyn Runtime>> {
        let mut rt = go::GoRuntime::new();
        rt.set_module_root(config.go_root.clone());
        Ok(Box::new(rt))
    }
}

struct JsRuntimeFactory;
impl RuntimeFactory for JsRuntimeFactory {
    fn lang(&self) -> Lang {
        Lang::TypeScript
    }
    fn name(&self) -> &'static str {
        "JavaScript/TypeScript Runtime"
    }
    fn create(&self, config: &RuntimeConfig) -> Result<Box<dyn Runtime>> {
        let mut rt = js::JsRuntime::new()?;
        rt.set_project_root(config.node_root.clone());
        Ok(Box::new(rt))
    }
}

struct RustRuntimeFactory;
impl RuntimeFactory for RustRuntimeFactory {
    fn lang(&self) -> Lang {
        Lang::Rust
    }
    fn name(&self) -> &'static str {
        "Rust Runtime"
    }
    fn create(&self, config: &RuntimeConfig) -> Result<Box<dyn Runtime>> {
        let mut rt = rust::RustRuntime::new();
        rt.set_project_root(config.rust_root.clone());
        Ok(Box::new(rt))
    }
}

struct PythonRuntimeFactory;
impl RuntimeFactory for PythonRuntimeFactory {
    fn lang(&self) -> Lang {
        Lang::Python
    }
    fn name(&self) -> &'static str {
        "Python Runtime"
    }
    fn create(&self, config: &RuntimeConfig) -> Result<Box<dyn Runtime>> {
        let mut rt = python::PythonRuntime::new()?;
        rt.set_project_root(config.python_root.clone());
        Ok(Box::new(rt))
    }
}

/// All registered runtime factories
static FACTORIES: &[&dyn RuntimeFactory] =
    &[&GoRuntimeFactory, &JsRuntimeFactory, &RustRuntimeFactory, &PythonRuntimeFactory];

/// Create a runtime for the given language
pub fn create_runtime(lang: Lang, config: &RuntimeConfig) -> Result<Box<dyn Runtime>> {
    for factory in FACTORIES {
        if factory.lang() == lang {
            return factory.create(config);
        }
    }
    Err(miette!("No runtime registered for language: {}", lang))
}

/// Get all supported languages
pub fn supported_languages() -> &'static [Lang] {
    static LANGS: &[Lang] = &[Lang::Go, Lang::TypeScript, Lang::Rust, Lang::Python];
    LANGS
}

/// Build a map of runtimes for the requested languages (owned, for scheduler).
/// Returns an error if any requested language fails to create (e.g. Node.js not found for TS).
pub fn create_runtimes(
    langs: &[Lang],
    config: &RuntimeConfig,
) -> Result<HashMap<Lang, Box<dyn Runtime>>> {
    let mut runtimes = HashMap::new();
    for lang in langs {
        let rt = create_runtime(*lang, config)?;
        runtimes.insert(*lang, rt);
    }
    Ok(runtimes)
}

/// Build a map of Arc-wrapped runtimes for validation (shared across parallel tasks).
/// Skips languages whose runtime fails to create (e.g. Node.js not found for TypeScript).
pub fn create_runtimes_arc(
    langs: &[Lang],
    config: &RuntimeConfig,
) -> HashMap<Lang, Arc<dyn Runtime>> {
    let mut runtimes = HashMap::new();
    for lang in langs {
        if let Ok(rt) = create_runtime(*lang, config) {
            runtimes.insert(*lang, Arc::from(rt));
        }
    }
    runtimes
}
