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
//! - `drawBarChart` - Vertical grouped bar chart comparing benchmark times
//! - `drawLineChart` - Line chart for trend visualization
//! - `drawPieChart` - Pie chart showing relative time distribution
//! - `drawSpeedupChart` - Speedup chart showing relative performance vs baseline
//! - `drawScalingChart` - Scaling efficiency chart
//! - `drawTable` - SVG data table with conditional formatting
//!
//! ## Common Parameters (all charts)
//!
//! ### Basic
//! - `title`, `description`, `output`, `width`, `height`
//!
//! ### Filtering
//! - `minSpeedup`, `filterWinner`, `includeBenchmarks`, `excludeBenchmarks`, `limit`
//!
//! ### Sorting
//! - `sortBy`: "speedup", "name", "time", "ops", "natural"
//! - `sortOrder`: "asc", "desc"
//!
//! ### Data Display
//! - `precision`, `timeUnit`
//!
//! Note: Each chart type has specific parameters. See individual function docs for details.

use crate::{StdlibSymbol, StdlibSymbolKind};

/// Symbols exported by std::charting
pub static CHARTING_SYMBOLS: &[StdlibSymbol] = &[
    StdlibSymbol {
        name: "drawBarChart",
        kind: StdlibSymbolKind::Function,
        description: "Draw a bar chart of benchmark results",
        documentation: "**charting.drawBarChart** `(...params)`\n\n\
            Generate a vertical grouped bar chart comparing benchmark execution times.\n\n\
            Each benchmark group shows bars for Go, TypeScript, and Rust side-by-side.\n\n\
            **Basic Parameters:**\n\
            - `title` - Chart title (default: \"Benchmark Results\")\n\
            - `description` - Subtitle/description text\n\
            - `xlabel`, `ylabel` - Axis labels\n\
            - `output` - Output filename (default: \"bar-chart.svg\")\n\n\
            **Dimensions:**\n\
            - `width` - Chart width in pixels (default: dynamic based on bar count)\n\
            - `height` - Chart height in pixels (default: 445)\n\n\
            **Display Toggles:** (default: true unless noted)\n\
            - `showStats` - Show ops/sec and time per op\n\
            - `showConfig` - Show iterations/warmup/timeout in footer\n\
            - `showWinCounts` - Show win counts in legend\n\
            - `showGeoMean` - Show geometric mean speedup\n\
            - `showDistribution` - Show p50/p99 percentiles\n\
            - `showMemory` - Show memory stats (default: false)\n\
            - `showTotalTime` - Show total execution time (default: false)\n\
            - `compact` - Minimal mode without extra info (default: false)\n\n\
            **Filtering:**\n\
            - `minSpeedup` - Only include benchmarks with speedup >= N\n\
            - `filterWinner` - \"go\", \"ts\", or \"all\"\n\
            - `includeBenchmarks` - Array of names to include\n\
            - `excludeBenchmarks` - Array of names to exclude\n\
            - `limit` - Max benchmarks to display\n\n\
            **Sorting:**\n\
            - `sortBy` - \"speedup\", \"name\", \"time\", \"ops\", or \"natural\" (default)\n\
            - `sortOrder` - \"asc\" or \"desc\"\n\n\
            **Bar Layout:**\n\
            - `barWidth` - Width of individual bars in pixels (default: 20)\n\
            - `barGroupGap` - Gap between benchmark groups in pixels (default: 20)\n\
            - `barWithinGroupGap` - Gap between bars within a group (default: 2)\n\n\
            **Axis Styling:**\n\
            - `axisThickness` - Stroke width for axes (default: 1.5)\n\
            - `yAxisMin` - Minimum Y-axis value\n\
            - `yAxisMax` - Maximum Y-axis value\n\
            - `yScale` - Y-axis scale: \"linear\" (default), \"log\", \"symlog\", \"percent\"\n\
            - `baselineBenchmark` - Benchmark name for 100% in percent scale\n\
            - `symlogThreshold` - Threshold for symlog scale linear region\n\n\
            **Grid:**\n\
            - `showGrid` - Toggle grid lines (default: true)\n\
            - `gridOpacity` - Grid line opacity 0.0-1.0 (default: 0.4)\n\
            - `showMinorGrid` - Show minor grid lines (default: false)\n\
            - `minorGridOpacity` - Minor grid opacity (default: 0.4)\n\
            - `showVerticalGrid` - Show vertical grid lines (default: false)\n\n\
            **Typography:**\n\
            - `titleFontSize` - Title font size (default: 16)\n\
            - `subtitleFontSize` - Subtitle font size (default: 11)\n\
            - `axisLabelFontSize` - Axis label font size (default: 11)\n\
            - `tickLabelFontSize` - Tick label font size (default: 10)\n\n\
            **Legend:**\n\
            - `legendPosition` - \"top-left\", \"top-right\", \"bottom-left\", \"bottom-right\", \"hidden\"\n\n\
            **Error Bars:**\n\
            - `showErrorBars` - Toggle error bars (default: false)\n\
            - `errorBarOpacity` - Error bar opacity (default: 0.6)\n\
            - `errorBarThickness` - Error bar stroke width (default: 1.5)\n\
            - `ciLevel` - Confidence interval: 90, 95, or 99 (default: 95)\n\n\
            **Regression:**\n\
            - `showRegression` - Toggle regression line (default: false)\n\
            - `regressionStyle` - \"solid\", \"dashed\", or \"dotted\" (default: \"dashed\")\n\
            - `regressionModel` - \"auto\", \"constant\", \"log\", \"linear\", \"nlogn\", \"quadratic\", \"cubic\"\n\
            - `showRegressionLabel` - Show complexity label e.g. \"O(n log n)\" (default: true)\n\
            - `showRSquared` - Show R² value (default: false)\n\
            - `showEquation` - Show regression equation (default: false)\n\
            - `showRegressionBand` - Show confidence band around regression (default: false)\n\
            - `regressionBandOpacity` - Regression band opacity (default: 0.15)\n\n\
            **Tick Formatting:**\n\
            - `roundTicks` - Round tick labels to whole numbers (default: false)\n\n\
            **Data Display:**\n\
            - `precision` - Decimal places (default: 2)\n\
            - `timeUnit` - \"auto\", \"ns\", \"us\", \"ms\", \"s\"\n\n\
            **Example:**\n\
            ```\nafter {\n    charting.drawBarChart(\n        title: \"Hash Function Performance\",\n        sortBy: \"speedup\",\n        sortOrder: \"desc\",\n        showErrorBars: true,\n        showRegression: true\n    )\n}\n```",
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
            **Dimensions:**\n\
            - `width` - Chart width in pixels\n\
            - `height` - Chart height in pixels\n\n\
            **Display Toggles:**\n\
            - `showStats` - Show timing info in legend (default: true)\n\
            - `showTotalTime` - Show total time below chart (default: false)\n\
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
            **Legend:**\n\
            - `legendPosition` - \"top-left\", \"top-right\", \"bottom-left\", \"bottom-right\", \"hidden\"\n\n\
            **Data Display:**\n\
            - `precision` - Decimal places (default: 2)\n\
            - `timeUnit` - \"auto\", \"ns\", \"us\", \"ms\", \"s\"\n\n\
            **Example:**\n\
            ```\nafter {\n    charting.drawPieChart(\n        title: \"Execution Time Breakdown\",\n        showStats: true,\n        legendPosition: \"bottom-right\"\n    )\n}\n```",
    },
    StdlibSymbol {
        name: "drawLineChart",
        kind: StdlibSymbolKind::Function,
        description: "Draw a line chart for benchmark trends",
        documentation: "**charting.drawLineChart** `(...params)`\n\n\
            Generate a line chart for visualizing benchmark trends.\n\n\
            Shows Go, TypeScript, and Rust performance lines with data points.\n\n\
            **Basic Parameters:**\n\
            - `title` - Chart title (default: \"Performance Trend\")\n\
            - `description` - Subtitle/description text\n\
            - `xlabel`, `ylabel` - Axis labels\n\
            - `output` - Output filename (default: \"line-chart.svg\")\n\n\
            **Dimensions:**\n\
            - `width` - Chart width in pixels (default: 620)\n\
            - `height` - Chart height in pixels (default: 445)\n\n\
            **Display Toggles:**\n\
            - `compact` - Minimal mode (default: false)\n\n\
            **Filtering:**\n\
            - `minSpeedup` - Only include benchmarks with speedup >= N\n\
            - `filterWinner` - \"go\", \"ts\", or \"all\"\n\
            - `includeBenchmarks` - Array of names to include\n\
            - `excludeBenchmarks` - Array of names to exclude\n\
            - `limit` - Max benchmarks to display\n\n\
            **Sorting:**\n\
            - `sortBy` - \"speedup\", \"name\", \"time\", \"ops\", or \"natural\" (default)\n\
            - `sortOrder` - \"asc\" or \"desc\"\n\n\
            **Axis Styling:**\n\
            - `axisThickness` - Stroke width for axes (default: 1.0)\n\
            - `yAxisMin` - Minimum Y-axis value\n\
            - `yAxisMax` - Maximum Y-axis value\n\
            - `yScale` - Y-axis scale: \"linear\" (default) or \"log\"\n\n\
            **Grid:**\n\
            - `showGrid` - Toggle grid lines (default: true)\n\
            - `gridOpacity` - Grid line opacity 0.0-1.0 (default: 0.4)\n\
            - `showMinorGrid` - Show minor grid lines (default: false)\n\
            - `minorGridOpacity` - Minor grid opacity (default: 0.4)\n\
            - `showVerticalGrid` - Show vertical grid lines (default: false)\n\n\
            **Typography:**\n\
            - `titleFontSize` - Title font size (default: 16)\n\
            - `subtitleFontSize` - Subtitle font size (default: 11)\n\
            - `axisLabelFontSize` - Axis label font size (default: 11)\n\
            - `tickLabelFontSize` - Tick label font size (default: 10)\n\n\
            **Legend:**\n\
            - `legendPosition` - \"top-left\", \"top-right\" (default), \"bottom-left\", \"bottom-right\", \"hidden\"\n\n\
            **Error Bars:**\n\
            - `showErrorBars` - Toggle error bars (default: false)\n\
            - `errorBarOpacity` - Error bar opacity (default: 0.4)\n\
            - `errorBarThickness` - Error bar stroke width (default: 1.5)\n\
            - `ciLevel` - Confidence interval: 90, 95, or 99 (default: 95)\n\
            - `showStdDevBand` - Show standard deviation band (default: false)\n\n\
            **Regression:**\n\
            - `showRegression` - Toggle regression line (default: false)\n\
            - `regressionStyle` - \"solid\", \"dashed\", or \"dotted\" (default: \"dashed\")\n\
            - `regressionModel` - \"auto\", \"constant\", \"log\", \"linear\", \"nlogn\", \"quadratic\", \"cubic\"\n\
            - `showRegressionLabel` - Show complexity label e.g. \"O(n log n)\" (default: true)\n\
            - `showRSquared` - Show R² value (default: false)\n\
            - `showEquation` - Show regression equation (default: false)\n\
            - `showRegressionBand` - Show confidence band around regression (default: false)\n\
            - `regressionBandOpacity` - Regression band opacity (default: 0.15)\n\n\
            **Tick Formatting:**\n\
            - `roundTicks` - Round tick labels to whole numbers (default: false)\n\n\
            **Data Display:**\n\
            - `precision` - Decimal places for Y-axis\n\
            - `timeUnit` - Time unit for Y-axis labels\n\n\
            **Example:**\n\
            ```\nafter {\n    charting.drawLineChart(\n        title: \"Scaling Performance\",\n        sortBy: \"natural\",\n        showRegression: true,\n        showRSquared: true,\n        showErrorBars: true\n    )\n}\n```",
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
            **Example:**\n\
            ```\nafter {\n    charting.drawSpeedupChart(\n        title: \"TypeScript vs Go Speedup\",\n        baseline: \"go\",\n        sortBy: \"speedup\",\n        sortOrder: \"desc\"\n    )\n}\n```",
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
            - `description` - Subtitle/description text\n\
            - `xlabel`, `ylabel` - Axis labels\n\
            - `output` - Output filename (default: \"scaling-chart.svg\")\n\n\
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
            **Axis Styling:**\n\
            - `yAxisMin` - Minimum Y-axis value\n\
            - `yAxisMax` - Maximum Y-axis value\n\
            - `yScale` - Y-axis scale: \"linear\" (default) or \"log\"\n\n\
            **Grid:**\n\
            - `showGrid` - Toggle grid lines (default: true)\n\
            - `gridOpacity` - Grid line opacity (default: 0.3)\n\
            - `showMinorGrid` - Show minor grid lines (default: false)\n\n\
            **Typography:**\n\
            - `titleFontSize` - Title font size (default: 16)\n\
            - `subtitleFontSize` - Subtitle font size (default: 11)\n\
            - `axisLabelFontSize` - Axis label font size (default: 11)\n\
            - `tickLabelFontSize` - Tick label font size (default: 10)\n\n\
            **Legend:**\n\
            - `legendPosition` - \"top-left\", \"top-right\", \"bottom-left\", \"bottom-right\", \"hidden\"\n\n\
            **Tick Formatting:**\n\
            - `roundTicks` - Round tick labels to whole numbers (default: false)\n\n\
            **Data Display:**\n\
            - `precision` - Decimal places (default: 2)\n\
            - `timeUnit` - \"auto\", \"ns\", \"us\", \"ms\", \"s\"\n\n\
            **Example:**\n\
            ```\nafter {\n    charting.drawScalingChart(\n        title: \"Algorithm Scaling Analysis\",\n        yScale: \"log\",\n        showGrid: true\n    )\n}\n```",
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
