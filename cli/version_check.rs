//! Fetch latest version from GitHub releases and compare with current; used for --version warning
//! and upgrade command.

const GITHUB_REPO: &str = "evm-tooling/poly-bench";

/// Print a warning to stderr if current version is older than the latest on GitHub.
/// Non-fatal: on network/parse errors, we do nothing.
pub fn warn_if_outdated(current: &str) {
    if let Some(latest) = fetch_latest_version() {
        if is_older(current, &latest) {
            eprintln!(
                "A new version ({}) is available. Run `poly-bench upgrade` to update.",
                latest
            );
        }
    }
}

/// Fetch latest version string from GitHub releases. Returns None on any error or timeout.
pub fn fetch_latest_version() -> Option<String> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", GITHUB_REPO);
    let response = ureq::get(&url)
        .set("User-Agent", "poly-bench-cli")
        .set("Accept", "application/vnd.github.v3+json")
        .timeout(std::time::Duration::from_secs(5))
        .call()
        .ok()?;
    let json: serde_json::Value = response.into_json().ok()?;
    let tag_name = json.get("tag_name")?.as_str()?;
    let version = tag_name.trim_start_matches('v');
    Some(version.to_string())
}

/// Get the download URL for a specific version and platform.
/// Returns None if the platform is not supported.
pub fn get_download_url(version: &str) -> Option<String> {
    let artifact_name = get_artifact_name()?;
    Some(format!(
        "https://github.com/{}/releases/download/v{}/{}",
        GITHUB_REPO, version, artifact_name
    ))
}

/// Get the artifact name for the current platform.
fn get_artifact_name() -> Option<&'static str> {
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        Some("poly-bench-linux-x86_64")
    }
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        Some("poly-bench-macos-aarch64")
    }
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        Some("poly-bench-windows-x86_64.exe")
    }
    #[cfg(not(any(
        all(target_os = "linux", target_arch = "x86_64"),
        all(target_os = "macos", target_arch = "aarch64"),
        all(target_os = "windows", target_arch = "x86_64")
    )))]
    {
        None
    }
}

/// Compare semver strings: true if current < latest (current is older).
pub fn is_older(current: &str, latest: &str) -> bool {
    let cur = parse_semver(current);
    let lat = parse_semver(latest);
    match (cur, lat) {
        (Some(c), Some(l)) => (c.0, c.1, c.2) < (l.0, l.1, l.2),
        _ => false,
    }
}

fn parse_semver(s: &str) -> Option<(u64, u64, u64)> {
    let s = s.trim().trim_start_matches('v');
    let parts: Vec<&str> = s.splitn(3, '.').collect();
    if parts.len() != 3 {
        return None;
    }
    let major = parts[0].parse::<u64>().ok()?;
    let minor = parts[1].parse::<u64>().ok()?;
    let patch = parts[2].split('-').next()?.parse::<u64>().ok()?;
    Some((major, minor, patch))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_older() {
        assert!(is_older("0.1.0", "0.1.1"));
        assert!(is_older("0.1.0", "0.2.0"));
        assert!(is_older("0.1.0", "1.0.0"));
        assert!(!is_older("0.1.0", "0.1.0"));
        assert!(!is_older("0.2.0", "0.1.0"));
    }

    #[test]
    fn test_fetch_latest_version_returns_semver() {
        let version = fetch_latest_version()
            .expect("fetch_latest_version returned None — TLS or network issue");
        assert!(
            parse_semver(&version).is_some(),
            "returned version '{}' is not valid semver",
            version
        );
    }
}
