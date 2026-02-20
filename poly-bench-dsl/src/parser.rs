//! Parser for the poly-bench DSL
//!
//! Parses a stream of tokens into an AST.

use crate::{
    ast::{HookStyle, *},
    chart_params::validate_param,
    error::{NamedSource, ParseError},
    lexer::Lexer,
    tokens::{Token, TokenKind},
};
use miette::{Report, Result};
use std::collections::HashMap;

/// Normalize code indentation by stripping common leading whitespace
/// This preserves relative indentation while removing absolute positioning
fn normalize_code_indent(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();
    if lines.is_empty() {
        return String::new();
    }

    // Find minimum indent among non-empty lines, but skip first line if it has 0 indent
    // (this handles cases where reconstruct_code starts mid-line)
    let non_empty_lines: Vec<_> = lines.iter().filter(|l| !l.trim().is_empty()).collect();

    let min_indent = if non_empty_lines.len() <= 1 {
        // Only one line or empty, use its indent
        non_empty_lines.first().map(|l| l.len() - l.trim_start().len()).unwrap_or(0)
    } else {
        // Multiple lines - calculate min indent, but if first line has 0 indent,
        // use the min of the remaining lines
        let first_indent = non_empty_lines[0].len() - non_empty_lines[0].trim_start().len();
        let rest_min = non_empty_lines
            .iter()
            .skip(1)
            .map(|l| l.len() - l.trim_start().len())
            .min()
            .unwrap_or(0);

        if first_indent == 0 && rest_min > 0 {
            // First line has no indent but others do - use rest_min and leave first line as-is
            rest_min
        } else {
            // Normal case - use overall min
            first_indent.min(rest_min)
        }
    };

    // Strip the minimum indent from each line
    lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            if line.trim().is_empty() {
                String::new()
            } else {
                let line_indent = line.len() - line.trim_start().len();
                if i == 0 && line_indent == 0 && min_indent > 0 {
                    // First line with 0 indent when min_indent > 0 - keep as-is
                    line.to_string()
                } else if line.len() >= min_indent {
                    line[min_indent..].to_string()
                } else {
                    line.trim_start().to_string()
                }
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Parser state
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    filename: String,
    source: String,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, filename: String, source: String) -> Self {
        Self { tokens, current: 0, filename, source }
    }

    /// Parse the entire file
    pub fn parse_file(&mut self) -> Result<File> {
        let mut use_stds = Vec::new();
        let mut global_setup = None;
        let mut suites = Vec::new();

        // Parse top-level use statements first
        while self.check(TokenKind::Use) {
            use_stds.push(self.parse_use_std()?);
        }

        // Parse optional file-level globalSetup block (legacy, still supported)
        if self.check(TokenKind::GlobalSetup) {
            global_setup = Some(self.parse_global_setup()?);
        }

        // Parse suites
        while !self.is_at_end() {
            let mut suite = self.parse_suite()?;

            // If suite doesn't have its own globalSetup, inherit from file-level
            // (for backward compatibility)
            if suite.global_setup.is_none() && global_setup.is_some() {
                suite.global_setup = global_setup.clone();
            }

            suites.push(suite);
        }

        Ok(File::with_global_setup(use_stds, global_setup, suites))
    }

    /// Parse a use std::module statement
    fn parse_use_std(&mut self) -> Result<UseStd> {
        let use_token = self.expect_keyword(TokenKind::Use)?;
        let use_span = use_token.span.clone();

        // Expect "std" identifier
        let std_token = self.expect_identifier()?;
        let std_span = std_token.span.clone();
        let std_name = match &std_token.kind {
            TokenKind::Identifier(s) => s.clone(),
            _ => unreachable!(),
        };

        if std_name != "std" {
            return Err(self.make_error(ParseError::ExpectedToken {
                expected: "std".to_string(),
                found: std_name,
                span: std_token.span.clone(),
            }));
        }

        // Expect "::"
        self.expect(TokenKind::DoubleColon)?;

        // Expect module name
        let module_token = self.expect_identifier()?;
        let module_span = module_token.span.clone();
        let module = match &module_token.kind {
            TokenKind::Identifier(s) => s.clone(),
            _ => unreachable!(),
        };

        // Full span from 'use' to end of module name
        let full_span = Span::new(use_span.start, module_span.end, use_span.line, use_span.col);

        Ok(UseStd::new(module, full_span, use_span, std_span, module_span))
    }

    /// Parse a globalSetup block
    ///
    /// Syntax:
    /// ```text
    /// globalSetup {
    ///     anvil.spawnAnvil()                    # namespaced (preferred)
    ///     anvil.spawnAnvil(fork: "https://...")
    ///     spawnAnvil()                          # legacy (still supported)
    /// }
    /// ```
    fn parse_global_setup(&mut self) -> Result<GlobalSetup> {
        let start_token = self.expect_keyword(TokenKind::GlobalSetup)?;
        let start_span = start_token.span.clone();

        self.expect(TokenKind::LBrace)?;

        let mut anvil_config = None;

        // Parse statements inside globalSetup
        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            // Look for namespaced calls: anvil.spawnAnvil()
            if self.check_identifier("anvil") {
                let _module_span = self.advance().span.clone();

                // Expect "."
                if self.check(TokenKind::Dot) {
                    self.advance(); // consume "."

                    // Expect "spawnAnvil"
                    if self.check_identifier("spawnAnvil") {
                        let anvil_span = self.advance().span.clone();
                        anvil_config = Some(self.parse_spawn_anvil_args(anvil_span)?);
                    } else {
                        // Unknown member
                        self.advance();
                    }
                } else {
                    // Just "anvil" without dot - skip
                }
            }
            // Legacy: Look for direct spawnAnvil() calls (backward compatibility)
            else if self.check_identifier("spawnAnvil") {
                let anvil_span = self.advance().span.clone();
                anvil_config = Some(self.parse_spawn_anvil_args(anvil_span)?);
            } else {
                // Skip unknown statements for now
                self.advance();
            }
        }

        let end_token = self.expect(TokenKind::RBrace)?;

        let full_span =
            Span::new(start_span.start, end_token.span.end, start_span.line, start_span.col);

        Ok(GlobalSetup::new(anvil_config, full_span))
    }

    /// Parse the arguments for spawnAnvil()
    fn parse_spawn_anvil_args(&mut self, anvil_span: Span) -> Result<AnvilSetupConfig> {
        self.expect(TokenKind::LParen)?;

        let mut fork_url = None;

        // Check for optional arguments: fork: "url"
        if !self.check(TokenKind::RParen) {
            // Expect "fork" identifier
            if self.check_identifier("fork") {
                self.advance(); // consume "fork"
                self.expect(TokenKind::Colon)?;

                // Expect string value
                fork_url = Some(self.expect_string()?);
            }
        }

        self.expect(TokenKind::RParen)?;

        Ok(AnvilSetupConfig::new(fork_url, anvil_span))
    }

    /// Parse a suite definition
    fn parse_suite(&mut self) -> Result<Suite> {
        // Expect 'suite' keyword
        self.expect_keyword(TokenKind::Suite)?;

        // Get suite name
        let name_token = self.expect_identifier()?;
        let name = match &name_token.kind {
            TokenKind::Identifier(s) => s.clone(),
            _ => unreachable!(),
        };

        // Expect opening brace
        self.expect(TokenKind::LBrace)?;

        let mut suite = Suite::new(name, name_token.span.clone());

        // Parse suite body
        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            self.parse_suite_item(&mut suite)?;
        }

        // Expect closing brace
        self.expect(TokenKind::RBrace)?;

        Ok(suite)
    }

    /// Parse a single item within a suite
    fn parse_suite_item(&mut self, suite: &mut Suite) -> Result<()> {
        let token = self.peek().clone();

        match &token.kind {
            TokenKind::Description => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_string()?;
                suite.description = Some(value);
            }
            TokenKind::Iterations => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_number()?;
                suite.iterations = Some(value);
            }
            TokenKind::Warmup => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_number()?;
                suite.warmup = Some(value);
            }
            // Phase 4: Suite-level configuration
            TokenKind::Timeout => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_duration()?;
                suite.timeout = Some(value);
            }
            TokenKind::Requires => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let langs = self.parse_lang_array()?;
                suite.requires = langs;
            }
            TokenKind::Order => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let order = self.expect_execution_order()?;
                suite.order = Some(order);
            }
            TokenKind::Compare => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_bool()?;
                suite.compare = value;
            }
            TokenKind::Baseline => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let lang = self.expect_lang_from_string()?;
                suite.baseline = Some(lang);
            }
            // Benchmark accuracy settings
            TokenKind::Mode => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let mode = self.expect_bench_mode()?;
                suite.mode = Some(mode);
            }
            TokenKind::Sink => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_bool()?;
                suite.sink = value;
            }
            TokenKind::TargetTime => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_duration()?;
                suite.target_time_ms = Some(value);
            }
            TokenKind::MinIterations => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_number()?;
                suite.min_iterations = Some(value);
            }
            TokenKind::MaxIterations => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_number()?;
                suite.max_iterations = Some(value);
            }
            TokenKind::OutlierDetection => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_bool()?;
                suite.outlier_detection = value;
            }
            TokenKind::CvThreshold => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_float()?;
                suite.cv_threshold = Some(value);
            }
            TokenKind::Count => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_number()?;
                suite.count = Some(value);
            }
            TokenKind::Memory => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_bool()?;
                suite.memory = value;
            }
            TokenKind::Concurrency => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_number()?;
                suite.concurrency = value as u32;
            }
            // globalSetup can now be inside suite
            TokenKind::GlobalSetup => {
                let global_setup = self.parse_global_setup()?;
                suite.global_setup = Some(global_setup);
            }
            TokenKind::Setup => {
                let (lang, setup) = self.parse_structured_setup()?;
                suite.setups.insert(lang, setup);
            }
            TokenKind::Fixture => {
                let fixture = self.parse_fixture()?;
                suite.fixtures.push(fixture);
            }
            TokenKind::Bench => {
                let benchmark = self.parse_benchmark()?;
                suite.benchmarks.push(benchmark);
            }
            // Suite-level after block for chart directives
            TokenKind::After => {
                let directives = self.parse_suite_after_block()?;
                suite.chart_directives.extend(directives);
            }
            TokenKind::Identifier(_) => {
                // Could be a property or language implementation
                // Check if it's a known property
                let ident = match &token.kind {
                    TokenKind::Identifier(s) => s.clone(),
                    _ => unreachable!(),
                };

                return Err(self.make_error(ParseError::InvalidProperty {
                    name: ident,
                    span: token.span.clone(),
                }));
            }
            _ => {
                return Err(self.make_error(ParseError::ExpectedToken {
                    expected: "suite item (globalSetup, setup, fixture, bench, after, or property)"
                        .to_string(),
                    found: format!("{:?}", token.kind),
                    span: token.span.clone(),
                }));
            }
        }

        Ok(())
    }

    /// Parse a suite-level after block containing chart directives
    /// Syntax: after { charting.drawBarChart(...) }
    fn parse_suite_after_block(&mut self) -> Result<Vec<ChartDirective>> {
        let after_token = self.expect_keyword(TokenKind::After)?;
        self.expect(TokenKind::LBrace)?;

        let mut directives = Vec::new();

        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            let directive = self.parse_chart_directive(&after_token.span)?;
            directives.push(directive);
        }

        self.expect(TokenKind::RBrace)?;

        Ok(directives)
    }

    /// Parse a chart directive: charting.drawBarChart(title: "...", ...)
    fn parse_chart_directive(&mut self, _block_span: &Span) -> Result<ChartDirective> {
        let start_span = self.peek().span.clone();

        // Expect "charting" identifier
        let module_token = self.expect_identifier()?;
        let module_name = match &module_token.kind {
            TokenKind::Identifier(s) => s.clone(),
            _ => unreachable!(),
        };

        if module_name != "charting" {
            return Err(self.make_error(ParseError::InvalidProperty {
                name: format!("Expected 'charting' module, found '{}'", module_name),
                span: module_token.span.clone(),
            }));
        }

        // Expect "."
        self.expect(TokenKind::Dot)?;

        // Expect function name (drawBarChart, drawPieChart, etc.)
        let func_token = self.expect_identifier()?;
        let func_name = match &func_token.kind {
            TokenKind::Identifier(s) => s.clone(),
            _ => unreachable!(),
        };

        let chart_type = ChartType::from_function_name(&func_name).ok_or_else(|| {
            self.make_error(ParseError::InvalidProperty {
                name: format!("Unknown charting function '{}'. Valid functions: drawBarChart, drawPieChart, drawLineChart", func_name),
                span: func_token.span.clone(),
            })
        })?;

        // Expect "("
        self.expect(TokenKind::LParen)?;

        let mut directive = ChartDirective::new(chart_type, start_span);

        // Parse optional named parameters with various types
        while !self.check(TokenKind::RParen) && !self.is_at_end() {
            let param_token = self.advance().clone();
            let param_name = match &param_token.kind {
                TokenKind::Identifier(s) => s.clone(),
                TokenKind::Description => "description".to_string(),
                TokenKind::Order => "order".to_string(),
                TokenKind::Timeout => "timeout".to_string(),
                TokenKind::Baseline => "baseline".to_string(),
                _ => {
                    return Err(self.make_error(ParseError::ExpectedIdentifier {
                        span: param_token.span.clone(),
                    }));
                }
            };

            self.expect(TokenKind::Colon)?;

            // Validate parameter is valid for this chart type
            if let Err(validation_err) = validate_param(chart_type, &param_name) {
                return Err(self.make_error(ParseError::InvalidProperty {
                    name: validation_err.message(),
                    span: param_token.span.clone(),
                }));
            }

            // Parse value based on expected type for each parameter
            match param_name.as_str() {
                // String parameters
                "title" => directive.title = Some(self.expect_string()?),
                "description" => directive.description = Some(self.expect_string()?),
                "xlabel" => directive.x_label = Some(self.expect_string()?),
                "ylabel" => directive.y_label = Some(self.expect_string()?),
                "output" => directive.output_file = Some(self.expect_string()?),
                "filterWinner" => directive.filter_winner = Some(self.expect_string()?),
                "sortBy" => directive.sort_by = Some(self.expect_string()?),
                "sortOrder" => directive.sort_order = Some(self.expect_string()?),
                "timeUnit" => directive.time_unit = Some(self.expect_string()?),
                "legendPosition" => directive.legend_position = Some(self.expect_string()?),
                "regressionStyle" => directive.regression_style = Some(self.expect_string()?),
                "yScale" => directive.y_scale = Some(self.expect_string()?),
                "regressionModel" => directive.regression_model = Some(self.expect_string()?),
                "baseline" | "baselineBenchmark" => {
                    directive.baseline_benchmark = Some(self.expect_string()?)
                }

                // Boolean parameters
                "showStats" => directive.show_stats = self.expect_bool()?,
                "showConfig" => directive.show_config = self.expect_bool()?,
                "showWinCounts" => directive.show_win_counts = self.expect_bool()?,
                "showGeoMean" => directive.show_geo_mean = self.expect_bool()?,
                "showDistribution" => directive.show_distribution = self.expect_bool()?,
                "showMemory" => directive.show_memory = self.expect_bool()?,
                "showTotalTime" => directive.show_total_time = self.expect_bool()?,
                "compact" => directive.compact = self.expect_bool()?,
                "showGrid" => directive.show_grid = Some(self.expect_bool()?),
                "showErrorBars" => directive.show_error_bars = Some(self.expect_bool()?),
                "showRegression" => directive.show_regression = Some(self.expect_bool()?),
                "showRegressionLabel" => {
                    directive.show_regression_label = Some(self.expect_bool()?)
                }
                "roundTicks" => directive.round_ticks = Some(self.expect_bool()?),
                "showRSquared" => directive.show_r_squared = Some(self.expect_bool()?),
                "showEquation" => directive.show_equation = Some(self.expect_bool()?),
                "showMinorGrid" => directive.show_minor_grid = Some(self.expect_bool()?),
                "showVerticalGrid" => directive.show_vertical_grid = Some(self.expect_bool()?),
                "showStdDevBand" => directive.show_std_dev_band = Some(self.expect_bool()?),
                "showRegressionBand" => directive.show_regression_band = Some(self.expect_bool()?),

                // Integer parameters
                "limit" => directive.limit = Some(self.expect_number()? as u32),
                "width" => directive.width = Some(self.expect_number()? as i32),
                "precision" => directive.precision = Some(self.expect_number()? as u32),
                "height" => directive.height = Some(self.expect_number()? as i32),
                "titleFontSize" => directive.title_font_size = Some(self.expect_number()? as i32),
                "subtitleFontSize" => {
                    directive.subtitle_font_size = Some(self.expect_number()? as i32)
                }
                "axisLabelFontSize" => {
                    directive.axis_label_font_size = Some(self.expect_number()? as i32)
                }
                "tickLabelFontSize" => {
                    directive.tick_label_font_size = Some(self.expect_number()? as i32)
                }
                "barGroupGap" => directive.bar_group_gap = Some(self.expect_number()? as i32),
                "barWithinGroupGap" => {
                    directive.bar_within_group_gap = Some(self.expect_number()? as i32)
                }
                "barWidth" => directive.bar_width = Some(self.expect_number()? as i32),
                "ciLevel" => directive.ci_level = Some(self.expect_number()? as u32),

                // Float parameters
                "minSpeedup" => directive.min_speedup = Some(self.expect_float()?),
                "axisThickness" => directive.axis_thickness = Some(self.expect_float()? as f32),
                "yAxisMin" => directive.y_axis_min = Some(self.expect_float()?),
                "yAxisMax" => directive.y_axis_max = Some(self.expect_float()?),
                "gridOpacity" => directive.grid_opacity = Some(self.expect_float()? as f32),
                "errorBarOpacity" => {
                    directive.error_bar_opacity = Some(self.expect_float()? as f32)
                }
                "errorBarThickness" => {
                    directive.error_bar_thickness = Some(self.expect_float()? as f32)
                }
                "minorGridOpacity" => {
                    directive.minor_grid_opacity = Some(self.expect_float()? as f32)
                }
                "regressionBandOpacity" => {
                    directive.regression_band_opacity = Some(self.expect_float()? as f32)
                }
                "symlogThreshold" => directive.symlog_threshold = Some(self.expect_float()?),

                // Array parameters
                "includeBenchmarks" => directive.include_benchmarks = self.expect_string_array()?,
                "excludeBenchmarks" => directive.exclude_benchmarks = self.expect_string_array()?,

                _ => {
                    return Err(self.make_error(ParseError::InvalidProperty {
                        name: format!("Unknown chart parameter '{}'", param_name),
                        span: param_token.span.clone(),
                    }));
                }
            }

            // Check for comma between parameters
            if !self.check(TokenKind::RParen) {
                if self.check(TokenKind::Comma) {
                    self.advance();
                }
            }
        }

        // Expect ")"
        self.expect(TokenKind::RParen)?;

        // Update span to include the full directive
        directive.span = Span {
            start: directive.span.start,
            end: self.previous().span.end,
            line: directive.span.line,
            col: directive.span.col,
        };

        Ok(directive)
    }

    /// Expect and parse a boolean value (true/false)
    fn expect_bool(&mut self) -> Result<bool> {
        let token = self.advance().clone();
        match token.kind {
            TokenKind::True => Ok(true),
            TokenKind::False => Ok(false),
            _ => Err(self.make_error(ParseError::InvalidProperty {
                name: "Expected 'true' or 'false'".to_string(),
                span: token.span,
            })),
        }
    }

    /// Expect and parse a number (integer)
    fn expect_number(&mut self) -> Result<u64> {
        let token = self.advance().clone();
        match token.kind {
            TokenKind::Number(n) => Ok(n),
            _ => Err(self.make_error(ParseError::InvalidProperty {
                name: "Expected a number".to_string(),
                span: token.span,
            })),
        }
    }

    /// Expect and parse a float (or integer as float)
    fn expect_float(&mut self) -> Result<f64> {
        let token = self.advance().clone();
        match token.kind {
            TokenKind::Float(f) => Ok(f),
            TokenKind::Number(n) => Ok(n as f64),
            _ => Err(self.make_error(ParseError::InvalidProperty {
                name: "Expected a number".to_string(),
                span: token.span,
            })),
        }
    }

    /// Expect and parse a string array: ["item1", "item2"]
    fn expect_string_array(&mut self) -> Result<Vec<String>> {
        self.expect(TokenKind::LBracket)?;

        let mut items = Vec::new();

        while !self.check(TokenKind::RBracket) && !self.is_at_end() {
            let value = self.expect_string()?;
            items.push(value);

            if !self.check(TokenKind::RBracket) {
                if self.check(TokenKind::Comma) {
                    self.advance();
                }
            }
        }

        self.expect(TokenKind::RBracket)?;
        Ok(items)
    }

    /// Parse a structured setup block with import/declare/init/helpers sections
    fn parse_structured_setup(&mut self) -> Result<(Lang, StructuredSetup)> {
        let setup_token = self.expect_keyword(TokenKind::Setup)?;
        let lang = self.expect_lang()?;

        self.expect(TokenKind::LBrace)?;

        let mut setup = StructuredSetup::new(setup_token.span.clone());

        // Parse setup sections in any order
        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            let token = self.peek().clone();

            match &token.kind {
                TokenKind::Import => {
                    self.advance();
                    // Parse import block - can be { ... } or ( ... ) for Go
                    let code = self.parse_import_block()?;
                    setup.imports = Some(code);
                }
                TokenKind::Declare => {
                    self.advance();
                    let code = self.parse_code_block()?;
                    setup.declarations = Some(code);
                }
                TokenKind::Async => {
                    // async init { ... }
                    self.advance();
                    self.expect_keyword(TokenKind::Init)?;
                    let code = self.parse_code_block()?;
                    setup.init = Some(code);
                    setup.async_init = true;
                }
                TokenKind::Init => {
                    self.advance();
                    let code = self.parse_code_block()?;
                    setup.init = Some(code);
                }
                TokenKind::Helpers => {
                    self.advance();
                    let code = self.parse_code_block()?;
                    setup.helpers = Some(code);
                }
                _ => {
                    return Err(self.make_error(ParseError::ExpectedToken {
                        expected: "setup section (import, declare, init, helpers)".to_string(),
                        found: format!("{:?}", token.kind),
                        span: token.span.clone(),
                    }));
                }
            }
        }

        self.expect(TokenKind::RBrace)?;

        Ok((lang, setup))
    }

    /// Parse an import block - handles both { ... } and ( ... ) for Go-style imports
    fn parse_import_block(&mut self) -> Result<CodeBlock> {
        // Check if it's ( for Go grouped imports or { for general block
        if self.check(TokenKind::LParen) {
            // Go-style grouped import: import ( "pkg1" "pkg2" )
            let _open_paren = self.advance().clone();
            let mut code_tokens = Vec::new();
            let mut depth = 1;

            while depth > 0 && !self.is_at_end() {
                let token = self.advance().clone();
                match token.kind {
                    TokenKind::LParen => depth += 1,
                    TokenKind::RParen => {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    _ => {}
                }
                if depth > 0 {
                    code_tokens.push(token);
                }
            }

            let code = self.reconstruct_code(&code_tokens);
            // Wrap in import ( ... ) for the code, preserving internal indentation
            let full_code = format!("import (\n{}\n)", normalize_code_indent(&code));

            // Compute span covering the import content
            let code_span = if code_tokens.is_empty() {
                Span::dummy()
            } else {
                let first = &code_tokens[0];
                let last = &code_tokens[code_tokens.len() - 1];
                Span {
                    start: first.span.start,
                    end: last.span.end,
                    line: first.span.line,
                    col: first.span.col,
                }
            };

            Ok(CodeBlock::new(full_code, true, code_span))
        } else if self.check(TokenKind::LBrace) {
            // Block-style imports: import { ... } from 'pkg' for TS
            self.parse_code_block()
        } else {
            // Single line import - consume until next section keyword or brace
            let mut code_tokens = Vec::new();

            while !self.is_at_end() {
                let token = self.peek();

                // Stop at section keywords or closing brace
                if matches!(
                    token.kind,
                    TokenKind::RBrace |
                        TokenKind::Import |
                        TokenKind::Declare |
                        TokenKind::Init |
                        TokenKind::Helpers |
                        TokenKind::Async
                ) {
                    break;
                }

                code_tokens.push(self.advance().clone());
            }

            let code = self.reconstruct_code(&code_tokens);

            // Compute span covering all collected tokens
            let code_span = if code_tokens.is_empty() {
                Span::dummy()
            } else {
                let first = &code_tokens[0];
                let last = &code_tokens[code_tokens.len() - 1];
                Span {
                    start: first.span.start,
                    end: last.span.end,
                    line: first.span.line,
                    col: first.span.col,
                }
            };

            Ok(CodeBlock::new(code.trim().to_string(), false, code_span))
        }
    }

    /// Parse a fixture definition
    fn parse_fixture(&mut self) -> Result<Fixture> {
        self.expect_keyword(TokenKind::Fixture)?;

        let name_token = self.expect_identifier()?;
        let name = match &name_token.kind {
            TokenKind::Identifier(s) => s.clone(),
            _ => unreachable!(),
        };

        let mut fixture = Fixture::new(name, name_token.span.clone());

        // Check for parameterized fixture: fixture name(param: type, ...)
        if self.check(TokenKind::LParen) {
            self.advance();
            fixture.params = self.parse_fixture_params()?;
            self.expect(TokenKind::RParen)?;
        }

        self.expect(TokenKind::LBrace)?;

        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            self.parse_fixture_item(&mut fixture)?;
        }

        self.expect(TokenKind::RBrace)?;

        Ok(fixture)
    }

    /// Parse fixture parameters: (name: type, name2: type2)
    fn parse_fixture_params(&mut self) -> Result<Vec<FixtureParam>> {
        let mut params = Vec::new();

        while !self.check(TokenKind::RParen) && !self.is_at_end() {
            let name_token = self.expect_identifier()?;
            let name = match &name_token.kind {
                TokenKind::Identifier(s) => s.clone(),
                _ => unreachable!(),
            };

            self.expect(TokenKind::Colon)?;

            let type_token = self.expect_identifier()?;
            let param_type = match &type_token.kind {
                TokenKind::Identifier(s) => s.clone(),
                _ => unreachable!(),
            };

            params.push(FixtureParam::new(name, param_type));

            // Check for comma
            if !self.check(TokenKind::RParen) {
                self.expect(TokenKind::Comma)?;
            }
        }

        Ok(params)
    }

    /// Parse a single item within a fixture
    fn parse_fixture_item(&mut self, fixture: &mut Fixture) -> Result<()> {
        let token = self.peek().clone();

        match &token.kind {
            TokenKind::Description => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_string()?;
                fixture.description = Some(value);
            }
            TokenKind::Hex => {
                self.advance();
                self.expect(TokenKind::Colon)?;

                // Check for @file reference or string literal
                if self.check(TokenKind::FileRef) {
                    self.advance();
                    self.expect(TokenKind::LParen)?;
                    let path = self.expect_string()?;
                    self.expect(TokenKind::RParen)?;
                    fixture.hex_file = Some(path);
                } else {
                    let value = self.expect_string()?;
                    fixture.hex_data = Some(value);
                }
            }
            // Phase 5: Shape annotation
            TokenKind::Shape => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let shape_code = self.parse_code_block()?;
                fixture.shape = Some(shape_code.code);
            }
            // Language-specific implementation
            TokenKind::Go |
            TokenKind::Ts |
            TokenKind::TypeScript |
            TokenKind::Rust |
            TokenKind::Python => {
                let lang = self.expect_lang()?;
                self.expect(TokenKind::Colon)?;
                let code = self.parse_inline_or_block_code()?;
                if !fixture.implementations.contains_key(&lang) {
                    fixture.impl_order.push(lang);
                }
                fixture.implementations.insert(lang, code);
            }
            TokenKind::Identifier(s) => {
                // Check if it's a language
                if let Some(lang) = Lang::from_str(s) {
                    self.advance();
                    self.expect(TokenKind::Colon)?;
                    let code = self.parse_inline_or_block_code()?;
                    if !fixture.implementations.contains_key(&lang) {
                        fixture.impl_order.push(lang);
                    }
                    fixture.implementations.insert(lang, code);
                } else {
                    return Err(self.make_error(ParseError::InvalidProperty {
                        name: s.clone(),
                        span: token.span.clone(),
                    }));
                }
            }
            _ => {
                return Err(self.make_error(ParseError::ExpectedToken {
                    expected: "fixture property (hex, description, shape) or language".to_string(),
                    found: format!("{:?}", token.kind),
                    span: token.span.clone(),
                }));
            }
        }

        Ok(())
    }

    /// Parse a benchmark definition
    fn parse_benchmark(&mut self) -> Result<Benchmark> {
        self.expect_keyword(TokenKind::Bench)?;

        let name_token = self.expect_identifier()?;
        let name = match &name_token.kind {
            TokenKind::Identifier(s) => s.clone(),
            _ => unreachable!(),
        };

        self.expect(TokenKind::LBrace)?;

        let mut benchmark = Benchmark::new(name, name_token.span.clone());

        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            self.parse_benchmark_item(&mut benchmark)?;
        }

        self.expect(TokenKind::RBrace)?;

        Ok(benchmark)
    }

    /// Parse a single item within a benchmark
    fn parse_benchmark_item(&mut self, benchmark: &mut Benchmark) -> Result<()> {
        let token = self.peek().clone();

        match &token.kind {
            TokenKind::Description => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_string()?;
                benchmark.description = Some(value);
            }
            TokenKind::Iterations => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_number()?;
                benchmark.iterations = Some(value);
            }
            // Phase 2: Benchmark configuration
            TokenKind::Warmup => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_number()?;
                benchmark.warmup = Some(value);
            }
            TokenKind::Timeout => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_duration()?;
                benchmark.timeout = Some(value);
            }
            TokenKind::Tags => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let tags = self.parse_string_array()?;
                benchmark.tags = tags;
            }
            TokenKind::Skip => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let skip_map = self.parse_lang_code_map()?;
                benchmark.skip = skip_map;
            }
            TokenKind::Validate => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let validate_map = self.parse_lang_code_map()?;
                benchmark.validate = validate_map;
            }
            // Benchmark accuracy settings (overrides)
            TokenKind::Mode => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let mode = self.expect_bench_mode()?;
                benchmark.mode = Some(mode);
            }
            TokenKind::Sink => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_bool()?;
                benchmark.sink = Some(value);
            }
            TokenKind::TargetTime => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_duration()?;
                benchmark.target_time_ms = Some(value);
            }
            TokenKind::MinIterations => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_number()?;
                benchmark.min_iterations = Some(value);
            }
            TokenKind::MaxIterations => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_number()?;
                benchmark.max_iterations = Some(value);
            }
            TokenKind::OutlierDetection => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_bool()?;
                benchmark.outlier_detection = Some(value);
            }
            TokenKind::CvThreshold => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_float()?;
                benchmark.cv_threshold = Some(value);
            }
            TokenKind::Count => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_number()?;
                benchmark.count = Some(value);
            }
            TokenKind::Memory => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_bool()?;
                benchmark.memory = Some(value);
            }
            TokenKind::Concurrency => {
                self.advance();
                self.expect(TokenKind::Colon)?;
                let value = self.expect_number()?;
                benchmark.concurrency = Some(value as u32);
            }
            // Phase 3: Lifecycle hooks - support both grouped and flat syntax
            TokenKind::Before => {
                self.advance();
                // Check if next token is a language (flat syntax) or colon (grouped syntax)
                if self.peek_is_lang() {
                    // Flat syntax: before go: CODE
                    let lang = self.expect_lang()?;
                    self.expect(TokenKind::Colon)?;
                    let code = self.parse_inline_or_block_code()?;
                    benchmark.before.insert(lang, code);
                    benchmark.hook_style = HookStyle::Flat;
                } else {
                    // Grouped syntax: before: { go: CODE, ts: CODE }
                    self.expect(TokenKind::Colon)?;
                    let before_map = self.parse_lang_code_map()?;
                    benchmark.before = before_map;
                    benchmark.hook_style = HookStyle::Grouped;
                }
            }
            TokenKind::After => {
                self.advance();
                // Check if next token is a language (flat syntax) or colon (grouped syntax)
                if self.peek_is_lang() {
                    // Flat syntax: after go: CODE
                    let lang = self.expect_lang()?;
                    self.expect(TokenKind::Colon)?;
                    let code = self.parse_inline_or_block_code()?;
                    benchmark.after.insert(lang, code);
                    benchmark.hook_style = HookStyle::Flat;
                } else {
                    // Grouped syntax: after: { go: CODE, ts: CODE }
                    self.expect(TokenKind::Colon)?;
                    let after_map = self.parse_lang_code_map()?;
                    benchmark.after = after_map;
                    benchmark.hook_style = HookStyle::Grouped;
                }
            }
            TokenKind::Each => {
                self.advance();
                // Check if next token is a language (flat syntax) or colon (grouped syntax)
                if self.peek_is_lang() {
                    // Flat syntax: each go: CODE
                    let lang = self.expect_lang()?;
                    self.expect(TokenKind::Colon)?;
                    let code = self.parse_inline_or_block_code()?;
                    benchmark.each.insert(lang, code);
                    benchmark.hook_style = HookStyle::Flat;
                } else {
                    // Grouped syntax: each: { go: CODE, ts: CODE }
                    self.expect(TokenKind::Colon)?;
                    let each_map = self.parse_lang_code_map()?;
                    benchmark.each = each_map;
                    benchmark.hook_style = HookStyle::Grouped;
                }
            }
            // Language-specific implementation
            TokenKind::Go |
            TokenKind::Ts |
            TokenKind::TypeScript |
            TokenKind::Rust |
            TokenKind::Python => {
                let lang = self.expect_lang()?;
                self.expect(TokenKind::Colon)?;
                let code = self.parse_inline_or_block_code()?;
                if !benchmark.implementations.contains_key(&lang) {
                    benchmark.impl_order.push(lang);
                }
                benchmark.implementations.insert(lang, code);
            }
            TokenKind::Identifier(s) => {
                // Check if it's a language
                if let Some(lang) = Lang::from_str(s) {
                    self.advance();
                    self.expect(TokenKind::Colon)?;
                    let code = self.parse_inline_or_block_code()?;
                    if !benchmark.implementations.contains_key(&lang) {
                        benchmark.impl_order.push(lang);
                    }
                    benchmark.implementations.insert(lang, code);
                } else {
                    return Err(self.make_error(ParseError::InvalidProperty {
                        name: s.clone(),
                        span: token.span.clone(),
                    }));
                }
            }
            _ => {
                return Err(self.make_error(ParseError::ExpectedToken {
                    expected: "benchmark property (iterations, warmup, timeout, tags, skip, validate, mode, sink, targetTime, minIterations, maxIterations, before, after, each) or language implementation".to_string(),
                    found: format!("{:?}", token.kind),
                    span: token.span.clone(),
                }));
            }
        }

        Ok(())
    }

    /// Parse a language-to-code map: { go: CODE, ts: CODE }
    fn parse_lang_code_map(&mut self) -> Result<HashMap<Lang, CodeBlock>> {
        let mut map = HashMap::new();

        self.expect(TokenKind::LBrace)?;

        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            let lang = self.expect_lang()?;
            self.expect(TokenKind::Colon)?;
            let code = self.parse_inline_or_block_code_in_map()?;
            map.insert(lang, code);
        }

        self.expect(TokenKind::RBrace)?;

        Ok(map)
    }

    /// Parse inline or block code within a lang code map
    /// Similar to parse_inline_or_block_code but stops at lang keywords too
    fn parse_inline_or_block_code_in_map(&mut self) -> Result<CodeBlock> {
        if self.check(TokenKind::LBrace) {
            self.parse_code_block()
        } else {
            let mut code_tokens = Vec::new();

            while !self.is_at_end() {
                let token = self.peek();

                // Stop conditions for inline code in a map
                if matches!(
                    token.kind,
                    TokenKind::RBrace |
                        TokenKind::Go |
                        TokenKind::Ts |
                        TokenKind::TypeScript |
                        TokenKind::Rust |
                        TokenKind::Python
                ) {
                    break;
                }

                // Check if identifier might be a language
                if let TokenKind::Identifier(s) = &token.kind {
                    if Lang::from_str(s).is_some() {
                        break;
                    }
                }

                code_tokens.push(self.advance().clone());
            }

            let code = self.reconstruct_code(&code_tokens);

            // Compute span covering all collected tokens
            let code_span = if code_tokens.is_empty() {
                Span::dummy()
            } else {
                let first = &code_tokens[0];
                let last = &code_tokens[code_tokens.len() - 1];
                Span {
                    start: first.span.start,
                    end: last.span.end,
                    line: first.span.line,
                    col: first.span.col,
                }
            };

            Ok(CodeBlock::new(code.trim().to_string(), false, code_span))
        }
    }

    /// Parse a code block (braces required)
    fn parse_code_block(&mut self) -> Result<CodeBlock> {
        let open_brace = self.expect(TokenKind::LBrace)?;
        let content_start = open_brace.span.end; // Start right after the opening brace

        // Find matching closing brace with brace counting
        let mut depth = 1;
        let mut close_brace_span: Option<Span> = None;

        while depth > 0 && !self.is_at_end() {
            let token = self.advance().clone();
            match token.kind {
                TokenKind::LBrace => depth += 1,
                TokenKind::RBrace => {
                    depth -= 1;
                    if depth == 0 {
                        close_brace_span = Some(token.span.clone());
                        break;
                    }
                }
                _ => {}
            }
        }

        if depth > 0 {
            return Err(self.make_error(ParseError::UnclosedBrace { span: open_brace.span.clone() }));
        }

        let close_brace = close_brace_span.unwrap();
        let content_end = close_brace.start; // End right before the closing brace

        // Extract the raw source text between braces, preserving comments
        let code = if content_end > content_start && content_end <= self.source.len() {
            self.source[content_start..content_end].to_string()
        } else {
            String::new()
        };

        // Compute the span covering the actual code content
        let code_span = Span {
            start: content_start,
            end: content_end,
            line: open_brace.span.line,
            col: open_brace.span.col + 1,
        };

        Ok(CodeBlock::new(code, true, code_span))
    }

    /// Parse inline code (single line) or block code
    fn parse_inline_or_block_code(&mut self) -> Result<CodeBlock> {
        if self.check(TokenKind::LBrace) {
            self.parse_code_block()
        } else {
            // Single line - collect tokens until we hit a newline-ish boundary
            // For simplicity, we'll collect until we see a language keyword,
            // closing brace, or known property keyword
            let mut code_tokens = Vec::new();

            while !self.is_at_end() {
                let token = self.peek();

                // Stop conditions for inline code - all keywords that could follow
                if matches!(
                    token.kind,
                    TokenKind::RBrace |
                        TokenKind::Go |
                        TokenKind::Ts |
                        TokenKind::TypeScript |
                        TokenKind::Rust |
                        TokenKind::Python |
                        TokenKind::Description |
                        TokenKind::Iterations |
                        TokenKind::Warmup |
                        TokenKind::Timeout |
                        TokenKind::Tags |
                        TokenKind::Skip |
                        TokenKind::Validate |
                        TokenKind::Mode |
                        TokenKind::Sink |
                        TokenKind::TargetTime |
                        TokenKind::MinIterations |
                        TokenKind::MaxIterations |
                        TokenKind::OutlierDetection |
                        TokenKind::CvThreshold |
                        TokenKind::Memory |
                        TokenKind::Concurrency |
                        TokenKind::Before |
                        TokenKind::After |
                        TokenKind::Each |
                        TokenKind::Hex |
                        TokenKind::Shape |
                        TokenKind::Bench |
                        TokenKind::Setup |
                        TokenKind::Fixture
                ) {
                    break;
                }

                // Check if identifier might be a language
                if let TokenKind::Identifier(s) = &token.kind {
                    if Lang::from_str(s).is_some() {
                        break;
                    }
                }

                code_tokens.push(self.advance().clone());
            }

            let code = self.reconstruct_code(&code_tokens);

            // Compute span covering all collected tokens
            let code_span = if code_tokens.is_empty() {
                Span::dummy()
            } else {
                let first = &code_tokens[0];
                let last = &code_tokens[code_tokens.len() - 1];
                Span {
                    start: first.span.start,
                    end: last.span.end,
                    line: first.span.line,
                    col: first.span.col,
                }
            };

            Ok(CodeBlock::new(code.trim().to_string(), false, code_span))
        }
    }

    /// Reconstruct code string from tokens by extracting from original source
    fn reconstruct_code(&self, tokens: &[Token]) -> String {
        if tokens.is_empty() {
            return String::new();
        }

        // Find the span covering all tokens
        let first_span = &tokens.first().unwrap().span;
        let last_span = &tokens.last().unwrap().span;

        // Extract the original source text between these spans
        let start = first_span.start;
        let end = last_span.end;

        if end <= self.source.len() {
            self.source[start..end].to_string()
        } else {
            // Fallback to token-based reconstruction if spans are invalid
            tokens.iter().map(|t| t.lexeme.as_str()).collect::<Vec<_>>().join(" ")
        }
    }

    // ========== Helper methods ==========

    fn peek(&self) -> &Token {
        self.tokens
            .get(self.current)
            .unwrap_or_else(|| self.tokens.last().expect("tokens should have at least EOF"))
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current.saturating_sub(1)]
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Eof)
    }

    fn check(&self, kind: TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(&kind)
    }

    /// Check if current token is an identifier with a specific name
    fn check_identifier(&self, name: &str) -> bool {
        if self.is_at_end() {
            return false;
        }
        matches!(&self.peek().kind, TokenKind::Identifier(s) if s == name)
    }

    /// Check if current token is a language keyword (go, ts, typescript, rust, python)
    fn peek_is_lang(&self) -> bool {
        if self.is_at_end() {
            return false;
        }
        let token = self.peek();
        match &token.kind {
            TokenKind::Go |
            TokenKind::Ts |
            TokenKind::TypeScript |
            TokenKind::Rust |
            TokenKind::Python => true,
            TokenKind::Identifier(s) => Lang::from_str(s).is_some(),
            _ => false,
        }
    }

    fn expect(&mut self, expected: TokenKind) -> Result<Token> {
        if self.check(expected.clone()) {
            Ok(self.advance().clone())
        } else {
            let token = self.peek().clone();
            Err(self.make_error(ParseError::ExpectedToken {
                expected: format!("{:?}", expected),
                found: format!("{:?}", token.kind),
                span: token.span,
            }))
        }
    }

    fn expect_keyword(&mut self, expected: TokenKind) -> Result<Token> {
        self.expect(expected)
    }

    fn expect_identifier(&mut self) -> Result<Token> {
        let token = self.peek().clone();
        if matches!(token.kind, TokenKind::Identifier(_)) {
            Ok(self.advance().clone())
        } else {
            Err(self.make_error(ParseError::ExpectedIdentifier { span: token.span }))
        }
    }

    fn expect_string(&mut self) -> Result<String> {
        let token = self.peek().clone();
        if let TokenKind::String(s) = &token.kind {
            let s = s.clone();
            self.advance();
            Ok(s)
        } else {
            Err(self.make_error(ParseError::ExpectedToken {
                expected: "string".to_string(),
                found: format!("{:?}", token.kind),
                span: token.span,
            }))
        }
    }

    fn expect_lang(&mut self) -> Result<Lang> {
        let token = self.peek().clone();
        let lang = match &token.kind {
            TokenKind::Go => Some(Lang::Go),
            TokenKind::Ts | TokenKind::TypeScript => Some(Lang::TypeScript),
            TokenKind::Rust => Some(Lang::Rust),
            TokenKind::Python => Some(Lang::Python),
            TokenKind::Identifier(s) => Lang::from_str(s),
            _ => None,
        };

        match lang {
            Some(l) => {
                self.advance();
                Ok(l)
            }
            None => Err(self.make_error(ParseError::UnknownLang {
                lang: token.lexeme.clone(),
                span: token.span,
            })),
        }
    }

    /// Parse a lang from a string literal (for baseline: "go")
    fn expect_lang_from_string(&mut self) -> Result<Lang> {
        let s = self.expect_string()?;
        Lang::from_str(&s).ok_or_else(|| {
            self.make_error(ParseError::UnknownLang {
                lang: s.clone(),
                span: self.previous().span.clone(),
            })
        })
    }

    /// Expect a duration token and return milliseconds
    fn expect_duration(&mut self) -> Result<u64> {
        let token = self.peek().clone();
        match token.kind {
            TokenKind::Duration(ms) => {
                self.advance();
                Ok(ms)
            }
            TokenKind::Number(n) => {
                // Allow plain numbers (interpreted as ms)
                self.advance();
                Ok(n)
            }
            _ => Err(self.make_error(ParseError::ExpectedToken {
                expected: "duration (e.g., 30s, 500ms, 1m)".to_string(),
                found: format!("{:?}", token.kind),
                span: token.span,
            })),
        }
    }

    /// Expect an execution order identifier
    fn expect_execution_order(&mut self) -> Result<ExecutionOrder> {
        let token = self.peek().clone();
        match &token.kind {
            TokenKind::Identifier(s) => ExecutionOrder::from_str(s)
                .map(|order| {
                    self.advance();
                    order
                })
                .ok_or_else(|| {
                    self.make_error(ParseError::ExpectedToken {
                        expected: "execution order (sequential, parallel, random)".to_string(),
                        found: s.clone(),
                        span: token.span.clone(),
                    })
                }),
            _ => Err(self.make_error(ParseError::ExpectedToken {
                expected: "execution order (sequential, parallel, random)".to_string(),
                found: format!("{:?}", token.kind),
                span: token.span,
            })),
        }
    }

    /// Expect a benchmark mode (auto/fixed) from string or identifier
    fn expect_bench_mode(&mut self) -> Result<BenchMode> {
        let token = self.peek().clone();
        match &token.kind {
            TokenKind::String(s) => BenchMode::from_str(s)
                .map(|mode| {
                    self.advance();
                    mode
                })
                .ok_or_else(|| {
                    self.make_error(ParseError::ExpectedToken {
                        expected: "benchmark mode (\"auto\" or \"fixed\")".to_string(),
                        found: s.clone(),
                        span: token.span.clone(),
                    })
                }),
            TokenKind::Identifier(s) => BenchMode::from_str(s)
                .map(|mode| {
                    self.advance();
                    mode
                })
                .ok_or_else(|| {
                    self.make_error(ParseError::ExpectedToken {
                        expected: "benchmark mode (auto or fixed)".to_string(),
                        found: s.clone(),
                        span: token.span.clone(),
                    })
                }),
            _ => Err(self.make_error(ParseError::ExpectedToken {
                expected: "benchmark mode (auto or fixed)".to_string(),
                found: format!("{:?}", token.kind),
                span: token.span,
            })),
        }
    }

    /// Parse a string array: ["foo", "bar"]
    fn parse_string_array(&mut self) -> Result<Vec<String>> {
        self.expect(TokenKind::LBracket)?;
        let mut items = Vec::new();

        while !self.check(TokenKind::RBracket) && !self.is_at_end() {
            items.push(self.expect_string()?);

            // Optional comma
            if !self.check(TokenKind::RBracket) {
                if self.check(TokenKind::Comma) {
                    self.advance();
                }
            }
        }

        self.expect(TokenKind::RBracket)?;
        Ok(items)
    }

    /// Parse a language array: ["go", "ts"]
    fn parse_lang_array(&mut self) -> Result<Vec<Lang>> {
        self.expect(TokenKind::LBracket)?;
        let mut items = Vec::new();

        while !self.check(TokenKind::RBracket) && !self.is_at_end() {
            let s = self.expect_string()?;
            let lang = Lang::from_str(&s).ok_or_else(|| {
                self.make_error(ParseError::UnknownLang {
                    lang: s.clone(),
                    span: self.previous().span.clone(),
                })
            })?;
            items.push(lang);

            // Optional comma
            if !self.check(TokenKind::RBracket) {
                if self.check(TokenKind::Comma) {
                    self.advance();
                }
            }
        }

        self.expect(TokenKind::RBracket)?;
        Ok(items)
    }

    fn make_error(&self, error: ParseError) -> Report {
        Report::new(error)
            .with_source_code(NamedSource::new(self.filename.clone(), self.source.clone()))
    }
}

/// Parse source code into an AST
pub fn parse(source: &str, filename: &str) -> Result<File> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().map_err(|e| {
        Report::new(e).with_source_code(NamedSource::new(filename, source.to_string()))
    })?;

    let mut parser = Parser::new(tokens, filename.to_string(), source.to_string());
    parser.parse_file()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_suite() {
        let source = r#"
suite hash {
    description: "Hash benchmarks"
    iterations: 5000
    
    bench keccak256 {
        go: hash.Keccak256(data)
        ts: keccak256(data)
    }
}
"#;
        let result = parse(source, "test.bench");
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());

        let file = result.unwrap();
        assert_eq!(file.suites.len(), 1);

        let suite = &file.suites[0];
        assert_eq!(suite.name, "hash");
        assert_eq!(suite.description, Some("Hash benchmarks".to_string()));
        assert_eq!(suite.iterations, Some(5000));
        assert_eq!(suite.benchmarks.len(), 1);

        let bench = &suite.benchmarks[0];
        assert_eq!(bench.name, "keccak256");
        assert!(bench.implementations.contains_key(&Lang::Go));
        assert!(bench.implementations.contains_key(&Lang::TypeScript));
    }

    #[test]
    fn test_parse_fixture() {
        let source = r#"
suite test {
    fixture data {
        hex: "deadbeef"
    }
    
    bench foo {
        go: test(data)
    }
}
"#;
        let result = parse(source, "test.bench");
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());

        let file = result.unwrap();
        let suite = &file.suites[0];
        assert_eq!(suite.fixtures.len(), 1);
        assert_eq!(suite.fixtures[0].name, "data");
        assert_eq!(suite.fixtures[0].hex_data, Some("deadbeef".to_string()));
    }

    #[test]
    fn test_parse_setup() {
        let source = r#"
suite test {
    setup go {
        import (
            "testing"
        )
        
        init {
            // setup code
        }
    }
    
    setup ts {
        import {
            import { foo } from 'bar'
        }
        
        init {
            // ts setup
        }
    }
    
    bench foo {
        go: test()
    }
}
"#;
        let result = parse(source, "test.bench");
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());

        let file = result.unwrap();
        let suite = &file.suites[0];
        assert!(suite.setups.contains_key(&Lang::Go));
        assert!(suite.setups.contains_key(&Lang::TypeScript));

        // Verify structured setup was parsed
        let go_setup = suite.setups.get(&Lang::Go).unwrap();
        assert!(go_setup.imports.is_some());
        assert!(go_setup.init.is_some());

        let ts_setup = suite.setups.get(&Lang::TypeScript).unwrap();
        assert!(ts_setup.imports.is_some());
        assert!(ts_setup.init.is_some());
    }

    #[test]
    fn test_parse_structured_setup_all_sections() {
        let source = r#"
suite test {
    setup go {
        import (
            "fmt"
            "github.com/pkg/errors"
        )
        
        declare {
            var globalState *State
        }
        
        init {
            globalState = NewState()
        }
        
        helpers {
            func doSomething() {
                // helper
            }
        }
    }
    
    bench foo {
        go: doSomething()
    }
}
"#;
        let result = parse(source, "test.bench");
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());

        let file = result.unwrap();
        let suite = &file.suites[0];
        let go_setup = suite.setups.get(&Lang::Go).unwrap();

        assert!(go_setup.imports.is_some());
        assert!(go_setup.declarations.is_some());
        assert!(go_setup.init.is_some());
        assert!(go_setup.helpers.is_some());
    }

    #[test]
    fn test_parse_benchmark_with_hooks() {
        let source = r#"
suite test {
    setup go {
        import ("fmt")
        init {}
    }
    
    bench with_hooks {
        iterations: 100
        warmup: 10
        timeout: 30s
        tags: ["performance", "critical"]
        
        before: {
            go: resetState()
        }
        
        after: {
            go: cleanup()
        }
        
        each: {
            go: prepareIteration()
        }
        
        go: runBenchmark()
    }
}
"#;
        let result = parse(source, "test.bench");
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());

        let file = result.unwrap();
        let bench = &file.suites[0].benchmarks[0];

        assert_eq!(bench.iterations, Some(100));
        assert_eq!(bench.warmup, Some(10));
        assert_eq!(bench.timeout, Some(30000)); // 30s in ms
        assert_eq!(bench.tags, vec!["performance", "critical"]);
        assert!(bench.before.contains_key(&Lang::Go));
        assert!(bench.after.contains_key(&Lang::Go));
        assert!(bench.each.contains_key(&Lang::Go));
    }

    #[test]
    fn test_parse_suite_config() {
        let source = r#"
suite test {
    description: "Test suite"
    iterations: 1000
    warmup: 100
    timeout: 60s
    requires: ["go", "ts"]
    order: sequential
    compare: true
    baseline: "go"
    
    setup go {
        import ("fmt")
        init {}
    }
    
    bench foo {
        go: test()
    }
}
"#;
        let result = parse(source, "test.bench");
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());

        let file = result.unwrap();
        let suite = &file.suites[0];

        assert_eq!(suite.timeout, Some(60000)); // 60s in ms
        assert_eq!(suite.requires.len(), 2);
        assert!(suite.compare);
        assert_eq!(suite.baseline, Some(Lang::Go));
    }

    #[test]
    fn test_parse_use_std() {
        let source = r#"
use std::constants

suite test {
    bench foo {
        go: compute(std_PI)
    }
}
"#;
        let result = parse(source, "test.bench");
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());

        let file = result.unwrap();
        assert_eq!(file.use_stds.len(), 1);
        assert_eq!(file.use_stds[0].module, "constants");
        assert_eq!(file.suites.len(), 1);
    }

    #[test]
    fn test_parse_multiple_use_stds() {
        let source = r#"
use std::constants
use std::math

suite test {
    bench foo {
        go: compute(std_PI)
    }
}
"#;
        let result = parse(source, "test.bench");
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());

        let file = result.unwrap();
        assert_eq!(file.use_stds.len(), 2);
        assert_eq!(file.use_stds[0].module, "constants");
        assert_eq!(file.use_stds[1].module, "math");
    }

    #[test]
    fn test_parse_namespaced_global_setup() {
        let source = r#"
use std::anvil

globalSetup {
    anvil.spawnAnvil()
}

suite test {
    bench foo {
        go: test()
    }
}
"#;
        let result = parse(source, "test.bench");
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());

        let file = result.unwrap();
        assert!(file.global_setup.is_some());
        let gs = file.global_setup.unwrap();
        assert!(gs.anvil_config.is_some());
    }

    #[test]
    fn test_parse_namespaced_global_setup_with_fork() {
        let source = r#"
use std::anvil

globalSetup {
    anvil.spawnAnvil(fork: "https://eth.example.com")
}

suite test {
    bench foo {
        go: test()
    }
}
"#;
        let result = parse(source, "test.bench");
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());

        let file = result.unwrap();
        assert!(file.global_setup.is_some());
        let gs = file.global_setup.unwrap();
        assert!(gs.anvil_config.is_some());
        let anvil = gs.anvil_config.unwrap();
        assert_eq!(anvil.fork_url, Some("https://eth.example.com".to_string()));
    }

    #[test]
    fn test_parse_use_std_with_multiple_suites() {
        let source = r#"
use std::constants

suite test1 {
    bench foo {
        go: compute(std_PI)
    }
}

suite test2 {
    bench bar {
        go: compute(std_E)
    }
}
"#;
        let result = parse(source, "test.bench");
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());

        let file = result.unwrap();
        assert_eq!(file.use_stds.len(), 1);
        assert_eq!(file.suites.len(), 2);
    }

    #[test]
    fn test_parse_global_setup_inside_suite() {
        let source = r#"
use std::anvil

suite evmTest {
    description: "EVM benchmarks"
    
    globalSetup {
        anvil.spawnAnvil()
    }
    
    bench rpcCall {
        go: callRpc()
    }
}
"#;
        let result = parse(source, "test.bench");
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());

        let file = result.unwrap();
        assert_eq!(file.suites.len(), 1);

        let suite = &file.suites[0];
        assert_eq!(suite.name, "evmTest");
        assert!(suite.global_setup.is_some());

        let gs = suite.global_setup.as_ref().unwrap();
        assert!(gs.anvil_config.is_some());
    }

    #[test]
    fn test_parse_global_setup_inside_suite_with_fork() {
        let source = r#"
use std::anvil

suite evmTest {
    globalSetup {
        anvil.spawnAnvil(fork: "https://mainnet.infura.io")
    }
    
    bench rpcCall {
        go: callRpc()
    }
}
"#;
        let result = parse(source, "test.bench");
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());

        let file = result.unwrap();
        let suite = &file.suites[0];
        let gs = suite.global_setup.as_ref().unwrap();
        let anvil = gs.anvil_config.as_ref().unwrap();
        assert_eq!(anvil.fork_url, Some("https://mainnet.infura.io".to_string()));
    }

    #[test]
    fn test_parse_file_level_global_setup_inherited() {
        // File-level globalSetup should be inherited by suites without their own
        let source = r#"
use std::anvil

globalSetup {
    anvil.spawnAnvil()
}

suite test1 {
    bench foo {
        go: test()
    }
}

suite test2 {
    bench bar {
        go: test()
    }
}
"#;
        let result = parse(source, "test.bench");
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());

        let file = result.unwrap();
        assert_eq!(file.suites.len(), 2);

        // Both suites should have inherited the file-level globalSetup
        assert!(file.suites[0].global_setup.is_some());
        assert!(file.suites[1].global_setup.is_some());
    }

    #[test]
    fn test_parse_suite_level_global_setup_overrides() {
        // Suite-level globalSetup should override file-level
        let source = r#"
use std::anvil

globalSetup {
    anvil.spawnAnvil()
}

suite test1 {
    globalSetup {
        anvil.spawnAnvil(fork: "https://custom.rpc")
    }
    
    bench foo {
        go: test()
    }
}
"#;
        let result = parse(source, "test.bench");
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());

        let file = result.unwrap();
        let suite = &file.suites[0];
        let gs = suite.global_setup.as_ref().unwrap();
        let anvil = gs.anvil_config.as_ref().unwrap();

        // Suite's own globalSetup should be used (with fork)
        assert_eq!(anvil.fork_url, Some("https://custom.rpc".to_string()));
    }
}
