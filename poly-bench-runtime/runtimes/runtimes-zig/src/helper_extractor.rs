//! Helper function extraction for Zig.

use std::collections::HashSet;

use poly_bench_traits::HelperFunctionExtractor;

static ZIG_BUILTINS: &[&str] = &[
    "@import",
    "@ptrCast",
    "@as",
    "@intCast",
    "@floatFromInt",
    "std",
    "std.mem",
    "std.fmt",
    "std.io",
    "std.time",
    "std.posix",
    "std.ArrayList",
    "std.heap",
];

pub(crate) struct ZigHelperFunctionExtractor;

impl HelperFunctionExtractor for ZigHelperFunctionExtractor {
    fn extract_functions(&self, code: &str) -> HashSet<String> {
        let mut functions = HashSet::new();
        for line in code.lines() {
            let trimmed = line.trim();
            if !trimmed.starts_with("fn ") {
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix("fn ") {
                if let Some(name_part) = rest.split('(').next() {
                    let name = name_part.trim();
                    if !name.is_empty() &&
                        name.chars()
                            .next()
                            .map(|c| c.is_ascii_alphabetic() || c == '_')
                            .unwrap_or(false)
                    {
                        functions.insert(name.to_string());
                    }
                }
            }
        }
        functions
    }

    fn builtins(&self) -> &'static [&'static str] {
        ZIG_BUILTINS
    }
}

pub(crate) static ZIG_HELPER_EXTRACTOR: ZigHelperFunctionExtractor = ZigHelperFunctionExtractor;
