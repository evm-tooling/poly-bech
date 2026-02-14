//! Generic LSP client for communicating with language servers
//!
//! This module provides a generic LSP client that can be used to communicate
//! with any language server over stdin/stdout (JSON-RPC).

use std::{
    io::{BufRead, BufReader, Write},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
    sync::{
        atomic::{AtomicBool, AtomicI64, Ordering},
        Arc, Mutex,
    },
    thread,
};

use dashmap::DashMap;
use serde_json::{json, Value};
use tower_lsp::lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind};

/// Timeout for LSP requests in milliseconds
pub const REQUEST_TIMEOUT_MS: u64 = 5000;

/// Configuration trait for language-specific LSP clients
pub trait LspConfig: Send + Sync + 'static {
    /// Name of the language server for logging
    const SERVER_NAME: &'static str;
    /// Language ID for document open notifications
    const LANGUAGE_ID: &'static str;

    /// Find the language server executable
    fn find_executable() -> Option<String>;

    /// Get the arguments to pass to the server
    fn server_args() -> Vec<String>;

    /// Whether the server needs special spawning (e.g., via npx)
    fn needs_special_spawn() -> bool {
        false
    }

    /// Additional capabilities for initialization
    fn additional_capabilities() -> Value {
        json!({})
    }
}

/// Generic LSP client for communicating with language servers
pub struct LspClient<C: LspConfig> {
    /// Child process
    process: Mutex<Option<Child>>,
    /// Stdin for sending requests
    stdin: Mutex<Option<ChildStdin>>,
    /// Pending request senders indexed by request ID
    pending: Arc<DashMap<i64, std::sync::mpsc::Sender<Value>>>,
    /// Next request ID
    next_id: AtomicI64,
    /// Whether server has been initialized
    initialized: AtomicBool,
    /// Whether server is available
    available: AtomicBool,
    /// Workspace root
    workspace_root: String,
    /// Virtual files that have been opened
    open_files: DashMap<String, i32>,
    /// Phantom data for config type
    _config: std::marker::PhantomData<C>,
}

impl<C: LspConfig> LspClient<C> {
    /// Create a new LSP client
    pub fn new(workspace_root: &str) -> Result<Self, String> {
        let server_path =
            C::find_executable().ok_or_else(|| format!("{} not found in PATH", C::SERVER_NAME))?;

        eprintln!("[{}] Found server at: {}", C::SERVER_NAME, server_path);

        Ok(Self {
            process: Mutex::new(None),
            stdin: Mutex::new(None),
            pending: Arc::new(DashMap::new()),
            next_id: AtomicI64::new(1),
            initialized: AtomicBool::new(false),
            available: AtomicBool::new(true),
            workspace_root: workspace_root.to_string(),
            open_files: DashMap::new(),
            _config: std::marker::PhantomData,
        })
    }

    /// Start the server if not already running
    fn ensure_started(&self) -> Result<(), String> {
        let mut process_guard = self.process.lock().map_err(|e| e.to_string())?;

        if process_guard.is_some() {
            return Ok(());
        }

        let server_path =
            C::find_executable().ok_or_else(|| format!("{} not found", C::SERVER_NAME))?;

        eprintln!("[{}] Starting subprocess...", C::SERVER_NAME);

        let args = C::server_args();
        let mut child = Command::new(&server_path)
            .args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn {}: {}", C::SERVER_NAME, e))?;

        let stdin =
            child.stdin.take().ok_or_else(|| format!("Failed to get {} stdin", C::SERVER_NAME))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| format!("Failed to get {} stdout", C::SERVER_NAME))?;

        *self.stdin.lock().map_err(|e| e.to_string())? = Some(stdin);
        *process_guard = Some(child);

        self.start_reader_thread(stdout);

        eprintln!("[{}] Subprocess started", C::SERVER_NAME);
        Ok(())
    }

    /// Start a thread to read responses from the server
    fn start_reader_thread(&self, stdout: ChildStdout) {
        let pending = self.pending.clone();
        let server_name = C::SERVER_NAME;

        thread::spawn(move || {
            eprintln!("[{}-reader] Reader thread started", server_name);
            let mut reader = BufReader::new(stdout);

            loop {
                // Read headers until we find Content-Length
                let mut content_length: Option<usize> = None;

                loop {
                    let mut header_line = String::new();
                    match reader.read_line(&mut header_line) {
                        Ok(0) => {
                            eprintln!("[{}-reader] EOF reached", server_name);
                            return;
                        }
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("[{}-reader] Error reading header: {}", server_name, e);
                            return;
                        }
                    }

                    let trimmed = header_line.trim();

                    if trimmed.is_empty() {
                        break;
                    }

                    if let Some(len_str) = trimmed.strip_prefix("Content-Length:") {
                        if let Ok(len) = len_str.trim().parse::<usize>() {
                            content_length = Some(len);
                        }
                    }
                }

                let content_length = match content_length {
                    Some(len) => len,
                    None => {
                        eprintln!("[{}-reader] No Content-Length header found", server_name);
                        continue;
                    }
                };

                let mut content = vec![0u8; content_length];
                if let Err(e) = std::io::Read::read_exact(&mut reader, &mut content) {
                    eprintln!("[{}-reader] Error reading content: {}", server_name, e);
                    continue;
                }

                let content_str = match String::from_utf8(content) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("[{}-reader] Invalid UTF-8 in response: {}", server_name, e);
                        continue;
                    }
                };

                let response: Value = match serde_json::from_str(&content_str) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("[{}-reader] Invalid JSON response: {}", server_name, e);
                        continue;
                    }
                };

                if let Some(id) = response.get("id") {
                    if let Some(id_num) = id.as_i64() {
                        eprintln!(
                            "[{}-reader] Received response for request {}",
                            server_name, id_num
                        );
                        if let Some((_, sender)) = pending.remove(&id_num) {
                            let _ = sender.send(response);
                        }
                    }
                } else if let Some(method) = response.get("method").and_then(|m| m.as_str()) {
                    eprintln!("[{}-reader] Received notification: {}", server_name, method);
                }
            }
        });
    }

    /// Send an LSP request and wait for response
    pub fn send_request(&self, method: &str, params: Value) -> Result<Value, String> {
        self.send_request_with_timeout(method, params, REQUEST_TIMEOUT_MS)
    }

    /// Send an LSP request and wait for response with custom timeout
    pub fn send_request_with_timeout(
        &self,
        method: &str,
        params: Value,
        timeout_ms: u64,
    ) -> Result<Value, String> {
        self.ensure_started()?;

        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let request = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params
        });

        eprintln!("[{}] Sending request: {} (id={})", C::SERVER_NAME, method, id);

        let request_str = serde_json::to_string(&request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        let message = format!("Content-Length: {}\r\n\r\n{}", request_str.len(), request_str);

        let (tx, rx) = std::sync::mpsc::channel();
        self.pending.insert(id, tx);

        {
            let mut stdin_guard = self.stdin.lock().map_err(|e| e.to_string())?;
            if let Some(ref mut stdin) = *stdin_guard {
                stdin
                    .write_all(message.as_bytes())
                    .map_err(|e| format!("Failed to write to {}: {}", C::SERVER_NAME, e))?;
                stdin
                    .flush()
                    .map_err(|e| format!("Failed to flush {} stdin: {}", C::SERVER_NAME, e))?;
            } else {
                self.pending.remove(&id);
                return Err(format!("{} stdin not available", C::SERVER_NAME));
            }
        }

        let response =
            rx.recv_timeout(std::time::Duration::from_millis(timeout_ms)).map_err(|e| {
                self.pending.remove(&id);
                format!("Timeout waiting for {} response ({}ms): {}", C::SERVER_NAME, timeout_ms, e)
            })?;

        if let Some(error) = response.get("error") {
            return Err(format!("{} error: {}", C::SERVER_NAME, error));
        }

        Ok(response.get("result").cloned().unwrap_or(Value::Null))
    }

    /// Send an LSP notification (no response expected)
    pub fn send_notification(&self, method: &str, params: Value) -> Result<(), String> {
        self.ensure_started()?;

        let notification = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params
        });

        let notification_str = serde_json::to_string(&notification)
            .map_err(|e| format!("Failed to serialize notification: {}", e))?;

        let message =
            format!("Content-Length: {}\r\n\r\n{}", notification_str.len(), notification_str);

        let mut stdin_guard = self.stdin.lock().map_err(|e| e.to_string())?;
        if let Some(ref mut stdin) = *stdin_guard {
            stdin
                .write_all(message.as_bytes())
                .map_err(|e| format!("Failed to write to {}: {}", C::SERVER_NAME, e))?;
            stdin
                .flush()
                .map_err(|e| format!("Failed to flush {} stdin: {}", C::SERVER_NAME, e))?;
        }

        Ok(())
    }

    /// Initialize the language server
    pub fn initialize(&self) -> Result<(), String> {
        if self.initialized.load(Ordering::SeqCst) {
            return Ok(());
        }

        eprintln!("[{}] Initializing with workspace: {}", C::SERVER_NAME, self.workspace_root);

        let mut init_params = json!({
            "processId": std::process::id(),
            "rootUri": format!("file://{}", self.workspace_root),
            "capabilities": {
                "textDocument": {
                    "hover": {
                        "contentFormat": ["markdown", "plaintext"]
                    }
                }
            },
            "workspaceFolders": [{
                "uri": format!("file://{}", self.workspace_root),
                "name": "workspace"
            }]
        });

        // Merge additional capabilities
        let additional = C::additional_capabilities();
        if let (Some(init_obj), Some(add_obj)) =
            (init_params.as_object_mut(), additional.as_object())
        {
            if let Some(caps) = init_obj.get_mut("capabilities").and_then(|c| c.as_object_mut()) {
                for (k, v) in add_obj {
                    caps.insert(k.clone(), v.clone());
                }
            }
        }

        let _result = self.send_request("initialize", init_params)?;
        self.send_notification("initialized", json!({}))?;

        self.initialized.store(true, Ordering::SeqCst);
        eprintln!("[{}] Initialization complete", C::SERVER_NAME);

        Ok(())
    }

    /// Open a virtual file in the server
    pub fn did_open(&self, uri: &str, content: &str, version: i32) -> Result<(), String> {
        if !self.initialized.load(Ordering::SeqCst) {
            self.initialize()?;
        }

        self.send_notification(
            "textDocument/didOpen",
            json!({
                "textDocument": {
                    "uri": uri,
                    "languageId": C::LANGUAGE_ID,
                    "version": version,
                    "text": content
                }
            }),
        )?;

        self.open_files.insert(uri.to_string(), version);
        Ok(())
    }

    /// Update a virtual file in the server
    pub fn did_change(&self, uri: &str, content: &str, version: i32) -> Result<(), String> {
        if !self.open_files.contains_key(uri) {
            return self.did_open(uri, content, version);
        }

        self.send_notification(
            "textDocument/didChange",
            json!({
                "textDocument": {
                    "uri": uri,
                    "version": version
                },
                "contentChanges": [{
                    "text": content
                }]
            }),
        )?;

        self.open_files.insert(uri.to_string(), version);
        Ok(())
    }

    /// Close a virtual file in the server
    pub fn did_close(&self, uri: &str) -> Result<(), String> {
        if !self.open_files.contains_key(uri) {
            return Ok(());
        }

        self.send_notification(
            "textDocument/didClose",
            json!({
                "textDocument": {
                    "uri": uri
                }
            }),
        )?;

        self.open_files.remove(uri);
        Ok(())
    }

    /// Request hover information at a position
    pub fn hover(&self, uri: &str, line: u32, character: u32) -> Result<Option<Hover>, String> {
        if !self.initialized.load(Ordering::SeqCst) {
            self.initialize()?;
        }

        let result = self.send_request(
            "textDocument/hover",
            json!({
                "textDocument": {
                    "uri": uri
                },
                "position": {
                    "line": line,
                    "character": character
                }
            }),
        )?;

        if result.is_null() {
            return Ok(None);
        }

        parse_hover_response(&result)
    }

    /// Check if server is available
    pub fn is_available(&self) -> bool {
        self.available.load(Ordering::SeqCst)
    }

    /// Shutdown the server
    pub fn shutdown(&self) {
        if let Ok(mut process_guard) = self.process.lock() {
            if let Some(ref mut process) = *process_guard {
                let _ = self.send_request("shutdown", json!(null));
                let _ = self.send_notification("exit", json!(null));
                std::thread::sleep(std::time::Duration::from_millis(100));
                let _ = process.kill();
            }
            *process_guard = None;
        }

        if let Ok(mut stdin_guard) = self.stdin.lock() {
            *stdin_guard = None;
        }

        self.initialized.store(false, Ordering::SeqCst);
        self.open_files.clear();
    }
}

impl<C: LspConfig> Drop for LspClient<C> {
    fn drop(&mut self) {
        self.shutdown();
    }
}

/// Parse a hover response from an LSP server
pub fn parse_hover_response(value: &Value) -> Result<Option<Hover>, String> {
    if value.is_null() {
        return Ok(None);
    }

    let contents = match value.get("contents") {
        Some(c) => c,
        None => return Ok(None),
    };

    let hover_contents = if let Some(obj) = contents.as_object() {
        // MarkupContent format
        let kind = obj.get("kind").and_then(|k| k.as_str()).unwrap_or("plaintext");
        let value_str = obj.get("value").and_then(|v| v.as_str()).unwrap_or("");

        HoverContents::Markup(MarkupContent {
            kind: if kind == "markdown" { MarkupKind::Markdown } else { MarkupKind::PlainText },
            value: value_str.to_string(),
        })
    } else if let Some(s) = contents.as_str() {
        HoverContents::Markup(MarkupContent { kind: MarkupKind::PlainText, value: s.to_string() })
    } else if let Some(arr) = contents.as_array() {
        // Array of MarkedString
        let combined: Vec<String> = arr
            .iter()
            .filter_map(|item| {
                if let Some(s) = item.as_str() {
                    Some(s.to_string())
                } else if let Some(obj) = item.as_object() {
                    obj.get("value").and_then(|v| v.as_str()).map(String::from)
                } else {
                    None
                }
            })
            .collect();

        HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: combined.join("\n\n"),
        })
    } else {
        return Ok(None);
    };

    // Parse range if present
    let range = value.get("range").and_then(|r| {
        let start = r.get("start")?;
        let end = r.get("end")?;
        Some(tower_lsp::lsp_types::Range {
            start: tower_lsp::lsp_types::Position {
                line: start.get("line")?.as_u64()? as u32,
                character: start.get("character")?.as_u64()? as u32,
            },
            end: tower_lsp::lsp_types::Position {
                line: end.get("line")?.as_u64()? as u32,
                character: end.get("character")?.as_u64()? as u32,
            },
        })
    });

    Ok(Some(Hover { contents: hover_contents, range }))
}
