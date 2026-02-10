//! Dependency management for poly-bench projects

use crate::project::{self, manifest, templates};
use colored::Colorize;
use miette::Result;
use std::path::Path;
use std::process::Command;

/// Parse a dependency spec like "package@version" into (package, version)
fn parse_dep_spec(spec: &str) -> (String, String) {
    if let Some(idx) = spec.rfind('@') {
        let package = spec[..idx].to_string();
        let version = spec[idx + 1..].to_string();
        (package, version)
    } else {
        // No version specified
        (spec.to_string(), "latest".to_string())
    }
}

/// Build the argument for `go get` so that transitive deps are added to go.sum.
/// Using "module/...@version" fetches all packages in the module and their
/// transitive dependencies; plain "module@version" can leave go.sum missing
/// entries and cause "missing go.sum entry" when the benchmark runs.
fn go_get_spec_for_transitives(package: &str, version: &str) -> String {
    let module = if package.contains('/') {
        let parts: Vec<&str> = package.split('/').collect();
        // Standard Go module path: host/org/repo (e.g. github.com/owner/repo)
        if parts.len() >= 3 {
            parts[..3].join("/")
        } else {
            package.to_string()
        }
    } else {
        package.to_string()
    };
    if version == "latest" {
        format!("{}/...@latest", module)
    } else {
        format!("{}/...@{}", module, version)
    }
}

/// Add a Go dependency to the project
pub fn add_go_dependency(spec: &str) -> Result<()> {
    let current_dir = std::env::current_dir()
        .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

    let project_root = project::find_project_root(&current_dir)
        .ok_or_else(|| miette::miette!("Not in a poly-bench project"))?;

    let mut manifest = project::load_manifest(&project_root)?;

    if !manifest.has_go() {
        return Err(miette::miette!("Go is not enabled in this project"));
    }

    let (package, version) = parse_dep_spec(spec);

    // Add to manifest
    manifest.add_go_dependency(&package, &version)?;
    project::save_manifest(&project_root, &manifest)?;

    println!(
        "{} Added Go dependency: {}@{}",
        "✓".green().bold(),
        package,
        version
    );

    // Use module/...@version so Go fetches all packages and transitive deps into go.sum
    let go_get_arg = go_get_spec_for_transitives(&package, &version);
    println!("{} Running go get {}...", "→".blue(), go_get_arg);

    let status = Command::new("go")
        .args(["get", &go_get_arg])
        .current_dir(&project_root)
        .status()
        .map_err(|e| miette::miette!("Failed to run go get: {}", e))?;

    if !status.success() {
        return Err(miette::miette!("go get failed"));
    }

    // Do not run go mod tidy: it can remove deps (no packages in root) or fail for
    // modules that have no root package (e.g. viem-go). go get already updated go.mod/go.sum.
    println!("{} Go dependency installed successfully", "✓".green().bold());

    Ok(())
}

/// Add a TypeScript dependency to the project
pub fn add_ts_dependency(spec: &str) -> Result<()> {
    let current_dir = std::env::current_dir()
        .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

    let project_root = project::find_project_root(&current_dir)
        .ok_or_else(|| miette::miette!("Not in a poly-bench project"))?;

    let mut manifest = project::load_manifest(&project_root)?;

    if !manifest.has_ts() {
        return Err(miette::miette!("TypeScript is not enabled in this project"));
    }

    let (package, version) = parse_dep_spec(spec);

    // Add to manifest
    manifest.add_ts_dependency(&package, &version)?;
    project::save_manifest(&project_root, &manifest)?;

    println!(
        "{} Added TypeScript dependency: {}@{}",
        "✓".green().bold(),
        package,
        version
    );

    // Sync new dependency to package.json so it appears under "dependencies"
    update_package_json_deps(&project_root, manifest.ts.as_ref().unwrap())?;

    // Run npm install
    let npm_spec = if version == "latest" {
        package.clone()
    } else {
        format!("{}@{}", package, version)
    };

    println!("{} Running npm install {}...", "→".blue(), npm_spec);

    let status = Command::new("npm")
        .args(["install", &npm_spec])
        .current_dir(&project_root)
        .status()
        .map_err(|e| miette::miette!("Failed to run npm install: {}", e))?;

    if !status.success() {
        return Err(miette::miette!("npm install failed"));
    }

    println!(
        "{} TypeScript dependency installed successfully",
        "✓".green().bold()
    );

    Ok(())
}

/// Install all dependencies from polybench.toml
pub fn install_all() -> Result<()> {
    let current_dir = std::env::current_dir()
        .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

    let project_root = project::find_project_root(&current_dir)
        .ok_or_else(|| miette::miette!("Not in a poly-bench project"))?;

    let manifest = project::load_manifest(&project_root)?;

    println!(
        "{} Installing dependencies for {}",
        "▶".green().bold(),
        manifest.project.name
    );
    println!();

    // Install Go dependencies
    if let Some(ref go_config) = manifest.go {
        install_go_deps(&project_root, go_config)?;
    }

    // Install TypeScript dependencies
    if let Some(ref ts_config) = manifest.ts {
        install_ts_deps(&project_root, ts_config, &manifest.project.name)?;
    }

    println!();
    println!("{} All dependencies installed!", "✓".green().bold());

    Ok(())
}

/// Install Go dependencies
fn install_go_deps(project_root: &Path, go_config: &manifest::GoConfig) -> Result<()> {
    println!("{} Go dependencies", "→".blue().bold());

    // Ensure go.mod exists
    let go_mod_path = project_root.join("go.mod");
    if !go_mod_path.exists() {
        let go_mod_content =
            templates::go_mod(&go_config.module, go_config.version.as_deref());
        std::fs::write(&go_mod_path, go_mod_content)
            .map_err(|e| miette::miette!("Failed to write go.mod: {}", e))?;
        println!("  {} Created go.mod", "✓".green());
    }

    // Run go get for each dependency (use module/...@version so transitives go into go.sum)
    for (package, version) in &go_config.dependencies {
        let go_get_arg = go_get_spec_for_transitives(package, version);
        println!("  {} Installing {}...", "→".blue(), go_get_arg);

        let output = Command::new("go")
            .args(["get", &go_get_arg])
            .current_dir(project_root)
            .output()
            .map_err(|e| miette::miette!("Failed to run go get: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(miette::miette!(
                "go get {} failed.\n{}",
                go_get_arg,
                if stderr.is_empty() {
                    "Run from project root and ensure 'go' is on PATH.".to_string()
                } else {
                    stderr.to_string()
                }
            ));
        }
        println!("  {} Installed {}", "✓".green(), package);
    }

    // Generate deps.go so the module has a buildable main that blank-imports each dep.
    // This keeps go.sum complete and allows `go build .` to work. Use package import
    // paths in polybench.toml (e.g. .../viem-go/abi) for modules with no root package.
    if !go_config.dependencies.is_empty() {
        let import_paths: Vec<String> = go_config.dependencies.keys().cloned().collect();
        let deps_content = templates::go_deps_file(&import_paths);
        let deps_path = project_root.join(templates::GO_DEPS_FILENAME);
        std::fs::write(&deps_path, deps_content)
            .map_err(|e| miette::miette!("Failed to write {}: {}", deps_path.display(), e))?;
        println!("  {} Created {}", "✓".green(), templates::GO_DEPS_FILENAME);
    }

    println!("  {} Go dependencies ready", "✓".green());

    println!();
    Ok(())
}

/// Install TypeScript dependencies
fn install_ts_deps(
    project_root: &Path,
    ts_config: &manifest::TsConfig,
    project_name: &str,
) -> Result<()> {
    println!("{} TypeScript dependencies", "→".blue().bold());

    // Ensure package.json exists
    let package_json_path = project_root.join("package.json");
    if !package_json_path.exists() {
        let package_json_content = templates::package_json_pretty(project_name);
        std::fs::write(&package_json_path, package_json_content)
            .map_err(|e| miette::miette!("Failed to write package.json: {}", e))?;
        println!("  {} Created package.json", "✓".green());
    }

    // Update package.json with dependencies from manifest
    if !ts_config.dependencies.is_empty() {
        update_package_json_deps(project_root, ts_config)?;
    }

    // Run npm install
    println!("  {} Running npm install...", "→".blue());
    let status = Command::new("npm")
        .args(["install"])
        .current_dir(project_root)
        .status()
        .map_err(|e| miette::miette!("Failed to run npm install: {}", e))?;

    if status.success() {
        println!("  {} TypeScript dependencies ready", "✓".green());
    }

    println!();
    Ok(())
}

/// Update package.json with dependencies from the manifest
fn update_package_json_deps(
    project_root: &Path,
    ts_config: &manifest::TsConfig,
) -> Result<()> {
    let package_json_path = project_root.join("package.json");
    let content = std::fs::read_to_string(&package_json_path)
        .map_err(|e| miette::miette!("Failed to read package.json: {}", e))?;

    let mut package: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| miette::miette!("Failed to parse package.json: {}", e))?;

    // Ensure dependencies object exists
    if package.get("dependencies").is_none() {
        package["dependencies"] = serde_json::json!({});
    }

    // Add dependencies from manifest
    if let Some(deps) = package["dependencies"].as_object_mut() {
        for (name, version) in &ts_config.dependencies {
            deps.insert(name.clone(), serde_json::Value::String(version.clone()));
        }
    }

    // Write back
    let updated = serde_json::to_string_pretty(&package)
        .map_err(|e| miette::miette!("Failed to serialize package.json: {}", e))?;

    std::fs::write(&package_json_path, updated)
        .map_err(|e| miette::miette!("Failed to write package.json: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dep_spec_with_version() {
        let (pkg, ver) = parse_dep_spec("github.com/pkg/errors@v0.9.1");
        assert_eq!(pkg, "github.com/pkg/errors");
        assert_eq!(ver, "v0.9.1");
    }

    #[test]
    fn test_parse_dep_spec_without_version() {
        let (pkg, ver) = parse_dep_spec("viem");
        assert_eq!(pkg, "viem");
        assert_eq!(ver, "latest");
    }

    #[test]
    fn test_parse_dep_spec_npm_style() {
        let (pkg, ver) = parse_dep_spec("@types/node@^20.0.0");
        assert_eq!(pkg, "@types/node");
        assert_eq!(ver, "^20.0.0");
    }
}
