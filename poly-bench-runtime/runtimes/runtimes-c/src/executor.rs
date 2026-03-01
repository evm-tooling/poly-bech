//! C runtime executor.

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

pub struct CRuntime {
    clang_binary: PathBuf,
    project_root: Option<PathBuf>,
    anvil_rpc_url: Option<String>,
    cached_binary: Option<(PathBuf, u64)>,
    last_precompile_nanos: Option<u64>,
}

struct CCmakeDep {
    find_package: String,
    components: Vec<String>,
    targets: Vec<String>,
}

impl CRuntime {
    pub fn new() -> Result<Self> {
        let clang_binary = which::which("clang").map_err(|_| miette!("clang not found in PATH"))?;
        Ok(Self {
            clang_binary,
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
            // Canonical poly-bench runtime location for C projects.
            let dir = root.join(".polybench").join("runtime-env").join("c");
            std::fs::create_dir_all(&dir).map_err(|e| {
                miette!("Failed to create .polybench/runtime-env/c directory: {}", e)
            })?;
            return Ok(dir);
        }
        let dir = std::env::temp_dir().join("polybench-c");
        std::fs::create_dir_all(&dir)
            .map_err(|e| miette!("Failed to create temp C directory: {}", e))?;
        Ok(dir)
    }

    /// Derive project root from work_dir (e.g. .polybench/runtime-env/c -> project root)
    fn resolve_project_root(&self, work_dir: &Path) -> Option<PathBuf> {
        let work_str = work_dir.to_string_lossy();
        if work_str.contains("runtime-env") {
            // .../project/.polybench/runtime-env/c -> .../project
            work_dir
                .parent()
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
                .map(|p| p.to_path_buf())
        } else if work_str.contains(".polybench") {
            // .../project/.polybench/c -> .../project
            work_dir.parent().and_then(|p| p.parent()).map(|p| p.to_path_buf())
        } else {
            None
        }
    }

    /// Load [c] section from polybench.toml (standard and dependencies)
    fn load_c_config(
        &self,
        project_root: &Path,
    ) -> Result<(String, std::collections::HashMap<String, String>)> {
        let manifest_path = project_root.join("polybench.toml");
        let content = std::fs::read_to_string(&manifest_path)
            .map_err(|e| miette!("Failed to read {}: {}", manifest_path.display(), e))?;
        let full: toml::Value =
            content.parse().map_err(|e| miette!("Invalid polybench.toml: {}", e))?;
        let c_table = full.get("c").and_then(|v| v.as_table());
        let standard = c_table
            .and_then(|t| t.get("standard"))
            .and_then(|v| v.as_str())
            .unwrap_or("c11")
            .to_string();
        let deps = c_table
            .and_then(|t| t.get("dependencies"))
            .and_then(|v| v.as_table())
            .map(|t| {
                t.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();
        Ok((standard, deps))
    }

    /// Fallback deps source for runtime-env/c: parse package names from vcpkg.json.
    fn load_deps_from_vcpkg_manifest(
        &self,
        work_dir: &Path,
    ) -> Result<std::collections::HashMap<String, String>> {
        let vcpkg_path = work_dir.join("vcpkg.json");
        if !vcpkg_path.exists() {
            return Ok(std::collections::HashMap::new());
        }
        let content = std::fs::read_to_string(&vcpkg_path)
            .map_err(|e| miette!("Failed to read {}: {}", vcpkg_path.display(), e))?;
        let json: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| miette!("Invalid vcpkg.json at {}: {}", vcpkg_path.display(), e))?;
        let mut deps = std::collections::HashMap::new();
        if let Some(arr) = json.get("dependencies").and_then(|v| v.as_array()) {
            for item in arr {
                if let Some(name) = item.as_str() {
                    deps.insert(name.to_string(), "latest".to_string());
                } else if let Some(name) = item.get("name").and_then(|v| v.as_str()) {
                    deps.insert(name.to_string(), "latest".to_string());
                }
            }
        }
        Ok(deps)
    }

    /// Resolve C config from manifest; if unavailable in runtime-env, recover deps from vcpkg.json.
    fn resolve_c_config_for_work_dir(
        &self,
        work_dir: &Path,
    ) -> Result<(String, std::collections::HashMap<String, String>)> {
        let is_runtime_env = work_dir.as_os_str().to_string_lossy().contains("runtime-env");
        let mut standard = "c11".to_string();
        let mut deps = std::collections::HashMap::new();

        if let Some(project_root) = self.resolve_project_root(work_dir) {
            if let Ok((std_cfg, dep_cfg)) = self.load_c_config(&project_root) {
                standard = std_cfg;
                deps = dep_cfg;
            }
        }

        if is_runtime_env && deps.is_empty() {
            let vcpkg_deps = self.load_deps_from_vcpkg_manifest(work_dir)?;
            if !vcpkg_deps.is_empty() {
                deps = vcpkg_deps;
            }
        }

        Ok((standard, deps))
    }

    /// Resolve compiler/linker flags for a C library via pkg-config (used when not using
    /// vcpkg/CMake)
    fn pkg_config_flags(&self, lib_name: &str) -> Result<Vec<String>> {
        let names_to_try: Vec<&str> = match lib_name.to_lowercase().as_str() {
            "openssl" => vec!["openssl", "openssl@3", "openssl@1.1"],
            _ => vec![lib_name],
        };

        let mut last_err = String::new();
        for name in names_to_try {
            let output = std::process::Command::new("pkg-config")
                .args(["--cflags", "--libs", name])
                .output()
                .map_err(|e| miette!("pkg-config not found or failed: {}", e))?;
            if output.status.success() {
                let out = String::from_utf8_lossy(&output.stdout);
                let flags: Vec<String> = out.split_whitespace().map(|s| s.to_string()).collect();
                return Ok(flags);
            }
            last_err = String::from_utf8_lossy(&output.stderr).trim().to_string();
        }

        Err(miette!(
            "pkg-config {} failed: {}. Install the library and pkg-config, then set PKG_CONFIG_PATH if needed (e.g. export PKG_CONFIG_PATH=\"/opt/homebrew/opt/openssl/lib/pkgconfig\").",
            lib_name,
            last_err
        ))
    }

    /// Map vcpkg package name to CMake package/components/targets metadata.
    fn cmake_dep_mapping(name: &str) -> CCmakeDep {
        match name.to_lowercase().as_str() {
            "openssl" => CCmakeDep {
                find_package: "OpenSSL".to_string(),
                components: vec!["SSL".to_string(), "Crypto".to_string()],
                targets: vec!["OpenSSL::Crypto".to_string(), "OpenSSL::SSL".to_string()],
            },
            "zlib" => CCmakeDep {
                find_package: "ZLIB".to_string(),
                components: vec![],
                targets: vec!["ZLIB::ZLIB".to_string()],
            },
            _ => {
                let pascal = name
                    .split(|c: char| c == '-' || c == '_' || c == '.')
                    .filter(|p| !p.is_empty())
                    .map(|p| {
                        let mut c = p.chars();
                        match c.next() {
                            None => String::new(),
                            Some(first) => first.to_uppercase().chain(c).collect(),
                        }
                    })
                    .collect::<String>();
                CCmakeDep {
                    find_package: pascal.clone(),
                    components: vec![],
                    targets: vec![format!("{}::{}", pascal, pascal)],
                }
            }
        }
    }

    /// Resolve vcpkg toolchain file path
    fn resolve_vcpkg_toolchain() -> Result<PathBuf> {
        fn discover_default_vcpkg_toolchain() -> Option<PathBuf> {
            let mut candidates: Vec<PathBuf> = Vec::new();

            if let Ok(home) = std::env::var("HOME") {
                let home = PathBuf::from(home);
                candidates.push(
                    home.join("vcpkg").join("scripts").join("buildsystems").join("vcpkg.cmake"),
                );
                candidates.push(
                    home.join("src")
                        .join("vcpkg")
                        .join("scripts")
                        .join("buildsystems")
                        .join("vcpkg.cmake"),
                );
            }

            candidates
                .push(PathBuf::from("/opt/homebrew/share/vcpkg/scripts/buildsystems/vcpkg.cmake"));
            candidates
                .push(PathBuf::from("/usr/local/share/vcpkg/scripts/buildsystems/vcpkg.cmake"));
            candidates.push(PathBuf::from("/opt/vcpkg/scripts/buildsystems/vcpkg.cmake"));

            candidates.into_iter().find(|p| p.exists())
        }

        if let Ok(path) = std::env::var("CMAKE_TOOLCHAIN_FILE") {
            let p = PathBuf::from(&path);
            if p.exists() {
                return Ok(p);
            }
        }
        if let Ok(root) = std::env::var("VCPKG_ROOT") {
            let toolchain =
                Path::new(&root).join("scripts").join("buildsystems").join("vcpkg.cmake");
            if toolchain.exists() {
                return Ok(toolchain);
            }
        }
        if let Some(toolchain) = discover_default_vcpkg_toolchain() {
            return Ok(toolchain);
        }
        Err(miette!(
            "vcpkg not found. Set VCPKG_ROOT or CMAKE_TOOLCHAIN_FILE. Example: export VCPKG_ROOT=/path/to/vcpkg"
        ))
    }

    /// Get VCPKG_TARGET_TRIPLET for macOS
    fn vcpkg_macos_triplet() -> Option<String> {
        #[cfg(target_os = "macos")]
        {
            let arch = std::env::consts::ARCH;
            Some(if arch == "aarch64" || arch == "arm64" {
                "arm64-osx".to_string()
            } else {
                "x64-osx".to_string()
            })
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = std::env::consts::ARCH;
            None
        }
    }

    /// Build via CMake + vcpkg (when runtime-env/c has vcpkg.json)
    fn build_with_cmake(
        &self,
        work_dir: &Path,
        source: &str,
        source_name: &str,
        output_name: &str,
        standard: &str,
        deps: &std::collections::HashMap<String, String>,
    ) -> Result<PathBuf> {
        let source_path = work_dir.join(source_name);
        std::fs::write(&source_path, source)
            .map_err(|e| miette!("Failed to write {}: {}", source_path.display(), e))?;

        let std_num = standard.trim_start_matches('c');
        let mut cmake_content = format!(
            r#"cmake_minimum_required(VERSION 3.20)
project(polybench-runner C)

set(CMAKE_C_STANDARD {})
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)
if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif()
set(CMAKE_C_FLAGS_RELEASE "-O3 -DNDEBUG")

add_executable({} {})
"#,
            std_num, output_name, source_name
        );
        let mut dep_summary = Vec::new();
        for (name, _) in deps {
            let dep = Self::cmake_dep_mapping(name);
            if dep.components.is_empty() {
                cmake_content.push_str(&format!("find_package({} REQUIRED)\n", dep.find_package));
            } else {
                cmake_content.push_str(&format!(
                    "find_package({} REQUIRED COMPONENTS {})\n",
                    dep.find_package,
                    dep.components.join(" ")
                ));
            }
            cmake_content.push_str(&format!(
                "target_link_libraries({} PRIVATE {})\n",
                output_name,
                dep.targets.join(" ")
            ));
            dep_summary.push(format!("{}=>{}", name, dep.targets.join(",")));
        }

        let cmake_path = work_dir.join("CMakeLists.txt");
        std::fs::write(&cmake_path, &cmake_content)
            .map_err(|e| miette!("Failed to write CMakeLists.txt: {}", e))?;

        let toolchain = Self::resolve_vcpkg_toolchain()?;
        let mut args: Vec<String> = vec![
            "-S".to_string(),
            ".".to_string(),
            "-B".to_string(),
            "build".to_string(),
            format!("-DCMAKE_TOOLCHAIN_FILE={}", toolchain.display()),
            "-DCMAKE_BUILD_TYPE=Release".to_string(),
        ];
        if let Some(triplet) = Self::vcpkg_macos_triplet() {
            args.push(format!("-DVCPKG_TARGET_TRIPLET={}", triplet));
        }

        let output = std::process::Command::new("cmake")
            .args(&args)
            .current_dir(work_dir)
            .output()
            .map_err(|e| miette!("Failed to run cmake: {}", e))?;

        if !output.status.success() {
            let triplet = Self::vcpkg_macos_triplet().unwrap_or_else(|| "default".to_string());
            return Err(miette!(
                "CMake configure failed (toolchain: {}, triplet: {}, deps: [{}]).\nGenerated CMakeLists.txt:\n{}\n--- stdout ---\n{}\n--- stderr ---\n{}",
                toolchain.display(),
                triplet,
                dep_summary.join("; "),
                cmake_content,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let build_output = std::process::Command::new("cmake")
            .args(["--build", "build", "--config", "Release"])
            .current_dir(work_dir)
            .output()
            .map_err(|e| miette!("Failed to run cmake --build: {}", e))?;

        if !build_output.status.success() {
            return Err(miette!(
                "C build failed:\n{}\n{}",
                String::from_utf8_lossy(&build_output.stdout),
                String::from_utf8_lossy(&build_output.stderr)
            ));
        }

        let binary_path = work_dir.join("build").join(output_name);
        if !binary_path.exists() {
            return Err(miette!(
                "C build succeeded but binary not found at {}",
                binary_path.display()
            ));
        }

        Ok(binary_path)
    }

    fn dep_likely_used_in_source(dep_name: &str, source: &str) -> bool {
        let dep = dep_name.to_lowercase();
        let src = source.to_lowercase();
        if dep == "openssl" {
            return src.contains("openssl/") ||
                src.contains("evp_") ||
                src.contains("sha256(") ||
                src.contains("sha3_");
        }
        if dep == "zlib" {
            return src.contains("zlib.h") || src.contains("deflate") || src.contains("inflate");
        }
        let normalized = dep.replace('-', "_").replace('.', "_");
        src.contains(&format!("<{}.h>", dep)) ||
            src.contains(&format!("<{}/", dep)) ||
            src.contains(&format!("<{}.h>", normalized)) ||
            src.contains(&format!("<{}/", normalized))
    }

    fn filter_deps_for_source(
        &self,
        deps: std::collections::HashMap<String, String>,
        source: &str,
    ) -> std::collections::HashMap<String, String> {
        deps.into_iter().filter(|(name, _)| Self::dep_likely_used_in_source(name, source)).collect()
    }

    fn write_source_and_build(
        &self,
        work_dir: &Path,
        source: &str,
        source_name: &str,
        output_name: &str,
    ) -> Result<PathBuf> {
        let (standard, deps) = self.resolve_c_config_for_work_dir(work_dir)?;
        let deps = self.filter_deps_for_source(deps, source);
        let use_cmake = work_dir.as_os_str().to_string_lossy().contains("runtime-env") &&
            work_dir.join("vcpkg.json").exists() &&
            !deps.is_empty();

        if use_cmake {
            return self.build_with_cmake(
                work_dir,
                source,
                source_name,
                output_name,
                &standard,
                &deps,
            );
        }

        let source_path = work_dir.join(source_name);
        std::fs::write(&source_path, source)
            .map_err(|e| miette!("Failed to write {}: {}", source_path.display(), e))?;

        let binary_path = work_dir.join(output_name);

        let mut clang_args: Vec<String> = vec!["-O3".to_string(), format!("-std={}", standard)];

        for (lib_name, _version) in deps {
            match self.pkg_config_flags(&lib_name) {
                Ok(flags) => clang_args.extend(flags),
                Err(e) => return Err(e),
            }
        }

        clang_args.push(source_path.to_string_lossy().to_string());
        clang_args.push("-o".to_string());
        clang_args.push(binary_path.to_string_lossy().to_string());
        let output = std::process::Command::new(&self.clang_binary)
            .args(&clang_args)
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

    fn last_precompile_nanos(&self) -> Option<u64> {
        self.last_precompile_nanos
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
                self.last_precompile_nanos = Some(0);
                return Ok(());
            }
        }

        let pc_start = std::time::Instant::now();
        let work_dir = self.resolve_work_dir()?;
        let binary_path = self.write_source_and_build(
            &work_dir,
            &source,
            "bench_standalone.c",
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

        let run_start = std::time::Instant::now();
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

fn emit_memory_helpers() -> String {
    // Use a tracking allocator that wraps malloc and counts total bytes allocated.
    // This is cumulative (never decreases) like Go's runtime.MemStats.TotalAlloc.
    // On macOS, we use malloc_size() to track the actual size of allocations.
    // On other platforms, we prepend each allocation with the size.
    r#"static uint64_t __polybench_total_allocated = 0;

#if defined(__APPLE__)
#include <malloc/malloc.h>

void* __polybench_malloc(size_t size) {
    void* ptr = malloc(size);
    if (ptr) {
        __polybench_total_allocated += malloc_size(ptr);
    }
    return ptr;
}

void* __polybench_calloc(size_t count, size_t size) {
    void* ptr = calloc(count, size);
    if (ptr) {
        __polybench_total_allocated += malloc_size(ptr);
    }
    return ptr;
}

void* __polybench_realloc(void* old_ptr, size_t size) {
    size_t old_size = old_ptr ? malloc_size(old_ptr) : 0;
    void* ptr = realloc(old_ptr, size);
    if (ptr) {
        size_t new_size = malloc_size(ptr);
        if (new_size > old_size) {
            __polybench_total_allocated += (new_size - old_size);
        }
    }
    return ptr;
}

void __polybench_free(void* ptr) {
    free(ptr);
}

#else

void* __polybench_malloc(size_t size) {
    __polybench_total_allocated += size;
    return malloc(size);
}

void* __polybench_calloc(size_t count, size_t size) {
    __polybench_total_allocated += count * size;
    return calloc(count, size);
}

void* __polybench_realloc(void* old_ptr, size_t size) {
    __polybench_total_allocated += size;
    return realloc(old_ptr, size);
}

void __polybench_free(void* ptr) {
    free(ptr);
}

#endif

#define malloc(size) __polybench_malloc(size)
#define calloc(count, size) __polybench_calloc(count, size)
#define realloc(ptr, size) __polybench_realloc(ptr, size)
#define free(ptr) __polybench_free(ptr)

static uint64_t __polybench_get_total_allocated(void) {
    return __polybench_total_allocated;
}
"#
    .to_string()
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
    if spec.memory {
        src.push_str(&emit_memory_helpers());
    }

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

    // Warmup (warmup_time_ms takes precedence over warmup_iterations)
    if spec.warmup_time_ms > 0 {
        src.push_str(&format!(
            "    uint64_t __warmup_start = __polybench_nanos_now();\n    uint64_t __warmup_limit = (uint64_t)({} * 1e6);\n    while ((__polybench_nanos_now() - __warmup_start) < __warmup_limit) {{\n",
            spec.warmup_time_ms
        ));
    } else if spec.warmup_iterations > 0 {
        src.push_str(&format!("    uint64_t __warmup_start = __polybench_nanos_now();\n    for (uint64_t i = 0; i < {}; i++) {{\n", spec.warmup_iterations));
    }
    if spec.warmup_time_ms > 0 || spec.warmup_iterations > 0 {
        src.push_str(&emit_hook(spec.each_hooks.get(&Lang::C), "        "));
        if spec.use_sink {
            src.push_str("        __polybench_sink = __polybench_bench();\n");
        } else {
            src.push_str("        __polybench_bench();\n");
        }
        src.push_str(
            "    }\n    uint64_t __warmup_nanos = __polybench_nanos_now() - __warmup_start;\n\n",
        );
    } else {
        src.push_str("    uint64_t __warmup_nanos = 0;\n");
    }

    let use_memory = spec.memory;
    if use_memory {
        // Reset the tracking allocator counter before the benchmark
        src.push_str("    __polybench_total_allocated = 0;\n");
        src.push_str("    uint64_t __mem_before = __polybench_get_total_allocated();\n");
    }

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
        if use_memory {
            src.push_str("    uint64_t __mem_after = __polybench_get_total_allocated();\n");
            src.push_str("    uint64_t __bytes_per_op = totalIterations ? (__mem_after - __mem_before) / totalIterations : 0;\n");
        }
        src.push_str(&emit_hook(spec.after_hooks.get(&Lang::C), "    "));
        if use_memory {
            src.push_str("    printf(\"{\\\"iterations\\\":%llu,\\\"totalNanos\\\":%.0f,\\\"warmupNanos\\\":%llu,\\\"nanosPerOp\\\":%.6f,\\\"opsPerSec\\\":%.6f,\\\"bytesPerOp\\\":%llu,\\\"samples\\\":[\", (unsigned long long)totalIterations, totalNs, (unsigned long long)__warmup_nanos, nanosPerOp, opsPerSec, (unsigned long long)__bytes_per_op);\n");
        } else {
            src.push_str("    printf(\"{\\\"iterations\\\":%llu,\\\"totalNanos\\\":%.0f,\\\"warmupNanos\\\":%llu,\\\"nanosPerOp\\\":%.6f,\\\"opsPerSec\\\":%.6f,\\\"samples\\\":[\", (unsigned long long)totalIterations, totalNs, (unsigned long long)__warmup_nanos, nanosPerOp, opsPerSec);\n");
        }
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
        src.push_str("        samples[i] = (double)(t1 - t0);\n");
        src.push_str("    }\n");
        src.push_str(
            "    double totalNs = 0.0;\n    for (uint64_t i = 0; i < iterations; i++) totalNs += samples[i];\n    double nanosPerOp = totalNs / (double)(iterations ? iterations : 1);\n    double opsPerSec = 1000000000.0 / (nanosPerOp > 0.0 ? nanosPerOp : 1.0);\n",
        );
        if use_memory {
            src.push_str("    uint64_t __mem_after = __polybench_get_total_allocated();\n");
            src.push_str("    uint64_t __bytes_per_op = iterations ? (__mem_after - __mem_before) / iterations : 0;\n");
        }
        src.push_str(&emit_hook(spec.after_hooks.get(&Lang::C), "    "));
        if use_memory {
            src.push_str("    printf(\"{\\\"iterations\\\":%llu,\\\"totalNanos\\\":%.0f,\\\"warmupNanos\\\":%llu,\\\"nanosPerOp\\\":%.6f,\\\"opsPerSec\\\":%.6f,\\\"bytesPerOp\\\":%llu,\\\"samples\\\":[\", (unsigned long long)iterations, totalNs, (unsigned long long)__warmup_nanos, nanosPerOp, opsPerSec, (unsigned long long)__bytes_per_op);\n");
        } else {
            src.push_str("    printf(\"{\\\"iterations\\\":%llu,\\\"totalNanos\\\":%.0f,\\\"warmupNanos\\\":%llu,\\\"nanosPerOp\\\":%.6f,\\\"opsPerSec\\\":%.6f,\\\"samples\\\":[\", (unsigned long long)iterations, totalNs, (unsigned long long)__warmup_nanos, nanosPerOp, opsPerSec);\n");
        }
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
        .ok_or_else(|| miette!("No benchmark output from C runtime"))?;
    let result: BenchResultJson = serde_json::from_str(json_line)
        .map_err(|e| miette!("Failed to parse C benchmark output: {}\n{}", e, stdout))?;
    Ok(result.into_measurement(outlier_detection, cv_threshold))
}
