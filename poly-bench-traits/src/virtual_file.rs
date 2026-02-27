//! Virtual file types and traits for LSP integration

use poly_bench_dsl::Lang;

use crate::{
    embedded::{BlockType, EmbeddedBlock},
    position::LspPosition,
};

/// Parameters for building a virtual file from embedded blocks
#[derive(Debug, Clone)]
pub struct VirtualFileParams<'a> {
    pub bench_uri: &'a str,
    pub bench_path: &'a str,
    pub module_root: &'a str,
    pub blocks: &'a [&'a EmbeddedBlock],
    pub fixture_names: &'a [String],
    pub version: i32,
}

/// Trait for building virtual files from embedded blocks
pub trait VirtualFileBuilder: Send + Sync {
    /// The language this builder handles
    fn lang(&self) -> Lang;

    /// Build a virtual file from the given parameters
    fn build(&self, params: VirtualFileParams<'_>) -> Box<dyn VirtualFile>;
}

/// A section in the virtual file with its mapping back to .bench source
#[derive(Debug, Clone)]
pub struct SectionMapping {
    /// Starting line in the virtual file (0-indexed)
    pub virtual_start_line: u32,
    /// Number of lines in this section
    pub line_count: u32,
    /// Original span in the .bench file
    pub bench_span: poly_bench_syntax::Span,
    /// Block type for this section
    pub block_type: BlockType,
    /// The code content (original for position mapping; must match bench_span)
    pub code: String,
    /// If Some(n), each virtual line has n leading indent chars; subtract from character when
    /// mapping
    pub bench_indent: Option<u32>,
    /// If Some(n), bench content had n leading chars stripped per line; subtract from col when
    /// mapping bench->virtual
    pub indent_stripped: Option<u32>,
}

/// Common interface for virtual files
pub trait VirtualFile: Send + Sync {
    /// Get the file URI
    fn uri(&self) -> &str;
    /// Get the file path
    fn path(&self) -> &str;
    /// Get the file content
    fn content(&self) -> &str;
    /// Get the file version
    fn version(&self) -> i32;
    /// Get section mappings
    fn section_mappings(&self) -> &[SectionMapping];
    /// Get the original bench URI
    fn bench_uri(&self) -> &str;

    /// Translate a .bench file offset to a position in the virtual file
    fn bench_to_virtual(&self, bench_offset: usize) -> Option<LspPosition> {
        for mapping in self.section_mappings() {
            let span = &mapping.bench_span;

            if bench_offset >= span.start && bench_offset < span.end {
                let relative_offset = bench_offset - span.start;
                let code_bytes = mapping.code.as_bytes();
                let mut line_in_block: u32 = 0;
                let mut col: u32 = 0;
                let mut current_offset = 0;

                for &byte in code_bytes.iter().take(relative_offset) {
                    if byte == b'\n' {
                        line_in_block += 1;
                        col = 0;
                    } else {
                        col += 1;
                    }
                    current_offset += 1;
                }

                if current_offset < relative_offset && relative_offset <= code_bytes.len() {
                    col += (relative_offset - current_offset) as u32;
                }

                let virtual_line = mapping.virtual_start_line + line_in_block;
                let virtual_char =
                    mapping.indent_stripped.map(|n| col.saturating_sub(n)).unwrap_or(col);

                return Some(LspPosition { line: virtual_line, character: virtual_char });
            }
        }

        None
    }

    /// Translate a position in the virtual file back to a .bench file offset
    fn virtual_to_bench(&self, line: u32, character: u32) -> Option<usize> {
        for mapping in self.section_mappings() {
            let section_end_line = mapping.virtual_start_line + mapping.line_count;

            if line >= mapping.virtual_start_line && line < section_end_line {
                let line_in_block = line - mapping.virtual_start_line;
                let mut char =
                    mapping.bench_indent.map(|n| character.saturating_sub(n)).unwrap_or(character);
                if let Some(n) = mapping.indent_stripped {
                    char += n;
                }
                let mut offset_in_code = 0usize;
                let mut current_line = 0u32;

                for (i, byte) in mapping.code.bytes().enumerate() {
                    if current_line == line_in_block {
                        offset_in_code = i + char as usize;
                        break;
                    }
                    if byte == b'\n' {
                        current_line += 1;
                    }
                }

                if current_line < line_in_block {
                    return None;
                }

                offset_in_code = offset_in_code.min(mapping.code.len());
                let bench_offset = mapping.bench_span.start + offset_in_code;

                return Some(bench_offset);
            }
        }

        None
    }

    /// Check if a .bench file offset is within a code block
    fn contains_offset(&self, bench_offset: usize) -> bool {
        self.section_mappings()
            .iter()
            .any(|m| bench_offset >= m.bench_span.start && bench_offset < m.bench_span.end)
    }

    /// Find the block containing a .bench file offset
    fn block_at_offset(&self, bench_offset: usize) -> Option<&SectionMapping> {
        self.section_mappings()
            .iter()
            .find(|m| bench_offset >= m.bench_span.start && bench_offset < m.bench_span.end)
    }
}
