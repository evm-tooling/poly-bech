//! Compile result caching for precheck validation
//!
//! This module provides content-based caching of compilation results to avoid
//! redundant compiler invocations when the generated source code hasn't changed.

use miette::Result;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::sync::RwLock;

/// Cache entry for a single compilation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// SHA256 hash of the source code
    pub source_hash: String,
    /// Whether compilation succeeded
    pub success: bool,
    /// Error message if compilation failed
    pub error: Option<String>,
    /// Timestamp when this entry was created (unix millis)
    pub timestamp: u64,
    /// Language this was compiled for
    pub lang: String,
}

impl CacheEntry {
    /// Check if this cache entry is still valid (not expired)
    pub fn is_valid(&self, max_age: Duration) -> bool {
        let now =
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64;
        let age_ms = now.saturating_sub(self.timestamp);
        age_ms < max_age.as_millis() as u64
    }
}

/// Persistent compile cache stored on disk
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompileCacheData {
    /// Version of the cache format
    pub version: u32,
    /// Map from cache key (benchmark_name:lang:source_hash) to result
    pub entries: HashMap<String, CacheEntry>,
}

impl CompileCacheData {
    pub const CURRENT_VERSION: u32 = 1;

    pub fn new() -> Self {
        Self { version: Self::CURRENT_VERSION, entries: HashMap::new() }
    }
}

/// Thread-safe compile cache with disk persistence
#[derive(Debug)]
pub struct CompileCache {
    /// In-memory cache data
    data: Arc<RwLock<CompileCacheData>>,
    /// Path to the cache file
    cache_path: PathBuf,
    /// Maximum age for cache entries
    max_age: Duration,
    /// Whether caching is enabled
    enabled: bool,
}

impl CompileCache {
    /// Default cache TTL: 24 hours
    pub const DEFAULT_MAX_AGE: Duration = Duration::from_secs(24 * 60 * 60);

    /// Create a new compile cache
    pub fn new(cache_dir: &Path, enabled: bool) -> Self {
        let cache_path = cache_dir.join("compile-cache.json");
        let data = if enabled {
            Self::load_from_disk(&cache_path).unwrap_or_default()
        } else {
            CompileCacheData::new()
        };

        Self {
            data: Arc::new(RwLock::new(data)),
            cache_path,
            max_age: Self::DEFAULT_MAX_AGE,
            enabled,
        }
    }

    /// Create a disabled cache (no-op)
    pub fn disabled() -> Self {
        Self {
            data: Arc::new(RwLock::new(CompileCacheData::new())),
            cache_path: PathBuf::new(),
            max_age: Duration::ZERO,
            enabled: false,
        }
    }

    /// Load cache from disk
    fn load_from_disk(path: &Path) -> Option<CompileCacheData> {
        let content = fs::read_to_string(path).ok()?;
        let data: CompileCacheData = serde_json::from_str(&content).ok()?;

        // Check version compatibility
        if data.version != CompileCacheData::CURRENT_VERSION {
            return None;
        }

        Some(data)
    }

    /// Save cache to disk
    pub async fn save(&self) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let data = self.data.read().await;

        // Ensure parent directory exists
        if let Some(parent) = self.cache_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| miette::miette!("Failed to create cache directory: {}", e))?;
        }

        let content = serde_json::to_string_pretty(&*data)
            .map_err(|e| miette::miette!("Failed to serialize cache: {}", e))?;

        fs::write(&self.cache_path, content)
            .map_err(|e| miette::miette!("Failed to write cache file: {}", e))?;

        Ok(())
    }

    /// Generate a cache key for a benchmark
    fn cache_key(benchmark_name: &str, lang: &str, source_hash: &str) -> String {
        format!("{}:{}:{}", benchmark_name, lang, source_hash)
    }

    /// Hash source code using SHA256
    pub fn hash_source(source: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(source.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Look up a cached compilation result
    pub async fn get(
        &self,
        benchmark_name: &str,
        lang: &str,
        source: &str,
    ) -> Option<std::result::Result<(), String>> {
        if !self.enabled {
            return None;
        }

        let source_hash = Self::hash_source(source);
        let key = Self::cache_key(benchmark_name, lang, &source_hash);

        let data = self.data.read().await;
        let entry = data.entries.get(&key)?;

        // Check if entry is still valid
        if !entry.is_valid(self.max_age) {
            return None;
        }

        // Verify the hash matches (sanity check)
        if entry.source_hash != source_hash {
            return None;
        }

        if entry.success {
            Some(Ok(()))
        } else {
            Some(Err(entry.error.clone().unwrap_or_default()))
        }
    }

    /// Store a compilation result in the cache
    pub async fn set(
        &self,
        benchmark_name: &str,
        lang: &str,
        source: &str,
        result: std::result::Result<(), String>,
    ) {
        if !self.enabled {
            return;
        }

        let source_hash = Self::hash_source(source);
        let key = Self::cache_key(benchmark_name, lang, &source_hash);

        let timestamp =
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64;

        let entry = CacheEntry {
            source_hash,
            success: result.is_ok(),
            error: result.err(),
            timestamp,
            lang: lang.to_string(),
        };

        let mut data = self.data.write().await;
        data.entries.insert(key, entry);
    }

    /// Clear all cache entries
    pub async fn clear(&self) {
        let mut data = self.data.write().await;
        data.entries.clear();
    }

    /// Remove expired entries from the cache
    pub async fn cleanup_expired(&self) {
        if !self.enabled {
            return;
        }

        let mut data = self.data.write().await;
        data.entries.retain(|_, entry| entry.is_valid(self.max_age));
    }

    /// Get cache statistics
    pub async fn stats(&self) -> CacheStats {
        let data = self.data.read().await;
        let total = data.entries.len();
        let valid = data.entries.values().filter(|e| e.is_valid(self.max_age)).count();
        let successes = data.entries.values().filter(|e| e.success).count();

        CacheStats { total_entries: total, valid_entries: valid, successful_compiles: successes }
    }

    /// Check if caching is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_entries: usize,
    pub valid_entries: usize,
    pub successful_compiles: usize,
}

impl std::fmt::Display for CacheStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} entries ({} valid, {} successful)",
            self.total_entries, self.valid_entries, self.successful_compiles
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_cache_hit() {
        let cache = CompileCache::disabled();
        // Disabled cache should always return None
        let result = cache.get("test", "go", "func main() {}").await;
        assert!(result.is_none());
    }

    #[test]
    fn test_hash_source() {
        let hash1 = CompileCache::hash_source("hello world");
        let hash2 = CompileCache::hash_source("hello world");
        let hash3 = CompileCache::hash_source("hello world!");

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
        assert_eq!(hash1.len(), 64); // SHA256 produces 64 hex chars
    }

    #[test]
    fn test_cache_entry_validity() {
        let entry = CacheEntry {
            source_hash: "abc".to_string(),
            success: true,
            error: None,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
            lang: "go".to_string(),
        };

        assert!(entry.is_valid(Duration::from_secs(60)));
        assert!(entry.is_valid(Duration::from_secs(3600)));
    }
}
