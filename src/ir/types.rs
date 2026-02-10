//! IR types - normalized benchmark specifications

use crate::dsl::Lang;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// A complete benchmark IR with all suites
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkIR {
    /// All benchmark suites
    pub suites: Vec<SuiteIR>,
}

impl BenchmarkIR {
    pub fn new(suites: Vec<SuiteIR>) -> Self {
        Self { suites }
    }

    /// Get all benchmarks across all suites
    pub fn all_benchmarks(&self) -> impl Iterator<Item = (&SuiteIR, &BenchmarkSpec)> {
        self.suites.iter().flat_map(|suite| {
            suite.benchmarks.iter().map(move |bench| (suite, bench))
        })
    }
}

/// A normalized benchmark suite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteIR {
    /// Suite name
    pub name: String,
    /// Optional description
    pub description: Option<String>,
    /// Default iterations for benchmarks
    pub default_iterations: u64,
    /// Default warmup iterations
    pub default_warmup: u64,
    /// Per-language imports (extracted from setup blocks)
    pub imports: HashMap<Lang, Vec<String>>,
    /// Per-language setup code (body only, imports extracted)
    pub setups: HashMap<Lang, String>,
    /// Resolved fixtures
    pub fixtures: Vec<FixtureIR>,
    /// Benchmark specifications
    pub benchmarks: Vec<BenchmarkSpec>,
}

impl SuiteIR {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            default_iterations: 1000,
            default_warmup: 100,
            imports: HashMap::new(),
            setups: HashMap::new(),
            fixtures: Vec::new(),
            benchmarks: Vec::new(),
        }
    }

    /// Get a fixture by name
    pub fn get_fixture(&self, name: &str) -> Option<&FixtureIR> {
        self.fixtures.iter().find(|f| f.name == name)
    }
}

/// A resolved fixture with data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixtureIR {
    /// Fixture name
    pub name: String,
    /// Optional description
    pub description: Option<String>,
    /// Raw bytes (decoded from hex)
    pub data: Vec<u8>,
    /// Per-language code for complex fixtures
    pub implementations: HashMap<Lang, String>,
}

impl FixtureIR {
    pub fn new(name: String, data: Vec<u8>) -> Self {
        Self {
            name,
            description: None,
            data,
            implementations: HashMap::new(),
        }
    }

    /// Get hex representation of the data
    pub fn as_hex(&self) -> String {
        hex::encode(&self.data)
    }

    /// Get hex with 0x prefix
    pub fn as_hex_prefixed(&self) -> String {
        format!("0x{}", self.as_hex())
    }

    /// Get Go byte slice literal
    pub fn as_go_bytes(&self) -> String {
        if self.data.is_empty() {
            return "[]byte{}".to_string();
        }
        
        let bytes: Vec<String> = self.data.iter()
            .map(|b| format!("0x{:02x}", b))
            .collect();
        
        format!("[]byte{{{}}}", bytes.join(", "))
    }

    /// Get JavaScript Uint8Array literal
    pub fn as_js_uint8array(&self) -> String {
        if self.data.is_empty() {
            return "new Uint8Array([])".to_string();
        }
        
        let bytes: Vec<String> = self.data.iter()
            .map(|b| format!("0x{:02x}", b))
            .collect();
        
        format!("new Uint8Array([{}])", bytes.join(", "))
    }
}

/// A single benchmark specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSpec {
    /// Benchmark name
    pub name: String,
    /// Full qualified name (suite_name.bench_name)
    pub full_name: String,
    /// Optional description
    pub description: Option<String>,
    /// Number of iterations to run
    pub iterations: u64,
    /// Number of warmup iterations
    pub warmup: u64,
    /// Per-language implementations
    pub implementations: HashMap<Lang, String>,
    /// Referenced fixtures
    pub fixture_refs: Vec<String>,
}

impl BenchmarkSpec {
    pub fn new(name: String, suite_name: &str, iterations: u64, warmup: u64) -> Self {
        Self {
            full_name: format!("{}_{}", suite_name, name),
            name,
            description: None,
            iterations,
            warmup,
            implementations: HashMap::new(),
            fixture_refs: Vec::new(),
        }
    }

    /// Check if this benchmark has an implementation for a language
    pub fn has_lang(&self, lang: Lang) -> bool {
        self.implementations.contains_key(&lang)
    }

    /// Get the implementation for a language
    pub fn get_impl(&self, lang: Lang) -> Option<&str> {
        self.implementations.get(&lang).map(|s| s.as_str())
    }
}
