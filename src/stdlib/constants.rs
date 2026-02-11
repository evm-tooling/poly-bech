//! Constants module - provides mathematical constants
//!
//! This module provides commonly used mathematical constants that are
//! available across all supported languages.
//!
//! ## Available Constants
//!
//! - `std_PI` - Pi (π), the ratio of a circle's circumference to its diameter
//! - `std_E` - Euler's number (e), the base of natural logarithms
//!
//! ## Usage
//!
//! ```bench
//! use std::constants
//!
//! suite mathBench {
//!     bench piCalc {
//!         go: compute(std_PI)
//!         ts: compute(std_PI)
//!     }
//! }
//! ```

use crate::dsl::Lang;

/// Get the language-specific code for the constants module
pub fn get_code(lang: Lang) -> String {
    match lang {
        Lang::Go => GO_CONSTANTS.to_string(),
        Lang::TypeScript => TS_CONSTANTS.to_string(),
        Lang::Rust => RUST_CONSTANTS.to_string(),
        Lang::Python => PYTHON_CONSTANTS.to_string(),
    }
}

const GO_CONSTANTS: &str = r#"
// std::constants - Mathematical constants from poly-bench standard library
const std_PI float64 = 3.14159265358979323846
const std_E float64 = 2.71828182845904523536
"#;

const TS_CONSTANTS: &str = r#"
// std::constants - Mathematical constants from poly-bench standard library
const std_PI: number = 3.14159265358979323846;
const std_E: number = 2.71828182845904523536;
"#;

const RUST_CONSTANTS: &str = r#"
// std::constants - Mathematical constants from poly-bench standard library
const STD_PI: f64 = 3.14159265358979323846;
const STD_E: f64 = 2.71828182845904523536;
"#;

const PYTHON_CONSTANTS: &str = r#"
# std::constants - Mathematical constants from poly-bench standard library
std_PI = 3.14159265358979323846
std_E = 2.71828182845904523536
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_go_constants() {
        let code = get_code(Lang::Go);
        assert!(code.contains("std_PI"));
        assert!(code.contains("std_E"));
        assert!(code.contains("float64"));
    }

    #[test]
    fn test_ts_constants() {
        let code = get_code(Lang::TypeScript);
        assert!(code.contains("std_PI"));
        assert!(code.contains("std_E"));
        assert!(code.contains("number"));
    }

    #[test]
    fn test_rust_constants() {
        let code = get_code(Lang::Rust);
        assert!(code.contains("STD_PI"));
        assert!(code.contains("STD_E"));
        assert!(code.contains("f64"));
    }

    #[test]
    fn test_python_constants() {
        let code = get_code(Lang::Python);
        assert!(code.contains("std_PI"));
        assert!(code.contains("std_E"));
    }
}
