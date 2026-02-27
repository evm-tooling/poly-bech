//! Runtime installer: download and install language runtimes (Go, Node, Rust, etc.)
//!
//! Installs to standard user-local paths and configures PATH:
//! - Unix: ~/.local/share/polybench/runtimes/<lang>/
//! - Windows: %LOCALAPPDATA%\polybench\runtimes\<lang>\

use crate::{runtime_check, terminal};
use flate2::read::GzDecoder;
use miette::Result;
use poly_bench_dsl::Lang;
use std::{
    env, fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};

/// Base directory for poly-bench managed runtimes.
/// Uses standard user-local paths (XDG on Unix, LOCALAPPDATA on Windows).
fn runtimes_base_dir() -> Result<PathBuf> {
    if cfg!(windows) {
        let local = env::var("LOCALAPPDATA")
            .or_else(|_| env::var("USERPROFILE").map(|p| format!("{}\\AppData\\Local", p)))
            .map_err(|_| miette::miette!("Could not determine LOCALAPPDATA or USERPROFILE"))?;
        Ok(PathBuf::from(local).join("polybench").join("runtimes"))
    } else {
        let home = env::var("HOME").or_else(|_| env::var("USERPROFILE")).map_err(|_| {
            miette::miette!("Could not determine home directory (HOME or USERPROFILE)")
        })?;
        Ok(PathBuf::from(home).join(".local").join("share").join("polybench").join("runtimes"))
    }
}

/// Legacy base dir (~/.polybench/runtimes) for backward compatibility.
fn legacy_runtimes_base_dir() -> Option<PathBuf> {
    let home = env::var("HOME").or_else(|_| env::var("USERPROFILE")).ok()?;
    Some(PathBuf::from(home).join(".polybench").join("runtimes"))
}

/// User-local bin directory for symlinks (~/.local/bin). Binaries here work globally.
fn local_bin_dir() -> Result<PathBuf> {
    if cfg!(windows) {
        let local = env::var("LOCALAPPDATA")
            .or_else(|_| env::var("USERPROFILE").map(|p| format!("{}\\AppData\\Local", p)))
            .map_err(|_| miette::miette!("Could not determine LOCALAPPDATA"))?;
        Ok(PathBuf::from(local).join("polybench").join("bin"))
    } else {
        let home = env::var("HOME")
            .or_else(|_| env::var("USERPROFILE"))
            .map_err(|_| miette::miette!("Could not determine home directory"))?;
        Ok(PathBuf::from(home).join(".local").join("bin"))
    }
}

/// System bin directory (/usr/local/bin). Requires sudo to write.
fn system_bin_dir() -> PathBuf {
    PathBuf::from("/usr/local/bin")
}

fn target_bin_dir(location: InstallLocation) -> Result<PathBuf> {
    match location {
        InstallLocation::UserLocal => local_bin_dir(),
        InstallLocation::System => Ok(system_bin_dir()),
    }
}

/// Symlink a binary into the target bin dir. Uses sudo for system install.
fn symlink_binary(src: &Path, bin_name: &str, location: InstallLocation) -> Result<()> {
    let bin_dir = target_bin_dir(location)?;
    let dest = bin_dir.join(bin_name);

    #[cfg(unix)]
    {
        if location == InstallLocation::System {
            let status = Command::new("sudo")
                .args(["ln", "-sf", src.to_str().unwrap(), dest.to_str().unwrap()])
                .status()
                .map_err(|e| miette::miette!("Failed to run sudo ln: {}", e))?;
            if !status.success() {
                return Err(miette::miette!(
                    "sudo ln failed. Ensure you have permission to write to /usr/local/bin."
                ));
            }
        } else {
            fs::create_dir_all(&bin_dir)
                .map_err(|e| miette::miette!("Failed to create {}: {}", bin_dir.display(), e))?;
            if dest.exists() {
                fs::remove_file(&dest).or_else(|_| fs::remove_dir_all(&dest)).map_err(|e| {
                    miette::miette!("Failed to remove existing {}: {}", dest.display(), e)
                })?;
            }
            std::os::unix::fs::symlink(src, &dest).map_err(|e| {
                miette::miette!("Failed to symlink {} -> {}: {}", src.display(), dest.display(), e)
            })?;
        }
    }
    #[cfg(windows)]
    {
        if location == InstallLocation::System {
            return Err(miette::miette!(
                "--system is not supported on Windows. Use default user-local install."
            ));
        }
        fs::create_dir_all(&bin_dir)
            .map_err(|e| miette::miette!("Failed to create {}: {}", bin_dir.display(), e))?;
        if dest.exists() {
            fs::remove_file(&dest).or_else(|_| fs::remove_dir_all(&dest)).map_err(|e| {
                miette::miette!("Failed to remove existing {}: {}", dest.display(), e)
            })?;
        }
        std::os::windows::fs::symlink_file(src, &dest)
            .or_else(|_| std::os::windows::fs::symlink_dir(src, &dest))
            .map_err(|e| {
                miette::miette!("Failed to symlink {} -> {}: {}", src.display(), dest.display(), e)
            })?;
    }
    Ok(())
}

/// Symlink all executables in a directory to the target bin dir.
fn symlink_bin_dir(bin_dir: &Path, location: InstallLocation) -> Result<()> {
    if !bin_dir.exists() {
        return Ok(());
    }
    let target = target_bin_dir(location)?;

    #[cfg(unix)]
    {
        if location == InstallLocation::System {
            for entry in fs::read_dir(bin_dir)
                .map_err(|e| miette::miette!("Failed to read {}: {}", bin_dir.display(), e))?
            {
                let entry =
                    entry.map_err(|e| miette::miette!("Failed to read dir entry: {}", e))?;
                let path = entry.path();
                if path.is_file() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        let dest = target.join(name);
                        let status = Command::new("sudo")
                            .args(["ln", "-sf", path.to_str().unwrap(), dest.to_str().unwrap()])
                            .status()
                            .map_err(|e| miette::miette!("Failed to run sudo ln: {}", e))?;
                        if !status.success() {
                            return Err(miette::miette!("sudo ln failed for {}", name));
                        }
                    }
                }
            }
        } else {
            fs::create_dir_all(&target)
                .map_err(|e| miette::miette!("Failed to create {}: {}", target.display(), e))?;
            for entry in fs::read_dir(bin_dir)
                .map_err(|e| miette::miette!("Failed to read {}: {}", bin_dir.display(), e))?
            {
                let entry =
                    entry.map_err(|e| miette::miette!("Failed to read dir entry: {}", e))?;
                let path = entry.path();
                if path.is_file() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        let dest = target.join(name);
                        if dest.exists() {
                            let _ = fs::remove_file(&dest).or_else(|_| fs::remove_dir_all(&dest));
                        }
                        std::os::unix::fs::symlink(&path, &dest).ok();
                    }
                }
            }
        }
    }
    #[cfg(windows)]
    {
        if location == InstallLocation::System {
            return Err(miette::miette!("--system is not supported on Windows."));
        }
        fs::create_dir_all(&target)
            .map_err(|e| miette::miette!("Failed to create {}: {}", target.display(), e))?;
        for entry in fs::read_dir(bin_dir)
            .map_err(|e| miette::miette!("Failed to read {}: {}", bin_dir.display(), e))?
        {
            let entry = entry.map_err(|e| miette::miette!("Failed to read dir entry: {}", e))?;
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    let dest = target.join(name);
                    if dest.exists() {
                        let _ = fs::remove_file(&dest).or_else(|_| fs::remove_dir_all(&dest));
                    }
                    let _ = std::os::windows::fs::symlink_file(&path, &dest)
                        .or_else(|_| std::os::windows::fs::symlink_dir(&path, &dest));
                }
            }
        }
    }
    Ok(())
}

/// Install directory for a specific language
fn lang_install_dir(lang: Lang) -> Result<PathBuf> {
    Ok(runtimes_base_dir()?.join(lang.as_str()))
}

/// Where to symlink installed binaries: user-local (~/.local/bin) or system (/usr/local/bin).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InstallLocation {
    /// ~/.local/bin (default, no sudo)
    UserLocal,
    /// /usr/local/bin (system-wide, requires sudo)
    System,
}

/// Returns true if poly-bench can auto-install this language.
/// C requires system package manager - we only provide install_hint.
pub fn can_auto_install(lang: Lang) -> bool {
    lang != Lang::C
}

/// Returns the PATH entry for a language if installed (in ~/.local/bin, /usr/local/bin, or runtimes
/// dir). Caller can prepend this to PATH so build commands find the binary in the current process.
pub fn polybench_runtime_path(lang: Lang) -> Option<PathBuf> {
    // Check /usr/local/bin (system install)
    let system_bin = system_bin_dir();
    let has_in_system = match lang {
        Lang::Go => system_bin.join("go").exists(),
        Lang::TypeScript => system_bin.join("node").exists(),
        Lang::Rust => system_bin.join("cargo").exists(),
        Lang::Python => system_bin.join("python3").exists() || system_bin.join("python").exists(),
        Lang::Zig => system_bin.join("zig").exists(),
        Lang::CSharp => system_bin.join("dotnet").exists(),
        Lang::C => false,
    };
    if has_in_system {
        return Some(system_bin);
    }
    // Check ~/.local/bin (user-local install)
    if let Ok(bin_dir) = local_bin_dir() {
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
            return Some(bin_dir);
        }
    }
    // Fallback: check runtimes dir
    let bases = [runtimes_base_dir().ok(), legacy_runtimes_base_dir()];
    for base in bases {
        let base = base?.join(lang.as_str());
        if !base.exists() {
            continue;
        }
        let path = match lang {
            Lang::Go => base.join("go").join("bin"),
            Lang::TypeScript => {
                let entry = std::fs::read_dir(&base)
                    .ok()?
                    .filter_map(|e| e.ok())
                    .find(|e| e.file_name().to_str().map_or(false, |n| n.starts_with("node-v")))?;
                entry.path().join("bin")
            }
            Lang::Rust => base.join(".cargo").join("bin"),
            Lang::Python => {
                let install = base.join("install");
                if cfg!(windows) {
                    install
                } else {
                    install.join("bin")
                }
            }
            Lang::Zig => {
                let entry = std::fs::read_dir(&base).ok()?.filter_map(|e| e.ok()).find(|e| {
                    let name = e.file_name();
                    let n = name.to_str().unwrap_or("");
                    (n.starts_with("zig-") || n.starts_with("zig_")) &&
                        (e.path().join("zig").exists() || e.path().join("zig.exe").exists())
                })?;
                entry.path()
            }
            Lang::CSharp => base.join("dotnet"),
            Lang::C => continue,
        };
        if path.exists() {
            return Some(path);
        }
    }
    None
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

    let install_dir = lang_install_dir(Lang::Go)?;
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

    fs::create_dir_all(&install_dir)
        .map_err(|e| miette::miette!("Failed to create install dir: {}", e))?;

    let decoder = GzDecoder::new(body.as_slice());
    let mut archive = tar::Archive::new(decoder);
    archive.unpack(&install_dir).map_err(|e| miette::miette!("Failed to extract Go: {}", e))?;

    terminal::install_step(3, 4, "Configuring PATH...");
    symlink_bin_dir(&bin_dir, location)?;
    terminal::install_step(4, 4, "Done");
    let dest = if location == InstallLocation::System { "/usr/local/bin" } else { "~/.local/bin" };
    terminal::success_indented(&format!(
        "Go installed at {} (symlinked to {})",
        bin_dir.display(),
        dest
    ));
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

    let install_dir = lang_install_dir(Lang::TypeScript)?;
    let bin_dir =
        install_dir.join(format!("node-v{}-{}-{}", version, node_os, node_arch)).join("bin");
    let alt_bin = install_dir.join("bin");
    if (bin_dir.exists() || alt_bin.exists()) && which::which("node").is_err() {
        // Installed but not on PATH
    } else if bin_dir.exists() || alt_bin.exists() {
        terminal::info_indented("Node.js already installed");
        return Ok(());
    }

    terminal::install_step(1, 4, &format!("Downloading Node.js {}...", version));
    let body = download(&url)?;
    terminal::install_step(2, 4, "Extracting...");

    fs::create_dir_all(&install_dir)
        .map_err(|e| miette::miette!("Failed to create install dir: {}", e))?;

    let decoder = GzDecoder::new(body.as_slice());
    let mut archive = tar::Archive::new(decoder);
    archive
        .unpack(&install_dir)
        .map_err(|e| miette::miette!("Failed to extract Node.js: {}", e))?;

    // Node tarball extracts to node-vX.Y.Z-os-arch/
    let extracted = install_dir
        .read_dir()
        .map_err(|e| miette::miette!("Failed to read install dir: {}", e))?
        .filter_map(|e| e.ok())
        .find(|e| {
            e.path().is_dir() && e.file_name().to_str().map_or(false, |n| n.starts_with("node-v"))
        });
    if let Some(entry) = extracted {
        let node_bin = entry.path().join("bin");
        terminal::install_step(3, 4, "Configuring PATH...");
        symlink_bin_dir(&node_bin, location)?;
        terminal::install_step(4, 4, "Done");
        let dest =
            if location == InstallLocation::System { "/usr/local/bin" } else { "~/.local/bin" };
        terminal::success_indented(&format!(
            "Node.js installed at {} (symlinked to {})",
            node_bin.display(),
            dest
        ));
    } else {
        return Err(miette::miette!("Node.js archive structure unexpected"));
    }
    Ok(())
}

fn install_rust(location: InstallLocation) -> Result<()> {
    let install_dir = lang_install_dir(Lang::Rust)?;
    let cargo_bin = install_dir.join(".cargo").join("bin");
    if cargo_bin.join("cargo").exists() || cargo_bin.join("cargo.exe").exists() {
        if which::which("cargo").is_ok() {
            terminal::info_indented("Rust already installed");
            return Ok(());
        }
        symlink_bin_dir(&cargo_bin, location)?;
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
    let rustup_path =
        install_dir.join(if ext == "exe" { "rustup-init.exe" } else { "rustup-init" });
    fs::create_dir_all(&install_dir)
        .map_err(|e| miette::miette!("Failed to create install dir: {}", e))?;
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
    let status = Command::new(&rustup_path)
        .args(["-y", "--default-toolchain", "stable"])
        .env("CARGO_HOME", &install_dir)
        .env("RUSTUP_HOME", install_dir.join(".rustup"))
        .status()
        .map_err(|e| miette::miette!("Failed to run rustup: {}", e))?;

    if !status.success() {
        return Err(miette::miette!("rustup failed with exit code {:?}", status.code()));
    }

    let cargo_bin = install_dir.join(".cargo").join("bin");
    terminal::install_step(3, 4, "Configuring PATH...");
    symlink_bin_dir(&cargo_bin, location)?;
    terminal::install_step(4, 4, "Done");
    let dest = if location == InstallLocation::System { "/usr/local/bin" } else { "~/.local/bin" };
    terminal::success_indented(&format!(
        "Rust installed at {} (symlinked to {})",
        cargo_bin.display(),
        dest
    ));
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

    let install_dir = lang_install_dir(Lang::Python)?;
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

    fs::create_dir_all(&install_dir)
        .map_err(|e| miette::miette!("Failed to create install dir: {}", e))?;

    let decoder = GzDecoder::new(body.as_slice());
    let mut archive = tar::Archive::new(decoder);
    archive.unpack(&install_dir).map_err(|e| miette::miette!("Failed to extract Python: {}", e))?;

    let path_to_add = if cfg!(windows) {
        install_dir.join("install")
    } else {
        install_dir.join("install").join("bin")
    };
    terminal::install_step(3, 4, "Configuring PATH...");
    symlink_bin_dir(&path_to_add, location)?;
    terminal::install_step(4, 4, "Done");
    let dest = if location == InstallLocation::System { "/usr/local/bin" } else { "~/.local/bin" };
    terminal::success_indented(&format!(
        "Python installed at {} (symlinked to {})",
        path_to_add.display(),
        dest
    ));
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

    let install_dir = lang_install_dir(Lang::Zig)?;
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

    fs::create_dir_all(&install_dir)
        .map_err(|e| miette::miette!("Failed to create install dir: {}", e))?;

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

    terminal::install_step(3, 4, "Configuring PATH...");
    let zig_bin = zig_extracted.join(if cfg!(windows) { "zig.exe" } else { "zig" });
    symlink_binary(&zig_bin, "zig", location)?;
    terminal::install_step(4, 4, "Done");
    let dest = if location == InstallLocation::System { "/usr/local/bin" } else { "~/.local/bin" };
    terminal::success_indented(&format!(
        "Zig installed at {} (symlinked to {})",
        zig_extracted.display(),
        dest
    ));
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

    let install_dir = lang_install_dir(Lang::CSharp)?;
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

    fs::create_dir_all(&install_dir)
        .map_err(|e| miette::miette!("Failed to create install dir: {}", e))?;

    let decoder = GzDecoder::new(body.as_slice());
    let mut archive = tar::Archive::new(decoder);
    archive.unpack(&install_dir).map_err(|e| miette::miette!("Failed to extract .NET: {}", e))?;

    terminal::install_step(3, 4, "Configuring PATH...");
    symlink_binary(&install_dir.join("dotnet"), "dotnet", location)?;
    terminal::install_step(4, 4, "Done");
    let dest = if location == InstallLocation::System { "/usr/local/bin" } else { "~/.local/bin" };
    terminal::success_indented(&format!(
        ".NET installed at {} (symlinked to {})",
        install_dir.display(),
        dest
    ));
    Ok(())
}

fn download(url: &str) -> Result<Vec<u8>> {
    let response = ureq::get(url).call().map_err(|e| {
        miette::miette!("Failed to download {}: {}. Ensure you have network access.", url, e)
    })?;
    let mut body = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut body)
        .map_err(|e| miette::miette!("Failed to read download: {}", e))?;
    Ok(body)
}
