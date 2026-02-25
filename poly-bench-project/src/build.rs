//! Build/regenerate the .polybench runtime environment
//!
//! This module recreates the `.polybench/runtime-env/` directory from:
//! - The `polybench.toml` manifest (languages, dependencies)
//! - Existing `.bench` files (for parsing imports if needed)
//!
//! Use this when the .polybench directory is deleted, corrupted, or after cloning
//! a repo where it was gitignored.

use crate::{
    error::ProjectError, manifest, runtime_env_go, runtime_env_python, runtime_env_rust,
    runtime_env_ts, templates, terminal,
};
use miette::Result;
use std::{
    path::Path,
    process::{Command, Output},
};

/// Options for the build command
pub struct BuildOptions {
    /// Force rebuild even if files exist
    pub force: bool,
    /// Skip npm/go install steps
    pub skip_install: bool,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self { force: false, skip_install: false }
    }
}

fn command_status_string(output: &Output) -> String {
    output
        .status
        .code()
        .map(|c| format!("exit code {}", c))
        .unwrap_or_else(|| "terminated by signal".to_string())
}

fn command_failure(command: &str, cwd: &Path, output: &Output, hint: &str) -> miette::Report {
    miette::miette!(
        "{}",
        ProjectError::command_failed(
            command,
            cwd.display().to_string(),
            command_status_string(output),
            terminal::stderr_excerpt(&output.stderr, 12),
            hint
        )
    )
}

/// Build/regenerate the .polybench runtime environment
pub fn build_project(options: &BuildOptions) -> Result<()> {
    let current_dir = std::env::current_dir()
        .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

    let project_root = crate::find_project_root(&current_dir).ok_or_else(|| {
        miette::miette!("Not in a poly-bench project. Run 'poly-bench init' first.")
    })?;

    let manifest = crate::load_manifest(&project_root)?;

    let spinner = terminal::step_spinner(&format!(
        "Building runtime environment for '{}'...",
        manifest.project.name
    ));
    terminal::ensure_min_display(&spinner);
    spinner.finish_and_clear();

    // Build Go environment
    if manifest.has_go() {
        build_go_env(&project_root, manifest.go.as_ref().unwrap(), options)?;
    }

    // Build TypeScript environment
    if manifest.has_ts() {
        build_ts_env(
            &project_root,
            manifest.ts.as_ref().unwrap(),
            &manifest.project.name,
            options,
        )?;
    }

    // Build Rust environment
    if manifest.has_rust() {
        build_rust_env(&project_root, manifest.rust.as_ref().unwrap(), options)?;
    }

    // Build Python environment
    if manifest.has_python() {
        build_python_env(&project_root, manifest.python.as_ref().unwrap(), options)?;
    }

    println!();
    terminal::success("Runtime environment ready!");

    Ok(())
}

/// Build/regenerate the Go runtime environment
fn build_go_env(
    project_root: &Path,
    go_config: &manifest::GoConfig,
    options: &BuildOptions,
) -> Result<()> {
    terminal::section("Go environment");

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
            terminal::success_indented("Regenerated go.mod");
        } else {
            terminal::success_indented("Created go.mod");
        }
    } else {
        terminal::info_indented("go.mod exists (use --force to regenerate)");
    }

    // Install dependencies if not skipped
    if !options.skip_install && !go_config.dependencies.is_empty() {
        for (package, version) in &go_config.dependencies {
            let go_get_arg = go_get_spec_for_transitives(package, version);
            let spinner = terminal::indented_spinner(&format!("Installing {}...", package));

            let output = terminal::run_command_with_spinner(
                &spinner,
                Command::new("go").args(["get", &go_get_arg]).current_dir(&go_env),
            )
            .map_err(|e| miette::miette!("Failed to run go get: {}", e))?;

            if !output.status.success() {
                terminal::finish_failure_indented(
                    &spinner,
                    &format!("Failed to install {}", package),
                );
                terminal::print_stderr_excerpt(&output.stderr, 6);
                return Err(command_failure(
                    &format!("go get {}", go_get_arg),
                    &go_env,
                    &output,
                    "Fix Go dependency resolution issues before continuing.",
                ));
            } else {
                terminal::finish_success_indented(&spinner, package);
            }
        }
    } else if options.skip_install {
        terminal::info_indented("Skipping go get (--skip-install)");
    }

    terminal::success_indented("Go environment ready");

    Ok(())
}

/// Build/regenerate the TypeScript runtime environment
fn build_ts_env(
    project_root: &Path,
    ts_config: &manifest::TsConfig,
    project_name: &str,
    options: &BuildOptions,
) -> Result<()> {
    terminal::section("TypeScript environment");

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
            terminal::success_indented("Regenerated package.json");
        } else {
            terminal::success_indented("Created package.json");
        }
    } else {
        terminal::info_indented("package.json exists (use --force to regenerate)");
    }

    let tsconfig_path = ts_env.join("tsconfig.json");
    let tsconfig_exists = tsconfig_path.exists();

    // Create or recreate tsconfig.json
    if !tsconfig_exists || options.force {
        let tsconfig_content = templates::tsconfig_json();
        std::fs::write(&tsconfig_path, &tsconfig_content)
            .map_err(|e| miette::miette!("Failed to write tsconfig.json: {}", e))?;

        if tsconfig_exists && options.force {
            terminal::success_indented("Regenerated tsconfig.json");
        } else {
            terminal::success_indented("Created tsconfig.json");
        }
    } else {
        terminal::info_indented("tsconfig.json exists (use --force to regenerate)");
    }

    // Add user dependencies from manifest to package.json
    if !ts_config.dependencies.is_empty() {
        update_package_json_deps(&ts_env, ts_config)?;
        terminal::success_indented(&format!(
            "Added {} dependencies to package.json",
            ts_config.dependencies.len()
        ));
    }

    // Run npm install if not skipped
    if !options.skip_install {
        let spinner = terminal::indented_spinner("Running npm install...");

        let output = terminal::run_command_with_spinner(
            &spinner,
            Command::new("npm").args(["install"]).current_dir(&ts_env),
        );

        match output {
            Ok(out) if out.status.success() => {
                terminal::finish_success_indented(&spinner, "npm dependencies installed");
            }
            Ok(out) => {
                terminal::finish_failure_indented(&spinner, "npm install failed");
                terminal::print_stderr_excerpt(&out.stderr, 6);
                return Err(command_failure(
                    "npm install",
                    &ts_env,
                    &out,
                    "Fix npm install errors and rerun build.",
                ));
            }
            Err(e) => {
                terminal::finish_warning_indented(&spinner, &format!("Could not run npm: {}", e));
                return Err(miette::miette!(
                    "Could not run npm install in {}: {}",
                    ts_env.display(),
                    e
                ));
            }
        }
    } else {
        terminal::info_indented("Skipping npm install (--skip-install)");
    }

    terminal::success_indented("TypeScript environment ready");

    Ok(())
}

/// Build/regenerate the Rust runtime environment
fn build_rust_env(
    project_root: &Path,
    rust_config: &manifest::RustConfig,
    options: &BuildOptions,
) -> Result<()> {
    terminal::section("Rust environment");

    let rust_env = runtime_env_rust(project_root);

    // Create directory structure
    std::fs::create_dir_all(rust_env.join("src"))
        .map_err(|e| miette::miette!("Failed to create {}/src: {}", rust_env.display(), e))?;

    let cargo_toml_path = rust_env.join("Cargo.toml");
    let cargo_toml_exists = cargo_toml_path.exists();

    // Create or recreate Cargo.toml
    if !cargo_toml_exists || options.force {
        let cargo_toml_content = templates::cargo_toml("polybench-runner", &rust_config.edition);
        std::fs::write(&cargo_toml_path, &cargo_toml_content)
            .map_err(|e| miette::miette!("Failed to write Cargo.toml: {}", e))?;

        if cargo_toml_exists && options.force {
            terminal::success_indented("Regenerated Cargo.toml");
        } else {
            terminal::success_indented("Created Cargo.toml");
        }
    } else {
        terminal::info_indented("Cargo.toml exists (use --force to regenerate)");
    }

    // Create placeholder main.rs if it doesn't exist
    let main_rs_path = rust_env.join("src").join("main.rs");
    if !main_rs_path.exists() {
        std::fs::write(&main_rs_path, "fn main() {}\n")
            .map_err(|e| miette::miette!("Failed to write src/main.rs: {}", e))?;
        terminal::success_indented("Created src/main.rs");
    }

    // Add user dependencies from manifest to Cargo.toml
    if !rust_config.dependencies.is_empty() {
        update_cargo_toml_deps(&rust_env, rust_config)?;
        terminal::success_indented(&format!(
            "Added {} dependencies to Cargo.toml",
            rust_config.dependencies.len()
        ));
    }

    // Run cargo check to download dependencies if not skipped
    if !options.skip_install && !rust_config.dependencies.is_empty() {
        let spinner = terminal::indented_spinner("Running cargo fetch...");

        let output = terminal::run_command_with_spinner(
            &spinner,
            Command::new("cargo").args(["fetch"]).current_dir(&rust_env),
        );

        match output {
            Ok(out) if out.status.success() => {
                terminal::finish_success_indented(&spinner, "Cargo dependencies fetched");
            }
            Ok(out) => {
                terminal::finish_failure_indented(&spinner, "cargo fetch failed");
                terminal::print_stderr_excerpt(&out.stderr, 6);
                return Err(command_failure(
                    "cargo fetch",
                    &rust_env,
                    &out,
                    "Fix Cargo fetch issues and rerun build.",
                ));
            }
            Err(e) => {
                terminal::finish_warning_indented(&spinner, &format!("Could not run cargo: {}", e));
                return Err(miette::miette!(
                    "Could not run cargo fetch in {}: {}",
                    rust_env.display(),
                    e
                ));
            }
        }
    } else if options.skip_install {
        terminal::info_indented("Skipping cargo fetch (--skip-install)");
    }

    terminal::success_indented("Rust environment ready");

    Ok(())
}

/// Build/regenerate the Python runtime environment
///
/// Creates a virtual environment (.venv) in the python runtime-env directory and installs
/// dependencies into it. This ensures benchmarks run with the same Python that has the
/// installed packages (avoids pip/python mismatch with pyenv, conda, or system Python).
fn build_python_env(
    project_root: &Path,
    python_config: &manifest::PythonConfig,
    options: &BuildOptions,
) -> Result<()> {
    terminal::section("Python environment");

    let python_env = runtime_env_python(project_root);

    std::fs::create_dir_all(&python_env)
        .map_err(|e| miette::miette!("Failed to create {}: {}", python_env.display(), e))?;

    let requirements_path = python_env.join("requirements.txt");
    let deps: Vec<(String, String)> =
        python_config.dependencies.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    let requirements_content = templates::requirements_txt_for_runtime_env(&deps);
    std::fs::write(&requirements_path, requirements_content)
        .map_err(|e| miette::miette!("Failed to write requirements.txt: {}", e))?;

    if python_config.dependencies.is_empty() {
        terminal::success_indented("Created requirements.txt (no dependencies)");
    } else {
        terminal::success_indented(&format!(
            "Created requirements.txt ({} dependencies)",
            python_config.dependencies.len()
        ));
    }

    if !options.skip_install {
        let venv_path = python_env.join(".venv");
        let venv_python = venv_path.join("bin").join("python");
        let venv_pip = venv_path.join("bin").join("pip");

        // Create venv if it doesn't exist (needed for deps and LSP tooling)
        if !venv_python.exists() {
            let spinner = terminal::indented_spinner("Creating Python virtual environment...");
            let python_cmd =
                which::which("python3").or_else(|_| which::which("python")).map_err(|_| {
                    miette::miette!("Python not found in PATH. Please install Python 3.")
                })?;

            let output = terminal::run_command_with_spinner(
                &spinner,
                Command::new(&python_cmd).args(["-m", "venv", ".venv"]).current_dir(&python_env),
            );

            match output {
                Ok(out) if out.status.success() => {
                    terminal::finish_success_indented(&spinner, "Virtual environment created");
                }
                Ok(out) => {
                    terminal::finish_failure_indented(&spinner, "venv creation failed");
                    terminal::print_stderr_excerpt(&out.stderr, 8);
                    return Err(command_failure(
                        "python -m venv .venv",
                        &python_env,
                        &out,
                        "Ensure Python 3 venv module is available (python3-venv on Debian/Ubuntu).",
                    ));
                }
                Err(e) => {
                    terminal::finish_warning_indented(
                        &spinner,
                        &format!("Could not create venv: {}", e),
                    );
                    return Err(miette::miette!(
                        "Could not create venv in {}: {}",
                        python_env.display(),
                        e
                    ));
                }
            }
        } else {
            terminal::info_indented("Virtual environment exists");
        }

        // Install dependencies into venv (always run: requirements.txt includes pyright for LSP)
        let spinner = terminal::indented_spinner("Installing Python dependencies...");

        let output = terminal::run_command_with_spinner(
            &spinner,
            Command::new(&venv_pip)
                .args(["install", "-r", "requirements.txt"])
                .current_dir(&python_env),
        );

        match output {
            Ok(out) if out.status.success() => {
                terminal::finish_success_indented(&spinner, "Python dependencies installed");
            }
            Ok(out) => {
                terminal::finish_failure_indented(&spinner, "pip install failed");
                terminal::print_stderr_excerpt(&out.stderr, 8);
                return Err(command_failure(
                    "pip install -r requirements.txt",
                    &python_env,
                    &out,
                    "Ensure pip is available in the venv and fix dependency issues.",
                ));
            }
            Err(e) => {
                terminal::finish_warning_indented(&spinner, &format!("Could not run pip: {}", e));
                return Err(miette::miette!(
                    "Could not run pip install in {}: {}",
                    python_env.display(),
                    e
                ));
            }
        }
    } else if options.skip_install {
        terminal::info_indented("Skipping pip install (--skip-install)");
    }

    terminal::success_indented("Python environment ready");

    Ok(())
}

/// Update Cargo.toml with dependencies from the manifest
fn update_cargo_toml_deps(rust_root: &Path, rust_config: &manifest::RustConfig) -> Result<()> {
    use std::process::Command;

    for (name, dep) in &rust_config.dependencies {
        let version = dep.version();

        // Build cargo add arguments
        let mut args = vec!["add".to_string()];

        // If version is "latest", just use the crate name; otherwise use name@version
        if version == "latest" {
            args.push(name.clone());
        } else {
            args.push(format!("{}@{}", name, version));
        }

        // Add features if present
        if let Some(features) = dep.features() {
            if !features.is_empty() {
                args.push("--features".to_string());
                args.push(features.join(","));
            }
        }

        // Run cargo add
        let output = Command::new("cargo")
            .args(&args)
            .current_dir(rust_root)
            .output()
            .map_err(|e| miette::miette!("Failed to run cargo add: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            // Don't fail if the dependency already exists with same version
            if !stderr.contains("already present") {
                return Err(miette::miette!(
                    "cargo add {} failed: {}",
                    name,
                    stderr.lines().next().unwrap_or("unknown error")
                ));
            }
        }
    }

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
fn update_package_json_deps(ts_root: &Path, ts_config: &manifest::TsConfig) -> Result<()> {
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
