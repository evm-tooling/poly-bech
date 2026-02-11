//! TypeScript language server client for embedded TypeScript code
//!
//! This module manages a typescript-language-server subprocess and communicates
//! with it via the Language Server Protocol over stdin/stdout.
//!
//! Note: We use typescript-language-server (which wraps tsserver) rather than
//! tsserver directly, as it provides standard LSP compatibility.

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

/// Global tsserver client instance (lazy initialized)
static TSSERVER_CLIENT: OnceCell<Arc<TsServerClient>> = OnceCell::new();

/// Get or initialize the global tsserver client
pub fn get_tsserver_client() -> Option<Arc<TsServerClient>> {
    TSSERVER_CLIENT.get().cloned()
}

/// Initialize the global tsserver client with a workspace root
pub fn init_tsserver_client(workspace_root: &str) -> Option<Arc<TsServerClient>> {
    TSSERVER_CLIENT.get_or_try_init(|| {
        TsServerClient::new(workspace_root).map(Arc::new)
    }).ok().cloned()
}

/// Find typescript-language-server executable
fn find_ts_language_server() -> Option<String> {
    // Check common locations
    let candidates = [
        // Direct command
        "typescript-language-server",
        // npm global installs
        "/usr/local/bin/typescript-language-server",
        "/opt/homebrew/bin/typescript-language-server",
    ];

    for candidate in candidates {
        if let Ok(output) = Command::new("which").arg(candidate).output() {
            if output.status.success() {
                let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path.is_empty() {
                    return Some(path);
                }
            }
        }
    }

    // Check if available via npx
    if let Ok(output) = Command::new("npx")
        .args(["--no", "typescript-language-server", "--version"])
        .output()
    {
        if output.status.success() {
            return Some("npx".to_string());
        }
    }

    // Check home directory npm bin
    if let Ok(home) = std::env::var("HOME") {
        let npm_bin = format!("{}/.npm-global/bin/typescript-language-server", home);
        if std::path::Path::new(&npm_bin).exists() {
            return Some(npm_bin);
        }

        // Check nvm locations
        let nvm_dir = format!("{}/.nvm/versions/node", home);
        if let Ok(entries) = std::fs::read_dir(&nvm_dir) {
            for entry in entries.flatten() {
                let ts_server = entry.path().join("bin/typescript-language-server");
                if ts_server.exists() {
                    return Some(ts_server.to_string_lossy().to_string());
                }
            }
        }
    }

    None
}

/// Client for communicating with typescript-language-server
pub struct TsServerClient {
    /// Child process
    process: Mutex<Option<Child>>,
    /// Stdin for sending requests
    stdin: Mutex<Option<ChildStdin>>,
    /// Pending request senders indexed by request ID (Arc for sharing with reader thread)
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
    open_files: DashMap<String, i32>, // uri -> version
    /// Whether to use npx
    use_npx: bool,
}

impl TsServerClient {
    /// Create a new tsserver client
    pub fn new(workspace_root: &str) -> Result<Self, String> {
        let ts_server_path = find_ts_language_server()
            .ok_or_else(|| "typescript-language-server not found in PATH".to_string())?;
        
        let use_npx = ts_server_path == "npx";
        
        eprintln!("[tsserver] Found typescript-language-server at: {}", ts_server_path);

        let client = Self {
            process: Mutex::new(None),
            stdin: Mutex::new(None),
            pending: Arc::new(DashMap::new()),
            next_id: AtomicI64::new(1),
            initialized: AtomicBool::new(false),
            available: AtomicBool::new(true),
            workspace_root: workspace_root.to_string(),
            open_files: DashMap::new(),
            use_npx,
        };

        Ok(client)
    }

    /// Start the server if not already running
    fn ensure_started(&self) -> Result<(), String> {
        let mut process_guard = self.process.lock().map_err(|e| e.to_string())?;
        
        if process_guard.is_some() {
            return Ok(());
        }

        eprintln!("[tsserver] Starting typescript-language-server subprocess...");

        let mut child = if self.use_npx {
            Command::new("npx")
                .args(["typescript-language-server", "--stdio"])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| format!("Failed to spawn typescript-language-server via npx: {}", e))?
        } else {
            let ts_server_path = find_ts_language_server()
                .ok_or_else(|| "typescript-language-server not found".to_string())?;
            
            Command::new(&ts_server_path)
                .args(["--stdio"])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| format!("Failed to spawn typescript-language-server: {}", e))?
        };

        let stdin = child.stdin.take().ok_or("Failed to get tsserver stdin")?;
        let stdout = child.stdout.take().ok_or("Failed to get tsserver stdout")?;

        // Store stdin for sending requests
        *self.stdin.lock().map_err(|e| e.to_string())? = Some(stdin);
        *process_guard = Some(child);

        // Start reader thread
        self.start_reader_thread(stdout);

        eprintln!("[tsserver] typescript-language-server subprocess started");
        Ok(())
    }

    /// Start a thread to read responses from the server
    fn start_reader_thread(&self, stdout: ChildStdout) {
        let pending = self.pending.clone();
        
        thread::spawn(move || {
            eprintln!("[tsserver-reader] Reader thread started");
            let mut reader = BufReader::new(stdout);
            
            loop {
                // Read headers until we find Content-Length
                let mut content_length: Option<usize> = None;
                
                loop {
                    let mut header_line = String::new();
                    match reader.read_line(&mut header_line) {
                        Ok(0) => {
                            eprintln!("[tsserver-reader] EOF reached");
                            return; // EOF
                        }
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("[tsserver-reader] Error reading header: {}", e);
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
                        eprintln!("[tsserver-reader] No Content-Length header found");
                        continue;
                    }
                };

                // Read the JSON content
                let mut content = vec![0u8; content_length];
                let content_str = match std::io::Read::read_exact(&mut reader, &mut content) {
                    Ok(()) => match String::from_utf8(content) {
                        Ok(s) => s,
                        Err(e) => {
                            eprintln!("[tsserver-reader] Invalid UTF-8 in response: {}", e);
                            continue;
                        }
                    },
                    Err(e) => {
                        eprintln!("[tsserver-reader] Failed to read content: {}", e);
                        continue;
                    }
                };

                // Parse JSON response
                let response: Value = match serde_json::from_str(&content_str) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("[tsserver-reader] Invalid JSON response: {}", e);
                        continue;
                    }
                };

                // Handle response - check if it has an ID (response) or not (notification)
                if let Some(id) = response.get("id") {
                    // Could be integer or null
                    if let Some(id_num) = id.as_i64() {
                        eprintln!("[tsserver-reader] Received response for request {}", id_num);
                        if let Some((_, sender)) = pending.remove(&id_num) {
                            eprintln!("[tsserver-reader] Sending response to channel for request {}", id_num);
                            match sender.send(response) {
                                Ok(()) => eprintln!("[tsserver-reader] Successfully sent response for request {}", id_num),
                                Err(e) => eprintln!("[tsserver-reader] Failed to send response for request {}: {:?}", id_num, e),
                            }
                        } else {
                            eprintln!("[tsserver-reader] No pending sender found for request {}", id_num);
                        }
                    }
                } else if let Some(method) = response.get("method").and_then(|m| m.as_str()) {
                    // It's a notification from the server
                    eprintln!("[tsserver-reader] Received notification: {}", method);
                }
            }
        });
    }

    /// Send an LSP request and wait for response
    fn send_request(&self, method: &str, params: Value) -> Result<Value, String> {
        self.ensure_started()?;

        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let request = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params
        });

        eprintln!("[tsserver] Sending request: {} (id={})", method, id);

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
                    .map_err(|e| format!("Failed to write to tsserver: {}", e))?;
                stdin.flush()
                    .map_err(|e| format!("Failed to flush tsserver stdin: {}", e))?;
            } else {
                self.pending.remove(&id);
                return Err("tsserver stdin not available".to_string());
            }
        }

        eprintln!("[tsserver] Waiting for response to request {} (timeout={}ms)", id, REQUEST_TIMEOUT_MS);
        
        // Wait for response with timeout
        let response = rx.recv_timeout(std::time::Duration::from_millis(REQUEST_TIMEOUT_MS))
            .map_err(|e| {
                eprintln!("[tsserver] Timeout for request {}: {}", id, e);
                self.pending.remove(&id);
                format!("Timeout waiting for tsserver response ({}ms): {}", REQUEST_TIMEOUT_MS, e)
            })?;

        eprintln!("[tsserver] Successfully received response for request {}", id);

        // Check for error response
        if let Some(error) = response.get("error") {
            return Err(format!("tsserver error: {:?}", error));
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
                .map_err(|e| format!("Failed to write to tsserver: {}", e))?;
            stdin.flush()
                .map_err(|e| format!("Failed to flush tsserver stdin: {}", e))?;
        }

        Ok(())
    }

    /// Initialize the language server
    pub fn initialize(&self) -> Result<(), String> {
        if self.initialized.load(Ordering::SeqCst) {
            return Ok(());
        }

        eprintln!("[tsserver] Initializing with workspace: {}", self.workspace_root);

        let init_params = json!({
            "processId": std::process::id(),
            "rootUri": format!("file://{}", self.workspace_root),
            "capabilities": {
                "textDocument": {
                    "hover": {
                        "contentFormat": ["markdown", "plaintext"]
                    },
                    "synchronization": {
                        "didOpen": true,
                        "didChange": true,
                        "didClose": true
                    }
                },
                "workspace": {
                    "workspaceFolders": true
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
        eprintln!("[tsserver] Initialization complete");

        Ok(())
    }

    /// Open a virtual TypeScript file
    pub fn did_open(&self, uri: &str, content: &str, version: i32) -> Result<(), String> {
        if !self.initialized.load(Ordering::SeqCst) {
            self.initialize()?;
        }

        self.send_notification("textDocument/didOpen", json!({
            "textDocument": {
                "uri": uri,
                "languageId": "typescript",
                "version": version,
                "text": content
            }
        }))?;

        self.open_files.insert(uri.to_string(), version);
        Ok(())
    }

    /// Update a virtual TypeScript file
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

    /// Close a virtual TypeScript file
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

    /// Get hover information for a position in a TypeScript file
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

        // Parse the hover result
        let contents = result.get("contents");
        
        let hover_contents = if let Some(contents) = contents {
            if let Some(obj) = contents.as_object() {
                // MarkupContent format
                let kind = obj.get("kind")
                    .and_then(|k| k.as_str())
                    .unwrap_or("plaintext");
                let value = obj.get("value")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                HoverContents::Markup(MarkupContent {
                    kind: if kind == "markdown" { MarkupKind::Markdown } else { MarkupKind::PlainText },
                    value: value.to_string(),
                })
            } else if let Some(s) = contents.as_str() {
                // Simple string
                HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::PlainText,
                    value: s.to_string(),
                })
            } else if let Some(arr) = contents.as_array() {
                // Array of MarkedString
                let combined: Vec<String> = arr.iter()
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
            }
        } else {
            return Ok(None);
        };

        Ok(Some(Hover {
            contents: hover_contents,
            range: None,
        }))
    }
}

impl Drop for TsServerClient {
    fn drop(&mut self) {
        // Try to gracefully shut down the server
        if let Ok(mut process_guard) = self.process.lock() {
            if let Some(ref mut child) = *process_guard {
                let _ = child.kill();
            }
        }
    }
}
