//! AST to IR lowering
//!
//! Transforms the parsed AST into a normalized IR suitable for execution.

use crate::dsl::{File, Suite, Fixture, Benchmark, Lang};
use crate::ir::{BenchmarkIR, SuiteIR, FixtureIR, BenchmarkSpec};
use crate::ir::fixtures::{decode_hex, load_hex_file, extract_fixture_refs};
use crate::ir::imports::{extract_go_imports, extract_ts_imports, ParsedSetup};
use miette::{Result, miette};
use std::path::Path;

/// Lower an AST File to BenchmarkIR
pub fn lower(ast: &File, base_dir: Option<&Path>) -> Result<BenchmarkIR> {
    let mut suites = Vec::new();

    for suite in &ast.suites {
        let suite_ir = lower_suite(suite, base_dir)?;
        suites.push(suite_ir);
    }

    Ok(BenchmarkIR::new(suites))
}

/// Lower a single Suite to SuiteIR
fn lower_suite(suite: &Suite, base_dir: Option<&Path>) -> Result<SuiteIR> {
    let mut ir = SuiteIR::new(suite.name.clone());
    
    ir.description = suite.description.clone();
    ir.default_iterations = suite.iterations.unwrap_or(1000);
    ir.default_warmup = suite.warmup.unwrap_or(100);

    // Extract imports from setup blocks and separate from body
    for (lang, code_block) in &suite.setups {
        let parsed = match lang {
            Lang::Go => extract_go_imports(&code_block.code),
            Lang::TypeScript => extract_ts_imports(&code_block.code),
            // For other languages, pass through as-is
            _ => ParsedSetup::passthrough(&code_block.code),
        };
        ir.imports.insert(*lang, parsed.imports);
        ir.setups.insert(*lang, parsed.body);
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
    } else {
        return Err(miette!(
            "Fixture '{}' has no hex data or implementations",
            fixture.name
        ));
    };

    let mut ir = FixtureIR::new(fixture.name.clone(), data);
    ir.description = fixture.description.clone();

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
    let warmup = suite_ir.default_warmup;

    let mut spec = BenchmarkSpec::new(
        benchmark.name.clone(),
        suite_name,
        iterations,
        warmup,
    );

    spec.description = benchmark.description.clone();

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
}
