//! C embedded code hover via clangd.

use poly_bench_syntax::Lang;
use poly_bench_traits::{EmbeddedHoverContext, EmbeddedHoverProvider};
use tower_lsp::lsp_types::Hover;

pub(crate) struct CEmbeddedHoverProvider;

impl EmbeddedHoverProvider for CEmbeddedHoverProvider {
    fn lang(&self) -> Lang {
        Lang::C
    }

    fn get_hover(&self, ctx: &dyn EmbeddedHoverContext) -> Option<Hover> {
        let module_root = ctx.module_root();
        let virtual_file = ctx.get_virtual_file(Lang::C)?;
        let client = ctx.get_lsp_client(poly_bench_dsl::Lang::C, module_root)?;

        let bench_offset = ctx.bench_offset();
        let c_position = virtual_file.bench_to_virtual(bench_offset)?;

        if let Err(e) =
            client.did_change(virtual_file.uri(), virtual_file.content(), virtual_file.version())
        {
            tracing::warn!("Failed to sync virtual C file: {}", e);
            return None;
        }

        match client.hover(virtual_file.uri(), c_position.line, c_position.character) {
            Ok(Some(mut hover)) => {
                if let Some(ref c_range) = hover.range {
                    if let Some(bench_start_offset) =
                        virtual_file.virtual_to_bench(c_range.start.line, c_range.start.character)
                    {
                        if let Some(bench_end_offset) =
                            virtual_file.virtual_to_bench(c_range.end.line, c_range.end.character)
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
                tracing::warn!("clangd hover failed: {}", e);
                None
            }
        }
    }
}

pub(crate) static C_EMBEDDED_HOVER_PROVIDER: CEmbeddedHoverProvider = CEmbeddedHoverProvider;
