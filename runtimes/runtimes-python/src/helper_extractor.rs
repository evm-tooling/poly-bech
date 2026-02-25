//! Helper function extraction for Python

use std::collections::HashSet;

use poly_bench_lsp_traits::HelperFunctionExtractor;

static PYTHON_BUILTINS: &[&str] = &[
    "print",
    "len",
    "range",
    "str",
    "int",
    "float",
    "bool",
    "list",
    "dict",
    "set",
    "tuple",
    "type",
    "isinstance",
    "issubclass",
    "hasattr",
    "getattr",
    "setattr",
    "delattr",
    "open",
    "input",
    "sorted",
    "reversed",
    "enumerate",
    "zip",
    "map",
    "filter",
    "sum",
    "min",
    "max",
    "abs",
    "round",
    "pow",
    "divmod",
    "hex",
    "oct",
    "bin",
    "ord",
    "chr",
    "repr",
    "format",
    "id",
    "hash",
    "callable",
    "iter",
    "next",
    "slice",
    "super",
    "staticmethod",
    "classmethod",
    "property",
];

pub(crate) struct PythonHelperFunctionExtractor;

impl HelperFunctionExtractor for PythonHelperFunctionExtractor {
    fn extract_functions(&self, code: &str) -> HashSet<String> {
        let mut functions = HashSet::new();
        extract_python_functions(code, &mut functions);
        functions
    }

    fn builtins(&self) -> &'static [&'static str] {
        PYTHON_BUILTINS
    }
}

pub(crate) static PYTHON_HELPER_EXTRACTOR: PythonHelperFunctionExtractor =
    PythonHelperFunctionExtractor;

fn extract_python_functions(code: &str, functions: &mut HashSet<String>) {
    for line in code.lines() {
        let trimmed = line.trim();

        let def_pos = if trimmed.starts_with("def ") {
            Some(4)
        } else if trimmed.starts_with("async def ") {
            Some(10)
        } else {
            None
        };

        if let Some(start) = def_pos {
            let rest = &trimmed[start..];
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
