//! Standard library for poly-bench DSL
//!
//! Provides built-in modules that can be imported with `use std::module` syntax.
//! Each module provides language-specific implementations that are injected into
//! the generated code during codegen.
//!
//! ## Namespaced Access
//!
//! When you import a module with `use std::anvil`, you access its members via
//! the module name: `anvil.spawnAnvil()`, `anvil.ANVIL_RPC_URL`.
//!
//! This enables autocomplete: typing `anvil.` will suggest all available members.

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

/// Describes an exported symbol from a stdlib module
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StdlibSymbol {
    /// The symbol name (e.g., "spawnAnvil", "ANVIL_RPC_URL")
    pub name: &'static str,
    /// The kind of symbol
    pub kind: StdlibSymbolKind,
    /// Short description
    pub description: &'static str,
    /// Detailed documentation (markdown)
    pub documentation: &'static str,
}

/// The kind of stdlib symbol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StdlibSymbolKind {
    /// A function that can be called (e.g., spawnAnvil())
    Function,
    /// A constant value (e.g., ANVIL_RPC_URL, std_PI)
    Constant,
    /// A variable
    Variable,
}

/// Get all symbols exported by a module
pub fn get_module_symbols(module: &str) -> &'static [StdlibSymbol] {
    match module {
        "anvil" => &ANVIL_SYMBOLS,
        "constants" => &CONSTANTS_SYMBOLS,
        _ => &[],
    }
}

/// Symbols exported by std::anvil
pub static ANVIL_SYMBOLS: &[StdlibSymbol] = &[
    StdlibSymbol {
        name: "spawnAnvil",
        kind: StdlibSymbolKind::Function,
        description: "Spawn a local Anvil Ethereum node",
        documentation: "**anvil.spawnAnvil** `()` or `(fork: \"url\")`\n\n\
            Spawn a local Anvil Ethereum node.\n\n\
            Anvil is started before benchmarks and stopped after.\n\
            The RPC URL is available as `anvil.ANVIL_RPC_URL` in benchmark code.\n\n\
            **Options:**\n\
            - `fork: \"url\"` - Fork from an existing chain\n\n\
            **Example:**\n\
            ```\nglobalSetup {\n    anvil.spawnAnvil()\n}\n```",
    },
    StdlibSymbol {
        name: "ANVIL_RPC_URL",
        kind: StdlibSymbolKind::Constant,
        description: "Anvil RPC endpoint URL",
        documentation: "**anvil.ANVIL_RPC_URL** `string`\n\n\
            The RPC endpoint URL for the spawned Anvil node.\n\n\
            Automatically set when `anvil.spawnAnvil()` is called.\n\n\
            **Example:**\n\
            ```go\nresp, err := http.Post(anvil.ANVIL_RPC_URL, \"application/json\", body)\n```\n\n\
            ```typescript\nfetch(anvil.ANVIL_RPC_URL, { method: 'POST', body })\n```",
    },
];

/// Symbols exported by std::constants
pub static CONSTANTS_SYMBOLS: &[StdlibSymbol] = &[
    StdlibSymbol {
        name: "PI",
        kind: StdlibSymbolKind::Constant,
        description: "Pi constant (≈ 3.14159)",
        documentation: "**constants.PI** `float64`\n\n\
            The mathematical constant Pi (π ≈ 3.141592653589793).\n\n\
            **Example:**\n\
            ```go\narea := constants.PI * radius * radius\n```",
    },
    StdlibSymbol {
        name: "E",
        kind: StdlibSymbolKind::Constant,
        description: "Euler's number (≈ 2.71828)",
        documentation: "**constants.E** `float64`\n\n\
            Euler's number (e ≈ 2.718281828459045).\n\n\
            **Example:**\n\
            ```go\nresult := math.Pow(constants.E, x)\n```",
    },
];

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
