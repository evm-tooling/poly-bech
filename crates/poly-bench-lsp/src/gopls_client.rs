//! Gopls client for embedded Go code
//!
//! This module manages a gopls subprocess and communicates with it
//! via the Language Server Protocol over stdin/stdout.

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

use dashmap::DashMap;
use once_cell::sync::OnceCell;
use serde_json::{json, Value};
use tower_lsp::lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind};

/// Timeout for LSP requests in milliseconds
const REQUEST_TIMEOUT_MS: u64 = 5000;

/// Global gopls client instance (lazy initialized)
static GOPLS_CLIENT: OnceCell<Arc<GoplsClient>> = OnceCell::new();

/// Get or initialize the global gopls client
pub fn get_gopls_client() -> Option<Arc<GoplsClient>> {
    GOPLS_CLIENT.get().cloned()
}

/// Initialize the global gopls client with a workspace root
pub fn init_gopls_client(workspace_root: &str) -> Option<Arc<GoplsClient>> {
    GOPLS_CLIENT.get_or_try_init(|| {
        GoplsClient::new(workspace_root).map(Arc::new)
    }).ok().cloned()
}

/// Client for communicating with gopls
pub struct GoplsClient {
    /// Gopls child process
    process: Mutex<Option<Child>>,
    /// Stdin for sending requests
    stdin: Mutex<Option<ChildStdin>>,
    /// Pending request senders indexed by request ID (Arc for sharing with reader thread)
    pending: Arc<DashMap<i64, std::sync::mpsc::Sender<Value>>>,
    /// Next request ID
    next_id: AtomicI64,
    /// Whether gopls has been initialized
    initialized: AtomicBool,
    /// Whether gopls is available
    available: AtomicBool,
    /// Workspace root for gopls
    workspace_root: String,
    /// Virtual files that have been opened in gopls
    open_files: DashMap<String, i32>, // uri -> version
}

impl GoplsClient {
    /// Create a new gopls client
    pub fn new(workspace_root: &str) -> Result<Self, String> {
        // Check if gopls is available
        let gopls_path = find_gopls().ok_or_else(|| "gopls not found in PATH".to_string())?;
        
        eprintln!("[gopls] Found gopls at: {}", gopls_path);

        let client = Self {
            process: Mutex::new(None),
            stdin: Mutex::new(None),
            pending: Arc::new(DashMap::new()),
            next_id: AtomicI64::new(1),
            initialized: AtomicBool::new(false),
            available: AtomicBool::new(true),
            workspace_root: workspace_root.to_string(),
            open_files: DashMap::new(),
        };

        Ok(client)
    }

    /// Start gopls if not already running
    fn ensure_started(&self) -> Result<(), String> {
        let mut process_guard = self.process.lock().map_err(|e| e.to_string())?;
        
        if process_guard.is_some() {
            return Ok(());
        }

        let gopls_path = find_gopls().ok_or_else(|| "gopls not found".to_string())?;
        
        eprintln!("[gopls] Starting gopls subprocess...");

        let mut child = Command::new(&gopls_path)
            .args(["serve"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn gopls: {}", e))?;

        let stdin = child.stdin.take().ok_or("Failed to get gopls stdin")?;
        let stdout = child.stdout.take().ok_or("Failed to get gopls stdout")?;

        // Store stdin for sending requests
        *self.stdin.lock().map_err(|e| e.to_string())? = Some(stdin);
        *process_guard = Some(child);

        // Start reader thread
        self.start_reader_thread(stdout);

        eprintln!("[gopls] Gopls subprocess started");
        Ok(())
    }

    /// Start a thread to read responses from gopls
    fn start_reader_thread(&self, stdout: ChildStdout) {
        let pending = self.pending.clone();
        
        thread::spawn(move || {
            eprintln!("[gopls-reader] Reader thread started");
            let mut reader = BufReader::new(stdout);
            
            loop {
                // Read headers until we find Content-Length
                let mut content_length: Option<usize> = None;
                
                loop {
                    let mut header_line = String::new();
                    match reader.read_line(&mut header_line) {
                        Ok(0) => {
                            eprintln!("[gopls-reader] EOF reached");
                            return; // EOF
                        }
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("[gopls-reader] Error reading header: {}", e);
                            return;
                        }
                    }

                    let trimmed = header_line.trim();
                    
                    // Empty line signals end of headers
                    if trimmed.is_empty() {
                        break;
                    }
                    
                    // Parse Content-Length header
                    if let Some(len_str) = trimmed.strip_prefix("Content-Length:") {
                        if let Ok(len) = len_str.trim().parse::<usize>() {
                            content_length = Some(len);
                        }
                    }
                }

                let content_length = match content_length {
                    Some(len) => len,
                    None => {
                        eprintln!("[gopls-reader] No Content-Length header found");
                        continue;
                    }
                };

                // Read content
                let mut content = vec![0u8; content_length];
                if let Err(e) = std::io::Read::read_exact(&mut reader, &mut content) {
                    eprintln!("[gopls-reader] Error reading content: {}", e);
                    continue;
                }

                let content_str = match String::from_utf8(content) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("[gopls-reader] Invalid UTF-8 in response: {}", e);
                        continue;
                    }
                };

                // Parse JSON response
                let response: Value = match serde_json::from_str(&content_str) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("[gopls-reader] Invalid JSON response: {}", e);
                        continue;
                    }
                };

                // Handle response - check if it has an ID (response) or not (notification)
                if let Some(id) = response.get("id") {
                    // Could be integer or null
                    if let Some(id_num) = id.as_i64() {
                        eprintln!("[gopls-reader] Received response for request {}", id_num);
                        if let Some((_, sender)) = pending.remove(&id_num) {
                            eprintln!("[gopls-reader] Sending response to channel for request {}", id_num);
                            match sender.send(response) {
                                Ok(()) => eprintln!("[gopls-reader] Successfully sent response for request {}", id_num),
                                Err(e) => eprintln!("[gopls-reader] Failed to send response for request {}: {:?}", id_num, e),
                            }
                        } else {
                            eprintln!("[gopls-reader] No pending sender found for request {}", id_num);
                        }
                    }
                } else if let Some(method) = response.get("method").and_then(|m| m.as_str()) {
                    // It's a notification from gopls
                    eprintln!("[gopls-reader] Received notification: {}", method);
                }
            }
        });
    }

    /// Send an LSP request and wait for response with default timeout
    fn send_request(&self, method: &str, params: Value) -> Result<Value, String> {
        self.send_request_with_timeout(method, params, REQUEST_TIMEOUT_MS)
    }

    /// Send an LSP request and wait for response with custom timeout
    fn send_request_with_timeout(&self, method: &str, params: Value, timeout_ms: u64) -> Result<Value, String> {
        self.ensure_started()?;

        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let request = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params
        });

        eprintln!("[gopls] Sending request: {} (id={})", method, id);

        let request_str = serde_json::to_string(&request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        let message = format!("Content-Length: {}\r\n\r\n{}", request_str.len(), request_str);

        // Create channel for response
        let (tx, rx) = std::sync::mpsc::channel();
        self.pending.insert(id, tx);

        // Send request
        {
            let mut stdin_guard = self.stdin.lock().map_err(|e| e.to_string())?;
            if let Some(ref mut stdin) = *stdin_guard {
                stdin.write_all(message.as_bytes())
                    .map_err(|e| format!("Failed to write to gopls: {}", e))?;
                stdin.flush()
                    .map_err(|e| format!("Failed to flush gopls stdin: {}", e))?;
            } else {
                self.pending.remove(&id);
                return Err("gopls stdin not available".to_string());
            }
        }

        eprintln!("[gopls] Waiting for response to request {} (timeout={}ms)", id, timeout_ms);
        
        // Wait for response with timeout
        let response = rx.recv_timeout(std::time::Duration::from_millis(timeout_ms))
            .map_err(|e| {
                eprintln!("[gopls] Timeout for request {}: {}", id, e);
                self.pending.remove(&id);
                format!("Timeout waiting for gopls response ({}ms): {}", timeout_ms, e)
            })?;

        eprintln!("[gopls] Successfully received response for request {}", id);

        // Check for error in response
        if let Some(error) = response.get("error") {
            return Err(format!("gopls error: {}", error));
        }

        Ok(response.get("result").cloned().unwrap_or(Value::Null))
    }

    /// Send an LSP notification (no response expected)
    fn send_notification(&self, method: &str, params: Value) -> Result<(), String> {
        self.ensure_started()?;

        let notification = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params
        });

        let notification_str = serde_json::to_string(&notification)
            .map_err(|e| format!("Failed to serialize notification: {}", e))?;

        let message = format!("Content-Length: {}\r\n\r\n{}", notification_str.len(), notification_str);

        let mut stdin_guard = self.stdin.lock().map_err(|e| e.to_string())?;
        if let Some(ref mut stdin) = *stdin_guard {
            stdin.write_all(message.as_bytes())
                .map_err(|e| format!("Failed to write to gopls: {}", e))?;
            stdin.flush()
                .map_err(|e| format!("Failed to flush gopls stdin: {}", e))?;
        }

        Ok(())
    }

    /// Initialize gopls with the workspace
    pub fn initialize(&self) -> Result<(), String> {
        if self.initialized.load(Ordering::SeqCst) {
            return Ok(());
        }

        eprintln!("[gopls] Initializing with workspace: {}", self.workspace_root);

        let init_params = json!({
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

        let _result = self.send_request("initialize", init_params)?;
        
        // Send initialized notification
        self.send_notification("initialized", json!({}))?;

        self.initialized.store(true, Ordering::SeqCst);
        eprintln!("[gopls] Initialization complete");

        Ok(())
    }

    /// Open a virtual Go file in gopls
    pub fn did_open(&self, uri: &str, content: &str, version: i32) -> Result<(), String> {
        if !self.initialized.load(Ordering::SeqCst) {
            self.initialize()?;
        }

        self.send_notification("textDocument/didOpen", json!({
            "textDocument": {
                "uri": uri,
                "languageId": "go",
                "version": version,
                "text": content
            }
        }))?;

        self.open_files.insert(uri.to_string(), version);
        Ok(())
    }

    /// Update a virtual Go file in gopls
    pub fn did_change(&self, uri: &str, content: &str, version: i32) -> Result<(), String> {
        if !self.open_files.contains_key(uri) {
            return self.did_open(uri, content, version);
        }

        self.send_notification("textDocument/didChange", json!({
            "textDocument": {
                "uri": uri,
                "version": version
            },
            "contentChanges": [{
                "text": content
            }]
        }))?;

        self.open_files.insert(uri.to_string(), version);
        Ok(())
    }

    /// Close a virtual Go file in gopls
    pub fn did_close(&self, uri: &str) -> Result<(), String> {
        if !self.open_files.contains_key(uri) {
            return Ok(());
        }

        self.send_notification("textDocument/didClose", json!({
            "textDocument": {
                "uri": uri
            }
        }))?;

        self.open_files.remove(uri);
        Ok(())
    }

    /// Request hover information at a position
    pub fn hover(&self, uri: &str, line: u32, character: u32) -> Result<Option<Hover>, String> {
        if !self.initialized.load(Ordering::SeqCst) {
            self.initialize()?;
        }

        let result = self.send_request("textDocument/hover", json!({
            "textDocument": {
                "uri": uri
            },
            "position": {
                "line": line,
                "character": character
            }
        }))?;

        if result.is_null() {
            return Ok(None);
        }

        // Parse hover response
        let hover = parse_hover_response(&result)?;
        Ok(hover)
    }

    /// Check if gopls is available
    pub fn is_available(&self) -> bool {
        self.available.load(Ordering::SeqCst)
    }

    /// Shutdown gopls
    pub fn shutdown(&self) {
        if let Ok(mut process_guard) = self.process.lock() {
            if let Some(ref mut process) = *process_guard {
                // Try graceful shutdown first
                let _ = self.send_request("shutdown", json!(null));
                let _ = self.send_notification("exit", json!(null));
                
                // Wait a bit then kill if still running
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

impl Drop for GoplsClient {
    fn drop(&mut self) {
        self.shutdown();
    }
}

/// Find gopls in PATH or common locations
fn find_gopls() -> Option<String> {
    // Try which first
    if let Ok(path) = which::which("gopls") {
        return Some(path.to_string_lossy().to_string());
    }

    // Check common Go bin locations
    let home = std::env::var("HOME").ok()?;
    let candidates = [
        format!("{}/go/bin/gopls", home),
        format!("{}/.local/bin/gopls", home),
        "/usr/local/go/bin/gopls".to_string(),
        "/opt/homebrew/bin/gopls".to_string(),
    ];

    for path in &candidates {
        if std::path::Path::new(path).exists() {
            return Some(path.clone());
        }
    }

    None
}

/// Parse a hover response from gopls
fn parse_hover_response(value: &Value) -> Result<Option<Hover>, String> {
    // Handle null response
    if value.is_null() {
        return Ok(None);
    }

    let contents = value.get("contents");
    if contents.is_none() {
        return Ok(None);
    }

    let contents = contents.unwrap();

    // gopls returns MarkupContent
    let hover_contents = if let Some(kind) = contents.get("kind") {
        let kind_str = kind.as_str().unwrap_or("plaintext");
        let value_str = contents.get("value").and_then(|v| v.as_str()).unwrap_or("");
        
        HoverContents::Markup(MarkupContent {
            kind: if kind_str == "markdown" {
                MarkupKind::Markdown
            } else {
                MarkupKind::PlainText
            },
            value: value_str.to_string(),
        })
    } else if let Some(value_str) = contents.as_str() {
        // Simple string content
        HoverContents::Markup(MarkupContent {
            kind: MarkupKind::PlainText,
            value: value_str.to_string(),
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

    Ok(Some(Hover {
        contents: hover_contents,
        range,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_gopls() {
        // This test just checks that find_gopls doesn't panic
        let _ = find_gopls();
    }
}
