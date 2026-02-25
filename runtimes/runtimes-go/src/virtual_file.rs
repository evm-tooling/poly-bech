//! Virtual Go file generation for LSP integration

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

/// Virtual Go file generated from embedded blocks
struct VirtualGoFile(VirtualFileData);

impl VirtualGoFile {
    fn from_params(params: VirtualFileParams<'_>) -> Self {
        let (uri, path) = compute_go_path(params.bench_path, params.module_root);
        let mut core = VirtualFileBuilderCore::new(params.bench_uri, uri, path, params.version);
        build_go(&mut core, params.blocks, params.fixture_names);
        Self(core.finish())
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

fn compute_go_path(bench_path: &str, go_mod_root: &str) -> (String, String) {
    let mut hasher = DefaultHasher::new();
    bench_path.hash(&mut hasher);
    let hash = hasher.finish();
    let subdir = Path::new(go_mod_root).join(".lsp_virtual");
    let filename = format!("virtual_{:016x}.go", hash);
    let path = subdir.join(&filename);
    let path_str = path.to_string_lossy().to_string();
    let uri = format!("file://{}", path_str);
    (uri, path_str)
}

fn build_go(
    core: &mut VirtualFileBuilderCore,
    blocks: &[&EmbeddedBlock],
    fixture_names: &[String],
) {
    let (imports, declares, helpers, inits, other) =
        VirtualFileBuilderCore::categorize_blocks(blocks);

    core.write_line("package main");
    core.write_line("");

    if !imports.is_empty() {
        core.write_line("import (");
        for block in &imports {
            core.add_block_content(block);
        }
        core.write_line(")");
        core.write_line("");
    }

    for block in &declares {
        core.add_block_content(block);
        core.write_line("");
    }

    for block in &helpers {
        core.add_block_content(block);
        core.write_line("");
    }

    if !inits.is_empty() {
        core.write_line("func init() {");
        for block in &inits {
            core.add_block_content(block);
        }
        core.write_line("}");
        core.write_line("");
    }

    for (i, block) in other.iter().enumerate() {
        let func_name = VirtualFileBuilderCore::func_name_for_block(block.block_type, i);
        core.write_line(&format!("func {}() {{", func_name));
        if matches!(block.block_type, BlockType::Benchmark | BlockType::Validate | BlockType::Skip)
        {
            for fixture_name in fixture_names {
                core.write_line(&format!("\tvar {} []byte", fixture_name));
            }
            for fixture_name in fixture_names {
                core.write_line(&format!("\t_ = {}", fixture_name));
            }
        }
        core.add_block_content(block);
        core.write_line("}");
        core.write_line("");
    }
}

pub(crate) struct GoVirtualFileBuilder;

impl VirtualFileBuilder for GoVirtualFileBuilder {
    fn lang(&self) -> Lang {
        Lang::Go
    }
    fn build(&self, params: VirtualFileParams<'_>) -> Box<dyn VirtualFile> {
        Box::new(VirtualGoFile::from_params(params))
    }
}

pub static GO_VIRTUAL_FILE_BUILDER: GoVirtualFileBuilder = GoVirtualFileBuilder;
