//! C# embedded code hover via csharp-ls.

use poly_bench_lsp_traits::{EmbeddedHoverContext, EmbeddedHoverProvider};
use poly_bench_syntax::Lang;
use tower_lsp::lsp_types::Hover;

pub(crate) struct CSharpEmbeddedHoverProvider;

impl EmbeddedHoverProvider for CSharpEmbeddedHoverProvider {
    fn lang(&self) -> Lang {
        Lang::CSharp
    }

    fn get_hover(&self, ctx: &dyn EmbeddedHoverContext) -> Option<Hover> {
        let module_root = ctx.module_root();

        let virtual_file = match ctx.get_virtual_file(Lang::CSharp) {
            Some(vf) => vf,
            None => {
                tracing::debug!("[csharp-hover] no virtual file available");
                return None;
            }
        };
        let client = match ctx.get_lsp_client(poly_bench_dsl::Lang::CSharp, module_root) {
            Some(c) => c,
            None => {
                eprintln!(
                    "[poly-bench:csharp-hover] no csharp-ls client for module_root={}",
                    module_root
                );
                tracing::warn!(
                    "[csharp-hover] no csharp-ls client for module_root={}",
                    module_root
                );
                return None;
            }
        };

        let bench_offset = ctx.bench_offset();
        let cs_position = virtual_file.bench_to_virtual(bench_offset)?;

        if let Err(e) =
            client.did_change(virtual_file.uri(), virtual_file.content(), virtual_file.version())
        {
            tracing::warn!("Failed to sync virtual C# file: {}", e);
            return None;
        }

        tracing::trace!(
            "[csharp-hover] requesting hover uri={} pos={}:{} module_root={}",
            virtual_file.uri(),
            cs_position.line,
            cs_position.character,
            module_root
        );

        match client.hover(virtual_file.uri(), cs_position.line, cs_position.character) {
            Ok(Some(mut hover)) => {
                if let Some(ref cs_range) = hover.range {
                    if let Some(bench_start_offset) =
                        virtual_file.virtual_to_bench(cs_range.start.line, cs_range.start.character)
                    {
                        if let Some(bench_end_offset) =
                            virtual_file.virtual_to_bench(cs_range.end.line, cs_range.end.character)
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
                tracing::trace!("[csharp-hover] hover resolved");
                Some(hover)
            }
            Ok(None) => {
                tracing::debug!("[csharp-hover] no hover returned by csharp-ls");
                None
            }
            Err(e) => {
                eprintln!("[poly-bench:csharp-hover] csharp-ls hover failed: {}", e);
                tracing::warn!("[csharp-hover] csharp-ls hover failed: {}", e);
                None
            }
        }
    }
}

pub(crate) static CSHARP_EMBEDDED_HOVER_PROVIDER: CSharpEmbeddedHoverProvider =
    CSharpEmbeddedHoverProvider;
