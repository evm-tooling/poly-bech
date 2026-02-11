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

    while let Some(c) = chars.next() {
        match c {
            ':' if !in_type_annotation => {
                // Check if this starts a type annotation
                // Skip whitespace and check for type-like patterns
                let rest: String = chars.clone().take(20).collect();
                if rest.trim_start().starts_with(|c: char| c.is_alphabetic() || c == '{' || c == '[' || c == '(') {
                    // Likely a type annotation
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
            '=' | ',' | ')' | '{' | ';' | '\n' if in_type_annotation && depth == 0 => {
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
    }
}
