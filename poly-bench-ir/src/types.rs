//! IR types - normalized benchmark specifications

use poly_bench_dsl::{BenchMode, ChartType, ExecutionOrder, Lang};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Configuration for spawning an Anvil instance (from globalSetup)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnvilConfigIR {
    /// Optional RPC URL to fork from
    pub fork_url: Option<String>,
}

impl AnvilConfigIR {
    pub fn new(fork_url: Option<String>) -> Self {
        Self { fork_url }
    }
}

/// A complete benchmark IR with all suites
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkIR {
    /// Standard library modules imported (e.g., "constants", "math")
    pub stdlib_imports: HashSet<String>,
    /// Anvil configuration from globalSetup (if spawnAnvil() was called)
    pub anvil_config: Option<AnvilConfigIR>,
    /// All benchmark suites
    pub suites: Vec<SuiteIR>,
    /// Chart directives to execute after all benchmarks complete
    pub chart_directives: Vec<ChartDirectiveIR>,
}

impl BenchmarkIR {
    pub fn new(suites: Vec<SuiteIR>) -> Self {
        Self {
            stdlib_imports: HashSet::new(),
            anvil_config: None,
            suites,
            chart_directives: Vec::new(),
        }
    }

    pub fn with_stdlib(stdlib_imports: HashSet<String>, suites: Vec<SuiteIR>) -> Self {
        Self { stdlib_imports, anvil_config: None, suites, chart_directives: Vec::new() }
    }

    pub fn with_anvil(
        stdlib_imports: HashSet<String>,
        anvil_config: Option<AnvilConfigIR>,
        suites: Vec<SuiteIR>,
    ) -> Self {
        Self { stdlib_imports, anvil_config, suites, chart_directives: Vec::new() }
    }

    pub fn with_charts(
        stdlib_imports: HashSet<String>,
        anvil_config: Option<AnvilConfigIR>,
        suites: Vec<SuiteIR>,
        chart_directives: Vec<ChartDirectiveIR>,
    ) -> Self {
        Self { stdlib_imports, anvil_config, suites, chart_directives }
    }

    /// Get all benchmarks across all suites
    pub fn all_benchmarks(&self) -> impl Iterator<Item = (&SuiteIR, &BenchmarkSpec)> {
        self.suites
            .iter()
            .flat_map(|suite| suite.benchmarks.iter().map(move |bench| (suite, bench)))
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

    // Benchmark accuracy settings
    /// Benchmark execution mode (auto-calibration vs fixed iterations)
    pub mode: BenchMode,
    /// Target time for auto-calibration in milliseconds
    pub target_time_ms: u64,
    /// Minimum iterations for auto-calibration
    pub min_iterations: u64,
    /// Maximum iterations for auto-calibration
    pub max_iterations: u64,
    /// Enable sink/black-box pattern to prevent dead code elimination
    pub sink: bool,

    // Statistical analysis settings
    /// Enable IQR-based outlier detection
    pub outlier_detection: bool,
    /// Coefficient of variation threshold percentage for stability check
    pub cv_threshold: f64,
    /// Number of times to run each benchmark for statistical consistency
    pub count: u64,

    // Observability settings (Phase 2B)
    /// Enable memory allocation profiling
    pub memory: bool,
    /// Number of concurrent goroutines/workers for parallel execution
    pub concurrency: u32,

    /// Standard library modules imported (e.g., "constants", "anvil")
    pub stdlib_imports: HashSet<String>,

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

/// Default CV threshold percentage (5%) - matches poly-bench-runtime
pub const DEFAULT_CV_THRESHOLD: f64 = 5.0;

impl SuiteIR {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            default_iterations: 1000,
            default_warmup: 1000, // Increased from 100 for better JIT optimization
            timeout: None,
            requires: Vec::new(),
            order: ExecutionOrder::Sequential,
            compare: false,
            baseline: None,
            // Benchmark accuracy defaults
            mode: BenchMode::Auto,     // Auto-calibration by default
            target_time_ms: 3000,      // 3 seconds target
            min_iterations: 10,        // At least 10 iterations
            max_iterations: 1_000_000, // Cap at 1M iterations
            sink: true,                // Enabled by default to prevent DCE
            // Statistical analysis defaults
            outlier_detection: true, // Enabled by default for statistical accuracy
            cv_threshold: DEFAULT_CV_THRESHOLD, // 5% threshold
            count: 1,                // Single run by default (backward compatible)
            // Observability defaults
            memory: false,  // Memory profiling disabled by default
            concurrency: 1, // Single-threaded by default
            stdlib_imports: HashSet::new(),
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

        let bytes: Vec<String> = self.data.iter().map(|b| format!("0x{:02x}", b)).collect();

        format!("[]byte{{{}}}", bytes.join(", "))
    }

    /// Get JavaScript Uint8Array literal
    pub fn as_js_uint8array(&self) -> String {
        if self.data.is_empty() {
            return "new Uint8Array([])".to_string();
        }

        let bytes: Vec<String> = self.data.iter().map(|b| format!("0x{:02x}", b)).collect();

        format!("new Uint8Array([{}])", bytes.join(", "))
    }

    /// Get Rust byte slice literal (comma-separated hex bytes for vec! or array)
    pub fn as_rust_bytes(&self) -> String {
        if self.data.is_empty() {
            return String::new();
        }

        let bytes: Vec<String> = self.data.iter().map(|b| format!("0x{:02x}", b)).collect();

        bytes.join(", ")
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
    /// Number of iterations to run (for fixed mode)
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

    // Benchmark accuracy settings (resolved from suite + benchmark overrides)
    /// Benchmark execution mode
    pub mode: BenchMode,
    /// Target time for auto-calibration in milliseconds
    pub target_time_ms: u64,
    /// Minimum iterations for auto-calibration
    pub min_iterations: u64,
    /// Maximum iterations for auto-calibration
    pub max_iterations: u64,
    /// Enable sink/black-box pattern
    pub use_sink: bool,
    /// Enable IQR-based outlier detection
    pub outlier_detection: bool,
    /// Coefficient of variation threshold percentage for stability check
    pub cv_threshold: f64,
    /// Number of times to run this benchmark for statistical consistency
    pub count: u64,

    // Observability settings (Phase 2B)
    /// Enable memory allocation profiling
    pub memory: bool,
    /// Number of concurrent goroutines/workers for parallel execution
    pub concurrency: u32,

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
            // Benchmark accuracy defaults (will be overwritten by lower.rs)
            mode: BenchMode::Auto,
            target_time_ms: 3000,
            min_iterations: 10,
            max_iterations: 1_000_000,
            use_sink: true,
            outlier_detection: true,
            cv_threshold: DEFAULT_CV_THRESHOLD,
            count: 1,
            memory: false,
            concurrency: 1,
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
        self.before_hooks.contains_key(&lang) ||
            self.after_hooks.contains_key(&lang) ||
            self.each_hooks.contains_key(&lang)
    }

    /// Check if this benchmark should be skipped for a language
    pub fn should_skip(&self, lang: Lang) -> bool {
        self.skip_conditions.contains_key(&lang)
    }
}

/// A chart directive to be executed after benchmarks complete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartDirectiveIR {
    /// Type of chart to generate
    pub chart_type: ChartType,
    /// Chart title
    pub title: Option<String>,
    /// Chart description/subtitle
    pub description: Option<String>,
    /// X-axis label
    pub x_label: Option<String>,
    /// Y-axis label
    pub y_label: Option<String>,
    /// Output filename
    pub output_file: String,
    /// Suite name this directive belongs to (for scoping results)
    pub suite_name: Option<String>,

    // Display toggles
    /// Show ops/sec and time per op stats
    pub show_stats: bool,
    /// Show iterations, warmup, timeout config
    pub show_config: bool,
    /// Show "Go: 5 wins" in legend
    pub show_win_counts: bool,
    /// Show geometric mean speedup
    pub show_geo_mean: bool,
    /// Show min/max/p50/p99 distribution
    pub show_distribution: bool,
    /// Show bytes/allocs memory stats
    pub show_memory: bool,
    /// Show total execution time
    pub show_total_time: bool,
    /// Minimal chart mode
    pub compact: bool,

    // Filtering
    /// Only show benchmarks with speedup >= N
    pub min_speedup: Option<f64>,
    /// Filter by winner: "go", "ts", or "all"
    pub filter_winner: Option<String>,
    /// Only include these benchmark names
    pub include_benchmarks: Vec<String>,
    /// Exclude these benchmark names
    pub exclude_benchmarks: Vec<String>,
    /// Max benchmarks to show
    pub limit: Option<u32>,

    // Sorting
    /// Sort by: "speedup", "name", "time", "ops"
    pub sort_by: Option<String>,
    /// Sort order: "asc" or "desc"
    pub sort_order: Option<String>,

    // Layout
    /// Chart width in pixels
    pub width: Option<i32>,
    /// Height of each bar in pixels
    pub bar_height: Option<i32>,
    /// Gap between bars in pixels
    pub bar_gap: Option<i32>,
    /// Left margin for labels in pixels
    pub margin_left: Option<i32>,

    // Data display
    /// Decimal places for numbers
    pub precision: Option<u32>,
    /// Time unit: "auto", "ns", "us", "ms", "s"
    pub time_unit: Option<String>,
}

impl ChartDirectiveIR {
    pub fn new(chart_type: ChartType, output_file: String) -> Self {
        Self {
            chart_type,
            title: None,
            description: None,
            x_label: None,
            y_label: None,
            output_file,
            suite_name: None,
            // Display toggles - defaults
            show_stats: true,
            show_config: true,
            show_win_counts: true,
            show_geo_mean: true,
            show_distribution: true, // Show distribution stats by default
            show_memory: false,
            show_total_time: false,
            compact: false,
            // Filtering
            min_speedup: None,
            filter_winner: None,
            include_benchmarks: Vec::new(),
            exclude_benchmarks: Vec::new(),
            limit: None,
            // Sorting
            sort_by: None,
            sort_order: None,
            // Layout
            width: None,
            bar_height: None,
            bar_gap: None,
            margin_left: None,
            // Data display
            precision: None,
            time_unit: None,
        }
    }

    /// Get the title to display, with a sensible default
    pub fn get_title(&self) -> String {
        self.title.clone().unwrap_or_else(|| match self.chart_type {
            ChartType::BarChart => "Benchmark Results".to_string(),
            ChartType::PieChart => "Time Distribution".to_string(),
            ChartType::LineChart => "Performance Trend".to_string(),
        })
    }

    /// Get the x-axis label with a sensible default
    pub fn get_x_label(&self) -> String {
        self.x_label.clone().unwrap_or_else(|| match self.chart_type {
            ChartType::BarChart => "Time".to_string(),
            ChartType::PieChart => "".to_string(),
            ChartType::LineChart => "Benchmark".to_string(),
        })
    }

    /// Get the y-axis label with a sensible default
    pub fn get_y_label(&self) -> String {
        self.y_label.clone().unwrap_or_else(|| match self.chart_type {
            ChartType::BarChart => "Benchmark".to_string(),
            ChartType::PieChart => "".to_string(),
            ChartType::LineChart => "Time".to_string(),
        })
    }
}
