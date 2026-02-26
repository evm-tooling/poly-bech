//! C runtime executor.

use async_trait::async_trait;
use miette::{miette, Result};
use poly_bench_dsl::{BenchMode, Lang};
use poly_bench_ir::{BenchmarkSpec, SuiteIR};
use poly_bench_runtime_traits::{Measurement, Runtime, RuntimeConfig, RuntimeFactory};
use poly_bench_stdlib as stdlib;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    process::Stdio,
};

pub struct CRuntime {
    clang_binary: PathBuf,
    project_root: Option<PathBuf>,
    anvil_rpc_url: Option<String>,
    cached_binary: Option<(PathBuf, u64)>,
}

impl CRuntime {
    pub fn new() -> Result<Self> {
        let clang_binary = which::which("clang").map_err(|_| miette!("clang not found in PATH"))?;
        Ok(Self { clang_binary, project_root: None, anvil_rpc_url: None, cached_binary: None })
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
            let dir = root.join(".polybench").join("c");
            std::fs::create_dir_all(&dir)
                .map_err(|e| miette!("Failed to create .polybench/c directory: {}", e))?;
            return Ok(dir);
        }
        let dir = std::env::temp_dir().join("polybench-c");
        std::fs::create_dir_all(&dir)
            .map_err(|e| miette!("Failed to create temp C directory: {}", e))?;
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
        let output = std::process::Command::new(&self.clang_binary)
            .args([
                "-O3",
                "-std=c11",
                source_path.to_string_lossy().as_ref(),
                "-o",
                binary_path.to_string_lossy().as_ref(),
            ])
            .current_dir(work_dir)
            .output()
            .map_err(|e| miette!("Failed to run clang: {}", e))?;

        if !output.status.success() {
            return Err(miette!(
                "C compilation failed:\n{}\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        if !binary_path.exists() {
            return Err(miette!(
                "C build succeeded but binary not found at {}",
                binary_path.display()
            ));
        }

        Ok(binary_path)
    }
}

impl Default for CRuntime {
    fn default() -> Self {
        Self::new().expect("Failed to create C runtime")
    }
}

pub struct CRuntimeFactory;
pub static C_RUNTIME_FACTORY: CRuntimeFactory = CRuntimeFactory;

impl RuntimeFactory for CRuntimeFactory {
    fn lang(&self) -> Lang {
        Lang::C
    }

    fn name(&self) -> &'static str {
        "C Runtime"
    }

    fn create(&self, config: &RuntimeConfig) -> Result<Box<dyn Runtime>> {
        let mut rt = CRuntime::new()?;
        rt.set_project_root(config.get_root(poly_bench_dsl::Lang::C));
        Ok(Box::new(rt))
    }
}

#[async_trait]
impl Runtime for CRuntime {
    fn name(&self) -> &'static str {
        "C Runtime"
    }

    fn lang(&self) -> Lang {
        Lang::C
    }

    fn set_anvil_rpc_url(&mut self, url: String) {
        self.anvil_rpc_url = Some(url);
    }

    async fn initialize(&mut self, _suite: &SuiteIR) -> Result<()> {
        which::which("clang").map_err(|_| miette!("clang not found in PATH"))?;
        Ok(())
    }

    fn generate_check_source(&self, spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<String> {
        generate_c_source(spec, suite, true)
    }

    async fn compile_check(&self, spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<()> {
        let source = self.generate_check_source(spec, suite)?;
        let work_dir = self.resolve_work_dir()?;
        let safe_name = spec.full_name.replace('.', "_").replace('/', "_");
        let source_name = format!("bench_check_{}.c", safe_name);
        self.write_source_and_build(&work_dir, &source, &source_name, "polybench_check")?;
        Ok(())
    }

    async fn precompile(&mut self, spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<()> {
        let source = generate_c_source(spec, suite, false)?;
        let source_hash = Self::hash_source(&source);

        if let Some((ref binary_path, cached_hash)) = self.cached_binary {
            if cached_hash == source_hash && binary_path.exists() {
                return Ok(());
            }
        }

        let work_dir = self.resolve_work_dir()?;
        let binary_path = self.write_source_and_build(
            &work_dir,
            &source,
            "bench_standalone.c",
            "polybench_runner",
        )?;
        self.cached_binary = Some((binary_path, source_hash));
        Ok(())
    }

    async fn run_benchmark(
        &mut self,
        spec: &BenchmarkSpec,
        suite: &SuiteIR,
    ) -> Result<Measurement> {
        let source = generate_c_source(spec, suite, false)?;
        let source_hash = Self::hash_source(&source);
        let work_dir = self.resolve_work_dir()?;

        let binary_path = if let Some((ref binary_path, cached_hash)) = self.cached_binary {
            if cached_hash == source_hash && binary_path.exists() {
                binary_path.clone()
            } else {
                let built = self.write_source_and_build(
                    &work_dir,
                    &source,
                    "bench_standalone.c",
                    "polybench_runner",
                )?;
                self.cached_binary = Some((built.clone(), source_hash));
                built
            }
        } else {
            let built = self.write_source_and_build(
                &work_dir,
                &source,
                "bench_standalone.c",
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

        let child = cmd.spawn().map_err(|e| miette!("Failed to run C benchmark: {}", e))?;
        let output = if let Some(timeout_ms) = spec.timeout {
            match tokio::time::timeout(
                tokio::time::Duration::from_millis(timeout_ms),
                child.wait_with_output(),
            )
            .await
            {
                Ok(r) => r.map_err(|e| miette!("Failed waiting for C benchmark: {}", e))?,
                Err(_) => return Err(miette!("C benchmark timed out after {}ms", timeout_ms)),
            }
        } else {
            child
                .wait_with_output()
                .await
                .map_err(|e| miette!("Failed waiting for C benchmark: {}", e))?
        };

        if !output.status.success() {
            return Err(miette!("C benchmark failed:\n{}", String::from_utf8_lossy(&output.stderr)));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        parse_benchmark_result(&stdout, spec.outlier_detection, spec.cv_threshold)
    }

    async fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

fn emit_fixtures(spec: &BenchmarkSpec, suite: &SuiteIR, indent: &str) -> String {
    let mut out = String::new();
    for fixture_name in &spec.fixture_refs {
        if let Some(fixture) = suite.get_fixture(fixture_name) {
            if let Some(fixture_impl) = fixture.implementations.get(&Lang::C) {
                out.push_str(&format!("{}void* {} = {};\n", indent, fixture_name, fixture_impl));
            } else if !fixture.data.is_empty() {
                let bytes: Vec<String> =
                    fixture.data.iter().map(|b| format!("0x{:02x}", b)).collect();
                out.push_str(&format!(
                    "{}unsigned char {}[] = {{ {} }};\n",
                    indent,
                    fixture_name,
                    bytes.join(", ")
                ));
            }
        }
    }
    out
}

fn emit_hook(code: Option<&String>, indent: &str) -> String {
    code.map(|c| {
        CRuntime::normalize_indent(c)
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

fn generate_c_source(spec: &BenchmarkSpec, suite: &SuiteIR, check_only: bool) -> Result<String> {
    let impl_code = spec
        .get_impl(Lang::C)
        .ok_or_else(|| miette!("No C implementation for benchmark {}", spec.name))?;
    let impl_code = CRuntime::normalize_indent(impl_code.trim());

    let mut src = String::new();
    src.push_str("#include <stdio.h>\n#include <stdint.h>\n#include <time.h>\n#include <math.h>\n");
    src.push_str("#include <stdlib.h>\n#include <string.h>\n\n");

    let mut include_set: HashSet<String> = HashSet::new();
    if let Some(imports) = suite.imports.get(&Lang::C) {
        for import_stmt in imports {
            include_set.insert(import_stmt.trim().to_string());
        }
    }
    let stdlib_imports = stdlib::get_stdlib_imports(&suite.stdlib_imports, &crate::C_STDLIB);
    for import_stmt in stdlib_imports {
        include_set.insert(import_stmt.trim().to_string());
    }
    let mut includes: Vec<_> = include_set.into_iter().collect();
    includes.sort();
    for include in includes {
        src.push_str(&include);
        src.push('\n');
    }
    if !src.ends_with("\n\n") {
        src.push('\n');
    }

    src.push_str("static void* __polybench_sink;\n\n");
    src.push_str(
        "static uint64_t __polybench_nanos_now(void) {\n    struct timespec ts;\n    clock_gettime(CLOCK_MONOTONIC, &ts);\n    return (uint64_t)ts.tv_sec * 1000000000ULL + (uint64_t)ts.tv_nsec;\n}\n\n",
    );

    let stdlib_code = stdlib::get_stdlib_code(&suite.stdlib_imports, &crate::C_STDLIB);
    if !stdlib_code.is_empty() {
        src.push_str(&stdlib_code);
        src.push_str("\n\n");
    }

    if let Some(declarations) = suite.declarations.get(&Lang::C) {
        if !declarations.trim().is_empty() {
            src.push_str(&CRuntime::normalize_indent(declarations));
            src.push_str("\n\n");
        }
    }
    if let Some(helpers) = suite.helpers.get(&Lang::C) {
        if !helpers.trim().is_empty() {
            src.push_str(&CRuntime::normalize_indent(helpers));
            src.push_str("\n\n");
        }
    }
    if let Some(init_code) = suite.init_code.get(&Lang::C) {
        if !init_code.trim().is_empty() {
            src.push_str("static void __polybench_init(void) {\n");
            for line in CRuntime::normalize_indent(init_code).lines() {
                src.push_str("    ");
                src.push_str(line);
                src.push('\n');
            }
            src.push_str("}\n\n");
        }
    }

    src.push_str("static void* __polybench_bench(void) {\n");
    src.push_str(&emit_fixtures(spec, suite, "    "));
    for line in impl_code.lines() {
        src.push_str("    ");
        src.push_str(line);
        if !line.trim_end().ends_with(';') && !line.trim_end().ends_with('}') {
            src.push(';');
        }
        src.push('\n');
    }
    src.push_str("    return NULL;\n");
    src.push_str("}\n\n");

    if check_only {
        src.push_str("int main(void) { return 0; }\n");
        return Ok(src);
    }

    src.push_str("int main(void) {\n");
    if suite.init_code.get(&Lang::C).is_some() {
        src.push_str("    __polybench_init();\n");
    }
    src.push_str(&emit_hook(spec.before_hooks.get(&Lang::C), "    "));

    src.push_str(&format!("    for (uint64_t i = 0; i < {}; i++) {{\n", spec.warmup));
    src.push_str(&emit_hook(spec.each_hooks.get(&Lang::C), "        "));
    if spec.use_sink {
        src.push_str("        __polybench_sink = __polybench_bench();\n");
    } else {
        src.push_str("        __polybench_bench();\n");
    }
    src.push_str("    }\n\n");

    if spec.mode == BenchMode::Auto {
        src.push_str(&format!(
            "    double targetNs = {:.0};\n    uint64_t totalIterations = 0;\n    double totalNs = 0.0;\n    uint64_t batch = 1;\n    size_t sampleCap = 4096;\n    size_t sampleCount = 0;\n    double* samples = (double*)malloc(sampleCap * sizeof(double));\n    if (!samples) return 1;\n    while (totalNs < targetNs) {{\n        uint64_t t0 = __polybench_nanos_now();\n        for (uint64_t i = 0; i < batch; i++) {{\n",
            (spec.target_time_ms as f64) * 1_000_000.0
        ));
        src.push_str(&emit_hook(spec.each_hooks.get(&Lang::C), "            "));
        if spec.use_sink {
            src.push_str("            __polybench_sink = __polybench_bench();\n");
        } else {
            src.push_str("            __polybench_bench();\n");
        }
        src.push_str("        }\n        uint64_t t1 = __polybench_nanos_now();\n");
        src.push_str("        double elapsedNs = (double)(t1 - t0);\n");
        src.push_str("        totalNs += elapsedNs;\n        totalIterations += batch;\n");
        src.push_str("        if (sampleCount == sampleCap) {\n");
        src.push_str("            sampleCap *= 2;\n            double* next = (double*)realloc(samples, sampleCap * sizeof(double));\n");
        src.push_str("            if (!next) { free(samples); return 1; }\n            samples = next;\n        }\n");
        src.push_str("        samples[sampleCount++] = elapsedNs / (double)(batch ? batch : 1);\n");
        src.push_str("        if (elapsedNs > 0.0) {\n");
        src.push_str(
            "            double remaining = targetNs - totalNs;\n            uint64_t nextBatch = (uint64_t)fmax(1.0, (batch * remaining / elapsedNs) * 1.1);\n            batch = nextBatch;\n",
        );
        src.push_str("        } else {\n            batch *= 2;\n        }\n    }\n");
        src.push_str(
            "    double nanosPerOp = totalNs / (double)(totalIterations ? totalIterations : 1);\n    double opsPerSec = 1000000000.0 / (nanosPerOp > 0.0 ? nanosPerOp : 1.0);\n",
        );
        src.push_str(&emit_hook(spec.after_hooks.get(&Lang::C), "    "));
        src.push_str("    printf(\"{\\\"iterations\\\":%llu,\\\"totalNanos\\\":%.0f,\\\"nanosPerOp\\\":%.6f,\\\"opsPerSec\\\":%.6f,\\\"samples\\\":[\", (unsigned long long)totalIterations, totalNs, nanosPerOp, opsPerSec);\n");
        src.push_str("    for (size_t i = 0; i < sampleCount; i++) {\n");
        src.push_str(
            "        if (i) printf(\",\");\n        printf(\"%.0f\", samples[i]);\n    }\n",
        );
        src.push_str("    if (__polybench_sink) {\n        printf(\"],\\\"rawResult\\\":\\\"sink\\\"}\");\n    } else {\n        printf(\"]}\");\n    }\n");
        src.push_str("    printf(\"\\n\");\n    free(samples);\n");
    } else {
        src.push_str(&format!(
            "    uint64_t iterations = {};\n    double* samples = (double*)malloc((size_t)iterations * sizeof(double));\n    if (!samples) return 1;\n    for (uint64_t i = 0; i < iterations; i++) {{\n        uint64_t t0 = __polybench_nanos_now();\n",
            spec.iterations
        ));
        src.push_str(&emit_hook(spec.each_hooks.get(&Lang::C), "        "));
        if spec.use_sink {
            src.push_str("        __polybench_sink = __polybench_bench();\n");
        } else {
            src.push_str("        __polybench_bench();\n");
        }
        src.push_str("        uint64_t t1 = __polybench_nanos_now();\n");
        src.push_str("        samples[i] = (double)(t1 - t0);\n    }\n");
        src.push_str(
            "    double totalNs = 0.0;\n    for (uint64_t i = 0; i < iterations; i++) totalNs += samples[i];\n    double nanosPerOp = totalNs / (double)(iterations ? iterations : 1);\n    double opsPerSec = 1000000000.0 / (nanosPerOp > 0.0 ? nanosPerOp : 1.0);\n",
        );
        src.push_str(&emit_hook(spec.after_hooks.get(&Lang::C), "    "));
        src.push_str("    printf(\"{\\\"iterations\\\":%llu,\\\"totalNanos\\\":%.0f,\\\"nanosPerOp\\\":%.6f,\\\"opsPerSec\\\":%.6f,\\\"samples\\\":[\", (unsigned long long)iterations, totalNs, nanosPerOp, opsPerSec);\n");
        src.push_str("    for (uint64_t i = 0; i < iterations; i++) {\n");
        src.push_str(
            "        if (i) printf(\",\");\n        printf(\"%.0f\", samples[i]);\n    }\n",
        );
        src.push_str("    if (__polybench_sink) {\n        printf(\"],\\\"rawResult\\\":\\\"sink\\\"}\");\n    } else {\n        printf(\"]}\");\n    }\n");
        src.push_str("    printf(\"\\n\");\n    free(samples);\n");
    }

    src.push_str("    return 0;\n}\n");
    Ok(src)
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct BenchResultJson {
    iterations: u64,
    total_nanos: f64,
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
            Measurement::from_samples_with_options(
                sample_u64,
                self.iterations,
                outlier_detection,
                cv_threshold,
            )
        };
        if let Some(raw) = self.raw_result {
            m.raw_result = Some(raw);
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
        .ok_or_else(|| miette!("No benchmark output from C runtime"))?;
    let result: BenchResultJson = serde_json::from_str(json_line)
        .map_err(|e| miette!("Failed to parse C benchmark output: {}\n{}", e, stdout))?;
    Ok(result.into_measurement(outlier_detection, cv_threshold))
}
