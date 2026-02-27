//! Python embedded code hover via pyright/pylsp

use poly_bench_syntax::Lang;
use poly_bench_traits::{EmbeddedHoverContext, EmbeddedHoverProvider};
use tower_lsp::lsp_types::Hover;

/// Python embedded hover provider
pub(crate) struct PythonEmbeddedHoverProvider;

impl EmbeddedHoverProvider for PythonEmbeddedHoverProvider {
    fn lang(&self) -> Lang {
        Lang::Python
    }

    fn get_hover(&self, ctx: &dyn EmbeddedHoverContext) -> Option<Hover> {
        let module_root = ctx.module_root();

        let virtual_file = ctx.get_virtual_file(Lang::Python)?;
        let client = ctx.get_lsp_client(poly_bench_dsl::Lang::Python, module_root)?;

        let bench_offset = ctx.bench_offset();
        let python_position = virtual_file.bench_to_virtual(bench_offset)?;

        if let Err(e) =
            client.did_change(virtual_file.uri(), virtual_file.content(), virtual_file.version())
        {
            tracing::warn!("Failed to sync virtual Python file: {}", e);
            return None;
        }

        match client.hover(virtual_file.uri(), python_position.line, python_position.character) {
            Ok(Some(mut hover)) => {
                if let Some(ref python_range) = hover.range {
                    if let Some(bench_start_offset) = virtual_file
                        .virtual_to_bench(python_range.start.line, python_range.start.character)
                    {
                        if let Some(bench_end_offset) = virtual_file
                            .virtual_to_bench(python_range.end.line, python_range.end.character)
                        {
                            let (start_line, start_col) = ctx.byte_to_position(bench_start_offset);
                            let (end_line, end_col) = ctx.byte_to_position(bench_end_offset);
                            hover.range = Some(tower_lsp::lsp_types::Range {
                                start: tower_lsp::lsp_types::Position {
                                    line: start_line as u32,
                                    character: start_col as u32,
                                },
                                end: tower_lsp::lsp_types::Position {
                                    line: end_line as u32,
                                    character: end_col as u32,
                                },
                            });
                        }
                    }
                }
                Some(hover)
            }
            Ok(None) => None,
            Err(e) => {
                tracing::warn!("pyright/pylsp hover failed: {}", e);
                None
            }
        }
    }
}

pub(crate) static PYTHON_EMBEDDED_HOVER_PROVIDER: PythonEmbeddedHoverProvider =
    PythonEmbeddedHoverProvider;
