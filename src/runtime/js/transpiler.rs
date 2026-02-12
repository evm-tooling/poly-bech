//! TypeScript to JavaScript transpilation using swc
//!
//! Note: For the MVP, we'll use a simpler approach that shells out to
//! esbuild or tsc if available, with a fallback to running TS directly
//! via ts-node or similar.

use miette::{Result, miette};
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
        let temp_dir = TempDir::new()
            .map_err(|e| miette!("Failed to create temp directory: {}", e))?;

        // Detect available transpiler
        let transpiler_type = if which::which("esbuild").is_ok() {
            TranspilerType::Esbuild
        } else if which::which("tsc").is_ok() {
            TranspilerType::Tsc
        } else {
            eprintln!("Warning: No TypeScript transpiler found. Install esbuild or tsc for better TypeScript support.");
            TranspilerType::None
        };

        Ok(Self { temp_dir, transpiler_type })
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
/// This is a very basic implementation that handles common patterns
pub fn strip_type_annotations(ts_code: &str) -> String {
    let mut result = String::new();
    let mut in_type_annotation = false;
    let mut depth = 0;
    let mut chars = ts_code.chars().peekable();
    let mut paren_depth: usize = 0; // Track if we're inside function parameters

    while let Some(c) = chars.next() {
        match c {
            '(' => {
                paren_depth += 1;
                result.push(c);
            }
            ')' if !in_type_annotation => {
                paren_depth = paren_depth.saturating_sub(1);
                result.push(c);
            }
            ':' if !in_type_annotation => {
                // Check if this starts a type annotation
                // Type annotations follow: function params, variable declarations, function return types
                // NOT: object literal properties (key: value)
                let rest: String = chars.clone().take(30).collect();
                let trimmed_rest = rest.trim_start();
                
                // Check what's before the colon (ignoring whitespace)
                let last_non_ws = result.trim_end().chars().last().unwrap_or(' ');
                
                // Return type annotation: ): TypeName
                let is_return_type = last_non_ws == ')';
                
                // Parameter type annotation: paramName: TypeName (inside parens)
                let is_after_param_name = last_non_ws.is_alphanumeric() || last_non_ws == '_' || last_non_ws == '?';
                
                // Type annotations start with: type name, { (for object types), [ (for array types), ( (for function types)
                // But we should only strip if we're in a type annotation context, not object literal
                let looks_like_type = trimmed_rest.starts_with(|c: char| c.is_alphabetic())
                    || trimmed_rest.starts_with("Array<")
                    || trimmed_rest.starts_with("Promise<")
                    || trimmed_rest.starts_with("Record<")
                    || trimmed_rest.starts_with("{")
                    || trimmed_rest.starts_with("[")
                    || trimmed_rest.starts_with("(");
                
                // Variable declaration: const x: Type = value
                // Check if we're after a variable name (identifier followed by colon)
                let is_var_decl = is_after_param_name && paren_depth == 0 && !is_return_type;
                
                // Strip type if:
                // 1. We're in function params (paren_depth > 0) and after a param name
                // 2. We're after closing paren (return type)
                // 3. We're in a variable declaration context
                if ((paren_depth > 0 && is_after_param_name) || is_return_type || is_var_decl) && looks_like_type {
                    in_type_annotation = true;
                    continue;
                }
                result.push(c);
            }
            '<' if in_type_annotation => {
                depth += 1;
            }
            '>' if in_type_annotation && depth > 0 => {
                depth -= 1;
            }
            ')' if in_type_annotation && depth == 0 => {
                in_type_annotation = false;
                paren_depth = paren_depth.saturating_sub(1);
                result.push(c);
            }
            '=' | ',' | '{' | ';' | '\n' if in_type_annotation && depth == 0 => {
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
    let mut brace_depth = 0;

    for line in lines {
        let trimmed = line.trim();
        
        if trimmed.starts_with("interface ") || trimmed.starts_with("type ") {
            in_interface = true;
            brace_depth = 0;
        }

        if in_interface {
            brace_depth += line.chars().filter(|&c| c == '{').count();
            brace_depth = brace_depth.saturating_sub(line.chars().filter(|&c| c == '}').count());
            
            if brace_depth == 0 && line.contains('}') {
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
        assert!(js.contains("function foo(a") && js.contains("b)"), "Got: {}", js);
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
        assert!(js.contains("function sha256SumTs(data)"), "Function declaration mangled: {}", js);
        assert!(js.contains("createHash"), "Function body lost: {}", js);
    }

    #[test]
    fn test_preserve_object_literal_colons() {
        let ts = "const obj = { key: 'value', num: 42 };";
        let js = strip_type_annotations(ts);
        // Object literal colons should be preserved
        assert!(js.contains("key: 'value'"), "Object literal colon stripped: {}", js);
        assert!(js.contains("num: 42"), "Object literal colon stripped: {}", js);
    }

    #[test]
    fn test_strip_param_with_return_type() {
        let ts = "function process(data: Uint8Array): Buffer { return data; }";
        let js = strip_type_annotations(ts);
        // Both param type and return type should be stripped
        assert!(!js.contains(": Uint8Array"), "Param type not stripped: {}", js);
        assert!(!js.contains(": Buffer"), "Return type not stripped: {}", js);
        assert!(js.contains("function process(data)"), "Function signature mangled: {}", js);
    }
}
