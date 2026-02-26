//! Shared Rust code generation utilities
//!
//! This module contains common code generation functions used by both
//! the codegen and executor modules for Rust benchmarks.

use poly_bench_dsl::{AsyncSamplingPolicy, Lang};
use poly_bench_ir::{BenchmarkSpec, SuiteIR};
use std::collections::HashSet;

/// The BenchResult Rust struct definition
pub const BENCH_RESULT_STRUCT: &str = r#"#[derive(serde::Serialize)]
struct BenchResult {
    iterations: u64,
    total_nanos: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    warmup_nanos: Option<u64>,
    nanos_per_op: f64,
    ops_per_sec: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    bytes_per_op: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allocs_per_op: Option<u64>,
    samples: Vec<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_result: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    successful_results: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    successful_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_count: Option<u64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    error_samples: Vec<String>,
}
"#;

/// Collected imports for Rust code generation
pub struct CollectedImports<'a> {
    pub all_imports: HashSet<&'a str>,
}

impl<'a> CollectedImports<'a> {
    /// Collect imports for an entire IR (all suites/benchmarks)
    pub fn for_ir(
        user_imports: &[&'a str],
        stdlib_imports: &[&'a str],
        needs_memory: bool,
        needs_sync: bool,
    ) -> Self {
        let mut all_imports: HashSet<&'a str> = HashSet::new();

        // Base imports - always needed for timing
        all_imports.insert("use std::time::Instant;");

        if needs_memory {
            // Rust doesn't have built-in memory profiling like Go
            // We could use jemalloc or similar, but skip for now
        }
        if needs_sync {
            all_imports.insert("use std::sync::Arc;");
            all_imports.insert("use std::thread;");
        }

        for import_spec in user_imports {
            all_imports.insert(import_spec);
        }

        for import_spec in stdlib_imports {
            all_imports.insert(import_spec);
        }

        Self { all_imports }
    }

    /// Generate the use statement block
    pub fn generate_use_block(&self) -> String {
        let mut code = String::new();

        let mut sorted_imports: Vec<_> = self.all_imports.iter().collect();
        sorted_imports.sort();

        for import_spec in sorted_imports {
            code.push_str(import_spec);
            if !import_spec.ends_with(';') {
                code.push(';');
            }
            code.push('\n');
        }

        if !code.is_empty() {
            code.push('\n');
        }
        code
    }
}

/// Sink and memory profiling declarations for Rust
pub struct SinkMemoryDecls {
    pub sink_decl: &'static str,
    pub sink_keepalive: &'static str,
    pub memory_decl: &'static str,
    pub memory_before: &'static str,
    pub memory_after: &'static str,
}

impl SinkMemoryDecls {
    /// Create declarations based on benchmark spec
    pub fn from_spec(spec: &BenchmarkSpec) -> Self {
        if spec.memory {
            Self {
                sink_decl: "",
                sink_keepalive: "",
                memory_decl: r#"    let mem_session = alloc_tracker::Session::new();
    let mut mem_op = mem_session.operation("bench");
    let mut mem_bytes_per_op: Option<u64> = None;
    let mut mem_allocs_per_op: Option<u64> = None;
"#,
                memory_before: "    let _mem_span = mem_op.measure_process();\n",
                memory_after: r#"    drop(_mem_span);
    let mem_report = mem_session.to_report();
    for (_, op) in mem_report.operations() {
        let total_bytes = op.total_bytes_allocated();
        let total_allocs = op.total_allocations_count();
        if total_iterations > 0 {
            mem_bytes_per_op = Some((total_bytes / total_iterations as u64));
            mem_allocs_per_op = Some((total_allocs / total_iterations as u64));
        }
        break;
    }
"#,
            }
        } else {
            Self {
                sink_decl: "",
                sink_keepalive: "",
                memory_decl: "",
                memory_before: "",
                memory_after: "",
            }
        }
    }

    /// Get memory result fields for BenchResult struct
    pub fn memory_result_fields(use_memory: bool) -> String {
        if use_memory {
            "        bytes_per_op: mem_bytes_per_op,\n        allocs_per_op: mem_allocs_per_op,\n"
                .to_string()
        } else {
            "        bytes_per_op: None,\n        allocs_per_op: None,\n".to_string()
        }
    }
}

/// Generate the benchmark call expression
/// Uses std::hint::black_box to prevent the compiler from optimizing away the result
/// without incurring heap allocation overhead
pub fn generate_bench_call(impl_code: &str, use_sink: bool) -> String {
    if use_sink {
        // Use black_box directly - it's zero-cost and prevents optimization
        format!("std::hint::black_box({})", impl_code)
    } else {
        format!("let _ = {}", impl_code)
    }
}

/// Generate hook code with proper indentation
pub fn format_hook(hook: Option<&String>, prefix: &str, indent: &str) -> String {
    hook.map(|h| {
        let mut result = format!("{indent}// {prefix} hook\n");
        for line in h.trim().lines() {
            result.push_str(&format!("{indent}{}\n", line));
        }
        result.push('\n');
        result
    })
    .unwrap_or_default()
}

/// Generate the warmup loop for Rust.
/// When warmup_time_ms > 0, uses time-based warmup (takes precedence).
/// Otherwise when warmup_iterations > 0, uses iteration-based warmup.
/// When both are 0, returns empty string (no warmup).
pub fn generate_warmup_loop(
    bench_call: &str,
    sink_keepalive: &str,
    each_hook: Option<&String>,
    warmup_iterations: u64,
    warmup_time_ms: u64,
) -> String {
    let each_hook_code = each_hook
        .map(|h| h.trim().lines().map(|line| format!("        {}\n", line)).collect::<String>())
        .unwrap_or_default();

    if warmup_time_ms > 0 {
        format!(
            r#"    // Warmup (time-based)
    let warmup_start = Instant::now();
    let warmup_limit = std::time::Duration::from_millis({});
    while warmup_start.elapsed() < warmup_limit {{
{each_hook_code}        {bench_call};
{sink_keepalive}    }}
    warmup_nanos = Some(warmup_start.elapsed().as_nanos() as u64);
"#,
            warmup_time_ms,
        )
    } else if warmup_iterations > 0 {
        format!(
            r#"    // Warmup
    let warmup_start = Instant::now();
    for _ in 0..{warmup_iterations} {{
{each_hook_code}        {bench_call};
{sink_keepalive}    }}
    warmup_nanos = Some(warmup_start.elapsed().as_nanos() as u64);
"#
        )
    } else {
        String::new()
    }
}

/// Generate fixed iteration measurement loop for Rust
pub fn generate_fixed_mode_loop(
    bench_call: &str,
    sink_keepalive: &str,
    each_hook: Option<&String>,
    iter_var: &str,
) -> String {
    let each_hook_code = each_hook
        .map(|h| h.trim().lines().map(|line| format!("        {}\n", line)).collect::<String>())
        .unwrap_or_default();

    format!(
        r#"    // Timed run
    let mut total_nanos: u64 = 0;
    for i in 0..{iter_var} {{
{each_hook_code}        let start = Instant::now();
        {bench_call};
{sink_keepalive}        let elapsed = start.elapsed().as_nanos() as u64;
        samples[i as usize] = elapsed;
        total_nanos += elapsed;
    }}
"#
    )
}

/// Generate fixed iteration measurement loop for async Rust benchmarks
/// Includes error tracking and result capture like TypeScript
pub fn generate_async_fixed_mode_loop(
    bench_call: &str,
    _sink_keepalive: &str,
    each_hook: Option<&String>,
    iter_var: &str,
    sample_cap: u64,
) -> String {
    let each_hook_code = each_hook
        .map(|h| h.trim().lines().map(|line| format!("        {}\n", line)).collect::<String>())
        .unwrap_or_default();

    format!(
        r#"    // Async fixed iteration timed run with error tracking
    let mut total_nanos: i64 = 0;
    let mut successful_results: Vec<String> = Vec::new();
    let mut successful_count: u64 = 0;
    let mut error_count: u64 = 0;
    let mut error_samples: Vec<String> = Vec::new();

    for i in 0..{iter_var} {{
{each_hook_code}        let start = Instant::now();
        let iter_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {{
            {bench_call}
        }}));
        let elapsed = start.elapsed().as_nanos() as i64;
        total_nanos += elapsed;
        samples[i as usize] = elapsed as u64;

        match iter_result {{
            Ok(result) => {{
                successful_count += 1;
                if successful_results.len() < {sample_cap} as usize {{
                    if let Ok(json) = serde_json::to_string(&result) {{
                        if json != "null" {{
                            successful_results.push(json);
                        }}
                    }}
                }}
            }}
            Err(e) => {{
                error_count += 1;
                if error_samples.len() < {sample_cap} as usize {{
                    let msg = if let Some(s) = e.downcast_ref::<&str>() {{
                        s.to_string()
                    }} else if let Some(s) = e.downcast_ref::<String>() {{
                        s.clone()
                    }} else {{
                        "panic".to_string()
                    }};
                    error_samples.push(msg);
                }}
            }}
        }}
    }}
    let total_iterations = {iter_var};
"#,
        iter_var = iter_var,
        each_hook_code = each_hook_code,
        bench_call = bench_call,
        sample_cap = sample_cap,
    )
}

/// Generate the auto-calibration measurement loop for Rust
pub fn generate_auto_mode_loop(
    bench_call: &str,
    sink_keepalive: &str,
    each_hook: Option<&String>,
    target_time_ms: u64,
) -> String {
    let each_hook_code = each_hook
        .map(|h| h.trim().lines().map(|line| format!("            {}\n", line)).collect::<String>())
        .unwrap_or_default();

    let target_nanos_val = target_time_ms * 1_000_000;
    format!(
        r#"    // Adaptive measurement phase
    let target_nanos: i64 = {target_nanos_val};
    let mut batch_size: i64 = 1;
    let mut total_iterations: i64 = 0;
    let mut total_nanos: i64 = 0;
    
    while total_nanos < target_nanos {{
        let batch_start = Instant::now();
        for _ in 0..batch_size {{
{each_hook_code}            {bench_call};
{sink_keepalive}        }}
        let batch_elapsed = batch_start.elapsed().as_nanos() as i64;
        
        total_iterations += batch_size;
        total_nanos += batch_elapsed;
        
        if total_nanos >= target_nanos {{
            break;
        }}
        
        // Scale up for next batch
        if batch_elapsed > 0 {{
            let remaining_nanos = target_nanos - total_nanos;
            let predicted = (batch_size as f64 * remaining_nanos as f64 / batch_elapsed as f64) as i64;
            
            let new_size = if remaining_nanos < batch_elapsed {{
                predicted.max(1)
            }} else if remaining_nanos < target_nanos / 5 {{
                ((predicted as f64 * 0.9) as i64).max(1)
            }} else {{
                let mut n = (predicted as f64 * 1.1) as i64;
                if n <= batch_size {{
                    n = batch_size * 2;
                }}
                if n > batch_size * 10 {{
                    n = batch_size * 10;
                }}
                n.max(1)
            }};
            batch_size = new_size;
        }} else {{
            batch_size *= 10;
        }}
    }}
"#
    )
}

/// Generate sample collection code for Rust
pub fn generate_sample_collection(
    bench_call: &str,
    sink_keepalive: &str,
    each_hook: Option<&String>,
    sample_count: &str,
    total_var: &str,
) -> String {
    let each_hook_code = each_hook
        .map(|h| h.trim().lines().map(|line| format!("        {}\n", line)).collect::<String>())
        .unwrap_or_default();

    format!(
        r#"    // Collect samples for statistical analysis
    let sample_count = ({sample_count} as i64).min({total_var}) as usize;
    let mut samples: Vec<u64> = Vec::with_capacity(sample_count);
    for _ in 0..sample_count {{
{each_hook_code}        let start = Instant::now();
        {bench_call};
{sink_keepalive}        samples.push(start.elapsed().as_nanos() as u64);
    }}
"#
    )
}

/// Generate async sequential auto-mode loop (TimeBudgeted policy)
/// Runs until target time budget is reached, with error tracking and reservoir sampling
pub fn generate_async_auto_mode_loop(
    bench_call: &str,
    _sink_keepalive: &str,
    each_hook: Option<&String>,
    target_time_ms: u64,
    sample_cap: u64,
) -> String {
    let each_hook_code = each_hook
        .map(|h| h.trim().lines().map(|line| format!("        {}\n", line)).collect::<String>())
        .unwrap_or_default();

    let target_nanos_val = target_time_ms * 1_000_000;
    format!(
        r#"    // Async-sequential auto mode: one completed call per iteration (TimeBudgeted)
    let target_nanos: i64 = {target_nanos_val};
    let mut total_iterations: i64 = 0;
    let mut total_nanos: i64 = 0;
    let mut samples: Vec<u64> = Vec::with_capacity({sample_cap} as usize);
    let mut rng_state: u64 = 0x9E37_79B9_7F4A_7C15;
    let mut successful_results: Vec<String> = Vec::new();
    let mut successful_count: u64 = 0;
    let mut error_count: u64 = 0;
    let mut error_samples: Vec<String> = Vec::new();

    while total_nanos < target_nanos {{
{each_hook_code}        let start = Instant::now();
        let iter_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {{
            {bench_call}
        }}));
        let elapsed = start.elapsed().as_nanos() as i64;
        total_nanos += elapsed;

        match iter_result {{
            Ok(result) => {{
                successful_count += 1;
                if let Ok(json) = serde_json::to_string(&result) {{
                    if json != "null" {{
                        successful_results.push(json);
                    }}
                }}
            }}
            Err(e) => {{
                error_count += 1;
                if error_samples.len() < {sample_cap} as usize {{
                    let msg = if let Some(s) = e.downcast_ref::<&str>() {{
                        s.to_string()
                    }} else if let Some(s) = e.downcast_ref::<String>() {{
                        s.clone()
                    }} else {{
                        "panic".to_string()
                    }};
                    error_samples.push(msg);
                }}
            }}
        }}

        // Reservoir sampling for samples
        if samples.len() < {sample_cap} as usize {{
            samples.push(elapsed as u64);
        }} else if {sample_cap} > 0 {{
            rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1);
            let replace_idx = (rng_state % (total_iterations as u64 + 1)) as usize;
            if replace_idx < {sample_cap} as usize {{
                samples[replace_idx] = elapsed as u64;
            }}
        }}
        total_iterations += 1;
    }}
"#,
        target_nanos_val = target_nanos_val,
        sample_cap = sample_cap,
        each_hook_code = each_hook_code,
        bench_call = bench_call,
    )
}

/// Generate async fixed-cap loop (FixedCap policy)
/// Runs exactly sample_cap iterations with error tracking and reservoir sampling
pub fn generate_async_fixed_cap_loop(
    bench_call: &str,
    _sink_keepalive: &str,
    each_hook: Option<&String>,
    sample_cap: u64,
) -> String {
    let each_hook_code = each_hook
        .map(|h| h.trim().lines().map(|line| format!("        {}\n", line)).collect::<String>())
        .unwrap_or_default();

    format!(
        r#"    // Async fixed-cap mode: run a bounded number of completed async calls (FixedCap)
    let mut total_iterations: i64 = 0;
    let mut total_nanos: i64 = 0;
    let mut samples: Vec<u64> = Vec::with_capacity({sample_cap} as usize);
    let mut rng_state: u64 = 0x9E37_79B9_7F4A_7C15;
    let mut successful_results: Vec<String> = Vec::new();
    let mut successful_count: u64 = 0;
    let mut error_count: u64 = 0;
    let mut error_samples: Vec<String> = Vec::new();

    while total_iterations < {sample_cap} as i64 {{
{each_hook_code}        let start = Instant::now();
        let iter_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {{
            {bench_call}
        }}));
        let elapsed = start.elapsed().as_nanos() as i64;
        total_nanos += elapsed;

        match iter_result {{
            Ok(result) => {{
                successful_count += 1;
                if let Ok(json) = serde_json::to_string(&result) {{
                    if json != "null" {{
                        successful_results.push(json);
                    }}
                }}
            }}
            Err(e) => {{
                error_count += 1;
                if error_samples.len() < {sample_cap} as usize {{
                    let msg = if let Some(s) = e.downcast_ref::<&str>() {{
                        s.to_string()
                    }} else if let Some(s) = e.downcast_ref::<String>() {{
                        s.clone()
                    }} else {{
                        "panic".to_string()
                    }};
                    error_samples.push(msg);
                }}
            }}
        }}

        // Reservoir sampling for samples
        if samples.len() < {sample_cap} as usize {{
            samples.push(elapsed as u64);
        }} else if {sample_cap} > 0 {{
            rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1);
            let replace_idx = (rng_state % (total_iterations as u64 + 1)) as usize;
            if replace_idx < {sample_cap} as usize {{
                samples[replace_idx] = elapsed as u64;
            }}
        }}
        total_iterations += 1;
    }}
"#,
        sample_cap = sample_cap,
        each_hook_code = each_hook_code,
        bench_call = bench_call,
    )
}

/// Select async loop strategy based on policy
pub fn generate_async_loop_by_policy(
    policy: AsyncSamplingPolicy,
    bench_call: &str,
    sink_keepalive: &str,
    each_hook: Option<&String>,
    target_time_ms: u64,
    sample_cap: u64,
) -> String {
    match policy {
        AsyncSamplingPolicy::FixedCap => {
            generate_async_fixed_cap_loop(bench_call, sink_keepalive, each_hook, sample_cap)
        }
        AsyncSamplingPolicy::TimeBudgeted => generate_async_auto_mode_loop(
            bench_call,
            sink_keepalive,
            each_hook,
            target_time_ms,
            sample_cap,
        ),
    }
}

/// Generate result calculation and output for async benchmarks
pub fn generate_async_result_return(iter_var: &str, memory_result: &str) -> String {
    format!(
        r#"
    let nanos_per_op = total_nanos as f64 / {iter_var} as f64;
    let ops_per_sec = 1e9 / nanos_per_op;
    
    let result = BenchResult {{
        iterations: {iter_var} as u64,
        total_nanos: total_nanos as u64,
        warmup_nanos,
        nanos_per_op,
        ops_per_sec,
{memory_result}        samples,
        raw_result: None,
        successful_results,
        successful_count: Some(successful_count),
        error_count: Some(error_count),
        error_samples,
    }};
    
    println!("{{}}", serde_json::to_string(&result).unwrap());
"#
    )
}

/// Generate suite-level code (declarations, init, helpers)
pub fn generate_suite_code(suite: &SuiteIR, lang: Lang) -> String {
    let mut code = String::new();

    // Add declarations
    if let Some(declarations) = suite.declarations.get(&lang) {
        if !declarations.trim().is_empty() {
            code.push_str("// Declarations\n");
            code.push_str(declarations);
            if !declarations.ends_with('\n') {
                code.push('\n');
            }
            code.push('\n');
        }
    }

    // Add helpers
    if let Some(helpers) = suite.helpers.get(&lang) {
        if !helpers.trim().is_empty() {
            code.push_str("// Helpers\n");
            code.push_str(helpers);
            if !helpers.ends_with('\n') {
                code.push('\n');
            }
            code.push('\n');
        }
    }

    code
}

/// Generate init code to be called at the start of main()
pub fn generate_init_code(suite: &SuiteIR, lang: Lang) -> String {
    let mut code = String::new();

    if let Some(init_code) = suite.init_code.get(&lang) {
        if !init_code.trim().is_empty() {
            code.push_str("    // Init\n");
            for line in init_code.lines() {
                code.push_str(&format!("    {}\n", line));
            }
            code.push('\n');
        }
    }

    code
}

/// Generate fixture code for a single benchmark's fixture references
pub fn generate_fixtures_for_spec(spec: &BenchmarkSpec, suite: &SuiteIR, lang: Lang) -> String {
    let mut code = String::new();

    for fixture_name in &spec.fixture_refs {
        if let Some(fixture) = suite.get_fixture(fixture_name) {
            if let Some(fixture_impl) = fixture.implementations.get(&lang) {
                code.push_str(&format!("    let {} = {};\n", fixture_name, fixture_impl));
            } else if !fixture.data.is_empty() {
                code.push_str(&format!(
                    "    let {}: Vec<u8> = vec![{}];\n",
                    fixture_name,
                    fixture.as_rust_bytes()
                ));
            }
        }
    }

    if !code.is_empty() {
        code.push('\n');
    }

    code
}

/// Generate result calculation and output
pub fn generate_result_return(iter_var: &str, memory_result: &str) -> String {
    format!(
        r#"
    let nanos_per_op = total_nanos as f64 / {iter_var} as f64;
    let ops_per_sec = 1e9 / nanos_per_op;
    
    let result = BenchResult {{
        iterations: {iter_var} as u64,
        total_nanos: total_nanos as u64,
        warmup_nanos,
        nanos_per_op,
        ops_per_sec,
{memory_result}        samples,
        raw_result: None,
        successful_results: Vec::new(),
        successful_count: None,
        error_count: None,
        error_samples: Vec::new(),
    }};
    
    println!("{{}}", serde_json::to_string(&result).unwrap());
"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bench_result_struct() {
        assert!(BENCH_RESULT_STRUCT.contains("struct BenchResult"));
        assert!(BENCH_RESULT_STRUCT.contains("iterations"));
        assert!(BENCH_RESULT_STRUCT.contains("serde::Serialize"));
        assert!(BENCH_RESULT_STRUCT.contains("successful_count"));
        assert!(BENCH_RESULT_STRUCT.contains("error_count"));
        assert!(BENCH_RESULT_STRUCT.contains("error_samples"));
    }

    #[test]
    fn test_generate_bench_call() {
        assert_eq!(generate_bench_call("foo()", true), "std::hint::black_box(foo())");
        assert_eq!(generate_bench_call("foo()", false), "let _ = foo()");
    }

    #[test]
    fn test_memory_result_fields() {
        let result = SinkMemoryDecls::memory_result_fields(true);
        assert!(result.contains("bytes_per_op"));
        assert!(result.contains("allocs_per_op"));
    }

    #[test]
    fn test_generate_async_loop_by_policy_time_budgeted() {
        let loop_code = generate_async_loop_by_policy(
            AsyncSamplingPolicy::TimeBudgeted,
            "std::hint::black_box(call())",
            "",
            None,
            1000,
            10,
        );
        assert!(loop_code.contains("target_nanos: i64 ="));
        assert!(loop_code.contains("while total_nanos < target_nanos"));
        assert!(loop_code.contains("catch_unwind"));
        assert!(loop_code.contains("error_count += 1"));
        assert!(loop_code.contains("total_nanos += elapsed"));
        assert!(loop_code.contains("rng_state"));
    }

    #[test]
    fn test_generate_async_loop_by_policy_fixed_cap() {
        let loop_code = generate_async_loop_by_policy(
            AsyncSamplingPolicy::FixedCap,
            "std::hint::black_box(call())",
            "",
            None,
            1000,
            10,
        );
        assert!(loop_code.contains("while total_iterations < 10"));
        assert!(!loop_code.contains("target_nanos: i64 ="));
        assert!(loop_code.contains("catch_unwind"));
        assert!(loop_code.contains("successful_count += 1"));
        assert!(loop_code.contains("total_nanos += elapsed"));
        assert!(loop_code.contains("rng_state"));
    }
}
