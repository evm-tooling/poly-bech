//! Project initialization and benchmark file creation

use crate::project::{
    manifest::{self, Manifest},
    runtime_env_go, runtime_env_ts,
    templates, BENCHMARKS_DIR, MANIFEST_FILENAME,
};
use colored::Colorize;
use miette::Result;
use std::path::PathBuf;

/// Options for initializing a project
pub struct InitOptions {
    /// Project name
    pub name: String,
    /// Languages to enable
    pub languages: Vec<String>,
    /// Skip generating example benchmark
    pub no_example: bool,
}

/// Initialize a new poly-bench project
pub fn init_project(options: &InitOptions) -> Result<PathBuf> {
    let project_dir = if options.name == "." {
        std::env::current_dir()
            .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?
    } else {
        PathBuf::from(&options.name)
    };

    // Get the actual project name from the directory
    let project_name = if options.name == "." {
        project_dir
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("my-project")
            .to_string()
    } else {
        options.name.clone()
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
        println!(
            "{} Created directory {}",
            "✓".green().bold(),
            project_dir.display()
        );
    }

    // Normalize languages
    let languages: Vec<String> = options
        .languages
        .iter()
        .map(|l| match l.as_str() {
            "typescript" => "ts".to_string(),
            other => other.to_string(),
        })
        .collect();

    let has_go = languages.iter().any(|l| l == "go");
    let has_ts = languages.iter().any(|l| l == "ts");

    // Create manifest
    let manifest = Manifest::new(&project_name, &languages);
    let manifest_path = project_dir.join(MANIFEST_FILENAME);
    manifest::save(&manifest_path, &manifest)?;
    println!(
        "{} Created {}",
        "✓".green().bold(),
        MANIFEST_FILENAME
    );

    // Create benchmarks directory
    let benchmarks_dir = project_dir.join(BENCHMARKS_DIR);
    std::fs::create_dir_all(&benchmarks_dir)
        .map_err(|e| miette::miette!("Failed to create benchmarks directory: {}", e))?;
    println!(
        "{} Created {}/",
        "✓".green().bold(),
        BENCHMARKS_DIR
    );

    // Create example benchmark
    if !options.no_example {
        let example_path = benchmarks_dir.join("example.bench");
        let example_content = templates::example_bench(has_go, has_ts);
        std::fs::write(&example_path, example_content)
            .map_err(|e| miette::miette!("Failed to write example.bench: {}", e))?;
        println!(
            "{} Created {}/example.bench",
            "✓".green().bold(),
            BENCHMARKS_DIR
        );
    }

    // Create runtime-env dirs and language-specific env files (keeps root uncluttered)
    if has_go {
        let go_env = runtime_env_go(&project_dir);
        std::fs::create_dir_all(&go_env)
            .map_err(|e| miette::miette!("Failed to create {}: {}", go_env.display(), e))?;
        let go_version = manifest.go.as_ref().and_then(|g| g.version.as_deref());
        let go_mod_content = templates::go_mod(&project_name, go_version);
        std::fs::write(go_env.join("go.mod"), go_mod_content)
            .map_err(|e| miette::miette!("Failed to write go.mod: {}", e))?;
        // Note: No .go file created yet - bench_standalone.go is generated when running benchmarks.
        // This keeps deps in go.mod since `go mod tidy` won't run until bench code exists.
        println!("{} Created .polybench/runtime-env/go/ (go.mod)", "✓".green().bold());
    }

    if has_ts {
        let ts_env = runtime_env_ts(&project_dir);
        std::fs::create_dir_all(&ts_env)
            .map_err(|e| miette::miette!("Failed to create {}: {}", ts_env.display(), e))?;
        let package_json_content = templates::package_json_pretty(&project_name);
        std::fs::write(ts_env.join("package.json"), package_json_content)
            .map_err(|e| miette::miette!("Failed to write package.json: {}", e))?;
        let tsconfig_content = templates::tsconfig_json();
        std::fs::write(ts_env.join("tsconfig.json"), tsconfig_content)
            .map_err(|e| miette::miette!("Failed to write tsconfig.json: {}", e))?;
        println!("{} Created .polybench/runtime-env/ts/ (package.json, tsconfig.json)", "✓".green().bold());
        
        // Run npm install to install dev dependencies (@types/node, typescript)
        println!("{} Installing TypeScript dependencies...", "→".blue().bold());
        let npm_result = std::process::Command::new("npm")
            .arg("install")
            .current_dir(&ts_env)
            .output();
        
        match npm_result {
            Ok(output) if output.status.success() => {
                println!("{} Installed TypeScript dependencies", "✓".green().bold());
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!(
                    "{} npm install failed: {}",
                    "⚠".yellow().bold(),
                    stderr.trim()
                );
                eprintln!("  Run 'npm install' manually in .polybench/runtime-env/ts/");
            }
            Err(e) => {
                eprintln!(
                    "{} Could not run npm: {}",
                    "⚠".yellow().bold(),
                    e
                );
                eprintln!("  Run 'npm install' manually in .polybench/runtime-env/ts/");
            }
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
    println!("{} Created .gitignore", "✓".green().bold());

    // Create README.md
    let readme_path = project_dir.join("README.md");
    if !readme_path.exists() {
        let readme_content = templates::readme(&project_name, has_go, has_ts);
        std::fs::write(&readme_path, readme_content)
            .map_err(|e| miette::miette!("Failed to write README.md: {}", e))?;
        println!("{} Created README.md", "✓".green().bold());
    }

    println!();
    println!(
        "{} Project '{}' initialized successfully!",
        "✓".green().bold(),
        project_name
    );
    println!();
    println!("Next steps:");
    if options.name != "." {
        println!("  cd {}", project_name);
    }
    println!("  poly-bench install    # Install dependencies");
    println!("  poly-bench run        # Run benchmarks");
    println!();

    Ok(project_dir)
}

/// Create a new benchmark file in the project
pub fn new_benchmark(name: &str) -> Result<PathBuf> {
    // Find project root
    let current_dir = std::env::current_dir()
        .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

    let project_root = crate::project::find_project_root(&current_dir)
        .ok_or_else(|| miette::miette!(
            "Not in a poly-bench project. Run 'poly-bench init' first."
        ))?;

    // Load manifest to get enabled languages
    let manifest = crate::project::load_manifest(&project_root)?;
    let has_go = manifest.has_go();
    let has_ts = manifest.has_ts();

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

    let content = templates::new_bench(name, has_go, has_ts);
    std::fs::write(&bench_path, content)
        .map_err(|e| miette::miette!("Failed to write {}: {}", bench_filename, e))?;

    println!(
        "{} Created {}/{}",
        "✓".green().bold(),
        BENCHMARKS_DIR,
        bench_filename
    );
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
        };

        let result = init_project(&options);
        assert!(result.is_ok());

        // Check files exist (runtime-env layout: deps under .polybench/runtime-env/)
        assert!(project_path.join(MANIFEST_FILENAME).exists());
        assert!(project_path.join(BENCHMARKS_DIR).exists());
        assert!(project_path.join(BENCHMARKS_DIR).join("example.bench").exists());
        assert!(crate::project::runtime_env_go(&project_path).join("go.mod").exists());
        // Note: bench_standalone.go is NOT created on init - only when running benchmarks
        assert!(crate::project::runtime_env_ts(&project_path).join("package.json").exists());
        assert!(crate::project::runtime_env_ts(&project_path).join("tsconfig.json").exists());
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
        };

        let result = init_project(&options);
        assert!(result.is_ok());

        assert!(crate::project::runtime_env_go(&project_path).join("go.mod").exists());
        assert!(!crate::project::runtime_env_ts(&project_path).join("package.json").exists());
    }

    #[test]
    fn test_init_prevents_duplicate() {
        let temp = TempDir::new().unwrap();
        let project_path = temp.path().join("duplicate");

        let options = InitOptions {
            name: project_path.to_string_lossy().to_string(),
            languages: vec!["go".to_string()],
            no_example: true,
        };

        // First init should succeed
        assert!(init_project(&options).is_ok());

        // Second init should fail (manifest already exists)
        assert!(init_project(&options).is_err());
    }
}
