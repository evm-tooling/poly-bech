//! Partial AST types that preserve error information
//!
//! Unlike traditional ASTs that require valid syntax, these types
//! can represent partially parsed code with error markers.

use std::collections::HashMap;

/// A span in the source code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
}

impl Span {
    pub fn new(
        start: usize,
        end: usize,
        start_line: usize,
        start_col: usize,
        end_line: usize,
        end_col: usize,
    ) -> Self {
        Self { start, end, start_line, start_col, end_line, end_col }
    }

    pub fn from_node(node: &tree_sitter::Node) -> Self {
        Self {
            start: node.start_byte(),
            end: node.end_byte(),
            start_line: node.start_position().row,
            start_col: node.start_position().column,
            end_line: node.end_position().row,
            end_col: node.end_position().column,
        }
    }

    /// Check if this span contains a byte offset
    pub fn contains(&self, offset: usize) -> bool {
        offset >= self.start && offset < self.end
    }

    /// Check if this span contains a point
    pub fn contains_point(&self, line: usize, col: usize) -> bool {
        if line < self.start_line || line > self.end_line {
            return false;
        }
        if line == self.start_line && col < self.start_col {
            return false;
        }
        if line == self.end_line && col >= self.end_col {
            return false;
        }
        true
    }
}

impl Default for Span {
    fn default() -> Self {
        Self { start: 0, end: 0, start_line: 0, start_col: 0, end_line: 0, end_col: 0 }
    }
}

/// A node that may be valid, contain an error, or be missing
#[derive(Debug, Clone)]
pub enum Node<T> {
    /// Successfully parsed node
    Valid(T),
    /// Node with a parse error
    Error { span: Span, message: String },
    /// Expected node that was not found
    Missing { span: Span, expected: &'static str },
}

impl<T> Node<T> {
    /// Check if this node is valid
    pub fn is_valid(&self) -> bool {
        matches!(self, Node::Valid(_))
    }

    /// Check if this node has an error
    pub fn is_error(&self) -> bool {
        matches!(self, Node::Error { .. })
    }

    /// Check if this node is missing
    pub fn is_missing(&self) -> bool {
        matches!(self, Node::Missing { .. })
    }

    /// Get the inner value if valid
    pub fn as_valid(&self) -> Option<&T> {
        match self {
            Node::Valid(v) => Some(v),
            _ => None,
        }
    }

    /// Get the inner value if valid (mutable)
    pub fn as_valid_mut(&mut self) -> Option<&mut T> {
        match self {
            Node::Valid(v) => Some(v),
            _ => None,
        }
    }

    /// Map the inner value if valid
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Node<U> {
        match self {
            Node::Valid(v) => Node::Valid(f(v)),
            Node::Error { span, message } => Node::Error { span, message },
            Node::Missing { span, expected } => Node::Missing { span, expected },
        }
    }
}

/// A parse error
#[derive(Debug, Clone)]
pub struct ParseError {
    pub span: Span,
    pub message: String,
    pub context: Option<String>,
}

impl ParseError {
    pub fn new(span: Span, message: impl Into<String>) -> Self {
        Self { span, message: message.into(), context: None }
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }
}

/// Supported languages for embedded code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
            "rust" => Some(Lang::Rust),
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

/// A partial file - always populated even with errors
#[derive(Debug, Clone, Default)]
pub struct PartialFile {
    pub use_stds: Vec<Node<UseStd>>,
    pub global_setup: Option<Node<GlobalSetup>>,
    pub suites: Vec<Node<PartialSuite>>,
    pub errors: Vec<ParseError>,
}

impl PartialFile {
    /// Check if the file has any errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty() ||
            self.use_stds.iter().any(|n| !n.is_valid()) ||
            self.global_setup.as_ref().map_or(false, |n| !n.is_valid()) ||
            self.suites.iter().any(|n| !n.is_valid())
    }

    /// Get all valid suites
    pub fn valid_suites(&self) -> impl Iterator<Item = &PartialSuite> {
        self.suites.iter().filter_map(|n| n.as_valid())
    }
}

/// A use statement
#[derive(Debug, Clone)]
pub struct UseStd {
    pub module: String,
    pub span: Span,
}

/// Global setup block
#[derive(Debug, Clone)]
pub struct GlobalSetup {
    pub statements: Vec<Node<GlobalSetupStatement>>,
    pub span: Span,
}

/// A statement in global setup
#[derive(Debug, Clone)]
pub enum GlobalSetupStatement {
    AnvilSpawn { fork_url: Option<String>, span: Span },
    FunctionCall { name: String, args: Vec<(String, PropertyValue)>, span: Span },
}

/// A partial suite - always populated even with errors
#[derive(Debug, Clone)]
pub struct PartialSuite {
    pub name: String,
    pub span: Span,
    pub properties: Vec<Node<Property>>,
    pub setups: HashMap<Lang, Node<StructuredSetup>>,
    pub fixtures: Vec<Node<PartialFixture>>,
    pub benchmarks: Vec<Node<PartialBenchmark>>,
    pub after_block: Option<Node<AfterBlock>>,
    pub global_setup: Option<Node<GlobalSetup>>,
}

impl PartialSuite {
    pub fn new(name: String, span: Span) -> Self {
        Self {
            name,
            span,
            properties: Vec::new(),
            setups: HashMap::new(),
            fixtures: Vec::new(),
            benchmarks: Vec::new(),
            after_block: None,
            global_setup: None,
        }
    }
}

/// A property (key: value pair)
#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub value: PropertyValue,
    pub span: Span,
}

/// Property value types
#[derive(Debug, Clone)]
pub enum PropertyValue {
    String(String),
    Number(i64),
    Float(f64),
    Duration(i64, DurationUnit),
    Boolean(bool),
    Identifier(String),
    StringArray(Vec<String>),
}

/// Duration units
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurationUnit {
    Milliseconds,
    Seconds,
    Minutes,
}

impl DurationUnit {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "ms" => Some(DurationUnit::Milliseconds),
            "s" => Some(DurationUnit::Seconds),
            "m" => Some(DurationUnit::Minutes),
            _ => None,
        }
    }
}

/// Structured setup block for a language
#[derive(Debug, Clone)]
pub struct StructuredSetup {
    pub lang: Lang,
    pub imports: Option<CodeBlock>,
    pub declare: Option<CodeBlock>,
    pub init: Option<CodeBlock>,
    pub helpers: Option<CodeBlock>,
    pub is_async_init: bool,
    pub span: Span,
}

impl StructuredSetup {
    pub fn new(lang: Lang, span: Span) -> Self {
        Self {
            lang,
            imports: None,
            declare: None,
            init: None,
            helpers: None,
            is_async_init: false,
            span,
        }
    }
}

/// A code block containing embedded language code
#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub code: String,
    pub span: Span,
}

/// A partial fixture
#[derive(Debug, Clone)]
pub struct PartialFixture {
    pub name: String,
    pub span: Span,
    pub params: Vec<FixtureParam>,
    pub properties: Vec<Node<Property>>,
    pub hex: Option<HexData>,
    pub shape: Option<CodeBlock>,
    pub implementations: HashMap<Lang, Node<CodeBlock>>,
}

impl PartialFixture {
    pub fn new(name: String, span: Span) -> Self {
        Self {
            name,
            span,
            params: Vec::new(),
            properties: Vec::new(),
            hex: None,
            shape: None,
            implementations: HashMap::new(),
        }
    }
}

/// Fixture parameter
#[derive(Debug, Clone)]
pub struct FixtureParam {
    pub name: String,
    pub type_name: String,
    pub span: Span,
}

/// Hex data for fixtures
#[derive(Debug, Clone)]
pub enum HexData {
    Inline(String),
    File(String),
}

/// A partial benchmark
#[derive(Debug, Clone)]
pub struct PartialBenchmark {
    pub name: String,
    pub span: Span,
    pub properties: Vec<Node<Property>>,
    pub tags: Vec<String>,
    pub skip: HashMap<Lang, Node<CodeBlock>>,
    pub validate: HashMap<Lang, Node<CodeBlock>>,
    pub before: HashMap<Lang, Node<CodeBlock>>,
    pub after: HashMap<Lang, Node<CodeBlock>>,
    pub each: HashMap<Lang, Node<CodeBlock>>,
    pub implementations: HashMap<Lang, Node<CodeBlock>>,
}

impl PartialBenchmark {
    pub fn new(name: String, span: Span) -> Self {
        Self {
            name,
            span,
            properties: Vec::new(),
            tags: Vec::new(),
            skip: HashMap::new(),
            validate: HashMap::new(),
            before: HashMap::new(),
            after: HashMap::new(),
            each: HashMap::new(),
            implementations: HashMap::new(),
        }
    }
}

/// After block with charting directives
#[derive(Debug, Clone)]
pub struct AfterBlock {
    pub directives: Vec<Node<ChartDirective>>,
    pub span: Span,
}

/// A charting directive
#[derive(Debug, Clone)]
pub struct ChartDirective {
    pub function: String,
    pub params: HashMap<String, PropertyValue>,
    pub span: Span,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_is_valid() {
        let valid: Node<i32> = Node::Valid(42);
        assert!(valid.is_valid());
        assert!(!valid.is_error());
        assert!(!valid.is_missing());
    }

    #[test]
    fn test_node_is_error() {
        let error: Node<i32> =
            Node::Error { span: Span::default(), message: "test error".to_string() };
        assert!(!error.is_valid());
        assert!(error.is_error());
        assert!(!error.is_missing());
    }

    #[test]
    fn test_node_map() {
        let valid: Node<i32> = Node::Valid(42);
        let mapped = valid.map(|x| x * 2);
        assert_eq!(mapped.as_valid(), Some(&84));
    }

    #[test]
    fn test_span_contains() {
        let span = Span::new(10, 20, 1, 0, 1, 10);
        assert!(span.contains(10));
        assert!(span.contains(15));
        assert!(!span.contains(20));
        assert!(!span.contains(5));
    }

    #[test]
    fn test_lang_from_str() {
        assert_eq!(Lang::from_str("go"), Some(Lang::Go));
        assert_eq!(Lang::from_str("ts"), Some(Lang::TypeScript));
        assert_eq!(Lang::from_str("typescript"), Some(Lang::TypeScript));
        assert_eq!(Lang::from_str("rust"), Some(Lang::Rust));
        assert_eq!(Lang::from_str("python"), Some(Lang::Python));
        assert_eq!(Lang::from_str("unknown"), None);
    }
}
