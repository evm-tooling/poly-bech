//! Dependency management for poly-bench projects

use crate::{manifest, runtime_env_go, runtime_env_ts, templates, terminal};
use miette::Result;
use std::path::Path;
use std::process::Command;

/// Resolve the directory used for Go (runtime-env if present, else project root)
fn resolve_go_root(project_root: &Path) -> std::path::PathBuf {
    let env_go = runtime_env_go(project_root);
    if env_go.exists() {
        env_go
    } else {
        project_root.to_path_buf()
    }
}

/// Resolve the directory used for TypeScript (runtime-env if present, else project root)
fn resolve_ts_root(project_root: &Path) -> std::path::PathBuf {
    let env_ts = runtime_env_ts(project_root);
    if env_ts.exists() {
        env_ts
    } else {
        project_root.to_path_buf()
    }
}

/// Ensure go.mod exists in go_root (for add when runtime-env exists but empty)
fn ensure_go_env(go_root: &Path, go_config: &manifest::GoConfig) -> Result<()> {
    std::fs::create_dir_all(go_root)
        .map_err(|e| miette::miette!("Failed to create Go env dir: {}", e))?;
    if !go_root.join("go.mod").exists() {
        let content = templates::go_mod(&go_config.module, go_config.version.as_deref());
        std::fs::write(go_root.join("go.mod"), content)
            .map_err(|e| miette::miette!("Failed to write go.mod: {}", e))?;
    }
    Ok(())
}

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

    let project_root = crate::find_project_root(&current_dir)
        .ok_or_else(|| miette::miette!("Not in a poly-bench project"))?;

    let mut manifest = crate::load_manifest(&project_root)?;

    if !manifest.has_go() {
        return Err(miette::miette!("Go is not enabled in this project"));
    }

    let (package, version) = parse_dep_spec(spec);

    // Add to manifest
    manifest.add_go_dependency(&package, &version)?;
    crate::save_manifest(&project_root, &manifest)?;

    let go_root = resolve_go_root(&project_root);
    ensure_go_env(&go_root, manifest.go.as_ref().unwrap())?;

    // Use module/...@version so Go fetches all packages and transitive deps into go.sum
    let go_get_arg = go_get_spec_for_transitives(&package, &version);
    let spinner = terminal::step_spinner(&format!("Installing {}...", go_get_arg));

    let output = terminal::run_command_with_spinner(
        &spinner,
        Command::new("go")
            .args(["get", &go_get_arg])
            .current_dir(&go_root),
    )
    .map_err(|e| miette::miette!("Failed to run go get: {}", e))?;

    if !output.status.success() {
        let err_msg = terminal::first_error_line(&output.stderr);
        terminal::finish_failure(&spinner, &format!("go get failed: {}", err_msg));
        return Err(miette::miette!("go get failed"));
    }

    // Note: We intentionally skip `go mod tidy` here.
    // Running tidy without a .go file that imports the deps would remove them.
    // Tidy will run automatically when `poly-bench run` generates bench code.

    terminal::finish_success(
        &spinner,
        &format!("Added {}@{} to polybench.toml", package, version),
    );

    Ok(())
}

/// Add a TypeScript dependency to the project
pub fn add_ts_dependency(spec: &str) -> Result<()> {
    let current_dir = std::env::current_dir()
        .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

    let project_root = crate::find_project_root(&current_dir)
        .ok_or_else(|| miette::miette!("Not in a poly-bench project"))?;

    let mut manifest = crate::load_manifest(&project_root)?;

    if !manifest.has_ts() {
        return Err(miette::miette!("TypeScript is not enabled in this project"));
    }

    let (package, version) = parse_dep_spec(spec);

    // Add to manifest
    manifest.add_ts_dependency(&package, &version)?;
    crate::save_manifest(&project_root, &manifest)?;

    let ts_root = resolve_ts_root(&project_root);
    std::fs::create_dir_all(&ts_root)
        .map_err(|e| miette::miette!("Failed to create TS env dir: {}", e))?;
    if !ts_root.join("package.json").exists() {
        let pkg = templates::package_json_pretty(&manifest.project.name);
        std::fs::write(ts_root.join("package.json"), pkg)
            .map_err(|e| miette::miette!("Failed to write package.json: {}", e))?;
    }
    update_package_json_deps(&ts_root, manifest.ts.as_ref().unwrap())?;

    // Run npm install
    let npm_spec = if version == "latest" {
        package.clone()
    } else {
        format!("{}@{}", package, version)
    };

    let spinner = terminal::step_spinner(&format!("Installing {}...", npm_spec));

    let output = terminal::run_command_with_spinner(
        &spinner,
        Command::new("npm")
            .args(["install", &npm_spec])
            .current_dir(&ts_root),
    )
    .map_err(|e| miette::miette!("Failed to run npm install: {}", e))?;

    if !output.status.success() {
        let err_msg = terminal::first_error_line(&output.stderr);
        terminal::finish_failure(&spinner, &format!("npm install failed: {}", err_msg));
        return Err(miette::miette!("npm install failed"));
    }

    terminal::finish_success(
        &spinner,
        &format!("Added {}@{} to polybench.toml", package, version),
    );

    Ok(())
}

/// Install all dependencies from polybench.toml
pub fn install_all() -> Result<()> {
    let current_dir = std::env::current_dir()
        .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

    let project_root = crate::find_project_root(&current_dir)
        .ok_or_else(|| miette::miette!("Not in a poly-bench project"))?;

    let manifest = crate::load_manifest(&project_root)?;

    let spinner = terminal::step_spinner(&format!(
        "Installing dependencies for {}...",
        manifest.project.name
    ));
    terminal::ensure_min_display(&spinner);
    spinner.finish_and_clear();

    // Install Go dependencies
    if let Some(ref go_config) = manifest.go {
        install_go_deps(&project_root, go_config)?;
    }

    // Install TypeScript dependencies
    if let Some(ref ts_config) = manifest.ts {
        install_ts_deps(&project_root, ts_config, &manifest.project.name)?;
    }

    println!();
    terminal::success("All dependencies installed!");

    Ok(())
}

/// Install Go dependencies
fn install_go_deps(project_root: &Path, go_config: &manifest::GoConfig) -> Result<()> {
    terminal::section("Go dependencies");

    let go_root = resolve_go_root(project_root);
    std::fs::create_dir_all(&go_root)
        .map_err(|e| miette::miette!("Failed to create Go env dir: {}", e))?;

    let go_mod_path = go_root.join("go.mod");
    if !go_mod_path.exists() {
        let go_mod_content = templates::go_mod(&go_config.module, go_config.version.as_deref());
        std::fs::write(&go_mod_path, go_mod_content)
            .map_err(|e| miette::miette!("Failed to write go.mod: {}", e))?;
        terminal::success_indented("Created go.mod");
    }

    // Run go get for each dependency (use module/...@version so transitives go into go.sum)
    for (package, version) in &go_config.dependencies {
        let go_get_arg = go_get_spec_for_transitives(package, version);
        let spinner = terminal::indented_spinner(&format!("Installing {}...", package));

        let output = terminal::run_command_with_spinner(
            &spinner,
            Command::new("go")
                .args(["get", &go_get_arg])
                .current_dir(&go_root),
        )
        .map_err(|e| miette::miette!("Failed to run go get: {}", e))?;

        if !output.status.success() {
            let err_msg = terminal::first_error_line(&output.stderr);
            terminal::finish_failure_indented(
                &spinner,
                &format!("Failed to install {}: {}", package, err_msg),
            );
            return Err(miette::miette!("go get {} failed: {}", go_get_arg, err_msg));
        }
        terminal::finish_success_indented(&spinner, package);
    }

    // Note: We skip `go mod tidy` here - it would remove deps since there's no .go file yet.
    // Tidy runs automatically when `poly-bench run` generates the benchmark code.

    terminal::success_indented("Go dependencies ready");

    Ok(())
}

/// Install TypeScript dependencies
fn install_ts_deps(
    project_root: &Path,
    ts_config: &manifest::TsConfig,
    project_name: &str,
) -> Result<()> {
    terminal::section("TypeScript dependencies");

    let ts_root = resolve_ts_root(project_root);
    std::fs::create_dir_all(&ts_root)
        .map_err(|e| miette::miette!("Failed to create TS env dir: {}", e))?;

    let package_json_path = ts_root.join("package.json");
    if !package_json_path.exists() {
        let package_json_content = templates::package_json_pretty(project_name);
        std::fs::write(&package_json_path, package_json_content)
            .map_err(|e| miette::miette!("Failed to write package.json: {}", e))?;
        terminal::success_indented("Created package.json");
    }

    if !ts_config.dependencies.is_empty() {
        update_package_json_deps(&ts_root, ts_config)?;
    }

    let spinner = terminal::indented_spinner("Running npm install...");
    let output = terminal::run_command_with_spinner(
        &spinner,
        Command::new("npm").args(["install"]).current_dir(&ts_root),
    )
    .map_err(|e| miette::miette!("Failed to run npm install: {}", e))?;

    if output.status.success() {
        terminal::finish_success_indented(&spinner, "TypeScript dependencies ready");
    } else {
        let err_msg = terminal::first_error_line(&output.stderr);
        terminal::finish_warning_indented(&spinner, &format!("npm install failed: {}", err_msg));
    }

    Ok(())
}

/// Update package.json with dependencies from the manifest
fn update_package_json_deps(project_root: &Path, ts_config: &manifest::TsConfig) -> Result<()> {
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
