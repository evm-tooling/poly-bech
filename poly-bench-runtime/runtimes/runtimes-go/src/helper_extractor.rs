//! Helper function extraction for Go

use std::collections::HashSet;

use poly_bench_traits::HelperFunctionExtractor;

static GO_BUILTINS: &[&str] = &[
    "len", "make", "append", "copy", "delete", "panic", "recover", "print", "println", "close",
    "cap", "new", "real", "imag", "complex", "error", "string", "int", "int8", "int16", "int32",
    "int64", "uint", "uint8", "uint16", "uint32", "uint64", "float32", "float64", "bool", "byte",
    "rune",
];

pub(crate) struct GoHelperFunctionExtractor;

impl HelperFunctionExtractor for GoHelperFunctionExtractor {
    fn extract_functions(&self, code: &str) -> HashSet<String> {
        let mut functions = HashSet::new();
        extract_go_functions(code, &mut functions);
        functions
    }

    fn builtins(&self) -> &'static [&'static str] {
        GO_BUILTINS
    }
}

pub(crate) static GO_HELPER_EXTRACTOR: GoHelperFunctionExtractor = GoHelperFunctionExtractor;

fn extract_go_functions(code: &str, functions: &mut HashSet<String>) {
    for line in code.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("func ") {
            let rest = &trimmed[5..];
            let name_start = if rest.starts_with('(') {
                if let Some(close_paren) = rest.find(')') {
                    close_paren + 1
                } else {
                    continue;
                }
            } else {
                0
            };

            let rest = rest[name_start..].trim_start();
            if let Some(paren_pos) = rest.find('(') {
                let name = rest[..paren_pos].trim();
                if !name.is_empty() && is_valid_identifier(name) {
                    functions.insert(name.to_string());
                }
            }
        }
    }
}

fn is_valid_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_alphanumeric() || c == '_')
}
