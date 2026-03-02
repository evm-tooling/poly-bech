//! Rust-based shim system for Polybench-managed toolchains.
//!
//! Shims are small executables that resolve to the correct toolchain binary.
//! They read a config file to determine whether to use the system binary or
//! a Polybench-managed toolchain.

use crate::toolchain::{
    global_bin_dir, global_symlink_path, is_global_bin_in_path, path_export_command,
    pinned_version, shim_config_path, shim_dir, shim_path,
};
use miette::{miette, Result};
use poly_bench_dsl::Lang;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

/// Configuration for a shim, stored as TOML.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShimConfig {
    pub lang: String,
    pub toolchain_path: PathBuf,
    pub version: String,
}

impl ShimConfig {
    pub fn new(lang: Lang, toolchain_path: PathBuf, version: &str) -> Self {
        Self { lang: lang.as_str().to_string(), toolchain_path, version: version.to_string() }
    }

    /// Load a shim config from disk.
    pub fn load(lang: Lang) -> Result<Self> {
        let config_path = shim_config_path(lang)?;
        let content = fs::read_to_string(&config_path).map_err(|e| {
            miette!("Failed to read shim config at {}: {}", config_path.display(), e)
        })?;
        toml::from_str(&content)
            .map_err(|e| miette!("Failed to parse shim config at {}: {}", config_path.display(), e))
    }

    /// Save a shim config to disk.
    pub fn save(&self, lang: Lang) -> Result<()> {
        let config_path = shim_config_path(lang)?;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| miette!("Failed to create shim config dir: {}", e))?;
        }
        let content = toml::to_string_pretty(self)
            .map_err(|e| miette!("Failed to serialize shim config: {}", e))?;
        fs::write(&config_path, content).map_err(|e| {
            miette!("Failed to write shim config to {}: {}", config_path.display(), e)
        })?;
        Ok(())
    }
}

/// Generate the Rust source code for a shim binary.
///
/// The shim reads its config file and execs the resolved binary.
fn generate_shim_source(lang: Lang) -> String {
    let binary_name = match lang {
        Lang::Go => "go",
        Lang::TypeScript => "node",
        Lang::Zig => "zig",
        Lang::Python => "python3",
        Lang::Rust => "cargo",
        Lang::CSharp => "dotnet",
        Lang::C => panic!("C has no shim"),
    };

    format!(
        r##"//! Auto-generated shim for {binary_name}
use std::env;
use std::fs;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;

fn main() {{
    let exe_path = env::current_exe().expect("Failed to get current exe path");
    let exe_dir = exe_path.parent().expect("Failed to get exe directory");
    let config_path = exe_dir.join("{lang}.toml");

    let config_content = fs::read_to_string(&config_path)
        .unwrap_or_else(|_| panic!("Failed to read shim config at {{}}", config_path.display()));

    let toolchain_path: PathBuf = config_content
        .lines()
        .find(|line| line.starts_with("toolchain_path"))
        .and_then(|line| line.split('=').nth(1))
        .map(|s| s.trim().trim_matches('"'))
        .map(PathBuf::from)
        .expect("Failed to parse toolchain_path from config");

    if !toolchain_path.exists() {{
        eprintln!("Error: Toolchain binary not found at {{}}", toolchain_path.display());
        eprintln!("Run 'poly-bench add-runtime {lang}' to reinstall.");
        std::process::exit(1);
    }}

    let args: Vec<String> = env::args().skip(1).collect();
    let err = Command::new(&toolchain_path).args(&args).exec();
    eprintln!("Failed to exec {{}}: {{}}", toolchain_path.display(), err);
    std::process::exit(1);
}}
"##,
        binary_name = binary_name,
        lang = lang.as_str()
    )
}

/// Generate a cross-platform shim source (works on Windows too).
#[cfg(windows)]
fn generate_shim_source_windows(lang: Lang) -> String {
    let binary_name = match lang {
        Lang::Go => "go",
        Lang::TypeScript => "node",
        Lang::Zig => "zig",
        Lang::Python => "python3",
        Lang::Rust => "cargo",
        Lang::CSharp => "dotnet",
        Lang::C => panic!("C has no shim"),
    };

    format!(
        r##"//! Auto-generated shim for {binary_name}
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {{
    let exe_path = env::current_exe().expect("Failed to get current exe path");
    let exe_dir = exe_path.parent().expect("Failed to get exe directory");
    let config_path = exe_dir.join("{lang}.toml");

    let config_content = fs::read_to_string(&config_path)
        .unwrap_or_else(|_| panic!("Failed to read shim config at {{}}", config_path.display()));

    let toolchain_path: PathBuf = config_content
        .lines()
        .find(|line| line.starts_with("toolchain_path"))
        .and_then(|line| line.split('=').nth(1))
        .map(|s| s.trim().trim_matches('"'))
        .map(PathBuf::from)
        .expect("Failed to parse toolchain_path from config");

    if !toolchain_path.exists() {{
        eprintln!("Error: Toolchain binary not found at {{}}", toolchain_path.display());
        eprintln!("Run 'poly-bench add-runtime {lang}' to reinstall.");
        std::process::exit(1);
    }}

    let args: Vec<String> = env::args().skip(1).collect();
    let status = Command::new(&toolchain_path)
        .args(&args)
        .status()
        .unwrap_or_else(|e| {{
            eprintln!("Failed to run {{}}: {{}}", toolchain_path.display(), e);
            std::process::exit(1);
        }});

    std::process::exit(status.code().unwrap_or(1));
}}
"##,
        binary_name = binary_name,
        lang = lang.as_str()
    )
}

/// Create a shim for a language by compiling a small Rust binary.
///
/// This requires `rustc` to be available (which it will be if Rust is installed).
pub fn create_shim(lang: Lang, toolchain_binary: &PathBuf) -> Result<PathBuf> {
    if lang == Lang::C {
        return Err(miette!("C does not use shims"));
    }

    let shim_binary_path = shim_path(lang)?;
    let shim_directory = shim_dir()?;

    fs::create_dir_all(&shim_directory)
        .map_err(|e| miette!("Failed to create shim directory: {}", e))?;

    // Write the shim config
    let version = pinned_version(lang);
    let config = ShimConfig::new(lang, toolchain_binary.clone(), version);
    config.save(lang)?;

    // Generate and compile the shim
    let source = if cfg!(windows) {
        #[cfg(windows)]
        {
            generate_shim_source_windows(lang)
        }
        #[cfg(not(windows))]
        {
            generate_shim_source(lang)
        }
    } else {
        generate_shim_source(lang)
    };

    let temp_dir = std::env::temp_dir().join("polybench-shims");
    fs::create_dir_all(&temp_dir).map_err(|e| miette!("Failed to create temp dir: {}", e))?;

    let source_path = temp_dir.join(format!("{}_shim.rs", lang.as_str()));
    fs::write(&source_path, &source).map_err(|e| miette!("Failed to write shim source: {}", e))?;

    // Compile with rustc
    let status = std::process::Command::new("rustc")
        .args(["-O", "-o", shim_binary_path.to_str().unwrap(), source_path.to_str().unwrap()])
        .status()
        .map_err(|e| miette!("Failed to run rustc: {}", e))?;

    if !status.success() {
        return Err(miette!("Failed to compile shim for {}", lang.as_str()));
    }

    // Clean up temp source
    let _ = fs::remove_file(&source_path);

    Ok(shim_binary_path)
}

/// Create a simple shell wrapper shim (fallback if rustc is not available).
#[cfg(not(windows))]
pub fn create_shell_shim(lang: Lang, toolchain_binary: &PathBuf) -> Result<PathBuf> {
    use std::os::unix::fs::PermissionsExt;

    if lang == Lang::C {
        return Err(miette!("C does not use shims"));
    }

    let shim_binary_path = shim_path(lang)?;
    let shim_directory = shim_dir()?;

    fs::create_dir_all(&shim_directory)
        .map_err(|e| miette!("Failed to create shim directory: {}", e))?;

    // Write the shim config
    let version = pinned_version(lang);
    let config = ShimConfig::new(lang, toolchain_binary.clone(), version);
    config.save(lang)?;

    // Create a shell wrapper
    let script = format!(
        r#"#!/bin/sh
exec "{}" "$@"
"#,
        toolchain_binary.display()
    );

    fs::write(&shim_binary_path, script)
        .map_err(|e| miette!("Failed to write shell shim: {}", e))?;

    // Make executable
    let mut perms = fs::metadata(&shim_binary_path)
        .map_err(|e| miette!("Failed to read shim metadata: {}", e))?
        .permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&shim_binary_path, perms)
        .map_err(|e| miette!("Failed to set shim permissions: {}", e))?;

    Ok(shim_binary_path)
}

/// Create a global symlink in ~/.local/bin pointing to the shim.
///
/// This allows users who don't have the runtime installed to use it globally.
#[cfg(not(windows))]
pub fn create_global_symlink(lang: Lang) -> Result<Option<PathBuf>> {
    use std::os::unix::fs::symlink;

    if lang == Lang::C {
        return Ok(None);
    }

    let shim_binary = shim_path(lang)?;
    if !shim_binary.exists() {
        return Err(miette!(
            "Shim does not exist at {}. Create the shim first.",
            shim_binary.display()
        ));
    }

    let global_dir = global_bin_dir()?;
    fs::create_dir_all(&global_dir)
        .map_err(|e| miette!("Failed to create global bin dir: {}", e))?;

    let symlink_path = global_symlink_path(lang)?;

    // Remove existing symlink if it exists
    if symlink_path.exists() || symlink_path.is_symlink() {
        fs::remove_file(&symlink_path)
            .map_err(|e| miette!("Failed to remove existing symlink: {}", e))?;
    }

    symlink(&shim_binary, &symlink_path).map_err(|e| miette!("Failed to create symlink: {}", e))?;

    Ok(Some(symlink_path))
}

#[cfg(windows)]
pub fn create_global_symlink(lang: Lang) -> Result<Option<PathBuf>> {
    // On Windows, the shim_dir is already the global bin dir, so no symlink needed
    Ok(None)
}

/// Check if the global bin directory needs to be added to PATH.
pub fn needs_path_setup() -> bool {
    !is_global_bin_in_path()
}

/// Get the command to add global bin to PATH.
pub fn get_path_setup_command() -> String {
    path_export_command()
}

/// Remove a shim and its config.
pub fn remove_shim(lang: Lang) -> Result<()> {
    let shim_binary = shim_path(lang)?;
    let config = shim_config_path(lang)?;

    if shim_binary.exists() {
        fs::remove_file(&shim_binary).map_err(|e| miette!("Failed to remove shim: {}", e))?;
    }

    if config.exists() {
        fs::remove_file(&config).map_err(|e| miette!("Failed to remove shim config: {}", e))?;
    }

    Ok(())
}

/// Remove the global symlink for a language.
#[cfg(not(windows))]
pub fn remove_global_symlink(lang: Lang) -> Result<()> {
    let symlink_path = global_symlink_path(lang)?;
    if symlink_path.exists() || symlink_path.is_symlink() {
        fs::remove_file(&symlink_path)
            .map_err(|e| miette!("Failed to remove global symlink: {}", e))?;
    }
    Ok(())
}

#[cfg(windows)]
pub fn remove_global_symlink(_lang: Lang) -> Result<()> {
    Ok(())
}

/// Attempt to create a compiled Rust shim, falling back to shell shim if rustc unavailable.
pub fn create_shim_with_fallback(lang: Lang, toolchain_binary: &PathBuf) -> Result<PathBuf> {
    // Check if rustc is available
    let has_rustc = std::process::Command::new("rustc")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if has_rustc {
        create_shim(lang, toolchain_binary)
    } else {
        #[cfg(not(windows))]
        {
            create_shell_shim(lang, toolchain_binary)
        }
        #[cfg(windows)]
        {
            Err(miette!("rustc is required to create shims on Windows. Install Rust first."))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shim_config_serialization() {
        let config = ShimConfig::new(Lang::Go, PathBuf::from("/path/to/go"), "1.24.0");
        let serialized = toml::to_string_pretty(&config).unwrap();
        assert!(serialized.contains("lang = \"go\""));
        assert!(serialized.contains("version = \"1.24.0\""));
    }

    #[test]
    fn test_generate_shim_source() {
        let source = generate_shim_source(Lang::Go);
        assert!(source.contains("go.toml"));
        assert!(source.contains("toolchain_path"));
    }
}
