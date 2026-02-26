//! Virtual Zig file generation for LSP integration.

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    path::Path,
};

use poly_bench_dsl::Lang;
use poly_bench_lsp_traits::{
    BlockType, EmbeddedBlock, SectionMapping, VirtualFile, VirtualFileBuilder,
    VirtualFileBuilderCore, VirtualFileData, VirtualFileParams,
};

fn normalize_zig_indent(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();
    if lines.is_empty() {
        return String::new();
    }
    let min_indent = lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min()
        .unwrap_or(0);
    lines
        .iter()
        .map(|l| {
            if l.trim().is_empty() {
                l.to_string()
            } else {
                l.get(min_indent.min(l.len())..).unwrap_or(l).to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

struct VirtualZigFile(VirtualFileData);

impl VirtualZigFile {
    fn from_params(params: VirtualFileParams<'_>) -> Self {
        let (uri, path) = compute_zig_path(params.bench_path, params.module_root);
        let mut core = VirtualFileBuilderCore::new(params.bench_uri, uri, path, params.version);
        build_zig(&mut core, params.blocks, params.fixture_names);
        Self(core.finish())
    }
}

impl VirtualFile for VirtualZigFile {
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

fn compute_zig_path(bench_path: &str, zig_root: &str) -> (String, String) {
    let mut hasher = DefaultHasher::new();
    bench_path.hash(&mut hasher);
    let hash = hasher.finish();
    let subdir = Path::new(zig_root).join(".lsp_virtual");
    let filename = format!("_lsp_virtual_{:016x}.zig", hash);
    let path = subdir.join(&filename);
    let path_str = path.to_string_lossy().to_string();
    let uri = format!("file://{}", path_str);
    (uri, path_str)
}

fn build_zig(
    core: &mut VirtualFileBuilderCore,
    blocks: &[&EmbeddedBlock],
    fixture_names: &[String],
) {
    let (imports, declares, helpers, inits, other) =
        VirtualFileBuilderCore::categorize_blocks(blocks);

    core.write_line("const std = @import(\"std\");");
    core.write_line("");

    for block in &imports {
        core.add_block_content_normalized(block, normalize_zig_indent);
        core.write_line("");
    }
    for block in &declares {
        core.add_block_content_normalized(block, normalize_zig_indent);
        core.write_line("");
    }
    for block in &helpers {
        core.add_block_content_normalized(block, normalize_zig_indent);
        core.write_line("");
    }
    for block in &inits {
        core.write_line("fn __polybench_init() void {");
        core.add_block_content_indented(block, "    ", normalize_zig_indent);
        core.write_line("}");
        core.write_line("");
    }
    for (i, block) in other.iter().enumerate() {
        let func_name = VirtualFileBuilderCore::func_name_for_block(block.block_type, i);
        core.write_line(&format!("fn {}() void {{", func_name));
        if matches!(block.block_type, BlockType::Benchmark | BlockType::Validate | BlockType::Skip)
        {
            for fixture_name in fixture_names {
                core.write_line(&format!("    var {}: [1]u8 = .{{0}};", fixture_name));
                core.write_line(&format!("    _ = {};", fixture_name));
            }
        }
        core.add_block_content_indented(block, "    ", normalize_zig_indent);
        core.write_line("}");
        core.write_line("");
    }
}

pub(crate) struct ZigVirtualFileBuilder;

impl VirtualFileBuilder for ZigVirtualFileBuilder {
    fn lang(&self) -> Lang {
        Lang::Zig
    }
    fn build(&self, params: VirtualFileParams<'_>) -> Box<dyn VirtualFile> {
        Box::new(VirtualZigFile::from_params(params))
    }
}

pub static ZIG_VIRTUAL_FILE_BUILDER: ZigVirtualFileBuilder = ZigVirtualFileBuilder;
