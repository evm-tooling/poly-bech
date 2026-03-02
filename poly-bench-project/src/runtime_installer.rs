//! Runtime installer: download and install language runtimes to Polybench-managed toolchains.
//!
//! Installs to the Polybench toolchain directory:
//! - macOS: `~/Library/Application Support/Polybench/toolchains/<lang>/<version>/<platform>/`
//! - Linux: `~/.local/share/polybench/toolchains/<lang>/<version>/<platform>/`
//! - Windows: `%LOCALAPPDATA%\Polybench\toolchains\<lang>\<version>\<platform>\`
//!
//! Each runtime is installed with the pinned version from `toolchain.rs`.

use crate::{
    runtime_check, terminal,
    toolchain::{self, pinned_version, toolchain_binary_path, toolchain_path, Platform},
};
use flate2::read::GzDecoder;
use miette::Result;
use poly_bench_dsl::Lang;
use std::{env, fs, io::Read, path::PathBuf, process::Command};

/// Returns true if poly-bench can auto-install this language.
pub fn can_auto_install(lang: Lang) -> bool {
    lang != Lang::C
}

/// Install a language runtime to the Polybench toolchain directory.
///
/// Uses the pinned version from `toolchain.rs`. Returns the path to the installed binary.
pub fn install_toolchain(lang: Lang) -> Result<PathBuf> {
    if !can_auto_install(lang) {
        return Err(runtime_check::not_installed_error(lang));
    }

    terminal::section(&format!("Installing {}", poly_bench_runtime::lang_label(lang)));

    let label = poly_bench_runtime::lang_label(lang);
    let started = std::time::Instant::now();

    let result = match lang {
        Lang::Go => install_go_toolchain(),
        Lang::TypeScript => install_node_toolchain(),
        Lang::Rust => install_rust_toolchain(),
        Lang::Python => install_python_toolchain(),
        Lang::Zig => install_zig_toolchain(),
        Lang::CSharp => install_dotnet_toolchain(),
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

fn platform() -> Platform {
    Platform::current()
}

/// Get Zig download URL from index.json. Zig uses different filename formats across
/// versions (e.g. zig-macos-aarch64-0.13.0 vs zig-aarch64-macos-0.15.2), so we must
/// use the URL from the index instead of constructing it.
fn zig_download_url(version: &str) -> Result<String> {
    let plat = platform();
    let platform_key = match (plat.os.as_str(), plat.arch.as_str()) {
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
            miette::miette!("Zig {} not available for {}-{}", version, plat.arch, plat.os)
        })?;

    Ok(tarball.to_string())
}

/// Fetch the latest .NET 8.x SDK version.
fn fetch_latest_dotnet_8_version() -> Result<String> {
    let url = "https://dotnetcli.blob.core.windows.net/dotnet/release-metadata/releases-index.json";
    let resp: serde_json::Value = ureq::get(url)
        .call()
        .map_err(|e| miette::miette!("Failed to fetch .NET versions: {}", e))?
        .body_mut()
        .read_json()
        .map_err(|e| miette::miette!("Failed to parse .NET versions: {}", e))?;

    let index = resp
        .get("releases-index")
        .and_then(|v| v.as_array())
        .ok_or_else(|| miette::miette!("Invalid .NET releases index"))?;

    // Find the latest 8.x version
    for entry in index {
        if let Some(channel) = entry.get("channel-version").and_then(|v| v.as_str()) {
            if channel.starts_with("8.") {
                if let Some(sdk) = entry.get("latest-sdk").and_then(|v| v.as_str()) {
                    return Ok(sdk.to_string());
                }
            }
        }
    }

    // Fallback to a known good version
    Ok("8.0.418".to_string())
}

fn install_go_toolchain() -> Result<PathBuf> {
    let version = pinned_version(Lang::Go);
    let plat = platform();
    let install_dir = toolchain_path(Lang::Go, version)?;
    let binary_path = toolchain_binary_path(Lang::Go, version)?;

    // Check if already installed
    if binary_path.exists() {
        terminal::info_indented(&format!("Go {} already installed", version));
        return Ok(binary_path);
    }

    let filename = format!("go{}.{}-{}.tar.gz", version, plat.os, plat.arch);
    let url = format!("https://go.dev/dl/{}", filename);

    terminal::install_step(1, 3, &format!("Downloading Go {}...", version));
    let body = download_with_progress(&url, &format!("Downloading Go {}...", version))?;

    terminal::install_step(2, 3, "Extracting...");
    fs::create_dir_all(&install_dir)
        .map_err(|e| miette::miette!("Failed to create install dir: {}", e))?;

    let decoder = GzDecoder::new(body.as_slice());
    let mut archive = tar::Archive::new(decoder);
    archive.unpack(&install_dir).map_err(|e| miette::miette!("Failed to extract Go: {}", e))?;

    terminal::install_step(3, 3, "Done");
    terminal::success_indented(&format!("Go {} installed at {}", version, install_dir.display()));

    Ok(binary_path)
}

fn install_node_toolchain() -> Result<PathBuf> {
    let version = pinned_version(Lang::TypeScript);
    let plat = platform();
    let install_dir = toolchain_path(Lang::TypeScript, version)?;
    let binary_path = toolchain_binary_path(Lang::TypeScript, version)?;

    // Check if already installed
    if binary_path.exists() {
        terminal::info_indented(&format!("Node.js {} already installed", version));
        return Ok(binary_path);
    }

    let (node_arch, node_os) = match (plat.os.as_str(), plat.arch.as_str()) {
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

    terminal::install_step(1, 3, &format!("Downloading Node.js {}...", version));
    let body = download_with_progress(&url, &format!("Downloading Node.js {}...", version))?;

    terminal::install_step(2, 3, "Extracting...");
    fs::create_dir_all(&install_dir)
        .map_err(|e| miette::miette!("Failed to create install dir: {}", e))?;

    let decoder = GzDecoder::new(body.as_slice());
    let mut archive = tar::Archive::new(decoder);
    archive
        .unpack(&install_dir)
        .map_err(|e| miette::miette!("Failed to extract Node.js: {}", e))?;

    // Node extracts to a subdirectory, we need to move contents up
    let extracted_name = format!("node-v{}-{}-{}", version, node_os, node_arch);
    let extracted_dir = install_dir.join(&extracted_name);
    if extracted_dir.exists() {
        // Move bin directory to the expected location
        let src_bin = extracted_dir.join("bin");
        let dst_bin = install_dir.join("bin");
        if src_bin.exists() && !dst_bin.exists() {
            fs::rename(&src_bin, &dst_bin)
                .map_err(|e| miette::miette!("Failed to move Node bin dir: {}", e))?;
        }
        // Move lib directory
        let src_lib = extracted_dir.join("lib");
        let dst_lib = install_dir.join("lib");
        if src_lib.exists() && !dst_lib.exists() {
            fs::rename(&src_lib, &dst_lib)
                .map_err(|e| miette::miette!("Failed to move Node lib dir: {}", e))?;
        }
        // Clean up extracted subdir
        let _ = fs::remove_dir_all(&extracted_dir);
    }

    terminal::install_step(3, 3, "Done");
    terminal::success_indented(&format!(
        "Node.js {} installed at {}",
        version,
        install_dir.display()
    ));

    Ok(binary_path)
}

fn install_rust_toolchain() -> Result<PathBuf> {
    let version = pinned_version(Lang::Rust); // "stable"
    let install_dir = toolchain_path(Lang::Rust, version)?;
    let binary_path = toolchain_binary_path(Lang::Rust, version)?;

    // Check if already installed
    if binary_path.exists() {
        terminal::info_indented("Rust already installed");
        return Ok(binary_path);
    }

    let cargo_home = install_dir.join("cargo");
    let rustup_home = install_dir.join("rustup");

    terminal::install_step(1, 3, "Downloading rustup...");
    let plat = platform();
    let (rustup_url, ext) = if plat.os == "windows" {
        let target =
            if plat.arch == "arm64" { "aarch64-pc-windows-msvc" } else { "x86_64-pc-windows-msvc" };
        (format!("https://static.rust-lang.org/rustup/dist/{}/rustup-init.exe", target), "exe")
    } else {
        let target = match (plat.arch.as_str(), plat.os.as_str()) {
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

    terminal::install_step(2, 3, "Running rustup (this may take a few minutes)...");
    fs::create_dir_all(&cargo_home)
        .map_err(|e| miette::miette!("Failed to create cargo home: {}", e))?;
    fs::create_dir_all(&rustup_home)
        .map_err(|e| miette::miette!("Failed to create rustup home: {}", e))?;

    let status = Command::new(&rustup_path)
        .args(["-y", "--default-toolchain", "stable", "--no-modify-path"])
        .env("CARGO_HOME", &cargo_home)
        .env("RUSTUP_HOME", &rustup_home)
        .status()
        .map_err(|e| miette::miette!("Failed to run rustup: {}", e))?;

    let _ = fs::remove_dir_all(&tmp_dir);

    if !status.success() {
        return Err(miette::miette!("rustup failed with exit code {:?}", status.code()));
    }

    // The binary is at cargo_home/bin/cargo
    let actual_binary =
        cargo_home.join("bin").join(if cfg!(windows) { "cargo.exe" } else { "cargo" });

    terminal::install_step(3, 3, "Done");
    terminal::success_indented(&format!("Rust installed at {}", install_dir.display()));

    Ok(actual_binary)
}

fn install_python_toolchain() -> Result<PathBuf> {
    let version = pinned_version(Lang::Python);
    let plat = platform();
    let install_dir = toolchain_path(Lang::Python, version)?;
    let binary_path = toolchain_binary_path(Lang::Python, version)?;

    // Check if already installed
    if binary_path.exists() {
        terminal::info_indented(&format!("Python {} already installed", version));
        return Ok(binary_path);
    }

    let target = match (plat.os.as_str(), plat.arch.as_str()) {
        ("darwin", "arm64") => "aarch64-apple-darwin-install_only",
        ("darwin", "amd64") => "x86_64-apple-darwin-install_only",
        ("linux", "amd64") => "x86_64-unknown-linux-gnu-install_only",
        ("linux", "arm64") => "aarch64-unknown-linux-gnu-install_only",
        ("windows", "amd64") => "x86_64-pc-windows-msvc-install_only",
        ("windows", "arm64") => "aarch64-pc-windows-msvc-install_only",
        _ => "x86_64-unknown-linux-gnu-install_only",
    };

    // python-build-standalone release tag for Python 3.12.0
    const PYTHON_RELEASE_TAG: &str = "20231016";
    let filename = format!("cpython-{}+{}-{}.tar.gz", version, PYTHON_RELEASE_TAG, target);
    let url = format!(
        "https://github.com/astral-sh/python-build-standalone/releases/download/{}/{}",
        PYTHON_RELEASE_TAG, filename
    );

    terminal::install_step(1, 3, &format!("Downloading Python {}...", version));
    let body = download_with_progress(&url, &format!("Downloading Python {}...", version))?;

    terminal::install_step(2, 3, "Extracting...");
    fs::create_dir_all(&install_dir)
        .map_err(|e| miette::miette!("Failed to create install dir: {}", e))?;

    let decoder = GzDecoder::new(body.as_slice());
    let mut archive = tar::Archive::new(decoder);
    archive.unpack(&install_dir).map_err(|e| miette::miette!("Failed to extract Python: {}", e))?;

    terminal::install_step(3, 3, "Done");
    terminal::success_indented(&format!(
        "Python {} installed at {}",
        version,
        install_dir.display()
    ));

    Ok(binary_path)
}

fn install_zig_toolchain() -> Result<PathBuf> {
    let version = pinned_version(Lang::Zig);
    let install_dir = toolchain_path(Lang::Zig, version)?;
    let binary_path = toolchain_binary_path(Lang::Zig, version)?;

    // Check if already installed
    if binary_path.exists() {
        terminal::info_indented(&format!("Zig {} already installed", version));
        return Ok(binary_path);
    }

    let url = zig_download_url(version)?;
    let filename = url.rsplit('/').next().unwrap_or("zig.tar.xz").to_string();

    terminal::install_step(1, 3, &format!("Downloading Zig {}...", version));
    let body = download_with_progress(&url, &format!("Downloading Zig {}...", version))?;

    terminal::install_step(2, 3, "Extracting...");
    fs::create_dir_all(&install_dir)
        .map_err(|e| miette::miette!("Failed to create install dir: {}", e))?;

    // Write archive and extract with tar (Zig uses .tar.xz)
    let archive_path = install_dir.join(&filename);
    fs::write(&archive_path, &body)
        .map_err(|e| miette::miette!("Failed to write archive: {}", e))?;

    let status = Command::new("tar")
        .args(["-xJf", archive_path.to_str().unwrap(), "-C", install_dir.to_str().unwrap()])
        .status()
        .map_err(|e| miette::miette!("Failed to extract Zig (tar required): {}", e))?;

    let _ = fs::remove_file(&archive_path);

    if !status.success() {
        return Err(miette::miette!("Failed to extract Zig archive"));
    }

    // Zig extracts to a subdirectory, move the zig binary up
    let extracted_dir_name = filename
        .strip_suffix(".tar.xz")
        .or_else(|| filename.strip_suffix(".zip"))
        .unwrap_or(&filename);
    let extracted_dir = install_dir.join(extracted_dir_name);
    if extracted_dir.exists() {
        let src_zig = extracted_dir.join("zig");
        let dst_zig = install_dir.join("zig");
        if src_zig.exists() && !dst_zig.exists() {
            fs::rename(&src_zig, &dst_zig)
                .map_err(|e| miette::miette!("Failed to move zig binary: {}", e))?;
        }
        // Move lib directory
        let src_lib = extracted_dir.join("lib");
        let dst_lib = install_dir.join("lib");
        if src_lib.exists() && !dst_lib.exists() {
            fs::rename(&src_lib, &dst_lib)
                .map_err(|e| miette::miette!("Failed to move zig lib: {}", e))?;
        }
        let _ = fs::remove_dir_all(&extracted_dir);
    }

    terminal::install_step(3, 3, "Done");
    terminal::success_indented(&format!("Zig {} installed at {}", version, install_dir.display()));

    Ok(binary_path)
}

fn install_dotnet_toolchain() -> Result<PathBuf> {
    let version = fetch_latest_dotnet_8_version()?;
    let plat = platform();
    let install_dir = toolchain_path(Lang::CSharp, &version)?;

    // .NET binary is directly in the install dir
    let binary_name = if cfg!(windows) { "dotnet.exe" } else { "dotnet" };
    let binary_path = install_dir.join(binary_name);

    // Check if already installed
    if binary_path.exists() {
        terminal::info_indented(&format!(".NET {} already installed", version));
        return Ok(binary_path);
    }

    let (dotnet_arch, dotnet_os) = match (plat.os.as_str(), plat.arch.as_str()) {
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

    terminal::install_step(1, 3, &format!("Downloading .NET SDK {}...", version));
    let body = download_with_progress(&url, &format!("Downloading .NET SDK {}...", version))?;

    terminal::install_step(2, 3, "Extracting...");
    fs::create_dir_all(&install_dir)
        .map_err(|e| miette::miette!("Failed to create install dir: {}", e))?;

    let decoder = GzDecoder::new(body.as_slice());
    let mut archive = tar::Archive::new(decoder);
    archive.unpack(&install_dir).map_err(|e| miette::miette!("Failed to extract .NET: {}", e))?;

    terminal::install_step(3, 3, "Done");
    terminal::success_indented(&format!(".NET {} installed at {}", version, install_dir.display()));

    Ok(binary_path)
}

/// Download a URL with progress bar or spinner.
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

// ============================================================================
// Legacy compatibility - these functions are kept for backward compatibility
// but should be migrated to the new system
// ============================================================================

/// Returns the bin directory to prepend to PATH when the binary is not on PATH but exists
/// in a Polybench toolchain. Returns None if already on PATH.
pub fn lang_bin_path_for_prepend(lang: Lang) -> Option<PathBuf> {
    // First check if already on PATH
    for bin in runtime_check::required_binary(lang) {
        if which::which(bin).is_ok() {
            return None;
        }
    }

    // Check if we have a Polybench-managed toolchain
    let version = pinned_version(lang);
    if let Ok(binary_path) = toolchain_binary_path(lang, version) {
        if binary_path.exists() {
            return binary_path.parent().map(|p| p.to_path_buf());
        }
    }

    // Check shim directory
    if let Ok(shim_dir) = toolchain::shim_dir() {
        if shim_dir.exists() {
            return Some(shim_dir);
        }
    }

    None
}

/// Backward compatibility: same as lang_bin_path_for_prepend.
pub fn polybench_runtime_path(lang: Lang) -> Option<PathBuf> {
    lang_bin_path_for_prepend(lang)
}

/// Returns the shell config file path based on $SHELL.
fn shell_config_path() -> Option<PathBuf> {
    let home = env::var("HOME").or_else(|_| env::var("USERPROFILE")).ok()?;
    let home = PathBuf::from(home);
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

/// Appends the given bin directory to PATH in the user's shell config.
fn append_path_to_shell_config(bin_dir: &std::path::Path) -> Result<Option<PathBuf>> {
    use std::io::Write;

    let config_path = match shell_config_path() {
        Some(p) => p,
        None => return Ok(None),
    };

    let bin_str = bin_dir.to_string_lossy();
    let content = fs::read_to_string(&config_path).unwrap_or_default();

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

/// Ensures the runtime's bin directory is in the user's shell config.
pub fn ensure_runtime_in_shell_config(lang: Lang) -> Result<Option<PathBuf>> {
    let bin_dir = match lang_bin_path_for_prepend(lang) {
        Some(p) => p,
        None => return Ok(None),
    };
    append_path_to_shell_config(&bin_dir)
}

/// Appends the given bin directory to the user's shell config.
pub fn ensure_path_in_shell_config(bin_dir: &std::path::Path) -> Result<Option<PathBuf>> {
    append_path_to_shell_config(bin_dir)
}

// Keep these for backward compatibility during migration
#[derive(Clone, Copy, PartialEq, Eq)]
#[deprecated(note = "Use install_toolchain instead")]
pub enum InstallLocation {
    UserLocal,
    System,
}

#[deprecated(note = "Use install_toolchain instead")]
pub fn default_install_path(lang: Lang, _location: InstallLocation) -> Result<PathBuf> {
    let version = pinned_version(lang);
    toolchain_path(lang, version)
}

#[deprecated(note = "Version selection is no longer supported - uses pinned versions")]
pub fn supports_version_selection(_lang: Lang) -> bool {
    false
}

#[deprecated(note = "Version selection is no longer supported - uses pinned versions")]
pub fn fetch_available_versions(_lang: Lang) -> Result<Vec<String>> {
    Err(miette::miette!(
        "Version selection is no longer supported. Polybench uses pinned versions."
    ))
}

#[deprecated(note = "Use install_toolchain instead")]
#[allow(deprecated)]
pub fn install_lang(
    lang: Lang,
    _location: InstallLocation,
    _custom_path: Option<PathBuf>,
    _version: Option<String>,
) -> Result<Option<PathBuf>> {
    let binary = install_toolchain(lang)?;
    Ok(Some(binary.parent().unwrap_or(&binary).to_path_buf()))
}
