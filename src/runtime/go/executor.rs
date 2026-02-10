//! Go runtime executor

use crate::dsl::Lang;
use crate::ir::{BenchmarkSpec, SuiteIR};
use crate::runtime::go::{codegen, compiler::GoCompiler};
use crate::runtime::measurement::Measurement;
use crate::runtime::traits::Runtime;
use async_trait::async_trait;
use libloading::{Library, Symbol};
use miette::{Result, miette};
use std::path::PathBuf;

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
}

impl GoRuntime {
    pub fn new() -> Self {
        Self {
            library: None,
            plugin_path: None,
            compiler: None,
            module_root: None,
        }
    }

    /// Set the Go module root directory where go.mod is located
    pub fn set_module_root(&mut self, path: Option<PathBuf>) {
        self.module_root = path;
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

    async fn initialize(&mut self, suite: &SuiteIR) -> Result<()> {
        // Go plugins only work on Linux, so we'll use subprocess execution on all platforms
        // for consistency and to avoid platform-specific issues.
        // 
        // The plugin approach is kept in the code but not used by default.
        // To enable plugins (Linux only), change the logic below.
        
        // Create a compiler for subprocess execution
        let compiler = GoCompiler::new()?;
        self.compiler = Some(compiler);
        
        Ok(())
    }

    async fn run_benchmark(&mut self, spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<Measurement> {
        // If we have a loaded library, try to use it
        if let Some(ref lib) = self.library {
            match self.run_via_plugin(lib, spec) {
                Ok(m) => return Ok(m),
                Err(e) => {
                    // Plugin failed, fall back to subprocess
                    eprintln!("Plugin execution failed: {}. Using subprocess.", e);
                }
            }
        }
        
        // Fall back to subprocess execution
        self.run_via_subprocess(spec, suite).await
    }

    async fn shutdown(&mut self) -> Result<()> {
        // Drop the library to unload it
        self.library = None;
        self.compiler = None;
        Ok(())
    }
}

impl GoRuntime {
    /// Run benchmark via loaded plugin
    fn run_via_plugin(&self, lib: &Library, spec: &BenchmarkSpec) -> Result<Measurement> {
        unsafe {
            // Get the RunBenchmark function
            let run_benchmark: Symbol<fn(&str, i32) -> String> = lib
                .get(b"RunBenchmark")
                .map_err(|e| miette!("Failed to get RunBenchmark symbol: {}", e))?;
            
            // Call the function
            let result_json = run_benchmark(&spec.full_name, spec.iterations as i32);
            
            // Parse the result
            let result: BenchResultJson = serde_json::from_str(&result_json)
                .map_err(|e| miette!("Failed to parse benchmark result: {}", e))?;
            
            Ok(result.into_measurement())
        }
    }

    /// Run benchmark via subprocess (fallback for unsupported platforms)
    async fn run_via_subprocess(&self, spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<Measurement> {
        // Generate a standalone Go program that runs the benchmark
        let source = generate_standalone_benchmark(spec, suite)?;
        
        // Determine where to write and run the benchmark
        let (src_path, working_dir) = if let Some(ref module_root) = self.module_root {
            // Use the module root directory - create a .polybench subdirectory
            let bench_dir = module_root.join(".polybench");
            std::fs::create_dir_all(&bench_dir)
                .map_err(|e| miette!("Failed to create .polybench directory: {}", e))?;
            
            let src_path = bench_dir.join("bench_standalone.go");
            (src_path, module_root.clone())
        } else {
            // Fall back to temp directory
            let compiler = self.compiler.as_ref()
                .ok_or_else(|| miette!("Compiler not initialized"))?;
            
            let src_path = compiler.temp_path().join("bench_standalone.go");
            (src_path, compiler.temp_path().to_path_buf())
        };
        
        std::fs::write(&src_path, &source)
            .map_err(|e| miette!("Failed to write benchmark source: {}", e))?;
        
        // Compile and run from the working directory (which has go.mod)
        let go_binary = which::which("go")
            .map_err(|_| miette!("Go not found in PATH"))?;
        
        let output = tokio::process::Command::new(&go_binary)
            .args(["run", src_path.to_str().unwrap()])
            .current_dir(&working_dir)
            .output()
            .await
            .map_err(|e| miette!("Failed to run Go benchmark: {}", e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(miette!("Go benchmark failed:\n{}", stderr));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let result: BenchResultJson = serde_json::from_str(&stdout)
            .map_err(|e| miette!("Failed to parse benchmark result: {}\nOutput: {}", e, stdout))?;
        
        Ok(result.into_measurement())
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
    fn into_measurement(self) -> Measurement {
        let mut m = if self.samples.is_empty() {
            Measurement::from_aggregate(self.iterations, self.total_nanos)
        } else {
            Measurement::from_samples(self.samples, self.iterations)
        };
        
        if self.bytes_per_op > 0 || self.allocs_per_op > 0 {
            m = m.with_allocs(self.bytes_per_op, self.allocs_per_op);
        }
        
        m
    }
}

/// Generate a standalone Go program for subprocess execution
fn generate_standalone_benchmark(spec: &BenchmarkSpec, suite: &SuiteIR) -> Result<String> {
    let impl_code = spec.get_impl(Lang::Go)
        .ok_or_else(|| miette!("No Go implementation for benchmark {}", spec.name))?;
    
    let mut code = String::new();
    
    // Start with package
    code.push_str("package main\n\n");

    // Emit unified import block: standard libs + user imports (pre-extracted at IR lowering)
    code.push_str("import (\n");
    code.push_str("\t\"encoding/json\"\n");
    code.push_str("\t\"fmt\"\n");
    code.push_str("\t\"time\"\n");
    
    // Add user imports from the pre-extracted imports in SuiteIR
    if let Some(user_imports) = suite.imports.get(&Lang::Go) {
        for import_spec in user_imports {
            // import_spec is already formatted (e.g., "pkg" or alias "pkg")
            code.push_str(&format!("\t{}\n", import_spec));
        }
    }
    code.push_str(")\n\n");

    // BenchResult type
    code.push_str(
        "type BenchResult struct {\n\
\tIterations  uint64   `json:\"iterations\"`\n\
\tTotalNanos  uint64   `json:\"total_nanos\"`\n\
\tNanosPerOp  float64  `json:\"nanos_per_op\"`\n\
\tOpsPerSec   float64  `json:\"ops_per_sec\"`\n\
\tBytesPerOp  uint64   `json:\"bytes_per_op\"`\n\
\tAllocsPerOp uint64   `json:\"allocs_per_op\"`\n\
\tSamples     []uint64 `json:\"samples\"`\n\
}\n\n",
    );

    // Add non-import setup body (vars, funcs, etc.)
    // The body has already had imports extracted at IR lowering time
    if let Some(setup_body) = suite.setups.get(&Lang::Go) {
        if !setup_body.trim().is_empty() {
            code.push_str(setup_body);
            if !setup_body.ends_with('\n') {
                code.push('\n');
            }
            code.push('\n');
        }
    }

    // Add fixtures
    for fixture_name in &spec.fixture_refs {
        if let Some(fixture) = suite.get_fixture(fixture_name) {
            if let Some(impl_code) = fixture.implementations.get(&Lang::Go) {
                // Wrap in IIFE if it contains multiple statements (has return)
                if impl_code.contains("return") {
                    code.push_str(&format!("var {} = func() []byte {{\n{}\n}}()\n", fixture_name, impl_code));
                } else {
                    code.push_str(&format!("var {} = {}\n", fixture_name, impl_code));
                }
            } else if !fixture.data.is_empty() {
                code.push_str(&format!("var {} = {}\n", fixture_name, fixture.as_go_bytes()));
            }
        }
    }

    code.push_str(&format!(r#"
func main() {{
	iterations := {}
	warmup := {}
	samples := make([]uint64, iterations)
	
	// Warmup
	for i := 0; i < warmup; i++ {{
		_ = {}
	}}
	
	// Timed run
	var totalNanos uint64
	for i := 0; i < iterations; i++ {{
		start := time.Now()
		_ = {}
		elapsed := time.Since(start).Nanoseconds()
		samples[i] = uint64(elapsed)
		totalNanos += uint64(elapsed)
	}}
	
	nanosPerOp := float64(totalNanos) / float64(iterations)
	opsPerSec := 1e9 / nanosPerOp
	
	result := BenchResult{{
		Iterations:  uint64(iterations),
		TotalNanos:  totalNanos,
		NanosPerOp:  nanosPerOp,
		OpsPerSec:   opsPerSec,
		Samples:     samples,
	}}
	
	jsonBytes, _ := json.Marshal(result)
	fmt.Println(string(jsonBytes))
}}
"#, spec.iterations, spec.warmup, impl_code, impl_code));

    Ok(code)
}

