//! Virtual file generation for language server integration
//!
//! This module creates virtual Go/TypeScript files from embedded code blocks
//! in .bench files, and provides position translation between the
//! .bench file and the virtual files.

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    path::Path,
};

use poly_bench_dsl::Span;
use poly_bench_project::templates;
use tower_lsp::lsp_types::Position;

use super::embedded::{BlockType, EmbeddedBlock};

// =============================================================================
// Shared Types
// =============================================================================

/// A section in the virtual file with its mapping back to .bench source
#[derive(Debug, Clone)]
pub struct SectionMapping {
    /// Starting line in the virtual file (0-indexed)
    pub virtual_start_line: u32,
    /// Number of lines in this section
    pub line_count: u32,
    /// Original span in the .bench file
    pub bench_span: Span,
    /// Block type for this section
    pub block_type: BlockType,
    /// The code content
    pub code: String,
}

/// Common interface for virtual files
pub trait VirtualFile {
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
    fn bench_to_virtual(&self, bench_offset: usize) -> Option<Position> {
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

                return Some(Position { line: virtual_line, character: col });
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
                let mut offset_in_code = 0usize;
                let mut current_line = 0u32;

                for (i, byte) in mapping.code.bytes().enumerate() {
                    if current_line == line_in_block {
                        offset_in_code = i + character as usize;
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

// =============================================================================
// Virtual File Data (shared struct for both Go and TS)
// =============================================================================

/// Internal data structure for virtual files
#[derive(Debug, Clone)]
struct VirtualFileData {
    uri: String,
    path: String,
    content: String,
    version: i32,
    section_mappings: Vec<SectionMapping>,
    bench_uri: String,
}

// =============================================================================
// Go Virtual Files
// =============================================================================

/// A virtual Go file generated from embedded blocks
#[derive(Debug, Clone)]
pub struct VirtualGoFile(VirtualFileData);

impl VirtualGoFile {
    /// Create a new virtual Go file from embedded blocks
    pub fn from_blocks(
        bench_uri: &str,
        bench_path: &str,
        go_mod_root: &str,
        blocks: &[&EmbeddedBlock],
        version: i32,
    ) -> Self {
        let mut builder = VirtualFileBuilder::new_go(bench_uri, bench_path, go_mod_root, version);
        builder.build_go(blocks);
        Self(builder.finish())
    }

    /// Translate bench offset to Go position (convenience method)
    pub fn bench_to_go(&self, bench_offset: usize) -> Option<Position> {
        self.bench_to_virtual(bench_offset)
    }

    /// Translate Go position to bench offset (convenience method)
    pub fn go_to_bench(&self, line: u32, character: u32) -> Option<usize> {
        self.virtual_to_bench(line, character)
    }
}

impl VirtualFile for VirtualGoFile {
    fn uri(&self) -> &str {
        &self.0.uri
    }
    fn path(&self) -> &str {
        &self.0.path
    }
    fn content(&self) -> &str {
        &self.0.content
    }
    fn version(&self) -> i32 {
        self.0.version
    }
    fn section_mappings(&self) -> &[SectionMapping] {
        &self.0.section_mappings
    }
    fn bench_uri(&self) -> &str {
        &self.0.bench_uri
    }
}

// =============================================================================
// TypeScript Virtual Files
// =============================================================================

/// A virtual TypeScript file generated from embedded blocks
#[derive(Debug, Clone)]
pub struct VirtualTsFile(VirtualFileData);

impl VirtualTsFile {
    /// Create a new virtual TypeScript file from embedded blocks
    pub fn from_blocks(
        bench_uri: &str,
        bench_path: &str,
        ts_module_root: &str,
        blocks: &[&EmbeddedBlock],
        version: i32,
    ) -> Self {
        let mut builder =
            VirtualFileBuilder::new_ts(bench_uri, bench_path, ts_module_root, version);
        builder.build_ts(blocks);
        Self(builder.finish())
    }

    /// Translate bench offset to TS position (convenience method)
    pub fn bench_to_ts(&self, bench_offset: usize) -> Option<Position> {
        self.bench_to_virtual(bench_offset)
    }

    /// Translate TS position to bench offset (convenience method)
    pub fn ts_to_bench(&self, line: u32, character: u32) -> Option<usize> {
        self.virtual_to_bench(line, character)
    }
}

impl VirtualFile for VirtualTsFile {
    fn uri(&self) -> &str {
        &self.0.uri
    }
    fn path(&self) -> &str {
        &self.0.path
    }
    fn content(&self) -> &str {
        &self.0.content
    }
    fn version(&self) -> i32 {
        self.0.version
    }
    fn section_mappings(&self) -> &[SectionMapping] {
        &self.0.section_mappings
    }
    fn bench_uri(&self) -> &str {
        &self.0.bench_uri
    }
}

// =============================================================================
// Rust Virtual Files
// =============================================================================

/// A virtual Rust file generated from embedded blocks
#[derive(Debug, Clone)]
pub struct VirtualRustFile(VirtualFileData);

impl VirtualRustFile {
    /// Create a new virtual Rust file from embedded blocks
    pub fn from_blocks(
        bench_uri: &str,
        bench_path: &str,
        rust_project_root: &str,
        blocks: &[&EmbeddedBlock],
        version: i32,
    ) -> Self {
        let mut builder =
            VirtualFileBuilder::new_rust(bench_uri, bench_path, rust_project_root, version);
        builder.build_rust(blocks);
        Self(builder.finish())
    }

    /// Translate bench offset to Rust position (convenience method)
    pub fn bench_to_rust(&self, bench_offset: usize) -> Option<Position> {
        self.bench_to_virtual(bench_offset)
    }

    /// Translate Rust position to bench offset (convenience method)
    pub fn rust_to_bench(&self, line: u32, character: u32) -> Option<usize> {
        self.virtual_to_bench(line, character)
    }
}

impl VirtualFile for VirtualRustFile {
    fn uri(&self) -> &str {
        &self.0.uri
    }
    fn path(&self) -> &str {
        &self.0.path
    }
    fn content(&self) -> &str {
        &self.0.content
    }
    fn version(&self) -> i32 {
        self.0.version
    }
    fn section_mappings(&self) -> &[SectionMapping] {
        &self.0.section_mappings
    }
    fn bench_uri(&self) -> &str {
        &self.0.bench_uri
    }
}

// =============================================================================
// Unified Builder
// =============================================================================

/// Builder for constructing virtual files
struct VirtualFileBuilder {
    bench_uri: String,
    uri: String,
    path: String,
    content: String,
    version: i32,
    current_line: u32,
    section_mappings: Vec<SectionMapping>,
}

impl VirtualFileBuilder {
    fn new_go(bench_uri: &str, bench_path: &str, go_mod_root: &str, version: i32) -> Self {
        let mut hasher = DefaultHasher::new();
        bench_path.hash(&mut hasher);
        let hash = hasher.finish();

        let subdir = Path::new(go_mod_root).join(".lsp_virtual");
        let filename = format!("virtual_{:016x}.go", hash);
        let path = subdir.join(&filename);
        let path_str = path.to_string_lossy().to_string();
        let uri = format!("file://{}", path_str);

        Self {
            bench_uri: bench_uri.to_string(),
            uri,
            path: path_str,
            content: String::new(),
            version,
            current_line: 0,
            section_mappings: Vec::new(),
        }
    }

    fn new_ts(bench_uri: &str, bench_path: &str, ts_module_root: &str, version: i32) -> Self {
        let mut hasher = DefaultHasher::new();
        bench_path.hash(&mut hasher);
        let hash = hasher.finish();

        let filename = format!("polybench_virtual_{:016x}.ts", hash);
        let path = Path::new(ts_module_root).join(&filename);
        let path_str = path.to_string_lossy().to_string();
        let uri = format!("file://{}", path_str);

        Self {
            bench_uri: bench_uri.to_string(),
            uri,
            path: path_str,
            content: String::new(),
            version,
            current_line: 0,
            section_mappings: Vec::new(),
        }
    }

    fn new_rust(bench_uri: &str, bench_path: &str, rust_project_root: &str, version: i32) -> Self {
        let mut hasher = DefaultHasher::new();
        bench_path.hash(&mut hasher);
        let hash = hasher.finish();

        // Place virtual files in src/bin/ so they're recognized as binary targets by rust-analyzer
        // This allows rust-analyzer to resolve dependencies from Cargo.toml
        let subdir = Path::new(rust_project_root).join("src/bin");
        let filename = format!("_lsp_virtual_{:016x}.rs", hash);
        let path = subdir.join(&filename);
        let path_str = path.to_string_lossy().to_string();
        let uri = format!("file://{}", path_str);

        Self {
            bench_uri: bench_uri.to_string(),
            uri,
            path: path_str,
            content: String::new(),
            version,
            current_line: 0,
            section_mappings: Vec::new(),
        }
    }

    fn build_go(&mut self, blocks: &[&EmbeddedBlock]) {
        let (imports, declares, helpers, inits, other) = Self::categorize_blocks(blocks);

        // Go package declaration
        self.write_line("package main");
        self.write_line("");

        // Imports
        for block in &imports {
            self.add_block_content(block);
            self.write_line("");
        }

        // Declarations
        for block in &declares {
            self.add_block_content(block);
            self.write_line("");
        }

        // Helpers
        for block in &helpers {
            self.add_block_content(block);
            self.write_line("");
        }

        // Init code wrapped in init()
        if !inits.is_empty() {
            self.write_line("func init() {");
            for block in &inits {
                self.add_block_content(block);
            }
            self.write_line("}");
            self.write_line("");
        }

        // Other blocks wrapped in Go functions
        for (i, block) in other.iter().enumerate() {
            let func_name = Self::func_name_for_block(block.block_type, i);
            self.write_line(&format!("func {}() {{", func_name));
            self.add_block_content(block);
            self.write_line("}");
            self.write_line("");
        }
    }

    fn build_ts(&mut self, blocks: &[&EmbeddedBlock]) {
        let (imports, declares, helpers, inits, other) = Self::categorize_blocks(blocks);

        // TypeScript imports
        for block in &imports {
            self.add_block_content(block);
            self.write_line("");
        }

        // Declarations (types, interfaces)
        for block in &declares {
            self.add_block_content(block);
            self.write_line("");
        }

        // Helpers
        for block in &helpers {
            self.add_block_content(block);
            self.write_line("");
        }

        // Init code (top-level in TS)
        for block in &inits {
            self.add_block_content(block);
            self.write_line("");
        }

        // Other blocks wrapped in TS functions
        for (i, block) in other.iter().enumerate() {
            let func_name = Self::func_name_for_block(block.block_type, i);
            self.write_line(&format!("function {}() {{", func_name));
            self.add_block_content(block);
            self.write_line("}");
            self.write_line("");
        }
    }

    fn build_rust(&mut self, blocks: &[&EmbeddedBlock]) {
        let (imports, declares, helpers, inits, other) = Self::categorize_blocks(blocks);

        // Rust header with allow attributes to suppress warnings
        self.write_line("#![allow(unused_imports, unused_variables, dead_code, unused_mut)]");
        self.write_line("");

        // Imports (use statements)
        for block in &imports {
            self.add_block_content(block);
            self.write_line("");
        }

        // Declarations (statics, consts, types)
        for block in &declares {
            self.add_block_content(block);
            self.write_line("");
        }

        // Helpers (functions)
        for block in &helpers {
            self.add_block_content(block);
            self.write_line("");
        }

        // Init code wrapped in function
        if !inits.is_empty() {
            self.write_line("fn __polybench_init() {");
            for block in &inits {
                self.add_block_content(block);
            }
            self.write_line("}");
            self.write_line("");
        }

        // Other blocks wrapped in Rust functions
        for (i, block) in other.iter().enumerate() {
            let func_name = Self::func_name_for_block(block.block_type, i);
            self.write_line(&format!("fn {}() {{", func_name));
            self.add_block_content(block);
            self.write_line("}");
            self.write_line("");
        }

        // Add main function to make it a valid Rust program
        self.write_line("fn main() {}");
    }

    fn categorize_blocks<'a>(
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
                    BlockType::SetupImport
                        | BlockType::SetupDeclare
                        | BlockType::SetupHelpers
                        | BlockType::SetupInit
                )
            })
            .copied()
            .collect();

        (imports, declares, helpers, inits, other)
    }

    fn func_name_for_block(block_type: BlockType, i: usize) -> String {
        match block_type {
            BlockType::Fixture => format!("__polybench_fixture_{}", i),
            BlockType::Benchmark => format!("__polybench_bench_{}", i),
            BlockType::Hook => format!("__polybench_hook_{}", i),
            BlockType::Skip => format!("__polybench_skip_{}", i),
            BlockType::Validate => format!("__polybench_validate_{}", i),
            _ => format!("__polybench_block_{}", i),
        }
    }

    fn write_line(&mut self, line: &str) {
        self.content.push_str(line);
        self.content.push('\n');
        self.current_line += 1;
    }

    fn add_block_content(&mut self, block: &EmbeddedBlock) {
        let code = &block.code;
        let line_count = code.lines().count().max(1) as u32;

        self.section_mappings.push(SectionMapping {
            virtual_start_line: self.current_line,
            line_count,
            bench_span: block.span.clone(),
            block_type: block.block_type,
            code: code.clone(),
        });

        self.content.push_str(code);
        if !code.ends_with('\n') {
            self.content.push('\n');
        }

        self.current_line += line_count;
    }

    fn finish(self) -> VirtualFileData {
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

// =============================================================================
// File Managers
// =============================================================================

/// Manager for virtual Go files
pub struct VirtualFileManager {
    files: dashmap::DashMap<String, VirtualGoFile>,
}

impl VirtualFileManager {
    pub fn new() -> Self {
        Self { files: dashmap::DashMap::new() }
    }

    pub fn get_or_create(
        &self,
        bench_uri: &str,
        bench_path: &str,
        go_mod_root: &str,
        blocks: &[&EmbeddedBlock],
        version: i32,
    ) -> VirtualGoFile {
        if let Some(existing) = self.files.get(bench_uri) {
            if existing.version() >= version {
                return existing.clone();
            }
        }

        let virtual_file =
            VirtualGoFile::from_blocks(bench_uri, bench_path, go_mod_root, blocks, version);

        // Ensure directory exists and write file
        if let Some(parent) = Path::new(virtual_file.path()).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Err(e) = std::fs::write(virtual_file.path(), virtual_file.content()) {
            eprintln!("[gopls] Failed to write virtual file: {}", e);
        }

        self.files.insert(bench_uri.to_string(), virtual_file.clone());
        virtual_file
    }

    pub fn remove(&self, bench_uri: &str) {
        if let Some((_, vf)) = self.files.remove(bench_uri) {
            let _ = std::fs::remove_file(vf.path());
        }
    }

    pub fn get(&self, bench_uri: &str) -> Option<VirtualGoFile> {
        self.files.get(bench_uri).map(|r| r.clone())
    }
}

impl Default for VirtualFileManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Manager for virtual TypeScript files
pub struct VirtualTsFileManager {
    files: dashmap::DashMap<String, VirtualTsFile>,
    initialized_roots: dashmap::DashMap<String, ()>,
}

impl VirtualTsFileManager {
    pub fn new() -> Self {
        Self { files: dashmap::DashMap::new(), initialized_roots: dashmap::DashMap::new() }
    }

    fn ensure_tsconfig(&self, ts_module_root: &str) {
        if self.initialized_roots.contains_key(ts_module_root) {
            return;
        }

        let tsconfig_path = Path::new(ts_module_root).join("tsconfig.json");
        if !tsconfig_path.exists() {
            let tsconfig_content = templates::tsconfig_json();
            let _ = std::fs::write(&tsconfig_path, tsconfig_content);
        }

        self.initialized_roots.insert(ts_module_root.to_string(), ());
    }

    pub fn get_or_create(
        &self,
        bench_uri: &str,
        bench_path: &str,
        ts_module_root: &str,
        blocks: &[&EmbeddedBlock],
        version: i32,
    ) -> VirtualTsFile {
        self.ensure_tsconfig(ts_module_root);

        if let Some(existing) = self.files.get(bench_uri) {
            if existing.version() >= version {
                return existing.clone();
            }
        }

        let virtual_file =
            VirtualTsFile::from_blocks(bench_uri, bench_path, ts_module_root, blocks, version);

        if let Err(e) = std::fs::write(virtual_file.path(), virtual_file.content()) {
            eprintln!("[tsserver] Failed to write virtual file: {}", e);
        }

        self.files.insert(bench_uri.to_string(), virtual_file.clone());
        virtual_file
    }

    pub fn remove(&self, bench_uri: &str) {
        if let Some((_, vf)) = self.files.remove(bench_uri) {
            let _ = std::fs::remove_file(vf.path());
        }
    }
}

impl Default for VirtualTsFileManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Manager for virtual Rust files
pub struct VirtualRustFileManager {
    files: dashmap::DashMap<String, VirtualRustFile>,
    initialized_roots: dashmap::DashMap<String, ()>,
}

impl VirtualRustFileManager {
    pub fn new() -> Self {
        Self { files: dashmap::DashMap::new(), initialized_roots: dashmap::DashMap::new() }
    }

    /// Ensure the virtual file directory exists
    fn ensure_project_setup(&self, rust_project_root: &str) {
        if self.initialized_roots.contains_key(rust_project_root) {
            return;
        }

        // Create src/bin/ directory for virtual files
        // Files in src/bin/*.rs are auto-discovered as binary targets by Cargo/rust-analyzer
        let virtual_dir = Path::new(rust_project_root).join("src/bin");
        let _ = std::fs::create_dir_all(&virtual_dir);

        self.initialized_roots.insert(rust_project_root.to_string(), ());
    }

    pub fn get_or_create(
        &self,
        bench_uri: &str,
        bench_path: &str,
        rust_project_root: &str,
        blocks: &[&EmbeddedBlock],
        version: i32,
    ) -> VirtualRustFile {
        self.ensure_project_setup(rust_project_root);

        if let Some(existing) = self.files.get(bench_uri) {
            if existing.version() >= version {
                return existing.clone();
            }
        }

        let virtual_file =
            VirtualRustFile::from_blocks(bench_uri, bench_path, rust_project_root, blocks, version);

        // Ensure directory exists and write file
        if let Some(parent) = Path::new(virtual_file.path()).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Err(e) = std::fs::write(virtual_file.path(), virtual_file.content()) {
            eprintln!("[rust-analyzer] Failed to write virtual file: {}", e);
        }

        self.files.insert(bench_uri.to_string(), virtual_file.clone());
        virtual_file
    }

    pub fn remove(&self, bench_uri: &str) {
        if let Some((_, vf)) = self.files.remove(bench_uri) {
            let _ = std::fs::remove_file(vf.path());
        }
    }

    pub fn get(&self, bench_uri: &str) -> Option<VirtualRustFile> {
        self.files.get(bench_uri).map(|r| r.clone())
    }
}

impl Default for VirtualRustFileManager {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_block(code: &str, block_type: BlockType, start: usize) -> EmbeddedBlock {
        let end = start + code.len();
        EmbeddedBlock {
            lang: poly_bench_dsl::Lang::Go,
            block_type,
            code: code.to_string(),
            span: Span { start, end, line: 1, col: 1 },
            context_name: "test".to_string(),
        }
    }

    #[test]
    fn test_virtual_file_generation() {
        let import_block = make_block("\"fmt\"", BlockType::SetupImport, 100);
        let bench_block = make_block("fmt.Println(\"hello\")", BlockType::Benchmark, 200);

        let blocks: Vec<&EmbeddedBlock> = vec![&import_block, &bench_block];

        let vf =
            VirtualGoFile::from_blocks("file:///test.bench", "/test.bench", "/tmp/go", &blocks, 1);

        assert!(vf.content().contains("package main"));
        assert!(vf.content().contains("\"fmt\""));
        assert!(vf.content().contains("fmt.Println"));
        assert_eq!(vf.section_mappings().len(), 2);
    }

    #[test]
    fn test_position_translation() {
        let code = "fmt.Println(\"hello\")";
        let block = make_block(code, BlockType::Benchmark, 100);
        let blocks: Vec<&EmbeddedBlock> = vec![&block];

        let vf =
            VirtualGoFile::from_blocks("file:///test.bench", "/test.bench", "/tmp/go", &blocks, 1);

        let pos = vf.bench_to_go(100);
        assert!(pos.is_some());
        let pos = pos.unwrap();
        assert_eq!(pos.character, 0);

        let offset = vf.go_to_bench(pos.line, pos.character);
        assert!(offset.is_some());
        assert_eq!(offset.unwrap(), 100);
    }
}
