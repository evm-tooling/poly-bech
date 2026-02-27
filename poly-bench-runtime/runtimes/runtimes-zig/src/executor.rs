//! Zig runtime executor.

use async_trait::async_trait;
use miette::{miette, Result};
use poly_bench_dsl::{BenchMode, Lang};
use poly_bench_ir::{BenchmarkSpec, SuiteIR};
use poly_bench_stdlib as stdlib;
use poly_bench_traits::{Measurement, Runtime, RuntimeConfig, RuntimeFactory};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    process::Stdio,
};

pub struct ZigRuntime {
    zig_binary: PathBuf,
    project_root: Option<PathBuf>,
    anvil_rpc_url: Option<String>,
    cached_binary: Option<(PathBuf, u64)>,
    last_precompile_nanos: Option<u64>,
}

impl ZigRuntime {
    pub fn new() -> Result<Self> {
        let zig_binary = which::which("zig").map_err(|_| miette!("zig not found in PATH"))?;
        Ok(Self {
            zig_binary,
            project_root: None,
            anvil_rpc_url: None,
            cached_binary: None,
            last_precompile_nanos: None,
        })
    }

    pub fn set_project_root(&mut self, path: Option<PathBuf>) {
        self.project_root = path;
    }

    fn normalize_indent(code: &str) -> String {
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
            .map(|l| if l.trim().is_empty() { String::new() } else { l[min_indent..].to_string() })
            .collect::<Vec<_>>()
            .join("\n")
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

    fn resolve_work_dir(&self) -> Result<PathBuf> {
        if let Some(ref root) = self.project_root {
            if root.as_os_str().to_string_lossy().contains("runtime-env") {
                return Ok(root.clone());
            }
            let dir = root.join(".polybench").join("zig");
            std::fs::create_dir_all(&dir)
                .map_err(|e| miette!("Failed to create .polybench/zig directory: {}", e))?;
            return Ok(dir);
        }
        let dir = std::env::temp_dir().join("polybench-zig");
        std::fs::create_dir_all(&dir)
            .map_err(|e| miette!("Failed to create temp Zig directory: {}", e))?;
        Ok(dir)
    }

    fn write_source_and_build(
        &self,
        work_dir: &Path,
        source: &str,
        source_name: &str,
        output_name: &str,
    ) -> Result<PathBuf> {
        let source_path = work_dir.join(source_name);
        std::fs::write(&source_path, source)
            .map_err(|e| miette!("Failed to write {}: {}", source_path.display(), e))?;

        let binary_path = work_dir.join(output_name);
        let output = std::process::Command::new(&self.zig_binary)
            .args([
                "build-exe",
                "-O",
                "ReleaseFast",
                source_path.to_string_lossy().as_ref(),
                &format!("-femit-bin={}", binary_path.to_string_lossy()),
            ])
            .current_dir(work_dir)
            .output()
            .map_err(|e| miette!("Failed to run zig: {}", e))?;

        if !output.status.success() {
            return Err(miette!(
                "Zig compilation failed:\n{}\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        if !binary_path.exists() {
            return Err(miette!(
                "Zig build succeeded but binary not found at {}",
                binary_path.display()
            ));
        }

        Ok(binary_path)
    }
}

impl Default for ZigRuntime {
    fn default() -> Self {
        Self::new().expect("Failed to create Zig runtime")
    }
}

pub struct ZigRuntimeFactory;
pub static ZIG_RUNTIME_FACTORY: ZigRuntimeFactory = ZigRuntimeFactory;

impl RuntimeFactory for ZigRuntimeFactory {
    fn lang(&self) -> Lang {
        Lang::Zig
    }

    fn name(&self) -> &'static str {
        "Zig Runtime"
    }

    fn create(&self, config: &RuntimeConfig) -> Result<Box<dyn Runtime>> {
        let mut rt = ZigRuntime::new()?;
        rt.set_project_root(config.get_root(poly_bench_dsl::Lang::Zig));
        Ok(Box::new(rt))
    }
}

#[async_trait]
impl Runtime for ZigRuntime {
    fn name(&self) -> &'static str {
        "Zig Runtime"
    }

    fn lang(&self) -> Lang {
        Lang::Zig
    }

    fn last_precompile_nanos(&self) -> Option<u64> {
        self.last_precompile_nanos
    }

    fn set_anvil_rpc_url(&mut self, url: String) {
        self.anvil_rpc_url = Some(url);
    }

    async fn initialize(&mut self, _suite: &SuiteIR) -> Result<()> {
        which::which("zig").map_err(|_| miette!("zig not found in PATH"))?;
        Ok(())
    }

    fn generate_check_source(&self, spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<String> {
        generate_zig_source(spec, suite, true)
    }

    async fn compile_check(&self, spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<()> {
        let source = self.generate_check_source(spec, suite)?;
        let work_dir = self.resolve_work_dir()?;
        let safe_name = spec.full_name.replace('.', "_").replace('/', "_");
        let source_name = format!("bench_check_{}.zig", safe_name);
        self.write_source_and_build(&work_dir, &source, &source_name, "polybench_check")?;
        Ok(())
    }

    async fn precompile(&mut self, spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<()> {
        let source = generate_zig_source(spec, suite, false)?;
        let source_hash = Self::hash_source(&source);

        if let Some((ref binary_path, cached_hash)) = self.cached_binary {
            if cached_hash == source_hash && binary_path.exists() {
                self.last_precompile_nanos = Some(0);
                return Ok(());
            }
        }

        let pc_start = std::time::Instant::now();
        let work_dir = self.resolve_work_dir()?;
        let binary_path = self.write_source_and_build(
            &work_dir,
            &source,
            "bench_standalone.zig",
            "polybench_runner",
        )?;
        self.cached_binary = Some((binary_path, source_hash));
        self.last_precompile_nanos = Some(pc_start.elapsed().as_nanos() as u64);
        Ok(())
    }

    async fn run_benchmark(
        &mut self,
        spec: &BenchmarkSpec,
        suite: &SuiteIR,
    ) -> Result<Measurement> {
        let source = generate_zig_source(spec, suite, false)?;
        let source_hash = Self::hash_source(&source);
        let work_dir = self.resolve_work_dir()?;

        let binary_path = if let Some((ref binary_path, cached_hash)) = self.cached_binary {
            if cached_hash == source_hash && binary_path.exists() {
                binary_path.clone()
            } else {
                let built = self.write_source_and_build(
                    &work_dir,
                    &source,
                    "bench_standalone.zig",
                    "polybench_runner",
                )?;
                self.cached_binary = Some((built.clone(), source_hash));
                built
            }
        } else {
            let built = self.write_source_and_build(
                &work_dir,
                &source,
                "bench_standalone.zig",
                "polybench_runner",
            )?;
            self.cached_binary = Some((built.clone(), source_hash));
            built
        };

        let mut cmd = tokio::process::Command::new(&binary_path);
        cmd.current_dir(&work_dir)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if let Some(ref url) = self.anvil_rpc_url {
            cmd.env("ANVIL_RPC_URL", url);
        }
        cmd.kill_on_drop(true);

        let run_start = std::time::Instant::now();
        let child = cmd.spawn().map_err(|e| miette!("Failed to run Zig benchmark: {}", e))?;
        let output = if let Some(timeout_ms) = spec.timeout {
            match tokio::time::timeout(
                tokio::time::Duration::from_millis(timeout_ms),
                child.wait_with_output(),
            )
            .await
            {
                Ok(r) => r.map_err(|e| miette!("Failed waiting for Zig benchmark: {}", e))?,
                Err(_) => return Err(miette!("Zig benchmark timed out after {}ms", timeout_ms)),
            }
        } else {
            child
                .wait_with_output()
                .await
                .map_err(|e| miette!("Failed waiting for Zig benchmark: {}", e))?
        };

        if !output.status.success() {
            return Err(miette!(
                "Zig benchmark failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            ));
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
        Ok(())
    }
}

fn emit_fixtures(spec: &BenchmarkSpec, suite: &SuiteIR, indent: &str) -> String {
    let mut out = String::new();
    for fixture_name in &spec.fixture_refs {
        if let Some(fixture) = suite.get_fixture(fixture_name) {
            if let Some(fixture_impl) = fixture.implementations.get(&Lang::Zig) {
                out.push_str(&format!("{}{} = {};\n", indent, fixture_name, fixture_impl));
            } else if !fixture.data.is_empty() {
                let bytes: Vec<String> =
                    fixture.data.iter().map(|b| format!("0x{:02x}", b)).collect();
                out.push_str(&format!(
                    "{}var {}: [{}]u8 = .{{ {} }};\n",
                    indent,
                    fixture_name,
                    fixture.data.len(),
                    bytes.join(", ")
                ));
            }
        }
    }
    out
}

fn emit_hook(code: Option<&String>, indent: &str) -> String {
    code.map(|c| {
        ZigRuntime::normalize_indent(c)
            .lines()
            .map(|l| format!("{}{}\n", indent, l))
            .collect::<String>()
    })
    .unwrap_or_default()
}

fn generate_zig_source(spec: &BenchmarkSpec, suite: &SuiteIR, check_only: bool) -> Result<String> {
    let impl_code = spec
        .get_impl(Lang::Zig)
        .ok_or_else(|| miette!("No Zig implementation for benchmark {}", spec.name))?;
    let impl_code = ZigRuntime::normalize_indent(impl_code.trim());

    let mut src = String::new();
    src.push_str("const std = @import(\"std\");\n\n");

    let mut import_set: HashSet<String> = HashSet::new();
    if let Some(imports) = suite.imports.get(&Lang::Zig) {
        for import_stmt in imports {
            import_set.insert(import_stmt.trim().to_string());
        }
    }
    let stdlib_imports = stdlib::get_stdlib_imports(&suite.stdlib_imports, &crate::ZIG_STDLIB);
    for import_stmt in stdlib_imports {
        import_set.insert(import_stmt.trim().to_string());
    }
    let mut imports: Vec<_> = import_set.into_iter().collect();
    imports.sort();
    for imp in imports {
        src.push_str(&imp);
        src.push('\n');
    }
    if !src.ends_with("\n\n") {
        src.push('\n');
    }

    src.push_str("var __polybench_sink: ?*const anyopaque = null;\n\n");

    let stdlib_code = stdlib::get_stdlib_code(&suite.stdlib_imports, &crate::ZIG_STDLIB);
    if !stdlib_code.is_empty() {
        src.push_str(&stdlib_code);
        src.push_str("\n\n");
    }

    if let Some(declarations) = suite.declarations.get(&Lang::Zig) {
        if !declarations.trim().is_empty() {
            src.push_str(&ZigRuntime::normalize_indent(declarations));
            src.push_str("\n\n");
        }
    }
    if let Some(helpers) = suite.helpers.get(&Lang::Zig) {
        if !helpers.trim().is_empty() {
            src.push_str(&ZigRuntime::normalize_indent(helpers));
            src.push_str("\n\n");
        }
    }
    if let Some(init_code) = suite.init_code.get(&Lang::Zig) {
        if !init_code.trim().is_empty() {
            src.push_str("fn __polybench_init() void {\n");
            for line in ZigRuntime::normalize_indent(init_code).lines() {
                src.push_str("    ");
                src.push_str(line);
                src.push('\n');
            }
            src.push_str("}\n\n");
        }
    }

    // Emit fixtures at module level so we can use doNotOptimizeAway (prevents optimizer from
    // eliminating benchmark)
    src.push_str(&emit_fixtures(spec, suite, ""));
    src.push_str("\n");
    src.push_str("fn __polybench_bench() void {\n");
    for line in impl_code.lines() {
        src.push_str("    ");
        src.push_str(line);
        src.push('\n');
    }
    src.push_str("}\n\n");

    if check_only {
        src.push_str("pub fn main() void {}\n");
        return Ok(src);
    }

    src.push_str("pub fn main() !void {\n");
    if suite.init_code.get(&Lang::Zig).is_some() {
        src.push_str("    __polybench_init();\n");
    }
    src.push_str(&emit_hook(spec.before_hooks.get(&Lang::Zig), "    "));

    if spec.warmup_time_ms > 0 {
        src.push_str(&format!(
            "    const __warmup_start = std.time.Instant.now() catch return;\n    const __warmup_limit = @as(u64, @intFromFloat({} * 1e6));\n    while ((std.time.Instant.now() catch return).since(__warmup_start) < __warmup_limit) {{\n",
            spec.warmup_time_ms
        ));
    } else if spec.warmup_iterations > 0 {
        src.push_str(&format!(
            "    const __warmup_start = std.time.Instant.now() catch return;\n    for (0..{}) |_| {{\n",
            spec.warmup_iterations
        ));
    }
    if spec.warmup_time_ms > 0 || spec.warmup_iterations > 0 {
        src.push_str(&emit_hook(spec.each_hooks.get(&Lang::Zig), "        "));
        src.push_str("        __polybench_bench();\n");
        src.push_str(
            "    }\n    const __warmup_nanos = (std.time.Instant.now() catch return).since(__warmup_start);\n\n",
        );
    } else {
        src.push_str("    const __warmup_nanos: u64 = 0;\n");
    }

    if spec.mode == BenchMode::Auto {
        src.push_str(&format!(
            "    const __allocator = std.heap.page_allocator;\n    const target_ns = {:.0};\n    var total_iterations: u64 = 0;\n    var total_ns: f64 = 0;\n    var batch: u64 = 100;\n    var samples = std.ArrayList(f64).initCapacity(__allocator, 16) catch return;\n    defer samples.deinit(__allocator);\n    while (total_ns < target_ns) {{\n        const t0 = std.time.Instant.now() catch return;\n        for (0..batch) |_| {{\n",
            (spec.target_time_ms as f64) * 1_000_000.0
        ));
        src.push_str(&emit_hook(spec.each_hooks.get(&Lang::Zig), "            "));
        src.push_str("            __polybench_bench();\n");
        if let Some(first_fixture) = spec.fixture_refs.first() {
            src.push_str(&format!(
                "            std.mem.doNotOptimizeAway({}[0..1]);\n",
                first_fixture
            ));
        }
        src.push_str("        }\n        const t1 = std.time.Instant.now() catch return;\n");
        src.push_str("        const elapsed_ns = @as(f64, @floatFromInt(t1.since(t0)));\n");
        src.push_str("        total_ns += elapsed_ns;\n        total_iterations += batch;\n");
        src.push_str("        _ = samples.append(__allocator, elapsed_ns / @as(f64, @floatFromInt(if (batch > 0) batch else 1))) catch break;\n");
        src.push_str("        if (elapsed_ns > 0) {\n");
        src.push_str(
            "            const remaining = @max(@as(f64, 0), target_ns - total_ns);\n            const next_f = (@as(f64, @floatFromInt(batch)) * remaining / elapsed_ns) * 1.1;\n            batch = @max(@as(u64, 1), @min(@as(u64, @intFromFloat(next_f)), 1000000));\n",
        );
        src.push_str(
            "        } else {\n            batch = @min(batch * 2, 1000000);\n        }\n    }\n",
        );
        src.push_str(
            "    const nanos_per_op = total_ns / @as(f64, @floatFromInt(if (total_iterations > 0) total_iterations else 1));\n    const ops_per_sec = 1000000000.0 / (if (nanos_per_op > 0) nanos_per_op else 1.0);\n",
        );
        src.push_str(&emit_hook(spec.after_hooks.get(&Lang::Zig), "    "));
        src.push_str("    var __stdout_buffer: [4096]u8 = undefined;\n    var __stdout_writer = std.fs.File.stdout().writer(&__stdout_buffer);\n    const stdout = &__stdout_writer.interface;\n");
        src.push_str("    try stdout.print(\"{{\\\"iterations\\\":{},\\\"totalNanos\\\":{d:.0},\\\"warmupNanos\\\":{},\\\"nanosPerOp\\\":{d:.6},\\\"opsPerSec\\\":{d:.6},\\\"samples\\\":[\", .{ total_iterations, total_ns, __warmup_nanos, nanos_per_op, ops_per_sec });\n");
        src.push_str("    for (samples.items, 0..) |s, i| {\n");
        src.push_str("        if (i > 0) _ = stdout.writeAll(\",\") catch {};\n");
        src.push_str("    try stdout.print(\"{d:.0}\", .{s});\n");
        src.push_str("    }\n");
        src.push_str("    _ = stdout.writeAll(\"]}\\n\") catch {};\n");
        src.push_str("    stdout.flush() catch {};\n");
    } else {
        src.push_str(&format!(
            "    const iterations: u64 = {};\n    var samples: [{}]f64 = undefined;\n    for (0..iterations) |i| {{\n        const t0 = std.time.Instant.now() catch return;\n",
            spec.iterations, spec.iterations
        ));
        src.push_str(&emit_hook(spec.each_hooks.get(&Lang::Zig), "        "));
        src.push_str("        __polybench_bench();\n");
        src.push_str("        const t1 = std.time.Instant.now() catch return;\n");
        src.push_str("        samples[i] = @as(f64, @floatFromInt(t1.since(t0)));\n");
        src.push_str("    }\n");
        src.push_str(
            "    var total_ns: f64 = 0;\n    for (samples) |s| total_ns += s;\n    const nanos_per_op = total_ns / @as(f64, @floatFromInt(if (iterations > 0) iterations else 1));\n    const ops_per_sec = 1000000000.0 / (if (nanos_per_op > 0) nanos_per_op else 1.0);\n",
        );
        src.push_str(&emit_hook(spec.after_hooks.get(&Lang::Zig), "    "));
        src.push_str("    var __stdout_buffer: [4096]u8 = undefined;\n    var __stdout_writer = std.fs.File.stdout().writer(&__stdout_buffer);\n    const stdout = &__stdout_writer.interface;\n");
        src.push_str("    try stdout.print(\"{{\\\"iterations\\\":{},\\\"totalNanos\\\":{d:.0},\\\"warmupNanos\\\":{},\\\"nanosPerOp\\\":{d:.6},\\\"opsPerSec\\\":{d:.6},\\\"samples\\\":[\", .{ iterations, total_ns, __warmup_nanos, nanos_per_op, ops_per_sec });\n");
        src.push_str("    for (samples, 0..) |s, i| {\n");
        src.push_str("        if (i > 0) _ = stdout.writeAll(\",\") catch {};\n");
        src.push_str("        try stdout.print(\"{d:.0}\", .{s});\n");
        src.push_str("    }\n");
        src.push_str("    _ = stdout.writeAll(\"]}\\n\") catch {};\n");
        src.push_str("    stdout.flush() catch {};\n");
    }

    src.push_str("}\n");
    Ok(src)
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct BenchResultJson {
    iterations: u64,
    total_nanos: f64,
    #[serde(default)]
    warmup_nanos: Option<f64>,
    #[serde(default)]
    samples: Vec<f64>,
    #[serde(default)]
    raw_result: Option<String>,
}

impl BenchResultJson {
    fn into_measurement(self, outlier_detection: bool, cv_threshold: f64) -> Measurement {
        let sample_u64: Vec<u64> = self.samples.iter().map(|s| *s as u64).collect();
        let mut m = if sample_u64.is_empty() {
            Measurement::from_aggregate(self.iterations, self.total_nanos as u64)
        } else {
            Measurement::from_aggregate_with_sample_stats(
                self.iterations,
                self.total_nanos as u64,
                sample_u64,
                outlier_detection,
                cv_threshold,
            )
        };
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
        .ok_or_else(|| miette!("No benchmark output from Zig runtime"))?;
    let result: BenchResultJson = serde_json::from_str(json_line)
        .map_err(|e| miette!("Failed to parse Zig benchmark output: {}\n{}", e, stdout))?;
    Ok(result.into_measurement(outlier_detection, cv_threshold))
}
