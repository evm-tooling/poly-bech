//! Go import extraction from setup blocks

use poly_bench_dsl::Lang;
use poly_bench_ir_traits::{ImportExtractor, ParsedSetup};

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
    let mut remaining = setup.to_string();

    // Pattern 1: import ( ... ) - grouped imports
    while let Some(start) = remaining.find("import (") {
        body.push_str(&remaining[..start]);
        remaining = remaining[start..].to_string();

        if let Some(end) = remaining.find(')') {
            let import_block = &remaining[8..end];
            extract_imports_from_block(import_block, &mut imports);
            remaining = remaining[end + 1..].to_string();
        } else {
            break;
        }
    }

    // Pattern 2: import "pkg" or import alias "pkg" - single imports
    let mut i = 0;
    let chars: Vec<char> = remaining.chars().collect();
    let mut new_body = String::new();

    while i < chars.len() {
        if remaining[i..].starts_with("import ") && !remaining[i..].starts_with("import (") {
            let import_start = i + 7;
            let rest = &remaining[import_start..];

            if let Some((import_spec, consumed)) = extract_single_import_spec(rest) {
                imports.push(import_spec);
                i = import_start + consumed;
                continue;
            }
        }

        new_body.push(chars[i]);
        i += 1;
    }

    body.push_str(&new_body);

    ParsedSetup::new(imports, body.trim().to_string())
}

fn extract_go_imports_line_based(setup: &str) -> ParsedSetup {
    let mut imports = Vec::new();
    let mut body = String::new();
    let mut in_import_block = false;

    for line in setup.lines() {
        let trimmed = line.trim();

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

        if trimmed.starts_with("import ") && !trimmed.contains('(') {
            if let Some(import_spec) = extract_single_go_import_from_line(trimmed) {
                imports.push(import_spec);
            }
            continue;
        }

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

fn extract_single_go_import_from_line(line: &str) -> Option<String> {
    let rest = line.trim().strip_prefix("import ")?.trim();
    if rest.is_empty() {
        return None;
    }
    Some(rest.to_string())
}

fn extract_go_import_from_block_line(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with("//") {
        return None;
    }
    Some(trimmed.to_string())
}

fn extract_imports_from_block(block: &str, imports: &mut Vec<String>) {
    let mut current_alias = String::new();
    let mut in_quotes = false;
    let mut current_pkg = String::new();

    for ch in block.chars() {
        if ch == '"' {
            if in_quotes {
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
                in_quotes = true;
            }
        } else if in_quotes {
            current_pkg.push(ch);
        } else if !ch.is_whitespace() {
            current_alias.push(ch);
        } else if !current_alias.is_empty() && ch.is_whitespace() {
            if current_alias == "import" {
                current_alias.clear();
            }
        }
    }
}

fn extract_single_import_spec(rest: &str) -> Option<(String, usize)> {
    let trimmed = rest.trim_start();
    let consumed_leading = rest.len() - trimmed.len();

    let mut chars = trimmed.chars().peekable();
    let mut alias = String::new();

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

    if chars.peek() != Some(&'"') {
        return None;
    }
    chars.next();

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

pub struct GoImportExtractor;

impl ImportExtractor for GoImportExtractor {
    fn lang(&self) -> Lang {
        Lang::Go
    }
    fn extract(&self, setup: &str) -> ParsedSetup {
        extract_go_imports(setup)
    }
}

pub static GO_IMPORT_EXTRACTOR: GoImportExtractor = GoImportExtractor;
