//! Main LSP server implementation
//!
//! This module implements the Language Server Protocol handlers
//! using tower-lsp.

use std::path::Path;

use crate::{
    diagnostics::compute_diagnostics,
    document::Document,
    embedded::EmbeddedConfig,
    embedded_diagnostics::check_embedded_code,
    formatter::format_document,
    hover::get_hover,
    hover_cache::invalidate_document_cache,
    semantic_tokens::{get_semantic_tokens, LEGEND},
    virtual_files::VirtualFileManagers,
};

use dashmap::DashMap;
use poly_bench_stdlib::VALID_MODULES;
use poly_bench_syntax::Node as AstNode;
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
        self.publish_diagnostics_impl(uri, false).await;
    }

    /// Publish diagnostics for a document, optionally including embedded code diagnostics
    async fn publish_diagnostics_with_embedded(&self, uri: &Url) {
        self.publish_diagnostics_impl(uri, true).await;
    }

    /// Internal implementation for publishing diagnostics
    async fn publish_diagnostics_impl(&self, uri: &Url, include_embedded: bool) {
        if let Some(doc) = self.documents.get(uri) {
            let mut diagnostics = compute_diagnostics(&doc);

            // On save, also check embedded code
            if include_embedded {
                let embedded_diags = check_embedded_code(&doc);
                diagnostics.extend(embedded_diags);
            }

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
                    trigger_characters: Some(completion_trigger_characters()),
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

        // On save, run full diagnostics including embedded code checks
        self.publish_diagnostics_with_embedded(&uri).await;
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

    async fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {
        for change in &params.changes {
            let path = change.uri.path();
            let filename = Path::new(path).file_name().and_then(|s| s.to_str()).unwrap_or("");

            match filename {
                "package.json" | "package-lock.json" | ".package-lock.json" => {
                    info!("Detected {} change, clearing TypeScript caches", filename);
                    self.virtual_file_managers.ts.clear_caches();
                }
                "Cargo.toml" | "Cargo.lock" => {
                    info!("Detected {} change, clearing Rust caches", filename);
                    self.virtual_file_managers.rust.clear_caches();
                }
                "go.mod" | "go.sum" => {
                    info!("Detected {} change, clearing Go caches", filename);
                    self.virtual_file_managers.go.clear_caches();
                }
                _ => {}
            }
        }

        // Re-run diagnostics for all open documents to pick up new modules
        for entry in self.documents.iter() {
            let uri = entry.key().clone();
            self.publish_diagnostics_with_embedded(&uri).await;
        }
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

        // Get trigger character if available
        let trigger_char = params.context.as_ref().and_then(|ctx| ctx.trigger_character.as_deref());

        // Helper to create exclusive completion list (tells VS Code not to add word completions)
        let exclusive_list = |items: Vec<CompletionItem>| -> CompletionResponse {
            // Mark as incomplete so the client re-requests as the user types.
            // This keeps prefix filtering responsive and avoids stale cached suggestions.
            CompletionResponse::List(CompletionList { is_incomplete: true, items })
        };

        if let Some(doc) = self.documents.get(&uri) {
            // Get the line text before cursor for context detection
            let line_text = get_line_text_before_cursor(&doc.source, position);
            let trimmed = line_text.trim();

            // Handle trigger character completions (bypass prefix filtering)
            if trigger_char == Some(".") {
                // Check for module dot access (e.g., "charting.", "anvil.")
                if let Some(module) = extract_module_before_dot(trimmed) {
                    if has_stdlib_import(&doc.source, &module) {
                        return Ok(Some(exclusive_list(get_module_member_completions(&module))));
                    }
                }
                // Dot pressed but not a known module - return empty exclusive list
                return Ok(Some(exclusive_list(vec![])));
            }

            if trigger_char == Some(":") {
                // Check for use statement
                if trimmed.starts_with("use") || trimmed.starts_with("use std") {
                    return Ok(Some(exclusive_list(get_stdlib_module_completions())));
                }
                // Colon pressed but not a use statement - return empty exclusive list
                return Ok(Some(exclusive_list(vec![])));
            }

            // Extract current prefix being typed
            let prefix = extract_current_prefix(trimmed);

            // Suite declaration header completions (top-level only, before opening brace).
            // This keeps keyword suggestions available when editing header tokens in-place.
            if let Some(slot) = suite_header_slot_for_context(&doc.source, position, &line_text) {
                let completions = get_suite_header_completions(slot);
                let header_prefix = header_prefix_for_completion(&line_text);
                let filtered = if header_prefix.is_empty() {
                    completions
                } else {
                    filter_completions_by_prefix(completions, &header_prefix, 1)
                };
                return Ok(Some(exclusive_list(filtered)));
            }

            // Setup keyword completions should remain available while editing incomplete code.
            // This line-based fast path avoids depending solely on parser context.
            if is_likely_inside_unclosed_setup(&doc.source, position) &&
                is_setup_top_level_scope(&doc.source, position) &&
                should_suggest_setup_keywords_from_line(&line_text)
            {
                let completions = get_setup_body_completions();
                let filtered = if prefix.is_empty() {
                    completions
                } else {
                    filter_completions_by_prefix(completions, &prefix, 1)
                };
                return Ok(Some(exclusive_list(filtered)));
            }

            // Suite top-level block keywords should stay responsive while typing.
            if is_likely_inside_unclosed_suite(&doc.source, position) &&
                is_suite_top_level_scope(&doc.source, position) &&
                should_suggest_block_keywords_from_line(&line_text)
            {
                let mut completions = get_suite_body_completions();
                completions.extend(get_imported_module_completions(&doc.source));
                let filtered = if prefix.is_empty() {
                    completions
                } else {
                    filter_completions_by_prefix(completions, &prefix, 1)
                };
                return Ok(Some(exclusive_list(filtered)));
            }

            // After top-level suggestions should stay responsive while typing.
            if is_likely_inside_unclosed_after(&doc.source, position) &&
                is_after_top_level_scope(&doc.source, position) &&
                !is_inside_charting_function_args(&doc.source, position) &&
                should_suggest_block_keywords_from_line(&line_text)
            {
                let completions = get_charting_completions();
                let filtered = if prefix.is_empty() {
                    completions
                } else {
                    filter_completions_by_prefix(completions, &prefix, 1)
                };
                return Ok(Some(exclusive_list(filtered)));
            }

            // Check for stdlib import context (e.g., "use ", "use st", "use std::ch")
            if let Some(module_prefix) = extract_stdlib_module_prefix(&line_text) {
                let completions = get_stdlib_module_completions();
                let filtered = if module_prefix.is_empty() {
                    completions
                } else {
                    filter_completions_by_prefix(completions, &module_prefix, 1)
                };
                return Ok(Some(exclusive_list(filtered)));
            }

            // Check for charting function parameter context
            if is_inside_charting_function_args(&doc.source, position) {
                let completions = get_charting_param_completions();
                let filtered = if prefix.is_empty() {
                    completions
                } else {
                    filter_completions_by_prefix(completions, &prefix, 1)
                };
                return Ok(Some(exclusive_list(filtered)));
            }

            // Check for module dot access without trigger (e.g., user typed "charting.dr")
            if let Some(module) = extract_module_before_dot(trimmed) {
                if has_stdlib_import(&doc.source, &module) {
                    let completions = get_module_member_completions(&module);
                    // Filter by text after the dot
                    let after_dot = trimmed.rsplit('.').next().unwrap_or("");
                    if !after_dot.is_empty() {
                        let filtered = filter_completions_by_prefix(completions, after_dot, 1);
                        return Ok(Some(exclusive_list(filtered)));
                    }
                    return Ok(Some(exclusive_list(completions)));
                }
            }

            // Use tree-sitter for context detection
            let point =
                tree_sitter::Point::new(position.line as usize, position.character as usize);

            if let Some(node) = doc.tree.root_node().descendant_for_point_range(point, point) {
                let mut completions = Vec::new();

                // Walk up to find the containing block
                let mut context = determine_completion_context(node, &doc.source);
                if context == CompletionContext::TopLevel || context == CompletionContext::SuiteBody
                {
                    if is_position_inside_any_setup_span(&doc.partial_ast, position) ||
                        is_likely_inside_unclosed_setup(&doc.source, position)
                    {
                        // Keep setup completions stable even when parser context degrades.
                        context = CompletionContext::SetupBody;
                    } else if is_position_inside_any_after_span(&doc.partial_ast, position) ||
                        is_likely_inside_unclosed_after(&doc.source, position)
                    {
                        // Keep after-block completions stable even when parser context degrades.
                        context = CompletionContext::AfterBody;
                    } else if context == CompletionContext::TopLevel &&
                        (is_position_inside_any_suite_span(&doc.partial_ast, position) ||
                            is_likely_inside_unclosed_suite(&doc.source, position))
                    {
                        // Tree-sitter can temporarily lose precise block context while typing.
                        // Fall back to partial AST span information so suite completions remain
                        // stable.
                        context = CompletionContext::SuiteBody;
                    }
                }

                match context {
                    CompletionContext::TopLevel => {
                        completions.extend(get_top_level_completions());
                    }
                    CompletionContext::SuiteBody => {
                        // When typing a suite parameter value, suppress key/block completions.
                        // This avoids noisy generic suggestions and keeps completion DSL-specific.
                        if is_suite_param_value_context(&line_text) {
                            return Ok(Some(exclusive_list(vec![])));
                        }
                        completions.extend(get_suite_body_completions());
                        // Add imported stdlib modules
                        completions.extend(get_imported_module_completions(&doc.source));
                    }
                    CompletionContext::SetupBody => {
                        if !is_setup_top_level_scope(&doc.source, position) {
                            return Ok(Some(exclusive_list(vec![])));
                        }
                        completions.extend(get_setup_body_completions());
                    }
                    CompletionContext::BenchmarkBody => {
                        completions.extend(get_benchmark_body_completions());
                    }
                    CompletionContext::FixtureBody => {
                        completions.extend(get_fixture_body_completions());
                    }
                    CompletionContext::AfterBody => {
                        if !is_after_top_level_scope(&doc.source, position) {
                            return Ok(Some(exclusive_list(vec![])));
                        }
                        completions.extend(get_charting_completions());
                    }
                    CompletionContext::EmbeddedCode => {
                        // Inside embedded Go/TS/Rust code - no DSL completions
                        return Ok(Some(exclusive_list(vec![])));
                    }
                    CompletionContext::GlobalSetup => {
                        completions.extend(get_global_setup_completions(&doc.source));
                    }
                }

                // Apply prefix filtering
                // For trigger characters, show all completions without filtering
                // For typed prefixes, filter by prefix to reduce noise
                if trigger_char.is_none() && !prefix.is_empty() {
                    completions = filter_completions_by_prefix(completions, &prefix, 1);
                }

                // Always return an exclusive list to prevent VS Code word completions
                // Even if empty, this tells VS Code we handled the request
                return Ok(Some(exclusive_list(completions)));
            }

            // If no AST node was found (common while editing incomplete code),
            // fall back to text-based block heuristics so completion still works
            // without requiring a save.
            let mut completions = if is_likely_inside_unclosed_setup(&doc.source, position) {
                if is_setup_top_level_scope(&doc.source, position) {
                    get_setup_body_completions()
                } else {
                    vec![]
                }
            } else if is_likely_inside_unclosed_after(&doc.source, position) {
                if is_after_top_level_scope(&doc.source, position) {
                    get_charting_completions()
                } else {
                    vec![]
                }
            } else if is_likely_inside_unclosed_suite(&doc.source, position) {
                if is_suite_param_value_context(&line_text) {
                    vec![]
                } else {
                    let mut items = get_suite_body_completions();
                    items.extend(get_imported_module_completions(&doc.source));
                    items
                }
            } else {
                get_top_level_completions()
            };

            if trigger_char.is_none() && !prefix.is_empty() {
                completions = filter_completions_by_prefix(completions, &prefix, 1);
            }

            return Ok(Some(exclusive_list(completions)));
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

fn completion_trigger_characters() -> Vec<String> {
    let mut chars = vec![".".to_string(), ":".to_string(), "\"".to_string(), "_".to_string()];
    chars.extend(('a'..='z').map(|c| c.to_string()));
    chars.extend(('A'..='Z').map(|c| c.to_string()));
    chars
}

/// Completion context determined from tree-sitter AST
#[derive(Debug, Clone, Copy, PartialEq)]
enum CompletionContext {
    TopLevel,
    SuiteBody,
    SetupBody,
    BenchmarkBody,
    FixtureBody,
    AfterBody,
    EmbeddedCode,
    GlobalSetup,
}

/// Get the line text before the cursor position
fn get_line_text_before_cursor(source: &ropey::Rope, position: Position) -> String {
    let line_idx = position.line as usize;
    if line_idx < source.len_lines() {
        let line = source.line(line_idx);
        let line_str: String = line.chars().collect();
        let char_pos = (position.character as usize).min(line_str.len());
        line_str[..char_pos].to_string()
    } else {
        String::new()
    }
}

/// Extract the current word/prefix being typed
fn extract_current_prefix(line_text: &str) -> String {
    let trimmed = line_text.trim_end();
    trimmed.rsplit(|c: char| !c.is_alphanumeric() && c != '_').next().unwrap_or("").to_string()
}

/// Filter completions by prefix (case-insensitive)
fn filter_completions_by_prefix(
    items: Vec<CompletionItem>,
    prefix: &str,
    min_chars: usize,
) -> Vec<CompletionItem> {
    if prefix.len() < min_chars {
        return vec![];
    }
    let prefix_lower = prefix.to_lowercase();
    items.into_iter().filter(|item| item.label.to_lowercase().starts_with(&prefix_lower)).collect()
}

/// Extract module name before a dot (e.g., "charting" from "charting." or "charting.draw")
fn extract_module_before_dot(line_text: &str) -> Option<String> {
    let trimmed = line_text.trim();
    if !trimmed.contains('.') {
        return None;
    }

    // Check for "module." at end or "module.partial" pattern
    for module in VALID_MODULES {
        let pattern = format!("{}.", module);
        if trimmed.contains(&pattern) {
            // Verify it's at a word boundary
            if let Some(pos) = trimmed.rfind(&pattern) {
                if pos == 0 || trimmed[..pos].ends_with(char::is_whitespace) {
                    return Some(module.to_string());
                }
            }
        }
    }

    None
}

/// Extract stdlib module prefix from a `use` statement being typed.
///
/// Examples:
/// - `use ` -> Some("")
/// - `use s` -> Some("")
/// - `use std::ch` -> Some("ch")
/// - `bench foo` -> None
fn extract_stdlib_module_prefix(line_text: &str) -> Option<String> {
    let trimmed = line_text.trim_start();
    if !trimmed.starts_with("use") {
        return None;
    }

    let after_use = trimmed.strip_prefix("use").unwrap_or_default().trim_start();
    if after_use.is_empty() {
        return Some(String::new());
    }

    if "std".starts_with(after_use) || "std::".starts_with(after_use) {
        return Some(String::new());
    }

    if let Some(after_std) = after_use.strip_prefix("std::") {
        let module_prefix: String =
            after_std.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
        return Some(module_prefix);
    }

    None
}

/// Check if a stdlib module is imported in the source
fn has_stdlib_import(source: &ropey::Rope, module: &str) -> bool {
    let pattern = format!("use std::{}", module);
    for line in source.lines() {
        let line_str: String = line.chars().collect();
        if line_str.trim().starts_with(&pattern) {
            return true;
        }
    }
    false
}

/// Get all text before cursor position (across multiple lines)
fn get_text_before_cursor(source: &ropey::Rope, position: Position) -> String {
    let cursor_line = position.line as usize;
    let cursor_col = position.character as usize;
    if source.len_lines() == 0 {
        return String::new();
    }

    let last_line = cursor_line.min(source.len_lines().saturating_sub(1));
    let mut out = String::new();
    for line_idx in 0..=last_line {
        let mut line: String = source.line(line_idx).chars().collect();
        if line_idx == last_line && cursor_col < line.len() {
            line.truncate(cursor_col);
        }
        out.push_str(&line);
    }
    out
}

/// Check if cursor is inside charting function arguments (supports multi-line args)
fn is_inside_charting_function_args(source: &ropey::Rope, position: Position) -> bool {
    let text = get_text_before_cursor(source, position);
    let patterns = ["charting.drawSpeedupChart(", "charting.drawTable("];

    let mut latest_pattern_idx = None;
    for pattern in patterns {
        if let Some(idx) = text.rfind(pattern) {
            if latest_pattern_idx.is_none_or(|(prev_idx, _)| idx > prev_idx) {
                latest_pattern_idx = Some((idx, pattern));
            }
        }
    }

    let Some((idx, pattern)) = latest_pattern_idx else {
        return false;
    };

    let mut depth: isize = 1;
    let after_open = &text[idx + pattern.len()..];
    for ch in after_open.chars() {
        if ch == '(' {
            depth += 1;
        } else if ch == ')' {
            depth -= 1;
            if depth == 0 {
                return false;
            }
        }
    }
    depth > 0
}

/// Determine completion context from tree-sitter node
fn determine_completion_context(
    node: tree_sitter::Node,
    source: &ropey::Rope,
) -> CompletionContext {
    let mut current = Some(node);

    while let Some(n) = current {
        match n.kind() {
            "suite_body" => return CompletionContext::SuiteBody,
            "benchmark_body" => return CompletionContext::BenchmarkBody,
            "fixture_body" => return CompletionContext::FixtureBody,
            "after_body" => return CompletionContext::AfterBody,
            "setup_body" | "setup_block" => return CompletionContext::SetupBody,
            "global_setup" | "global_setup_body" => return CompletionContext::GlobalSetup,
            "init_block" | "helpers_block" | "import_block" | "declare_block" => {
                return CompletionContext::EmbeddedCode;
            }
            "embedded_code" | "code_block" => return CompletionContext::EmbeddedCode,
            "source_file" => return CompletionContext::TopLevel,
            _ => {}
        }
        current = n.parent();
    }

    // Check line content for additional context
    let row = node.start_position().row;
    if row < source.len_lines() {
        let line: String = source.line(row).chars().collect();
        let line_trimmed = line.trim();
        if line_trimmed.starts_with("go:") ||
            line_trimmed.starts_with("ts:") ||
            line_trimmed.starts_with("rust:")
        {
            return CompletionContext::EmbeddedCode;
        }
    }

    CompletionContext::TopLevel
}

fn is_position_inside_any_suite_span(
    partial_file: &poly_bench_syntax::PartialFile,
    position: Position,
) -> bool {
    partial_file.suites.iter().any(|suite_node| {
        let span = match suite_node {
            AstNode::Valid(suite) => suite.span,
            AstNode::Error { span, .. } => *span,
            AstNode::Missing { span, .. } => *span,
        };
        is_position_in_span(position, span)
    })
}

fn is_position_inside_any_setup_span(
    partial_file: &poly_bench_syntax::PartialFile,
    position: Position,
) -> bool {
    partial_file.suites.iter().any(|suite_node| match suite_node {
        AstNode::Valid(suite) => suite.setups.values().any(|setup_node| {
            let span = match setup_node {
                AstNode::Valid(setup) => setup.span,
                AstNode::Error { span, .. } => *span,
                AstNode::Missing { span, .. } => *span,
            };
            is_position_in_span(position, span)
        }),
        AstNode::Error { .. } | AstNode::Missing { .. } => false,
    })
}

fn is_position_inside_any_after_span(
    partial_file: &poly_bench_syntax::PartialFile,
    position: Position,
) -> bool {
    partial_file.suites.iter().any(|suite_node| match suite_node {
        AstNode::Valid(suite) => suite.after_block.as_ref().is_some_and(|after_node| {
            let span = match after_node {
                AstNode::Valid(after) => after.span,
                AstNode::Error { span, .. } => *span,
                AstNode::Missing { span, .. } => *span,
            };
            is_position_in_span(position, span)
        }),
        AstNode::Error { .. } | AstNode::Missing { .. } => false,
    })
}

fn is_position_in_span(position: Position, span: poly_bench_syntax::Span) -> bool {
    let line = position.line as usize;
    let col = position.character as usize;
    let starts_before =
        line > span.start_line || (line == span.start_line && col >= span.start_col);
    let ends_before = line < span.end_line || (line == span.end_line && col <= span.end_col);
    starts_before && ends_before
}

fn is_suite_param_value_context(line_text: &str) -> bool {
    let Some((before_colon, _after_colon)) = line_text.rsplit_once(':') else {
        return false;
    };

    let key = before_colon.split_whitespace().last().unwrap_or("");
    if key.is_empty() {
        return false;
    }

    matches!(
        key,
        "description" |
            "iterations" |
            "warmup" |
            "timeout" |
            "requires" |
            "order" |
            "baseline" |
            "targetTime" |
            "sink" |
            "outlierDetection" |
            "cvThreshold" |
            "count" |
            "memory" |
            "fairness" |
            "fairnessSeed" |
            "asyncSamplingPolicy" |
            "asyncWarmupCap" |
            "asyncSampleCap"
    )
}

fn is_likely_inside_unclosed_setup(source: &ropey::Rope, position: Position) -> bool {
    setup_brace_depth_at_cursor(source, position).is_some_and(|depth| depth > 0)
}

fn is_setup_top_level_scope(source: &ropey::Rope, position: Position) -> bool {
    setup_brace_depth_at_cursor(source, position) == Some(1)
}

fn setup_brace_depth_at_cursor(source: &ropey::Rope, position: Position) -> Option<isize> {
    let cursor_line = position.line as usize;
    let cursor_col = position.character as usize;

    let mut setup_start_line = None;
    for line_idx in 0..=cursor_line.min(source.len_lines().saturating_sub(1)) {
        let line: String = source.line(line_idx).chars().collect();
        let trimmed = line.trim_start();
        if trimmed.starts_with("setup ") && trimmed.contains('{') {
            setup_start_line = Some(line_idx);
        }
    }

    let Some(start_line) = setup_start_line else {
        return None;
    };

    let mut balance: isize = 0;
    for line_idx in start_line..=cursor_line.min(source.len_lines().saturating_sub(1)) {
        let mut line: String = source.line(line_idx).chars().collect();
        if line_idx == cursor_line && cursor_col < line.len() {
            line.truncate(cursor_col);
        }
        for ch in line.chars() {
            if ch == '{' {
                balance += 1;
            } else if ch == '}' {
                balance -= 1;
            }
        }
    }

    Some(balance)
}

fn is_likely_inside_unclosed_after(source: &ropey::Rope, position: Position) -> bool {
    after_brace_depth_at_cursor(source, position).is_some_and(|depth| depth > 0)
}

fn is_after_top_level_scope(source: &ropey::Rope, position: Position) -> bool {
    after_brace_depth_at_cursor(source, position) == Some(1)
}

fn after_brace_depth_at_cursor(source: &ropey::Rope, position: Position) -> Option<isize> {
    let cursor_line = position.line as usize;
    let cursor_col = position.character as usize;

    let mut after_start_line = None;
    for line_idx in 0..=cursor_line.min(source.len_lines().saturating_sub(1)) {
        let line: String = source.line(line_idx).chars().collect();
        let trimmed = line.trim_start();
        if trimmed.starts_with("after") && trimmed.contains('{') {
            after_start_line = Some(line_idx);
        }
    }

    let Some(start_line) = after_start_line else {
        return None;
    };

    let mut balance: isize = 0;
    for line_idx in start_line..=cursor_line.min(source.len_lines().saturating_sub(1)) {
        let mut line: String = source.line(line_idx).chars().collect();
        if line_idx == cursor_line && cursor_col < line.len() {
            line.truncate(cursor_col);
        }
        for ch in line.chars() {
            if ch == '{' {
                balance += 1;
            } else if ch == '}' {
                balance -= 1;
            }
        }
    }

    Some(balance)
}

fn should_suggest_setup_keywords_from_line(line_text: &str) -> bool {
    let trimmed = line_text.trim_start();
    if trimmed.is_empty() {
        return true;
    }

    // If the user is clearly writing code/config punctuation, defer to other contexts.
    if trimmed.contains(':') ||
        trimmed.contains('.') ||
        trimmed.contains('(') ||
        trimmed.contains(')') ||
        trimmed.contains('{') ||
        trimmed.contains('}')
    {
        return false;
    }

    // Setup keywords are identifiers; show completions for identifier-like input.
    trimmed.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c.is_whitespace())
}

fn should_suggest_block_keywords_from_line(line_text: &str) -> bool {
    let trimmed = line_text.trim_start();
    if trimmed.is_empty() {
        return true;
    }

    if trimmed.contains(':') ||
        trimmed.contains('.') ||
        trimmed.contains('(') ||
        trimmed.contains(')') ||
        trimmed.contains('{') ||
        trimmed.contains('}')
    {
        return false;
    }

    trimmed.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c.is_whitespace())
}

fn is_suite_top_level_scope(source: &ropey::Rope, position: Position) -> bool {
    suite_brace_depth_at_cursor(source, position) == Some(1)
}

fn is_likely_inside_unclosed_suite(source: &ropey::Rope, position: Position) -> bool {
    suite_brace_depth_at_cursor(source, position).is_some_and(|depth| depth > 0)
}

fn suite_brace_depth_at_cursor(source: &ropey::Rope, position: Position) -> Option<isize> {
    let cursor_line = position.line as usize;
    let cursor_col = position.character as usize;

    let mut suite_start_line = None;
    for line_idx in 0..=cursor_line.min(source.len_lines().saturating_sub(1)) {
        let line: String = source.line(line_idx).chars().collect();
        let trimmed = line.trim_start();
        if is_suite_declaration_start(trimmed) {
            suite_start_line = Some(line_idx);
        }
    }

    let Some(start_line) = suite_start_line else {
        return None;
    };

    let mut balance: isize = 0;
    for line_idx in start_line..=cursor_line.min(source.len_lines().saturating_sub(1)) {
        let mut line: String = source.line(line_idx).chars().collect();
        if line_idx == cursor_line && cursor_col < line.len() {
            line.truncate(cursor_col);
        }
        for ch in line.chars() {
            if ch == '{' {
                balance += 1;
            } else if ch == '}' {
                balance -= 1;
            }
        }
    }

    Some(balance)
}

fn is_suite_declaration_start(trimmed_line: &str) -> bool {
    trimmed_line.starts_with("suite ") ||
        trimmed_line.starts_with("suite\t") ||
        trimmed_line.starts_with("declare suite ")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SuiteHeaderSlot {
    StartKeyword,
    SuiteKeyword,
    SuiteType,
    RunMode,
    SameDatasetKey,
    SameDatasetBool,
}

fn suite_header_slot_at_cursor(line_text_before_cursor: &str) -> Option<SuiteHeaderSlot> {
    // Header suggestions are only meaningful before the suite body opens.
    if line_text_before_cursor.contains('{') {
        return None;
    }

    let trimmed = line_text_before_cursor.trim_start();
    if trimmed.is_empty() {
        return None;
    }

    let tokens: Vec<&str> = trimmed.split_whitespace().collect();
    if tokens.is_empty() {
        return Some(SuiteHeaderSlot::StartKeyword);
    }

    let first = tokens[0];
    let looks_like_header_start =
        "declare".starts_with(first) || "suite".starts_with(first) || first == "declare" || first == "suite";
    if !looks_like_header_start {
        return None;
    }
    if "declare".starts_with(first) && first != "declare" {
        return Some(SuiteHeaderSlot::StartKeyword);
    }

    let trailing_ws = line_text_before_cursor
        .chars()
        .last()
        .is_some_and(char::is_whitespace);
    let slot = if trailing_ws {
        tokens.len() + 1
    } else {
        tokens.len()
    };

    let starts_with_declare = tokens.first().is_some_and(|t| *t == "declare");
    if starts_with_declare {
        // declare suite <name> <suiteType> <runMode> sameDataset: <bool>
        match slot {
            1 => Some(SuiteHeaderSlot::StartKeyword),
            2 => Some(SuiteHeaderSlot::SuiteKeyword),
            4 => Some(SuiteHeaderSlot::SuiteType),
            5 => Some(SuiteHeaderSlot::RunMode),
            6 => Some(SuiteHeaderSlot::SameDatasetKey),
            7 => Some(SuiteHeaderSlot::SameDatasetBool),
            _ => None,
        }
    } else {
        // suite <name> <suiteType> <runMode> sameDataset: <bool>
        match slot {
            1 => Some(SuiteHeaderSlot::SuiteKeyword),
            3 => Some(SuiteHeaderSlot::SuiteType),
            4 => Some(SuiteHeaderSlot::RunMode),
            5 => Some(SuiteHeaderSlot::SameDatasetKey),
            6 => Some(SuiteHeaderSlot::SameDatasetBool),
            _ => None,
        }
    }
}

fn suite_header_slot_for_context(
    source: &ropey::Rope,
    position: Position,
    line_text_before_cursor: &str,
) -> Option<SuiteHeaderSlot> {
    // Never show suite-header keyword suggestions inside an already-open suite body.
    if is_likely_inside_unclosed_suite(source, position) {
        return None;
    }
    suite_header_slot_at_cursor(line_text_before_cursor)
}

fn header_prefix_for_completion(line_text_before_cursor: &str) -> String {
    // When a completion inserts a trailing space (e.g. "performance "),
    // show the next slot's options immediately without requiring a typed character.
    if line_text_before_cursor.chars().last().is_some_and(char::is_whitespace) {
        return String::new();
    }
    extract_current_prefix(line_text_before_cursor)
}

fn suggest_again_command() -> Command {
    Command {
        title: "Trigger Suggest".to_string(),
        command: "editor.action.triggerSuggest".to_string(),
        arguments: None,
    }
}

fn header_keyword_item(label: &str, detail: &str, insert_text: &str) -> CompletionItem {
    CompletionItem {
        label: label.to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        insert_text: Some(insert_text.to_string()),
        detail: Some(detail.to_string()),
        command: Some(suggest_again_command()),
        ..Default::default()
    }
}

fn get_suite_header_completions(slot: SuiteHeaderSlot) -> Vec<CompletionItem> {
    match slot {
        SuiteHeaderSlot::StartKeyword => vec![
            CompletionItem {
                label: "declare".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                insert_text: Some("declare suite ${1:name} ".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Start a declared suite header".to_string()),
                command: Some(suggest_again_command()),
                ..Default::default()
            },
            header_keyword_item("suite", "Start a suite declaration", "suite "),
        ],
        SuiteHeaderSlot::SuiteKeyword => {
            vec![header_keyword_item("suite", "Suite declaration keyword", "suite ")]
        }
        SuiteHeaderSlot::SuiteType => vec![
            header_keyword_item(
                "performance",
                "Suite type for runtime performance benchmarking",
                "performance ",
            ),
            header_keyword_item(
                "memory",
                "Suite type for memory-focused benchmarking",
                "memory ",
            ),
            // Keep run-mode keywords visible here as a forgiving UX fallback.
            header_keyword_item("timeBased", "Run mode calibrated by target time", "timeBased "),
            header_keyword_item(
                "iterationBased",
                "Run mode with fixed iterations",
                "iterationBased ",
            ),
        ],
        SuiteHeaderSlot::RunMode => vec![
            header_keyword_item("timeBased", "Run mode calibrated by target time", "timeBased "),
            header_keyword_item(
                "iterationBased",
                "Run mode with fixed iterations",
                "iterationBased ",
            ),
        ],
        SuiteHeaderSlot::SameDatasetKey => vec![CompletionItem {
            label: "sameDataset".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("sameDataset: ".to_string()),
            detail: Some("Control dataset reuse across runtimes".to_string()),
            command: Some(suggest_again_command()),
            ..Default::default()
        }],
        SuiteHeaderSlot::SameDatasetBool => vec![
            CompletionItem {
                label: "true".to_string(),
                kind: Some(CompletionItemKind::VALUE),
                insert_text: Some("true {\n\t$0\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "false".to_string(),
                kind: Some(CompletionItemKind::VALUE),
                insert_text: Some("false {\n\t$0\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
        ],
    }
}

/// Get completions for stdlib module members (after "module.")
fn get_module_member_completions(module: &str) -> Vec<CompletionItem> {
    match module {
        "anvil" => vec![
            CompletionItem {
                label: "spawnAnvil".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                insert_text: Some("spawnAnvil()".to_string()),
                detail: Some("Spawn a local Anvil Ethereum node".to_string()),
                documentation: Some(Documentation::String(
                    "Spawn a local Anvil node. Use fork: \"url\" to fork from a chain.".to_string(),
                )),
                ..Default::default()
            },
            CompletionItem {
                label: "spawnAnvil with fork".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                insert_text: Some("spawnAnvil(fork: \"$0\")".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Spawn Anvil with chain forking".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "ANVIL_RPC_URL".to_string(),
                kind: Some(CompletionItemKind::CONSTANT),
                detail: Some("Anvil RPC endpoint URL".to_string()),
                documentation: Some(Documentation::String(
                    "The RPC endpoint URL for the spawned Anvil node.".to_string(),
                )),
                ..Default::default()
            },
        ],
        "charting" => vec![
            CompletionItem {
                label: "drawSpeedupChart".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                insert_text: Some("drawSpeedupChart($0)".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Draw a speedup comparison chart".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "drawTable".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                insert_text: Some("drawTable($0)".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Generate a results table".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "drawLineChart".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                insert_text: Some("drawLineChart($0)".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Draw a line chart with overlays".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "drawBarChart".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                insert_text: Some("drawBarChart($0)".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Draw a grouped bar chart with overlays".to_string()),
                ..Default::default()
            },
        ],
        "constants" => vec![
            CompletionItem {
                label: "PI".to_string(),
                kind: Some(CompletionItemKind::CONSTANT),
                detail: Some("Pi constant (≈ 3.14159)".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "E".to_string(),
                kind: Some(CompletionItemKind::CONSTANT),
                detail: Some("Euler's number (≈ 2.71828)".to_string()),
                ..Default::default()
            },
        ],
        _ => vec![],
    }
}

/// Get completions for stdlib modules (after "use std::")
fn get_stdlib_module_completions() -> Vec<CompletionItem> {
    VALID_MODULES
        .iter()
        .map(|module| CompletionItem {
            label: (*module).to_string(),
            kind: Some(CompletionItemKind::MODULE),
            detail: Some(stdlib_module_detail(module).to_string()),
            documentation: Some(Documentation::String(stdlib_module_docs(module).to_string())),
            insert_text: Some((*module).to_string()),
            ..Default::default()
        })
        .collect()
}

/// Get completions for imported stdlib modules (module names for dot access)
fn get_imported_module_completions(source: &ropey::Rope) -> Vec<CompletionItem> {
    let mut items = Vec::new();

    for module in VALID_MODULES {
        if has_stdlib_import(source, module) {
            items.push(CompletionItem {
                label: module.to_string(),
                kind: Some(CompletionItemKind::MODULE),
                detail: Some(format!("std::{} module - type '.' for members", module)),
                insert_text: Some(module.to_string()),
                ..Default::default()
            });
        }
    }

    items
}

fn stdlib_module_detail(module: &str) -> &'static str {
    match module {
        "anvil" => "Anvil Ethereum node module",
        "charting" => "Chart generation module",
        "constants" => "Mathematical constants module",
        _ => "Standard library module",
    }
}

fn stdlib_module_docs(module: &str) -> &'static str {
    match module {
        "anvil" => "Provides ANVIL_RPC_URL and spawnAnvil() for Ethereum testing.",
        "charting" => "Provides drawSpeedupChart(), drawTable().",
        "constants" => "Provides PI and E constants.",
        _ => "Standard library module for poly-bench.",
    }
}

/// Get completions for charting function parameters
fn get_charting_param_completions() -> Vec<CompletionItem> {
    vec![
        param_completion("title", "string", "Chart title"),
        param_completion("description", "string", "Chart description"),
        param_completion("output", "string", "Output filename"),
        param_completion("width", "number", "Chart width in pixels"),
        param_completion(
            "rowCount",
            "number",
            "Max number of benchmark cards per row in combined charts",
        ),
        param_completion("height", "number", "Chart height in pixels"),
        param_completion("minSpeedup", "number", "Only show benchmarks with speedup >= N"),
        enum_param_completion("filterWinner", &["go", "ts", "all"], "Filter by winner language"),
        array_param_completion("includeBenchmarks", "Only include these benchmark names"),
        array_param_completion("excludeBenchmarks", "Exclude these benchmark names"),
        param_completion("limit", "number", "Max benchmarks to show"),
        enum_param_completion(
            "sortBy",
            &["speedup", "name", "time", "ops", "natural"],
            "Sort benchmarks by",
        ),
        enum_param_completion("sortOrder", &["asc", "desc"], "Sort order"),
        param_completion(
            "baselineBenchmark",
            "string",
            "Baseline benchmark name for percentage scale",
        ),
        enum_param_completion("theme", &["dark", "light"], "Chart color theme"),
    ]
}

fn param_completion(name: &str, param_type: &str, description: &str) -> CompletionItem {
    CompletionItem {
        label: name.to_string(),
        kind: Some(CompletionItemKind::PROPERTY),
        detail: Some(format!("{} ({})", description, param_type)),
        insert_text: Some(format!("{}: $0", name)),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        ..Default::default()
    }
}

fn enum_param_completion(name: &str, values: &[&str], description: &str) -> CompletionItem {
    let choices = values.join(",");
    CompletionItem {
        label: name.to_string(),
        kind: Some(CompletionItemKind::PROPERTY),
        detail: Some(format!("{} (enum)", description)),
        insert_text: Some(format!("{}: \"${{1|{}|}}\"", name, choices)),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        ..Default::default()
    }
}

fn array_param_completion(name: &str, description: &str) -> CompletionItem {
    CompletionItem {
        label: name.to_string(),
        kind: Some(CompletionItemKind::PROPERTY),
        detail: Some(format!("{} (array)", description)),
        insert_text: Some(format!("{}: [\"$0\"]", name)),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        ..Default::default()
    }
}

/// Get completions for setup blocks
fn get_setup_body_completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "import".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("import {\n\t$0\n}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Import statements block".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "declare".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("declare {\n\t$0\n}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Package-level declarations".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "init".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("init {\n\t$0\n}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Initialization code block".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "helpers".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("helpers {\n\t$0\n}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Helper function definitions".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "async init".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("async init {\n\t$0\n}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Async initialization (TypeScript only)".to_string()),
            ..Default::default()
        },
    ]
}

/// Get completions for globalSetup blocks
fn get_global_setup_completions(source: &ropey::Rope) -> Vec<CompletionItem> {
    let mut items = Vec::new();

    // If anvil is imported, suggest anvil.spawnAnvil()
    if has_stdlib_import(source, "anvil") {
        items.push(CompletionItem {
            label: "anvil.spawnAnvil".to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            insert_text: Some("anvil.spawnAnvil()".to_string()),
            detail: Some("Spawn a local Anvil Ethereum node".to_string()),
            ..Default::default()
        });
        items.push(CompletionItem {
            label: "anvil".to_string(),
            kind: Some(CompletionItemKind::MODULE),
            detail: Some("std::anvil module - type '.' for members".to_string()),
            ..Default::default()
        });
    }

    items
}

fn get_top_level_completions() -> Vec<CompletionItem> {
    let mut items = vec![
        CompletionItem {
            label: "declare".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("declare suite ${1:name} ".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Start a declared suite header".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "suite".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some(
                "declare suite ${1:name} ${2|performance,memory|} ${3|timeBased,iterationBased|} sameDataset: ${4|true,false|} {\n\t$0\n}"
                    .to_string(),
            ),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Define a benchmark suite".to_string()),
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
        CompletionItem {
            label: "performance".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("performance".to_string()),
            detail: Some("Suite type for runtime performance benchmarking".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "memory".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("memory".to_string()),
            detail: Some("Suite type for memory-focused benchmarking".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "timeBased".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("timeBased".to_string()),
            detail: Some("Run mode calibrated by target time".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "iterationBased".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("iterationBased".to_string()),
            detail: Some("Run mode with a fixed iteration count".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "sameDataset".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("sameDataset: ${1|true,false|}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Control dataset reuse across runtimes".to_string()),
            ..Default::default()
        },
    ];

    items.extend(VALID_MODULES.iter().map(|module| CompletionItem {
        label: format!("use std::{}", module),
        kind: Some(CompletionItemKind::MODULE),
        insert_text: Some(format!("use std::{}", module)),
        detail: Some(format!("Import {} module", module)),
        ..Default::default()
    }));

    items
}

fn get_suite_body_completions() -> Vec<CompletionItem> {
    vec![
        // Block definitions
        CompletionItem {
            label: "bench".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("bench ${1:name} {\n\t${2:go}: ${3:code()}\n}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Define a benchmark".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "benchAsync".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some(
                "benchAsync ${1:name} {\n\t${2:ts}: ${3:await code()}\n}".to_string(),
            ),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Define an async-sequential benchmark".to_string()),
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
            label: "setup rust".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some(
                "setup rust {\n\timport {\n\t\tuse $1;\n\t}\n\n\thelpers {\n\t\t$0\n\t}\n}"
                    .to_string(),
            ),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Define Rust setup block".to_string()),
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
            label: "before".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("before ${1|go,ts|}: {\n\t$0\n}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Suite-level before hook".to_string()),
            ..Default::default()
        },
        // Basic configuration
        CompletionItem {
            label: "description".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("description: \"\"".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Suite description".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "iterations".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("iterations: 1000".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Default iteration count for benchmarks".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "warmup".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("warmup: 1000".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Warmup iterations before timing".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "requires".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("requires: []".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Required language implementations".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "order".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("order: sequential".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Benchmark execution order".to_string()),
            ..Default::default()
        },
        // Run-mode controlled timing settings
        CompletionItem {
            label: "targetTime".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("targetTime: 3000".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Target duration for declaration run mode: timeBased".to_string()),
            ..Default::default()
        },
        // Performance settings
        CompletionItem {
            label: "sink".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("sink: true".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Use sink/black-box pattern to prevent dead code elimination".to_string()),
            ..Default::default()
        },
        // Statistical settings
        CompletionItem {
            label: "outlierDetection".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("outlierDetection: true".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Enable IQR-based outlier detection and removal".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "cvThreshold".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("cvThreshold: 5.0".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some(
                "Coefficient of variation threshold (%) for stability warnings".to_string(),
            ),
            ..Default::default()
        },
        CompletionItem {
            label: "count".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("count: 1".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some(
                "Number of times to run each benchmark for statistical consistency".to_string(),
            ),
            ..Default::default()
        },
        CompletionItem {
            label: "fairness".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("fairness: \"${1|legacy,strict|}\"".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Fairness scheduling mode across runtimes".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "fairnessSeed".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("fairnessSeed: 20260223".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Deterministic seed for strict fairness ordering".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "asyncSamplingPolicy".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("asyncSamplingPolicy: \"${1|timeBudgeted,fixedCap|}\"".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Sampling policy for async auto mode".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "asyncWarmupCap".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("asyncWarmupCap: 8".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Maximum async warmup iterations in auto mode".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "asyncSampleCap".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("asyncSampleCap: 50".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Maximum stored async samples per benchmark run".to_string()),
            ..Default::default()
        },
        // Observability settings
        CompletionItem {
            label: "memory".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("memory: false".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Enable memory allocation profiling".to_string()),
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
            detail: Some("Benchmark tags for filtering".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "description".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("description: \"$0\"".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Benchmark description".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "iterations".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("iterations: ${0:1000}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Override iterations for this benchmark".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "warmup".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("warmup: ${0:100}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Override warmup iterations".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "timeout".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("timeout: ${0:30s}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Benchmark timeout".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "each".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("each ${1:go}: $0".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Per-iteration hook".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "skip".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("skip ${1:go}: $0".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Skip condition for a language".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "validate".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("validate: $0".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Validation expression".to_string()),
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
        CompletionItem {
            label: "rust".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("rust: $0".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Rust fixture implementation".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "description".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("description: \"$0\"".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Fixture description".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "shape".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("shape: \"$0\"".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Type shape annotation".to_string()),
            ..Default::default()
        },
    ]
}

fn get_charting_completions() -> Vec<CompletionItem> {
    // In after blocks, suggest "charting" module for dot access
    vec![CompletionItem {
        label: "charting".to_string(),
        kind: Some(CompletionItemKind::MODULE),
        detail: Some("Chart generation module - type '.' for methods".to_string()),
        documentation: Some(Documentation::String(
            "Type charting. to see available chart functions:\n\
                 - drawSpeedupChart()\n\
                 - drawTable()"
                .to_string(),
        )),
        insert_text: Some("charting".to_string()),
        ..Default::default()
    }]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use tower_lsp::lsp_types::Url;

    #[test]
    fn stdlib_module_completions_include_all_valid_modules() {
        let completion_labels: HashSet<String> =
            get_stdlib_module_completions().into_iter().map(|item| item.label).collect();
        let expected: HashSet<String> =
            VALID_MODULES.iter().map(|module| module.to_string()).collect();
        assert_eq!(completion_labels, expected);
    }

    #[test]
    fn detects_use_std_prefix_context() {
        assert_eq!(extract_stdlib_module_prefix("use "), Some(String::new()));
        assert_eq!(extract_stdlib_module_prefix("use st"), Some(String::new()));
        assert_eq!(extract_stdlib_module_prefix("use std::"), Some(String::new()));
        assert_eq!(extract_stdlib_module_prefix("use std::ch"), Some("ch".to_string()));
        assert_eq!(extract_stdlib_module_prefix("bench foo"), None);
    }

    #[test]
    fn suite_completions_insert_runtime_defaults() {
        let top_level = get_top_level_completions();
        let completions = top_level
            .clone()
            .into_iter()
            .chain(get_suite_body_completions())
            .collect::<Vec<_>>();
        let mut inserts = std::collections::HashMap::new();
        for item in completions {
            if let Some(insert_text) = item.insert_text {
                inserts.insert(item.label, insert_text);
            }
        }

        assert_eq!(inserts.get("warmup"), Some(&"warmup: 1000".to_string()));
        assert_eq!(
            inserts.get("suite"),
            Some(
                &"declare suite ${1:name} ${2|performance,memory|} ${3|timeBased,iterationBased|} sameDataset: ${4|true,false|} {\n\t$0\n}"
                    .to_string()
            )
        );
        assert_eq!(
            inserts.get("declare"),
            Some(&"declare suite ${1:name} ".to_string())
        );
        assert_eq!(inserts.get("targetTime"), Some(&"targetTime: 3000".to_string()));
        assert_eq!(inserts.get("count"), Some(&"count: 1".to_string()));
        assert_eq!(
            inserts.get("benchAsync"),
            Some(&"benchAsync ${1:name} {\n\t${2:ts}: ${3:await code()}\n}".to_string())
        );

        let has_top_level_insert = |label: &str, insert_text: &str| {
            top_level.iter().any(|item| {
                item.label == label && item.insert_text.as_deref() == Some(insert_text)
            })
        };
        assert!(has_top_level_insert("performance", "performance"));
        assert!(has_top_level_insert("memory", "memory"));
        assert!(has_top_level_insert("timeBased", "timeBased"));
        assert!(has_top_level_insert("iterationBased", "iterationBased"));
        assert!(has_top_level_insert("sameDataset", "sameDataset: ${1|true,false|}"));
    }

    #[test]
    fn suite_header_slot_detection() {
        assert_eq!(
            suite_header_slot_at_cursor("dec"),
            Some(SuiteHeaderSlot::StartKeyword)
        );
        assert_eq!(
            suite_header_slot_at_cursor("declare su"),
            Some(SuiteHeaderSlot::SuiteKeyword)
        );
        assert_eq!(
            suite_header_slot_at_cursor("declare suite demo per"),
            Some(SuiteHeaderSlot::SuiteType)
        );
        assert_eq!(
            suite_header_slot_at_cursor("declare suite demo performance tim"),
            Some(SuiteHeaderSlot::RunMode)
        );
        assert_eq!(
            suite_header_slot_at_cursor("declare suite demo performance timeBased same"),
            Some(SuiteHeaderSlot::SameDatasetKey)
        );
        assert_eq!(
            suite_header_slot_at_cursor("declare suite demo performance timeBased sameDataset: tr"),
            Some(SuiteHeaderSlot::SameDatasetBool)
        );
        assert_eq!(
            suite_header_slot_at_cursor("declare suite demo performance timeBased sameDataset: true {"),
            None
        );
    }

    #[test]
    fn suite_header_keyword_items_chain_with_spaces() {
        let items = get_suite_header_completions(SuiteHeaderSlot::SuiteType);
        let mut inserts = std::collections::HashMap::new();
        for item in items {
            if let Some(insert_text) = item.insert_text {
                inserts.insert(item.label, insert_text);
            }
        }

        assert_eq!(inserts.get("performance"), Some(&"performance ".to_string()));
        assert_eq!(inserts.get("memory"), Some(&"memory ".to_string()));
        assert_eq!(inserts.get("timeBased"), Some(&"timeBased ".to_string()));
        assert_eq!(inserts.get("iterationBased"), Some(&"iterationBased ".to_string()));
    }

    #[test]
    fn suite_header_run_mode_slot_excludes_suite_types() {
        let items = get_suite_header_completions(SuiteHeaderSlot::RunMode);
        let labels: Vec<String> = items.into_iter().map(|item| item.label).collect();
        assert!(labels.contains(&"timeBased".to_string()));
        assert!(labels.contains(&"iterationBased".to_string()));
        assert!(!labels.contains(&"performance".to_string()));
        assert!(!labels.contains(&"memory".to_string()));
    }

    #[test]
    fn suite_header_bool_completion_finishes_with_braces() {
        let items = get_suite_header_completions(SuiteHeaderSlot::SameDatasetBool);
        let mut inserts = std::collections::HashMap::new();
        for item in items {
            if let Some(insert_text) = item.insert_text {
                inserts.insert(item.label, insert_text);
            }
        }

        assert_eq!(inserts.get("true"), Some(&"true {\n\t$0\n}".to_string()));
        assert_eq!(inserts.get("false"), Some(&"false {\n\t$0\n}".to_string()));
    }

    #[test]
    fn header_prefix_ignores_trailing_space() {
        assert_eq!(header_prefix_for_completion("declare suite demo performance "), "");
        assert_eq!(header_prefix_for_completion("declare suite demo per"), "per");
    }

    #[test]
    fn suite_header_not_suggested_inside_suite_body() {
        let source = "declare suite demo performance iterationBased sameDataset: true {\n    de\n}";
        let doc = crate::document::Document::new(
            Url::parse("file:///suite_body_no_header.bench").expect("valid file URL"),
            source.to_string(),
            1,
        );
        let pos = Position { line: 1, character: 6 };
        let line_text = "    de";
        assert_eq!(
            suite_header_slot_for_context(&doc.source, pos, line_text),
            None
        );
    }

    #[test]
    fn suite_value_context_detection() {
        assert!(is_suite_param_value_context("warmup: "));
        assert!(is_suite_param_value_context("targetTime: 30"));
        assert!(!is_suite_param_value_context("bench foo {"));
        assert!(!is_suite_param_value_context("charting.drawTable("));
    }

    #[test]
    fn detects_position_inside_suite_span() {
        let source = "suite demo {\n    warmup: 1000\n}\n";
        let doc = crate::document::Document::new(
            Url::parse("file:///suite.bench").expect("valid file URL"),
            source.to_string(),
            1,
        );

        assert!(is_position_inside_any_suite_span(
            &doc.partial_ast,
            Position { line: 1, character: 8 },
        ));
        assert!(!is_position_inside_any_suite_span(
            &doc.partial_ast,
            Position { line: 3, character: 0 },
        ));
    }

    #[test]
    fn detects_unclosed_suite_by_text_fallback() {
        let source = "suite demo {\n    warmup: 1000\n    target";
        let doc = crate::document::Document::new(
            Url::parse("file:///suite_unclosed.bench").expect("valid file URL"),
            source.to_string(),
            1,
        );

        assert!(is_likely_inside_unclosed_suite(&doc.source, Position { line: 2, character: 10 },));

        let source_closed = "suite demo {\n    warmup: 1000\n}\n";
        let doc_closed = crate::document::Document::new(
            Url::parse("file:///suite_closed.bench").expect("valid file URL"),
            source_closed.to_string(),
            1,
        );
        assert!(!is_likely_inside_unclosed_suite(
            &doc_closed.source,
            Position { line: 3, character: 0 },
        ));

        let declared_source = "declare suite demo performance timeBased sameDataset: true {\n    warmup: 1000\n    target";
        let declared_doc = crate::document::Document::new(
            Url::parse("file:///suite_declared_unclosed.bench").expect("valid file URL"),
            declared_source.to_string(),
            1,
        );
        assert!(is_likely_inside_unclosed_suite(
            &declared_doc.source,
            Position { line: 2, character: 10 },
        ));
    }

    #[test]
    fn detects_position_inside_setup_span() {
        let source = "suite demo {\n    setup go {\n        helpers {\n        }\n    }\n}\n";
        let doc = crate::document::Document::new(
            Url::parse("file:///setup_span.bench").expect("valid file URL"),
            source.to_string(),
            1,
        );

        assert!(is_position_inside_any_setup_span(
            &doc.partial_ast,
            Position { line: 2, character: 8 },
        ));
        assert!(!is_position_inside_any_setup_span(
            &doc.partial_ast,
            Position { line: 0, character: 1 },
        ));
    }

    #[test]
    fn detects_unclosed_setup_by_text_fallback() {
        let source = "suite demo {\n    setup go {\n        he\n";
        let doc = crate::document::Document::new(
            Url::parse("file:///setup_unclosed.bench").expect("valid file URL"),
            source.to_string(),
            1,
        );
        assert!(is_likely_inside_unclosed_setup(&doc.source, Position { line: 2, character: 10 },));

        let source_closed =
            "suite demo {\n    setup go {\n        helpers {\n        }\n    }\n}\n";
        let doc_closed = crate::document::Document::new(
            Url::parse("file:///setup_closed.bench").expect("valid file URL"),
            source_closed.to_string(),
            1,
        );
        assert!(!is_likely_inside_unclosed_setup(
            &doc_closed.source,
            Position { line: 5, character: 0 },
        ));
    }

    #[test]
    fn setup_keyword_line_heuristic() {
        assert!(should_suggest_setup_keywords_from_line("    im"));
        assert!(should_suggest_setup_keywords_from_line("helpers"));
        assert!(should_suggest_setup_keywords_from_line("    "));
        assert!(!should_suggest_setup_keywords_from_line("charting.drawTable("));
        assert!(!should_suggest_setup_keywords_from_line("targetTime: 3000"));
    }

    #[test]
    fn setup_top_level_scope_detection() {
        let source = r#"suite demo {
    setup go {
        im
        helpers {
            im
        }
    }
}"#;
        let doc = crate::document::Document::new(
            Url::parse("file:///setup_scope.bench").expect("valid file URL"),
            source.to_string(),
            1,
        );

        assert!(is_setup_top_level_scope(&doc.source, Position { line: 2, character: 10 },));
        assert!(!is_setup_top_level_scope(&doc.source, Position { line: 4, character: 12 },));
    }

    #[test]
    fn suite_top_level_scope_detection() {
        let source = r#"suite demo {
    be
    setup go {
        be
    }
}"#;
        let doc = crate::document::Document::new(
            Url::parse("file:///suite_scope.bench").expect("valid file URL"),
            source.to_string(),
            1,
        );

        assert!(is_suite_top_level_scope(&doc.source, Position { line: 1, character: 6 },));
        assert!(!is_suite_top_level_scope(&doc.source, Position { line: 3, character: 10 },));
    }

    #[test]
    fn suite_block_keyword_line_heuristic() {
        assert!(should_suggest_block_keywords_from_line("    be"));
        assert!(should_suggest_block_keywords_from_line("after"));
        assert!(should_suggest_block_keywords_from_line("    "));
        assert!(!should_suggest_block_keywords_from_line("targetTime: 3000"));
        assert!(!should_suggest_block_keywords_from_line("charting.drawSpeedupChart("));
    }

    #[test]
    fn after_top_level_scope_detection() {
        let source = r#"suite demo {
    after {
        charting
        group {
            charting
        }
    }
}"#;
        let doc = crate::document::Document::new(
            Url::parse("file:///after_scope.bench").expect("valid file URL"),
            source.to_string(),
            1,
        );

        assert!(is_after_top_level_scope(&doc.source, Position { line: 2, character: 10 },));
        assert!(!is_after_top_level_scope(&doc.source, Position { line: 4, character: 12 },));
    }

    #[test]
    fn detects_multiline_charting_args_context() {
        let source = r#"suite demo {
    after {
        charting.drawSpeedupChart(
            ti
        )
    }
}"#;
        let doc = crate::document::Document::new(
            Url::parse("file:///charting_args.bench").expect("valid file URL"),
            source.to_string(),
            1,
        );

        assert!(
            is_inside_charting_function_args(&doc.source, Position { line: 3, character: 14 },)
        );
        assert!(
            !is_inside_charting_function_args(&doc.source, Position { line: 5, character: 5 },)
        );
    }
}
