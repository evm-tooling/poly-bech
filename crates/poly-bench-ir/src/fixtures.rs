//! Fixture resolution and hex parsing

use miette::{Result, miette};
use std::path::Path;

/// Decode a hex string (with or without 0x prefix) to bytes
pub fn decode_hex(s: &str) -> Result<Vec<u8>> {
    let s = s.trim();
    let s = s.strip_prefix("0x").unwrap_or(s);
    
    hex::decode(s).map_err(|e| miette!("Invalid hex string: {}", e))
}

/// Load hex data from a file
pub fn load_hex_file(path: &Path, base_dir: Option<&Path>) -> Result<Vec<u8>> {
    let full_path = if path.is_absolute() {
        path.to_path_buf()
    } else if let Some(base) = base_dir {
        base.join(path)
    } else {
        path.to_path_buf()
    };

    let content = std::fs::read_to_string(&full_path)
        .map_err(|e| miette!("Failed to read fixture file {}: {}", full_path.display(), e))?;
    
    decode_hex(&content)
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
    fn test_extract_fixture_refs() {
        let code = "hash.Keccak256(short_data)";
        let fixtures = vec!["short_data".to_string(), "long_data".to_string()];
        
        let refs = extract_fixture_refs(code, &fixtures);
        assert_eq!(refs, vec!["short_data"]);
    }
}
