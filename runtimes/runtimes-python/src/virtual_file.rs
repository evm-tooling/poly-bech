//! Virtual Python file generation for LSP integration

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

/// Normalize Python code indentation by stripping common leading whitespace
fn normalize_python_indent(code: &str) -> String {
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

struct VirtualPythonFile(VirtualFileData);

impl VirtualPythonFile {
    fn from_params(params: VirtualFileParams<'_>) -> Self {
        let (uri, path) = compute_python_path(params.bench_path, params.module_root);
        let mut core = VirtualFileBuilderCore::new(params.bench_uri, uri, path, params.version);
        build_python(&mut core, params.blocks, params.fixture_names);
        Self(core.finish())
    }
}

impl VirtualFile for VirtualPythonFile {
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

fn compute_python_path(bench_path: &str, python_root: &str) -> (String, String) {
    let mut hasher = DefaultHasher::new();
    bench_path.hash(&mut hasher);
    let hash = hasher.finish();
    let subdir = Path::new(python_root).join(".lsp_virtual");
    let filename = format!("_lsp_virtual_{:016x}.py", hash);
    let path = subdir.join(&filename);
    let path_str = path.to_string_lossy().to_string();
    let uri = format!("file://{}", path_str);
    (uri, path_str)
}

fn build_python(
    core: &mut VirtualFileBuilderCore,
    blocks: &[&EmbeddedBlock],
    fixture_names: &[String],
) {
    let (imports, declares, helpers, inits, other) =
        VirtualFileBuilderCore::categorize_blocks(blocks);

    for block in &imports {
        core.add_block_content_normalized(block, normalize_python_indent);
        core.write_line("");
    }
    for block in &declares {
        core.add_block_content_normalized(block, normalize_python_indent);
        core.write_line("");
    }
    for block in &helpers {
        core.add_block_content_normalized(block, normalize_python_indent);
        core.write_line("");
    }
    for block in &inits {
        core.add_block_content_normalized(block, normalize_python_indent);
        core.write_line("");
    }
    for (i, block) in other.iter().enumerate() {
        let func_name = VirtualFileBuilderCore::func_name_for_block(block.block_type, i);
        core.write_line(&format!("def {}():", func_name));
        if matches!(block.block_type, BlockType::Benchmark | BlockType::Validate | BlockType::Skip)
        {
            for fixture_name in fixture_names {
                core.write_line(&format!("    {} = bytes()", fixture_name));
            }
            for fixture_name in fixture_names {
                core.write_line(&format!("    _ = {}", fixture_name));
            }
        }
        core.add_block_content_indented(block, "    ", normalize_python_indent);
        core.write_line("");
    }
}

pub(crate) struct PythonVirtualFileBuilder;

impl VirtualFileBuilder for PythonVirtualFileBuilder {
    fn lang(&self) -> Lang {
        Lang::Python
    }
    fn build(&self, params: VirtualFileParams<'_>) -> Box<dyn VirtualFile> {
        Box::new(VirtualPythonFile::from_params(params))
    }
}

pub static PYTHON_VIRTUAL_FILE_BUILDER: PythonVirtualFileBuilder = PythonVirtualFileBuilder;
