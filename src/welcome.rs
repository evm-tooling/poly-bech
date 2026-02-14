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
    std::env::var("HOME")
        .ok()
        .map(|h| PathBuf::from(h).join(".polybench"))
}

fn sentinel_path() -> Option<PathBuf> {
    config_dir().map(|d| d.join(SENTINEL_FILENAME))
}

/// Returns true if the welcome has been shown before (sentinel exists).
fn welcome_already_shown() -> bool {
    sentinel_path().map(|p| p.exists()).unwrap_or(false)
}

/// True if this is the first run (sentinel not yet created). Used to show welcome before running a subcommand.
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
    println!("Commands:");
    println!("  init             Create a new poly-bench project (interactive or with NAME)");
    println!("  run              Run benchmarks from a DSL file or project");
    println!("  build            Build/regenerate the .polybench runtime environment");
    println!("  new <name>       Create a new benchmark file");
    println!("  add              Add a Go or TypeScript dependency (--go, --ts)");
    println!("  install          Install dependencies from polybench.toml");
    println!("  fmt              Format .bench files");
    println!("  check <file>      Parse and validate a benchmark DSL file");
    println!("  codegen          Generate code from a DSL file without running");
    println!("  upgrade          Upgrade to the latest poly-bench binary");
    println!("  help             Print this message or the help of the given subcommand(s)");
    println!();
    println!("Options:");
    println!("  -h, --help       Print help (see more with '--help')");
    println!("  -V, --version    Print version");
    println!();
    println!("Display options:");
    println!("      --color <WHEN>  Colorize output [possible values: auto, always, never]");
    println!("  -q, --quiet        Reduce log output");
    println!();
}
