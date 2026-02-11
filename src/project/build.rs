//! Build/regenerate the .polybench runtime environment
//!
//! This module recreates the `.polybench/runtime-env/` directory from:
//! - The `polybench.toml` manifest (languages, dependencies)
//! - Existing `.bench` files (for parsing imports if needed)
//!
//! Use this when the .polybench directory is deleted, corrupted, or after cloning
//! a repo where it was gitignored.

use crate::project::{self, manifest, runtime_env_go, runtime_env_ts, templates};
use colored::Colorize;
use miette::Result;
use std::path::Path;
use std::process::Command;

/// Options for the build command
pub struct BuildOptions {
    /// Force rebuild even if files exist
    pub force: bool,
    /// Skip npm/go install steps
    pub skip_install: bool,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            force: false,
            skip_install: false,
        }
    }
}

/// Build/regenerate the .polybench runtime environment
pub fn build_project(options: &BuildOptions) -> Result<()> {
    let current_dir = std::env::current_dir()
        .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

    let project_root = project::find_project_root(&current_dir)
        .ok_or_else(|| miette::miette!(
            "Not in a poly-bench project. Run 'poly-bench init' first."
        ))?;

    let manifest = project::load_manifest(&project_root)?;

    println!(
        "{} Building runtime environment for '{}'",
        "▶".green().bold(),
        manifest.project.name
    );
    println!();

    // Build Go environment
    if manifest.has_go() {
        build_go_env(&project_root, manifest.go.as_ref().unwrap(), options)?;
    }

    // Build TypeScript environment
    if manifest.has_ts() {
        build_ts_env(&project_root, manifest.ts.as_ref().unwrap(), &manifest.project.name, options)?;
    }

    println!();
    println!(
        "{} Runtime environment ready!",
        "✓".green().bold()
    );

    Ok(())
}

/// Build/regenerate the Go runtime environment
fn build_go_env(
    project_root: &Path,
    go_config: &manifest::GoConfig,
    options: &BuildOptions,
) -> Result<()> {
    println!("{} Go environment", "→".blue().bold());

    let go_env = runtime_env_go(project_root);

    // Create directory
    std::fs::create_dir_all(&go_env)
        .map_err(|e| miette::miette!("Failed to create {}: {}", go_env.display(), e))?;

    let go_mod_path = go_env.join("go.mod");
    let go_mod_exists = go_mod_path.exists();

    // Create or recreate go.mod
    if !go_mod_exists || options.force {
        let go_mod_content = templates::go_mod(&go_config.module, go_config.version.as_deref());
        std::fs::write(&go_mod_path, &go_mod_content)
            .map_err(|e| miette::miette!("Failed to write go.mod: {}", e))?;
        
        if go_mod_exists && options.force {
            println!("  {} Regenerated go.mod", "✓".green());
        } else {
            println!("  {} Created go.mod", "✓".green());
        }
    } else {
        println!("  {} go.mod exists (use --force to regenerate)", "·".dimmed());
    }

    // Install dependencies if not skipped
    if !options.skip_install && !go_config.dependencies.is_empty() {
        println!("  {} Installing {} Go dependencies...", "→".blue(), go_config.dependencies.len());
        
        for (package, version) in &go_config.dependencies {
            let go_get_arg = go_get_spec_for_transitives(package, version);
            
            let output = Command::new("go")
                .args(["get", &go_get_arg])
                .current_dir(&go_env)
                .output()
                .map_err(|e| miette::miette!("Failed to run go get: {}", e))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!(
                    "  {} Failed to install {}: {}",
                    "⚠".yellow(),
                    package,
                    stderr.trim()
                );
            } else {
                println!("  {} Installed {}", "✓".green(), package);
            }
        }
    } else if options.skip_install {
        println!("  {} Skipping go get (--skip-install)", "·".dimmed());
    }

    println!("  {} Go environment ready", "✓".green());
    println!();

    Ok(())
}

/// Build/regenerate the TypeScript runtime environment
fn build_ts_env(
    project_root: &Path,
    ts_config: &manifest::TsConfig,
    project_name: &str,
    options: &BuildOptions,
) -> Result<()> {
    println!("{} TypeScript environment", "→".blue().bold());

    let ts_env = runtime_env_ts(project_root);

    // Create directory
    std::fs::create_dir_all(&ts_env)
        .map_err(|e| miette::miette!("Failed to create {}: {}", ts_env.display(), e))?;

    let package_json_path = ts_env.join("package.json");
    let package_json_exists = package_json_path.exists();

    // Create or recreate package.json
    if !package_json_exists || options.force {
        let package_json_content = templates::package_json_pretty(project_name);
        std::fs::write(&package_json_path, &package_json_content)
            .map_err(|e| miette::miette!("Failed to write package.json: {}", e))?;
        
        if package_json_exists && options.force {
            println!("  {} Regenerated package.json", "✓".green());
        } else {
            println!("  {} Created package.json", "✓".green());
        }
    } else {
        println!("  {} package.json exists (use --force to regenerate)", "·".dimmed());
    }

    let tsconfig_path = ts_env.join("tsconfig.json");
    let tsconfig_exists = tsconfig_path.exists();

    // Create or recreate tsconfig.json
    if !tsconfig_exists || options.force {
        let tsconfig_content = templates::tsconfig_json();
        std::fs::write(&tsconfig_path, &tsconfig_content)
            .map_err(|e| miette::miette!("Failed to write tsconfig.json: {}", e))?;
        
        if tsconfig_exists && options.force {
            println!("  {} Regenerated tsconfig.json", "✓".green());
        } else {
            println!("  {} Created tsconfig.json", "✓".green());
        }
    } else {
        println!("  {} tsconfig.json exists (use --force to regenerate)", "·".dimmed());
    }

    // Add user dependencies from manifest to package.json
    if !ts_config.dependencies.is_empty() {
        update_package_json_deps(&ts_env, ts_config)?;
        println!(
            "  {} Added {} dependencies to package.json",
            "✓".green(),
            ts_config.dependencies.len()
        );
    }

    // Run npm install if not skipped
    if !options.skip_install {
        println!("  {} Running npm install...", "→".blue());
        
        let output = Command::new("npm")
            .args(["install"])
            .current_dir(&ts_env)
            .output();

        match output {
            Ok(out) if out.status.success() => {
                println!("  {} Installed npm dependencies", "✓".green());
            }
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                eprintln!(
                    "  {} npm install failed: {}",
                    "⚠".yellow(),
                    stderr.trim()
                );
                eprintln!("    Run 'npm install' manually in {}", ts_env.display());
            }
            Err(e) => {
                eprintln!(
                    "  {} Could not run npm: {}",
                    "⚠".yellow(),
                    e
                );
                eprintln!("    Run 'npm install' manually in {}", ts_env.display());
            }
        }
    } else {
        println!("  {} Skipping npm install (--skip-install)", "·".dimmed());
    }

    println!("  {} TypeScript environment ready", "✓".green());
    println!();

    Ok(())
}

/// Build the argument for `go get` so that transitive deps are added to go.sum
fn go_get_spec_for_transitives(package: &str, version: &str) -> String {
    let module = if package.contains('/') {
        let parts: Vec<&str> = package.split('/').collect();
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

/// Update package.json with dependencies from the manifest
fn update_package_json_deps(
    ts_root: &Path,
    ts_config: &manifest::TsConfig,
) -> Result<()> {
    let package_json_path = ts_root.join("package.json");
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
    use tempfile::TempDir;

    #[test]
    fn test_build_creates_runtime_env() {
        let temp = TempDir::new().unwrap();
        let project_path = temp.path();

        // Create a minimal project manually
        let manifest_content = r#"
[project]
name = "test-project"

[go]
module = "test-project"

[ts]
"#;
        std::fs::write(project_path.join("polybench.toml"), manifest_content).unwrap();

        // Change to project directory and build
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(project_path).unwrap();

        let options = BuildOptions {
            force: false,
            skip_install: true, // Skip actual installs in tests
        };

        let result = build_project(&options);
        
        // Restore directory
        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert!(project_path.join(".polybench/runtime-env/go/go.mod").exists());
        assert!(project_path.join(".polybench/runtime-env/ts/package.json").exists());
        assert!(project_path.join(".polybench/runtime-env/ts/tsconfig.json").exists());
    }
}
