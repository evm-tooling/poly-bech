//! Standard library for poly-bench DSL
//!
//! Provides built-in modules that can be imported with `use std::module` syntax.
//! Each module provides language-specific implementations that are injected into
//! the generated code during codegen.

use crate::dsl::Lang;
use std::collections::HashSet;

pub mod anvil;
pub mod constants;

/// Valid stdlib module names
pub const VALID_MODULES: &[&str] = &["anvil", "constants"];

/// Check if a module name is a valid stdlib module
pub fn is_valid_module(name: &str) -> bool {
    VALID_MODULES.contains(&name)
}

/// Get the imports required by stdlib modules for a given language
pub fn get_stdlib_imports(imports: &HashSet<String>, lang: Lang) -> Vec<&'static str> {
    let mut all_imports = Vec::new();
    
    for module in imports {
        match module.as_str() {
            "anvil" => all_imports.extend(anvil::get_imports(lang)),
            "constants" => {} // constants module has no imports
            _ => {} // Unknown module - validation should catch this earlier
        }
    }
    
    all_imports
}

/// Get the code to inject for a given set of stdlib imports and target language
pub fn get_stdlib_code(imports: &HashSet<String>, lang: Lang) -> String {
    let mut code = String::new();
    
    for module in imports {
        match module.as_str() {
            "anvil" => code.push_str(&anvil::get_code(lang)),
            "constants" => code.push_str(&constants::get_code(lang)),
            _ => {} // Unknown module - validation should catch this earlier
        }
    }
    
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_modules() {
        assert!(is_valid_module("anvil"));
        assert!(is_valid_module("constants"));
        assert!(!is_valid_module("nonexistent"));
    }

    #[test]
    fn test_get_stdlib_code_go() {
        let mut imports = HashSet::new();
        imports.insert("constants".to_string());
        
        let code = get_stdlib_code(&imports, Lang::Go);
        assert!(code.contains("std_PI"));
        assert!(code.contains("std_E"));
        assert!(code.contains("3.14159"));
    }

    #[test]
    fn test_get_stdlib_code_ts() {
        let mut imports = HashSet::new();
        imports.insert("constants".to_string());
        
        let code = get_stdlib_code(&imports, Lang::TypeScript);
        assert!(code.contains("std_PI"));
        assert!(code.contains("std_E"));
        assert!(code.contains("3.14159"));
    }

    #[test]
    fn test_empty_imports() {
        let imports = HashSet::new();
        let code = get_stdlib_code(&imports, Lang::Go);
        assert!(code.is_empty());
    }

    #[test]
    fn test_get_stdlib_code_anvil_go() {
        let mut imports = HashSet::new();
        imports.insert("anvil".to_string());
        
        let code = get_stdlib_code(&imports, Lang::Go);
        assert!(code.contains("ANVIL_RPC_URL"));
        assert!(code.contains("os.Getenv"));
    }

    #[test]
    fn test_get_stdlib_code_anvil_ts() {
        let mut imports = HashSet::new();
        imports.insert("anvil".to_string());
        
        let code = get_stdlib_code(&imports, Lang::TypeScript);
        assert!(code.contains("ANVIL_RPC_URL"));
        assert!(code.contains("process.env"));
    }
}
