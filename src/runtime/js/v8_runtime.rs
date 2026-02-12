//! V8-based JavaScript runtime
//!
//! For the MVP, we use subprocess execution with Node.js as the V8 embedding
//! requires significant setup. This provides a working implementation that
//! can be upgraded to embedded V8 later.

use crate::dsl::Lang;
use crate::ir::{BenchmarkSpec, SuiteIR};
use crate::runtime::js::{codegen, builtins, transpiler};
use crate::runtime::measurement::Measurement;
use crate::runtime::traits::Runtime;
use async_trait::async_trait;
use miette::{Result, miette};
use std::path::PathBuf;
use tempfile::TempDir;

/// JavaScript runtime using Node.js subprocess
pub struct JsRuntime {
    /// Temp directory for generated files
    temp_dir: Option<TempDir>,
    /// Path to node binary
    node_binary: PathBuf,
    /// Generated benchmark code
    generated_code: Option<String>,
    /// Project root directory (where package.json/node_modules is located)
    project_root: Option<PathBuf>,
    /// Anvil RPC URL if std::anvil is enabled
    anvil_rpc_url: Option<String>,
}

impl JsRuntime {
    pub fn new() -> Result<Self> {
        let node_binary = which::which("node")
            .map_err(|_| miette!("Node.js not found in PATH. Please install Node.js."))?;

        Ok(Self {
            temp_dir: None,
            node_binary,
            generated_code: None,
            project_root: None,
            anvil_rpc_url: None,
        })
    }
    
    /// Set the project root directory where package.json/node_modules is located
    pub fn set_project_root(&mut self, path: Option<PathBuf>) {
        self.project_root = path;
    }
    
    /// Set the Anvil RPC URL to pass to subprocess
    pub fn set_anvil_rpc_url(&mut self, url: String) {
        self.anvil_rpc_url = Some(url);
    }
}

impl Default for JsRuntime {
    fn default() -> Self {
        Self::new().expect("Failed to create JS runtime")
    }
}

#[async_trait]
impl Runtime for JsRuntime {
    fn name(&self) -> &'static str {
        "JavaScript Runtime (Node.js)"
    }

    fn lang(&self) -> Lang {
        Lang::TypeScript
    }

    async fn initialize(&mut self, suite: &SuiteIR) -> Result<()> {
        // Create temp directory
        let temp_dir = TempDir::new()
            .map_err(|e| miette!("Failed to create temp directory: {}", e))?;

        // Generate TypeScript code for the suite
        let ir = crate::ir::BenchmarkIR::new(vec![suite.clone()]);
        let code = codegen::generate(&ir)?;

        self.generated_code = Some(code);
        self.temp_dir = Some(temp_dir);

        Ok(())
    }

    async fn run_benchmark(&mut self, spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<Measurement> {
        // Generate standalone script for this benchmark
        let script = generate_standalone_script(spec, suite)?;

        // Determine where to write and run the benchmark
        let (script_path, working_dir) = if let Some(ref project_root) = self.project_root {
            // When using .polybench/runtime-env/ts, write directly there; else use .polybench subdir
            let is_runtime_env = project_root.as_os_str().to_string_lossy().contains("runtime-env");
            let script_path = if is_runtime_env {
                project_root.join("bench.mjs")
            } else {
                let bench_dir = project_root.join(".polybench");
                std::fs::create_dir_all(&bench_dir)
                    .map_err(|e| miette!("Failed to create .polybench directory: {}", e))?;
                bench_dir.join("bench.mjs")
            };
            (script_path, project_root.clone())
        } else {
            // Fall back to temp directory
            let temp_dir = self.temp_dir.as_ref()
                .ok_or_else(|| miette!("Runtime not initialized"))?;
            
            let script_path = temp_dir.path().join("bench.js");
            (script_path, temp_dir.path().to_path_buf())
        };

        std::fs::write(&script_path, &script)
            .map_err(|e| miette!("Failed to write benchmark script: {}", e))?;

        // Run with Node.js from the working directory (which has node_modules)
        let mut cmd = tokio::process::Command::new(&self.node_binary);
        cmd.arg(&script_path)
            .current_dir(&working_dir);
        
        // Pass Anvil RPC URL if available
        if let Some(ref url) = self.anvil_rpc_url {
            cmd.env("ANVIL_RPC_URL", url);
        }
        
        let output = cmd.output()
            .await
            .map_err(|e| miette!("Failed to run Node.js: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(miette!("JavaScript benchmark failed:\n{}", stderr));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        parse_benchmark_result(&stdout)
    }

    async fn shutdown(&mut self) -> Result<()> {
        self.temp_dir = None;
        self.generated_code = None;
        Ok(())
    }
}

/// Strip TypeScript-specific syntax that Node.js can't handle
/// Delegates to the more robust implementation in transpiler module
fn strip_typescript_syntax(code: &str) -> String {
    transpiler::strip_type_annotations(code)
}

/// Generate a standalone JavaScript benchmark script
fn generate_standalone_script(spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<String> {
    let impl_code = spec.get_impl(Lang::TypeScript)
        .ok_or_else(|| miette!("No TypeScript implementation for benchmark {}", spec.name))?;

    let mut script = String::new();

    // User imports first (ES modules require imports at top of file)
    if let Some(user_imports) = suite.imports.get(&Lang::TypeScript) {
        for import_stmt in user_imports {
            script.push_str(import_stmt);
            script.push('\n');
        }
        if !user_imports.is_empty() {
            script.push('\n');
        }
    }

    // Add harness
    script.push_str(builtins::BENCHMARK_HARNESS);
    script.push_str("\n\n");

    // Phase 1: Add declarations section
    if let Some(declarations) = suite.declarations.get(&Lang::TypeScript) {
        if !declarations.trim().is_empty() {
            script.push_str("// Declarations\n");
            script.push_str(&strip_typescript_syntax(declarations));
            script.push_str("\n\n");
        }
    }

    // Phase 1: Add init section (with async support)
    if let Some(init_code) = suite.init_code.get(&Lang::TypeScript) {
        if !init_code.trim().is_empty() {
            let is_async = suite.has_async_init(Lang::TypeScript);
            if is_async {
                script.push_str("// Async init\n");
                script.push_str("await (async () => {\n");
                script.push_str(&strip_typescript_syntax(init_code));
                script.push_str("\n})();\n\n");
            } else {
                script.push_str("// Init\n");
                script.push_str("(() => {\n");
                script.push_str(&strip_typescript_syntax(init_code));
                script.push_str("\n})();\n\n");
            }
        }
    }

    // Phase 1: Add helper functions
    if let Some(helpers) = suite.helpers.get(&Lang::TypeScript) {
        if !helpers.trim().is_empty() {
            script.push_str("// Helpers\n");
            script.push_str(&strip_typescript_syntax(helpers));
            script.push_str("\n\n");
        }
    }

    // Add fixtures
    for fixture_name in &spec.fixture_refs {
        if let Some(fixture) = suite.get_fixture(fixture_name) {
            if let Some(fixture_impl) = fixture.implementations.get(&Lang::TypeScript) {
                // Wrap in IIFE if it contains multiple statements (has return)
                let stripped = strip_typescript_syntax(fixture_impl);
                if stripped.contains("return") {
                    script.push_str(&format!("const {} = (() => {{\n{}\n}})();\n", fixture_name, stripped));
                } else {
                    script.push_str(&format!("const {} = {};\n", fixture_name, stripped));
                }
            } else if !fixture.data.is_empty() {
                script.push_str(&builtins::generate_fixture_code(fixture_name, &fixture.as_hex()));
            }
        }
    }
    script.push_str("\n");

    // Phase 3: Get lifecycle hooks
    let before_hook = spec.before_hooks.get(&Lang::TypeScript);
    let after_hook = spec.after_hooks.get(&Lang::TypeScript);
    let each_hook = spec.each_hooks.get(&Lang::TypeScript);

    // Phase 3: Before hook (runs once before benchmark)
    if let Some(before) = before_hook {
        script.push_str("// Before hook\n");
        let stripped = strip_typescript_syntax(before);
        if stripped.contains("await") {
            script.push_str(&format!("await (async () => {{ {} }})();\n", stripped));
        } else {
            script.push_str(&format!("{};\n", stripped));
        }
        script.push_str("\n");
    }

    // Generate benchmark execution with hooks
    if each_hook.is_some() {
        // Custom benchmark loop with each hook
        script.push_str(&format!("const __result = __polybench.runBenchmarkWithHook(\n"));
        script.push_str("    function() {\n");
        script.push_str("        ");
        script.push_str(impl_code);
        script.push_str(";\n");
        script.push_str("    },\n");
        script.push_str("    function() {\n");
        let each = each_hook.unwrap();
        script.push_str("        ");
        script.push_str(&strip_typescript_syntax(each));
        script.push_str(";\n");
        script.push_str("    },\n");
        script.push_str(&format!("    {}, {}\n", spec.iterations, spec.warmup));
        script.push_str(");\n");
    } else {
        // Standard benchmark execution
        script.push_str("const __result = __polybench.runBenchmark(function() {\n");
        script.push_str("    ");
        script.push_str(impl_code);
        script.push_str(";\n");
        script.push_str(&format!("}}, {}, {});\n", spec.iterations, spec.warmup));
    }

    // Phase 3: After hook (runs once after benchmark)
    if let Some(after) = after_hook {
        script.push_str("\n// After hook\n");
        let stripped = strip_typescript_syntax(after);
        if stripped.contains("await") {
            script.push_str(&format!("await (async () => {{ {} }})();\n", stripped));
        } else {
            script.push_str(&format!("{};\n", stripped));
        }
    }

    script.push_str("\nconsole.log(JSON.stringify(__result));\n");

    Ok(script)
}

/// Parse benchmark result JSON from stdout
fn parse_benchmark_result(stdout: &str) -> Result<Measurement> {
    // Find the JSON line (last non-empty line)
    let json_line = stdout.lines()
        .filter(|l| !l.trim().is_empty())
        .last()
        .ok_or_else(|| miette!("No output from benchmark"))?;

    let result: BenchResultJson = serde_json::from_str(json_line)
        .map_err(|e| miette!("Failed to parse benchmark result: {}\nOutput: {}", e, stdout))?;

    Ok(result.into_measurement())
}

/// JSON format for benchmark results
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct BenchResultJson {
    iterations: u64,
    total_nanos: f64,
    nanos_per_op: f64,
    ops_per_sec: f64,
    #[serde(default)]
    samples: Vec<f64>,
}

impl BenchResultJson {
    fn into_measurement(self) -> Measurement {
        let samples: Vec<u64> = self.samples.iter()
            .map(|&s| s as u64)
            .collect();

        if samples.is_empty() {
            Measurement::from_aggregate(self.iterations, self.total_nanos as u64)
        } else {
            Measurement::from_samples(samples, self.iterations)
        }
    }
}
