//! AST types for the poly-bench DSL

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Source location span for error reporting
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    /// Byte offset start
    pub start: usize,
    /// Byte offset end
    pub end: usize,
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub col: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, col: usize) -> Self {
        Self { start, end, line, col }
    }

    pub fn dummy() -> Self {
        Self { start: 0, end: 0, line: 1, col: 1 }
    }
}

impl Default for Span {
    fn default() -> Self {
        Self::dummy()
    }
}

/// Supported programming languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Lang {
    Go,
    TypeScript,
    Rust,
    Python,
}

impl Lang {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "go" => Some(Lang::Go),
            "ts" | "typescript" => Some(Lang::TypeScript),
            "rust" | "rs" => Some(Lang::Rust),
            "python" | "py" => Some(Lang::Python),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Lang::Go => "go",
            Lang::TypeScript => "ts",
            Lang::Rust => "rust",
            Lang::Python => "python",
        }
    }
}

impl std::fmt::Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A code block with source information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeBlock {
    /// The actual code content
    pub code: String,
    /// Whether this is a multi-line block (braces) or single-line
    pub is_multiline: bool,
    /// Source location
    pub span: Span,
}

impl CodeBlock {
    pub fn new(code: String, is_multiline: bool, span: Span) -> Self {
        Self { code, is_multiline, span }
    }

    /// Create an empty code block
    pub fn empty() -> Self {
        Self {
            code: String::new(),
            is_multiline: false,
            span: Span::dummy(),
        }
    }
}

/// Execution order for benchmarks within a suite
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionOrder {
    /// Run benchmarks in definition order
    Sequential,
    /// Run benchmarks in parallel (where supported)
    Parallel,
    /// Randomize benchmark order
    Random,
}

impl ExecutionOrder {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "sequential" => Some(ExecutionOrder::Sequential),
            "parallel" => Some(ExecutionOrder::Parallel),
            "random" => Some(ExecutionOrder::Random),
            _ => None,
        }
    }
}

impl Default for ExecutionOrder {
    fn default() -> Self {
        ExecutionOrder::Sequential
    }
}

/// Benchmark execution mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BenchMode {
    /// Auto-calibrate iterations based on target time (default)
    Auto,
    /// Fixed number of iterations (legacy behavior)
    Fixed,
}

impl BenchMode {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "auto" => Some(BenchMode::Auto),
            "fixed" => Some(BenchMode::Fixed),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            BenchMode::Auto => "auto",
            BenchMode::Fixed => "fixed",
        }
    }
}

impl Default for BenchMode {
    fn default() -> Self {
        BenchMode::Auto
    }
}

impl std::fmt::Display for BenchMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Structured setup block with explicit sections (Phase 1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructuredSetup {
    /// Import statements (extracted for codegen)
    pub imports: Option<CodeBlock>,
    /// Package-level declarations (vars, types, consts)
    pub declarations: Option<CodeBlock>,
    /// Init code (runs once before benchmarks)
    pub init: Option<CodeBlock>,
    /// Whether init is async (TypeScript only)
    pub async_init: bool,
    /// Helper functions (reusable across benchmarks)
    pub helpers: Option<CodeBlock>,
    /// Source span
    pub span: Span,
}

impl StructuredSetup {
    pub fn new(span: Span) -> Self {
        Self {
            imports: None,
            declarations: None,
            init: None,
            async_init: false,
            helpers: None,
            span,
        }
    }

    /// Check if any section is defined
    pub fn is_empty(&self) -> bool {
        self.imports.is_none()
            && self.declarations.is_none()
            && self.init.is_none()
            && self.helpers.is_none()
    }
}

/// Parameter definition for parameterized fixtures (Phase 5)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FixtureParam {
    /// Parameter name
    pub name: String,
    /// Parameter type (e.g., "int", "string")
    pub param_type: String,
}

impl FixtureParam {
    pub fn new(name: String, param_type: String) -> Self {
        Self { name, param_type }
    }
}

/// A property value in the DSL
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    String(String),
    Number(u64),
    Float(f64),
    Bool(bool),
    Identifier(String),
    /// File reference: @file("path")
    FileRef(String),
}

impl Value {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }
}

/// A standard library import: use std::module
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UseStd {
    /// The module name (e.g., "constants", "math", "chart")
    pub module: String,
    /// Full statement span (from 'use' to end of module name)
    pub span: Span,
    /// Span of the 'use' keyword
    pub use_span: Span,
    /// Span of the 'std' identifier
    pub std_span: Span,
    /// Span of the module name
    pub module_span: Span,
}

impl UseStd {
    pub fn new(module: String, span: Span, use_span: Span, std_span: Span, module_span: Span) -> Self {
        Self { module, span, use_span, std_span, module_span }
    }
}

/// Configuration for spawning an Anvil instance
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnvilSetupConfig {
    /// Optional RPC URL to fork from
    pub fork_url: Option<String>,
    /// Source location of the spawnAnvil() call
    pub span: Span,
}

impl AnvilSetupConfig {
    pub fn new(fork_url: Option<String>, span: Span) -> Self {
        Self { fork_url, span }
    }
}

/// Global setup block for file-level initialization
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalSetup {
    /// Anvil configuration if spawnAnvil() is called
    pub anvil_config: Option<AnvilSetupConfig>,
    /// Source location
    pub span: Span,
}

impl GlobalSetup {
    pub fn new(anvil_config: Option<AnvilSetupConfig>, span: Span) -> Self {
        Self { anvil_config, span }
    }
}

/// Type of chart to generate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChartType {
    /// Horizontal bar chart comparing benchmark times
    BarChart,
    /// Pie chart showing relative time distribution
    PieChart,
    /// Line chart for trend visualization
    LineChart,
}

impl ChartType {
    pub fn from_function_name(name: &str) -> Option<Self> {
        match name {
            "drawBarChart" => Some(ChartType::BarChart),
            "drawPieChart" => Some(ChartType::PieChart),
            "drawLineChart" => Some(ChartType::LineChart),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ChartType::BarChart => "bar",
            ChartType::PieChart => "pie",
            ChartType::LineChart => "line",
        }
    }

    pub fn default_filename(&self) -> &'static str {
        match self {
            ChartType::BarChart => "bar-chart.svg",
            ChartType::PieChart => "pie-chart.svg",
            ChartType::LineChart => "line-chart.svg",
        }
    }
}

impl std::fmt::Display for ChartType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A chart directive to be executed after benchmarks complete
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChartDirective {
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
    /// Output filename (without path)
    pub output_file: Option<String>,
    /// Source location
    pub span: Span,
    
    // Display toggles (default: true unless noted)
    /// Show ops/sec and time per op stats
    pub show_stats: bool,
    /// Show iterations, warmup, timeout config
    pub show_config: bool,
    /// Show "Go: 5 wins" in legend
    pub show_win_counts: bool,
    /// Show geometric mean speedup
    pub show_geo_mean: bool,
    /// Show min/max/p50/p99 distribution (default: false)
    pub show_distribution: bool,
    /// Show bytes/allocs memory stats (default: false)
    pub show_memory: bool,
    /// Show total execution time (default: false)
    pub show_total_time: bool,
    /// Minimal chart mode without extra info (default: false)
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
    /// Max benchmarks to show (0 = all)
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
    /// Decimal places for numbers (default: 2)
    pub precision: Option<u32>,
    /// Time unit: "auto", "ns", "us", "ms", "s"
    pub time_unit: Option<String>,
}

impl ChartDirective {
    pub fn new(chart_type: ChartType, span: Span) -> Self {
        Self {
            chart_type,
            title: None,
            description: None,
            x_label: None,
            y_label: None,
            output_file: None,
            span,
            // Display toggles - defaults
            show_stats: true,
            show_config: true,
            show_win_counts: true,
            show_geo_mean: true,
            show_distribution: true,  // Show distribution stats by default
            show_memory: false,
            show_total_time: false,
            compact: false,
            // Filtering - defaults
            min_speedup: None,
            filter_winner: None,
            include_benchmarks: Vec::new(),
            exclude_benchmarks: Vec::new(),
            limit: None,
            // Sorting - defaults
            sort_by: None,
            sort_order: None,
            // Layout - defaults (None means use chart defaults)
            width: None,
            bar_height: None,
            bar_gap: None,
            margin_left: None,
            // Data display - defaults
            precision: None,
            time_unit: None,
        }
    }

    /// Get the output filename, using default if not specified
    pub fn get_output_file(&self) -> String {
        self.output_file
            .clone()
            .unwrap_or_else(|| self.chart_type.default_filename().to_string())
    }
}

/// Top-level file containing one or more suites
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    /// Standard library imports (use std::module)
    pub use_stds: Vec<UseStd>,
    /// Global setup block (optional)
    pub global_setup: Option<GlobalSetup>,
    /// Benchmark suites
    pub suites: Vec<Suite>,
}

impl File {
    pub fn new(suites: Vec<Suite>) -> Self {
        Self { use_stds: Vec::new(), global_setup: None, suites }
    }

    pub fn with_use_stds(use_stds: Vec<UseStd>, suites: Vec<Suite>) -> Self {
        Self { use_stds, global_setup: None, suites }
    }
    
    pub fn with_global_setup(use_stds: Vec<UseStd>, global_setup: Option<GlobalSetup>, suites: Vec<Suite>) -> Self {
        Self { use_stds, global_setup, suites }
    }
}

/// A benchmark suite
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Suite {
    /// Suite name (identifier)
    pub name: String,
    /// Source location
    pub span: Span,
    /// Optional description
    pub description: Option<String>,
    /// Default iterations for benchmarks in this suite
    pub iterations: Option<u64>,
    /// Warmup iterations
    pub warmup: Option<u64>,
    
    // Phase 4: Suite configuration
    /// Suite-level timeout in milliseconds
    pub timeout: Option<u64>,
    /// Required language implementations for all benchmarks
    pub requires: Vec<Lang>,
    /// Execution order for benchmarks
    pub order: Option<ExecutionOrder>,
    /// Whether to enable comparison tables
    pub compare: bool,
    /// Baseline language for comparison ratios
    pub baseline: Option<Lang>,
    
    // Benchmark accuracy settings
    /// Benchmark execution mode (auto-calibration vs fixed iterations)
    pub mode: Option<BenchMode>,
    /// Target time for auto-calibration in milliseconds (e.g., "3s" = 3000)
    pub target_time_ms: Option<u64>,
    /// Minimum iterations for auto-calibration
    pub min_iterations: Option<u64>,
    /// Maximum iterations for auto-calibration
    pub max_iterations: Option<u64>,
    /// Enable sink/black-box pattern to prevent dead code elimination (default: true)
    pub sink: bool,
    
    /// Global setup block for suite-level initialization (runs once before all benchmarks)
    pub global_setup: Option<GlobalSetup>,
    
    /// Per-language structured setup blocks (Phase 1)
    pub setups: HashMap<Lang, StructuredSetup>,
    /// Named fixtures
    pub fixtures: Vec<Fixture>,
    /// Benchmark definitions
    pub benchmarks: Vec<Benchmark>,
    /// Chart directives to execute after benchmarks complete
    pub chart_directives: Vec<ChartDirective>,
}

impl Suite {
    pub fn new(name: String, span: Span) -> Self {
        Self {
            name,
            span,
            description: None,
            iterations: None,
            warmup: None,
            timeout: None,
            requires: Vec::new(),
            order: None,
            compare: false,
            baseline: None,
            mode: None,
            target_time_ms: None,
            min_iterations: None,
            max_iterations: None,
            sink: true, // Enabled by default to prevent DCE
            global_setup: None,
            setups: HashMap::new(),
            fixtures: Vec::new(),
            benchmarks: Vec::new(),
            chart_directives: Vec::new(),
        }
    }
}

/// A fixture definition for shared test data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fixture {
    /// Fixture name (identifier)
    pub name: String,
    /// Source location
    pub span: Span,
    /// Optional description
    pub description: Option<String>,
    /// Hex-encoded data (portable format)
    pub hex_data: Option<String>,
    /// File reference for hex data
    pub hex_file: Option<String>,
    /// Per-language implementations (alternative to hex)
    pub implementations: HashMap<Lang, CodeBlock>,
    
    // Phase 5: Enhanced fixture system
    /// Shape annotation for documentation (JSON-like descriptor)
    pub shape: Option<String>,
    /// Parameters for parameterized fixtures
    pub params: Vec<FixtureParam>,
}

impl Fixture {
    pub fn new(name: String, span: Span) -> Self {
        Self {
            name,
            span,
            description: None,
            hex_data: None,
            hex_file: None,
            implementations: HashMap::new(),
            shape: None,
            params: Vec::new(),
        }
    }

    /// Check if this is a parameterized fixture
    pub fn is_parameterized(&self) -> bool {
        !self.params.is_empty()
    }
}

/// A benchmark definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Benchmark {
    /// Benchmark name (identifier)
    pub name: String,
    /// Source location
    pub span: Span,
    /// Optional description
    pub description: Option<String>,
    /// Override iterations for this specific benchmark
    pub iterations: Option<u64>,
    
    // Phase 2: Benchmark configuration
    /// Override warmup iterations for this benchmark
    pub warmup: Option<u64>,
    /// Timeout in milliseconds for this benchmark
    pub timeout: Option<u64>,
    /// Tags for filtering/grouping
    pub tags: Vec<String>,
    /// Per-language skip conditions
    pub skip: HashMap<Lang, CodeBlock>,
    /// Per-language result validation expressions
    pub validate: HashMap<Lang, CodeBlock>,
    
    // Benchmark accuracy settings (overrides suite-level)
    /// Override benchmark execution mode
    pub mode: Option<BenchMode>,
    /// Override target time for auto-calibration
    pub target_time_ms: Option<u64>,
    /// Override minimum iterations
    pub min_iterations: Option<u64>,
    /// Override maximum iterations
    pub max_iterations: Option<u64>,
    /// Override sink/black-box setting (None = inherit from suite)
    pub sink: Option<bool>,
    
    // Phase 3: Lifecycle hooks
    /// Pre-benchmark hook (runs once before iterations)
    pub before: HashMap<Lang, CodeBlock>,
    /// Post-benchmark hook (runs once after iterations)
    pub after: HashMap<Lang, CodeBlock>,
    /// Per-iteration hook (runs before each iteration, outside timing)
    pub each: HashMap<Lang, CodeBlock>,
    
    /// Per-language implementations
    pub implementations: HashMap<Lang, CodeBlock>,
}

impl Benchmark {
    pub fn new(name: String, span: Span) -> Self {
        Self {
            name,
            span,
            description: None,
            iterations: None,
            warmup: None,
            timeout: None,
            tags: Vec::new(),
            skip: HashMap::new(),
            validate: HashMap::new(),
            mode: None,
            target_time_ms: None,
            min_iterations: None,
            max_iterations: None,
            sink: None,
            before: HashMap::new(),
            after: HashMap::new(),
            each: HashMap::new(),
            implementations: HashMap::new(),
        }
    }

    /// Check if this benchmark has any lifecycle hooks for a language
    pub fn has_hooks(&self, lang: Lang) -> bool {
        self.before.contains_key(&lang)
            || self.after.contains_key(&lang)
            || self.each.contains_key(&lang)
    }
}
