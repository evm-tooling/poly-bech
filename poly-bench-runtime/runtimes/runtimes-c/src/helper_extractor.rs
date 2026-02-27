//! Helper function extraction for C.

use std::collections::HashSet;

use poly_bench_traits::HelperFunctionExtractor;

static C_BUILTINS: &[&str] = &[
    "printf", "fprintf", "snprintf", "puts", "malloc", "calloc", "realloc", "free", "memcpy",
    "memset", "memcmp", "strlen", "strcmp", "strncmp", "atoi", "atol", "strtol", "strtod",
    "sizeof", // C built-in operator, not a function
];

pub(crate) struct CHelperFunctionExtractor;

impl HelperFunctionExtractor for CHelperFunctionExtractor {
    fn extract_functions(&self, code: &str) -> HashSet<String> {
        let mut functions = HashSet::new();
        for line in code.lines() {
            let trimmed = line.trim();
            if !trimmed.contains('(') || !trimmed.contains(')') || !trimmed.contains('{') {
                continue;
            }

            let head = trimmed.split('(').next().unwrap_or("").trim();
            let maybe_name = head.split_whitespace().last().unwrap_or("").trim_start_matches('*');
            if !maybe_name.is_empty() &&
                maybe_name
                    .chars()
                    .next()
                    .map(|c| c.is_ascii_alphabetic() || c == '_')
                    .unwrap_or(false)
            {
                functions.insert(maybe_name.to_string());
            }
        }
        functions
    }

    fn builtins(&self) -> &'static [&'static str] {
        C_BUILTINS
    }
}

pub(crate) static C_HELPER_EXTRACTOR: CHelperFunctionExtractor = CHelperFunctionExtractor;
