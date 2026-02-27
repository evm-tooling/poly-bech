//! Helper function extraction for Rust

use std::collections::HashSet;

use poly_bench_traits::HelperFunctionExtractor;

static RUST_BUILTINS: &[&str] = &[
    "vec",
    "println",
    "print",
    "format",
    "panic",
    "assert",
    "assert_eq",
    "assert_ne",
    "debug_assert",
    "todo",
    "unimplemented",
    "unreachable",
    "Some",
    "None",
    "Ok",
    "Err",
    "Box",
    "Rc",
    "Arc",
    "Vec",
    "String",
    "HashMap",
    "HashSet",
    "BTreeMap",
    "BTreeSet",
];

pub(crate) struct RustHelperFunctionExtractor;

impl HelperFunctionExtractor for RustHelperFunctionExtractor {
    fn extract_functions(&self, code: &str) -> HashSet<String> {
        let mut functions = HashSet::new();
        extract_rust_functions(code, &mut functions);
        functions
    }

    fn builtins(&self) -> &'static [&'static str] {
        RUST_BUILTINS
    }
}

pub(crate) static RUST_HELPER_EXTRACTOR: RustHelperFunctionExtractor = RustHelperFunctionExtractor;

fn extract_rust_functions(code: &str, functions: &mut HashSet<String>) {
    for line in code.lines() {
        let trimmed = line.trim();

        let fn_keyword_pos = trimmed.find("fn ");
        if let Some(pos) = fn_keyword_pos {
            let before_fn = &trimmed[..pos];
            let valid_prefix = before_fn.is_empty() ||
                before_fn
                    .trim()
                    .split_whitespace()
                    .all(|word| matches!(word, "pub" | "async" | "unsafe" | "const" | "extern"));

            if valid_prefix {
                let rest = &trimmed[pos + 3..];
                let end_pos = rest.find(|c| c == '(' || c == '<').unwrap_or(rest.len());
                let name = rest[..end_pos].trim();
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
