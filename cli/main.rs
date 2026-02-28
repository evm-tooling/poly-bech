//! poly-bench CLI entrypoint

mod init_t3;
mod ui;
mod version_check;
mod welcome;

use clap::{
    builder::styling::{AnsiColor, Effects, Styles},
    Args, Parser, Subcommand,
};
use indicatif::ProgressBar;
use miette::Result;
use std::{io::IsTerminal, path::PathBuf};

use poly_bench_dsl as dsl;
use poly_bench_executor as executor;
use poly_bench_ir as ir;
use poly_bench_project as project;
use poly_bench_reporter as reporter;
use poly_bench_runtime as runtime;

/// Current binary version (set at compile time).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_BANNER: &str = r#"
POLY BENCH
"#;

const ROOT_HELP_TEMPLATE: &str = "\
{before-help}\
{about-with-newline}\
{usage-heading} {usage}\n\n\
{subcommands}\
{options}\
{after-help}\
";

fn cli_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default() | Effects::BOLD)
        .usage(AnsiColor::Cyan.on_default() | Effects::BOLD)
        .literal(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Magenta.on_default())
        .error(AnsiColor::Red.on_default() | Effects::BOLD)
        .valid(AnsiColor::Green.on_default() | Effects::BOLD)
        .invalid(AnsiColor::Red.on_default() | Effects::BOLD)
}

#[derive(Parser)]
#[command(name = "poly-bench")]
#[command(author = "Evan McGrane")]
#[command(disable_version_flag(true))]
#[command(before_help = HELP_BANNER)]
#[command(help_template = ROOT_HELP_TEMPLATE)]
#[command(styles = cli_styles())]
#[command(about = "Build, run, and compare benchmarks across Go and TypeScript with a custom DSL.", long_about = None)]
struct Cli {
    /// Print version and exit
    #[arg(long, short = 'V', global = true)]
    version: bool,

    /// Colorize output [possible values: auto, always, never]
    #[arg(long, global = true, value_name = "WHEN", help_heading = "Display options")]
    color: Option<String>,

    /// Reduce log output
    #[arg(short, long, global = true, help_heading = "Display options")]
    quiet: bool,

    /// Show verbose runtime diagnostics (raw external traces)
    #[arg(short = 'v', long, global = true, help_heading = "Display options")]
    verbose: bool,

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

        /// Check only benchmarks for a specific language (go, ts, rust, python, c, csharp)
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

        /// Run only benchmarks for a specific language (go, ts, rust, python, c, csharp)
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

        /// Project root for a language (repeatable). Format: LANG:DIR (e.g. go:./my-go-mod,
        /// csharp:./src)
        #[arg(long, value_name = "LANG:DIR")]
        project_dir: Vec<String>,
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

        /// Languages to include (comma-separated: go,ts,rust,python,c,csharp); used only when NAME
        /// is provided
        #[arg(long, short, value_delimiter = ',', default_value = "go,ts,c")]
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

        /// Python package (e.g., "numpy==1.0" or "requests")
        #[arg(long)]
        py: Option<String>,

        /// C package/library tag (e.g., "openssl@3.2")
        #[arg(long)]
        c: Option<String>,

        /// C# package (e.g., "Newtonsoft.Json@13.0.3")
        #[arg(long)]
        cs: Option<String>,

        /// Zig package (e.g., "package@0.1.0")
        #[arg(long)]
        zig: Option<String>,

        /// Rust crate features (comma-separated, e.g., "keccak,sha3")
        #[arg(long, value_delimiter = ',')]
        features: Option<Vec<String>>,
    },

    /// Add a runtime to the project (adds to polybench.toml and builds .polybench)
    AddRuntime {
        /// Runtime to add (go, ts, rust, python, c, csharp)
        #[arg(value_name = "RUNTIME")]
        runtime: String,

        /// Install to ~/.local/bin (user-local) instead of /usr/local/bin (default, requires sudo)
        #[arg(long)]
        user_local: bool,
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

        /// Python package to remove (e.g., "numpy")
        #[arg(long)]
        py: Option<String>,

        /// C package/library tag to remove (e.g., "openssl")
        #[arg(long)]
        c: Option<String>,

        /// C# package to remove (e.g., "Newtonsoft.Json")
        #[arg(long)]
        cs: Option<String>,

        /// Zig package to remove
        #[arg(long)]
        zig: Option<String>,
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

    /// Generate charts from results.json without running benchmarks
    Plot {
        #[command(subcommand)]
        subcommand: PlotSubcommand,
    },

    /// Start the language server (for editors, v2)
    Lsp {
        /// Accepted for editor compatibility; stdio is always used
        #[arg(long, hide = true)]
        stdio: bool,
    },
}

#[derive(Subcommand)]
enum PlotSubcommand {
    /// Plot using chart directives from a .bench file's after block
    FromFile {
        /// Path to the .bench file (optional if in a poly-bench project)
        #[arg(value_name = "FILE")]
        file: Option<PathBuf>,

        /// Path to results JSON (default: out/results.json)
        #[arg(long, value_name = "PATH")]
        results: Option<PathBuf>,

        /// Output directory for charts (default: out/)
        #[arg(long, short, value_name = "DIR")]
        output: Option<PathBuf>,
    },

    /// Generate a bar chart from results
    BarChart {
        #[command(flatten)]
        params: ChartDirectParams,
    },

    /// Generate a line chart from results
    LineChart {
        #[command(flatten)]
        params: ChartDirectParams,
    },

    /// Generate a speedup chart from results
    SpeedupChart {
        #[command(flatten)]
        params: ChartDirectParams,
    },

    /// Generate a table from results
    Table {
        #[command(flatten)]
        params: ChartDirectParams,
    },
}

/// Shared parameters for direct chart subcommands (bar-chart, line-chart, etc.)
#[derive(Args)]
struct ChartDirectParams {
    /// Path to results JSON (default: out/results.json)
    #[arg(long, value_name = "PATH")]
    results: Option<PathBuf>,

    /// Output directory for charts (default: out/)
    #[arg(long, short, value_name = "DIR")]
    output: Option<PathBuf>,

    /// Output filename for the chart (e.g. results.svg)
    #[arg(long, value_name = "FILE")]
    output_file: Option<String>,

    /// Chart title
    #[arg(long)]
    title: Option<String>,

    /// Chart description
    #[arg(long)]
    description: Option<String>,

    /// Chart width in pixels
    #[arg(long, value_name = "N")]
    width: Option<i32>,

    /// Chart height in pixels
    #[arg(long, value_name = "N")]
    height: Option<i32>,

    /// Filter to a specific suite by name
    #[arg(long, value_name = "NAME")]
    suite: Option<String>,

    /// Only show benchmarks with speedup >= N
    #[arg(long, value_name = "N")]
    min_speedup: Option<f64>,

    /// Filter by winner: go, ts, or all
    #[arg(long, value_name = "LANG")]
    filter_winner: Option<String>,

    /// Only include these benchmark names (comma-separated)
    #[arg(long, value_name = "NAMES")]
    include: Option<String>,

    /// Exclude these benchmark names (comma-separated)
    #[arg(long, value_name = "NAMES")]
    exclude: Option<String>,

    /// Max benchmarks to show
    #[arg(long, value_name = "N")]
    limit: Option<u32>,

    /// Sort by: speedup, name, time, ops
    #[arg(long, value_name = "KEY")]
    sort_by: Option<String>,

    /// Sort order: asc or desc
    #[arg(long, value_name = "ORDER")]
    sort_order: Option<String>,

    /// Baseline benchmark name (speedup-chart only)
    #[arg(long, value_name = "NAME")]
    baseline: Option<String>,

    /// Color theme: dark or light
    #[arg(long, value_name = "THEME")]
    theme: Option<String>,

    /// Number of benchmark cards per row (speedup-chart only)
    #[arg(long, value_name = "N")]
    row_count: Option<u32>,

    /// Show standard deviation overlays
    #[arg(long)]
    show_std_dev: Option<bool>,

    /// Show error bars
    #[arg(long)]
    show_error_bars: Option<bool>,

    /// Show regression trendline
    #[arg(long)]
    show_regression: Option<bool>,

    /// Regression model: auto, linear, quadratic, etc.
    #[arg(long, value_name = "MODEL")]
    regression_model: Option<String>,

    /// Y-axis scale: linear, log10, symlog, split
    #[arg(long, value_name = "SCALE")]
    y_scale: Option<String>,

    /// Show stats table below chart
    #[arg(long)]
    show_stats_table: Option<bool>,
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
    runtime::init_import_extractors();

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
            cmd_compile(file, lang, no_cache, clear_cache, cli.verbose).await?;
        }
        Commands::Cache { action } => {
            cmd_cache(action).await?;
        }
        Commands::Run { file, lang, iterations, report, output, project_dir } => {
            cmd_run(file, lang, iterations, &report, output, project_dir, cli.verbose).await?;
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
        Commands::Add { go, ts, rs, py, c, cs, zig, features } => {
            cmd_add(go, ts, rs, py, c, cs, zig, features)?;
        }
        Commands::AddRuntime { runtime, user_local } => {
            cmd_add_runtime(&runtime, user_local)?;
        }
        Commands::Remove { go, ts, rs, py, c, cs, zig } => {
            cmd_remove(go, ts, rs, py, c, cs, zig)?;
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
        Commands::Plot { subcommand } => {
            cmd_plot(subcommand).await?;
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

    ui::section("Check benchmark file");
    ui::kv("file", file.display().to_string());

    let source = std::fs::read_to_string(file)
        .map_err(|e| miette::miette!("Failed to read file {}: {}", file.display(), e))?;

    let filename = file.file_name().and_then(|s| s.to_str()).unwrap_or("unknown");

    match dsl::parse(&source, filename) {
        Ok(ast) => {
            ui::success(format!("Parsed {}", file.display()));
            ui::kv("suites", ast.suites.len().to_string());

            for suite in &ast.suites {
                ui::indented_line(format!(
                    "{}: {} benchmark(s), {} fixture(s)",
                    suite.name.bold(),
                    suite.benchmarks.len(),
                    suite.fixtures.len()
                ));
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
    verbose: bool,
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
    let langs = selected_languages(lang.as_deref())?;

    if run_parallel && files.len() > 1 {
        compile_files_parallel_cached(&files, &langs, &cache, verbose).await
    } else {
        compile_files_sequential_cached(&files, &langs, &cache, verbose).await
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
    let project_roots = resolve_project_roots(&std::collections::HashMap::new(), &bench_file)?;

    let (compile_errors, stats) =
        executor::validate_benchmarks_with_cache(&ir_result, &langs, &project_roots, cache).await?;

    Ok(CompileResultCached { file: bench_file, bench_count, errors: compile_errors, stats })
}

async fn compile_files_parallel_cached(
    files: &[PathBuf],
    langs: &[dsl::Lang],
    cache: &executor::CompileCache,
    verbose: bool,
) -> Result<()> {
    use colored::Colorize;
    use futures::future::join_all;

    let cache_status = if cache.is_enabled() { " (with caching)" } else { "" };
    ui::section("Compile");
    ui::kv("mode", "parallel");
    ui::kv("files", files.len().to_string());
    ui::kv("cache", if cache.is_enabled() { "enabled" } else { "disabled" });
    if !cache_status.is_empty() {
        ui::info("Using compile cache");
    }

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
        ui::success(format!(
            "{} - {} benchmark(s) compiled successfully{}",
            result.file.display(),
            result.bench_count,
            cache_info.dimmed()
        ));
    }

    for result in &failed_results {
        print_compile_errors_for_file(&result.file, &result.errors, verbose);
    }

    if total_errors > 0 {
        std::process::exit(1);
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
    verbose: bool,
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

        let project_roots = resolve_project_roots(&std::collections::HashMap::new(), bench_file)?;

        let spinner = create_compiling_spinner();
        let (compile_errors, stats) =
            executor::validate_benchmarks_with_cache(&ir_result, langs, &project_roots, cache)
                .await?;
        spinner.finish_and_clear();

        total_cache_hits += stats.cache_hits;
        total_cache_misses += stats.cache_misses;

        if !compile_errors.is_empty() {
            total_errors += compile_errors.len();
            print_compile_errors_for_file(bench_file, &compile_errors, verbose);
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
        std::process::exit(1);
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
    project_dir: Vec<String>,
    verbose: bool,
) -> Result<()> {
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
    let langs = selected_languages(lang.as_deref())?;

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
        let project_roots = resolve_project_roots(&parse_project_dirs(&project_dir)?, bench_file)?;

        println!("▸ Compile validation");
        // Pre-run validation: compile-check all benchmarks before running
        let spinner = create_compiling_spinner();
        let compile_errors = executor::validate_benchmarks(&ir, &langs, &project_roots).await?;
        spinner.finish_and_clear();

        if !compile_errors.is_empty() {
            print_compile_errors_for_file(bench_file, &compile_errors, verbose);
            std::process::exit(1);
        }
        println!("  ✓ Compile validation passed");
        println!();
        println!("{}", "─".repeat(78));
        println!("■ Run benchmarks ▸ Executing {}", bench_file.display());
        println!("{}", "─".repeat(78));
        println!();

        // Execute benchmarks
        let run_opts = executor::RunOptions { verbose };
        let results = executor::run(&ir, &langs, iterations, &project_roots, &run_opts).await?;
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

    // Auto-save results to out/results.json
    std::fs::create_dir_all(&default_output_dir)
        .map_err(|e| miette::miette!("Failed to create output directory: {}", e))?;
    let json = reporter::json::report(&results)?;
    let results_path = default_output_dir.join("results.json");
    std::fs::write(&results_path, &json)
        .map_err(|e| miette::miette!("Failed to save results: {}", e))?;

    // Execute chart directives if any
    if !all_chart_directives.is_empty() {
        let chart_output_dir = output.clone().unwrap_or_else(|| default_output_dir.clone());
        let _ =
            reporter::execute_chart_directives(&all_chart_directives, &results, &chart_output_dir)?;
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

    println!("Benchmark successful. Results saved to {}", results_path.display());

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

const DEFAULT_RESULTS_PATH: &str = "out/results.json";
const DEFAULT_OUTPUT_DIR: &str = "out";

async fn cmd_plot(subcommand: PlotSubcommand) -> Result<()> {
    match subcommand {
        PlotSubcommand::FromFile { file, results, output } => {
            cmd_plot_from_file(file, results, output).await
        }
        PlotSubcommand::BarChart { params } => {
            cmd_plot_direct(dsl::ChartType::BarChart, params).await
        }
        PlotSubcommand::LineChart { params } => {
            cmd_plot_direct(dsl::ChartType::LineChart, params).await
        }
        PlotSubcommand::SpeedupChart { params } => {
            cmd_plot_direct(dsl::ChartType::SpeedupChart, params).await
        }
        PlotSubcommand::Table { params } => cmd_plot_direct(dsl::ChartType::Table, params).await,
    }
}

async fn cmd_plot_from_file(
    file: Option<PathBuf>,
    results: Option<PathBuf>,
    output: Option<PathBuf>,
) -> Result<()> {
    let files = match file {
        Some(f) => vec![f],
        None => {
            let current_dir = std::env::current_dir()
                .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

            let project_root = project::find_project_root(&current_dir).ok_or_else(|| {
                miette::miette!(
                    "No .bench file specified and not in a poly-bench project.\n\
                    Either specify a file: poly-bench plot from-file <file.bench>\n\
                    Or run from a project directory"
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

    let mut all_chart_directives = Vec::new();
    for bench_file in &files {
        let source = std::fs::read_to_string(bench_file)
            .map_err(|e| miette::miette!("Failed to read file {}: {}", bench_file.display(), e))?;

        let filename = bench_file.file_name().and_then(|s| s.to_str()).unwrap_or("unknown");
        let ast = dsl::parse(&source, filename)?;
        let ir = ir::lower(&ast, bench_file.parent())?;
        all_chart_directives.extend(ir.chart_directives);
    }

    if all_chart_directives.is_empty() {
        return Err(miette::miette!(
            "No chart directives found in the specified .bench file(s).\n\
            Add charting directives in an after block, e.g.:\n\
            after {{ charting.drawSpeedupChart(title: \"Results\", output: \"chart.svg\") }}"
        ));
    }

    let results_path = results.unwrap_or_else(|| PathBuf::from(DEFAULT_RESULTS_PATH));
    let json = std::fs::read_to_string(&results_path).map_err(|e| {
        miette::miette!(
            "Failed to read results file {}: {}.\n\
            Run benchmarks first: poly-bench run",
            results_path.display(),
            e
        )
    })?;

    let benchmark_results: BenchmarkResults = serde_json::from_str(&json).map_err(|e| {
        miette::miette!(
            "Failed to parse results JSON: {}. File may be from an incompatible version.",
            e
        )
    })?;

    let output_dir = output.unwrap_or_else(|| PathBuf::from(DEFAULT_OUTPUT_DIR));
    let generated =
        reporter::execute_chart_directives(&all_chart_directives, &benchmark_results, &output_dir)?;

    println!("Generated {} chart(s) in {}", generated.len(), output_dir.display());
    for chart in &generated {
        println!("  {}", chart.path);
    }

    Ok(())
}

async fn cmd_plot_direct(chart_type: dsl::ChartType, params: ChartDirectParams) -> Result<()> {
    let output_file = params.output_file.clone().ok_or_else(|| {
        miette::miette!(
            "--output-file is required for direct chart generation.\n\
            Example: poly-bench plot bar-chart --output-file results.svg"
        )
    })?;

    let results_path =
        params.results.clone().unwrap_or_else(|| PathBuf::from(DEFAULT_RESULTS_PATH));
    let json = std::fs::read_to_string(&results_path).map_err(|e| {
        miette::miette!(
            "Failed to read results file {}: {}.\n\
            Run benchmarks first: poly-bench run",
            results_path.display(),
            e
        )
    })?;

    let benchmark_results: BenchmarkResults = serde_json::from_str(&json).map_err(|e| {
        miette::miette!(
            "Failed to parse results JSON: {}. File may be from an incompatible version.",
            e
        )
    })?;

    let directive = chart_params_to_directive(chart_type, &output_file, &params);

    let output_dir = params.output.clone().unwrap_or_else(|| PathBuf::from(DEFAULT_OUTPUT_DIR));
    let generated =
        reporter::execute_chart_directives(&[directive], &benchmark_results, &output_dir)?;

    println!("Generated {} chart(s) in {}", generated.len(), output_dir.display());
    for chart in &generated {
        println!("  {}", chart.path);
    }

    Ok(())
}

fn chart_params_to_directive(
    chart_type: dsl::ChartType,
    output_file: &str,
    params: &ChartDirectParams,
) -> ir::ChartDirectiveIR {
    use poly_bench_ir::ChartDirectiveIR;

    let mut ir = ChartDirectiveIR::new(chart_type, output_file.to_string());

    ir.title = params.title.clone();
    ir.description = params.description.clone();
    ir.suite_name = params.suite.clone();
    ir.min_speedup = params.min_speedup;
    ir.filter_winner = params.filter_winner.clone();
    ir.include_benchmarks = params
        .include
        .as_ref()
        .map(|s| s.split(',').map(|x| x.trim().to_string()).collect())
        .unwrap_or_default();
    ir.exclude_benchmarks = params
        .exclude
        .as_ref()
        .map(|s| s.split(',').map(|x| x.trim().to_string()).collect())
        .unwrap_or_default();
    ir.limit = params.limit;
    ir.sort_by = params.sort_by.clone();
    ir.sort_order = params.sort_order.clone();
    ir.width = params.width;
    ir.row_count = params.row_count;
    ir.height = params.height;
    ir.baseline_benchmark = params.baseline.clone();
    ir.theme = params.theme.clone();

    if let Some(v) = params.show_std_dev {
        ir.show_std_dev = v;
    }
    if let Some(v) = params.show_error_bars {
        ir.show_error_bars = v;
    }
    if let Some(v) = params.show_regression {
        ir.show_regression = v;
    }
    if let Some(ref v) = params.regression_model {
        ir.regression_model = v.clone();
    }
    if let Some(ref v) = params.y_scale {
        ir.y_scale = v.clone();
    }
    if let Some(v) = params.show_stats_table {
        ir.show_stats_table = v;
    }

    ir
}

/// Parse --project-dir LANG:DIR args into (Lang, PathBuf) pairs
fn parse_project_dirs(specs: &[String]) -> Result<std::collections::HashMap<dsl::Lang, PathBuf>> {
    let mut map = std::collections::HashMap::new();
    for spec in specs {
        let (lang_str, dir_str) = spec.split_once(':').ok_or_else(|| {
            miette::miette!(
                "Invalid --project-dir '{}': expected LANG:DIR (e.g. go:./my-mod)",
                spec
            )
        })?;
        let lang = parse_lang_arg(lang_str.trim(), "--project-dir")?;
        map.insert(lang, PathBuf::from(dir_str.trim()));
    }
    Ok(map)
}

/// Resolve project roots for module resolution
fn resolve_project_roots(
    explicit: &std::collections::HashMap<dsl::Lang, PathBuf>,
    bench_file: &PathBuf,
) -> Result<ProjectRoots> {
    let mut roots = ProjectRoots::default();

    // Handle explicit project roots from --project-dir
    for (lang, dir) in explicit {
        let canonical = dir
            .canonicalize()
            .map_err(|e| miette::miette!("Cannot access project root {}: {}", dir.display(), e))?;
        let valid = project::is_valid_project_root_for_lang(&canonical, *lang);
        if !valid {
            return Err(miette::miette!(
                "Invalid project root for {}: {} does not contain expected markers",
                lang,
                canonical.display()
            ));
        }
        roots.set_root(*lang, Some(canonical));
    }

    // Search parent directories of the bench file for any missing roots
    let start_dir = bench_file.parent().unwrap_or(std::path::Path::new("."));
    let mut current = start_dir.canonicalize().ok();

    while let Some(dir) = current {
        // Inside a poly-bench project: prefer .polybench/runtime-env/{lang}
        if dir.join(project::MANIFEST_FILENAME).exists() {
            for lang in runtime::supported_languages() {
                if roots.get_root(*lang).is_none() {
                    let env = project::runtime_env(&dir, *lang);
                    if project::is_valid_project_root_for_lang(&env, *lang) {
                        roots.set_root(*lang, Some(env));
                    }
                }
            }
        }
        // Fallback: classic layout via detectors
        for lang in runtime::supported_languages() {
            if roots.get_root(*lang).is_none() {
                if let Some(det) = project::get_detector(*lang) {
                    roots.set_root(*lang, det.detect(&dir));
                }
            }
        }
        if runtime::supported_languages().iter().all(|l| roots.get_root(*l).is_some()) {
            break;
        }
        current = dir.parent().map(|p| p.to_path_buf());
    }

    Ok(roots)
}

fn parse_lang_arg(raw: &str, arg_name: &str) -> Result<dsl::Lang> {
    dsl::Lang::from_str(raw).ok_or_else(|| {
        miette::miette!(
            "Unknown language '{}' for {}. Supported: {}",
            raw,
            arg_name,
            supported_languages_help()
        )
    })
}

fn selected_languages(lang: Option<&str>) -> Result<Vec<dsl::Lang>> {
    match lang {
        Some(raw) => Ok(vec![parse_lang_arg(raw, "--lang")?]),
        None => Ok(runtime::supported_languages().to_vec()),
    }
}

fn supported_languages_help() -> String {
    runtime::supported_languages()
        .iter()
        .map(|l| l.as_str().to_string())
        .collect::<Vec<_>>()
        .join(", ")
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

    let mut response = match ureq::get(&download_url).header("User-Agent", "poly-bench-cli").call()
    {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{} Failed to download: {}", "⚠".yellow(), e);
            return Ok(());
        }
    };

    let bytes = match response.body_mut().with_config().limit(200 * 1024 * 1024).read_to_vec() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{} Failed to read download: {}", "⚠".yellow(), e);
            return Ok(());
        }
    };

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
    let options = project::init::InitOptions {
        name: name.clone(),
        languages,
        no_example,
        quiet,
        defer_final_message: true,
    };
    let project_dir = project::init::init_project(&options)?;

    // Run build to install LSPs and initialize runtime-env
    let project_dir = project_dir.canonicalize().unwrap_or_else(|_| project_dir.clone());
    let prev_cwd = std::env::current_dir().ok();
    std::env::set_current_dir(&project_dir)
        .map_err(|e| miette::miette!("Failed to change to project directory: {}", e))?;

    // Prepend poly-bench runtime paths so build commands find binaries (e.g. after fresh install)
    let paths_to_prepend: Vec<_> = options
        .languages
        .iter()
        .filter_map(|r| poly_bench_dsl::Lang::from_str(r.trim()))
        .filter_map(|l| project::runtime_installer::polybench_runtime_path(l))
        .collect();
    if !paths_to_prepend.is_empty() {
        if let Ok(current) = std::env::var("PATH") {
            let sep = if cfg!(windows) { ";" } else { ":" };
            let prepended = paths_to_prepend
                .iter()
                .map(|p| p.display().to_string())
                .collect::<Vec<_>>()
                .join(sep);
            std::env::set_var("PATH", format!("{}{}{}", prepended, sep, current));
        }
    }

    let build_options = project::build::BuildOptions { force: false, skip_install: false };
    if let Err(e) = project::build::build_project(&build_options) {
        if let Some(ref prev) = prev_cwd {
            let _ = std::env::set_current_dir(prev);
        }
        return Err(e);
    }

    // Output summary after build
    if quiet {
        if is_current_dir {
            let dir_name =
                project_dir.file_name().and_then(|s| s.to_str()).unwrap_or(".").to_string();
            init_t3::print_init_success_block_current_dir(&dir_name);
        } else {
            init_t3::print_init_success_block(&options.name);
        }
    } else {
        let project_name =
            project_dir.file_name().and_then(|s| s.to_str()).unwrap_or(&options.name).to_string();
        println!();
        project::terminal::success(&format!(
            "Project '{}' initialized successfully!",
            project_name
        ));
        println!();
        println!("Next steps:");
        println!("  poly-bench run        # Run benchmarks");
        println!();
    }

    // Cd into project and start shell when we created a new project (not current dir)
    if !is_current_dir && std::io::stdout().is_terminal() && std::io::stderr().is_terminal() {
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            println!("  Starting shell in project directory...");
            let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
            let err = std::process::Command::new(&shell).arg("-i").current_dir(&project_dir).exec();
            eprintln!("Failed to start shell: {}", err);
            std::process::exit(1);
        }
        #[cfg(not(unix))]
        {
            println!("  cd {}", project_dir.display());
        }
    }
    if let Some(ref prev) = prev_cwd {
        let _ = std::env::set_current_dir(prev);
    }

    Ok(())
}

/// User choice when a runtime is not installed
enum InstallChoice {
    Install,
    Skip,
}

/// Interactive init: T3-style menu flow with │ ◇ prompts and blocky logo.
fn init_interactive() -> Result<(String, Vec<String>)> {
    use colored::Colorize;
    use dialoguer::{Input, MultiSelect};
    use miette::miette;
    use poly_bench_dsl::Lang;
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

    // Build choices from supported_languages (All + each language)
    let supported = poly_bench_runtime::supported_languages();
    let all_label = format!(
        "All ({})",
        supported
            .iter()
            .map(|l| poly_bench_runtime::lang_label(*l))
            .collect::<Vec<_>>()
            .join(" + ")
    );
    let mut lang_choices = vec![all_label.as_str()];
    for lang in supported {
        lang_choices.push(poly_bench_runtime::lang_label(*lang));
    }
    let defaults = vec![false; lang_choices.len()];
    let prompt = "Which languages to include? (Space = toggle)";
    let selected: Vec<usize> = MultiSelect::with_theme(&theme)
        .with_prompt(prompt)
        .items(&lang_choices)
        .defaults(&defaults)
        .interact()
        .map_err(|e| miette!("Prompt failed: {}", e))?;

    if selected.is_empty() {
        return Err(miette!("Select at least one language"));
    }

    // Resolve selection to language list.
    // Index 0 = All, 1..=supported.len() = individual languages
    let lang_to_str = |i: usize| -> Option<String> {
        if i == 0 {
            None
        } else {
            supported.get(i - 1).map(|l| l.as_str().to_string())
        }
    };
    let mut languages: Vec<String> = if selected.contains(&0) {
        let individual_langs: Vec<String> =
            selected.iter().filter_map(|&i| lang_to_str(i)).collect();
        if individual_langs.is_empty() {
            supported.iter().map(|l| l.as_str().to_string()).collect()
        } else {
            individual_langs
        }
    } else {
        selected.into_iter().filter_map(lang_to_str).collect()
    };

    // For each selected language, check if runtime is installed. If not, prompt install or skip.
    let mut to_remove = Vec::new();
    for (i, raw) in languages.iter().enumerate() {
        let Some(lang) = Lang::from_str(raw.trim()) else { continue };
        if project::runtime_check::is_lang_installed(lang) {
            continue;
        }
        let label = poly_bench_runtime::lang_label(lang);
        let choice = prompt_install_or_skip(&theme, lang, &label)?;
        match choice {
            InstallChoice::Install => {
                if project::runtime_installer::can_auto_install(lang) {
                    let version = prompt_version_select(&theme, lang, &label)?;
                    let custom_path = prompt_install_path(
                        &theme,
                        lang,
                        project::runtime_installer::InstallLocation::UserLocal,
                        &label,
                    )?;
                    match project::runtime_installer::install_lang(
                        lang,
                        project::runtime_installer::InstallLocation::UserLocal,
                        custom_path,
                        version,
                    ) {
                        Err(e) => {
                            eprintln!("{} Failed to install {}: {}", "✗".red(), label, e);
                            to_remove.push(i);
                            print_runtime_skip_warning(lang, &label);
                        }
                        Ok(Some(custom_bin_dir)) => {
                            if let Ok(Some(config_path)) =
                                project::runtime_installer::ensure_path_in_shell_config(
                                    &custom_bin_dir,
                                )
                            {
                                project::terminal::info_indented(&format!(
                                    "Added to PATH in {}. Run 'source {}' or restart your terminal.",
                                    config_path.display(),
                                    config_path.display()
                                ));
                            }
                        }
                        Ok(None) => {
                            if let Ok(Some(config_path)) =
                                project::runtime_installer::ensure_runtime_in_shell_config(lang)
                            {
                                project::terminal::info_indented(&format!(
                                    "Added to PATH in {}. Run 'source {}' or restart your terminal.",
                                    config_path.display(),
                                    config_path.display()
                                ));
                            }
                        }
                    }
                } else {
                    println!("{} {} requires manual install.", "⚠".yellow(), label);
                    println!("  {}", project::runtime_check::install_hint(lang));
                    to_remove.push(i);
                }
            }
            InstallChoice::Skip => {
                to_remove.push(i);
                print_runtime_skip_warning(lang, &label);
            }
        }
    }
    // Remove in reverse order to preserve indices
    for i in to_remove.into_iter().rev() {
        languages.remove(i);
    }
    if languages.is_empty() {
        return Err(miette!(
            "No languages with installed runtimes. Install at least one runtime manually and run 'poly-bench add-runtime <lang>' later."
        ));
    }

    Ok((name, languages))
}

fn prompt_install_or_skip(
    theme: &init_t3::T3StyleTheme,
    _lang: poly_bench_dsl::Lang,
    label: &str,
) -> Result<InstallChoice> {
    use dialoguer::Select;
    use miette::miette;

    let choices = [
        format!("Install now (poly-bench will download and configure {})", label),
        format!("Skip (install manually later, add via poly-bench add-runtime {})", label),
    ];
    let selected = Select::with_theme(theme)
        .with_prompt(&format!("{} is not installed. What would you like to do?", label))
        .items(&choices)
        .default(0)
        .interact()
        .map_err(|e| miette!("Prompt failed: {}", e))?;
    Ok(if selected == 0 { InstallChoice::Install } else { InstallChoice::Skip })
}

/// Prompts for version selection: latest (recommended) or choose from fetched top 5.
/// Returns None to use default/latest, Some(version) when user picks a specific version.
fn prompt_version_select(
    theme: &init_t3::T3StyleTheme,
    lang: poly_bench_dsl::Lang,
    label: &str,
) -> Result<Option<String>> {
    use colored::Colorize;
    use dialoguer::Select;
    use miette::miette;

    if !project::runtime_installer::supports_version_selection(lang) {
        return Ok(None);
    }

    let choices = ["Latest (recommended)".to_string(), "Choose version...".to_string()];
    let selected = Select::with_theme(theme)
        .with_prompt(&format!("Which {} version to install?", label))
        .items(&choices)
        .default(0)
        .interact()
        .map_err(|e| miette!("Prompt failed: {}", e))?;

    if selected == 0 {
        return Ok(None);
    }

    let versions = match project::runtime_installer::fetch_available_versions(lang) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} Could not fetch versions: {}. Using latest.", "⚠".yellow(), e);
            return Ok(None);
        }
    };

    if versions.is_empty() {
        return Ok(None);
    }

    let selected_idx = Select::with_theme(theme)
        .with_prompt("Select version")
        .items(&versions)
        .default(0)
        .interact()
        .map_err(|e| miette!("Prompt failed: {}", e))?;

    Ok(versions.get(selected_idx).cloned())
}

/// Prompts for install path: default or custom. Returns None for default, Some(path) for custom.
fn prompt_install_path(
    theme: &init_t3::T3StyleTheme,
    lang: poly_bench_dsl::Lang,
    location: project::runtime_installer::InstallLocation,
    label: &str,
) -> Result<Option<PathBuf>> {
    use dialoguer::{Input, Select};
    use miette::miette;

    let default_path = project::runtime_installer::default_install_path(lang, location)
        .map_err(|e| miette!("Could not determine default path: {}", e))?;
    let default_str = default_path.display().to_string();

    let choices =
        [format!("Install to default path: {}", default_str), "Install to custom path".to_string()];
    let selected = Select::with_theme(theme)
        .with_prompt(&format!("Where to install {}?", label))
        .items(&choices)
        .default(0)
        .interact()
        .map_err(|e| miette!("Prompt failed: {}", e))?;

    if selected == 1 {
        let path: String = Input::with_theme(theme)
            .with_prompt("Enter install path")
            .default(default_str)
            .interact_text()
            .map_err(|e| miette!("Input failed: {}", e))?;
        let p = PathBuf::from(path.trim());
        if p.as_os_str().is_empty() {
            Ok(None)
        } else {
            Ok(Some(p))
        }
    } else {
        Ok(None)
    }
}

fn print_runtime_skip_warning(lang: poly_bench_dsl::Lang, label: &str) {
    use colored::Colorize;
    println!();
    println!("{} {} is not installed.", "⚠".yellow(), label);
    println!("  Install manually and run 'poly-bench add-runtime {}' later.", lang.as_str());
    println!("  {}", project::runtime_check::install_hint(lang));
    println!();
}

fn cmd_new(name: &str) -> Result<()> {
    project::init::new_benchmark(name)?;
    Ok(())
}

fn cmd_add(
    go: Option<String>,
    ts: Option<String>,
    rs: Option<String>,
    py: Option<String>,
    c: Option<String>,
    cs: Option<String>,
    zig: Option<String>,
    features: Option<Vec<String>>,
) -> Result<()> {
    if go.is_none() &&
        ts.is_none() &&
        rs.is_none() &&
        py.is_none() &&
        c.is_none() &&
        cs.is_none() &&
        zig.is_none()
    {
        return Err(miette::miette!(
            "No dependency specified. Use --go, --ts, --rs, --py, --c, --cs, or --zig to add a dependency."
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

    if let Some(ref spec) = py {
        project::deps::add_python_dependency(spec)?;
    }

    if let Some(ref spec) = c {
        project::deps::add_c_dependency(spec)?;
    }

    if let Some(ref spec) = cs {
        project::deps::add_csharp_dependency(spec)?;
    }

    if let Some(ref spec) = zig {
        project::deps::add_zig_dependency(spec)?;
    }

    Ok(())
}

fn cmd_remove(
    go: Option<String>,
    ts: Option<String>,
    rs: Option<String>,
    py: Option<String>,
    c: Option<String>,
    cs: Option<String>,
    zig: Option<String>,
) -> Result<()> {
    if go.is_none() &&
        ts.is_none() &&
        rs.is_none() &&
        py.is_none() &&
        c.is_none() &&
        cs.is_none() &&
        zig.is_none()
    {
        return Err(miette::miette!(
            "No dependency specified. Use --go, --ts, --rs, --py, --c, --cs, or --zig to remove a dependency."
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

    if let Some(ref package) = py {
        project::deps::remove_python_dependency(package)?;
    }

    if let Some(ref package) = c {
        project::deps::remove_c_dependency(package)?;
    }

    if let Some(ref package) = cs {
        project::deps::remove_csharp_dependency(package)?;
    }

    if let Some(ref package) = zig {
        project::deps::remove_zig_dependency(package)?;
    }

    Ok(())
}

fn cmd_add_runtime(runtime: &str, user_local: bool) -> Result<()> {
    use colored::Colorize;
    use poly_bench_dsl::Lang;

    let lang = parse_lang_arg(runtime, "add-runtime")?;

    let supported = poly_bench_runtime::supported_languages();
    if !supported.contains(&lang) {
        return Err(miette::miette!(
            "Runtime '{}' is not supported. Supported: {}",
            runtime,
            supported.iter().map(|l| l.as_str()).collect::<Vec<_>>().join(", ")
        ));
    }

    // If runtime not installed, prompt install or exit
    let _did_install = if !project::runtime_check::is_lang_installed(lang) {
        let label = poly_bench_runtime::lang_label(lang);
        let theme = init_t3::T3StyleTheme::new();
        let choice = prompt_install_or_skip(&theme, lang, &label)?;
        match choice {
            InstallChoice::Install => {
                if project::runtime_installer::can_auto_install(lang) {
                    let loc = if user_local {
                        project::runtime_installer::InstallLocation::UserLocal
                    } else {
                        project::runtime_installer::InstallLocation::System
                    };
                    let version = prompt_version_select(&theme, lang, &label)?;
                    let custom_path = prompt_install_path(&theme, lang, loc, &label)?;
                    match project::runtime_installer::install_lang(lang, loc, custom_path, version)?
                    {
                        Some(custom_bin_dir) => {
                            if let Ok(current) = std::env::var("PATH") {
                                let sep = if cfg!(windows) { ";" } else { ":" };
                                std::env::set_var(
                                    "PATH",
                                    format!("{}{}{}", custom_bin_dir.display(), sep, current),
                                );
                            }
                            if let Ok(Some(config_path)) =
                                project::runtime_installer::ensure_path_in_shell_config(
                                    &custom_bin_dir,
                                )
                            {
                                println!("  Added to PATH in {}.", config_path.display());
                                println!(
                                    "  Run 'source {}' or open a new terminal to use {} in future sessions.",
                                    config_path.display(),
                                    poly_bench_runtime::lang_label(lang)
                                );
                            }
                        }
                        None => {
                            if let Some(path) =
                                project::runtime_installer::polybench_runtime_path(lang)
                            {
                                if let Ok(current) = std::env::var("PATH") {
                                    let sep = if cfg!(windows) { ";" } else { ":" };
                                    std::env::set_var(
                                        "PATH",
                                        format!("{}{}{}", path.display(), sep, current),
                                    );
                                }
                            }
                            if let Ok(Some(config_path)) =
                                project::runtime_installer::ensure_runtime_in_shell_config(lang)
                            {
                                println!("  Added to PATH in {}.", config_path.display());
                                println!(
                                    "  Run 'source {}' or open a new terminal to use {} in future sessions.",
                                    config_path.display(),
                                    poly_bench_runtime::lang_label(lang)
                                );
                            }
                        }
                    }
                    true
                } else {
                    println!("{} {} requires manual install.", "⚠".yellow(), label);
                    println!("  {}", project::runtime_check::install_hint(lang));
                    return Err(miette::miette!(
                        "Cannot add {} without runtime. Install manually and run 'poly-bench add-runtime {}' again.",
                        label,
                        lang.as_str()
                    ));
                }
            }
            InstallChoice::Skip => {
                print_runtime_skip_warning(lang, &label);
                return Err(miette::miette!(
                    "Cannot add {} without runtime. Install manually and run 'poly-bench add-runtime {}' again.",
                    label,
                    lang.as_str()
                ));
            }
        }
    } else {
        false
    };

    let current_dir = std::env::current_dir()
        .map_err(|e| miette::miette!("Failed to get current directory: {}", e))?;

    let project_root = project::find_project_root(&current_dir).ok_or_else(|| {
        miette::miette!("Not in a poly-bench project. Run 'poly-bench init' first.")
    })?;

    let mut manifest = project::load_manifest(&project_root)?;

    let lang_str = lang.as_str();
    let already_has = match lang {
        lang => manifest.has_runtime(lang),
    };

    if already_has {
        return Err(miette::miette!(
            "{} is already enabled in this project.",
            poly_bench_runtime::lang_label(lang)
        ));
    }

    // Add to defaults.languages
    if !manifest.defaults.languages.iter().any(|l| Lang::from_str(l.trim()) == Some(lang)) {
        manifest.defaults.languages.push(lang_str.to_string());
    }

    // Add the runtime config section
    match lang {
        Lang::Go => {
            manifest.go = Some(project::manifest::GoConfig {
                module: manifest.project.name.clone(),
                version: Some("1.21".to_string()),
                dependencies: std::collections::HashMap::new(),
            });
        }
        Lang::TypeScript => {
            manifest.ts = Some(project::manifest::TsConfig {
                runtime: "node".to_string(),
                dependencies: std::collections::HashMap::new(),
            });
        }
        Lang::Rust => {
            manifest.rust = Some(project::manifest::RustConfig {
                edition: "2021".to_string(),
                dependencies: std::collections::HashMap::new(),
            });
        }
        Lang::Python => {
            manifest.python = Some(project::manifest::PythonConfig {
                version: Some("3.11".to_string()),
                dependencies: std::collections::HashMap::new(),
            });
        }
        Lang::C => {
            manifest.c = Some(project::manifest::CConfig {
                standard: "c11".to_string(),
                dependencies: std::collections::HashMap::new(),
            });
        }
        Lang::CSharp => {
            manifest.csharp = Some(project::manifest::CSharpConfig {
                target_framework: "net8.0".to_string(),
                dependencies: std::collections::HashMap::new(),
            });
        }
        Lang::Zig => {
            manifest.zig = Some(project::manifest::ZigConfig {
                version: Some("0.13".to_string()),
                dependencies: std::collections::HashMap::new(),
            });
        }
    }

    project::save_manifest(&project_root, &manifest)?;

    println!("{} Added {} to polybench.toml", "✔".green(), poly_bench_runtime::lang_label(lang));

    // Run build to set up .polybench
    let options = project::build::BuildOptions { force: false, skip_install: false };
    project::build::build_project(&options)?;

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
    ui::spinner("Compiling...")
}

fn print_compile_errors_for_file(
    file: &std::path::Path,
    errors: &[executor::CompileError],
    verbose: bool,
) {
    for err in errors {
        print_compile_error(file, err, verbose);
    }
}

fn extract_compile_reason(message: &str) -> String {
    for line in message.lines() {
        let l = line.trim();
        if l.is_empty() {
            continue;
        }
        if l.contains(" error TS") || l.starts_with("error[") || l.starts_with("error:") {
            return l.to_string();
        }
    }
    message
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty())
        .unwrap_or("Unknown compile error")
        .to_string()
}

fn extract_compile_location(message: &str) -> Option<String> {
    for line in message.lines() {
        let l = line.trim();
        if l.contains(".bench file line") {
            return Some(l.to_string());
        }
    }
    None
}

fn extract_compile_snippet(message: &str) -> Option<Vec<String>> {
    let lines: Vec<&str> = message.lines().collect();
    // TS-style single-line diagnostic already carries useful context in the reason line.
    // For Rust/go style, try to show the source line and caret marker.
    let caret_idx = lines.iter().position(|l| l.trim() == "^")?;
    if caret_idx == 0 {
        return None;
    }
    let code_idx = caret_idx - 1;
    let start = code_idx.saturating_sub(1);
    let end = std::cmp::min(lines.len().saturating_sub(1), caret_idx + 1);
    let mut out = Vec::new();
    for (i, line) in lines[start..=end].iter().enumerate() {
        let abs = start + i;
        let marker = if abs == code_idx { ">" } else { " " };
        out.push(format!("{} {}", marker, line.trim_end()));
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

fn compile_lang_label(lang: dsl::Lang) -> &'static str {
    poly_bench_runtime::lang_label(lang)
}

fn print_compile_error(file: &std::path::Path, err: &executor::CompileError, verbose: bool) {
    use colored::Colorize;

    let scope = if err.benchmarks.len() == 1 {
        err.benchmarks[0].clone()
    } else {
        format!("{} error (affects {} benchmarks)", err.source, err.benchmarks.len())
    };
    eprintln!(
        "Error:   {} {} compile failed for {}",
        "×".red().bold(),
        compile_lang_label(err.lang),
        scope
    );
    eprintln!("  │ file: {}", file.display());
    eprintln!("  │ reason: {}", extract_compile_reason(&err.message));

    if let Some(loc) = extract_compile_location(&err.message) {
        eprintln!("  │ location: {}", loc.yellow());
    }
    if let Some(snippet) = extract_compile_snippet(&err.message) {
        eprintln!("  │ snippet:");
        for line in snippet.into_iter().take(4) {
            eprintln!("  │   {}", line.dimmed());
        }
    }

    if verbose {
        eprintln!("  │ raw trace:");
        let lines: Vec<&str> = err.message.lines().collect();
        for line in lines.iter().take(40) {
            if line.trim().is_empty() {
                continue;
            }
            eprintln!("  │   {}", line.dimmed());
        }
        if lines.len() > 40 {
            eprintln!("  │   {}", "... (truncated)".dimmed());
        }
    } else {
        eprintln!("  │ hint: re-run with -v to see full compiler trace");
    }
    eprintln!();
}
