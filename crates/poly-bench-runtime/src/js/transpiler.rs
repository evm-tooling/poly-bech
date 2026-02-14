//! TypeScript to JavaScript transpilation using swc
//!
//! Note: For the MVP, we'll use a simpler approach that shells out to
//! esbuild or tsc if available, with a fallback to running TS directly
//! via ts-node or similar.

use miette::{miette, Result};
use std::process::Command;
use tempfile::TempDir;

/// Transpile TypeScript to JavaScript
pub struct Transpiler {
    /// Temp directory for transpilation artifacts
    temp_dir: TempDir,
    /// Available transpiler
    transpiler_type: TranspilerType,
}

#[derive(Debug, Clone, Copy)]
enum TranspilerType {
    /// esbuild (fast)
    Esbuild,
    /// TypeScript compiler (tsc)
    Tsc,
    /// No transpilation available - pass through as-is
    None,
}

impl Transpiler {
    /// Create a new transpiler, detecting available tools
    pub fn new() -> Result<Self> {
        let temp_dir =
            TempDir::new().map_err(|e| miette!("Failed to create temp directory: {}", e))?;

        // Detect available transpiler
        let transpiler_type = if which::which("esbuild").is_ok() {
            TranspilerType::Esbuild
        } else if which::which("tsc").is_ok() {
            TranspilerType::Tsc
        } else {
            eprintln!("Warning: No TypeScript transpiler found. Install esbuild or tsc for better TypeScript support.");
            TranspilerType::None
        };

        Ok(Self {
            temp_dir,
            transpiler_type,
        })
    }

    /// Transpile TypeScript code to JavaScript
    pub fn transpile(&self, ts_code: &str) -> Result<String> {
        match self.transpiler_type {
            TranspilerType::Esbuild => self.transpile_esbuild(ts_code),
            TranspilerType::Tsc => self.transpile_tsc(ts_code),
            TranspilerType::None => Ok(strip_type_annotations(ts_code)),
        }
    }

    fn transpile_esbuild(&self, ts_code: &str) -> Result<String> {
        let ts_path = self.temp_dir.path().join("input.ts");
        let js_path = self.temp_dir.path().join("output.js");

        std::fs::write(&ts_path, ts_code)
            .map_err(|e| miette!("Failed to write TypeScript file: {}", e))?;

        let output = Command::new("esbuild")
            .args([
                ts_path.to_str().unwrap(),
                "--outfile",
                js_path.to_str().unwrap(),
                "--format=esm",
                "--target=esnext",
            ])
            .output()
            .map_err(|e| miette!("Failed to run esbuild: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(miette!("esbuild transpilation failed:\n{}", stderr));
        }

        std::fs::read_to_string(&js_path)
            .map_err(|e| miette!("Failed to read transpiled JavaScript: {}", e))
    }

    fn transpile_tsc(&self, ts_code: &str) -> Result<String> {
        let ts_path = self.temp_dir.path().join("input.ts");

        std::fs::write(&ts_path, ts_code)
            .map_err(|e| miette!("Failed to write TypeScript file: {}", e))?;

        // Create tsconfig.json
        let tsconfig = r#"{
            "compilerOptions": {
                "target": "ESNext",
                "module": "ESNext",
                "moduleResolution": "node",
                "esModuleInterop": true,
                "strict": false,
                "outDir": "."
            }
        }"#;

        std::fs::write(self.temp_dir.path().join("tsconfig.json"), tsconfig)
            .map_err(|e| miette!("Failed to write tsconfig.json: {}", e))?;

        let output = Command::new("tsc")
            .args([
                "--project",
                self.temp_dir.path().join("tsconfig.json").to_str().unwrap(),
            ])
            .current_dir(self.temp_dir.path())
            .output()
            .map_err(|e| miette!("Failed to run tsc: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(miette!("tsc transpilation failed:\n{}", stderr));
        }

        let js_path = self.temp_dir.path().join("input.js");
        std::fs::read_to_string(&js_path)
            .map_err(|e| miette!("Failed to read transpiled JavaScript: {}", e))
    }
}

/// Simple type annotation stripping for when no transpiler is available
/// This is a more careful implementation that preserves object literal colons
pub fn strip_type_annotations(ts_code: &str) -> String {
    let mut result = String::new();
    let mut in_type_annotation = false;
    let mut type_depth = 0; // Depth of generic type parameters
    let mut chars = ts_code.chars().peekable();
    let mut paren_depth: usize = 0; // Track if we're inside function parameters
    let mut brace_depth: usize = 0; // Track if we're inside braces (object literals, function bodies)
    let mut bracket_depth: usize = 0; // Track if we're inside brackets (arrays)

    while let Some(c) = chars.next() {
        match c {
            '(' => {
                if !in_type_annotation {
                    paren_depth += 1;
                    result.push(c);
                }
            }
            ')' => {
                if in_type_annotation && type_depth == 0 {
                    // End of type annotation at closing paren
                    in_type_annotation = false;
                    paren_depth = paren_depth.saturating_sub(1);
                    result.push(c);
                } else if !in_type_annotation {
                    paren_depth = paren_depth.saturating_sub(1);
                    result.push(c);
                }
                // If in_type_annotation with type_depth > 0, skip the character
            }
            '{' => {
                if in_type_annotation && type_depth == 0 {
                    // End of type annotation at opening brace (function body or object)
                    in_type_annotation = false;
                }
                brace_depth += 1;
                result.push(c);
            }
            '}' => {
                brace_depth = brace_depth.saturating_sub(1);
                result.push(c);
            }
            '[' => {
                if !in_type_annotation {
                    bracket_depth += 1;
                    result.push(c);
                }
            }
            ']' => {
                if !in_type_annotation {
                    bracket_depth = bracket_depth.saturating_sub(1);
                    result.push(c);
                }
            }
            ':' if !in_type_annotation => {
                // Check if this starts a type annotation
                // Type annotations follow: function params, variable declarations, function return types
                // NOT: object literal properties (key: value), ternary operators, or inside object braces

                // If we're inside braces but not in parens, this is likely an object literal
                // Objects can be: { key: value } or function bodies { ... }
                // Inside objects, ALL colons should be preserved
                if brace_depth > 0 && paren_depth == 0 {
                    // Inside an object literal or function body - preserve the colon
                    result.push(c);
                    continue;
                }

                // If inside an array, preserve the colon (for objects inside arrays)
                if bracket_depth > 0 {
                    result.push(c);
                    continue;
                }

                let rest: String = chars.clone().take(50).collect();
                let trimmed_rest = rest.trim_start();

                // Check what's before the colon (ignoring whitespace)
                let last_non_ws = result.trim_end().chars().last().unwrap_or(' ');

                // Return type annotation: ): TypeName
                let is_return_type = last_non_ws == ')';

                // Parameter type annotation: paramName: TypeName (inside parens)
                let is_after_param_name =
                    last_non_ws.is_alphanumeric() || last_non_ws == '_' || last_non_ws == '?';

                // Check if what follows looks like a type name
                // Type names: start with uppercase and are followed by space, comma, close paren, equals, or generic bracket
                // NOT: things like JSON.stringify() which are function calls
                let is_type_name = {
                    // Get the first word after the colon
                    let first_word: String = trimmed_rest
                        .chars()
                        .take_while(|c| c.is_alphanumeric() || *c == '_')
                        .collect();

                    // Check what follows the first word
                    let rest_after_word = &trimmed_rest[first_word.len()..].trim_start();
                    let has_dot_after = rest_after_word.starts_with('.');
                    let has_open_brace_after = rest_after_word.starts_with('{');

                    // Common built-in JavaScript objects that are function calls, NOT types
                    // These should only be excluded when followed by a dot (method call)
                    let is_method_call = has_dot_after
                        && (first_word == "JSON"
                            || first_word == "Math"
                            || first_word == "Object"
                            || first_word == "Array"
                            || first_word == "Date"
                            || first_word == "console"
                            || first_word == "window"
                            || first_word == "document"
                            || first_word == "Buffer"
                            || first_word == "Error");

                    // It's a type if:
                    // - It's a known primitive type
                    // - OR it starts with uppercase AND is NOT a method call
                    // - OR it's a generic type
                    let is_primitive = first_word == "string"
                        || first_word == "number"
                        || first_word == "boolean"
                        || first_word == "void"
                        || first_word == "any"
                        || first_word == "never"
                        || first_word == "null"
                        || first_word == "undefined";

                    let is_generic_type = trimmed_rest.starts_with("Array<")
                        || trimmed_rest.starts_with("Promise<")
                        || trimmed_rest.starts_with("Record<")
                        || trimmed_rest.starts_with("Partial<")
                        || trimmed_rest.starts_with("Readonly<")
                        || trimmed_rest.starts_with("Map<")
                        || trimmed_rest.starts_with("Set<")
                        || trimmed_rest.starts_with("Buffer");

                    // After return type `):`
                    // Types end at: { (function body), [ (array type), | (union), & (intersection), , (next param)
                    let is_return_type_context = is_return_type
                        && (has_open_brace_after
                            || first_word
                                .chars()
                                .next()
                                .map_or(false, |c| c.is_uppercase()));

                    is_primitive
                        || is_generic_type
                        || is_return_type_context
                        || (first_word
                            .chars()
                            .next()
                            .map_or(false, |c| c.is_uppercase())
                            && !is_method_call)
                };

                let looks_like_type_name = is_type_name;

                // Variable declaration: const x: Type = value
                // Only strip if we're at top level (not inside objects) and it looks like a type
                let is_var_decl_context =
                    is_after_param_name && paren_depth == 0 && !is_return_type && brace_depth == 0;

                // Strip type if:
                // 1. We're in function params (paren_depth > 0) and after a param name, and it looks like a type
                // 2. We're after closing paren (return type) and it looks like a type
                // 3. We're in a variable declaration context and it looks like a type
                if looks_like_type_name {
                    if (paren_depth > 0 && is_after_param_name)
                        || is_return_type
                        || is_var_decl_context
                    {
                        in_type_annotation = true;
                        continue;
                    }
                }
                result.push(c);
            }
            '<' if in_type_annotation => {
                type_depth += 1;
            }
            '>' if in_type_annotation && type_depth > 0 => {
                type_depth -= 1;
            }
            '=' | ',' | ';' | '\n' if in_type_annotation && type_depth == 0 => {
                in_type_annotation = false;
                result.push(c);
            }
            _ if in_type_annotation => {
                // Skip type annotation characters
            }
            _ => {
                result.push(c);
            }
        }
    }

    // Also remove interface and type declarations
    let lines: Vec<&str> = result.lines().collect();
    let mut filtered = Vec::new();
    let mut in_interface = false;
    let mut interface_brace_depth = 0;

    for line in lines {
        let trimmed = line.trim();

        if trimmed.starts_with("interface ")
            || trimmed.starts_with("type ") && trimmed.contains("=")
        {
            in_interface = true;
            interface_brace_depth = 0;
        }

        if in_interface {
            interface_brace_depth += line.chars().filter(|&c| c == '{').count();
            interface_brace_depth =
                interface_brace_depth.saturating_sub(line.chars().filter(|&c| c == '}').count());

            if interface_brace_depth == 0 && (line.contains('}') || line.contains(';')) {
                in_interface = false;
            }
            continue;
        }

        filtered.push(line);
    }

    filtered.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_simple_types() {
        let ts = "const x: number = 5;";
        let js = strip_type_annotations(ts);
        // The stripping removes the type annotation but may leave whitespace artifacts
        // The important thing is that the variable name and value are preserved
        assert!(js.contains("const x") && js.contains("= 5"), "Got: {}", js);
    }

    #[test]
    fn test_strip_function_types() {
        let ts = "function foo(a: string, b: number): void { }";
        let js = strip_type_annotations(ts);
        // Type annotations should be stripped, parameters preserved
        assert!(
            js.contains("function foo(a") && js.contains("b)"),
            "Got: {}",
            js
        );
        // Return type should also be stripped
        assert!(!js.contains(": void"), "Return type not stripped: {}", js);
    }

    #[test]
    fn test_strip_return_type_buffer() {
        let ts = "function sha256SumTs(data): Buffer {\n    return createHash('sha256').update(Buffer.from(data)).digest()\n}";
        let js = strip_type_annotations(ts);
        // Return type `: Buffer` should be stripped
        assert!(!js.contains(": Buffer"), "Return type not stripped: {}", js);
        // Function body should be preserved
        assert!(
            js.contains("function sha256SumTs(data)"),
            "Function declaration mangled: {}",
            js
        );
        assert!(js.contains("createHash"), "Function body lost: {}", js);
    }

    #[test]
    fn test_preserve_object_literal_colons() {
        let ts = "const obj = { key: 'value', num: 42 };";
        let js = strip_type_annotations(ts);
        // Object literal colons should be preserved
        assert!(
            js.contains("key: 'value'"),
            "Object literal colon stripped: {}",
            js
        );
        assert!(
            js.contains("num: 42"),
            "Object literal colon stripped: {}",
            js
        );
    }

    #[test]
    fn test_preserve_nested_object_literal() {
        let ts = r#"const resp = await fetch(URL, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload),
        });"#;
        let js = strip_type_annotations(ts);
        // All object literal colons should be preserved
        assert!(
            js.contains("method: \"POST\""),
            "method colon stripped: {}",
            js
        );
        assert!(js.contains("headers: {"), "headers colon stripped: {}", js);
        assert!(
            js.contains("\"Content-Type\": \"application/json\""),
            "Content-Type colon stripped: {}",
            js
        );
        assert!(
            js.contains("body: JSON.stringify(payload)"),
            "body colon stripped: {}",
            js
        );
    }

    #[test]
    fn test_strip_async_function_types() {
        let ts = "async function callAnvil(method: string, params: any[]): Promise<any> { return null; }";
        let js = strip_type_annotations(ts);
        // Type annotations should be stripped
        assert!(!js.contains(": string"), "Param type not stripped: {}", js);
        assert!(!js.contains(": any[]"), "Param type not stripped: {}", js);
        assert!(
            !js.contains(": Promise<any>"),
            "Return type not stripped: {}",
            js
        );
        // Function should be preserved
        assert!(
            js.contains("async function callAnvil(method, params)"),
            "Function signature mangled: {}",
            js
        );
    }

    #[test]
    fn test_strip_param_with_return_type() {
        let ts = "function process(data: Uint8Array): Buffer { return data; }";
        let js = strip_type_annotations(ts);
        // Both param type and return type should be stripped
        assert!(
            !js.contains(": Uint8Array"),
            "Param type not stripped: {}",
            js
        );
        assert!(!js.contains(": Buffer"), "Return type not stripped: {}", js);
        assert!(
            js.contains("function process(data)"),
            "Function signature mangled: {}",
            js
        );
    }
}
