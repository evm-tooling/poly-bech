//! poly-bench CLI entrypoint

use clap::{Parser, Subcommand};
use miette::Result;
use std::path::PathBuf;

use poly_bench_dsl as dsl;
use poly_bench_stdlib as stdlib;
mod ir;
mod runtime;
mod executor;
mod reporter;
mod project;

#[derive(Parser)]
#[command(name = "poly-bench")]
#[command(author = "Evan McGrane")]
#[command(version = "0.1.0")]
#[command(about = "A high-performance multi-language benchmarking framework", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse and validate a benchmark DSL file
    Check {
        /// Path to the .bench file
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Show the parsed AST
        #[arg(long, default_value = "false")]
        show_ast: bool,
    },

    /// Run benchmarks from a DSL file or project
    Run {
        /// Path to the .bench file (optional if in a poly-bench project)
        #[arg(value_name = "FILE")]
        file: Option<PathBuf>,

        /// Run only benchmarks for a specific language (go, ts)
        #[arg(long, value_name = "LANG")]
        lang: Option<String>,

        /// Override the number of iterations
        #[arg(long, value_name = "N")]
        iterations: Option<u64>,

        /// Output format for reports (console, markdown, json)
        #[arg(long, default_value = "console")]
        report: String,

        /// Output directory for reports
        #[arg(long, short, value_name = "DIR")]
        output: Option<PathBuf>,

        /// Go module root directory (where go.mod is located).
        /// If not specified, searches parent directories of the bench file.
        #[arg(long, value_name = "DIR")]
        go_project: Option<PathBuf>,

        /// Node.js project root directory (where package.json/node_modules is located).
        /// If not specified, searches parent directories of the bench file.
        #[arg(long, value_name = "DIR")]
        ts_project: Option<PathBuf>,
    },

    /// Generate code from a DSL file without running
    Codegen {
        /// Path to the .bench file
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Language to generate code for (go, ts)
        #[arg(long, value_name = "LANG")]
        lang: String,

        /// Output directory for generated code
        #[arg(long, short, value_name = "DIR")]
        output: PathBuf,
    },

    /// Initialize a new poly-bench project
    Init {
        /// Project name or "." for current directory
        #[arg(value_name = "NAME")]
        name: String,

        /// Languages to include (comma-separated: go,ts)
        #[arg(long, short, value_delimiter = ',', default_value = "go,ts")]
        languages: Vec<String>,

        /// Skip generating example benchmark
        #[arg(long)]
        no_example: bool,
    },

    /// Create a new benchmark file
    New {
        /// Benchmark/suite name
        #[arg(value_name = "NAME")]
        name: String,
    },

    /// Add a dependency
    Add {
        /// Go package (e.g., "github.com/ethereum/go-ethereum@v1.13.0")
        #[arg(long)]
        go: Option<String>,

        /// NPM package (e.g., "viem@^2.0.0")
        #[arg(long)]
        ts: Option<String>,
    },

    /// Install dependencies from polybench.toml
    Install,

    /// Build/regenerate the .polybench runtime environment
    Build {
        /// Force regenerate all files even if they exist
        #[arg(long, short)]
        force: bool,

        /// Skip running npm install / go get
        #[arg(long)]
        skip_install: bool,
    },

    /// Format .bench files
    Fmt {
        /// Paths to .bench files (default: all in benchmarks/)
        #[arg(value_name = "FILES")]
        files: Vec<PathBuf>,

        /// Write formatted output to files instead of stdout
        #[arg(long, short)]
        write: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { file, show_ast } => {
            cmd_check(&file, show_ast).await?;
        }
        Commands::Run {
            file,
            lang,
            iterations,
            report,
            output,
            go_project,
            ts_project,
        } => {
            cmd_run(file, lang, iterations, &report, output, go_project, ts_project).await?;
        }
        Commands::Codegen { file, lang, output } => {
            cmd_codegen(&file, &lang, &output).await?;
        }
        Commands::Init {
            name,
            languages,
            no_example,
        } => {
            cmd_init(&name, languages, no_example)?;
        }
        Commands::New { name } => {
            cmd_new(&name)?;
        }
        Commands::Add { go, ts } => {
            cmd_add(go, ts)?;
        }
        Commands::Install => {
            cmd_install()?;
        }
        Commands::Build { force, skip_install } => {
            cmd_build(force, skip_install)?;
        }
        Commands::Fmt { files, write } => {
            cmd_fmt(files, write).await?;
        }
    }

    Ok(())
}

async fn cmd_check(file: &PathBuf, show_ast: bool) -> Result<()> {
    use colored::Colorize;

    let source = std::fs::read_to_string(file)
        .map_err(|e| miette::miette!("Failed to read file {}: {}", file.display(), e))?;

    let filename = file.file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    match dsl::parse(&source, filename) {
        Ok(ast) => {
            println!("{} {}", "✓".green().bold(), file.display());
            println!("  {} suite(s) parsed successfully", ast.suites.len());
            
            for suite in &ast.suites {
                println!("  {} suite '{}': {} benchmarks, {} fixtures",
                    "→".blue(),
                    suite.name,
                    suite.benchmarks.len(),
                    suite.fixtures.len()
                );
            }

            if show_ast {
                println!("\n{}", "AST:".bold());
                println!("{:#?}", ast);
            }

            Ok(())
        }
        Err(e) => {
            Err(e)
        }
    }
}

async fn cmd_run(
    file: Option<PathBuf>,
    lang: Option<String>,
    iterations: Option<u64>,
    report_format: &str,
    output: Option<PathBuf>,
    go_project: Option<PathBuf>,
    ts_project: Option<PathBuf>,
) -> Result<()> {
    use colored::Colorize;

    // Get benchmark files to run
    let files = match file {
        Some(f) => vec![f],
        None => {
            // Try to find project root and discover benchmark files
            let current_dir = std::env::current_dir()
                .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;
            
            let project_root = project::find_project_root(&current_dir)
                .ok_or_else(|| miette::miette!(
                    "No .bench file specified and not in a poly-bench project.\n\
                    Either specify a file: poly-bench run <file.bench>\n\
                    Or initialize a project: poly-bench init <name>"
                ))?;
            
            let bench_files = project::find_bench_files(&project_root)?;
            
            if bench_files.is_empty() {
                return Err(miette::miette!(
                    "No .bench files found in {}/",
                    project::BENCHMARKS_DIR
                ));
            }
            
            bench_files
        }
    };

    // Filter languages if specified
    let langs: Vec<dsl::Lang> = match lang.as_deref() {
        Some("go") => vec![dsl::Lang::Go],
        Some("ts") | Some("typescript") => vec![dsl::Lang::TypeScript],
        Some(l) => return Err(miette::miette!("Unknown language: {}", l)),
        None => vec![dsl::Lang::Go, dsl::Lang::TypeScript],
    };

    // Run each benchmark file
    let mut all_results = Vec::new();
    
    for bench_file in &files {
        // Parse the DSL file
        let source = std::fs::read_to_string(bench_file)
            .map_err(|e| miette::miette!("Failed to read file {}: {}", bench_file.display(), e))?;

        let filename = bench_file.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        let ast = dsl::parse(&source, filename)?;

        // Lower to IR
        let ir = ir::lower(&ast, bench_file.parent())?;

        // Resolve project roots for module resolution
        let project_roots = resolve_project_roots(go_project.clone(), ts_project.clone(), bench_file)?;

        println!("{} Running benchmarks from {}", "▶".green().bold(), bench_file.display());

        // Execute benchmarks
        let results = executor::run(&ir, &langs, iterations, &project_roots).await?;
        all_results.push(results);
    }

    // Merge results if multiple files
    let results = if all_results.len() == 1 {
        all_results.remove(0)
    } else {
        // Merge multiple results into one
        merge_results(all_results)
    };

    // Generate reports
    match report_format {
        "console" => {
            reporter::console::report(&results)?;
        }
        "markdown" => {
            let md = reporter::markdown::report(&results)?;
            if let Some(ref out_dir) = output {
                std::fs::create_dir_all(out_dir)
                    .map_err(|e| miette::miette!("Failed to create output directory: {}", e))?;
                let out_path = out_dir.join("benchmark-report.md");
                std::fs::write(&out_path, &md)
                    .map_err(|e| miette::miette!("Failed to write report: {}", e))?;
                println!("Report written to {}", out_path.display());
            } else {
                println!("{}", md);
            }
        }
        "json" => {
            let json = reporter::json::report(&results)?;
            if let Some(ref out_dir) = output {
                std::fs::create_dir_all(out_dir)
                    .map_err(|e| miette::miette!("Failed to create output directory: {}", e))?;
                let out_path = out_dir.join("benchmark-results.json");
                std::fs::write(&out_path, &json)
                    .map_err(|e| miette::miette!("Failed to write report: {}", e))?;
                println!("Report written to {}", out_path.display());
            } else {
                println!("{}", json);
            }
        }
        _ => {
            return Err(miette::miette!("Unknown report format: {}", report_format));
        }
    }

    Ok(())
}

use executor::{ProjectRoots, BenchmarkResults};

/// Merge multiple benchmark results into one
fn merge_results(mut results: Vec<BenchmarkResults>) -> BenchmarkResults {
    if results.is_empty() {
        return BenchmarkResults::new(vec![]);
    }
    
    let mut all_suites = Vec::new();
    for result in results.drain(..) {
        all_suites.extend(result.suites);
    }
    
    BenchmarkResults::new(all_suites)
}

/// Resolve project roots for module resolution
fn resolve_project_roots(
    go_explicit: Option<PathBuf>,
    ts_explicit: Option<PathBuf>,
    bench_file: &PathBuf,
) -> Result<ProjectRoots> {
    let mut roots = ProjectRoots::default();
    
    // Handle explicit Go project root
    if let Some(ref dir) = go_explicit {
        let canonical = dir.canonicalize()
            .map_err(|e| miette::miette!("Cannot access Go project root {}: {}", dir.display(), e))?;
        
        if !canonical.join("go.mod").exists() {
            return Err(miette::miette!("No go.mod found in {}", canonical.display()));
        }
        roots.go_root = Some(canonical);
    }
    
    // Handle explicit TypeScript project root
    if let Some(ref dir) = ts_explicit {
        let canonical = dir.canonicalize()
            .map_err(|e| miette::miette!("Cannot access TS project root {}: {}", dir.display(), e))?;
        
        if !canonical.join("package.json").exists() && !canonical.join("node_modules").exists() {
            return Err(miette::miette!("No package.json or node_modules found in {}", canonical.display()));
        }
        roots.node_root = Some(canonical);
    }

    // Search parent directories of the bench file for any missing roots
    let start_dir = bench_file.parent().unwrap_or(std::path::Path::new("."));
    let mut current = start_dir.canonicalize().ok();

    while let Some(dir) = current {
        // Inside a poly-bench project: prefer .polybench/runtime-env/go and .../ts
        if roots.go_root.is_none() && dir.join(project::MANIFEST_FILENAME).exists() {
            let go_env = project::runtime_env_go(&dir);
            if go_env.join("go.mod").exists() {
                roots.go_root = Some(go_env);
            }
        }
        if roots.node_root.is_none() && dir.join(project::MANIFEST_FILENAME).exists() {
            let ts_env = project::runtime_env_ts(&dir);
            if ts_env.join("package.json").exists() || ts_env.join("node_modules").exists() {
                roots.node_root = Some(ts_env);
            }
        }
        // Fallback: classic layout (go.mod / package.json at project root)
        if roots.go_root.is_none() && dir.join("go.mod").exists() {
            roots.go_root = Some(dir.clone());
        }
        if roots.node_root.is_none()
            && (dir.join("package.json").exists() || dir.join("node_modules").exists())
        {
            roots.node_root = Some(dir.clone());
        }
        if roots.go_root.is_some() && roots.node_root.is_some() {
            break;
        }
        current = dir.parent().map(|p| p.to_path_buf());
    }

    Ok(roots)
}

async fn cmd_codegen(file: &PathBuf, lang: &str, output: &PathBuf) -> Result<()> {
    use colored::Colorize;

    // Parse the DSL file
    let source = std::fs::read_to_string(file)
        .map_err(|e| miette::miette!("Failed to read file {}: {}", file.display(), e))?;

    let filename = file.file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    let ast = dsl::parse(&source, filename)?;

    // Lower to IR
    let ir = ir::lower(&ast, file.parent())?;

    // Create output directory
    std::fs::create_dir_all(output)
        .map_err(|e| miette::miette!("Failed to create output directory: {}", e))?;

    match lang {
        "go" => {
            let code = runtime::go::codegen::generate(&ir)?;
            let out_path = output.join("benchmark_plugin.go");
            std::fs::write(&out_path, &code)
                .map_err(|e| miette::miette!("Failed to write generated code: {}", e))?;
            println!("{} Generated Go plugin: {}", "✓".green().bold(), out_path.display());
        }
        "ts" | "typescript" => {
            let code = runtime::js::codegen::generate(&ir)?;
            let out_path = output.join("benchmark.ts");
            std::fs::write(&out_path, &code)
                .map_err(|e| miette::miette!("Failed to write generated code: {}", e))?;
            println!("{} Generated TypeScript: {}", "✓".green().bold(), out_path.display());
        }
        _ => {
            return Err(miette::miette!("Unknown language: {}", lang));
        }
    }

    Ok(())
}

fn cmd_init(name: &str, languages: Vec<String>, no_example: bool) -> Result<()> {
    let options = project::init::InitOptions {
        name: name.to_string(),
        languages,
        no_example,
    };
    project::init::init_project(&options)?;
    Ok(())
}

fn cmd_new(name: &str) -> Result<()> {
    project::init::new_benchmark(name)?;
    Ok(())
}

fn cmd_add(go: Option<String>, ts: Option<String>) -> Result<()> {
    if go.is_none() && ts.is_none() {
        return Err(miette::miette!(
            "No dependency specified. Use --go or --ts to add a dependency."
        ));
    }

    if let Some(ref spec) = go {
        project::deps::add_go_dependency(spec)?;
    }

    if let Some(ref spec) = ts {
        project::deps::add_ts_dependency(spec)?;
    }

    Ok(())
}

fn cmd_install() -> Result<()> {
    project::deps::install_all()
}

fn cmd_build(force: bool, skip_install: bool) -> Result<()> {
    let options = project::build::BuildOptions {
        force,
        skip_install,
    };
    project::build::build_project(&options)
}

async fn cmd_fmt(files: Vec<PathBuf>, write: bool) -> Result<()> {
    use colored::Colorize;

    let files = if files.is_empty() {
        let current_dir = std::env::current_dir()
            .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;
        let project_root = project::find_project_root(&current_dir).ok_or_else(|| {
            miette::miette!(
                "No poly-bench project found. Specify files: poly-bench fmt <file.bench> ..."
            )
        })?;
        project::find_bench_files(&project_root)?
    } else {
        files
    };

    if files.is_empty() {
        return Err(miette::miette!("No .bench files to format"));
    }

    for file in &files {
        let source = std::fs::read_to_string(file)
            .map_err(|e| miette::miette!("Failed to read {}: {}", file.display(), e))?;
        let filename = file
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");
        match dsl::parse(&source, filename) {
            Ok(ast) => {
                let formatted = dsl::format_file(&ast);
                if write {
                    std::fs::write(file, &formatted)
                        .map_err(|e| miette::miette!("Failed to write {}: {}", file.display(), e))?;
                    println!("{} {}", "✓".green().bold(), file.display());
                } else {
                    print!("{}", formatted);
                }
            }
            Err(e) => return Err(e),
        }
    }

    Ok(())
}
