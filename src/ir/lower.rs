//! AST to IR lowering
//!
//! Transforms the parsed AST into a normalized IR suitable for execution.

use crate::dsl::{File, Suite, Fixture, Benchmark, Lang, ExecutionOrder};
use crate::ir::{BenchmarkIR, SuiteIR, FixtureIR, FixtureParamIR, BenchmarkSpec, AnvilConfigIR};
use crate::ir::fixtures::{decode_hex, load_hex_file, extract_fixture_refs};
use crate::ir::imports::{extract_go_imports, extract_ts_imports, ParsedSetup};
use miette::{Result, miette};
use std::collections::HashSet;
use std::path::Path;

/// Lower an AST File to BenchmarkIR
pub fn lower(ast: &File, base_dir: Option<&Path>) -> Result<BenchmarkIR> {
    // Collect stdlib imports from use statements
    let stdlib_imports: HashSet<String> = ast.use_stds
        .iter()
        .map(|u| u.module.clone())
        .collect();

    // Lower globalSetup -> anvil_config
    // First check file-level globalSetup, then check suite-level globalSetup
    let mut anvil_config = ast.global_setup.as_ref()
        .and_then(|gs| gs.anvil_config.as_ref())
        .map(|cfg| AnvilConfigIR::new(cfg.fork_url.clone()));
    
    // If no file-level globalSetup, check suite-level globalSetup
    if anvil_config.is_none() {
        for suite in &ast.suites {
            if let Some(ref gs) = suite.global_setup {
                if let Some(ref cfg) = gs.anvil_config {
                    anvil_config = Some(AnvilConfigIR::new(cfg.fork_url.clone()));
                    break;
                }
            }
        }
    }

    let mut suites = Vec::new();

    for suite in &ast.suites {
        let suite_ir = lower_suite(suite, base_dir, &stdlib_imports)?;
        suites.push(suite_ir);
    }

    Ok(BenchmarkIR::with_anvil(stdlib_imports, anvil_config, suites))
}

/// Lower a single Suite to SuiteIR
fn lower_suite(suite: &Suite, base_dir: Option<&Path>, stdlib_imports: &HashSet<String>) -> Result<SuiteIR> {
    let mut ir = SuiteIR::new(suite.name.clone());
    
    ir.description = suite.description.clone();
    ir.default_iterations = suite.iterations.unwrap_or(1000);
    ir.default_warmup = suite.warmup.unwrap_or(100);
    
    // Phase 4: Suite-level configuration
    ir.timeout = suite.timeout;
    ir.requires = suite.requires.clone();
    ir.order = suite.order.unwrap_or(ExecutionOrder::Sequential);
    ir.compare = suite.compare;
    ir.baseline = suite.baseline;
    
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
                _ => ParsedSetup::passthrough(&import_block.code),
            };
            ir.imports.insert(*lang, parsed.imports);
        }
        
        // Handle declarations
        if let Some(ref decl_block) = structured_setup.declarations {
            ir.declarations.insert(*lang, decl_block.code.clone());
        }
        
        // Handle init code
        if let Some(ref init_block) = structured_setup.init {
            ir.init_code.insert(*lang, init_block.code.clone());
            ir.async_init.insert(*lang, structured_setup.async_init);
        }
        
        // Handle helpers
        if let Some(ref helpers_block) = structured_setup.helpers {
            ir.helpers.insert(*lang, helpers_block.code.clone());
        }
    }

    // Lower fixtures
    let fixture_names: Vec<String> = suite.fixtures.iter()
        .map(|f| f.name.clone())
        .collect();

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
    let data = if let Some(ref hex) = fixture.hex_data {
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
        return Err(miette!(
            "Fixture '{}' has no hex data or implementations",
            fixture.name
        ));
    };

    let mut ir = FixtureIR::new(fixture.name.clone(), data);
    ir.description = fixture.description.clone();
    ir.shape = fixture.shape.clone();
    
    // Copy parameters for parameterized fixtures
    ir.params = fixture.params.iter()
        .map(|p| FixtureParamIR {
            name: p.name.clone(),
            param_type: p.param_type.clone(),
        })
        .collect();

    // Copy language-specific implementations
    for (lang, code_block) in &fixture.implementations {
        ir.implementations.insert(*lang, code_block.code.clone());
    }

    Ok(ir)
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

    let mut spec = BenchmarkSpec::new(
        benchmark.name.clone(),
        suite_name,
        iterations,
        warmup,
    );

    spec.description = benchmark.description.clone();
    
    // Phase 2: Benchmark configuration
    spec.timeout = benchmark.timeout;
    spec.tags = benchmark.tags.clone();
    
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::parse;

    #[test]
    fn test_lower_simple() {
        let source = r#"
suite hash {
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
        assert_eq!(bench.full_name, "hash_keccak256");
        assert_eq!(bench.iterations, 5000);
        assert!(bench.has_lang(Lang::Go));
        assert!(bench.has_lang(Lang::TypeScript));
    }

    #[test]
    fn test_lower_with_stdlib_imports() {
        let source = r#"
use std::constants

suite math {
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

suite test {
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

suite evm {
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
        assert!(ir.anvil_config.is_some(), "anvil_config should be set from suite-level globalSetup");
        let anvil_cfg = ir.anvil_config.as_ref().unwrap();
        assert!(anvil_cfg.fork_url.is_none(), "spawnAnvil() without args should have no fork_url");
    }

    #[test]
    fn test_lower_without_stdlib() {
        let source = r#"
suite test {
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
}
