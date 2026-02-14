//! Rust runtime executor

use crate::{measurement::Measurement, traits::Runtime};
use async_trait::async_trait;
use miette::{miette, Result};
use poly_bench_dsl::{BenchMode, Lang};
use poly_bench_ir::{BenchmarkSpec, SuiteIR};
use poly_bench_stdlib as stdlib;
use std::{collections::HashSet, path::PathBuf};

use super::shared::{
    self, generate_bench_call, generate_fixtures_for_spec, generate_suite_code, SinkMemoryDecls,
    BENCH_RESULT_STRUCT,
};

/// Rust runtime using cargo subprocess
pub struct RustRuntime {
    /// Rust project root directory (where Cargo.toml exists)
    project_root: Option<PathBuf>,
    /// Anvil RPC URL if std::anvil is enabled
    anvil_rpc_url: Option<String>,
}

impl RustRuntime {
    pub fn new() -> Self {
        Self { project_root: None, anvil_rpc_url: None }
    }

    /// Set the Rust project root directory where Cargo.toml is located
    pub fn set_project_root(&mut self, path: Option<PathBuf>) {
        self.project_root = path;
    }

    /// Set the Anvil RPC URL to pass to subprocess
    pub fn set_anvil_rpc_url(&mut self, url: String) {
        self.anvil_rpc_url = Some(url);
    }
}

impl Default for RustRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Runtime for RustRuntime {
    fn name(&self) -> &'static str {
        "Rust Runtime"
    }

    fn lang(&self) -> Lang {
        Lang::Rust
    }

    async fn initialize(&mut self, _suite: &SuiteIR) -> Result<()> {
        // Verify rustc/cargo is available
        which::which("cargo").map_err(|_| miette!("Cargo not found in PATH"))?;
        Ok(())
    }

    async fn run_benchmark(&mut self, spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<Measurement> {
        self.run_via_subprocess(spec, suite).await
    }

    async fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

impl RustRuntime {
    /// Run benchmark via cargo subprocess
    async fn run_via_subprocess(
        &self,
        spec: &BenchmarkSpec,
        suite: &SuiteIR,
    ) -> Result<Measurement> {
        let source = generate_standalone_benchmark(spec, suite)?;

        let (src_path, working_dir) = if let Some(ref project_root) = self.project_root {
            let is_runtime_env =
                project_root.as_os_str().to_string_lossy().contains("runtime-env");
            let src_path = if is_runtime_env {
                project_root.join("src").join("main.rs")
            } else {
                let bench_dir = project_root.join(".polybench").join("rust");
                std::fs::create_dir_all(&bench_dir)
                    .map_err(|e| miette!("Failed to create .polybench/rust directory: {}", e))?;
                bench_dir.join("src").join("main.rs")
            };
            
            // Ensure src directory exists
            if let Some(parent) = src_path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| miette!("Failed to create src directory: {}", e))?;
            }
            
            // Ensure Cargo.toml exists (create minimal one if missing)
            let cargo_path = project_root.join("Cargo.toml");
            if !cargo_path.exists() {
                let cargo_toml = generate_cargo_toml(suite);
                std::fs::write(&cargo_path, cargo_toml)
                    .map_err(|e| miette!("Failed to write Cargo.toml: {}", e))?;
            }
            
            (src_path, project_root.clone())
        } else {
            // Create temp directory for standalone execution
            let temp_dir = std::env::temp_dir().join("polybench-rust");
            std::fs::create_dir_all(&temp_dir)
                .map_err(|e| miette!("Failed to create temp directory: {}", e))?;
            
            let src_dir = temp_dir.join("src");
            std::fs::create_dir_all(&src_dir)
                .map_err(|e| miette!("Failed to create src directory: {}", e))?;
            
            // Create minimal Cargo.toml
            let cargo_toml = generate_minimal_cargo_toml();
            std::fs::write(temp_dir.join("Cargo.toml"), cargo_toml)
                .map_err(|e| miette!("Failed to write Cargo.toml: {}", e))?;
            
            (src_dir.join("main.rs"), temp_dir)
        };

        std::fs::write(&src_path, &source)
            .map_err(|e| miette!("Failed to write benchmark source: {}", e))?;

        let cargo_binary = which::which("cargo").map_err(|_| miette!("Cargo not found in PATH"))?;

        let mut cmd = tokio::process::Command::new(&cargo_binary);
        cmd.args(["run", "--release", "--quiet"]).current_dir(&working_dir);

        if let Some(ref url) = self.anvil_rpc_url {
            cmd.env("ANVIL_RPC_URL", url);
        }

        let output =
            cmd.output().await.map_err(|e| miette!("Failed to run Rust benchmark: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(miette!("Rust benchmark failed:\n{}", stderr));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let result: BenchResultJson = serde_json::from_str(&stdout)
            .map_err(|e| miette!("Failed to parse benchmark result: {}\nOutput: {}", e, stdout))?;

        Ok(result.into_measurement_with_options(spec.outlier_detection, spec.cv_threshold))
    }
}

/// JSON format for benchmark results from Rust
#[derive(Debug, serde::Deserialize)]
struct BenchResultJson {
    iterations: u64,
    total_nanos: u64,
    nanos_per_op: f64,
    ops_per_sec: f64,
    #[serde(default)]
    bytes_per_op: Option<u64>,
    #[serde(default)]
    allocs_per_op: Option<u64>,
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

        if let (Some(bytes), Some(allocs)) = (self.bytes_per_op, self.allocs_per_op) {
            if bytes > 0 || allocs > 0 {
                m = m.with_allocs(bytes, allocs);
            }
        }

        m
    }
}

/// Generate a minimal Cargo.toml for standalone execution
fn generate_minimal_cargo_toml() -> String {
    r#"[package]
name = "polybench-standalone"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
"#
    .to_string()
}

/// Generate Cargo.toml with dependencies extracted from suite imports
fn generate_cargo_toml(suite: &SuiteIR) -> String {
    let mut cargo = String::from(
        r#"[package]
name = "polybench_runner"
version = "0.1.0"
edition = "2021"

# Mark this as a standalone workspace to avoid being included in parent workspaces
[workspace]

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
"#,
    );

    // Extract dependencies from Rust imports
    if let Some(imports) = suite.imports.get(&Lang::Rust) {
        for import in imports {
            // Parse "use crate_name::{...};" or "use crate_name;"
            let import_trimmed = import.trim();
            if import_trimmed.starts_with("use ") {
                let rest = &import_trimmed[4..];
                // Extract the crate name (first identifier before :: or ;)
                let crate_name: String = rest
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '_')
                    .collect();
                
                // Skip std/core/alloc
                if crate_name == "std" || crate_name == "core" || crate_name == "alloc" {
                    continue;
                }
                
                // Add common crate versions (could be extended with a dependency map)
                // Some crates require features - return the full dependency spec
                let dep_spec = match crate_name.as_str() {
                    "sha2" => "sha2 = \"0.10\"",
                    "sha3" => "sha3 = \"0.10\"",
                    "tiny_keccak" => "tiny-keccak = { version = \"2.0\", features = [\"keccak\"] }",
                    "keccak" => "keccak = \"0.1\"",
                    "hex" => "hex = \"0.4\"",
                    "rand" => "rand = \"0.8\"",
                    "tokio" => "tokio = \"1\"",
                    "alloy_primitives" | "alloy" => continue, // Complex deps, skip for now
                    _ => {
                        cargo.push_str(&format!("{} = \"0.1\"\n", crate_name));
                        continue;
                    }
                };
                
                cargo.push_str(&format!("{}\n", dep_spec));
            }
        }
    }

    // Add release profile optimizations for accurate benchmarking
    cargo.push_str(
        r#"
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
"#,
    );

    cargo
}

/// Generate a standalone Rust program for subprocess execution
fn generate_standalone_benchmark(spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<String> {
    let impl_code = spec
        .get_impl(Lang::Rust)
        .ok_or_else(|| miette!("No Rust implementation for benchmark {}", spec.name))?;

    let mut code = String::new();

    // Collect imports
    let stdlib_imports = stdlib::get_stdlib_imports(&suite.stdlib_imports, Lang::Rust);
    let mut all_imports: HashSet<&str> = HashSet::new();
    all_imports.insert("use std::time::Instant;");

    if spec.concurrency > 1 {
        all_imports.insert("use std::thread;");
    }
    if let Some(user_imports) = suite.imports.get(&Lang::Rust) {
        for import_spec in user_imports {
            all_imports.insert(import_spec);
        }
    }
    for import_spec in &stdlib_imports {
        all_imports.insert(import_spec);
    }

    // Generate imports
    let mut sorted_imports: Vec<_> = all_imports.into_iter().collect();
    sorted_imports.sort();
    for import_spec in sorted_imports {
        code.push_str(import_spec);
        if !import_spec.ends_with(';') {
            code.push(';');
        }
        code.push('\n');
    }
    code.push('\n');

    // BenchResult type
    code.push_str(BENCH_RESULT_STRUCT);
    code.push('\n');

    // Inject stdlib code
    let stdlib_code = stdlib::get_stdlib_code(&suite.stdlib_imports, Lang::Rust);
    if !stdlib_code.is_empty() {
        code.push_str(&stdlib_code);
        code.push_str("\n");
    }

    // Suite-level code (declarations, helpers)
    code.push_str(&generate_suite_code(suite, Lang::Rust));

    // Get shared declarations
    let decls = SinkMemoryDecls::from_spec(spec);
    let bench_call = generate_bench_call(impl_code, spec.use_sink);
    let before_hook = spec.before_hooks.get(&Lang::Rust);
    let after_hook = spec.after_hooks.get(&Lang::Rust);
    let each_hook = spec.each_hooks.get(&Lang::Rust);

    // Main function
    code.push_str("fn main() {\n");

    // Fixtures
    code.push_str(&generate_fixtures_for_spec(spec, suite, Lang::Rust));

    // Variable declarations
    code.push_str(decls.sink_decl);
    code.push_str(decls.memory_decl);

    // Before hook
    if let Some(before) = before_hook {
        code.push_str("\n    // Before hook\n");
        for line in before.lines() {
            code.push_str(&format!("    {}\n", line));
        }
    }

    code.push_str(decls.memory_before);

    // Generate based on mode
    if spec.concurrency > 1 {
        // Concurrent execution
        code.push_str(&shared::generate_concurrent_execution(
            &bench_call,
            &bench_call,
            spec.concurrency,
            &spec.iterations.to_string(),
        ));
        
        // Sample collection
        code.push_str(&shared::generate_sample_collection(
            &bench_call,
            decls.sink_keepalive,
            each_hook,
            "100",
            "total_iterations",
        ));
    } else {
        match spec.mode {
            BenchMode::Auto => {
                // Warmup
                code.push_str(&shared::generate_warmup_loop(
                    &bench_call,
                    decls.sink_keepalive,
                    each_hook,
                    "100",
                ));

                // Auto-calibration loop
                code.push_str(&shared::generate_auto_mode_loop(
                    &bench_call,
                    decls.sink_keepalive,
                    each_hook,
                    spec.target_time_ms,
                ));

                // Sample collection
                code.push_str(&shared::generate_sample_collection(
                    &bench_call,
                    decls.sink_keepalive,
                    each_hook,
                    "1000",
                    "total_iterations",
                ));
            }
            BenchMode::Fixed => {
                let iterations = spec.iterations;
                code.push_str(&format!("    let iterations: i64 = {};\n", iterations));
                code.push_str(&format!(
                    "    let mut samples: Vec<u64> = vec![0; {} as usize];\n\n",
                    iterations
                ));

                // Warmup
                code.push_str(&shared::generate_warmup_loop(
                    &bench_call,
                    decls.sink_keepalive,
                    each_hook,
                    &spec.warmup.to_string(),
                ));

                // Fixed measurement loop
                code.push_str(&shared::generate_fixed_mode_loop(
                    &bench_call,
                    decls.sink_keepalive,
                    each_hook,
                    "iterations",
                ));

                // Use iterations as total for result
                code.push_str("    let total_iterations = iterations;\n");
            }
        }
    }

    code.push_str(decls.memory_after);

    // After hook
    if let Some(after) = after_hook {
        code.push_str("\n    // After hook\n");
        for line in after.lines() {
            code.push_str(&format!("    {}\n", line));
        }
    }

    // Result calculation and output
    let memory_result = SinkMemoryDecls::memory_result_fields(spec.memory);
    code.push_str(&shared::generate_result_return("total_iterations", &memory_result));

    code.push_str("}\n");

    Ok(code)
}
