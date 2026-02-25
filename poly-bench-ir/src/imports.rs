//! Import extraction for setup blocks
//!
//! This module provides language-specific import extraction from setup blocks.
//! Imports are separated from the body so generators can place them at the
//! correct location in generated code (e.g., top of file for Go/TypeScript).

use poly_bench_dsl::Lang;

/// Trait for language-specific import extraction from setup blocks
pub trait ImportExtractor: Send + Sync {
    /// The language this extractor handles
    fn lang(&self) -> Lang;

    /// Extract imports from setup block code, returning imports and remaining body
    fn extract(&self, setup: &str) -> ParsedSetup;
}

/// Parsed setup block with imports separated from body
#[derive(Debug, Clone)]
pub struct ParsedSetup {
    /// Extracted import statements (formatted for output)
    pub imports: Vec<String>,
    /// Remaining body code (non-import declarations)
    pub body: String,
}

impl ParsedSetup {
    pub fn new(imports: Vec<String>, body: String) -> Self {
        Self { imports, body }
    }

    /// Create a ParsedSetup with no imports (passthrough)
    pub fn passthrough(code: &str) -> Self {
        Self { imports: Vec::new(), body: code.to_string() }
    }
}

/// Extract imports from a Go setup block.
///
/// Handles both line-based and single-line (reconstructed) formats:
/// - `import "pkg"` (single import)
/// - `import ( "pkg1" "pkg2" )` (grouped imports)
/// - `import alias "pkg"` (aliased import)
/// - `import . "pkg"` (dot import)
/// - `import _ "pkg"` (blank import)
///
/// This works on tokenized/reconstructed code where all newlines may be converted to spaces.
pub fn extract_go_imports(setup: &str) -> ParsedSetup {
    let mut imports = Vec::new();
    let mut body = String::new();

    // First, try to handle line-based format (original source)
    if setup.contains('\n') {
        return extract_go_imports_line_based(setup);
    }

    // Handle single-line reconstructed code with imports
    // We need to find and extract import statements
    let mut remaining = setup.to_string();

    // Pattern 1: import ( ... ) - grouped imports
    while let Some(start) = remaining.find("import (") {
        // Add content before the import to body
        body.push_str(&remaining[..start]);
        remaining = remaining[start..].to_string();

        // Find the matching closing paren
        if let Some(end) = remaining.find(')') {
            let import_block = &remaining[8..end]; // Skip "import ("
                                                   // Extract each import from the block
            for part in import_block.split('"') {
                let trimmed = part.trim();
                if !trimmed.is_empty() && !trimmed.starts_with(')') {
                    // Check if it's an alias or special prefix
                    if trimmed.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '.') {
                        // This is an alias, wait for next quoted part
                    }
                }
            }
            // Parse the import block content
            extract_imports_from_block(import_block, &mut imports);
            remaining = remaining[end + 1..].to_string();
        } else {
            break; // Malformed, stop processing
        }
    }

    // Pattern 2: import "pkg" or import alias "pkg" - single imports
    // Handle remaining single imports
    let mut i = 0;
    let chars: Vec<char> = remaining.chars().collect();
    let mut new_body = String::new();

    while i < chars.len() {
        // Look for "import " not followed by "("
        if remaining[i..].starts_with("import ") && !remaining[i..].starts_with("import (") {
            // Find the import spec
            let import_start = i + 7; // Skip "import "
            let rest = &remaining[import_start..];

            // Find the end of this import (next import or non-import code)
            if let Some((import_spec, consumed)) = extract_single_import_spec(rest) {
                imports.push(import_spec);
                i = import_start + consumed;
                continue;
            }
        }

        new_body.push(chars[i]);
        i += 1;
    }

    // Combine body parts
    body.push_str(&new_body);

    ParsedSetup::new(imports, body.trim().to_string())
}

/// Extract Go imports from line-based source (with newlines preserved)
fn extract_go_imports_line_based(setup: &str) -> ParsedSetup {
    let mut imports = Vec::new();
    let mut body = String::new();
    let mut in_import_block = false;

    for line in setup.lines() {
        let trimmed = line.trim();

        // Skip empty lines and comments at the start
        if trimmed.is_empty() || trimmed.starts_with("//") {
            if in_import_block {
                continue;
            }
            if !body.is_empty() || !imports.is_empty() {
                body.push_str(line);
                body.push('\n');
            }
            continue;
        }

        // Single-line import: import "pkg" or import alias "pkg"
        if trimmed.starts_with("import ") && !trimmed.contains('(') {
            if let Some(import_spec) = extract_single_go_import_from_line(trimmed) {
                imports.push(import_spec);
            }
            continue;
        }

        // Start of import block: import (
        if trimmed.starts_with("import(") || trimmed.starts_with("import (") {
            in_import_block = true;
            continue;
        }

        if in_import_block {
            if trimmed == ")" || trimmed.starts_with(")") {
                in_import_block = false;
                continue;
            }

            if let Some(import_spec) = extract_go_import_from_block_line(trimmed) {
                imports.push(import_spec);
            }
            continue;
        }

        body.push_str(line);
        body.push('\n');
    }

    ParsedSetup::new(imports, body)
}

/// Extract a single Go import from a line like `import "pkg"` or `import alias "pkg"`
fn extract_single_go_import_from_line(line: &str) -> Option<String> {
    let rest = line.trim().strip_prefix("import ")?.trim();
    if rest.is_empty() {
        return None;
    }
    Some(rest.to_string())
}

/// Extract an import spec from inside an import block line
fn extract_go_import_from_block_line(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with("//") {
        return None;
    }
    Some(trimmed.to_string())
}

/// Extract imports from a Go import block content (content between `(` and `)`)
fn extract_imports_from_block(block: &str, imports: &mut Vec<String>) {
    // Split by quotes to find package paths
    let mut current_alias = String::new();
    let mut in_quotes = false;
    let mut current_pkg = String::new();

    for ch in block.chars() {
        if ch == '"' {
            if in_quotes {
                // End of package path
                let import_spec = if current_alias.is_empty() {
                    format!("\"{}\"", current_pkg)
                } else {
                    format!("{} \"{}\"", current_alias.trim(), current_pkg)
                };
                imports.push(import_spec);
                current_pkg.clear();
                current_alias.clear();
                in_quotes = false;
            } else {
                // Start of package path
                in_quotes = true;
            }
        } else if in_quotes {
            current_pkg.push(ch);
        } else if !ch.is_whitespace() {
            current_alias.push(ch);
        } else if !current_alias.is_empty() && ch.is_whitespace() {
            // End of alias or identifier
            if current_alias == "import" {
                current_alias.clear();
            }
        }
    }
}

/// Extract a single import spec from reconstructed code
/// Returns (import_spec, chars_consumed)
fn extract_single_import_spec(rest: &str) -> Option<(String, usize)> {
    let trimmed = rest.trim_start();
    let consumed_leading = rest.len() - trimmed.len();

    // Look for quoted string (the package path)
    let mut chars = trimmed.chars().peekable();
    let mut alias = String::new();

    // Check for alias (., _, or identifier)
    while let Some(&ch) = chars.peek() {
        if ch == '"' {
            break;
        }
        if ch.is_whitespace() {
            chars.next();
            continue;
        }
        alias.push(ch);
        chars.next();
    }

    // Now find the quoted package path
    if chars.peek() != Some(&'"') {
        return None;
    }
    chars.next(); // Skip opening quote

    let mut pkg_path = String::new();
    while let Some(ch) = chars.next() {
        if ch == '"' {
            break;
        }
        pkg_path.push(ch);
    }

    if pkg_path.is_empty() {
        return None;
    }

    let alias = alias.trim();
    let import_spec = if alias.is_empty() || alias == "import" {
        format!("\"{}\"", pkg_path)
    } else {
        format!("{} \"{}\"", alias, pkg_path)
    };

    // Calculate how many characters we consumed
    let _spec_len = if alias.is_empty() || alias == "import" {
        pkg_path.len() + 2 // quotes
    } else {
        alias.len() + 1 + pkg_path.len() + 2 // alias + space + quotes
    };

    // Find actual consumed length by looking for the closing quote in original
    let mut consumed = consumed_leading;
    let mut found_pkg = false;
    for ch in trimmed.chars() {
        consumed += 1;
        if ch == '"' {
            if found_pkg {
                break;
            }
            found_pkg = !found_pkg;
        }
    }

    Some((import_spec, consumed))
}

/// Extract imports from a TypeScript/JavaScript setup block.
///
/// Handles:
/// - `import { a, b } from 'pkg'`
/// - `import * as x from 'pkg'`
/// - `import x from 'pkg'`
/// - `import 'pkg'` (side-effect import)
/// - Multi-line imports with { ... }
pub fn extract_ts_imports(setup: &str) -> ParsedSetup {
    let mut imports = Vec::new();
    let mut body = String::new();
    let mut in_multiline_import = false;
    let mut current_import = String::new();

    for line in setup.lines() {
        let trimmed = line.trim();

        // Handle multi-line import continuation
        if in_multiline_import {
            current_import.push_str(line);
            current_import.push('\n');

            // Import is complete when we have balanced braces AND have the `from 'pkg'` part
            if is_import_complete(&current_import) {
                imports.push(current_import.trim().to_string());
                current_import.clear();
                in_multiline_import = false;
            }
            continue;
        }

        // Check if this is an import statement
        if trimmed.starts_with("import ") || trimmed.starts_with("import{") {
            // Check if it's a complete single-line import
            if is_import_complete(trimmed) {
                imports.push(trimmed.to_string());
            } else {
                // Start of multi-line import
                in_multiline_import = true;
                current_import = line.to_string();
                current_import.push('\n');
            }
            continue;
        }

        // Non-import line goes into body
        body.push_str(line);
        body.push('\n');
    }

    // Handle any unclosed import (shouldn't happen with valid code)
    if !current_import.is_empty() {
        imports.push(current_import.trim().to_string());
    }

    ParsedSetup::new(imports, body)
}

/// Check if a TypeScript import statement is complete
fn is_import_complete(import_text: &str) -> bool {
    // Must have balanced braces
    let open_braces = import_text.matches('{').count();
    let close_braces = import_text.matches('}').count();

    if open_braces != close_braces {
        return false;
    }

    // Must have a module path (quoted string after 'from' or standalone)
    // Check for 'from' followed by quotes, or just quotes for side-effect imports
    let has_from_clause = import_text.contains(" from ") &&
        (count_quotes(import_text, '\'') >= 2 || count_quotes(import_text, '"') >= 2);

    // Side effect import: import 'pkg' (no 'from', just the quoted path)
    let is_side_effect = !import_text.contains(" from ") &&
        (count_quotes(import_text, '\'') >= 2 || count_quotes(import_text, '"') >= 2);

    has_from_clause || is_side_effect
}

/// Count occurrences of a quote character
fn count_quotes(s: &str, quote: char) -> usize {
    s.chars().filter(|&c| c == quote).count()
}

/// Extract imports from a Rust setup block.
///
/// Handles:
/// - `use std::collections::HashMap;`
/// - `use std::{io::Read, fs::File};`
/// - `use crate::module::*;`
/// - `use super::something;`
/// - Multi-line use statements with { ... }
/// - `extern crate` statements
pub fn extract_rust_imports(setup: &str) -> ParsedSetup {
    let mut imports = Vec::new();
    let mut body = String::new();
    let mut in_multiline_use = false;
    let mut current_use = String::new();
    let mut brace_depth = 0;

    for line in setup.lines() {
        let trimmed = line.trim();

        // Handle multi-line use continuation
        if in_multiline_use {
            current_use.push_str(line);
            current_use.push('\n');

            // Track brace depth
            for ch in line.chars() {
                match ch {
                    '{' => brace_depth += 1,
                    '}' => brace_depth -= 1,
                    _ => {}
                }
            }

            // Use statement is complete when braces are balanced and ends with ;
            if brace_depth == 0 && trimmed.ends_with(';') {
                imports.push(current_use.trim().to_string());
                current_use.clear();
                in_multiline_use = false;
            }
            continue;
        }

        // Skip empty lines and comments at the start
        if trimmed.is_empty() || trimmed.starts_with("//") {
            if !body.is_empty() || !imports.is_empty() {
                body.push_str(line);
                body.push('\n');
            }
            continue;
        }

        // Check if this is a use statement
        if trimmed.starts_with("use ") || trimmed.starts_with("use\t") {
            // Count braces
            brace_depth = 0;
            for ch in trimmed.chars() {
                match ch {
                    '{' => brace_depth += 1,
                    '}' => brace_depth -= 1,
                    _ => {}
                }
            }

            // Check if it's a complete single-line use
            if brace_depth == 0 && trimmed.ends_with(';') {
                imports.push(trimmed.to_string());
            } else {
                // Start of multi-line use
                in_multiline_use = true;
                current_use = line.to_string();
                current_use.push('\n');
            }
            continue;
        }

        // Check for extern crate statements
        if trimmed.starts_with("extern crate ") {
            imports.push(trimmed.to_string());
            continue;
        }

        // Non-import line goes into body
        body.push_str(line);
        body.push('\n');
    }

    // Handle any unclosed use (shouldn't happen with valid code)
    if !current_use.is_empty() {
        imports.push(current_use.trim().to_string());
    }

    ParsedSetup::new(imports, body)
}

// ImportExtractor implementations

struct GoImportExtractor;
impl ImportExtractor for GoImportExtractor {
    fn lang(&self) -> Lang {
        Lang::Go
    }
    fn extract(&self, setup: &str) -> ParsedSetup {
        extract_go_imports(setup)
    }
}

struct TsImportExtractor;
impl ImportExtractor for TsImportExtractor {
    fn lang(&self) -> Lang {
        Lang::TypeScript
    }
    fn extract(&self, setup: &str) -> ParsedSetup {
        extract_ts_imports(setup)
    }
}

struct RustImportExtractor;
impl ImportExtractor for RustImportExtractor {
    fn lang(&self) -> Lang {
        Lang::Rust
    }
    fn extract(&self, setup: &str) -> ParsedSetup {
        extract_rust_imports(setup)
    }
}

/// Extract imports from a Python setup block.
///
/// Handles:
/// - `import foo`
/// - `import foo as bar`
/// - `from foo import bar`
/// - `from foo import bar, baz`
pub fn extract_python_imports(setup: &str) -> ParsedSetup {
    let mut imports = Vec::new();
    let mut body = String::new();

    for line in setup.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            if !body.is_empty() || !imports.is_empty() {
                body.push_str(line);
                body.push('\n');
            }
            continue;
        }

        if trimmed.starts_with("import ") || trimmed.starts_with("from ") {
            imports.push(trimmed.to_string());
            continue;
        }

        body.push_str(line);
        body.push('\n');
    }

    ParsedSetup::new(imports, body.trim().to_string())
}

struct PythonImportExtractor;
impl ImportExtractor for PythonImportExtractor {
    fn lang(&self) -> Lang {
        Lang::Python
    }
    fn extract(&self, setup: &str) -> ParsedSetup {
        extract_python_imports(setup)
    }
}

/// Extract imports from setup block code for the given language
pub fn extract_imports(lang: Lang, setup: &str) -> ParsedSetup {
    match lang {
        Lang::Go => GoImportExtractor.extract(setup),
        Lang::TypeScript => TsImportExtractor.extract(setup),
        Lang::Rust => RustImportExtractor.extract(setup),
        Lang::Python => PythonImportExtractor.extract(setup),
        _ => ParsedSetup::passthrough(setup),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_go_single_import() {
        let setup = r#"import "fmt"

func foo() {}"#;
        let parsed = extract_go_imports(setup);
        assert_eq!(parsed.imports, vec!["\"fmt\""]);
        assert!(parsed.body.contains("func foo()"));
    }

    #[test]
    fn test_go_abi_bench_setup() {
        // This is the actual setup from abi.bench
        let setup = r#"import "math/big"
        import "github.com/ethereum/go-ethereum/common"
        import "github.com/ChefBingbong/viem-go/abi"

        // Mirror ../go/abi_bench_test.go setup
        var benchERC20ABI *abi.ABI
        var balanceOfAddr = common.HexToAddress("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045")

        func init() {
            var err error
        }"#;

        let parsed = extract_go_imports(setup);

        // Should extract 3 imports
        assert_eq!(parsed.imports.len(), 3, "Expected 3 imports, got: {:?}", parsed.imports);

        // Body should NOT contain import statements
        assert!(
            !parsed.body.contains("import \""),
            "Body should not contain imports: {}",
            parsed.body
        );

        // Body should contain the rest
        assert!(parsed.body.contains("var benchERC20ABI"), "Body should contain var declaration");
    }

    #[test]
    fn test_go_single_line_reconstructed() {
        // This simulates what the parser's reconstruct_code produces
        let setup = r#"import "math/big" import "github.com/ethereum/go-ethereum/common" import "github.com/ChefBingbong/viem-go/abi" // Mirror setup var benchERC20ABI * abi . ABI var balanceOfAddr = common . HexToAddress ( "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045" )"#;

        let parsed = extract_go_imports(setup);

        // Should extract 3 imports
        assert_eq!(parsed.imports.len(), 3, "Expected 3 imports, got: {:?}", parsed.imports);
        assert!(parsed.imports.iter().any(|i| i.contains("math/big")), "Should have math/big");
        assert!(
            parsed.imports.iter().any(|i| i.contains("go-ethereum")),
            "Should have go-ethereum"
        );
        assert!(parsed.imports.iter().any(|i| i.contains("viem-go")), "Should have viem-go");

        // Body should NOT contain import statements
        assert!(
            !parsed.body.contains("import \""),
            "Body should not contain imports: {}",
            parsed.body
        );

        // Body should contain the rest
        assert!(parsed.body.contains("var benchERC20ABI"), "Body should contain var declaration");
    }

    #[test]
    fn test_go_import_block() {
        let setup = r#"import (
    "fmt"
    "time"
    "github.com/example/pkg"
)

var x = 1"#;
        let parsed = extract_go_imports(setup);
        assert_eq!(parsed.imports.len(), 3);
        assert!(parsed.imports.contains(&"\"fmt\"".to_string()));
        assert!(parsed.imports.contains(&"\"time\"".to_string()));
        assert!(parsed.imports.contains(&"\"github.com/example/pkg\"".to_string()));
        assert!(parsed.body.contains("var x = 1"));
    }

    #[test]
    fn test_go_aliased_import() {
        let setup = r#"import (
    "fmt"
    big "math/big"
    . "github.com/example/dot"
    _ "github.com/example/blank"
)

func bar() {}"#;
        let parsed = extract_go_imports(setup);
        assert_eq!(parsed.imports.len(), 4);
        assert!(parsed.imports.contains(&"big \"math/big\"".to_string()));
        assert!(parsed.imports.contains(&". \"github.com/example/dot\"".to_string()));
        assert!(parsed.imports.contains(&"_ \"github.com/example/blank\"".to_string()));
    }

    #[test]
    fn test_ts_single_import() {
        let setup = r#"import { foo } from 'bar'

const x = 1"#;
        let parsed = extract_ts_imports(setup);
        assert_eq!(parsed.imports.len(), 1);
        assert!(parsed.imports[0].contains("import { foo } from 'bar'"));
        assert!(parsed.body.contains("const x = 1"));
    }

    #[test]
    fn test_ts_multiple_imports() {
        let setup = r#"import { foo, bar } from 'pkg1'
import * as utils from 'pkg2'
import defaultExport from 'pkg3'
import 'side-effect'

const x = 1"#;
        let parsed = extract_ts_imports(setup);
        assert_eq!(parsed.imports.len(), 4);
        assert!(parsed.body.contains("const x = 1"));
    }

    #[test]
    fn test_ts_multiline_import() {
        let setup = r#"import {
    foo,
    bar,
    baz,
} from 'pkg'

const x = 1"#;
        let parsed = extract_ts_imports(setup);
        assert_eq!(parsed.imports.len(), 1);
        assert!(parsed.imports[0].contains("foo"));
        assert!(parsed.imports[0].contains("bar"));
        assert!(parsed.imports[0].contains("from 'pkg'"));
        assert!(parsed.body.contains("const x = 1"));
    }

    #[test]
    fn test_rust_single_use() {
        let setup = r#"use std::collections::HashMap;

fn foo() {}"#;
        let parsed = extract_rust_imports(setup);
        assert_eq!(parsed.imports.len(), 1);
        assert!(parsed.imports[0].contains("std::collections::HashMap"));
        assert!(parsed.body.contains("fn foo()"));
    }

    #[test]
    fn test_rust_multiple_uses() {
        let setup = r#"use std::io::Read;
use std::fs::File;
use crate::module::*;

fn bar() {}"#;
        let parsed = extract_rust_imports(setup);
        assert_eq!(parsed.imports.len(), 3);
        assert!(parsed.body.contains("fn bar()"));
    }

    #[test]
    fn test_rust_grouped_use() {
        let setup = r#"use std::{
    io::Read,
    fs::File,
    collections::HashMap,
};

fn baz() {}"#;
        let parsed = extract_rust_imports(setup);
        assert_eq!(parsed.imports.len(), 1);
        assert!(parsed.imports[0].contains("io::Read"));
        assert!(parsed.imports[0].contains("fs::File"));
        assert!(parsed.imports[0].contains("collections::HashMap"));
        assert!(parsed.body.contains("fn baz()"));
    }

    #[test]
    fn test_rust_extern_crate() {
        let setup = r#"extern crate serde;
use serde::Serialize;

fn qux() {}"#;
        let parsed = extract_rust_imports(setup);
        assert_eq!(parsed.imports.len(), 2);
        assert!(parsed.imports.iter().any(|i| i.contains("extern crate serde")));
        assert!(parsed.imports.iter().any(|i| i.contains("serde::Serialize")));
        assert!(parsed.body.contains("fn qux()"));
    }
}
