//! IR types - normalized benchmark specifications

use crate::dsl::{Lang, ExecutionOrder};
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};

/// A complete benchmark IR with all suites
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkIR {
    /// Standard library modules imported (e.g., "constants", "math")
    pub stdlib_imports: HashSet<String>,
    /// All benchmark suites
    pub suites: Vec<SuiteIR>,
}

impl BenchmarkIR {
    pub fn new(suites: Vec<SuiteIR>) -> Self {
        Self { stdlib_imports: HashSet::new(), suites }
    }

    pub fn with_stdlib(stdlib_imports: HashSet<String>, suites: Vec<SuiteIR>) -> Self {
        Self { stdlib_imports, suites }
    }

    /// Get all benchmarks across all suites
    pub fn all_benchmarks(&self) -> impl Iterator<Item = (&SuiteIR, &BenchmarkSpec)> {
        self.suites.iter().flat_map(|suite| {
            suite.benchmarks.iter().map(move |bench| (suite, bench))
        })
    }

    /// Check if a stdlib module is imported
    pub fn has_stdlib(&self, module: &str) -> bool {
        self.stdlib_imports.contains(module)
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
    
    // Phase 4: Suite-level configuration
    /// Suite-level timeout in milliseconds
    pub timeout: Option<u64>,
    /// Required language implementations for all benchmarks
    pub requires: Vec<Lang>,
    /// Execution order for benchmarks
    pub order: ExecutionOrder,
    /// Whether to enable comparison tables
    pub compare: bool,
    /// Baseline language for comparison ratios
    pub baseline: Option<Lang>,
    
    // Phase 1: Structured setup (separated into sections)
    /// Per-language imports (extracted from setup blocks)
    pub imports: HashMap<Lang, Vec<String>>,
    /// Per-language declarations (package-level vars, types, consts)
    pub declarations: HashMap<Lang, String>,
    /// Per-language init code (runs once before benchmarks)
    pub init_code: HashMap<Lang, String>,
    /// Whether init is async (TypeScript only)
    pub async_init: HashMap<Lang, bool>,
    /// Per-language helper functions
    pub helpers: HashMap<Lang, String>,
    
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
            timeout: None,
            requires: Vec::new(),
            order: ExecutionOrder::Sequential,
            compare: false,
            baseline: None,
            imports: HashMap::new(),
            declarations: HashMap::new(),
            init_code: HashMap::new(),
            async_init: HashMap::new(),
            helpers: HashMap::new(),
            fixtures: Vec::new(),
            benchmarks: Vec::new(),
        }
    }

    /// Get a fixture by name
    pub fn get_fixture(&self, name: &str) -> Option<&FixtureIR> {
        self.fixtures.iter().find(|f| f.name == name)
    }

    /// Check if any setup section has async init
    pub fn has_async_init(&self, lang: Lang) -> bool {
        self.async_init.get(&lang).copied().unwrap_or(false)
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
    
    // Phase 5: Enhanced fixture system
    /// Shape annotation for documentation
    pub shape: Option<String>,
    /// Parameter definitions for parameterized fixtures
    pub params: Vec<FixtureParamIR>,
}

/// Parameter definition for parameterized fixtures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixtureParamIR {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: String,
}

impl FixtureIR {
    pub fn new(name: String, data: Vec<u8>) -> Self {
        Self {
            name,
            description: None,
            data,
            implementations: HashMap::new(),
            shape: None,
            params: Vec::new(),
        }
    }

    /// Check if this is a parameterized fixture
    pub fn is_parameterized(&self) -> bool {
        !self.params.is_empty()
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
    
    // Phase 2: Benchmark configuration
    /// Timeout in milliseconds
    pub timeout: Option<u64>,
    /// Tags for filtering/grouping
    pub tags: Vec<String>,
    /// Per-language skip conditions
    pub skip_conditions: HashMap<Lang, String>,
    /// Per-language result validations
    pub validations: HashMap<Lang, String>,
    
    // Phase 3: Lifecycle hooks
    /// Pre-benchmark hook (runs once before iterations)
    pub before_hooks: HashMap<Lang, String>,
    /// Post-benchmark hook (runs once after iterations)
    pub after_hooks: HashMap<Lang, String>,
    /// Per-iteration hook (runs before each iteration, outside timing)
    pub each_hooks: HashMap<Lang, String>,
    
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
            timeout: None,
            tags: Vec::new(),
            skip_conditions: HashMap::new(),
            validations: HashMap::new(),
            before_hooks: HashMap::new(),
            after_hooks: HashMap::new(),
            each_hooks: HashMap::new(),
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

    /// Check if this benchmark has any lifecycle hooks for a language
    pub fn has_hooks(&self, lang: Lang) -> bool {
        self.before_hooks.contains_key(&lang)
            || self.after_hooks.contains_key(&lang)
            || self.each_hooks.contains_key(&lang)
    }

    /// Check if this benchmark should be skipped for a language
    pub fn should_skip(&self, lang: Lang) -> bool {
        self.skip_conditions.contains_key(&lang)
    }
}
