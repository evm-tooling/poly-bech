//! Fixture resolution and hex parsing

use miette::{miette, Result};
use serde_json::Value;
use std::path::Path;

/// Decode a hex string (with or without 0x prefix) to bytes
pub fn decode_hex(s: &str) -> Result<Vec<u8>> {
    let s = s.trim();
    let s = s.strip_prefix("0x").unwrap_or(s);

    hex::decode(s).map_err(|e| miette!("Invalid hex string: {}", e))
}

/// Decode a base64 string to bytes
pub fn decode_base64(s: &str) -> Result<Vec<u8>> {
    let s = s.trim();
    use base64::Engine;
    base64::engine::general_purpose::STANDARD
        .decode(s)
        .map_err(|e| miette!("Invalid base64 string: {}", e))
}

/// Decode a UTF-8 text value to bytes
pub fn decode_utf8(s: &str) -> Result<Vec<u8>> {
    Ok(s.as_bytes().to_vec())
}

/// Decode a raw payload string to bytes
pub fn decode_raw(s: &str) -> Result<Vec<u8>> {
    Ok(s.as_bytes().to_vec())
}

fn resolve_source_path(path: &Path, base_dir: Option<&Path>) -> std::path::PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else if let Some(base) = base_dir {
        base.join(path)
    } else {
        path.to_path_buf()
    }
}

/// Load hex data from a file
pub fn load_hex_file(path: &Path, base_dir: Option<&Path>) -> Result<Vec<u8>> {
    let full_path = resolve_source_path(path, base_dir);

    let content = std::fs::read_to_string(&full_path)
        .map_err(|e| miette!("Failed to read fixture file {}: {}", full_path.display(), e))?;

    decode_hex(&content)
}

/// Load base64 data from a file
pub fn load_base64_file(path: &Path, base_dir: Option<&Path>) -> Result<Vec<u8>> {
    let full_path = resolve_source_path(path, base_dir);
    let content = std::fs::read_to_string(&full_path)
        .map_err(|e| miette!("Failed to read fixture file {}: {}", full_path.display(), e))?;
    decode_base64(&content)
}

/// Load UTF-8 data from a file
pub fn load_utf8_file(path: &Path, base_dir: Option<&Path>) -> Result<Vec<u8>> {
    let full_path = resolve_source_path(path, base_dir);
    let content = std::fs::read_to_string(&full_path)
        .map_err(|e| miette!("Failed to read fixture file {}: {}", full_path.display(), e))?;
    decode_utf8(&content)
}

/// Load raw bytes from a file
pub fn load_raw_file(path: &Path, base_dir: Option<&Path>) -> Result<Vec<u8>> {
    let full_path = resolve_source_path(path, base_dir);
    std::fs::read(&full_path)
        .map_err(|e| miette!("Failed to read fixture file {}: {}", full_path.display(), e))
}

/// Normalize JSON source and optional selector to bytes
pub fn normalize_json_to_bytes(input: &str, selector: Option<&str>) -> Result<Vec<u8>> {
    let value: Value =
        serde_json::from_str(input).map_err(|e| miette!("Invalid JSON fixture data: {}", e))?;
    let selected = select_json_value(&value, selector)?;
    match selected {
        Value::String(s) => Ok(s.as_bytes().to_vec()),
        _ => serde_json::to_vec(selected)
            .map_err(|e| miette!("Failed to serialize selected JSON value: {}", e)),
    }
}

/// Normalize CSV source and optional selector (`row,col`) to bytes.
pub fn normalize_csv_to_bytes(input: &str, selector: Option<&str>) -> Result<Vec<u8>> {
    let lines: Vec<&str> = input.lines().filter(|l| !l.trim().is_empty()).collect();
    if lines.is_empty() {
        return Err(miette!("CSV fixture data is empty"));
    }

    if let Some(sel) = selector {
        let (row_idx, col_idx) = parse_csv_selector(sel)?;
        let row = lines
            .get(row_idx)
            .ok_or_else(|| miette!("CSV selector row {} out of range", row_idx))?;
        let cols: Vec<&str> = row.split(',').collect();
        let col = cols
            .get(col_idx)
            .ok_or_else(|| miette!("CSV selector col {} out of range", col_idx))?;
        Ok(col.trim().as_bytes().to_vec())
    } else {
        Ok(input.as_bytes().to_vec())
    }
}

fn parse_csv_selector(selector: &str) -> Result<(usize, usize)> {
    let parts: Vec<&str> = selector.split(',').map(|s| s.trim()).collect();
    if parts.len() != 2 {
        return Err(miette!(
            "CSV selector must be in 'row,col' format, got '{}'",
            selector
        ));
    }
    let row = parts[0]
        .parse::<usize>()
        .map_err(|_| miette!("Invalid CSV selector row '{}'", parts[0]))?;
    let col = parts[1]
        .parse::<usize>()
        .map_err(|_| miette!("Invalid CSV selector col '{}'", parts[1]))?;
    Ok((row, col))
}

fn select_json_value<'a>(root: &'a Value, selector: Option<&str>) -> Result<&'a Value> {
    let Some(raw_selector) = selector else { return Ok(root) };
    let mut selector = raw_selector.trim();
    if selector.is_empty() || selector == "$" {
        return Ok(root);
    }
    if let Some(rest) = selector.strip_prefix("$.") {
        selector = rest;
    } else if let Some(rest) = selector.strip_prefix('$') {
        selector = rest;
    }

    let mut current = root;
    for segment in selector.split('.') {
        if segment.is_empty() {
            continue;
        }

        // Support key[index] and plain key
        if let Some(open) = segment.find('[') {
            let key = &segment[..open];
            let close = segment
                .find(']')
                .ok_or_else(|| miette!("Invalid JSON selector segment '{}'", segment))?;
            let idx_text = &segment[open + 1..close];
            let idx = idx_text
                .parse::<usize>()
                .map_err(|_| miette!("Invalid array index '{}' in selector", idx_text))?;

            current = if key.is_empty() {
                current
            } else {
                current
                    .get(key)
                    .ok_or_else(|| miette!("JSON selector key '{}' not found", key))?
            };
            current = current
                .get(idx)
                .ok_or_else(|| miette!("JSON selector index {} out of range", idx))?;
        } else {
            current = current
                .get(segment)
                .ok_or_else(|| miette!("JSON selector key '{}' not found", segment))?;
        }
    }

    Ok(current)
}

/// Extract fixture references from code
///
/// Looks for identifiers that match known fixture names
pub fn extract_fixture_refs(code: &str, known_fixtures: &[String]) -> Vec<String> {
    let mut refs = Vec::new();

    for fixture in known_fixtures {
        // Simple check: if the fixture name appears in the code
        // A more sophisticated version would parse the code
        if code.contains(fixture.as_str()) {
            refs.push(fixture.clone());
        }
    }

    refs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_hex() {
        assert_eq!(decode_hex("deadbeef").unwrap(), vec![0xde, 0xad, 0xbe, 0xef]);
        assert_eq!(decode_hex("0xdeadbeef").unwrap(), vec![0xde, 0xad, 0xbe, 0xef]);
        assert_eq!(decode_hex("  0xDEADBEEF  ").unwrap(), vec![0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn test_decode_hex_invalid() {
        assert!(decode_hex("ghij").is_err());
        assert!(decode_hex("0xgg").is_err());
    }

    #[test]
    fn test_decode_base64() {
        assert_eq!(decode_base64("aGVsbG8=").unwrap(), b"hello");
    }

    #[test]
    fn test_decode_base64_invalid() {
        assert!(decode_base64("%%%").is_err());
    }

    #[test]
    fn test_normalize_json_to_bytes() {
        let src = r#"{"items":[{"name":"alice"},{"name":"bob"}]}"#;
        let out = normalize_json_to_bytes(src, Some("$.items[1].name")).unwrap();
        assert_eq!(out, b"bob");
    }

    #[test]
    fn test_normalize_csv_to_bytes() {
        let src = "a,b,c\n1,2,3\n";
        let out = normalize_csv_to_bytes(src, Some("1,2")).unwrap();
        assert_eq!(out, b"3");
    }

    #[test]
    fn test_extract_fixture_refs() {
        let code = "hash.Keccak256(short_data)";
        let fixtures = vec!["short_data".to_string(), "long_data".to_string()];

        let refs = extract_fixture_refs(code, &fixtures);
        assert_eq!(refs, vec!["short_data"]);
    }
}
