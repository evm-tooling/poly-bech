//! Manifest parsing and serialization for polybench.toml

use miette::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

/// The main manifest structure for a poly-bench project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    /// Project metadata
    pub project: ProjectConfig,

    /// Default benchmark settings
    #[serde(default)]
    pub defaults: DefaultsConfig,

    /// Go-specific configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub go: Option<GoConfig>,

    /// TypeScript-specific configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts: Option<TsConfig>,

    /// Output configuration
    #[serde(default)]
    pub output: OutputConfig,
}

/// Project metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    /// Project name
    pub name: String,

    /// Project version
    #[serde(default = "default_version")]
    pub version: String,

    /// Project description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

fn default_version() -> String {
    "0.0.1".to_string()
}

/// Default benchmark settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultsConfig {
    /// Default number of iterations
    #[serde(default = "default_iterations")]
    pub iterations: u64,

    /// Default warmup iterations
    #[serde(default = "default_warmup")]
    pub warmup: u64,

    /// Enabled languages
    #[serde(default = "default_languages")]
    pub languages: Vec<String>,
}

fn default_iterations() -> u64 {
    1000
}

fn default_warmup() -> u64 {
    100
}

fn default_languages() -> Vec<String> {
    vec!["go".to_string(), "ts".to_string()]
}

impl Default for DefaultsConfig {
    fn default() -> Self {
        Self {
            iterations: default_iterations(),
            warmup: default_warmup(),
            languages: default_languages(),
        }
    }
}

/// Go-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoConfig {
    /// Go module path (e.g., "github.com/user/project")
    pub module: String,

    /// Minimum Go version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Go dependencies (package -> version)
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub dependencies: HashMap<String, String>,
}

/// TypeScript-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TsConfig {
    /// Runtime to use (node, bun, deno)
    #[serde(default = "default_ts_runtime")]
    pub runtime: String,

    /// NPM dependencies (package -> version)
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub dependencies: HashMap<String, String>,
}

fn default_ts_runtime() -> String {
    "node".to_string()
}

/// Output configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Report formats to generate
    #[serde(default)]
    pub report_formats: Vec<String>,

    /// Output directory for reports and charts
    #[serde(default = "default_output_dir")]
    pub output_dir: String,

    /// Whether to auto-save benchmark results to JSON
    #[serde(default = "default_auto_save")]
    pub auto_save_results: bool,
}

fn default_output_dir() -> String {
    "out".to_string()
}

fn default_auto_save() -> bool {
    true
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            report_formats: Vec::new(),
            output_dir: default_output_dir(),
            auto_save_results: default_auto_save(),
        }
    }
}

impl Manifest {
    /// Create a new manifest with the given project name and languages
    pub fn new(name: &str, languages: &[String]) -> Self {
        let has_go = languages.iter().any(|l| l == "go");
        let has_ts = languages.iter().any(|l| l == "ts" || l == "typescript");

        Self {
            project: ProjectConfig {
                name: name.to_string(),
                version: default_version(),
                description: None,
            },
            defaults: DefaultsConfig { languages: languages.to_vec(), ..Default::default() },
            go: if has_go {
                Some(GoConfig {
                    module: name.to_string(),
                    version: Some("1.21".to_string()),
                    dependencies: HashMap::new(),
                })
            } else {
                None
            },
            ts: if has_ts {
                Some(TsConfig { runtime: default_ts_runtime(), dependencies: HashMap::new() })
            } else {
                None
            },
            output: OutputConfig::default(),
        }
    }

    /// Check if Go is enabled
    pub fn has_go(&self) -> bool {
        self.go.is_some()
    }

    /// Check if TypeScript is enabled
    pub fn has_ts(&self) -> bool {
        self.ts.is_some()
    }

    /// Get enabled languages
    pub fn enabled_languages(&self) -> Vec<String> {
        let mut langs = Vec::new();
        if self.has_go() {
            langs.push("go".to_string());
        }
        if self.has_ts() {
            langs.push("ts".to_string());
        }
        langs
    }

    /// Add a Go dependency
    pub fn add_go_dependency(&mut self, package: &str, version: &str) -> Result<()> {
        let go =
            self.go.as_mut().ok_or_else(|| miette::miette!("Go is not enabled in this project"))?;
        go.dependencies.insert(package.to_string(), version.to_string());
        Ok(())
    }

    /// Add a TypeScript dependency
    pub fn add_ts_dependency(&mut self, package: &str, version: &str) -> Result<()> {
        let ts = self
            .ts
            .as_mut()
            .ok_or_else(|| miette::miette!("TypeScript is not enabled in this project"))?;
        ts.dependencies.insert(package.to_string(), version.to_string());
        Ok(())
    }
}

/// Load a manifest from a file
pub fn load(path: &Path) -> Result<Manifest> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| miette::miette!("Failed to read {}: {}", path.display(), e))?;

    toml::from_str(&content)
        .map_err(|e| miette::miette!("Failed to parse {}: {}", path.display(), e))
}

/// Save a manifest to a file
pub fn save(path: &Path, manifest: &Manifest) -> Result<()> {
    let content = toml::to_string_pretty(manifest)
        .map_err(|e| miette::miette!("Failed to serialize manifest: {}", e))?;

    std::fs::write(path, content)
        .map_err(|e| miette::miette!("Failed to write {}: {}", path.display(), e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_new() {
        let manifest = Manifest::new("my-project", &["go".to_string(), "ts".to_string()]);

        assert_eq!(manifest.project.name, "my-project");
        assert!(manifest.has_go());
        assert!(manifest.has_ts());
    }

    #[test]
    fn test_manifest_roundtrip() {
        let manifest = Manifest::new("test-project", &["go".to_string()]);
        let toml_str = toml::to_string_pretty(&manifest).unwrap();
        let parsed: Manifest = toml::from_str(&toml_str).unwrap();

        assert_eq!(parsed.project.name, "test-project");
        assert!(parsed.has_go());
        assert!(!parsed.has_ts());
    }

    #[test]
    fn test_add_dependency() {
        let mut manifest = Manifest::new("test", &["go".to_string(), "ts".to_string()]);

        manifest.add_go_dependency("github.com/pkg/errors", "v0.9.1").unwrap();
        manifest.add_ts_dependency("viem", "^2.0.0").unwrap();

        assert_eq!(
            manifest.go.as_ref().unwrap().dependencies.get("github.com/pkg/errors"),
            Some(&"v0.9.1".to_string())
        );
        assert_eq!(
            manifest.ts.as_ref().unwrap().dependencies.get("viem"),
            Some(&"^2.0.0".to_string())
        );
    }
}
