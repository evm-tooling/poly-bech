//! Semantic validation for poly-bench DSL
//!
//! This module provides core validation logic that runs after parsing
//! to catch semantic errors and generate warnings.
//!
//! Note: Stdlib-specific validation (e.g., validating use std::module names)
//! is handled by higher-level crates that depend on both dsl and stdlib.

use crate::{Benchmark, CodeBlock, File, Lang, RunMode, StructuredSetup, Suite, UseStd};
use std::collections::HashSet;

/// A validation warning (non-fatal issue)
#[derive(Debug, Clone)]
pub struct ValidationWarning {
    pub message: String,
    pub location: Option<String>,
}

impl ValidationWarning {
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into(), location: None }
    }

    pub fn with_location(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }
}

/// A validation error (fatal issue)
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub message: String,
    pub location: Option<String>,
}

impl ValidationError {
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into(), location: None }
    }

    pub fn with_location(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }
}

/// Result of validating a suite
#[derive(Debug, Default)]
pub struct ValidationResult {
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    pub fn add_error(&mut self, error: ValidationError) {
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: ValidationWarning) {
        self.warnings.push(warning);
    }

    /// Merge another result into this one
    pub fn merge(&mut self, other: ValidationResult) {
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
    }
}

/// Validate a complete file and return any errors or warnings.
///
/// Note: This performs core DSL validation only. Stdlib module validation
/// should be done by crates that have access to the stdlib definitions.
pub fn validate_file(file: &File) -> ValidationResult {
    let mut result = ValidationResult::new();

    // Check for duplicate use statements
    validate_use_stds_duplicates(&file.use_stds, &mut result);

    // Validate charting imports
    let charting_result = validate_charting_imports(file);
    result.merge(charting_result);

    // Validate each suite
    for suite in &file.suites {
        let suite_result = validate_suite(suite);
        result.merge(suite_result);
    }

    result
}

/// Check for duplicate use statements (core validation, no stdlib check)
fn validate_use_stds_duplicates(use_stds: &[UseStd], result: &mut ValidationResult) {
    let mut seen = HashSet::new();
    for use_std in use_stds {
        if !seen.insert(&use_std.module) {
            result.add_warning(
                ValidationWarning::new(format!("Duplicate import of std::{}", use_std.module))
                    .with_location(format!("line {}", use_std.span.line)),
            );
        }
    }
}

/// Validate a suite and return any errors or warnings
pub fn validate_suite(suite: &Suite) -> ValidationResult {
    let mut result = ValidationResult::new();

    // Validate suite-level semantic contract
    validate_suite_semantics(suite, &mut result);

    // Validate: all benchmarks have required languages
    validate_requires(suite, &mut result);

    // Validate: setup sections are properly defined
    validate_setups(suite, &mut result);

    // Validate: benchmarks are valid
    for benchmark in &suite.benchmarks {
        validate_benchmark(benchmark, suite, &mut result);
    }

    // Validate: fixtures referenced in benchmarks exist
    validate_fixture_references(suite, &mut result);

    // Validate: baseline language is valid
    validate_baseline(suite, &mut result);

    // Validate: spawnAnvil only allowed in globalSetup
    validate_spawn_anvil_restrictions(suite, &mut result);

    result
}

/// Validate required suite-level semantics and run-mode dependent property constraints
fn validate_suite_semantics(suite: &Suite, result: &mut ValidationResult) {
    let suite_location = format!("suite.{}", suite.name);

    if suite.suite_type.is_none() {
        result.add_error(
            ValidationError::new(format!(
                "Suite '{}' is missing required property 'suiteType' (expected \"memory\" or \"performance\")",
                suite.name
            ))
            .with_location(suite_location.clone()),
        );
    }

    if suite.run_mode.is_none() {
        result.add_error(
            ValidationError::new(format!(
                "Suite '{}' is missing required run mode in suite declaration (expected \"timeBased\" or \"iterationBased\")",
                suite.name
            ))
            .with_location(suite_location.clone()),
        );
    }

    if suite.same_dataset.is_none() {
        result.add_error(
            ValidationError::new(format!(
                "Suite '{}' is missing required property 'sameDataset' (expected true or false)",
                suite.name
            ))
            .with_location(suite_location.clone()),
        );
    }

    if suite.mode.is_some() {
        result.add_error(
            ValidationError::new(
                "Property 'mode' is no longer supported. Use suite declaration run mode: timeBased | iterationBased",
            )
            .with_location(suite_location.clone()),
        );
    }

    if let Some(run_mode) = suite.run_mode {
        match run_mode {
            RunMode::Time => {
                if suite.iterations.is_some() {
                    result.add_error(
                        ValidationError::new(
                            "Property 'iterations' is invalid when run mode is timeBased. Use 'targetTime' instead",
                        )
                        .with_location(suite_location.clone()),
                    );
                }
            }
            RunMode::Iterations => {
                if suite.target_time_ms.is_some() {
                    result.add_error(
                        ValidationError::new(
                            "Property 'targetTime' is invalid when run mode is iterationBased. Use 'iterations' instead",
                        )
                        .with_location(suite_location.clone()),
                    );
                }
            }
        }
    }
}

/// Validate charting directives: check that std::charting is imported when charting is used
pub fn validate_charting_imports(file: &File) -> ValidationResult {
    let mut result = ValidationResult::new();

    // Check if charting module is imported
    let has_charting_import = file.use_stds.iter().any(|u| u.module == "charting");

    // Check each suite for chart directives
    for suite in &file.suites {
        if !suite.chart_directives.is_empty() && !has_charting_import {
            result.add_error(
                ValidationError::new(format!(
                    "Suite '{}' uses charting functions but 'use std::charting' is missing",
                    suite.name
                ))
                .with_location(format!("suite.{}", suite.name)),
            );
        }
    }

    result
}

/// Validate that all benchmarks have required language implementations
fn validate_requires(suite: &Suite, result: &mut ValidationResult) {
    if suite.requires.is_empty() {
        return;
    }

    for benchmark in &suite.benchmarks {
        for lang in &suite.requires {
            if !benchmark.implementations.contains_key(lang) {
                result.add_error(
                    ValidationError::new(format!(
                        "Benchmark '{}' missing required language implementation '{}'",
                        benchmark.name, lang
                    ))
                    .with_location(format!("suite.{}.bench.{}", suite.name, benchmark.name)),
                );
            }
        }
    }
}

/// Validate setup sections
fn validate_setups(suite: &Suite, result: &mut ValidationResult) {
    for (lang, setup) in &suite.setups {
        validate_structured_setup(*lang, setup, &suite.name, result);
    }
}

/// Validate a structured setup block
fn validate_structured_setup(
    lang: Lang,
    setup: &StructuredSetup,
    suite_name: &str,
    result: &mut ValidationResult,
) {
    let location = format!("suite.{}.setup.{}", suite_name, lang);

    // Warning: Go setup should have init section
    if lang == Lang::Go && setup.init.is_none() && !setup.is_empty() {
        result.add_warning(
            ValidationWarning::new(
                "Go setup has no init section - consider adding one for initialization code",
            )
            .with_location(location.clone()),
        );
    }

    // Warning: Rust setup should have init section
    if lang == Lang::Rust && setup.init.is_none() && !setup.is_empty() {
        result.add_warning(
            ValidationWarning::new(
                "Rust setup has no init section - consider adding one for initialization code",
            )
            .with_location(location.clone()),
        );
    }

    // Warning: TypeScript async init but no init code
    if setup.async_init && setup.init.is_none() {
        result.add_warning(
            ValidationWarning::new("async modifier specified but no init code provided")
                .with_location(location),
        );
    }
}

/// Validate a benchmark
fn validate_benchmark(benchmark: &Benchmark, suite: &Suite, result: &mut ValidationResult) {
    let location = format!("suite.{}.bench.{}", suite.name, benchmark.name);

    // Warning: benchmark has no implementations
    if benchmark.implementations.is_empty() {
        result.add_error(
            ValidationError::new(format!(
                "Benchmark '{}' has no language implementations",
                benchmark.name
            ))
            .with_location(location.clone()),
        );
    }

    // Warning: skip condition for non-existent language
    for lang in benchmark.skip.keys() {
        if !benchmark.implementations.contains_key(lang) {
            result.add_warning(
                ValidationWarning::new(format!(
                    "Skip condition for '{}' but no implementation for that language",
                    lang
                ))
                .with_location(location.clone()),
            );
        }
    }

    // Warning: validate for non-existent language
    for lang in benchmark.validate.keys() {
        if !benchmark.implementations.contains_key(lang) {
            result.add_warning(
                ValidationWarning::new(format!(
                    "Validation for '{}' but no implementation for that language",
                    lang
                ))
                .with_location(location.clone()),
            );
        }
    }

    // Warning: hook for non-existent language
    for lang in benchmark.before.keys() {
        if !benchmark.implementations.contains_key(lang) {
            result.add_warning(
                ValidationWarning::new(format!(
                    "Before hook for '{}' but no implementation for that language",
                    lang
                ))
                .with_location(location.clone()),
            );
        }
    }

    for lang in benchmark.after.keys() {
        if !benchmark.implementations.contains_key(lang) {
            result.add_warning(
                ValidationWarning::new(format!(
                    "After hook for '{}' but no implementation for that language",
                    lang
                ))
                .with_location(location.clone()),
            );
        }
    }

    for lang in benchmark.each.keys() {
        if !benchmark.implementations.contains_key(lang) {
            result.add_warning(
                ValidationWarning::new(format!(
                    "Each hook for '{}' but no implementation for that language",
                    lang
                ))
                .with_location(location.clone()),
            );
        }
    }

    if benchmark.mode.is_some() {
        result.add_error(
            ValidationError::new(
                "Benchmark-level 'mode' is no longer supported. Configure suite-level 'runMode' instead",
            )
            .with_location(location.clone()),
        );
    }

    if let Some(run_mode) = suite.run_mode {
        match run_mode {
            RunMode::Time => {
                if benchmark.iterations.is_some() {
                    result.add_error(
                        ValidationError::new(
                            "Benchmark-level 'iterations' is invalid when suite run mode is timeBased",
                        )
                        .with_location(location.clone()),
                    );
                }
            }
            RunMode::Iterations => {
                if benchmark.target_time_ms.is_some() {
                    result.add_error(
                        ValidationError::new(
                            "Benchmark-level 'targetTime' is invalid when suite run mode is iterationBased",
                        )
                        .with_location(location.clone()),
                    );
                }
            }
        }
    }
}

/// Validate that fixtures referenced in benchmarks exist
fn validate_fixture_references(suite: &Suite, result: &mut ValidationResult) {
    // Validate fixture definitions
    for fixture in &suite.fixtures {
        let has_data = fixture.hex_data.is_some() || fixture.hex_file.is_some();
        let has_implementations = !fixture.implementations.is_empty();
        let has_params = !fixture.params.is_empty();

        if !has_data && !has_implementations && !has_params {
            result.add_error(
                ValidationError::new(format!(
                    "Fixture '{}' has no hex data, implementations, or parameters",
                    fixture.name
                ))
                .with_location(format!("suite.{}.fixture.{}", suite.name, fixture.name)),
            );
        }
    }
}

/// Validate baseline language configuration
fn validate_baseline(suite: &Suite, result: &mut ValidationResult) {
    if let Some(baseline) = suite.baseline {
        // Check that at least one benchmark has this language
        let has_baseline_impl =
            suite.benchmarks.iter().any(|b| b.implementations.contains_key(&baseline));

        if !has_baseline_impl {
            result.add_warning(
                ValidationWarning::new(format!(
                    "Baseline language '{}' has no implementations in any benchmark",
                    baseline
                ))
                .with_location(format!("suite.{}", suite.name)),
            );
        }
    }
}

/// Validate that spawnAnvil() is only called in globalSetup blocks
fn validate_spawn_anvil_restrictions(suite: &Suite, result: &mut ValidationResult) {
    // Check setup blocks (init, helpers, declarations, imports)
    for (lang, setup) in &suite.setups {
        check_code_block_for_spawn_anvil(
            &setup.init,
            &format!("setup {} init", lang),
            &suite.name,
            result,
        );
        check_code_block_for_spawn_anvil(
            &setup.helpers,
            &format!("setup {} helpers", lang),
            &suite.name,
            result,
        );
        check_code_block_for_spawn_anvil(
            &setup.declarations,
            &format!("setup {} declarations", lang),
            &suite.name,
            result,
        );
        check_code_block_for_spawn_anvil(
            &setup.imports,
            &format!("setup {} imports", lang),
            &suite.name,
            result,
        );
    }

    // Check benchmark blocks
    for benchmark in &suite.benchmarks {
        let bench_loc = format!("bench {}", benchmark.name);

        // Check implementations (go:, ts:)
        for (lang, code) in &benchmark.implementations {
            check_code_block_for_spawn_anvil_ref(
                code,
                &format!("{} {} implementation", bench_loc, lang),
                &suite.name,
                result,
            );
        }

        // Check before hooks
        for (lang, code) in &benchmark.before {
            check_code_block_for_spawn_anvil_ref(
                code,
                &format!("{} {} before hook", bench_loc, lang),
                &suite.name,
                result,
            );
        }

        // Check after hooks
        for (lang, code) in &benchmark.after {
            check_code_block_for_spawn_anvil_ref(
                code,
                &format!("{} {} after hook", bench_loc, lang),
                &suite.name,
                result,
            );
        }

        // Check each hooks
        for (lang, code) in &benchmark.each {
            check_code_block_for_spawn_anvil_ref(
                code,
                &format!("{} {} each hook", bench_loc, lang),
                &suite.name,
                result,
            );
        }

        // Check skip conditions
        for (lang, code) in &benchmark.skip {
            check_code_block_for_spawn_anvil_ref(
                code,
                &format!("{} {} skip condition", bench_loc, lang),
                &suite.name,
                result,
            );
        }

        // Check validate blocks
        for (lang, code) in &benchmark.validate {
            check_code_block_for_spawn_anvil_ref(
                code,
                &format!("{} {} validate block", bench_loc, lang),
                &suite.name,
                result,
            );
        }
    }

    // Check fixture blocks
    for fixture in &suite.fixtures {
        for (lang, code) in &fixture.implementations {
            check_code_block_for_spawn_anvil_ref(
                code,
                &format!("fixture {} {} implementation", fixture.name, lang),
                &suite.name,
                result,
            );
        }
    }
}

/// Check an optional code block for spawnAnvil calls
fn check_code_block_for_spawn_anvil(
    block: &Option<CodeBlock>,
    context: &str,
    suite_name: &str,
    result: &mut ValidationResult,
) {
    if let Some(code) = block {
        check_code_block_for_spawn_anvil_ref(code, context, suite_name, result);
    }
}

/// Check a code block reference for spawnAnvil calls
fn check_code_block_for_spawn_anvil_ref(
    code: &CodeBlock,
    context: &str,
    suite_name: &str,
    result: &mut ValidationResult,
) {
    // Check for both forms: spawnAnvil( and anvil.spawnAnvil(
    if code.code.contains("spawnAnvil(") {
        result.add_error(
            ValidationError::new(format!(
                "spawnAnvil() can only be called in globalSetup blocks, found in {}",
                context
            ))
            .with_location(format!("suite.{}", suite_name)),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    #[test]
    fn test_validate_missing_required_lang() {
        let source = r#"
suite test {
    requires: ["go", "ts"]
    
    setup go {
        import ("fmt")
        init {}
    }
    
    bench only_go {
        go: doSomething()
    }
}
"#;
        let ast = parse(source, "test.bench").unwrap();
        let result = validate_suite(&ast.suites[0]);

        assert!(!result.is_ok());
        assert!(result.errors.iter().any(|e| e.message.contains("missing required language")));
    }

    #[test]
    fn test_validate_empty_benchmark() {
        let source = r#"
suite test {
    bench empty {
    }
}
"#;
        let ast = parse(source, "test.bench").unwrap();
        let result = validate_suite(&ast.suites[0]);

        assert!(!result.is_ok());
        assert!(result.errors.iter().any(|e| e.message.contains("no language implementations")));
    }

    #[test]
    fn test_validate_duplicate_use_stds() {
        let source = r#"
use std::constants
use std::constants

suite test {
    bench test {
        go: compute(std_PI)
    }
}
"#;
        let ast = parse(source, "test.bench").unwrap();
        let result = validate_file(&ast);

        assert!(result.has_warnings());
        assert!(result.warnings.iter().any(|w| w.message.contains("Duplicate import")));
    }

    #[test]
    fn test_validate_requires_suite_semantics() {
        let source = r#"
suite test {
    bench foo {
        go: work()
    }
}
"#;
        let ast = parse(source, "test.bench").unwrap();
        let result = validate_suite(&ast.suites[0]);
        assert!(result.errors.iter().any(|e| e.message.contains("suiteType")));
        assert!(result
            .errors
            .iter()
            .any(|e| e.message.contains("run mode") || e.message.contains("timeBased")));
        assert!(result.errors.iter().any(|e| e.message.contains("sameDataset")));
    }

    #[test]
    fn test_validate_run_mode_property_rules() {
        let source = r#"
declare suite test performance timeBased sameDataset: true {
    iterations: 1000

    bench foo {
        iterations: 10
        go: work()
    }
}
"#;
        let ast = parse(source, "test.bench").unwrap();
        let result = validate_suite(&ast.suites[0]);
        assert!(result.errors.iter().any(|e| e.message.contains("timeBased")));
    }
}
