//! Shared helpers for building virtual files
//!
//! VirtualFileBuilderCore provides the common logic (categorize_blocks,
//! add_block_content, etc.) that each language runtime uses when building
//! virtual files.

use crate::{
    embedded::{BlockType, EmbeddedBlock},
    virtual_file::SectionMapping,
};

/// Internal data structure for virtual files
#[derive(Debug, Clone)]
pub struct VirtualFileData {
    pub uri: String,
    pub path: String,
    pub content: String,
    pub version: i32,
    pub section_mappings: Vec<SectionMapping>,
    pub bench_uri: String,
}

/// Shared builder for constructing virtual files
/// Each runtime creates this with language-specific uri/path and uses the helpers
pub struct VirtualFileBuilderCore {
    pub bench_uri: String,
    pub uri: String,
    pub path: String,
    pub content: String,
    pub version: i32,
    pub current_line: u32,
    pub section_mappings: Vec<SectionMapping>,
}

impl VirtualFileBuilderCore {
    pub fn new(bench_uri: &str, uri: String, path: String, version: i32) -> Self {
        Self {
            bench_uri: bench_uri.to_string(),
            uri,
            path,
            content: String::new(),
            version,
            current_line: 0,
            section_mappings: Vec::new(),
        }
    }

    pub fn categorize_blocks<'a>(
        blocks: &[&'a EmbeddedBlock],
    ) -> (
        Vec<&'a EmbeddedBlock>,
        Vec<&'a EmbeddedBlock>,
        Vec<&'a EmbeddedBlock>,
        Vec<&'a EmbeddedBlock>,
        Vec<&'a EmbeddedBlock>,
    ) {
        let imports: Vec<_> =
            blocks.iter().filter(|b| b.block_type == BlockType::SetupImport).copied().collect();
        let declares: Vec<_> =
            blocks.iter().filter(|b| b.block_type == BlockType::SetupDeclare).copied().collect();
        let helpers: Vec<_> =
            blocks.iter().filter(|b| b.block_type == BlockType::SetupHelpers).copied().collect();
        let inits: Vec<_> =
            blocks.iter().filter(|b| b.block_type == BlockType::SetupInit).copied().collect();
        let other: Vec<_> = blocks
            .iter()
            .filter(|b| {
                !matches!(
                    b.block_type,
                    BlockType::SetupImport |
                        BlockType::SetupDeclare |
                        BlockType::SetupHelpers |
                        BlockType::SetupInit
                )
            })
            .copied()
            .collect();

        (imports, declares, helpers, inits, other)
    }

    pub fn func_name_for_block(block_type: BlockType, i: usize) -> String {
        match block_type {
            BlockType::Fixture => format!("__polybench_fixture_{}", i),
            BlockType::Benchmark => format!("__polybench_bench_{}", i),
            BlockType::Hook => format!("__polybench_hook_{}", i),
            BlockType::Skip => format!("__polybench_skip_{}", i),
            BlockType::Validate => format!("__polybench_validate_{}", i),
            _ => format!("__polybench_block_{}", i),
        }
    }

    pub fn write_line(&mut self, line: &str) {
        self.content.push_str(line);
        self.content.push('\n');
        self.current_line += 1;
    }

    pub fn add_block_content(&mut self, block: &EmbeddedBlock) {
        let code = &block.code;
        let line_count = code.lines().count().max(1) as u32;

        self.section_mappings.push(SectionMapping {
            virtual_start_line: self.current_line,
            line_count,
            bench_span: block.span,
            block_type: block.block_type,
            code: code.clone(),
            bench_indent: None,
            indent_stripped: None,
        });

        self.content.push_str(code);
        if !code.ends_with('\n') {
            self.content.push('\n');
        }

        self.current_line += line_count;
    }

    pub fn add_block_content_with_semicolon(&mut self, block: &EmbeddedBlock) {
        let code = &block.code;
        let trimmed = code.trim_end();
        let line_count = code.lines().count().max(1) as u32;

        self.section_mappings.push(SectionMapping {
            virtual_start_line: self.current_line,
            line_count,
            bench_span: block.span,
            block_type: block.block_type,
            code: code.clone(),
            bench_indent: None,
            indent_stripped: None,
        });

        self.content.push_str(trimmed);
        if !trimmed.ends_with(';') && !trimmed.ends_with('}') {
            self.content.push(';');
        }
        self.content.push('\n');

        self.current_line += line_count;
    }

    pub fn add_block_content_normalized<F>(&mut self, block: &EmbeddedBlock, normalizer: F)
    where
        F: Fn(&str) -> String,
    {
        let normalized = normalizer(&block.code);
        let min_indent = block
            .code
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.len().saturating_sub(l.trim_start().len()))
            .min()
            .unwrap_or(0) as u32;
        let line_count = normalized.lines().count().max(1) as u32;
        self.section_mappings.push(SectionMapping {
            virtual_start_line: self.current_line,
            line_count,
            bench_span: block.span,
            block_type: block.block_type,
            code: block.code.clone(),
            bench_indent: None,
            indent_stripped: if min_indent > 0 { Some(min_indent) } else { None },
        });
        self.content.push_str(&normalized);
        if !normalized.ends_with('\n') {
            self.content.push('\n');
        }
        self.current_line += line_count;
    }

    pub fn add_block_content_indented<F>(
        &mut self,
        block: &EmbeddedBlock,
        indent: &str,
        normalizer: F,
    ) where
        F: Fn(&str) -> String,
    {
        let code = normalizer(&block.code);
        let line_count = code.lines().count().max(1) as u32;
        let indent_len = indent.len() as u32;
        self.section_mappings.push(SectionMapping {
            virtual_start_line: self.current_line,
            line_count,
            bench_span: block.span,
            block_type: block.block_type,
            code: code.clone(),
            bench_indent: Some(indent_len),
            indent_stripped: None,
        });
        for line in code.lines() {
            self.write_line(&format!("{}{}", indent, line));
        }
    }

    pub fn finish(self) -> VirtualFileData {
        VirtualFileData {
            uri: self.uri,
            path: self.path,
            content: self.content,
            version: self.version,
            section_mappings: self.section_mappings,
            bench_uri: self.bench_uri,
        }
    }
}
