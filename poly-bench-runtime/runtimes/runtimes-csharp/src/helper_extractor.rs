//! Helper function extraction for C#.

use std::collections::HashSet;

use poly_bench_traits::HelperFunctionExtractor;

static CSHARP_BUILTINS: &[&str] = &[
    "Console",
    "Math",
    "Convert",
    "string",
    "int",
    "long",
    "double",
    "float",
    "decimal",
    "bool",
    "DateTime",
    "Guid",
    "Array",
    "List",
    "Dictionary",
    "HashSet",
    "Span",
    "ReadOnlySpan",
];

pub(crate) struct CSharpHelperFunctionExtractor;

impl HelperFunctionExtractor for CSharpHelperFunctionExtractor {
    fn extract_functions(&self, code: &str) -> HashSet<String> {
        let mut functions = HashSet::new();
        for line in code.lines() {
            let trimmed = line.trim();
            if !trimmed.contains('(') || !trimmed.contains(')') || !trimmed.contains('{') {
                continue;
            }
            let head = trimmed.split('(').next().unwrap_or("").trim();
            let maybe_name = head.split_whitespace().last().unwrap_or("");
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
        CSHARP_BUILTINS
    }
}

pub(crate) static CSHARP_HELPER_EXTRACTOR: CSharpHelperFunctionExtractor =
    CSharpHelperFunctionExtractor;
