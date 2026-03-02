//! Toolchain management for Polybench-managed language runtimes.
//!
//! This module provides:
//! - Cross-platform paths for toolchain installation
//! - Pinned and minimum version constants
//! - Platform detection helpers

use miette::Result;
use poly_bench_dsl::Lang;
use std::path::PathBuf;

/// Pinned versions that Polybench installs when a runtime is missing or incompatible.
pub mod pinned {
    pub const GO: &str = "1.24.0";
    pub const NODE: &str = "22.11.0";
    pub const ZIG: &str = "0.15.2";
    pub const PYTHON: &str = "3.12.0";
    pub const DOTNET: &str = "8.0"; // Fetches latest 8.0.x patch
    pub const RUST: &str = "stable"; // Uses rustup's channel
}

/// Minimum compatible versions - runtimes at or above these are accepted.
pub mod minimum {
    pub const GO: (u32, u32, u32) = (1, 21, 0);
    pub const NODE: (u32, u32, u32) = (22, 0, 0);
    pub const ZIG: (u32, u32, u32) = (0, 15, 2);
    pub const PYTHON: (u32, u32, u32) = (3, 8, 0);
    pub const DOTNET: (u32, u32, u32) = (8, 0, 0);
    pub const RUST: (u32, u32, u32) = (1, 70, 0);
}

/// A semantic version with major.minor.patch components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }

    pub fn from_tuple(t: (u32, u32, u32)) -> Self {
        Self { major: t.0, minor: t.1, patch: t.2 }
    }

    pub fn as_tuple(&self) -> (u32, u32, u32) {
        (self.major, self.minor, self.patch)
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Get the pinned version string for a language.
pub fn pinned_version(lang: Lang) -> &'static str {
    match lang {
        Lang::Go => pinned::GO,
        Lang::TypeScript => pinned::NODE,
        Lang::Zig => pinned::ZIG,
        Lang::Python => pinned::PYTHON,
        Lang::CSharp => pinned::DOTNET,
        Lang::Rust => pinned::RUST,
        Lang::C => panic!("C has no pinned version - manual install required"),
    }
}

/// Get the minimum compatible version for a language.
pub fn minimum_version(lang: Lang) -> Version {
    match lang {
        Lang::Go => Version::from_tuple(minimum::GO),
        Lang::TypeScript => Version::from_tuple(minimum::NODE),
        Lang::Zig => Version::from_tuple(minimum::ZIG),
        Lang::Python => Version::from_tuple(minimum::PYTHON),
        Lang::CSharp => Version::from_tuple(minimum::DOTNET),
        Lang::Rust => Version::from_tuple(minimum::RUST),
        Lang::C => Version::new(0, 0, 0), // C accepts any clang version
    }
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

fn home_dir() -> Result<PathBuf> {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map(PathBuf::from)
        .map_err(|_| miette::miette!("Could not determine home directory"))
}

#[cfg(windows)]
fn local_app_data() -> Result<PathBuf> {
    std::env::var("LOCALAPPDATA")
        .map(PathBuf::from)
        .map_err(|_| miette::miette!("Could not determine LOCALAPPDATA"))
}

/// Returns the base directory for Polybench-managed toolchains.
///
/// - macOS: `~/Library/Application Support/Polybench/`
/// - Linux: `~/.local/share/polybench/`
/// - Windows: `%LOCALAPPDATA%\Polybench\`
pub fn polybench_base_dir() -> Result<PathBuf> {
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
        local_app_data().map(|l| l.join("Polybench"))
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
    {
        home_dir().map(|h| h.join(".polybench"))
    }
}

/// Returns the directory where toolchains are installed.
///
/// Layout: `<base>/toolchains/<lang>/<version>/<platform>/`
pub fn toolchains_dir() -> Result<PathBuf> {
    polybench_base_dir().map(|b| b.join("toolchains"))
}

/// Returns the full path to a specific toolchain installation.
///
/// Example: `~/Library/Application Support/Polybench/toolchains/go/1.24.0/darwin-arm64/`
pub fn toolchain_path(lang: Lang, version: &str) -> Result<PathBuf> {
    let platform = Platform::current();
    toolchains_dir().map(|t| t.join(lang.as_str()).join(version).join(platform.as_str()))
}

/// Returns the path to the binary within a toolchain installation.
pub fn toolchain_binary_path(lang: Lang, version: &str) -> Result<PathBuf> {
    let base = toolchain_path(lang, version)?;
    Ok(match lang {
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
        Lang::C => panic!("C has no toolchain binary"),
    })
}

/// Check if the Polybench-managed toolchain for a language is already installed.
pub fn is_polybench_toolchain_installed(lang: Lang) -> bool {
    let version = pinned_version(lang);
    toolchain_binary_path(lang, version).map(|p| p.exists()).unwrap_or(false)
}

/// Returns the directory where Polybench shims are installed.
///
/// These are small binaries that resolve to the correct toolchain version.
pub fn shim_dir() -> Result<PathBuf> {
    polybench_base_dir().map(|b| b.join("bin"))
}

/// Returns the path to a specific shim binary.
pub fn shim_path(lang: Lang) -> Result<PathBuf> {
    let binary_name = match lang {
        Lang::Go => "go",
        Lang::TypeScript => "node",
        Lang::Zig => "zig",
        Lang::Python => "python3",
        Lang::Rust => "cargo",
        Lang::CSharp => "dotnet",
        Lang::C => panic!("C has no shim"),
    };
    let name = if cfg!(windows) { format!("{}.exe", binary_name) } else { binary_name.to_string() };
    shim_dir().map(|d| d.join(name))
}

/// Returns the path to the shim config file.
pub fn shim_config_path(lang: Lang) -> Result<PathBuf> {
    shim_dir().map(|d| d.join(format!("{}.toml", lang.as_str())))
}

/// Returns the directory for user-global symlinks.
///
/// - macOS/Linux: `~/.local/bin/`
/// - Windows: Uses the shim_dir directly (added to PATH)
pub fn global_bin_dir() -> Result<PathBuf> {
    #[cfg(windows)]
    {
        shim_dir()
    }
    #[cfg(not(windows))]
    {
        home_dir().map(|h| h.join(".local").join("bin"))
    }
}

/// Returns the path to a global symlink for a language binary.
pub fn global_symlink_path(lang: Lang) -> Result<PathBuf> {
    let binary_name = match lang {
        Lang::Go => "go",
        Lang::TypeScript => "node",
        Lang::Zig => "zig",
        Lang::Python => "python3",
        Lang::Rust => "cargo",
        Lang::CSharp => "dotnet",
        Lang::C => panic!("C has no global symlink"),
    };
    global_bin_dir().map(|d| d.join(binary_name))
}

/// Check if `~/.local/bin` is in the user's PATH.
pub fn is_global_bin_in_path() -> bool {
    if let Ok(path) = std::env::var("PATH") {
        if let Ok(global) = global_bin_dir() {
            let global_str = global.to_string_lossy();
            return path.split(if cfg!(windows) { ';' } else { ':' }).any(|p| p == global_str);
        }
    }
    false
}

/// Returns the shell config file path based on $SHELL.
pub fn shell_config_path() -> Option<PathBuf> {
    let home = home_dir().ok()?;
    let shell = std::env::var("SHELL").unwrap_or_default();
    let shell_lower = shell.to_lowercase();
    if shell_lower.contains("fish") {
        Some(home.join(".config").join("fish").join("config.fish"))
    } else if shell_lower.contains("zsh") {
        Some(home.join(".zshrc"))
    } else if shell_lower.contains("bash") {
        #[cfg(target_os = "macos")]
        if home.join(".bash_profile").exists() {
            return Some(home.join(".bash_profile"));
        }
        Some(home.join(".bashrc"))
    } else {
        Some(home.join(".profile"))
    }
}

/// Returns the export command to add `~/.local/bin` to PATH.
pub fn path_export_command() -> String {
    if let Some(config) = shell_config_path() {
        if config.extension().and_then(|e| e.to_str()) == Some("fish") {
            return "fish_add_path --global ~/.local/bin".to_string();
        }
    }
    "export PATH=\"$HOME/.local/bin:$PATH\"".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_comparison() {
        let v1 = Version::new(1, 21, 0);
        let v2 = Version::new(1, 24, 0);
        let v3 = Version::new(2, 0, 0);
        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v1 < v3);
    }

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
    fn test_minimum_versions() {
        assert_eq!(minimum_version(Lang::Go).as_tuple(), (1, 21, 0));
        assert_eq!(minimum_version(Lang::Zig).as_tuple(), (0, 15, 2));
        assert_eq!(minimum_version(Lang::TypeScript).as_tuple(), (22, 0, 0));
    }
}
