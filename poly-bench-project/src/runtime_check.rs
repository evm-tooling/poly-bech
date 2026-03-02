//! Runtime detection, version checking, and install hints for external language runtimes.
//!
//! Checks if language binaries (go, node, cargo, etc.) are on PATH, parses their versions,
//! and determines compatibility with Polybench's minimum requirements.

use crate::toolchain::{minimum_version, Version};
use miette::miette;
use poly_bench_dsl::Lang;
use std::{path::PathBuf, process::Command};

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

/// Returns the path to the installed binary, if found.
pub fn find_binary_path(lang: Lang) -> Option<PathBuf> {
    for bin in required_binary(lang) {
        if let Ok(path) = which::which(bin) {
            return Some(path);
        }
    }
    None
}

/// Batch check: returns (lang, installed) for each language.
pub fn check_langs_installed(langs: &[Lang]) -> Vec<(Lang, bool)> {
    langs.iter().map(|&lang| (lang, is_lang_installed(lang))).collect()
}

/// Get the installed version of a language runtime by running its version command.
pub fn get_installed_version(lang: Lang) -> Option<Version> {
    let output = match lang {
        Lang::Go => Command::new("go").arg("version").output().ok()?,
        Lang::TypeScript => Command::new("node").arg("-v").output().ok()?,
        Lang::Rust => Command::new("rustc").arg("--version").output().ok()?,
        Lang::Python => Command::new("python3")
            .arg("--version")
            .output()
            .ok()
            .or_else(|| Command::new("python").arg("--version").output().ok())?,
        Lang::C => Command::new("clang").arg("--version").output().ok()?,
        Lang::CSharp => Command::new("dotnet").arg("--version").output().ok()?,
        Lang::Zig => Command::new("zig").arg("version").output().ok()?,
    };

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_version(lang, &stdout)
}

/// Parse version string from command output.
fn parse_version(lang: Lang, output: &str) -> Option<Version> {
    let version_str = match lang {
        // go version go1.24.0 darwin/arm64
        Lang::Go => {
            let parts: Vec<&str> = output.split_whitespace().collect();
            parts.get(2).and_then(|v| v.strip_prefix("go"))?
        }
        // v22.11.0
        Lang::TypeScript => output.trim().strip_prefix('v')?,
        // rustc 1.70.0 (90c541806 2023-05-31)
        Lang::Rust => output.split_whitespace().nth(1)?,
        // Python 3.12.0
        Lang::Python => output.split_whitespace().nth(1)?,
        // Apple clang version 15.0.0 (clang-1500.0.40.1) OR clang version 17.0.1
        Lang::C => {
            let lower = output.to_lowercase();
            if lower.contains("clang version") {
                output.lines().next()?.split("version").nth(1)?.split_whitespace().next()?
            } else {
                return Some(Version::new(0, 0, 0)); // Accept any clang
            }
        }
        // 8.0.418
        Lang::CSharp => output.trim(),
        // 0.15.2
        Lang::Zig => output.trim(),
    };

    parse_semver(version_str)
}

/// Parse a semver-like version string (X.Y.Z) into a Version struct.
fn parse_semver(s: &str) -> Option<Version> {
    let clean = s.split(['-', '+', ' ']).next()?;
    let parts: Vec<&str> = clean.split('.').collect();
    let major = parts.first()?.parse().ok()?;
    let minor = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    Some(Version::new(major, minor, patch))
}

/// Check if an installed version meets the minimum requirement.
pub fn is_version_compatible(lang: Lang, installed: &Version) -> bool {
    let min = minimum_version(lang);
    installed >= &min
}

/// The status of a runtime on the system.
#[derive(Debug, Clone)]
pub enum VersionStatus {
    /// Runtime is installed and meets minimum version requirements.
    Compatible { path: PathBuf, version: Version },
    /// Runtime is installed but below minimum version.
    Incompatible { path: PathBuf, installed: Version, minimum: Version },
    /// Runtime is not installed.
    NotInstalled,
}

impl VersionStatus {
    pub fn is_compatible(&self) -> bool {
        matches!(self, VersionStatus::Compatible { .. })
    }

    pub fn is_installed(&self) -> bool {
        !matches!(self, VersionStatus::NotInstalled)
    }
}

/// Check the runtime status for a language.
///
/// Returns:
/// - `Compatible` if the runtime is installed and meets minimum version
/// - `Incompatible` if installed but below minimum version
/// - `NotInstalled` if not found on PATH
pub fn check_runtime_status(lang: Lang) -> VersionStatus {
    let Some(path) = find_binary_path(lang) else {
        return VersionStatus::NotInstalled;
    };

    let Some(installed) = get_installed_version(lang) else {
        // Can't determine version - assume compatible (e.g., C/clang)
        return VersionStatus::Compatible { path, version: Version::new(0, 0, 0) };
    };

    let minimum = minimum_version(lang);
    if installed >= minimum {
        VersionStatus::Compatible { path, version: installed }
    } else {
        VersionStatus::Incompatible { path, installed, minimum }
    }
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

/// Returns a miette error for when a language version is incompatible.
pub fn incompatible_version_error(
    lang: Lang,
    installed: &Version,
    minimum: &Version,
) -> miette::Report {
    let label = poly_bench_runtime::lang_label(lang);
    miette!(
        "{} version {} is below minimum required version {}. Polybench will install a compatible version.",
        label,
        installed,
        minimum
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_go_version() {
        let output = "go version go1.24.0 darwin/arm64";
        let v = parse_version(Lang::Go, output).unwrap();
        assert_eq!(v, Version::new(1, 24, 0));
    }

    #[test]
    fn test_parse_node_version() {
        let output = "v22.11.0";
        let v = parse_version(Lang::TypeScript, output).unwrap();
        assert_eq!(v, Version::new(22, 11, 0));
    }

    #[test]
    fn test_parse_rust_version() {
        let output = "rustc 1.70.0 (90c541806 2023-05-31)";
        let v = parse_version(Lang::Rust, output).unwrap();
        assert_eq!(v, Version::new(1, 70, 0));
    }

    #[test]
    fn test_parse_python_version() {
        let output = "Python 3.12.0";
        let v = parse_version(Lang::Python, output).unwrap();
        assert_eq!(v, Version::new(3, 12, 0));
    }

    #[test]
    fn test_parse_zig_version() {
        let output = "0.15.2";
        let v = parse_version(Lang::Zig, output).unwrap();
        assert_eq!(v, Version::new(0, 15, 2));
    }

    #[test]
    fn test_parse_dotnet_version() {
        let output = "8.0.418";
        let v = parse_version(Lang::CSharp, output).unwrap();
        assert_eq!(v, Version::new(8, 0, 418));
    }

    #[test]
    fn test_version_compatibility() {
        let installed = Version::new(1, 24, 0);
        assert!(is_version_compatible(Lang::Go, &installed));

        let old = Version::new(1, 20, 0);
        assert!(!is_version_compatible(Lang::Go, &old));
    }

    #[test]
    fn test_parse_semver_with_prerelease() {
        let v = parse_semver("1.24.0-beta.1").unwrap();
        assert_eq!(v, Version::new(1, 24, 0));
    }
}
