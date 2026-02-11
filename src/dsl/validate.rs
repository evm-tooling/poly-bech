//! Semantic validation for poly-bench DSL
//!
//! This module provides validation logic that runs after parsing
//! to catch semantic errors and generate warnings.

use crate::dsl::{Suite, Benchmark, Lang, StructuredSetup, UseStd, File};
use crate::stdlib;
use std::collections::HashSet;

/// A validation warning (non-fatal issue)
#[derive(Debug, Clone)]
pub struct ValidationWarning {
    pub message: String,
    pub location: Option<String>,
}

impl ValidationWarning {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            location: None,
        }
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
        Self {
            message: message.into(),
            location: None,
        }
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

    fn add_error(&mut self, error: ValidationError) {
        self.errors.push(error);
    }

    fn add_warning(&mut self, warning: ValidationWarning) {
        self.warnings.push(warning);
    }
}

/// Validate a complete file and return any errors or warnings
pub fn validate_file(file: &File) -> ValidationResult {
    let mut result = ValidationResult::new();

    // Validate: use statements reference valid stdlib modules
    validate_use_stds(&file.use_stds, &mut result);

    // Validate each suite
    for suite in &file.suites {
        let suite_result = validate_suite(suite);
        result.errors.extend(suite_result.errors);
        result.warnings.extend(suite_result.warnings);
    }

    result
}

/// Validate use std::module statements
fn validate_use_stds(use_stds: &[UseStd], result: &mut ValidationResult) {
    for use_std in use_stds {
        if !stdlib::is_valid_module(&use_std.module) {
            result.add_error(
                ValidationError::new(format!(
                    "Unknown stdlib module: std::{}. Valid modules are: {}",
                    use_std.module,
                    stdlib::VALID_MODULES.join(", ")
                ))
                .with_location(format!("line {}", use_std.span.line)),
            );
        }
    }

    // Check for duplicate imports
    let mut seen = HashSet::new();
    for use_std in use_stds {
        if !seen.insert(&use_std.module) {
            result.add_warning(
                ValidationWarning::new(format!(
                    "Duplicate import of std::{}",
                    use_std.module
                ))
                .with_location(format!("line {}", use_std.span.line)),
            );
        }
    }
}

/// Validate a suite and return any errors or warnings
pub fn validate_suite(suite: &Suite) -> ValidationResult {
    let mut result = ValidationResult::new();

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
                        benchmark.name,
                        lang
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
            ValidationWarning::new("Go setup has no init section - consider adding one for initialization code")
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
}

/// Validate that fixtures referenced in benchmarks exist
fn validate_fixture_references(suite: &Suite, result: &mut ValidationResult) {
    let fixture_names: HashSet<&str> = suite.fixtures.iter().map(|f| f.name.as_str()).collect();

    for benchmark in &suite.benchmarks {
        for (lang, code_block) in &benchmark.implementations {
            // Simple check: look for fixture names in the code
            for fixture in &suite.fixtures {
                // Skip - we'd need proper AST analysis to do this accurately
                // For now, just validate that fixture definitions are valid
                let _ = (lang, code_block, fixture);
            }
        }
    }

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
        let has_baseline_impl = suite
            .benchmarks
            .iter()
            .any(|b| b.implementations.contains_key(&baseline));

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::parse;

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
    fn test_validate_valid_stdlib_import() {
        let source = r#"
use std::constants

suite test {
    bench pi_test {
        go: compute(std_PI)
    }
}
"#;
        let ast = parse(source, "test.bench").unwrap();
        let result = validate_file(&ast);
        
        // Should not have errors related to stdlib
        assert!(!result.errors.iter().any(|e| e.message.contains("Unknown stdlib module")));
    }

    #[test]
    fn test_validate_invalid_stdlib_import() {
        let source = r#"
use std::nonexistent

suite test {
    bench test {
        go: test()
    }
}
"#;
        let ast = parse(source, "test.bench").unwrap();
        let result = validate_file(&ast);
        
        assert!(!result.is_ok());
        assert!(result.errors.iter().any(|e| e.message.contains("Unknown stdlib module")));
        assert!(result.errors.iter().any(|e| e.message.contains("nonexistent")));
    }

    #[test]
    fn test_validate_duplicate_stdlib_import() {
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
}
