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

    // Filtering
    MinSpeedup,
    FilterWinner,
    IncludeBenchmarks,
    ExcludeBenchmarks,
    Limit,

    // Sorting
    SortBy,
    SortOrder,
    BaselineBenchmark,

    // Theme
    Theme,
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
            ChartParam::MinSpeedup => "minSpeedup",
            ChartParam::FilterWinner => "filterWinner",
            ChartParam::IncludeBenchmarks => "includeBenchmarks",
            ChartParam::ExcludeBenchmarks => "excludeBenchmarks",
            ChartParam::Limit => "limit",
            ChartParam::SortBy => "sortBy",
            ChartParam::SortOrder => "sortOrder",
            ChartParam::BaselineBenchmark => "baselineBenchmark",
            ChartParam::Theme => "theme",
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
            "minSpeedup" => Some(ChartParam::MinSpeedup),
            "filterWinner" => Some(ChartParam::FilterWinner),
            "includeBenchmarks" => Some(ChartParam::IncludeBenchmarks),
            "excludeBenchmarks" => Some(ChartParam::ExcludeBenchmarks),
            "limit" => Some(ChartParam::Limit),
            "sortBy" => Some(ChartParam::SortBy),
            "sortOrder" => Some(ChartParam::SortOrder),
            "baselineBenchmark" | "baseline" => Some(ChartParam::BaselineBenchmark),
            "theme" => Some(ChartParam::Theme),
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

/// Get the set of valid parameters for a given chart type
pub fn get_valid_params(chart_type: ChartType) -> HashSet<ChartParam> {
    let mut params = common_params();
    params.extend(filtering_params());
    params.extend(sorting_params());

    match chart_type {
        ChartType::SpeedupChart => {
            params.insert(ChartParam::BaselineBenchmark);
            params.insert(ChartParam::Theme);
        }
        ChartType::Table => {
            params.insert(ChartParam::Theme);
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
        let all_chart_types = [ChartType::SpeedupChart, ChartType::Table];

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
    fn test_speedup_chart_params() {
        let params = get_valid_params(ChartType::SpeedupChart);
        assert!(params.contains(&ChartParam::Title));
        assert!(params.contains(&ChartParam::BaselineBenchmark));
        assert!(params.contains(&ChartParam::Theme));
    }

    #[test]
    fn test_table_chart_params() {
        let params = get_valid_params(ChartType::Table);
        assert!(params.contains(&ChartParam::Title));
        assert!(params.contains(&ChartParam::Theme));
        assert!(!params.contains(&ChartParam::BaselineBenchmark));
    }

    #[test]
    fn test_validate_valid_param() {
        assert!(validate_param(ChartType::Table, "title").is_ok());
        assert!(validate_param(ChartType::SpeedupChart, "baseline").is_ok());
    }

    #[test]
    fn test_validate_invalid_param() {
        let result = validate_param(ChartType::SpeedupChart, "showStats");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.valid_chart_types.is_empty());
    }

    #[test]
    fn test_validate_unknown_param() {
        let result = validate_param(ChartType::Table, "unknownParam");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.valid_chart_types.is_empty());
    }

    #[test]
    fn test_param_dsl_name_roundtrip() {
        let params = [ChartParam::Title, ChartParam::BaselineBenchmark, ChartParam::Theme];
        for param in params {
            let name = param.dsl_name();
            let parsed = ChartParam::from_dsl_name(name);
            assert_eq!(parsed, Some(param));
        }
    }
}
