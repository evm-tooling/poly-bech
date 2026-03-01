//! V8-based JavaScript runtime
//!
//! For the MVP, we use subprocess execution with Node.js as the V8 embedding
//! requires significant setup. This provides a working implementation that
//! can be upgraded to embedded V8 later.

use crate::{builtins, codegen, error_mapping::TsErrorMapper, transpiler};
use async_trait::async_trait;
use miette::{miette, Result};
use poly_bench_dsl::{AsyncSamplingPolicy, BenchMode, BenchmarkKind, Lang};
use poly_bench_ir::{BenchmarkSpec, SuiteIR};
use poly_bench_stdlib as stdlib;
use poly_bench_traits::{ErrorMapper, Measurement, Runtime, RuntimeConfig, RuntimeFactory};
use std::{path::PathBuf, process::Stdio, time::Instant};
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
    /// Cached script path and source hash for reuse across runs
    cached_script: Option<(PathBuf, PathBuf, u64)>,
    /// Duration of last precompile in nanoseconds (for accurate reporting)
    last_precompile_nanos: Option<u64>,
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
            cached_script: None,
            last_precompile_nanos: None,
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

    /// Compute a hash of the source code for cache invalidation
    fn hash_source(source: &str) -> u64 {
        use std::{
            collections::hash_map::DefaultHasher,
            hash::{Hash, Hasher},
        };
        let mut hasher = DefaultHasher::new();
        source.hash(&mut hasher);
        hasher.finish()
    }

    /// Helper to write script to disk and update cache
    fn write_script_and_cache(
        &mut self,
        script: &str,
        source_hash: u64,
    ) -> Result<(PathBuf, PathBuf)> {
        let (script_path, working_dir) = if let Some(ref project_root) = self.project_root {
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
            let temp_dir =
                self.temp_dir.as_ref().ok_or_else(|| miette!("Runtime not initialized"))?;

            let script_path = temp_dir.path().join("bench.js");
            (script_path, temp_dir.path().to_path_buf())
        };

        std::fs::write(&script_path, script)
            .map_err(|e| miette!("Failed to write benchmark script: {}", e))?;

        // Update cache
        self.cached_script = Some((script_path.clone(), working_dir.clone(), source_hash));

        Ok((script_path, working_dir))
    }
}

impl Default for JsRuntime {
    fn default() -> Self {
        Self::new().expect("Failed to create JS runtime")
    }
}

/// Factory for creating TypeScript runtime instances
pub struct JsRuntimeFactory;

pub static JS_RUNTIME_FACTORY: JsRuntimeFactory = JsRuntimeFactory;

impl RuntimeFactory for JsRuntimeFactory {
    fn lang(&self) -> Lang {
        Lang::TypeScript
    }
    fn name(&self) -> &'static str {
        "JavaScript/TypeScript Runtime"
    }
    fn create(&self, config: &RuntimeConfig) -> Result<Box<dyn Runtime>> {
        let mut rt = JsRuntime::new()?;
        rt.set_project_root(config.get_root(poly_bench_dsl::Lang::TypeScript));
        Ok(Box::new(rt))
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

    fn last_precompile_nanos(&self) -> Option<u64> {
        self.last_precompile_nanos
    }

    async fn initialize(&mut self, suite: &SuiteIR) -> Result<()> {
        // Create temp directory
        let temp_dir =
            TempDir::new().map_err(|e| miette!("Failed to create temp directory: {}", e))?;

        // Generate TypeScript code for the suite
        let ir = poly_bench_ir::BenchmarkIR::new(vec![suite.clone()]);
        let code = codegen::generate(&ir)?;

        self.generated_code = Some(code);
        self.temp_dir = Some(temp_dir);

        Ok(())
    }

    fn generate_check_source(&self, spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<String> {
        generate_typescript_check_source(spec, suite)
    }

    async fn compile_check(&self, spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<()> {
        let script = self.generate_check_source(spec, suite)?;

        // Use a unique filename per benchmark to avoid race conditions in parallel validation
        let safe_name = spec.name.replace('.', "_").replace('/', "_");
        let filename = format!("bench_check_{}.ts", safe_name);

        // Determine where to write the file
        let (script_path, working_dir) = if let Some(ref project_root) = self.project_root {
            let is_runtime_env = project_root.as_os_str().to_string_lossy().contains("runtime-env");
            let script_path = if is_runtime_env {
                project_root.join(&filename)
            } else {
                let bench_dir = project_root.join(".polybench");
                std::fs::create_dir_all(&bench_dir)
                    .map_err(|e| miette!("Failed to create .polybench directory: {}", e))?;
                bench_dir.join(&filename)
            };
            (script_path, project_root.clone())
        } else {
            let temp_dir =
                self.temp_dir.as_ref().ok_or_else(|| miette!("Runtime not initialized"))?;
            (temp_dir.path().join(&filename), temp_dir.path().to_path_buf())
        };

        std::fs::write(&script_path, &script)
            .map_err(|e| miette!("Failed to write TypeScript check file: {}", e))?;

        // Build line mappings for error remapping
        let mapper = TsErrorMapper;
        let mappings = mapper.build_mappings(suite, &script);

        // Try esbuild first (much faster for syntax validation)
        // esbuild doesn't do full type checking but catches most errors quickly
        if let Ok(esbuild_binary) = which::which("esbuild") {
            let output = tokio::process::Command::new(&esbuild_binary)
                .args([
                    script_path.to_str().unwrap(),
                    "--bundle",
                    "--platform=node",
                    "--format=esm",
                    "--outfile=/dev/null",
                    "--log-level=error",
                ])
                .current_dir(&working_dir)
                .output()
                .await
                .map_err(|e| miette!("Failed to run esbuild: {}", e))?;

            if output.status.success() {
                // esbuild succeeded - fast path complete
                return Ok(());
            }

            // esbuild failed - fall through to tsc for detailed type errors
            // (esbuild errors are often less informative than tsc)
        }

        // Try tsc for full type checking (slower but more thorough)
        if let Ok(tsc_binary) = which::which("tsc") {
            let args = vec![
                "--noEmit",
                "--skipLibCheck",
                "--target",
                "ES2022",
                "--module",
                "ESNext",
                "--moduleResolution",
                "bundler",
                "--esModuleInterop",
                "--allowSyntheticDefaultImports",
                script_path.to_str().unwrap(),
            ];

            let output = tokio::process::Command::new(&tsc_binary)
                .args(&args)
                .current_dir(&working_dir)
                .output()
                .await
                .map_err(|e| miette!("Failed to run TypeScript compiler: {}", e))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let stdout = String::from_utf8_lossy(&output.stdout);
                let error_output = if stderr.is_empty() { stdout } else { stderr };
                let remapped = mapper.remap_error(&error_output, &mappings);
                return Err(miette!("TypeScript compilation failed:\n{}", remapped));
            }
        } else {
            // Fallback: use Node.js --check for basic syntax validation
            let js_script = strip_typescript_syntax(&script);
            let js_path = script_path.with_extension("mjs");
            std::fs::write(&js_path, &js_script)
                .map_err(|e| miette!("Failed to write JS check file: {}", e))?;

            let output = tokio::process::Command::new(&self.node_binary)
                .args(["--check", js_path.to_str().unwrap()])
                .current_dir(&working_dir)
                .output()
                .await
                .map_err(|e| miette!("Failed to check JavaScript syntax: {}", e))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let remapped = mapper.remap_error(&stderr, &mappings);
                return Err(miette!("JavaScript syntax check failed:\n{}", remapped));
            }
        }

        Ok(())
    }

    async fn precompile(&mut self, spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<()> {
        let script = generate_standalone_script(spec, suite)?;
        let source_hash = Self::hash_source(&script);

        // Check if we already have a cached script with matching source hash
        if let Some((ref script_path, _, cached_hash)) = self.cached_script {
            if cached_hash == source_hash && script_path.exists() {
                // Already prepared, nothing to do
                self.last_precompile_nanos = Some(0);
                return Ok(());
            }
        }

        let pc_start = Instant::now();

        // Determine where to write the script
        let (script_path, working_dir) = if let Some(ref project_root) = self.project_root {
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
            let temp_dir =
                self.temp_dir.as_ref().ok_or_else(|| miette!("Runtime not initialized"))?;

            let script_path = temp_dir.path().join("bench.js");
            (script_path, temp_dir.path().to_path_buf())
        };

        std::fs::write(&script_path, &script)
            .map_err(|e| miette!("Failed to write benchmark script: {}", e))?;

        // Optionally do a syntax check (--check flag just parses without executing)
        let check_output = tokio::process::Command::new(&self.node_binary)
            .args(["--check", script_path.to_str().unwrap()])
            .current_dir(&working_dir)
            .output()
            .await
            .map_err(|e| miette!("Failed to check TypeScript benchmark: {}", e))?;

        if !check_output.status.success() {
            let stderr = String::from_utf8_lossy(&check_output.stderr);
            return Err(miette!("TypeScript benchmark syntax check failed:\n{}", stderr));
        }

        // Cache the script path and source hash for reuse
        self.cached_script = Some((script_path, working_dir, source_hash));
        self.last_precompile_nanos = Some(pc_start.elapsed().as_nanos() as u64);

        Ok(())
    }

    async fn run_benchmark(
        &mut self,
        spec: &BenchmarkSpec,
        suite: &SuiteIR,
    ) -> Result<Measurement> {
        // Generate standalone script for this benchmark
        let script = generate_standalone_script(spec, suite)?;
        let source_hash = Self::hash_source(&script);

        // Check if we have a cached script with matching source hash
        let (script_path, working_dir) =
            if let Some((ref cached_path, ref cached_dir, cached_hash)) = self.cached_script {
                if cached_hash == source_hash && cached_path.exists() {
                    // Reuse cached script - skip writing
                    (cached_path.clone(), cached_dir.clone())
                } else {
                    // Cache miss - need to write script
                    self.write_script_and_cache(&script, source_hash)?
                }
            } else {
                // No cache - need to write script
                self.write_script_and_cache(&script, source_hash)?
            };

        // Run with Node.js from the working directory (which has node_modules)
        let mut cmd = tokio::process::Command::new(&self.node_binary);
        if spec.memory {
            cmd.arg("--expose-gc");
        }
        cmd.arg(&script_path).current_dir(&working_dir);

        // Pass Anvil RPC URL if available
        if let Some(ref url) = self.anvil_rpc_url {
            cmd.env("ANVIL_RPC_URL", url);
        }

        cmd.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());
        cmd.kill_on_drop(true);
        let run_start = std::time::Instant::now();
        let child = cmd.spawn().map_err(|e| miette!("Failed to run Node.js: {}", e))?;
        let output = if let Some(timeout_ms) = spec.timeout {
            match tokio::time::timeout(
                tokio::time::Duration::from_millis(timeout_ms),
                child.wait_with_output(),
            )
            .await
            {
                Ok(result) => result.map_err(|e| miette!("Failed to run Node.js: {}", e))?,
                Err(_) => {
                    return Err(miette!("JavaScript benchmark timed out after {}ms", timeout_ms));
                }
            }
        } else {
            child.wait_with_output().await.map_err(|e| miette!("Failed to run Node.js: {}", e))?
        };

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(miette!("JavaScript benchmark failed:\n{}", stderr));
        }

        let run_wall_nanos = run_start.elapsed().as_nanos() as u64;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut m = parse_benchmark_result(&stdout, spec.outlier_detection, spec.cv_threshold)?;
        // spawn = wall - warmup - exec (remainder after subtracting in-process phases)
        if let Some(warmup) = m.warmup_nanos {
            let exec = m.total_nanos;
            let spawn = run_wall_nanos.saturating_sub(warmup).saturating_sub(exec);
            m.spawn_nanos = Some(spawn);
        }
        Ok(m)
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
    let impl_code_raw = spec
        .get_impl(Lang::TypeScript)
        .ok_or_else(|| miette!("No TypeScript implementation for benchmark {}", spec.name))?;
    let impl_code = impl_code_raw.trim().trim_end_matches(';');

    let mut script = String::new();

    // User imports first (ES modules require imports at top of file)
    // Strip type-only imports and "type X" specifiers so Node.js can execute bench.mjs directly
    if let Some(user_imports) = suite.imports.get(&Lang::TypeScript) {
        for import_stmt in user_imports {
            if let Some(stripped) = transpiler::strip_type_imports(import_stmt) {
                script.push_str(&stripped);
                script.push('\n');
            }
        }
        if !user_imports.is_empty() {
            script.push('\n');
        }
    }

    // ESM: require is undefined; memory harness needs node:v8. Add createRequire so require works.
    if spec.memory {
        script.push_str("import { createRequire } from 'node:module';\nconst require = createRequire(import.meta.url);\n\n");
    }

    // Add harness (performance or memory path - memory uses total_allocated_bytes + gc)
    script.push_str(builtins::get_bench_harness(spec.memory));
    script.push_str("\n\n");

    // Add stdlib code (e.g., ANVIL_RPC_URL for std::anvil)
    let stdlib_code = stdlib::get_stdlib_code(&suite.stdlib_imports, &crate::TS_STDLIB);
    if !stdlib_code.is_empty() {
        script.push_str("// stdlib imports\n");
        // Strip TypeScript type annotations from stdlib code
        script.push_str(&strip_typescript_syntax(&stdlib_code));
        script.push_str("\n\n");
    }

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
                    script.push_str(&format!(
                        "const {} = (() => {{\n{}\n}})();\n",
                        fixture_name, stripped
                    ));
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
    let is_async_bench = spec.kind == BenchmarkKind::Async;
    let impl_is_async = is_async_bench || impl_code.contains("await");
    let use_auto_mode = spec.mode == BenchMode::Auto;
    let use_sink = if spec.use_sink { "true" } else { "false" };
    let track_memory = if spec.memory { "true" } else { "false" };
    let async_sampling_policy = match spec.async_sampling_policy {
        AsyncSamplingPolicy::FixedCap => "fixedCap",
        AsyncSamplingPolicy::TimeBudgeted => "timeBudgeted",
    };

    if each_hook.is_some() {
        // Custom benchmark loop with each hook
        let each = each_hook.unwrap();
        let each_is_async = each.contains("await");
        let needs_async = impl_is_async || each_is_async;

        // Note: async hooks not yet supported with auto mode
        if needs_async {
            script.push_str("const __result = await __polybench.runBenchmarkWithHookAsync(\n");
        } else {
            script.push_str("const __result = __polybench.runBenchmarkWithHook(\n");
        }

        if impl_is_async {
            script.push_str("    async function() {\n");
        } else {
            script.push_str("    function() {\n");
        }
        script.push_str("        return (");
        script.push_str(impl_code);
        script.push_str(");\n");
        script.push_str("    },\n");

        if each_is_async {
            script.push_str("    async function() {\n");
        } else {
            script.push_str("    function() {\n");
        }
        script.push_str("        ");
        script.push_str(&strip_typescript_syntax(each));
        script.push_str(";\n");
        script.push_str("    },\n");
        script.push_str(&format!(
            "    {}, {}, {}, {}, {}, {}, {}, \"{}\"\n",
            spec.iterations,
            spec.warmup_iterations,
            spec.warmup_time_ms,
            use_sink,
            track_memory,
            spec.async_sample_cap,
            spec.async_warmup_cap,
            async_sampling_policy
        ));
        script.push_str(");\n");
    } else if use_auto_mode {
        // Auto-calibration mode: warmupTimeMs takes precedence over warmupIterations
        if impl_is_async {
            script.push_str(
                "const __result = await __polybench.runBenchmarkAutoAsync(async function() {\n",
            );
            script.push_str("    return (");
            script.push_str(impl_code);
            script.push_str(");\n");
            script.push_str(&format!(
                "}}, {}, {}, {}, {}, {}, {}, {}, \"{}\");\n",
                spec.target_time_ms,
                use_sink,
                track_memory,
                spec.warmup_iterations,
                spec.warmup_time_ms,
                spec.async_sample_cap,
                spec.async_warmup_cap,
                async_sampling_policy
            ));
        } else {
            script.push_str("const __result = __polybench.runBenchmarkAuto(function() {\n");
            script.push_str("    return (");
            script.push_str(impl_code);
            script.push_str(");\n");
            script.push_str(&format!(
                "}}, {}, {}, {}, {}, {});\n",
                spec.target_time_ms,
                use_sink,
                track_memory,
                spec.warmup_iterations,
                spec.warmup_time_ms
            ));
        }
    } else {
        // Fixed iteration mode
        if impl_is_async {
            script.push_str(
                "const __result = await __polybench.runBenchmarkAsync(async function() {\n",
            );
        } else {
            script.push_str("const __result = __polybench.runBenchmark(function() {\n");
        }
        script.push_str("    return (");
        script.push_str(impl_code);
        script.push_str(");\n");
        if impl_is_async {
            script.push_str(&format!(
                "}}, {}, {}, {}, {}, {}, {}, {}, \"{}\");\n",
                spec.iterations,
                spec.warmup_iterations,
                spec.warmup_time_ms,
                use_sink,
                track_memory,
                spec.async_sample_cap,
                spec.async_warmup_cap,
                async_sampling_policy
            ));
        } else {
            script.push_str(&format!(
                "}}, {}, {}, {}, {}, {});\n",
                spec.iterations,
                spec.warmup_iterations,
                spec.warmup_time_ms,
                use_sink,
                track_memory
            ));
        }
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
fn parse_benchmark_result(
    stdout: &str,
    outlier_detection: bool,
    cv_threshold: f64,
) -> Result<Measurement> {
    // Find the JSON line (last non-empty line)
    let json_line = stdout
        .lines()
        .filter(|l| !l.trim().is_empty())
        .last()
        .ok_or_else(|| miette!("No output from benchmark"))?;

    let result: BenchResultJson = serde_json::from_str(json_line)
        .map_err(|e| miette!("Failed to parse benchmark result: {}\nOutput: {}", e, stdout))?;

    Ok(result.into_measurement_with_options(outlier_detection, cv_threshold))
}

pub fn extract_runtime_error_reason(raw: &str) -> String {
    let mut last_non_empty = "";
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with("at ") || trimmed.contains("node:internal/") {
            continue;
        }
        last_non_empty = trimmed;
        if trimmed.contains("Error:") || trimmed.contains("Exception:") {
            return trimmed.to_string();
        }
    }
    if last_non_empty.is_empty() {
        "Unknown runtime error".to_string()
    } else {
        last_non_empty.to_string()
    }
}

pub fn extract_generated_snippet(raw: &str, context: usize) -> Option<Vec<String>> {
    let lines: Vec<&str> = raw.lines().collect();
    if lines.is_empty() {
        return None;
    }
    // Keep original line ordering but search for the caret marker.
    let caret_idx = lines.iter().position(|l| l.trim() == "^")?;
    if caret_idx == 0 {
        return None;
    }

    // Try to locate a "file:line" header right before the source line and skip it.
    let mut code_idx = caret_idx.saturating_sub(1);
    if code_idx > 0 {
        let candidate = lines[code_idx - 1].trim();
        if candidate.contains(':') && candidate.contains(".mjs") {
            code_idx = code_idx.saturating_sub(0);
        }
    }

    let start = code_idx.saturating_sub(context);
    let end = std::cmp::min(lines.len().saturating_sub(1), code_idx + context);
    let mut out = Vec::new();
    for (i, line) in lines[start..=end].iter().enumerate() {
        let marker = if start + i == code_idx { ">" } else { " " };
        out.push(format!("{} {}", marker, line.trim_end()));
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

/// JSON format for benchmark results
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct BenchResultJson {
    iterations: u64,
    total_nanos: f64,
    /// Warmup phase duration in nanoseconds (if reported by harness)
    #[serde(default)]
    warmup_nanos: Option<f64>,
    nanos_per_op: f64,
    ops_per_sec: f64,
    #[serde(default)]
    samples: Vec<f64>,
    /// Bytes allocated per operation (from process.memoryUsage().heapUsed delta)
    #[serde(default)]
    bytes_per_op: Option<f64>,
    #[serde(default)]
    raw_result: Option<serde_json::Value>,
    #[serde(default)]
    successful_results: Vec<serde_json::Value>,
    #[serde(default)]
    successful_count: Option<u64>,
    #[serde(default)]
    error_count: Option<u64>,
    #[serde(default)]
    error_samples: Vec<String>,
}

impl BenchResultJson {
    fn into_measurement_with_options(
        self,
        outlier_detection: bool,
        cv_threshold: f64,
    ) -> Measurement {
        let samples: Vec<u64> = self.samples.iter().map(|&s| s as u64).collect();

        let mut m = if samples.is_empty() {
            Measurement::from_aggregate(self.iterations, self.total_nanos as u64)
        } else {
            Measurement::from_aggregate_with_sample_stats(
                self.iterations,
                self.total_nanos as u64,
                samples,
                outlier_detection,
                cv_threshold,
            )
        };
        m.raw_result = self.raw_result.as_ref().map(|v| v.to_string());
        m.warmup_nanos = self.warmup_nanos.map(|n| n as u64);
        if !self.successful_results.is_empty() {
            m.successful_results =
                Some(self.successful_results.iter().map(|v| v.to_string()).collect());
        }
        m.async_success_count = self.successful_count;
        m.async_error_count = self.error_count;
        if !self.error_samples.is_empty() {
            m.async_error_samples = Some(self.error_samples);
        }

        // Apply memory stats from JS (total_allocated_bytes or heapUsed fallback); allocs_per_op
        // not available in Node
        if let Some(bytes) = self.bytes_per_op {
            let bytes_u64 = bytes.max(0.0).round() as u64;
            m = m.with_allocs(bytes_u64, 0);
        }

        m
    }
}

#[cfg(test)]
mod tests {
    use super::BenchResultJson;

    #[test]
    fn test_parse_async_outcome_fields_from_js_result() {
        let json = r#"{
            "iterations": 10,
            "totalNanos": 1000,
            "nanosPerOp": 100,
            "opsPerSec": 10000000,
            "samples": [100, 101],
            "successfulResults": [1,2,3],
            "successfulCount": 3,
            "errorCount": 2,
            "errorSamples": ["boom","bad rpc"]
        }"#;
        let parsed: BenchResultJson = serde_json::from_str(json).unwrap();
        let measurement = parsed.into_measurement_with_options(false, 5.0);
        assert_eq!(measurement.async_success_count, Some(3));
        assert_eq!(measurement.async_error_count, Some(2));
        assert_eq!(
            measurement.async_error_samples,
            Some(vec!["boom".to_string(), "bad rpc".to_string()])
        );
    }
}

/// Generate TypeScript source code for compile checking
fn generate_typescript_check_source(spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<String> {
    let impl_code = spec
        .get_impl(Lang::TypeScript)
        .ok_or_else(|| miette!("No TypeScript implementation for benchmark {}", spec.name))?;

    let mut script = String::new();

    // User imports first - strip type-only imports so tsc/esbuild/node can parse
    if let Some(user_imports) = suite.imports.get(&Lang::TypeScript) {
        for import_stmt in user_imports {
            if let Some(stripped) = transpiler::strip_type_imports(import_stmt) {
                script.push_str(&stripped);
                script.push('\n');
            }
        }
    }

    // Add declarations - strip type annotations and "as Type"/"as const" for valid JS/TS
    if let Some(declarations) = suite.declarations.get(&Lang::TypeScript) {
        if !declarations.trim().is_empty() {
            script.push_str(&strip_typescript_syntax(declarations));
            script.push_str("\n\n");
        }
    }

    // Add helpers - strip type annotations for valid JS/TS
    if let Some(helpers) = suite.helpers.get(&Lang::TypeScript) {
        if !helpers.trim().is_empty() {
            script.push_str(&strip_typescript_syntax(helpers));
            script.push_str("\n\n");
        }
    }

    // Add fixtures
    for fixture_name in &spec.fixture_refs {
        if let Some(fixture) = suite.get_fixture(fixture_name) {
            if let Some(fixture_impl) = fixture.implementations.get(&Lang::TypeScript) {
                if fixture_impl.contains("return") {
                    script.push_str(&format!(
                        "const {} = (() => {{\n{}\n}})();\n",
                        fixture_name, fixture_impl
                    ));
                } else {
                    script.push_str(&format!("const {} = {};\n", fixture_name, fixture_impl));
                }
            } else if !fixture.data.is_empty() {
                script.push_str(&format!(
                    "const {} = new Uint8Array([{}]);\n",
                    fixture_name,
                    fixture.data.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(", ")
                ));
            }
        }
    }

    // Add the benchmark implementation wrapped in a function
    let is_async = spec.kind == BenchmarkKind::Async || impl_code.contains("await ");
    if is_async {
        script.push_str(&format!(
            "\nasync function __benchmark() {{\n    {};\n}}\n__benchmark();\n",
            impl_code
        ));
    } else {
        script.push_str(&format!(
            "\nfunction __benchmark() {{\n    {};\n}}\n__benchmark();\n",
            impl_code
        ));
    }

    Ok(script)
}
