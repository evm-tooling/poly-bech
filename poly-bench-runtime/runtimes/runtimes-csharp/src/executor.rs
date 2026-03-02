//! C# runtime executor.

use async_trait::async_trait;
use miette::{miette, Result};
use poly_bench_dsl::{BenchMode, Lang};
use poly_bench_ir::{BenchmarkSpec, SuiteIR};
use poly_bench_traits::{Measurement, Runtime, RuntimeConfig, RuntimeFactory};
use regex::Regex;
use std::{
    path::{Path, PathBuf},
    process::Stdio,
    time::{SystemTime, UNIX_EPOCH},
};

pub struct CSharpRuntime {
    dotnet_binary: PathBuf,
    project_root: Option<PathBuf>,
    anvil_rpc_url: Option<String>,
    /// Cached compiled DLL path and source hash for reuse across runs
    cached_binary: Option<(PathBuf, u64)>,
    /// Duration of last precompile in nanoseconds (for accurate reporting)
    last_precompile_nanos: Option<u64>,
}

impl CSharpRuntime {
    pub fn new() -> Result<Self> {
        let (dotnet_binary, _) = poly_bench_traits::resolve_binary(poly_bench_dsl::Lang::CSharp)
            .map_err(|_| miette!("dotnet not found. Install via 'poly-bench add-runtime csharp' or ensure dotnet is in PATH"))?;
        Ok(Self {
            dotnet_binary,
            project_root: None,
            anvil_rpc_url: None,
            cached_binary: None,
            last_precompile_nanos: None,
        })
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

    pub fn set_project_root(&mut self, path: Option<PathBuf>) {
        self.project_root = path;
    }

    fn resolve_work_dir(&self) -> Result<PathBuf> {
        if let Some(ref root) = self.project_root {
            if root.as_os_str().to_string_lossy().contains("runtime-env") {
                return Ok(root.clone());
            }
            let dir = root.join(".polybench").join("csharp");
            std::fs::create_dir_all(&dir)
                .map_err(|e| miette!("Failed to create .polybench/csharp directory: {}", e))?;
            return Ok(dir);
        }
        let dir = std::env::temp_dir().join("polybench-csharp");
        std::fs::create_dir_all(&dir)
            .map_err(|e| miette!("Failed to create temp csharp directory: {}", e))?;
        Ok(dir)
    }

    fn ensure_project_files(&self, dir: &Path) -> Result<PathBuf> {
        let project_path = dir.join("polybench.csproj");
        if !project_path.exists() {
            let is_runtime_env = dir.as_os_str().to_string_lossy().contains("runtime-env");
            if !is_runtime_env {
                return Err(miette!(
                    "C# benchmarks with external dependencies require runtime-env. Run `poly-bench build` first."
                ));
            }
            std::fs::write(&project_path, default_csproj())
                .map_err(|e| miette!("Failed to write project file: {}", e))?;
        }
        Ok(project_path)
    }

    /// Parse TargetFramework from csproj (e.g. "net8.0"). Falls back to "net8.0" if not found.
    fn parse_target_framework(project_path: &Path) -> Result<String> {
        let content = std::fs::read_to_string(project_path).map_err(|e| {
            miette!("Failed to read project file {}: {}", project_path.display(), e)
        })?;
        let re = Regex::new(r"<TargetFramework>([^<]+)</TargetFramework>")
            .map_err(|e| miette!("Invalid regex: {}", e))?;
        if let Some(cap) = re.captures(&content) {
            return Ok(cap.get(1).unwrap().as_str().trim().to_string());
        }
        // Fallback for TargetFrameworks (multi-target): take first
        let re_multi = Regex::new(r"<TargetFrameworks>([^<]+)</TargetFrameworks>")
            .map_err(|e| miette!("Invalid regex: {}", e))?;
        if let Some(cap) = re_multi.captures(&content) {
            let first = cap.get(1).unwrap().as_str().split(';').next().unwrap_or("net8.0").trim();
            return Ok(first.to_string());
        }
        Ok("net8.0".to_string())
    }

    /// Create global.json to pin SDK to the version matching TargetFramework (e.g. net8.0 ->
    /// 8.0.x). This avoids duplicate assembly attribute errors when a newer SDK (e.g. .NET 11)
    /// builds older targets.
    fn ensure_global_json(work_dir: &Path, target_framework: &str) -> Result<()> {
        let sdk_version: String = match target_framework {
            "net8.0" => "8.0.0".to_string(),
            "net9.0" => "9.0.0".to_string(),
            "net7.0" => "7.0.0".to_string(),
            "net6.0" => "6.0.0".to_string(),
            _ => {
                let re =
                    Regex::new(r"net(\d+)\.(\d+)").map_err(|e| miette!("Invalid regex: {}", e))?;
                if let Some(cap) = re.captures(target_framework) {
                    format!("{}.{}.0", cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str())
                } else {
                    return Ok(());
                }
            }
        };
        let json =
            format!(r#"{{"sdk":{{"version":"{}","rollForward":"latestFeature"}}}}"#, sdk_version);
        let path = work_dir.join("global.json");
        std::fs::write(&path, json).map_err(|e| miette!("Failed to write global.json: {}", e))?;
        Ok(())
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

    fn emit_hook(code: Option<&String>, indent: &str) -> String {
        code.map(|c| {
            Self::normalize_indent(c)
                .lines()
                .map(|l| {
                    let mut line = l.to_string();
                    let trimmed = line.trim_end();
                    if !trimmed.is_empty() &&
                        !trimmed.ends_with(';') &&
                        !trimmed.ends_with('{') &&
                        !trimmed.ends_with('}')
                    {
                        line.push(';');
                    }
                    format!("{}{}\n", indent, line)
                })
                .collect::<String>()
        })
        .unwrap_or_default()
    }

    fn unique_build_root(work_dir: &Path) -> Result<PathBuf> {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
        let root = std::env::temp_dir()
            .join("polybench-csharp-build")
            .join(work_dir.file_name().unwrap_or_default())
            .join(format!("{}-{}", std::process::id(), stamp));
        std::fs::create_dir_all(&root)
            .map_err(|e| miette!("Failed to create C# build root {}: {}", root.display(), e))?;
        Ok(root)
    }

    fn cleanup_local_build_artifacts(work_dir: &Path) {
        let _ = std::fs::remove_dir_all(work_dir.join("obj"));
        let _ = std::fs::remove_dir_all(work_dir.join("bin"));
        let _ = std::fs::remove_dir_all(work_dir.join(".polybench-build"));
    }

    /// Build the DLL and cache it for reuse
    async fn build_dll(
        &mut self,
        _spec: &BenchmarkSpec,
        _suite: &SuiteIR,
        work_dir: &Path,
        source: &str,
        source_hash: u64,
    ) -> Result<PathBuf> {
        let project_path = self.ensure_project_files(work_dir)?;
        let target_framework = Self::parse_target_framework(&project_path)?;
        Self::ensure_global_json(work_dir, &target_framework)?;
        Self::cleanup_local_build_artifacts(work_dir);
        let build_root = Self::unique_build_root(work_dir)?;
        let obj_path = build_root.join("obj");
        let bin_path = build_root.join("bin");

        std::fs::write(work_dir.join("Program.cs"), source)
            .map_err(|e| miette!("Failed to write Program.cs: {}", e))?;

        let build = tokio::process::Command::new(&self.dotnet_binary)
            .args([
                "build",
                "-nologo",
                "-c",
                "Release",
                "-f",
                &target_framework,
                "-p:UseAppHost=false",
                &format!("-p:BaseIntermediateOutputPath={}/", obj_path.to_string_lossy()),
                &format!("-p:BaseOutputPath={}/", bin_path.to_string_lossy()),
                project_path.to_string_lossy().as_ref(),
            ])
            .current_dir(work_dir)
            .output()
            .await
            .map_err(|e| miette!("Failed to run dotnet build: {}", e))?;

        if !build.status.success() {
            return Err(miette!(
                "C# build failed:\n{}\n{}",
                String::from_utf8_lossy(&build.stdout),
                String::from_utf8_lossy(&build.stderr)
            ));
        }

        let dll_path = bin_path.join("Release").join(&target_framework).join("polybench.dll");
        if !dll_path.exists() {
            return Err(miette!(
                "C# build succeeded but output DLL not found at {}",
                dll_path.display()
            ));
        }

        // Cache the DLL path and source hash for reuse
        self.cached_binary = Some((dll_path.clone(), source_hash));

        Ok(dll_path)
    }
}

impl Default for CSharpRuntime {
    fn default() -> Self {
        Self::new().expect("Failed to create C# runtime")
    }
}

pub struct CSharpRuntimeFactory;
pub static CSHARP_RUNTIME_FACTORY: CSharpRuntimeFactory = CSharpRuntimeFactory;

impl RuntimeFactory for CSharpRuntimeFactory {
    fn lang(&self) -> Lang {
        Lang::CSharp
    }

    fn name(&self) -> &'static str {
        "C# Runtime"
    }

    fn create(&self, config: &RuntimeConfig) -> Result<Box<dyn Runtime>> {
        let mut rt = CSharpRuntime::new()?;
        rt.set_project_root(config.get_root(poly_bench_dsl::Lang::CSharp));
        Ok(Box::new(rt))
    }
}

#[async_trait]
impl Runtime for CSharpRuntime {
    fn name(&self) -> &'static str {
        "C# Runtime"
    }

    fn lang(&self) -> Lang {
        Lang::CSharp
    }

    fn set_anvil_rpc_url(&mut self, url: String) {
        self.anvil_rpc_url = Some(url);
    }

    fn last_precompile_nanos(&self) -> Option<u64> {
        self.last_precompile_nanos
    }

    async fn initialize(&mut self, _suite: &SuiteIR) -> Result<()> {
        poly_bench_traits::resolve_binary(poly_bench_dsl::Lang::CSharp)
            .map_err(|_| miette!("dotnet not found. Install via 'poly-bench add-runtime csharp' or ensure dotnet is in PATH"))?;
        Ok(())
    }

    fn generate_check_source(&self, spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<String> {
        generate_csharp_source(spec, suite, true)
    }

    async fn compile_check(&self, spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<()> {
        let work_dir = self.resolve_work_dir()?;
        let project_path = self.ensure_project_files(&work_dir)?;
        let target_framework = Self::parse_target_framework(&project_path)?;
        Self::ensure_global_json(&work_dir, &target_framework)?;
        Self::cleanup_local_build_artifacts(&work_dir);
        let build_root = Self::unique_build_root(&work_dir)?;
        let obj_path = build_root.join("obj");
        let bin_path = build_root.join("bin");
        let source = self.generate_check_source(spec, suite)?;
        std::fs::write(work_dir.join("Program.cs"), source)
            .map_err(|e| miette!("Failed to write Program.cs: {}", e))?;

        let output = tokio::process::Command::new(&self.dotnet_binary)
            .args([
                "build",
                "-nologo",
                "-c",
                "Release",
                "-f",
                &target_framework,
                "-p:UseAppHost=false",
                &format!("-p:BaseIntermediateOutputPath={}/", obj_path.to_string_lossy()),
                &format!("-p:BaseOutputPath={}/", bin_path.to_string_lossy()),
                project_path.to_string_lossy().as_ref(),
            ])
            .current_dir(&work_dir)
            .output()
            .await
            .map_err(|e| miette!("Failed to run dotnet build: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            return Err(miette!("C# compilation failed:\n{}\n{}", stdout.trim(), stderr.trim()));
        }

        Ok(())
    }

    async fn precompile(&mut self, spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<()> {
        let source = generate_csharp_source(spec, suite, false)?;
        let source_hash = Self::hash_source(&source);

        // Check if we already have a cached binary with matching source hash
        if let Some((ref dll_path, cached_hash)) = self.cached_binary {
            if cached_hash == source_hash && dll_path.exists() {
                // Already compiled, nothing to do
                self.last_precompile_nanos = Some(0);
                return Ok(());
            }
        }

        let pc_start = std::time::Instant::now();

        let work_dir = self.resolve_work_dir()?;
        let project_path = self.ensure_project_files(&work_dir)?;
        let target_framework = Self::parse_target_framework(&project_path)?;
        Self::ensure_global_json(&work_dir, &target_framework)?;
        Self::cleanup_local_build_artifacts(&work_dir);
        let build_root = Self::unique_build_root(&work_dir)?;
        let obj_path = build_root.join("obj");
        let bin_path = build_root.join("bin");

        std::fs::write(work_dir.join("Program.cs"), &source)
            .map_err(|e| miette!("Failed to write Program.cs: {}", e))?;

        let build = tokio::process::Command::new(&self.dotnet_binary)
            .args([
                "build",
                "-nologo",
                "-c",
                "Release",
                "-f",
                &target_framework,
                "-p:UseAppHost=false",
                &format!("-p:BaseIntermediateOutputPath={}/", obj_path.to_string_lossy()),
                &format!("-p:BaseOutputPath={}/", bin_path.to_string_lossy()),
                project_path.to_string_lossy().as_ref(),
            ])
            .current_dir(&work_dir)
            .output()
            .await
            .map_err(|e| miette!("Failed to run dotnet build: {}", e))?;

        if !build.status.success() {
            return Err(miette!(
                "C# build failed:\n{}\n{}",
                String::from_utf8_lossy(&build.stdout),
                String::from_utf8_lossy(&build.stderr)
            ));
        }

        let dll_path = bin_path.join("Release").join(&target_framework).join("polybench.dll");
        if !dll_path.exists() {
            return Err(miette!(
                "C# build succeeded but output DLL not found at {}",
                dll_path.display()
            ));
        }

        // Cache the DLL path and source hash for reuse
        self.cached_binary = Some((dll_path, source_hash));
        self.last_precompile_nanos = Some(pc_start.elapsed().as_nanos() as u64);

        Ok(())
    }

    async fn run_benchmark(
        &mut self,
        spec: &BenchmarkSpec,
        suite: &SuiteIR,
    ) -> Result<Measurement> {
        let source = generate_csharp_source(spec, suite, false)?;
        let source_hash = Self::hash_source(&source);
        let work_dir = self.resolve_work_dir()?;

        // Check if we have a cached DLL with matching source hash
        let dll_path = if let Some((ref cached_path, cached_hash)) = self.cached_binary {
            if cached_hash == source_hash && cached_path.exists() {
                // Reuse cached DLL
                cached_path.clone()
            } else {
                // Cache miss - need to rebuild
                self.build_dll(spec, suite, &work_dir, &source, source_hash).await?
            }
        } else {
            // No cache - need to build
            self.build_dll(spec, suite, &work_dir, &source, source_hash).await?
        };

        let mut cmd = tokio::process::Command::new(&self.dotnet_binary);
        cmd.arg(dll_path.to_string_lossy().as_ref())
            .current_dir(&work_dir)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if let Some(ref url) = self.anvil_rpc_url {
            cmd.env("ANVIL_RPC_URL", url);
        }
        cmd.kill_on_drop(true);

        let run_start = std::time::Instant::now();
        let child = cmd.spawn().map_err(|e| miette!("Failed to run C# benchmark: {}", e))?;
        let output = if let Some(timeout_ms) = spec.timeout {
            match tokio::time::timeout(
                tokio::time::Duration::from_millis(timeout_ms),
                child.wait_with_output(),
            )
            .await
            {
                Ok(r) => r.map_err(|e| miette!("Failed waiting for C# benchmark: {}", e))?,
                Err(_) => return Err(miette!("C# benchmark timed out after {}ms", timeout_ms)),
            }
        } else {
            child
                .wait_with_output()
                .await
                .map_err(|e| miette!("Failed waiting for C# benchmark: {}", e))?
        };

        if !output.status.success() {
            return Err(miette!(
                "C# benchmark failed:\n{}",
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

fn default_csproj() -> String {
    r#"<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net8.0</TargetFramework>
    <ImplicitUsings>enable</ImplicitUsings>
    <Nullable>enable</Nullable>
    <UseAppHost>false</UseAppHost>
  </PropertyGroup>
</Project>
"#
    .to_string()
}

fn emit_fixtures(spec: &BenchmarkSpec, suite: &SuiteIR, indent: &str) -> String {
    let mut out = String::new();
    for fixture_name in &spec.fixture_refs {
        if let Some(fixture) = suite.get_fixture(fixture_name) {
            if let Some(fixture_impl) = fixture.implementations.get(&Lang::CSharp) {
                out.push_str(&format!("{}var {} = {};\n", indent, fixture_name, fixture_impl));
            } else if !fixture.data.is_empty() {
                let bytes: Vec<String> =
                    fixture.data.iter().map(|b| format!("0x{:02x}", b)).collect();
                out.push_str(&format!(
                    "{}byte[] {} = new byte[] {{ {} }};\n",
                    indent,
                    fixture_name,
                    bytes.join(", ")
                ));
            }
        }
    }
    out
}

fn generate_csharp_source(
    spec: &BenchmarkSpec,
    suite: &SuiteIR,
    check_only: bool,
) -> Result<String> {
    let impl_code = spec
        .get_impl(Lang::CSharp)
        .ok_or_else(|| miette!("No C# implementation for benchmark {}", spec.name))?;
    let impl_code = CSharpRuntime::normalize_indent(impl_code.trim());

    let mut src = String::new();
    src.push_str("using System;\nusing System.Diagnostics;\nusing System.Collections.Generic;\n");
    src.push_str("using System.Text.Json;\n\n");
    if let Some(imports) = suite.imports.get(&Lang::CSharp) {
        for import_stmt in imports {
            src.push_str(import_stmt);
            src.push('\n');
        }
    }
    src.push('\n');

    // Use a non-Program entry type to avoid conflicts with compiler-generated Program.
    src.push_str("public static class PolybenchProgram {\n");
    src.push_str("    static object? __polybench_sink;\n\n");

    if let Some(declarations) = suite.declarations.get(&Lang::CSharp) {
        if !declarations.trim().is_empty() {
            for line in CSharpRuntime::normalize_indent(declarations).lines() {
                src.push_str("    ");
                src.push_str(line);
                src.push('\n');
            }
            src.push('\n');
        }
    }
    if let Some(helpers) = suite.helpers.get(&Lang::CSharp) {
        if !helpers.trim().is_empty() {
            for line in CSharpRuntime::normalize_indent(helpers).lines() {
                src.push_str("    ");
                src.push_str(line);
                src.push('\n');
            }
            src.push('\n');
        }
    }

    src.push_str("    static object? __polybench_bench() {\n");
    src.push_str(&emit_fixtures(spec, suite, "        "));
    for line in impl_code.lines() {
        src.push_str("        ");
        src.push_str(line);
        if !line.trim_end().ends_with(';') && !line.trim_end().ends_with('}') {
            src.push(';');
        }
        src.push('\n');
    }
    src.push_str("        return null;\n");
    src.push_str("    }\n\n");

    if check_only {
        src.push_str("    public static void Main() { }\n}\n");
        return Ok(src);
    }

    src.push_str("    public static void Main() {\n");
    if let Some(init_code) = suite.init_code.get(&Lang::CSharp) {
        if !init_code.trim().is_empty() {
            for line in CSharpRuntime::normalize_indent(init_code).lines() {
                src.push_str("        ");
                src.push_str(line);
                src.push('\n');
            }
            src.push('\n');
        }
    }
    src.push_str("        var samples = new List<double>();\n");
    src.push_str("        long warmupNanos = 0;\n");
    // Warmup (warmup_time_ms takes precedence over warmup_iterations)
    if spec.warmup_time_ms > 0 {
        src.push_str(&format!(
            "        var warmupStart = Stopwatch.GetTimestamp();\n        var warmupLimitNs = (double){} * 1_000_000;\n        while ((Stopwatch.GetTimestamp() - warmupStart) * (1_000_000_000.0 / Stopwatch.Frequency) < warmupLimitNs) {{\n",
            spec.warmup_time_ms
        ));
    } else if spec.warmup_iterations > 0 {
        src.push_str(&format!("        var warmupStart = Stopwatch.GetTimestamp();\n        for (int i = 0; i < {}; i++) {{\n", spec.warmup_iterations));
    }
    if spec.warmup_time_ms > 0 || spec.warmup_iterations > 0 {
        src.push_str(&CSharpRuntime::emit_hook(spec.each_hooks.get(&Lang::CSharp), "            "));
        if spec.use_sink {
            src.push_str("            __polybench_sink = __polybench_bench();\n");
        } else {
            src.push_str("            __polybench_bench();\n");
        }
        src.push_str("        }\n        warmupNanos = (long)((Stopwatch.GetTimestamp() - warmupStart) * (1_000_000_000.0 / Stopwatch.Frequency));\n\n");
    }

    src.push_str(&CSharpRuntime::emit_hook(spec.before_hooks.get(&Lang::CSharp), "        "));

    let use_memory = spec.memory;
    if use_memory {
        // Use GC.GetTotalAllocatedBytes() for cumulative allocation tracking (like Go's TotalAlloc)
        // This never decreases, so it captures allocations even if they're freed during the
        // benchmark
        src.push_str("        long memBefore = GC.GetTotalAllocatedBytes(true);\n");
    }

    if spec.mode == BenchMode::Auto {
        src.push_str(&format!(
            "        double targetNs = {};\n        long totalIterations = 0;\n        double totalNs = 0;\n        int batch = 1;\n        while (totalNs < targetNs) {{\n            var t0 = Stopwatch.GetTimestamp();\n            for (int i = 0; i < batch; i++) {{\n",
            (spec.target_time_ms as f64) * 1_000_000.0
        ));
        src.push_str(&CSharpRuntime::emit_hook(
            spec.each_hooks.get(&Lang::CSharp),
            "                ",
        ));
        if spec.use_sink {
            src.push_str("                __polybench_sink = __polybench_bench();\n");
        } else {
            src.push_str("                __polybench_bench();\n");
        }
        src.push_str("            }\n            var t1 = Stopwatch.GetTimestamp();\n");
        src.push_str(
            "            double elapsedNs = (t1 - t0) * (1_000_000_000.0 / Stopwatch.Frequency);\n",
        );
        src.push_str("            totalNs += elapsedNs;\n            totalIterations += batch;\n");
        src.push_str("            samples.Add(elapsedNs / Math.Max(1, batch));\n");
        src.push_str("            if (elapsedNs > 0) {\n");
        src.push_str(
            "                var remaining = targetNs - totalNs; batch = (int)Math.Max(1, Math.Min(int.MaxValue, (batch * remaining / elapsedNs) * 1.1));\n",
        );
        src.push_str("            } else { batch *= 2; }\n        }\n");
        src.push_str("        double nanosPerOp = totalNs / Math.Max(1, totalIterations);\n");
        src.push_str("        double opsPerSec = 1_000_000_000.0 / Math.Max(1, nanosPerOp);\n");
        if use_memory {
            src.push_str("        long memAfter = GC.GetTotalAllocatedBytes(true);\n");
            src.push_str("        long bytesPerOp = Math.Max(0, (memAfter - memBefore) / Math.Max(1, totalIterations));\n");
        }
        src.push_str("        var result = new Dictionary<string, object?> {\n");
        src.push_str("            [\"iterations\"] = totalIterations,\n");
        src.push_str("            [\"totalNanos\"] = totalNs,\n");
        src.push_str("            [\"warmupNanos\"] = warmupNanos,\n");
        src.push_str("            [\"nanosPerOp\"] = nanosPerOp,\n");
        src.push_str("            [\"opsPerSec\"] = opsPerSec,\n");
        if use_memory {
            src.push_str("            [\"bytesPerOp\"] = bytesPerOp,\n");
        }
        src.push_str("            [\"samples\"] = samples,\n");
        if spec.use_sink {
            src.push_str("            [\"rawResult\"] = __polybench_sink,\n");
        }
        src.push_str("        };\n");
        src.push_str(&CSharpRuntime::emit_hook(spec.after_hooks.get(&Lang::CSharp), "        "));
        src.push_str("        Console.WriteLine(JsonSerializer.Serialize(result));\n");
    } else {
        src.push_str(&format!(
            "        long iterations = {};\n        for (long i = 0; i < iterations; i++) {{\n            var t0 = Stopwatch.GetTimestamp();\n",
            spec.iterations
        ));
        src.push_str(&CSharpRuntime::emit_hook(spec.each_hooks.get(&Lang::CSharp), "            "));
        if spec.use_sink {
            src.push_str("            __polybench_sink = __polybench_bench();\n");
        } else {
            src.push_str("            __polybench_bench();\n");
        }
        src.push_str("            var t1 = Stopwatch.GetTimestamp();\n");
        src.push_str(
            "            samples.Add((t1 - t0) * (1_000_000_000.0 / Stopwatch.Frequency));\n",
        );
        src.push_str("        }\n");
        src.push_str("        double totalNs = 0; foreach (var s in samples) totalNs += s;\n");
        src.push_str("        double nanosPerOp = totalNs / Math.Max(1, iterations);\n");
        src.push_str("        double opsPerSec = 1_000_000_000.0 / Math.Max(1, nanosPerOp);\n");
        if use_memory {
            src.push_str("        long memAfter = GC.GetTotalAllocatedBytes(true);\n");
            src.push_str("        long bytesPerOp = Math.Max(0, (memAfter - memBefore) / Math.Max(1, iterations));\n");
        }
        src.push_str("        var result = new Dictionary<string, object?> {\n");
        src.push_str("            [\"iterations\"] = iterations,\n");
        src.push_str("            [\"totalNanos\"] = totalNs,\n");
        src.push_str("            [\"warmupNanos\"] = warmupNanos,\n");
        src.push_str("            [\"nanosPerOp\"] = nanosPerOp,\n");
        src.push_str("            [\"opsPerSec\"] = opsPerSec,\n");
        if use_memory {
            src.push_str("            [\"bytesPerOp\"] = bytesPerOp,\n");
        }
        src.push_str("            [\"samples\"] = samples,\n");
        if spec.use_sink {
            src.push_str("            [\"rawResult\"] = __polybench_sink,\n");
        }
        src.push_str("        };\n");
        src.push_str(&CSharpRuntime::emit_hook(spec.after_hooks.get(&Lang::CSharp), "        "));
        src.push_str("        Console.WriteLine(JsonSerializer.Serialize(result));\n");
    }
    src.push_str("    }\n}\n");
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
    #[serde(default)]
    bytes_per_op: Option<u64>,
    #[serde(default)]
    allocs_per_op: Option<u64>,
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
        if let Some(bytes) = self.bytes_per_op {
            m = m.with_allocs(bytes, self.allocs_per_op.unwrap_or(0));
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
        .ok_or_else(|| miette!("No benchmark output from C# runtime"))?;
    let result: BenchResultJson = serde_json::from_str(json_line)
        .map_err(|e| miette!("Failed to parse C# benchmark output: {}\n{}", e, stdout))?;
    Ok(result.into_measurement(outlier_detection, cv_threshold))
}
