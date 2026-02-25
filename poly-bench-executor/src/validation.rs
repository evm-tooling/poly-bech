//! Pre-run validation for benchmarks
//!
//! This module provides compile-time checking for all benchmark implementations
//! before running them, catching errors early rather than mid-execution.

use super::{compile_cache::CompileCache, ProjectRoots};
use miette::Result;
use poly_bench_dsl::Lang;
use poly_bench_ir::BenchmarkIR;
use poly_bench_runtime::{go::GoRuntime, js::JsRuntime, rust::RustRuntime, traits::Runtime};
use std::{
    collections::HashSet,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

/// Source of a compile error - helps identify shared vs unique errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSource {
    /// Error in shared setup code (imports, declarations, init)
    Setup,
    /// Error in shared helper functions
    Helper,
    /// Error in benchmark implementation (unique to this benchmark)
    Implementation,
}

impl std::fmt::Display for ErrorSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorSource::Setup => write!(f, "setup"),
            ErrorSource::Helper => write!(f, "helper"),
            ErrorSource::Implementation => write!(f, "implementation"),
        }
    }
}

/// A compile error from pre-run validation
#[derive(Debug, Clone)]
pub struct CompileError {
    /// The benchmark name(s) affected by this error
    pub benchmarks: Vec<String>,
    /// The language that failed to compile
    pub lang: Lang,
    /// The error message from the compiler
    pub message: String,
    /// Source of the error (setup, helper, or implementation)
    pub source: ErrorSource,
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.benchmarks.len() == 1 {
            write!(f, "[{}] {} - {}", self.lang, self.benchmarks[0], self.message)
        } else {
            write!(
                f,
                "[{}] {} ({} benchmarks affected) - {}",
                self.lang,
                self.source,
                self.benchmarks.len(),
                self.message
            )
        }
    }
}

/// Normalize compiler error message to extract the core error for deduplication.
/// Removes file paths and line numbers that vary between compilations.
fn normalize_error_message(msg: &str) -> String {
    let mut normalized = String::new();
    for line in msg.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Skip lines that are just file paths with line numbers
        if line.starts_with("-->") || line.starts_with("|") {
            continue;
        }
        // Skip "help:" suggestions as they're redundant
        if line.starts_with("help:") {
            continue;
        }
        // Keep error codes and messages
        if line.starts_with("error") || line.contains("error TS") {
            normalized.push_str(line);
            normalized.push('\n');
        }
    }
    normalized
}

/// Classify error source based on the error message content
fn classify_error_source(message: &str) -> ErrorSource {
    // Check for common patterns that indicate helper/setup errors
    let lower = message.to_lowercase();

    // Function/method not found errors typically come from helpers
    if lower.contains("no function") ||
        lower.contains("no method") ||
        lower.contains("not found") ||
        lower.contains("does not exist on type") ||
        lower.contains("cannot find")
    {
        return ErrorSource::Helper;
    }

    // Import errors come from setup
    if lower.contains("import") || lower.contains("use ") {
        return ErrorSource::Setup;
    }

    // Default to implementation
    ErrorSource::Implementation
}

/// Validate all benchmarks by compile-checking each language implementation.
/// Returns a list of deduplicated compile errors, or an empty vec if all pass.
///
/// Optimization: Validates the first benchmark per language sequentially to catch
/// shared code errors early, then runs remaining checks in parallel.
pub async fn validate_benchmarks(
    ir: &BenchmarkIR,
    langs: &[Lang],
    project_roots: &ProjectRoots,
) -> Result<Vec<CompileError>> {
    use std::sync::Arc;
    use tokio::sync::Mutex;

    // Track seen errors by (lang, normalized_message) to deduplicate
    let seen_errors: Arc<Mutex<HashSet<(Lang, String)>>> = Arc::new(Mutex::new(HashSet::new()));
    // Map from (lang, normalized_message) to the full error info
    let error_map: Arc<Mutex<std::collections::HashMap<(Lang, String), CompileError>>> =
        Arc::new(Mutex::new(std::collections::HashMap::new()));

    // Initialize runtimes for each language
    let go_runtime: Option<Arc<GoRuntime>> = if langs.contains(&Lang::Go) {
        let mut rt = GoRuntime::new();
        rt.set_module_root(project_roots.go_root.clone());
        Some(Arc::new(rt))
    } else {
        None
    };

    let js_runtime: Option<Arc<JsRuntime>> = if langs.contains(&Lang::TypeScript) {
        JsRuntime::new().ok().map(|mut rt| {
            rt.set_project_root(project_roots.node_root.clone());
            Arc::new(rt)
        })
    } else {
        None
    };

    let rust_runtime: Option<Arc<RustRuntime>> = if langs.contains(&Lang::Rust) {
        let mut rt = RustRuntime::new();
        rt.set_project_root(project_roots.rust_root.clone());
        Some(Arc::new(rt))
    } else {
        None
    };

    // Helper to add an error with deduplication
    async fn add_error(
        seen_errors: &Arc<Mutex<HashSet<(Lang, String)>>>,
        error_map: &Arc<Mutex<std::collections::HashMap<(Lang, String), CompileError>>>,
        lang: Lang,
        benchmark: String,
        message: String,
    ) {
        let normalized = normalize_error_message(&message);
        let key = (lang, normalized.clone());

        let mut seen = seen_errors.lock().await;
        let mut map = error_map.lock().await;

        if seen.contains(&key) {
            // Error already seen - just add this benchmark to the list
            if let Some(existing) = map.get_mut(&key) {
                if !existing.benchmarks.contains(&benchmark) {
                    existing.benchmarks.push(benchmark);
                }
            }
        } else {
            // New error - create entry
            seen.insert(key.clone());
            let source = classify_error_source(&message);
            map.insert(key, CompileError { benchmarks: vec![benchmark], lang, message, source });
        }
    }

    // Check each suite
    for suite in &ir.suites {
        // Track which languages have had their shared code validated
        let mut shared_validated: HashSet<Lang> = HashSet::new();
        let mut shared_failed: HashSet<Lang> = HashSet::new();

        // Phase 1: Validate shared code by checking the first benchmark per language
        // This catches setup/helper errors early without running all benchmarks
        for spec in &suite.benchmarks {
            // Check Go (first benchmark only)
            if spec.has_lang(Lang::Go) && !shared_validated.contains(&Lang::Go) {
                if let Some(ref rt) = go_runtime {
                    if let Err(e) = rt.compile_check(spec, suite).await {
                        let msg = e.to_string();
                        // If this is a shared code error, mark all benchmarks as affected
                        let source = classify_error_source(&msg);
                        if matches!(source, ErrorSource::Setup | ErrorSource::Helper) {
                            let all_go_benchmarks: Vec<String> = suite
                                .benchmarks
                                .iter()
                                .filter(|s| s.has_lang(Lang::Go))
                                .map(|s| s.full_name.clone())
                                .collect();

                            let normalized = normalize_error_message(&msg);
                            let key = (Lang::Go, normalized);
                            let mut map = error_map.lock().await;
                            map.insert(
                                key,
                                CompileError {
                                    benchmarks: all_go_benchmarks,
                                    lang: Lang::Go,
                                    message: msg,
                                    source,
                                },
                            );
                            shared_failed.insert(Lang::Go);
                        } else {
                            add_error(
                                &seen_errors,
                                &error_map,
                                Lang::Go,
                                spec.full_name.clone(),
                                msg,
                            )
                            .await;
                        }
                    }
                    shared_validated.insert(Lang::Go);
                }
            }

            // Check TypeScript (first benchmark only)
            if spec.has_lang(Lang::TypeScript) && !shared_validated.contains(&Lang::TypeScript) {
                if let Some(ref rt) = js_runtime {
                    if let Err(e) = rt.compile_check(spec, suite).await {
                        let msg = e.to_string();
                        let source = classify_error_source(&msg);
                        if matches!(source, ErrorSource::Setup | ErrorSource::Helper) {
                            let all_ts_benchmarks: Vec<String> = suite
                                .benchmarks
                                .iter()
                                .filter(|s| s.has_lang(Lang::TypeScript))
                                .map(|s| s.full_name.clone())
                                .collect();

                            let normalized = normalize_error_message(&msg);
                            let key = (Lang::TypeScript, normalized);
                            let mut map = error_map.lock().await;
                            map.insert(
                                key,
                                CompileError {
                                    benchmarks: all_ts_benchmarks,
                                    lang: Lang::TypeScript,
                                    message: msg,
                                    source,
                                },
                            );
                            shared_failed.insert(Lang::TypeScript);
                        } else {
                            add_error(
                                &seen_errors,
                                &error_map,
                                Lang::TypeScript,
                                spec.full_name.clone(),
                                msg,
                            )
                            .await;
                        }
                    }
                    shared_validated.insert(Lang::TypeScript);
                }
            }

            // Check Rust (first benchmark only)
            if spec.has_lang(Lang::Rust) && !shared_validated.contains(&Lang::Rust) {
                if let Some(ref rt) = rust_runtime {
                    if let Err(e) = rt.compile_check(spec, suite).await {
                        let msg = e.to_string();
                        let source = classify_error_source(&msg);
                        if matches!(source, ErrorSource::Setup | ErrorSource::Helper) {
                            let all_rust_benchmarks: Vec<String> = suite
                                .benchmarks
                                .iter()
                                .filter(|s| s.has_lang(Lang::Rust))
                                .map(|s| s.full_name.clone())
                                .collect();

                            let normalized = normalize_error_message(&msg);
                            let key = (Lang::Rust, normalized);
                            let mut map = error_map.lock().await;
                            map.insert(
                                key,
                                CompileError {
                                    benchmarks: all_rust_benchmarks,
                                    lang: Lang::Rust,
                                    message: msg,
                                    source,
                                },
                            );
                            shared_failed.insert(Lang::Rust);
                        } else {
                            add_error(
                                &seen_errors,
                                &error_map,
                                Lang::Rust,
                                spec.full_name.clone(),
                                msg,
                            )
                            .await;
                        }
                    }
                    shared_validated.insert(Lang::Rust);
                }
            }

            // Once all languages are validated, break out of the first-pass loop
            if shared_validated.len() >= langs.len() {
                break;
            }
        }

        // Phase 2: Check remaining benchmarks in parallel (skip languages with shared code
        // failures)
        let remaining_checks: Vec<_> = suite
            .benchmarks
            .iter()
            .skip(1) // Skip the first benchmark (already checked)
            .flat_map(|spec| {
                let mut checks = Vec::new();

                if spec.has_lang(Lang::Go) && !shared_failed.contains(&Lang::Go) {
                    if let Some(ref rt) = go_runtime {
                        checks.push((spec.clone(), Lang::Go, rt.clone() as Arc<dyn Runtime>));
                    }
                }
                if spec.has_lang(Lang::TypeScript) && !shared_failed.contains(&Lang::TypeScript) {
                    if let Some(ref rt) = js_runtime {
                        checks.push((
                            spec.clone(),
                            Lang::TypeScript,
                            rt.clone() as Arc<dyn Runtime>,
                        ));
                    }
                }
                if spec.has_lang(Lang::Rust) && !shared_failed.contains(&Lang::Rust) {
                    if let Some(ref rt) = rust_runtime {
                        checks.push((spec.clone(), Lang::Rust, rt.clone() as Arc<dyn Runtime>));
                    }
                }

                checks
            })
            .collect();

        // Run remaining checks in parallel
        let suite_arc = Arc::new(suite.clone());
        let futures: Vec<_> = remaining_checks
            .into_iter()
            .map(|(spec, lang, rt)| {
                let seen = Arc::clone(&seen_errors);
                let map = Arc::clone(&error_map);
                let suite = Arc::clone(&suite_arc);
                async move {
                    if let Err(e) = rt.compile_check(&spec, &suite).await {
                        add_error(&seen, &map, lang, spec.full_name.clone(), e.to_string()).await;
                    }
                }
            })
            .collect();

        futures::future::join_all(futures).await;
    }

    // Collect errors, sorted by language then by number of affected benchmarks (most affected
    // first)
    let map = error_map.lock().await;
    let mut errors: Vec<CompileError> = map.values().cloned().collect();
    errors.sort_by(|a, b| {
        a.lang
            .as_str()
            .cmp(b.lang.as_str())
            .then_with(|| b.benchmarks.len().cmp(&a.benchmarks.len()))
    });

    Ok(errors)
}

/// Statistics from a cached validation run
#[derive(Debug, Clone, Default)]
pub struct ValidationStats {
    /// Number of cache hits (skipped compilation)
    pub cache_hits: usize,
    /// Number of cache misses (required compilation)
    pub cache_misses: usize,
    /// Total benchmarks checked
    pub total_checks: usize,
}

impl std::fmt::Display for ValidationStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.cache_hits > 0 {
            let hit_rate = (self.cache_hits as f64 / self.total_checks as f64) * 100.0;
            write!(
                f,
                "{} checks ({} cached, {:.0}% hit rate)",
                self.total_checks, self.cache_hits, hit_rate
            )
        } else {
            write!(f, "{} checks", self.total_checks)
        }
    }
}

/// Validate all benchmarks with compile result caching.
/// Returns a list of deduplicated compile errors and validation statistics.
pub async fn validate_benchmarks_with_cache(
    ir: &BenchmarkIR,
    langs: &[Lang],
    project_roots: &ProjectRoots,
    cache: &CompileCache,
) -> Result<(Vec<CompileError>, ValidationStats)> {
    use tokio::sync::Mutex;

    // Statistics tracking
    let cache_hits = Arc::new(AtomicUsize::new(0));
    let cache_misses = Arc::new(AtomicUsize::new(0));
    let total_checks = Arc::new(AtomicUsize::new(0));

    // Track seen errors by (lang, normalized_message) to deduplicate
    let seen_errors: Arc<Mutex<HashSet<(Lang, String)>>> = Arc::new(Mutex::new(HashSet::new()));
    // Map from (lang, normalized_message) to the full error info
    let error_map: Arc<Mutex<std::collections::HashMap<(Lang, String), CompileError>>> =
        Arc::new(Mutex::new(std::collections::HashMap::new()));

    // Initialize runtimes for each language
    let go_runtime: Option<Arc<GoRuntime>> = if langs.contains(&Lang::Go) {
        let mut rt = GoRuntime::new();
        rt.set_module_root(project_roots.go_root.clone());
        Some(Arc::new(rt))
    } else {
        None
    };

    let js_runtime: Option<Arc<JsRuntime>> = if langs.contains(&Lang::TypeScript) {
        JsRuntime::new().ok().map(|mut rt| {
            rt.set_project_root(project_roots.node_root.clone());
            Arc::new(rt)
        })
    } else {
        None
    };

    let rust_runtime: Option<Arc<RustRuntime>> = if langs.contains(&Lang::Rust) {
        let mut rt = RustRuntime::new();
        rt.set_project_root(project_roots.rust_root.clone());
        Some(Arc::new(rt))
    } else {
        None
    };

    // Helper to add an error with deduplication
    async fn add_error(
        seen_errors: &Arc<Mutex<HashSet<(Lang, String)>>>,
        error_map: &Arc<Mutex<std::collections::HashMap<(Lang, String), CompileError>>>,
        lang: Lang,
        benchmark: String,
        message: String,
    ) {
        let normalized = normalize_error_message(&message);
        let key = (lang, normalized.clone());

        let mut seen = seen_errors.lock().await;
        let mut map = error_map.lock().await;

        if seen.contains(&key) {
            if let Some(existing) = map.get_mut(&key) {
                if !existing.benchmarks.contains(&benchmark) {
                    existing.benchmarks.push(benchmark);
                }
            }
        } else {
            seen.insert(key.clone());
            let source = classify_error_source(&message);
            map.insert(key, CompileError { benchmarks: vec![benchmark], lang, message, source });
        }
    }

    /// Check a single benchmark with caching
    async fn check_with_cache(
        rt: &Arc<dyn Runtime>,
        spec: &poly_bench_ir::BenchmarkSpec,
        suite: &poly_bench_ir::SuiteIR,
        cache: &CompileCache,
        cache_hits: &AtomicUsize,
        cache_misses: &AtomicUsize,
        total_checks: &AtomicUsize,
    ) -> Result<(), String> {
        total_checks.fetch_add(1, Ordering::Relaxed);

        // Generate source code for cache lookup
        let source = match rt.generate_check_source(spec, suite) {
            Ok(s) => s,
            Err(e) => return Err(e.to_string()),
        };

        let lang_str = rt.lang().as_str();

        // Check cache first
        if let Some(cached_result) = cache.get(&spec.full_name, lang_str, &source).await {
            cache_hits.fetch_add(1, Ordering::Relaxed);
            return cached_result;
        }

        cache_misses.fetch_add(1, Ordering::Relaxed);

        // Not in cache, perform actual compilation
        let result = rt.compile_check(spec, suite).await;

        // Store result in cache
        let cache_result = match &result {
            Ok(()) => Ok(()),
            Err(e) => Err(e.to_string()),
        };
        cache.set(&spec.full_name, lang_str, &source, cache_result).await;

        result.map_err(|e| e.to_string())
    }

    // Check each suite
    for suite in &ir.suites {
        let shared_failed: Arc<tokio::sync::Mutex<HashSet<Lang>>> =
            Arc::new(tokio::sync::Mutex::new(HashSet::new()));

        // Phase 1: Validate shared code by checking the first benchmark per language IN PARALLEL
        // This is a major optimization - previously these ran sequentially
        let mut phase1_checks: Vec<(
            poly_bench_ir::BenchmarkSpec,
            Lang,
            Arc<dyn Runtime>,
            Vec<String>,
        )> = Vec::new();

        // Find the first benchmark for each language
        for lang in langs {
            let first_spec = suite.benchmarks.iter().find(|s| s.has_lang(*lang));
            if let Some(spec) = first_spec {
                let all_benchmarks: Vec<String> = suite
                    .benchmarks
                    .iter()
                    .filter(|s| s.has_lang(*lang))
                    .map(|s| s.full_name.clone())
                    .collect();

                let rt: Option<Arc<dyn Runtime>> = match lang {
                    Lang::Go => go_runtime.as_ref().map(|r| r.clone() as Arc<dyn Runtime>),
                    Lang::TypeScript => js_runtime.as_ref().map(|r| r.clone() as Arc<dyn Runtime>),
                    Lang::Rust => rust_runtime.as_ref().map(|r| r.clone() as Arc<dyn Runtime>),
                    _ => None,
                };

                if let Some(runtime) = rt {
                    phase1_checks.push((spec.clone(), *lang, runtime, all_benchmarks));
                }
            }
        }

        // Run Phase 1 checks in parallel
        let phase1_futures: Vec<_> = phase1_checks
            .into_iter()
            .map(|(spec, lang, rt, all_benchmarks)| {
                let seen = Arc::clone(&seen_errors);
                let map = Arc::clone(&error_map);
                let failed = Arc::clone(&shared_failed);
                let hits = Arc::clone(&cache_hits);
                let misses = Arc::clone(&cache_misses);
                let total = Arc::clone(&total_checks);
                let cache = cache;
                let suite = suite.clone();

                async move {
                    let result =
                        check_with_cache(&rt, &spec, &suite, cache, &hits, &misses, &total).await;

                    if let Err(msg) = result {
                        let source = classify_error_source(&msg);
                        if matches!(source, ErrorSource::Setup | ErrorSource::Helper) {
                            // Shared code error - mark all benchmarks of this language as affected
                            let normalized = normalize_error_message(&msg);
                            let key = (lang, normalized);
                            let mut map_guard = map.lock().await;
                            map_guard.insert(
                                key,
                                CompileError {
                                    benchmarks: all_benchmarks,
                                    lang,
                                    message: msg,
                                    source,
                                },
                            );
                            let mut failed_guard = failed.lock().await;
                            failed_guard.insert(lang);
                        } else {
                            // Implementation-specific error
                            add_error(&seen, &map, lang, spec.full_name.clone(), msg).await;
                        }
                    }
                }
            })
            .collect();

        futures::future::join_all(phase1_futures).await;

        // Get the set of failed languages for Phase 2
        let shared_failed_set = shared_failed.lock().await.clone();

        // Phase 2: Check remaining benchmarks in parallel with caching
        // Skip languages that failed in Phase 1 (shared code errors)
        let remaining_checks: Vec<_> = suite
            .benchmarks
            .iter()
            .skip(1)
            .flat_map(|spec| {
                let mut checks = Vec::new();

                if spec.has_lang(Lang::Go) && !shared_failed_set.contains(&Lang::Go) {
                    if let Some(ref rt) = go_runtime {
                        checks.push((spec.clone(), Lang::Go, rt.clone() as Arc<dyn Runtime>));
                    }
                }
                if spec.has_lang(Lang::TypeScript) && !shared_failed_set.contains(&Lang::TypeScript)
                {
                    if let Some(ref rt) = js_runtime {
                        checks.push((
                            spec.clone(),
                            Lang::TypeScript,
                            rt.clone() as Arc<dyn Runtime>,
                        ));
                    }
                }
                if spec.has_lang(Lang::Rust) && !shared_failed_set.contains(&Lang::Rust) {
                    if let Some(ref rt) = rust_runtime {
                        checks.push((spec.clone(), Lang::Rust, rt.clone() as Arc<dyn Runtime>));
                    }
                }

                checks
            })
            .collect();

        let suite_arc = Arc::new(suite.clone());
        let cache_arc = Arc::new(cache);
        let futures: Vec<_> = remaining_checks
            .into_iter()
            .map(|(spec, lang, rt)| {
                let seen = Arc::clone(&seen_errors);
                let map = Arc::clone(&error_map);
                let suite = Arc::clone(&suite_arc);
                let cache = Arc::clone(&cache_arc);
                let hits = Arc::clone(&cache_hits);
                let misses = Arc::clone(&cache_misses);
                let total = Arc::clone(&total_checks);
                async move {
                    let result =
                        check_with_cache(&rt, &spec, &suite, &cache, &hits, &misses, &total).await;
                    if let Err(msg) = result {
                        add_error(&seen, &map, lang, spec.full_name.clone(), msg).await;
                    }
                }
            })
            .collect();

        futures::future::join_all(futures).await;
    }

    // Save cache to disk
    cache.save().await.map_err(|e| miette::miette!("Failed to save compile cache: {}", e))?;

    // Collect errors
    let map = error_map.lock().await;
    let mut errors: Vec<CompileError> = map.values().cloned().collect();
    errors.sort_by(|a, b| {
        a.lang
            .as_str()
            .cmp(b.lang.as_str())
            .then_with(|| b.benchmarks.len().cmp(&a.benchmarks.len()))
    });

    let stats = ValidationStats {
        cache_hits: cache_hits.load(Ordering::Relaxed),
        cache_misses: cache_misses.load(Ordering::Relaxed),
        total_checks: total_checks.load(Ordering::Relaxed),
    };

    Ok((errors, stats))
}
