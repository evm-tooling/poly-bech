//! AST types for the poly-bench DSL

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    C,
    CSharp,
}

impl Lang {
    pub const ALL: [Lang; 6] =
        [Lang::Go, Lang::TypeScript, Lang::Rust, Lang::Python, Lang::C, Lang::CSharp];

    pub fn all() -> &'static [Lang] {
        &Self::ALL
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "go" => Some(Lang::Go),
            "ts" | "typescript" => Some(Lang::TypeScript),
            "rust" | "rs" => Some(Lang::Rust),
            "python" | "py" => Some(Lang::Python),
            "c" => Some(Lang::C),
            "csharp" | "cs" => Some(Lang::CSharp),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Lang::Go => "go",
            Lang::TypeScript => "ts",
            Lang::Rust => "rust",
            Lang::Python => "python",
            Lang::C => "c",
            Lang::CSharp => "csharp",
        }
    }

    /// Language tags accepted in grammar language positions.
    pub fn aliases(&self) -> &'static [&'static str] {
        match self {
            Lang::Go => &["go"],
            Lang::TypeScript => &["ts", "typescript"],
            Lang::Rust => &["rust"],
            Lang::Python => &["python", "py"],
            Lang::C => &["c"],
            Lang::CSharp => &["csharp", "cs"],
        }
    }

    /// Display name used in generated grammar comments.
    pub fn grammar_display_name(&self) -> &'static str {
        match self {
            Lang::Go => "Go",
            Lang::TypeScript => "TypeScript",
            Lang::Rust => "Rust",
            Lang::Python => "Python",
            Lang::C => "C",
            Lang::CSharp => "C#",
        }
    }

    /// Tree-sitter injection language identifier.
    pub fn tree_sitter_injection_name(&self) -> &'static str {
        match self {
            Lang::CSharp => "c_sharp",
            _ => self.as_str(),
        }
    }

    /// Import sections use paren blocks for Go only.
    pub fn uses_paren_import_block(&self) -> bool {
        matches!(self, Lang::Go)
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
        Self { code: String::new(), is_multiline: false, span: Span::dummy() }
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

/// Fairness strategy for cross-runtime execution and comparison
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FairnessMode {
    /// Legacy behavior (fixed runtime order, legacy stats)
    Legacy,
    /// Strict fairness behavior (interleaving, robust paired stats)
    Strict,
}

impl FairnessMode {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "legacy" => Some(FairnessMode::Legacy),
            "strict" => Some(FairnessMode::Strict),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            FairnessMode::Legacy => "legacy",
            FairnessMode::Strict => "strict",
        }
    }
}

impl Default for FairnessMode {
    fn default() -> Self {
        FairnessMode::Strict
    }
}

/// Sampling policy for async benchmarks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AsyncSamplingPolicy {
    /// Legacy fixed cap behavior
    FixedCap,
    /// Time budgeted adaptive behavior
    TimeBudgeted,
}

impl AsyncSamplingPolicy {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "fixedcap" | "fixed_cap" | "fixed-cap" => Some(AsyncSamplingPolicy::FixedCap),
            "timebudgeted" | "time_budgeted" | "time-budgeted" => {
                Some(AsyncSamplingPolicy::TimeBudgeted)
            }
            _ => None,
        }
    }
}

impl Default for AsyncSamplingPolicy {
    fn default() -> Self {
        AsyncSamplingPolicy::TimeBudgeted
    }
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

/// High-level suite category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuiteType {
    /// Memory-focused benchmarking suites
    Memory,
    /// CPU/performance-focused benchmarking suites
    Performance,
}

impl SuiteType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "memory" => Some(SuiteType::Memory),
            "performance" => Some(SuiteType::Performance),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            SuiteType::Memory => "memory",
            SuiteType::Performance => "performance",
        }
    }
}

/// Suite-level run configuration mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunMode {
    /// Time-budgeted mode (targetTime is valid)
    Time,
    /// Fixed-iterations mode (iterations is valid)
    Iterations,
}

impl RunMode {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "time" | "timebased" => Some(RunMode::Time),
            "iterations" | "iterationbased" => Some(RunMode::Iterations),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            RunMode::Time => "time",
            RunMode::Iterations => "iterations",
        }
    }

    pub fn as_header_str(&self) -> &'static str {
        match self {
            RunMode::Time => "timeBased",
            RunMode::Iterations => "iterationBased",
        }
    }
}

/// Benchmark kind (sync vs async sequential)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BenchmarkKind {
    /// Traditional synchronous benchmark
    #[default]
    Sync,
    /// Async sequential benchmark (one awaited completion per iteration)
    Async,
}

impl BenchmarkKind {
    pub fn as_keyword(&self) -> &'static str {
        match self {
            BenchmarkKind::Sync => "bench",
            BenchmarkKind::Async => "benchAsync",
        }
    }
}

/// Style for lifecycle hooks (before/after/each) - tracks original syntax
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum HookStyle {
    /// Grouped syntax: before: { go: ... ts: ... }
    #[default]
    Grouped,
    /// Flat syntax: before go: ... \n before ts: ...
    Flat,
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
        self.imports.is_none() &&
            self.declarations.is_none() &&
            self.init.is_none() &&
            self.helpers.is_none()
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
    pub fn new(
        module: String,
        span: Span,
        use_span: Span,
        std_span: Span,
        module_span: Span,
    ) -> Self {
        Self { module, span, use_span, std_span, module_span }
    }
}

/// Configuration for spawning an Anvil instance
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnvilSetupConfig {
    /// Optional RPC URL to fork from
    pub fork_url: Option<String>,
    /// Whether to route Anvil RPC through toxiproxy/tokio proxy
    pub use_proxy: bool,
    /// Source location of the spawnAnvil() call
    pub span: Span,
}

impl AnvilSetupConfig {
    pub fn new(fork_url: Option<String>, use_proxy: bool, span: Span) -> Self {
        Self { fork_url, use_proxy, span }
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
    /// Speedup chart showing relative performance vs baseline
    SpeedupChart,
    /// Data table rendered as SVG
    Table,
    /// Line chart for trend visualization across related benchmarks
    LineChart,
    /// Bar chart for grouped comparisons across related benchmarks
    BarChart,
}

impl ChartType {
    pub fn from_function_name(name: &str) -> Option<Self> {
        match name {
            "drawSpeedupChart" => Some(ChartType::SpeedupChart),
            "drawTable" => Some(ChartType::Table),
            "drawLineChart" => Some(ChartType::LineChart),
            "drawBarChart" => Some(ChartType::BarChart),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ChartType::SpeedupChart => "speedup",
            ChartType::Table => "table",
            ChartType::LineChart => "line",
            ChartType::BarChart => "bar",
        }
    }

    pub fn default_filename(&self) -> &'static str {
        match self {
            ChartType::SpeedupChart => "speedup-chart.svg",
            ChartType::Table => "table.svg",
            ChartType::LineChart => "line-chart.svg",
            ChartType::BarChart => "bar-chart.svg",
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
    /// Output filename (without path)
    pub output_file: Option<String>,
    /// Source location
    pub span: Span,

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
    /// Number of charts per row for combined speedup charts
    pub row_count: Option<u32>,

    // === NEW PARAMETERS ===

    // Dimensions
    /// Chart height in pixels
    pub height: Option<i32>,

    /// Baseline benchmark name for percentage scale (the benchmark that equals 100%)
    pub baseline_benchmark: Option<String>,

    // Theme
    /// Color theme: "dark" (default) or "light"
    pub theme: Option<String>,
    /// Show standard deviation overlays
    pub show_std_dev: bool,
    /// Show confidence-interval/error bars
    pub show_error_bars: bool,
    /// Show regression trendline
    pub show_regression: bool,
    /// Regression model type or "auto"
    pub regression_model: String,
    /// Y-axis scale type: "linear" (default), "log10", "symlog", "split"
    pub y_scale: String,

    /// Order of parameters as they appeared in source (for formatting)
    #[serde(default)]
    pub param_order: Vec<String>,
}

impl ChartDirective {
    pub fn new(chart_type: ChartType, span: Span) -> Self {
        Self {
            chart_type,
            title: None,
            description: None,
            output_file: None,
            span,
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
            row_count: None,
            // Dimensions
            height: None,
            baseline_benchmark: None,
            // Theme
            theme: None,
            show_std_dev: true,
            show_error_bars: true,
            show_regression: true,
            regression_model: "auto".to_string(),
            y_scale: "linear".to_string(),
            // Parameter order tracking
            param_order: Vec::new(),
        }
    }

    /// Get the output filename, using default if not specified
    pub fn get_output_file(&self) -> String {
        self.output_file.clone().unwrap_or_else(|| self.chart_type.default_filename().to_string())
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

    pub fn with_global_setup(
        use_stds: Vec<UseStd>,
        global_setup: Option<GlobalSetup>,
        suites: Vec<Suite>,
    ) -> Self {
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
    /// Warmup iterations (legacy: also set by "warmup")
    pub warmup_iterations: Option<u64>,
    /// Warmup duration in ms (takes precedence over warmup_iterations when both set)
    pub warmup_time_ms: Option<u64>,

    // Phase 4: Suite configuration
    /// Suite-level timeout in milliseconds
    pub timeout: Option<u64>,
    /// Required language implementations for all benchmarks
    pub requires: Vec<Lang>,
    /// Execution order for benchmarks
    pub order: Option<ExecutionOrder>,
    /// Baseline language for comparison ratios
    pub baseline: Option<Lang>,
    /// Suite category (required by semantic validation)
    pub suite_type: Option<SuiteType>,
    /// Suite run mode (required by semantic validation)
    pub run_mode: Option<RunMode>,
    /// Whether benchmarks in this suite operate on the same dataset
    pub same_dataset: Option<bool>,

    // Benchmark accuracy settings
    /// Legacy benchmark execution mode (deprecated; rejected by semantic validation)
    /// Benchmark execution mode (auto-calibration vs fixed iterations)
    pub mode: Option<BenchMode>,
    /// Target time for auto-calibration in milliseconds (e.g., "3s" = 3000)
    pub target_time_ms: Option<u64>,
    /// Enable sink/black-box pattern to prevent dead code elimination (default: true)
    pub sink: bool,

    // Statistical analysis settings
    /// Enable IQR-based outlier detection (default: true)
    pub outlier_detection: bool,
    /// Coefficient of variation threshold percentage for stability check (default: 5.0)
    pub cv_threshold: Option<f64>,
    /// Number of times to run each benchmark for statistical consistency (default: 1)
    pub count: Option<u64>,
    /// Fairness mode for execution and comparison (default: strict)
    pub fairness_mode: Option<FairnessMode>,
    /// Optional deterministic seed for fairness randomization
    pub fairness_seed: Option<u64>,

    // Observability settings (Phase 2B)
    /// Async sampling policy (default: timeBudgeted)
    pub async_sampling_policy: Option<AsyncSamplingPolicy>,
    /// Async warmup cap override
    pub async_warmup_cap: Option<u64>,
    /// Async sample cap override
    pub async_sample_cap: Option<u64>,

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
            warmup_iterations: None,
            warmup_time_ms: None,
            timeout: None,
            requires: Vec::new(),
            order: None,
            baseline: None,
            suite_type: None,
            run_mode: None,
            same_dataset: None,
            mode: None,
            target_time_ms: None,
            sink: true,              // Enabled by default to prevent DCE
            outlier_detection: true, // Enabled by default for statistical accuracy
            cv_threshold: None,      // Uses default (5.0%) when None
            count: None,             // Uses default (1) when None - single run
            fairness_mode: None,     // Uses default (strict) when None
            fairness_seed: None,
            async_sampling_policy: None, // Uses default (timeBudgeted) when None
            async_warmup_cap: None,
            async_sample_cap: None,
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
    /// Generic fixture data source (inline string)
    pub data: Option<String>,
    /// Generic fixture data source file reference
    pub data_file: Option<String>,
    /// Data encoding (hex/raw/utf8/base64)
    pub encoding: Option<String>,
    /// Structured data format (json/csv)
    pub format: Option<String>,
    /// Optional selector for structured data extraction
    pub selector: Option<String>,
    /// Compatibility: inline hex fixture data
    pub hex_data: Option<String>,
    /// Compatibility: file-based hex fixture data
    pub hex_file: Option<String>,
    /// Per-language implementations (alternative to hex)
    pub implementations: HashMap<Lang, CodeBlock>,
    /// Order of language implementations as defined in source
    pub impl_order: Vec<Lang>,

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
            data: None,
            data_file: None,
            encoding: None,
            format: None,
            selector: None,
            hex_data: None,
            hex_file: None,
            implementations: HashMap::new(),
            impl_order: Vec::new(),
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
    /// Benchmark kind (bench vs benchAsync)
    pub kind: BenchmarkKind,
    /// Source location
    pub span: Span,
    /// Optional description
    pub description: Option<String>,
    /// Override iterations for this specific benchmark
    pub iterations: Option<u64>,

    // Phase 2: Benchmark configuration
    /// Override warmup iterations for this benchmark (legacy: also set by "warmup")
    pub warmup_iterations: Option<u64>,
    /// Override warmup duration in ms (takes precedence over warmup_iterations)
    pub warmup_time_ms: Option<u64>,
    /// Timeout in milliseconds for this benchmark
    pub timeout: Option<u64>,
    /// Tags for filtering/grouping
    pub tags: Vec<String>,
    /// Per-language skip conditions
    pub skip: HashMap<Lang, CodeBlock>,
    /// Per-language result validation expressions
    pub validate: HashMap<Lang, CodeBlock>,

    // Benchmark accuracy settings (overrides suite-level)
    /// Legacy benchmark execution mode override (deprecated; rejected by semantic validation)
    /// Override benchmark execution mode
    pub mode: Option<BenchMode>,
    /// Override target time for auto-calibration
    pub target_time_ms: Option<u64>,
    /// Override sink/black-box setting (None = inherit from suite)
    pub sink: Option<bool>,
    /// Override outlier detection setting (None = inherit from suite)
    pub outlier_detection: Option<bool>,
    /// Override CV threshold (None = inherit from suite)
    pub cv_threshold: Option<f64>,
    /// Override count setting (None = inherit from suite)
    pub count: Option<u64>,

    // Phase 3: Lifecycle hooks
    /// Pre-benchmark hook (runs once before iterations)
    pub before: HashMap<Lang, CodeBlock>,
    /// Post-benchmark hook (runs once after iterations)
    pub after: HashMap<Lang, CodeBlock>,
    /// Per-iteration hook (runs before each iteration, outside timing)
    pub each: HashMap<Lang, CodeBlock>,
    /// Style used for lifecycle hooks in source (for formatting)
    pub hook_style: HookStyle,

    /// Per-language implementations
    pub implementations: HashMap<Lang, CodeBlock>,
    /// Order of language implementations as defined in source
    pub impl_order: Vec<Lang>,
}

impl Benchmark {
    pub fn new(name: String, kind: BenchmarkKind, span: Span) -> Self {
        Self {
            name,
            kind,
            span,
            description: None,
            iterations: None,
            warmup_iterations: None,
            warmup_time_ms: None,
            timeout: None,
            tags: Vec::new(),
            skip: HashMap::new(),
            validate: HashMap::new(),
            mode: None,
            target_time_ms: None,
            sink: None,
            outlier_detection: None,
            cv_threshold: None,
            count: None,
            before: HashMap::new(),
            after: HashMap::new(),
            each: HashMap::new(),
            hook_style: HookStyle::default(),
            implementations: HashMap::new(),
            impl_order: Vec::new(),
        }
    }

    /// Check if this benchmark has any lifecycle hooks for a language
    pub fn has_hooks(&self, lang: Lang) -> bool {
        self.before.contains_key(&lang) ||
            self.after.contains_key(&lang) ||
            self.each.contains_key(&lang)
    }
}
