//! Hover result caching for the LSP v2
//!
//! This module provides TTL-based caching for hover results to reduce
//! redundant language server calls and improve responsiveness.

use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant},
};

use dashmap::DashMap;
use once_cell::sync::Lazy;
use tower_lsp::lsp_types::{Hover, Position, Url};

/// Cache TTL for embedded language hover results (in milliseconds)
const HOVER_CACHE_TTL_MS: u64 = 3000;

/// Cache key for hover requests
#[derive(Clone, PartialEq, Eq, Hash)]
struct HoverCacheKey {
    uri: String,
    line: u32,
    character: u32,
}

/// Cached hover result
struct CachedHover {
    hover: Option<Hover>,
    timestamp: Instant,
}

/// Global hover cache for embedded language results
static HOVER_CACHE: Lazy<DashMap<HoverCacheKey, CachedHover>> = Lazy::new(DashMap::new);

/// Counter for cache cleanup (run cleanup every N requests)
static CACHE_CLEANUP_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Check if a cached hover result is still valid
pub fn get_cached_hover(uri: &Url, position: Position) -> Option<Option<Hover>> {
    let key =
        HoverCacheKey { uri: uri.to_string(), line: position.line, character: position.character };

    if let Some(cached) = HOVER_CACHE.get(&key) {
        if cached.timestamp.elapsed() < Duration::from_millis(HOVER_CACHE_TTL_MS) {
            tracing::trace!(
                "[hover-cache] Cache hit for {:?} at {}:{}",
                uri.path(),
                position.line,
                position.character
            );
            return Some(cached.hover.clone());
        }
    }
    None
}

/// Store a hover result in the cache
pub fn cache_hover(uri: &Url, position: Position, hover: Option<Hover>) {
    let key =
        HoverCacheKey { uri: uri.to_string(), line: position.line, character: position.character };

    HOVER_CACHE.insert(key, CachedHover { hover, timestamp: Instant::now() });

    // Periodically clean up old entries (every 100 requests)
    let count = CACHE_CLEANUP_COUNTER.fetch_add(1, Ordering::Relaxed);
    if count % 100 == 0 {
        cleanup_expired_cache();
    }
}

/// Remove expired entries from the cache
fn cleanup_expired_cache() {
    let ttl = Duration::from_millis(HOVER_CACHE_TTL_MS * 2);
    HOVER_CACHE.retain(|_, v| v.timestamp.elapsed() < ttl);
}

/// Invalidate all cache entries for a specific document
pub fn invalidate_document_cache(uri: &Url) {
    let uri_str = uri.to_string();
    HOVER_CACHE.retain(|k, _| k.uri != uri_str);
}

/// Clear the entire hover cache
pub fn clear_cache() {
    HOVER_CACHE.clear();
}

/// Get cache statistics for debugging
pub fn cache_stats() -> (usize, usize) {
    let total = HOVER_CACHE.len();
    let valid = HOVER_CACHE
        .iter()
        .filter(|entry| entry.timestamp.elapsed() < Duration::from_millis(HOVER_CACHE_TTL_MS))
        .count();
    (total, valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_hit() {
        let uri = Url::parse("file:///test.bench").unwrap();
        let position = Position { line: 10, character: 5 };
        let hover = Hover {
            contents: tower_lsp::lsp_types::HoverContents::Markup(
                tower_lsp::lsp_types::MarkupContent {
                    kind: tower_lsp::lsp_types::MarkupKind::Markdown,
                    value: "test".to_string(),
                },
            ),
            range: None,
        };

        cache_hover(&uri, position, Some(hover.clone()));

        let cached = get_cached_hover(&uri, position);
        assert!(cached.is_some());
        assert!(cached.unwrap().is_some());
    }

    #[test]
    fn test_cache_miss() {
        let uri = Url::parse("file:///test2.bench").unwrap();
        let position = Position { line: 20, character: 10 };

        let cached = get_cached_hover(&uri, position);
        assert!(cached.is_none());
    }

    #[test]
    fn test_invalidate_document() {
        let uri = Url::parse("file:///test3.bench").unwrap();
        let position1 = Position { line: 1, character: 0 };
        let position2 = Position { line: 2, character: 0 };

        cache_hover(&uri, position1, None);
        cache_hover(&uri, position2, None);

        invalidate_document_cache(&uri);

        assert!(get_cached_hover(&uri, position1).is_none());
        assert!(get_cached_hover(&uri, position2).is_none());
    }
}
