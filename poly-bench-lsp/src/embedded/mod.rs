//! Embedded language support for poly-bench LSP
//!
//! This module provides type checking for Go and TypeScript code
//! embedded within `.bench` files.

pub mod go_bridge;
pub mod rust_bridge;
pub mod ts_bridge;

use poly_bench_dsl::{Benchmark, Fixture, Lang, Span, StructuredSetup, Suite};
use poly_bench_stdlib as stdlib;
use std::collections::HashSet;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Range};

use super::document::ParsedDocument;

/// The type of embedded block (affects how code is wrapped)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    /// Setup section: imports
    SetupImport,
    /// Setup section: declarations
    SetupDeclare,
    /// Setup section: init code
    SetupInit,
    /// Setup section: helper functions
    SetupHelpers,
    /// Fixture implementation
    Fixture,
    /// Benchmark implementation
    Benchmark,
    /// Lifecycle hook (before/after/each)
    Hook,
    /// Skip condition
    Skip,
    /// Validation expression
    Validate,
}

/// An embedded code block extracted from a .bench file
#[derive(Debug, Clone)]
pub struct EmbeddedBlock {
    /// The programming language
    pub lang: Lang,
    /// The type of block (affects wrapping)
    pub block_type: BlockType,
    /// The code content
    pub code: String,
    /// Source span in the .bench file
    pub span: Span,
    /// Name of the containing construct (for error messages)
    pub context_name: String,
}

/// Configuration for embedded language checking
#[derive(Debug, Clone, Default)]
pub struct EmbeddedConfig {
    /// Path to Go module root (containing go.mod)
    pub go_mod_root: Option<String>,
    /// Path to TypeScript module root (containing package.json/node_modules)
    pub ts_module_root: Option<String>,
    /// Path to Rust project root (containing Cargo.toml)
    pub rust_project_root: Option<String>,
}

/// Extract all embedded code blocks from a parsed document
pub fn extract_embedded_blocks(doc: &ParsedDocument) -> Vec<EmbeddedBlock> {
    let mut blocks = Vec::new();

    if let Some(ref ast) = doc.ast {
        for suite in &ast.suites {
            extract_from_suite(suite, &mut blocks);
        }
    }

    blocks
}

fn extract_from_suite(suite: &Suite, blocks: &mut Vec<EmbeddedBlock>) {
    // Extract from setup blocks
    for (lang, setup) in &suite.setups {
        extract_from_setup(*lang, setup, &suite.name, blocks);
    }

    // Extract from fixtures
    for fixture in &suite.fixtures {
        extract_from_fixture(fixture, &suite.name, blocks);
    }

    // Extract from benchmarks
    for benchmark in &suite.benchmarks {
        extract_from_benchmark(benchmark, &suite.name, blocks);
    }
}

fn extract_from_setup(
    lang: Lang,
    setup: &StructuredSetup,
    suite_name: &str,
    blocks: &mut Vec<EmbeddedBlock>,
) {
    let context = format!("{}.setup.{}", suite_name, lang.as_str());

    if let Some(ref imports) = setup.imports {
        if !imports.code.trim().is_empty() {
            blocks.push(EmbeddedBlock {
                lang,
                block_type: BlockType::SetupImport,
                code: imports.code.clone(),
                span: imports.span.clone(),
                context_name: context.clone(),
            });
        }
    }

    if let Some(ref decl) = setup.declarations {
        if !decl.code.trim().is_empty() {
            blocks.push(EmbeddedBlock {
                lang,
                block_type: BlockType::SetupDeclare,
                code: decl.code.clone(),
                span: decl.span.clone(),
                context_name: context.clone(),
            });
        }
    }

    if let Some(ref init) = setup.init {
        if !init.code.trim().is_empty() {
            blocks.push(EmbeddedBlock {
                lang,
                block_type: BlockType::SetupInit,
                code: init.code.clone(),
                span: init.span.clone(),
                context_name: context.clone(),
            });
        }
    }

    if let Some(ref helpers) = setup.helpers {
        if !helpers.code.trim().is_empty() {
            blocks.push(EmbeddedBlock {
                lang,
                block_type: BlockType::SetupHelpers,
                code: helpers.code.clone(),
                span: helpers.span.clone(),
                context_name: context.clone(),
            });
        }
    }
}

fn extract_from_fixture(fixture: &Fixture, suite_name: &str, blocks: &mut Vec<EmbeddedBlock>) {
    let context = format!("{}.fixture.{}", suite_name, fixture.name);

    for (lang, code_block) in &fixture.implementations {
        if !code_block.code.trim().is_empty() {
            blocks.push(EmbeddedBlock {
                lang: *lang,
                block_type: BlockType::Fixture,
                code: code_block.code.clone(),
                span: code_block.span.clone(),
                context_name: context.clone(),
            });
        }
    }
}

fn extract_from_benchmark(
    benchmark: &Benchmark,
    suite_name: &str,
    blocks: &mut Vec<EmbeddedBlock>,
) {
    let context = format!("{}.bench.{}", suite_name, benchmark.name);

    // Main implementations
    for (lang, code_block) in &benchmark.implementations {
        if !code_block.code.trim().is_empty() {
            blocks.push(EmbeddedBlock {
                lang: *lang,
                block_type: BlockType::Benchmark,
                code: code_block.code.clone(),
                span: code_block.span.clone(),
                context_name: context.clone(),
            });
        }
    }

    // Skip conditions
    for (lang, code_block) in &benchmark.skip {
        if !code_block.code.trim().is_empty() {
            blocks.push(EmbeddedBlock {
                lang: *lang,
                block_type: BlockType::Skip,
                code: code_block.code.clone(),
                span: code_block.span.clone(),
                context_name: format!("{}.skip", context),
            });
        }
    }

    // Validation expressions
    for (lang, code_block) in &benchmark.validate {
        if !code_block.code.trim().is_empty() {
            blocks.push(EmbeddedBlock {
                lang: *lang,
                block_type: BlockType::Validate,
                code: code_block.code.clone(),
                span: code_block.span.clone(),
                context_name: format!("{}.validate", context),
            });
        }
    }

    // Lifecycle hooks
    for (lang, code_block) in &benchmark.before {
        if !code_block.code.trim().is_empty() {
            blocks.push(EmbeddedBlock {
                lang: *lang,
                block_type: BlockType::Hook,
                code: code_block.code.clone(),
                span: code_block.span.clone(),
                context_name: format!("{}.before", context),
            });
        }
    }

    for (lang, code_block) in &benchmark.after {
        if !code_block.code.trim().is_empty() {
            blocks.push(EmbeddedBlock {
                lang: *lang,
                block_type: BlockType::Hook,
                code: code_block.code.clone(),
                span: code_block.span.clone(),
                context_name: format!("{}.after", context),
            });
        }
    }

    for (lang, code_block) in &benchmark.each {
        if !code_block.code.trim().is_empty() {
            blocks.push(EmbeddedBlock {
                lang: *lang,
                block_type: BlockType::Hook,
                code: code_block.code.clone(),
                span: code_block.span.clone(),
                context_name: format!("{}.each", context),
            });
        }
    }
}

/// Context from a setup block needed to check other blocks
#[derive(Debug, Clone, Default)]
pub struct SetupContext {
    /// Import statements (Go: "import (...)", TS: "import { ... } from ...")
    pub imports: Option<String>,
    /// Package-level declarations
    pub declarations: Option<String>,
    /// Helper functions
    pub helpers: Option<String>,
    /// Standard library code to inject (from `use std::module` statements)
    pub stdlib_code: Option<String>,
}

/// Result from embedded checking including debug info
pub struct EmbeddedCheckResult {
    pub diagnostics: Vec<Diagnostic>,
    pub go_blocks_checked: usize,
    pub ts_blocks_checked: usize,
    pub rust_blocks_checked: usize,
    pub debug_messages: Vec<String>,
}

/// Check all embedded blocks and return diagnostics with debug info
pub fn check_embedded_blocks(
    doc: &ParsedDocument,
    blocks: &[EmbeddedBlock],
    config: &EmbeddedConfig,
) -> EmbeddedCheckResult {
    let mut result = EmbeddedCheckResult {
        diagnostics: Vec::new(),
        go_blocks_checked: 0,
        ts_blocks_checked: 0,
        rust_blocks_checked: 0,
        debug_messages: Vec::new(),
    };

    // Build context from setup blocks (imports + declarations) per language
    let mut go_context = SetupContext::default();
    let mut ts_context = SetupContext::default();
    let mut rust_context = SetupContext::default();

    for block in blocks {
        match (block.lang, block.block_type) {
            (Lang::Go, BlockType::SetupImport) => {
                go_context.imports = Some(block.code.clone());
            }
            (Lang::Go, BlockType::SetupDeclare) => {
                go_context.declarations = Some(block.code.clone());
            }
            (Lang::Go, BlockType::SetupHelpers) => {
                go_context.helpers = Some(block.code.clone());
            }
            (Lang::TypeScript, BlockType::SetupImport) => {
                ts_context.imports = Some(block.code.clone());
            }
            (Lang::TypeScript, BlockType::SetupDeclare) => {
                ts_context.declarations = Some(block.code.clone());
            }
            (Lang::TypeScript, BlockType::SetupHelpers) => {
                ts_context.helpers = Some(block.code.clone());
            }
            (Lang::Rust, BlockType::SetupImport) => {
                rust_context.imports = Some(block.code.clone());
            }
            (Lang::Rust, BlockType::SetupDeclare) => {
                rust_context.declarations = Some(block.code.clone());
            }
            (Lang::Rust, BlockType::SetupHelpers) => {
                rust_context.helpers = Some(block.code.clone());
            }
            _ => {}
        }
    }

    // Extract stdlib imports from AST and generate stdlib code
    if let Some(ref ast) = doc.ast {
        if !ast.use_stds.is_empty() {
            let stdlib_imports: HashSet<String> =
                ast.use_stds.iter().map(|u| u.module.clone()).collect();

            // Generate Go stdlib code
            let go_stdlib = stdlib::get_stdlib_code(&stdlib_imports, Lang::Go);
            if !go_stdlib.is_empty() {
                go_context.stdlib_code = Some(go_stdlib);
            }

            // Generate TypeScript stdlib code
            let ts_stdlib = stdlib::get_stdlib_code(&stdlib_imports, Lang::TypeScript);
            if !ts_stdlib.is_empty() {
                ts_context.stdlib_code = Some(ts_stdlib);
            }

            // Generate Rust stdlib code
            let rust_stdlib = stdlib::get_stdlib_code(&stdlib_imports, Lang::Rust);
            if !rust_stdlib.is_empty() {
                rust_context.stdlib_code = Some(rust_stdlib);
            }

            result.debug_messages.push(format!("Stdlib imports: {:?}", stdlib_imports));
        }
    }

    result.debug_messages.push(format!(
        "Go context: imports={}, decl={}, helpers={}, stdlib={}",
        go_context.imports.is_some(),
        go_context.declarations.is_some(),
        go_context.helpers.is_some(),
        go_context.stdlib_code.is_some()
    ));

    // === Go blocks ===
    // First, check all setup sections together (import, declare, helpers, init)
    // This avoids false "unused import" or "unused variable" errors
    let go_setup_blocks: Vec<_> = blocks
        .iter()
        .filter(|b| b.lang == Lang::Go)
        .filter(|b| {
            matches!(
                b.block_type,
                BlockType::SetupImport
                    | BlockType::SetupDeclare
                    | BlockType::SetupHelpers
                    | BlockType::SetupInit
            )
        })
        .collect();

    if !go_setup_blocks.is_empty() {
        result
            .debug_messages
            .push(format!("Checking {} Go setup blocks combined", go_setup_blocks.len()));
        let setup_diags = go_bridge::check_go_setup_combined(
            &go_setup_blocks,
            &go_context,
            config.go_mod_root.as_deref(),
        );
        result.debug_messages.push(format!("  -> {} diagnostics from setup", setup_diags.len()));
        result.go_blocks_checked += go_setup_blocks.len();
        for diag in setup_diags {
            result.diagnostics.push(convert_diagnostic(doc, &diag));
        }
    }

    // Check non-setup Go blocks (fixtures, benchmarks, hooks, etc.) individually with context
    let go_other_blocks: Vec<_> = blocks
        .iter()
        .filter(|b| b.lang == Lang::Go)
        .filter(|b| {
            !matches!(
                b.block_type,
                BlockType::SetupImport
                    | BlockType::SetupDeclare
                    | BlockType::SetupHelpers
                    | BlockType::SetupInit
            )
        })
        .collect();

    for block in &go_other_blocks {
        result.debug_messages.push(format!("Checking Go {:?} block", block.block_type));
        let go_diags = go_bridge::check_go_block(block, &go_context, config.go_mod_root.as_deref());
        result.debug_messages.push(format!("  -> {} diagnostics", go_diags.len()));
        result.go_blocks_checked += 1;
        for diag in go_diags {
            result.diagnostics.push(convert_diagnostic(doc, &diag));
        }
    }

    // === TypeScript blocks ===
    // First, check all setup sections together (import, declare, helpers, init)
    // This avoids false "unused import" or "unused variable" errors
    let ts_setup_blocks: Vec<_> = blocks
        .iter()
        .filter(|b| b.lang == Lang::TypeScript)
        .filter(|b| {
            matches!(
                b.block_type,
                BlockType::SetupImport
                    | BlockType::SetupDeclare
                    | BlockType::SetupHelpers
                    | BlockType::SetupInit
            )
        })
        .collect();

    if !ts_setup_blocks.is_empty() {
        result
            .debug_messages
            .push(format!("Checking {} TS setup blocks combined", ts_setup_blocks.len()));
        let setup_diags = ts_bridge::check_ts_setup_combined(
            &ts_setup_blocks,
            &ts_context,
            config.ts_module_root.as_deref(),
        );
        result.debug_messages.push(format!("  -> {} diagnostics from TS setup", setup_diags.len()));
        result.ts_blocks_checked += ts_setup_blocks.len();
        for diag in setup_diags {
            result.diagnostics.push(convert_diagnostic(doc, &diag));
        }
    }

    // Check non-setup TS blocks (fixtures, benchmarks, hooks, etc.) individually with context
    let ts_other_blocks: Vec<_> = blocks
        .iter()
        .filter(|b| b.lang == Lang::TypeScript)
        .filter(|b| {
            !matches!(
                b.block_type,
                BlockType::SetupImport
                    | BlockType::SetupDeclare
                    | BlockType::SetupHelpers
                    | BlockType::SetupInit
            )
        })
        .collect();

    for block in &ts_other_blocks {
        result.debug_messages.push(format!("Checking TS {:?} block", block.block_type));
        let ts_diags =
            ts_bridge::check_ts_block(block, &ts_context, config.ts_module_root.as_deref());
        result.debug_messages.push(format!("  -> {} diagnostics", ts_diags.len()));
        result.ts_blocks_checked += 1;
        for diag in ts_diags {
            result.diagnostics.push(convert_diagnostic(doc, &diag));
        }
    }

    // === Rust blocks ===
    // First, check all setup sections together (import, declare, helpers, init)
    let rust_setup_blocks: Vec<_> = blocks
        .iter()
        .filter(|b| b.lang == Lang::Rust)
        .filter(|b| {
            matches!(
                b.block_type,
                BlockType::SetupImport
                    | BlockType::SetupDeclare
                    | BlockType::SetupHelpers
                    | BlockType::SetupInit
            )
        })
        .collect();

    if !rust_setup_blocks.is_empty() {
        result
            .debug_messages
            .push(format!("Checking {} Rust setup blocks combined", rust_setup_blocks.len()));
        let setup_diags = rust_bridge::check_rust_setup_combined(
            &rust_setup_blocks,
            &rust_context,
            config.rust_project_root.as_deref(),
        );
        result
            .debug_messages
            .push(format!("  -> {} diagnostics from Rust setup", setup_diags.len()));
        result.rust_blocks_checked += rust_setup_blocks.len();
        for diag in setup_diags {
            result.diagnostics.push(convert_diagnostic(doc, &diag));
        }
    }

    // Check non-setup Rust blocks (fixtures, benchmarks, hooks, etc.) individually with context
    let rust_other_blocks: Vec<_> = blocks
        .iter()
        .filter(|b| b.lang == Lang::Rust)
        .filter(|b| {
            !matches!(
                b.block_type,
                BlockType::SetupImport
                    | BlockType::SetupDeclare
                    | BlockType::SetupHelpers
                    | BlockType::SetupInit
            )
        })
        .collect();

    for block in &rust_other_blocks {
        result.debug_messages.push(format!("Checking Rust {:?} block", block.block_type));
        let rust_diags = rust_bridge::check_rust_block(
            block,
            &rust_context,
            config.rust_project_root.as_deref(),
        );
        result.debug_messages.push(format!("  -> {} diagnostics", rust_diags.len()));
        result.rust_blocks_checked += 1;
        for diag in rust_diags {
            result.diagnostics.push(convert_diagnostic(doc, &diag));
        }
    }

    result
}

/// A diagnostic from an embedded language checker
#[derive(Debug)]
pub struct EmbeddedDiagnostic {
    pub start_offset: usize,
    pub end_offset: usize,
    pub message: String,
    pub severity: EmbeddedSeverity,
}

#[derive(Debug, Clone, Copy)]
pub enum EmbeddedSeverity {
    Error,
    Warning,
}

fn convert_diagnostic(doc: &ParsedDocument, diag: &EmbeddedDiagnostic) -> Diagnostic {
    let start = doc.offset_to_position(diag.start_offset);
    let end = doc.offset_to_position(diag.end_offset);

    Diagnostic {
        range: Range { start, end },
        severity: Some(match diag.severity {
            EmbeddedSeverity::Error => DiagnosticSeverity::ERROR,
            EmbeddedSeverity::Warning => DiagnosticSeverity::WARNING,
        }),
        code: None,
        code_description: None,
        source: Some("poly-bench".to_string()),
        message: diag.message.clone(),
        related_information: None,
        tags: None,
        data: None,
    }
}
