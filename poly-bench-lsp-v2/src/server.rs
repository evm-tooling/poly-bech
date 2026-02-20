//! Main LSP server implementation
//!
//! This module implements the Language Server Protocol handlers
//! using tower-lsp.

use crate::{
    diagnostics::compute_diagnostics,
    document::Document,
    formatter::format_document,
    semantic_tokens::{get_semantic_tokens, LEGEND},
};

use dashmap::DashMap;
use tower_lsp::{jsonrpc::Result, lsp_types::*, Client, LanguageServer};
use tracing::{debug, info};

/// The poly-bench language server
pub struct PolyBenchLanguageServer {
    /// LSP client for sending notifications
    client: Client,
    /// Open documents
    documents: DashMap<Url, Document>,
}

impl PolyBenchLanguageServer {
    /// Create a new language server
    pub fn new(client: Client) -> Self {
        Self { client, documents: DashMap::new() }
    }

    /// Publish diagnostics for a document
    async fn publish_diagnostics(&self, uri: &Url) {
        if let Some(doc) = self.documents.get(uri) {
            let diagnostics = compute_diagnostics(&doc);
            self.client.publish_diagnostics(uri.clone(), diagnostics, Some(doc.version)).await;
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for PolyBenchLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        info!("Initializing poly-bench LSP v2");

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::INCREMENTAL),
                        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                            include_text: Some(false),
                        })),
                        ..Default::default()
                    },
                )),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            legend: LEGEND.clone(),
                            full: Some(SemanticTokensFullOptions::Bool(true)),
                            range: Some(false),
                            ..Default::default()
                        },
                    ),
                ),
                document_formatting_provider: Some(OneOf::Left(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![
                        ".".to_string(),
                        ":".to_string(),
                        "\"".to_string(),
                    ]),
                    ..Default::default()
                }),
                definition_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "poly-bench-lsp-v2".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        info!("poly-bench LSP v2 initialized");
    }

    async fn shutdown(&self) -> Result<()> {
        info!("Shutting down poly-bench LSP v2");
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        let version = params.text_document.version;

        debug!("Document opened: {}", uri);

        let doc = Document::new(uri.clone(), text, version);
        self.documents.insert(uri.clone(), doc);

        self.publish_diagnostics(&uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let version = params.text_document.version;

        debug!("Document changed: {} (v{})", uri, version);

        if let Some(mut doc) = self.documents.get_mut(&uri) {
            for change in params.content_changes {
                doc.apply_edit(&change, version);
            }
        }

        self.publish_diagnostics(&uri).await;
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri;
        debug!("Document saved: {}", uri);

        self.publish_diagnostics(&uri).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        debug!("Document closed: {}", uri);

        self.documents.remove(&uri);

        self.client.publish_diagnostics(uri, vec![], None).await;
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri;

        if let Some(doc) = self.documents.get(&uri) {
            let tokens = get_semantic_tokens(&doc);
            Ok(Some(SemanticTokensResult::Tokens(SemanticTokens { result_id: None, data: tokens })))
        } else {
            Ok(None)
        }
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri;

        if let Some(doc) = self.documents.get(&uri) {
            let edits = format_document(&doc);
            Ok(Some(edits))
        } else {
            Ok(None)
        }
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        if let Some(doc) = self.documents.get(&uri) {
            let source = doc.source_text();
            let point =
                tree_sitter::Point::new(position.line as usize, position.character as usize);

            if let Some(node) = doc.tree.root_node().descendant_for_point_range(point, point) {
                let kind = node.kind();
                let text = node.utf8_text(source.as_bytes()).unwrap_or("");

                let content = match kind {
                    "suite" => {
                        let name = node
                            .child_by_field_name("name")
                            .and_then(|n| n.utf8_text(source.as_bytes()).ok())
                            .unwrap_or("<unnamed>");
                        format!("**Suite**: `{}`", name)
                    }
                    "benchmark" => {
                        let name = node
                            .child_by_field_name("name")
                            .and_then(|n| n.utf8_text(source.as_bytes()).ok())
                            .unwrap_or("<unnamed>");
                        format!("**Benchmark**: `{}`", name)
                    }
                    "fixture" => {
                        let name = node
                            .child_by_field_name("name")
                            .and_then(|n| n.utf8_text(source.as_bytes()).ok())
                            .unwrap_or("<unnamed>");
                        format!("**Fixture**: `{}`", name)
                    }
                    "property_name" => get_property_documentation(text),
                    "chart_function_name" => get_chart_function_documentation(text),
                    "language_tag" => {
                        format!("**Language**: `{}`\n\nEmbedded {} code block", text, text)
                    }
                    _ => return Ok(None),
                };

                return Ok(Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: content,
                    }),
                    range: None,
                }));
            }
        }

        Ok(None)
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        if let Some(doc) = self.documents.get(&uri) {
            let point =
                tree_sitter::Point::new(position.line as usize, position.character as usize);

            if let Some(node) = doc.tree.root_node().descendant_for_point_range(point, point) {
                let mut completions = Vec::new();

                let parent = node.parent();
                let parent_kind = parent.map(|p| p.kind()).unwrap_or("");

                match parent_kind {
                    "suite_body" => {
                        completions.extend(get_suite_body_completions());
                    }
                    "benchmark_body" => {
                        completions.extend(get_benchmark_body_completions());
                    }
                    "fixture_body" => {
                        completions.extend(get_fixture_body_completions());
                    }
                    "after_body" => {
                        completions.extend(get_charting_completions());
                    }
                    _ => {
                        completions.extend(get_top_level_completions());
                    }
                }

                return Ok(Some(CompletionResponse::Array(completions)));
            }
        }

        Ok(None)
    }

    async fn goto_definition(
        &self,
        _params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        Ok(None)
    }
}

fn get_property_documentation(name: &str) -> String {
    match name {
        "description" => {
            "**description**: `string`\n\nA human-readable description of the suite or benchmark."
                .to_string()
        }
        "iterations" => {
            "**iterations**: `number`\n\nNumber of benchmark iterations to run.".to_string()
        }
        "warmup" => {
            "**warmup**: `number`\n\nNumber of warmup iterations before measurement.".to_string()
        }
        "timeout" => {
            "**timeout**: `duration`\n\nMaximum time allowed for the benchmark (e.g., `30s`, `5000ms`)."
                .to_string()
        }
        "order" => {
            "**order**: `sequential | random`\n\nOrder in which benchmarks are executed."
                .to_string()
        }
        "compare" => {
            "**compare**: `boolean`\n\nWhether to compare results across languages.".to_string()
        }
        "baseline" => {
            "**baseline**: `string`\n\nLanguage to use as the baseline for comparison.".to_string()
        }
        "mode" => {
            "**mode**: `auto | fixed | adaptive`\n\nBenchmark execution mode.".to_string()
        }
        "targetTime" => {
            "**targetTime**: `duration`\n\nTarget time for adaptive mode.".to_string()
        }
        "sink" => {
            "**sink**: `boolean`\n\nWhether to sink (consume) the result to prevent optimization."
                .to_string()
        }
        _ => format!("**{}**", name),
    }
}

fn get_chart_function_documentation(name: &str) -> String {
    match name {
        "drawBarChart" => {
            "**drawBarChart**\n\nDraw a bar chart comparing benchmark results.".to_string()
        }
        "drawLineChart" => {
            "**drawLineChart**\n\nDraw a line chart showing performance over time or input size."
                .to_string()
        }
        "drawSpeedupChart" => {
            "**drawSpeedupChart**\n\nDraw a chart showing relative speedup compared to baseline."
                .to_string()
        }
        "drawTable" => "**drawTable**\n\nGenerate a table of benchmark results.".to_string(),
        _ => format!("**{}**", name),
    }
}

fn get_top_level_completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "suite".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("suite ${1:name} {\n\t$0\n}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Define a benchmark suite".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "use std::charting".to_string(),
            kind: Some(CompletionItemKind::MODULE),
            insert_text: Some("use std::charting".to_string()),
            detail: Some("Import charting module".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "use std::anvil".to_string(),
            kind: Some(CompletionItemKind::MODULE),
            insert_text: Some("use std::anvil".to_string()),
            detail: Some("Import anvil module for Ethereum testing".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "globalSetup".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("globalSetup {\n\t$0\n}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Define global setup".to_string()),
            ..Default::default()
        },
    ]
}

fn get_suite_body_completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "bench".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("bench ${1:name} {\n\t${2:go}: ${3:code()}\n}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Define a benchmark".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "fixture".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("fixture ${1:name} {\n\t$0\n}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Define a fixture".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "setup go".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some(
                "setup go {\n\timport (\n\t\t$1\n\t)\n\n\thelpers {\n\t\t$0\n\t}\n}".to_string(),
            ),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Define Go setup block".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "setup ts".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some(
                "setup ts {\n\timport {\n\t\t$1\n\t}\n\n\thelpers {\n\t\t$0\n\t}\n}".to_string(),
            ),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Define TypeScript setup block".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "after".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("after {\n\t$0\n}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Define after block for charting".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "description".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("description: \"$0\"".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "iterations".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("iterations: ${0:1000}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
    ]
}

fn get_benchmark_body_completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "go".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("go: $0".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Go implementation".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "ts".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("ts: $0".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("TypeScript implementation".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "rust".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("rust: $0".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Rust implementation".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "before".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("before ${1:go}: $0".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Before hook".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "after".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("after ${1:go}: $0".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("After hook".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "tags".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("tags: [\"$0\"]".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
    ]
}

fn get_fixture_body_completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "hex".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("hex: \"$0\"".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Hex data".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "hex @file".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("hex: @file(\"$0\")".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Hex data from file".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "go".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("go: $0".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Go fixture implementation".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "ts".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("ts: $0".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("TypeScript fixture implementation".to_string()),
            ..Default::default()
        },
    ]
}

fn get_charting_completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "charting.drawBarChart".to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            insert_text: Some(
                "charting.drawBarChart(\n\ttitle: \"$1\",\n\toutput: \"$2\"\n)".to_string(),
            ),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Draw a bar chart".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "charting.drawLineChart".to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            insert_text: Some(
                "charting.drawLineChart(\n\ttitle: \"$1\",\n\txlabel: \"$2\",\n\tylabel: \"$3\"\n)"
                    .to_string(),
            ),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Draw a line chart".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "charting.drawSpeedupChart".to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            insert_text: Some(
                "charting.drawSpeedupChart(\n\ttitle: \"$1\",\n\tbaselineBenchmark: \"$2\"\n)"
                    .to_string(),
            ),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Draw a speedup chart".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "charting.drawTable".to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            insert_text: Some("charting.drawTable(\n\ttitle: \"$1\"\n)".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Generate a results table".to_string()),
            ..Default::default()
        },
    ]
}
