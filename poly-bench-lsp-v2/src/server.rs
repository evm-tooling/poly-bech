//! Main LSP server implementation
//!
//! This module implements the Language Server Protocol handlers
//! using tower-lsp.

use std::path::Path;

use crate::{
    diagnostics::compute_diagnostics,
    document::Document,
    embedded::EmbeddedConfig,
    formatter::format_document,
    hover::get_hover,
    hover_cache::invalidate_document_cache,
    semantic_tokens::{get_semantic_tokens, LEGEND},
    virtual_files::VirtualFileManagers,
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
    /// Embedded language configuration (detected from workspace)
    embedded_config: parking_lot::RwLock<EmbeddedConfig>,
    /// Virtual file managers for embedded language support
    virtual_file_managers: VirtualFileManagers,
    /// Workspace root path
    workspace_root: parking_lot::RwLock<Option<String>>,
}

impl PolyBenchLanguageServer {
    /// Create a new language server
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: DashMap::new(),
            embedded_config: parking_lot::RwLock::new(EmbeddedConfig::default()),
            virtual_file_managers: VirtualFileManagers::new(),
            workspace_root: parking_lot::RwLock::new(None),
        }
    }

    /// Publish diagnostics for a document
    async fn publish_diagnostics(&self, uri: &Url) {
        if let Some(doc) = self.documents.get(uri) {
            let diagnostics = compute_diagnostics(&doc);
            self.client.publish_diagnostics(uri.clone(), diagnostics, Some(doc.version)).await;
        }
    }

    /// Detect embedded language configuration from workspace
    fn detect_embedded_config(&self, workspace_root: &str) {
        let mut config = EmbeddedConfig::default();

        // Detect Go module root
        if let Some(go_mod_root) = find_go_mod_root(workspace_root) {
            info!("Detected Go module at: {}", go_mod_root);
            config.go_mod_root = Some(go_mod_root);
        }

        // Detect TypeScript module root
        if let Some(ts_module_root) = find_ts_module_root(workspace_root) {
            info!("Detected TypeScript module at: {}", ts_module_root);
            config.ts_module_root = Some(ts_module_root);
        }

        // Detect Rust project root
        if let Some(rust_project_root) = find_rust_project_root(workspace_root) {
            info!("Detected Rust project at: {}", rust_project_root);
            config.rust_project_root = Some(rust_project_root);
        }

        *self.embedded_config.write() = config;
    }

    /// Update embedded config when a document is opened (detect from document path)
    ///
    /// Always re-detects from the document path since different documents might be
    /// in different poly-bench projects with different runtime environments.
    fn update_config_for_document(&self, uri: &Url) {
        if let Ok(path) = uri.to_file_path() {
            if let Some(parent) = path.parent() {
                let parent_str = parent.to_string_lossy();

                let mut config = self.embedded_config.write();

                // Always re-detect from document path - different documents may have
                // different runtime environments (e.g., in a monorepo with multiple
                // poly-bench projects)
                if let Some(go_mod_root) = find_go_mod_root(&parent_str) {
                    info!("Detected Go module at: {}", go_mod_root);
                    config.go_mod_root = Some(go_mod_root);
                }

                if let Some(ts_module_root) = find_ts_module_root(&parent_str) {
                    info!("Detected TypeScript module at: {}", ts_module_root);
                    config.ts_module_root = Some(ts_module_root);
                }

                if let Some(rust_project_root) = find_rust_project_root(&parent_str) {
                    info!("Detected Rust project at: {}", rust_project_root);
                    config.rust_project_root = Some(rust_project_root);
                }
            }
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for PolyBenchLanguageServer {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        info!("Initializing poly-bench LSP v2");

        // Store workspace root and detect embedded config
        if let Some(root_uri) = params.root_uri {
            if let Ok(path) = root_uri.to_file_path() {
                let root_str = path.to_string_lossy().to_string();
                *self.workspace_root.write() = Some(root_str.clone());
                self.detect_embedded_config(&root_str);
            }
        }

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
        self.virtual_file_managers.clear_all_caches();
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        let version = params.text_document.version;

        debug!("Document opened: {}", uri);

        // Update embedded config if needed
        self.update_config_for_document(&uri);

        let doc = Document::new(uri.clone(), text, version);
        self.documents.insert(uri.clone(), doc);

        self.publish_diagnostics(&uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let version = params.text_document.version;

        debug!("Document changed: {} (v{})", uri, version);

        // Invalidate hover cache for this document
        invalidate_document_cache(&uri);

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

        // Clean up virtual files and cache
        self.virtual_file_managers.remove_all(uri.as_str());
        invalidate_document_cache(&uri);

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
            let config = self.embedded_config.read().clone();
            let hover = get_hover(&doc, position, &config, &uri, &self.virtual_file_managers);
            Ok(hover)
        } else {
            Ok(None)
        }
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

/// Find the Go module root (directory containing go.mod)
///
/// First walks up to find a `.polybench/runtime-env/go/` directory (preferred for poly-bench
/// projects), then falls back to finding a regular `go.mod` file.
fn find_go_mod_root(start_path: &str) -> Option<String> {
    // First pass: look for .polybench/runtime-env/go/ (poly-bench project)
    let mut current = Path::new(start_path);
    loop {
        let polybench_go = current.join(".polybench/runtime-env/go");
        if polybench_go.join("go.mod").exists() {
            return Some(polybench_go.to_string_lossy().to_string());
        }

        match current.parent() {
            Some(parent) => current = parent,
            None => break,
        }
    }

    // Second pass: fall back to regular go.mod lookup
    let mut current = Path::new(start_path);
    loop {
        if current.join("go.mod").exists() {
            return Some(current.to_string_lossy().to_string());
        }

        match current.parent() {
            Some(parent) => current = parent,
            None => return None,
        }
    }
}

/// Find the TypeScript module root (directory containing package.json or node_modules)
///
/// First walks up to find a `.polybench/runtime-env/ts/` directory (preferred for poly-bench
/// projects), then falls back to finding a regular `package.json` or `node_modules`.
fn find_ts_module_root(start_path: &str) -> Option<String> {
    // First pass: look for .polybench/runtime-env/ts/ (poly-bench project)
    let mut current = Path::new(start_path);
    loop {
        let polybench_ts = current.join(".polybench/runtime-env/ts");
        if polybench_ts.join("package.json").exists() || polybench_ts.join("node_modules").exists()
        {
            return Some(polybench_ts.to_string_lossy().to_string());
        }

        match current.parent() {
            Some(parent) => current = parent,
            None => break,
        }
    }

    // Second pass: fall back to regular package.json/node_modules lookup
    let mut current = Path::new(start_path);
    loop {
        let package_json = current.join("package.json");
        let node_modules = current.join("node_modules");

        if package_json.exists() || node_modules.exists() {
            return Some(current.to_string_lossy().to_string());
        }

        match current.parent() {
            Some(parent) => current = parent,
            None => return None,
        }
    }
}

/// Find the Rust project root (directory containing Cargo.toml)
///
/// First walks up to find a `.polybench/runtime-env/rust/` directory (preferred for poly-bench
/// projects), then falls back to finding a regular `Cargo.toml` file.
fn find_rust_project_root(start_path: &str) -> Option<String> {
    // First pass: look for .polybench/runtime-env/rust/ (poly-bench project)
    let mut current = Path::new(start_path);
    loop {
        let polybench_rust = current.join(".polybench/runtime-env/rust");
        if polybench_rust.join("Cargo.toml").exists() {
            return Some(polybench_rust.to_string_lossy().to_string());
        }

        match current.parent() {
            Some(parent) => current = parent,
            None => break,
        }
    }

    // Second pass: fall back to regular Cargo.toml lookup
    let mut current = Path::new(start_path);
    loop {
        if current.join("Cargo.toml").exists() {
            return Some(current.to_string_lossy().to_string());
        }

        match current.parent() {
            Some(parent) => current = parent,
            None => return None,
        }
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
