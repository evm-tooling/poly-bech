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
    /// Cached compiled binary path and source hash for reuse across runs
    cached_binary: Option<(PathBuf, u64)>,
}

impl RustRuntime {
    pub fn new() -> Self {
        Self { project_root: None, anvil_rpc_url: None, cached_binary: None }
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

    fn generate_check_source(&self, spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<String> {
        generate_standalone_benchmark(spec, suite)
    }

    async fn compile_check(&self, spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<()> {
        let source = self.generate_check_source(spec, suite)?;

        // Build line mappings for error remapping
        let mappings = crate::build_rust_mappings(suite, &source);

        // Generate Cargo.toml for this suite's dependencies
        let cargo_toml = generate_cargo_toml(suite);

        // Use a persistent directory keyed by Cargo.toml hash to get incremental compilation
        // while keeping different suites (with different dependencies) isolated
        let (work_dir, cleanup) = if let Some(ref project_root) = self.project_root {
            // Hash the Cargo.toml to create a unique directory per dependency set
            use std::{
                collections::hash_map::DefaultHasher,
                hash::{Hash, Hasher},
            };
            let mut hasher = DefaultHasher::new();
            cargo_toml.hash(&mut hasher);
            let cargo_hash = hasher.finish();

            let rust_dir =
                project_root.join(".polybench").join("rust").join(format!("{:016x}", cargo_hash));
            std::fs::create_dir_all(&rust_dir)
                .map_err(|e| miette!("Failed to create .polybench/rust directory: {}", e))?;
            (rust_dir, false)
        } else {
            // Fall back to temp directory with unique name
            let safe_name = spec.full_name.replace('.', "_").replace('/', "_");
            let check_id = format!(
                "{:x}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos() %
                    0xFFFFFFFF
            );
            let temp_dir = std::env::temp_dir()
                .join(format!("polybench-rust-check-{}-{}", safe_name, check_id));
            std::fs::create_dir_all(&temp_dir)
                .map_err(|e| miette!("Failed to create temp directory: {}", e))?;
            (temp_dir, true)
        };

        let src_dir = work_dir.join("src");
        std::fs::create_dir_all(&src_dir)
            .map_err(|e| miette!("Failed to create src directory: {}", e))?;

        // Write Cargo.toml (only if changed to preserve Cargo.lock)
        let cargo_path = work_dir.join("Cargo.toml");
        let should_write_cargo = if cargo_path.exists() {
            let existing = std::fs::read_to_string(&cargo_path).unwrap_or_default();
            existing != cargo_toml
        } else {
            true
        };
        if should_write_cargo {
            std::fs::write(&cargo_path, &cargo_toml)
                .map_err(|e| miette!("Failed to write Cargo.toml: {}", e))?;
        }

        // Write source as main.rs
        let main_path = src_dir.join("main.rs");
        std::fs::write(&main_path, &source)
            .map_err(|e| miette!("Failed to write main.rs: {}", e))?;

        let cargo_binary = which::which("cargo").map_err(|_| miette!("Cargo not found in PATH"))?;

        // Use 'cargo check' for fast compilation checking without codegen
        let output = tokio::process::Command::new(&cargo_binary)
            .args(["check", "--release", "--quiet"])
            .current_dir(&work_dir)
            .output()
            .await
            .map_err(|e| miette!("Failed to compile Rust benchmark: {}", e))?;

        // Clean up temp directory only (keep persistent directory for incremental builds)
        if cleanup {
            let _ = std::fs::remove_dir_all(&work_dir);
        }

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            // Remap error line numbers to .bench file locations
            let remapped = crate::remap_rust_error(&stderr, &mappings);
            return Err(miette!("Rust compilation failed:\n{}", remapped));
        }

        Ok(())
    }

    async fn run_benchmark(
        &mut self,
        spec: &BenchmarkSpec,
        suite: &SuiteIR,
    ) -> Result<Measurement> {
        self.run_via_subprocess(spec, suite).await
    }

    async fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

impl RustRuntime {
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

    /// Pre-compile the benchmark binary without running it.
    /// This allows compilation to happen before timing starts.
    pub async fn precompile(&mut self, spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<()> {
        let source = generate_standalone_benchmark(spec, suite)?;
        let source_hash = Self::hash_source(&source);

        // Check if we already have a cached binary with matching source hash
        if let Some((ref binary_path, cached_hash)) = self.cached_binary {
            if cached_hash == source_hash && binary_path.exists() {
                // Already compiled, nothing to do
                return Ok(());
            }
        }

        // Need to compile - set up directories and source
        let (src_path, working_dir) = if let Some(ref project_root) = self.project_root {
            let is_runtime_env = project_root.as_os_str().to_string_lossy().contains("runtime-env");
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
            let cargo_path = if is_runtime_env {
                project_root.join("Cargo.toml")
            } else {
                project_root.join(".polybench").join("rust").join("Cargo.toml")
            };
            if !cargo_path.exists() {
                let cargo_toml = generate_cargo_toml(suite);
                std::fs::write(&cargo_path, cargo_toml)
                    .map_err(|e| miette!("Failed to write Cargo.toml: {}", e))?;
            }

            let working = if is_runtime_env {
                project_root.clone()
            } else {
                project_root.join(".polybench").join("rust")
            };
            (src_path, working)
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

        // Build the binary
        let build_output = tokio::process::Command::new(&cargo_binary)
            .args(["build", "--release", "--quiet"])
            .current_dir(&working_dir)
            .output()
            .await
            .map_err(|e| miette!("Failed to build Rust benchmark: {}", e))?;

        if !build_output.status.success() {
            let stderr = String::from_utf8_lossy(&build_output.stderr);
            return Err(miette!("Rust benchmark build failed:\n{}", stderr));
        }

        // Binary name matches the package name in Cargo.toml
        let binary_name = "polybench_runner";
        let binary_path = working_dir.join("target").join("release").join(binary_name);

        if !binary_path.exists() {
            return Err(miette!(
                "Compiled binary not found at expected path: {}",
                binary_path.display()
            ));
        }

        // Cache the binary path and source hash for reuse
        self.cached_binary = Some((binary_path, source_hash));

        Ok(())
    }

    /// Run benchmark via cargo subprocess
    /// Compiles once and reuses the binary for subsequent runs with the same source
    async fn run_via_subprocess(
        &mut self,
        spec: &BenchmarkSpec,
        suite: &SuiteIR,
    ) -> Result<Measurement> {
        let source = generate_standalone_benchmark(spec, suite)?;
        let source_hash = Self::hash_source(&source);

        // Check if we have a cached binary with matching source hash
        if let Some((ref binary_path, cached_hash)) = self.cached_binary {
            if cached_hash == source_hash && binary_path.exists() {
                // Reuse cached binary - skip compilation
                return self.run_binary(binary_path, spec).await;
            }
        }

        // Need to compile - set up directories and source
        let (src_path, working_dir) = if let Some(ref project_root) = self.project_root {
            let is_runtime_env = project_root.as_os_str().to_string_lossy().contains("runtime-env");
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
            let cargo_path = if is_runtime_env {
                project_root.join("Cargo.toml")
            } else {
                project_root.join(".polybench").join("rust").join("Cargo.toml")
            };
            if !cargo_path.exists() {
                let cargo_toml = generate_cargo_toml(suite);
                std::fs::write(&cargo_path, cargo_toml)
                    .map_err(|e| miette!("Failed to write Cargo.toml: {}", e))?;
            }

            let working = if is_runtime_env {
                project_root.clone()
            } else {
                project_root.join(".polybench").join("rust")
            };
            (src_path, working)
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

        // Build the binary (separate from running)
        let build_output = tokio::process::Command::new(&cargo_binary)
            .args(["build", "--release", "--quiet"])
            .current_dir(&working_dir)
            .output()
            .await
            .map_err(|e| miette!("Failed to build Rust benchmark: {}", e))?;

        if !build_output.status.success() {
            let stderr = String::from_utf8_lossy(&build_output.stderr);
            return Err(miette!("Rust benchmark build failed:\n{}", stderr));
        }

        // Binary name matches the package name in Cargo.toml
        let binary_name = "polybench_runner";
        let binary_path = working_dir.join("target").join("release").join(binary_name);

        if !binary_path.exists() {
            return Err(miette!(
                "Compiled binary not found at expected path: {}",
                binary_path.display()
            ));
        }

        // Cache the binary path and source hash for reuse
        self.cached_binary = Some((binary_path.clone(), source_hash));

        // Run the binary
        self.run_binary(&binary_path, spec).await
    }

    /// Run a pre-compiled binary and parse the result
    async fn run_binary(&self, binary_path: &PathBuf, spec: &BenchmarkSpec) -> Result<Measurement> {
        let mut cmd = tokio::process::Command::new(binary_path);

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
    #[serde(default)]
    raw_result: Option<String>,
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
        m.raw_result = self.raw_result;

        m
    }
}

/// Generate a minimal Cargo.toml for standalone execution
fn generate_minimal_cargo_toml() -> String {
    r#"[package]
name = "polybench_runner"
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
                let crate_name: String =
                    rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();

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
                    "regex" => "regex = \"1\"",
                    "once_cell" => "once_cell = \"1\"",
                    "lazy_static" => "lazy_static = \"1\"",
                    "itertools" => "itertools = \"0.12\"",
                    "num" => "num = \"0.4\"",
                    "num_traits" => "num-traits = \"0.2\"",
                    "byteorder" => "byteorder = \"1\"",
                    "base64" => "base64 = \"0.21\"",
                    "chrono" => "chrono = \"0.4\"",
                    "uuid" => "uuid = \"1\"",
                    "alloy_primitives" | "alloy" => continue, // Complex deps, skip for now
                    _ => {
                        // Default to version 1.0 for unknown crates (safer than 0.1)
                        cargo.push_str(&format!("{} = \"*\"\n", crate_name));
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
    match spec.mode {
        BenchMode::Auto => {
            // Warmup
            code.push_str(&shared::generate_warmup_loop(
                &bench_call,
                decls.sink_keepalive,
                each_hook,
                &spec.warmup.to_string(),
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
