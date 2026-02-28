//! Build/regenerate the .polybench runtime environment
//!
//! This module recreates the `.polybench/runtime-env/` directory from:
//! - The `polybench.toml` manifest (languages, dependencies)
//! - Existing `.bench` files (for parsing imports if needed)
//!
//! Use this when the .polybench directory is deleted, corrupted, or after cloning
//! a repo where it was gitignored.

use crate::{error::ProjectError, manifest, runtime_env, templates, terminal};
use flate2::read::GzDecoder;
use miette::Result;
use poly_bench_dsl::Lang;
use std::{
    io::{Read, Write},
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

/// Build/regenerate the .polybench runtime environment.
/// Uses `find_project_root(&current_dir)` to locate the project.
pub fn build_project(options: &BuildOptions) -> Result<()> {
    let current_dir = std::env::current_dir()
        .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

    let project_root = crate::find_project_root(&current_dir).ok_or_else(|| {
        miette::miette!("Not in a poly-bench project. Run 'poly-bench init' first.")
    })?;

    build_project_at(&project_root, options)
}

/// Build/regenerate the .polybench runtime environment at a given project root.
/// Use this when you have the project path (e.g. in tests) to avoid cwd manipulation.
/// Calls the same install functions as `build_project` (install_local_gopls, etc.).
pub fn build_project_at(project_root: &Path, options: &BuildOptions) -> Result<()> {
    let manifest = crate::load_manifest(project_root)?;

    let spinner = terminal::step_spinner(&format!(
        "Building runtime environment for '{}'...",
        manifest.project.name
    ));
    terminal::ensure_min_display(&spinner);
    spinner.finish_and_clear();

    for lang in poly_bench_runtime::supported_languages() {
        if manifest.has_runtime(*lang) {
            build_runtime_env_for_lang(*lang, project_root, &manifest, options)?;
        }
    }

    println!();
    terminal::success("Runtime environment ready!");

    Ok(())
}

fn build_runtime_env_for_lang(
    lang: Lang,
    project_root: &Path,
    manifest: &manifest::Manifest,
    options: &BuildOptions,
) -> Result<()> {
    if !crate::runtime_check::is_lang_installed(lang) {
        return Err(crate::runtime_check::not_installed_error(lang));
    }
    match lang {
        Lang::Go => build_go_env(project_root, manifest.go.as_ref().unwrap(), options),
        Lang::TypeScript => build_ts_env(
            project_root,
            manifest.ts.as_ref().unwrap(),
            &manifest.project.name,
            options,
        ),
        Lang::Rust => build_rust_env(project_root, manifest.rust.as_ref().unwrap(), options),
        Lang::Python => build_python_env(project_root, manifest.python.as_ref().unwrap(), options),
        Lang::C => build_c_env(project_root, manifest.c.as_ref().unwrap(), options),
        Lang::CSharp => build_csharp_env(project_root, manifest.csharp.as_ref().unwrap(), options),
        Lang::Zig => build_zig_env(project_root, manifest.zig.as_ref().unwrap(), options),
    }
}

/// Build/regenerate the Go runtime environment
fn build_go_env(
    project_root: &Path,
    go_config: &manifest::GoConfig,
    options: &BuildOptions,
) -> Result<()> {
    terminal::section("Go environment");

    let go_env = runtime_env(project_root, Lang::Go);

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

    install_local_gopls(&go_env, options)?;

    terminal::success_indented("Go environment ready");

    Ok(())
}

/// Install gopls locally for LSP support
fn install_local_gopls(go_env: &Path, options: &BuildOptions) -> Result<()> {
    let bin_dir = go_env.join("bin");
    std::fs::create_dir_all(&bin_dir)
        .map_err(|e| miette::miette!("Failed to create bin directory: {}", e))?;

    let gopls_name = if cfg!(windows) { "gopls.exe" } else { "gopls" };
    let gopls_path = bin_dir.join(gopls_name);

    if gopls_path.exists() && !options.force {
        terminal::info_indented("gopls already installed (use --force to reinstall)");
        return Ok(());
    }

    if options.skip_install {
        terminal::info_indented("Skipping gopls install (--skip-install)");
        return Ok(());
    }

    let go_binary = which::which("go").map_err(|_| {
        miette::miette!("Go not found in PATH. Install Go to enable gopls for LSP support.")
    })?;

    let spinner = terminal::indented_spinner("Installing gopls for LSP support...");

    let output = terminal::run_command_with_spinner(
        &spinner,
        Command::new(&go_binary)
            .args(["install", "golang.org/x/tools/gopls@latest"])
            .env("GOBIN", bin_dir.to_str().unwrap())
            .current_dir(go_env),
    )
    .map_err(|e| miette::miette!("Failed to run go install: {}", e))?;

    if output.status.success() {
        terminal::finish_success_indented(&spinner, "gopls installed");
    } else {
        terminal::finish_warning_indented(
            &spinner,
            "gopls install failed; embedded Go hover may not work",
        );
        terminal::print_stderr_excerpt(&output.stderr, 6);
    }

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

    let ts_env = runtime_env(project_root, Lang::TypeScript);

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

    let rust_env = runtime_env(project_root, Lang::Rust);

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

    install_local_rust_analyzer(&rust_env, options)?;

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

    let python_env = runtime_env(project_root, Lang::Python);

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

fn build_csharp_env(
    project_root: &Path,
    csharp_config: &manifest::CSharpConfig,
    options: &BuildOptions,
) -> Result<()> {
    terminal::section("C# environment");

    let csharp_env = runtime_env(project_root, Lang::CSharp);
    std::fs::create_dir_all(&csharp_env)
        .map_err(|e| miette::miette!("Failed to create {}: {}", csharp_env.display(), e))?;

    let csproj_path = csharp_env.join("polybench.csproj");
    if !csproj_path.exists() || options.force {
        let csproj_content = templates::csharp_csproj(&csharp_config.target_framework);
        std::fs::write(&csproj_path, csproj_content)
            .map_err(|e| miette::miette!("Failed to write polybench.csproj: {}", e))?;
        if options.force {
            terminal::success_indented("Regenerated polybench.csproj");
        } else {
            terminal::success_indented("Created polybench.csproj");
        }
    } else {
        terminal::info_indented("polybench.csproj exists (use --force to regenerate)");
    }

    let program_path = csharp_env.join("Program.cs");
    if !program_path.exists() {
        std::fs::write(
            &program_path,
            "public static class Program { public static void Main() {} }\n",
        )
        .map_err(|e| miette::miette!("Failed to write Program.cs: {}", e))?;
        terminal::success_indented("Created Program.cs");
    }

    let nuget_config_path = csharp_env.join("NuGet.config");
    if !nuget_config_path.exists() || options.force {
        std::fs::write(&nuget_config_path, templates::csharp_nuget_config())
            .map_err(|e| miette::miette!("Failed to write NuGet.config: {}", e))?;
        if nuget_config_path.exists() && options.force {
            terminal::info_indented(
                "Regenerated NuGet.config (vs-impl feed for roslyn-language-server)",
            );
        }
    }

    install_local_roslyn_language_server(&csharp_env, options)?;

    if !csharp_config.dependencies.is_empty() {
        for (package, version) in &csharp_config.dependencies {
            let spec = if version == "latest" {
                package.clone()
            } else {
                format!("{}@{}", package, version)
            };
            let spinner = terminal::indented_spinner(&format!("Adding {}...", spec));
            let output = terminal::run_command_with_spinner(
                &spinner,
                Command::new("dotnet")
                    .args(["add", "polybench.csproj", "package", package, "--version", version])
                    .current_dir(&csharp_env),
            )
            .map_err(|e| miette::miette!("Failed to run dotnet add package: {}", e))?;
            if !output.status.success() {
                terminal::finish_failure_indented(
                    &spinner,
                    &format!("Failed to add package {}", package),
                );
                terminal::print_stderr_excerpt(&output.stderr, 8);
                return Err(command_failure(
                    &format!("dotnet add package {}", package),
                    &csharp_env,
                    &output,
                    "Fix NuGet package/version and rerun build.",
                ));
            }
            terminal::finish_success_indented(&spinner, package);
        }
    }

    if !options.skip_install {
        let spinner = terminal::indented_spinner("Running dotnet restore...");
        let output = terminal::run_command_with_spinner(
            &spinner,
            Command::new("dotnet").args(["restore", "polybench.csproj"]).current_dir(&csharp_env),
        )
        .map_err(|e| miette::miette!("Failed to run dotnet restore: {}", e))?;

        if output.status.success() {
            terminal::finish_success_indented(&spinner, "C# dependencies restored");
        } else {
            terminal::finish_failure_indented(&spinner, "dotnet restore failed");
            terminal::print_stderr_excerpt(&output.stderr, 8);
            return Err(command_failure(
                "dotnet restore polybench.csproj",
                &csharp_env,
                &output,
                "Fix NuGet restore issues and rerun build.",
            ));
        }
    } else {
        terminal::info_indented("Skipping dotnet restore (--skip-install)");
    }

    terminal::success_indented("C# environment ready");
    Ok(())
}

fn build_c_env(
    project_root: &Path,
    c_config: &manifest::CConfig,
    _options: &BuildOptions,
) -> Result<()> {
    terminal::section("C environment");

    let c_env = runtime_env(project_root, Lang::C);
    std::fs::create_dir_all(&c_env)
        .map_err(|e| miette::miette!("Failed to create {}: {}", c_env.display(), e))?;

    let main_c_path = c_env.join("main.c");
    if !main_c_path.exists() {
        std::fs::write(&main_c_path, "int main(void) {\n    return 0;\n}\n")
            .map_err(|e| miette::miette!("Failed to write main.c: {}", e))?;
        terminal::success_indented("Created main.c");
    } else {
        terminal::info_indented("main.c exists");
    }

    if c_config.dependencies.is_empty() {
        terminal::info_indented("No C dependencies declared");
    } else {
        terminal::info_indented(&format!(
            "C dependencies recorded in manifest: {}",
            c_config.dependencies.len()
        ));
    }

    terminal::success_indented("C environment ready");
    Ok(())
}

fn build_zig_env(
    project_root: &Path,
    _zig_config: &manifest::ZigConfig,
    _options: &BuildOptions,
) -> Result<()> {
    terminal::section("Zig environment");

    let zig_env = runtime_env(project_root, Lang::Zig);
    std::fs::create_dir_all(&zig_env)
        .map_err(|e| miette::miette!("Failed to create {}: {}", zig_env.display(), e))?;

    let build_zig_path = zig_env.join("build.zig");
    if !build_zig_path.exists() {
        std::fs::write(&build_zig_path, templates::build_zig())
            .map_err(|e| miette::miette!("Failed to write build.zig: {}", e))?;
        terminal::success_indented("Created build.zig");
    } else {
        terminal::info_indented("build.zig exists");
    }

    let build_zig_zon_path = zig_env.join("build.zig.zon");
    if !build_zig_zon_path.exists() {
        std::fs::write(&build_zig_zon_path, templates::build_zig_zon())
            .map_err(|e| miette::miette!("Failed to write build.zig.zon: {}", e))?;
        terminal::success_indented("Created build.zig.zon");
    } else {
        terminal::info_indented("build.zig.zon exists");
    }

    let src_dir = zig_env.join("src");
    std::fs::create_dir_all(&src_dir)
        .map_err(|e| miette::miette!("Failed to create src directory: {}", e))?;

    let main_zig_path = src_dir.join("main.zig");
    if !main_zig_path.exists() {
        std::fs::write(&main_zig_path, templates::main_zig())
            .map_err(|e| miette::miette!("Failed to write main.zig: {}", e))?;
        terminal::success_indented("Created src/main.zig");
    } else {
        terminal::info_indented("src/main.zig exists");
    }

    install_local_zls(&zig_env, _options)?;

    terminal::success_indented("Zig environment ready");
    Ok(())
}

/// ZLS version to download (pinned for reproducibility)
const ZLS_VERSION: &str = "0.15.1";

/// Download and install ZLS locally for LSP support
fn install_local_zls(zig_env: &Path, options: &BuildOptions) -> Result<()> {
    let bin_dir = zig_env.join("bin");
    std::fs::create_dir_all(&bin_dir)
        .map_err(|e| miette::miette!("Failed to create bin directory: {}", e))?;

    let zls_path = bin_dir.join(if cfg!(windows) { "zls.exe" } else { "zls" });

    if zls_path.exists() && !options.force {
        terminal::info_indented("ZLS already installed (use --force to reinstall)");
        return Ok(());
    }

    let (arch, os) = zls_platform();
    let archive_name = format!("zls-{}-{}.tar.xz", arch, os);
    let url = format!(
        "https://github.com/zigtools/zls/releases/download/{}/{}",
        ZLS_VERSION, archive_name
    );

    terminal::info_indented(&format!("Downloading ZLS {} for {}-{}...", ZLS_VERSION, arch, os));

    let mut response = ureq::get(&url).call().map_err(|e| {
        miette::miette!("Failed to download ZLS: {}. Ensure you have network access.", e)
    })?;

    let body = response
        .body_mut()
        .with_config()
        .limit(200 * 1024 * 1024)
        .read_to_vec()
        .map_err(|e| miette::miette!("Failed to read ZLS download: {}", e))?;

    let temp_dir =
        tempfile::tempdir().map_err(|e| miette::miette!("Failed to create temp dir: {}", e))?;
    let archive_path = temp_dir.path().join(&archive_name);
    std::fs::write(&archive_path, &body)
        .map_err(|e| miette::miette!("Failed to write ZLS archive: {}", e))?;

    let extract_dir = temp_dir.path().join("extract");
    std::fs::create_dir_all(&extract_dir)
        .map_err(|e| miette::miette!("Failed to create extract dir: {}", e))?;

    let status = Command::new("tar")
        .args(["-xJf", archive_path.to_str().unwrap(), "-C", extract_dir.to_str().unwrap()])
        .status()
        .map_err(|e| miette::miette!("Failed to extract ZLS (tar required): {}", e))?;

    if !status.success() {
        return Err(miette::miette!(
            "Failed to extract ZLS archive. Ensure 'tar' supports xz (e.g. tar -xJf)."
        ));
    }

    let zls_name = if cfg!(windows) { "zls.exe" } else { "zls" };
    let zls_binary = find_zls_in_dir(&extract_dir, zls_name);

    let src = zls_binary.ok_or_else(|| {
        miette::miette!("ZLS binary not found in archive. Archive structure may have changed.")
    })?;

    std::fs::copy(&src, &zls_path)
        .map_err(|e| miette::miette!("Failed to copy ZLS to {}: {}", zls_path.display(), e))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&zls_path)
            .map_err(|e| miette::miette!("Failed to read ZLS metadata: {}", e))?
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&zls_path, perms)
            .map_err(|e| miette::miette!("Failed to set ZLS executable: {}", e))?;
    }

    terminal::success_indented(&format!("Installed ZLS at {}", zls_path.display()));
    Ok(())
}

fn find_zls_in_dir(dir: &Path, zls_name: &str) -> Option<std::path::PathBuf> {
    let direct = dir.join(zls_name);
    if direct.exists() {
        return Some(direct);
    }
    for entry in std::fs::read_dir(dir).ok()?.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name == "zls" || name == "zls.exe" {
                return Some(path);
            }
        } else if path.is_dir() {
            if let Some(found) = find_zls_in_dir(&path, zls_name) {
                return Some(found);
            }
        }
    }
    None
}

fn zls_platform() -> (&'static str, &'static str) {
    let arch = std::env::consts::ARCH;
    let os = std::env::consts::OS;
    let arch = match arch {
        "x86_64" => "x86_64",
        "aarch64" | "arm64" => "aarch64",
        _ => "x86_64",
    };
    let os = match os {
        "macos" => "macos",
        "linux" => "linux",
        "windows" => "windows",
        _ => "linux",
    };
    (arch, os)
}

/// rust-analyzer version to download (pinned for reproducibility)
const RUST_ANALYZER_VERSION: &str = "2024-11-25";

/// Download and install rust-analyzer locally for LSP support
fn install_local_rust_analyzer(rust_env: &Path, options: &BuildOptions) -> Result<()> {
    let bin_dir = rust_env.join("bin");
    std::fs::create_dir_all(&bin_dir)
        .map_err(|e| miette::miette!("Failed to create bin directory: {}", e))?;

    let ra_name = if cfg!(windows) { "rust-analyzer.exe" } else { "rust-analyzer" };
    let ra_path = bin_dir.join(ra_name);

    if ra_path.exists() && !options.force {
        terminal::info_indented("rust-analyzer already installed (use --force to reinstall)");
        return Ok(());
    }

    if options.skip_install {
        terminal::info_indented("Skipping rust-analyzer install (--skip-install)");
        return Ok(());
    }

    let (arch, os) = rust_analyzer_platform();
    let target = format!("{}-{}", arch, os);

    let (url, is_zip) = if cfg!(windows) {
        (
            format!(
                "https://github.com/rust-lang/rust-analyzer/releases/download/{}/rust-analyzer-{}.zip",
                RUST_ANALYZER_VERSION, target
            ),
            true,
        )
    } else {
        (
            format!(
                "https://github.com/rust-lang/rust-analyzer/releases/download/{}/rust-analyzer-{}.gz",
                RUST_ANALYZER_VERSION, target
            ),
            false,
        )
    };

    terminal::info_indented(&format!(
        "Downloading rust-analyzer {} for {}...",
        RUST_ANALYZER_VERSION, target
    ));

    let mut response = ureq::get(&url).call().map_err(|e| {
        miette::miette!("Failed to download rust-analyzer: {}. Ensure you have network access.", e)
    })?;

    let body = response
        .body_mut()
        .with_config()
        .limit(200 * 1024 * 1024)
        .read_to_vec()
        .map_err(|e| miette::miette!("Failed to read rust-analyzer download: {}", e))?;

    let _temp_dir =
        tempfile::tempdir().map_err(|e| miette::miette!("Failed to create temp dir: {}", e))?;

    if is_zip {
        // Windows: extract from zip (contains rust-analyzer.exe)
        let cursor = std::io::Cursor::new(&body);
        let mut archive = zip::ZipArchive::new(cursor)
            .map_err(|e| miette::miette!("Invalid zip archive: {}", e))?;

        // Find index of rust-analyzer.exe or rust-analyzer
        let idx = (0..archive.len())
            .find_map(|i| {
                let name = archive.by_index(i).ok()?.name().to_string();
                if name.ends_with("rust-analyzer.exe") || name.ends_with("rust-analyzer") {
                    Some(i)
                } else {
                    None
                }
            })
            .unwrap_or(0);

        let mut file = archive
            .by_index(idx)
            .map_err(|e| miette::miette!("Failed to extract rust-analyzer from archive: {}", e))?;

        let mut out = std::fs::File::create(&ra_path)
            .map_err(|e| miette::miette!("Failed to create {}: {}", ra_path.display(), e))?;
        std::io::copy(&mut file, &mut out)
            .map_err(|e| miette::miette!("Failed to extract rust-analyzer: {}", e))?;
    } else {
        // Linux/macOS: decompress gzip
        let mut decoder = GzDecoder::new(&body[..]);
        let mut binary = Vec::new();
        decoder
            .read_to_end(&mut binary)
            .map_err(|e| miette::miette!("Failed to decompress rust-analyzer: {}", e))?;

        let mut out = std::fs::File::create(&ra_path)
            .map_err(|e| miette::miette!("Failed to create {}: {}", ra_path.display(), e))?;
        out.write_all(&binary)
            .map_err(|e| miette::miette!("Failed to write rust-analyzer: {}", e))?;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&ra_path)
            .map_err(|e| miette::miette!("Failed to read rust-analyzer metadata: {}", e))?
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&ra_path, perms)
            .map_err(|e| miette::miette!("Failed to set rust-analyzer executable: {}", e))?;
    }

    terminal::success_indented(&format!("Installed rust-analyzer at {}", ra_path.display()));
    Ok(())
}

fn rust_analyzer_platform() -> (&'static str, &'static str) {
    let arch = std::env::consts::ARCH;
    let os = std::env::consts::OS;
    let arch = match arch {
        "x86_64" => "x86_64",
        "aarch64" | "arm64" => "aarch64",
        _ => "x86_64",
    };
    let os = match os {
        "macos" => "apple-darwin",
        "linux" => "unknown-linux-gnu",
        "windows" => "pc-windows-msvc",
        _ => "unknown-linux-gnu",
    };
    (arch, os)
}

/// Roslyn Language Server version (pinned for reproducibility)
const ROSLYN_LANGUAGE_SERVER_VERSION: &str = "5.5.0-2.26103.6";
/// csharp-ls version (fallback when roslyn-language-server has DotnetToolSettings.xml packaging
/// issues). Pinned to 0.16.0 for .NET 8 compatibility; 0.20+ requires .NET 10.
const CSHARP_LS_VERSION: &str = "0.16.0";

/// dnceng dotnet-tools feed (https://github.com/dotnet/roslyn#nuget-feeds) - may have
/// builds with correct DotnetToolSettings.xml when NuGet.org package is broken.
const ROSLYN_DOTNET_TOOLS_FEED: &str =
    "https://pkgs.dev.azure.com/dnceng/public/_packaging/dotnet-tools/nuget/v3/index.json";

/// Install C# LSP locally. Tries roslyn-language-server first, falls back to csharp-ls when
/// roslyn-language-server has packaging issues (e.g. DotnetToolSettings.xml).
/// Uses `dotnet tool install --tool-path` so the LSP is self-contained and version-matched to the
/// SDK.
fn install_local_roslyn_language_server(csharp_env: &Path, options: &BuildOptions) -> Result<()> {
    let local_dir = csharp_env.join(".csharp-ls");
    std::fs::create_dir_all(&local_dir)
        .map_err(|e| miette::miette!("Failed to create {}: {}", local_dir.display(), e))?;

    let (roslyn_bin, csharp_ls_bin) = if cfg!(windows) {
        ("roslyn-language-server.exe", "csharp-ls.cmd")
    } else {
        ("roslyn-language-server", "csharp-ls")
    };
    let roslyn_path = local_dir.join(roslyn_bin);
    let csharp_ls_path = local_dir.join(csharp_ls_bin);

    if (roslyn_path.exists() || csharp_ls_path.exists()) && !options.force {
        terminal::info_indented("C# LSP already installed (use --force to reinstall)");
        return Ok(());
    }

    if options.skip_install {
        terminal::info_indented("Skipping Roslyn Language Server install (--skip-install)");
        return Ok(());
    }

    // Prefer POLYBENCH_CSHARP_LSP_BIN or roslyn-language-server in PATH (copy works)
    // Don't copy csharp-ls from PATH - the shim has path resolution issues; use local install
    // instead
    let src = std::env::var("POLYBENCH_CSHARP_LSP_BIN")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .map(std::path::PathBuf::from)
        .filter(|p| p.exists())
        .or_else(|| which::which("roslyn-language-server").ok());

    if let Some(src_path) = src {
        let dest_path = local_dir.join(roslyn_bin);
        std::fs::copy(&src_path, &dest_path).map_err(|e| {
            miette::miette!(
                "Failed to copy Roslyn Language Server from {} to {}: {}",
                src_path.display(),
                dest_path.display(),
                e
            )
        })?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&dest_path)
                .map_err(|e| {
                    miette::miette!(
                        "Failed to read local Roslyn Language Server metadata ({}): {}",
                        dest_path.display(),
                        e
                    )
                })?
                .permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&dest_path, perms).map_err(|e| {
                miette::miette!("Failed to set executable bit on {}: {}", dest_path.display(), e)
            })?;
        }
        terminal::success_indented(&format!(
            "Installed local Roslyn Language Server at {}",
            dest_path.display()
        ));
        return Ok(());
    }

    // Install via dotnet tool install
    let dotnet = which::which("dotnet").map_err(|_| {
        miette::miette!("dotnet not found in PATH. Install .NET SDK to enable C# LSP support.")
    })?;

    let spinner = terminal::indented_spinner("Installing Roslyn Language Server...");

    let output = terminal::run_command_with_spinner(
        &spinner,
        Command::new(&dotnet)
            .args([
                "tool",
                "install",
                "roslyn-language-server",
                "--tool-path",
                local_dir.to_str().unwrap(),
                "--version",
                ROSLYN_LANGUAGE_SERVER_VERSION,
                "--add-source",
                ROSLYN_DOTNET_TOOLS_FEED,
            ])
            .current_dir(csharp_env),
    )
    .map_err(|e| miette::miette!("Failed to run dotnet tool install: {}", e))?;

    if output.status.success() {
        terminal::finish_success_indented(&spinner, "Roslyn Language Server installed");
        return Ok(());
    }

    let roslyn_package_bug =
        String::from_utf8_lossy(&output.stderr).contains("DotnetToolSettings.xml");
    if roslyn_package_bug {
        terminal::finish_warning_indented(
            &spinner,
            "roslyn-language-server has packaging issues; trying csharp-ls...",
        );
    } else {
        terminal::finish_warning_indented(
            &spinner,
            "roslyn-language-server install failed; trying csharp-ls...",
        );
    }

    // Fallback: csharp-ls via local manifest (dotnet tool run resolves paths correctly)
    let spinner2 = terminal::indented_spinner("Installing csharp-ls (local)...");
    // Create manifest if missing (required for dotnet tool install --local)
    let manifest_path = csharp_env.join("dotnet-tools.json");
    if !manifest_path.exists() {
        let out = Command::new(&dotnet)
            .args(["new", "tool-manifest"])
            .current_dir(csharp_env)
            .output()
            .map_err(|e| miette::miette!("Failed to create tool manifest: {}", e))?;
        if !out.status.success() {
            terminal::finish_warning_indented(
                &spinner2,
                "C# LSP install failed; could not create tool manifest",
            );
            terminal::print_stderr_excerpt(&out.stderr, 8);
            return Ok(());
        }
    }
    let output2 = terminal::run_command_with_spinner(
        &spinner2,
        Command::new(&dotnet)
            .args(["tool", "install", "csharp-ls", "--version", CSHARP_LS_VERSION])
            .current_dir(csharp_env),
    )
    .map_err(|e| miette::miette!("Failed to run dotnet tool install: {}", e))?;

    if output2.status.success() {
        terminal::finish_success_indented(&spinner2, "csharp-ls installed");
    } else {
        terminal::finish_warning_indented(
            &spinner2,
            "C# LSP install failed; C# LSP may not work (requires .NET 8+)",
        );
        terminal::print_stderr_excerpt(&output2.stderr, 8);
    }

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
