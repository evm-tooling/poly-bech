//! Virtual TypeScript file generation for LSP integration

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

struct VirtualTsFile(VirtualFileData);

impl VirtualTsFile {
    fn from_params(params: VirtualFileParams<'_>) -> Self {
        let (uri, path) = compute_ts_path(params.bench_path, params.module_root);
        let mut core = VirtualFileBuilderCore::new(params.bench_uri, uri, path, params.version);
        build_ts(&mut core, params.blocks, params.fixture_names);
        Self(core.finish())
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

fn compute_ts_path(bench_path: &str, ts_module_root: &str) -> (String, String) {
    let mut hasher = DefaultHasher::new();
    bench_path.hash(&mut hasher);
    let hash = hasher.finish();
    let filename = format!("polybench_virtual_{:016x}.ts", hash);
    let path = Path::new(ts_module_root).join(&filename);
    let path_str = path.to_string_lossy().to_string();
    let uri = format!("file://{}", path_str);
    (uri, path_str)
}

fn build_ts(
    core: &mut VirtualFileBuilderCore,
    blocks: &[&EmbeddedBlock],
    fixture_names: &[String],
) {
    let (imports, declares, helpers, inits, other) =
        VirtualFileBuilderCore::categorize_blocks(blocks);

    for block in &imports {
        core.add_block_content(block);
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
    for block in &inits {
        core.add_block_content(block);
        core.write_line("");
    }
    for (i, block) in other.iter().enumerate() {
        let func_name = VirtualFileBuilderCore::func_name_for_block(block.block_type, i);
        core.write_line(&format!("function {}() {{", func_name));
        if matches!(block.block_type, BlockType::Benchmark | BlockType::Validate | BlockType::Skip)
        {
            for fixture_name in fixture_names {
                core.write_line(&format!("  declare const {}: Uint8Array;", fixture_name));
            }
        }
        core.add_block_content(block);
        core.write_line("}");
        core.write_line("");
    }
}

pub(crate) struct TsVirtualFileBuilder;

impl VirtualFileBuilder for TsVirtualFileBuilder {
    fn lang(&self) -> Lang {
        Lang::TypeScript
    }
    fn build(&self, params: VirtualFileParams<'_>) -> Box<dyn VirtualFile> {
        Box::new(VirtualTsFile::from_params(params))
    }
}

pub static TS_VIRTUAL_FILE_BUILDER: TsVirtualFileBuilder = TsVirtualFileBuilder;
