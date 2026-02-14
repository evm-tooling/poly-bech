//! Token types for the poly-bench DSL lexer

use crate::ast::Span;

/// Token types for the DSL
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Suite,
    Bench,
    Setup,
    Fixture,
    Hex,
    Description,
    Iterations,
    Warmup,

    // Phase 1: Structured setup keywords
    Declare, // declare
    Init,    // init
    Helpers, // helpers
    Import,  // import

    // Phase 2: Benchmark configuration keywords
    Timeout,  // timeout
    Tags,     // tags
    Skip,     // skip
    Validate, // validate

    // Phase 3: Lifecycle hook keywords
    Before, // before
    After,  // after
    Each,   // each

    // Phase 4: Suite configuration keywords
    Requires, // requires
    Order,    // order
    Compare,  // compare
    Baseline, // baseline

    // Benchmark accuracy keywords
    Mode,          // mode (auto/fixed)
    Sink,          // sink (true/false)
    TargetTime,    // targetTime
    MinIterations, // minIterations
    MaxIterations, // maxIterations

    // Statistical analysis keywords
    OutlierDetection, // outlierDetection (true/false)
    CvThreshold,      // cvThreshold (percentage)
    Count,            // count (number) - run benchmark N times for statistical consistency

    // Observability keywords (Phase 2B)
    Memory,      // memory (true/false) - enable memory profiling
    Concurrency, // concurrency (number) - concurrent goroutines/workers

    // Phase 5: Fixture keywords
    Shape, // shape

    // Phase 8: Async keyword
    Async, // async

    // Standard library keywords
    Use, // use

    // Global setup keywords
    GlobalSetup, // globalSetup

    // Language keywords
    Go,
    Ts,
    TypeScript,
    Rust,
    Python,

    // Boolean literals
    True,  // true
    False, // false

    // Literals
    Identifier(String),
    String(String),
    Number(u64),
    Float(f64),    // Floating point number
    Duration(u64), // Duration in milliseconds (30s, 500ms, 1m)
    HexLiteral(String),

    // Punctuation
    LBrace,      // {
    RBrace,      // }
    LParen,      // (
    RParen,      // )
    LBracket,    // [
    RBracket,    // ]
    Colon,       // :
    DoubleColon, // ::
    Dot,         // .
    Comma,       // ,
    At,          // @

    // Special
    FileRef, // @file
    Comment(String),

    // End of file
    Eof,
}

impl TokenKind {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            TokenKind::Suite
                | TokenKind::Bench
                | TokenKind::Setup
                | TokenKind::Fixture
                | TokenKind::Hex
                | TokenKind::Description
                | TokenKind::Iterations
                | TokenKind::Warmup
                | TokenKind::Declare
                | TokenKind::Init
                | TokenKind::Helpers
                | TokenKind::Import
                | TokenKind::Timeout
                | TokenKind::Tags
                | TokenKind::Skip
                | TokenKind::Validate
                | TokenKind::Before
                | TokenKind::After
                | TokenKind::Each
                | TokenKind::Requires
                | TokenKind::Order
                | TokenKind::Compare
                | TokenKind::Baseline
                | TokenKind::Mode
                | TokenKind::Sink
                | TokenKind::TargetTime
                | TokenKind::MinIterations
                | TokenKind::MaxIterations
                | TokenKind::OutlierDetection
                | TokenKind::CvThreshold
                | TokenKind::Count
                | TokenKind::Memory
                | TokenKind::Concurrency
                | TokenKind::Shape
                | TokenKind::Async
                | TokenKind::Use
                | TokenKind::GlobalSetup
        )
    }

    pub fn is_lang(&self) -> bool {
        matches!(
            self,
            TokenKind::Go
                | TokenKind::Ts
                | TokenKind::TypeScript
                | TokenKind::Rust
                | TokenKind::Python
        )
    }

    /// Check if this is a setup section keyword
    pub fn is_setup_section(&self) -> bool {
        matches!(
            self,
            TokenKind::Import
                | TokenKind::Declare
                | TokenKind::Init
                | TokenKind::Helpers
                | TokenKind::Async
        )
    }

    /// Check if this is a benchmark hook keyword
    pub fn is_benchmark_hook(&self) -> bool {
        matches!(self, TokenKind::Before | TokenKind::After | TokenKind::Each)
    }
}

/// A token with its span information
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub lexeme: String,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span, lexeme: String) -> Self {
        Self { kind, span, lexeme }
    }
}

/// Map string to keyword token kind
pub fn keyword_from_str(s: &str) -> Option<TokenKind> {
    match s {
        // Core keywords
        "suite" => Some(TokenKind::Suite),
        "bench" => Some(TokenKind::Bench),
        "setup" => Some(TokenKind::Setup),
        "fixture" => Some(TokenKind::Fixture),
        "hex" => Some(TokenKind::Hex),
        "description" => Some(TokenKind::Description),
        "iterations" => Some(TokenKind::Iterations),
        "warmup" => Some(TokenKind::Warmup),

        // Phase 1: Structured setup keywords
        "declare" => Some(TokenKind::Declare),
        "init" => Some(TokenKind::Init),
        "helpers" => Some(TokenKind::Helpers),
        "import" => Some(TokenKind::Import),

        // Phase 2: Benchmark configuration keywords
        "timeout" => Some(TokenKind::Timeout),
        "tags" => Some(TokenKind::Tags),
        "skip" => Some(TokenKind::Skip),
        "validate" => Some(TokenKind::Validate),

        // Phase 3: Lifecycle hook keywords
        "before" => Some(TokenKind::Before),
        "after" => Some(TokenKind::After),
        "each" => Some(TokenKind::Each),

        // Phase 4: Suite configuration keywords
        "requires" => Some(TokenKind::Requires),
        "order" => Some(TokenKind::Order),
        "compare" => Some(TokenKind::Compare),
        "baseline" => Some(TokenKind::Baseline),

        // Benchmark accuracy keywords
        "mode" => Some(TokenKind::Mode),
        "sink" => Some(TokenKind::Sink),
        "targetTime" => Some(TokenKind::TargetTime),
        "minIterations" => Some(TokenKind::MinIterations),
        "maxIterations" => Some(TokenKind::MaxIterations),

        // Statistical analysis keywords
        "outlierDetection" => Some(TokenKind::OutlierDetection),
        "cvThreshold" => Some(TokenKind::CvThreshold),
        "count" => Some(TokenKind::Count),

        // Observability keywords (Phase 2B)
        "memory" => Some(TokenKind::Memory),
        "concurrency" => Some(TokenKind::Concurrency),

        // Phase 5: Fixture keywords
        "shape" => Some(TokenKind::Shape),

        // Phase 8: Async keyword
        "async" => Some(TokenKind::Async),

        // Standard library keywords
        "use" => Some(TokenKind::Use),

        // Global setup keywords
        "globalSetup" => Some(TokenKind::GlobalSetup),

        // Language keywords
        "go" => Some(TokenKind::Go),
        "ts" => Some(TokenKind::Ts),
        "typescript" => Some(TokenKind::TypeScript),
        "rust" => Some(TokenKind::Rust),
        "python" => Some(TokenKind::Python),

        // Boolean literals
        "true" => Some(TokenKind::True),
        "false" => Some(TokenKind::False),

        _ => None,
    }
}
