//! Embedded language support for poly-bench LSP v2
//!
//! This module extracts embedded code blocks (Go, TypeScript, Rust, Python) from
//! the partial AST and provides utilities for working with them.

pub use poly_bench_lsp_traits::{BlockType, EmbeddedBlock};
use poly_bench_syntax::{
    CodeBlock, Lang, Node, PartialBenchmark, PartialFile, PartialFixture, PartialSuite, Span,
    StructuredSetup,
};
use std::collections::HashMap;

/// Configuration for embedded language checking
/// Module roots are keyed by language (registry-based)
#[derive(Debug, Clone, Default)]
pub struct EmbeddedConfig {
    /// Map from language to module/project root path
    module_roots: HashMap<Lang, String>,
}

impl EmbeddedConfig {
    /// Get the module root for a language
    pub fn module_root(&self, lang: Lang) -> Option<&str> {
        self.module_roots.get(&lang).map(String::as_str)
    }

    /// Set the module root for a language
    pub fn set_module_root(&mut self, lang: Lang, root: String) {
        self.module_roots.insert(lang, root);
    }

    /// Clear all module roots
    pub fn clear(&mut self) {
        self.module_roots.clear();
    }
}

/// Extract all embedded code blocks from a partial AST
pub fn extract_embedded_blocks(partial_ast: &PartialFile) -> Vec<EmbeddedBlock> {
    let mut blocks = Vec::new();

    for suite_node in &partial_ast.suites {
        if let Some(suite) = suite_node.as_valid() {
            extract_from_suite(suite, &mut blocks);
        }
    }

    blocks
}

/// Find the block containing the given byte offset
pub fn find_block_at_offset(blocks: &[EmbeddedBlock], offset: usize) -> Option<&EmbeddedBlock> {
    blocks.iter().find(|b| b.span.contains(offset))
}

/// Find all blocks for a specific language
pub fn blocks_for_language(blocks: &[EmbeddedBlock], lang: Lang) -> Vec<&EmbeddedBlock> {
    blocks.iter().filter(|b| b.lang == lang).collect()
}

fn extract_from_suite(suite: &PartialSuite, blocks: &mut Vec<EmbeddedBlock>) {
    // Extract from setup blocks
    for (lang, setup_node) in &suite.setups {
        if let Some(setup) = setup_node.as_valid() {
            extract_from_setup(*lang, setup, &suite.name, blocks);
        }
    }

    // Extract from fixtures
    for fixture_node in &suite.fixtures {
        if let Some(fixture) = fixture_node.as_valid() {
            extract_from_fixture(fixture, &suite.name, blocks);
        }
    }

    // Extract from benchmarks
    for benchmark_node in &suite.benchmarks {
        if let Some(benchmark) = benchmark_node.as_valid() {
            extract_from_benchmark(benchmark, &suite.name, blocks);
        }
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
                span: imports.span,
                context_name: format!("{}.import", context),
            });
        }
    }

    if let Some(ref declare) = setup.declare {
        if !declare.code.trim().is_empty() {
            blocks.push(EmbeddedBlock {
                lang,
                block_type: BlockType::SetupDeclare,
                code: declare.code.clone(),
                span: declare.span,
                context_name: format!("{}.declare", context),
            });
        }
    }

    if let Some(ref init) = setup.init {
        if !init.code.trim().is_empty() {
            blocks.push(EmbeddedBlock {
                lang,
                block_type: BlockType::SetupInit,
                code: init.code.clone(),
                span: init.span,
                context_name: format!("{}.init", context),
            });
        }
    }

    if let Some(ref helpers) = setup.helpers {
        if !helpers.code.trim().is_empty() {
            blocks.push(EmbeddedBlock {
                lang,
                block_type: BlockType::SetupHelpers,
                code: helpers.code.clone(),
                span: helpers.span,
                context_name: format!("{}.helpers", context),
            });
        }
    }
}

fn extract_from_fixture(
    fixture: &PartialFixture,
    suite_name: &str,
    blocks: &mut Vec<EmbeddedBlock>,
) {
    let context = format!("{}.fixture.{}", suite_name, fixture.name);

    for (lang, code_node) in &fixture.implementations {
        if let Some(code_block) = code_node.as_valid() {
            if !code_block.code.trim().is_empty() {
                blocks.push(EmbeddedBlock {
                    lang: *lang,
                    block_type: BlockType::Fixture,
                    code: code_block.code.clone(),
                    span: code_block.span,
                    context_name: context.clone(),
                });
            }
        }
    }
}

fn extract_from_benchmark(
    benchmark: &PartialBenchmark,
    suite_name: &str,
    blocks: &mut Vec<EmbeddedBlock>,
) {
    let context = format!("{}.bench.{}", suite_name, benchmark.name);

    // Main implementations
    for (lang, code_node) in &benchmark.implementations {
        if let Some(code_block) = code_node.as_valid() {
            if !code_block.code.trim().is_empty() {
                blocks.push(EmbeddedBlock {
                    lang: *lang,
                    block_type: BlockType::Benchmark,
                    code: code_block.code.clone(),
                    span: code_block.span,
                    context_name: context.clone(),
                });
            }
        }
    }

    // Skip conditions
    for (lang, code_node) in &benchmark.skip {
        if let Some(code_block) = code_node.as_valid() {
            if !code_block.code.trim().is_empty() {
                blocks.push(EmbeddedBlock {
                    lang: *lang,
                    block_type: BlockType::Skip,
                    code: code_block.code.clone(),
                    span: code_block.span,
                    context_name: format!("{}.skip", context),
                });
            }
        }
    }

    // Validation expressions
    for (lang, code_node) in &benchmark.validate {
        if let Some(code_block) = code_node.as_valid() {
            if !code_block.code.trim().is_empty() {
                blocks.push(EmbeddedBlock {
                    lang: *lang,
                    block_type: BlockType::Validate,
                    code: code_block.code.clone(),
                    span: code_block.span,
                    context_name: format!("{}.validate", context),
                });
            }
        }
    }

    // Lifecycle hooks
    extract_hooks(&benchmark.before, *&Lang::Go, BlockType::Hook, &context, "before", blocks);
    extract_hooks(&benchmark.after, *&Lang::Go, BlockType::Hook, &context, "after", blocks);
    extract_hooks(&benchmark.each, *&Lang::Go, BlockType::Hook, &context, "each", blocks);
}

fn extract_hooks(
    hooks: &std::collections::HashMap<Lang, Node<CodeBlock>>,
    _default_lang: Lang,
    block_type: BlockType,
    context: &str,
    hook_name: &str,
    blocks: &mut Vec<EmbeddedBlock>,
) {
    for (lang, code_node) in hooks {
        if let Some(code_block) = code_node.as_valid() {
            if !code_block.code.trim().is_empty() {
                blocks.push(EmbeddedBlock {
                    lang: *lang,
                    block_type,
                    code: code_block.code.clone(),
                    span: code_block.span,
                    context_name: format!("{}.{}", context, hook_name),
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_block_at_offset() {
        let blocks = vec![
            EmbeddedBlock {
                lang: Lang::Go,
                block_type: BlockType::Benchmark,
                code: "test()".to_string(),
                span: Span::new(10, 20, 1, 0, 1, 10),
                context_name: "test".to_string(),
            },
            EmbeddedBlock {
                lang: Lang::TypeScript,
                block_type: BlockType::Benchmark,
                code: "test()".to_string(),
                span: Span::new(30, 40, 2, 0, 2, 10),
                context_name: "test".to_string(),
            },
        ];

        assert!(find_block_at_offset(&blocks, 15).is_some());
        assert_eq!(find_block_at_offset(&blocks, 15).unwrap().lang, Lang::Go);

        assert!(find_block_at_offset(&blocks, 35).is_some());
        assert_eq!(find_block_at_offset(&blocks, 35).unwrap().lang, Lang::TypeScript);

        assert!(find_block_at_offset(&blocks, 25).is_none());
    }

    #[test]
    fn test_blocks_for_language() {
        let blocks = vec![
            EmbeddedBlock {
                lang: Lang::Go,
                block_type: BlockType::Benchmark,
                code: "go1()".to_string(),
                span: Span::new(10, 20, 1, 0, 1, 10),
                context_name: "test".to_string(),
            },
            EmbeddedBlock {
                lang: Lang::TypeScript,
                block_type: BlockType::Benchmark,
                code: "ts()".to_string(),
                span: Span::new(30, 40, 2, 0, 2, 10),
                context_name: "test".to_string(),
            },
            EmbeddedBlock {
                lang: Lang::Go,
                block_type: BlockType::SetupHelpers,
                code: "go2()".to_string(),
                span: Span::new(50, 60, 3, 0, 3, 10),
                context_name: "test".to_string(),
            },
        ];

        let go_blocks = blocks_for_language(&blocks, Lang::Go);
        assert_eq!(go_blocks.len(), 2);

        let ts_blocks = blocks_for_language(&blocks, Lang::TypeScript);
        assert_eq!(ts_blocks.len(), 1);

        let rust_blocks = blocks_for_language(&blocks, Lang::Rust);
        assert_eq!(rust_blocks.len(), 0);
    }
}
