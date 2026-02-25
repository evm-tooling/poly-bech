//! Welcome screen shown on first run and when running `poly-bench` with no args.

use std::path::PathBuf;

/// Block-drawing ASCII art that spells "POLY" then "BENCH" (same graphic for welcome and init).
const POLY_BENCH_LOGO: &str = r#"
  ██████╗  ██████╗ ██╗  ██╗   ██╗   ██████╗ ███████╗███╗   ██╗ ██████╗██╗  ██╗
  ██╔══██╗██╔═══██╗██║  ╚██╗ ██╔╝   ██╔══██╗██╔════╝████╗  ██║██╔════╝██║  ██║
  ██████╔╝██║   ██║██║   ╚████╔╝    ██████╔╝█████╗  ██╔██╗ ██║██║     ███████║
  ██╔═══╝ ██║   ██║██║    ╚██╔╝     ██╔══██╗██╔══╝  ██║╚██╗██║██║     ██╔══██║
  ██║     ╚██████╔╝███████╗██║      ██████╔╝███████╗██║ ╚████║╚██████╗██║  ██║
  ╚═╝      ╚═════╝ ╚══════╝╚═╝      ╚═════╝ ╚══════╝╚═╝  ╚═══╝ ╚═════╝╚═╝  ╚═╝
"#;

const SENTINEL_FILENAME: &str = ".welcome_shown";

fn config_dir() -> Option<PathBuf> {
    std::env::var("HOME").ok().map(|h| PathBuf::from(h).join(".polybench"))
}

fn sentinel_path() -> Option<PathBuf> {
    config_dir().map(|d| d.join(SENTINEL_FILENAME))
}

/// Returns true if the welcome has been shown before (sentinel exists).
fn welcome_already_shown() -> bool {
    sentinel_path().map(|p| p.exists()).unwrap_or(false)
}

/// True if this is the first run (sentinel not yet created). Used to show welcome before running a
/// subcommand.
pub fn is_first_run() -> bool {
    !welcome_already_shown()
}

/// Create config dir and write sentinel file.
fn mark_welcome_shown() {
    if let Some(dir) = config_dir() {
        let _ = std::fs::create_dir_all(&dir);
        if let Some(p) = sentinel_path() {
            let _ = std::fs::File::create(p);
        }
    }
}

/// Prints the Poly Bench block-drawing logo in yellow (used by welcome screen and init flow).
pub fn print_poly_bench_logo() {
    use colored::Colorize;
    for line in POLY_BENCH_LOGO.lines() {
        println!("{}", line.green());
    }
}

/// Show the welcome/help screen (forge-style: Usage, Commands, Options, Display options).
pub fn show_welcome_and_maybe_mark_seen() {
    let first_run = !welcome_already_shown();
    if first_run {
        mark_welcome_shown();
    }

    print_poly_bench_logo();
    println!();
    println!("Build, run, and compare benchmarks across Go and TypeScript with a custom DSL.");
    println!();
    println!("Usage: poly-bench [OPTIONS] <COMMAND>");
    println!();
    print_table(
        "Commands",
        &[
            ("init", "Create a new poly-bench project"),
            ("run", "Run benchmarks from a DSL file or project"),
            ("compile", "Compile-check benchmarks without running"),
            ("check", "Parse and validate a benchmark DSL file"),
            ("codegen", "Generate code from a DSL file without running"),
            ("new", "Create a new benchmark file"),
            ("add", "Add dependency (--go / --ts / --rs / --py)"),
            ("add-runtime", "Add a runtime (go, ts, rust, python) to the project"),
            ("remove", "Remove dependency (--go / --ts / --rs / --py)"),
            ("install", "Install dependencies from polybench.toml"),
            ("build", "Build/regenerate .polybench runtime environment"),
            ("cache", "Show/clear/clean compile cache and workspace"),
            ("fmt", "Format .bench files"),
            ("upgrade", "Upgrade to the latest poly-bench binary"),
            ("lsp", "Start the language server"),
            ("help", "Print this message or subcommand help"),
        ],
    );
    print_table(
        "Options",
        &[
            ("-h, --help", "Print help (use '--help' with a command for details)"),
            ("-V, --version", "Print version"),
            ("--color <WHEN>", "Colorize output [auto, always, never]"),
            ("-q, --quiet", "Reduce log output"),
        ],
    );
    println!();
}

fn print_table(title: &str, rows: &[(&str, &str)]) {
    use colored::Colorize;

    const COL_WIDTH: usize = 18;
    println!("{}:", title.bold().green());
    for (left, right) in rows {
        println!("  {:<width$} {}", left.bright_cyan().bold(), right.dimmed(), width = COL_WIDTH);
    }
    println!();
}
