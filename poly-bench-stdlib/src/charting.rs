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
//!     after {
//!         charting.drawSpeedupChart(
//!             title: "TypeScript vs Go Speedup",
//!             baseline: "go"
//!         )
//!     }
//! }
//! ```
//!
//! ## Available Functions
//!
//! - `drawSpeedupChart` - Speedup chart showing relative performance vs baseline
//! - `drawTable` - SVG data table with conditional formatting

use crate::{StdlibSymbol, StdlibSymbolKind};

/// Symbols exported by std::charting
pub static CHARTING_SYMBOLS: &[StdlibSymbol] = &[
    StdlibSymbol {
        name: "drawSpeedupChart",
        kind: StdlibSymbolKind::Function,
        description: "Draw a speedup chart showing relative performance vs baseline",
        documentation: "**charting.drawSpeedupChart** `(...params)`\n\n\
            Generate a speedup chart showing relative performance compared to a baseline language.\n\n\
            Shows horizontal bars indicating how much faster/slower each language is vs the baseline.\n\n\
            **Basic Parameters:**\n\
            - `title` - Chart title (default: \"Speedup vs Baseline\")\n\
            - `description` - Subtitle/description text\n\
            - `baseline` - Baseline language: \"go\", \"ts\", or \"rust\" (default: \"go\")\n\
            - `output` - Output filename (default: \"speedup-chart.svg\")\n\n\
            **Dimensions:**\n\
            - `width` - Chart width in pixels\n\
            - `height` - Chart height in pixels\n\n\
            **Filtering:**\n\
            - `minSpeedup` - Only include benchmarks with speedup >= N\n\
            - `filterWinner` - \"go\", \"ts\", or \"all\"\n\
            - `includeBenchmarks` - Array of names to include\n\
            - `excludeBenchmarks` - Array of names to exclude\n\
            - `limit` - Max benchmarks to display\n\n\
            **Sorting:**\n\
            - `sortBy` - \"speedup\", \"name\", \"time\", \"ops\", or \"natural\"\n\
            - `sortOrder` - \"asc\" or \"desc\"\n\n\
            **Grid:**\n\
            - `showGrid` - Toggle grid lines (default: true)\n\
            - `gridOpacity` - Grid line opacity (default: 0.3)\n\n\
            **Typography:**\n\
            - `titleFontSize` - Title font size (default: 16)\n\
            - `subtitleFontSize` - Subtitle font size (default: 11)\n\
            - `axisLabelFontSize` - Axis label font size (default: 11)\n\
            - `tickLabelFontSize` - Tick label font size (default: 10)\n\n\
            **Legend:**\n\
            - `legendPosition` - \"top-left\", \"top-right\", \"bottom-left\", \"bottom-right\", \"hidden\"\n\n\
            **Data Display:**\n\
            - `precision` - Decimal places (default: 2)\n\
            - `timeUnit` - \"auto\", \"ns\", \"us\", \"ms\", \"s\"\n\n\
            **Theme:**\n\
            - `theme` - Color theme: \"dark\" (default) or \"light\"\n\n\
            **Example:**\n\
            ```\nafter {\n    charting.drawSpeedupChart(\n        title: \"TypeScript vs Go Speedup\",\n        baseline: \"go\",\n        sortBy: \"speedup\",\n        sortOrder: \"desc\",\n        theme: \"light\"\n    )\n}\n```",
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
            - `description` - Subtitle/description text\n\
            - `output` - Output filename (default: \"table.svg\")\n\n\
            **Dimensions:**\n\
            - `width` - Table width in pixels\n\
            - `height` - Table height in pixels\n\n\
            **Display Toggles:**\n\
            - `showStats` - Show ops/sec column (default: true)\n\
            - `showConfig` - Show iterations/warmup/timeout in footer\n\
            - `showWinCounts` - Show win counts in legend\n\
            - `showGeoMean` - Show geometric mean speedup\n\
            - `compact` - Minimal mode (default: false)\n\n\
            **Filtering:**\n\
            - `minSpeedup` - Only include benchmarks with speedup >= N\n\
            - `filterWinner` - \"go\", \"ts\", or \"all\"\n\
            - `includeBenchmarks` - Array of names to include\n\
            - `excludeBenchmarks` - Array of names to exclude\n\
            - `limit` - Max benchmarks to display\n\n\
            **Sorting:**\n\
            - `sortBy` - \"speedup\", \"name\", \"time\", \"ops\", or \"natural\"\n\
            - `sortOrder` - \"asc\" or \"desc\"\n\n\
            **Typography:**\n\
            - `titleFontSize` - Title font size (default: 16)\n\
            - `subtitleFontSize` - Subtitle font size (default: 11)\n\n\
            **Data Display:**\n\
            - `precision` - Decimal places (default: 2)\n\
            - `timeUnit` - \"auto\", \"ns\", \"us\", \"ms\", \"s\"\n\n\
            **Example:**\n\
            ```\nafter {\n    charting.drawTable(\n        title: \"Detailed Results\",\n        sortBy: \"name\",\n        precision: 3\n    )\n}\n```",
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
        assert_eq!(symbols.len(), 2);

        let names: Vec<_> = symbols.iter().map(|s| s.name).collect();
        assert!(names.contains(&"drawSpeedupChart"));
        assert!(names.contains(&"drawTable"));
    }

    #[test]
    fn test_all_symbols_are_functions() {
        for symbol in get_symbols() {
            assert_eq!(symbol.kind, StdlibSymbolKind::Function);
        }
    }
}
