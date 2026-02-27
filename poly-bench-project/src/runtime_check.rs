//! Runtime detection and install hints for external language runtimes.
//!
//! Checks if language binaries (go, node, cargo, etc.) are on PATH and provides
//! manual install instructions when they are not.

use miette::miette;
use poly_bench_dsl::Lang;
use std::path::PathBuf;

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

/// Base dir for poly-bench runtimes (matches runtime_installer).
fn runtimes_base_dir() -> Option<PathBuf> {
    if cfg!(windows) {
        let local = std::env::var("LOCALAPPDATA")
            .or_else(|_| std::env::var("USERPROFILE").map(|p| format!("{}\\AppData\\Local", p)))
            .ok()?;
        Some(PathBuf::from(local).join("polybench").join("runtimes"))
    } else {
        let home = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")).ok()?;
        Some(PathBuf::from(home).join(".local").join("share").join("polybench").join("runtimes"))
    }
}

/// Legacy base dir (~/.polybench/runtimes) for backward compatibility.
fn legacy_runtimes_base_dir() -> Option<PathBuf> {
    let home = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")).ok()?;
    Some(PathBuf::from(home).join(".polybench").join("runtimes"))
}

/// User-local bin dir (~/.local/bin) where poly-bench symlinks binaries.
fn local_bin_dir() -> Option<PathBuf> {
    if cfg!(windows) {
        let local = std::env::var("LOCALAPPDATA")
            .or_else(|_| std::env::var("USERPROFILE").map(|p| format!("{}\\AppData\\Local", p)))
            .ok()?;
        Some(PathBuf::from(local).join("polybench").join("bin"))
    } else {
        let home = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")).ok()?;
        Some(PathBuf::from(home).join(".local").join("bin"))
    }
}

/// Returns true if the binary exists in /usr/local/bin, ~/.local/bin, or poly-bench runtimes dir.
fn is_installed_in_polybench_dir(lang: Lang) -> bool {
    // Check /usr/local/bin (system install)
    #[cfg(unix)]
    {
        let system_bin = PathBuf::from("/usr/local/bin");
        let has_binary = match lang {
            Lang::Go => system_bin.join("go").exists(),
            Lang::TypeScript => system_bin.join("node").exists(),
            Lang::Rust => system_bin.join("cargo").exists(),
            Lang::Python => {
                system_bin.join("python3").exists() || system_bin.join("python").exists()
            }
            Lang::Zig => system_bin.join("zig").exists(),
            Lang::CSharp => system_bin.join("dotnet").exists(),
            Lang::C => false,
        };
        if has_binary {
            return true;
        }
    }
    // Check ~/.local/bin (user-local install)
    if let Some(bin_dir) = local_bin_dir() {
        let has_binary = match lang {
            Lang::Go => bin_dir.join("go").exists(),
            Lang::TypeScript => bin_dir.join("node").exists(),
            Lang::Rust => bin_dir.join("cargo").exists(),
            Lang::Python => bin_dir.join("python3").exists() || bin_dir.join("python").exists(),
            Lang::Zig => bin_dir.join("zig").exists(),
            Lang::CSharp => bin_dir.join("dotnet").exists(),
            Lang::C => false,
        };
        if has_binary {
            return true;
        }
    }
    // Fallback: check runtimes dir
    for base in [runtimes_base_dir(), legacy_runtimes_base_dir()] {
        let base = match base {
            Some(b) => b.join(lang.as_str()),
            None => continue,
        };
        if !base.exists() {
            continue;
        }
        let found = match lang {
            Lang::Go => base.join("go").join("bin").join("go").exists(),
            Lang::TypeScript => {
                if let Ok(entries) = std::fs::read_dir(&base) {
                    entries.filter_map(|e| e.ok()).any(|e| {
                        let name = e.file_name();
                        let name = name.to_str().unwrap_or("");
                        name.starts_with("node-v") && e.path().join("bin").join("node").exists()
                    })
                } else {
                    false
                }
            }
            Lang::Rust => {
                base.join(".cargo").join("bin").join("cargo").exists() ||
                    base.join(".cargo").join("bin").join("cargo.exe").exists()
            }
            Lang::Python => {
                base.join("install").join("bin").join("python3").exists() ||
                    base.join("install").join("bin").join("python").exists() ||
                    base.join("install").join("python.exe").exists()
            }
            Lang::Zig => {
                if let Ok(entries) = std::fs::read_dir(&base) {
                    entries.filter_map(|e| e.ok()).any(|e| {
                        let name = e.file_name();
                        let name = name.to_str().unwrap_or("");
                        (name.starts_with("zig-") || name.starts_with("zig_")) &&
                            (e.path().join("zig").exists() || e.path().join("zig.exe").exists())
                    })
                } else {
                    false
                }
            }
            Lang::CSharp => base.join("dotnet").exists(),
            Lang::C => false, // C is never auto-installed by poly-bench
        };
        if found {
            return true;
        }
    }
    false
}

/// Returns true if the language runtime is installed (binary found on PATH or in poly-bench dir).
pub fn is_lang_installed(lang: Lang) -> bool {
    for bin in required_binary(lang) {
        if which::which(bin).is_ok() {
            return true;
        }
    }
    is_installed_in_polybench_dir(lang)
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
