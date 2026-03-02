//! Zig runtime executor.

use async_trait::async_trait;
use miette::{miette, Result};
use poly_bench_dsl::{BenchMode, Lang};
use poly_bench_ir::{BenchmarkSpec, SuiteIR};
use poly_bench_stdlib as stdlib;
use poly_bench_traits::{Measurement, Runtime, RuntimeConfig, RuntimeFactory};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    process::Stdio,
    time::{Duration, Instant},
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
        // Use Polybench-managed toolchain if available, otherwise fall back to system PATH
        let (zig_binary, _is_managed) =
            poly_bench_traits::resolve_binary(poly_bench_dsl::Lang::Zig).map_err(|_| {
                miette!(
                "zig not found. Install via 'poly-bench add-runtime zig' or ensure zig is in PATH"
            )
            })?;

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

    fn resolve_project_root(&self, work_dir: &Path) -> Option<PathBuf> {
        let work_str = work_dir.to_string_lossy();
        if work_str.contains("runtime-env") {
            work_dir
                .parent()
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
                .map(|p| p.to_path_buf())
        } else if work_str.contains(".polybench") {
            work_dir.parent().and_then(|p| p.parent()).map(|p| p.to_path_buf())
        } else {
            None
        }
    }

    fn load_zig_dependencies(
        &self,
        project_root: &Path,
    ) -> std::collections::HashMap<String, String> {
        let manifest_path = project_root.join("polybench.toml");
        let Ok(content) = std::fs::read_to_string(&manifest_path) else {
            return std::collections::HashMap::new();
        };
        let Ok(full) = content.parse::<toml::Value>() else {
            return std::collections::HashMap::new();
        };
        full.get("zig")
            .and_then(|v| v.get("dependencies"))
            .and_then(|v| v.as_table())
            .map(|t| {
                t.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Parse dep names from build.zig.zon .dependencies = .{ .dep_name = .{ ... }, }
    fn parse_zon_dep_names(&self, zon_path: &Path) -> Vec<String> {
        let Ok(content) = std::fs::read_to_string(zon_path) else {
            return vec![];
        };
        let re = regex::Regex::new(r"\.(\w+)\s*=\s*\.\{").unwrap();
        let deps_start_re = regex::Regex::new(r"\.dependencies\s*=\s*\.\{").unwrap();
        let mut in_deps = false;
        let mut depth = 0;
        let mut deps = Vec::new();
        for line in content.lines() {
            if deps_start_re.is_match(line) {
                in_deps = true;
                depth = 1;
                continue;
            }
            if in_deps {
                for cap in re.captures_iter(line) {
                    if let Some(name) = cap.get(1) {
                        deps.push(name.as_str().to_string());
                    }
                }
                depth += line.matches('{').count();
                depth -= line.matches('}').count();
                if depth == 0 {
                    break;
                }
            }
        }
        deps
    }

    fn parse_zon_dep_hashes(&self, zon_path: &Path) -> HashMap<String, String> {
        let mut out = HashMap::new();
        let Ok(content) = std::fs::read_to_string(zon_path) else {
            return out;
        };
        let Ok(dep_start_re) =
            regex::Regex::new(r#"^\s*\.([A-Za-z_][A-Za-z0-9_]*)\s*=\s*\.\{\s*$"#)
        else {
            return out;
        };
        let Ok(hash_re) = regex::Regex::new(r#"^\s*\.hash\s*=\s*"([^"]+)""#) else {
            return out;
        };

        let mut in_deps_block = false;
        let mut current_dep: Option<String> = None;
        for line in content.lines() {
            let trimmed = line.trim();
            if !in_deps_block {
                if trimmed.starts_with(".dependencies") && trimmed.contains(".{") {
                    in_deps_block = true;
                }
                continue;
            }
            if current_dep.is_none() && trimmed == "}," {
                in_deps_block = false;
                continue;
            }
            if let Some(dep) = current_dep.as_ref() {
                if let Some(cap) = hash_re.captures(trimmed) {
                    if let Some(m) = cap.get(1) {
                        out.insert(dep.clone(), m.as_str().to_string());
                    }
                }
                if trimmed == "}," {
                    current_dep = None;
                }
                continue;
            }
            if let Some(cap) = dep_start_re.captures(trimmed) {
                current_dep = cap.get(1).map(|m| m.as_str().to_string());
            }
        }
        out
    }

    fn resolve_dep_module_source(&self, dep: &str, dep_hash: Option<&str>) -> String {
        let mut candidates = vec!["src/root.zig".to_string(), format!("src/{dep}.zig")];

        if let Some(hash) = dep_hash {
            if let Ok(home) = std::env::var("HOME") {
                let cache_root =
                    PathBuf::from(home).join(".cache").join("zig").join("p").join(hash);
                for rel in &candidates {
                    if cache_root.join(rel).exists() {
                        return rel.clone();
                    }
                }
                let src_dir = cache_root.join("src");
                if let Ok(entries) = std::fs::read_dir(src_dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.extension().and_then(|e| e.to_str()) == Some("zig") {
                            if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                                return format!("src/{name}");
                            }
                        }
                    }
                }
            }
        }

        // Fallback for simple single-file packages.
        candidates.remove(0); // prefer src/{dep}.zig over src/root.zig when unknown
        candidates.into_iter().next().unwrap_or_else(|| format!("src/{dep}.zig"))
    }

    fn patch_dep_for_zig_compat(&self, dep_hash: &str) -> Result<()> {
        let home = match std::env::var("HOME") {
            Ok(h) => h,
            Err(_) => return Ok(()),
        };
        let cache_root = PathBuf::from(home).join(".cache").join("zig").join("p").join(dep_hash);
        if !cache_root.exists() {
            return Ok(());
        }
        let src_dir = cache_root.join("src");
        if !src_dir.exists() {
            return Ok(());
        }
        // Only patch legacy "single-sequence + index capture" loops:
        // `for (items) |item, i|` -> `for (items, 0..) |item, i|`
        // Keep multi-input loops untouched (e.g. `for (a, b, 0..) |...|`).
        let re_two_capture_loop =
            regex::Regex::new(r#"for\s*\(\s*([^,\)\n]+)\s*\)\s*\|([^,\|]+),\s*([^\|]+)\|"#)
                .map_err(|e| miette!("Invalid regex: {}", e))?;
        let entries = std::fs::read_dir(&src_dir)
            .map_err(|e| miette!("Failed to read {}: {}", src_dir.display(), e))?;
        for entry in entries {
            let entry = entry.map_err(|e| miette!("Failed to read dir entry: {}", e))?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("zig") {
                continue;
            }
            let content = std::fs::read_to_string(&path)
                .map_err(|e| miette!("Failed to read {}: {}", path.display(), e))?;
            let updated =
                re_two_capture_loop.replace_all(&content, "for ($1, 0..) |$2, $3|").to_string();
            if updated != content {
                std::fs::write(&path, updated)
                    .map_err(|e| miette!("Failed to write {}: {}", path.display(), e))?;
            }
        }
        Ok(())
    }

    fn dedupe_zon_deps(
        &self,
        zon_deps: &[String],
        dep_hashes: &HashMap<String, String>,
    ) -> Vec<String> {
        let mut seen = std::collections::HashSet::new();
        let mut out = Vec::new();
        for dep in zon_deps {
            if let Some(base) = dep.strip_suffix("_git") {
                if let (Some(base_hash), Some(dep_hash)) =
                    (dep_hashes.get(base), dep_hashes.get(dep))
                {
                    if base_hash == dep_hash {
                        continue;
                    }
                }
            }
            if seen.insert(dep.clone()) {
                out.push(dep.clone());
            }
        }
        out
    }

    /// Normalize legacy build.zig.zon name syntax (`.name = "pkg"`) to enum literal form.
    fn normalize_build_zig_zon(&self, zon_path: &Path) -> Result<()> {
        let content = std::fs::read_to_string(zon_path)
            .map_err(|e| miette!("Failed to read {}: {}", zon_path.display(), e))?;
        if !content.contains(".name = \"") {
            return Ok(());
        }
        let re = regex::Regex::new(r#"(?m)^(\s*\.name\s*=\s*)"([^"]+)"(\s*,\s*)$"#)
            .map_err(|e| miette!("Invalid regex: {}", e))?;
        let updated = re
            .replace(&content, |caps: &regex::Captures| {
                let raw = caps.get(2).map(|m| m.as_str()).unwrap_or("polybench");
                let enum_name: String = raw
                    .chars()
                    .map(|c| if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' })
                    .collect();
                format!("{}.{enum_name}{}", &caps[1], &caps[3])
            })
            .to_string();
        if updated != content {
            std::fs::write(zon_path, updated)
                .map_err(|e| miette!("Failed to write {}: {}", zon_path.display(), e))?;
        }
        Ok(())
    }

    fn extract_suggested_fingerprint(stderr: &str) -> Option<String> {
        let re = regex::Regex::new(r"suggested value:\s*(0x[0-9a-fA-F]+)").ok()?;
        re.captures(stderr).and_then(|c| c.get(1)).map(|m| m.as_str().to_string())
    }

    fn set_build_zig_zon_fingerprint(&self, zon_path: &Path, fingerprint: &str) -> Result<()> {
        let content = std::fs::read_to_string(zon_path)
            .map_err(|e| miette!("Failed to read {}: {}", zon_path.display(), e))?;
        let fp_line = format!("    .fingerprint = {fingerprint},");
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        if let Some(idx) = lines.iter().position(|l| l.trim_start().starts_with(".fingerprint")) {
            lines[idx] = fp_line;
        } else if let Some(version_idx) =
            lines.iter().position(|l| l.trim_start().starts_with(".version"))
        {
            lines.insert(version_idx + 1, fp_line);
        } else {
            // Fallback: inject right after the opening .{ line.
            let insert_idx =
                lines.iter().position(|l| l.trim() == ".{").map(|i| i + 1).unwrap_or(0);
            lines.insert(insert_idx, fp_line);
        }

        std::fs::write(zon_path, lines.join("\n") + "\n")
            .map_err(|e| miette!("Failed to write {}: {}", zon_path.display(), e))?;
        Ok(())
    }

    fn detect_used_external_deps(
        &self,
        source: &str,
        declared_deps: &HashSet<String>,
    ) -> Result<HashSet<String>> {
        let mut used = HashSet::new();

        // Match `const alias = @import("module");` so we can ignore aliases never used.
        let alias_import_re = regex::Regex::new(
            r#"(?m)^\s*const\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*@import\(\s*"([^"]+)"\s*\)\s*;"#,
        )
        .map_err(|e| miette!("Invalid regex: {}", e))?;
        let direct_import_re = regex::Regex::new(r#"@import\(\s*"([^"]+)"\s*\)"#)
            .map_err(|e| miette!("Invalid regex: {}", e))?;

        let mut source_without_import_lines = source.to_string();
        for caps in alias_import_re.captures_iter(source) {
            let alias = caps.get(1).map(|m| m.as_str()).unwrap_or_default();
            let module = caps.get(2).map(|m| m.as_str()).unwrap_or_default();
            if !declared_deps.contains(module) {
                continue;
            }
            if let Some(full) = caps.get(0) {
                source_without_import_lines =
                    source_without_import_lines.replace(full.as_str(), "");
            }
            let alias_use_re = regex::Regex::new(&format!(r"\b{}\b", regex::escape(alias)))
                .map_err(|e| miette!("Invalid regex: {}", e))?;
            if alias_use_re.is_match(&source_without_import_lines) {
                used.insert(module.to_string());
            }
        }

        // If a dependency is imported in a non-alias form, treat it as used.
        for caps in direct_import_re.captures_iter(source) {
            let module = caps.get(1).map(|m| m.as_str()).unwrap_or_default();
            if declared_deps.contains(module) {
                used.insert(module.to_string());
            }
        }

        Ok(used)
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

        let zon_path = work_dir.join("build.zig.zon");
        if zon_path.exists() {
            self.normalize_build_zig_zon(&zon_path)?;
        }
        let manifest_deps = self
            .resolve_project_root(work_dir)
            .map(|p| self.load_zig_dependencies(&p))
            .unwrap_or_default();
        let zon_deps = if zon_path.exists() { self.parse_zon_dep_names(&zon_path) } else { vec![] };

        let declared_deps: HashSet<String> =
            manifest_deps.keys().cloned().chain(zon_deps.iter().cloned()).collect();
        let used_external_deps = self.detect_used_external_deps(source, &declared_deps)?;
        let used_zon_deps: Vec<String> =
            zon_deps.iter().filter(|dep| used_external_deps.contains(*dep)).cloned().collect();
        let use_zig_build = !used_zon_deps.is_empty();

        let link_libc = source.contains("__polybench_mem_snapshot");
        if use_zig_build && zon_path.exists() {
            self.build_with_zig_build(work_dir, source_name, output_name, &used_zon_deps, link_libc)
        } else {
            self.build_with_build_exe(work_dir, &source_path, output_name, link_libc)
        }
    }

    fn build_with_build_exe(
        &self,
        work_dir: &Path,
        source_path: &Path,
        output_name: &str,
        link_libc: bool,
    ) -> Result<PathBuf> {
        let binary_path = work_dir.join(output_name);
        let emit_bin = format!("-femit-bin={}", binary_path.to_string_lossy());
        let mut cmd = std::process::Command::new(&self.zig_binary);
        cmd.args([
            "build-exe",
            "-O",
            "ReleaseFast",
            source_path.to_string_lossy().as_ref(),
            &emit_bin,
        ]);
        if link_libc {
            cmd.arg("-lc");
        }
        let output =
            cmd.current_dir(work_dir).output().map_err(|e| miette!("Failed to run zig: {}", e))?;

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

    fn build_with_zig_build(
        &self,
        work_dir: &Path,
        source_name: &str,
        output_name: &str,
        zon_deps: &[String],
        link_libc: bool,
    ) -> Result<PathBuf> {
        let zon_path = work_dir.join("build.zig.zon");
        let dep_hashes = self.parse_zon_dep_hashes(&zon_path);
        let zon_deps = self.dedupe_zon_deps(zon_deps, &dep_hashes);
        for hash in dep_hashes.values() {
            self.patch_dep_for_zig_compat(hash)?;
        }
        let dep_modules: Vec<(String, String)> = zon_deps
            .iter()
            .map(|dep| {
                let module_source =
                    self.resolve_dep_module_source(dep, dep_hashes.get(dep).map(|s| s.as_str()));
                (dep.clone(), module_source)
            })
            .collect();
        let build_zig_content =
            self.generate_build_zig_for_bench(source_name, output_name, &dep_modules, link_libc);
        let build_zig_path = work_dir.join("build.zig");
        let backup_path = work_dir.join("build.zig.polybench.bak");
        if build_zig_path.exists() {
            std::fs::copy(&build_zig_path, &backup_path)
                .map_err(|e| miette!("Failed to backup build.zig: {}", e))?;
        }
        std::fs::write(&build_zig_path, build_zig_content)
            .map_err(|e| miette!("Failed to write build.zig: {}", e))?;

        let result = (|| {
            let run_build = || -> Result<std::process::Output> {
                self.run_zig_build_with_timeout(work_dir, Duration::from_secs(10))
            };

            let mut output = run_build()?;
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                if stderr.contains("missing top-level 'fingerprint' field") {
                    if let Some(fp) = Self::extract_suggested_fingerprint(&stderr) {
                        let zon_path = work_dir.join("build.zig.zon");
                        self.set_build_zig_zon_fingerprint(&zon_path, &fp)?;
                        output = run_build()?;
                    }
                }
            }

            if !output.status.success() {
                return Err(miette!(
                    "Zig compilation failed:\n{}\n{}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                ));
            }

            let binary_path = work_dir.join("zig-out").join("bin").join(output_name);
            if !binary_path.exists() {
                return Err(miette!(
                    "Zig build succeeded but binary not found at {}",
                    binary_path.display()
                ));
            }
            Ok(binary_path)
        })();

        if backup_path.exists() {
            let _ = std::fs::rename(&backup_path, &build_zig_path);
        }
        result
    }

    fn run_zig_build_with_timeout(
        &self,
        work_dir: &Path,
        timeout: Duration,
    ) -> Result<std::process::Output> {
        let mut child = std::process::Command::new(&self.zig_binary)
            .args(["build", "-Doptimize=ReleaseFast"])
            .current_dir(work_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| miette!("Failed to spawn zig build: {}", e))?;

        let started = Instant::now();
        loop {
            if child
                .try_wait()
                .map_err(|e| miette!("Failed while waiting for zig build: {}", e))?
                .is_some()
            {
                return child
                    .wait_with_output()
                    .map_err(|e| miette!("Failed to collect zig build output: {}", e));
            }

            if started.elapsed() >= timeout {
                let _ = child.kill();
                let output = child
                    .wait_with_output()
                    .map_err(|e| miette!("Failed to collect timed-out zig build output: {}", e))?;
                return Err(miette!(
                    "Zig build timed out after {}s.\n{}\n{}",
                    timeout.as_secs(),
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                ));
            }

            std::thread::sleep(Duration::from_millis(100));
        }
    }

    fn generate_build_zig_for_bench(
        &self,
        source_name: &str,
        output_name: &str,
        dep_modules: &[(String, String)],
        link_libc: bool,
    ) -> String {
        let source_str = source_name.replace('\\', "/");
        let mut s = format!(
            r#"const std = @import("std");
const builtin = @import("builtin");

pub fn build(b: *std.Build) void {{
    const target = b.standardTargetOptions(.{{}});
    const optimize = b.standardOptimizeOption(.{{}});
    const zig_ver = builtin.zig_version;
    const zig_0_15 = std.SemanticVersion{{ .major = 0, .minor = 15, .patch = 0 }};

    const exe = if (comptime zig_ver.order(zig_0_15) != .lt) b.addExecutable(.{{
        .name = "{}",
        .root_module = b.createModule(.{{
            .root_source_file = b.path("{}"),
            .target = target,
            .optimize = optimize,
        }}),
    }}) else b.addExecutable(.{{
        .name = "{}",
        .root_source_file = b.path("{}"),
        .target = target,
        .optimize = optimize,
    }});
"#,
            output_name, source_str, output_name, source_str
        );
        for (dep, module_source) in dep_modules {
            let dep_var = dep.replace('-', "_");
            s.push_str(&format!(
                r#"
    const {0} = b.dependency("{1}", .{{
        .target = target,
        .optimize = optimize,
    }});
    const {0}_mod = b.createModule(.{{
        .root_source_file = {0}.path("{2}"),
        .target = target,
        .optimize = optimize,
    }});
    exe.root_module.addImport("{1}", {0}_mod);
"#,
                dep_var, dep, module_source
            ));
        }
        if link_libc {
            s.push_str("    exe.link_libc = true;\n");
        }
        s.push_str("    b.installArtifact(exe);\n}\n");
        s
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

fn emit_memory_helpers() -> String {
    // Simple memory tracking using OS-level heap stats
    // Note: This tracks current heap usage, not cumulative allocations.
    // If allocations are freed within each benchmark iteration, this may show 0.
    r#"fn __polybench_mem_snapshot() u64 {
    const builtin = @import("builtin");
    if (builtin.os.tag == .linux) {
        const c = @cImport({
            @cInclude("malloc.h");
        });
        const mi = c.mallinfo();
        return @as(u64, @intCast(mi.uordblks));
    } else if (builtin.os.tag == .macos) {
        const c = @cImport({
            @cInclude("malloc/malloc.h");
        });
        var stats: c.malloc_statistics_t = undefined;
        c.malloc_zone_statistics(c.malloc_default_zone(), &stats);
        return @as(u64, @intCast(stats.size_in_use));
    } else if (builtin.os.tag == .windows) {
        const c = @cImport({
            @cInclude("windows.h");
            @cInclude("psapi.h");
        });
        var pmc: c.PROCESS_MEMORY_COUNTERS_EX = undefined;
        pmc.cb = @sizeOf(c.PROCESS_MEMORY_COUNTERS_EX);
        if (c.GetProcessMemoryInfo(c.GetCurrentProcess(), @ptrCast(&pmc), @sizeOf(c.PROCESS_MEMORY_COUNTERS_EX)) != 0) {
            return @as(u64, pmc.WorkingSetSize);
        }
        return 0;
    } else {
        return 0;
    }
}
"#
    .to_string()
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
    // Wrap impl code to discard any return value (Zig 0.15+ is strict about unused values)
    let impl_trimmed = impl_code.trim();
    if impl_trimmed.contains('\n') || impl_trimmed.starts_with('{') {
        // Multi-line or block - emit as-is
        for line in impl_code.lines() {
            src.push_str("    ");
            src.push_str(line);
            src.push('\n');
        }
    } else {
        // Single expression - wrap with _ = to discard return value
        src.push_str("    _ = ");
        src.push_str(impl_trimmed.trim_end_matches(';'));
        src.push_str(";\n");
    }
    src.push_str("}\n\n");
    if spec.memory {
        src.push_str(&emit_memory_helpers());
    }

    if check_only {
        src.push_str("pub fn main() void {}\n");
        return Ok(src);
    }

    src.push_str("pub fn main() !void {\n");
    src.push_str("    const __zig_ver = @import(\"builtin\").zig_version;\n");
    src.push_str(
        "    const __ver_0_13 = std.SemanticVersion{ .major = 0, .minor = 13, .patch = 0 };\n",
    );
    src.push_str(
        "    const __ver_0_15 = std.SemanticVersion{ .major = 0, .minor = 15, .patch = 0 };\n",
    );
    src.push_str("    const __is_zig_13_or_14 = comptime (__zig_ver.order(__ver_0_13) != .lt and __zig_ver.order(__ver_0_15) == .lt);\n");
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

    let use_memory = spec.memory;
    if use_memory {
        // Track memory using snapshots - accumulate positive deltas
        src.push_str("    var __total_allocated: u64 = 0;\n");
        src.push_str("    var __mem_last = __polybench_mem_snapshot();\n");
    }

    if spec.mode == BenchMode::Auto {
        src.push_str(&format!(
            "    const __allocator = std.heap.page_allocator;\n    const target_ns = {:.0};\n    var total_iterations: u64 = 0;\n    var total_ns: f64 = 0;\n    var batch: u64 = 100;\n    var samples = std.ArrayList(f64).initCapacity(__allocator, 16) catch return;\n    defer if (__is_zig_13_or_14) samples.deinit() else samples.deinit(__allocator);\n    while (total_ns < target_ns) {{\n        const t0 = std.time.Instant.now() catch return;\n        for (0..batch) |_| {{\n",
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
        if use_memory {
            // Sample memory after each batch and accumulate positive deltas
            src.push_str("        const __mem_now = __polybench_mem_snapshot();\n");
            src.push_str("        if (__mem_now > __mem_last) __total_allocated += (__mem_now - __mem_last);\n");
            src.push_str("        __mem_last = __mem_now;\n");
        }
        src.push_str("        const elapsed_ns = @as(f64, @floatFromInt(t1.since(t0)));\n");
        src.push_str("        total_ns += elapsed_ns;\n        total_iterations += batch;\n");
        src.push_str("        if (__is_zig_13_or_14) _ = samples.append(elapsed_ns / @as(f64, @floatFromInt(if (batch > 0) batch else 1))) catch break else _ = samples.append(__allocator, elapsed_ns / @as(f64, @floatFromInt(if (batch > 0) batch else 1))) catch break;\n");
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
        if use_memory {
            src.push_str("    const __bytes_per_op = if (total_iterations > 0) __total_allocated / total_iterations else @as(u64, 0);\n");
        }
        src.push_str(&emit_hook(spec.after_hooks.get(&Lang::Zig), "    "));
        if use_memory {
            src.push_str("    if (__is_zig_13_or_14) {\n");
            src.push_str(
                "        var __stdout_writer = std.io.bufferedWriter(std.io.getStdOut().writer());\n",
            );
            src.push_str("        const stdout = __stdout_writer.writer();\n");
            src.push_str("        try stdout.print(\"{{\\\"iterations\\\":{},\\\"totalNanos\\\":{d:.0},\\\"warmupNanos\\\":{},\\\"nanosPerOp\\\":{d:.6},\\\"opsPerSec\\\":{d:.6},\\\"bytesPerOp\\\":{},\\\"samples\\\":[\", .{ total_iterations, total_ns, __warmup_nanos, nanos_per_op, ops_per_sec, __bytes_per_op });\n");
        } else {
            src.push_str("    if (__is_zig_13_or_14) {\n");
            src.push_str(
                "        var __stdout_writer = std.io.bufferedWriter(std.io.getStdOut().writer());\n",
            );
            src.push_str("        const stdout = __stdout_writer.writer();\n");
            src.push_str("        try stdout.print(\"{{\\\"iterations\\\":{},\\\"totalNanos\\\":{d:.0},\\\"warmupNanos\\\":{},\\\"nanosPerOp\\\":{d:.6},\\\"opsPerSec\\\":{d:.6},\\\"samples\\\":[\", .{ total_iterations, total_ns, __warmup_nanos, nanos_per_op, ops_per_sec });\n");
        }
        src.push_str("        for (samples.items, 0..) |s, i| {\n");
        src.push_str("            if (i > 0) _ = stdout.writeAll(\",\") catch {};\n");
        src.push_str("            try stdout.print(\"{d:.0}\", .{s});\n");
        src.push_str("        }\n");
        src.push_str("        _ = stdout.writeAll(\"]}\\n\") catch {};\n");
        src.push_str("        __stdout_writer.flush() catch {};\n");
        src.push_str("    } else {\n");
        src.push_str("        var __stdout_buffer: [4096]u8 = undefined;\n");
        src.push_str(
            "        var __stdout_writer = std.fs.File.stdout().writer(&__stdout_buffer);\n",
        );
        src.push_str("        const stdout = &__stdout_writer.interface;\n");
        if use_memory {
            src.push_str("        try stdout.print(\"{{\\\"iterations\\\":{},\\\"totalNanos\\\":{d:.0},\\\"warmupNanos\\\":{},\\\"nanosPerOp\\\":{d:.6},\\\"opsPerSec\\\":{d:.6},\\\"bytesPerOp\\\":{},\\\"samples\\\":[\", .{ total_iterations, total_ns, __warmup_nanos, nanos_per_op, ops_per_sec, __bytes_per_op });\n");
        } else {
            src.push_str("        try stdout.print(\"{{\\\"iterations\\\":{},\\\"totalNanos\\\":{d:.0},\\\"warmupNanos\\\":{},\\\"nanosPerOp\\\":{d:.6},\\\"opsPerSec\\\":{d:.6},\\\"samples\\\":[\", .{ total_iterations, total_ns, __warmup_nanos, nanos_per_op, ops_per_sec });\n");
        }
        src.push_str("        for (samples.items, 0..) |s, i| {\n");
        src.push_str("            if (i > 0) _ = stdout.writeAll(\",\") catch {};\n");
        src.push_str("            try stdout.print(\"{d:.0}\", .{s});\n");
        src.push_str("        }\n");
        src.push_str("        _ = stdout.writeAll(\"]}\\n\") catch {};\n");
        src.push_str("        stdout.flush() catch {};\n");
        src.push_str("    }\n");
    } else {
        src.push_str(&format!(
            "    const iterations: u64 = {};\n    var samples: [{}]f64 = undefined;\n    for (0..iterations) |i| {{\n        const t0 = std.time.Instant.now() catch return;\n",
            spec.iterations, spec.iterations
        ));
        src.push_str(&emit_hook(spec.each_hooks.get(&Lang::Zig), "        "));
        src.push_str("        __polybench_bench();\n");
        src.push_str("        const t1 = std.time.Instant.now() catch return;\n");
        src.push_str("        samples[i] = @as(f64, @floatFromInt(t1.since(t0)));\n");
        if use_memory {
            // Sample memory after each iteration and accumulate positive deltas
            src.push_str("        const __mem_now = __polybench_mem_snapshot();\n");
            src.push_str("        if (__mem_now > __mem_last) __total_allocated += (__mem_now - __mem_last);\n");
            src.push_str("        __mem_last = __mem_now;\n");
        }
        src.push_str("    }\n");
        src.push_str(
            "    var total_ns: f64 = 0;\n    for (samples) |s| total_ns += s;\n    const nanos_per_op = total_ns / @as(f64, @floatFromInt(if (iterations > 0) iterations else 1));\n    const ops_per_sec = 1000000000.0 / (if (nanos_per_op > 0) nanos_per_op else 1.0);\n",
        );
        if use_memory {
            src.push_str("    const __bytes_per_op = if (iterations > 0) __total_allocated / iterations else @as(u64, 0);\n");
        }
        src.push_str(&emit_hook(spec.after_hooks.get(&Lang::Zig), "    "));
        if use_memory {
            src.push_str("    if (__is_zig_13_or_14) {\n");
            src.push_str(
                "        var __stdout_writer = std.io.bufferedWriter(std.io.getStdOut().writer());\n",
            );
            src.push_str("        const stdout = __stdout_writer.writer();\n");
            src.push_str("        try stdout.print(\"{{\\\"iterations\\\":{},\\\"totalNanos\\\":{d:.0},\\\"warmupNanos\\\":{},\\\"nanosPerOp\\\":{d:.6},\\\"opsPerSec\\\":{d:.6},\\\"bytesPerOp\\\":{},\\\"samples\\\":[\", .{ iterations, total_ns, __warmup_nanos, nanos_per_op, ops_per_sec, __bytes_per_op });\n");
        } else {
            src.push_str("    if (__is_zig_13_or_14) {\n");
            src.push_str(
                "        var __stdout_writer = std.io.bufferedWriter(std.io.getStdOut().writer());\n",
            );
            src.push_str("        const stdout = __stdout_writer.writer();\n");
            src.push_str("        try stdout.print(\"{{\\\"iterations\\\":{},\\\"totalNanos\\\":{d:.0},\\\"warmupNanos\\\":{},\\\"nanosPerOp\\\":{d:.6},\\\"opsPerSec\\\":{d:.6},\\\"samples\\\":[\", .{ iterations, total_ns, __warmup_nanos, nanos_per_op, ops_per_sec });\n");
        }
        src.push_str("        for (samples, 0..) |s, i| {\n");
        src.push_str("            if (i > 0) _ = stdout.writeAll(\",\") catch {};\n");
        src.push_str("            try stdout.print(\"{d:.0}\", .{s});\n");
        src.push_str("        }\n");
        src.push_str("        _ = stdout.writeAll(\"]}\\n\") catch {};\n");
        src.push_str("        __stdout_writer.flush() catch {};\n");
        src.push_str("    } else {\n");
        src.push_str("        var __stdout_buffer: [4096]u8 = undefined;\n");
        src.push_str(
            "        var __stdout_writer = std.fs.File.stdout().writer(&__stdout_buffer);\n",
        );
        src.push_str("        const stdout = &__stdout_writer.interface;\n");
        if use_memory {
            src.push_str("        try stdout.print(\"{{\\\"iterations\\\":{},\\\"totalNanos\\\":{d:.0},\\\"warmupNanos\\\":{},\\\"nanosPerOp\\\":{d:.6},\\\"opsPerSec\\\":{d:.6},\\\"bytesPerOp\\\":{},\\\"samples\\\":[\", .{ iterations, total_ns, __warmup_nanos, nanos_per_op, ops_per_sec, __bytes_per_op });\n");
        } else {
            src.push_str("        try stdout.print(\"{{\\\"iterations\\\":{},\\\"totalNanos\\\":{d:.0},\\\"warmupNanos\\\":{},\\\"nanosPerOp\\\":{d:.6},\\\"opsPerSec\\\":{d:.6},\\\"samples\\\":[\", .{ iterations, total_ns, __warmup_nanos, nanos_per_op, ops_per_sec });\n");
        }
        src.push_str("        for (samples, 0..) |s, i| {\n");
        src.push_str("            if (i > 0) _ = stdout.writeAll(\",\") catch {};\n");
        src.push_str("            try stdout.print(\"{d:.0}\", .{s});\n");
        src.push_str("        }\n");
        src.push_str("        _ = stdout.writeAll(\"]}\\n\") catch {};\n");
        src.push_str("        stdout.flush() catch {};\n");
        src.push_str("    }\n");
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
        .ok_or_else(|| miette!("No benchmark output from Zig runtime"))?;
    let result: BenchResultJson = serde_json::from_str(json_line)
        .map_err(|e| miette!("Failed to parse Zig benchmark output: {}\n{}", e, stdout))?;
    Ok(result.into_measurement(outlier_detection, cv_threshold))
}
