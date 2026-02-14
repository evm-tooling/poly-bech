//! Go runtime executor

use crate::{go::compiler::GoCompiler, measurement::Measurement, traits::Runtime};
use async_trait::async_trait;
use libloading::{Library, Symbol};
use miette::{miette, Result};
use poly_bench_dsl::{BenchMode, Lang};
use poly_bench_ir::{BenchmarkSpec, SuiteIR};
use poly_bench_stdlib as stdlib;
use std::{collections::HashSet, path::PathBuf};

use super::shared::{
    self, generate_bench_call, generate_fixtures_for_spec, generate_suite_code, SinkMemoryDecls,
    BENCH_RESULT_STRUCT,
};

/// Go runtime using plugin system
pub struct GoRuntime {
    /// Compiled plugin library
    library: Option<Library>,
    /// Path to the plugin file
    plugin_path: Option<PathBuf>,
    /// Compiler instance
    compiler: Option<GoCompiler>,
    /// Go module root directory (where go.mod exists)
    module_root: Option<PathBuf>,
    /// Anvil RPC URL if std::anvil is enabled
    anvil_rpc_url: Option<String>,
}

impl GoRuntime {
    pub fn new() -> Self {
        Self {
            library: None,
            plugin_path: None,
            compiler: None,
            module_root: None,
            anvil_rpc_url: None,
        }
    }

    /// Set the Go module root directory where go.mod is located
    pub fn set_module_root(&mut self, path: Option<PathBuf>) {
        self.module_root = path;
    }

    /// Set the Anvil RPC URL to pass to subprocess
    pub fn set_anvil_rpc_url(&mut self, url: String) {
        self.anvil_rpc_url = Some(url);
    }
}

impl Default for GoRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Runtime for GoRuntime {
    fn name(&self) -> &'static str {
        "Go Plugin Runtime"
    }

    fn lang(&self) -> Lang {
        Lang::Go
    }

    async fn initialize(&mut self, _suite: &SuiteIR) -> Result<()> {
        // Go plugins only work on Linux, so we'll use subprocess execution on all platforms
        let compiler = GoCompiler::new()?;
        self.compiler = Some(compiler);
        Ok(())
    }

    async fn run_benchmark(
        &mut self,
        spec: &BenchmarkSpec,
        suite: &SuiteIR,
    ) -> Result<Measurement> {
        // If we have a loaded library, try to use it
        if let Some(ref lib) = self.library {
            match self.run_via_plugin(lib, spec) {
                Ok(m) => return Ok(m),
                Err(e) => {
                    eprintln!("Plugin execution failed: {}. Using subprocess.", e);
                }
            }
        }

        // Fall back to subprocess execution
        self.run_via_subprocess(spec, suite).await
    }

    async fn shutdown(&mut self) -> Result<()> {
        self.library = None;
        self.compiler = None;
        Ok(())
    }
}

impl GoRuntime {
    /// Run benchmark via loaded plugin
    fn run_via_plugin(&self, lib: &Library, spec: &BenchmarkSpec) -> Result<Measurement> {
        unsafe {
            let run_benchmark: Symbol<fn(&str, i32) -> String> = lib
                .get(b"RunBenchmark")
                .map_err(|e| miette!("Failed to get RunBenchmark symbol: {}", e))?;

            let result_json = run_benchmark(&spec.full_name, spec.iterations as i32);

            let result: BenchResultJson = serde_json::from_str(&result_json)
                .map_err(|e| miette!("Failed to parse benchmark result: {}", e))?;

            Ok(result.into_measurement_with_options(spec.outlier_detection, spec.cv_threshold))
        }
    }

    /// Run benchmark via subprocess (fallback for unsupported platforms)
    async fn run_via_subprocess(
        &self,
        spec: &BenchmarkSpec,
        suite: &SuiteIR,
    ) -> Result<Measurement> {
        let source = generate_standalone_benchmark(spec, suite)?;

        let (src_path, working_dir) = if let Some(ref module_root) = self.module_root {
            let is_runtime_env = module_root.as_os_str().to_string_lossy().contains("runtime-env");
            let src_path = if is_runtime_env {
                module_root.join("bench_standalone.go")
            } else {
                let bench_dir = module_root.join(".polybench");
                std::fs::create_dir_all(&bench_dir)
                    .map_err(|e| miette!("Failed to create .polybench directory: {}", e))?;
                bench_dir.join("bench_standalone.go")
            };
            (src_path, module_root.clone())
        } else {
            let compiler =
                self.compiler.as_ref().ok_or_else(|| miette!("Compiler not initialized"))?;

            let src_path = compiler.temp_path().join("bench_standalone.go");
            (src_path, compiler.temp_path().to_path_buf())
        };

        std::fs::write(&src_path, &source)
            .map_err(|e| miette!("Failed to write benchmark source: {}", e))?;

        let go_binary = which::which("go").map_err(|_| miette!("Go not found in PATH"))?;

        let mut cmd = tokio::process::Command::new(&go_binary);
        cmd.args(["run", src_path.to_str().unwrap()]).current_dir(&working_dir);

        if let Some(ref url) = self.anvil_rpc_url {
            cmd.env("ANVIL_RPC_URL", url);
        }

        let output =
            cmd.output().await.map_err(|e| miette!("Failed to run Go benchmark: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(miette!("Go benchmark failed:\n{}", stderr));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let result: BenchResultJson = serde_json::from_str(&stdout)
            .map_err(|e| miette!("Failed to parse benchmark result: {}\nOutput: {}", e, stdout))?;

        Ok(result.into_measurement_with_options(spec.outlier_detection, spec.cv_threshold))
    }
}

/// JSON format for benchmark results from Go
#[derive(Debug, serde::Deserialize)]
struct BenchResultJson {
    iterations: u64,
    total_nanos: u64,
    nanos_per_op: f64,
    ops_per_sec: f64,
    #[serde(default)]
    bytes_per_op: u64,
    #[serde(default)]
    allocs_per_op: u64,
    #[serde(default)]
    samples: Vec<u64>,
}

impl BenchResultJson {
    fn into_measurement_with_options(
        self,
        outlier_detection: bool,
        cv_threshold: f64,
    ) -> Measurement {
        let mut m = if self.samples.is_empty() {
            Measurement::from_aggregate(self.iterations, self.total_nanos)
        } else {
            Measurement::from_samples_with_options(
                self.samples,
                self.iterations,
                outlier_detection,
                cv_threshold,
            )
        };

        if self.bytes_per_op > 0 || self.allocs_per_op > 0 {
            m = m.with_allocs(self.bytes_per_op, self.allocs_per_op);
        }

        m
    }
}

/// Generate a standalone Go program for subprocess execution
fn generate_standalone_benchmark(spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<String> {
    let impl_code = spec
        .get_impl(Lang::Go)
        .ok_or_else(|| miette!("No Go implementation for benchmark {}", spec.name))?;

    let mut code = String::new();

    // Package declaration
    code.push_str("package main\n\n");

    // Collect imports (similar to shared but with fmt for standalone)
    let stdlib_imports = stdlib::get_stdlib_imports(&suite.stdlib_imports, Lang::Go);
    let mut all_imports: HashSet<&str> = HashSet::new();
    all_imports.insert("\"encoding/json\"");
    all_imports.insert("\"fmt\"");
    all_imports.insert("\"time\"");

    if spec.use_sink || spec.memory {
        all_imports.insert("\"runtime\"");
    }
    if spec.concurrency > 1 {
        all_imports.insert("\"sync\"");
    }
    if let Some(user_imports) = suite.imports.get(&Lang::Go) {
        for import_spec in user_imports {
            all_imports.insert(import_spec);
        }
    }
    for import_spec in &stdlib_imports {
        all_imports.insert(import_spec);
    }

    code.push_str("import (\n");
    let mut sorted_imports: Vec<_> = all_imports.into_iter().collect();
    sorted_imports.sort();
    for import_spec in sorted_imports {
        code.push_str(&format!("\t{}\n", import_spec));
    }
    code.push_str(")\n\n");

    // BenchResult type
    code.push_str(BENCH_RESULT_STRUCT);
    code.push('\n');

    // Inject stdlib code
    let stdlib_code = stdlib::get_stdlib_code(&suite.stdlib_imports, Lang::Go);
    if !stdlib_code.is_empty() {
        code.push_str(&stdlib_code);
        code.push_str("\n");
    }

    // Suite-level code (declarations, init, helpers)
    code.push_str(&generate_suite_code(suite, Lang::Go));

    // Fixtures
    code.push_str(&generate_fixtures_for_spec(spec, suite, Lang::Go));

    // Get shared declarations
    let decls = SinkMemoryDecls::from_spec(spec);
    let bench_call = generate_bench_call(impl_code, spec.use_sink);
    let before_hook = spec.before_hooks.get(&Lang::Go);
    let after_hook = spec.after_hooks.get(&Lang::Go);
    let each_hook = spec.each_hooks.get(&Lang::Go);

    // Generate main function based on mode
    if spec.concurrency > 1 {
        return generate_concurrent_main(
            &mut code,
            spec,
            &decls,
            &bench_call,
            before_hook,
            after_hook,
        );
    }

    match spec.mode {
        BenchMode::Auto => generate_auto_main(
            &mut code,
            spec,
            &decls,
            &bench_call,
            before_hook,
            after_hook,
            each_hook,
        ),
        BenchMode::Fixed => generate_fixed_main(
            &mut code,
            spec,
            &decls,
            &bench_call,
            before_hook,
            after_hook,
            each_hook,
        ),
    }

    // Memory profiling after measurement
    code.push_str(decls.memory_after);

    // After hook
    if let Some(after) = after_hook {
        code.push_str("\n\t// After hook\n");
        for line in after.lines() {
            code.push_str(&format!("\t{}\n", line));
        }
    }

    // Result calculation and output
    let memory_result = SinkMemoryDecls::memory_result_fields(spec.memory, "iterations");
    code.push_str(&shared::generate_result_return("iterations", &memory_result, true));
    code.push_str("}\n");

    Ok(code)
}

/// Generate auto-calibration main function
fn generate_auto_main(
    code: &mut String,
    spec: &BenchmarkSpec,
    decls: &SinkMemoryDecls,
    bench_call: &str,
    before_hook: Option<&String>,
    _after_hook: Option<&String>,
    each_hook: Option<&String>,
) {
    // Note: targetNanos is declared inside generate_auto_mode_loop, not here
    code.push_str(&format!(
        r#"
func main() {{
{}{}
"#,
        decls.sink_decl, decls.memory_decl
    ));

    // Before hook
    if let Some(before) = before_hook {
        code.push_str("\n\t// Before hook\n");
        for line in before.lines() {
            code.push_str(&format!("\t{}\n", line));
        }
    }

    code.push_str(decls.memory_before);

    // Warmup
    code.push_str(&format!(
        "\n{}",
        shared::generate_warmup_loop(bench_call, decls.sink_keepalive, each_hook, "100")
    ));

    // Auto-calibration loop
    code.push_str(&format!(
        "\n{}",
        shared::generate_auto_mode_loop(
            bench_call,
            decls.sink_keepalive,
            each_hook,
            spec.target_time_ms
        )
    ));

    // Sample collection
    code.push_str(&format!(
        "\n{}",
        shared::generate_sample_collection(
            bench_call,
            decls.sink_keepalive,
            each_hook,
            "1000",
            "totalIterations"
        )
    ));

    code.push_str("\n\titerations := totalIterations\n");
}

/// Generate fixed iteration main function
fn generate_fixed_main(
    code: &mut String,
    spec: &BenchmarkSpec,
    decls: &SinkMemoryDecls,
    bench_call: &str,
    before_hook: Option<&String>,
    _after_hook: Option<&String>,
    each_hook: Option<&String>,
) {
    code.push_str(&format!(
        r#"
func main() {{
	iterations := {}
	warmup := {}
	samples := make([]uint64, iterations)
{}{}
"#,
        spec.iterations, spec.warmup, decls.sink_decl, decls.memory_decl
    ));

    // Before hook
    if let Some(before) = before_hook {
        code.push_str("\n\t// Before hook\n");
        for line in before.lines() {
            code.push_str(&format!("\t{}\n", line));
        }
    }

    code.push_str(decls.memory_before);

    // Warmup
    code.push_str(&format!(
        "\n{}",
        shared::generate_warmup_loop(bench_call, decls.sink_keepalive, each_hook, "warmup")
    ));

    // Fixed measurement loop
    code.push_str(&format!(
        "\n{}",
        shared::generate_fixed_mode_loop(bench_call, decls.sink_keepalive, each_hook, "iterations")
    ));
}

/// Generate concurrent execution main function
fn generate_concurrent_main(
    code: &mut String,
    spec: &BenchmarkSpec,
    decls: &SinkMemoryDecls,
    bench_call: &str,
    before_hook: Option<&String>,
    after_hook: Option<&String>,
) -> Result<String> {
    let concurrency = spec.concurrency;
    let memory_result = SinkMemoryDecls::memory_result_fields(spec.memory, "totalIterations");
    let sink_keepalive = if spec.use_sink { "\n\truntime.KeepAlive(__sink)\n" } else { "" };

    let warmup_call = if spec.use_sink {
        format!("_ = {}", spec.get_impl(Lang::Go).unwrap_or(&"nil".to_string()))
    } else {
        bench_call.to_string()
    };

    code.push_str(&format!(
        r#"
func main() {{
{}{}
"#,
        decls.sink_decl, decls.memory_decl
    ));

    // Before hook
    if let Some(before) = before_hook {
        code.push_str("\n\t// Before hook\n");
        for line in before.lines() {
            code.push_str(&format!("\t{}\n", line));
        }
    }

    code.push_str(decls.memory_before);

    // Concurrent execution
    code.push_str(&format!(
        "\n{}",
        shared::generate_concurrent_execution(
            bench_call,
            &warmup_call,
            concurrency,
            &spec.iterations.to_string()
        )
    ));
    code.push_str(sink_keepalive);
    code.push_str(decls.memory_after);

    // After hook
    if let Some(after) = after_hook {
        code.push_str("\n\t// After hook\n");
        for line in after.lines() {
            code.push_str(&format!("\t{}\n", line));
        }
    }

    // Sample collection
    code.push_str(&format!(
        "\n{}",
        shared::generate_sample_collection(bench_call, "", None, "100", "totalIterations")
    ));

    // Result output
    code.push_str(&format!(
        r#"
	nanosPerOp := float64(totalNanos) / float64(totalIterations)
	opsPerSec := 1e9 / nanosPerOp
	
	result := BenchResult{{
		Iterations:  uint64(totalIterations),
		TotalNanos:  uint64(totalNanos),
		NanosPerOp:  nanosPerOp,
		OpsPerSec:   opsPerSec,
{}		Samples:     samples,
	}}
	
	jsonBytes, _ := json.Marshal(result)
	fmt.Println(string(jsonBytes))
}}
"#,
        memory_result
    ));

    Ok(code.clone())
}
