//! Helper function extraction for TypeScript/JavaScript

use std::collections::HashSet;

use poly_bench_traits::HelperFunctionExtractor;

static TS_BUILTINS: &[&str] = &[
    "console",
    "Math",
    "JSON",
    "Array",
    "Object",
    "String",
    "Number",
    "Boolean",
    "parseInt",
    "parseFloat",
    "isNaN",
    "isFinite",
    "Date",
    "RegExp",
    "Error",
    "Map",
    "Set",
    "Promise",
    "Uint8Array",
    "Int32Array",
    "Float64Array",
    "ArrayBuffer",
    "DataView",
    "setTimeout",
    "setInterval",
    "clearTimeout",
    "clearInterval",
    "fetch",
    "require",
    "import",
];

pub(crate) struct TsHelperFunctionExtractor;

impl HelperFunctionExtractor for TsHelperFunctionExtractor {
    fn extract_functions(&self, code: &str) -> HashSet<String> {
        let mut functions = HashSet::new();
        extract_ts_functions(code, &mut functions);
        functions
    }

    fn builtins(&self) -> &'static [&'static str] {
        TS_BUILTINS
    }
}

pub(crate) static TS_HELPER_EXTRACTOR: TsHelperFunctionExtractor = TsHelperFunctionExtractor;

fn extract_ts_functions(code: &str, functions: &mut HashSet<String>) {
    for line in code.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("function ") {
            let rest = &trimmed[9..];
            if let Some(paren_pos) = rest.find('(') {
                let name = rest[..paren_pos].trim();
                if !name.is_empty() && is_valid_identifier(name) {
                    functions.insert(name.to_string());
                }
            }
        } else if trimmed.starts_with("const ") ||
            trimmed.starts_with("let ") ||
            trimmed.starts_with("var ")
        {
            let keyword_len = if trimmed.starts_with("const ") { 6 } else { 4 };
            let rest = &trimmed[keyword_len..];

            if let Some(eq_pos) = rest.find('=') {
                let name = rest[..eq_pos].trim();
                let after_eq = rest[eq_pos + 1..].trim();

                if (after_eq.contains("=>") || after_eq.starts_with("function")) &&
                    is_valid_identifier(name)
                {
                    functions.insert(name.to_string());
                }
            }
        } else if trimmed.starts_with("async function ") {
            let rest = &trimmed[15..];
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
