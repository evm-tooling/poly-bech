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
            CompletionResponse::List(CompletionList { is_incomplete: false, items })
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

            // Check for charting function parameter context
            if is_inside_charting_function_args(trimmed) {
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
                let context = determine_completion_context(node, &doc.source);

                match context {
                    CompletionContext::TopLevel => {
                        completions.extend(get_top_level_completions());
                    }
                    CompletionContext::SuiteBody => {
                        completions.extend(get_suite_body_completions());
                        // Add imported stdlib modules
                        completions.extend(get_imported_module_completions(&doc.source));
                    }
                    CompletionContext::SetupBody => {
                        completions.extend(get_setup_body_completions());
                    }
                    CompletionContext::BenchmarkBody => {
                        completions.extend(get_benchmark_body_completions());
                    }
                    CompletionContext::FixtureBody => {
                        completions.extend(get_fixture_body_completions());
                    }
                    CompletionContext::AfterBody => {
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

    let known_modules = ["anvil", "charting", "constants"];

    // Check for "module." at end or "module.partial" pattern
    for module in known_modules {
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

/// Check if cursor is inside charting function arguments
fn is_inside_charting_function_args(text: &str) -> bool {
    // Match patterns like "charting.drawBarChart(" with unclosed paren
    let patterns = [
        "charting.drawBarChart(",
        "charting.drawLineChart(",
        "charting.drawPieChart(",
        "charting.drawSpeedupChart(",
        "charting.drawScalingChart(",
        "charting.drawTable(",
    ];

    for pattern in patterns {
        if text.contains(pattern) {
            // Check if there's an unclosed paren
            let after_pattern = text.rsplit(pattern).next().unwrap_or("");
            let open_parens = after_pattern.matches('(').count();
            let close_parens = after_pattern.matches(')').count();
            if open_parens >= close_parens {
                return true;
            }
        }
    }
    false
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
                label: "drawBarChart".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                insert_text: Some("drawBarChart($0)".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Draw a bar chart of benchmark results".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "drawLineChart".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                insert_text: Some("drawLineChart($0)".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Draw a line chart for trends".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "drawPieChart".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                insert_text: Some("drawPieChart($0)".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Draw a pie chart of time distribution".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "drawSpeedupChart".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                insert_text: Some("drawSpeedupChart($0)".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Draw a speedup comparison chart".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "drawScalingChart".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                insert_text: Some("drawScalingChart($0)".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Draw a scaling efficiency chart".to_string()),
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
    vec![
        CompletionItem {
            label: "anvil".to_string(),
            kind: Some(CompletionItemKind::MODULE),
            detail: Some("Anvil Ethereum node module".to_string()),
            documentation: Some(Documentation::String(
                "Provides ANVIL_RPC_URL and spawnAnvil() for Ethereum testing.".to_string(),
            )),
            ..Default::default()
        },
        CompletionItem {
            label: "charting".to_string(),
            kind: Some(CompletionItemKind::MODULE),
            detail: Some("Chart generation module".to_string()),
            documentation: Some(Documentation::String(
                "Provides drawBarChart, drawLineChart, drawPieChart, etc.".to_string(),
            )),
            ..Default::default()
        },
        CompletionItem {
            label: "constants".to_string(),
            kind: Some(CompletionItemKind::MODULE),
            detail: Some("Mathematical constants module".to_string()),
            documentation: Some(Documentation::String("Provides PI and E constants.".to_string())),
            ..Default::default()
        },
    ]
}

/// Get completions for imported stdlib modules (module names for dot access)
fn get_imported_module_completions(source: &ropey::Rope) -> Vec<CompletionItem> {
    let mut items = Vec::new();
    let modules = ["anvil", "charting", "constants"];

    for module in modules {
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

/// Get completions for charting function parameters
fn get_charting_param_completions() -> Vec<CompletionItem> {
    vec![
        // Basic parameters (all charts)
        param_completion("title", "string", "Chart title"),
        param_completion("description", "string", "Chart description"),
        param_completion("output", "string", "Output filename"),
        param_completion("width", "number", "Chart width in pixels"),
        param_completion("height", "number", "Chart height in pixels"),
        param_completion("xlabel", "string", "X-axis label"),
        param_completion("ylabel", "string", "Y-axis label"),
        // Display toggles
        bool_param_completion("showStats", "Show ops/sec and time per op"),
        bool_param_completion("showConfig", "Show config in footer"),
        bool_param_completion("showWinCounts", "Show win counts in legend"),
        bool_param_completion("showGeoMean", "Show geometric mean speedup"),
        bool_param_completion("showDistribution", "Show min/max/p50/p99 distribution"),
        bool_param_completion("showMemory", "Show bytes/allocs memory stats"),
        bool_param_completion("showTotalTime", "Show total execution time"),
        bool_param_completion("compact", "Minimal chart mode"),
        // Filtering
        param_completion("minSpeedup", "number", "Only show benchmarks with speedup >= N"),
        enum_param_completion("filterWinner", &["go", "ts", "all"], "Filter by winner language"),
        array_param_completion("includeBenchmarks", "Only include these benchmark names"),
        array_param_completion("excludeBenchmarks", "Exclude these benchmark names"),
        param_completion("limit", "number", "Max benchmarks to show"),
        // Sorting
        enum_param_completion(
            "sortBy",
            &["speedup", "name", "time", "ops", "natural"],
            "Sort benchmarks by",
        ),
        enum_param_completion("sortOrder", &["asc", "desc"], "Sort order"),
        // Data display
        param_completion("precision", "number", "Decimal places for numbers"),
        enum_param_completion("timeUnit", &["auto", "ns", "us", "ms", "s"], "Time unit display"),
        // Axis styling
        param_completion("axisThickness", "number", "Stroke width for axes"),
        param_completion("yAxisMin", "number", "Minimum y-axis value"),
        param_completion("yAxisMax", "number", "Maximum y-axis value"),
        enum_param_completion(
            "yScale",
            &["linear", "log", "symlog", "percent"],
            "Y-axis scale type",
        ),
        param_completion(
            "baselineBenchmark",
            "string",
            "Baseline benchmark name for percentage scale",
        ),
        param_completion("symlogThreshold", "number", "Threshold for symlog scale"),
        // Grid
        bool_param_completion("showGrid", "Toggle grid lines"),
        param_completion("gridOpacity", "number", "Grid line opacity (0.0-1.0)"),
        bool_param_completion("showMinorGrid", "Show minor grid lines"),
        param_completion("minorGridOpacity", "number", "Minor grid line opacity"),
        bool_param_completion("showVerticalGrid", "Show vertical grid lines"),
        // Typography
        param_completion("titleFontSize", "number", "Title font size"),
        param_completion("subtitleFontSize", "number", "Subtitle font size"),
        param_completion("axisLabelFontSize", "number", "X/Y axis title font size"),
        param_completion("tickLabelFontSize", "number", "Tick mark label font size"),
        // Legend
        enum_param_completion(
            "legendPosition",
            &["top-left", "top-right", "bottom-left", "bottom-right", "hidden"],
            "Legend position",
        ),
        // Error bars
        bool_param_completion("showErrorBars", "Toggle error bars"),
        param_completion("errorBarOpacity", "number", "Error bar opacity"),
        param_completion("errorBarThickness", "number", "Error bar stroke width"),
        enum_param_completion("ciLevel", &["90", "95", "99"], "Confidence interval level"),
        bool_param_completion("showStdDevBand", "Show standard deviation band (line charts)"),
        // Regression
        bool_param_completion("showRegression", "Toggle regression line"),
        enum_param_completion(
            "regressionStyle",
            &["solid", "dashed", "dotted"],
            "Regression line style",
        ),
        enum_param_completion(
            "regressionModel",
            &["auto", "constant", "log", "linear", "nlogn", "quadratic", "cubic"],
            "Regression model type",
        ),
        bool_param_completion("showRegressionLabel", "Show detected model label"),
        bool_param_completion("showRSquared", "Show R² value"),
        bool_param_completion("showEquation", "Show regression equation"),
        bool_param_completion("showRegressionBand", "Show confidence band around regression"),
        param_completion(
            "regressionBandOpacity",
            "number",
            "Opacity of regression confidence band",
        ),
        // Bar chart specific
        param_completion("barWidth", "number", "Width of individual bars"),
        param_completion("barGroupGap", "number", "Gap between benchmark groups"),
        param_completion("barWithinGroupGap", "number", "Gap between bars within a group"),
        // Tick formatting
        bool_param_completion("roundTicks", "Round tick labels to whole numbers"),
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

fn bool_param_completion(name: &str, description: &str) -> CompletionItem {
    CompletionItem {
        label: name.to_string(),
        kind: Some(CompletionItemKind::PROPERTY),
        detail: Some(format!("{} (bool)", description)),
        insert_text: Some(format!("{}: ${{1|true,false|}}", name)),
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
            insert_text: Some("description: \"$0\"".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Suite description".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "iterations".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("iterations: ${1:1000}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Default iteration count for benchmarks".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "warmup".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("warmup: ${1:100}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Warmup iterations before timing".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "timeout".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("timeout: ${1:30s}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Suite-level timeout".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "requires".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("requires: [\"${1:go}\", \"${2:ts}\"]".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Required language implementations".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "order".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("order: ${1|sequential,parallel,random|}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Benchmark execution order".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "compare".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("compare: true".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Enable comparison tables".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "baseline".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("baseline: \"${1|go,ts|}\"".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Baseline language for comparison".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "tags".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("tags: [\"$0\"]".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Suite-level tags".to_string()),
            ..Default::default()
        },
        // Auto-calibration settings
        CompletionItem {
            label: "mode".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("mode: ${1|auto,fixed|}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some(
                "Execution mode: auto (time-based) or fixed (iteration count)".to_string(),
            ),
            ..Default::default()
        },
        CompletionItem {
            label: "targetTime".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("targetTime: ${1:3000ms}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Target duration for auto-calibration mode".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "minIterations".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("minIterations: ${1:100}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Minimum iterations for auto-calibration".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "maxIterations".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("maxIterations: ${1:1000000}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Maximum iterations for auto-calibration".to_string()),
            ..Default::default()
        },
        // Performance settings
        CompletionItem {
            label: "sink".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("sink: ${1|true,false|}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Use sink/black-box pattern to prevent dead code elimination".to_string()),
            ..Default::default()
        },
        // Statistical settings
        CompletionItem {
            label: "outlierDetection".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("outlierDetection: ${1|true,false|}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Enable IQR-based outlier detection and removal".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "cvThreshold".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("cvThreshold: ${1:5}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some(
                "Coefficient of variation threshold (%) for stability warnings".to_string(),
            ),
            ..Default::default()
        },
        CompletionItem {
            label: "count".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("count: ${1:10}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some(
                "Number of times to run each benchmark for statistical consistency".to_string(),
            ),
            ..Default::default()
        },
        // Observability settings
        CompletionItem {
            label: "memory".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("memory: ${1|true,false|}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Enable memory allocation profiling".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "concurrency".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("concurrency: ${1:1}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some(
                "Number of concurrent goroutines/workers for parallel execution".to_string(),
            ),
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
                 - drawBarChart()\n\
                 - drawLineChart()\n\
                 - drawPieChart()\n\
                 - drawSpeedupChart()\n\
                 - drawTable()"
                .to_string(),
        )),
        insert_text: Some("charting".to_string()),
        ..Default::default()
    }]
}
