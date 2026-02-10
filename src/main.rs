//! poly-bench CLI entrypoint

use clap::{Parser, Subcommand};
use miette::Result;
use std::path::PathBuf;

mod dsl;
mod ir;
mod runtime;
mod executor;
mod reporter;

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

    /// Run benchmarks from a DSL file
    Run {
        /// Path to the .bench file
        #[arg(value_name = "FILE")]
        file: PathBuf,

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
            cmd_run(&file, lang, iterations, &report, output, go_project, ts_project).await?;
        }
        Commands::Codegen { file, lang, output } => {
            cmd_codegen(&file, &lang, &output).await?;
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
    file: &PathBuf,
    lang: Option<String>,
    iterations: Option<u64>,
    report_format: &str,
    output: Option<PathBuf>,
    go_project: Option<PathBuf>,
    ts_project: Option<PathBuf>,
) -> Result<()> {
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

    // Filter languages if specified
    let langs: Vec<dsl::Lang> = match lang.as_deref() {
        Some("go") => vec![dsl::Lang::Go],
        Some("ts") | Some("typescript") => vec![dsl::Lang::TypeScript],
        Some(l) => return Err(miette::miette!("Unknown language: {}", l)),
        None => vec![dsl::Lang::Go, dsl::Lang::TypeScript],
    };

    // Resolve project roots for module resolution
    let project_roots = resolve_project_roots(go_project, ts_project, file)?;

    println!("{} Running benchmarks from {}", "▶".green().bold(), file.display());

    // Execute benchmarks
    let results = executor::run(&ir, &langs, iterations, &project_roots).await?;

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

use executor::ProjectRoots;

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
        // Check for go.mod (only set if not already found)
        if roots.go_root.is_none() && dir.join("go.mod").exists() {
            roots.go_root = Some(dir.clone());
        }
        
        // Check for package.json or node_modules (only set if not already found)
        if roots.node_root.is_none() && 
           (dir.join("package.json").exists() || dir.join("node_modules").exists()) {
            roots.node_root = Some(dir.clone());
        }
        
        // Stop if we've found both
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
