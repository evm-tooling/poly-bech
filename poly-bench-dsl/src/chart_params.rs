//! Chart parameter definitions and validation
//!
//! This module defines which parameters are valid for each chart type and provides
//! validation functions to ensure only valid parameters are used.

use crate::ChartType;
use std::collections::HashSet;

/// All valid chart parameter names
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChartParam {
    // Basic parameters (all charts)
    Title,
    Description,
    Output,
    Width,
    Height,
    XLabel,

    // Display toggles
    ShowStats,
    ShowConfig,
    ShowWinCounts,
    ShowGeoMean,
    ShowDistribution,
    ShowMemory,
    ShowTotalTime,
    Compact,

    // Filtering
    MinSpeedup,
    FilterWinner,
    IncludeBenchmarks,
    ExcludeBenchmarks,
    Limit,

    // Sorting
    SortBy,
    SortOrder,

    // Bar chart specific layout
    BarWidth,
    BarGroupGap,
    BarWithinGroupGap,

    // Data display
    Precision,
    TimeUnit,

    // Axis styling
    AxisThickness,
    YAxisMin,
    YAxisMax,
    YScale,
    BaselineBenchmark,
    SymlogThreshold,

    // Grid
    ShowGrid,
    GridOpacity,
    ShowMinorGrid,
    MinorGridOpacity,
    ShowVerticalGrid,

    // Typography
    TitleFontSize,
    SubtitleFontSize,
    AxisLabelFontSize,
    TickLabelFontSize,

    // Legend
    LegendPosition,

    // Error bars
    ShowErrorBars,
    ErrorBarOpacity,
    ErrorBarThickness,
    CiLevel,
    ShowStdDevBand,

    // Regression
    ShowRegression,
    RegressionStyle,
    RegressionModel,
    ShowRegressionLabel,
    ShowRSquared,
    ShowEquation,
    ShowRegressionBand,
    RegressionBandOpacity,

    // Tick formatting
    RoundTicks,

    // Chart mode (performance vs throughput)
    ChartMode,
}

impl ChartParam {
    /// Get the DSL parameter name (camelCase as used in .bench files)
    pub fn dsl_name(&self) -> &'static str {
        match self {
            ChartParam::Title => "title",
            ChartParam::Description => "description",
            ChartParam::Output => "output",
            ChartParam::Width => "width",
            ChartParam::Height => "height",
            ChartParam::XLabel => "xlabel",
            ChartParam::ShowStats => "showStats",
            ChartParam::ShowConfig => "showConfig",
            ChartParam::ShowWinCounts => "showWinCounts",
            ChartParam::ShowGeoMean => "showGeoMean",
            ChartParam::ShowDistribution => "showDistribution",
            ChartParam::ShowMemory => "showMemory",
            ChartParam::ShowTotalTime => "showTotalTime",
            ChartParam::Compact => "compact",
            ChartParam::MinSpeedup => "minSpeedup",
            ChartParam::FilterWinner => "filterWinner",
            ChartParam::IncludeBenchmarks => "includeBenchmarks",
            ChartParam::ExcludeBenchmarks => "excludeBenchmarks",
            ChartParam::Limit => "limit",
            ChartParam::SortBy => "sortBy",
            ChartParam::SortOrder => "sortOrder",
            ChartParam::BarWidth => "barWidth",
            ChartParam::BarGroupGap => "barGroupGap",
            ChartParam::BarWithinGroupGap => "barWithinGroupGap",
            ChartParam::Precision => "precision",
            ChartParam::TimeUnit => "timeUnit",
            ChartParam::AxisThickness => "axisThickness",
            ChartParam::YAxisMin => "yAxisMin",
            ChartParam::YAxisMax => "yAxisMax",
            ChartParam::YScale => "yScale",
            ChartParam::BaselineBenchmark => "baselineBenchmark",
            ChartParam::SymlogThreshold => "symlogThreshold",
            ChartParam::ShowGrid => "showGrid",
            ChartParam::GridOpacity => "gridOpacity",
            ChartParam::ShowMinorGrid => "showMinorGrid",
            ChartParam::MinorGridOpacity => "minorGridOpacity",
            ChartParam::ShowVerticalGrid => "showVerticalGrid",
            ChartParam::TitleFontSize => "titleFontSize",
            ChartParam::SubtitleFontSize => "subtitleFontSize",
            ChartParam::AxisLabelFontSize => "axisLabelFontSize",
            ChartParam::TickLabelFontSize => "tickLabelFontSize",
            ChartParam::LegendPosition => "legendPosition",
            ChartParam::ShowErrorBars => "showErrorBars",
            ChartParam::ErrorBarOpacity => "errorBarOpacity",
            ChartParam::ErrorBarThickness => "errorBarThickness",
            ChartParam::CiLevel => "ciLevel",
            ChartParam::ShowStdDevBand => "showStdDevBand",
            ChartParam::ShowRegression => "showRegression",
            ChartParam::RegressionStyle => "regressionStyle",
            ChartParam::RegressionModel => "regressionModel",
            ChartParam::ShowRegressionLabel => "showRegressionLabel",
            ChartParam::ShowRSquared => "showRSquared",
            ChartParam::ShowEquation => "showEquation",
            ChartParam::ShowRegressionBand => "showRegressionBand",
            ChartParam::RegressionBandOpacity => "regressionBandOpacity",
            ChartParam::RoundTicks => "roundTicks",
            ChartParam::ChartMode => "chartMode",
        }
    }

    /// Try to parse a DSL parameter name into a ChartParam
    pub fn from_dsl_name(name: &str) -> Option<ChartParam> {
        match name {
            "title" => Some(ChartParam::Title),
            "description" => Some(ChartParam::Description),
            "output" => Some(ChartParam::Output),
            "width" => Some(ChartParam::Width),
            "height" => Some(ChartParam::Height),
            "xlabel" => Some(ChartParam::XLabel),
            "showStats" => Some(ChartParam::ShowStats),
            "showConfig" => Some(ChartParam::ShowConfig),
            "showWinCounts" => Some(ChartParam::ShowWinCounts),
            "showGeoMean" => Some(ChartParam::ShowGeoMean),
            "showDistribution" => Some(ChartParam::ShowDistribution),
            "showMemory" => Some(ChartParam::ShowMemory),
            "showTotalTime" => Some(ChartParam::ShowTotalTime),
            "compact" => Some(ChartParam::Compact),
            "minSpeedup" => Some(ChartParam::MinSpeedup),
            "filterWinner" => Some(ChartParam::FilterWinner),
            "includeBenchmarks" => Some(ChartParam::IncludeBenchmarks),
            "excludeBenchmarks" => Some(ChartParam::ExcludeBenchmarks),
            "limit" => Some(ChartParam::Limit),
            "sortBy" => Some(ChartParam::SortBy),
            "sortOrder" => Some(ChartParam::SortOrder),
            "barWidth" => Some(ChartParam::BarWidth),
            "barGroupGap" => Some(ChartParam::BarGroupGap),
            "barWithinGroupGap" => Some(ChartParam::BarWithinGroupGap),
            "precision" => Some(ChartParam::Precision),
            "timeUnit" => Some(ChartParam::TimeUnit),
            "axisThickness" => Some(ChartParam::AxisThickness),
            "yAxisMin" => Some(ChartParam::YAxisMin),
            "yAxisMax" => Some(ChartParam::YAxisMax),
            "yScale" => Some(ChartParam::YScale),
            "baselineBenchmark" | "baseline" => Some(ChartParam::BaselineBenchmark),
            "symlogThreshold" => Some(ChartParam::SymlogThreshold),
            "showGrid" => Some(ChartParam::ShowGrid),
            "gridOpacity" => Some(ChartParam::GridOpacity),
            "showMinorGrid" => Some(ChartParam::ShowMinorGrid),
            "minorGridOpacity" => Some(ChartParam::MinorGridOpacity),
            "showVerticalGrid" => Some(ChartParam::ShowVerticalGrid),
            "titleFontSize" => Some(ChartParam::TitleFontSize),
            "subtitleFontSize" => Some(ChartParam::SubtitleFontSize),
            "axisLabelFontSize" => Some(ChartParam::AxisLabelFontSize),
            "tickLabelFontSize" => Some(ChartParam::TickLabelFontSize),
            "legendPosition" => Some(ChartParam::LegendPosition),
            "showErrorBars" => Some(ChartParam::ShowErrorBars),
            "errorBarOpacity" => Some(ChartParam::ErrorBarOpacity),
            "errorBarThickness" => Some(ChartParam::ErrorBarThickness),
            "ciLevel" => Some(ChartParam::CiLevel),
            "showStdDevBand" => Some(ChartParam::ShowStdDevBand),
            "showRegression" => Some(ChartParam::ShowRegression),
            "regressionStyle" => Some(ChartParam::RegressionStyle),
            "regressionModel" => Some(ChartParam::RegressionModel),
            "showRegressionLabel" => Some(ChartParam::ShowRegressionLabel),
            "showRSquared" => Some(ChartParam::ShowRSquared),
            "showEquation" => Some(ChartParam::ShowEquation),
            "showRegressionBand" => Some(ChartParam::ShowRegressionBand),
            "regressionBandOpacity" => Some(ChartParam::RegressionBandOpacity),
            "roundTicks" => Some(ChartParam::RoundTicks),
            "chartMode" => Some(ChartParam::ChartMode),
            _ => None,
        }
    }
}

/// Parameters common to all chart types
fn common_params() -> HashSet<ChartParam> {
    [
        ChartParam::Title,
        ChartParam::Description,
        ChartParam::Output,
        ChartParam::Width,
        ChartParam::Height,
    ]
    .into_iter()
    .collect()
}

/// Filtering parameters (available to all charts)
fn filtering_params() -> HashSet<ChartParam> {
    [
        ChartParam::MinSpeedup,
        ChartParam::FilterWinner,
        ChartParam::IncludeBenchmarks,
        ChartParam::ExcludeBenchmarks,
        ChartParam::Limit,
    ]
    .into_iter()
    .collect()
}

/// Sorting parameters (available to all charts)
fn sorting_params() -> HashSet<ChartParam> {
    [ChartParam::SortBy, ChartParam::SortOrder].into_iter().collect()
}

/// Typography parameters
fn typography_params() -> HashSet<ChartParam> {
    [
        ChartParam::TitleFontSize,
        ChartParam::SubtitleFontSize,
        ChartParam::AxisLabelFontSize,
        ChartParam::TickLabelFontSize,
    ]
    .into_iter()
    .collect()
}

/// Data display parameters
fn data_display_params() -> HashSet<ChartParam> {
    [ChartParam::Precision, ChartParam::TimeUnit].into_iter().collect()
}

/// Grid parameters
fn grid_params() -> HashSet<ChartParam> {
    [
        ChartParam::ShowGrid,
        ChartParam::GridOpacity,
        ChartParam::ShowMinorGrid,
        ChartParam::MinorGridOpacity,
        ChartParam::ShowVerticalGrid,
    ]
    .into_iter()
    .collect()
}

/// Axis styling parameters
fn axis_styling_params() -> HashSet<ChartParam> {
    [
        ChartParam::AxisThickness,
        ChartParam::YAxisMin,
        ChartParam::YAxisMax,
        ChartParam::YScale,
        ChartParam::BaselineBenchmark,
        ChartParam::SymlogThreshold,
    ]
    .into_iter()
    .collect()
}

/// Error bar parameters
fn error_bar_params() -> HashSet<ChartParam> {
    [
        ChartParam::ShowErrorBars,
        ChartParam::ErrorBarOpacity,
        ChartParam::ErrorBarThickness,
        ChartParam::CiLevel,
    ]
    .into_iter()
    .collect()
}

/// Regression parameters
fn regression_params() -> HashSet<ChartParam> {
    [
        ChartParam::ShowRegression,
        ChartParam::RegressionStyle,
        ChartParam::RegressionModel,
        ChartParam::ShowRegressionLabel,
        ChartParam::ShowRSquared,
        ChartParam::ShowEquation,
        ChartParam::ShowRegressionBand,
        ChartParam::RegressionBandOpacity,
    ]
    .into_iter()
    .collect()
}

/// Get the set of valid parameters for a given chart type
pub fn get_valid_params(chart_type: ChartType) -> HashSet<ChartParam> {
    let mut params = common_params();
    params.extend(filtering_params());
    params.extend(sorting_params());
    params.extend(data_display_params());

    match chart_type {
        ChartType::BarChart => {
            params.insert(ChartParam::XLabel);
            params.extend([
                ChartParam::ShowStats,
                ChartParam::ShowConfig,
                ChartParam::ShowWinCounts,
                ChartParam::ShowGeoMean,
                ChartParam::ShowDistribution,
                ChartParam::ShowMemory,
                ChartParam::ShowTotalTime,
                ChartParam::Compact,
            ]);
            params.extend([
                ChartParam::BarWidth,
                ChartParam::BarGroupGap,
                ChartParam::BarWithinGroupGap,
            ]);
            params.extend(axis_styling_params());
            params.extend(grid_params());
            params.extend(typography_params());
            params.insert(ChartParam::LegendPosition);
            params.extend(error_bar_params());
            params.extend(regression_params());
            params.insert(ChartParam::RoundTicks);
            params.insert(ChartParam::ChartMode);
        }
        ChartType::LineChart => {
            params.insert(ChartParam::XLabel);
            params.insert(ChartParam::Compact);
            params.extend(axis_styling_params());
            params.extend(grid_params());
            params.extend(typography_params());
            params.insert(ChartParam::LegendPosition);
            params.extend(error_bar_params());
            params.insert(ChartParam::ShowStdDevBand);
            params.extend(regression_params());
            params.insert(ChartParam::RoundTicks);
            params.insert(ChartParam::ChartMode);
        }
        ChartType::SpeedupChart => {
            params.insert(ChartParam::BaselineBenchmark);
            params.extend([ChartParam::ShowGrid, ChartParam::GridOpacity]);
            params.extend(typography_params());
            params.insert(ChartParam::LegendPosition);
        }
        ChartType::Table => {
            params.extend([
                ChartParam::ShowStats,
                ChartParam::ShowConfig,
                ChartParam::ShowWinCounts,
                ChartParam::ShowGeoMean,
                ChartParam::Compact,
            ]);
            params.extend([ChartParam::TitleFontSize, ChartParam::SubtitleFontSize]);
        }
    }

    params
}

/// Result of parameter validation
#[derive(Debug)]
pub struct ParamValidationError {
    pub param_name: String,
    pub chart_type: ChartType,
    pub valid_chart_types: Vec<ChartType>,
}

impl ParamValidationError {
    pub fn message(&self) -> String {
        if self.valid_chart_types.is_empty() {
            format!(
                "Parameter '{}' is not valid for {} charts (unknown parameter)",
                self.param_name,
                self.chart_type.as_str()
            )
        } else {
            let valid_types: Vec<_> = self.valid_chart_types.iter().map(|t| t.as_str()).collect();
            format!(
                "Parameter '{}' is not valid for {} charts. Valid for: {}",
                self.param_name,
                self.chart_type.as_str(),
                valid_types.join(", ")
            )
        }
    }
}

/// Validate a parameter name for a given chart type
pub fn validate_param(chart_type: ChartType, param_name: &str) -> Result<(), ParamValidationError> {
    let valid_params = get_valid_params(chart_type);

    // Try to parse the parameter name
    if let Some(param) = ChartParam::from_dsl_name(param_name) {
        if valid_params.contains(&param) {
            return Ok(());
        }

        // Parameter exists but not valid for this chart type
        // Find which chart types it IS valid for
        let all_chart_types =
            [ChartType::BarChart, ChartType::LineChart, ChartType::SpeedupChart, ChartType::Table];

        let valid_chart_types: Vec<_> = all_chart_types
            .into_iter()
            .filter(|ct| get_valid_params(*ct).contains(&param))
            .collect();

        return Err(ParamValidationError {
            param_name: param_name.to_string(),
            chart_type,
            valid_chart_types,
        });
    }

    // Unknown parameter - return error with empty valid types
    Err(ParamValidationError {
        param_name: param_name.to_string(),
        chart_type,
        valid_chart_types: vec![],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bar_chart_params() {
        let params = get_valid_params(ChartType::BarChart);
        assert!(params.contains(&ChartParam::Title));
        assert!(params.contains(&ChartParam::ShowStats));
        assert!(params.contains(&ChartParam::ShowConfig));
        assert!(params.contains(&ChartParam::BarWidth));
        assert!(params.contains(&ChartParam::ShowRegression));
        assert!(!params.contains(&ChartParam::ShowStdDevBand)); // Line chart only
    }

    #[test]
    fn test_line_chart_params() {
        let params = get_valid_params(ChartType::LineChart);
        assert!(params.contains(&ChartParam::Title));
        assert!(params.contains(&ChartParam::ShowStdDevBand));
        assert!(params.contains(&ChartParam::ShowRegression));
        assert!(!params.contains(&ChartParam::ShowConfig)); // Bar/Table only
        assert!(!params.contains(&ChartParam::BarWidth)); // Bar only
    }

    #[test]
    fn test_validate_valid_param() {
        assert!(validate_param(ChartType::BarChart, "showStats").is_ok());
        assert!(validate_param(ChartType::LineChart, "showStdDevBand").is_ok());
    }

    #[test]
    fn test_validate_invalid_param() {
        let result = validate_param(ChartType::LineChart, "showConfig");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.valid_chart_types.contains(&ChartType::BarChart));
        assert!(err.valid_chart_types.contains(&ChartType::Table));
    }

    #[test]
    fn test_validate_unknown_param() {
        let result = validate_param(ChartType::BarChart, "unknownParam");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.valid_chart_types.is_empty());
    }

    #[test]
    fn test_param_dsl_name_roundtrip() {
        let params = [
            ChartParam::Title,
            ChartParam::ShowStats,
            ChartParam::BarWidth,
            ChartParam::ShowStdDevBand,
        ];
        for param in params {
            let name = param.dsl_name();
            let parsed = ChartParam::from_dsl_name(name);
            assert_eq!(parsed, Some(param));
        }
    }
}
