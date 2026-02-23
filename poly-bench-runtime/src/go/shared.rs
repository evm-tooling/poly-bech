//! Shared Go code generation utilities
//!
//! This module contains common code generation functions used by both
//! the plugin codegen (codegen.rs) and the standalone executor (executor.rs).

use poly_bench_dsl::Lang;
use poly_bench_ir::{BenchmarkSpec, SuiteIR};
use std::collections::HashSet;

/// The BenchResult Go struct definition
pub const BENCH_RESULT_STRUCT: &str = r#"type BenchResult struct {
	Iterations  uint64   `json:"iterations"`
	TotalNanos  uint64   `json:"total_nanos"`
	NanosPerOp  float64  `json:"nanos_per_op"`
	OpsPerSec   float64  `json:"ops_per_sec"`
	BytesPerOp  uint64   `json:"bytes_per_op"`
	AllocsPerOp uint64   `json:"allocs_per_op"`
	Samples     []uint64 `json:"samples"`
	RawResult   string   `json:"raw_result,omitempty"`
	SuccessfulResults []string `json:"successful_results,omitempty"`
}
"#;

/// Collected imports for code generation
pub struct CollectedImports<'a> {
    pub all_imports: HashSet<&'a str>,
}

impl<'a> CollectedImports<'a> {
    /// Collect imports for an entire IR (all suites/benchmarks)
    pub fn for_ir(
        user_imports: &[&'a str],
        stdlib_imports: &[&'a str],
        needs_runtime: bool,
        needs_sync: bool,
    ) -> Self {
        let mut all_imports: HashSet<&'a str> = HashSet::new();

        // Base imports
        all_imports.insert("\"encoding/json\"");
        all_imports.insert("\"time\"");

        if needs_runtime {
            all_imports.insert("\"runtime\"");
        }
        if needs_sync {
            all_imports.insert("\"sync\"");
        }

        for import_spec in user_imports {
            all_imports.insert(import_spec);
        }

        for import_spec in stdlib_imports {
            all_imports.insert(import_spec);
        }

        Self { all_imports }
    }

    /// Generate the import block code
    pub fn generate_import_block(&self) -> String {
        let mut code = String::new();
        code.push_str("import (\n");

        let mut sorted_imports: Vec<_> = self.all_imports.iter().collect();
        sorted_imports.sort();

        for import_spec in sorted_imports {
            code.push_str(&format!("\t{}\n", import_spec));
        }

        code.push_str(")\n\n");
        code
    }
}

/// Sink and memory profiling declarations
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
            sink_decl: if spec.use_sink { "\tvar __sink interface{}\n" } else { "" },
            sink_keepalive: if spec.use_sink { "\t\truntime.KeepAlive(__sink)\n" } else { "" },
            memory_decl: if spec.memory {
                "\tvar memBefore, memAfter runtime.MemStats\n"
            } else {
                ""
            },
            memory_before: if spec.memory {
                "\n\truntime.GC()\n\truntime.ReadMemStats(&memBefore)\n"
            } else {
                ""
            },
            memory_after: if spec.memory {
                "\n\truntime.GC()\n\truntime.ReadMemStats(&memAfter)\n"
            } else {
                ""
            },
        }
    }

    /// Get memory result fields for BenchResult struct
    pub fn memory_result_fields(use_memory: bool, iter_var: &str) -> String {
        if use_memory {
            format!(
                "\t\tBytesPerOp:  (memAfter.TotalAlloc - memBefore.TotalAlloc) / uint64({}),\n\t\tAllocsPerOp: (memAfter.Mallocs - memBefore.Mallocs) / uint64({}),\n",
                iter_var, iter_var
            )
        } else {
            String::new()
        }
    }
}

/// Generate the benchmark call expression
pub fn generate_bench_call(impl_code: &str, use_sink: bool) -> String {
    if use_sink {
        format!("__sink = {}", impl_code)
    } else {
        impl_code.to_string()
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

/// Generate the auto-calibration measurement loop
pub fn generate_auto_mode_loop(
    bench_call: &str,
    sink_keepalive: &str,
    each_hook: Option<&String>,
    target_time_ms: u64,
) -> String {
    let each_hook_code = each_hook
        .map(|h| h.trim().lines().map(|line| format!("\t\t\t{}\n", line)).collect::<String>())
        .unwrap_or_default();

    format!(
        r#"	// Adaptive measurement phase (like Go's testing.B)
	// Run batches, scale up N, stop when totalElapsed >= targetTime
	targetNanos := int64({})
	batchSize := 1
	totalIterations := 0
	var totalNanos int64
	
	for totalNanos < targetNanos {{
		// Run batch without per-iteration timing (fast)
		batchStart := time.Now()
		for i := 0; i < batchSize; i++ {{
{}			{}
{}		}}
		batchElapsed := time.Since(batchStart).Nanoseconds()
		
		totalIterations += batchSize
		totalNanos += batchElapsed
		
		if totalNanos >= targetNanos {{
			break
		}}
		
		// Scale up for next batch (like Go's predictN)
		if batchElapsed > 0 {{
			remainingNanos := targetNanos - totalNanos
			predicted := int(float64(batchSize) * float64(remainingNanos) / float64(batchElapsed))
			
			var newSize int
			if remainingNanos < batchElapsed {{
				newSize = predicted
				if newSize < 1 {{
					newSize = 1
				}}
			}} else if remainingNanos < targetNanos / 5 {{
				newSize = int(float64(predicted) * 0.9)
				if newSize < 1 {{
					newSize = 1
				}}
			}} else {{
				newSize = int(float64(predicted) * 1.1)
				if newSize <= batchSize {{
					newSize = batchSize * 2
				}}
				if newSize > batchSize * 10 {{
					newSize = batchSize * 10
				}}
			}}
			if newSize < 1 {{
				newSize = 1
			}}
			batchSize = newSize
		}} else {{
			batchSize *= 10
		}}
	}}
"#,
        target_time_ms * 1_000_000,
        each_hook_code,
        bench_call,
        sink_keepalive
    )
}

/// Generate sample collection code
pub fn generate_sample_collection(
    bench_call: &str,
    sink_keepalive: &str,
    each_hook: Option<&String>,
    sample_count: &str,
    total_var: &str,
) -> String {
    let each_hook_code = each_hook
        .map(|h| h.trim().lines().map(|line| format!("\t\t{}\n", line)).collect::<String>())
        .unwrap_or_default();

    format!(
        r#"	// Collect samples for statistical analysis
	sampleCount := {sample_count}
	if sampleCount > {total_var} {{
		sampleCount = {total_var}
	}}
	samples := make([]uint64, sampleCount)
	for i := 0; i < sampleCount; i++ {{
{}		start := time.Now()
		{}
{}		samples[i] = uint64(time.Since(start).Nanoseconds())
	}}
"#,
        each_hook_code, bench_call, sink_keepalive
    )
}

/// Generate strict async sequential auto-mode loop (no adaptive batching)
pub fn generate_async_auto_mode_loop(
    bench_call: &str,
    sink_keepalive: &str,
    each_hook: Option<&String>,
    target_time_ms: u64,
) -> String {
    let each_hook_code = each_hook
        .map(|h| h.trim().lines().map(|line| format!("\t\t{}\n", line)).collect::<String>())
        .unwrap_or_default();

    format!(
        r#"	// Async-sequential auto mode: one awaited/completed call per iteration.
	targetNanos := int64({})
	var totalIterations int
	var totalNanos int64
	samples := make([]uint64, 0, 50)

	for totalNanos < targetNanos {{
{}		start := time.Now()
		{}
{}		elapsed := time.Since(start).Nanoseconds()
		totalNanos += elapsed
		totalIterations++
		if len(samples) < 50 {{
			samples = append(samples, uint64(elapsed))
		}}

		resultBytes, _ := json.Marshal(__sink)
		if string(resultBytes) != "null" {{
			successfulResults = append(successfulResults, string(resultBytes))
		}}
	}}
"#,
        target_time_ms * 1_000_000,
        each_hook_code,
        bench_call,
        sink_keepalive
    )
}

/// Generate the warmup loop
pub fn generate_warmup_loop(
    bench_call: &str,
    sink_keepalive: &str,
    each_hook: Option<&String>,
    warmup_count: &str,
) -> String {
    let each_hook_code = each_hook
        .map(|h| h.trim().lines().map(|line| format!("\t\t{}\n", line)).collect::<String>())
        .unwrap_or_default();

    format!(
        r#"	// Warmup
	for i := 0; i < {warmup_count}; i++ {{
{}		{}
{}	}}
"#,
        each_hook_code, bench_call, sink_keepalive
    )
}

/// Generate fixed iteration measurement loop
pub fn generate_fixed_mode_loop(
    bench_call: &str,
    sink_keepalive: &str,
    each_hook: Option<&String>,
    iter_var: &str,
) -> String {
    let each_hook_code = each_hook
        .map(|h| h.trim().lines().map(|line| format!("\t\t{}\n", line)).collect::<String>())
        .unwrap_or_default();

    format!(
        r#"	// Timed run
	var totalNanos uint64
	for i := 0; i < {iter_var}; i++ {{
{}		start := time.Now()
		{}
{}		elapsed := time.Since(start).Nanoseconds()
		if i < len(samples) {{
			samples[i] = uint64(elapsed)
		}}
		totalNanos += uint64(elapsed)
	}}
"#,
        each_hook_code, bench_call, sink_keepalive
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

    // Add init function if present
    if let Some(init_code) = suite.init_code.get(&lang) {
        if !init_code.trim().is_empty() {
            code.push_str("func init() {\n");
            code.push_str(init_code);
            if !init_code.ends_with('\n') {
                code.push('\n');
            }
            code.push_str("}\n\n");
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
                // Wrap in IIFE if it contains return statement
                if fixture_impl.contains("return") {
                    code.push_str(&format!(
                        "var {} = func() []byte {{\n{}\n}}()\n",
                        fixture_name, fixture_impl
                    ));
                } else {
                    code.push_str(&format!("var {} = {}\n", fixture_name, fixture_impl));
                }
            } else if !fixture.data.is_empty() {
                code.push_str(&format!("var {} = {}\n", fixture_name, fixture.as_go_bytes()));
            }
        }
    }

    if !code.is_empty() {
        code.push('\n');
    }

    code
}

/// Generate result calculation and return
pub fn generate_result_return(
    iter_var: &str,
    memory_result: &str,
    include_println: bool,
) -> String {
    let output = if include_println {
        r#"
	jsonBytes, _ := json.Marshal(result)
	fmt.Println(string(jsonBytes))
"#
    } else {
        r#"
	jsonBytes, _ := json.Marshal(result)
	return string(jsonBytes)
"#
    };

    format!(
        r#"
	nanosPerOp := float64(totalNanos) / float64({iter_var})
	opsPerSec := 1e9 / nanosPerOp
	rawResultBytes, _ := json.Marshal(__sink)
	rawResult := ""
	if string(rawResultBytes) != "null" {{
		rawResult = string(rawResultBytes)
	}}
	if successfulResults == nil {{
		successfulResults = []string{{}}
	}}
	
	result := BenchResult{{
		Iterations:  uint64({iter_var}),
		TotalNanos:  uint64(totalNanos),
		NanosPerOp:  nanosPerOp,
		OpsPerSec:   opsPerSec,
{memory_result}		Samples:     samples,
		RawResult:   rawResult,
		SuccessfulResults: successfulResults,
	}}
{output}"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bench_result_struct() {
        assert!(BENCH_RESULT_STRUCT.contains("type BenchResult struct"));
        assert!(BENCH_RESULT_STRUCT.contains("Iterations"));
        assert!(BENCH_RESULT_STRUCT.contains("json:"));
    }

    #[test]
    fn test_generate_bench_call() {
        assert_eq!(generate_bench_call("foo()", true), "__sink = foo()");
        assert_eq!(generate_bench_call("foo()", false), "foo()");
    }

    #[test]
    fn test_memory_result_fields() {
        let result = SinkMemoryDecls::memory_result_fields(true, "iterations");
        assert!(result.contains("BytesPerOp"));
        assert!(result.contains("AllocsPerOp"));

        let result = SinkMemoryDecls::memory_result_fields(false, "iterations");
        assert!(result.is_empty());
    }
}
