//! LSP Backend implementation
//!
//! This module implements the `LanguageServer` trait from tower-lsp,
//! handling all LSP protocol requests.

use std::path::Path;
use std::sync::RwLock;

use dashmap::DashMap;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{OneOf, *};
use tower_lsp::{Client, LanguageServer};

use poly_bench::dsl::format_file_with_source;

use super::completion::get_completions;
use super::diagnostics::compute_diagnostics_with_config;
use super::document::ParsedDocument;
use super::embedded::EmbeddedConfig;
use super::hover::{get_hover, get_hover_with_gopls};
use super::semantic_tokens::{get_semantic_tokens, LEGEND};
use super::virtual_files::{VirtualFileManager, VirtualTsFileManager};

/// The LSP backend holding all state
pub struct Backend {
    /// LSP client for sending notifications
    client: Client,
    /// Parsed documents indexed by URI
    documents: DashMap<Url, ParsedDocument>,
    /// Workspace root folders
    workspace_roots: RwLock<Vec<String>>,
    /// Embedded language config cache per document
    embedded_configs: DashMap<Url, EmbeddedConfig>,
    /// Virtual file manager for gopls integration
    virtual_file_manager: VirtualFileManager,
    /// Virtual file manager for tsserver integration
    virtual_ts_file_manager: VirtualTsFileManager,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: DashMap::new(),
            workspace_roots: RwLock::new(Vec::new()),
            embedded_configs: DashMap::new(),
            virtual_file_manager: VirtualFileManager::new(),
            virtual_ts_file_manager: VirtualTsFileManager::new(),
        }
    }

    /// Re-parse a document and publish diagnostics
    async fn on_change(&self, uri: Url, text: String, version: i32) {
        let filename = uri
            .path_segments()
            .and_then(|s| s.last())
            .unwrap_or("unknown.bench")
            .to_string();

        let doc = ParsedDocument::parse(&text, &filename, version);

        // Find embedded language roots based on document location
        let config = self.find_embedded_config(&uri);
        
        // Log embedded config for debugging
        self.client
            .log_message(
                MessageType::INFO,
                format!(
                    "Checking {} - go_mod_root: {:?}, ts_module_root: {:?}",
                    filename, config.go_mod_root, config.ts_module_root
                ),
            )
            .await;
        
        // Log parse status
        if let Some(ref err) = doc.parse_error {
            self.client
                .log_message(MessageType::WARNING, format!("Parse error in {}: {}", filename, err.message))
                .await;
        } else if let Some(ref ast) = doc.ast {
            let suite_count = ast.suites.len();
            let setup_count: usize = ast.suites.iter().map(|s| s.setups.len()).sum();
            self.client
                .log_message(MessageType::INFO, format!("{} has {} suites, {} setups", filename, suite_count, setup_count))
                .await;
        }

        let result = compute_diagnostics_with_config(&doc, &config);
        
        // Log embedded checking debug info
        if let Some(ref embedded) = result.embedded_debug {
            self.client
                .log_message(
                    MessageType::INFO,
                    format!(
                        "Embedded check: {} Go blocks, {} TS blocks",
                        embedded.go_blocks_checked, embedded.ts_blocks_checked
                    ),
                )
                .await;
            
            for msg in &embedded.debug_messages {
                self.client
                    .log_message(MessageType::INFO, format!("  {}", msg))
                    .await;
            }
        }
        
        // Log diagnostic count with details
        self.client
            .log_message(
                MessageType::INFO,
                format!("Found {} diagnostics for {}", result.diagnostics.len(), filename),
            )
            .await;
        
        // Log first few diagnostics for debugging
        for (i, diag) in result.diagnostics.iter().take(3).enumerate() {
            self.client
                .log_message(MessageType::INFO, format!("  Diag {}: {}", i + 1, diag.message))
                .await;
        }

        self.documents.insert(uri.clone(), doc);
        
        // Cache the embedded config for hover requests
        self.embedded_configs.insert(uri.clone(), config);

        self.client
            .publish_diagnostics(uri, result.diagnostics, Some(version))
            .await;
    }

    /// Find Go module root and TypeScript module root for embedded checking
    fn find_embedded_config(&self, uri: &Url) -> EmbeddedConfig {
        let mut config = EmbeddedConfig::default();

        // Get the directory containing the .bench file
        let doc_path = uri.to_file_path().ok();
        let doc_dir = doc_path.as_ref().and_then(|p| p.parent());

        if let Some(dir) = doc_dir {
            // Look for project root (polybench.toml)
            let project_root = find_project_root(dir);

            // Look for Go module root
            config.go_mod_root = find_go_mod_root(dir, project_root.as_deref());

            // Look for TypeScript module root
            config.ts_module_root = find_ts_module_root(dir, project_root.as_deref());
        }

        config
    }
}

/// Find the project root by looking for polybench.toml
fn find_project_root(start: &Path) -> Option<String> {
    let mut current = start;
    loop {
        if current.join("polybench.toml").exists() {
            return Some(current.to_string_lossy().to_string());
        }
        match current.parent() {
            Some(parent) => current = parent,
            None => return None,
        }
    }
}

/// Find Go module root (directory containing go.mod)
fn find_go_mod_root(start: &Path, project_root: Option<&str>) -> Option<String> {
    // First check .polybench/runtime-env/go relative to project root
    if let Some(root) = project_root {
        let polybench_go = Path::new(root).join(".polybench/runtime-env/go");
        if polybench_go.join("go.mod").exists() {
            return Some(polybench_go.to_string_lossy().to_string());
        }
    }

    // Walk up from start looking for go.mod
    let mut current = start;
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

/// Find TypeScript module root (directory containing package.json or node_modules)
fn find_ts_module_root(start: &Path, project_root: Option<&str>) -> Option<String> {
    // First check .polybench/runtime-env/ts relative to project root
    if let Some(root) = project_root {
        let polybench_ts = Path::new(root).join(".polybench/runtime-env/ts");
        if polybench_ts.join("package.json").exists() || polybench_ts.join("node_modules").exists()
        {
            return Some(polybench_ts.to_string_lossy().to_string());
        }
    }

    // Walk up from start looking for package.json
    let mut current = start;
    loop {
        if current.join("package.json").exists() {
            return Some(current.to_string_lossy().to_string());
        }
        match current.parent() {
            Some(parent) => current = parent,
            None => return None,
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                // Full document sync - we re-parse on every change
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                // Hover support
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                // Completion support
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![
                        ":".to_string(),
                        ".".to_string(),  // For module.member access (anvil.spawnAnvil)
                        " ".to_string(),
                        "{".to_string(),
                    ]),
                    ..Default::default()
                }),
                // Semantic tokens support
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            legend: LEGEND.clone(),
                            range: Some(false),
                            full: Some(SemanticTokensFullOptions::Bool(true)),
                            ..Default::default()
                        },
                    ),
                ),
                // Document formatting support
                document_formatting_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "poly-bench-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "poly-bench-lsp initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        let version = params.text_document.version;

        self.on_change(uri, text, version).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let version = params.text_document.version;

        // We use full sync, so there's exactly one change with the full content
        if let Some(change) = params.content_changes.into_iter().next() {
            self.on_change(uri, change.text, version).await;
        }
    }

    async fn did_save(&self, _params: DidSaveTextDocumentParams) {
        // No special handling needed - diagnostics already updated on change
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = &params.text_document.uri;
        self.documents.remove(uri);
        self.embedded_configs.remove(uri);
        self.virtual_file_manager.remove(uri.as_str());
        self.virtual_ts_file_manager.remove(uri.as_str());
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        if let Some(doc) = self.documents.get(uri) {
            // Try to get hover with embedded language integration (Go/TypeScript)
            if let Some(config) = self.embedded_configs.get(uri) {
                let hover = get_hover_with_gopls(
                    &doc,
                    position,
                    &config,
                    uri,
                    &self.virtual_file_manager,
                    &self.virtual_ts_file_manager,
                );
                Ok(hover)
            } else {
                // Fall back to standard hover if no config
                Ok(get_hover(&doc, position))
            }
        } else {
            Ok(None)
        }
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = &params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        
        // Get trigger character if available (e.g., "." when user types "anvil.")
        let trigger_char = params.context
            .as_ref()
            .and_then(|ctx| ctx.trigger_character.as_deref());

        if let Some(doc) = self.documents.get(uri) {
            let items = get_completions(&doc, position, trigger_char);
            Ok(Some(CompletionResponse::Array(items)))
        } else {
            Ok(None)
        }
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let uri = &params.text_document.uri;

        if let Some(doc) = self.documents.get(uri) {
            let tokens = get_semantic_tokens(&doc);
            Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
                result_id: None,
                data: tokens,
            })))
        } else {
            Ok(None)
        }
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = &params.text_document.uri;

        if let Some(doc) = self.documents.get(uri) {
            let Some(ref ast) = doc.ast else {
                return Ok(None);
            };

            // Use the source-preserving formatter to keep comments and use statements
            let formatted = format_file_with_source(ast, &doc.source);

            // Return a single edit replacing the entire document
            let line_count = doc.rope.len_lines();
            let last_line = (line_count.saturating_sub(1)) as u32;
            let last_line_len = if line_count > 0 {
                doc.rope.line(line_count - 1).len_chars() as u32
            } else {
                0
            };

            let range = Range {
                start: Position { line: 0, character: 0 },
                end: Position {
                    line: last_line,
                    character: last_line_len,
                },
            };

            Ok(Some(vec![TextEdit {
                range,
                new_text: formatted,
            }]))
        } else {
            Ok(None)
        }
    }
}
