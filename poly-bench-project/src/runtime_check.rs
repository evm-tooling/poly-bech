//! Runtime detection and install hints for external language runtimes.
//!
//! Checks if language binaries (go, node, cargo, etc.) are on PATH and provides
//! manual install instructions when they are not.

use miette::miette;
use poly_bench_dsl::Lang;

/// Returns the binary name(s) to check for a language.
/// For languages with alternatives (e.g. python3/python), we check in order.
pub fn required_binary(lang: Lang) -> &'static [&'static str] {
    match lang {
        Lang::Go => &["go"],
        Lang::TypeScript => &["node"],
        Lang::Rust => &["cargo"],
        Lang::Python => &["python3", "python"],
        Lang::C => &["clang"],
        Lang::CSharp => &["dotnet"],
        Lang::Zig => &["zig"],
    }
}

/// Returns true if the language runtime is installed (binary found on PATH).
pub fn is_lang_installed(lang: Lang) -> bool {
    for bin in required_binary(lang) {
        if which::which(bin).is_ok() {
            return true;
        }
    }
    false
}

/// Batch check: returns (lang, installed) for each language.
pub fn check_langs_installed(langs: &[Lang]) -> Vec<(Lang, bool)> {
    langs.iter().map(|&lang| (lang, is_lang_installed(lang))).collect()
}

/// Returns manual install instructions for a language.
/// For C, returns platform-specific commands (brew, apt, choco).
pub fn install_hint(lang: Lang) -> String {
    match lang {
        Lang::Go => "Install Go: https://go.dev/dl/".to_string(),
        Lang::TypeScript => "Install Node.js: https://nodejs.org/".to_string(),
        Lang::Rust => "Install Rust: https://rustup.rs/".to_string(),
        Lang::Python => "Install Python: https://www.python.org/downloads/".to_string(),
        Lang::C => platform_clang_hint(),
        Lang::CSharp => "Install .NET: https://dotnet.microsoft.com/download".to_string(),
        Lang::Zig => "Install Zig: https://ziglang.org/download/".to_string(),
    }
}

fn platform_clang_hint() -> String {
    let (pkg_manager, cmd) = if cfg!(target_os = "macos") {
        ("Homebrew", "brew install llvm")
    } else if cfg!(target_os = "linux") {
        ("apt", "sudo apt install clang")
    } else if cfg!(target_os = "windows") {
        ("Chocolatey", "choco install llvm")
    } else {
        ("your package manager", "install clang or llvm")
    };
    format!("Install clang via {}: {}", pkg_manager, cmd)
}

/// Returns a miette error for when a language is not installed.
/// Includes install hint and add-runtime suggestion.
pub fn not_installed_error(lang: Lang) -> miette::Report {
    let label = poly_bench_runtime::lang_label(lang);
    let hint = install_hint(lang);
    miette!(
        "{} is not installed. {} Run 'poly-bench add-runtime {}' to install, or install manually: {}",
        label,
        if lang == Lang::C {
            "C requires clang. "
        } else {
            ""
        },
        lang.as_str(),
        hint
    )
}
