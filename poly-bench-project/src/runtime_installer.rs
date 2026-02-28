//! Runtime installer: download and install language runtimes (Go, Node, Rust, etc.)
//!
//! Installs to each language's canonical locations (same as manual install):
//! - Rust: ~/.cargo, ~/.rustup (rustup defaults)
//! - .NET: ~/.dotnet or /usr/local/share/dotnet
//! - Go: ~/sdk/go or /usr/local/go
//! - Node: ~/.local/share/node or /usr/local
//! - Python: ~/.local/share/python or /usr/local
//! - Zig: ~/.local/share/zig or /usr/local

use crate::{runtime_check, terminal};
use flate2::read::GzDecoder;
use miette::Result;
use poly_bench_dsl::Lang;
use std::{
    env, fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
};

fn home_dir() -> Result<PathBuf> {
    env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .map(PathBuf::from)
        .map_err(|_| miette::miette!("Could not determine home directory (HOME or USERPROFILE)"))
}

#[cfg(windows)]
fn local_share() -> Result<PathBuf> {
    let local = env::var("LOCALAPPDATA")
        .or_else(|_| env::var("USERPROFILE").map(|p| format!("{}\\AppData\\Local", p)))
        .map_err(|_| miette::miette!("Could not determine LOCALAPPDATA"))?;
    Ok(PathBuf::from(local))
}

/// Install root for a language. Extraction happens here; tarball structure may add subdirs.
fn lang_install_dir(lang: Lang, location: InstallLocation) -> Result<PathBuf> {
    match (lang, location) {
        (Lang::Rust, _) => {
            // Rust uses rustup defaults (~/.cargo, ~/.rustup) - we don't control the dir
            home_dir().map(|h| h.join(".cargo"))
        }
        (Lang::Go, InstallLocation::UserLocal) => home_dir().map(|h| h.join("sdk")),
        (Lang::Go, InstallLocation::System) => Ok(PathBuf::from("/usr/local")),
        (Lang::TypeScript, InstallLocation::UserLocal) => {
            #[cfg(windows)]
            {
                local_share().map(|l| l.join("node"))
            }
            #[cfg(not(windows))]
            {
                home_dir().map(|h| h.join(".local").join("share").join("node"))
            }
        }
        (Lang::TypeScript, InstallLocation::System) => Ok(PathBuf::from("/usr/local")),
        (Lang::Python, InstallLocation::UserLocal) => {
            #[cfg(windows)]
            {
                local_share().map(|l| l.join("python"))
            }
            #[cfg(not(windows))]
            {
                home_dir().map(|h| h.join(".local").join("share").join("python"))
            }
        }
        (Lang::Python, InstallLocation::System) => Ok(PathBuf::from("/usr/local")),
        (Lang::Zig, InstallLocation::UserLocal) => {
            #[cfg(windows)]
            {
                local_share().map(|l| l.join("zig"))
            }
            #[cfg(not(windows))]
            {
                home_dir().map(|h| h.join(".local").join("share").join("zig"))
            }
        }
        (Lang::Zig, InstallLocation::System) => Ok(PathBuf::from("/usr/local")),
        (Lang::CSharp, InstallLocation::UserLocal) => home_dir().map(|h| h.join(".dotnet")),
        (Lang::CSharp, InstallLocation::System) => Ok(PathBuf::from("/usr/local/share/dotnet")),
        (Lang::C, _) => Err(miette::miette!("C is not auto-installed")),
    }
}

/// Where to install: user-local (no sudo) or system (/usr/local, requires sudo).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InstallLocation {
    /// User-writable standard paths (e.g. ~/sdk/go, ~/.dotnet)
    UserLocal,
    /// System paths (e.g. /usr/local/go) - requires sudo
    System,
}

/// Returns the default install root for a language (for display in prompts).
pub fn default_install_path(lang: Lang, location: InstallLocation) -> Result<PathBuf> {
    lang_install_dir(lang, location)
}

/// Returns true if poly-bench can auto-install this language.
pub fn can_auto_install(lang: Lang) -> bool {
    lang != Lang::C
}

/// Returns true if this language supports interactive version selection (fetch + pick from list).
pub fn supports_version_selection(lang: Lang) -> bool {
    matches!(lang, Lang::Go | Lang::TypeScript | Lang::Zig | Lang::CSharp)
}

/// Fetches available versions for a language from official APIs. Returns up to 5 versions.
/// Returns error on network failure or parse error.
pub fn fetch_available_versions(lang: Lang) -> Result<Vec<String>> {
    match lang {
        Lang::Go => fetch_go_versions(),
        Lang::TypeScript => fetch_node_versions(),
        Lang::Zig => fetch_zig_versions(),
        Lang::CSharp => fetch_dotnet_versions(),
        _ => Err(miette::miette!(
            "Version selection not supported for {}",
            poly_bench_runtime::lang_label(lang)
        )),
    }
}

fn fetch_go_versions() -> Result<Vec<String>> {
    let url = "https://go.dev/dl/?mode=json";
    let resp: Vec<serde_json::Value> = ureq::get(url)
        .call()
        .map_err(|e| miette::miette!("Failed to fetch Go versions: {}", e))?
        .body_mut()
        .read_json()
        .map_err(|e| miette::miette!("Failed to parse Go versions: {}", e))?;

    let versions: Vec<String> = resp
        .into_iter()
        .filter_map(|v: serde_json::Value| {
            let stable = v.get("stable").and_then(|s| s.as_bool()).unwrap_or(false);
            if !stable {
                return None;
            }
            v.get("version").and_then(|s: &serde_json::Value| s.as_str()).map(|s| s.to_string())
        })
        .take(5)
        .collect();

    if versions.is_empty() {
        return Err(miette::miette!("No stable Go versions found"));
    }
    Ok(versions)
}

fn fetch_node_versions() -> Result<Vec<String>> {
    let url = "https://nodejs.org/dist/index.json";
    let resp: Vec<serde_json::Value> = ureq::get(url)
        .call()
        .map_err(|e| miette::miette!("Failed to fetch Node.js versions: {}", e))?
        .body_mut()
        .read_json()
        .map_err(|e| miette::miette!("Failed to parse Node.js versions: {}", e))?;

    let versions: Vec<String> = resp
        .into_iter()
        .filter_map(|v: serde_json::Value| {
            v.get("version").and_then(|s: &serde_json::Value| s.as_str()).map(|s| s.to_string())
        })
        .take(5)
        .collect();

    if versions.is_empty() {
        return Err(miette::miette!("No Node.js versions found"));
    }
    Ok(versions)
}

fn fetch_zig_versions() -> Result<Vec<String>> {
    let url = "https://ziglang.org/download/index.json";
    let resp: serde_json::Value = ureq::get(url)
        .call()
        .map_err(|e| miette::miette!("Failed to fetch Zig versions: {}", e))?
        .body_mut()
        .read_json()
        .map_err(|e| miette::miette!("Failed to parse Zig versions: {}", e))?;

    let obj = resp.as_object().ok_or_else(|| miette::miette!("Zig index is not an object"))?;

    let mut versions: Vec<String> = obj
        .keys()
        .filter(|k: &&String| {
            let k = k.as_str();
            k != "master" &&
                !k.contains("-dev") &&
                k.chars().next().map(|c: char| c.is_ascii_digit()).unwrap_or(false)
        })
        .cloned()
        .collect();

    versions.sort_by(|a, b| semver_compare_zig(a, b).unwrap_or(std::cmp::Ordering::Equal));
    versions.reverse();
    versions.truncate(5);

    if versions.is_empty() {
        return Err(miette::miette!("No Zig versions found"));
    }
    Ok(versions)
}

/// Get Zig download URL from index.json. Zig uses different filename formats across
/// versions (e.g. zig-macos-aarch64-0.13.0 vs zig-aarch64-macos-0.15.2), so we must
/// use the URL from the index instead of constructing it.
fn zig_download_url(version: &str) -> Result<String> {
    let (arch, os) = platform();
    let platform_key = match (os.as_str(), arch.as_str()) {
        ("darwin", "arm64") => "aarch64-macos",
        ("darwin", "amd64") => "x86_64-macos",
        ("linux", "amd64") => "x86_64-linux",
        ("linux", "arm64") => "aarch64-linux",
        ("windows", "amd64") => "x86_64-windows",
        ("windows", "arm64") => "aarch64-windows",
        _ => "x86_64-linux",
    };

    let url = "https://ziglang.org/download/index.json";
    let resp: serde_json::Value = ureq::get(url)
        .call()
        .map_err(|e| miette::miette!("Failed to fetch Zig index: {}", e))?
        .body_mut()
        .read_json()
        .map_err(|e| miette::miette!("Failed to parse Zig index: {}", e))?;

    let tarball = resp
        .get(version)
        .and_then(|v| v.get(platform_key))
        .and_then(|v| v.get("tarball"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            miette::miette!(
                "Zig {} not available for {}-{}",
                version,
                arch,
                os
            )
        })?;

    Ok(tarball.to_string())
}

fn semver_compare_zig(a: &str, b: &str) -> Option<std::cmp::Ordering> {
    let parse = |s: &str| {
        let parts: Vec<u32> = s.split('.').filter_map(|p| p.parse::<u32>().ok()).collect();
        (
            parts.get(0).copied().unwrap_or(0),
            parts.get(1).copied().unwrap_or(0),
            parts.get(2).copied().unwrap_or(0),
        )
    };
    let va = parse(a);
    let vb = parse(b);
    Some(va.cmp(&vb))
}

fn fetch_dotnet_versions() -> Result<Vec<String>> {
    let url = "https://dotnetcli.blob.core.windows.net/dotnet/release-metadata/releases-index.json";
    let resp: serde_json::Value = ureq::get(url)
        .call()
        .map_err(|e| miette::miette!("Failed to fetch .NET versions: {}", e))?
        .body_mut()
        .read_json()
        .map_err(|e| miette::miette!("Failed to parse .NET versions: {}", e))?;

    let index = resp
        .get("releases-index")
        .and_then(|v: &serde_json::Value| v.as_array())
        .ok_or_else(|| miette::miette!("Invalid .NET releases index"))?;

    let versions: Vec<String> = index
        .iter()
        .filter_map(|v: &serde_json::Value| {
            v.get("latest-sdk").and_then(|s: &serde_json::Value| s.as_str()).map(|s| s.to_string())
        })
        .take(5)
        .collect();

    if versions.is_empty() {
        return Err(miette::miette!("No .NET SDK versions found"));
    }
    Ok(versions)
}

/// Returns the bin directory to prepend to PATH when the binary is not on PATH but exists in a
/// standard location. Returns None if already on PATH.
pub fn lang_bin_path_for_prepend(lang: Lang) -> Option<PathBuf> {
    for bin in runtime_check::required_binary(lang) {
        if which::which(bin).is_ok() {
            return None;
        }
    }
    // Check standard locations
    let home = home_dir().ok()?;
    #[cfg(not(windows))]
    let local_share = home.join(".local").join("share");
    #[cfg(windows)]
    let local_share = env::var("LOCALAPPDATA").ok().map(PathBuf::from)?;

    let candidates: Vec<PathBuf> = match lang {
        Lang::Rust => vec![home.join(".cargo").join("bin")],
        Lang::CSharp => vec![home.join(".dotnet"), PathBuf::from("/usr/local/share/dotnet")],
        Lang::Go => {
            vec![home.join("sdk").join("go").join("bin"), PathBuf::from("/usr/local/go/bin")]
        }
        Lang::TypeScript => {
            let node_base =
                if cfg!(windows) { local_share.join("node") } else { local_share.join("node") };
            let mut paths = Vec::new();
            if let Ok(entries) = fs::read_dir(&node_base) {
                for e in entries.filter_map(|e| e.ok()) {
                    let p = e.path().join("bin");
                    if p.exists() {
                        paths.push(p);
                    }
                }
            }
            paths.push(PathBuf::from("/usr/local/bin"));
            paths
        }
        Lang::Python => {
            let py_base =
                if cfg!(windows) { local_share.join("python") } else { local_share.join("python") };
            vec![
                py_base.join("install").join("bin"),
                py_base.join("install"),
                PathBuf::from("/usr/local/bin"),
            ]
        }
        Lang::Zig => {
            let zig_base =
                if cfg!(windows) { local_share.join("zig") } else { local_share.join("zig") };
            let mut paths = Vec::new();
            if let Ok(entries) = fs::read_dir(&zig_base) {
                for e in entries.filter_map(|e| e.ok()) {
                    let p = e.path();
                    if p.join("zig").exists() || p.join("zig.exe").exists() {
                        paths.push(p);
                    }
                }
            }
            paths.push(PathBuf::from("/usr/local/bin"));
            paths
        }
        Lang::C => return None,
    };

    for p in candidates {
        if p.exists() {
            let has_binary = match lang {
                Lang::Go => p.join("go").exists(),
                Lang::TypeScript => p.join("node").exists(),
                Lang::Rust => p.join("cargo").exists(),
                Lang::Python => p.join("python3").exists() || p.join("python").exists(),
                Lang::Zig => p.join("zig").exists() || p.join("zig.exe").exists(),
                Lang::CSharp => p.join("dotnet").exists(),
                Lang::C => false,
            };
            if has_binary {
                return Some(p);
            }
        }
    }
    None
}

/// Backward compatibility: same as lang_bin_path_for_prepend.
pub fn polybench_runtime_path(lang: Lang) -> Option<PathBuf> {
    lang_bin_path_for_prepend(lang)
}

/// Returns the shell config file path based on $SHELL. None if we can't determine it.
fn shell_config_path() -> Option<PathBuf> {
    let home = home_dir().ok()?;
    let shell = env::var("SHELL").unwrap_or_default();
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

/// Appends the given bin directory to PATH in the user's shell config. Returns None if the path
/// is already present or we couldn't write. Returns Some(config_path) if we appended.
fn append_path_to_shell_config(bin_dir: &Path) -> Result<Option<PathBuf>> {
    let config_path = match shell_config_path() {
        Some(p) => p,
        None => return Ok(None),
    };

    let bin_str = bin_dir.to_string_lossy();
    let content = fs::read_to_string(&config_path).unwrap_or_default();

    // Skip if this path is already in the config
    if content.contains(bin_str.as_ref()) {
        return Ok(None);
    }

    let line = if config_path.extension().and_then(|e| e.to_str()) == Some("fish") {
        format!(
            "\n# Added by poly-bench for runtime PATH\nfish_add_path --global \"{}\"\n",
            bin_str
        )
    } else {
        format!("\n# Added by poly-bench for runtime PATH\nexport PATH=\"{}\":$PATH\n", bin_str)
    };

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            miette::miette!("Failed to create config directory {}: {}", parent.display(), e)
        })?;
    }

    let mut f = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&config_path)
        .map_err(|e| miette::miette!("Failed to open {}: {}", config_path.display(), e))?;

    f.write_all(line.as_bytes())
        .map_err(|e| miette::miette!("Failed to write to {}: {}", config_path.display(), e))?;

    Ok(Some(config_path))
}

/// Ensures the runtime's bin directory is in the user's shell config. Returns Some(config_path) if
/// we appended. Returns None if the binary is already on PATH or we couldn't determine the path.
pub fn ensure_runtime_in_shell_config(lang: Lang) -> Result<Option<PathBuf>> {
    let bin_dir = match lang_bin_path_for_prepend(lang) {
        Some(p) => p,
        None => return Ok(None),
    };
    append_path_to_shell_config(&bin_dir)
}

/// Appends the given bin directory to the user's shell config. Use when installing to a custom
/// path.
pub fn ensure_path_in_shell_config(bin_dir: &Path) -> Result<Option<PathBuf>> {
    append_path_to_shell_config(bin_dir)
}

/// Install a language runtime. For C, returns error with install_hint.
/// Returns Some(bin_dir) when installed to custom path (caller should add to shell config).
/// `version` is optional; when None, uses the default/latest for that language.
pub fn install_lang(
    lang: Lang,
    location: InstallLocation,
    custom_path: Option<PathBuf>,
    version: Option<String>,
) -> Result<Option<PathBuf>> {
    if !can_auto_install(lang) {
        return Err(runtime_check::not_installed_error(lang));
    }

    terminal::section(&format!("Installing {}", poly_bench_runtime::lang_label(lang)));

    let label = poly_bench_runtime::lang_label(lang);
    let started = std::time::Instant::now();

    let result = match lang {
        Lang::Go => install_go(location, custom_path, version),
        Lang::TypeScript => install_node(location, custom_path, version),
        Lang::Rust => install_rust(location, custom_path),
        Lang::Python => install_python(location, custom_path),
        Lang::Zig => install_zig(location, custom_path, version),
        Lang::CSharp => install_dotnet(location, custom_path, version),
        Lang::C => Err(runtime_check::not_installed_error(lang)),
    };

    match &result {
        Ok(_) => {
            terminal::ensure_min_display(started.elapsed());
        }
        Err(_) => terminal::failure_indented(&format!("{} installation failed", label)),
    }
    result
}

fn platform() -> (String, String) {
    let arch = env::consts::ARCH;
    let os = env::consts::OS;
    let arch = match arch {
        "x86_64" => "amd64".to_string(),
        "aarch64" | "arm64" => "arm64".to_string(),
        _ => "amd64".to_string(),
    };
    let os = match os {
        "macos" => "darwin".to_string(),
        "linux" => "linux".to_string(),
        "windows" => "windows".to_string(),
        _ => "linux".to_string(),
    };
    (arch, os)
}

fn install_go(
    location: InstallLocation,
    custom_path: Option<PathBuf>,
    version_opt: Option<String>,
) -> Result<Option<PathBuf>> {
    let (arch, os) = platform();
    let version = version_opt
        .map(|v| v.trim_start_matches("go").to_string())
        .unwrap_or_else(|| "1.22.4".to_string());
    let filename = format!("go{}.{}-{}.tar.gz", version, os, arch);
    let url = format!("https://go.dev/dl/{}", filename);

    let install_dir =
        custom_path.clone().unwrap_or_else(|| lang_install_dir(Lang::Go, location).unwrap());
    let bin_dir = install_dir.join("go").join("bin");
    if bin_dir.exists() && which::which("go").is_err() {
        // Installed but not on PATH - we'll add it
    } else if bin_dir.exists() {
        terminal::info_indented("Go already installed");
        return Ok(None);
    }

    let use_sudo = custom_path.is_none() && location == InstallLocation::System;

    terminal::install_step(1, 4, &format!("Downloading Go {}...", version));
    let body = download_with_progress(&url, &format!("Downloading Go {}...", version))?;
    terminal::install_step(2, 4, "Extracting...");

    if use_sudo {
        let tmp = std::env::temp_dir().join(&filename);
        fs::write(&tmp, &body).map_err(|e| miette::miette!("Failed to write archive: {}", e))?;
        let status = Command::new("sudo")
            .args(["tar", "-xzf", tmp.to_str().unwrap(), "-C", install_dir.to_str().unwrap()])
            .status()
            .map_err(|e| miette::miette!("Failed to run sudo tar: {}", e))?;
        let _ = fs::remove_file(&tmp);
        if !status.success() {
            return Err(miette::miette!("Failed to extract Go to {}", install_dir.display()));
        }
    } else {
        fs::create_dir_all(&install_dir)
            .map_err(|e| miette::miette!("Failed to create install dir: {}", e))?;
        let decoder = GzDecoder::new(body.as_slice());
        let mut archive = tar::Archive::new(decoder);
        archive.unpack(&install_dir).map_err(|e| miette::miette!("Failed to extract Go: {}", e))?;
    }

    terminal::install_step(3, 4, "Done");
    let dest = install_dir.join("go");
    terminal::success_indented(&format!("Go installed at {}", dest.display()));
    Ok(custom_path.map(|_| bin_dir))
}

fn install_node(
    location: InstallLocation,
    custom_path: Option<PathBuf>,
    version_opt: Option<String>,
) -> Result<Option<PathBuf>> {
    let (arch, os) = platform();
    let version = version_opt
        .map(|v| v.trim_start_matches('v').to_string())
        .unwrap_or_else(|| "22.11.0".to_string());
    let (node_arch, node_os) = match (os.as_str(), arch.as_str()) {
        ("darwin", "arm64") => ("arm64", "darwin"),
        ("darwin", "amd64") => ("x64", "darwin"),
        ("linux", "amd64") => ("x64", "linux"),
        ("linux", "arm64") => ("arm64", "linux"),
        ("windows", "amd64") => ("x64", "win"),
        ("windows", "arm64") => ("arm64", "win"),
        _ => ("x64", "linux"),
    };
    let filename = format!("node-v{}-{}-{}.tar.gz", version, node_os, node_arch);
    let url = format!("https://nodejs.org/dist/v{}/{}", version, filename);

    let install_dir = custom_path
        .clone()
        .unwrap_or_else(|| lang_install_dir(Lang::TypeScript, location).unwrap());
    let extracted_name = format!("node-v{}-{}-{}", version, node_os, node_arch);
    let bin_dir = install_dir.join(&extracted_name).join("bin");
    if bin_dir.exists() && which::which("node").is_err() {
        // Installed but not on PATH
    } else if bin_dir.exists() {
        terminal::info_indented("Node.js already installed");
        return Ok(None);
    }

    let use_sudo = custom_path.is_none() && location == InstallLocation::System;

    terminal::install_step(1, 4, &format!("Downloading Node.js {}...", version));
    let body = download_with_progress(&url, &format!("Downloading Node.js {}...", version))?;
    terminal::install_step(2, 4, "Extracting...");

    if use_sudo {
        let tmp = std::env::temp_dir().join(&filename);
        fs::write(&tmp, &body).map_err(|e| miette::miette!("Failed to write archive: {}", e))?;
        let status = Command::new("sudo")
            .args(["tar", "-xzf", tmp.to_str().unwrap(), "-C", install_dir.to_str().unwrap()])
            .status()
            .map_err(|e| miette::miette!("Failed to run sudo tar: {}", e))?;
        let _ = fs::remove_file(&tmp);
        if !status.success() {
            return Err(miette::miette!("Failed to extract Node.js to {}", install_dir.display()));
        }
    } else {
        fs::create_dir_all(&install_dir)
            .map_err(|e| miette::miette!("Failed to create install dir: {}", e))?;
        let decoder = GzDecoder::new(body.as_slice());
        let mut archive = tar::Archive::new(decoder);
        archive
            .unpack(&install_dir)
            .map_err(|e| miette::miette!("Failed to extract Node.js: {}", e))?;
    }

    terminal::install_step(3, 4, "Done");
    terminal::success_indented(&format!(
        "Node.js installed at {}",
        install_dir.join(extracted_name).display()
    ));
    Ok(custom_path.map(|_| bin_dir))
}

fn install_rust(
    _location: InstallLocation,
    custom_path: Option<PathBuf>,
) -> Result<Option<PathBuf>> {
    let (cargo_home, cargo_bin) = if let Some(ref p) = custom_path {
        let cargo = p.join(".cargo");
        (cargo.clone(), cargo.join("bin"))
    } else {
        let home = home_dir()?;
        (home.join(".cargo"), home.join(".cargo").join("bin"))
    };

    if cargo_bin.join("cargo").exists() || cargo_bin.join("cargo.exe").exists() {
        if which::which("cargo").is_ok() {
            terminal::info_indented("Rust already installed");
            return Ok(None);
        }
        return Ok(None);
    }

    terminal::install_step(1, 4, "Downloading rustup...");
    let (arch, os) = platform();
    let (rustup_url, ext) = if os == "windows" {
        let target =
            if arch == "arm64" { "aarch64-pc-windows-msvc" } else { "x86_64-pc-windows-msvc" };
        (format!("https://static.rust-lang.org/rustup/dist/{}/rustup-init.exe", target), "exe")
    } else {
        let target = match (arch.as_str(), os.as_str()) {
            ("arm64", "darwin") => "aarch64-apple-darwin",
            ("amd64", "darwin") => "x86_64-apple-darwin",
            ("arm64", "linux") => "aarch64-unknown-linux-gnu",
            _ => "x86_64-unknown-linux-gnu",
        };
        (format!("https://static.rust-lang.org/rustup/dist/{}/rustup-init", target), "bin")
    };

    let body = download_with_progress(&rustup_url, "Downloading rustup...")?;
    let tmp_dir = std::env::temp_dir().join("polybench-rustup");
    fs::create_dir_all(&tmp_dir)
        .map_err(|e| miette::miette!("Failed to create temp dir: {}", e))?;
    let rustup_path = tmp_dir.join(if ext == "exe" { "rustup-init.exe" } else { "rustup-init" });
    fs::write(&rustup_path, &body)
        .map_err(|e| miette::miette!("Failed to write rustup-init: {}", e))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&rustup_path)
            .map_err(|e| miette::miette!("Failed to read metadata: {}", e))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&rustup_path, perms)
            .map_err(|e| miette::miette!("Failed to set executable: {}", e))?;
    }

    terminal::install_step(2, 4, "Running rustup (this may take a few minutes)...");
    let rustup_home = cargo_home.parent().unwrap().join(".rustup");
    let mut cmd = Command::new(&rustup_path);
    cmd.args(["-y", "--default-toolchain", "stable"]);
    if custom_path.is_some() {
        cmd.env("CARGO_HOME", &cargo_home).env("RUSTUP_HOME", &rustup_home);
    }
    let status = cmd.status().map_err(|e| miette::miette!("Failed to run rustup: {}", e))?;

    let _ = fs::remove_dir_all(&tmp_dir);

    if !status.success() {
        return Err(miette::miette!("rustup failed with exit code {:?}", status.code()));
    }

    terminal::install_step(3, 4, "Done");
    terminal::success_indented(&format!("Rust installed at {}", cargo_bin.display()));
    Ok(custom_path.map(|_| cargo_bin))
}

fn install_python(
    location: InstallLocation,
    custom_path: Option<PathBuf>,
) -> Result<Option<PathBuf>> {
    // Use python-build-standalone for pre-built binaries
    let (arch, os) = platform();
    let target = match (os.as_str(), arch.as_str()) {
        ("darwin", "arm64") => "aarch64-apple-darwin-install_only",
        ("darwin", "amd64") => "x86_64-apple-darwin-install_only",
        ("linux", "amd64") => "x86_64-unknown-linux-gnu-install_only",
        ("linux", "arm64") => "aarch64-unknown-linux-gnu-install_only",
        ("windows", "amd64") => "x86_64-pc-windows-msvc-install_only",
        ("windows", "arm64") => "aarch64-pc-windows-msvc-install_only",
        _ => "x86_64-unknown-linux-gnu-install_only",
    };
    // python-build-standalone: release tag from https://github.com/astral-sh/python-build-standalone/releases
    const PYTHON_RELEASE_TAG: &str = "20231016"; // Contains Python 3.12.0
    let py_version = "3.12.0";
    let release_tag = PYTHON_RELEASE_TAG;
    let filename = format!("cpython-{}+{}-{}.tar.gz", py_version, release_tag, target);
    let url = format!(
        "https://github.com/astral-sh/python-build-standalone/releases/download/{}/{}",
        release_tag, filename
    );

    let install_dir =
        custom_path.clone().unwrap_or_else(|| lang_install_dir(Lang::Python, location).unwrap());
    let python_bin = install_dir.join("install").join("bin");
    let python_bin_win = install_dir.join("install");
    let path_to_add = if cfg!(windows) {
        install_dir.join("install")
    } else {
        install_dir.join("install").join("bin")
    };
    if (python_bin.exists() || python_bin_win.join("python.exe").exists()) &&
        which::which("python3").is_err() &&
        which::which("python").is_err()
    {
        // Installed but not on PATH
    } else if python_bin.exists() || python_bin_win.join("python.exe").exists() {
        terminal::info_indented("Python already installed");
        return Ok(None);
    }

    let use_sudo = custom_path.is_none() && location == InstallLocation::System;

    terminal::install_step(1, 4, &format!("Downloading Python {}...", py_version));
    let body = download_with_progress(&url, &format!("Downloading Python {}...", py_version))?;
    terminal::install_step(2, 4, "Extracting...");

    if use_sudo {
        let tmp = std::env::temp_dir().join(&filename);
        fs::write(&tmp, &body).map_err(|e| miette::miette!("Failed to write archive: {}", e))?;
        let status = Command::new("sudo")
            .args(["tar", "-xzf", tmp.to_str().unwrap(), "-C", install_dir.to_str().unwrap()])
            .status()
            .map_err(|e| miette::miette!("Failed to run sudo tar: {}", e))?;
        let _ = fs::remove_file(&tmp);
        if !status.success() {
            return Err(miette::miette!("Failed to extract Python to {}", install_dir.display()));
        }
    } else {
        fs::create_dir_all(&install_dir)
            .map_err(|e| miette::miette!("Failed to create install dir: {}", e))?;
        let decoder = GzDecoder::new(body.as_slice());
        let mut archive = tar::Archive::new(decoder);
        archive
            .unpack(&install_dir)
            .map_err(|e| miette::miette!("Failed to extract Python: {}", e))?;
    }

    terminal::install_step(3, 4, "Done");
    terminal::success_indented(&format!("Python installed at {}", path_to_add.display()));
    Ok(custom_path.map(|_| path_to_add))
}

fn install_zig(
    location: InstallLocation,
    custom_path: Option<PathBuf>,
    version_opt: Option<String>,
) -> Result<Option<PathBuf>> {
    let version = version_opt.unwrap_or_else(|| "0.13.0".to_string());
    let url = zig_download_url(&version)?;
    let filename = url
        .rsplit('/')
        .next()
        .unwrap_or("zig.tar.xz")
        .to_string();
    let extracted_dir_name = filename
        .strip_suffix(".tar.xz")
        .or_else(|| filename.strip_suffix(".zip"))
        .unwrap_or(&filename)
        .to_string();

    let install_dir =
        custom_path.clone().unwrap_or_else(|| lang_install_dir(Lang::Zig, location).unwrap());
    let zig_extracted = install_dir.join(&extracted_dir_name);
    let zig_bin = zig_extracted.join("zig");
    let zig_bin_exe = zig_extracted.join("zig.exe");
    if (zig_bin.exists() || zig_bin_exe.exists()) && which::which("zig").is_err() {
        // Installed but not on PATH
    } else if zig_bin.exists() || zig_bin_exe.exists() {
        terminal::info_indented("Zig already installed");
        return Ok(None);
    }

    let use_sudo = custom_path.is_none() && location == InstallLocation::System;

    terminal::install_step(1, 4, &format!("Downloading Zig {}...", version));
    let body = download_with_progress(&url, &format!("Downloading Zig {}...", version))?;
    terminal::install_step(2, 4, "Extracting...");

    let archive_path =
        if use_sudo { std::env::temp_dir().join(&filename) } else { install_dir.join(&filename) };
    fs::create_dir_all(archive_path.parent().unwrap())
        .map_err(|e| miette::miette!("Failed to create dir: {}", e))?;
    fs::write(&archive_path, &body)
        .map_err(|e| miette::miette!("Failed to write archive: {}", e))?;

    let status = if use_sudo {
        Command::new("sudo")
            .args([
                "tar",
                "-xJf",
                archive_path.to_str().unwrap(),
                "-C",
                install_dir.to_str().unwrap(),
            ])
            .status()
    } else {
        Command::new("tar")
            .args(["-xJf", archive_path.to_str().unwrap(), "-C", install_dir.to_str().unwrap()])
            .status()
    };
    let status =
        status.map_err(|e| miette::miette!("Failed to extract Zig (tar required): {}", e))?;

    let _ = fs::remove_file(&archive_path);

    if !status.success() {
        return Err(miette::miette!("Failed to extract Zig archive"));
    }

    terminal::install_step(3, 4, "Done");
    terminal::success_indented(&format!("Zig installed at {}", zig_extracted.display()));
    Ok(custom_path.map(|_| zig_extracted))
}

fn install_dotnet(
    location: InstallLocation,
    custom_path: Option<PathBuf>,
    version_opt: Option<String>,
) -> Result<Option<PathBuf>> {
    let (arch, os) = platform();
    let version = version_opt.unwrap_or_else(|| {
        fetch_dotnet_versions()
            .ok()
            .and_then(|v| v.into_iter().next())
            .unwrap_or_else(|| "8.0.418".to_string())
    });
    let (dotnet_arch, dotnet_os) = match (os.as_str(), arch.as_str()) {
        ("darwin", "arm64") => ("arm64", "osx"),
        ("darwin", "amd64") => ("x64", "osx"),
        ("linux", "amd64") => ("x64", "linux"),
        ("linux", "arm64") => ("arm64", "linux"),
        ("windows", "amd64") => ("x64", "win"),
        ("windows", "arm64") => ("arm64", "win"),
        _ => ("x64", "linux"),
    };
    let url = format!(
        "https://builds.dotnet.microsoft.com/dotnet/Sdk/{}/dotnet-sdk-{}-{}-{}.tar.gz",
        version, version, dotnet_os, dotnet_arch
    );

    let install_dir =
        custom_path.clone().unwrap_or_else(|| lang_install_dir(Lang::CSharp, location).unwrap());
    let dotnet_bin = install_dir.join("dotnet");
    if dotnet_bin.exists() && which::which("dotnet").is_err() {
        // Installed but not on PATH
    } else if dotnet_bin.exists() {
        terminal::info_indented(".NET already installed");
        return Ok(None);
    }

    let use_sudo = custom_path.is_none() && location == InstallLocation::System;

    terminal::install_step(1, 4, &format!("Downloading .NET SDK {}...", version));
    let body = download_with_progress(&url, &format!("Downloading .NET SDK {}...", version))?;
    terminal::install_step(2, 4, "Extracting...");

    if use_sudo {
        let filename = format!("dotnet-sdk-{}-{}-{}.tar.gz", version, dotnet_os, dotnet_arch);
        let tmp = std::env::temp_dir().join(&filename);
        fs::write(&tmp, &body).map_err(|e| miette::miette!("Failed to write archive: {}", e))?;
        // Ensure target dir exists (e.g. /usr/local/share/dotnet)
        let mkdir =
            Command::new("sudo").args(["mkdir", "-p", install_dir.to_str().unwrap()]).status();
        if let Ok(s) = mkdir {
            if !s.success() {
                return Err(miette::miette!("Failed to create {}", install_dir.display()));
            }
        }
        let status = Command::new("sudo")
            .args(["tar", "-xzf", tmp.to_str().unwrap(), "-C", install_dir.to_str().unwrap()])
            .status()
            .map_err(|e| miette::miette!("Failed to run sudo tar: {}", e))?;
        let _ = fs::remove_file(&tmp);
        if !status.success() {
            return Err(miette::miette!("Failed to extract .NET to {}", install_dir.display()));
        }
    } else {
        fs::create_dir_all(&install_dir)
            .map_err(|e| miette::miette!("Failed to create install dir: {}", e))?;
        let decoder = GzDecoder::new(body.as_slice());
        let mut archive = tar::Archive::new(decoder);
        archive
            .unpack(&install_dir)
            .map_err(|e| miette::miette!("Failed to extract .NET: {}", e))?;
    }

    terminal::install_step(3, 4, "Done");
    terminal::success_indented(&format!(".NET installed at {}", install_dir.display()));
    Ok(custom_path.map(|_| install_dir))
}

/// Download a URL with progress bar or spinner. Used by runtime installer and build.
pub fn download_with_progress(url: &str, msg: &str) -> Result<Vec<u8>> {
    let mut response = ureq::get(url).call().map_err(|e| {
        miette::miette!("Failed to download {}: {}. Ensure you have network access.", url, e)
    })?;

    let total = response.body().content_length();

    let pb = match total {
        Some(n) => terminal::download_progress_bar(n, msg),
        None => terminal::download_spinner(msg),
    };

    let mut reader = response.body_mut().with_config().limit(200 * 1024 * 1024).reader();
    let mut body = Vec::with_capacity(total.unwrap_or(0) as usize);
    let mut buf = [0u8; 64 * 1024];
    loop {
        let n =
            reader.read(&mut buf).map_err(|e| miette::miette!("Failed to read download: {}", e))?;
        if n == 0 {
            break;
        }
        body.extend_from_slice(&buf[..n]);
        pb.inc(n as u64);
    }

    terminal::ensure_min_display(pb.elapsed());
    pb.finish_and_clear();
    Ok(body)
}
