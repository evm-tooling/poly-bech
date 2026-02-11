//! Virtual Go file generation for gopls integration
//!
//! This module creates virtual Go files from embedded Go code blocks
//! in .bench files, and provides position translation between the
//! .bench file and the virtual Go file.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;

use poly_bench::dsl::Span;
use poly_bench::project::templates;
use tower_lsp::lsp_types::Position;

use super::embedded::{BlockType, EmbeddedBlock};

/// A section in the virtual Go file with its mapping back to .bench source
#[derive(Debug, Clone)]
pub struct SectionMapping {
    /// Starting line in the virtual Go file (0-indexed)
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

/// A virtual Go file generated from embedded blocks
#[derive(Debug)]
pub struct VirtualGoFile {
    /// URI for the virtual file (file:///path/to/_polybench_virtual_XXX.go)
    pub uri: String,
    /// Full path to the virtual file
    pub path: String,
    /// The generated Go source code
    pub content: String,
    /// File version (for LSP synchronization)
    pub version: i32,
    /// Mappings from virtual file sections to .bench file spans
    pub section_mappings: Vec<SectionMapping>,
    /// URI of the original .bench file
    pub bench_uri: String,
}

impl VirtualGoFile {
    /// Create a new virtual Go file from embedded blocks
    pub fn from_blocks(
        bench_uri: &str,
        bench_path: &str,
        go_mod_root: &str,
        blocks: &[&EmbeddedBlock],
        version: i32,
    ) -> Self {
        let mut builder = VirtualFileBuilder::new(bench_uri, bench_path, go_mod_root, version);
        builder.build(blocks);
        builder.finish()
    }

    /// Translate a position in the .bench file to a position in the virtual Go file
    ///
    /// Returns None if the position is not within a Go code block
    pub fn bench_to_go(&self, bench_offset: usize) -> Option<Position> {
        for mapping in &self.section_mappings {
            let span = &mapping.bench_span;
            
            // Check if offset falls within this block's span
            if bench_offset >= span.start && bench_offset < span.end {
                // Calculate relative position within the code block
                let relative_offset = bench_offset - span.start;
                
                // Convert relative offset to line/column within the code
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
                
                // Handle case where relative_offset is at exact boundary
                if current_offset < relative_offset && relative_offset <= code_bytes.len() {
                    col += (relative_offset - current_offset) as u32;
                }
                
                // Translate to virtual file position
                let virtual_line = mapping.virtual_start_line + line_in_block;
                
                return Some(Position {
                    line: virtual_line,
                    character: col,
                });
            }
        }
        
        None
    }

    /// Translate a position in the virtual Go file back to an offset in the .bench file
    ///
    /// Returns None if the position is in the wrapper code (not in original block)
    pub fn go_to_bench(&self, line: u32, character: u32) -> Option<usize> {
        for mapping in &self.section_mappings {
            // Check if line falls within this section
            let section_end_line = mapping.virtual_start_line + mapping.line_count;
            
            if line >= mapping.virtual_start_line && line < section_end_line {
                // Calculate relative line within the block
                let line_in_block = line - mapping.virtual_start_line;
                
                // Convert to offset within the code
                let mut offset_in_code = 0usize;
                let mut current_line = 0u32;
                
                for (i, byte) in mapping.code.bytes().enumerate() {
                    if current_line == line_in_block {
                        // Found the line, now add column offset
                        offset_in_code = i + character as usize;
                        break;
                    }
                    if byte == b'\n' {
                        current_line += 1;
                    }
                }
                
                // If we're on the last line and didn't find it via newline
                if current_line < line_in_block {
                    // Position is past the content
                    return None;
                }
                
                // Clamp offset to code length
                offset_in_code = offset_in_code.min(mapping.code.len());
                
                // Translate back to .bench offset
                let bench_offset = mapping.bench_span.start + offset_in_code;
                
                return Some(bench_offset);
            }
        }
        
        None
    }

    /// Check if a .bench file offset is within a Go code block
    pub fn contains_offset(&self, bench_offset: usize) -> bool {
        self.section_mappings.iter().any(|m| {
            bench_offset >= m.bench_span.start && bench_offset < m.bench_span.end
        })
    }

    /// Find the block containing a .bench file offset
    pub fn block_at_offset(&self, bench_offset: usize) -> Option<&SectionMapping> {
        self.section_mappings.iter().find(|m| {
            bench_offset >= m.bench_span.start && bench_offset < m.bench_span.end
        })
    }
}

/// Builder for constructing virtual Go files
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
    fn new(bench_uri: &str, bench_path: &str, go_mod_root: &str, version: i32) -> Self {
        // Generate a unique filename based on the bench file path
        let mut hasher = DefaultHasher::new();
        bench_path.hash(&mut hasher);
        let hash = hasher.finish();
        
        // Put virtual files in a subdirectory to avoid conflicts with bench_standalone.go
        // This way both can use "package main" without duplicate declaration errors
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

    fn build(&mut self, blocks: &[&EmbeddedBlock]) {
        // Separate blocks by type
        let imports: Vec<_> = blocks.iter()
            .filter(|b| b.block_type == BlockType::SetupImport)
            .collect();
        let declares: Vec<_> = blocks.iter()
            .filter(|b| b.block_type == BlockType::SetupDeclare)
            .collect();
        let helpers: Vec<_> = blocks.iter()
            .filter(|b| b.block_type == BlockType::SetupHelpers)
            .collect();
        let inits: Vec<_> = blocks.iter()
            .filter(|b| b.block_type == BlockType::SetupInit)
            .collect();
        let other: Vec<_> = blocks.iter()
            .filter(|b| !matches!(b.block_type, 
                BlockType::SetupImport | BlockType::SetupDeclare | 
                BlockType::SetupHelpers | BlockType::SetupInit))
            .collect();

        // Write package declaration
        self.write_line("package main");
        self.write_line("");

        // Write imports - the code already contains full import statements
        for block in &imports {
            self.add_block_content(block);
            self.write_line("");
        }

        // Write declarations
        for block in &declares {
            self.add_block_content(block);
            self.write_line("");
        }

        // Write helpers
        for block in &helpers {
            self.add_block_content(block);
            self.write_line("");
        }

        // Write init code wrapped in init()
        if !inits.is_empty() {
            self.write_line("func init() {");
            for block in &inits {
                self.add_block_content(block);
            }
            self.write_line("}");
            self.write_line("");
        }

        // Write other blocks (fixtures, benchmarks, etc.) wrapped in functions
        for (i, block) in other.iter().enumerate() {
            let func_name = match block.block_type {
                BlockType::Fixture => format!("__polybench_fixture_{}", i),
                BlockType::Benchmark => format!("__polybench_bench_{}", i),
                BlockType::Hook => format!("__polybench_hook_{}", i),
                BlockType::Skip => format!("__polybench_skip_{}", i),
                BlockType::Validate => format!("__polybench_validate_{}", i),
                _ => format!("__polybench_block_{}", i),
            };
            
            self.write_line(&format!("func {}() {{", func_name));
            self.add_block_content(block);
            self.write_line("}");
            self.write_line("");
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
        
        // Record the mapping
        self.section_mappings.push(SectionMapping {
            virtual_start_line: self.current_line,
            line_count,
            bench_span: block.span.clone(),
            block_type: block.block_type,
            code: code.clone(),
        });

        // Write the code
        self.content.push_str(code);
        if !code.ends_with('\n') {
            self.content.push('\n');
        }
        
        self.current_line += line_count;
    }

    fn finish(self) -> VirtualGoFile {
        VirtualGoFile {
            uri: self.uri,
            path: self.path,
            content: self.content,
            version: self.version,
            section_mappings: self.section_mappings,
            bench_uri: self.bench_uri,
        }
    }
}

/// Manager for virtual Go files
pub struct VirtualFileManager {
    /// Virtual files indexed by .bench URI
    files: dashmap::DashMap<String, VirtualGoFile>,
}

impl VirtualFileManager {
    pub fn new() -> Self {
        Self {
            files: dashmap::DashMap::new(),
        }
    }

    /// Get or create a virtual file for a .bench document
    /// 
    /// This also writes the virtual file to disk so gopls can access it.
    pub fn get_or_create(
        &self,
        bench_uri: &str,
        bench_path: &str,
        go_mod_root: &str,
        blocks: &[&EmbeddedBlock],
        version: i32,
    ) -> VirtualGoFile {
        // Check if we have an up-to-date version
        if let Some(existing) = self.files.get(bench_uri) {
            if existing.version >= version {
                // Clone the virtual file data we need
                return VirtualGoFile {
                    uri: existing.uri.clone(),
                    path: existing.path.clone(),
                    content: existing.content.clone(),
                    version: existing.version,
                    section_mappings: existing.section_mappings.clone(),
                    bench_uri: existing.bench_uri.clone(),
                };
            }
        }

        // Create new virtual file
        let virtual_file = VirtualGoFile::from_blocks(
            bench_uri,
            bench_path,
            go_mod_root,
            blocks,
            version,
        );

        // Ensure the .lsp_virtual subdirectory exists
        if let Some(parent) = Path::new(&virtual_file.path).parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                eprintln!("[gopls] Failed to create virtual file directory {}: {}", parent.display(), e);
            }
        }

        // Write the virtual file to disk so gopls can access it
        if let Err(e) = std::fs::write(&virtual_file.path, &virtual_file.content) {
            eprintln!("[gopls] Failed to write virtual file {}: {}", virtual_file.path, e);
        } else {
            eprintln!("[gopls] Wrote virtual file: {} ({} bytes)", virtual_file.path, virtual_file.content.len());
        }

        // Store it (we need to clone since we're returning ownership)
        let result = VirtualGoFile {
            uri: virtual_file.uri.clone(),
            path: virtual_file.path.clone(),
            content: virtual_file.content.clone(),
            version: virtual_file.version,
            section_mappings: virtual_file.section_mappings.clone(),
            bench_uri: virtual_file.bench_uri.clone(),
        };
        
        self.files.insert(bench_uri.to_string(), virtual_file);
        
        result
    }

    /// Remove a virtual file when the .bench file is closed
    pub fn remove(&self, bench_uri: &str) {
        if let Some((_, vf)) = self.files.remove(bench_uri) {
            // Also delete the file from disk
            if let Err(e) = std::fs::remove_file(&vf.path) {
                // Don't warn if file doesn't exist
                if e.kind() != std::io::ErrorKind::NotFound {
                    eprintln!("[gopls] Failed to remove virtual file {}: {}", vf.path, e);
                }
            }
        }
    }

    /// Get a virtual file if it exists
    pub fn get(&self, bench_uri: &str) -> Option<VirtualGoFile> {
        self.files.get(bench_uri).map(|r| VirtualGoFile {
            uri: r.uri.clone(),
            path: r.path.clone(),
            content: r.content.clone(),
            version: r.version,
            section_mappings: r.section_mappings.clone(),
            bench_uri: r.bench_uri.clone(),
        })
    }
}

impl Default for VirtualFileManager {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// TypeScript Virtual File Support
// =============================================================================

/// A virtual TypeScript file generated from embedded blocks
#[derive(Debug)]
pub struct VirtualTsFile {
    /// URI for the virtual file (file:///path/to/polybench_virtual_XXX.ts)
    pub uri: String,
    /// Full path to the virtual file
    pub path: String,
    /// The generated TypeScript source code
    pub content: String,
    /// File version (for LSP synchronization)
    pub version: i32,
    /// Mappings from virtual file sections to .bench file spans
    pub section_mappings: Vec<SectionMapping>,
    /// URI of the original .bench file
    pub bench_uri: String,
}

impl VirtualTsFile {
    /// Create a new virtual TypeScript file from embedded blocks
    pub fn from_blocks(
        bench_uri: &str,
        bench_path: &str,
        ts_module_root: &str,
        blocks: &[&EmbeddedBlock],
        version: i32,
    ) -> Self {
        let mut builder = VirtualTsFileBuilder::new(bench_uri, bench_path, ts_module_root, version);
        builder.build(blocks);
        builder.finish()
    }

    /// Translate a position in the .bench file to a position in the virtual TS file
    ///
    /// Returns None if the position is not within a TypeScript code block
    pub fn bench_to_ts(&self, bench_offset: usize) -> Option<Position> {
        for mapping in &self.section_mappings {
            let span = &mapping.bench_span;
            
            // Check if offset falls within this block's span
            if bench_offset >= span.start && bench_offset < span.end {
                // Calculate relative position within the code block
                let relative_offset = bench_offset - span.start;
                
                // Convert relative offset to line/column within the code
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
                
                // Handle case where relative_offset is at exact boundary
                if current_offset < relative_offset && relative_offset <= code_bytes.len() {
                    col += (relative_offset - current_offset) as u32;
                }
                
                // Translate to virtual file position
                let virtual_line = mapping.virtual_start_line + line_in_block;
                
                return Some(Position {
                    line: virtual_line,
                    character: col,
                });
            }
        }
        
        None
    }

    /// Translate a position in the virtual TS file back to an offset in the .bench file
    ///
    /// Returns None if the position is in the wrapper code (not in original block)
    pub fn ts_to_bench(&self, line: u32, character: u32) -> Option<usize> {
        for mapping in &self.section_mappings {
            // Check if line falls within this section
            let section_end_line = mapping.virtual_start_line + mapping.line_count;
            
            if line >= mapping.virtual_start_line && line < section_end_line {
                // Calculate relative line within the block
                let line_in_block = line - mapping.virtual_start_line;
                
                // Convert to offset within the code
                let mut offset_in_code = 0usize;
                let mut current_line = 0u32;
                
                for (i, byte) in mapping.code.bytes().enumerate() {
                    if current_line == line_in_block {
                        // Found the line, now add column offset
                        offset_in_code = i + character as usize;
                        break;
                    }
                    if byte == b'\n' {
                        current_line += 1;
                    }
                }
                
                // If we're on the last line and didn't find it via newline
                if current_line < line_in_block {
                    // Position is past the content
                    return None;
                }
                
                // Clamp offset to code length
                offset_in_code = offset_in_code.min(mapping.code.len());
                
                // Translate back to .bench offset
                let bench_offset = mapping.bench_span.start + offset_in_code;
                
                return Some(bench_offset);
            }
        }
        
        None
    }

    /// Check if a .bench file offset is within a TypeScript code block
    pub fn contains_offset(&self, bench_offset: usize) -> bool {
        self.section_mappings.iter().any(|m| {
            bench_offset >= m.bench_span.start && bench_offset < m.bench_span.end
        })
    }

    /// Find the block containing a .bench file offset
    pub fn block_at_offset(&self, bench_offset: usize) -> Option<&SectionMapping> {
        self.section_mappings.iter().find(|m| {
            bench_offset >= m.bench_span.start && bench_offset < m.bench_span.end
        })
    }
}

/// Builder for constructing virtual TypeScript files
struct VirtualTsFileBuilder {
    bench_uri: String,
    uri: String,
    path: String,
    content: String,
    version: i32,
    current_line: u32,
    section_mappings: Vec<SectionMapping>,
}

impl VirtualTsFileBuilder {
    fn new(bench_uri: &str, bench_path: &str, ts_module_root: &str, version: i32) -> Self {
        // Generate a unique filename based on the bench file path
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

    fn build(&mut self, blocks: &[&EmbeddedBlock]) {
        // Separate blocks by type
        let imports: Vec<_> = blocks.iter()
            .filter(|b| b.block_type == BlockType::SetupImport)
            .collect();
        let declares: Vec<_> = blocks.iter()
            .filter(|b| b.block_type == BlockType::SetupDeclare)
            .collect();
        let helpers: Vec<_> = blocks.iter()
            .filter(|b| b.block_type == BlockType::SetupHelpers)
            .collect();
        let inits: Vec<_> = blocks.iter()
            .filter(|b| b.block_type == BlockType::SetupInit)
            .collect();
        let other: Vec<_> = blocks.iter()
            .filter(|b| !matches!(b.block_type, 
                BlockType::SetupImport | BlockType::SetupDeclare | 
                BlockType::SetupHelpers | BlockType::SetupInit))
            .collect();

        // Write imports - TypeScript imports are already complete statements
        for block in &imports {
            self.add_block_content(block);
            self.write_line("");
        }

        // Write declarations (types, interfaces, etc.)
        for block in &declares {
            self.add_block_content(block);
            self.write_line("");
        }

        // Write helpers (functions, constants, etc.)
        for block in &helpers {
            self.add_block_content(block);
            self.write_line("");
        }

        // Write init code - in TypeScript, this is just top-level code
        for block in &inits {
            self.add_block_content(block);
            self.write_line("");
        }

        // Write other blocks (fixtures, benchmarks, etc.) wrapped in functions
        for (i, block) in other.iter().enumerate() {
            let func_name = match block.block_type {
                BlockType::Fixture => format!("__polybench_fixture_{}", i),
                BlockType::Benchmark => format!("__polybench_bench_{}", i),
                BlockType::Hook => format!("__polybench_hook_{}", i),
                BlockType::Skip => format!("__polybench_skip_{}", i),
                BlockType::Validate => format!("__polybench_validate_{}", i),
                _ => format!("__polybench_block_{}", i),
            };
            
            self.write_line(&format!("function {}() {{", func_name));
            self.add_block_content(block);
            self.write_line("}");
            self.write_line("");
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
        
        // Record the mapping
        self.section_mappings.push(SectionMapping {
            virtual_start_line: self.current_line,
            line_count,
            bench_span: block.span.clone(),
            block_type: block.block_type,
            code: code.clone(),
        });

        // Write the code
        self.content.push_str(code);
        if !code.ends_with('\n') {
            self.content.push('\n');
        }
        
        self.current_line += line_count;
    }

    fn finish(self) -> VirtualTsFile {
        VirtualTsFile {
            uri: self.uri,
            path: self.path,
            content: self.content,
            version: self.version,
            section_mappings: self.section_mappings,
            bench_uri: self.bench_uri,
        }
    }
}

/// Manager for virtual TypeScript files
pub struct VirtualTsFileManager {
    /// Virtual files indexed by .bench URI
    files: dashmap::DashMap<String, VirtualTsFile>,
    /// Track which ts_module_roots we've already ensured have tsconfig.json
    initialized_roots: dashmap::DashMap<String, ()>,
}

impl VirtualTsFileManager {
    pub fn new() -> Self {
        Self {
            files: dashmap::DashMap::new(),
            initialized_roots: dashmap::DashMap::new(),
        }
    }

    /// Ensure a tsconfig.json exists in the TypeScript module root
    /// This is needed for typescript-language-server to resolve modules correctly
    fn ensure_tsconfig(&self, ts_module_root: &str) {
        // Only do this once per root
        if self.initialized_roots.contains_key(ts_module_root) {
            return;
        }

        let tsconfig_path = Path::new(ts_module_root).join("tsconfig.json");
        
        if !tsconfig_path.exists() {
            let tsconfig_content = templates::tsconfig_json();
            match std::fs::write(&tsconfig_path, tsconfig_content) {
                Ok(()) => eprintln!("[tsserver] Created tsconfig.json at {}", tsconfig_path.display()),
                Err(e) => eprintln!("[tsserver] Failed to create tsconfig.json: {}", e),
            }
        }

        self.initialized_roots.insert(ts_module_root.to_string(), ());
    }

    /// Get or create a virtual file for a .bench document
    /// 
    /// This also writes the virtual file to disk so the language server can access it.
    pub fn get_or_create(
        &self,
        bench_uri: &str,
        bench_path: &str,
        ts_module_root: &str,
        blocks: &[&EmbeddedBlock],
        version: i32,
    ) -> VirtualTsFile {
        // Ensure tsconfig.json exists for proper module resolution
        self.ensure_tsconfig(ts_module_root);

        // Check if we have an up-to-date version
        if let Some(existing) = self.files.get(bench_uri) {
            if existing.version >= version {
                // Clone the virtual file data we need
                return VirtualTsFile {
                    uri: existing.uri.clone(),
                    path: existing.path.clone(),
                    content: existing.content.clone(),
                    version: existing.version,
                    section_mappings: existing.section_mappings.clone(),
                    bench_uri: existing.bench_uri.clone(),
                };
            }
        }

        // Create new virtual file
        let virtual_file = VirtualTsFile::from_blocks(
            bench_uri,
            bench_path,
            ts_module_root,
            blocks,
            version,
        );

        eprintln!("[tsserver] Creating virtual file from {} TS blocks", blocks.len());

        // Write the virtual file to disk so the language server can access it
        if let Err(e) = std::fs::write(&virtual_file.path, &virtual_file.content) {
            eprintln!("[tsserver] Failed to write virtual file {}: {}", virtual_file.path, e);
        } else {
            eprintln!("[tsserver] Wrote virtual file: {} ({} bytes)", virtual_file.path, virtual_file.content.len());
        }

        // Store it (we need to clone since we're returning ownership)
        let result = VirtualTsFile {
            uri: virtual_file.uri.clone(),
            path: virtual_file.path.clone(),
            content: virtual_file.content.clone(),
            version: virtual_file.version,
            section_mappings: virtual_file.section_mappings.clone(),
            bench_uri: virtual_file.bench_uri.clone(),
        };
        
        self.files.insert(bench_uri.to_string(), virtual_file);
        
        result
    }

    /// Remove a virtual file when the .bench file is closed
    pub fn remove(&self, bench_uri: &str) {
        if let Some((_, vf)) = self.files.remove(bench_uri) {
            // Also delete the file from disk
            if let Err(e) = std::fs::remove_file(&vf.path) {
                // Don't warn if file doesn't exist
                if e.kind() != std::io::ErrorKind::NotFound {
                    eprintln!("[tsserver] Failed to remove virtual file {}: {}", vf.path, e);
                }
            }
        }
    }

    /// Get a virtual file if it exists
    #[allow(dead_code)]
    pub fn get(&self, bench_uri: &str) -> Option<VirtualTsFile> {
        self.files.get(bench_uri).map(|r| VirtualTsFile {
            uri: r.uri.clone(),
            path: r.path.clone(),
            content: r.content.clone(),
            version: r.version,
            section_mappings: r.section_mappings.clone(),
            bench_uri: r.bench_uri.clone(),
        })
    }
}

impl Default for VirtualTsFileManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poly_bench::dsl::Lang;

    fn make_block(code: &str, block_type: BlockType, start: usize) -> EmbeddedBlock {
        let end = start + code.len();
        EmbeddedBlock {
            lang: Lang::Go,
            block_type,
            code: code.to_string(),
            span: Span {
                start,
                end,
                line: 1,
                col: 1,
            },
            context_name: "test".to_string(),
        }
    }

    #[test]
    fn test_virtual_file_generation() {
        let import_block = make_block("\"fmt\"", BlockType::SetupImport, 100);
        let bench_block = make_block("fmt.Println(\"hello\")", BlockType::Benchmark, 200);
        
        let blocks: Vec<&EmbeddedBlock> = vec![&import_block, &bench_block];
        
        let vf = VirtualGoFile::from_blocks(
            "file:///test.bench",
            "/test.bench",
            "/tmp/go",
            &blocks,
            1,
        );
        
        assert!(vf.content.contains("package main"));
        assert!(vf.content.contains("import ("));
        assert!(vf.content.contains("\"fmt\""));
        assert!(vf.content.contains("fmt.Println"));
        assert_eq!(vf.section_mappings.len(), 2);
    }

    #[test]
    fn test_position_translation() {
        let code = "fmt.Println(\"hello\")";
        let block = make_block(code, BlockType::Benchmark, 100);
        let blocks: Vec<&EmbeddedBlock> = vec![&block];
        
        let vf = VirtualGoFile::from_blocks(
            "file:///test.bench",
            "/test.bench",
            "/tmp/go",
            &blocks,
            1,
        );
        
        // Test bench_to_go - offset 100 is start of block
        let pos = vf.bench_to_go(100);
        assert!(pos.is_some());
        let pos = pos.unwrap();
        assert_eq!(pos.character, 0);
        
        // Test go_to_bench - should translate back
        let offset = vf.go_to_bench(pos.line, pos.character);
        assert!(offset.is_some());
        assert_eq!(offset.unwrap(), 100);
    }
}
