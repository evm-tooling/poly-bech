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

    let c_config = manifest.c.as_ref().unwrap();
    let c_root = runtime_env(&project_root, Lang::C);
    std::fs::create_dir_all(&c_root)
        .map_err(|e| miette::miette!("Failed to create C env dir: {}", e))?;

    let deps: Vec<(String, String)> =
        c_config.dependencies.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    std::fs::write(
        &c_root.join("vcpkg.json"),
        templates::c_vcpkg_json(&manifest.project.name, &deps),
    )
    .map_err(|e| miette::miette!("Failed to write vcpkg.json: {}", e))?;
    std::fs::write(
        &c_root.join("CMakeLists.txt"),
        templates::c_cmake_lists(&c_config.standard, &deps),
    )
    .map_err(|e| miette::miette!("Failed to write CMakeLists.txt: {}", e))?;

    c_run_cmake_configure(&c_root)?;

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

    let c_config = manifest.c.as_ref().unwrap();
    let c_root = runtime_env(&project_root, Lang::C);
    if c_root.exists() {
        let deps: Vec<(String, String)> =
            c_config.dependencies.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        if deps.is_empty() {
            let _ = std::fs::remove_file(c_root.join("vcpkg.json"));
            let _ = std::fs::remove_file(c_root.join("CMakeLists.txt"));
        } else {
            std::fs::write(
                &c_root.join("vcpkg.json"),
                templates::c_vcpkg_json(&manifest.project.name, &deps),
            )
            .map_err(|e| miette::miette!("Failed to write vcpkg.json: {}", e))?;
            std::fs::write(
                &c_root.join("CMakeLists.txt"),
                templates::c_cmake_lists(&c_config.standard, &deps),
            )
            .map_err(|e| miette::miette!("Failed to write CMakeLists.txt: {}", e))?;
            c_run_cmake_configure(&c_root)?;
        }
    }

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

/// Our Zig version for compatibility checks (0.15.2)
const POLYBENCH_ZIG_VERSION: (u32, u32, u32) = (0, 15, 2);

/// Parsed Zig git dependency URL
#[derive(Debug, Clone, PartialEq)]
struct ZigGitUrl {
    /// Full URL for zig fetch (e.g., "git+https://github.com/owner/repo.git#abc123")
    fetch_url: String,
    /// Commit hash or "HEAD" if not specified
    commit_ref: String,
    /// Derived dependency name for --save
    dep_name: String,
}

/// Parse and validate a Zig git URL in format: git+https://github.com/owner/repo.git/#COMMITHASH
///
/// Accepted formats:
/// - `git+https://github.com/owner/repo.git/#abc123def` (with commit hash)
/// - `git+https://github.com/owner/repo/#HEAD` (explicit HEAD)
/// - `git+https://github.com/owner/repo/` (defaults to HEAD)
/// - `git+https://github.com/owner/repo.git` (defaults to HEAD)
///
/// Returns error for URLs not starting with `git+https://`
fn parse_zig_git_url(spec: &str) -> Result<ZigGitUrl> {
    // Must start with "git+"
    let Some(url_part) = spec.strip_prefix("git+") else {
        return Err(miette::miette!(
            "Zig dependencies must start with 'git+'. \n\n\
             Expected format: git+https://github.com/owner/repo/#COMMITHASH\n\n\
             Examples:\n  \
               poly-bench add --zig \"git+https://github.com/discord-zig/discord.zig/#abc123def\"\n  \
               poly-bench add --zig \"git+https://github.com/discord-zig/discord.zig/#HEAD\"\n  \
               poly-bench add --zig \"git+https://github.com/discord-zig/discord.zig/\"  (defaults to HEAD)"
        ));
    };

    // Must use HTTPS
    if !url_part.starts_with("https://") {
        return Err(miette::miette!(
            "Zig git URLs must use HTTPS.\n\n\
             Expected format: git+https://github.com/owner/repo/#COMMITHASH"
        ));
    }

    // Split URL and fragment (commit ref)
    let (base_url, commit_ref) = match url_part.split_once('#') {
        Some((base, ref_part)) => {
            let ref_str = ref_part.trim();
            if ref_str.is_empty() || ref_str.eq_ignore_ascii_case("head") {
                (base, "HEAD".to_string())
            } else {
                (base, ref_str.to_string())
            }
        }
        None => (url_part, "HEAD".to_string()),
    };

    // Validate it's a supported git host (GitHub or GitLab)
    let base_url_trimmed = base_url.trim_end_matches('/');
    let is_github = base_url_trimmed.starts_with("https://github.com/");
    let is_gitlab = base_url_trimmed.starts_with("https://gitlab.com/");

    if !is_github && !is_gitlab {
        return Err(miette::miette!(
            "Only GitHub and GitLab URLs are supported.\n\n\
             Expected format: git+https://github.com/owner/repo/#COMMITHASH\n\
             Or: git+https://gitlab.com/owner/repo/#COMMITHASH"
        ));
    }

    // Validate URL has owner/repo structure (at least 5 segments: https / / host / owner / repo)
    let segments: Vec<&str> = base_url_trimmed.split('/').collect();
    if segments.len() < 5 {
        return Err(miette::miette!(
            "Invalid git URL structure. Expected owner/repo path.\n\n\
             Example: git+https://github.com/owner/repo/#COMMITHASH"
        ));
    }

    // Ensure URL ends with .git for consistency
    let normalized_base = if base_url_trimmed.ends_with(".git") {
        base_url_trimmed.to_string()
    } else {
        format!("{}.git", base_url_trimmed)
    };

    // Build the fetch URL
    let fetch_url = format!("git+{}#{}", normalized_base, commit_ref);

    // Derive dependency name from repo name
    let dep_name = zig_dep_name_from_url(&fetch_url);

    Ok(ZigGitUrl { fetch_url, commit_ref, dep_name })
}

/// Check if a Zig dependency spec looks like a URL (zig fetch requires URLs)
fn is_zig_url(spec: &str) -> bool {
    spec.contains("://") || spec.starts_with("git+")
}

/// True for raw GitHub/GitLab repository URLs (not tarballs/releases/archive links).
fn is_raw_git_repo_http_url(url: &str) -> bool {
    let no_query = url.split(['#', '?']).next().unwrap_or(url).trim_end_matches('/');
    let is_hosted_git = no_query.starts_with("https://github.com/") ||
        no_query.starts_with("http://github.com/") ||
        no_query.starts_with("https://gitlab.com/") ||
        no_query.starts_with("http://gitlab.com/");
    if !is_hosted_git {
        return false;
    }
    if no_query.ends_with(".git") ||
        no_query.ends_with(".tar.gz") ||
        no_query.ends_with(".tgz") ||
        no_query.ends_with(".zip") ||
        no_query.contains("/archive/") ||
        no_query.contains("/releases/download/")
    {
        return false;
    }
    let segs: Vec<&str> = no_query.split('/').collect();
    // host + org + repo minimum (e.g. https://github.com/org/repo)
    segs.len() >= 5
}

/// Normalize Zig package source to a fetchable URL for `zig fetch`.
/// Converts raw GitHub/GitLab repo URLs to `git+https://...` automatically.
fn normalize_zig_package_source(package: &str) -> String {
    fn add_git_suffix_for_hosted_repo(url: &str) -> String {
        let (no_fragment, fragment) = match url.split_once('#') {
            Some((base, frag)) => (base, Some(frag)),
            None => (url, None),
        };
        let (no_query, query) = match no_fragment.split_once('?') {
            Some((base, q)) => (base, Some(q)),
            None => (no_fragment, None),
        };
        let mut base = no_query.trim_end_matches('/').to_string();
        if (base.starts_with("https://github.com/") || base.starts_with("https://gitlab.com/")) &&
            !base.ends_with(".git")
        {
            base.push_str(".git");
        }
        if let Some(q) = query {
            base.push('?');
            base.push_str(q);
        }
        if let Some(frag) = fragment {
            base.push('#');
            base.push_str(frag);
        }
        base
    }

    if let Some(rest) = package.strip_prefix("git+") {
        if is_raw_git_repo_http_url(rest) {
            format!("git+{}", add_git_suffix_for_hosted_repo(rest))
        } else {
            package.to_string()
        }
    } else if is_raw_git_repo_http_url(package) {
        format!("git+{}", add_git_suffix_for_hosted_repo(package))
    } else {
        package.to_string()
    }
}

/// Extract a Zig-safe dependency name from a URL for use in build.zig.zon
fn zig_dep_name_from_url(url: &str) -> String {
    // Strip fragment (#...) and query (?...)
    let path_part = url.split(['#', '?']).next().unwrap_or(url);
    // Get last path component (e.g. "zig-bench" from ".../Hejsil/zig-bench")
    let last = path_part.trim_end_matches('/').rsplit('/').next().unwrap_or("dep");
    // Hosted git URLs often end with ".git"; strip it so module name matches repo name.
    let last = last.strip_suffix(".git").unwrap_or(last);
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
    fn normalize_git_ref(version: &str) -> String {
        if version.starts_with('v') {
            return version.to_string();
        }
        let looks_semver = version
            .chars()
            .all(|c| c.is_ascii_digit() || c == '.' || c == '-' || c.is_ascii_alphabetic());
        if looks_semver && version.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            format!("v{}", version)
        } else {
            version.to_string()
        }
    }

    let normalized = normalize_zig_package_source(package);
    if version.is_empty() || version == "latest" {
        return normalized;
    }
    // For git URLs, append #version as ref (tag, branch, or commit)
    if normalized.contains("github.com") ||
        normalized.contains("gitlab.com") ||
        normalized.starts_with("git+")
    {
        if normalized.contains('#') {
            normalized
        } else {
            format!("{}#{}", normalized, normalize_git_ref(version))
        }
    } else {
        normalized
    }
}

/// Parse minimum_zig_version from a build.zig.zon file content.
/// Returns None if the field is not present.
fn parse_zon_minimum_version(zon_content: &str) -> Option<crate::toolchain::Version> {
    // Look for: .minimum_zig_version = "X.Y.Z",
    let re = regex::Regex::new(r#"\.minimum_zig_version\s*=\s*"([^"]+)""#).ok()?;
    let caps = re.captures(zon_content)?;
    let version_str = caps.get(1)?.as_str();

    // Parse semver-like version
    let clean = version_str.split(['-', '+', ' ']).next()?;
    let parts: Vec<&str> = clean.split('.').collect();
    let major: u32 = parts.first()?.parse().ok()?;
    let minor: u32 = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch: u32 = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);

    Some(crate::toolchain::Version::new(major, minor, patch))
}

/// Get the Zig cache directory path (typically ~/.cache/zig/p/)
fn zig_cache_path() -> std::path::PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        std::path::PathBuf::from(home).join(".cache").join("zig").join("p")
    } else if let Ok(home) = std::env::var("USERPROFILE") {
        std::path::PathBuf::from(home).join("AppData").join("Local").join("zig").join("p")
    } else {
        std::path::PathBuf::from("/tmp/.cache/zig/p")
    }
}

/// Clear any existing cached entries for a dependency name from the Zig cache.
/// This prevents stale/corrupted cache entries from blocking fresh fetches.
/// The cache uses format: `<dep_name>-<version>-<hash_prefix>-<hash_suffix>`
fn clear_zig_cache_for_dep(dep_name: &str) {
    let cache_path = zig_cache_path();
    if !cache_path.exists() {
        return;
    }

    let Ok(entries) = std::fs::read_dir(&cache_path) else {
        return;
    };

    let prefix = format!("{}-", dep_name);
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if name_str.starts_with(&prefix) {
            // This is a cached entry for this dependency - remove it
            if let Err(e) = std::fs::remove_dir_all(entry.path()) {
                eprintln!(
                    "Warning: Failed to clear stale cache entry {}: {}",
                    entry.path().display(),
                    e
                );
            }
        }
    }
}

/// Extract the package hash from the project's build.zig.zon after zig fetch.
/// Looks for: .dep_name = .{ ... .hash = "...", }
fn extract_dep_hash_from_zon(zig_root: &Path, dep_name: &str) -> Option<String> {
    let zon_path = zig_root.join("build.zig.zon");
    let content = std::fs::read_to_string(&zon_path).ok()?;

    // Look for the dependency block and extract its hash
    // Pattern: .dep_name = .{ ... .hash = "HASH", ... }
    let dep_pattern = format!(r#"\.{}\s*=\s*\.\{{"#, regex::escape(dep_name));
    let dep_re = regex::Regex::new(&dep_pattern).ok()?;

    // Find where the dependency block starts
    let dep_match = dep_re.find(&content)?;
    let after_dep = &content[dep_match.start()..];

    // Find the .hash = "..." within this block
    let hash_re = regex::Regex::new(r#"\.hash\s*=\s*"([^"]+)""#).ok()?;
    if let Some(caps) = hash_re.captures(after_dep) {
        return caps.get(1).map(|m| m.as_str().to_string());
    }

    None
}

/// Result of validating a Zig dependency's compatibility
#[derive(Debug)]
enum ZigDepValidation {
    /// Package is compatible
    Compatible,
    /// Package has no minimum version specified (compatible)
    NoMinVersionSpecified,
    /// Package is missing build.zig.zon
    MissingBuildZon,
    /// Package requires a newer Zig version
    IncompatibleVersion {
        required: crate::toolchain::Version,
        polybench: crate::toolchain::Version,
    },
}

/// Validate a fetched Zig dependency's version compatibility.
/// Checks the package's build.zig.zon for minimum_zig_version.
fn validate_zig_dep_compatibility(dep_hash: &str, _dep_name: &str) -> ZigDepValidation {
    let cache_path = zig_cache_path();
    let package_path = cache_path.join(dep_hash);

    // Check if package directory exists
    if !package_path.exists() {
        // Try without the full hash (some versions use shorter paths)
        return ZigDepValidation::MissingBuildZon;
    }

    // Check for build.zig.zon in the package
    let zon_path = package_path.join("build.zig.zon");
    if !zon_path.exists() {
        return ZigDepValidation::MissingBuildZon;
    }

    // Read and parse build.zig.zon
    let Ok(zon_content) = std::fs::read_to_string(&zon_path) else {
        return ZigDepValidation::MissingBuildZon;
    };

    // Check for minimum_zig_version
    match parse_zon_minimum_version(&zon_content) {
        None => ZigDepValidation::NoMinVersionSpecified,
        Some(required) => {
            let polybench = crate::toolchain::Version::new(
                POLYBENCH_ZIG_VERSION.0,
                POLYBENCH_ZIG_VERSION.1,
                POLYBENCH_ZIG_VERSION.2,
            );

            // If required version is higher than polybench version, it's incompatible
            if required > polybench {
                ZigDepValidation::IncompatibleVersion { required, polybench }
            } else {
                ZigDepValidation::Compatible
            }
        }
    }
}

/// Revert a failed Zig dependency add by removing it from polybench.toml and build.zig.zon
fn revert_zig_dependency(
    project_root: &Path,
    zig_root: &Path,
    package: &str,
    dep_name: &str,
) -> Result<()> {
    // Remove from polybench.toml
    let mut manifest = crate::load_manifest(project_root)?;
    manifest.remove_zig_dependency(package)?;
    crate::save_manifest(project_root, &manifest)?;

    // Remove from build.zig.zon if it exists
    let zon_path = zig_root.join("build.zig.zon");
    if zon_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&zon_path) {
            // Remove the dependency entry using regex
            // Pattern: .dep_name = .{ ... },
            let pattern = format!(r"(?s)\s*\.{}\s*=\s*\.\{{[^}}]*\}},?", regex::escape(dep_name));
            if let Ok(re) = regex::Regex::new(&pattern) {
                let updated = re.replace(&content, "").to_string();
                if updated != content {
                    let _ = std::fs::write(&zon_path, updated);
                }
            }
        }
    }

    Ok(())
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

    // Parse and validate the git URL format
    let parsed_url = parse_zig_git_url(spec)?;

    // Store the original spec as the package identifier in manifest
    let package = spec.to_string();
    let version = parsed_url.commit_ref.clone();

    manifest.add_zig_dependency(&package, &version)?;
    crate::save_manifest(&project_root, &manifest)?;

    let zig_root = resolve_runtime_root(&project_root, Lang::Zig);
    std::fs::create_dir_all(&zig_root)
        .map_err(|e| miette::miette!("Failed to create Zig env dir: {}", e))?;

    ensure_zig_build_files(&zig_root)?;

    let dep_name = &parsed_url.dep_name;
    let fetch_url = &parsed_url.fetch_url;

    // Clear any stale cache entries for this dependency before fetching.
    // This ensures we get a fresh download and prevents corrupted cache issues.
    clear_zig_cache_for_dep(dep_name);

    let spinner = terminal::step_spinner(&format!("Fetching {}...", dep_name));
    let output = terminal::run_command_with_spinner(
        &spinner,
        Command::new("zig")
            .args(["fetch", &format!("--save={}", dep_name), fetch_url])
            .current_dir(&zig_root),
    )
    .map_err(|e| miette::miette!("Failed to run zig fetch: {}", e))?;

    if !output.status.success() {
        terminal::finish_failure(&spinner, "zig fetch failed");
        terminal::print_stderr_excerpt(&output.stderr, 8);
        // Revert the manifest changes since fetch failed
        let _ = revert_zig_dependency(&project_root, &zig_root, &package, dep_name);
        return Err(command_failure(
            &format!("zig fetch --save={} {}", dep_name, fetch_url),
            &zig_root,
            &output,
            "Verify the URL is valid and the package is accessible.",
        ));
    }

    // Extract the hash from the project's build.zig.zon for validation
    let dep_hash = extract_dep_hash_from_zon(&zig_root, dep_name);

    if let Some(hash) = dep_hash {
        // Validate the fetched package
        match validate_zig_dep_compatibility(&hash, dep_name) {
            ZigDepValidation::Compatible | ZigDepValidation::NoMinVersionSpecified => {
                terminal::finish_success(
                    &spinner,
                    &format!("Added {} ({})", dep_name, parsed_url.commit_ref),
                );
            }
            ZigDepValidation::MissingBuildZon => {
                terminal::finish_failure(&spinner, "Package validation failed");
                let _ = revert_zig_dependency(&project_root, &zig_root, &package, dep_name);
                return Err(miette::miette!(
                    "Package '{}' does not have a build.zig.zon file.\n\
                     Zig packages without build.zig.zon are not supported by Polybench.",
                    dep_name
                ));
            }
            ZigDepValidation::IncompatibleVersion { required, polybench } => {
                terminal::finish_failure(&spinner, "Package version incompatible");
                let _ = revert_zig_dependency(&project_root, &zig_root, &package, dep_name);
                return Err(miette::miette!(
                    "Package '{}' requires Zig >= {} (minimum_zig_version = \"{}\")\n\
                     Polybench uses Zig {}. This package is not compatible.",
                    dep_name,
                    required,
                    required,
                    polybench
                ));
            }
        }
    } else {
        // Could not extract hash from build.zig.zon - this shouldn't happen if fetch succeeded
        terminal::finish_failure(&spinner, "Could not validate package");
        let _ = revert_zig_dependency(&project_root, &zig_root, &package, dep_name);
        return Err(miette::miette!(
            "Could not extract package hash from build.zig.zon after fetch.\n\
             This may indicate a problem with the package or zig fetch."
        ));
    }

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
    } else {
        // Heal legacy build.zig.zon generated without required top-level fields or old name syntax.
        let content = std::fs::read_to_string(&build_zig_zon)
            .map_err(|e| miette::miette!("Failed to read build.zig.zon: {}", e))?;
        let mut updated = content.clone();
        if !updated.contains(".paths") {
            updated = templates::build_zig_zon();
        } else if updated.contains(".name = \"") {
            // Zig package manager expects enum literal for .name in modern build.zig.zon.
            let mut normalized = String::new();
            for line in updated.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with(".name = \"") && trimmed.ends_with("\",") {
                    let quote_start = line.find('"');
                    let quote_end = line.rfind('"');
                    if let (Some(start), Some(end)) = (quote_start, quote_end) {
                        let raw = &line[start + 1..end];
                        let enum_name: String = raw
                            .chars()
                            .map(|c| if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' })
                            .collect();
                        let prefix = &line[..start];
                        let suffix = &line[end + 1..];
                        normalized.push_str(prefix);
                        normalized.push('.');
                        normalized.push_str(&enum_name);
                        normalized.push_str(suffix);
                        normalized.push('\n');
                        continue;
                    }
                }
                normalized.push_str(line);
                normalized.push('\n');
            }
            updated = normalized;
        }
        if updated != content {
            std::fs::write(&build_zig_zon, updated)
                .map_err(|e| miette::miette!("Failed to refresh legacy build.zig.zon: {}", e))?;
        }
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

    // Extract dep_name from the package URL to clear cache
    if let Ok(parsed) = parse_zig_git_url(package) {
        clear_zig_cache_for_dep(&parsed.dep_name);
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

    for lang in poly_bench_runtime::supported_languages() {
        if manifest.has_runtime(*lang) {
            install_runtime_deps_for_lang(*lang, &project_root, &manifest)?;
        }
    }

    println!();
    terminal::success("All dependencies installed!");

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
        Lang::C => {
            install_c_deps(project_root, &manifest.project.name, manifest.c.as_ref().unwrap())
        }
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

/// Resolve vcpkg toolchain file path from VCPKG_ROOT or CMAKE_TOOLCHAIN_FILE env.
fn resolve_vcpkg_toolchain() -> Result<std::path::PathBuf> {
    fn discover_default_vcpkg_toolchain() -> Option<std::path::PathBuf> {
        let mut candidates: Vec<std::path::PathBuf> = Vec::new();

        if let Ok(home) = std::env::var("HOME") {
            let home = std::path::PathBuf::from(home);
            candidates
                .push(home.join("vcpkg").join("scripts").join("buildsystems").join("vcpkg.cmake"));
            candidates.push(
                home.join("src")
                    .join("vcpkg")
                    .join("scripts")
                    .join("buildsystems")
                    .join("vcpkg.cmake"),
            );
        }

        candidates.push(std::path::PathBuf::from(
            "/opt/homebrew/share/vcpkg/scripts/buildsystems/vcpkg.cmake",
        ));
        candidates.push(std::path::PathBuf::from(
            "/usr/local/share/vcpkg/scripts/buildsystems/vcpkg.cmake",
        ));
        candidates.push(std::path::PathBuf::from("/opt/vcpkg/scripts/buildsystems/vcpkg.cmake"));

        candidates.into_iter().find(|p| p.exists())
    }

    if let Ok(path) = std::env::var("CMAKE_TOOLCHAIN_FILE") {
        let p = std::path::PathBuf::from(&path);
        if p.exists() {
            return Ok(p);
        }
    }
    if let Ok(root) = std::env::var("VCPKG_ROOT") {
        let toolchain =
            std::path::Path::new(&root).join("scripts").join("buildsystems").join("vcpkg.cmake");
        if toolchain.exists() {
            return Ok(toolchain);
        }
    }
    if let Some(toolchain) = discover_default_vcpkg_toolchain() {
        return Ok(toolchain);
    }
    Err(miette::miette!(
        "vcpkg not found. Set VCPKG_ROOT to your vcpkg directory, or CMAKE_TOOLCHAIN_FILE to vcpkg.cmake. \
         Example: export VCPKG_ROOT=/path/to/vcpkg"
    ))
}

/// Get VCPKG_TARGET_TRIPLET for macOS (arm64-osx or x64-osx).
fn vcpkg_macos_triplet() -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        let arch = std::env::consts::ARCH;
        Some(if arch == "aarch64" || arch == "arm64" {
            "arm64-osx".to_string()
        } else {
            "x64-osx".to_string()
        })
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = std::env::consts::ARCH;
        None
    }
}

/// Minimal stub for bench_standalone.c (required by CMakeLists.txt at configure time).
const C_BENCH_STUB: &str = "int main(void) { return 0; }\n";

/// Run cmake configure in c_root to install vcpkg deps (manifest mode).
fn c_run_cmake_configure(c_root: &Path) -> Result<()> {
    which::which("cmake")
        .map_err(|_| miette::miette!("cmake not found in PATH. Install CMake 3.20+."))?;
    let stub_path = c_root.join("bench_standalone.c");
    if !stub_path.exists() {
        std::fs::write(&stub_path, C_BENCH_STUB)
            .map_err(|e| miette::miette!("Failed to write bench_standalone.c stub: {}", e))?;
    }
    let toolchain = resolve_vcpkg_toolchain()?;

    let mut args: Vec<String> = vec![
        "-S".to_string(),
        ".".to_string(),
        "-B".to_string(),
        "build".to_string(),
        format!("-DCMAKE_TOOLCHAIN_FILE={}", toolchain.display()),
    ];
    if let Some(triplet) = vcpkg_macos_triplet() {
        args.push(format!("-DVCPKG_TARGET_TRIPLET={}", triplet));
    }

    let spinner = terminal::indented_spinner("Running cmake configure (vcpkg manifest install)...");
    let output = terminal::run_command_with_spinner(
        &spinner,
        Command::new("cmake").args(&args).current_dir(c_root),
    )
    .map_err(|e| miette::miette!("Failed to run cmake: {}", e))?;

    if !output.status.success() {
        terminal::finish_failure_indented(&spinner, "cmake configure failed");
        terminal::print_stderr_excerpt(&output.stderr, 12);
        return Err(command_failure(
            &format!("cmake {}", args.join(" ")),
            c_root,
            &output,
            "Ensure vcpkg is installed, VCPKG_ROOT is set, and dependencies exist in vcpkg.",
        ));
    }
    terminal::finish_success_indented(&spinner, "vcpkg dependencies installed");
    Ok(())
}

/// Install C dependencies from manifest (vcpkg + CMake)
fn install_c_deps(
    project_root: &Path,
    project_name: &str,
    c_config: &manifest::CConfig,
) -> Result<()> {
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

    let deps: Vec<(String, String)> =
        c_config.dependencies.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

    // Sync vcpkg.json and CMakeLists.txt from manifest
    std::fs::write(&c_root.join("vcpkg.json"), templates::c_vcpkg_json(project_name, &deps))
        .map_err(|e| miette::miette!("Failed to write vcpkg.json: {}", e))?;
    std::fs::write(
        &c_root.join("CMakeLists.txt"),
        templates::c_cmake_lists(&c_config.standard, &deps),
    )
    .map_err(|e| miette::miette!("Failed to write CMakeLists.txt: {}", e))?;

    c_run_cmake_configure(&c_root)?;

    terminal::success_indented(&format!("C dependencies ready: {}", deps.len()));
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

    #[test]
    fn test_normalize_raw_github_repo_url_for_zig_fetch() {
        let normalized = normalize_zig_package_source("https://github.com/cod1r/sha3");
        assert_eq!(normalized, "git+https://github.com/cod1r/sha3.git");
    }

    #[test]
    fn test_keep_archive_url_unchanged_for_zig_fetch() {
        let url = "https://github.com/cod1r/sha3/archive/refs/heads/main.tar.gz";
        let normalized = normalize_zig_package_source(url);
        assert_eq!(normalized, url);
    }

    #[test]
    fn test_zig_package_url_appends_ref_to_normalized_git_url() {
        let url = zig_package_url("https://github.com/cod1r/sha3", "main");
        assert_eq!(url, "git+https://github.com/cod1r/sha3.git#main");
    }

    #[test]
    fn test_zig_package_url_adds_v_prefix_for_semver_ref() {
        let url = zig_package_url("https://github.com/StrobeLabs/eth.zig", "0.2.2");
        assert_eq!(url, "git+https://github.com/StrobeLabs/eth.zig.git#v0.2.2");
    }

    #[test]
    fn test_zig_dep_name_strips_git_suffix() {
        let name = zig_dep_name_from_url("git+https://github.com/StrobeLabs/eth.zig.git#v0.2.2");
        assert_eq!(name, "eth_zig");
    }

    // Tests for parse_zig_git_url

    #[test]
    fn test_parse_zig_git_url_with_commit_hash() {
        let result = parse_zig_git_url("git+https://github.com/discord-zig/discord.zig/#abc123def");
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(
            parsed.fetch_url,
            "git+https://github.com/discord-zig/discord.zig.git#abc123def"
        );
        assert_eq!(parsed.commit_ref, "abc123def");
        assert_eq!(parsed.dep_name, "discord_zig");
    }

    #[test]
    fn test_parse_zig_git_url_with_head() {
        let result = parse_zig_git_url("git+https://github.com/discord-zig/discord.zig/#HEAD");
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.fetch_url, "git+https://github.com/discord-zig/discord.zig.git#HEAD");
        assert_eq!(parsed.commit_ref, "HEAD");
    }

    #[test]
    fn test_parse_zig_git_url_defaults_to_head() {
        let result = parse_zig_git_url("git+https://github.com/discord-zig/discord.zig/");
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.commit_ref, "HEAD");
        assert!(parsed.fetch_url.ends_with("#HEAD"));
    }

    #[test]
    fn test_parse_zig_git_url_without_trailing_slash() {
        let result = parse_zig_git_url("git+https://github.com/discord-zig/discord.zig");
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.commit_ref, "HEAD");
    }

    #[test]
    fn test_parse_zig_git_url_with_git_suffix() {
        let result = parse_zig_git_url("git+https://github.com/discord-zig/discord.zig.git#main");
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.fetch_url, "git+https://github.com/discord-zig/discord.zig.git#main");
        assert_eq!(parsed.commit_ref, "main");
    }

    #[test]
    fn test_parse_zig_git_url_gitlab() {
        let result = parse_zig_git_url("git+https://gitlab.com/owner/repo/#abc123");
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.fetch_url, "git+https://gitlab.com/owner/repo.git#abc123");
    }

    #[test]
    fn test_parse_zig_git_url_rejects_missing_git_prefix() {
        let result = parse_zig_git_url("https://github.com/discord-zig/discord.zig");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("git+"));
    }

    #[test]
    fn test_parse_zig_git_url_rejects_http() {
        let result = parse_zig_git_url("git+http://github.com/discord-zig/discord.zig");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("HTTPS"));
    }

    #[test]
    fn test_parse_zig_git_url_rejects_unsupported_host() {
        let result = parse_zig_git_url("git+https://bitbucket.org/owner/repo");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("GitHub") || err.contains("GitLab"));
    }

    #[test]
    fn test_parse_zig_git_url_rejects_invalid_structure() {
        let result = parse_zig_git_url("git+https://github.com/invalid");
        assert!(result.is_err());
    }

    // Tests for parse_zon_minimum_version

    #[test]
    fn test_parse_zon_minimum_version_present() {
        let zon = r#"
.{
    .name = .test_package,
    .version = "0.1.0",
    .minimum_zig_version = "0.14.0",
}
"#;
        let version = parse_zon_minimum_version(zon);
        assert!(version.is_some());
        let v = version.unwrap();
        assert_eq!(v.major, 0);
        assert_eq!(v.minor, 14);
        assert_eq!(v.patch, 0);
    }

    #[test]
    fn test_parse_zon_minimum_version_with_patch() {
        let zon = r#".minimum_zig_version = "0.15.2","#;
        let version = parse_zon_minimum_version(zon);
        assert!(version.is_some());
        let v = version.unwrap();
        assert_eq!(v.major, 0);
        assert_eq!(v.minor, 15);
        assert_eq!(v.patch, 2);
    }

    #[test]
    fn test_parse_zon_minimum_version_absent() {
        let zon = r#"
.{
    .name = .test_package,
    .version = "0.1.0",
}
"#;
        let version = parse_zon_minimum_version(zon);
        assert!(version.is_none());
    }

    #[test]
    fn test_parse_zon_minimum_version_with_prerelease() {
        let zon = r#".minimum_zig_version = "0.15.0-dev+abc123","#;
        let version = parse_zon_minimum_version(zon);
        assert!(version.is_some());
        let v = version.unwrap();
        assert_eq!(v.major, 0);
        assert_eq!(v.minor, 15);
        assert_eq!(v.patch, 0);
    }

    // Tests for version compatibility

    #[test]
    fn test_polybench_zig_version_constant() {
        assert_eq!(POLYBENCH_ZIG_VERSION, (0, 15, 2));
    }

    #[test]
    fn test_extract_dep_hash_from_zon_content() {
        // Test that regex pattern matches Zig's hash format in build.zig.zon
        let zon_content = r#".{
    .name = .test_pkg,
    .version = "0.0.1",
    .dependencies = .{
        .sha3 = .{
            .url = "git+https://github.com/cod1r/sha3/",
            .hash = "N-V-__8AAHMxAACiSWxTMCAeALG0mPMHjnhdGUx07YnKBsmV",
        },
    },
    .paths = .{ "" },
}"#;

        // Test the regex pattern used in extract_dep_hash_from_zon
        let dep_pattern = format!(r#"\.{}\s*=\s*\.\{{"#, regex::escape("sha3"));
        let dep_re = regex::Regex::new(&dep_pattern).unwrap();
        assert!(dep_re.is_match(zon_content));

        let dep_match = dep_re.find(zon_content).unwrap();
        let after_dep = &zon_content[dep_match.start()..];

        let hash_re = regex::Regex::new(r#"\.hash\s*=\s*"([^"]+)""#).unwrap();
        let caps = hash_re.captures(after_dep).unwrap();
        let hash = caps.get(1).unwrap().as_str();
        assert_eq!(hash, "N-V-__8AAHMxAACiSWxTMCAeALG0mPMHjnhdGUx07YnKBsmV");
    }

    #[test]
    fn test_extract_dep_hash_pattern_with_underscore_name() {
        let zon_content = r#".{
    .dependencies = .{
        .eth_zig = .{
            .url = "git+https://github.com/StrobeLabs/eth.zig.git#v0.2.2",
            .hash = "efEm-kNdCQBcyYFAZG0FVkmPeN7lJkLLBDaTFp0T7lSD",
        },
    },
}"#;

        let dep_pattern = format!(r#"\.{}\s*=\s*\.\{{"#, regex::escape("eth_zig"));
        let dep_re = regex::Regex::new(&dep_pattern).unwrap();
        assert!(dep_re.is_match(zon_content));

        let dep_match = dep_re.find(zon_content).unwrap();
        let after_dep = &zon_content[dep_match.start()..];

        let hash_re = regex::Regex::new(r#"\.hash\s*=\s*"([^"]+)""#).unwrap();
        let caps = hash_re.captures(after_dep).unwrap();
        let hash = caps.get(1).unwrap().as_str();
        assert_eq!(hash, "efEm-kNdCQBcyYFAZG0FVkmPeN7lJkLLBDaTFp0T7lSD");
    }
}
