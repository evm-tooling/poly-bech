//! AST to IR lowering
//!
//! Transforms the parsed AST into a normalized IR suitable for execution.

use crate::{
    fixtures::{
        decode_base64, decode_hex, decode_raw, decode_utf8, extract_fixture_refs, load_base64_file,
        load_hex_file, load_raw_file, load_utf8_file, normalize_csv_to_bytes,
        normalize_json_to_bytes,
    },
    imports::{extract_go_imports, extract_rust_imports, extract_ts_imports, ParsedSetup},
    AnvilConfigIR, BenchmarkIR, BenchmarkSpec, ChartDirectiveIR, FixtureIR, FixtureParamIR,
    SourceLocation, SuiteIR,
};
use miette::{miette, Result};
use poly_bench_dsl::{
    BenchMode, Benchmark, ChartDirective, ExecutionOrder, File, Fixture, Lang, RunMode, Suite,
};
use std::{collections::HashSet, path::Path};

/// Lower an AST File to BenchmarkIR
pub fn lower(ast: &File, base_dir: Option<&Path>) -> Result<BenchmarkIR> {
    // Collect stdlib imports from use statements
    let stdlib_imports: HashSet<String> = ast.use_stds.iter().map(|u| u.module.clone()).collect();

    // Lower globalSetup -> anvil_config
    // First check file-level globalSetup, then check suite-level globalSetup
    let mut anvil_config = ast
        .global_setup
        .as_ref()
        .and_then(|gs| gs.anvil_config.as_ref())
        .map(|cfg| AnvilConfigIR::new(cfg.fork_url.clone(), cfg.use_proxy));

    // If no file-level globalSetup, check suite-level globalSetup
    if anvil_config.is_none() {
        for suite in &ast.suites {
            if let Some(ref gs) = suite.global_setup {
                if let Some(ref cfg) = gs.anvil_config {
                    anvil_config = Some(AnvilConfigIR::new(cfg.fork_url.clone(), cfg.use_proxy));
                    break;
                }
            }
        }
    }

    let mut suites = Vec::new();
    let mut chart_directives = Vec::new();

    for suite in &ast.suites {
        let suite_ir = lower_suite(suite, base_dir, &stdlib_imports)?;

        // Lower chart directives from suite, associating them with the suite name
        for directive in &suite.chart_directives {
            let directive_ir = lower_chart_directive(directive, Some(&suite.name));
            chart_directives.push(directive_ir);
        }

        suites.push(suite_ir);
    }

    Ok(BenchmarkIR::with_charts(stdlib_imports, anvil_config, suites, chart_directives))
}

/// Lower a single Suite to SuiteIR
fn lower_suite(
    suite: &Suite,
    base_dir: Option<&Path>,
    stdlib_imports: &HashSet<String>,
) -> Result<SuiteIR> {
    let mut ir = SuiteIR::new(suite.name.clone());

    ir.description = suite.description.clone();
    ir.default_iterations = suite.iterations.unwrap_or(1000);
    ir.default_warmup = suite.warmup.unwrap_or(1000); // Increased from 100 for better JIT optimization

    // Phase 4: Suite-level configuration
    ir.timeout = suite.timeout;
    ir.requires = suite.requires.clone();
    ir.order = suite.order.unwrap_or(ExecutionOrder::Sequential);
    ir.baseline = suite.baseline;
    ir.suite_type = suite.suite_type.ok_or_else(|| {
        miette!("Suite '{}' is missing required property 'suiteType'", suite.name)
    })?;
    ir.run_mode = suite
        .run_mode
        .ok_or_else(|| miette!("Suite '{}' is missing required property 'runMode'", suite.name))?;
    ir.same_dataset = suite.same_dataset.ok_or_else(|| {
        miette!("Suite '{}' is missing required property 'sameDataset'", suite.name)
    })?;

    // Benchmark accuracy settings
    if suite.mode.is_some() {
        return Err(miette!(
            "Suite '{}' uses legacy 'mode'. Use suite declaration run mode: timeBased | iterationBased",
            suite.name
        ));
    }
    ir.mode = match ir.run_mode {
        RunMode::Time => BenchMode::Auto,
        RunMode::Iterations => BenchMode::Fixed,
    };
    ir.target_time_ms = suite.target_time_ms.unwrap_or(3000); // 3 seconds
    ir.sink = suite.sink; // Already defaults to true in AST

    // Statistical analysis settings
    ir.outlier_detection = suite.outlier_detection; // Already defaults to true in AST
    ir.cv_threshold = suite.cv_threshold.unwrap_or(crate::DEFAULT_CV_THRESHOLD);
    ir.count = suite.count.unwrap_or(1); // Default: single run (backward compatible)
    ir.fairness_mode = suite.fairness_mode.unwrap_or_default();
    ir.fairness_seed = suite.fairness_seed;

    // Observability settings (Phase 2B)
    ir.async_sampling_policy = suite.async_sampling_policy.unwrap_or_default();
    ir.async_warmup_cap = suite.async_warmup_cap.unwrap_or(5);
    ir.async_sample_cap = suite.async_sample_cap.unwrap_or(50);

    // Copy stdlib imports to suite
    ir.stdlib_imports = stdlib_imports.clone();

    // Phase 1: Extract structured setup sections
    for (lang, structured_setup) in &suite.setups {
        // Handle imports - extract from import section or parse from code
        if let Some(ref import_block) = structured_setup.imports {
            // For structured setups, imports are already in the import block
            let parsed = match lang {
                Lang::Go => extract_go_imports(&import_block.code),
                Lang::TypeScript => extract_ts_imports(&import_block.code),
                Lang::Rust => extract_rust_imports(&import_block.code),
                _ => ParsedSetup::passthrough(&import_block.code),
            };
            ir.imports.insert(*lang, parsed.imports);
            // Preserve source location
            ir.imports_source
                .insert(*lang, SourceLocation::new(import_block.span.line, import_block.span.col));
        }

        // Handle declarations
        if let Some(ref decl_block) = structured_setup.declarations {
            ir.declarations.insert(*lang, decl_block.code.clone());
            ir.declarations_source
                .insert(*lang, SourceLocation::new(decl_block.span.line, decl_block.span.col));
        }

        // Handle init code
        if let Some(ref init_block) = structured_setup.init {
            ir.init_code.insert(*lang, init_block.code.clone());
            ir.async_init.insert(*lang, structured_setup.async_init);
            ir.init_source
                .insert(*lang, SourceLocation::new(init_block.span.line, init_block.span.col));
        }

        // Handle helpers
        if let Some(ref helpers_block) = structured_setup.helpers {
            ir.helpers.insert(*lang, helpers_block.code.clone());
            ir.helpers_source.insert(
                *lang,
                SourceLocation::new(helpers_block.span.line, helpers_block.span.col),
            );
        }
    }

    // Lower fixtures
    let fixture_names: Vec<String> = suite.fixtures.iter().map(|f| f.name.clone()).collect();

    for fixture in &suite.fixtures {
        let fixture_ir = lower_fixture(fixture, base_dir)?;
        ir.fixtures.push(fixture_ir);
    }

    // Lower benchmarks
    for benchmark in &suite.benchmarks {
        let bench_ir = lower_benchmark(benchmark, &suite.name, &ir, &fixture_names)?;
        ir.benchmarks.push(bench_ir);
    }

    Ok(ir)
}

/// Lower a Fixture to FixtureIR
fn lower_fixture(fixture: &Fixture, base_dir: Option<&Path>) -> Result<FixtureIR> {
    // Resolve the fixture data
    let data = if let Some(ref data) = fixture.data {
        decode_fixture_source(
            data,
            fixture.encoding.as_deref().unwrap_or("utf8"),
            fixture.format.as_deref(),
            fixture.selector.as_deref(),
            &fixture.name,
        )?
    } else if let Some(ref file_path) = fixture.data_file {
        load_fixture_file(
            Path::new(file_path),
            base_dir,
            fixture.encoding.as_deref().unwrap_or("utf8"),
            fixture.format.as_deref(),
            fixture.selector.as_deref(),
            &fixture.name,
        )?
    } else if let Some(ref hex) = fixture.hex_data {
        decode_hex(hex)?
    } else if let Some(ref file_path) = fixture.hex_file {
        load_hex_file(Path::new(file_path), base_dir)?
    } else if !fixture.implementations.is_empty() {
        // Has language-specific implementations but no portable hex
        // Use empty data and rely on implementations
        Vec::new()
    } else if !fixture.params.is_empty() {
        // Parameterized fixture - no static data
        Vec::new()
    } else {
        return Err(miette!("Fixture '{}' has no hex data or implementations", fixture.name));
    };

    let mut ir = FixtureIR::new(fixture.name.clone(), data);
    ir.description = fixture.description.clone();
    ir.shape = fixture.shape.clone();

    // Copy parameters for parameterized fixtures
    ir.params = fixture
        .params
        .iter()
        .map(|p| FixtureParamIR { name: p.name.clone(), param_type: p.param_type.clone() })
        .collect();

    // Copy language-specific implementations
    for (lang, code_block) in &fixture.implementations {
        ir.implementations.insert(*lang, code_block.code.clone());
    }

    Ok(ir)
}

fn decode_fixture_source(
    source: &str,
    encoding: &str,
    format: Option<&str>,
    selector: Option<&str>,
    fixture_name: &str,
) -> Result<Vec<u8>> {
    if let Some(fmt) = format {
        return match fmt.trim().to_ascii_lowercase().as_str() {
            "json" => normalize_json_to_bytes(source, selector)
                .map_err(|e| miette!("Fixture '{}' JSON decode failed: {}", fixture_name, e)),
            "csv" => normalize_csv_to_bytes(source, selector)
                .map_err(|e| miette!("Fixture '{}' CSV decode failed: {}", fixture_name, e)),
            other => Err(miette!(
                "Fixture '{}' has unsupported format '{}'; expected json|csv",
                fixture_name,
                other
            )),
        };
    }

    match encoding.trim().to_ascii_lowercase().as_str() {
        "hex" => decode_hex(source)
            .map_err(|e| miette!("Fixture '{}' hex decode failed: {}", fixture_name, e)),
        "raw" => decode_raw(source)
            .map_err(|e| miette!("Fixture '{}' raw decode failed: {}", fixture_name, e)),
        "utf8" => decode_utf8(source)
            .map_err(|e| miette!("Fixture '{}' utf8 decode failed: {}", fixture_name, e)),
        "base64" => decode_base64(source)
            .map_err(|e| miette!("Fixture '{}' base64 decode failed: {}", fixture_name, e)),
        other => Err(miette!(
            "Fixture '{}' has unsupported encoding '{}'; expected hex|raw|utf8|base64",
            fixture_name,
            other
        )),
    }
}

fn load_fixture_file(
    path: &Path,
    base_dir: Option<&Path>,
    encoding: &str,
    format: Option<&str>,
    selector: Option<&str>,
    fixture_name: &str,
) -> Result<Vec<u8>> {
    if let Some(fmt) = format {
        let text = std::fs::read_to_string(if path.is_absolute() {
            path.to_path_buf()
        } else if let Some(base) = base_dir {
            base.join(path)
        } else {
            path.to_path_buf()
        })
        .map_err(|e| miette!("Failed to read fixture file {}: {}", path.display(), e))?;
        return decode_fixture_source(&text, encoding, Some(fmt), selector, fixture_name);
    }

    match encoding.trim().to_ascii_lowercase().as_str() {
        "hex" => load_hex_file(path, base_dir)
            .map_err(|e| miette!("Fixture '{}' file hex decode failed: {}", fixture_name, e)),
        "raw" => load_raw_file(path, base_dir)
            .map_err(|e| miette!("Fixture '{}' file raw decode failed: {}", fixture_name, e)),
        "utf8" => load_utf8_file(path, base_dir)
            .map_err(|e| miette!("Fixture '{}' file utf8 decode failed: {}", fixture_name, e)),
        "base64" => load_base64_file(path, base_dir)
            .map_err(|e| miette!("Fixture '{}' file base64 decode failed: {}", fixture_name, e)),
        other => Err(miette!(
            "Fixture '{}' has unsupported encoding '{}'; expected hex|raw|utf8|base64",
            fixture_name,
            other
        )),
    }
}

/// Lower a Benchmark to BenchmarkSpec
fn lower_benchmark(
    benchmark: &Benchmark,
    suite_name: &str,
    suite_ir: &SuiteIR,
    fixture_names: &[String],
) -> Result<BenchmarkSpec> {
    let iterations = benchmark.iterations.unwrap_or(suite_ir.default_iterations);
    let warmup = benchmark.warmup.unwrap_or(suite_ir.default_warmup);

    let mut spec = BenchmarkSpec::new(benchmark.name.clone(), suite_name, iterations, warmup);
    spec.kind = benchmark.kind;

    spec.description = benchmark.description.clone();

    // Phase 2: Benchmark configuration
    spec.timeout = benchmark.timeout;
    spec.tags = benchmark.tags.clone();

    // Benchmark accuracy settings (benchmark overrides suite, or inherit from suite)
    if benchmark.mode.is_some() {
        return Err(miette!(
            "Benchmark '{}' in suite '{}' uses legacy 'mode'. Configure suite-level 'runMode' instead",
            benchmark.name,
            suite_name
        ));
    }
    spec.mode = suite_ir.mode;
    spec.target_time_ms = benchmark.target_time_ms.unwrap_or(suite_ir.target_time_ms);
    spec.use_sink = if benchmark.kind == poly_bench_dsl::BenchmarkKind::Async {
        true
    } else {
        benchmark.sink.unwrap_or(suite_ir.sink)
    };
    spec.outlier_detection = benchmark.outlier_detection.unwrap_or(suite_ir.outlier_detection);
    spec.cv_threshold = benchmark.cv_threshold.unwrap_or(suite_ir.cv_threshold);
    spec.count = benchmark.count.unwrap_or(suite_ir.count);
    spec.fairness_mode = suite_ir.fairness_mode;
    spec.fairness_seed = suite_ir.fairness_seed;

    // Observability settings (Phase 2B)
    // Memory tracking: derived from suiteType (memory = enabled, performance = disabled)
    spec.memory = suite_ir.suite_type == poly_bench_dsl::SuiteType::Memory;
    spec.async_sampling_policy = suite_ir.async_sampling_policy;
    spec.async_warmup_cap = suite_ir.async_warmup_cap;
    spec.async_sample_cap = suite_ir.async_sample_cap;

    // Copy skip conditions
    for (lang, code_block) in &benchmark.skip {
        spec.skip_conditions.insert(*lang, code_block.code.clone());
    }

    // Copy validations
    for (lang, code_block) in &benchmark.validate {
        spec.validations.insert(*lang, code_block.code.clone());
    }

    // Phase 3: Lifecycle hooks
    for (lang, code_block) in &benchmark.before {
        spec.before_hooks.insert(*lang, code_block.code.clone());
    }

    for (lang, code_block) in &benchmark.after {
        spec.after_hooks.insert(*lang, code_block.code.clone());
    }

    for (lang, code_block) in &benchmark.each {
        spec.each_hooks.insert(*lang, code_block.code.clone());
    }

    // Copy implementations and extract fixture references
    for (lang, code_block) in &benchmark.implementations {
        spec.implementations.insert(*lang, code_block.code.clone());

        // Extract fixture references from the code
        let refs = extract_fixture_refs(&code_block.code, fixture_names);
        for r in refs {
            if !spec.fixture_refs.contains(&r) {
                spec.fixture_refs.push(r);
            }
        }
    }

    Ok(spec)
}

/// Lower a ChartDirective to ChartDirectiveIR
fn lower_chart_directive(directive: &ChartDirective, suite_name: Option<&str>) -> ChartDirectiveIR {
    let output_file = directive.get_output_file();

    let mut ir = ChartDirectiveIR::new(directive.chart_type, output_file);
    ir.title = directive.title.clone();
    ir.description = directive.description.clone();
    ir.suite_name = suite_name.map(|s| s.to_string());

    // Filtering
    ir.min_speedup = directive.min_speedup;
    ir.filter_winner = directive.filter_winner.clone();
    ir.include_benchmarks = directive.include_benchmarks.clone();
    ir.exclude_benchmarks = directive.exclude_benchmarks.clone();
    ir.limit = directive.limit;

    // Sorting
    ir.sort_by = directive.sort_by.clone();
    ir.sort_order = directive.sort_order.clone();

    // Layout
    ir.width = directive.width;
    ir.row_count = directive.row_count;

    // Dimensions
    ir.height = directive.height;
    ir.baseline_benchmark = directive.baseline_benchmark.clone();

    // Theme
    ir.theme = directive.theme.clone();
    ir.show_std_dev = directive.show_std_dev;
    ir.show_error_bars = directive.show_error_bars;
    ir.show_regression = directive.show_regression;
    ir.regression_model = directive.regression_model.clone();
    ir.y_scale = directive.y_scale.clone();

    ir
}

#[cfg(test)]
mod tests {
    use super::*;
    use poly_bench_dsl::parse;

    #[test]
    fn test_lower_simple() {
        let source = r#"
declare suite hash performance iterationBased sameDataset: false {
    iterations: 5000
    
    fixture data {
        hex: "deadbeef"
    }
    
    bench keccak256 {
        go: hash.Keccak256(data)
        ts: keccak256(data)
    }
}
"#;
        let ast = parse(source, "test.bench").unwrap();
        let ir = lower(&ast, None).unwrap();

        assert_eq!(ir.suites.len(), 1);

        let suite = &ir.suites[0];
        assert_eq!(suite.name, "hash");
        assert_eq!(suite.default_iterations, 5000);
        assert_eq!(suite.fixtures.len(), 1);
        assert_eq!(suite.benchmarks.len(), 1);

        let fixture = &suite.fixtures[0];
        assert_eq!(fixture.name, "data");
        assert_eq!(fixture.data, vec![0xde, 0xad, 0xbe, 0xef]);

        let bench = &suite.benchmarks[0];
        assert_eq!(bench.name, "keccak256");
        assert_eq!(bench.kind, poly_bench_dsl::BenchmarkKind::Sync);
        assert_eq!(bench.full_name, "hash_keccak256");
        assert_eq!(bench.iterations, 5000);
        assert!(bench.has_lang(Lang::Go));
        assert!(bench.has_lang(Lang::TypeScript));
    }

    #[test]
    fn test_lower_bench_async_kind() {
        let source = r#"
declare suite async_suite performance timeBased sameDataset: false {
    benchAsync rpc_call {
        ts: await getBlockNumber()
    }
}
"#;
        let ast = parse(source, "test.bench").unwrap();
        let ir = lower(&ast, None).unwrap();
        let bench = &ir.suites[0].benchmarks[0];
        assert_eq!(bench.kind, poly_bench_dsl::BenchmarkKind::Async);
    }

    #[test]
    fn test_lower_with_stdlib_imports() {
        let source = r#"
use std::constants

declare suite math performance iterationBased sameDataset: false {
    iterations: 100
    
    bench pi_calc {
        go: compute(std_PI)
        ts: compute(std_PI)
    }
}
"#;
        let ast = parse(source, "test.bench").unwrap();
        let ir = lower(&ast, None).unwrap();

        assert!(ir.has_stdlib("constants"));
        assert!(!ir.has_stdlib("nonexistent"));
        assert_eq!(ir.stdlib_imports.len(), 1);
    }

    #[test]
    fn test_lower_with_multiple_stdlib_imports() {
        let source = r#"
use std::constants
use std::math

declare suite test performance timeBased sameDataset: false {
    bench foo {
        go: test()
    }
}
"#;
        let ast = parse(source, "test.bench").unwrap();
        let ir = lower(&ast, None).unwrap();

        assert_eq!(ir.stdlib_imports.len(), 2);
        assert!(ir.has_stdlib("constants"));
        assert!(ir.has_stdlib("math"));
    }

    #[test]
    fn test_lower_suite_level_global_setup() {
        let source = r#"
use std::anvil

declare suite evm performance timeBased sameDataset: false {
    globalSetup {
        anvil.spawnAnvil()
    }
    
    bench test {
        go: test()
    }
}
"#;
        let ast = parse(source, "test.bench").unwrap();
        let ir = lower(&ast, None).unwrap();

        // Suite-level globalSetup should be captured as anvil_config
        assert!(
            ir.anvil_config.is_some(),
            "anvil_config should be set from suite-level globalSetup"
        );
        let anvil_cfg = ir.anvil_config.as_ref().unwrap();
        assert!(anvil_cfg.fork_url.is_none(), "spawnAnvil() without args should have no fork_url");
        assert!(!anvil_cfg.use_proxy, "tokio proxy should default to off");
    }

    #[test]
    fn test_lower_without_stdlib() {
        let source = r#"
declare suite test performance timeBased sameDataset: false {
    fixture data {
        hex: "deadbeef"
    }
    
    bench foo {
        go: test(data)
    }
}
"#;
        let ast = parse(source, "test.bench").unwrap();
        let ir = lower(&ast, None).unwrap();

        assert!(ir.stdlib_imports.is_empty());
    }

    #[test]
    fn test_lower_with_fairness_and_async_policy() {
        let source = r#"
declare suite fairnessTest performance timeBased sameDataset: false {
    fairness: "strict"
    fairnessSeed: 42
    asyncSamplingPolicy: "timeBudgeted"
    asyncWarmupCap: 7
    asyncSampleCap: 88

    benchAsync call {
        ts: fetchValue()
    }
}
"#;
        let ast = parse(source, "test.bench").unwrap();
        let ir = lower(&ast, None).unwrap();
        let suite = &ir.suites[0];
        let bench = &suite.benchmarks[0];

        assert_eq!(suite.fairness_seed, Some(42));
        assert_eq!(suite.async_warmup_cap, 7);
        assert_eq!(suite.async_sample_cap, 88);
        assert_eq!(bench.fairness_seed, Some(42));
        assert_eq!(bench.async_warmup_cap, 7);
        assert_eq!(bench.async_sample_cap, 88);
    }

    #[test]
    fn test_lower_fixture_raw_data() {
        let source = r#"
declare suite test performance timeBased sameDataset: false {
    fixture payload {
        data: "hello"
        encoding: raw
    }

    bench foo {
        ts: run(payload)
    }
}
"#;
        let ast = parse(source, "test.bench").unwrap();
        let ir = lower(&ast, None).unwrap();
        assert_eq!(ir.suites[0].fixtures[0].data, b"hello");
    }

    #[test]
    fn test_lower_fixture_base64_data() {
        let source = r#"
declare suite test performance timeBased sameDataset: false {
    fixture payload {
        data: "aGVsbG8="
        encoding: base64
    }

    bench foo {
        ts: run(payload)
    }
}
"#;
        let ast = parse(source, "test.bench").unwrap();
        let ir = lower(&ast, None).unwrap();
        assert_eq!(ir.suites[0].fixtures[0].data, b"hello");
    }

    #[test]
    fn test_lower_fixture_json_selector() {
        let source = r#"
declare suite test performance timeBased sameDataset: false {
    fixture payload {
        data: "{\"items\":[{\"id\":\"abc\"},{\"id\":\"xyz\"}]}"
        format: json
        selector: "$.items[1].id"
    }

    bench foo {
        ts: run(payload)
    }
}
"#;
        let ast = parse(source, "test.bench").unwrap();
        let ir = lower(&ast, None).unwrap();
        assert_eq!(ir.suites[0].fixtures[0].data, b"xyz");
    }

    #[test]
    fn test_lower_chart_stats_flags() {
        let source = r#"
use std::charting

declare suite chartSuite performance timeBased sameDataset: true {
    targetTime: 2s
    bench n10 {
        go: run()
        ts: run()
    }
    after {
        charting.drawLineChart(
            title: "Trend",
            showStdDev: false,
            showErrorBars: true,
            showRegression: false,
            regressionModel: "linear"
        )
    }
}
"#;
        let ast = parse(source, "test.bench").unwrap();
        let ir = lower(&ast, None).unwrap();
        assert_eq!(ir.chart_directives.len(), 1);
        let chart = &ir.chart_directives[0];
        assert_eq!(chart.chart_type, poly_bench_dsl::ChartType::LineChart);
        assert!(!chart.show_std_dev);
        assert!(chart.show_error_bars);
        assert!(!chart.show_regression);
        assert_eq!(chart.regression_model, "linear");
        assert_eq!(chart.y_scale, "linear");
    }

    #[test]
    fn test_lower_chart_y_scale_round_trip() {
        let source = r#"
use std::charting

declare suite chartSuite performance timeBased sameDataset: true {
    targetTime: 2s
    bench n10 {
        go: run()
        ts: run()
    }
    after {
        charting.drawLineChart(
            title: "Trend",
            description: "desc",
            yScale: "log10"
        )
    }
}
"#;
        let ast = parse(source, "test.bench").unwrap();
        let ir = lower(&ast, None).unwrap();
        assert_eq!(ir.chart_directives.len(), 1);
        assert_eq!(ir.chart_directives[0].y_scale, "log10");
    }
}
