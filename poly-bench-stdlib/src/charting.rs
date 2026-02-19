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
//!             sortBy: "speedup",
//!             sortOrder: "desc",
//!             limit: 10
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
//!
//! ## Common Parameters
//!
//! All chart functions support these parameter categories:
//!
//! ### Basic
//! - `title`, `description`, `xlabel`, `ylabel`, `output`
//!
//! ### Display Toggles
//! - `showStats`, `showConfig`, `showWinCounts`, `showGeoMean`
//! - `showDistribution`, `showMemory`, `showTotalTime`, `compact`
//!
//! ### Filtering
//! - `minSpeedup`, `filterWinner`, `includeBenchmarks`, `excludeBenchmarks`, `limit`
//!
//! ### Sorting
//! - `sortBy`: "speedup", "name", "time", "ops"
//! - `sortOrder`: "asc", "desc"
//!
//! ### Layout
//! - `width`, `barHeight`, `barGap`, `marginLeft`
//!
//! ### Data Display
//! - `precision`, `timeUnit`

use crate::{StdlibSymbol, StdlibSymbolKind};

/// Symbols exported by std::charting
pub static CHARTING_SYMBOLS: &[StdlibSymbol] = &[
    StdlibSymbol {
        name: "drawBarChart",
        kind: StdlibSymbolKind::Function,
        description: "Draw a bar chart of benchmark results",
        documentation: "**charting.drawBarChart** `(...params)`\n\n\
            Generate a horizontal bar chart comparing benchmark execution times.\n\n\
            The chart shows each benchmark as a bar, with length proportional to speedup.\n\
            Go and TypeScript results are shown with different colors.\n\n\
            **Basic Parameters:**\n\
            - `title` - Chart title (default: \"Benchmark Results\")\n\
            - `description` - Subtitle/description text\n\
            - `xlabel`, `ylabel` - Axis labels\n\
            - `output` - Output filename (default: \"bar-chart.svg\")\n\n\
            **Display Toggles:** (default: true unless noted)\n\
            - `showStats` - Show ops/sec and time per op\n\
            - `showConfig` - Show iterations/warmup/timeout in footer\n\
            - `showWinCounts` - Show win counts in legend\n\
            - `showGeoMean` - Show geometric mean speedup\n\
            - `showDistribution` - Show p50/p99 percentiles (default: false)\n\
            - `compact` - Minimal mode without extra info (default: false)\n\n\
            **Filtering:**\n\
            - `minSpeedup` - Only include benchmarks with speedup >= N\n\
            - `filterWinner` - \"go\", \"ts\", or \"all\"\n\
            - `includeBenchmarks` - Array of names to include\n\
            - `excludeBenchmarks` - Array of names to exclude\n\
            - `limit` - Max benchmarks to display\n\n\
            **Sorting:**\n\
            - `sortBy` - \"speedup\", \"name\", \"time\", or \"ops\"\n\
            - `sortOrder` - \"asc\" or \"desc\"\n\n\
            **Layout:**\n\
            - `width`, `barHeight`, `barGap`, `marginLeft` - Pixel values\n\n\
            **Data Display:**\n\
            - `precision` - Decimal places (default: 2)\n\
            - `timeUnit` - \"auto\", \"ns\", \"us\", \"ms\", \"s\"\n\n\
            **Example:**\n\
            ```\nafter {\n    charting.drawBarChart(\n        title: \"Hash Function Performance\",\n        sortBy: \"speedup\",\n        sortOrder: \"desc\",\n        limit: 10\n    )\n}\n```",
    },
    StdlibSymbol {
        name: "drawPieChart",
        kind: StdlibSymbolKind::Function,
        description: "Draw a pie chart of benchmark time distribution",
        documentation: "**charting.drawPieChart** `(...params)`\n\n\
            Generate a pie chart showing the relative time distribution across benchmarks.\n\n\
            Each slice represents a benchmark's proportion of total execution time.\n\n\
            **Basic Parameters:**\n\
            - `title` - Chart title (default: \"Time Distribution\")\n\
            - `description` - Subtitle/description text\n\
            - `output` - Output filename (default: \"pie-chart.svg\")\n\n\
            **Display Toggles:**\n\
            - `showStats` - Show timing info in legend (default: true)\n\
            - `showTotalTime` - Show total time below chart (default: false)\n\
            - `compact` - Minimal mode (default: false)\n\n\
            **Filtering:** Same as drawBarChart\n\n\
            **Example:**\n\
            ```\nafter {\n    charting.drawPieChart(\n        title: \"Execution Time Breakdown\",\n        showStats: true\n    )\n}\n```",
    },
    StdlibSymbol {
        name: "drawLineChart",
        kind: StdlibSymbolKind::Function,
        description: "Draw a line chart for benchmark trends",
        documentation: "**charting.drawLineChart** `(...params)`\n\n\
            Generate a line chart for visualizing benchmark trends.\n\n\
            Shows Go and TypeScript performance lines with data points.\n\n\
            **Basic Parameters:**\n\
            - `title` - Chart title (default: \"Performance Trend\")\n\
            - `description` - Subtitle/description text\n\
            - `xlabel`, `ylabel` - Axis labels\n\
            - `output` - Output filename (default: \"line-chart.svg\")\n\n\
            **Display Toggles:**\n\
            - `showStats` - Show timing tooltips on hover (default: true)\n\
            - `compact` - Minimal mode (default: false)\n\n\
            **Filtering & Sorting:** Same as drawBarChart\n\n\
            **Data Display:**\n\
            - `precision` - Decimal places for Y-axis\n\
            - `timeUnit` - Time unit for Y-axis labels\n\n\
            **Example:**\n\
            ```\nafter {\n    charting.drawLineChart(\n        title: \"Speedup Comparison\",\n        sortBy: \"name\"\n    )\n}\n```",
    },
    StdlibSymbol {
        name: "drawSpeedupChart",
        kind: StdlibSymbolKind::Function,
        description: "Draw a speedup chart showing relative performance vs baseline",
        documentation: "**charting.drawSpeedupChart** `(...params)`\n\n\
            Generate a speedup chart showing relative performance compared to a baseline language.\n\n\
            Shows horizontal bars indicating how much faster/slower each language is vs the baseline.\n\n\
            **Basic Parameters:**\n\
            - `title` - Chart title (default: \"Speedup vs Baseline\")\n\
            - `baseline` - Baseline language: \"go\", \"ts\", or \"rust\" (default: \"go\")\n\
            - `output` - Output filename (default: \"speedup-chart.svg\")\n\n\
            **Filtering & Sorting:** Same as drawBarChart\n\n\
            **Example:**\n\
            ```\nafter {\n    charting.drawSpeedupChart(\n        title: \"TypeScript vs Go Speedup\",\n        baseline: \"go\"\n    )\n}\n```",
    },
    StdlibSymbol {
        name: "drawScalingChart",
        kind: StdlibSymbolKind::Function,
        description: "Draw a scaling efficiency chart with ideal line overlay",
        documentation: "**charting.drawScalingChart** `(...params)`\n\n\
            Generate a scaling efficiency chart showing how performance scales with input size.\n\n\
            Extracts numeric values from benchmark names (e.g., n100, size1000) for the X-axis.\n\
            Shows efficiency relative to O(n) scaling - values > 1 indicate worse than linear.\n\n\
            **Basic Parameters:**\n\
            - `title` - Chart title (default: \"Scaling Efficiency\")\n\
            - `output` - Output filename (default: \"scaling-chart.svg\")\n\n\
            **Filtering & Sorting:** Same as drawBarChart\n\n\
            **Example:**\n\
            ```\nafter {\n    charting.drawScalingChart(\n        title: \"Algorithm Scaling Analysis\"\n    )\n}\n```",
    },
    StdlibSymbol {
        name: "drawTable",
        kind: StdlibSymbolKind::Function,
        description: "Draw an SVG data table with conditional formatting",
        documentation: "**charting.drawTable** `(...params)`\n\n\
            Generate an SVG table showing benchmark results with conditional formatting.\n\n\
            Displays benchmark name, timing for each language, and winner with color highlighting.\n\n\
            **Basic Parameters:**\n\
            - `title` - Table title (default: \"Benchmark Results\")\n\
            - `output` - Output filename (default: \"table.svg\")\n\n\
            **Filtering & Sorting:** Same as drawBarChart\n\n\
            **Example:**\n\
            ```\nafter {\n    charting.drawTable(\n        title: \"Detailed Results\",\n        sortBy: \"name\"\n    )\n}\n```",
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
        assert_eq!(symbols.len(), 6);

        let names: Vec<_> = symbols.iter().map(|s| s.name).collect();
        assert!(names.contains(&"drawBarChart"));
        assert!(names.contains(&"drawPieChart"));
        assert!(names.contains(&"drawLineChart"));
        assert!(names.contains(&"drawSpeedupChart"));
        assert!(names.contains(&"drawScalingChart"));
        assert!(names.contains(&"drawTable"));
    }

    #[test]
    fn test_all_symbols_are_functions() {
        for symbol in get_symbols() {
            assert_eq!(symbol.kind, StdlibSymbolKind::Function);
        }
    }
}
