//! Runtime configuration for project roots

use poly_bench_dsl::Lang;
use std::{collections::HashMap, path::PathBuf};

/// Configuration passed to runtime factories when creating a runtime.
#[derive(Debug, Clone, Default)]
pub struct RuntimeConfig {
    /// Project roots per language (e.g. go.mod dir for Go, package.json dir for TypeScript)
    pub roots: HashMap<Lang, Option<PathBuf>>,
}

impl RuntimeConfig {
    /// Get the project root for a language
    pub fn get_root(&self, lang: Lang) -> Option<PathBuf> {
        self.roots.get(&lang).and_then(|o| o.clone())
    }

    /// Set the project root for a language
    pub fn set_root(&mut self, lang: Lang, path: Option<PathBuf>) {
        self.roots.insert(lang, path);
    }
}
