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

/// Top-level file containing one or more suites
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub suites: Vec<Suite>,
}

impl File {
    pub fn new(suites: Vec<Suite>) -> Self {
        Self { suites }
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
    /// Per-language setup code blocks
    pub setups: HashMap<Lang, CodeBlock>,
    /// Named fixtures
    pub fixtures: Vec<Fixture>,
    /// Benchmark definitions
    pub benchmarks: Vec<Benchmark>,
}

impl Suite {
    pub fn new(name: String, span: Span) -> Self {
        Self {
            name,
            span,
            description: None,
            iterations: None,
            warmup: None,
            setups: HashMap::new(),
            fixtures: Vec::new(),
            benchmarks: Vec::new(),
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
        }
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
            implementations: HashMap::new(),
        }
    }
}
