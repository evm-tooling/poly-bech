//! Shared Rust code generation utilities
//!
//! This module contains common code generation functions used by both
//! the codegen and executor modules for Rust benchmarks.

use poly_bench_dsl::Lang;
use poly_bench_ir::{BenchmarkSpec, SuiteIR};
use std::collections::HashSet;

/// The BenchResult Rust struct definition
pub const BENCH_RESULT_STRUCT: &str = r#"#[derive(serde::Serialize)]
struct BenchResult {
    iterations: u64,
    total_nanos: u64,
    nanos_per_op: f64,
    ops_per_sec: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    bytes_per_op: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allocs_per_op: Option<u64>,
    samples: Vec<u64>,
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
        Self {
            // No sink declaration needed - we use black_box directly on the result
            sink_decl: "",
            sink_keepalive: "",
            // Rust doesn't have built-in memory profiling like Go's runtime.MemStats
            // For now, we skip memory profiling in Rust
            memory_decl: "",
            memory_before: "",
            memory_after: "",
        }
    }

    /// Get memory result fields for BenchResult struct
    pub fn memory_result_fields(use_memory: bool) -> String {
        if use_memory {
            // Rust doesn't have easy memory profiling - return None for now
            "        bytes_per_op: None,\n        allocs_per_op: None,\n".to_string()
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

/// Generate the warmup loop for Rust
pub fn generate_warmup_loop(
    bench_call: &str,
    sink_keepalive: &str,
    each_hook: Option<&String>,
    warmup_count: &str,
) -> String {
    let each_hook_code = each_hook
        .map(|h| h.trim().lines().map(|line| format!("        {}\n", line)).collect::<String>())
        .unwrap_or_default();

    format!(
        r#"    // Warmup
    for _ in 0..{warmup_count} {{
{each_hook_code}        {bench_call};
{sink_keepalive}    }}
"#
    )
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
        nanos_per_op,
        ops_per_sec,
{memory_result}        samples,
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
}
