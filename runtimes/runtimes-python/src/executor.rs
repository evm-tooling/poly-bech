//! Python runtime executor
//!
//! Executes Python benchmarks via subprocess using the system Python interpreter.
//! Uses time.perf_counter_ns() for high-resolution timing.

use async_trait::async_trait;
use miette::{miette, Result};
use poly_bench_dsl::{BenchMode, Lang};
use poly_bench_ir::{BenchmarkSpec, SuiteIR};
use poly_bench_runtime_traits::{Measurement, Runtime, RuntimeConfig, RuntimeFactory};
use poly_bench_stdlib as stdlib;
use std::{path::PathBuf, process::Stdio};
use tempfile::TempDir;

/// Python runtime using subprocess execution
pub struct PythonRuntime {
    temp_dir: Option<TempDir>,
    python_binary: PathBuf,
    project_root: Option<PathBuf>,
    anvil_rpc_url: Option<String>,
    cached_script: Option<(PathBuf, PathBuf, u64)>,
    /// Duration of last precompile in nanoseconds (for accurate reporting)
    last_precompile_nanos: Option<u64>,
}

impl PythonRuntime {
    pub fn new() -> Result<Self> {
        let python_binary = which::which("python3")
            .or_else(|_| which::which("python"))
            .map_err(|_| miette!("Python not found in PATH. Please install Python 3."))?;

        Ok(Self {
            temp_dir: None,
            python_binary,
            project_root: None,
            anvil_rpc_url: None,
            cached_script: None,
            last_precompile_nanos: None,
        })
    }

    pub fn set_project_root(&mut self, path: Option<PathBuf>) {
        self.project_root = path;

        // When using poly-bench runtime-env, prefer the venv's Python so we use the
        // same interpreter that has the installed dependencies (from pip install).
        if let Some(ref project_root) = self.project_root {
            let is_runtime_env = project_root.as_os_str().to_string_lossy().contains("runtime-env");
            if is_runtime_env {
                let venv_python = project_root.join(".venv").join("bin").join("python");
                if venv_python.exists() {
                    self.python_binary = venv_python;
                }
            }
        }
    }

    pub fn set_anvil_rpc_url(&mut self, url: String) {
        self.anvil_rpc_url = Some(url);
    }

    fn hash_source(source: &str) -> u64 {
        use std::{
            collections::hash_map::DefaultHasher,
            hash::{Hash, Hasher},
        };
        let mut hasher = DefaultHasher::new();
        source.hash(&mut hasher);
        hasher.finish()
    }

    fn write_script_and_cache(
        &mut self,
        script: &str,
        source_hash: u64,
    ) -> Result<(PathBuf, PathBuf)> {
        let (script_path, working_dir) = if let Some(ref project_root) = self.project_root {
            let is_runtime_env = project_root.as_os_str().to_string_lossy().contains("runtime-env");
            let script_path = if is_runtime_env {
                project_root.join("bench.py")
            } else {
                let bench_dir = project_root.join(".polybench");
                std::fs::create_dir_all(&bench_dir)
                    .map_err(|e| miette!("Failed to create .polybench directory: {}", e))?;
                bench_dir.join("bench.py")
            };
            (script_path, project_root.clone())
        } else {
            let temp_dir =
                self.temp_dir.as_ref().ok_or_else(|| miette!("Runtime not initialized"))?;
            let script_path = temp_dir.path().join("bench.py");
            (script_path, temp_dir.path().to_path_buf())
        };

        std::fs::write(&script_path, script)
            .map_err(|e| miette!("Failed to write benchmark script: {}", e))?;

        self.cached_script = Some((script_path.clone(), working_dir.clone(), source_hash));

        Ok((script_path, working_dir))
    }
}

impl Default for PythonRuntime {
    fn default() -> Self {
        Self::new().expect("Failed to create Python runtime")
    }
}

/// Factory for creating Python runtime instances
pub struct PythonRuntimeFactory;

pub static PYTHON_RUNTIME_FACTORY: PythonRuntimeFactory = PythonRuntimeFactory;

impl RuntimeFactory for PythonRuntimeFactory {
    fn lang(&self) -> Lang {
        Lang::Python
    }
    fn name(&self) -> &'static str {
        "Python Runtime"
    }
    fn create(&self, config: &RuntimeConfig) -> Result<Box<dyn Runtime>> {
        let mut rt = PythonRuntime::new()?;
        rt.set_project_root(config.get_root(poly_bench_dsl::Lang::Python));
        Ok(Box::new(rt))
    }
}

#[async_trait]
impl Runtime for PythonRuntime {
    fn name(&self) -> &'static str {
        "Python Runtime"
    }

    fn lang(&self) -> Lang {
        Lang::Python
    }

    fn last_precompile_nanos(&self) -> Option<u64> {
        self.last_precompile_nanos
    }

    fn set_anvil_rpc_url(&mut self, url: String) {
        self.anvil_rpc_url = Some(url);
    }

    async fn initialize(&mut self, suite: &SuiteIR) -> Result<()> {
        let temp_dir =
            TempDir::new().map_err(|e| miette!("Failed to create temp directory: {}", e))?;
        self.temp_dir = Some(temp_dir);
        Ok(())
    }

    fn generate_check_source(&self, spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<String> {
        generate_python_check_source(spec, suite)
    }

    async fn compile_check(&self, spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<()> {
        let script = self.generate_check_source(spec, suite)?;

        let (script_path, working_dir) = if let Some(ref project_root) = self.project_root {
            // project_root is the runtime-env path (e.g. project/.polybench/runtime-env/python)
            // Derive the actual project root and use project/.polybench/python-check (like Rust
            // uses .polybench/rust)
            let is_runtime_env = project_root.as_os_str().to_string_lossy().contains("runtime-env");
            let (bench_dir, work_dir) = if is_runtime_env {
                // runtime-env/python -> go up to project root
                let project_root_dir = project_root
                    .parent()
                    .and_then(|p| p.parent())
                    .and_then(|p| p.parent())
                    .unwrap_or(project_root.as_path());
                let python_check = project_root_dir.join(".polybench").join("python-check");
                (python_check.clone(), python_check)
            } else {
                let bench_dir = project_root.join(".polybench").join("python-check");
                (bench_dir.clone(), bench_dir)
            };
            std::fs::create_dir_all(&bench_dir)
                .map_err(|e| miette!("Failed to create python-check directory: {}", e))?;
            let script_path = bench_dir.join("check.py");
            (script_path, work_dir)
        } else {
            // No project root (e.g. validation before roots resolved) - use temp dir
            // Do NOT use current_dir() which can create .polybench in the wrong place (e.g. repo
            // root)
            let work_dir = std::env::temp_dir().join("polybench-python-check");
            std::fs::create_dir_all(&work_dir)
                .map_err(|e| miette!("Failed to create check directory: {}", e))?;
            let script_path = work_dir.join("check.py");
            (script_path, work_dir)
        };

        std::fs::write(&script_path, &script).map_err(|e| {
            miette!("Failed to write check script to {}: {}", script_path.display(), e)
        })?;

        let output = tokio::process::Command::new(&self.python_binary)
            .args(["-m", "py_compile", script_path.to_str().unwrap()])
            .current_dir(&working_dir)
            .output()
            .await
            .map_err(|e| miette!("Failed to run Python check: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(miette!("Python syntax check failed:\n{}", stderr));
        }

        Ok(())
    }

    async fn precompile(&mut self, spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<()> {
        let script = generate_standalone_script(spec, suite)?;
        let source_hash = Self::hash_source(&script);

        if let Some((ref cached_path, _, cached_hash)) = self.cached_script {
            if cached_hash == source_hash && cached_path.exists() {
                self.last_precompile_nanos = Some(0);
                return Ok(());
            }
        }

        let pc_start = std::time::Instant::now();
        let (script_path, working_dir) = self.write_script_and_cache(&script, source_hash)?;

        // Compile to bytecode (.pyc) - this is what Python does on first import
        // Using py_compile ensures syntax is valid and generates cached bytecode
        let output = tokio::process::Command::new(&self.python_binary)
            .args(["-m", "py_compile", script_path.to_str().unwrap()])
            .current_dir(&working_dir)
            .output()
            .await
            .map_err(|e| miette!("Failed to run Python bytecode compilation: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(miette!("Python bytecode compilation failed:\n{}", stderr));
        }

        self.last_precompile_nanos = Some(pc_start.elapsed().as_nanos() as u64);
        Ok(())
    }

    async fn run_benchmark(
        &mut self,
        spec: &BenchmarkSpec,
        suite: &SuiteIR,
    ) -> Result<Measurement> {
        let script = generate_standalone_script(spec, suite)?;
        let source_hash = Self::hash_source(&script);

        let (script_path, working_dir) =
            if let Some((ref cached_path, ref cached_dir, cached_hash)) = self.cached_script {
                if cached_hash == source_hash && cached_path.exists() {
                    (cached_path.clone(), cached_dir.clone())
                } else {
                    self.write_script_and_cache(&script, source_hash)?
                }
            } else {
                self.write_script_and_cache(&script, source_hash)?
            };

        let mut cmd = tokio::process::Command::new(&self.python_binary);
        cmd.arg(&script_path).current_dir(&working_dir);

        if let Some(ref url) = self.anvil_rpc_url {
            cmd.env("ANVIL_RPC_URL", url);
        }

        cmd.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());
        cmd.kill_on_drop(true);

        // Python runs a fresh script each time; imports (e.g. matplotlib) run before the benchmark
        // loop and add significant wall time. Use a timeout that allows import overhead (min 20s)
        // while still capping runaway runs.
        let effective_timeout_ms = spec.timeout.or_else(|| {
            if spec.mode == BenchMode::Auto && spec.target_time_ms > 0 {
                let base = (spec.target_time_ms * 2).saturating_add(2000);
                Some(base.max(20_000))
            } else {
                None
            }
        });

        let run_start = std::time::Instant::now();
        let child = cmd.spawn().map_err(|e| miette!("Failed to run Python: {}", e))?;
        let output = if let Some(timeout_ms) = effective_timeout_ms {
            match tokio::time::timeout(
                tokio::time::Duration::from_millis(timeout_ms),
                child.wait_with_output(),
            )
            .await
            {
                Ok(result) => result.map_err(|e| miette!("Failed to run Python: {}", e))?,
                Err(_) => {
                    return Err(miette!("Python benchmark timed out after {}ms", timeout_ms));
                }
            }
        } else {
            child.wait_with_output().await.map_err(|e| miette!("Failed to run Python: {}", e))?
        };

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(miette!("Python benchmark failed:\n{}", stderr));
        }

        let run_wall_nanos = run_start.elapsed().as_nanos() as u64;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut m = parse_benchmark_result(&stdout, spec.outlier_detection, spec.cv_threshold)?;
        let spawn = if let Some(w) = m.warmup_nanos {
            run_wall_nanos.saturating_sub(w).saturating_sub(m.total_nanos)
        } else {
            run_wall_nanos.saturating_sub(m.total_nanos)
        };
        m.spawn_nanos = Some(spawn);
        Ok(m)
    }

    async fn shutdown(&mut self) -> Result<()> {
        self.temp_dir = None;
        Ok(())
    }
}

/// Normalize Python code indentation by stripping common leading whitespace
fn normalize_python_indent(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();
    if lines.is_empty() {
        return String::new();
    }
    let min_indent = lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min()
        .unwrap_or(0);
    lines
        .iter()
        .map(|l| {
            if l.trim().is_empty() {
                l.to_string()
            } else {
                l.get(min_indent.min(l.len())..).unwrap_or(l).to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn generate_python_check_source(spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<String> {
    let impl_code = spec
        .get_impl(Lang::Python)
        .ok_or_else(|| miette!("No Python implementation for benchmark {}", spec.name))?;

    let mut script = String::new();
    script.push_str("# Code generated by poly-bench. DO NOT EDIT.\n\n");

    if let Some(user_imports) = suite.imports.get(&Lang::Python) {
        for import_stmt in user_imports {
            script.push_str(import_stmt);
            script.push('\n');
        }
        if !user_imports.is_empty() {
            script.push('\n');
        }
    }

    let stdlib_code = stdlib::get_stdlib_code(&suite.stdlib_imports, &crate::PYTHON_STDLIB);
    if !stdlib_code.is_empty() {
        script.push_str(&stdlib_code);
        script.push_str("\n\n");
    }

    if let Some(declarations) = suite.declarations.get(&Lang::Python) {
        if !declarations.trim().is_empty() {
            script.push_str(&normalize_python_indent(declarations));
            script.push_str("\n\n");
        }
    }

    if let Some(helpers) = suite.helpers.get(&Lang::Python) {
        if !helpers.trim().is_empty() {
            script.push_str(&normalize_python_indent(helpers));
            script.push_str("\n\n");
        }
    }

    script.push_str(&normalize_python_indent(impl_code));
    script.push('\n');

    Ok(script)
}

fn generate_standalone_script(spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<String> {
    let impl_code = spec
        .get_impl(Lang::Python)
        .ok_or_else(|| miette!("No Python implementation for benchmark {}", spec.name))?;
    let impl_code = normalize_python_indent(impl_code.trim());

    let use_sink = spec.use_sink;
    let use_memory = spec.memory;
    let before_hook = spec.before_hooks.get(&Lang::Python);
    let after_hook = spec.after_hooks.get(&Lang::Python);
    let each_hook = spec.each_hooks.get(&Lang::Python);

    let mut script = String::new();
    script.push_str("# Code generated by poly-bench. DO NOT EDIT.\n\n");
    script.push_str("import time\nimport json\n");
    if use_memory {
        script.push_str("\nimport gc\nimport tracemalloc\n");
    }
    script.push_str("\n");

    if use_sink {
        script.push_str("__polybench_sink = None\n\n");
    }

    if let Some(user_imports) = suite.imports.get(&Lang::Python) {
        for import_stmt in user_imports {
            script.push_str(import_stmt);
            script.push('\n');
        }
        if !user_imports.is_empty() {
            script.push('\n');
        }
    }

    let stdlib_code = stdlib::get_stdlib_code(&suite.stdlib_imports, &crate::PYTHON_STDLIB);
    if !stdlib_code.is_empty() {
        script.push_str(&stdlib_code);
        script.push_str("\n\n");
    }

    if let Some(declarations) = suite.declarations.get(&Lang::Python) {
        if !declarations.trim().is_empty() {
            script.push_str(&normalize_python_indent(declarations));
            script.push_str("\n\n");
        }
    }

    if let Some(init_code) = suite.init_code.get(&Lang::Python) {
        if !init_code.trim().is_empty() {
            script.push_str(&normalize_python_indent(init_code));
            script.push_str("\n\n");
        }
    }

    if let Some(helpers) = suite.helpers.get(&Lang::Python) {
        if !helpers.trim().is_empty() {
            script.push_str(&normalize_python_indent(helpers));
            script.push_str("\n\n");
        }
    }

    for fixture_name in &spec.fixture_refs {
        if let Some(fixture) = suite.get_fixture(fixture_name) {
            if let Some(fixture_impl) = fixture.implementations.get(&Lang::Python) {
                script.push_str(&format!("{} = {}\n", fixture_name, fixture_impl));
            } else if !fixture.data.is_empty() {
                let bytes: Vec<String> =
                    fixture.data.iter().map(|b| format!("0x{:02x}", b)).collect();
                script.push_str(&format!("{} = bytes([{}])\n", fixture_name, bytes.join(", ")));
            }
        }
    }
    script.push('\n');

    let use_auto_mode = spec.mode == BenchMode::Auto;
    let target_nanos = (spec.target_time_ms as f64) * 1e6;
    let iterations = spec.iterations;

    // Wrap impl in a function for consistent execution
    // When use_sink, add return so we can assign to __polybench_sink (prevents DCE)
    script.push_str("def __polybench_bench():\n");
    let impl_lines: Vec<&str> = impl_code.lines().collect();
    for (i, line) in impl_lines.iter().enumerate() {
        script.push_str("    ");
        let is_last = i == impl_lines.len().saturating_sub(1);
        if use_sink && is_last && !line.trim().is_empty() {
            script.push_str("return ");
        }
        script.push_str(line);
        script.push('\n');
    }
    script.push_str("\n");

    let each_hook_code = each_hook
        .map(|h| h.trim().lines().map(|l| format!("        {}\n", l)).collect::<String>())
        .unwrap_or_default();
    let each_hook_code_inner = each_hook
        .map(|h| h.trim().lines().map(|l| format!("            {}\n", l)).collect::<String>())
        .unwrap_or_default();

    script.push_str("def __polybench_run():\n");
    if use_memory {
        script.push_str("    tracemalloc.start()\n");
    }
    script.push_str("    samples = []\n");
    script.push_str("    warmup_nanos = 0\n\n");
    // Warmup (warmup_time_ms takes precedence over warmup_iterations)
    if spec.warmup_time_ms > 0 {
        script.push_str(&format!(
            "    warmup_start = time.perf_counter()\n    warmup_limit_s = {} / 1000.0\n    while (time.perf_counter() - warmup_start) < warmup_limit_s:\n",
            spec.warmup_time_ms
        ));
        script.push_str(&each_hook_code);
        if use_sink {
            script.push_str("        __polybench_sink = __polybench_bench()\n");
        } else {
            script.push_str("        __polybench_bench()\n");
        }
        script.push_str("    warmup_nanos = int((time.perf_counter() - warmup_start) * 1e9)\n\n");
    } else if spec.warmup_iterations > 0 {
        script.push_str("    warmup_start = time.perf_counter_ns()\n");
        script.push_str(&format!("    for _ in range({}):\n", spec.warmup_iterations));
        script.push_str(&each_hook_code);
        if use_sink {
            script.push_str("        __polybench_sink = __polybench_bench()\n");
        } else {
            script.push_str("        __polybench_bench()\n");
        }
        script.push_str("    warmup_nanos = time.perf_counter_ns() - warmup_start\n\n");
    }

    if let Some(before) = before_hook {
        script.push_str("    # Before hook\n");
        for line in before.trim().lines() {
            script.push_str("    ");
            script.push_str(line);
            script.push_str("\n");
        }
        script.push_str("\n");
    }

    if use_memory {
        script.push_str("    gc.collect()\n");
        script.push_str("    mem_before = tracemalloc.get_traced_memory()[0]\n");
    }

    if use_auto_mode {
        // Batched execution (like Go/TS): time whole batches, not each iteration.
        // Per-iteration timing adds ~300ns overhead per call; for 60ns ops that's 5x overhead
        // and causes 15s+ wall time to reach 3s target. Batching keeps wall time ~ target.
        script.push_str(&format!("    target_ns = {:.0}\n", target_nanos));
        script.push_str("    batch_size = 1\n");
        script.push_str("    total_iterations = 0\n");
        script.push_str("    total_ns = 0.0\n");
        script.push_str("    samples = []\n");
        script.push_str("    while total_ns < target_ns:\n");
        script.push_str("        batch_start = time.perf_counter_ns()\n");
        script.push_str("        for _ in range(batch_size):\n");
        script.push_str(&each_hook_code_inner);
        if use_sink {
            script.push_str("            __polybench_sink = __polybench_bench()\n");
        } else {
            script.push_str("            __polybench_bench()\n");
        }
        script.push_str("        batch_elapsed = time.perf_counter_ns() - batch_start\n");
        script.push_str("        total_iterations += batch_size\n");
        script.push_str("        total_ns += batch_elapsed\n");
        script.push_str("        samples.append(batch_elapsed / batch_size)\n");
        script.push_str("        if total_ns >= target_ns:\n");
        script.push_str("            break\n");
        script.push_str("        remaining = target_ns - total_ns\n");
        script.push_str("        if batch_elapsed > 0:\n");
        script.push_str("            predicted = int(batch_size * remaining / batch_elapsed)\n");
        script.push_str("            if remaining < batch_elapsed:\n");
        script.push_str("                new_size = max(1, predicted)\n");
        script.push_str("            elif remaining < target_ns / 5:\n");
        script.push_str("                new_size = max(1, int(predicted * 0.9))\n");
        script.push_str("            else:\n");
        script.push_str("                new_size = int(predicted * 1.1)\n");
        script.push_str("                if new_size <= batch_size:\n");
        script.push_str("                    new_size = batch_size * 2\n");
        script.push_str("                if new_size > batch_size * 10:\n");
        script.push_str("                    new_size = batch_size * 10\n");
        script.push_str("            batch_size = max(1, new_size)\n");
        script.push_str("        else:\n");
        script.push_str("            batch_size *= 10\n");
        script.push_str("    total_nanos = total_ns\n");
        script.push_str("    iterations = total_iterations\n");
    } else {
        script.push_str(&format!("    for _ in range({}):\n", iterations));
        script.push_str("        start = time.perf_counter_ns()\n");
        script.push_str(&each_hook_code);
        if use_sink {
            script.push_str("        __polybench_sink = __polybench_bench()\n");
        } else {
            script.push_str("        __polybench_bench()\n");
        }
        script.push_str("        samples.append(time.perf_counter_ns() - start)\n");
        script.push_str("    total_nanos = sum(samples)\n");
        script.push_str(&format!("    iterations = {}\n", iterations));
    }

    if use_memory {
        script.push_str("    gc.collect()\n");
        script.push_str("    mem_after = tracemalloc.get_traced_memory()[0]\n");
        script.push_str("    tracemalloc.stop()\n");
    }

    if let Some(after) = after_hook {
        script.push_str("\n    # After hook\n");
        for line in after.trim().lines() {
            script.push_str("    ");
            script.push_str(line);
            script.push_str("\n");
        }
        script.push_str("\n");
    }

    script.push_str("    nanos_per_op = total_nanos / iterations\n");
    script.push_str("    ops_per_sec = 1e9 / nanos_per_op\n");
    script.push_str("    result = {\n");
    script.push_str("        \"iterations\": iterations,\n");
    script.push_str("        \"totalNanos\": float(total_nanos),\n");
    script.push_str("        \"warmupNanos\": float(warmup_nanos),\n");
    script.push_str("        \"nanosPerOp\": nanos_per_op,\n");
    script.push_str("        \"opsPerSec\": ops_per_sec,\n");
    script.push_str("        \"samples\": samples\n");
    if use_memory {
        script.push_str(
            "        , \"bytesPerOp\": int(max(0, mem_after - mem_before) / iterations)\n",
        );
    }
    if use_sink {
        script.push_str("        , \"rawResult\": json.dumps(__polybench_sink, default=str)\n");
    }
    script.push_str("    }\n");
    script.push_str("    print(json.dumps(result))\n\n");
    script.push_str("__polybench_run()\n");

    Ok(script)
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct BenchResultJson {
    iterations: u64,
    total_nanos: f64,
    #[serde(default)]
    warmup_nanos: Option<f64>,
    nanos_per_op: f64,
    ops_per_sec: f64,
    #[serde(default)]
    samples: Vec<f64>,
    #[serde(default)]
    bytes_per_op: Option<u64>,
    #[serde(default)]
    raw_result: Option<String>,
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

        if let Some(bytes) = self.bytes_per_op {
            m = m.with_allocs(bytes, 0);
        }
        if let Some(raw) = self.raw_result {
            m.raw_result = Some(raw);
        }
        if let Some(w) = self.warmup_nanos {
            m.warmup_nanos = Some(w as u64);
        }

        m
    }
}

fn parse_benchmark_result(
    stdout: &str,
    outlier_detection: bool,
    cv_threshold: f64,
) -> Result<Measurement> {
    let json_line = stdout
        .lines()
        .filter(|l| !l.trim().is_empty())
        .last()
        .ok_or_else(|| miette!("No output from benchmark"))?;

    let result: BenchResultJson = serde_json::from_str(json_line)
        .map_err(|e| miette!("Failed to parse benchmark result: {}\nOutput: {}", e, stdout))?;

    Ok(result.into_measurement_with_options(outlier_detection, cv_threshold))
}
