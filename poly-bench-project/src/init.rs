//! Project initialization and benchmark file creation

use crate::{
    manifest::{self, Manifest},
    runtime_env, templates, terminal, BENCHMARKS_DIR, MANIFEST_FILENAME,
};
use miette::Result;
use poly_bench_dsl::Lang;
use std::{path::PathBuf, process::Command};

/// Options for initializing a project
pub struct InitOptions {
    /// Project name
    pub name: String,
    /// Languages to enable
    pub languages: Vec<String>,
    /// Skip generating example benchmark
    pub no_example: bool,
    /// When true, skip all success messages and final "Next steps" (caller prints T3-style output)
    pub quiet: bool,
    /// When true, skip the final success/next-steps message (caller will print after running
    /// build)
    pub defer_final_message: bool,
}

/// Initialize a new poly-bench project
pub fn init_project(options: &InitOptions) -> Result<PathBuf> {
    let project_dir = if options.name == "." {
        std::env::current_dir()
            .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?
    } else {
        PathBuf::from(&options.name)
    };

    // Get the actual project name for the manifest (used in go.mod, package.json, etc.).
    // When given a path, use the directory component so we get a valid module name (e.g.
    // "lsp-test-go" not "/full/path/lsp-test-go" which would be invalid for Go).
    let project_name = if options.name == "." {
        project_dir.file_name().and_then(|s| s.to_str()).unwrap_or("my-project").to_string()
    } else {
        let p = std::path::Path::new(&options.name);
        if p.has_root() || p.components().count() > 1 {
            p.file_name().and_then(|s| s.to_str()).unwrap_or("my-project").to_string()
        } else {
            options.name.clone()
        }
    };

    // Check if already a project
    if project_dir.join(MANIFEST_FILENAME).exists() {
        return Err(miette::miette!(
            "A poly-bench project already exists in {}",
            project_dir.display()
        ));
    }

    // Create project directory if it doesn't exist
    if !project_dir.exists() {
        std::fs::create_dir_all(&project_dir)
            .map_err(|e| miette::miette!("Failed to create project directory: {}", e))?;
        if !options.quiet {
            terminal::success(&format!("Created directory {}", project_dir.display()));
        }
    }

    let enabled_langs: Vec<Lang> = poly_bench_runtime::supported_languages()
        .iter()
        .copied()
        .filter(|lang| {
            options.languages.iter().any(|raw| Lang::from_str(raw.trim()) == Some(*lang))
        })
        .collect();
    let languages: Vec<String> = enabled_langs.iter().map(|l| l.as_str().to_string()).collect();

    // Create manifest
    let manifest = Manifest::new(&project_name, &languages);
    let manifest_path = project_dir.join(MANIFEST_FILENAME);
    manifest::save(&manifest_path, &manifest)?;
    if !options.quiet {
        terminal::success(&format!("Created {}", MANIFEST_FILENAME));
    }

    // Create benchmarks directory
    let benchmarks_dir = project_dir.join(BENCHMARKS_DIR);
    std::fs::create_dir_all(&benchmarks_dir)
        .map_err(|e| miette::miette!("Failed to create benchmarks directory: {}", e))?;
    if !options.quiet {
        terminal::success(&format!("Created {}/", BENCHMARKS_DIR));
    }

    // Create example benchmark and fixtures
    if !options.no_example {
        let example_path = benchmarks_dir.join("example.bench");
        let example_content = templates::example_bench_for_langs(&enabled_langs);
        std::fs::write(&example_path, example_content)
            .map_err(|e| miette::miette!("Failed to write example.bench: {}", e))?;
        templates::write_bubble_fixtures(&benchmarks_dir)?;
        if !options.quiet {
            terminal::success(&format!("Created {}/example.bench", BENCHMARKS_DIR));
            terminal::success(&format!("Created {}/fixtures/sort/", BENCHMARKS_DIR));
        }
    }

    // Create runtime-env dirs and language-specific env files (keeps root uncluttered)
    for lang in poly_bench_runtime::supported_languages() {
        if manifest.has_runtime(*lang) {
            init_runtime_env_for_lang(
                *lang,
                &project_dir,
                &manifest,
                &project_name,
                options.quiet,
            )?;
        }
    }

    // Create .gitignore
    let gitignore_path = project_dir.join(".gitignore");
    // Append to existing .gitignore or create new one
    let gitignore_content = if gitignore_path.exists() {
        let existing = std::fs::read_to_string(&gitignore_path).unwrap_or_default();
        if !existing.contains(".polybench/") {
            format!("{}\n\n# poly-bench\n{}", existing.trim(), templates::gitignore())
        } else {
            existing
        }
    } else {
        templates::gitignore().to_string()
    };
    std::fs::write(&gitignore_path, gitignore_content)
        .map_err(|e| miette::miette!("Failed to write .gitignore: {}", e))?;
    if !options.quiet {
        terminal::success("Created .gitignore");
    }

    // Create README.md
    let readme_path = project_dir.join("README.md");
    if !readme_path.exists() {
        let readme_content = templates::readme_for_langs(&project_name, &enabled_langs);
        std::fs::write(&readme_path, readme_content)
            .map_err(|e| miette::miette!("Failed to write README.md: {}", e))?;
        if !options.quiet {
            terminal::success("Created README.md");
        }
    }

    if !options.quiet && !options.defer_final_message {
        println!();
        terminal::success(&format!("Project '{}' initialized successfully!", project_name));
        println!();
        println!("Next steps:");
        if options.name != "." {
            println!("  cd {}", project_name);
        }
        println!("  poly-bench install    # Install dependencies");
        println!("  poly-bench run        # Run benchmarks");
        println!();
    }

    Ok(project_dir)
}

fn init_runtime_env_for_lang(
    lang: Lang,
    project_dir: &std::path::Path,
    manifest: &Manifest,
    project_name: &str,
    quiet: bool,
) -> Result<()> {
    match lang {
        Lang::Go => {
            let go_env = runtime_env(project_dir, Lang::Go);
            std::fs::create_dir_all(&go_env)
                .map_err(|e| miette::miette!("Failed to create {}: {}", go_env.display(), e))?;
            let go_version = manifest.go.as_ref().and_then(|g| g.version.as_deref());
            let go_mod_content = templates::go_mod(project_name, go_version);
            std::fs::write(go_env.join("go.mod"), go_mod_content)
                .map_err(|e| miette::miette!("Failed to write go.mod: {}", e))?;
            if !quiet {
                terminal::success("Created .polybench/runtime-env/go/ (go.mod)");
            }
        }
        Lang::TypeScript => {
            let ts_env = runtime_env(project_dir, Lang::TypeScript);
            std::fs::create_dir_all(&ts_env)
                .map_err(|e| miette::miette!("Failed to create {}: {}", ts_env.display(), e))?;
            let package_json_content = templates::package_json_pretty(project_name);
            std::fs::write(ts_env.join("package.json"), package_json_content)
                .map_err(|e| miette::miette!("Failed to write package.json: {}", e))?;
            let tsconfig_content = templates::tsconfig_json();
            std::fs::write(ts_env.join("tsconfig.json"), tsconfig_content)
                .map_err(|e| miette::miette!("Failed to write tsconfig.json: {}", e))?;
            if !quiet {
                terminal::success(
                    "Created .polybench/runtime-env/ts/ (package.json, tsconfig.json)",
                );
            }

            if !crate::runtime_check::is_lang_installed(Lang::TypeScript) {
                return Err(crate::runtime_check::not_installed_error(Lang::TypeScript));
            }
            if quiet {
                let output = Command::new("npm")
                    .arg("install")
                    .current_dir(&ts_env)
                    .output()
                    .map_err(|e| miette::miette!("Could not run npm install: {}", e))?;
                if !output.status.success() {
                    return Err(miette::miette!(
                        "npm install failed in {}:\n{}",
                        ts_env.display(),
                        terminal::stderr_excerpt(&output.stderr, 10)
                    ));
                }
            } else {
                let spinner = terminal::step_spinner("Installing TypeScript dependencies...");
                let npm_result = terminal::run_command_with_spinner(
                    &spinner,
                    Command::new("npm").arg("install").current_dir(&ts_env),
                );
                match npm_result {
                    Ok(output) if output.status.success() => {
                        terminal::finish_success(&spinner, "TypeScript dependencies installed");
                    }
                    Ok(output) => {
                        terminal::finish_failure(&spinner, "npm install failed");
                        terminal::print_stderr_excerpt(&output.stderr, 8);
                        return Err(miette::miette!("npm install failed in {}", ts_env.display()));
                    }
                    Err(e) => {
                        terminal::finish_failure(&spinner, &format!("Could not run npm: {}", e));
                        return Err(miette::miette!("Could not run npm install: {}", e));
                    }
                }
            }
        }
        Lang::Rust => {
            let rust_env = runtime_env(project_dir, Lang::Rust);
            std::fs::create_dir_all(&rust_env)
                .map_err(|e| miette::miette!("Failed to create {}: {}", rust_env.display(), e))?;
            let rust_edition = manifest.rust.as_ref().map(|r| r.edition.as_str()).unwrap_or("2021");
            let cargo_toml_content = templates::cargo_toml(project_name, rust_edition);
            std::fs::write(rust_env.join("Cargo.toml"), cargo_toml_content)
                .map_err(|e| miette::miette!("Failed to write Cargo.toml: {}", e))?;
            let src_dir = rust_env.join("src");
            std::fs::create_dir_all(&src_dir)
                .map_err(|e| miette::miette!("Failed to create src dir: {}", e))?;
            std::fs::write(src_dir.join("main.rs"), "fn main() {}\n")
                .map_err(|e| miette::miette!("Failed to write main.rs: {}", e))?;
            if !quiet {
                terminal::success("Created .polybench/runtime-env/rust/ (Cargo.toml, src/main.rs)");
            }
        }
        Lang::Python => {
            let python_env = runtime_env(project_dir, Lang::Python);
            std::fs::create_dir_all(&python_env)
                .map_err(|e| miette::miette!("Failed to create {}: {}", python_env.display(), e))?;
            let requirements_content = templates::requirements_txt_for_runtime_env(&[]);
            std::fs::write(python_env.join("requirements.txt"), requirements_content)
                .map_err(|e| miette::miette!("Failed to write requirements.txt: {}", e))?;
            if !quiet {
                terminal::success("Created .polybench/runtime-env/python/ (requirements.txt)");
            }
        }
        Lang::C => {
            let c_env = runtime_env(project_dir, Lang::C);
            std::fs::create_dir_all(&c_env)
                .map_err(|e| miette::miette!("Failed to create {}: {}", c_env.display(), e))?;
            std::fs::write(c_env.join("main.c"), "int main(void) {\n    return 0;\n}\n")
                .map_err(|e| miette::miette!("Failed to write main.c: {}", e))?;
            if !quiet {
                terminal::success("Created .polybench/runtime-env/c/ (main.c)");
            }
        }
        Lang::CSharp => {
            let csharp_env = runtime_env(project_dir, Lang::CSharp);
            std::fs::create_dir_all(&csharp_env)
                .map_err(|e| miette::miette!("Failed to create {}: {}", csharp_env.display(), e))?;
            let target_framework =
                manifest.csharp.as_ref().map(|c| c.target_framework.as_str()).unwrap_or("net8.0");
            std::fs::write(
                csharp_env.join("polybench.csproj"),
                templates::csharp_csproj(target_framework),
            )
            .map_err(|e| miette::miette!("Failed to write polybench.csproj: {}", e))?;
            std::fs::write(
                csharp_env.join("Program.cs"),
                "public static class Program { public static void Main() {} }\n",
            )
            .map_err(|e| miette::miette!("Failed to write Program.cs: {}", e))?;
            std::fs::write(csharp_env.join("NuGet.config"), templates::csharp_nuget_config())
                .map_err(|e| miette::miette!("Failed to write NuGet.config: {}", e))?;
            if !quiet {
                terminal::success(
                    "Created .polybench/runtime-env/csharp/ (polybench.csproj, Program.cs, NuGet.config)",
                );
            }
        }
        Lang::Zig => {
            let zig_env = runtime_env(project_dir, Lang::Zig);
            std::fs::create_dir_all(&zig_env)
                .map_err(|e| miette::miette!("Failed to create {}: {}", zig_env.display(), e))?;
            std::fs::write(zig_env.join("build.zig"), templates::build_zig())
                .map_err(|e| miette::miette!("Failed to write build.zig: {}", e))?;
            std::fs::write(zig_env.join("build.zig.zon"), templates::build_zig_zon())
                .map_err(|e| miette::miette!("Failed to write build.zig.zon: {}", e))?;
            let src_dir = zig_env.join("src");
            std::fs::create_dir_all(&src_dir)
                .map_err(|e| miette::miette!("Failed to create src directory: {}", e))?;
            std::fs::write(src_dir.join("main.zig"), templates::main_zig())
                .map_err(|e| miette::miette!("Failed to write main.zig: {}", e))?;
            if !quiet {
                terminal::success(
                    "Created .polybench/runtime-env/zig/ (build.zig, build.zig.zon, src/main.zig)",
                );
            }
        }
    }

    Ok(())
}

/// Create a new benchmark file in the project
pub fn new_benchmark(name: &str) -> Result<PathBuf> {
    // Find project root
    let current_dir = std::env::current_dir()
        .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

    let project_root = crate::find_project_root(&current_dir).ok_or_else(|| {
        miette::miette!("Not in a poly-bench project. Run 'poly-bench init' first.")
    })?;

    // Create benchmarks directory if it doesn't exist
    let benchmarks_dir = project_root.join(BENCHMARKS_DIR);
    std::fs::create_dir_all(&benchmarks_dir)
        .map_err(|e| miette::miette!("Failed to create benchmarks directory: {}", e))?;

    // Generate the benchmark file
    let bench_filename = format!("{}.bench", name);
    let bench_path = benchmarks_dir.join(&bench_filename);

    if bench_path.exists() {
        return Err(miette::miette!(
            "Benchmark file already exists: {}/{}",
            BENCHMARKS_DIR,
            bench_filename
        ));
    }

    // Create a blank .bench file with just an empty suite
    let content = format!("suite {} {{\n}}\n", name);
    std::fs::write(&bench_path, content)
        .map_err(|e| miette::miette!("Failed to write {}: {}", bench_filename, e))?;

    terminal::success(&format!("Created {}/{}", BENCHMARKS_DIR, bench_filename));
    println!();
    println!("Edit the file to add your benchmarks, then run:");
    println!("  poly-bench run {}/{}", BENCHMARKS_DIR, bench_filename);

    Ok(bench_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_init_project() {
        let temp = TempDir::new().unwrap();
        let project_path = temp.path().join("test-project");

        let options = InitOptions {
            name: project_path.to_string_lossy().to_string(),
            languages: vec!["go".to_string(), "ts".to_string()],
            no_example: false,
            quiet: false,
            defer_final_message: false,
        };

        let result = init_project(&options);
        assert!(result.is_ok());

        // Check files exist (runtime-env layout: deps under .polybench/runtime-env/)
        assert!(project_path.join(MANIFEST_FILENAME).exists());
        assert!(project_path.join(BENCHMARKS_DIR).exists());
        assert!(project_path.join(BENCHMARKS_DIR).join("example.bench").exists());
        assert!(crate::runtime_env(&project_path, Lang::Go).join("go.mod").exists());
        // Note: bench_standalone.go is NOT created on init - only when running benchmarks
        assert!(crate::runtime_env(&project_path, Lang::TypeScript).join("package.json").exists());
        assert!(crate::runtime_env(&project_path, Lang::TypeScript).join("tsconfig.json").exists());
        assert!(project_path.join(".gitignore").exists());
        assert!(project_path.join("README.md").exists());
    }

    #[test]
    fn test_init_project_go_only() {
        let temp = TempDir::new().unwrap();
        let project_path = temp.path().join("go-only");

        let options = InitOptions {
            name: project_path.to_string_lossy().to_string(),
            languages: vec!["go".to_string()],
            no_example: false,
            quiet: false,
            defer_final_message: false,
        };

        let result = init_project(&options);
        assert!(result.is_ok());

        assert!(crate::runtime_env(&project_path, Lang::Go).join("go.mod").exists());
        assert!(!crate::runtime_env(&project_path, Lang::TypeScript).join("package.json").exists());
    }

    #[test]
    fn test_init_prevents_duplicate() {
        let temp = TempDir::new().unwrap();
        let project_path = temp.path().join("duplicate");

        let options = InitOptions {
            name: project_path.to_string_lossy().to_string(),
            languages: vec!["go".to_string()],
            no_example: true,
            quiet: false,
            defer_final_message: false,
        };

        // First init should succeed
        assert!(init_project(&options).is_ok());

        // Second init should fail (manifest already exists)
        assert!(init_project(&options).is_err());
    }
}
