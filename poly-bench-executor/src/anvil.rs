//! Anvil service management
//!
//! Handles spawning and managing a local Anvil Ethereum node for EVM benchmarks.

use miette::{miette, Result};
use std::{
    env,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    process::{Child, Command, Stdio},
    time::{Duration, Instant},
};

/// Configuration for spawning an Anvil instance
#[derive(Debug, Clone, Default)]
pub struct AnvilConfig {
    /// Optional RPC URL to fork from
    pub fork_url: Option<String>,
    /// Optional block number to fork from (requires fork_url)
    pub fork_block: Option<u64>,
}

/// A running Anvil instance managed by the scheduler
pub struct AnvilService {
    /// The Anvil child process
    anvil_child: Child,
    /// Optional Toxiproxy server child process
    toxiproxy_child: Option<Child>,
    /// The externally exposed RPC URL (proxy when enabled)
    pub rpc_url: String,
    /// The externally exposed RPC port (proxy when enabled)
    pub port: u16,
    /// The direct Anvil RPC port
    anvil_port: u16,
}

impl AnvilService {
    /// Spawn a new Anvil instance and wait for it to be ready
    pub fn spawn(config: &AnvilConfig) -> Result<Self> {
        // Find an available port
        let anvil_port = Self::find_available_port()?;

        // Build command arguments
        let mut args = vec![
            "--port".to_string(),
            anvil_port.to_string(),
            "--silent".to_string(),
            "--block-time".to_string(),
            "1".to_string(),
            "--no-storage-caching".to_string(),
        ];

        if let Some(ref fork_url) = config.fork_url {
            args.push("--fork-url".to_string());
            args.push(fork_url.clone());

            if let Some(block) = config.fork_block {
                args.push("--fork-block-number".to_string());
                args.push(block.to_string());
            }
        }

        // Start Anvil process
        let anvil_child = Command::new("anvil")
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| miette!("Failed to spawn anvil (is it installed?): {}", e))?;

        let mut service = Self {
            anvil_child,
            toxiproxy_child: None,
            rpc_url: format!("http://127.0.0.1:{}", anvil_port),
            port: anvil_port,
            anvil_port,
        };

        // Wait for direct Anvil first.
        service.wait_ready_on_port(anvil_port, Duration::from_secs(30))?;

        // Default behavior: route through Toxiproxy and inject latency.
        if Self::use_toxiproxy_enabled() {
            let proxy_port = Self::find_available_port()?;
            let api_port = Self::find_available_port()?;
            let toxiproxy_child = Command::new("toxiproxy-server")
                .args(["-host", "127.0.0.1", "-port", &api_port.to_string()])
                // Don't pipe logs if we're not reading them: a full pipe can block
                // toxiproxy and make benchmark calls appear hung.
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .map_err(|e| {
                    miette!(
                        "Failed to spawn toxiproxy-server (is it installed?): {}. \
Install with `brew install toxiproxy` or disable via POLY_BENCH_ANVIL_USE_TOXIPROXY=0",
                        e
                    )
                })?;

            service.toxiproxy_child = Some(toxiproxy_child);
            Self::wait_toxiproxy_ready(api_port, Duration::from_secs(10))?;

            let latency_ms = Self::proxy_latency_ms();
            let jitter_ms = Self::proxy_jitter_ms();
            Self::configure_toxiproxy(api_port, proxy_port, anvil_port, latency_ms, jitter_ms)?;
            service.port = proxy_port;
            service.rpc_url = format!("http://127.0.0.1:{}", proxy_port);
            service.wait_ready(Duration::from_secs(10))?;
        }

        Ok(service)
    }

    /// Find an available port for Anvil to listen on
    fn find_available_port() -> Result<u16> {
        let listener = TcpListener::bind("127.0.0.1:0")
            .map_err(|e| miette!("Failed to find available port: {}", e))?;
        let port = listener
            .local_addr()
            .map_err(|e| miette!("Failed to get local address: {}", e))?
            .port();
        drop(listener);
        Ok(port)
    }

    /// Wait for Anvil to be ready by polling the RPC endpoint
    fn wait_ready(&mut self, timeout: Duration) -> Result<()> {
        self.wait_ready_on_port(self.port, timeout)
    }

    fn wait_ready_on_port(&mut self, port: u16, timeout: Duration) -> Result<()> {
        let deadline = Instant::now() + timeout;

        while Instant::now() < deadline {
            if Self::is_ready_on_port(port) {
                return Ok(());
            }
            std::thread::sleep(Duration::from_millis(100));
        }

        // Check if Anvil process died
        if let Ok(Some(status)) = self.anvil_child.try_wait() {
            return Err(miette!("Anvil process exited with status: {}", status));
        }
        // Check if Toxiproxy process died (when waiting on proxy port)
        if port != self.anvil_port {
            if let Some(ref mut child) = self.toxiproxy_child {
                if let Ok(Some(status)) = child.try_wait() {
                    return Err(miette!("Toxiproxy process exited with status: {}", status));
                }
            }
        }

        Err(miette!("Timeout waiting for Anvil RPC to be ready after {:?}", timeout))
    }

    /// Check if an RPC endpoint is ready by sending a raw HTTP request
    fn is_ready_on_port(port: u16) -> bool {
        let addr = format!("127.0.0.1:{}", port);
        let payload = r#"{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}"#;

        let request = format!(
            "POST / HTTP/1.1\r\n\
             Host: {}\r\n\
             Content-Type: application/json\r\n\
             Content-Length: {}\r\n\
             Connection: close\r\n\
             \r\n\
             {}",
            addr,
            payload.len(),
            payload
        );

        // Try to connect and send request
        if let Ok(mut stream) = TcpStream::connect(&addr) {
            stream.set_read_timeout(Some(Duration::from_secs(1))).ok();
            stream.set_write_timeout(Some(Duration::from_secs(1))).ok();

            if stream.write_all(request.as_bytes()).is_ok() {
                let mut response = String::new();
                if stream.read_to_string(&mut response).is_ok() {
                    // Check if response contains a valid JSON-RPC result
                    return response.contains("\"result\"");
                }
            }
        }

        false
    }

    fn use_toxiproxy_enabled() -> bool {
        match env::var("POLY_BENCH_ANVIL_USE_TOXIPROXY") {
            Ok(v) => {
                let normalized = v.trim().to_ascii_lowercase();
                normalized != "0" && normalized != "false" && normalized != "off"
            }
            Err(_) => true,
        }
    }

    fn proxy_latency_ms() -> u64 {
        env::var("POLY_BENCH_ANVIL_PROXY_LATENCY_MS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(40)
    }

    fn proxy_jitter_ms() -> u64 {
        env::var("POLY_BENCH_ANVIL_PROXY_JITTER_MS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(10)
    }

    fn wait_toxiproxy_ready(api_port: u16, timeout: Duration) -> Result<()> {
        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            if Self::toxiproxy_request(api_port, "GET", "/proxies", None).is_ok() {
                return Ok(());
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        Err(miette!("Timeout waiting for toxiproxy-server API on port {}", api_port))
    }

    fn configure_toxiproxy(
        api_port: u16,
        proxy_port: u16,
        upstream_port: u16,
        latency_ms: u64,
        jitter_ms: u64,
    ) -> Result<()> {
        let create_proxy_body = format!(
            r#"{{"name":"anvil","listen":"127.0.0.1:{proxy_port}","upstream":"127.0.0.1:{upstream_port}"}}"#
        );
        Self::toxiproxy_request(api_port, "POST", "/proxies", Some(&create_proxy_body))?;

        if latency_ms > 0 {
            let downstream_latency = format!(
                r#"{{"name":"latency_downstream","type":"latency","stream":"downstream","attributes":{{"latency":{latency_ms},"jitter":{jitter_ms}}}}}"#
            );
            Self::toxiproxy_request(
                api_port,
                "POST",
                "/proxies/anvil/toxics",
                Some(&downstream_latency),
            )?;

            let upstream_latency = format!(
                r#"{{"name":"latency_upstream","type":"latency","stream":"upstream","attributes":{{"latency":{latency_ms},"jitter":{jitter_ms}}}}}"#
            );
            Self::toxiproxy_request(
                api_port,
                "POST",
                "/proxies/anvil/toxics",
                Some(&upstream_latency),
            )?;
        }

        Ok(())
    }

    fn toxiproxy_request(
        api_port: u16,
        method: &str,
        path: &str,
        body: Option<&str>,
    ) -> Result<String> {
        let addr = format!("127.0.0.1:{}", api_port);
        let payload = body.unwrap_or("");
        let request = if method == "POST" {
            format!(
                "POST {path} HTTP/1.1\r\n\
                 Host: {addr}\r\n\
                 Content-Type: application/json\r\n\
                 Content-Length: {}\r\n\
                 Connection: close\r\n\
                 \r\n\
                 {}",
                payload.len(),
                payload
            )
        } else {
            format!(
                "GET {path} HTTP/1.1\r\n\
                 Host: {addr}\r\n\
                 Connection: close\r\n\
                 \r\n"
            )
        };

        let mut stream = TcpStream::connect(&addr)
            .map_err(|e| miette!("Failed to connect to toxiproxy API {}: {}", addr, e))?;
        stream.set_read_timeout(Some(Duration::from_secs(2))).ok();
        stream.set_write_timeout(Some(Duration::from_secs(2))).ok();
        stream
            .write_all(request.as_bytes())
            .map_err(|e| miette!("Failed to write toxiproxy API request: {}", e))?;
        let mut response = String::new();
        stream
            .read_to_string(&mut response)
            .map_err(|e| miette!("Failed to read toxiproxy API response: {}", e))?;

        if response.starts_with("HTTP/1.1 2") || response.starts_with("HTTP/1.0 2") {
            Ok(response)
        } else {
            Err(miette!("Toxiproxy API error response: {}", response.lines().next().unwrap_or("")))
        }
    }

    /// Stop the Anvil process gracefully
    pub fn stop(&mut self) {
        if let Some(ref mut child) = self.toxiproxy_child {
            let _ = child.kill();
            let _ = child.wait();
        }
        let _ = self.anvil_child.kill();
        let _ = self.anvil_child.wait();
    }
}

impl Drop for AnvilService {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require Anvil to be installed
    // They are ignored by default to avoid CI failures

    #[test]
    #[ignore]
    fn test_spawn_anvil() {
        let config = AnvilConfig::default();
        let service = AnvilService::spawn(&config);
        assert!(service.is_ok());

        let service = service.unwrap();
        assert!(service.rpc_url.starts_with("http://127.0.0.1:"));
        assert!(service.port > 0);
    }
}
