//! poly-bench CLI entrypoint

mod init_t3;
mod version_check;
mod welcome;

use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use miette::Result;
use std::{io::Read, path::PathBuf, time::Duration};

use poly_bench_dsl as dsl;
use poly_bench_executor as executor;
use poly_bench_ir as ir;
use poly_bench_project as project;
use poly_bench_reporter as reporter;
use poly_bench_runtime as runtime;

/// Current binary version (set at compile time).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "poly-bench")]
#[command(author = "Evan McGrane")]
#[command(disable_version_flag(true))]
#[command(about = "Build, run, and compare benchmarks across Go and TypeScript with a custom DSL.", long_about = None)]
struct Cli {
    /// Print version and exit
    #[arg(long, short = 'V', alias("v"), short_alias('v'), global = true)]
    version: bool,

    /// Colorize output [possible values: auto, always, never]
    #[arg(long, global = true, value_name = "WHEN", help_heading = "Display options")]
    color: Option<String>,

    /// Reduce log output
    #[arg(short, long, global = true, help_heading = "Display options")]
    quiet: bool,

    #[command(subcommand)]
    command: Option<Commands>,
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

    /// Compile-check benchmarks without running them
    Compile {
        /// Path to a .bench file (optional; if omitted, compiles all in benchmarks/)
        #[arg(value_name = "FILE")]
        file: Option<PathBuf>,

        /// Check only benchmarks for a specific language (go, ts, rust)
        #[arg(long, value_name = "LANG")]
        lang: Option<String>,

        /// Disable compile result caching (always recompile)
        #[arg(long)]
        no_cache: bool,

        /// Clear the compile cache before checking
        #[arg(long)]
        clear_cache: bool,
    },

    /// Manage the compile cache
    Cache {
        #[command(subcommand)]
        action: CacheAction,
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
        /// Project name or "." for current directory (omit for interactive prompt)
        #[arg(value_name = "NAME")]
        name: Option<String>,

        /// Languages to include (comma-separated: go,ts); used only when NAME is provided
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

        /// Rust crate (e.g., "sha2@0.10" or "tiny-keccak@2.0")
        #[arg(long)]
        rs: Option<String>,

        /// Rust crate features (comma-separated, e.g., "keccak,sha3")
        #[arg(long, value_delimiter = ',')]
        features: Option<Vec<String>>,
    },

    /// Remove a dependency
    Remove {
        /// Go package to remove (e.g., "github.com/ethereum/go-ethereum")
        #[arg(long)]
        go: Option<String>,

        /// NPM package to remove (e.g., "viem")
        #[arg(long)]
        ts: Option<String>,

        /// Rust crate to remove (e.g., "sha2")
        #[arg(long)]
        rs: Option<String>,
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

    /// Upgrade to the latest poly-bench binary
    Upgrade,

    /// Start the language server (for editors, v2)
    Lsp {
        /// Accepted for editor compatibility; stdio is always used
        #[arg(long, hide = true)]
        stdio: bool,
    },
}

#[derive(Subcommand)]
enum CacheAction {
    /// Show cache statistics
    Stats,
    /// Clear all cached compile results
    Clear,
    /// Clean the entire .polybench workspace
    Clean,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.version {
        println!("poly-bench {}", VERSION);
        version_check::warn_if_outdated(VERSION);
        return Ok(());
    }

    let command = match cli.command {
        None => {
            welcome::show_welcome_and_maybe_mark_seen();
            version_check::warn_if_outdated(VERSION);
            return Ok(());
        }
        Some(c) => c,
    };

    // LSP mode: no welcome or other stdout; use stdio for LSP protocol
    if let Commands::Lsp { .. } = &command {
        return cmd_lsp().await;
    }

    // First run: show welcome once, then proceed with the command
    if welcome::is_first_run() {
        welcome::show_welcome_and_maybe_mark_seen();
    }

    match command {
        Commands::Check { file, show_ast } => {
            cmd_check(&file, show_ast).await?;
        }
        Commands::Compile { file, lang, no_cache, clear_cache } => {
            cmd_compile(file, lang, no_cache, clear_cache).await?;
        }
        Commands::Cache { action } => {
            cmd_cache(action).await?;
        }
        Commands::Run { file, lang, iterations, report, output, go_project, ts_project } => {
            cmd_run(file, lang, iterations, &report, output, go_project, ts_project).await?;
        }
        Commands::Codegen { file, lang, output } => {
            cmd_codegen(&file, &lang, &output).await?;
        }
        Commands::Init { name, languages, no_example } => {
            cmd_init(name.as_deref(), languages, no_example)?;
        }
        Commands::New { name } => {
            cmd_new(&name)?;
        }
        Commands::Add { go, ts, rs, features } => {
            cmd_add(go, ts, rs, features)?;
        }
        Commands::Remove { go, ts, rs } => {
            cmd_remove(go, ts, rs)?;
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
        Commands::Upgrade => {
            cmd_upgrade()?;
        }
        Commands::Lsp { .. } => {
            // Handled above; unreachable here
            unreachable!()
        }
    }

    // Check for updates after command completes (non-blocking, best-effort)
    version_check::warn_if_outdated(VERSION);

    Ok(())
}

async fn cmd_lsp() -> Result<()> {
    poly_bench_lsp_v2::run_server().await;
    Ok(())
}

async fn cmd_check(file: &PathBuf, show_ast: bool) -> Result<()> {
    use colored::Colorize;

    let source = std::fs::read_to_string(file)
        .map_err(|e| miette::miette!("Failed to read file {}: {}", file.display(), e))?;

    let filename = file.file_name().and_then(|s| s.to_str()).unwrap_or("unknown");

    match dsl::parse(&source, filename) {
        Ok(ast) => {
            println!("{} {}", "✓".green().bold(), file.display());
            println!("  {} suite(s) parsed successfully", ast.suites.len());

            for suite in &ast.suites {
                println!(
                    "  {} suite '{}': {} benchmarks, {} fixtures",
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
        Err(e) => Err(e),
    }
}

async fn cmd_compile(
    file: Option<PathBuf>,
    lang: Option<String>,
    no_cache: bool,
    clear_cache: bool,
) -> Result<()> {
    use colored::Colorize;

    // Get benchmark files to compile
    let (files, run_parallel, project_root) = match file {
        Some(f) => {
            let root = project::find_project_root(f.parent().unwrap_or(&f))
                .unwrap_or_else(|| f.parent().unwrap_or(&f).to_path_buf());
            (vec![f], false, root)
        }
        None => {
            let current_dir = std::env::current_dir()
                .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

            let project_root = project::find_project_root(&current_dir).ok_or_else(|| {
                miette::miette!(
                    "No .bench file specified and not in a poly-bench project.\n\
                    Either specify a file: poly-bench compile <file.bench>\n\
                    Or run from a poly-bench project directory"
                )
            })?;

            let bench_files = project::find_bench_files(&project_root)?;

            if bench_files.is_empty() {
                return Err(miette::miette!(
                    "No .bench files found in {}/",
                    project::BENCHMARKS_DIR
                ));
            }

            (bench_files, true, project_root)
        }
    };

    // Initialize compile cache
    let cache_dir = project_root.join(".polybench").join("cache");
    let cache = executor::CompileCache::new(&cache_dir, !no_cache);

    // Clear cache if requested
    if clear_cache {
        cache.clear().await;
        println!("{} Cleared compile cache", "✓".green());
    }

    // Filter languages if specified
    let langs: Vec<dsl::Lang> = match lang.as_deref() {
        Some("go") => vec![dsl::Lang::Go],
        Some("ts") | Some("typescript") => vec![dsl::Lang::TypeScript],
        Some("rust") | Some("rs") => vec![dsl::Lang::Rust],
        Some(l) => return Err(miette::miette!("Unknown language: {}", l)),
        None => vec![dsl::Lang::Go, dsl::Lang::TypeScript, dsl::Lang::Rust],
    };

    if run_parallel && files.len() > 1 {
        compile_files_parallel_cached(&files, &langs, &cache).await
    } else {
        compile_files_sequential_cached(&files, &langs, &cache).await
    }
}

struct CompileResultCached {
    file: PathBuf,
    bench_count: usize,
    errors: Vec<executor::CompileError>,
    stats: executor::ValidationStats,
}

async fn compile_single_file_cached(
    bench_file: PathBuf,
    langs: Vec<dsl::Lang>,
    cache: &executor::CompileCache,
) -> Result<CompileResultCached> {
    let source = std::fs::read_to_string(&bench_file)
        .map_err(|e| miette::miette!("Failed to read file {}: {}", bench_file.display(), e))?;

    let filename = bench_file.file_name().and_then(|s| s.to_str()).unwrap_or("unknown");

    let ast = dsl::parse(&source, filename)?;
    let ir_result = ir::lower(&ast, bench_file.parent())?;

    let bench_count: usize = ir_result.suites.iter().map(|s| s.benchmarks.len()).sum();
    let project_roots = resolve_project_roots(None, None, &bench_file)?;

    let (compile_errors, stats) =
        executor::validate_benchmarks_with_cache(&ir_result, &langs, &project_roots, cache).await?;

    Ok(CompileResultCached { file: bench_file, bench_count, errors: compile_errors, stats })
}

async fn compile_files_parallel_cached(
    files: &[PathBuf],
    langs: &[dsl::Lang],
    cache: &executor::CompileCache,
) -> Result<()> {
    use colored::Colorize;
    use futures::future::join_all;

    let cache_status = if cache.is_enabled() { " (with caching)" } else { "" };
    println!("{} Compiling {} file(s) in parallel{}...\n", "⚡".cyan(), files.len(), cache_status);

    let spinner = create_compiling_spinner();

    let futures: Vec<_> = files
        .iter()
        .map(|f| compile_single_file_cached(f.clone(), langs.to_vec(), cache))
        .collect();

    let results = join_all(futures).await;
    spinner.finish_and_clear();

    let mut total_errors = 0;
    let mut total_benchmarks = 0;
    let mut total_cache_hits = 0;
    let mut total_cache_misses = 0;
    let mut failed_results = Vec::new();
    let mut success_results = Vec::new();

    for result in results {
        match result {
            Ok(compile_result) => {
                total_benchmarks += compile_result.bench_count;
                total_cache_hits += compile_result.stats.cache_hits;
                total_cache_misses += compile_result.stats.cache_misses;
                if compile_result.errors.is_empty() {
                    success_results.push(compile_result);
                } else {
                    total_errors += compile_result.errors.len();
                    failed_results.push(compile_result);
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    for result in &success_results {
        let cache_info = if result.stats.cache_hits > 0 {
            format!(" ({} cached)", result.stats.cache_hits)
        } else {
            String::new()
        };
        println!(
            "{} {} - {} benchmark(s) compiled successfully{}",
            "✓".green().bold(),
            result.file.display(),
            result.bench_count,
            cache_info.dimmed()
        );
    }

    for result in &failed_results {
        eprintln!(
            "{} {} - {} error(s):",
            "✗".red().bold(),
            result.file.display(),
            result.errors.len()
        );

        for err in &result.errors {
            let header = if err.benchmarks.len() == 1 {
                format!("[{}] {}", err.lang, err.benchmarks[0])
            } else {
                format!(
                    "[{}] {} error (affects {} benchmarks)",
                    err.lang,
                    err.source,
                    err.benchmarks.len()
                )
            };
            eprintln!("  {} {}", "•".red(), header);

            let mut shown_lines = 0;
            for line in err.message.lines() {
                if line.contains(".bench file line") {
                    eprintln!("    {}", line.yellow());
                } else if line.starts_with("error") || line.contains("error TS") {
                    eprintln!("    {}", line.red());
                } else if shown_lines < 8 {
                    eprintln!("    {}", line.dimmed());
                }
                shown_lines += 1;
                if shown_lines >= 12 {
                    eprintln!("    {}", "... (truncated)".dimmed());
                    break;
                }
            }
            eprintln!();
        }
    }

    if total_errors > 0 {
        return Err(miette::miette!(
            "Compilation failed with {} error(s) across {} file(s)",
            total_errors,
            files.len()
        ));
    }

    if files.len() > 1 {
        let cache_summary = if total_cache_hits > 0 {
            format!(" ({} cached, {} compiled)", total_cache_hits, total_cache_misses)
        } else {
            String::new()
        };
        println!(
            "\n{} All {} benchmark(s) across {} file(s) compiled successfully{}",
            "✓".green().bold(),
            total_benchmarks,
            files.len(),
            cache_summary.dimmed()
        );
    }

    Ok(())
}

async fn compile_files_sequential_cached(
    files: &[PathBuf],
    langs: &[dsl::Lang],
    cache: &executor::CompileCache,
) -> Result<()> {
    use colored::Colorize;

    let mut total_errors = 0;
    let mut total_benchmarks = 0;
    let mut total_cache_hits = 0;
    let mut total_cache_misses = 0;

    for bench_file in files {
        let source = std::fs::read_to_string(bench_file)
            .map_err(|e| miette::miette!("Failed to read file {}: {}", bench_file.display(), e))?;

        let filename = bench_file.file_name().and_then(|s| s.to_str()).unwrap_or("unknown");

        let ast = dsl::parse(&source, filename)?;
        let ir_result = ir::lower(&ast, bench_file.parent())?;

        let bench_count: usize = ir_result.suites.iter().map(|s| s.benchmarks.len()).sum();
        total_benchmarks += bench_count;

        let project_roots = resolve_project_roots(None, None, bench_file)?;

        let spinner = create_compiling_spinner();
        let (compile_errors, stats) =
            executor::validate_benchmarks_with_cache(&ir_result, langs, &project_roots, cache)
                .await?;
        spinner.finish_and_clear();

        total_cache_hits += stats.cache_hits;
        total_cache_misses += stats.cache_misses;

        if !compile_errors.is_empty() {
            total_errors += compile_errors.len();
            eprintln!(
                "{} {} - {} error(s):",
                "✗".red().bold(),
                bench_file.display(),
                compile_errors.len()
            );

            for err in &compile_errors {
                let header = if err.benchmarks.len() == 1 {
                    format!("[{}] {}", err.lang, err.benchmarks[0])
                } else {
                    format!(
                        "[{}] {} error (affects {} benchmarks)",
                        err.lang,
                        err.source,
                        err.benchmarks.len()
                    )
                };
                eprintln!("  {} {}", "•".red(), header);

                let mut shown_lines = 0;
                for line in err.message.lines() {
                    if line.contains(".bench file line") {
                        eprintln!("    {}", line.yellow());
                    } else if line.starts_with("error") || line.contains("error TS") {
                        eprintln!("    {}", line.red());
                    } else if shown_lines < 8 {
                        eprintln!("    {}", line.dimmed());
                    }
                    shown_lines += 1;
                    if shown_lines >= 12 {
                        eprintln!("    {}", "... (truncated)".dimmed());
                        break;
                    }
                }
                eprintln!();
            }
        } else {
            let cache_info = if stats.cache_hits > 0 {
                format!(" ({} cached)", stats.cache_hits)
            } else {
                String::new()
            };
            println!(
                "{} {} - {} benchmark(s) compiled successfully{}",
                "✓".green().bold(),
                bench_file.display(),
                bench_count,
                cache_info.dimmed()
            );
        }
    }

    if total_errors > 0 {
        return Err(miette::miette!(
            "Compilation failed with {} error(s) across {} file(s)",
            total_errors,
            files.len()
        ));
    }

    if files.len() > 1 {
        let cache_summary = if total_cache_hits > 0 {
            format!(" ({} cached, {} compiled)", total_cache_hits, total_cache_misses)
        } else {
            String::new()
        };
        println!(
            "\n{} All {} benchmark(s) across {} file(s) compiled successfully{}",
            "✓".green().bold(),
            total_benchmarks,
            files.len(),
            cache_summary.dimmed()
        );
    }

    Ok(())
}

async fn cmd_cache(action: CacheAction) -> Result<()> {
    use colored::Colorize;

    let current_dir = std::env::current_dir()
        .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

    let project_root = project::find_project_root(&current_dir).ok_or_else(|| {
        miette::miette!("Not in a poly-bench project. Run from a project directory.")
    })?;

    match action {
        CacheAction::Stats => {
            let cache_dir = project_root.join(".polybench").join("cache");
            let cache = executor::CompileCache::new(&cache_dir, true);
            let stats = cache.stats().await;

            println!("{}", "Compile Cache Statistics".bold());
            println!("  Location: {}", cache_dir.display());
            println!("  {}", stats);

            // Also show workspace size
            let workspace = executor::CompileWorkspace::new(&project_root)?;
            let size = workspace.size();
            println!("\n{}", "Workspace Size".bold());
            println!("  .polybench/: {}", executor::format_size(size));
        }
        CacheAction::Clear => {
            let cache_dir = project_root.join(".polybench").join("cache");
            let cache = executor::CompileCache::new(&cache_dir, true);
            cache.clear().await;
            println!("{} Cleared compile cache", "✓".green());
        }
        CacheAction::Clean => {
            let workspace = executor::CompileWorkspace::new(&project_root)?;
            workspace.clean()?;
            println!("{} Cleaned .polybench/ workspace", "✓".green());
        }
    }

    Ok(())
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

            let project_root = project::find_project_root(&current_dir).ok_or_else(|| {
                miette::miette!(
                    "No .bench file specified and not in a poly-bench project.\n\
                    Either specify a file: poly-bench run <file.bench>\n\
                    Or initialize a project: poly-bench init <name>"
                )
            })?;

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
        Some("rust") | Some("rs") => vec![dsl::Lang::Rust],
        Some(l) => return Err(miette::miette!("Unknown language: {}", l)),
        None => vec![dsl::Lang::Go, dsl::Lang::TypeScript, dsl::Lang::Rust],
    };

    // Run each benchmark file
    let mut all_results = Vec::new();
    let mut all_chart_directives = Vec::new();

    for bench_file in &files {
        // Parse the DSL file
        let source = std::fs::read_to_string(bench_file)
            .map_err(|e| miette::miette!("Failed to read file {}: {}", bench_file.display(), e))?;

        let filename = bench_file.file_name().and_then(|s| s.to_str()).unwrap_or("unknown");

        let ast = dsl::parse(&source, filename)?;

        // Lower to IR
        let ir = ir::lower(&ast, bench_file.parent())?;

        // Collect chart directives
        all_chart_directives.extend(ir.chart_directives.clone());

        // Resolve project roots for module resolution
        let project_roots =
            resolve_project_roots(go_project.clone(), ts_project.clone(), bench_file)?;

        // Pre-run validation: compile-check all benchmarks before running
        let spinner = create_compiling_spinner();
        let compile_errors = executor::validate_benchmarks(&ir, &langs, &project_roots).await?;
        spinner.finish_and_clear();

        if !compile_errors.is_empty() {
            eprintln!("\n{} Compilation errors in {}:\n", "✗".red().bold(), bench_file.display());

            for err in &compile_errors {
                // Show error header with language and source type
                let header = if err.benchmarks.len() == 1 {
                    format!("[{}] {}", err.lang, err.benchmarks[0])
                } else {
                    format!(
                        "[{}] {} error (affects {} benchmarks: {})",
                        err.lang,
                        err.source,
                        err.benchmarks.len(),
                        if err.benchmarks.len() <= 3 {
                            err.benchmarks.join(", ")
                        } else {
                            format!(
                                "{}, {} more...",
                                err.benchmarks[..2].join(", "),
                                err.benchmarks.len() - 2
                            )
                        }
                    )
                };
                eprintln!("  {} {}", "•".red(), header);

                // Show error message with better formatting
                let mut shown_lines = 0;
                for line in err.message.lines() {
                    // Highlight lines that show .bench file locations
                    if line.contains(".bench file line") {
                        eprintln!("    {}", line.yellow());
                    } else if line.starts_with("error") || line.contains("error TS") {
                        eprintln!("    {}", line.red());
                    } else if shown_lines < 8 {
                        eprintln!("    {}", line.dimmed());
                    }
                    shown_lines += 1;
                    if shown_lines >= 12 {
                        eprintln!("    {}", "... (truncated)".dimmed());
                        break;
                    }
                }
                eprintln!();
            }

            // Count total affected benchmarks for the summary
            let total_affected: usize = compile_errors.iter().map(|e| e.benchmarks.len()).sum();
            return Err(miette::miette!(
                "Found {} unique error(s) affecting {} benchmark(s). Fix the errors above before running benchmarks.",
                compile_errors.len(),
                total_affected
            ));
        }
        println!("  {} All benchmarks compile successfully\n", "✓".green());

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

    // Default output directory for auto-saved results
    let default_output_dir = PathBuf::from("out");

    // Auto-save results to .polybench/results.json
    {
        std::fs::create_dir_all(&default_output_dir)
            .map_err(|e| miette::miette!("Failed to create output directory: {}", e))?;

        let json = reporter::json::report(&results)?;
        let results_path = default_output_dir.join("results.json");
        std::fs::write(&results_path, &json)
            .map_err(|e| miette::miette!("Failed to save results: {}", e))?;

        println!("\n{} Results saved to {}", "💾".cyan(), results_path.display());
    }

    // Execute chart directives if any
    if !all_chart_directives.is_empty() {
        let chart_output_dir = output.clone().unwrap_or_else(|| default_output_dir.clone());

        println!("{} Generating {} chart(s)...", "📊".cyan(), all_chart_directives.len());

        let generated_charts =
            reporter::execute_chart_directives(&all_chart_directives, &results, &chart_output_dir)?;

        for chart in &generated_charts {
            println!("  {} Generated: {}", "✓".green(), chart.path);
        }
    }

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

use executor::{BenchmarkResults, ProjectRoots};

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
        let canonical = dir.canonicalize().map_err(|e| {
            miette::miette!("Cannot access Go project root {}: {}", dir.display(), e)
        })?;

        if !canonical.join("go.mod").exists() {
            return Err(miette::miette!("No go.mod found in {}", canonical.display()));
        }
        roots.go_root = Some(canonical);
    }

    // Handle explicit TypeScript project root
    if let Some(ref dir) = ts_explicit {
        let canonical = dir.canonicalize().map_err(|e| {
            miette::miette!("Cannot access TS project root {}: {}", dir.display(), e)
        })?;

        if !canonical.join("package.json").exists() && !canonical.join("node_modules").exists() {
            return Err(miette::miette!(
                "No package.json or node_modules found in {}",
                canonical.display()
            ));
        }
        roots.node_root = Some(canonical);
    }

    // Search parent directories of the bench file for any missing roots
    let start_dir = bench_file.parent().unwrap_or(std::path::Path::new("."));
    let mut current = start_dir.canonicalize().ok();

    while let Some(dir) = current {
        // Inside a poly-bench project: prefer .polybench/runtime-env/go, .../ts, .../rust
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
        if roots.rust_root.is_none() && dir.join(project::MANIFEST_FILENAME).exists() {
            let rust_env = project::runtime_env_rust(&dir);
            if rust_env.join("Cargo.toml").exists() {
                roots.rust_root = Some(rust_env);
            }
        }
        // Fallback: classic layout (go.mod / package.json / Cargo.toml at project root)
        if roots.go_root.is_none() && dir.join("go.mod").exists() {
            roots.go_root = Some(dir.clone());
        }
        if roots.node_root.is_none() &&
            (dir.join("package.json").exists() || dir.join("node_modules").exists())
        {
            roots.node_root = Some(dir.clone());
        }
        if roots.rust_root.is_none() && dir.join("Cargo.toml").exists() {
            roots.rust_root = Some(dir.clone());
        }
        if roots.go_root.is_some() && roots.node_root.is_some() && roots.rust_root.is_some() {
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

    let filename = file.file_name().and_then(|s| s.to_str()).unwrap_or("unknown");

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
        "rust" | "rs" => {
            let code = runtime::rust::codegen::generate(&ir)?;
            let out_path = output.join("main.rs");
            std::fs::write(&out_path, &code)
                .map_err(|e| miette::miette!("Failed to write generated code: {}", e))?;
            println!("{} Generated Rust: {}", "✓".green().bold(), out_path.display());
        }
        _ => {
            return Err(miette::miette!("Unknown language: {}", lang));
        }
    }

    Ok(())
}

fn cmd_upgrade() -> Result<()> {
    use colored::Colorize;
    use std::io::Write;

    let current = VERSION;
    let latest = match version_check::fetch_latest_version() {
        Some(v) => v,
        None => {
            eprintln!(
                "{} Could not fetch latest version from GitHub. Check your internet connection.",
                "⚠".yellow()
            );
            return Ok(());
        }
    };

    if !version_check::is_older(current, &latest) {
        println!("{} Already on latest version ({}).", "✓".green().bold(), current);
        return Ok(());
    }

    let download_url = match version_check::get_download_url(&latest) {
        Some(url) => url,
        None => {
            eprintln!(
                "{} No pre-built binary available for this platform. Build from source:",
                "⚠".yellow()
            );
            eprintln!("    git clone https://github.com/evm-tooling/poly-bench");
            eprintln!("    cd poly-bench && cargo build --release");
            return Ok(());
        }
    };

    println!("Upgrading from {} to {}...", current, latest);
    println!("Downloading from: {}", download_url);

    let current_exe = match std::env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("{} Could not determine current executable path: {}", "⚠".yellow(), e);
            return Ok(());
        }
    };

    let temp_path = current_exe.with_extension("new");

    let response = match ureq::get(&download_url).set("User-Agent", "poly-bench-cli").call() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{} Failed to download: {}", "⚠".yellow(), e);
            return Ok(());
        }
    };

    let mut bytes = Vec::new();
    if let Err(e) = response.into_reader().read_to_end(&mut bytes) {
        eprintln!("{} Failed to read download: {}", "⚠".yellow(), e);
        return Ok(());
    }

    if let Err(e) = std::fs::File::create(&temp_path).and_then(|mut f| f.write_all(&bytes)) {
        eprintln!("{} Failed to write temporary file: {}", "⚠".yellow(), e);
        return Ok(());
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Err(e) = std::fs::set_permissions(&temp_path, std::fs::Permissions::from_mode(0o755))
        {
            eprintln!("{} Failed to set executable permissions: {}", "⚠".yellow(), e);
            let _ = std::fs::remove_file(&temp_path);
            return Ok(());
        }
    }

    if let Err(e) = std::fs::rename(&temp_path, &current_exe) {
        eprintln!("{} Failed to replace binary: {}", "⚠".yellow(), e);
        eprintln!("    You may need to run with elevated permissions (sudo).");
        let _ = std::fs::remove_file(&temp_path);
        return Ok(());
    }

    println!("{} Upgraded to poly-bench {}.", "✓".green().bold(), latest);
    Ok(())
}

fn cmd_init(name: Option<&str>, languages: Vec<String>, no_example: bool) -> Result<()> {
    let (name, languages, quiet) = match name {
        Some(n) => (n.to_string(), languages, false),
        None => {
            let (name, languages) = init_interactive()?;
            (name, languages, true)
        }
    };
    let is_current_dir = name == ".";
    let options = project::init::InitOptions { name: name.clone(), languages, no_example, quiet };
    project::init::init_project(&options)?;
    if quiet {
        if is_current_dir {
            // Get actual directory name for display
            let dir_name = std::env::current_dir()
                .ok()
                .and_then(|p| p.file_name().map(|s| s.to_string_lossy().to_string()))
                .unwrap_or_else(|| ".".to_string());
            init_t3::print_init_success_block_current_dir(&dir_name);
        } else {
            init_t3::print_init_success_block(&options.name);
        }
    }
    Ok(())
}

/// Interactive init: T3-style menu flow with │ ◇ prompts and blocky logo.
fn init_interactive() -> Result<(String, Vec<String>)> {
    use dialoguer::{Input, MultiSelect};
    use miette::miette;
    use std::{thread, time::Duration};

    init_t3::print_init_logo();
    thread::sleep(Duration::from_millis(120));

    let theme = init_t3::T3StyleTheme::new();
    let name: String = Input::with_theme(&theme)
        .with_prompt("What will your project be called?")
        .default("my-bench".into())
        .validate_with(|s: &String| {
            if s.trim().is_empty() {
                return Err("Project name cannot be empty".to_string());
            }
            if s.contains(std::path::MAIN_SEPARATOR) {
                return Err("Project name must not contain path separators".to_string());
            }
            Ok(())
        })
        .interact_text()
        .map_err(|e| miette!("Prompt failed: {}", e))?;
    let name = name.trim().to_string();

    thread::sleep(Duration::from_millis(80));

    let lang_choices = &["All (Go + TypeScript + Rust)", "Go", "TypeScript", "Rust"];
    let defaults = vec![false, false, false, false]; // Nothing selected by default
    let prompt = "Which languages to include? (Space = toggle)";
    let selected: Vec<usize> = MultiSelect::with_theme(&theme)
        .with_prompt(prompt)
        .items(lang_choices)
        .defaults(&defaults)
        .interact()
        .map_err(|e| miette!("Prompt failed: {}", e))?;

    if selected.is_empty() {
        return Err(miette!("Select at least one language"));
    }

    // Resolve selection to language list.
    // Index mapping: 0 = All, 1 = Go, 2 = TypeScript, 3 = Rust
    let languages: Vec<String> = if selected.contains(&0) {
        // "All" is selected - check which individual langs are also selected
        let individual_langs: Vec<String> = selected
            .iter()
            .filter_map(|&i| match i {
                1 => Some("go".to_string()),
                2 => Some("ts".to_string()),
                3 => Some("rust".to_string()),
                _ => None,
            })
            .collect();
        if individual_langs.is_empty() {
            // All alone = all three languages
            vec!["go".to_string(), "ts".to_string(), "rust".to_string()]
        } else {
            individual_langs
        }
    } else {
        // No "All" selected, just use individual selections
        selected
            .into_iter()
            .filter_map(|i| match i {
                1 => Some("go".to_string()),
                2 => Some("ts".to_string()),
                3 => Some("rust".to_string()),
                _ => None,
            })
            .collect()
    };

    Ok((name, languages))
}

fn cmd_new(name: &str) -> Result<()> {
    project::init::new_benchmark(name)?;
    Ok(())
}

fn cmd_add(
    go: Option<String>,
    ts: Option<String>,
    rs: Option<String>,
    features: Option<Vec<String>>,
) -> Result<()> {
    if go.is_none() && ts.is_none() && rs.is_none() {
        return Err(miette::miette!(
            "No dependency specified. Use --go, --ts, or --rs to add a dependency."
        ));
    }

    if let Some(ref spec) = go {
        project::deps::add_go_dependency(spec)?;
    }

    if let Some(ref spec) = ts {
        project::deps::add_ts_dependency(spec)?;
    }

    if let Some(ref spec) = rs {
        project::deps::add_rust_dependency_with_features(spec, features.as_deref())?;
    }

    Ok(())
}

fn cmd_remove(go: Option<String>, ts: Option<String>, rs: Option<String>) -> Result<()> {
    if go.is_none() && ts.is_none() && rs.is_none() {
        return Err(miette::miette!(
            "No dependency specified. Use --go, --ts, or --rs to remove a dependency."
        ));
    }

    if let Some(ref package) = go {
        project::deps::remove_go_dependency(package)?;
    }

    if let Some(ref package) = ts {
        project::deps::remove_ts_dependency(package)?;
    }

    if let Some(ref crate_name) = rs {
        project::deps::remove_rust_dependency(crate_name)?;
    }

    Ok(())
}

fn cmd_install() -> Result<()> {
    project::deps::install_all()
}

fn cmd_build(force: bool, skip_install: bool) -> Result<()> {
    let options = project::build::BuildOptions { force, skip_install };
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

        // Use the LSP formatter for consistent behavior with on-save formatting.
        // This formatter correctly removes empty code blocks (init, declare, helpers, import)
        // while preserving globalSetup blocks that have statements.
        let formatted = poly_bench_lsp_v2::formatter::format_source(&source);

        if write {
            std::fs::write(file, &formatted)
                .map_err(|e| miette::miette!("Failed to write {}: {}", file.display(), e))?;
            println!("{} {}", "✓".green().bold(), file.display());
        } else {
            print!("{}", formatted);
        }
    }

    Ok(())
}

/// Create a spinner for the "Compiling..." phase
fn create_compiling_spinner() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap()
            .tick_strings(&["[±]", "[∓]", "[±]", "[∓]"]),
    );
    pb.set_message("Compiling...");
    pb.enable_steady_tick(Duration::from_millis(120));
    pb
}
