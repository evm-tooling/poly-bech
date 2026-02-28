//! Dependency management for poly-bench projects

use crate::{error::ProjectError, manifest, runtime_env, templates, terminal};
use miette::Result;
use poly_bench_dsl::Lang;
use std::{
    path::Path,
    process::{Command, Output},
};

/// Resolve the directory used for a runtime (runtime-env if present, else project root)
fn resolve_runtime_root(project_root: &Path, lang: Lang) -> std::path::PathBuf {
    let env = runtime_env(project_root, lang);
    if env.exists() {
        env
    } else {
        project_root.to_path_buf()
    }
}

/// Ensure Python venv exists in python_root and return path to pip (venv's pip or "pip")
fn ensure_python_venv_and_get_pip(python_root: &Path) -> Result<std::path::PathBuf> {
    let venv_path = python_root.join(".venv");
    let venv_pip = venv_path.join("bin").join("pip");

    if venv_pip.exists() {
        return Ok(venv_pip);
    }

    // Create venv
    let python_cmd = which::which("python3")
        .or_else(|_| which::which("python"))
        .map_err(|_| miette::miette!("Python not found in PATH. Please install Python 3."))?;

    let output = Command::new(&python_cmd)
        .args(["-m", "venv", ".venv"])
        .current_dir(python_root)
        .output()
        .map_err(|e| miette::miette!("Failed to create venv: {}", e))?;

    if !output.status.success() {
        return Err(command_failure(
            "python -m venv .venv",
            python_root,
            &output,
            "Ensure Python 3 venv module is available (python3-venv on Debian/Ubuntu).",
        ));
    }

    Ok(venv_pip)
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
    if !crate::runtime_check::is_lang_installed(Lang::Go) {
        return Err(crate::runtime_check::not_installed_error(Lang::Go));
    }
    let current_dir = std::env::current_dir()
        .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

    let project_root = crate::find_project_root(&current_dir)
        .ok_or_else(|| miette::miette!("Not in a poly-bench project"))?;

    let mut manifest = crate::load_manifest(&project_root)?;

    if !manifest.has_runtime(Lang::Go) {
        return Err(miette::miette!("Go is not enabled in this project"));
    }

    let (package, version) = parse_dep_spec(spec);

    // Add to manifest
    manifest.add_go_dependency(&package, &version)?;
    crate::save_manifest(&project_root, &manifest)?;

    let go_root = resolve_runtime_root(&project_root, Lang::Go);
    ensure_go_env(&go_root, manifest.go.as_ref().unwrap())?;

    // Use module/...@version so Go fetches all packages and transitive deps into go.sum
    let go_get_arg = go_get_spec_for_transitives(&package, &version);
    let spinner = terminal::step_spinner(&format!("Installing {}...", go_get_arg));

    let output = terminal::run_command_with_spinner(
        &spinner,
        Command::new("go").args(["get", &go_get_arg]).current_dir(&go_root),
    )
    .map_err(|e| miette::miette!("Failed to run go get: {}", e))?;

    if !output.status.success() {
        terminal::finish_failure(&spinner, "go get failed");
        terminal::print_stderr_excerpt(&output.stderr, 8);
        return Err(command_failure(
            &format!("go get {}", go_get_arg),
            &go_root,
            &output,
            "Verify the package path/version and ensure network access to Go modules.",
        ));
    }

    // Note: We intentionally skip `go mod tidy` here.
    // Running tidy without a .go file that imports the deps would remove them.
    // Tidy will run automatically when `poly-bench run` generates bench code.

    terminal::finish_success(&spinner, &format!("Added {}@{} to polybench.toml", package, version));

    Ok(())
}

/// Add a TypeScript dependency to the project
pub fn add_ts_dependency(spec: &str) -> Result<()> {
    if !crate::runtime_check::is_lang_installed(Lang::TypeScript) {
        return Err(crate::runtime_check::not_installed_error(Lang::TypeScript));
    }
    let current_dir = std::env::current_dir()
        .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

    let project_root = crate::find_project_root(&current_dir)
        .ok_or_else(|| miette::miette!("Not in a poly-bench project"))?;

    let mut manifest = crate::load_manifest(&project_root)?;

    if !manifest.has_runtime(Lang::TypeScript) {
        return Err(miette::miette!("TypeScript is not enabled in this project"));
    }

    let (package, version) = parse_dep_spec(spec);

    // Add to manifest
    manifest.add_ts_dependency(&package, &version)?;
    crate::save_manifest(&project_root, &manifest)?;

    let ts_root = resolve_runtime_root(&project_root, Lang::TypeScript);
    std::fs::create_dir_all(&ts_root)
        .map_err(|e| miette::miette!("Failed to create TS env dir: {}", e))?;
    if !ts_root.join("package.json").exists() {
        let pkg = templates::package_json_pretty(&manifest.project.name);
        std::fs::write(ts_root.join("package.json"), pkg)
            .map_err(|e| miette::miette!("Failed to write package.json: {}", e))?;
    }
    update_package_json_deps(&ts_root, manifest.ts.as_ref().unwrap())?;

    // Run npm install
    let npm_spec =
        if version == "latest" { package.clone() } else { format!("{}@{}", package, version) };

    let spinner = terminal::step_spinner(&format!("Installing {}...", npm_spec));

    let output = terminal::run_command_with_spinner(
        &spinner,
        Command::new("npm").args(["install", &npm_spec]).current_dir(&ts_root),
    )
    .map_err(|e| miette::miette!("Failed to run npm install: {}", e))?;

    if !output.status.success() {
        terminal::finish_failure(&spinner, "npm install failed");
        terminal::print_stderr_excerpt(&output.stderr, 8);
        return Err(command_failure(
            &format!("npm install {}", npm_spec),
            &ts_root,
            &output,
            "Check package name/version and npm registry/network configuration.",
        ));
    }

    terminal::finish_success(&spinner, &format!("Added {}@{} to polybench.toml", package, version));

    Ok(())
}

/// Add a Rust dependency to the project
pub fn add_rust_dependency(spec: &str) -> Result<()> {
    add_rust_dependency_with_features(spec, None)
}

/// Add a Rust dependency with optional features to the project
pub fn add_rust_dependency_with_features(spec: &str, features: Option<&[String]>) -> Result<()> {
    if !crate::runtime_check::is_lang_installed(Lang::Rust) {
        return Err(crate::runtime_check::not_installed_error(Lang::Rust));
    }
    let current_dir = std::env::current_dir()
        .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

    let project_root = crate::find_project_root(&current_dir)
        .ok_or_else(|| miette::miette!("Not in a poly-bench project"))?;

    let mut manifest = crate::load_manifest(&project_root)?;

    if !manifest.has_runtime(Lang::Rust) {
        return Err(miette::miette!("Rust is not enabled in this project"));
    }

    let (crate_name, version) = parse_dep_spec(spec);

    let rust_root = resolve_runtime_root(&project_root, Lang::Rust);
    ensure_rust_env(&rust_root, manifest.rust.as_ref().unwrap(), &manifest.project.name)?;

    // Use cargo add for dependency installation - it resolves "latest" automatically
    let cargo_spec = if version == "latest" {
        crate_name.clone()
    } else {
        format!("{}@{}", crate_name, version)
    };

    let display_spec = if let Some(feats) = features {
        if !feats.is_empty() {
            format!("{} --features {}", cargo_spec, feats.join(","))
        } else {
            cargo_spec.clone()
        }
    } else {
        cargo_spec.clone()
    };

    let spinner = terminal::step_spinner(&format!("Installing {}...", display_spec));

    // Build cargo add args
    let mut args = vec!["add".to_string(), cargo_spec.clone()];
    if let Some(feats) = features {
        if !feats.is_empty() {
            args.push("--features".to_string());
            args.push(feats.join(","));
        }
    }

    let output = terminal::run_command_with_spinner(
        &spinner,
        Command::new("cargo").args(&args).current_dir(&rust_root),
    )
    .map_err(|e| miette::miette!("Failed to run cargo add: {}", e))?;

    if !output.status.success() {
        terminal::finish_failure(&spinner, "cargo add failed");
        terminal::print_stderr_excerpt(&output.stderr, 8);
        return Err(command_failure(
            &format!("cargo {}", args.join(" ")),
            &rust_root,
            &output,
            "Confirm crate name/features and Cargo registry access.",
        ));
    }

    // Read the resolved version from Cargo.toml
    let resolved_version =
        read_cargo_dep_version(&rust_root, &crate_name).unwrap_or_else(|| version.clone());

    // Add to manifest with features if specified
    if let Some(feats) = features {
        if !feats.is_empty() {
            manifest.add_rust_dependency_with_features(&crate_name, &resolved_version, feats)?;
        } else {
            manifest.add_rust_dependency(&crate_name, &resolved_version)?;
        }
    } else {
        manifest.add_rust_dependency(&crate_name, &resolved_version)?;
    }
    crate::save_manifest(&project_root, &manifest)?;

    let feature_suffix = if let Some(feats) = features {
        if !feats.is_empty() {
            format!(" with features [{}]", feats.join(", "))
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    terminal::finish_success(
        &spinner,
        &format!("Added {}@{}{} to polybench.toml", crate_name, resolved_version, feature_suffix),
    );

    Ok(())
}

/// Remove a Go dependency from the project
pub fn remove_go_dependency(package: &str) -> Result<()> {
    let current_dir = std::env::current_dir()
        .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

    let project_root = crate::find_project_root(&current_dir)
        .ok_or_else(|| miette::miette!("Not in a poly-bench project"))?;

    let mut manifest = crate::load_manifest(&project_root)?;

    if !manifest.has_runtime(Lang::Go) {
        return Err(miette::miette!("Go is not enabled in this project"));
    }

    // Check if dependency exists in manifest
    let go_config = manifest.go.as_ref().unwrap();
    if !go_config.dependencies.contains_key(package) {
        return Err(miette::miette!(
            "Dependency '{}' is not installed. Check polybench.toml for installed Go dependencies.",
            package
        ));
    }

    // Remove from manifest
    manifest.remove_go_dependency(package)?;
    crate::save_manifest(&project_root, &manifest)?;

    let go_root = resolve_runtime_root(&project_root, Lang::Go);

    // Run go mod tidy to clean up go.mod and go.sum
    let spinner = terminal::step_spinner(&format!("Removing {}...", package));

    let output = terminal::run_command_with_spinner(
        &spinner,
        Command::new("go").args(["mod", "tidy"]).current_dir(&go_root),
    )
    .map_err(|e| miette::miette!("Failed to run go mod tidy: {}", e))?;

    if !output.status.success() {
        terminal::finish_failure(&spinner, "go mod tidy failed");
        terminal::print_stderr_excerpt(&output.stderr, 8);
        return Err(command_failure(
            "go mod tidy",
            &go_root,
            &output,
            "Check go.mod consistency and module import paths.",
        ));
    }

    terminal::finish_success(&spinner, &format!("Removed {} from polybench.toml", package));

    Ok(())
}

/// Remove a TypeScript dependency from the project
pub fn remove_ts_dependency(package: &str) -> Result<()> {
    let current_dir = std::env::current_dir()
        .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

    let project_root = crate::find_project_root(&current_dir)
        .ok_or_else(|| miette::miette!("Not in a poly-bench project"))?;

    let mut manifest = crate::load_manifest(&project_root)?;

    if !manifest.has_runtime(Lang::TypeScript) {
        return Err(miette::miette!("TypeScript is not enabled in this project"));
    }

    // Check if dependency exists in manifest
    let ts_config = manifest.ts.as_ref().unwrap();
    if !ts_config.dependencies.contains_key(package) {
        return Err(miette::miette!(
            "Dependency '{}' is not installed. Check polybench.toml for installed TypeScript dependencies.",
            package
        ));
    }

    // Remove from manifest
    manifest.remove_ts_dependency(package)?;
    crate::save_manifest(&project_root, &manifest)?;

    let ts_root = resolve_runtime_root(&project_root, Lang::TypeScript);

    // Run npm uninstall
    let spinner = terminal::step_spinner(&format!("Removing {}...", package));

    let output = terminal::run_command_with_spinner(
        &spinner,
        Command::new("npm").args(["uninstall", package]).current_dir(&ts_root),
    )
    .map_err(|e| miette::miette!("Failed to run npm uninstall: {}", e))?;

    if !output.status.success() {
        terminal::finish_failure(&spinner, "npm uninstall failed");
        terminal::print_stderr_excerpt(&output.stderr, 8);
        return Err(command_failure(
            &format!("npm uninstall {}", package),
            &ts_root,
            &output,
            "Verify npm project state and lockfile integrity.",
        ));
    }

    terminal::finish_success(&spinner, &format!("Removed {} from polybench.toml", package));

    Ok(())
}

/// Remove a Rust dependency from the project
pub fn remove_rust_dependency(crate_name: &str) -> Result<()> {
    let current_dir = std::env::current_dir()
        .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

    let project_root = crate::find_project_root(&current_dir)
        .ok_or_else(|| miette::miette!("Not in a poly-bench project"))?;

    let mut manifest = crate::load_manifest(&project_root)?;

    if !manifest.has_runtime(Lang::Rust) {
        return Err(miette::miette!("Rust is not enabled in this project"));
    }

    // Check if dependency exists in manifest
    let rust_config = manifest.rust.as_ref().unwrap();
    if !rust_config.dependencies.contains_key(crate_name) {
        return Err(miette::miette!(
            "Dependency '{}' is not installed. Check polybench.toml for installed Rust dependencies.",
            crate_name
        ));
    }

    // Remove from manifest
    manifest.remove_rust_dependency(crate_name)?;
    crate::save_manifest(&project_root, &manifest)?;

    let rust_root = resolve_runtime_root(&project_root, Lang::Rust);

    // Run cargo remove
    let spinner = terminal::step_spinner(&format!("Removing {}...", crate_name));

    let output = terminal::run_command_with_spinner(
        &spinner,
        Command::new("cargo").args(["remove", crate_name]).current_dir(&rust_root),
    )
    .map_err(|e| miette::miette!("Failed to run cargo remove: {}", e))?;

    if !output.status.success() {
        terminal::finish_failure(&spinner, "cargo remove failed");
        terminal::print_stderr_excerpt(&output.stderr, 8);
        return Err(command_failure(
            &format!("cargo remove {}", crate_name),
            &rust_root,
            &output,
            "Ensure Cargo.toml is valid and dependency exists in the selected workspace.",
        ));
    }

    terminal::finish_success(&spinner, &format!("Removed {} from polybench.toml", crate_name));

    Ok(())
}

/// Parse Python dependency spec: "numpy==1.0" or "numpy" (latest) or "numpy@1.0"
fn parse_python_dep_spec(spec: &str) -> (String, String) {
    if let Some(idx) = spec.find("==") {
        (spec[..idx].to_string(), spec[idx + 2..].to_string())
    } else {
        let (pkg, ver) = parse_dep_spec(spec);
        (pkg, ver)
    }
}

/// Add a Python dependency to the project
pub fn add_python_dependency(spec: &str) -> Result<()> {
    if !crate::runtime_check::is_lang_installed(Lang::Python) {
        return Err(crate::runtime_check::not_installed_error(Lang::Python));
    }
    let current_dir = std::env::current_dir()
        .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

    let project_root = crate::find_project_root(&current_dir)
        .ok_or_else(|| miette::miette!("Not in a poly-bench project"))?;

    let mut manifest = crate::load_manifest(&project_root)?;

    if !manifest.has_runtime(Lang::Python) {
        return Err(miette::miette!("Python is not enabled in this project"));
    }

    let (package, version) = parse_python_dep_spec(spec);

    manifest.add_python_dependency(&package, &version)?;
    crate::save_manifest(&project_root, &manifest)?;

    let python_root = resolve_runtime_root(&project_root, Lang::Python);
    std::fs::create_dir_all(&python_root)
        .map_err(|e| miette::miette!("Failed to create Python env dir: {}", e))?;

    let deps: Vec<(String, String)> = manifest
        .python
        .as_ref()
        .unwrap()
        .dependencies
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    let requirements_content = templates::requirements_txt_for_runtime_env(&deps);
    std::fs::write(python_root.join("requirements.txt"), requirements_content)
        .map_err(|e| miette::miette!("Failed to write requirements.txt: {}", e))?;

    // Always install the package so it's immediately available (no need to run install separately)
    let pip_path = ensure_python_venv_and_get_pip(&python_root)?;
    let pip_spec =
        if version == "latest" { package.clone() } else { format!("{}=={}", package, version) };
    let spinner = terminal::step_spinner(&format!("Installing {}...", pip_spec));
    let output = terminal::run_command_with_spinner(
        &spinner,
        Command::new(&pip_path).args(["install", &pip_spec]).current_dir(&python_root),
    )
    .map_err(|e| miette::miette!("Failed to run pip install: {}", e))?;

    if !output.status.success() {
        terminal::finish_failure(&spinner, "pip install failed");
        terminal::print_stderr_excerpt(&output.stderr, 8);
        return Err(command_failure(
            &format!("pip install {}", pip_spec),
            &python_root,
            &output,
            "Fix pip dependency issues.",
        ));
    }
    terminal::finish_success(&spinner, &pip_spec);

    terminal::success(&format!("Added {} to polybench.toml", package));

    Ok(())
}

/// Add a C dependency to the project
pub fn add_c_dependency(spec: &str) -> Result<()> {
    let current_dir = std::env::current_dir()
        .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

    let project_root = crate::find_project_root(&current_dir)
        .ok_or_else(|| miette::miette!("Not in a poly-bench project"))?;

    let mut manifest = crate::load_manifest(&project_root)?;

    if !manifest.has_runtime(Lang::C) {
        return Err(miette::miette!("C is not enabled in this project"));
    }

    let (library, version) = parse_dep_spec(spec);
    manifest.add_c_dependency(&library, &version)?;
    crate::save_manifest(&project_root, &manifest)?;

    terminal::success(&format!("Added {}@{} to polybench.toml", library, version));
    Ok(())
}

/// Remove a C dependency from the project
pub fn remove_c_dependency(library: &str) -> Result<()> {
    let current_dir = std::env::current_dir()
        .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

    let project_root = crate::find_project_root(&current_dir)
        .ok_or_else(|| miette::miette!("Not in a poly-bench project"))?;

    let mut manifest = crate::load_manifest(&project_root)?;

    if !manifest.has_runtime(Lang::C) {
        return Err(miette::miette!("C is not enabled in this project"));
    }

    if !manifest.c.as_ref().unwrap().dependencies.contains_key(library) {
        return Err(miette::miette!(
            "Dependency '{}' is not installed. Check polybench.toml for installed C dependencies.",
            library
        ));
    }

    manifest.remove_c_dependency(library)?;
    crate::save_manifest(&project_root, &manifest)?;

    terminal::success(&format!("Removed {} from polybench.toml", library));
    Ok(())
}

/// Remove a Python dependency from the project
pub fn remove_python_dependency(package: &str) -> Result<()> {
    let current_dir = std::env::current_dir()
        .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

    let project_root = crate::find_project_root(&current_dir)
        .ok_or_else(|| miette::miette!("Not in a poly-bench project"))?;

    let mut manifest = crate::load_manifest(&project_root)?;

    if !manifest.has_runtime(Lang::Python) {
        return Err(miette::miette!("Python is not enabled in this project"));
    }

    if !manifest.python.as_ref().unwrap().dependencies.contains_key(package) {
        return Err(miette::miette!(
            "Dependency '{}' is not installed. Check polybench.toml for installed Python dependencies.",
            package
        ));
    }

    manifest.remove_python_dependency(package)?;
    crate::save_manifest(&project_root, &manifest)?;

    let python_root = resolve_runtime_root(&project_root, Lang::Python);
    let deps: Vec<(String, String)> = manifest
        .python
        .as_ref()
        .unwrap()
        .dependencies
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    let requirements_content = templates::requirements_txt_for_runtime_env(&deps);
    std::fs::write(python_root.join("requirements.txt"), requirements_content)
        .map_err(|e| miette::miette!("Failed to write requirements.txt: {}", e))?;

    terminal::success(&format!("Removed {} from polybench.toml", package));

    Ok(())
}

/// Add a C# dependency to the project
pub fn add_csharp_dependency(spec: &str) -> Result<()> {
    if !crate::runtime_check::is_lang_installed(Lang::CSharp) {
        return Err(crate::runtime_check::not_installed_error(Lang::CSharp));
    }
    let current_dir = std::env::current_dir()
        .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

    let project_root = crate::find_project_root(&current_dir)
        .ok_or_else(|| miette::miette!("Not in a poly-bench project"))?;

    let mut manifest = crate::load_manifest(&project_root)?;

    if !manifest.has_runtime(Lang::CSharp) {
        return Err(miette::miette!("C# is not enabled in this project"));
    }

    let (package, version) = parse_dep_spec(spec);
    manifest.add_csharp_dependency(&package, &version)?;
    crate::save_manifest(&project_root, &manifest)?;

    let csharp_root = resolve_runtime_root(&project_root, Lang::CSharp);
    std::fs::create_dir_all(&csharp_root)
        .map_err(|e| miette::miette!("Failed to create C# env dir: {}", e))?;

    let csproj_path = csharp_root.join("polybench.csproj");
    if !csproj_path.exists() {
        let tfm = manifest.csharp.as_ref().map(|c| c.target_framework.as_str()).unwrap_or("net8.0");
        std::fs::write(&csproj_path, templates::csharp_csproj(tfm))
            .map_err(|e| miette::miette!("Failed to write polybench.csproj: {}", e))?;
    }

    let spinner = terminal::step_spinner(&format!("Installing {}...", spec));
    let output = terminal::run_command_with_spinner(
        &spinner,
        Command::new("dotnet")
            .args(["add", "polybench.csproj", "package", &package, "--version", &version])
            .current_dir(&csharp_root),
    )
    .map_err(|e| miette::miette!("Failed to run dotnet add package: {}", e))?;

    if !output.status.success() {
        terminal::finish_failure(&spinner, "dotnet add package failed");
        terminal::print_stderr_excerpt(&output.stderr, 8);
        return Err(command_failure(
            &format!("dotnet add package {} --version {}", package, version),
            &csharp_root,
            &output,
            "Verify package/version exists on NuGet.",
        ));
    }

    terminal::finish_success(&spinner, &format!("Added {}@{} to polybench.toml", package, version));
    Ok(())
}

/// Remove a C# dependency from the project
pub fn remove_csharp_dependency(package: &str) -> Result<()> {
    let current_dir = std::env::current_dir()
        .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

    let project_root = crate::find_project_root(&current_dir)
        .ok_or_else(|| miette::miette!("Not in a poly-bench project"))?;

    let mut manifest = crate::load_manifest(&project_root)?;

    if !manifest.has_runtime(Lang::CSharp) {
        return Err(miette::miette!("C# is not enabled in this project"));
    }

    if !manifest.csharp.as_ref().unwrap().dependencies.contains_key(package) {
        return Err(miette::miette!(
            "Dependency '{}' is not installed. Check polybench.toml for installed C# dependencies.",
            package
        ));
    }

    manifest.remove_csharp_dependency(package)?;
    crate::save_manifest(&project_root, &manifest)?;

    let csharp_root = resolve_runtime_root(&project_root, Lang::CSharp);
    let spinner = terminal::step_spinner(&format!("Removing {}...", package));
    let output = terminal::run_command_with_spinner(
        &spinner,
        Command::new("dotnet")
            .args(["remove", "polybench.csproj", "package", package])
            .current_dir(&csharp_root),
    )
    .map_err(|e| miette::miette!("Failed to run dotnet remove package: {}", e))?;

    if !output.status.success() {
        terminal::finish_failure(&spinner, "dotnet remove package failed");
        terminal::print_stderr_excerpt(&output.stderr, 8);
        return Err(command_failure(
            &format!("dotnet remove package {}", package),
            &csharp_root,
            &output,
            "Ensure package exists in project and .csproj is valid.",
        ));
    }

    terminal::finish_success(&spinner, &format!("Removed {} from polybench.toml", package));
    Ok(())
}

/// Check if a Zig dependency spec looks like a URL (zig fetch requires URLs)
fn is_zig_url(spec: &str) -> bool {
    spec.contains("://") || spec.starts_with("git+")
}

/// Extract a Zig-safe dependency name from a URL for use in build.zig.zon
fn zig_dep_name_from_url(url: &str) -> String {
    // Strip fragment (#...) and query (?...)
    let path_part = url.split(['#', '?']).next().unwrap_or(url);
    // Get last path component (e.g. "zig-bench" from ".../Hejsil/zig-bench")
    let last = path_part.trim_end_matches('/').rsplit('/').next().unwrap_or("dep");
    // Zig identifiers: alphanumeric + underscore, replace - with _
    let sanitized: String = last
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else if c == '-' {
                '_'
            } else {
                '_'
            }
        })
        .take(32)
        .collect();
    if sanitized.is_empty() {
        "dep".to_string()
    } else {
        sanitized
    }
}

/// Build the full Zig package URL from package and version (for git URLs with ref)
fn zig_package_url(package: &str, version: &str) -> String {
    if version.is_empty() || version == "latest" {
        return package.to_string();
    }
    // For git URLs, append #version as ref (tag, branch, or commit)
    if package.contains("github.com") ||
        package.contains("gitlab.com") ||
        package.starts_with("git+")
    {
        if package.contains('#') {
            package.to_string()
        } else {
            format!("{}#{}", package, version)
        }
    } else {
        package.to_string()
    }
}

/// Add a Zig dependency to the project
pub fn add_zig_dependency(spec: &str) -> Result<()> {
    if !crate::runtime_check::is_lang_installed(Lang::Zig) {
        return Err(crate::runtime_check::not_installed_error(Lang::Zig));
    }
    let current_dir = std::env::current_dir()
        .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

    let project_root = crate::find_project_root(&current_dir)
        .ok_or_else(|| miette::miette!("Not in a poly-bench project"))?;

    let mut manifest = crate::load_manifest(&project_root)?;

    if !manifest.has_runtime(Lang::Zig) {
        return Err(miette::miette!("Zig is not enabled in this project"));
    }

    let (package, version) = parse_dep_spec(spec);

    if !is_zig_url(&package) {
        return Err(miette::miette!(
            "Zig dependencies must be specified as URLs. Example: poly-bench add --zig \"git+https://github.com/Hejsil/zig-bench#main\"\n\
             Or: poly-bench add --zig \"https://github.com/foo/bar/archive/refs/tags/v0.1.0.tar.gz\""
        ));
    }

    manifest.add_zig_dependency(&package, &version)?;
    crate::save_manifest(&project_root, &manifest)?;

    let zig_root = resolve_runtime_root(&project_root, Lang::Zig);
    std::fs::create_dir_all(&zig_root)
        .map_err(|e| miette::miette!("Failed to create Zig env dir: {}", e))?;

    ensure_zig_build_files(&zig_root)?;

    let zig_url = zig_package_url(&package, &version);
    let dep_name = zig_dep_name_from_url(&zig_url);

    let spinner = terminal::step_spinner(&format!("Fetching {}...", dep_name));
    let output = terminal::run_command_with_spinner(
        &spinner,
        Command::new("zig")
            .args(["fetch", &format!("--save={}", dep_name), &zig_url])
            .current_dir(&zig_root),
    )
    .map_err(|e| miette::miette!("Failed to run zig fetch: {}", e))?;

    if !output.status.success() {
        terminal::finish_failure(&spinner, "zig fetch failed");
        terminal::print_stderr_excerpt(&output.stderr, 8);
        return Err(command_failure(
            &format!("zig fetch --save={} {}", dep_name, zig_url),
            &zig_root,
            &output,
            "Verify the URL is valid and the package is accessible.",
        ));
    }
    terminal::finish_success(&spinner, &format!("Added {} to polybench.toml", package));

    Ok(())
}

/// Ensure build.zig and build.zig.zon exist in zig_root (required for zig fetch)
fn ensure_zig_build_files(zig_root: &Path) -> Result<()> {
    let build_zig = zig_root.join("build.zig");
    if !build_zig.exists() {
        std::fs::write(&build_zig, templates::build_zig())
            .map_err(|e| miette::miette!("Failed to write build.zig: {}", e))?;
    }
    let build_zig_zon = zig_root.join("build.zig.zon");
    if !build_zig_zon.exists() {
        std::fs::write(&build_zig_zon, templates::build_zig_zon())
            .map_err(|e| miette::miette!("Failed to write build.zig.zon: {}", e))?;
    }
    Ok(())
}

/// Remove a Zig dependency from the project
pub fn remove_zig_dependency(package: &str) -> Result<()> {
    let current_dir = std::env::current_dir()
        .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

    let project_root = crate::find_project_root(&current_dir)
        .ok_or_else(|| miette::miette!("Not in a poly-bench project"))?;

    let mut manifest = crate::load_manifest(&project_root)?;

    if !manifest.has_runtime(Lang::Zig) {
        return Err(miette::miette!("Zig is not enabled in this project"));
    }

    if !manifest.zig.as_ref().unwrap().dependencies.contains_key(package) {
        return Err(miette::miette!(
            "Dependency '{}' is not installed. Check polybench.toml for installed Zig dependencies.",
            package
        ));
    }

    manifest.remove_zig_dependency(package)?;
    crate::save_manifest(&project_root, &manifest)?;

    terminal::success(&format!("Removed {} from polybench.toml", package));
    Ok(())
}

/// Read a dependency version from Cargo.toml
fn read_cargo_dep_version(rust_root: &Path, crate_name: &str) -> Option<String> {
    let cargo_toml_path = rust_root.join("Cargo.toml");
    let content = std::fs::read_to_string(&cargo_toml_path).ok()?;
    let doc: toml_edit::DocumentMut = content.parse().ok()?;

    let deps = doc.get("dependencies")?.as_table()?;
    let dep = deps.get(crate_name)?;

    // Handle both simple string and inline table formats
    if let Some(version) = dep.as_str() {
        return Some(version.to_string());
    }

    if let Some(table) = dep.as_inline_table() {
        if let Some(version) = table.get("version").and_then(|v| v.as_str()) {
            return Some(version.to_string());
        }
    }

    if let Some(table) = dep.as_table() {
        if let Some(version) = table.get("version").and_then(|v| v.as_str()) {
            return Some(version.to_string());
        }
    }

    None
}

/// Ensure Cargo.toml exists in rust_root
fn ensure_rust_env(
    rust_root: &Path,
    rust_config: &manifest::RustConfig,
    project_name: &str,
) -> Result<()> {
    std::fs::create_dir_all(rust_root)
        .map_err(|e| miette::miette!("Failed to create Rust env dir: {}", e))?;

    let cargo_toml_path = rust_root.join("Cargo.toml");
    if !cargo_toml_path.exists() {
        let content = templates::cargo_toml(project_name, &rust_config.edition);
        std::fs::write(&cargo_toml_path, content)
            .map_err(|e| miette::miette!("Failed to write Cargo.toml: {}", e))?;

        // Also create src/main.rs placeholder
        let src_dir = rust_root.join("src");
        std::fs::create_dir_all(&src_dir)
            .map_err(|e| miette::miette!("Failed to create src dir: {}", e))?;
        std::fs::write(src_dir.join("main.rs"), "fn main() {}\n")
            .map_err(|e| miette::miette!("Failed to write main.rs: {}", e))?;
    }
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

    for lang in poly_bench_runtime::supported_languages() {
        if manifest.has_runtime(*lang) {
            spinner.set_message(format!(
                "Installing {} dependencies...",
                poly_bench_runtime::lang_label(*lang)
            ));
            install_runtime_deps_for_lang(*lang, &project_root, &manifest)?;
        }
    }

    terminal::finish_success(&spinner, "All dependencies installed!");

    Ok(())
}

fn install_runtime_deps_for_lang(
    lang: Lang,
    project_root: &Path,
    manifest: &manifest::Manifest,
) -> Result<()> {
    match lang {
        Lang::Go => install_go_deps(project_root, manifest.go.as_ref().unwrap()),
        Lang::TypeScript => {
            install_ts_deps(project_root, manifest.ts.as_ref().unwrap(), &manifest.project.name)
        }
        Lang::Rust => {
            install_rust_deps(project_root, manifest.rust.as_ref().unwrap(), &manifest.project.name)
        }
        Lang::Python => install_python_deps(project_root, manifest.python.as_ref().unwrap()),
        Lang::C => install_c_deps(project_root, manifest.c.as_ref().unwrap()),
        Lang::CSharp => install_csharp_deps(project_root, manifest.csharp.as_ref().unwrap()),
        Lang::Zig => install_zig_deps(project_root, manifest.zig.as_ref().unwrap()),
    }
}

/// Install Go dependencies
fn install_go_deps(project_root: &Path, go_config: &manifest::GoConfig) -> Result<()> {
    if !crate::runtime_check::is_lang_installed(Lang::Go) {
        return Err(crate::runtime_check::not_installed_error(Lang::Go));
    }
    terminal::section("Go dependencies");

    let go_root = resolve_runtime_root(project_root, Lang::Go);
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
            Command::new("go").args(["get", &go_get_arg]).current_dir(&go_root),
        )
        .map_err(|e| miette::miette!("Failed to run go get: {}", e))?;

        if !output.status.success() {
            terminal::finish_failure_indented(&spinner, &format!("Failed to install {}", package));
            terminal::print_stderr_excerpt(&output.stderr, 6);
            return Err(command_failure(
                &format!("go get {}", go_get_arg),
                &go_root,
                &output,
                "Validate Go dependency declarations and module connectivity.",
            ));
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
    if !crate::runtime_check::is_lang_installed(Lang::TypeScript) {
        return Err(crate::runtime_check::not_installed_error(Lang::TypeScript));
    }
    terminal::section("TypeScript dependencies");

    let ts_root = resolve_runtime_root(project_root, Lang::TypeScript);
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
        terminal::finish_failure_indented(&spinner, "npm install failed");
        terminal::print_stderr_excerpt(&output.stderr, 6);
        return Err(command_failure(
            "npm install",
            &ts_root,
            &output,
            "Resolve npm install errors before running TypeScript benchmarks.",
        ));
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

/// Install Rust dependencies
fn install_rust_deps(
    project_root: &Path,
    rust_config: &manifest::RustConfig,
    project_name: &str,
) -> Result<()> {
    if !crate::runtime_check::is_lang_installed(Lang::Rust) {
        return Err(crate::runtime_check::not_installed_error(Lang::Rust));
    }
    terminal::section("Rust dependencies");

    let rust_root = resolve_runtime_root(project_root, Lang::Rust);
    std::fs::create_dir_all(&rust_root)
        .map_err(|e| miette::miette!("Failed to create Rust env dir: {}", e))?;

    let cargo_toml_path = rust_root.join("Cargo.toml");
    if !cargo_toml_path.exists() {
        let cargo_toml_content = templates::cargo_toml(project_name, &rust_config.edition);
        std::fs::write(&cargo_toml_path, cargo_toml_content)
            .map_err(|e| miette::miette!("Failed to write Cargo.toml: {}", e))?;

        // Also create src/main.rs placeholder
        let src_dir = rust_root.join("src");
        std::fs::create_dir_all(&src_dir)
            .map_err(|e| miette::miette!("Failed to create src dir: {}", e))?;
        std::fs::write(src_dir.join("main.rs"), "fn main() {}\n")
            .map_err(|e| miette::miette!("Failed to write main.rs: {}", e))?;

        terminal::success_indented("Created Cargo.toml");
    }

    // Update Cargo.toml with dependencies from manifest
    if !rust_config.dependencies.is_empty() {
        update_cargo_toml_deps(&rust_root, rust_config)?;
    }

    // Run cargo check to download dependencies
    let spinner = terminal::indented_spinner("Downloading dependencies...");
    let output = terminal::run_command_with_spinner(
        &spinner,
        Command::new("cargo").args(["fetch"]).current_dir(&rust_root),
    )
    .map_err(|e| miette::miette!("Failed to run cargo fetch: {}", e))?;

    if output.status.success() {
        terminal::finish_success_indented(&spinner, "Rust dependencies ready");
    } else {
        terminal::finish_failure_indented(&spinner, "cargo fetch failed");
        terminal::print_stderr_excerpt(&output.stderr, 6);
        return Err(command_failure(
            "cargo fetch",
            &rust_root,
            &output,
            "Resolve Cargo registry/dependency issues before running Rust benchmarks.",
        ));
    }

    Ok(())
}

/// Install Python dependencies from manifest
fn install_python_deps(project_root: &Path, python_config: &manifest::PythonConfig) -> Result<()> {
    if !crate::runtime_check::is_lang_installed(Lang::Python) {
        return Err(crate::runtime_check::not_installed_error(Lang::Python));
    }
    terminal::section("Python dependencies");

    let python_root = runtime_env(project_root, Lang::Python);
    std::fs::create_dir_all(&python_root)
        .map_err(|e| miette::miette!("Failed to create Python env dir: {}", e))?;

    let deps: Vec<(String, String)> =
        python_config.dependencies.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    let requirements_content = templates::requirements_txt_for_runtime_env(&deps);
    std::fs::write(python_root.join("requirements.txt"), requirements_content)
        .map_err(|e| miette::miette!("Failed to write requirements.txt: {}", e))?;

    if python_config.dependencies.is_empty() {
        terminal::success_indented("No Python dependencies to install");
        return Ok(());
    }

    let pip_path = ensure_python_venv_and_get_pip(&python_root)?;
    let spinner = terminal::indented_spinner("Installing Python dependencies...");
    let output = terminal::run_command_with_spinner(
        &spinner,
        Command::new(&pip_path)
            .args(["install", "-r", "requirements.txt"])
            .current_dir(&python_root),
    )
    .map_err(|e| miette::miette!("Failed to run pip install: {}", e))?;

    if output.status.success() {
        terminal::finish_success_indented(&spinner, "Python dependencies ready");
    } else {
        terminal::finish_failure_indented(&spinner, "pip install failed");
        terminal::print_stderr_excerpt(&output.stderr, 8);
        return Err(command_failure(
            "pip install -r requirements.txt",
            &python_root,
            &output,
            "Ensure pip is available and fix dependency issues.",
        ));
    }

    Ok(())
}

/// Install C dependencies from manifest
fn install_c_deps(project_root: &Path, c_config: &manifest::CConfig) -> Result<()> {
    if !crate::runtime_check::is_lang_installed(Lang::C) {
        return Err(crate::runtime_check::not_installed_error(Lang::C));
    }
    terminal::section("C dependencies");

    let c_root = runtime_env(project_root, Lang::C);
    std::fs::create_dir_all(&c_root)
        .map_err(|e| miette::miette!("Failed to create C env dir: {}", e))?;

    if c_config.dependencies.is_empty() {
        terminal::success_indented("No C dependencies to install");
        return Ok(());
    }

    terminal::success_indented(&format!(
        "C dependencies recorded in manifest: {}",
        c_config.dependencies.len()
    ));
    Ok(())
}

/// Install C# dependencies from manifest
fn install_csharp_deps(project_root: &Path, csharp_config: &manifest::CSharpConfig) -> Result<()> {
    if !crate::runtime_check::is_lang_installed(Lang::CSharp) {
        return Err(crate::runtime_check::not_installed_error(Lang::CSharp));
    }
    terminal::section("C# dependencies");

    let csharp_root = runtime_env(project_root, Lang::CSharp);
    std::fs::create_dir_all(&csharp_root)
        .map_err(|e| miette::miette!("Failed to create C# env dir: {}", e))?;

    let csproj_path = csharp_root.join("polybench.csproj");
    if !csproj_path.exists() {
        std::fs::write(&csproj_path, templates::csharp_csproj(&csharp_config.target_framework))
            .map_err(|e| miette::miette!("Failed to write polybench.csproj: {}", e))?;
        terminal::success_indented("Created polybench.csproj");
    }

    for (package, version) in &csharp_config.dependencies {
        let spinner = terminal::indented_spinner(&format!("Adding {}...", package));
        let output = terminal::run_command_with_spinner(
            &spinner,
            Command::new("dotnet")
                .args(["add", "polybench.csproj", "package", package, "--version", version])
                .current_dir(&csharp_root),
        )
        .map_err(|e| miette::miette!("Failed to run dotnet add package: {}", e))?;
        if !output.status.success() {
            terminal::finish_failure_indented(&spinner, &format!("Failed to add {}", package));
            terminal::print_stderr_excerpt(&output.stderr, 6);
            return Err(command_failure(
                &format!("dotnet add package {} --version {}", package, version),
                &csharp_root,
                &output,
                "Fix package/version or NuGet connectivity issues.",
            ));
        }
        terminal::finish_success_indented(&spinner, package);
    }

    let spinner = terminal::indented_spinner("Running dotnet restore...");
    let output = terminal::run_command_with_spinner(
        &spinner,
        Command::new("dotnet").args(["restore", "polybench.csproj"]).current_dir(&csharp_root),
    )
    .map_err(|e| miette::miette!("Failed to run dotnet restore: {}", e))?;
    if output.status.success() {
        terminal::finish_success_indented(&spinner, "C# dependencies ready");
    } else {
        terminal::finish_failure_indented(&spinner, "dotnet restore failed");
        terminal::print_stderr_excerpt(&output.stderr, 6);
        return Err(command_failure(
            "dotnet restore polybench.csproj",
            &csharp_root,
            &output,
            "Resolve NuGet restore errors before running C# benchmarks.",
        ));
    }
    Ok(())
}

/// Install Zig dependencies from manifest
fn install_zig_deps(project_root: &Path, zig_config: &manifest::ZigConfig) -> Result<()> {
    if !crate::runtime_check::is_lang_installed(Lang::Zig) {
        return Err(crate::runtime_check::not_installed_error(Lang::Zig));
    }
    terminal::section("Zig dependencies");

    let zig_root = runtime_env(project_root, Lang::Zig);
    std::fs::create_dir_all(&zig_root)
        .map_err(|e| miette::miette!("Failed to create Zig env dir: {}", e))?;

    if zig_config.dependencies.is_empty() {
        terminal::success_indented("No Zig dependencies to install");
        return Ok(());
    }

    ensure_zig_build_files(&zig_root)?;

    for (package, version) in &zig_config.dependencies {
        if !is_zig_url(package) {
            terminal::info_indented(&format!(
                "Skipping {} (Zig deps must be URLs; run 'poly-bench add --zig \"<url>\"')",
                package
            ));
            continue;
        }
        let zig_url = zig_package_url(package, version);
        let dep_name = zig_dep_name_from_url(&zig_url);
        let spinner = terminal::indented_spinner(&format!("Fetching {}...", dep_name));
        let output = terminal::run_command_with_spinner(
            &spinner,
            Command::new("zig")
                .args(["fetch", &format!("--save={}", dep_name), &zig_url])
                .current_dir(&zig_root),
        )
        .map_err(|e| miette::miette!("Failed to run zig fetch: {}", e))?;

        if output.status.success() {
            terminal::finish_success_indented(&spinner, &dep_name);
        } else {
            terminal::finish_failure_indented(&spinner, &format!("Failed to fetch {}", dep_name));
            terminal::print_stderr_excerpt(&output.stderr, 6);
            return Err(command_failure(
                &format!("zig fetch --save={} {}", dep_name, zig_url),
                &zig_root,
                &output,
                "Verify the URL is valid and the package is accessible.",
            ));
        }
    }

    terminal::success_indented("Zig dependencies ready");
    Ok(())
}

/// Update Cargo.toml with dependencies from the manifest using cargo add
fn update_cargo_toml_deps(rust_root: &Path, rust_config: &manifest::RustConfig) -> Result<()> {
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
