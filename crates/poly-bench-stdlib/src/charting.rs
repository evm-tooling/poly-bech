//! Charting standard library module
//!
//! Provides DSL-native chart generation functions that execute after benchmarks complete.
//! Charts are generated from benchmark results and saved as SVG files.
//!
//! ## Usage
//!
//! Import with `use std::charting` and call chart functions in suite `after` blocks:
//!
//! ```text
//! use std::charting
//!
//! suite myBenchmarks {
//!     // ... benchmarks ...
//!     
//!     after {
//!         charting.drawBarChart(
//!             title: "Performance Comparison",
//!             xlabel: "Benchmark",
//!             ylabel: "Time (ns)"
//!         )
//!     }
//! }
//! ```
//!
//! ## Available Functions
//!
//! - `drawBarChart` - Horizontal bar chart comparing benchmark times
//! - `drawPieChart` - Pie chart showing relative time distribution
//! - `drawLineChart` - Line chart for trend visualization

use crate::{StdlibSymbol, StdlibSymbolKind};

/// Symbols exported by std::charting
pub static CHARTING_SYMBOLS: &[StdlibSymbol] = &[
    StdlibSymbol {
        name: "drawBarChart",
        kind: StdlibSymbolKind::Function,
        description: "Draw a bar chart of benchmark results",
        documentation: "**charting.drawBarChart** `(title?: string, description?: string, xlabel?: string, ylabel?: string, output?: string)`\n\n\
            Generate a horizontal bar chart comparing benchmark execution times.\n\n\
            The chart shows each benchmark as a bar, with length proportional to execution time.\n\
            Go and TypeScript results are shown with different colors.\n\n\
            **Parameters:**\n\
            - `title` - Chart title (default: \"Benchmark Results\")\n\
            - `description` - Subtitle/description text\n\
            - `xlabel` - X-axis label (default: \"Time\")\n\
            - `ylabel` - Y-axis label (default: \"Benchmark\")\n\
            - `output` - Output filename (default: \"bar-chart.svg\")\n\n\
            **Example:**\n\
            ```\nafter {\n    charting.drawBarChart(\n        title: \"Hash Function Performance\",\n        xlabel: \"Time (ns/op)\"\n    )\n}\n```",
    },
    StdlibSymbol {
        name: "drawPieChart",
        kind: StdlibSymbolKind::Function,
        description: "Draw a pie chart of benchmark time distribution",
        documentation: "**charting.drawPieChart** `(title?: string, description?: string, output?: string)`\n\n\
            Generate a pie chart showing the relative time distribution across benchmarks.\n\n\
            Each slice represents a benchmark's proportion of total execution time.\n\n\
            **Parameters:**\n\
            - `title` - Chart title (default: \"Time Distribution\")\n\
            - `description` - Subtitle/description text\n\
            - `output` - Output filename (default: \"pie-chart.svg\")\n\n\
            **Example:**\n\
            ```\nafter {\n    charting.drawPieChart(\n        title: \"Execution Time Breakdown\"\n    )\n}\n```",
    },
    StdlibSymbol {
        name: "drawLineChart",
        kind: StdlibSymbolKind::Function,
        description: "Draw a line chart for benchmark trends",
        documentation: "**charting.drawLineChart** `(title?: string, description?: string, xlabel?: string, ylabel?: string, output?: string)`\n\n\
            Generate a line chart for visualizing benchmark trends.\n\n\
            Useful for comparing performance across multiple benchmarks or showing speedup ratios.\n\n\
            **Parameters:**\n\
            - `title` - Chart title (default: \"Performance Trend\")\n\
            - `description` - Subtitle/description text\n\
            - `xlabel` - X-axis label (default: \"Benchmark\")\n\
            - `ylabel` - Y-axis label (default: \"Time\")\n\
            - `output` - Output filename (default: \"line-chart.svg\")\n\n\
            **Example:**\n\
            ```\nafter {\n    charting.drawLineChart(\n        title: \"Speedup Comparison\",\n        ylabel: \"Speedup (x)\"\n    )\n}\n```",
    },
];

/// Get all symbols exported by the charting module
pub fn get_symbols() -> &'static [StdlibSymbol] {
    CHARTING_SYMBOLS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charting_symbols() {
        let symbols = get_symbols();
        assert_eq!(symbols.len(), 3);
        
        let names: Vec<_> = symbols.iter().map(|s| s.name).collect();
        assert!(names.contains(&"drawBarChart"));
        assert!(names.contains(&"drawPieChart"));
        assert!(names.contains(&"drawLineChart"));
    }

    #[test]
    fn test_all_symbols_are_functions() {
        for symbol in get_symbols() {
            assert_eq!(symbol.kind, StdlibSymbolKind::Function);
        }
    }
}
