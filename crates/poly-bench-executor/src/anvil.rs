//! Anvil service management
//!
//! Handles spawning and managing a local Anvil Ethereum node for EVM benchmarks.

use miette::{miette, Result};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

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
    /// The child process
    child: Child,
    /// The RPC URL (http://127.0.0.1:PORT)
    pub rpc_url: String,
    /// The port Anvil is listening on
    pub port: u16,
}

impl AnvilService {
    /// Spawn a new Anvil instance and wait for it to be ready
    pub fn spawn(config: &AnvilConfig) -> Result<Self> {
        // Find an available port
        let port = Self::find_available_port()?;

        // Build command arguments
        let mut args = vec![
            "--port".to_string(),
            port.to_string(),
            "--silent".to_string(),
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
        let child = Command::new("anvil")
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| miette!("Failed to spawn anvil (is it installed?): {}", e))?;

        let rpc_url = format!("http://127.0.0.1:{}", port);

        let mut service = Self {
            child,
            rpc_url,
            port,
        };

        // Wait for Anvil to be ready
        service.wait_ready(Duration::from_secs(30))?;

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
        let deadline = Instant::now() + timeout;

        while Instant::now() < deadline {
            if self.is_ready() {
                return Ok(());
            }
            std::thread::sleep(Duration::from_millis(100));
        }

        // Check if process died
        if let Ok(Some(status)) = self.child.try_wait() {
            return Err(miette!("Anvil process exited with status: {}", status));
        }

        Err(miette!(
            "Timeout waiting for Anvil to be ready after {:?}",
            timeout
        ))
    }

    /// Check if Anvil is ready by sending a raw HTTP request
    fn is_ready(&self) -> bool {
        let addr = format!("127.0.0.1:{}", self.port);
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

    /// Stop the Anvil process gracefully
    pub fn stop(&mut self) {
        // Try SIGTERM first
        let _ = self.child.kill();

        // Wait for process to exit
        let _ = self.child.wait();
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
