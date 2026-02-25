//! TypeScript embedded code hover via typescript-language-server

use poly_bench_lsp_traits::{EmbeddedHoverContext, EmbeddedHoverProvider};
use poly_bench_syntax::Lang;
use tower_lsp::lsp_types::Hover;

/// TypeScript embedded hover provider
pub(crate) struct TsEmbeddedHoverProvider;

impl EmbeddedHoverProvider for TsEmbeddedHoverProvider {
    fn lang(&self) -> Lang {
        Lang::TypeScript
    }

    fn get_hover(&self, ctx: &dyn EmbeddedHoverContext) -> Option<Hover> {
        let module_root = ctx.module_root();

        let virtual_file = ctx.get_virtual_file(Lang::TypeScript)?;
        let client = ctx.get_ts_client(module_root)?;

        let bench_offset = ctx.bench_offset();
        let ts_position = virtual_file.bench_to_virtual(bench_offset)?;

        if let Err(e) =
            client.did_change(virtual_file.uri(), virtual_file.content(), virtual_file.version())
        {
            tracing::warn!("Failed to sync virtual TS file: {}", e);
            return None;
        }

        match client.hover(virtual_file.uri(), ts_position.line, ts_position.character) {
            Ok(Some(mut hover)) => {
                if let Some(ref ts_range) = hover.range {
                    if let Some(bench_start_offset) =
                        virtual_file.virtual_to_bench(ts_range.start.line, ts_range.start.character)
                    {
                        if let Some(bench_end_offset) =
                            virtual_file.virtual_to_bench(ts_range.end.line, ts_range.end.character)
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
                tracing::warn!("tsserver hover failed: {}", e);
                None
            }
        }
    }
}

pub(crate) static TS_EMBEDDED_HOVER_PROVIDER: TsEmbeddedHoverProvider = TsEmbeddedHoverProvider;
