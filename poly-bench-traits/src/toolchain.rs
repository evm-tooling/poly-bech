//! Toolchain resolution for Polybench-managed language runtimes.
//!
//! This module provides utilities to locate the correct binary for each language,
//! preferring Polybench-managed toolchains over system-installed versions.

use miette::Result;
use poly_bench_dsl::Lang;
use std::path::PathBuf;

/// Pinned versions that Polybench manages.
pub mod pinned {
    pub const GO: &str = "1.24.0";
    pub const NODE: &str = "22.11.0";
    pub const ZIG: &str = "0.15.2";
    pub const PYTHON: &str = "3.12.0";
    pub const DOTNET: &str = "8.0";
    pub const RUST: &str = "stable";
}

/// Platform identifier (os-arch).
#[derive(Debug, Clone)]
pub struct Platform {
    pub os: String,
    pub arch: String,
}

impl Platform {
    /// Detect the current platform.
    pub fn current() -> Self {
        let arch = match std::env::consts::ARCH {
            "x86_64" => "amd64",
            "aarch64" | "arm64" => "arm64",
            other => other,
        };
        let os = match std::env::consts::OS {
            "macos" => "darwin",
            other => other,
        };
        Self { os: os.to_string(), arch: arch.to_string() }
    }

    /// Returns the platform string (e.g., "darwin-arm64").
    pub fn as_str(&self) -> String {
        format!("{}-{}", self.os, self.arch)
    }
}

fn home_dir() -> Option<PathBuf> {
    std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")).map(PathBuf::from).ok()
}

/// Returns the base directory for Polybench-managed toolchains.
///
/// - macOS: `~/Library/Application Support/Polybench/`
/// - Linux: `~/.local/share/polybench/`
/// - Windows: `%LOCALAPPDATA%\Polybench\`
pub fn polybench_base_dir() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        home_dir().map(|h| h.join("Library").join("Application Support").join("Polybench"))
    }
    #[cfg(target_os = "linux")]
    {
        home_dir().map(|h| h.join(".local").join("share").join("polybench"))
    }
    #[cfg(windows)]
    {
        std::env::var("LOCALAPPDATA").ok().map(|l| PathBuf::from(l).join("Polybench"))
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
    {
        home_dir().map(|h| h.join(".polybench"))
    }
}

/// Returns the directory where toolchains are installed.
pub fn toolchains_dir() -> Option<PathBuf> {
    polybench_base_dir().map(|b| b.join("toolchains"))
}

/// Get the pinned version for a language.
pub fn pinned_version(lang: Lang) -> &'static str {
    match lang {
        Lang::Go => pinned::GO,
        Lang::TypeScript => pinned::NODE,
        Lang::Zig => pinned::ZIG,
        Lang::Python => pinned::PYTHON,
        Lang::CSharp => pinned::DOTNET,
        Lang::Rust => pinned::RUST,
        Lang::C => "system", // C uses system clang
    }
}

/// Returns the path to a specific toolchain installation.
pub fn toolchain_path(lang: Lang, version: &str) -> Option<PathBuf> {
    let platform = Platform::current();
    toolchains_dir().map(|t| t.join(lang.as_str()).join(version).join(platform.as_str()))
}

/// Returns the path to the binary within a toolchain installation.
pub fn toolchain_binary_path(lang: Lang, version: &str) -> Option<PathBuf> {
    let base = toolchain_path(lang, version)?;
    Some(match lang {
        Lang::Go => base.join("go").join("bin").join("go"),
        Lang::TypeScript => base.join("bin").join("node"),
        Lang::Zig => base.join("zig"),
        Lang::Python => {
            if cfg!(windows) {
                base.join("install").join("python.exe")
            } else {
                base.join("install").join("bin").join("python3")
            }
        }
        Lang::Rust => base.join("bin").join("cargo"),
        Lang::CSharp => base.join("dotnet"),
        Lang::C => return None, // C uses system clang
    })
}

/// Check if the Polybench-managed toolchain for a language is installed.
pub fn is_polybench_toolchain_installed(lang: Lang) -> bool {
    let version = pinned_version(lang);
    toolchain_binary_path(lang, version).map(|p| p.exists()).unwrap_or(false)
}

/// Resolve the binary path for a language, preferring Polybench-managed toolchains.
///
/// Resolution order:
/// 1. Polybench-managed toolchain at `~/Library/Application Support/Polybench/toolchains/<lang>/`
/// 2. System PATH (via `which`)
///
/// Returns the path to the binary and whether it's from Polybench-managed toolchain.
pub fn resolve_binary(lang: Lang) -> Result<(PathBuf, bool)> {
    let version = pinned_version(lang);

    // First, check if Polybench-managed toolchain exists
    if let Some(polybench_path) = toolchain_binary_path(lang, version) {
        if polybench_path.exists() {
            return Ok((polybench_path, true));
        }
    }

    // Fall back to system PATH
    let binary_name = match lang {
        Lang::Go => "go",
        Lang::TypeScript => "node",
        Lang::Zig => "zig",
        Lang::Python => "python3",
        Lang::Rust => "cargo",
        Lang::CSharp => "dotnet",
        Lang::C => "clang",
    };

    // For Python, try python3 first, then python
    if lang == Lang::Python {
        if let Ok(path) = which::which("python3") {
            return Ok((path, false));
        }
        if let Ok(path) = which::which("python") {
            return Ok((path, false));
        }
        return Err(miette::miette!("{} not found in PATH", binary_name));
    }

    which::which(binary_name)
        .map(|p| (p, false))
        .map_err(|_| miette::miette!("{} not found in PATH", binary_name))
}

/// Resolve the binary path for a language, returning only the path.
pub fn resolve_binary_path(lang: Lang) -> Result<PathBuf> {
    resolve_binary(lang).map(|(path, _)| path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_detection() {
        let platform = Platform::current();
        assert!(!platform.os.is_empty());
        assert!(!platform.arch.is_empty());
    }

    #[test]
    fn test_pinned_versions() {
        assert_eq!(pinned_version(Lang::Go), "1.24.0");
        assert_eq!(pinned_version(Lang::Zig), "0.15.2");
        assert_eq!(pinned_version(Lang::TypeScript), "22.11.0");
    }

    #[test]
    fn test_polybench_base_dir() {
        let base = polybench_base_dir();
        assert!(base.is_some());
    }
}
