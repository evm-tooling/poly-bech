//! Fetch latest version from crates.io and compare with current; used for --version warning and
//! upgrade command.

/// Print a warning to stderr if current version is older than the latest on crates.io.
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

/// Fetch latest version string from crates.io. Returns None on any error or timeout.
pub fn fetch_latest_version() -> Option<String> {
    let response = ureq::get("https://crates.io/api/v1/crates/poly-bench")
        .timeout(std::time::Duration::from_secs(2))
        .call()
        .ok()?;
    let json: serde_json::Value = response.into_json().ok()?;
    let latest = json.get("crate")?.get("newest_version")?.as_str()?;
    Some(latest.to_string())
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
}
