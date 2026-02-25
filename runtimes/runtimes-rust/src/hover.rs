//! Rust embedded code hover via rust-analyzer

use poly_bench_lsp_traits::{EmbeddedHoverContext, EmbeddedHoverProvider};
use poly_bench_syntax::Lang;
use tower_lsp::lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind};

/// Rust embedded hover provider
pub(crate) struct RustEmbeddedHoverProvider;

impl EmbeddedHoverProvider for RustEmbeddedHoverProvider {
    fn lang(&self) -> Lang {
        Lang::Rust
    }

    fn get_hover(&self, ctx: &dyn EmbeddedHoverContext) -> Option<Hover> {
        let module_root = ctx.module_root();

        let virtual_file = ctx.get_virtual_file(Lang::Rust)?;
        let client = ctx.get_lsp_client(poly_bench_dsl::Lang::Rust, module_root)?;

        let bench_offset = ctx.bench_offset();
        let rust_position = virtual_file.bench_to_virtual(bench_offset)?;

        if let Err(e) =
            client.did_change(virtual_file.uri(), virtual_file.content(), virtual_file.version())
        {
            tracing::warn!("Failed to sync virtual Rust file: {}", e);
            return None;
        }

        match client.hover(virtual_file.uri(), rust_position.line, rust_position.character) {
            Ok(Some(mut hover)) => {
                hover.contents = enhance_rust_hover_content(hover.contents);

                if let Some(ref rust_range) = hover.range {
                    if let Some(bench_start_offset) = virtual_file
                        .virtual_to_bench(rust_range.start.line, rust_range.start.character)
                    {
                        if let Some(bench_end_offset) = virtual_file
                            .virtual_to_bench(rust_range.end.line, rust_range.end.character)
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
                tracing::warn!("rust-analyzer hover failed: {}", e);
                None
            }
        }
    }
}

pub(crate) static RUST_EMBEDDED_HOVER_PROVIDER: RustEmbeddedHoverProvider =
    RustEmbeddedHoverProvider;

/// Enhance Rust hover content with proper markdown formatting
fn enhance_rust_hover_content(contents: HoverContents) -> HoverContents {
    match contents {
        HoverContents::Markup(markup) => {
            let value = &markup.value;

            if value.contains("```rust") || value.contains("```rs") {
                return HoverContents::Markup(markup);
            }

            let lines: Vec<&str> = value.lines().collect();
            if lines.is_empty() {
                return HoverContents::Markup(markup);
            }

            let mut formatted_parts = Vec::new();
            let mut code_lines = Vec::new();
            let mut doc_lines = Vec::new();
            let mut in_signature = false;
            let mut seen_signature = false;

            for line in &lines {
                let trimmed = line.trim();

                if trimmed.is_empty() {
                    if in_signature && !code_lines.is_empty() {
                        formatted_parts.push(format!("```rust\n{}\n```", code_lines.join("\n")));
                        code_lines.clear();
                        in_signature = false;
                        seen_signature = true;
                    }
                    continue;
                }

                if trimmed.starts_with("_lsp_virtual") || trimmed.starts_with("polybench_runner") {
                    continue;
                }

                if !seen_signature &&
                    !in_signature &&
                    !trimmed.contains(' ') &&
                    trimmed.chars().all(|c| c.is_alphanumeric() || c == '_' || c == ':')
                {
                    continue;
                }

                let is_signature_start = trimmed.starts_with("fn ") ||
                    trimmed.starts_with("pub fn ") ||
                    trimmed.starts_with("async fn ") ||
                    trimmed.starts_with("pub async fn ") ||
                    trimmed.starts_with("struct ") ||
                    trimmed.starts_with("pub struct ") ||
                    trimmed.starts_with("type ") ||
                    trimmed.starts_with("pub type ") ||
                    trimmed.starts_with("const ") ||
                    trimmed.starts_with("pub const ") ||
                    trimmed.starts_with("impl ") ||
                    trimmed.starts_with("impl<") ||
                    trimmed.starts_with("trait ") ||
                    trimmed.starts_with("pub trait ") ||
                    trimmed.starts_with("enum ") ||
                    trimmed.starts_with("pub enum ") ||
                    trimmed.starts_with("use ") ||
                    trimmed.starts_with("pub use ");

                if is_signature_start {
                    if !code_lines.is_empty() {
                        formatted_parts.push(format!("```rust\n{}\n```", code_lines.join("\n")));
                        code_lines.clear();
                    }
                    in_signature = true;
                    code_lines.push(*line);
                } else if in_signature &&
                    (trimmed.starts_with("where") ||
                        trimmed.ends_with(',') ||
                        trimmed.ends_with('{') ||
                        trimmed.ends_with('>'))
                {
                    code_lines.push(*line);
                } else {
                    if in_signature && !code_lines.is_empty() {
                        formatted_parts.push(format!("```rust\n{}\n```", code_lines.join("\n")));
                        code_lines.clear();
                        in_signature = false;
                        seen_signature = true;
                    }
                    doc_lines.push(*line);
                }
            }

            if !code_lines.is_empty() {
                formatted_parts.push(format!("```rust\n{}\n```", code_lines.join("\n")));
            }

            if !doc_lines.is_empty() {
                let doc_text = doc_lines.join(" ");
                let cleaned: String = doc_text.split_whitespace().collect::<Vec<_>>().join(" ");

                let max_doc_len = 400;
                let truncated_doc = if cleaned.len() > max_doc_len {
                    let truncate_at = cleaned[..max_doc_len]
                        .rfind(". ")
                        .map(|i| i + 1)
                        .or_else(|| cleaned[..max_doc_len].rfind(' '))
                        .unwrap_or(max_doc_len);
                    format!("{}...", &cleaned[..truncate_at])
                } else {
                    cleaned
                };

                if !truncated_doc.is_empty() {
                    formatted_parts.push(truncated_doc);
                }
            }

            if formatted_parts.is_empty() && !value.trim().is_empty() {
                let trimmed_value = value.trim();
                if trimmed_value.contains("fn ") ||
                    trimmed_value.contains("struct ") ||
                    trimmed_value.contains("::")
                {
                    let display_value = if trimmed_value.len() > 500 {
                        format!("{}...", &trimmed_value[..500])
                    } else {
                        trimmed_value.to_string()
                    };
                    formatted_parts.push(format!("```rust\n{}\n```", display_value));
                } else {
                    formatted_parts.push(trimmed_value.to_string());
                }
            }

            HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: formatted_parts.join("\n\n"),
            })
        }
        other => other,
    }
}
