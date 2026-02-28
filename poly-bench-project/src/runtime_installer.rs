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
    io::Write,
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

/// Returns true if poly-bench can auto-install this language.
pub fn can_auto_install(lang: Lang) -> bool {
    lang != Lang::C
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

/// Install a language runtime. For C, returns error with install_hint.
pub fn install_lang(lang: Lang, location: InstallLocation) -> Result<()> {
    if !can_auto_install(lang) {
        return Err(runtime_check::not_installed_error(lang));
    }

    terminal::section(&format!("Installing {}", poly_bench_runtime::lang_label(lang)));

    match lang {
        Lang::Go => install_go(location),
        Lang::TypeScript => install_node(location),
        Lang::Rust => install_rust(location),
        Lang::Python => install_python(location),
        Lang::Zig => install_zig(location),
        Lang::CSharp => install_dotnet(location),
        Lang::C => Err(runtime_check::not_installed_error(lang)),
    }
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

fn install_go(location: InstallLocation) -> Result<()> {
    let (arch, os) = platform();
    let version = "1.22.4";
    let filename = format!("go{}.{}-{}.tar.gz", version, os, arch);
    let url = format!("https://go.dev/dl/{}", filename);

    let install_dir = lang_install_dir(Lang::Go, location)?;
    let bin_dir = install_dir.join("go").join("bin");
    if bin_dir.exists() && which::which("go").is_err() {
        // Installed but not on PATH - we'll add it
    } else if bin_dir.exists() {
        terminal::info_indented("Go already installed");
        return Ok(());
    }

    terminal::install_step(1, 4, &format!("Downloading Go {}...", version));
    let body = download(&url)?;
    terminal::install_step(2, 4, "Extracting...");

    if location == InstallLocation::System {
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
    Ok(())
}

fn install_node(location: InstallLocation) -> Result<()> {
    let (arch, os) = platform();
    let version = "22.11.0";
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

    let install_dir = lang_install_dir(Lang::TypeScript, location)?;
    let extracted_name = format!("node-v{}-{}-{}", version, node_os, node_arch);
    let bin_dir = install_dir.join(&extracted_name).join("bin");
    if bin_dir.exists() && which::which("node").is_err() {
        // Installed but not on PATH
    } else if bin_dir.exists() {
        terminal::info_indented("Node.js already installed");
        return Ok(());
    }

    terminal::install_step(1, 4, &format!("Downloading Node.js {}...", version));
    let body = download(&url)?;
    terminal::install_step(2, 4, "Extracting...");

    if location == InstallLocation::System {
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
    Ok(())
}

fn install_rust(_location: InstallLocation) -> Result<()> {
    // Rust uses rustup defaults (~/.cargo, ~/.rustup) - we don't override CARGO_HOME/RUSTUP_HOME
    let cargo_bin = home_dir()?.join(".cargo").join("bin");
    if cargo_bin.join("cargo").exists() || cargo_bin.join("cargo.exe").exists() {
        if which::which("cargo").is_ok() {
            terminal::info_indented("Rust already installed");
            return Ok(());
        }
        // Installed but not on PATH - lang_bin_path_for_prepend will find it
        return Ok(());
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

    let body = download(&rustup_url)?;
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
    // Do not set CARGO_HOME/RUSTUP_HOME - let rustup use defaults (~/.cargo, ~/.rustup)
    let status = Command::new(&rustup_path)
        .args(["-y", "--default-toolchain", "stable"])
        .status()
        .map_err(|e| miette::miette!("Failed to run rustup: {}", e))?;

    let _ = fs::remove_dir_all(&tmp_dir);

    if !status.success() {
        return Err(miette::miette!("rustup failed with exit code {:?}", status.code()));
    }

    terminal::install_step(3, 4, "Done");
    terminal::success_indented(&format!("Rust installed at {}", cargo_bin.display()));
    Ok(())
}

fn install_python(location: InstallLocation) -> Result<()> {
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
    // python-build-standalone release 20231016 has Python 3.12.0
    let py_version = "3.12.0";
    let release_tag = "20231016";
    let filename = format!("cpython-{}+{}-{}.tar.gz", py_version, release_tag, target);
    let url = format!(
        "https://github.com/astral-sh/python-build-standalone/releases/download/{}/{}",
        release_tag, filename
    );

    let install_dir = lang_install_dir(Lang::Python, location)?;
    let python_bin = install_dir.join("install").join("bin");
    let python_bin_win = install_dir.join("install");
    if (python_bin.exists() || python_bin_win.join("python.exe").exists()) &&
        which::which("python3").is_err() &&
        which::which("python").is_err()
    {
        // Installed but not on PATH
    } else if python_bin.exists() || python_bin_win.join("python.exe").exists() {
        terminal::info_indented("Python already installed");
        return Ok(());
    }

    terminal::install_step(1, 4, &format!("Downloading Python {}...", py_version));
    let body = download(&url)?;
    terminal::install_step(2, 4, "Extracting...");

    if location == InstallLocation::System {
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

    let path_to_add = if cfg!(windows) {
        install_dir.join("install")
    } else {
        install_dir.join("install").join("bin")
    };
    terminal::install_step(3, 4, "Done");
    terminal::success_indented(&format!("Python installed at {}", path_to_add.display()));
    Ok(())
}

fn install_zig(location: InstallLocation) -> Result<()> {
    let (arch, os) = platform();
    let version = "0.13.0";
    let (zig_arch, zig_os) = match (os.as_str(), arch.as_str()) {
        ("darwin", "arm64") => ("aarch64", "macos"),
        ("darwin", "amd64") => ("x86_64", "macos"),
        ("linux", "amd64") => ("x86_64", "linux"),
        ("linux", "arm64") => ("aarch64", "linux"),
        ("windows", "amd64") => ("x86_64", "windows"),
        ("windows", "arm64") => ("aarch64", "windows"),
        _ => ("x86_64", "linux"),
    };
    let filename = format!("zig-{}-{}-{}.tar.xz", zig_os, zig_arch, version);
    let url = format!("https://ziglang.org/download/{}/{}", version, filename);

    let install_dir = lang_install_dir(Lang::Zig, location)?;
    let zig_extracted = install_dir.join(format!("zig-{}-{}-{}", zig_os, zig_arch, version));
    let zig_bin = zig_extracted.join("zig");
    let zig_bin_exe = zig_extracted.join("zig.exe");
    if (zig_bin.exists() || zig_bin_exe.exists()) && which::which("zig").is_err() {
        // Installed but not on PATH
    } else if zig_bin.exists() || zig_bin_exe.exists() {
        terminal::info_indented("Zig already installed");
        return Ok(());
    }

    terminal::install_step(1, 4, &format!("Downloading Zig {}...", version));
    let body = download(&url)?;
    terminal::install_step(2, 4, "Extracting...");

    let archive_path = if location == InstallLocation::System {
        std::env::temp_dir().join(&filename)
    } else {
        install_dir.join(&filename)
    };
    fs::create_dir_all(archive_path.parent().unwrap())
        .map_err(|e| miette::miette!("Failed to create dir: {}", e))?;
    fs::write(&archive_path, &body)
        .map_err(|e| miette::miette!("Failed to write archive: {}", e))?;

    let status = if location == InstallLocation::System {
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
    Ok(())
}

fn install_dotnet(location: InstallLocation) -> Result<()> {
    let (arch, os) = platform();
    let version = "8.0.203";
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
        "https://dotnetcli.azureedge.net/dotnet/Sdk/{}/dotnet-sdk-{}-{}-{}.tar.gz",
        version, version, dotnet_os, dotnet_arch
    );

    let install_dir = lang_install_dir(Lang::CSharp, location)?;
    let dotnet_bin = install_dir.join("dotnet");
    if dotnet_bin.exists() && which::which("dotnet").is_err() {
        // Installed but not on PATH
    } else if dotnet_bin.exists() {
        terminal::info_indented(".NET already installed");
        return Ok(());
    }

    terminal::install_step(1, 4, &format!("Downloading .NET SDK {}...", version));
    let body = download(&url)?;
    terminal::install_step(2, 4, "Extracting...");

    if location == InstallLocation::System {
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
    Ok(())
}

fn download(url: &str) -> Result<Vec<u8>> {
    let mut response = ureq::get(url).call().map_err(|e| {
        miette::miette!("Failed to download {}: {}. Ensure you have network access.", url, e)
    })?;
    let body = response
        .body_mut()
        .with_config()
        .limit(200 * 1024 * 1024)
        .read_to_vec()
        .map_err(|e| miette::miette!("Failed to read download: {}", e))?;
    Ok(body)
}
