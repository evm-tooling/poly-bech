//! LSP position type - avoids requiring tower-lsp in all consumers

/// Position in a document (line, character) - compatible with LSP
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LspPosition {
    pub line: u32,
    pub character: u32,
}

impl LspPosition {
    pub fn new(line: u32, character: u32) -> Self {
        Self { line, character }
    }
}
