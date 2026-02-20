#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 221
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 221
#define ALIAS_COUNT 0
#define TOKEN_COUNT 144
#define EXTERNAL_TOKEN_COUNT 2
#define FIELD_COUNT 6
#define MAX_ALIAS_SEQUENCE_LENGTH 6
#define PRODUCTION_ID_COUNT 8

enum ts_symbol_identifiers {
  sym_identifier = 1,
  anon_sym_use = 2,
  anon_sym_std = 3,
  anon_sym_COLON_COLON = 4,
  anon_sym_globalSetup = 5,
  anon_sym_LBRACE = 6,
  anon_sym_RBRACE = 7,
  anon_sym_anvil = 8,
  anon_sym_DOT = 9,
  anon_sym_spawnAnvil = 10,
  anon_sym_LPAREN = 11,
  anon_sym_RPAREN = 12,
  anon_sym_fork = 13,
  anon_sym_COLON = 14,
  anon_sym_COMMA = 15,
  anon_sym_suite = 16,
  anon_sym_setup = 17,
  anon_sym_import = 18,
  anon_sym_declare = 19,
  anon_sym_async = 20,
  anon_sym_init = 21,
  anon_sym_helpers = 22,
  anon_sym_fixture = 23,
  anon_sym_hex = 24,
  anon_sym_shape = 25,
  anon_sym_ATfile = 26,
  anon_sym_bench = 27,
  anon_sym_tags = 28,
  anon_sym_skip = 29,
  anon_sym_validate = 30,
  anon_sym_before = 31,
  anon_sym_after = 32,
  anon_sym_each = 33,
  anon_sym_charting = 34,
  anon_sym_drawBarChart = 35,
  anon_sym_drawLineChart = 36,
  anon_sym_drawPieChart = 37,
  anon_sym_drawScatterPlot = 38,
  anon_sym_drawHistogram = 39,
  anon_sym_drawHeatmap = 40,
  anon_sym_drawBoxPlot = 41,
  anon_sym_drawAreaChart = 42,
  anon_sym_drawSpeedupChart = 43,
  anon_sym_drawScalingChart = 44,
  anon_sym_drawTable = 45,
  anon_sym_title = 46,
  anon_sym_description = 47,
  anon_sym_xlabel = 48,
  anon_sym_ylabel = 49,
  anon_sym_output = 50,
  anon_sym_sortBy = 51,
  anon_sym_sortOrder = 52,
  anon_sym_timeUnit = 53,
  anon_sym_legendPosition = 54,
  anon_sym_regressionStyle = 55,
  anon_sym_regressionModel = 56,
  anon_sym_yScale = 57,
  anon_sym_baselineBenchmark = 58,
  anon_sym_filterWinner = 59,
  anon_sym_showStats = 60,
  anon_sym_showConfig = 61,
  anon_sym_showWinCounts = 62,
  anon_sym_showGeoMean = 63,
  anon_sym_showDistribution = 64,
  anon_sym_showMemory = 65,
  anon_sym_showTotalTime = 66,
  anon_sym_showLegend = 67,
  anon_sym_showGrid = 68,
  anon_sym_showErrorBars = 69,
  anon_sym_showRegression = 70,
  anon_sym_showRegressionLabel = 71,
  anon_sym_showRSquared = 72,
  anon_sym_showEquation = 73,
  anon_sym_showRegressionBand = 74,
  anon_sym_showMinorGrid = 75,
  anon_sym_showVerticalGrid = 76,
  anon_sym_showStdDevBand = 77,
  anon_sym_roundTicks = 78,
  anon_sym_compact = 79,
  anon_sym_width = 80,
  anon_sym_height = 81,
  anon_sym_precision = 82,
  anon_sym_limit = 83,
  anon_sym_titleFontSize = 84,
  anon_sym_subtitleFontSize = 85,
  anon_sym_axisLabelFontSize = 86,
  anon_sym_tickLabelFontSize = 87,
  anon_sym_barGroupGap = 88,
  anon_sym_barWithinGroupGap = 89,
  anon_sym_barWidth = 90,
  anon_sym_ciLevel = 91,
  anon_sym_minSpeedup = 92,
  anon_sym_axisThickness = 93,
  anon_sym_yAxisMin = 94,
  anon_sym_yAxisMax = 95,
  anon_sym_gridOpacity = 96,
  anon_sym_minorGridOpacity = 97,
  anon_sym_errorBarOpacity = 98,
  anon_sym_errorBarThickness = 99,
  anon_sym_regressionBandOpacity = 100,
  anon_sym_symlogThreshold = 101,
  anon_sym_includeBenchmarks = 102,
  anon_sym_excludeBenchmarks = 103,
  anon_sym_iterations = 104,
  anon_sym_warmup = 105,
  anon_sym_timeout = 106,
  anon_sym_requires = 107,
  anon_sym_order = 108,
  anon_sym_compare = 109,
  anon_sym_baseline = 110,
  anon_sym_mode = 111,
  anon_sym_targetTime = 112,
  anon_sym_minIterations = 113,
  anon_sym_maxIterations = 114,
  anon_sym_sink = 115,
  anon_sym_outlierDetection = 116,
  anon_sym_cvThreshold = 117,
  anon_sym_count = 118,
  anon_sym_memory = 119,
  anon_sym_concurrency = 120,
  anon_sym_go = 121,
  anon_sym_ts = 122,
  anon_sym_typescript = 123,
  anon_sym_rust = 124,
  anon_sym_python = 125,
  sym_inline_code = 126,
  anon_sym_DQUOTE = 127,
  anon_sym_SQUOTE = 128,
  aux_sym_string_content_token1 = 129,
  aux_sym_single_string_content_token1 = 130,
  sym_escape_sequence = 131,
  sym_number = 132,
  sym_float = 133,
  anon_sym_ms = 134,
  anon_sym_s = 135,
  anon_sym_m = 136,
  anon_sym_true = 137,
  anon_sym_false = 138,
  anon_sym_LBRACK = 139,
  anon_sym_RBRACK = 140,
  sym_comment = 141,
  sym_embedded_code = 142,
  sym__embedded_code_start = 143,
  sym_source_file = 144,
  sym_use_statement = 145,
  sym_global_setup = 146,
  sym_global_setup_body = 147,
  sym_global_setup_statement = 148,
  sym_anvil_call = 149,
  sym_anvil_args = 150,
  sym_function_call = 151,
  sym_argument_list = 152,
  sym_argument = 153,
  sym_suite = 154,
  sym_suite_body = 155,
  sym__suite_item = 156,
  sym_setup_block = 157,
  sym_setup_body = 158,
  sym__setup_section = 159,
  sym_import_section = 160,
  sym_declare_section = 161,
  sym_init_section = 162,
  sym_helpers_section = 163,
  sym_fixture = 164,
  sym_fixture_params = 165,
  sym_fixture_param = 166,
  sym_fixture_body = 167,
  sym__fixture_item = 168,
  sym_hex_property = 169,
  sym_shape_property = 170,
  sym_file_ref = 171,
  sym_benchmark = 172,
  sym_benchmark_body = 173,
  sym__benchmark_item = 174,
  sym_tags_property = 175,
  sym_skip_hook = 176,
  sym_validate_hook = 177,
  sym_before_hook = 178,
  sym_after_hook = 179,
  sym_each_hook = 180,
  sym_hook_flat = 181,
  sym_hook_grouped = 182,
  sym_after_block = 183,
  sym_after_body = 184,
  sym_chart_directive = 185,
  sym_chart_function_name = 186,
  sym_chart_params = 187,
  sym_chart_param = 188,
  sym_chart_param_name = 189,
  sym__chart_value = 190,
  sym_property = 191,
  sym_property_name = 192,
  sym__value = 193,
  sym_language_implementation = 194,
  sym_language_tag = 195,
  sym__code_or_inline = 196,
  sym_code_block = 197,
  sym_paren_code_block = 198,
  sym_string = 199,
  sym_string_content = 200,
  sym_single_string_content = 201,
  sym_duration = 202,
  sym_duration_unit = 203,
  sym_boolean = 204,
  sym_string_array = 205,
  aux_sym_source_file_repeat1 = 206,
  aux_sym_source_file_repeat2 = 207,
  aux_sym_global_setup_body_repeat1 = 208,
  aux_sym_argument_list_repeat1 = 209,
  aux_sym_suite_body_repeat1 = 210,
  aux_sym_setup_body_repeat1 = 211,
  aux_sym_fixture_params_repeat1 = 212,
  aux_sym_fixture_body_repeat1 = 213,
  aux_sym_benchmark_body_repeat1 = 214,
  aux_sym_hook_grouped_repeat1 = 215,
  aux_sym_after_body_repeat1 = 216,
  aux_sym_chart_params_repeat1 = 217,
  aux_sym_string_content_repeat1 = 218,
  aux_sym_single_string_content_repeat1 = 219,
  aux_sym_string_array_repeat1 = 220,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_identifier] = "identifier",
  [anon_sym_use] = "use",
  [anon_sym_std] = "std",
  [anon_sym_COLON_COLON] = "::",
  [anon_sym_globalSetup] = "globalSetup",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_anvil] = "anvil",
  [anon_sym_DOT] = ".",
  [anon_sym_spawnAnvil] = "spawnAnvil",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_fork] = "fork",
  [anon_sym_COLON] = ":",
  [anon_sym_COMMA] = ",",
  [anon_sym_suite] = "suite",
  [anon_sym_setup] = "setup",
  [anon_sym_import] = "import",
  [anon_sym_declare] = "declare",
  [anon_sym_async] = "async",
  [anon_sym_init] = "init",
  [anon_sym_helpers] = "helpers",
  [anon_sym_fixture] = "fixture",
  [anon_sym_hex] = "hex",
  [anon_sym_shape] = "shape",
  [anon_sym_ATfile] = "@file",
  [anon_sym_bench] = "bench",
  [anon_sym_tags] = "tags",
  [anon_sym_skip] = "skip",
  [anon_sym_validate] = "validate",
  [anon_sym_before] = "before",
  [anon_sym_after] = "after",
  [anon_sym_each] = "each",
  [anon_sym_charting] = "charting",
  [anon_sym_drawBarChart] = "drawBarChart",
  [anon_sym_drawLineChart] = "drawLineChart",
  [anon_sym_drawPieChart] = "drawPieChart",
  [anon_sym_drawScatterPlot] = "drawScatterPlot",
  [anon_sym_drawHistogram] = "drawHistogram",
  [anon_sym_drawHeatmap] = "drawHeatmap",
  [anon_sym_drawBoxPlot] = "drawBoxPlot",
  [anon_sym_drawAreaChart] = "drawAreaChart",
  [anon_sym_drawSpeedupChart] = "drawSpeedupChart",
  [anon_sym_drawScalingChart] = "drawScalingChart",
  [anon_sym_drawTable] = "drawTable",
  [anon_sym_title] = "title",
  [anon_sym_description] = "description",
  [anon_sym_xlabel] = "xlabel",
  [anon_sym_ylabel] = "ylabel",
  [anon_sym_output] = "output",
  [anon_sym_sortBy] = "sortBy",
  [anon_sym_sortOrder] = "sortOrder",
  [anon_sym_timeUnit] = "timeUnit",
  [anon_sym_legendPosition] = "legendPosition",
  [anon_sym_regressionStyle] = "regressionStyle",
  [anon_sym_regressionModel] = "regressionModel",
  [anon_sym_yScale] = "yScale",
  [anon_sym_baselineBenchmark] = "baselineBenchmark",
  [anon_sym_filterWinner] = "filterWinner",
  [anon_sym_showStats] = "showStats",
  [anon_sym_showConfig] = "showConfig",
  [anon_sym_showWinCounts] = "showWinCounts",
  [anon_sym_showGeoMean] = "showGeoMean",
  [anon_sym_showDistribution] = "showDistribution",
  [anon_sym_showMemory] = "showMemory",
  [anon_sym_showTotalTime] = "showTotalTime",
  [anon_sym_showLegend] = "showLegend",
  [anon_sym_showGrid] = "showGrid",
  [anon_sym_showErrorBars] = "showErrorBars",
  [anon_sym_showRegression] = "showRegression",
  [anon_sym_showRegressionLabel] = "showRegressionLabel",
  [anon_sym_showRSquared] = "showRSquared",
  [anon_sym_showEquation] = "showEquation",
  [anon_sym_showRegressionBand] = "showRegressionBand",
  [anon_sym_showMinorGrid] = "showMinorGrid",
  [anon_sym_showVerticalGrid] = "showVerticalGrid",
  [anon_sym_showStdDevBand] = "showStdDevBand",
  [anon_sym_roundTicks] = "roundTicks",
  [anon_sym_compact] = "compact",
  [anon_sym_width] = "width",
  [anon_sym_height] = "height",
  [anon_sym_precision] = "precision",
  [anon_sym_limit] = "limit",
  [anon_sym_titleFontSize] = "titleFontSize",
  [anon_sym_subtitleFontSize] = "subtitleFontSize",
  [anon_sym_axisLabelFontSize] = "axisLabelFontSize",
  [anon_sym_tickLabelFontSize] = "tickLabelFontSize",
  [anon_sym_barGroupGap] = "barGroupGap",
  [anon_sym_barWithinGroupGap] = "barWithinGroupGap",
  [anon_sym_barWidth] = "barWidth",
  [anon_sym_ciLevel] = "ciLevel",
  [anon_sym_minSpeedup] = "minSpeedup",
  [anon_sym_axisThickness] = "axisThickness",
  [anon_sym_yAxisMin] = "yAxisMin",
  [anon_sym_yAxisMax] = "yAxisMax",
  [anon_sym_gridOpacity] = "gridOpacity",
  [anon_sym_minorGridOpacity] = "minorGridOpacity",
  [anon_sym_errorBarOpacity] = "errorBarOpacity",
  [anon_sym_errorBarThickness] = "errorBarThickness",
  [anon_sym_regressionBandOpacity] = "regressionBandOpacity",
  [anon_sym_symlogThreshold] = "symlogThreshold",
  [anon_sym_includeBenchmarks] = "includeBenchmarks",
  [anon_sym_excludeBenchmarks] = "excludeBenchmarks",
  [anon_sym_iterations] = "iterations",
  [anon_sym_warmup] = "warmup",
  [anon_sym_timeout] = "timeout",
  [anon_sym_requires] = "requires",
  [anon_sym_order] = "order",
  [anon_sym_compare] = "compare",
  [anon_sym_baseline] = "baseline",
  [anon_sym_mode] = "mode",
  [anon_sym_targetTime] = "targetTime",
  [anon_sym_minIterations] = "minIterations",
  [anon_sym_maxIterations] = "maxIterations",
  [anon_sym_sink] = "sink",
  [anon_sym_outlierDetection] = "outlierDetection",
  [anon_sym_cvThreshold] = "cvThreshold",
  [anon_sym_count] = "count",
  [anon_sym_memory] = "memory",
  [anon_sym_concurrency] = "concurrency",
  [anon_sym_go] = "go",
  [anon_sym_ts] = "ts",
  [anon_sym_typescript] = "typescript",
  [anon_sym_rust] = "rust",
  [anon_sym_python] = "python",
  [sym_inline_code] = "inline_code",
  [anon_sym_DQUOTE] = "\"",
  [anon_sym_SQUOTE] = "'",
  [aux_sym_string_content_token1] = "string_content_token1",
  [aux_sym_single_string_content_token1] = "single_string_content_token1",
  [sym_escape_sequence] = "escape_sequence",
  [sym_number] = "number",
  [sym_float] = "float",
  [anon_sym_ms] = "ms",
  [anon_sym_s] = "s",
  [anon_sym_m] = "m",
  [anon_sym_true] = "true",
  [anon_sym_false] = "false",
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
  [sym_comment] = "comment",
  [sym_embedded_code] = "embedded_code",
  [sym__embedded_code_start] = "_embedded_code_start",
  [sym_source_file] = "source_file",
  [sym_use_statement] = "use_statement",
  [sym_global_setup] = "global_setup",
  [sym_global_setup_body] = "global_setup_body",
  [sym_global_setup_statement] = "global_setup_statement",
  [sym_anvil_call] = "anvil_call",
  [sym_anvil_args] = "anvil_args",
  [sym_function_call] = "function_call",
  [sym_argument_list] = "argument_list",
  [sym_argument] = "argument",
  [sym_suite] = "suite",
  [sym_suite_body] = "suite_body",
  [sym__suite_item] = "_suite_item",
  [sym_setup_block] = "setup_block",
  [sym_setup_body] = "setup_body",
  [sym__setup_section] = "_setup_section",
  [sym_import_section] = "import_section",
  [sym_declare_section] = "declare_section",
  [sym_init_section] = "init_section",
  [sym_helpers_section] = "helpers_section",
  [sym_fixture] = "fixture",
  [sym_fixture_params] = "fixture_params",
  [sym_fixture_param] = "fixture_param",
  [sym_fixture_body] = "fixture_body",
  [sym__fixture_item] = "_fixture_item",
  [sym_hex_property] = "hex_property",
  [sym_shape_property] = "shape_property",
  [sym_file_ref] = "file_ref",
  [sym_benchmark] = "benchmark",
  [sym_benchmark_body] = "benchmark_body",
  [sym__benchmark_item] = "_benchmark_item",
  [sym_tags_property] = "tags_property",
  [sym_skip_hook] = "skip_hook",
  [sym_validate_hook] = "validate_hook",
  [sym_before_hook] = "before_hook",
  [sym_after_hook] = "after_hook",
  [sym_each_hook] = "each_hook",
  [sym_hook_flat] = "hook_flat",
  [sym_hook_grouped] = "hook_grouped",
  [sym_after_block] = "after_block",
  [sym_after_body] = "after_body",
  [sym_chart_directive] = "chart_directive",
  [sym_chart_function_name] = "chart_function_name",
  [sym_chart_params] = "chart_params",
  [sym_chart_param] = "chart_param",
  [sym_chart_param_name] = "chart_param_name",
  [sym__chart_value] = "_chart_value",
  [sym_property] = "property",
  [sym_property_name] = "property_name",
  [sym__value] = "_value",
  [sym_language_implementation] = "language_implementation",
  [sym_language_tag] = "language_tag",
  [sym__code_or_inline] = "_code_or_inline",
  [sym_code_block] = "code_block",
  [sym_paren_code_block] = "paren_code_block",
  [sym_string] = "string",
  [sym_string_content] = "string_content",
  [sym_single_string_content] = "single_string_content",
  [sym_duration] = "duration",
  [sym_duration_unit] = "duration_unit",
  [sym_boolean] = "boolean",
  [sym_string_array] = "string_array",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_source_file_repeat2] = "source_file_repeat2",
  [aux_sym_global_setup_body_repeat1] = "global_setup_body_repeat1",
  [aux_sym_argument_list_repeat1] = "argument_list_repeat1",
  [aux_sym_suite_body_repeat1] = "suite_body_repeat1",
  [aux_sym_setup_body_repeat1] = "setup_body_repeat1",
  [aux_sym_fixture_params_repeat1] = "fixture_params_repeat1",
  [aux_sym_fixture_body_repeat1] = "fixture_body_repeat1",
  [aux_sym_benchmark_body_repeat1] = "benchmark_body_repeat1",
  [aux_sym_hook_grouped_repeat1] = "hook_grouped_repeat1",
  [aux_sym_after_body_repeat1] = "after_body_repeat1",
  [aux_sym_chart_params_repeat1] = "chart_params_repeat1",
  [aux_sym_string_content_repeat1] = "string_content_repeat1",
  [aux_sym_single_string_content_repeat1] = "single_string_content_repeat1",
  [aux_sym_string_array_repeat1] = "string_array_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_identifier] = sym_identifier,
  [anon_sym_use] = anon_sym_use,
  [anon_sym_std] = anon_sym_std,
  [anon_sym_COLON_COLON] = anon_sym_COLON_COLON,
  [anon_sym_globalSetup] = anon_sym_globalSetup,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_anvil] = anon_sym_anvil,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_spawnAnvil] = anon_sym_spawnAnvil,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_fork] = anon_sym_fork,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_suite] = anon_sym_suite,
  [anon_sym_setup] = anon_sym_setup,
  [anon_sym_import] = anon_sym_import,
  [anon_sym_declare] = anon_sym_declare,
  [anon_sym_async] = anon_sym_async,
  [anon_sym_init] = anon_sym_init,
  [anon_sym_helpers] = anon_sym_helpers,
  [anon_sym_fixture] = anon_sym_fixture,
  [anon_sym_hex] = anon_sym_hex,
  [anon_sym_shape] = anon_sym_shape,
  [anon_sym_ATfile] = anon_sym_ATfile,
  [anon_sym_bench] = anon_sym_bench,
  [anon_sym_tags] = anon_sym_tags,
  [anon_sym_skip] = anon_sym_skip,
  [anon_sym_validate] = anon_sym_validate,
  [anon_sym_before] = anon_sym_before,
  [anon_sym_after] = anon_sym_after,
  [anon_sym_each] = anon_sym_each,
  [anon_sym_charting] = anon_sym_charting,
  [anon_sym_drawBarChart] = anon_sym_drawBarChart,
  [anon_sym_drawLineChart] = anon_sym_drawLineChart,
  [anon_sym_drawPieChart] = anon_sym_drawPieChart,
  [anon_sym_drawScatterPlot] = anon_sym_drawScatterPlot,
  [anon_sym_drawHistogram] = anon_sym_drawHistogram,
  [anon_sym_drawHeatmap] = anon_sym_drawHeatmap,
  [anon_sym_drawBoxPlot] = anon_sym_drawBoxPlot,
  [anon_sym_drawAreaChart] = anon_sym_drawAreaChart,
  [anon_sym_drawSpeedupChart] = anon_sym_drawSpeedupChart,
  [anon_sym_drawScalingChart] = anon_sym_drawScalingChart,
  [anon_sym_drawTable] = anon_sym_drawTable,
  [anon_sym_title] = anon_sym_title,
  [anon_sym_description] = anon_sym_description,
  [anon_sym_xlabel] = anon_sym_xlabel,
  [anon_sym_ylabel] = anon_sym_ylabel,
  [anon_sym_output] = anon_sym_output,
  [anon_sym_sortBy] = anon_sym_sortBy,
  [anon_sym_sortOrder] = anon_sym_sortOrder,
  [anon_sym_timeUnit] = anon_sym_timeUnit,
  [anon_sym_legendPosition] = anon_sym_legendPosition,
  [anon_sym_regressionStyle] = anon_sym_regressionStyle,
  [anon_sym_regressionModel] = anon_sym_regressionModel,
  [anon_sym_yScale] = anon_sym_yScale,
  [anon_sym_baselineBenchmark] = anon_sym_baselineBenchmark,
  [anon_sym_filterWinner] = anon_sym_filterWinner,
  [anon_sym_showStats] = anon_sym_showStats,
  [anon_sym_showConfig] = anon_sym_showConfig,
  [anon_sym_showWinCounts] = anon_sym_showWinCounts,
  [anon_sym_showGeoMean] = anon_sym_showGeoMean,
  [anon_sym_showDistribution] = anon_sym_showDistribution,
  [anon_sym_showMemory] = anon_sym_showMemory,
  [anon_sym_showTotalTime] = anon_sym_showTotalTime,
  [anon_sym_showLegend] = anon_sym_showLegend,
  [anon_sym_showGrid] = anon_sym_showGrid,
  [anon_sym_showErrorBars] = anon_sym_showErrorBars,
  [anon_sym_showRegression] = anon_sym_showRegression,
  [anon_sym_showRegressionLabel] = anon_sym_showRegressionLabel,
  [anon_sym_showRSquared] = anon_sym_showRSquared,
  [anon_sym_showEquation] = anon_sym_showEquation,
  [anon_sym_showRegressionBand] = anon_sym_showRegressionBand,
  [anon_sym_showMinorGrid] = anon_sym_showMinorGrid,
  [anon_sym_showVerticalGrid] = anon_sym_showVerticalGrid,
  [anon_sym_showStdDevBand] = anon_sym_showStdDevBand,
  [anon_sym_roundTicks] = anon_sym_roundTicks,
  [anon_sym_compact] = anon_sym_compact,
  [anon_sym_width] = anon_sym_width,
  [anon_sym_height] = anon_sym_height,
  [anon_sym_precision] = anon_sym_precision,
  [anon_sym_limit] = anon_sym_limit,
  [anon_sym_titleFontSize] = anon_sym_titleFontSize,
  [anon_sym_subtitleFontSize] = anon_sym_subtitleFontSize,
  [anon_sym_axisLabelFontSize] = anon_sym_axisLabelFontSize,
  [anon_sym_tickLabelFontSize] = anon_sym_tickLabelFontSize,
  [anon_sym_barGroupGap] = anon_sym_barGroupGap,
  [anon_sym_barWithinGroupGap] = anon_sym_barWithinGroupGap,
  [anon_sym_barWidth] = anon_sym_barWidth,
  [anon_sym_ciLevel] = anon_sym_ciLevel,
  [anon_sym_minSpeedup] = anon_sym_minSpeedup,
  [anon_sym_axisThickness] = anon_sym_axisThickness,
  [anon_sym_yAxisMin] = anon_sym_yAxisMin,
  [anon_sym_yAxisMax] = anon_sym_yAxisMax,
  [anon_sym_gridOpacity] = anon_sym_gridOpacity,
  [anon_sym_minorGridOpacity] = anon_sym_minorGridOpacity,
  [anon_sym_errorBarOpacity] = anon_sym_errorBarOpacity,
  [anon_sym_errorBarThickness] = anon_sym_errorBarThickness,
  [anon_sym_regressionBandOpacity] = anon_sym_regressionBandOpacity,
  [anon_sym_symlogThreshold] = anon_sym_symlogThreshold,
  [anon_sym_includeBenchmarks] = anon_sym_includeBenchmarks,
  [anon_sym_excludeBenchmarks] = anon_sym_excludeBenchmarks,
  [anon_sym_iterations] = anon_sym_iterations,
  [anon_sym_warmup] = anon_sym_warmup,
  [anon_sym_timeout] = anon_sym_timeout,
  [anon_sym_requires] = anon_sym_requires,
  [anon_sym_order] = anon_sym_order,
  [anon_sym_compare] = anon_sym_compare,
  [anon_sym_baseline] = anon_sym_baseline,
  [anon_sym_mode] = anon_sym_mode,
  [anon_sym_targetTime] = anon_sym_targetTime,
  [anon_sym_minIterations] = anon_sym_minIterations,
  [anon_sym_maxIterations] = anon_sym_maxIterations,
  [anon_sym_sink] = anon_sym_sink,
  [anon_sym_outlierDetection] = anon_sym_outlierDetection,
  [anon_sym_cvThreshold] = anon_sym_cvThreshold,
  [anon_sym_count] = anon_sym_count,
  [anon_sym_memory] = anon_sym_memory,
  [anon_sym_concurrency] = anon_sym_concurrency,
  [anon_sym_go] = anon_sym_go,
  [anon_sym_ts] = anon_sym_ts,
  [anon_sym_typescript] = anon_sym_typescript,
  [anon_sym_rust] = anon_sym_rust,
  [anon_sym_python] = anon_sym_python,
  [sym_inline_code] = sym_inline_code,
  [anon_sym_DQUOTE] = anon_sym_DQUOTE,
  [anon_sym_SQUOTE] = anon_sym_SQUOTE,
  [aux_sym_string_content_token1] = aux_sym_string_content_token1,
  [aux_sym_single_string_content_token1] = aux_sym_single_string_content_token1,
  [sym_escape_sequence] = sym_escape_sequence,
  [sym_number] = sym_number,
  [sym_float] = sym_float,
  [anon_sym_ms] = anon_sym_ms,
  [anon_sym_s] = anon_sym_s,
  [anon_sym_m] = anon_sym_m,
  [anon_sym_true] = anon_sym_true,
  [anon_sym_false] = anon_sym_false,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [sym_comment] = sym_comment,
  [sym_embedded_code] = sym_embedded_code,
  [sym__embedded_code_start] = sym__embedded_code_start,
  [sym_source_file] = sym_source_file,
  [sym_use_statement] = sym_use_statement,
  [sym_global_setup] = sym_global_setup,
  [sym_global_setup_body] = sym_global_setup_body,
  [sym_global_setup_statement] = sym_global_setup_statement,
  [sym_anvil_call] = sym_anvil_call,
  [sym_anvil_args] = sym_anvil_args,
  [sym_function_call] = sym_function_call,
  [sym_argument_list] = sym_argument_list,
  [sym_argument] = sym_argument,
  [sym_suite] = sym_suite,
  [sym_suite_body] = sym_suite_body,
  [sym__suite_item] = sym__suite_item,
  [sym_setup_block] = sym_setup_block,
  [sym_setup_body] = sym_setup_body,
  [sym__setup_section] = sym__setup_section,
  [sym_import_section] = sym_import_section,
  [sym_declare_section] = sym_declare_section,
  [sym_init_section] = sym_init_section,
  [sym_helpers_section] = sym_helpers_section,
  [sym_fixture] = sym_fixture,
  [sym_fixture_params] = sym_fixture_params,
  [sym_fixture_param] = sym_fixture_param,
  [sym_fixture_body] = sym_fixture_body,
  [sym__fixture_item] = sym__fixture_item,
  [sym_hex_property] = sym_hex_property,
  [sym_shape_property] = sym_shape_property,
  [sym_file_ref] = sym_file_ref,
  [sym_benchmark] = sym_benchmark,
  [sym_benchmark_body] = sym_benchmark_body,
  [sym__benchmark_item] = sym__benchmark_item,
  [sym_tags_property] = sym_tags_property,
  [sym_skip_hook] = sym_skip_hook,
  [sym_validate_hook] = sym_validate_hook,
  [sym_before_hook] = sym_before_hook,
  [sym_after_hook] = sym_after_hook,
  [sym_each_hook] = sym_each_hook,
  [sym_hook_flat] = sym_hook_flat,
  [sym_hook_grouped] = sym_hook_grouped,
  [sym_after_block] = sym_after_block,
  [sym_after_body] = sym_after_body,
  [sym_chart_directive] = sym_chart_directive,
  [sym_chart_function_name] = sym_chart_function_name,
  [sym_chart_params] = sym_chart_params,
  [sym_chart_param] = sym_chart_param,
  [sym_chart_param_name] = sym_chart_param_name,
  [sym__chart_value] = sym__chart_value,
  [sym_property] = sym_property,
  [sym_property_name] = sym_property_name,
  [sym__value] = sym__value,
  [sym_language_implementation] = sym_language_implementation,
  [sym_language_tag] = sym_language_tag,
  [sym__code_or_inline] = sym__code_or_inline,
  [sym_code_block] = sym_code_block,
  [sym_paren_code_block] = sym_paren_code_block,
  [sym_string] = sym_string,
  [sym_string_content] = sym_string_content,
  [sym_single_string_content] = sym_single_string_content,
  [sym_duration] = sym_duration,
  [sym_duration_unit] = sym_duration_unit,
  [sym_boolean] = sym_boolean,
  [sym_string_array] = sym_string_array,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_source_file_repeat2] = aux_sym_source_file_repeat2,
  [aux_sym_global_setup_body_repeat1] = aux_sym_global_setup_body_repeat1,
  [aux_sym_argument_list_repeat1] = aux_sym_argument_list_repeat1,
  [aux_sym_suite_body_repeat1] = aux_sym_suite_body_repeat1,
  [aux_sym_setup_body_repeat1] = aux_sym_setup_body_repeat1,
  [aux_sym_fixture_params_repeat1] = aux_sym_fixture_params_repeat1,
  [aux_sym_fixture_body_repeat1] = aux_sym_fixture_body_repeat1,
  [aux_sym_benchmark_body_repeat1] = aux_sym_benchmark_body_repeat1,
  [aux_sym_hook_grouped_repeat1] = aux_sym_hook_grouped_repeat1,
  [aux_sym_after_body_repeat1] = aux_sym_after_body_repeat1,
  [aux_sym_chart_params_repeat1] = aux_sym_chart_params_repeat1,
  [aux_sym_string_content_repeat1] = aux_sym_string_content_repeat1,
  [aux_sym_single_string_content_repeat1] = aux_sym_single_string_content_repeat1,
  [aux_sym_string_array_repeat1] = aux_sym_string_array_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_use] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_std] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_globalSetup] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_anvil] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_spawnAnvil] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_fork] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_suite] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_setup] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_import] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_declare] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_async] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_init] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_helpers] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_fixture] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_hex] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_shape] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ATfile] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_bench] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_tags] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_skip] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_validate] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_before] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_after] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_each] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_charting] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_drawBarChart] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_drawLineChart] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_drawPieChart] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_drawScatterPlot] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_drawHistogram] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_drawHeatmap] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_drawBoxPlot] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_drawAreaChart] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_drawSpeedupChart] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_drawScalingChart] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_drawTable] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_title] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_description] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_xlabel] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ylabel] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_output] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_sortBy] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_sortOrder] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_timeUnit] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_legendPosition] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_regressionStyle] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_regressionModel] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_yScale] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_baselineBenchmark] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_filterWinner] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_showStats] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_showConfig] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_showWinCounts] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_showGeoMean] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_showDistribution] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_showMemory] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_showTotalTime] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_showLegend] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_showGrid] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_showErrorBars] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_showRegression] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_showRegressionLabel] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_showRSquared] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_showEquation] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_showRegressionBand] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_showMinorGrid] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_showVerticalGrid] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_showStdDevBand] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_roundTicks] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_compact] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_width] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_height] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_precision] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_limit] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_titleFontSize] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_subtitleFontSize] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_axisLabelFontSize] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_tickLabelFontSize] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_barGroupGap] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_barWithinGroupGap] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_barWidth] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ciLevel] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_minSpeedup] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_axisThickness] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_yAxisMin] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_yAxisMax] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_gridOpacity] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_minorGridOpacity] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_errorBarOpacity] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_errorBarThickness] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_regressionBandOpacity] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_symlogThreshold] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_includeBenchmarks] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_excludeBenchmarks] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_iterations] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_warmup] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_timeout] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_requires] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_order] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_compare] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_baseline] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_mode] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_targetTime] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_minIterations] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_maxIterations] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_sink] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_outlierDetection] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_cvThreshold] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_count] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_memory] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_concurrency] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_go] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ts] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_typescript] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_rust] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_python] = {
    .visible = true,
    .named = false,
  },
  [sym_inline_code] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SQUOTE] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_string_content_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_single_string_content_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_escape_sequence] = {
    .visible = true,
    .named = true,
  },
  [sym_number] = {
    .visible = true,
    .named = true,
  },
  [sym_float] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_ms] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_s] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_m] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_true] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_false] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_embedded_code] = {
    .visible = true,
    .named = true,
  },
  [sym__embedded_code_start] = {
    .visible = false,
    .named = true,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym_use_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_global_setup] = {
    .visible = true,
    .named = true,
  },
  [sym_global_setup_body] = {
    .visible = true,
    .named = true,
  },
  [sym_global_setup_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_anvil_call] = {
    .visible = true,
    .named = true,
  },
  [sym_anvil_args] = {
    .visible = true,
    .named = true,
  },
  [sym_function_call] = {
    .visible = true,
    .named = true,
  },
  [sym_argument_list] = {
    .visible = true,
    .named = true,
  },
  [sym_argument] = {
    .visible = true,
    .named = true,
  },
  [sym_suite] = {
    .visible = true,
    .named = true,
  },
  [sym_suite_body] = {
    .visible = true,
    .named = true,
  },
  [sym__suite_item] = {
    .visible = false,
    .named = true,
  },
  [sym_setup_block] = {
    .visible = true,
    .named = true,
  },
  [sym_setup_body] = {
    .visible = true,
    .named = true,
  },
  [sym__setup_section] = {
    .visible = false,
    .named = true,
  },
  [sym_import_section] = {
    .visible = true,
    .named = true,
  },
  [sym_declare_section] = {
    .visible = true,
    .named = true,
  },
  [sym_init_section] = {
    .visible = true,
    .named = true,
  },
  [sym_helpers_section] = {
    .visible = true,
    .named = true,
  },
  [sym_fixture] = {
    .visible = true,
    .named = true,
  },
  [sym_fixture_params] = {
    .visible = true,
    .named = true,
  },
  [sym_fixture_param] = {
    .visible = true,
    .named = true,
  },
  [sym_fixture_body] = {
    .visible = true,
    .named = true,
  },
  [sym__fixture_item] = {
    .visible = false,
    .named = true,
  },
  [sym_hex_property] = {
    .visible = true,
    .named = true,
  },
  [sym_shape_property] = {
    .visible = true,
    .named = true,
  },
  [sym_file_ref] = {
    .visible = true,
    .named = true,
  },
  [sym_benchmark] = {
    .visible = true,
    .named = true,
  },
  [sym_benchmark_body] = {
    .visible = true,
    .named = true,
  },
  [sym__benchmark_item] = {
    .visible = false,
    .named = true,
  },
  [sym_tags_property] = {
    .visible = true,
    .named = true,
  },
  [sym_skip_hook] = {
    .visible = true,
    .named = true,
  },
  [sym_validate_hook] = {
    .visible = true,
    .named = true,
  },
  [sym_before_hook] = {
    .visible = true,
    .named = true,
  },
  [sym_after_hook] = {
    .visible = true,
    .named = true,
  },
  [sym_each_hook] = {
    .visible = true,
    .named = true,
  },
  [sym_hook_flat] = {
    .visible = true,
    .named = true,
  },
  [sym_hook_grouped] = {
    .visible = true,
    .named = true,
  },
  [sym_after_block] = {
    .visible = true,
    .named = true,
  },
  [sym_after_body] = {
    .visible = true,
    .named = true,
  },
  [sym_chart_directive] = {
    .visible = true,
    .named = true,
  },
  [sym_chart_function_name] = {
    .visible = true,
    .named = true,
  },
  [sym_chart_params] = {
    .visible = true,
    .named = true,
  },
  [sym_chart_param] = {
    .visible = true,
    .named = true,
  },
  [sym_chart_param_name] = {
    .visible = true,
    .named = true,
  },
  [sym__chart_value] = {
    .visible = false,
    .named = true,
  },
  [sym_property] = {
    .visible = true,
    .named = true,
  },
  [sym_property_name] = {
    .visible = true,
    .named = true,
  },
  [sym__value] = {
    .visible = false,
    .named = true,
  },
  [sym_language_implementation] = {
    .visible = true,
    .named = true,
  },
  [sym_language_tag] = {
    .visible = true,
    .named = true,
  },
  [sym__code_or_inline] = {
    .visible = false,
    .named = true,
  },
  [sym_code_block] = {
    .visible = true,
    .named = true,
  },
  [sym_paren_code_block] = {
    .visible = true,
    .named = true,
  },
  [sym_string] = {
    .visible = true,
    .named = true,
  },
  [sym_string_content] = {
    .visible = true,
    .named = true,
  },
  [sym_single_string_content] = {
    .visible = true,
    .named = true,
  },
  [sym_duration] = {
    .visible = true,
    .named = true,
  },
  [sym_duration_unit] = {
    .visible = true,
    .named = true,
  },
  [sym_boolean] = {
    .visible = true,
    .named = true,
  },
  [sym_string_array] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_source_file_repeat2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_global_setup_body_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_argument_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_suite_body_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_setup_body_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_fixture_params_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_fixture_body_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_benchmark_body_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_hook_grouped_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_after_body_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_chart_params_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_string_content_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_single_string_content_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_string_array_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum ts_field_identifiers {
  field_function = 1,
  field_language = 2,
  field_module = 3,
  field_name = 4,
  field_type = 5,
  field_value = 6,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_function] = "function",
  [field_language] = "language",
  [field_module] = "module",
  [field_name] = "name",
  [field_type] = "type",
  [field_value] = "value",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 1},
  [3] = {.index = 2, .length = 1},
  [4] = {.index = 3, .length = 2},
  [5] = {.index = 5, .length = 1},
  [6] = {.index = 6, .length = 2},
  [7] = {.index = 8, .length = 1},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_name, 1},
  [1] =
    {field_module, 3},
  [2] =
    {field_language, 1},
  [3] =
    {field_name, 0},
    {field_value, 2},
  [5] =
    {field_language, 0},
  [6] =
    {field_name, 0},
    {field_type, 2},
  [8] =
    {field_function, 2},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 24,
  [32] = 32,
  [33] = 33,
  [34] = 34,
  [35] = 35,
  [36] = 36,
  [37] = 37,
  [38] = 38,
  [39] = 24,
  [40] = 40,
  [41] = 41,
  [42] = 42,
  [43] = 43,
  [44] = 44,
  [45] = 45,
  [46] = 46,
  [47] = 47,
  [48] = 48,
  [49] = 49,
  [50] = 50,
  [51] = 51,
  [52] = 52,
  [53] = 53,
  [54] = 54,
  [55] = 55,
  [56] = 56,
  [57] = 57,
  [58] = 58,
  [59] = 59,
  [60] = 59,
  [61] = 61,
  [62] = 59,
  [63] = 63,
  [64] = 64,
  [65] = 65,
  [66] = 66,
  [67] = 67,
  [68] = 68,
  [69] = 69,
  [70] = 70,
  [71] = 71,
  [72] = 72,
  [73] = 73,
  [74] = 74,
  [75] = 75,
  [76] = 76,
  [77] = 77,
  [78] = 78,
  [79] = 79,
  [80] = 80,
  [81] = 81,
  [82] = 82,
  [83] = 83,
  [84] = 24,
  [85] = 85,
  [86] = 86,
  [87] = 87,
  [88] = 88,
  [89] = 89,
  [90] = 90,
  [91] = 91,
  [92] = 92,
  [93] = 93,
  [94] = 94,
  [95] = 95,
  [96] = 96,
  [97] = 97,
  [98] = 98,
  [99] = 99,
  [100] = 100,
  [101] = 101,
  [102] = 102,
  [103] = 103,
  [104] = 104,
  [105] = 105,
  [106] = 106,
  [107] = 107,
  [108] = 108,
  [109] = 109,
  [110] = 110,
  [111] = 111,
  [112] = 112,
  [113] = 113,
  [114] = 114,
  [115] = 115,
  [116] = 116,
  [117] = 117,
  [118] = 118,
  [119] = 119,
  [120] = 120,
  [121] = 121,
  [122] = 122,
  [123] = 123,
  [124] = 124,
  [125] = 125,
  [126] = 126,
  [127] = 127,
  [128] = 128,
  [129] = 129,
  [130] = 130,
  [131] = 131,
  [132] = 132,
  [133] = 133,
  [134] = 134,
  [135] = 135,
  [136] = 136,
  [137] = 137,
  [138] = 138,
  [139] = 139,
  [140] = 140,
  [141] = 141,
  [142] = 142,
  [143] = 143,
  [144] = 144,
  [145] = 145,
  [146] = 146,
  [147] = 147,
  [148] = 148,
  [149] = 149,
  [150] = 150,
  [151] = 151,
  [152] = 152,
  [153] = 153,
  [154] = 154,
  [155] = 155,
  [156] = 156,
  [157] = 157,
  [158] = 158,
  [159] = 159,
  [160] = 160,
  [161] = 161,
  [162] = 162,
  [163] = 163,
  [164] = 164,
  [165] = 165,
  [166] = 166,
  [167] = 167,
  [168] = 168,
  [169] = 169,
  [170] = 170,
  [171] = 171,
  [172] = 172,
  [173] = 173,
  [174] = 174,
  [175] = 175,
  [176] = 176,
  [177] = 177,
  [178] = 178,
  [179] = 179,
  [180] = 180,
  [181] = 181,
  [182] = 182,
  [183] = 183,
  [184] = 184,
  [185] = 185,
  [186] = 186,
  [187] = 187,
  [188] = 188,
  [189] = 189,
  [190] = 190,
  [191] = 191,
  [192] = 192,
  [193] = 193,
  [194] = 194,
  [195] = 195,
  [196] = 196,
  [197] = 197,
  [198] = 198,
  [199] = 199,
  [200] = 200,
  [201] = 201,
  [202] = 202,
  [203] = 203,
  [204] = 204,
  [205] = 205,
  [206] = 206,
  [207] = 207,
  [208] = 208,
  [209] = 209,
  [210] = 210,
  [211] = 211,
  [212] = 212,
  [213] = 213,
  [214] = 214,
  [215] = 215,
  [216] = 216,
  [217] = 217,
  [218] = 218,
  [219] = 176,
  [220] = 176,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(12);
      ADVANCE_MAP(
        '"', 26,
        '#', 40,
        '\'', 27,
        '(', 18,
        ')', 19,
        ',', 21,
        '.', 17,
        ':', 20,
        '@', 7,
        '[', 38,
        '\\', 10,
        ']', 39,
        '{', 14,
        '}', 16,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(35);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(37);
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(26);
      if (lookahead == '#') ADVANCE(28);
      if (lookahead == '\\') ADVANCE(10);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(29);
      if (lookahead != 0) ADVANCE(30);
      END_STATE();
    case 2:
      if (lookahead == '#') ADVANCE(40);
      if (lookahead == ':') ADVANCE(5);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(2);
      END_STATE();
    case 3:
      if (lookahead == '#') ADVANCE(31);
      if (lookahead == '\'') ADVANCE(27);
      if (lookahead == '\\') ADVANCE(10);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(32);
      if (lookahead != 0) ADVANCE(33);
      END_STATE();
    case 4:
      if (lookahead == '#') ADVANCE(23);
      if (lookahead == '{') ADVANCE(15);
      if (lookahead == '\n' ||
          lookahead == '\r') SKIP(4);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(24);
      if (lookahead != 0) ADVANCE(25);
      END_STATE();
    case 5:
      if (lookahead == ':') ADVANCE(13);
      END_STATE();
    case 6:
      if (lookahead == 'e') ADVANCE(22);
      END_STATE();
    case 7:
      if (lookahead == 'f') ADVANCE(8);
      END_STATE();
    case 8:
      if (lookahead == 'i') ADVANCE(9);
      END_STATE();
    case 9:
      if (lookahead == 'l') ADVANCE(6);
      END_STATE();
    case 10:
      if (lookahead == '"' ||
          lookahead == '\'' ||
          lookahead == '\\' ||
          lookahead == 'n' ||
          lookahead == 'r' ||
          lookahead == 't') ADVANCE(34);
      END_STATE();
    case 11:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(36);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(anon_sym_COLON_COLON);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(25);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(anon_sym_ATfile);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(sym_inline_code);
      if (lookahead == '\r') ADVANCE(40);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(23);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(sym_inline_code);
      if (lookahead == '#') ADVANCE(23);
      if (lookahead == '{') ADVANCE(15);
      if (lookahead == '\t' ||
          lookahead == 0x0b ||
          lookahead == '\f' ||
          lookahead == ' ') ADVANCE(24);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead)) ADVANCE(25);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(sym_inline_code);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(25);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(aux_sym_string_content_token1);
      if (lookahead == '\n') ADVANCE(30);
      if (lookahead == '"' ||
          lookahead == '\\') ADVANCE(40);
      if (lookahead != 0) ADVANCE(28);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(aux_sym_string_content_token1);
      if (lookahead == '#') ADVANCE(28);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(29);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '#' &&
          lookahead != '\\') ADVANCE(30);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(aux_sym_string_content_token1);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(30);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(aux_sym_single_string_content_token1);
      if (lookahead == '\n') ADVANCE(33);
      if (lookahead == '\'' ||
          lookahead == '\\') ADVANCE(40);
      if (lookahead != 0) ADVANCE(31);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(aux_sym_single_string_content_token1);
      if (lookahead == '#') ADVANCE(31);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(32);
      if (lookahead != 0 &&
          lookahead != '\'' &&
          lookahead != '\\') ADVANCE(33);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(aux_sym_single_string_content_token1);
      if (lookahead != 0 &&
          lookahead != '\'' &&
          lookahead != '\\') ADVANCE(33);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(sym_escape_sequence);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(sym_number);
      if (lookahead == '.') ADVANCE(11);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(35);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(sym_float);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(36);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(37);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(40);
      END_STATE();
    default:
      return false;
  }
}

static bool ts_lex_keywords(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      ADVANCE_MAP(
        'a', 1,
        'b', 2,
        'c', 3,
        'd', 4,
        'e', 5,
        'f', 6,
        'g', 7,
        'h', 8,
        'i', 9,
        'l', 10,
        'm', 11,
        'o', 12,
        'p', 13,
        'r', 14,
        's', 15,
        't', 16,
        'u', 17,
        'v', 18,
        'w', 19,
        'x', 20,
        'y', 21,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0);
      END_STATE();
    case 1:
      if (lookahead == 'f') ADVANCE(22);
      if (lookahead == 'n') ADVANCE(23);
      if (lookahead == 's') ADVANCE(24);
      if (lookahead == 'x') ADVANCE(25);
      END_STATE();
    case 2:
      if (lookahead == 'a') ADVANCE(26);
      if (lookahead == 'e') ADVANCE(27);
      END_STATE();
    case 3:
      if (lookahead == 'h') ADVANCE(28);
      if (lookahead == 'i') ADVANCE(29);
      if (lookahead == 'o') ADVANCE(30);
      if (lookahead == 'v') ADVANCE(31);
      END_STATE();
    case 4:
      if (lookahead == 'e') ADVANCE(32);
      if (lookahead == 'r') ADVANCE(33);
      END_STATE();
    case 5:
      if (lookahead == 'a') ADVANCE(34);
      if (lookahead == 'r') ADVANCE(35);
      if (lookahead == 'x') ADVANCE(36);
      END_STATE();
    case 6:
      if (lookahead == 'a') ADVANCE(37);
      if (lookahead == 'i') ADVANCE(38);
      if (lookahead == 'o') ADVANCE(39);
      END_STATE();
    case 7:
      if (lookahead == 'l') ADVANCE(40);
      if (lookahead == 'o') ADVANCE(41);
      if (lookahead == 'r') ADVANCE(42);
      END_STATE();
    case 8:
      if (lookahead == 'e') ADVANCE(43);
      END_STATE();
    case 9:
      if (lookahead == 'm') ADVANCE(44);
      if (lookahead == 'n') ADVANCE(45);
      if (lookahead == 't') ADVANCE(46);
      END_STATE();
    case 10:
      if (lookahead == 'e') ADVANCE(47);
      if (lookahead == 'i') ADVANCE(48);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_m);
      if (lookahead == 'a') ADVANCE(49);
      if (lookahead == 'e') ADVANCE(50);
      if (lookahead == 'i') ADVANCE(51);
      if (lookahead == 'o') ADVANCE(52);
      if (lookahead == 's') ADVANCE(53);
      END_STATE();
    case 12:
      if (lookahead == 'r') ADVANCE(54);
      if (lookahead == 'u') ADVANCE(55);
      END_STATE();
    case 13:
      if (lookahead == 'r') ADVANCE(56);
      if (lookahead == 'y') ADVANCE(57);
      END_STATE();
    case 14:
      if (lookahead == 'e') ADVANCE(58);
      if (lookahead == 'o') ADVANCE(59);
      if (lookahead == 'u') ADVANCE(60);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_s);
      ADVANCE_MAP(
        'e', 61,
        'h', 62,
        'i', 63,
        'k', 64,
        'o', 65,
        'p', 66,
        't', 67,
        'u', 68,
        'y', 69,
      );
      END_STATE();
    case 16:
      if (lookahead == 'a') ADVANCE(70);
      if (lookahead == 'i') ADVANCE(71);
      if (lookahead == 'r') ADVANCE(72);
      if (lookahead == 's') ADVANCE(73);
      if (lookahead == 'y') ADVANCE(74);
      END_STATE();
    case 17:
      if (lookahead == 's') ADVANCE(75);
      END_STATE();
    case 18:
      if (lookahead == 'a') ADVANCE(76);
      END_STATE();
    case 19:
      if (lookahead == 'a') ADVANCE(77);
      if (lookahead == 'i') ADVANCE(78);
      END_STATE();
    case 20:
      if (lookahead == 'l') ADVANCE(79);
      END_STATE();
    case 21:
      if (lookahead == 'A') ADVANCE(80);
      if (lookahead == 'S') ADVANCE(81);
      if (lookahead == 'l') ADVANCE(82);
      END_STATE();
    case 22:
      if (lookahead == 't') ADVANCE(83);
      END_STATE();
    case 23:
      if (lookahead == 'v') ADVANCE(84);
      END_STATE();
    case 24:
      if (lookahead == 'y') ADVANCE(85);
      END_STATE();
    case 25:
      if (lookahead == 'i') ADVANCE(86);
      END_STATE();
    case 26:
      if (lookahead == 'r') ADVANCE(87);
      if (lookahead == 's') ADVANCE(88);
      END_STATE();
    case 27:
      if (lookahead == 'f') ADVANCE(89);
      if (lookahead == 'n') ADVANCE(90);
      END_STATE();
    case 28:
      if (lookahead == 'a') ADVANCE(91);
      END_STATE();
    case 29:
      if (lookahead == 'L') ADVANCE(92);
      END_STATE();
    case 30:
      if (lookahead == 'm') ADVANCE(93);
      if (lookahead == 'n') ADVANCE(94);
      if (lookahead == 'u') ADVANCE(95);
      END_STATE();
    case 31:
      if (lookahead == 'T') ADVANCE(96);
      END_STATE();
    case 32:
      if (lookahead == 'c') ADVANCE(97);
      if (lookahead == 's') ADVANCE(98);
      END_STATE();
    case 33:
      if (lookahead == 'a') ADVANCE(99);
      END_STATE();
    case 34:
      if (lookahead == 'c') ADVANCE(100);
      END_STATE();
    case 35:
      if (lookahead == 'r') ADVANCE(101);
      END_STATE();
    case 36:
      if (lookahead == 'c') ADVANCE(102);
      END_STATE();
    case 37:
      if (lookahead == 'l') ADVANCE(103);
      END_STATE();
    case 38:
      if (lookahead == 'l') ADVANCE(104);
      if (lookahead == 'x') ADVANCE(105);
      END_STATE();
    case 39:
      if (lookahead == 'r') ADVANCE(106);
      END_STATE();
    case 40:
      if (lookahead == 'o') ADVANCE(107);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(anon_sym_go);
      END_STATE();
    case 42:
      if (lookahead == 'i') ADVANCE(108);
      END_STATE();
    case 43:
      if (lookahead == 'i') ADVANCE(109);
      if (lookahead == 'l') ADVANCE(110);
      if (lookahead == 'x') ADVANCE(111);
      END_STATE();
    case 44:
      if (lookahead == 'p') ADVANCE(112);
      END_STATE();
    case 45:
      if (lookahead == 'c') ADVANCE(113);
      if (lookahead == 'i') ADVANCE(114);
      END_STATE();
    case 46:
      if (lookahead == 'e') ADVANCE(115);
      END_STATE();
    case 47:
      if (lookahead == 'g') ADVANCE(116);
      END_STATE();
    case 48:
      if (lookahead == 'm') ADVANCE(117);
      END_STATE();
    case 49:
      if (lookahead == 'x') ADVANCE(118);
      END_STATE();
    case 50:
      if (lookahead == 'm') ADVANCE(119);
      END_STATE();
    case 51:
      if (lookahead == 'n') ADVANCE(120);
      END_STATE();
    case 52:
      if (lookahead == 'd') ADVANCE(121);
      END_STATE();
    case 53:
      ACCEPT_TOKEN(anon_sym_ms);
      END_STATE();
    case 54:
      if (lookahead == 'd') ADVANCE(122);
      END_STATE();
    case 55:
      if (lookahead == 't') ADVANCE(123);
      END_STATE();
    case 56:
      if (lookahead == 'e') ADVANCE(124);
      END_STATE();
    case 57:
      if (lookahead == 't') ADVANCE(125);
      END_STATE();
    case 58:
      if (lookahead == 'g') ADVANCE(126);
      if (lookahead == 'q') ADVANCE(127);
      END_STATE();
    case 59:
      if (lookahead == 'u') ADVANCE(128);
      END_STATE();
    case 60:
      if (lookahead == 's') ADVANCE(129);
      END_STATE();
    case 61:
      if (lookahead == 't') ADVANCE(130);
      END_STATE();
    case 62:
      if (lookahead == 'a') ADVANCE(131);
      if (lookahead == 'o') ADVANCE(132);
      END_STATE();
    case 63:
      if (lookahead == 'n') ADVANCE(133);
      END_STATE();
    case 64:
      if (lookahead == 'i') ADVANCE(134);
      END_STATE();
    case 65:
      if (lookahead == 'r') ADVANCE(135);
      END_STATE();
    case 66:
      if (lookahead == 'a') ADVANCE(136);
      END_STATE();
    case 67:
      if (lookahead == 'd') ADVANCE(137);
      END_STATE();
    case 68:
      if (lookahead == 'b') ADVANCE(138);
      if (lookahead == 'i') ADVANCE(139);
      END_STATE();
    case 69:
      if (lookahead == 'm') ADVANCE(140);
      END_STATE();
    case 70:
      if (lookahead == 'g') ADVANCE(141);
      if (lookahead == 'r') ADVANCE(142);
      END_STATE();
    case 71:
      if (lookahead == 'c') ADVANCE(143);
      if (lookahead == 'm') ADVANCE(144);
      if (lookahead == 't') ADVANCE(145);
      END_STATE();
    case 72:
      if (lookahead == 'u') ADVANCE(146);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(anon_sym_ts);
      END_STATE();
    case 74:
      if (lookahead == 'p') ADVANCE(147);
      END_STATE();
    case 75:
      if (lookahead == 'e') ADVANCE(148);
      END_STATE();
    case 76:
      if (lookahead == 'l') ADVANCE(149);
      END_STATE();
    case 77:
      if (lookahead == 'r') ADVANCE(150);
      END_STATE();
    case 78:
      if (lookahead == 'd') ADVANCE(151);
      END_STATE();
    case 79:
      if (lookahead == 'a') ADVANCE(152);
      END_STATE();
    case 80:
      if (lookahead == 'x') ADVANCE(153);
      END_STATE();
    case 81:
      if (lookahead == 'c') ADVANCE(154);
      END_STATE();
    case 82:
      if (lookahead == 'a') ADVANCE(155);
      END_STATE();
    case 83:
      if (lookahead == 'e') ADVANCE(156);
      END_STATE();
    case 84:
      if (lookahead == 'i') ADVANCE(157);
      END_STATE();
    case 85:
      if (lookahead == 'n') ADVANCE(158);
      END_STATE();
    case 86:
      if (lookahead == 's') ADVANCE(159);
      END_STATE();
    case 87:
      if (lookahead == 'G') ADVANCE(160);
      if (lookahead == 'W') ADVANCE(161);
      END_STATE();
    case 88:
      if (lookahead == 'e') ADVANCE(162);
      END_STATE();
    case 89:
      if (lookahead == 'o') ADVANCE(163);
      END_STATE();
    case 90:
      if (lookahead == 'c') ADVANCE(164);
      END_STATE();
    case 91:
      if (lookahead == 'r') ADVANCE(165);
      END_STATE();
    case 92:
      if (lookahead == 'e') ADVANCE(166);
      END_STATE();
    case 93:
      if (lookahead == 'p') ADVANCE(167);
      END_STATE();
    case 94:
      if (lookahead == 'c') ADVANCE(168);
      END_STATE();
    case 95:
      if (lookahead == 'n') ADVANCE(169);
      END_STATE();
    case 96:
      if (lookahead == 'h') ADVANCE(170);
      END_STATE();
    case 97:
      if (lookahead == 'l') ADVANCE(171);
      END_STATE();
    case 98:
      if (lookahead == 'c') ADVANCE(172);
      END_STATE();
    case 99:
      if (lookahead == 'w') ADVANCE(173);
      END_STATE();
    case 100:
      if (lookahead == 'h') ADVANCE(174);
      END_STATE();
    case 101:
      if (lookahead == 'o') ADVANCE(175);
      END_STATE();
    case 102:
      if (lookahead == 'l') ADVANCE(176);
      END_STATE();
    case 103:
      if (lookahead == 's') ADVANCE(177);
      END_STATE();
    case 104:
      if (lookahead == 't') ADVANCE(178);
      END_STATE();
    case 105:
      if (lookahead == 't') ADVANCE(179);
      END_STATE();
    case 106:
      if (lookahead == 'k') ADVANCE(180);
      END_STATE();
    case 107:
      if (lookahead == 'b') ADVANCE(181);
      END_STATE();
    case 108:
      if (lookahead == 'd') ADVANCE(182);
      END_STATE();
    case 109:
      if (lookahead == 'g') ADVANCE(183);
      END_STATE();
    case 110:
      if (lookahead == 'p') ADVANCE(184);
      END_STATE();
    case 111:
      ACCEPT_TOKEN(anon_sym_hex);
      END_STATE();
    case 112:
      if (lookahead == 'o') ADVANCE(185);
      END_STATE();
    case 113:
      if (lookahead == 'l') ADVANCE(186);
      END_STATE();
    case 114:
      if (lookahead == 't') ADVANCE(187);
      END_STATE();
    case 115:
      if (lookahead == 'r') ADVANCE(188);
      END_STATE();
    case 116:
      if (lookahead == 'e') ADVANCE(189);
      END_STATE();
    case 117:
      if (lookahead == 'i') ADVANCE(190);
      END_STATE();
    case 118:
      if (lookahead == 'I') ADVANCE(191);
      END_STATE();
    case 119:
      if (lookahead == 'o') ADVANCE(192);
      END_STATE();
    case 120:
      if (lookahead == 'I') ADVANCE(193);
      if (lookahead == 'S') ADVANCE(194);
      if (lookahead == 'o') ADVANCE(195);
      END_STATE();
    case 121:
      if (lookahead == 'e') ADVANCE(196);
      END_STATE();
    case 122:
      if (lookahead == 'e') ADVANCE(197);
      END_STATE();
    case 123:
      if (lookahead == 'l') ADVANCE(198);
      if (lookahead == 'p') ADVANCE(199);
      END_STATE();
    case 124:
      if (lookahead == 'c') ADVANCE(200);
      END_STATE();
    case 125:
      if (lookahead == 'h') ADVANCE(201);
      END_STATE();
    case 126:
      if (lookahead == 'r') ADVANCE(202);
      END_STATE();
    case 127:
      if (lookahead == 'u') ADVANCE(203);
      END_STATE();
    case 128:
      if (lookahead == 'n') ADVANCE(204);
      END_STATE();
    case 129:
      if (lookahead == 't') ADVANCE(205);
      END_STATE();
    case 130:
      if (lookahead == 'u') ADVANCE(206);
      END_STATE();
    case 131:
      if (lookahead == 'p') ADVANCE(207);
      END_STATE();
    case 132:
      if (lookahead == 'w') ADVANCE(208);
      END_STATE();
    case 133:
      if (lookahead == 'k') ADVANCE(209);
      END_STATE();
    case 134:
      if (lookahead == 'p') ADVANCE(210);
      END_STATE();
    case 135:
      if (lookahead == 't') ADVANCE(211);
      END_STATE();
    case 136:
      if (lookahead == 'w') ADVANCE(212);
      END_STATE();
    case 137:
      ACCEPT_TOKEN(anon_sym_std);
      END_STATE();
    case 138:
      if (lookahead == 't') ADVANCE(213);
      END_STATE();
    case 139:
      if (lookahead == 't') ADVANCE(214);
      END_STATE();
    case 140:
      if (lookahead == 'l') ADVANCE(215);
      END_STATE();
    case 141:
      if (lookahead == 's') ADVANCE(216);
      END_STATE();
    case 142:
      if (lookahead == 'g') ADVANCE(217);
      END_STATE();
    case 143:
      if (lookahead == 'k') ADVANCE(218);
      END_STATE();
    case 144:
      if (lookahead == 'e') ADVANCE(219);
      END_STATE();
    case 145:
      if (lookahead == 'l') ADVANCE(220);
      END_STATE();
    case 146:
      if (lookahead == 'e') ADVANCE(221);
      END_STATE();
    case 147:
      if (lookahead == 'e') ADVANCE(222);
      END_STATE();
    case 148:
      ACCEPT_TOKEN(anon_sym_use);
      END_STATE();
    case 149:
      if (lookahead == 'i') ADVANCE(223);
      END_STATE();
    case 150:
      if (lookahead == 'm') ADVANCE(224);
      END_STATE();
    case 151:
      if (lookahead == 't') ADVANCE(225);
      END_STATE();
    case 152:
      if (lookahead == 'b') ADVANCE(226);
      END_STATE();
    case 153:
      if (lookahead == 'i') ADVANCE(227);
      END_STATE();
    case 154:
      if (lookahead == 'a') ADVANCE(228);
      END_STATE();
    case 155:
      if (lookahead == 'b') ADVANCE(229);
      END_STATE();
    case 156:
      if (lookahead == 'r') ADVANCE(230);
      END_STATE();
    case 157:
      if (lookahead == 'l') ADVANCE(231);
      END_STATE();
    case 158:
      if (lookahead == 'c') ADVANCE(232);
      END_STATE();
    case 159:
      if (lookahead == 'L') ADVANCE(233);
      if (lookahead == 'T') ADVANCE(234);
      END_STATE();
    case 160:
      if (lookahead == 'r') ADVANCE(235);
      END_STATE();
    case 161:
      if (lookahead == 'i') ADVANCE(236);
      END_STATE();
    case 162:
      if (lookahead == 'l') ADVANCE(237);
      END_STATE();
    case 163:
      if (lookahead == 'r') ADVANCE(238);
      END_STATE();
    case 164:
      if (lookahead == 'h') ADVANCE(239);
      END_STATE();
    case 165:
      if (lookahead == 't') ADVANCE(240);
      END_STATE();
    case 166:
      if (lookahead == 'v') ADVANCE(241);
      END_STATE();
    case 167:
      if (lookahead == 'a') ADVANCE(242);
      END_STATE();
    case 168:
      if (lookahead == 'u') ADVANCE(243);
      END_STATE();
    case 169:
      if (lookahead == 't') ADVANCE(244);
      END_STATE();
    case 170:
      if (lookahead == 'r') ADVANCE(245);
      END_STATE();
    case 171:
      if (lookahead == 'a') ADVANCE(246);
      END_STATE();
    case 172:
      if (lookahead == 'r') ADVANCE(247);
      END_STATE();
    case 173:
      if (lookahead == 'A') ADVANCE(248);
      if (lookahead == 'B') ADVANCE(249);
      if (lookahead == 'H') ADVANCE(250);
      if (lookahead == 'L') ADVANCE(251);
      if (lookahead == 'P') ADVANCE(252);
      if (lookahead == 'S') ADVANCE(253);
      if (lookahead == 'T') ADVANCE(254);
      END_STATE();
    case 174:
      ACCEPT_TOKEN(anon_sym_each);
      END_STATE();
    case 175:
      if (lookahead == 'r') ADVANCE(255);
      END_STATE();
    case 176:
      if (lookahead == 'u') ADVANCE(256);
      END_STATE();
    case 177:
      if (lookahead == 'e') ADVANCE(257);
      END_STATE();
    case 178:
      if (lookahead == 'e') ADVANCE(258);
      END_STATE();
    case 179:
      if (lookahead == 'u') ADVANCE(259);
      END_STATE();
    case 180:
      ACCEPT_TOKEN(anon_sym_fork);
      END_STATE();
    case 181:
      if (lookahead == 'a') ADVANCE(260);
      END_STATE();
    case 182:
      if (lookahead == 'O') ADVANCE(261);
      END_STATE();
    case 183:
      if (lookahead == 'h') ADVANCE(262);
      END_STATE();
    case 184:
      if (lookahead == 'e') ADVANCE(263);
      END_STATE();
    case 185:
      if (lookahead == 'r') ADVANCE(264);
      END_STATE();
    case 186:
      if (lookahead == 'u') ADVANCE(265);
      END_STATE();
    case 187:
      ACCEPT_TOKEN(anon_sym_init);
      END_STATE();
    case 188:
      if (lookahead == 'a') ADVANCE(266);
      END_STATE();
    case 189:
      if (lookahead == 'n') ADVANCE(267);
      END_STATE();
    case 190:
      if (lookahead == 't') ADVANCE(268);
      END_STATE();
    case 191:
      if (lookahead == 't') ADVANCE(269);
      END_STATE();
    case 192:
      if (lookahead == 'r') ADVANCE(270);
      END_STATE();
    case 193:
      if (lookahead == 't') ADVANCE(271);
      END_STATE();
    case 194:
      if (lookahead == 'p') ADVANCE(272);
      END_STATE();
    case 195:
      if (lookahead == 'r') ADVANCE(273);
      END_STATE();
    case 196:
      ACCEPT_TOKEN(anon_sym_mode);
      END_STATE();
    case 197:
      if (lookahead == 'r') ADVANCE(274);
      END_STATE();
    case 198:
      if (lookahead == 'i') ADVANCE(275);
      END_STATE();
    case 199:
      if (lookahead == 'u') ADVANCE(276);
      END_STATE();
    case 200:
      if (lookahead == 'i') ADVANCE(277);
      END_STATE();
    case 201:
      if (lookahead == 'o') ADVANCE(278);
      END_STATE();
    case 202:
      if (lookahead == 'e') ADVANCE(279);
      END_STATE();
    case 203:
      if (lookahead == 'i') ADVANCE(280);
      END_STATE();
    case 204:
      if (lookahead == 'd') ADVANCE(281);
      END_STATE();
    case 205:
      ACCEPT_TOKEN(anon_sym_rust);
      END_STATE();
    case 206:
      if (lookahead == 'p') ADVANCE(282);
      END_STATE();
    case 207:
      if (lookahead == 'e') ADVANCE(283);
      END_STATE();
    case 208:
      ADVANCE_MAP(
        'C', 284,
        'D', 285,
        'E', 286,
        'G', 287,
        'L', 288,
        'M', 289,
        'R', 290,
        'S', 291,
        'T', 292,
        'V', 293,
        'W', 294,
      );
      END_STATE();
    case 209:
      ACCEPT_TOKEN(anon_sym_sink);
      END_STATE();
    case 210:
      ACCEPT_TOKEN(anon_sym_skip);
      END_STATE();
    case 211:
      if (lookahead == 'B') ADVANCE(295);
      if (lookahead == 'O') ADVANCE(296);
      END_STATE();
    case 212:
      if (lookahead == 'n') ADVANCE(297);
      END_STATE();
    case 213:
      if (lookahead == 'i') ADVANCE(298);
      END_STATE();
    case 214:
      if (lookahead == 'e') ADVANCE(299);
      END_STATE();
    case 215:
      if (lookahead == 'o') ADVANCE(300);
      END_STATE();
    case 216:
      ACCEPT_TOKEN(anon_sym_tags);
      END_STATE();
    case 217:
      if (lookahead == 'e') ADVANCE(301);
      END_STATE();
    case 218:
      if (lookahead == 'L') ADVANCE(302);
      END_STATE();
    case 219:
      if (lookahead == 'U') ADVANCE(303);
      if (lookahead == 'o') ADVANCE(304);
      END_STATE();
    case 220:
      if (lookahead == 'e') ADVANCE(305);
      END_STATE();
    case 221:
      ACCEPT_TOKEN(anon_sym_true);
      END_STATE();
    case 222:
      if (lookahead == 's') ADVANCE(306);
      END_STATE();
    case 223:
      if (lookahead == 'd') ADVANCE(307);
      END_STATE();
    case 224:
      if (lookahead == 'u') ADVANCE(308);
      END_STATE();
    case 225:
      if (lookahead == 'h') ADVANCE(309);
      END_STATE();
    case 226:
      if (lookahead == 'e') ADVANCE(310);
      END_STATE();
    case 227:
      if (lookahead == 's') ADVANCE(311);
      END_STATE();
    case 228:
      if (lookahead == 'l') ADVANCE(312);
      END_STATE();
    case 229:
      if (lookahead == 'e') ADVANCE(313);
      END_STATE();
    case 230:
      ACCEPT_TOKEN(anon_sym_after);
      END_STATE();
    case 231:
      ACCEPT_TOKEN(anon_sym_anvil);
      END_STATE();
    case 232:
      ACCEPT_TOKEN(anon_sym_async);
      END_STATE();
    case 233:
      if (lookahead == 'a') ADVANCE(314);
      END_STATE();
    case 234:
      if (lookahead == 'h') ADVANCE(315);
      END_STATE();
    case 235:
      if (lookahead == 'o') ADVANCE(316);
      END_STATE();
    case 236:
      if (lookahead == 'd') ADVANCE(317);
      if (lookahead == 't') ADVANCE(318);
      END_STATE();
    case 237:
      if (lookahead == 'i') ADVANCE(319);
      END_STATE();
    case 238:
      if (lookahead == 'e') ADVANCE(320);
      END_STATE();
    case 239:
      ACCEPT_TOKEN(anon_sym_bench);
      END_STATE();
    case 240:
      if (lookahead == 'i') ADVANCE(321);
      END_STATE();
    case 241:
      if (lookahead == 'e') ADVANCE(322);
      END_STATE();
    case 242:
      if (lookahead == 'c') ADVANCE(323);
      if (lookahead == 'r') ADVANCE(324);
      END_STATE();
    case 243:
      if (lookahead == 'r') ADVANCE(325);
      END_STATE();
    case 244:
      ACCEPT_TOKEN(anon_sym_count);
      END_STATE();
    case 245:
      if (lookahead == 'e') ADVANCE(326);
      END_STATE();
    case 246:
      if (lookahead == 'r') ADVANCE(327);
      END_STATE();
    case 247:
      if (lookahead == 'i') ADVANCE(328);
      END_STATE();
    case 248:
      if (lookahead == 'r') ADVANCE(329);
      END_STATE();
    case 249:
      if (lookahead == 'a') ADVANCE(330);
      if (lookahead == 'o') ADVANCE(331);
      END_STATE();
    case 250:
      if (lookahead == 'e') ADVANCE(332);
      if (lookahead == 'i') ADVANCE(333);
      END_STATE();
    case 251:
      if (lookahead == 'i') ADVANCE(334);
      END_STATE();
    case 252:
      if (lookahead == 'i') ADVANCE(335);
      END_STATE();
    case 253:
      if (lookahead == 'c') ADVANCE(336);
      if (lookahead == 'p') ADVANCE(337);
      END_STATE();
    case 254:
      if (lookahead == 'a') ADVANCE(338);
      END_STATE();
    case 255:
      if (lookahead == 'B') ADVANCE(339);
      END_STATE();
    case 256:
      if (lookahead == 'd') ADVANCE(340);
      END_STATE();
    case 257:
      ACCEPT_TOKEN(anon_sym_false);
      END_STATE();
    case 258:
      if (lookahead == 'r') ADVANCE(341);
      END_STATE();
    case 259:
      if (lookahead == 'r') ADVANCE(342);
      END_STATE();
    case 260:
      if (lookahead == 'l') ADVANCE(343);
      END_STATE();
    case 261:
      if (lookahead == 'p') ADVANCE(344);
      END_STATE();
    case 262:
      if (lookahead == 't') ADVANCE(345);
      END_STATE();
    case 263:
      if (lookahead == 'r') ADVANCE(346);
      END_STATE();
    case 264:
      if (lookahead == 't') ADVANCE(347);
      END_STATE();
    case 265:
      if (lookahead == 'd') ADVANCE(348);
      END_STATE();
    case 266:
      if (lookahead == 't') ADVANCE(349);
      END_STATE();
    case 267:
      if (lookahead == 'd') ADVANCE(350);
      END_STATE();
    case 268:
      ACCEPT_TOKEN(anon_sym_limit);
      END_STATE();
    case 269:
      if (lookahead == 'e') ADVANCE(351);
      END_STATE();
    case 270:
      if (lookahead == 'y') ADVANCE(352);
      END_STATE();
    case 271:
      if (lookahead == 'e') ADVANCE(353);
      END_STATE();
    case 272:
      if (lookahead == 'e') ADVANCE(354);
      END_STATE();
    case 273:
      if (lookahead == 'G') ADVANCE(355);
      END_STATE();
    case 274:
      ACCEPT_TOKEN(anon_sym_order);
      END_STATE();
    case 275:
      if (lookahead == 'e') ADVANCE(356);
      END_STATE();
    case 276:
      if (lookahead == 't') ADVANCE(357);
      END_STATE();
    case 277:
      if (lookahead == 's') ADVANCE(358);
      END_STATE();
    case 278:
      if (lookahead == 'n') ADVANCE(359);
      END_STATE();
    case 279:
      if (lookahead == 's') ADVANCE(360);
      END_STATE();
    case 280:
      if (lookahead == 'r') ADVANCE(361);
      END_STATE();
    case 281:
      if (lookahead == 'T') ADVANCE(362);
      END_STATE();
    case 282:
      ACCEPT_TOKEN(anon_sym_setup);
      END_STATE();
    case 283:
      ACCEPT_TOKEN(anon_sym_shape);
      END_STATE();
    case 284:
      if (lookahead == 'o') ADVANCE(363);
      END_STATE();
    case 285:
      if (lookahead == 'i') ADVANCE(364);
      END_STATE();
    case 286:
      if (lookahead == 'q') ADVANCE(365);
      if (lookahead == 'r') ADVANCE(366);
      END_STATE();
    case 287:
      if (lookahead == 'e') ADVANCE(367);
      if (lookahead == 'r') ADVANCE(368);
      END_STATE();
    case 288:
      if (lookahead == 'e') ADVANCE(369);
      END_STATE();
    case 289:
      if (lookahead == 'e') ADVANCE(370);
      if (lookahead == 'i') ADVANCE(371);
      END_STATE();
    case 290:
      if (lookahead == 'S') ADVANCE(372);
      if (lookahead == 'e') ADVANCE(373);
      END_STATE();
    case 291:
      if (lookahead == 't') ADVANCE(374);
      END_STATE();
    case 292:
      if (lookahead == 'o') ADVANCE(375);
      END_STATE();
    case 293:
      if (lookahead == 'e') ADVANCE(376);
      END_STATE();
    case 294:
      if (lookahead == 'i') ADVANCE(377);
      END_STATE();
    case 295:
      if (lookahead == 'y') ADVANCE(378);
      END_STATE();
    case 296:
      if (lookahead == 'r') ADVANCE(379);
      END_STATE();
    case 297:
      if (lookahead == 'A') ADVANCE(380);
      END_STATE();
    case 298:
      if (lookahead == 't') ADVANCE(381);
      END_STATE();
    case 299:
      ACCEPT_TOKEN(anon_sym_suite);
      END_STATE();
    case 300:
      if (lookahead == 'g') ADVANCE(382);
      END_STATE();
    case 301:
      if (lookahead == 't') ADVANCE(383);
      END_STATE();
    case 302:
      if (lookahead == 'a') ADVANCE(384);
      END_STATE();
    case 303:
      if (lookahead == 'n') ADVANCE(385);
      END_STATE();
    case 304:
      if (lookahead == 'u') ADVANCE(386);
      END_STATE();
    case 305:
      ACCEPT_TOKEN(anon_sym_title);
      if (lookahead == 'F') ADVANCE(387);
      END_STATE();
    case 306:
      if (lookahead == 'c') ADVANCE(388);
      END_STATE();
    case 307:
      if (lookahead == 'a') ADVANCE(389);
      END_STATE();
    case 308:
      if (lookahead == 'p') ADVANCE(390);
      END_STATE();
    case 309:
      ACCEPT_TOKEN(anon_sym_width);
      END_STATE();
    case 310:
      if (lookahead == 'l') ADVANCE(391);
      END_STATE();
    case 311:
      if (lookahead == 'M') ADVANCE(392);
      END_STATE();
    case 312:
      if (lookahead == 'e') ADVANCE(393);
      END_STATE();
    case 313:
      if (lookahead == 'l') ADVANCE(394);
      END_STATE();
    case 314:
      if (lookahead == 'b') ADVANCE(395);
      END_STATE();
    case 315:
      if (lookahead == 'i') ADVANCE(396);
      END_STATE();
    case 316:
      if (lookahead == 'u') ADVANCE(397);
      END_STATE();
    case 317:
      if (lookahead == 't') ADVANCE(398);
      END_STATE();
    case 318:
      if (lookahead == 'h') ADVANCE(399);
      END_STATE();
    case 319:
      if (lookahead == 'n') ADVANCE(400);
      END_STATE();
    case 320:
      ACCEPT_TOKEN(anon_sym_before);
      END_STATE();
    case 321:
      if (lookahead == 'n') ADVANCE(401);
      END_STATE();
    case 322:
      if (lookahead == 'l') ADVANCE(402);
      END_STATE();
    case 323:
      if (lookahead == 't') ADVANCE(403);
      END_STATE();
    case 324:
      if (lookahead == 'e') ADVANCE(404);
      END_STATE();
    case 325:
      if (lookahead == 'r') ADVANCE(405);
      END_STATE();
    case 326:
      if (lookahead == 's') ADVANCE(406);
      END_STATE();
    case 327:
      if (lookahead == 'e') ADVANCE(407);
      END_STATE();
    case 328:
      if (lookahead == 'p') ADVANCE(408);
      END_STATE();
    case 329:
      if (lookahead == 'e') ADVANCE(409);
      END_STATE();
    case 330:
      if (lookahead == 'r') ADVANCE(410);
      END_STATE();
    case 331:
      if (lookahead == 'x') ADVANCE(411);
      END_STATE();
    case 332:
      if (lookahead == 'a') ADVANCE(412);
      END_STATE();
    case 333:
      if (lookahead == 's') ADVANCE(413);
      END_STATE();
    case 334:
      if (lookahead == 'n') ADVANCE(414);
      END_STATE();
    case 335:
      if (lookahead == 'e') ADVANCE(415);
      END_STATE();
    case 336:
      if (lookahead == 'a') ADVANCE(416);
      END_STATE();
    case 337:
      if (lookahead == 'e') ADVANCE(417);
      END_STATE();
    case 338:
      if (lookahead == 'b') ADVANCE(418);
      END_STATE();
    case 339:
      if (lookahead == 'a') ADVANCE(419);
      END_STATE();
    case 340:
      if (lookahead == 'e') ADVANCE(420);
      END_STATE();
    case 341:
      if (lookahead == 'W') ADVANCE(421);
      END_STATE();
    case 342:
      if (lookahead == 'e') ADVANCE(422);
      END_STATE();
    case 343:
      if (lookahead == 'S') ADVANCE(423);
      END_STATE();
    case 344:
      if (lookahead == 'a') ADVANCE(424);
      END_STATE();
    case 345:
      ACCEPT_TOKEN(anon_sym_height);
      END_STATE();
    case 346:
      if (lookahead == 's') ADVANCE(425);
      END_STATE();
    case 347:
      ACCEPT_TOKEN(anon_sym_import);
      END_STATE();
    case 348:
      if (lookahead == 'e') ADVANCE(426);
      END_STATE();
    case 349:
      if (lookahead == 'i') ADVANCE(427);
      END_STATE();
    case 350:
      if (lookahead == 'P') ADVANCE(428);
      END_STATE();
    case 351:
      if (lookahead == 'r') ADVANCE(429);
      END_STATE();
    case 352:
      ACCEPT_TOKEN(anon_sym_memory);
      END_STATE();
    case 353:
      if (lookahead == 'r') ADVANCE(430);
      END_STATE();
    case 354:
      if (lookahead == 'e') ADVANCE(431);
      END_STATE();
    case 355:
      if (lookahead == 'r') ADVANCE(432);
      END_STATE();
    case 356:
      if (lookahead == 'r') ADVANCE(433);
      END_STATE();
    case 357:
      ACCEPT_TOKEN(anon_sym_output);
      END_STATE();
    case 358:
      if (lookahead == 'i') ADVANCE(434);
      END_STATE();
    case 359:
      ACCEPT_TOKEN(anon_sym_python);
      END_STATE();
    case 360:
      if (lookahead == 's') ADVANCE(435);
      END_STATE();
    case 361:
      if (lookahead == 'e') ADVANCE(436);
      END_STATE();
    case 362:
      if (lookahead == 'i') ADVANCE(437);
      END_STATE();
    case 363:
      if (lookahead == 'n') ADVANCE(438);
      END_STATE();
    case 364:
      if (lookahead == 's') ADVANCE(439);
      END_STATE();
    case 365:
      if (lookahead == 'u') ADVANCE(440);
      END_STATE();
    case 366:
      if (lookahead == 'r') ADVANCE(441);
      END_STATE();
    case 367:
      if (lookahead == 'o') ADVANCE(442);
      END_STATE();
    case 368:
      if (lookahead == 'i') ADVANCE(443);
      END_STATE();
    case 369:
      if (lookahead == 'g') ADVANCE(444);
      END_STATE();
    case 370:
      if (lookahead == 'm') ADVANCE(445);
      END_STATE();
    case 371:
      if (lookahead == 'n') ADVANCE(446);
      END_STATE();
    case 372:
      if (lookahead == 'q') ADVANCE(447);
      END_STATE();
    case 373:
      if (lookahead == 'g') ADVANCE(448);
      END_STATE();
    case 374:
      if (lookahead == 'a') ADVANCE(449);
      if (lookahead == 'd') ADVANCE(450);
      END_STATE();
    case 375:
      if (lookahead == 't') ADVANCE(451);
      END_STATE();
    case 376:
      if (lookahead == 'r') ADVANCE(452);
      END_STATE();
    case 377:
      if (lookahead == 'n') ADVANCE(453);
      END_STATE();
    case 378:
      ACCEPT_TOKEN(anon_sym_sortBy);
      END_STATE();
    case 379:
      if (lookahead == 'd') ADVANCE(454);
      END_STATE();
    case 380:
      if (lookahead == 'n') ADVANCE(455);
      END_STATE();
    case 381:
      if (lookahead == 'l') ADVANCE(456);
      END_STATE();
    case 382:
      if (lookahead == 'T') ADVANCE(457);
      END_STATE();
    case 383:
      if (lookahead == 'T') ADVANCE(458);
      END_STATE();
    case 384:
      if (lookahead == 'b') ADVANCE(459);
      END_STATE();
    case 385:
      if (lookahead == 'i') ADVANCE(460);
      END_STATE();
    case 386:
      if (lookahead == 't') ADVANCE(461);
      END_STATE();
    case 387:
      if (lookahead == 'o') ADVANCE(462);
      END_STATE();
    case 388:
      if (lookahead == 'r') ADVANCE(463);
      END_STATE();
    case 389:
      if (lookahead == 't') ADVANCE(464);
      END_STATE();
    case 390:
      ACCEPT_TOKEN(anon_sym_warmup);
      END_STATE();
    case 391:
      ACCEPT_TOKEN(anon_sym_xlabel);
      END_STATE();
    case 392:
      if (lookahead == 'a') ADVANCE(465);
      if (lookahead == 'i') ADVANCE(466);
      END_STATE();
    case 393:
      ACCEPT_TOKEN(anon_sym_yScale);
      END_STATE();
    case 394:
      ACCEPT_TOKEN(anon_sym_ylabel);
      END_STATE();
    case 395:
      if (lookahead == 'e') ADVANCE(467);
      END_STATE();
    case 396:
      if (lookahead == 'c') ADVANCE(468);
      END_STATE();
    case 397:
      if (lookahead == 'p') ADVANCE(469);
      END_STATE();
    case 398:
      if (lookahead == 'h') ADVANCE(470);
      END_STATE();
    case 399:
      if (lookahead == 'i') ADVANCE(471);
      END_STATE();
    case 400:
      if (lookahead == 'e') ADVANCE(472);
      END_STATE();
    case 401:
      if (lookahead == 'g') ADVANCE(473);
      END_STATE();
    case 402:
      ACCEPT_TOKEN(anon_sym_ciLevel);
      END_STATE();
    case 403:
      ACCEPT_TOKEN(anon_sym_compact);
      END_STATE();
    case 404:
      ACCEPT_TOKEN(anon_sym_compare);
      END_STATE();
    case 405:
      if (lookahead == 'e') ADVANCE(474);
      END_STATE();
    case 406:
      if (lookahead == 'h') ADVANCE(475);
      END_STATE();
    case 407:
      ACCEPT_TOKEN(anon_sym_declare);
      END_STATE();
    case 408:
      if (lookahead == 't') ADVANCE(476);
      END_STATE();
    case 409:
      if (lookahead == 'a') ADVANCE(477);
      END_STATE();
    case 410:
      if (lookahead == 'C') ADVANCE(478);
      END_STATE();
    case 411:
      if (lookahead == 'P') ADVANCE(479);
      END_STATE();
    case 412:
      if (lookahead == 't') ADVANCE(480);
      END_STATE();
    case 413:
      if (lookahead == 't') ADVANCE(481);
      END_STATE();
    case 414:
      if (lookahead == 'e') ADVANCE(482);
      END_STATE();
    case 415:
      if (lookahead == 'C') ADVANCE(483);
      END_STATE();
    case 416:
      if (lookahead == 'l') ADVANCE(484);
      if (lookahead == 't') ADVANCE(485);
      END_STATE();
    case 417:
      if (lookahead == 'e') ADVANCE(486);
      END_STATE();
    case 418:
      if (lookahead == 'l') ADVANCE(487);
      END_STATE();
    case 419:
      if (lookahead == 'r') ADVANCE(488);
      END_STATE();
    case 420:
      if (lookahead == 'B') ADVANCE(489);
      END_STATE();
    case 421:
      if (lookahead == 'i') ADVANCE(490);
      END_STATE();
    case 422:
      ACCEPT_TOKEN(anon_sym_fixture);
      END_STATE();
    case 423:
      if (lookahead == 'e') ADVANCE(491);
      END_STATE();
    case 424:
      if (lookahead == 'c') ADVANCE(492);
      END_STATE();
    case 425:
      ACCEPT_TOKEN(anon_sym_helpers);
      END_STATE();
    case 426:
      if (lookahead == 'B') ADVANCE(493);
      END_STATE();
    case 427:
      if (lookahead == 'o') ADVANCE(494);
      END_STATE();
    case 428:
      if (lookahead == 'o') ADVANCE(495);
      END_STATE();
    case 429:
      if (lookahead == 'a') ADVANCE(496);
      END_STATE();
    case 430:
      if (lookahead == 'a') ADVANCE(497);
      END_STATE();
    case 431:
      if (lookahead == 'd') ADVANCE(498);
      END_STATE();
    case 432:
      if (lookahead == 'i') ADVANCE(499);
      END_STATE();
    case 433:
      if (lookahead == 'D') ADVANCE(500);
      END_STATE();
    case 434:
      if (lookahead == 'o') ADVANCE(501);
      END_STATE();
    case 435:
      if (lookahead == 'i') ADVANCE(502);
      END_STATE();
    case 436:
      if (lookahead == 's') ADVANCE(503);
      END_STATE();
    case 437:
      if (lookahead == 'c') ADVANCE(504);
      END_STATE();
    case 438:
      if (lookahead == 'f') ADVANCE(505);
      END_STATE();
    case 439:
      if (lookahead == 't') ADVANCE(506);
      END_STATE();
    case 440:
      if (lookahead == 'a') ADVANCE(507);
      END_STATE();
    case 441:
      if (lookahead == 'o') ADVANCE(508);
      END_STATE();
    case 442:
      if (lookahead == 'M') ADVANCE(509);
      END_STATE();
    case 443:
      if (lookahead == 'd') ADVANCE(510);
      END_STATE();
    case 444:
      if (lookahead == 'e') ADVANCE(511);
      END_STATE();
    case 445:
      if (lookahead == 'o') ADVANCE(512);
      END_STATE();
    case 446:
      if (lookahead == 'o') ADVANCE(513);
      END_STATE();
    case 447:
      if (lookahead == 'u') ADVANCE(514);
      END_STATE();
    case 448:
      if (lookahead == 'r') ADVANCE(515);
      END_STATE();
    case 449:
      if (lookahead == 't') ADVANCE(516);
      END_STATE();
    case 450:
      if (lookahead == 'D') ADVANCE(517);
      END_STATE();
    case 451:
      if (lookahead == 'a') ADVANCE(518);
      END_STATE();
    case 452:
      if (lookahead == 't') ADVANCE(519);
      END_STATE();
    case 453:
      if (lookahead == 'C') ADVANCE(520);
      END_STATE();
    case 454:
      if (lookahead == 'e') ADVANCE(521);
      END_STATE();
    case 455:
      if (lookahead == 'v') ADVANCE(522);
      END_STATE();
    case 456:
      if (lookahead == 'e') ADVANCE(523);
      END_STATE();
    case 457:
      if (lookahead == 'h') ADVANCE(524);
      END_STATE();
    case 458:
      if (lookahead == 'i') ADVANCE(525);
      END_STATE();
    case 459:
      if (lookahead == 'e') ADVANCE(526);
      END_STATE();
    case 460:
      if (lookahead == 't') ADVANCE(527);
      END_STATE();
    case 461:
      ACCEPT_TOKEN(anon_sym_timeout);
      END_STATE();
    case 462:
      if (lookahead == 'n') ADVANCE(528);
      END_STATE();
    case 463:
      if (lookahead == 'i') ADVANCE(529);
      END_STATE();
    case 464:
      if (lookahead == 'e') ADVANCE(530);
      END_STATE();
    case 465:
      if (lookahead == 'x') ADVANCE(531);
      END_STATE();
    case 466:
      if (lookahead == 'n') ADVANCE(532);
      END_STATE();
    case 467:
      if (lookahead == 'l') ADVANCE(533);
      END_STATE();
    case 468:
      if (lookahead == 'k') ADVANCE(534);
      END_STATE();
    case 469:
      if (lookahead == 'G') ADVANCE(535);
      END_STATE();
    case 470:
      ACCEPT_TOKEN(anon_sym_barWidth);
      END_STATE();
    case 471:
      if (lookahead == 'n') ADVANCE(536);
      END_STATE();
    case 472:
      ACCEPT_TOKEN(anon_sym_baseline);
      if (lookahead == 'B') ADVANCE(537);
      END_STATE();
    case 473:
      ACCEPT_TOKEN(anon_sym_charting);
      END_STATE();
    case 474:
      if (lookahead == 'n') ADVANCE(538);
      END_STATE();
    case 475:
      if (lookahead == 'o') ADVANCE(539);
      END_STATE();
    case 476:
      if (lookahead == 'i') ADVANCE(540);
      END_STATE();
    case 477:
      if (lookahead == 'C') ADVANCE(541);
      END_STATE();
    case 478:
      if (lookahead == 'h') ADVANCE(542);
      END_STATE();
    case 479:
      if (lookahead == 'l') ADVANCE(543);
      END_STATE();
    case 480:
      if (lookahead == 'm') ADVANCE(544);
      END_STATE();
    case 481:
      if (lookahead == 'o') ADVANCE(545);
      END_STATE();
    case 482:
      if (lookahead == 'C') ADVANCE(546);
      END_STATE();
    case 483:
      if (lookahead == 'h') ADVANCE(547);
      END_STATE();
    case 484:
      if (lookahead == 'i') ADVANCE(548);
      END_STATE();
    case 485:
      if (lookahead == 't') ADVANCE(549);
      END_STATE();
    case 486:
      if (lookahead == 'd') ADVANCE(550);
      END_STATE();
    case 487:
      if (lookahead == 'e') ADVANCE(551);
      END_STATE();
    case 488:
      if (lookahead == 'O') ADVANCE(552);
      if (lookahead == 'T') ADVANCE(553);
      END_STATE();
    case 489:
      if (lookahead == 'e') ADVANCE(554);
      END_STATE();
    case 490:
      if (lookahead == 'n') ADVANCE(555);
      END_STATE();
    case 491:
      if (lookahead == 't') ADVANCE(556);
      END_STATE();
    case 492:
      if (lookahead == 'i') ADVANCE(557);
      END_STATE();
    case 493:
      if (lookahead == 'e') ADVANCE(558);
      END_STATE();
    case 494:
      if (lookahead == 'n') ADVANCE(559);
      END_STATE();
    case 495:
      if (lookahead == 's') ADVANCE(560);
      END_STATE();
    case 496:
      if (lookahead == 't') ADVANCE(561);
      END_STATE();
    case 497:
      if (lookahead == 't') ADVANCE(562);
      END_STATE();
    case 498:
      if (lookahead == 'u') ADVANCE(563);
      END_STATE();
    case 499:
      if (lookahead == 'd') ADVANCE(564);
      END_STATE();
    case 500:
      if (lookahead == 'e') ADVANCE(565);
      END_STATE();
    case 501:
      if (lookahead == 'n') ADVANCE(566);
      END_STATE();
    case 502:
      if (lookahead == 'o') ADVANCE(567);
      END_STATE();
    case 503:
      ACCEPT_TOKEN(anon_sym_requires);
      END_STATE();
    case 504:
      if (lookahead == 'k') ADVANCE(568);
      END_STATE();
    case 505:
      if (lookahead == 'i') ADVANCE(569);
      END_STATE();
    case 506:
      if (lookahead == 'r') ADVANCE(570);
      END_STATE();
    case 507:
      if (lookahead == 't') ADVANCE(571);
      END_STATE();
    case 508:
      if (lookahead == 'r') ADVANCE(572);
      END_STATE();
    case 509:
      if (lookahead == 'e') ADVANCE(573);
      END_STATE();
    case 510:
      ACCEPT_TOKEN(anon_sym_showGrid);
      END_STATE();
    case 511:
      if (lookahead == 'n') ADVANCE(574);
      END_STATE();
    case 512:
      if (lookahead == 'r') ADVANCE(575);
      END_STATE();
    case 513:
      if (lookahead == 'r') ADVANCE(576);
      END_STATE();
    case 514:
      if (lookahead == 'a') ADVANCE(577);
      END_STATE();
    case 515:
      if (lookahead == 'e') ADVANCE(578);
      END_STATE();
    case 516:
      if (lookahead == 's') ADVANCE(579);
      END_STATE();
    case 517:
      if (lookahead == 'e') ADVANCE(580);
      END_STATE();
    case 518:
      if (lookahead == 'l') ADVANCE(581);
      END_STATE();
    case 519:
      if (lookahead == 'i') ADVANCE(582);
      END_STATE();
    case 520:
      if (lookahead == 'o') ADVANCE(583);
      END_STATE();
    case 521:
      if (lookahead == 'r') ADVANCE(584);
      END_STATE();
    case 522:
      if (lookahead == 'i') ADVANCE(585);
      END_STATE();
    case 523:
      if (lookahead == 'F') ADVANCE(586);
      END_STATE();
    case 524:
      if (lookahead == 'r') ADVANCE(587);
      END_STATE();
    case 525:
      if (lookahead == 'm') ADVANCE(588);
      END_STATE();
    case 526:
      if (lookahead == 'l') ADVANCE(589);
      END_STATE();
    case 527:
      ACCEPT_TOKEN(anon_sym_timeUnit);
      END_STATE();
    case 528:
      if (lookahead == 't') ADVANCE(590);
      END_STATE();
    case 529:
      if (lookahead == 'p') ADVANCE(591);
      END_STATE();
    case 530:
      ACCEPT_TOKEN(anon_sym_validate);
      END_STATE();
    case 531:
      ACCEPT_TOKEN(anon_sym_yAxisMax);
      END_STATE();
    case 532:
      ACCEPT_TOKEN(anon_sym_yAxisMin);
      END_STATE();
    case 533:
      if (lookahead == 'F') ADVANCE(592);
      END_STATE();
    case 534:
      if (lookahead == 'n') ADVANCE(593);
      END_STATE();
    case 535:
      if (lookahead == 'a') ADVANCE(594);
      END_STATE();
    case 536:
      if (lookahead == 'G') ADVANCE(595);
      END_STATE();
    case 537:
      if (lookahead == 'e') ADVANCE(596);
      END_STATE();
    case 538:
      if (lookahead == 'c') ADVANCE(597);
      END_STATE();
    case 539:
      if (lookahead == 'l') ADVANCE(598);
      END_STATE();
    case 540:
      if (lookahead == 'o') ADVANCE(599);
      END_STATE();
    case 541:
      if (lookahead == 'h') ADVANCE(600);
      END_STATE();
    case 542:
      if (lookahead == 'a') ADVANCE(601);
      END_STATE();
    case 543:
      if (lookahead == 'o') ADVANCE(602);
      END_STATE();
    case 544:
      if (lookahead == 'a') ADVANCE(603);
      END_STATE();
    case 545:
      if (lookahead == 'g') ADVANCE(604);
      END_STATE();
    case 546:
      if (lookahead == 'h') ADVANCE(605);
      END_STATE();
    case 547:
      if (lookahead == 'a') ADVANCE(606);
      END_STATE();
    case 548:
      if (lookahead == 'n') ADVANCE(607);
      END_STATE();
    case 549:
      if (lookahead == 'e') ADVANCE(608);
      END_STATE();
    case 550:
      if (lookahead == 'u') ADVANCE(609);
      END_STATE();
    case 551:
      ACCEPT_TOKEN(anon_sym_drawTable);
      END_STATE();
    case 552:
      if (lookahead == 'p') ADVANCE(610);
      END_STATE();
    case 553:
      if (lookahead == 'h') ADVANCE(611);
      END_STATE();
    case 554:
      if (lookahead == 'n') ADVANCE(612);
      END_STATE();
    case 555:
      if (lookahead == 'n') ADVANCE(613);
      END_STATE();
    case 556:
      if (lookahead == 'u') ADVANCE(614);
      END_STATE();
    case 557:
      if (lookahead == 't') ADVANCE(615);
      END_STATE();
    case 558:
      if (lookahead == 'n') ADVANCE(616);
      END_STATE();
    case 559:
      if (lookahead == 's') ADVANCE(617);
      END_STATE();
    case 560:
      if (lookahead == 'i') ADVANCE(618);
      END_STATE();
    case 561:
      if (lookahead == 'i') ADVANCE(619);
      END_STATE();
    case 562:
      if (lookahead == 'i') ADVANCE(620);
      END_STATE();
    case 563:
      if (lookahead == 'p') ADVANCE(621);
      END_STATE();
    case 564:
      if (lookahead == 'O') ADVANCE(622);
      END_STATE();
    case 565:
      if (lookahead == 't') ADVANCE(623);
      END_STATE();
    case 566:
      ACCEPT_TOKEN(anon_sym_precision);
      END_STATE();
    case 567:
      if (lookahead == 'n') ADVANCE(624);
      END_STATE();
    case 568:
      if (lookahead == 's') ADVANCE(625);
      END_STATE();
    case 569:
      if (lookahead == 'g') ADVANCE(626);
      END_STATE();
    case 570:
      if (lookahead == 'i') ADVANCE(627);
      END_STATE();
    case 571:
      if (lookahead == 'i') ADVANCE(628);
      END_STATE();
    case 572:
      if (lookahead == 'B') ADVANCE(629);
      END_STATE();
    case 573:
      if (lookahead == 'a') ADVANCE(630);
      END_STATE();
    case 574:
      if (lookahead == 'd') ADVANCE(631);
      END_STATE();
    case 575:
      if (lookahead == 'y') ADVANCE(632);
      END_STATE();
    case 576:
      if (lookahead == 'G') ADVANCE(633);
      END_STATE();
    case 577:
      if (lookahead == 'r') ADVANCE(634);
      END_STATE();
    case 578:
      if (lookahead == 's') ADVANCE(635);
      END_STATE();
    case 579:
      ACCEPT_TOKEN(anon_sym_showStats);
      END_STATE();
    case 580:
      if (lookahead == 'v') ADVANCE(636);
      END_STATE();
    case 581:
      if (lookahead == 'T') ADVANCE(637);
      END_STATE();
    case 582:
      if (lookahead == 'c') ADVANCE(638);
      END_STATE();
    case 583:
      if (lookahead == 'u') ADVANCE(639);
      END_STATE();
    case 584:
      ACCEPT_TOKEN(anon_sym_sortOrder);
      END_STATE();
    case 585:
      if (lookahead == 'l') ADVANCE(640);
      END_STATE();
    case 586:
      if (lookahead == 'o') ADVANCE(641);
      END_STATE();
    case 587:
      if (lookahead == 'e') ADVANCE(642);
      END_STATE();
    case 588:
      if (lookahead == 'e') ADVANCE(643);
      END_STATE();
    case 589:
      if (lookahead == 'F') ADVANCE(644);
      END_STATE();
    case 590:
      if (lookahead == 'S') ADVANCE(645);
      END_STATE();
    case 591:
      if (lookahead == 't') ADVANCE(646);
      END_STATE();
    case 592:
      if (lookahead == 'o') ADVANCE(647);
      END_STATE();
    case 593:
      if (lookahead == 'e') ADVANCE(648);
      END_STATE();
    case 594:
      if (lookahead == 'p') ADVANCE(649);
      END_STATE();
    case 595:
      if (lookahead == 'r') ADVANCE(650);
      END_STATE();
    case 596:
      if (lookahead == 'n') ADVANCE(651);
      END_STATE();
    case 597:
      if (lookahead == 'y') ADVANCE(652);
      END_STATE();
    case 598:
      if (lookahead == 'd') ADVANCE(653);
      END_STATE();
    case 599:
      if (lookahead == 'n') ADVANCE(654);
      END_STATE();
    case 600:
      if (lookahead == 'a') ADVANCE(655);
      END_STATE();
    case 601:
      if (lookahead == 'r') ADVANCE(656);
      END_STATE();
    case 602:
      if (lookahead == 't') ADVANCE(657);
      END_STATE();
    case 603:
      if (lookahead == 'p') ADVANCE(658);
      END_STATE();
    case 604:
      if (lookahead == 'r') ADVANCE(659);
      END_STATE();
    case 605:
      if (lookahead == 'a') ADVANCE(660);
      END_STATE();
    case 606:
      if (lookahead == 'r') ADVANCE(661);
      END_STATE();
    case 607:
      if (lookahead == 'g') ADVANCE(662);
      END_STATE();
    case 608:
      if (lookahead == 'r') ADVANCE(663);
      END_STATE();
    case 609:
      if (lookahead == 'p') ADVANCE(664);
      END_STATE();
    case 610:
      if (lookahead == 'a') ADVANCE(665);
      END_STATE();
    case 611:
      if (lookahead == 'i') ADVANCE(666);
      END_STATE();
    case 612:
      if (lookahead == 'c') ADVANCE(667);
      END_STATE();
    case 613:
      if (lookahead == 'e') ADVANCE(668);
      END_STATE();
    case 614:
      if (lookahead == 'p') ADVANCE(669);
      END_STATE();
    case 615:
      if (lookahead == 'y') ADVANCE(670);
      END_STATE();
    case 616:
      if (lookahead == 'c') ADVANCE(671);
      END_STATE();
    case 617:
      ACCEPT_TOKEN(anon_sym_iterations);
      END_STATE();
    case 618:
      if (lookahead == 't') ADVANCE(672);
      END_STATE();
    case 619:
      if (lookahead == 'o') ADVANCE(673);
      END_STATE();
    case 620:
      if (lookahead == 'o') ADVANCE(674);
      END_STATE();
    case 621:
      ACCEPT_TOKEN(anon_sym_minSpeedup);
      END_STATE();
    case 622:
      if (lookahead == 'p') ADVANCE(675);
      END_STATE();
    case 623:
      if (lookahead == 'e') ADVANCE(676);
      END_STATE();
    case 624:
      if (lookahead == 'B') ADVANCE(677);
      if (lookahead == 'M') ADVANCE(678);
      if (lookahead == 'S') ADVANCE(679);
      END_STATE();
    case 625:
      ACCEPT_TOKEN(anon_sym_roundTicks);
      END_STATE();
    case 626:
      ACCEPT_TOKEN(anon_sym_showConfig);
      END_STATE();
    case 627:
      if (lookahead == 'b') ADVANCE(680);
      END_STATE();
    case 628:
      if (lookahead == 'o') ADVANCE(681);
      END_STATE();
    case 629:
      if (lookahead == 'a') ADVANCE(682);
      END_STATE();
    case 630:
      if (lookahead == 'n') ADVANCE(683);
      END_STATE();
    case 631:
      ACCEPT_TOKEN(anon_sym_showLegend);
      END_STATE();
    case 632:
      ACCEPT_TOKEN(anon_sym_showMemory);
      END_STATE();
    case 633:
      if (lookahead == 'r') ADVANCE(684);
      END_STATE();
    case 634:
      if (lookahead == 'e') ADVANCE(685);
      END_STATE();
    case 635:
      if (lookahead == 's') ADVANCE(686);
      END_STATE();
    case 636:
      if (lookahead == 'B') ADVANCE(687);
      END_STATE();
    case 637:
      if (lookahead == 'i') ADVANCE(688);
      END_STATE();
    case 638:
      if (lookahead == 'a') ADVANCE(689);
      END_STATE();
    case 639:
      if (lookahead == 'n') ADVANCE(690);
      END_STATE();
    case 640:
      ACCEPT_TOKEN(anon_sym_spawnAnvil);
      END_STATE();
    case 641:
      if (lookahead == 'n') ADVANCE(691);
      END_STATE();
    case 642:
      if (lookahead == 's') ADVANCE(692);
      END_STATE();
    case 643:
      ACCEPT_TOKEN(anon_sym_targetTime);
      END_STATE();
    case 644:
      if (lookahead == 'o') ADVANCE(693);
      END_STATE();
    case 645:
      if (lookahead == 'i') ADVANCE(694);
      END_STATE();
    case 646:
      ACCEPT_TOKEN(anon_sym_typescript);
      END_STATE();
    case 647:
      if (lookahead == 'n') ADVANCE(695);
      END_STATE();
    case 648:
      if (lookahead == 's') ADVANCE(696);
      END_STATE();
    case 649:
      ACCEPT_TOKEN(anon_sym_barGroupGap);
      END_STATE();
    case 650:
      if (lookahead == 'o') ADVANCE(697);
      END_STATE();
    case 651:
      if (lookahead == 'c') ADVANCE(698);
      END_STATE();
    case 652:
      ACCEPT_TOKEN(anon_sym_concurrency);
      END_STATE();
    case 653:
      ACCEPT_TOKEN(anon_sym_cvThreshold);
      END_STATE();
    case 654:
      ACCEPT_TOKEN(anon_sym_description);
      END_STATE();
    case 655:
      if (lookahead == 'r') ADVANCE(699);
      END_STATE();
    case 656:
      if (lookahead == 't') ADVANCE(700);
      END_STATE();
    case 657:
      ACCEPT_TOKEN(anon_sym_drawBoxPlot);
      END_STATE();
    case 658:
      ACCEPT_TOKEN(anon_sym_drawHeatmap);
      END_STATE();
    case 659:
      if (lookahead == 'a') ADVANCE(701);
      END_STATE();
    case 660:
      if (lookahead == 'r') ADVANCE(702);
      END_STATE();
    case 661:
      if (lookahead == 't') ADVANCE(703);
      END_STATE();
    case 662:
      if (lookahead == 'C') ADVANCE(704);
      END_STATE();
    case 663:
      if (lookahead == 'P') ADVANCE(705);
      END_STATE();
    case 664:
      if (lookahead == 'C') ADVANCE(706);
      END_STATE();
    case 665:
      if (lookahead == 'c') ADVANCE(707);
      END_STATE();
    case 666:
      if (lookahead == 'c') ADVANCE(708);
      END_STATE();
    case 667:
      if (lookahead == 'h') ADVANCE(709);
      END_STATE();
    case 668:
      if (lookahead == 'r') ADVANCE(710);
      END_STATE();
    case 669:
      ACCEPT_TOKEN(anon_sym_globalSetup);
      END_STATE();
    case 670:
      ACCEPT_TOKEN(anon_sym_gridOpacity);
      END_STATE();
    case 671:
      if (lookahead == 'h') ADVANCE(711);
      END_STATE();
    case 672:
      if (lookahead == 'i') ADVANCE(712);
      END_STATE();
    case 673:
      if (lookahead == 'n') ADVANCE(713);
      END_STATE();
    case 674:
      if (lookahead == 'n') ADVANCE(714);
      END_STATE();
    case 675:
      if (lookahead == 'a') ADVANCE(715);
      END_STATE();
    case 676:
      if (lookahead == 'c') ADVANCE(716);
      END_STATE();
    case 677:
      if (lookahead == 'a') ADVANCE(717);
      END_STATE();
    case 678:
      if (lookahead == 'o') ADVANCE(718);
      END_STATE();
    case 679:
      if (lookahead == 't') ADVANCE(719);
      END_STATE();
    case 680:
      if (lookahead == 'u') ADVANCE(720);
      END_STATE();
    case 681:
      if (lookahead == 'n') ADVANCE(721);
      END_STATE();
    case 682:
      if (lookahead == 'r') ADVANCE(722);
      END_STATE();
    case 683:
      ACCEPT_TOKEN(anon_sym_showGeoMean);
      END_STATE();
    case 684:
      if (lookahead == 'i') ADVANCE(723);
      END_STATE();
    case 685:
      if (lookahead == 'd') ADVANCE(724);
      END_STATE();
    case 686:
      if (lookahead == 'i') ADVANCE(725);
      END_STATE();
    case 687:
      if (lookahead == 'a') ADVANCE(726);
      END_STATE();
    case 688:
      if (lookahead == 'm') ADVANCE(727);
      END_STATE();
    case 689:
      if (lookahead == 'l') ADVANCE(728);
      END_STATE();
    case 690:
      if (lookahead == 't') ADVANCE(729);
      END_STATE();
    case 691:
      if (lookahead == 't') ADVANCE(730);
      END_STATE();
    case 692:
      if (lookahead == 'h') ADVANCE(731);
      END_STATE();
    case 693:
      if (lookahead == 'n') ADVANCE(732);
      END_STATE();
    case 694:
      if (lookahead == 'z') ADVANCE(733);
      END_STATE();
    case 695:
      if (lookahead == 't') ADVANCE(734);
      END_STATE();
    case 696:
      if (lookahead == 's') ADVANCE(735);
      END_STATE();
    case 697:
      if (lookahead == 'u') ADVANCE(736);
      END_STATE();
    case 698:
      if (lookahead == 'h') ADVANCE(737);
      END_STATE();
    case 699:
      if (lookahead == 't') ADVANCE(738);
      END_STATE();
    case 700:
      ACCEPT_TOKEN(anon_sym_drawBarChart);
      END_STATE();
    case 701:
      if (lookahead == 'm') ADVANCE(739);
      END_STATE();
    case 702:
      if (lookahead == 't') ADVANCE(740);
      END_STATE();
    case 703:
      ACCEPT_TOKEN(anon_sym_drawPieChart);
      END_STATE();
    case 704:
      if (lookahead == 'h') ADVANCE(741);
      END_STATE();
    case 705:
      if (lookahead == 'l') ADVANCE(742);
      END_STATE();
    case 706:
      if (lookahead == 'h') ADVANCE(743);
      END_STATE();
    case 707:
      if (lookahead == 'i') ADVANCE(744);
      END_STATE();
    case 708:
      if (lookahead == 'k') ADVANCE(745);
      END_STATE();
    case 709:
      if (lookahead == 'm') ADVANCE(746);
      END_STATE();
    case 710:
      ACCEPT_TOKEN(anon_sym_filterWinner);
      END_STATE();
    case 711:
      if (lookahead == 'm') ADVANCE(747);
      END_STATE();
    case 712:
      if (lookahead == 'o') ADVANCE(748);
      END_STATE();
    case 713:
      if (lookahead == 's') ADVANCE(749);
      END_STATE();
    case 714:
      if (lookahead == 's') ADVANCE(750);
      END_STATE();
    case 715:
      if (lookahead == 'c') ADVANCE(751);
      END_STATE();
    case 716:
      if (lookahead == 't') ADVANCE(752);
      END_STATE();
    case 717:
      if (lookahead == 'n') ADVANCE(753);
      END_STATE();
    case 718:
      if (lookahead == 'd') ADVANCE(754);
      END_STATE();
    case 719:
      if (lookahead == 'y') ADVANCE(755);
      END_STATE();
    case 720:
      if (lookahead == 't') ADVANCE(756);
      END_STATE();
    case 721:
      ACCEPT_TOKEN(anon_sym_showEquation);
      END_STATE();
    case 722:
      if (lookahead == 's') ADVANCE(757);
      END_STATE();
    case 723:
      if (lookahead == 'd') ADVANCE(758);
      END_STATE();
    case 724:
      ACCEPT_TOKEN(anon_sym_showRSquared);
      END_STATE();
    case 725:
      if (lookahead == 'o') ADVANCE(759);
      END_STATE();
    case 726:
      if (lookahead == 'n') ADVANCE(760);
      END_STATE();
    case 727:
      if (lookahead == 'e') ADVANCE(761);
      END_STATE();
    case 728:
      if (lookahead == 'G') ADVANCE(762);
      END_STATE();
    case 729:
      if (lookahead == 's') ADVANCE(763);
      END_STATE();
    case 730:
      if (lookahead == 'S') ADVANCE(764);
      END_STATE();
    case 731:
      if (lookahead == 'o') ADVANCE(765);
      END_STATE();
    case 732:
      if (lookahead == 't') ADVANCE(766);
      END_STATE();
    case 733:
      if (lookahead == 'e') ADVANCE(767);
      END_STATE();
    case 734:
      if (lookahead == 'S') ADVANCE(768);
      END_STATE();
    case 735:
      ACCEPT_TOKEN(anon_sym_axisThickness);
      END_STATE();
    case 736:
      if (lookahead == 'p') ADVANCE(769);
      END_STATE();
    case 737:
      if (lookahead == 'm') ADVANCE(770);
      END_STATE();
    case 738:
      ACCEPT_TOKEN(anon_sym_drawAreaChart);
      END_STATE();
    case 739:
      ACCEPT_TOKEN(anon_sym_drawHistogram);
      END_STATE();
    case 740:
      ACCEPT_TOKEN(anon_sym_drawLineChart);
      END_STATE();
    case 741:
      if (lookahead == 'a') ADVANCE(771);
      END_STATE();
    case 742:
      if (lookahead == 'o') ADVANCE(772);
      END_STATE();
    case 743:
      if (lookahead == 'a') ADVANCE(773);
      END_STATE();
    case 744:
      if (lookahead == 't') ADVANCE(774);
      END_STATE();
    case 745:
      if (lookahead == 'n') ADVANCE(775);
      END_STATE();
    case 746:
      if (lookahead == 'a') ADVANCE(776);
      END_STATE();
    case 747:
      if (lookahead == 'a') ADVANCE(777);
      END_STATE();
    case 748:
      if (lookahead == 'n') ADVANCE(778);
      END_STATE();
    case 749:
      ACCEPT_TOKEN(anon_sym_maxIterations);
      END_STATE();
    case 750:
      ACCEPT_TOKEN(anon_sym_minIterations);
      END_STATE();
    case 751:
      if (lookahead == 'i') ADVANCE(779);
      END_STATE();
    case 752:
      if (lookahead == 'i') ADVANCE(780);
      END_STATE();
    case 753:
      if (lookahead == 'd') ADVANCE(781);
      END_STATE();
    case 754:
      if (lookahead == 'e') ADVANCE(782);
      END_STATE();
    case 755:
      if (lookahead == 'l') ADVANCE(783);
      END_STATE();
    case 756:
      if (lookahead == 'i') ADVANCE(784);
      END_STATE();
    case 757:
      ACCEPT_TOKEN(anon_sym_showErrorBars);
      END_STATE();
    case 758:
      ACCEPT_TOKEN(anon_sym_showMinorGrid);
      END_STATE();
    case 759:
      if (lookahead == 'n') ADVANCE(785);
      END_STATE();
    case 760:
      if (lookahead == 'd') ADVANCE(786);
      END_STATE();
    case 761:
      ACCEPT_TOKEN(anon_sym_showTotalTime);
      END_STATE();
    case 762:
      if (lookahead == 'r') ADVANCE(787);
      END_STATE();
    case 763:
      ACCEPT_TOKEN(anon_sym_showWinCounts);
      END_STATE();
    case 764:
      if (lookahead == 'i') ADVANCE(788);
      END_STATE();
    case 765:
      if (lookahead == 'l') ADVANCE(789);
      END_STATE();
    case 766:
      if (lookahead == 'S') ADVANCE(790);
      END_STATE();
    case 767:
      ACCEPT_TOKEN(anon_sym_titleFontSize);
      END_STATE();
    case 768:
      if (lookahead == 'i') ADVANCE(791);
      END_STATE();
    case 769:
      if (lookahead == 'G') ADVANCE(792);
      END_STATE();
    case 770:
      if (lookahead == 'a') ADVANCE(793);
      END_STATE();
    case 771:
      if (lookahead == 'r') ADVANCE(794);
      END_STATE();
    case 772:
      if (lookahead == 't') ADVANCE(795);
      END_STATE();
    case 773:
      if (lookahead == 'r') ADVANCE(796);
      END_STATE();
    case 774:
      if (lookahead == 'y') ADVANCE(797);
      END_STATE();
    case 775:
      if (lookahead == 'e') ADVANCE(798);
      END_STATE();
    case 776:
      if (lookahead == 'r') ADVANCE(799);
      END_STATE();
    case 777:
      if (lookahead == 'r') ADVANCE(800);
      END_STATE();
    case 778:
      ACCEPT_TOKEN(anon_sym_legendPosition);
      END_STATE();
    case 779:
      if (lookahead == 't') ADVANCE(801);
      END_STATE();
    case 780:
      if (lookahead == 'o') ADVANCE(802);
      END_STATE();
    case 781:
      if (lookahead == 'O') ADVANCE(803);
      END_STATE();
    case 782:
      if (lookahead == 'l') ADVANCE(804);
      END_STATE();
    case 783:
      if (lookahead == 'e') ADVANCE(805);
      END_STATE();
    case 784:
      if (lookahead == 'o') ADVANCE(806);
      END_STATE();
    case 785:
      ACCEPT_TOKEN(anon_sym_showRegression);
      if (lookahead == 'B') ADVANCE(807);
      if (lookahead == 'L') ADVANCE(808);
      END_STATE();
    case 786:
      ACCEPT_TOKEN(anon_sym_showStdDevBand);
      END_STATE();
    case 787:
      if (lookahead == 'i') ADVANCE(809);
      END_STATE();
    case 788:
      if (lookahead == 'z') ADVANCE(810);
      END_STATE();
    case 789:
      if (lookahead == 'd') ADVANCE(811);
      END_STATE();
    case 790:
      if (lookahead == 'i') ADVANCE(812);
      END_STATE();
    case 791:
      if (lookahead == 'z') ADVANCE(813);
      END_STATE();
    case 792:
      if (lookahead == 'a') ADVANCE(814);
      END_STATE();
    case 793:
      if (lookahead == 'r') ADVANCE(815);
      END_STATE();
    case 794:
      if (lookahead == 't') ADVANCE(816);
      END_STATE();
    case 795:
      ACCEPT_TOKEN(anon_sym_drawScatterPlot);
      END_STATE();
    case 796:
      if (lookahead == 't') ADVANCE(817);
      END_STATE();
    case 797:
      ACCEPT_TOKEN(anon_sym_errorBarOpacity);
      END_STATE();
    case 798:
      if (lookahead == 's') ADVANCE(818);
      END_STATE();
    case 799:
      if (lookahead == 'k') ADVANCE(819);
      END_STATE();
    case 800:
      if (lookahead == 'k') ADVANCE(820);
      END_STATE();
    case 801:
      if (lookahead == 'y') ADVANCE(821);
      END_STATE();
    case 802:
      if (lookahead == 'n') ADVANCE(822);
      END_STATE();
    case 803:
      if (lookahead == 'p') ADVANCE(823);
      END_STATE();
    case 804:
      ACCEPT_TOKEN(anon_sym_regressionModel);
      END_STATE();
    case 805:
      ACCEPT_TOKEN(anon_sym_regressionStyle);
      END_STATE();
    case 806:
      if (lookahead == 'n') ADVANCE(824);
      END_STATE();
    case 807:
      if (lookahead == 'a') ADVANCE(825);
      END_STATE();
    case 808:
      if (lookahead == 'a') ADVANCE(826);
      END_STATE();
    case 809:
      if (lookahead == 'd') ADVANCE(827);
      END_STATE();
    case 810:
      if (lookahead == 'e') ADVANCE(828);
      END_STATE();
    case 811:
      ACCEPT_TOKEN(anon_sym_symlogThreshold);
      END_STATE();
    case 812:
      if (lookahead == 'z') ADVANCE(829);
      END_STATE();
    case 813:
      if (lookahead == 'e') ADVANCE(830);
      END_STATE();
    case 814:
      if (lookahead == 'p') ADVANCE(831);
      END_STATE();
    case 815:
      if (lookahead == 'k') ADVANCE(832);
      END_STATE();
    case 816:
      ACCEPT_TOKEN(anon_sym_drawScalingChart);
      END_STATE();
    case 817:
      ACCEPT_TOKEN(anon_sym_drawSpeedupChart);
      END_STATE();
    case 818:
      if (lookahead == 's') ADVANCE(833);
      END_STATE();
    case 819:
      if (lookahead == 's') ADVANCE(834);
      END_STATE();
    case 820:
      if (lookahead == 's') ADVANCE(835);
      END_STATE();
    case 821:
      ACCEPT_TOKEN(anon_sym_minorGridOpacity);
      END_STATE();
    case 822:
      ACCEPT_TOKEN(anon_sym_outlierDetection);
      END_STATE();
    case 823:
      if (lookahead == 'a') ADVANCE(836);
      END_STATE();
    case 824:
      ACCEPT_TOKEN(anon_sym_showDistribution);
      END_STATE();
    case 825:
      if (lookahead == 'n') ADVANCE(837);
      END_STATE();
    case 826:
      if (lookahead == 'b') ADVANCE(838);
      END_STATE();
    case 827:
      ACCEPT_TOKEN(anon_sym_showVerticalGrid);
      END_STATE();
    case 828:
      ACCEPT_TOKEN(anon_sym_subtitleFontSize);
      END_STATE();
    case 829:
      if (lookahead == 'e') ADVANCE(839);
      END_STATE();
    case 830:
      ACCEPT_TOKEN(anon_sym_axisLabelFontSize);
      END_STATE();
    case 831:
      ACCEPT_TOKEN(anon_sym_barWithinGroupGap);
      END_STATE();
    case 832:
      ACCEPT_TOKEN(anon_sym_baselineBenchmark);
      END_STATE();
    case 833:
      ACCEPT_TOKEN(anon_sym_errorBarThickness);
      END_STATE();
    case 834:
      ACCEPT_TOKEN(anon_sym_excludeBenchmarks);
      END_STATE();
    case 835:
      ACCEPT_TOKEN(anon_sym_includeBenchmarks);
      END_STATE();
    case 836:
      if (lookahead == 'c') ADVANCE(840);
      END_STATE();
    case 837:
      if (lookahead == 'd') ADVANCE(841);
      END_STATE();
    case 838:
      if (lookahead == 'e') ADVANCE(842);
      END_STATE();
    case 839:
      ACCEPT_TOKEN(anon_sym_tickLabelFontSize);
      END_STATE();
    case 840:
      if (lookahead == 'i') ADVANCE(843);
      END_STATE();
    case 841:
      ACCEPT_TOKEN(anon_sym_showRegressionBand);
      END_STATE();
    case 842:
      if (lookahead == 'l') ADVANCE(844);
      END_STATE();
    case 843:
      if (lookahead == 't') ADVANCE(845);
      END_STATE();
    case 844:
      ACCEPT_TOKEN(anon_sym_showRegressionLabel);
      END_STATE();
    case 845:
      if (lookahead == 'y') ADVANCE(846);
      END_STATE();
    case 846:
      ACCEPT_TOKEN(anon_sym_regressionBandOpacity);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0, .external_lex_state = 1},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 0},
  [22] = {.lex_state = 0},
  [23] = {.lex_state = 0},
  [24] = {.lex_state = 0},
  [25] = {.lex_state = 0},
  [26] = {.lex_state = 0},
  [27] = {.lex_state = 0},
  [28] = {.lex_state = 0},
  [29] = {.lex_state = 0},
  [30] = {.lex_state = 0},
  [31] = {.lex_state = 0},
  [32] = {.lex_state = 0},
  [33] = {.lex_state = 0},
  [34] = {.lex_state = 0},
  [35] = {.lex_state = 0},
  [36] = {.lex_state = 0},
  [37] = {.lex_state = 0},
  [38] = {.lex_state = 0},
  [39] = {.lex_state = 0},
  [40] = {.lex_state = 0},
  [41] = {.lex_state = 0},
  [42] = {.lex_state = 0},
  [43] = {.lex_state = 0},
  [44] = {.lex_state = 0},
  [45] = {.lex_state = 0},
  [46] = {.lex_state = 0},
  [47] = {.lex_state = 0},
  [48] = {.lex_state = 0},
  [49] = {.lex_state = 0},
  [50] = {.lex_state = 0},
  [51] = {.lex_state = 0},
  [52] = {.lex_state = 0},
  [53] = {.lex_state = 0},
  [54] = {.lex_state = 0},
  [55] = {.lex_state = 0},
  [56] = {.lex_state = 0},
  [57] = {.lex_state = 0},
  [58] = {.lex_state = 0},
  [59] = {.lex_state = 0},
  [60] = {.lex_state = 0},
  [61] = {.lex_state = 0},
  [62] = {.lex_state = 0},
  [63] = {.lex_state = 0},
  [64] = {.lex_state = 0},
  [65] = {.lex_state = 0},
  [66] = {.lex_state = 0},
  [67] = {.lex_state = 0},
  [68] = {.lex_state = 0},
  [69] = {.lex_state = 0},
  [70] = {.lex_state = 0},
  [71] = {.lex_state = 0},
  [72] = {.lex_state = 0},
  [73] = {.lex_state = 0},
  [74] = {.lex_state = 0},
  [75] = {.lex_state = 0},
  [76] = {.lex_state = 0},
  [77] = {.lex_state = 0},
  [78] = {.lex_state = 0},
  [79] = {.lex_state = 0},
  [80] = {.lex_state = 0},
  [81] = {.lex_state = 0},
  [82] = {.lex_state = 0},
  [83] = {.lex_state = 0},
  [84] = {.lex_state = 0},
  [85] = {.lex_state = 0},
  [86] = {.lex_state = 0},
  [87] = {.lex_state = 0},
  [88] = {.lex_state = 0},
  [89] = {.lex_state = 0},
  [90] = {.lex_state = 0},
  [91] = {.lex_state = 3},
  [92] = {.lex_state = 1},
  [93] = {.lex_state = 0},
  [94] = {.lex_state = 1},
  [95] = {.lex_state = 0},
  [96] = {.lex_state = 0},
  [97] = {.lex_state = 0},
  [98] = {.lex_state = 0},
  [99] = {.lex_state = 0},
  [100] = {.lex_state = 0},
  [101] = {.lex_state = 0},
  [102] = {.lex_state = 0},
  [103] = {.lex_state = 0},
  [104] = {.lex_state = 0},
  [105] = {.lex_state = 4},
  [106] = {.lex_state = 0},
  [107] = {.lex_state = 3},
  [108] = {.lex_state = 1},
  [109] = {.lex_state = 0},
  [110] = {.lex_state = 3},
  [111] = {.lex_state = 4},
  [112] = {.lex_state = 0},
  [113] = {.lex_state = 0},
  [114] = {.lex_state = 0},
  [115] = {.lex_state = 0},
  [116] = {.lex_state = 0},
  [117] = {.lex_state = 0},
  [118] = {.lex_state = 0},
  [119] = {.lex_state = 0},
  [120] = {.lex_state = 0},
  [121] = {.lex_state = 0},
  [122] = {.lex_state = 0},
  [123] = {.lex_state = 0},
  [124] = {.lex_state = 0},
  [125] = {.lex_state = 0},
  [126] = {.lex_state = 0},
  [127] = {.lex_state = 0},
  [128] = {.lex_state = 0},
  [129] = {.lex_state = 0},
  [130] = {.lex_state = 0},
  [131] = {.lex_state = 0},
  [132] = {.lex_state = 0},
  [133] = {.lex_state = 0},
  [134] = {.lex_state = 0},
  [135] = {.lex_state = 0},
  [136] = {.lex_state = 0},
  [137] = {.lex_state = 0},
  [138] = {.lex_state = 0},
  [139] = {.lex_state = 0},
  [140] = {.lex_state = 0},
  [141] = {.lex_state = 0},
  [142] = {.lex_state = 0},
  [143] = {.lex_state = 0},
  [144] = {.lex_state = 0},
  [145] = {.lex_state = 0},
  [146] = {.lex_state = 0},
  [147] = {.lex_state = 0},
  [148] = {.lex_state = 0},
  [149] = {.lex_state = 0},
  [150] = {.lex_state = 0},
  [151] = {.lex_state = 0},
  [152] = {.lex_state = 0},
  [153] = {.lex_state = 0},
  [154] = {.lex_state = 0},
  [155] = {.lex_state = 0},
  [156] = {.lex_state = 0},
  [157] = {.lex_state = 0},
  [158] = {.lex_state = 0},
  [159] = {.lex_state = 0},
  [160] = {.lex_state = 0},
  [161] = {.lex_state = 0, .external_lex_state = 2},
  [162] = {.lex_state = 0},
  [163] = {.lex_state = 0},
  [164] = {.lex_state = 0, .external_lex_state = 2},
  [165] = {.lex_state = 0},
  [166] = {.lex_state = 0},
  [167] = {.lex_state = 0},
  [168] = {.lex_state = 0},
  [169] = {.lex_state = 0},
  [170] = {.lex_state = 0},
  [171] = {.lex_state = 0},
  [172] = {.lex_state = 0},
  [173] = {.lex_state = 0},
  [174] = {.lex_state = 0},
  [175] = {.lex_state = 0},
  [176] = {.lex_state = 0},
  [177] = {.lex_state = 0},
  [178] = {.lex_state = 0},
  [179] = {.lex_state = 0},
  [180] = {.lex_state = 0},
  [181] = {.lex_state = 0},
  [182] = {.lex_state = 0},
  [183] = {.lex_state = 0},
  [184] = {.lex_state = 0},
  [185] = {.lex_state = 0},
  [186] = {.lex_state = 0},
  [187] = {.lex_state = 0},
  [188] = {.lex_state = 0},
  [189] = {.lex_state = 0},
  [190] = {.lex_state = 0},
  [191] = {.lex_state = 0},
  [192] = {.lex_state = 0},
  [193] = {.lex_state = 0},
  [194] = {.lex_state = 0},
  [195] = {.lex_state = 0},
  [196] = {.lex_state = 0},
  [197] = {.lex_state = 0},
  [198] = {.lex_state = 0},
  [199] = {.lex_state = 0},
  [200] = {.lex_state = 0},
  [201] = {.lex_state = 0},
  [202] = {.lex_state = 0},
  [203] = {.lex_state = 0},
  [204] = {.lex_state = 0},
  [205] = {.lex_state = 0},
  [206] = {.lex_state = 0},
  [207] = {.lex_state = 0},
  [208] = {.lex_state = 0},
  [209] = {.lex_state = 0},
  [210] = {.lex_state = 0},
  [211] = {.lex_state = 0},
  [212] = {.lex_state = 0},
  [213] = {.lex_state = 2},
  [214] = {.lex_state = 0},
  [215] = {.lex_state = 0},
  [216] = {.lex_state = 0},
  [217] = {.lex_state = 0},
  [218] = {.lex_state = 0},
  [219] = {.lex_state = 0},
  [220] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [anon_sym_use] = ACTIONS(1),
    [anon_sym_std] = ACTIONS(1),
    [anon_sym_globalSetup] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_anvil] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_spawnAnvil] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_fork] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_suite] = ACTIONS(1),
    [anon_sym_setup] = ACTIONS(1),
    [anon_sym_import] = ACTIONS(1),
    [anon_sym_declare] = ACTIONS(1),
    [anon_sym_async] = ACTIONS(1),
    [anon_sym_init] = ACTIONS(1),
    [anon_sym_helpers] = ACTIONS(1),
    [anon_sym_fixture] = ACTIONS(1),
    [anon_sym_hex] = ACTIONS(1),
    [anon_sym_shape] = ACTIONS(1),
    [anon_sym_ATfile] = ACTIONS(1),
    [anon_sym_bench] = ACTIONS(1),
    [anon_sym_tags] = ACTIONS(1),
    [anon_sym_skip] = ACTIONS(1),
    [anon_sym_validate] = ACTIONS(1),
    [anon_sym_before] = ACTIONS(1),
    [anon_sym_after] = ACTIONS(1),
    [anon_sym_each] = ACTIONS(1),
    [anon_sym_charting] = ACTIONS(1),
    [anon_sym_drawBarChart] = ACTIONS(1),
    [anon_sym_drawLineChart] = ACTIONS(1),
    [anon_sym_drawPieChart] = ACTIONS(1),
    [anon_sym_drawScatterPlot] = ACTIONS(1),
    [anon_sym_drawHistogram] = ACTIONS(1),
    [anon_sym_drawHeatmap] = ACTIONS(1),
    [anon_sym_drawBoxPlot] = ACTIONS(1),
    [anon_sym_drawAreaChart] = ACTIONS(1),
    [anon_sym_drawSpeedupChart] = ACTIONS(1),
    [anon_sym_drawScalingChart] = ACTIONS(1),
    [anon_sym_drawTable] = ACTIONS(1),
    [anon_sym_title] = ACTIONS(1),
    [anon_sym_description] = ACTIONS(1),
    [anon_sym_xlabel] = ACTIONS(1),
    [anon_sym_ylabel] = ACTIONS(1),
    [anon_sym_output] = ACTIONS(1),
    [anon_sym_sortBy] = ACTIONS(1),
    [anon_sym_sortOrder] = ACTIONS(1),
    [anon_sym_timeUnit] = ACTIONS(1),
    [anon_sym_legendPosition] = ACTIONS(1),
    [anon_sym_regressionStyle] = ACTIONS(1),
    [anon_sym_regressionModel] = ACTIONS(1),
    [anon_sym_yScale] = ACTIONS(1),
    [anon_sym_baselineBenchmark] = ACTIONS(1),
    [anon_sym_filterWinner] = ACTIONS(1),
    [anon_sym_showStats] = ACTIONS(1),
    [anon_sym_showConfig] = ACTIONS(1),
    [anon_sym_showWinCounts] = ACTIONS(1),
    [anon_sym_showGeoMean] = ACTIONS(1),
    [anon_sym_showDistribution] = ACTIONS(1),
    [anon_sym_showMemory] = ACTIONS(1),
    [anon_sym_showTotalTime] = ACTIONS(1),
    [anon_sym_showLegend] = ACTIONS(1),
    [anon_sym_showGrid] = ACTIONS(1),
    [anon_sym_showErrorBars] = ACTIONS(1),
    [anon_sym_showRegression] = ACTIONS(1),
    [anon_sym_showRegressionLabel] = ACTIONS(1),
    [anon_sym_showRSquared] = ACTIONS(1),
    [anon_sym_showEquation] = ACTIONS(1),
    [anon_sym_showRegressionBand] = ACTIONS(1),
    [anon_sym_showMinorGrid] = ACTIONS(1),
    [anon_sym_showVerticalGrid] = ACTIONS(1),
    [anon_sym_showStdDevBand] = ACTIONS(1),
    [anon_sym_roundTicks] = ACTIONS(1),
    [anon_sym_compact] = ACTIONS(1),
    [anon_sym_width] = ACTIONS(1),
    [anon_sym_height] = ACTIONS(1),
    [anon_sym_precision] = ACTIONS(1),
    [anon_sym_limit] = ACTIONS(1),
    [anon_sym_titleFontSize] = ACTIONS(1),
    [anon_sym_subtitleFontSize] = ACTIONS(1),
    [anon_sym_axisLabelFontSize] = ACTIONS(1),
    [anon_sym_tickLabelFontSize] = ACTIONS(1),
    [anon_sym_barGroupGap] = ACTIONS(1),
    [anon_sym_barWithinGroupGap] = ACTIONS(1),
    [anon_sym_barWidth] = ACTIONS(1),
    [anon_sym_ciLevel] = ACTIONS(1),
    [anon_sym_minSpeedup] = ACTIONS(1),
    [anon_sym_axisThickness] = ACTIONS(1),
    [anon_sym_yAxisMin] = ACTIONS(1),
    [anon_sym_yAxisMax] = ACTIONS(1),
    [anon_sym_gridOpacity] = ACTIONS(1),
    [anon_sym_minorGridOpacity] = ACTIONS(1),
    [anon_sym_errorBarOpacity] = ACTIONS(1),
    [anon_sym_errorBarThickness] = ACTIONS(1),
    [anon_sym_regressionBandOpacity] = ACTIONS(1),
    [anon_sym_symlogThreshold] = ACTIONS(1),
    [anon_sym_includeBenchmarks] = ACTIONS(1),
    [anon_sym_excludeBenchmarks] = ACTIONS(1),
    [anon_sym_iterations] = ACTIONS(1),
    [anon_sym_warmup] = ACTIONS(1),
    [anon_sym_timeout] = ACTIONS(1),
    [anon_sym_requires] = ACTIONS(1),
    [anon_sym_order] = ACTIONS(1),
    [anon_sym_compare] = ACTIONS(1),
    [anon_sym_baseline] = ACTIONS(1),
    [anon_sym_mode] = ACTIONS(1),
    [anon_sym_targetTime] = ACTIONS(1),
    [anon_sym_minIterations] = ACTIONS(1),
    [anon_sym_maxIterations] = ACTIONS(1),
    [anon_sym_sink] = ACTIONS(1),
    [anon_sym_outlierDetection] = ACTIONS(1),
    [anon_sym_cvThreshold] = ACTIONS(1),
    [anon_sym_count] = ACTIONS(1),
    [anon_sym_memory] = ACTIONS(1),
    [anon_sym_concurrency] = ACTIONS(1),
    [anon_sym_go] = ACTIONS(1),
    [anon_sym_ts] = ACTIONS(1),
    [anon_sym_typescript] = ACTIONS(1),
    [anon_sym_rust] = ACTIONS(1),
    [anon_sym_python] = ACTIONS(1),
    [anon_sym_DQUOTE] = ACTIONS(1),
    [anon_sym_SQUOTE] = ACTIONS(1),
    [sym_escape_sequence] = ACTIONS(1),
    [sym_number] = ACTIONS(1),
    [sym_float] = ACTIONS(1),
    [anon_sym_ms] = ACTIONS(1),
    [anon_sym_s] = ACTIONS(1),
    [anon_sym_m] = ACTIONS(1),
    [anon_sym_true] = ACTIONS(1),
    [anon_sym_false] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [sym_comment] = ACTIONS(3),
    [sym_embedded_code] = ACTIONS(1),
    [sym__embedded_code_start] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(216),
    [sym_use_statement] = STATE(76),
    [sym_global_setup] = STATE(95),
    [sym_suite] = STATE(96),
    [aux_sym_source_file_repeat1] = STATE(76),
    [aux_sym_source_file_repeat2] = STATE(96),
    [ts_builtin_sym_end] = ACTIONS(5),
    [anon_sym_use] = ACTIONS(7),
    [anon_sym_globalSetup] = ACTIONS(9),
    [anon_sym_suite] = ACTIONS(11),
    [sym_comment] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(13), 1,
      anon_sym_RPAREN,
    STATE(133), 1,
      sym_chart_param,
    STATE(196), 1,
      sym_chart_params,
    STATE(198), 1,
      sym_chart_param_name,
    ACTIONS(15), 2,
      anon_sym_title,
      anon_sym_showRegression,
    ACTIONS(17), 56,
      anon_sym_description,
      anon_sym_xlabel,
      anon_sym_ylabel,
      anon_sym_output,
      anon_sym_sortBy,
      anon_sym_sortOrder,
      anon_sym_timeUnit,
      anon_sym_legendPosition,
      anon_sym_regressionStyle,
      anon_sym_regressionModel,
      anon_sym_yScale,
      anon_sym_baselineBenchmark,
      anon_sym_filterWinner,
      anon_sym_showStats,
      anon_sym_showConfig,
      anon_sym_showWinCounts,
      anon_sym_showGeoMean,
      anon_sym_showDistribution,
      anon_sym_showMemory,
      anon_sym_showTotalTime,
      anon_sym_showLegend,
      anon_sym_showGrid,
      anon_sym_showErrorBars,
      anon_sym_showRegressionLabel,
      anon_sym_showRSquared,
      anon_sym_showEquation,
      anon_sym_showRegressionBand,
      anon_sym_showMinorGrid,
      anon_sym_showVerticalGrid,
      anon_sym_showStdDevBand,
      anon_sym_roundTicks,
      anon_sym_compact,
      anon_sym_width,
      anon_sym_height,
      anon_sym_precision,
      anon_sym_limit,
      anon_sym_titleFontSize,
      anon_sym_subtitleFontSize,
      anon_sym_axisLabelFontSize,
      anon_sym_tickLabelFontSize,
      anon_sym_barGroupGap,
      anon_sym_barWithinGroupGap,
      anon_sym_barWidth,
      anon_sym_ciLevel,
      anon_sym_minSpeedup,
      anon_sym_axisThickness,
      anon_sym_yAxisMin,
      anon_sym_yAxisMax,
      anon_sym_gridOpacity,
      anon_sym_minorGridOpacity,
      anon_sym_errorBarOpacity,
      anon_sym_errorBarThickness,
      anon_sym_regressionBandOpacity,
      anon_sym_symlogThreshold,
      anon_sym_includeBenchmarks,
      anon_sym_excludeBenchmarks,
  [78] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(19), 1,
      anon_sym_RPAREN,
    STATE(156), 1,
      sym_chart_param,
    STATE(198), 1,
      sym_chart_param_name,
    ACTIONS(15), 2,
      anon_sym_title,
      anon_sym_showRegression,
    ACTIONS(17), 56,
      anon_sym_description,
      anon_sym_xlabel,
      anon_sym_ylabel,
      anon_sym_output,
      anon_sym_sortBy,
      anon_sym_sortOrder,
      anon_sym_timeUnit,
      anon_sym_legendPosition,
      anon_sym_regressionStyle,
      anon_sym_regressionModel,
      anon_sym_yScale,
      anon_sym_baselineBenchmark,
      anon_sym_filterWinner,
      anon_sym_showStats,
      anon_sym_showConfig,
      anon_sym_showWinCounts,
      anon_sym_showGeoMean,
      anon_sym_showDistribution,
      anon_sym_showMemory,
      anon_sym_showTotalTime,
      anon_sym_showLegend,
      anon_sym_showGrid,
      anon_sym_showErrorBars,
      anon_sym_showRegressionLabel,
      anon_sym_showRSquared,
      anon_sym_showEquation,
      anon_sym_showRegressionBand,
      anon_sym_showMinorGrid,
      anon_sym_showVerticalGrid,
      anon_sym_showStdDevBand,
      anon_sym_roundTicks,
      anon_sym_compact,
      anon_sym_width,
      anon_sym_height,
      anon_sym_precision,
      anon_sym_limit,
      anon_sym_titleFontSize,
      anon_sym_subtitleFontSize,
      anon_sym_axisLabelFontSize,
      anon_sym_tickLabelFontSize,
      anon_sym_barGroupGap,
      anon_sym_barWithinGroupGap,
      anon_sym_barWidth,
      anon_sym_ciLevel,
      anon_sym_minSpeedup,
      anon_sym_axisThickness,
      anon_sym_yAxisMin,
      anon_sym_yAxisMax,
      anon_sym_gridOpacity,
      anon_sym_minorGridOpacity,
      anon_sym_errorBarOpacity,
      anon_sym_errorBarThickness,
      anon_sym_regressionBandOpacity,
      anon_sym_symlogThreshold,
      anon_sym_includeBenchmarks,
      anon_sym_excludeBenchmarks,
  [153] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(21), 1,
      anon_sym_RPAREN,
    STATE(156), 1,
      sym_chart_param,
    STATE(198), 1,
      sym_chart_param_name,
    ACTIONS(15), 2,
      anon_sym_title,
      anon_sym_showRegression,
    ACTIONS(17), 56,
      anon_sym_description,
      anon_sym_xlabel,
      anon_sym_ylabel,
      anon_sym_output,
      anon_sym_sortBy,
      anon_sym_sortOrder,
      anon_sym_timeUnit,
      anon_sym_legendPosition,
      anon_sym_regressionStyle,
      anon_sym_regressionModel,
      anon_sym_yScale,
      anon_sym_baselineBenchmark,
      anon_sym_filterWinner,
      anon_sym_showStats,
      anon_sym_showConfig,
      anon_sym_showWinCounts,
      anon_sym_showGeoMean,
      anon_sym_showDistribution,
      anon_sym_showMemory,
      anon_sym_showTotalTime,
      anon_sym_showLegend,
      anon_sym_showGrid,
      anon_sym_showErrorBars,
      anon_sym_showRegressionLabel,
      anon_sym_showRSquared,
      anon_sym_showEquation,
      anon_sym_showRegressionBand,
      anon_sym_showMinorGrid,
      anon_sym_showVerticalGrid,
      anon_sym_showStdDevBand,
      anon_sym_roundTicks,
      anon_sym_compact,
      anon_sym_width,
      anon_sym_height,
      anon_sym_precision,
      anon_sym_limit,
      anon_sym_titleFontSize,
      anon_sym_subtitleFontSize,
      anon_sym_axisLabelFontSize,
      anon_sym_tickLabelFontSize,
      anon_sym_barGroupGap,
      anon_sym_barWithinGroupGap,
      anon_sym_barWidth,
      anon_sym_ciLevel,
      anon_sym_minSpeedup,
      anon_sym_axisThickness,
      anon_sym_yAxisMin,
      anon_sym_yAxisMax,
      anon_sym_gridOpacity,
      anon_sym_minorGridOpacity,
      anon_sym_errorBarOpacity,
      anon_sym_errorBarThickness,
      anon_sym_regressionBandOpacity,
      anon_sym_symlogThreshold,
      anon_sym_includeBenchmarks,
      anon_sym_excludeBenchmarks,
  [228] = 5,
    ACTIONS(3), 1,
      sym_comment,
    STATE(156), 1,
      sym_chart_param,
    STATE(198), 1,
      sym_chart_param_name,
    ACTIONS(15), 2,
      anon_sym_title,
      anon_sym_showRegression,
    ACTIONS(17), 56,
      anon_sym_description,
      anon_sym_xlabel,
      anon_sym_ylabel,
      anon_sym_output,
      anon_sym_sortBy,
      anon_sym_sortOrder,
      anon_sym_timeUnit,
      anon_sym_legendPosition,
      anon_sym_regressionStyle,
      anon_sym_regressionModel,
      anon_sym_yScale,
      anon_sym_baselineBenchmark,
      anon_sym_filterWinner,
      anon_sym_showStats,
      anon_sym_showConfig,
      anon_sym_showWinCounts,
      anon_sym_showGeoMean,
      anon_sym_showDistribution,
      anon_sym_showMemory,
      anon_sym_showTotalTime,
      anon_sym_showLegend,
      anon_sym_showGrid,
      anon_sym_showErrorBars,
      anon_sym_showRegressionLabel,
      anon_sym_showRSquared,
      anon_sym_showEquation,
      anon_sym_showRegressionBand,
      anon_sym_showMinorGrid,
      anon_sym_showVerticalGrid,
      anon_sym_showStdDevBand,
      anon_sym_roundTicks,
      anon_sym_compact,
      anon_sym_width,
      anon_sym_height,
      anon_sym_precision,
      anon_sym_limit,
      anon_sym_titleFontSize,
      anon_sym_subtitleFontSize,
      anon_sym_axisLabelFontSize,
      anon_sym_tickLabelFontSize,
      anon_sym_barGroupGap,
      anon_sym_barWithinGroupGap,
      anon_sym_barWidth,
      anon_sym_ciLevel,
      anon_sym_minSpeedup,
      anon_sym_axisThickness,
      anon_sym_yAxisMin,
      anon_sym_yAxisMax,
      anon_sym_gridOpacity,
      anon_sym_minorGridOpacity,
      anon_sym_errorBarOpacity,
      anon_sym_errorBarThickness,
      anon_sym_regressionBandOpacity,
      anon_sym_symlogThreshold,
      anon_sym_includeBenchmarks,
      anon_sym_excludeBenchmarks,
  [300] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(23), 1,
      anon_sym_RBRACE,
    ACTIONS(25), 1,
      anon_sym_tags,
    ACTIONS(27), 1,
      anon_sym_skip,
    ACTIONS(29), 1,
      anon_sym_validate,
    ACTIONS(31), 1,
      anon_sym_before,
    ACTIONS(33), 1,
      anon_sym_after,
    ACTIONS(35), 1,
      anon_sym_each,
    STATE(181), 1,
      sym_language_tag,
    STATE(220), 1,
      sym_property_name,
    ACTIONS(39), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
    STATE(7), 10,
      sym__benchmark_item,
      sym_tags_property,
      sym_skip_hook,
      sym_validate_hook,
      sym_before_hook,
      sym_after_hook,
      sym_each_hook,
      sym_property,
      sym_language_implementation,
      aux_sym_benchmark_body_repeat1,
    ACTIONS(37), 18,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
  [370] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(41), 1,
      anon_sym_RBRACE,
    ACTIONS(43), 1,
      anon_sym_tags,
    ACTIONS(46), 1,
      anon_sym_skip,
    ACTIONS(49), 1,
      anon_sym_validate,
    ACTIONS(52), 1,
      anon_sym_before,
    ACTIONS(55), 1,
      anon_sym_after,
    ACTIONS(58), 1,
      anon_sym_each,
    STATE(181), 1,
      sym_language_tag,
    STATE(220), 1,
      sym_property_name,
    ACTIONS(64), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
    STATE(7), 10,
      sym__benchmark_item,
      sym_tags_property,
      sym_skip_hook,
      sym_validate_hook,
      sym_before_hook,
      sym_after_hook,
      sym_each_hook,
      sym_property,
      sym_language_implementation,
      aux_sym_benchmark_body_repeat1,
    ACTIONS(61), 18,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
  [440] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(25), 1,
      anon_sym_tags,
    ACTIONS(27), 1,
      anon_sym_skip,
    ACTIONS(29), 1,
      anon_sym_validate,
    ACTIONS(31), 1,
      anon_sym_before,
    ACTIONS(33), 1,
      anon_sym_after,
    ACTIONS(35), 1,
      anon_sym_each,
    ACTIONS(67), 1,
      anon_sym_RBRACE,
    STATE(181), 1,
      sym_language_tag,
    STATE(220), 1,
      sym_property_name,
    ACTIONS(39), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
    STATE(6), 10,
      sym__benchmark_item,
      sym_tags_property,
      sym_skip_hook,
      sym_validate_hook,
      sym_before_hook,
      sym_after_hook,
      sym_each_hook,
      sym_property,
      sym_language_implementation,
      aux_sym_benchmark_body_repeat1,
    ACTIONS(37), 18,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
  [510] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(69), 39,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_shape,
      anon_sym_bench,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
      anon_sym_RBRACK,
  [555] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(71), 39,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_shape,
      anon_sym_bench,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
      anon_sym_RBRACK,
  [600] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(73), 38,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_shape,
      anon_sym_bench,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [644] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(75), 38,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_shape,
      anon_sym_bench,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [688] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(77), 38,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_shape,
      anon_sym_bench,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [732] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(79), 38,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_shape,
      anon_sym_bench,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [776] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(81), 38,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_shape,
      anon_sym_bench,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [820] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(83), 38,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_shape,
      anon_sym_bench,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [864] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(85), 38,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_shape,
      anon_sym_bench,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [908] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(87), 37,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
      anon_sym_hex,
      anon_sym_shape,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [951] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(89), 37,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
      anon_sym_hex,
      anon_sym_shape,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [994] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(91), 36,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_shape,
      anon_sym_bench,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1036] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(93), 1,
      anon_sym_RBRACE,
    ACTIONS(95), 1,
      anon_sym_hex,
    ACTIONS(97), 1,
      anon_sym_shape,
    STATE(181), 1,
      sym_language_tag,
    STATE(219), 1,
      sym_property_name,
    ACTIONS(39), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
    STATE(22), 6,
      sym__fixture_item,
      sym_hex_property,
      sym_shape_property,
      sym_property,
      sym_language_implementation,
      aux_sym_fixture_body_repeat1,
    ACTIONS(37), 18,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
  [1090] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(95), 1,
      anon_sym_hex,
    ACTIONS(97), 1,
      anon_sym_shape,
    ACTIONS(99), 1,
      anon_sym_RBRACE,
    STATE(181), 1,
      sym_language_tag,
    STATE(219), 1,
      sym_property_name,
    ACTIONS(39), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
    STATE(23), 6,
      sym__fixture_item,
      sym_hex_property,
      sym_shape_property,
      sym_property,
      sym_language_implementation,
      aux_sym_fixture_body_repeat1,
    ACTIONS(37), 18,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
  [1144] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(101), 1,
      anon_sym_RBRACE,
    ACTIONS(103), 1,
      anon_sym_hex,
    ACTIONS(106), 1,
      anon_sym_shape,
    STATE(181), 1,
      sym_language_tag,
    STATE(219), 1,
      sym_property_name,
    ACTIONS(112), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
    STATE(23), 6,
      sym__fixture_item,
      sym_hex_property,
      sym_shape_property,
      sym_property,
      sym_language_implementation,
      aux_sym_fixture_body_repeat1,
    ACTIONS(109), 18,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
  [1198] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      anon_sym_ms,
    STATE(12), 1,
      sym_duration_unit,
    ACTIONS(119), 2,
      anon_sym_s,
      anon_sym_m,
    ACTIONS(115), 30,
      anon_sym_RBRACE,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1244] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(121), 1,
      anon_sym_globalSetup,
    ACTIONS(124), 1,
      anon_sym_RBRACE,
    ACTIONS(126), 1,
      anon_sym_setup,
    ACTIONS(129), 1,
      anon_sym_fixture,
    ACTIONS(132), 1,
      anon_sym_bench,
    ACTIONS(135), 1,
      anon_sym_after,
    STATE(176), 1,
      sym_property_name,
    STATE(25), 8,
      sym_global_setup,
      sym__suite_item,
      sym_setup_block,
      sym_fixture,
      sym_benchmark,
      sym_after_block,
      sym_property,
      aux_sym_suite_body_repeat1,
    ACTIONS(138), 18,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
  [1299] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      anon_sym_globalSetup,
    ACTIONS(141), 1,
      anon_sym_RBRACE,
    ACTIONS(143), 1,
      anon_sym_setup,
    ACTIONS(145), 1,
      anon_sym_fixture,
    ACTIONS(147), 1,
      anon_sym_bench,
    ACTIONS(149), 1,
      anon_sym_after,
    STATE(176), 1,
      sym_property_name,
    STATE(27), 8,
      sym_global_setup,
      sym__suite_item,
      sym_setup_block,
      sym_fixture,
      sym_benchmark,
      sym_after_block,
      sym_property,
      aux_sym_suite_body_repeat1,
    ACTIONS(37), 18,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
  [1354] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      anon_sym_globalSetup,
    ACTIONS(143), 1,
      anon_sym_setup,
    ACTIONS(145), 1,
      anon_sym_fixture,
    ACTIONS(147), 1,
      anon_sym_bench,
    ACTIONS(149), 1,
      anon_sym_after,
    ACTIONS(151), 1,
      anon_sym_RBRACE,
    STATE(176), 1,
      sym_property_name,
    STATE(25), 8,
      sym_global_setup,
      sym__suite_item,
      sym_setup_block,
      sym_fixture,
      sym_benchmark,
      sym_after_block,
      sym_property,
      aux_sym_suite_body_repeat1,
    ACTIONS(37), 18,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
  [1409] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(153), 32,
      anon_sym_RBRACE,
      anon_sym_hex,
      anon_sym_shape,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1447] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(155), 30,
      anon_sym_RBRACE,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1483] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(157), 30,
      anon_sym_RBRACE,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1519] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      anon_sym_ms,
    STATE(12), 1,
      sym_duration_unit,
    ACTIONS(119), 2,
      anon_sym_s,
      anon_sym_m,
    ACTIONS(115), 26,
      anon_sym_RBRACE,
      anon_sym_hex,
      anon_sym_shape,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1561] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(159), 30,
      anon_sym_RBRACE,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1597] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(161), 30,
      anon_sym_RBRACE,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1633] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(163), 30,
      anon_sym_RBRACE,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1669] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(165), 30,
      anon_sym_RBRACE,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1705] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(167), 30,
      anon_sym_RBRACE,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1741] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(169), 30,
      anon_sym_RBRACE,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1777] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(171), 30,
      anon_sym_RBRACE,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1813] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      anon_sym_ms,
    STATE(12), 1,
      sym_duration_unit,
    ACTIONS(119), 2,
      anon_sym_s,
      anon_sym_m,
    ACTIONS(115), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
      anon_sym_after,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
  [1853] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(173), 26,
      ts_builtin_sym_end,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_suite,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
      anon_sym_after,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
  [1885] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(175), 26,
      anon_sym_RBRACE,
      anon_sym_hex,
      anon_sym_shape,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1917] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(177), 26,
      ts_builtin_sym_end,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_suite,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
      anon_sym_after,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
  [1949] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(179), 26,
      anon_sym_RBRACE,
      anon_sym_hex,
      anon_sym_shape,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1981] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(181), 26,
      anon_sym_RBRACE,
      anon_sym_hex,
      anon_sym_shape,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2013] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(183), 26,
      ts_builtin_sym_end,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_suite,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
      anon_sym_after,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
  [2045] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(185), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
      anon_sym_after,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
  [2075] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(187), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
      anon_sym_after,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
  [2105] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(189), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
      anon_sym_after,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
  [2135] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(191), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
      anon_sym_after,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
  [2165] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(193), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
      anon_sym_after,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
  [2195] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(195), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
      anon_sym_after,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
  [2225] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(197), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
      anon_sym_after,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
  [2255] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(199), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
      anon_sym_after,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
  [2285] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(201), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
      anon_sym_after,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
  [2315] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(203), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
      anon_sym_after,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
  [2345] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(205), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
      anon_sym_after,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
  [2375] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(207), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
      anon_sym_after,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
  [2405] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(209), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
      anon_sym_after,
      anon_sym_description,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_compare,
      anon_sym_baseline,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_minIterations,
      anon_sym_maxIterations,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_concurrency,
  [2435] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(211), 1,
      sym_identifier,
    ACTIONS(213), 1,
      anon_sym_DQUOTE,
    ACTIONS(215), 1,
      anon_sym_SQUOTE,
    ACTIONS(217), 1,
      sym_number,
    ACTIONS(219), 1,
      sym_float,
    ACTIONS(223), 1,
      anon_sym_LBRACK,
    ACTIONS(221), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(20), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2468] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(211), 1,
      sym_identifier,
    ACTIONS(213), 1,
      anon_sym_DQUOTE,
    ACTIONS(215), 1,
      anon_sym_SQUOTE,
    ACTIONS(219), 1,
      sym_float,
    ACTIONS(223), 1,
      anon_sym_LBRACK,
    ACTIONS(225), 1,
      sym_number,
    ACTIONS(221), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(20), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2501] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(213), 1,
      anon_sym_DQUOTE,
    ACTIONS(215), 1,
      anon_sym_SQUOTE,
    ACTIONS(223), 1,
      anon_sym_LBRACK,
    ACTIONS(227), 1,
      sym_identifier,
    ACTIONS(229), 1,
      sym_number,
    ACTIONS(231), 1,
      sym_float,
    ACTIONS(221), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(160), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2534] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(211), 1,
      sym_identifier,
    ACTIONS(213), 1,
      anon_sym_DQUOTE,
    ACTIONS(215), 1,
      anon_sym_SQUOTE,
    ACTIONS(219), 1,
      sym_float,
    ACTIONS(223), 1,
      anon_sym_LBRACK,
    ACTIONS(233), 1,
      sym_number,
    ACTIONS(221), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(20), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2567] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(235), 1,
      anon_sym_RBRACE,
    ACTIONS(237), 1,
      anon_sym_import,
    ACTIONS(239), 1,
      anon_sym_declare,
    ACTIONS(241), 1,
      anon_sym_async,
    ACTIONS(243), 1,
      anon_sym_init,
    ACTIONS(245), 1,
      anon_sym_helpers,
    STATE(64), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [2597] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(237), 1,
      anon_sym_import,
    ACTIONS(239), 1,
      anon_sym_declare,
    ACTIONS(241), 1,
      anon_sym_async,
    ACTIONS(243), 1,
      anon_sym_init,
    ACTIONS(245), 1,
      anon_sym_helpers,
    ACTIONS(247), 1,
      anon_sym_RBRACE,
    STATE(66), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [2627] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(199), 1,
      sym_chart_function_name,
    ACTIONS(249), 11,
      anon_sym_drawBarChart,
      anon_sym_drawLineChart,
      anon_sym_drawPieChart,
      anon_sym_drawScatterPlot,
      anon_sym_drawHistogram,
      anon_sym_drawHeatmap,
      anon_sym_drawBoxPlot,
      anon_sym_drawAreaChart,
      anon_sym_drawSpeedupChart,
      anon_sym_drawScalingChart,
      anon_sym_drawTable,
  [2647] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(251), 1,
      anon_sym_RBRACE,
    ACTIONS(253), 1,
      anon_sym_import,
    ACTIONS(256), 1,
      anon_sym_declare,
    ACTIONS(259), 1,
      anon_sym_async,
    ACTIONS(262), 1,
      anon_sym_init,
    ACTIONS(265), 1,
      anon_sym_helpers,
    STATE(66), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [2677] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(213), 1,
      anon_sym_DQUOTE,
    ACTIONS(215), 1,
      anon_sym_SQUOTE,
    ACTIONS(223), 1,
      anon_sym_LBRACK,
    ACTIONS(268), 1,
      sym_number,
    ACTIONS(270), 1,
      sym_float,
    ACTIONS(272), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(171), 4,
      sym__chart_value,
      sym_string,
      sym_boolean,
      sym_string_array,
  [2706] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(274), 1,
      anon_sym_RBRACE,
    STATE(181), 1,
      sym_language_tag,
    STATE(73), 2,
      sym_language_implementation,
      aux_sym_hook_grouped_repeat1,
    ACTIONS(39), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2727] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(276), 1,
      anon_sym_COLON,
    STATE(205), 1,
      sym_language_tag,
    STATE(29), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(39), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2748] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(276), 1,
      anon_sym_COLON,
    STATE(205), 1,
      sym_language_tag,
    STATE(33), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(39), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2769] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(276), 1,
      anon_sym_COLON,
    STATE(205), 1,
      sym_language_tag,
    STATE(35), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(39), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2790] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(278), 1,
      anon_sym_RBRACE,
    STATE(181), 1,
      sym_language_tag,
    STATE(72), 2,
      sym_language_implementation,
      aux_sym_hook_grouped_repeat1,
    ACTIONS(280), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2811] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(283), 1,
      anon_sym_RBRACE,
    STATE(181), 1,
      sym_language_tag,
    STATE(72), 2,
      sym_language_implementation,
      aux_sym_hook_grouped_repeat1,
    ACTIONS(39), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2832] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(276), 1,
      anon_sym_COLON,
    STATE(205), 1,
      sym_language_tag,
    STATE(36), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(39), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2853] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(276), 1,
      anon_sym_COLON,
    STATE(205), 1,
      sym_language_tag,
    STATE(38), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(39), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2874] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(7), 1,
      anon_sym_use,
    ACTIONS(9), 1,
      anon_sym_globalSetup,
    ACTIONS(11), 1,
      anon_sym_suite,
    ACTIONS(285), 1,
      ts_builtin_sym_end,
    STATE(99), 1,
      sym_global_setup,
    STATE(81), 2,
      sym_use_statement,
      aux_sym_source_file_repeat1,
    STATE(97), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [2901] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(287), 1,
      sym_identifier,
    ACTIONS(290), 1,
      anon_sym_RBRACE,
    ACTIONS(292), 1,
      anon_sym_anvil,
    STATE(77), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(117), 2,
      sym_anvil_call,
      sym_function_call,
  [2922] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(295), 1,
      sym_identifier,
    ACTIONS(297), 1,
      anon_sym_RBRACE,
    ACTIONS(299), 1,
      anon_sym_anvil,
    STATE(79), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(117), 2,
      sym_anvil_call,
      sym_function_call,
  [2943] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(295), 1,
      sym_identifier,
    ACTIONS(299), 1,
      anon_sym_anvil,
    ACTIONS(301), 1,
      anon_sym_RBRACE,
    STATE(77), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(117), 2,
      sym_anvil_call,
      sym_function_call,
  [2964] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(303), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [2976] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(307), 1,
      anon_sym_use,
    STATE(81), 2,
      sym_use_statement,
      aux_sym_source_file_repeat1,
    ACTIONS(305), 3,
      ts_builtin_sym_end,
      anon_sym_globalSetup,
      anon_sym_suite,
  [2992] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(310), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3004] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(312), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3016] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(119), 1,
      anon_sym_m,
    STATE(12), 1,
      sym_duration_unit,
    ACTIONS(115), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
    ACTIONS(117), 2,
      anon_sym_ms,
      anon_sym_s,
  [3034] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(314), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3046] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(316), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3058] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(318), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3070] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(320), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3082] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(151), 1,
      sym_language_tag,
    ACTIONS(39), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [3096] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(213), 1,
      anon_sym_DQUOTE,
    ACTIONS(215), 1,
      anon_sym_SQUOTE,
    ACTIONS(322), 1,
      anon_sym_ATfile,
    STATE(44), 2,
      sym_file_ref,
      sym_string,
  [3113] = 5,
    ACTIONS(324), 1,
      anon_sym_SQUOTE,
    ACTIONS(328), 1,
      sym_comment,
    STATE(110), 1,
      aux_sym_single_string_content_repeat1,
    STATE(212), 1,
      sym_single_string_content,
    ACTIONS(326), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [3130] = 5,
    ACTIONS(324), 1,
      anon_sym_DQUOTE,
    ACTIONS(328), 1,
      sym_comment,
    STATE(94), 1,
      aux_sym_string_content_repeat1,
    STATE(214), 1,
      sym_string_content,
    ACTIONS(330), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [3147] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(332), 1,
      sym_identifier,
    ACTIONS(334), 1,
      anon_sym_RPAREN,
    STATE(135), 1,
      sym_argument,
    STATE(218), 1,
      sym_argument_list,
  [3163] = 4,
    ACTIONS(328), 1,
      sym_comment,
    ACTIONS(336), 1,
      anon_sym_DQUOTE,
    STATE(108), 1,
      aux_sym_string_content_repeat1,
    ACTIONS(338), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [3177] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_suite,
    ACTIONS(285), 1,
      ts_builtin_sym_end,
    STATE(97), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3191] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_suite,
    ACTIONS(285), 1,
      ts_builtin_sym_end,
    STATE(100), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3205] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_suite,
    ACTIONS(340), 1,
      ts_builtin_sym_end,
    STATE(100), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3219] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(342), 1,
      anon_sym_LBRACE,
    ACTIONS(344), 1,
      anon_sym_LPAREN,
    STATE(88), 2,
      sym_code_block,
      sym_paren_code_block,
  [3233] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_suite,
    ACTIONS(340), 1,
      ts_builtin_sym_end,
    STATE(101), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3247] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(346), 1,
      ts_builtin_sym_end,
    ACTIONS(348), 1,
      anon_sym_suite,
    STATE(100), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3261] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_suite,
    ACTIONS(351), 1,
      ts_builtin_sym_end,
    STATE(100), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3275] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(353), 4,
      ts_builtin_sym_end,
      anon_sym_use,
      anon_sym_globalSetup,
      anon_sym_suite,
  [3285] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(213), 1,
      anon_sym_DQUOTE,
    ACTIONS(215), 1,
      anon_sym_SQUOTE,
    ACTIONS(355), 1,
      anon_sym_RBRACK,
    STATE(166), 1,
      sym_string,
  [3301] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(332), 1,
      sym_identifier,
    ACTIONS(357), 1,
      anon_sym_RPAREN,
    STATE(135), 1,
      sym_argument,
    STATE(204), 1,
      sym_argument_list,
  [3317] = 4,
    ACTIONS(328), 1,
      sym_comment,
    ACTIONS(359), 1,
      anon_sym_LBRACE,
    ACTIONS(361), 1,
      sym_inline_code,
    STATE(37), 2,
      sym__code_or_inline,
      sym_code_block,
  [3331] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(213), 1,
      anon_sym_DQUOTE,
    ACTIONS(215), 1,
      anon_sym_SQUOTE,
    ACTIONS(363), 1,
      anon_sym_RBRACK,
    STATE(166), 1,
      sym_string,
  [3347] = 4,
    ACTIONS(328), 1,
      sym_comment,
    ACTIONS(365), 1,
      anon_sym_SQUOTE,
    STATE(107), 1,
      aux_sym_single_string_content_repeat1,
    ACTIONS(367), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [3361] = 4,
    ACTIONS(328), 1,
      sym_comment,
    ACTIONS(370), 1,
      anon_sym_DQUOTE,
    STATE(108), 1,
      aux_sym_string_content_repeat1,
    ACTIONS(372), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [3375] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(375), 1,
      anon_sym_LBRACE,
    ACTIONS(377), 1,
      anon_sym_LPAREN,
    STATE(56), 1,
      sym_fixture_body,
    STATE(153), 1,
      sym_fixture_params,
  [3391] = 4,
    ACTIONS(328), 1,
      sym_comment,
    ACTIONS(379), 1,
      anon_sym_SQUOTE,
    STATE(107), 1,
      aux_sym_single_string_content_repeat1,
    ACTIONS(381), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [3405] = 4,
    ACTIONS(328), 1,
      sym_comment,
    ACTIONS(359), 1,
      anon_sym_LBRACE,
    ACTIONS(383), 1,
      sym_inline_code,
    STATE(28), 2,
      sym__code_or_inline,
      sym_code_block,
  [3419] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(385), 1,
      anon_sym_RBRACE,
    ACTIONS(387), 1,
      anon_sym_charting,
    STATE(113), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [3433] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(389), 1,
      anon_sym_RBRACE,
    ACTIONS(391), 1,
      anon_sym_charting,
    STATE(113), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [3447] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(213), 1,
      anon_sym_DQUOTE,
    ACTIONS(215), 1,
      anon_sym_SQUOTE,
    ACTIONS(394), 1,
      anon_sym_RBRACK,
    STATE(126), 1,
      sym_string,
  [3463] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(387), 1,
      anon_sym_charting,
    ACTIONS(396), 1,
      anon_sym_RBRACE,
    STATE(112), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [3477] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(213), 1,
      anon_sym_DQUOTE,
    ACTIONS(215), 1,
      anon_sym_SQUOTE,
    STATE(201), 1,
      sym_string,
  [3490] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(400), 1,
      anon_sym_RBRACE,
    ACTIONS(398), 2,
      anon_sym_anvil,
      sym_identifier,
  [3501] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(402), 1,
      anon_sym_RPAREN,
    ACTIONS(404), 1,
      anon_sym_COMMA,
    STATE(118), 1,
      aux_sym_fixture_params_repeat1,
  [3514] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(332), 1,
      sym_identifier,
    ACTIONS(407), 1,
      anon_sym_RPAREN,
    STATE(144), 1,
      sym_argument,
  [3527] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(411), 1,
      anon_sym_RBRACE,
    ACTIONS(409), 2,
      anon_sym_anvil,
      sym_identifier,
  [3538] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(413), 1,
      anon_sym_RPAREN,
    ACTIONS(415), 1,
      anon_sym_COMMA,
    STATE(121), 1,
      aux_sym_argument_list_repeat1,
  [3551] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(418), 1,
      sym_identifier,
    ACTIONS(420), 1,
      anon_sym_RPAREN,
    STATE(170), 1,
      sym_fixture_param,
  [3564] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(422), 1,
      anon_sym_RPAREN,
    ACTIONS(424), 1,
      anon_sym_COMMA,
    STATE(123), 1,
      aux_sym_chart_params_repeat1,
  [3577] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(427), 1,
      anon_sym_COMMA,
    ACTIONS(430), 1,
      anon_sym_RBRACK,
    STATE(124), 1,
      aux_sym_string_array_repeat1,
  [3590] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(434), 1,
      anon_sym_RBRACE,
    ACTIONS(432), 2,
      anon_sym_anvil,
      sym_identifier,
  [3601] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(436), 1,
      anon_sym_COMMA,
    ACTIONS(438), 1,
      anon_sym_RBRACK,
    STATE(130), 1,
      aux_sym_string_array_repeat1,
  [3614] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(213), 1,
      anon_sym_DQUOTE,
    ACTIONS(215), 1,
      anon_sym_SQUOTE,
    STATE(193), 1,
      sym_string,
  [3627] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(442), 1,
      anon_sym_RBRACE,
    ACTIONS(440), 2,
      anon_sym_anvil,
      sym_identifier,
  [3638] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(446), 1,
      anon_sym_RBRACE,
    ACTIONS(444), 2,
      anon_sym_anvil,
      sym_identifier,
  [3649] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(363), 1,
      anon_sym_RBRACK,
    ACTIONS(448), 1,
      anon_sym_COMMA,
    STATE(124), 1,
      aux_sym_string_array_repeat1,
  [3662] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(452), 1,
      anon_sym_RBRACE,
    ACTIONS(450), 2,
      anon_sym_anvil,
      sym_identifier,
  [3673] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(454), 1,
      anon_sym_RPAREN,
    ACTIONS(456), 1,
      anon_sym_COMMA,
    STATE(140), 1,
      aux_sym_fixture_params_repeat1,
  [3686] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(458), 1,
      anon_sym_RPAREN,
    ACTIONS(460), 1,
      anon_sym_COMMA,
    STATE(143), 1,
      aux_sym_chart_params_repeat1,
  [3699] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(213), 1,
      anon_sym_DQUOTE,
    ACTIONS(215), 1,
      anon_sym_SQUOTE,
    STATE(166), 1,
      sym_string,
  [3712] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(462), 1,
      anon_sym_RPAREN,
    ACTIONS(464), 1,
      anon_sym_COMMA,
    STATE(137), 1,
      aux_sym_argument_list_repeat1,
  [3725] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(418), 1,
      sym_identifier,
    ACTIONS(466), 1,
      anon_sym_RPAREN,
    STATE(132), 1,
      sym_fixture_param,
  [3738] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(468), 1,
      anon_sym_RPAREN,
    ACTIONS(470), 1,
      anon_sym_COMMA,
    STATE(121), 1,
      aux_sym_argument_list_repeat1,
  [3751] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(332), 1,
      sym_identifier,
    ACTIONS(468), 1,
      anon_sym_RPAREN,
    STATE(144), 1,
      sym_argument,
  [3764] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(474), 1,
      anon_sym_RBRACE,
    ACTIONS(472), 2,
      anon_sym_anvil,
      sym_identifier,
  [3775] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(476), 1,
      anon_sym_RPAREN,
    ACTIONS(478), 1,
      anon_sym_COMMA,
    STATE(118), 1,
      aux_sym_fixture_params_repeat1,
  [3788] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(480), 1,
      anon_sym_RPAREN,
    ACTIONS(482), 1,
      anon_sym_fork,
    STATE(215), 1,
      sym_anvil_args,
  [3801] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(418), 1,
      sym_identifier,
    ACTIONS(476), 1,
      anon_sym_RPAREN,
    STATE(170), 1,
      sym_fixture_param,
  [3814] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(21), 1,
      anon_sym_RPAREN,
    ACTIONS(484), 1,
      anon_sym_COMMA,
    STATE(123), 1,
      aux_sym_chart_params_repeat1,
  [3827] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(413), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [3835] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(342), 1,
      anon_sym_LBRACE,
    STATE(87), 1,
      sym_code_block,
  [3845] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(342), 1,
      anon_sym_LBRACE,
    STATE(43), 1,
      sym_code_block,
  [3855] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(223), 1,
      anon_sym_LBRACK,
    STATE(34), 1,
      sym_string_array,
  [3865] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(342), 1,
      anon_sym_LBRACE,
    STATE(82), 1,
      sym_code_block,
  [3875] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(486), 2,
      anon_sym_RBRACE,
      anon_sym_charting,
  [3883] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(488), 1,
      anon_sym_LBRACE,
    STATE(54), 1,
      sym_benchmark_body,
  [3893] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(490), 1,
      anon_sym_LBRACE,
    STATE(57), 1,
      sym_setup_body,
  [3903] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(492), 2,
      anon_sym_LBRACE,
      anon_sym_COLON,
  [3911] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(375), 1,
      anon_sym_LBRACE,
    STATE(48), 1,
      sym_fixture_body,
  [3921] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(342), 1,
      anon_sym_LBRACE,
    STATE(86), 1,
      sym_code_block,
  [3931] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(494), 1,
      anon_sym_DOT,
    ACTIONS(496), 1,
      anon_sym_LPAREN,
  [3941] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(422), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [3949] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(498), 2,
      anon_sym_RBRACE,
      anon_sym_charting,
  [3957] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(500), 2,
      ts_builtin_sym_end,
      anon_sym_suite,
  [3965] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(342), 1,
      anon_sym_LBRACE,
    STATE(80), 1,
      sym_code_block,
  [3975] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(502), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [3983] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(504), 1,
      anon_sym_RBRACE,
    ACTIONS(506), 1,
      sym_embedded_code,
  [3993] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(332), 1,
      sym_identifier,
    STATE(144), 1,
      sym_argument,
  [4003] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(508), 2,
      ts_builtin_sym_end,
      anon_sym_suite,
  [4011] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(510), 1,
      anon_sym_RPAREN,
    ACTIONS(512), 1,
      sym_embedded_code,
  [4021] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(418), 1,
      sym_identifier,
    STATE(170), 1,
      sym_fixture_param,
  [4031] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(430), 2,
      anon_sym_COMMA,
      anon_sym_RBRACK,
  [4039] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(514), 1,
      anon_sym_LBRACE,
    STATE(163), 1,
      sym_suite_body,
  [4049] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(516), 1,
      anon_sym_LBRACE,
    STATE(53), 1,
      sym_after_body,
  [4059] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(518), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4067] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(402), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4075] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(520), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4083] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(522), 1,
      anon_sym_LBRACE,
    STATE(40), 1,
      sym_global_setup_body,
  [4093] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(524), 2,
      ts_builtin_sym_end,
      anon_sym_suite,
  [4101] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(526), 1,
      anon_sym_LPAREN,
  [4108] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(528), 1,
      anon_sym_COLON,
  [4115] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(530), 1,
      anon_sym_COLON,
  [4122] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(532), 1,
      sym_identifier,
  [4129] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(534), 1,
      sym_identifier,
  [4136] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(536), 1,
      anon_sym_COLON,
  [4143] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(538), 1,
      anon_sym_LBRACE,
  [4150] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(540), 1,
      anon_sym_COLON,
  [4157] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(542), 1,
      sym_identifier,
  [4164] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(544), 1,
      anon_sym_LBRACE,
  [4171] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(546), 1,
      anon_sym_COLON,
  [4178] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(548), 1,
      anon_sym_std,
  [4185] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(550), 1,
      anon_sym_spawnAnvil,
  [4192] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(552), 1,
      anon_sym_COLON,
  [4199] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(554), 1,
      anon_sym_LPAREN,
  [4206] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(556), 1,
      anon_sym_LBRACE,
  [4213] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(558), 1,
      anon_sym_RPAREN,
  [4220] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(560), 1,
      anon_sym_LPAREN,
  [4227] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(562), 1,
      anon_sym_RBRACE,
  [4234] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(564), 1,
      anon_sym_RPAREN,
  [4241] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(566), 1,
      anon_sym_COLON,
  [4248] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(568), 1,
      anon_sym_COLON,
  [4255] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(570), 1,
      anon_sym_RPAREN,
  [4262] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(572), 1,
      anon_sym_COLON,
  [4269] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(574), 1,
      anon_sym_COLON,
  [4276] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(576), 1,
      anon_sym_LPAREN,
  [4283] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(578), 1,
      anon_sym_LPAREN,
  [4290] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(580), 1,
      anon_sym_RPAREN,
  [4297] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(582), 1,
      anon_sym_DOT,
  [4304] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(584), 1,
      sym_identifier,
  [4311] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(586), 1,
      anon_sym_RPAREN,
  [4318] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(588), 1,
      anon_sym_COLON,
  [4325] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(590), 1,
      anon_sym_LBRACE,
  [4332] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(592), 1,
      anon_sym_init,
  [4339] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(594), 1,
      anon_sym_LBRACE,
  [4346] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(596), 1,
      sym_identifier,
  [4353] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(598), 1,
      anon_sym_DOT,
  [4360] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(600), 1,
      anon_sym_COLON,
  [4367] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(602), 1,
      anon_sym_SQUOTE,
  [4374] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(604), 1,
      anon_sym_COLON_COLON,
  [4381] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(602), 1,
      anon_sym_DQUOTE,
  [4388] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(606), 1,
      anon_sym_RPAREN,
  [4395] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(608), 1,
      ts_builtin_sym_end,
  [4402] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(610), 1,
      sym_identifier,
  [4409] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(612), 1,
      anon_sym_RPAREN,
  [4416] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(614), 1,
      anon_sym_COLON,
  [4423] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(616), 1,
      anon_sym_COLON,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 78,
  [SMALL_STATE(4)] = 153,
  [SMALL_STATE(5)] = 228,
  [SMALL_STATE(6)] = 300,
  [SMALL_STATE(7)] = 370,
  [SMALL_STATE(8)] = 440,
  [SMALL_STATE(9)] = 510,
  [SMALL_STATE(10)] = 555,
  [SMALL_STATE(11)] = 600,
  [SMALL_STATE(12)] = 644,
  [SMALL_STATE(13)] = 688,
  [SMALL_STATE(14)] = 732,
  [SMALL_STATE(15)] = 776,
  [SMALL_STATE(16)] = 820,
  [SMALL_STATE(17)] = 864,
  [SMALL_STATE(18)] = 908,
  [SMALL_STATE(19)] = 951,
  [SMALL_STATE(20)] = 994,
  [SMALL_STATE(21)] = 1036,
  [SMALL_STATE(22)] = 1090,
  [SMALL_STATE(23)] = 1144,
  [SMALL_STATE(24)] = 1198,
  [SMALL_STATE(25)] = 1244,
  [SMALL_STATE(26)] = 1299,
  [SMALL_STATE(27)] = 1354,
  [SMALL_STATE(28)] = 1409,
  [SMALL_STATE(29)] = 1447,
  [SMALL_STATE(30)] = 1483,
  [SMALL_STATE(31)] = 1519,
  [SMALL_STATE(32)] = 1561,
  [SMALL_STATE(33)] = 1597,
  [SMALL_STATE(34)] = 1633,
  [SMALL_STATE(35)] = 1669,
  [SMALL_STATE(36)] = 1705,
  [SMALL_STATE(37)] = 1741,
  [SMALL_STATE(38)] = 1777,
  [SMALL_STATE(39)] = 1813,
  [SMALL_STATE(40)] = 1853,
  [SMALL_STATE(41)] = 1885,
  [SMALL_STATE(42)] = 1917,
  [SMALL_STATE(43)] = 1949,
  [SMALL_STATE(44)] = 1981,
  [SMALL_STATE(45)] = 2013,
  [SMALL_STATE(46)] = 2045,
  [SMALL_STATE(47)] = 2075,
  [SMALL_STATE(48)] = 2105,
  [SMALL_STATE(49)] = 2135,
  [SMALL_STATE(50)] = 2165,
  [SMALL_STATE(51)] = 2195,
  [SMALL_STATE(52)] = 2225,
  [SMALL_STATE(53)] = 2255,
  [SMALL_STATE(54)] = 2285,
  [SMALL_STATE(55)] = 2315,
  [SMALL_STATE(56)] = 2345,
  [SMALL_STATE(57)] = 2375,
  [SMALL_STATE(58)] = 2405,
  [SMALL_STATE(59)] = 2435,
  [SMALL_STATE(60)] = 2468,
  [SMALL_STATE(61)] = 2501,
  [SMALL_STATE(62)] = 2534,
  [SMALL_STATE(63)] = 2567,
  [SMALL_STATE(64)] = 2597,
  [SMALL_STATE(65)] = 2627,
  [SMALL_STATE(66)] = 2647,
  [SMALL_STATE(67)] = 2677,
  [SMALL_STATE(68)] = 2706,
  [SMALL_STATE(69)] = 2727,
  [SMALL_STATE(70)] = 2748,
  [SMALL_STATE(71)] = 2769,
  [SMALL_STATE(72)] = 2790,
  [SMALL_STATE(73)] = 2811,
  [SMALL_STATE(74)] = 2832,
  [SMALL_STATE(75)] = 2853,
  [SMALL_STATE(76)] = 2874,
  [SMALL_STATE(77)] = 2901,
  [SMALL_STATE(78)] = 2922,
  [SMALL_STATE(79)] = 2943,
  [SMALL_STATE(80)] = 2964,
  [SMALL_STATE(81)] = 2976,
  [SMALL_STATE(82)] = 2992,
  [SMALL_STATE(83)] = 3004,
  [SMALL_STATE(84)] = 3016,
  [SMALL_STATE(85)] = 3034,
  [SMALL_STATE(86)] = 3046,
  [SMALL_STATE(87)] = 3058,
  [SMALL_STATE(88)] = 3070,
  [SMALL_STATE(89)] = 3082,
  [SMALL_STATE(90)] = 3096,
  [SMALL_STATE(91)] = 3113,
  [SMALL_STATE(92)] = 3130,
  [SMALL_STATE(93)] = 3147,
  [SMALL_STATE(94)] = 3163,
  [SMALL_STATE(95)] = 3177,
  [SMALL_STATE(96)] = 3191,
  [SMALL_STATE(97)] = 3205,
  [SMALL_STATE(98)] = 3219,
  [SMALL_STATE(99)] = 3233,
  [SMALL_STATE(100)] = 3247,
  [SMALL_STATE(101)] = 3261,
  [SMALL_STATE(102)] = 3275,
  [SMALL_STATE(103)] = 3285,
  [SMALL_STATE(104)] = 3301,
  [SMALL_STATE(105)] = 3317,
  [SMALL_STATE(106)] = 3331,
  [SMALL_STATE(107)] = 3347,
  [SMALL_STATE(108)] = 3361,
  [SMALL_STATE(109)] = 3375,
  [SMALL_STATE(110)] = 3391,
  [SMALL_STATE(111)] = 3405,
  [SMALL_STATE(112)] = 3419,
  [SMALL_STATE(113)] = 3433,
  [SMALL_STATE(114)] = 3447,
  [SMALL_STATE(115)] = 3463,
  [SMALL_STATE(116)] = 3477,
  [SMALL_STATE(117)] = 3490,
  [SMALL_STATE(118)] = 3501,
  [SMALL_STATE(119)] = 3514,
  [SMALL_STATE(120)] = 3527,
  [SMALL_STATE(121)] = 3538,
  [SMALL_STATE(122)] = 3551,
  [SMALL_STATE(123)] = 3564,
  [SMALL_STATE(124)] = 3577,
  [SMALL_STATE(125)] = 3590,
  [SMALL_STATE(126)] = 3601,
  [SMALL_STATE(127)] = 3614,
  [SMALL_STATE(128)] = 3627,
  [SMALL_STATE(129)] = 3638,
  [SMALL_STATE(130)] = 3649,
  [SMALL_STATE(131)] = 3662,
  [SMALL_STATE(132)] = 3673,
  [SMALL_STATE(133)] = 3686,
  [SMALL_STATE(134)] = 3699,
  [SMALL_STATE(135)] = 3712,
  [SMALL_STATE(136)] = 3725,
  [SMALL_STATE(137)] = 3738,
  [SMALL_STATE(138)] = 3751,
  [SMALL_STATE(139)] = 3764,
  [SMALL_STATE(140)] = 3775,
  [SMALL_STATE(141)] = 3788,
  [SMALL_STATE(142)] = 3801,
  [SMALL_STATE(143)] = 3814,
  [SMALL_STATE(144)] = 3827,
  [SMALL_STATE(145)] = 3835,
  [SMALL_STATE(146)] = 3845,
  [SMALL_STATE(147)] = 3855,
  [SMALL_STATE(148)] = 3865,
  [SMALL_STATE(149)] = 3875,
  [SMALL_STATE(150)] = 3883,
  [SMALL_STATE(151)] = 3893,
  [SMALL_STATE(152)] = 3903,
  [SMALL_STATE(153)] = 3911,
  [SMALL_STATE(154)] = 3921,
  [SMALL_STATE(155)] = 3931,
  [SMALL_STATE(156)] = 3941,
  [SMALL_STATE(157)] = 3949,
  [SMALL_STATE(158)] = 3957,
  [SMALL_STATE(159)] = 3965,
  [SMALL_STATE(160)] = 3975,
  [SMALL_STATE(161)] = 3983,
  [SMALL_STATE(162)] = 3993,
  [SMALL_STATE(163)] = 4003,
  [SMALL_STATE(164)] = 4011,
  [SMALL_STATE(165)] = 4021,
  [SMALL_STATE(166)] = 4031,
  [SMALL_STATE(167)] = 4039,
  [SMALL_STATE(168)] = 4049,
  [SMALL_STATE(169)] = 4059,
  [SMALL_STATE(170)] = 4067,
  [SMALL_STATE(171)] = 4075,
  [SMALL_STATE(172)] = 4083,
  [SMALL_STATE(173)] = 4093,
  [SMALL_STATE(174)] = 4101,
  [SMALL_STATE(175)] = 4108,
  [SMALL_STATE(176)] = 4115,
  [SMALL_STATE(177)] = 4122,
  [SMALL_STATE(178)] = 4129,
  [SMALL_STATE(179)] = 4136,
  [SMALL_STATE(180)] = 4143,
  [SMALL_STATE(181)] = 4150,
  [SMALL_STATE(182)] = 4157,
  [SMALL_STATE(183)] = 4164,
  [SMALL_STATE(184)] = 4171,
  [SMALL_STATE(185)] = 4178,
  [SMALL_STATE(186)] = 4185,
  [SMALL_STATE(187)] = 4192,
  [SMALL_STATE(188)] = 4199,
  [SMALL_STATE(189)] = 4206,
  [SMALL_STATE(190)] = 4213,
  [SMALL_STATE(191)] = 4220,
  [SMALL_STATE(192)] = 4227,
  [SMALL_STATE(193)] = 4234,
  [SMALL_STATE(194)] = 4241,
  [SMALL_STATE(195)] = 4248,
  [SMALL_STATE(196)] = 4255,
  [SMALL_STATE(197)] = 4262,
  [SMALL_STATE(198)] = 4269,
  [SMALL_STATE(199)] = 4276,
  [SMALL_STATE(200)] = 4283,
  [SMALL_STATE(201)] = 4290,
  [SMALL_STATE(202)] = 4297,
  [SMALL_STATE(203)] = 4304,
  [SMALL_STATE(204)] = 4311,
  [SMALL_STATE(205)] = 4318,
  [SMALL_STATE(206)] = 4325,
  [SMALL_STATE(207)] = 4332,
  [SMALL_STATE(208)] = 4339,
  [SMALL_STATE(209)] = 4346,
  [SMALL_STATE(210)] = 4353,
  [SMALL_STATE(211)] = 4360,
  [SMALL_STATE(212)] = 4367,
  [SMALL_STATE(213)] = 4374,
  [SMALL_STATE(214)] = 4381,
  [SMALL_STATE(215)] = 4388,
  [SMALL_STATE(216)] = 4395,
  [SMALL_STATE(217)] = 4402,
  [SMALL_STATE(218)] = 4409,
  [SMALL_STATE(219)] = 4416,
  [SMALL_STATE(220)] = 4423,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0, 0, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(185),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(172),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(217),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(157),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(195),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(195),
  [19] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 3, 0, 0),
  [21] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 2, 0, 0),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(179),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(175),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(152),
  [41] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0),
  [43] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(179),
  [46] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(74),
  [49] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(71),
  [52] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(70),
  [55] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(69),
  [58] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(75),
  [61] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(175),
  [64] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(152),
  [67] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [69] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 3, 0, 0),
  [71] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 2, 0, 0),
  [73] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_duration_unit, 1, 0, 0),
  [75] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_duration, 2, 0, 0),
  [77] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_boolean, 1, 0, 0),
  [79] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 2, 0, 0),
  [81] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 5, 0, 0),
  [83] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 3, 0, 0),
  [85] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 4, 0, 0),
  [87] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_code_block, 2, 0, 0),
  [89] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_code_block, 3, 0, 0),
  [91] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property, 3, 0, 4),
  [93] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [95] = {.entry = {.count = 1, .reusable = true}}, SHIFT(187),
  [97] = {.entry = {.count = 1, .reusable = true}}, SHIFT(184),
  [99] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [101] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0),
  [103] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(187),
  [106] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(184),
  [109] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(175),
  [112] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(152),
  [115] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__value, 1, 0, 0),
  [117] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [119] = {.entry = {.count = 1, .reusable = false}}, SHIFT(11),
  [121] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(172),
  [124] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0),
  [126] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(89),
  [129] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(178),
  [132] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(177),
  [135] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(168),
  [138] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(175),
  [141] = {.entry = {.count = 1, .reusable = true}}, SHIFT(173),
  [143] = {.entry = {.count = 1, .reusable = true}}, SHIFT(89),
  [145] = {.entry = {.count = 1, .reusable = true}}, SHIFT(178),
  [147] = {.entry = {.count = 1, .reusable = true}}, SHIFT(177),
  [149] = {.entry = {.count = 1, .reusable = true}}, SHIFT(168),
  [151] = {.entry = {.count = 1, .reusable = true}}, SHIFT(158),
  [153] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_language_implementation, 3, 0, 5),
  [155] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_hook, 2, 0, 0),
  [157] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_grouped, 4, 0, 0),
  [159] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_grouped, 3, 0, 0),
  [161] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_before_hook, 2, 0, 0),
  [163] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tags_property, 3, 0, 0),
  [165] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_validate_hook, 2, 0, 0),
  [167] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_skip_hook, 2, 0, 0),
  [169] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_flat, 3, 0, 5),
  [171] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_each_hook, 2, 0, 0),
  [173] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup, 2, 0, 0),
  [175] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_file_ref, 4, 0, 0),
  [177] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_body, 3, 0, 0),
  [179] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_shape_property, 3, 0, 0),
  [181] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hex_property, 3, 0, 0),
  [183] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_body, 2, 0, 0),
  [185] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark_body, 3, 0, 0),
  [187] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_body, 3, 0, 0),
  [189] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture, 4, 0, 1),
  [191] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_body, 2, 0, 0),
  [193] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark_body, 2, 0, 0),
  [195] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_body, 2, 0, 0),
  [197] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_body, 2, 0, 0),
  [199] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_block, 2, 0, 0),
  [201] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark, 3, 0, 1),
  [203] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_body, 3, 0, 0),
  [205] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture, 3, 0, 1),
  [207] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_block, 3, 0, 3),
  [209] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_body, 3, 0, 0),
  [211] = {.entry = {.count = 1, .reusable = false}}, SHIFT(20),
  [213] = {.entry = {.count = 1, .reusable = true}}, SHIFT(92),
  [215] = {.entry = {.count = 1, .reusable = true}}, SHIFT(91),
  [217] = {.entry = {.count = 1, .reusable = false}}, SHIFT(39),
  [219] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [221] = {.entry = {.count = 1, .reusable = false}}, SHIFT(13),
  [223] = {.entry = {.count = 1, .reusable = true}}, SHIFT(114),
  [225] = {.entry = {.count = 1, .reusable = false}}, SHIFT(31),
  [227] = {.entry = {.count = 1, .reusable = false}}, SHIFT(160),
  [229] = {.entry = {.count = 1, .reusable = false}}, SHIFT(84),
  [231] = {.entry = {.count = 1, .reusable = true}}, SHIFT(160),
  [233] = {.entry = {.count = 1, .reusable = false}}, SHIFT(24),
  [235] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [237] = {.entry = {.count = 1, .reusable = true}}, SHIFT(98),
  [239] = {.entry = {.count = 1, .reusable = true}}, SHIFT(159),
  [241] = {.entry = {.count = 1, .reusable = true}}, SHIFT(207),
  [243] = {.entry = {.count = 1, .reusable = true}}, SHIFT(148),
  [245] = {.entry = {.count = 1, .reusable = true}}, SHIFT(145),
  [247] = {.entry = {.count = 1, .reusable = true}}, SHIFT(58),
  [249] = {.entry = {.count = 1, .reusable = true}}, SHIFT(200),
  [251] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0),
  [253] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(98),
  [256] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(159),
  [259] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(207),
  [262] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(148),
  [265] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(145),
  [268] = {.entry = {.count = 1, .reusable = false}}, SHIFT(171),
  [270] = {.entry = {.count = 1, .reusable = true}}, SHIFT(171),
  [272] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [274] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [276] = {.entry = {.count = 1, .reusable = true}}, SHIFT(206),
  [278] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_hook_grouped_repeat1, 2, 0, 0),
  [280] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_hook_grouped_repeat1, 2, 0, 0), SHIFT_REPEAT(152),
  [283] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [285] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, 0, 0),
  [287] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(155),
  [290] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0),
  [292] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(202),
  [295] = {.entry = {.count = 1, .reusable = false}}, SHIFT(155),
  [297] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [299] = {.entry = {.count = 1, .reusable = false}}, SHIFT(202),
  [301] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [303] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declare_section, 2, 0, 0),
  [305] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0),
  [307] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(185),
  [310] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_init_section, 2, 0, 0),
  [312] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_paren_code_block, 2, 0, 0),
  [314] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_paren_code_block, 3, 0, 0),
  [316] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_init_section, 3, 0, 0),
  [318] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_helpers_section, 2, 0, 0),
  [320] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_section, 2, 0, 0),
  [322] = {.entry = {.count = 1, .reusable = true}}, SHIFT(174),
  [324] = {.entry = {.count = 1, .reusable = false}}, SHIFT(10),
  [326] = {.entry = {.count = 1, .reusable = false}}, SHIFT(110),
  [328] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [330] = {.entry = {.count = 1, .reusable = false}}, SHIFT(94),
  [332] = {.entry = {.count = 1, .reusable = true}}, SHIFT(197),
  [334] = {.entry = {.count = 1, .reusable = true}}, SHIFT(120),
  [336] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_content, 1, 0, 0),
  [338] = {.entry = {.count = 1, .reusable = false}}, SHIFT(108),
  [340] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, 0, 0),
  [342] = {.entry = {.count = 1, .reusable = true}}, SHIFT(161),
  [344] = {.entry = {.count = 1, .reusable = true}}, SHIFT(164),
  [346] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat2, 2, 0, 0),
  [348] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat2, 2, 0, 0), SHIFT_REPEAT(217),
  [351] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, 0, 0),
  [353] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_use_statement, 4, 0, 2),
  [355] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [357] = {.entry = {.count = 1, .reusable = true}}, SHIFT(131),
  [359] = {.entry = {.count = 1, .reusable = false}}, SHIFT(161),
  [361] = {.entry = {.count = 1, .reusable = false}}, SHIFT(37),
  [363] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [365] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_single_string_content_repeat1, 2, 0, 0),
  [367] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_single_string_content_repeat1, 2, 0, 0), SHIFT_REPEAT(107),
  [370] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_string_content_repeat1, 2, 0, 0),
  [372] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_string_content_repeat1, 2, 0, 0), SHIFT_REPEAT(108),
  [375] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [377] = {.entry = {.count = 1, .reusable = true}}, SHIFT(136),
  [379] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_single_string_content, 1, 0, 0),
  [381] = {.entry = {.count = 1, .reusable = false}}, SHIFT(107),
  [383] = {.entry = {.count = 1, .reusable = false}}, SHIFT(28),
  [385] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [387] = {.entry = {.count = 1, .reusable = true}}, SHIFT(210),
  [389] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_after_body_repeat1, 2, 0, 0),
  [391] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_after_body_repeat1, 2, 0, 0), SHIFT_REPEAT(210),
  [394] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [396] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [398] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup_statement, 1, 0, 0),
  [400] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_statement, 1, 0, 0),
  [402] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fixture_params_repeat1, 2, 0, 0),
  [404] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_params_repeat1, 2, 0, 0), SHIFT_REPEAT(165),
  [407] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 3, 0, 0),
  [409] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 5, 0, 0),
  [411] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 5, 0, 0),
  [413] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_argument_list_repeat1, 2, 0, 0),
  [415] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_argument_list_repeat1, 2, 0, 0), SHIFT_REPEAT(162),
  [418] = {.entry = {.count = 1, .reusable = true}}, SHIFT(194),
  [420] = {.entry = {.count = 1, .reusable = true}}, SHIFT(189),
  [422] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_chart_params_repeat1, 2, 0, 0),
  [424] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_chart_params_repeat1, 2, 0, 0), SHIFT_REPEAT(5),
  [427] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_string_array_repeat1, 2, 0, 0), SHIFT_REPEAT(134),
  [430] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_string_array_repeat1, 2, 0, 0),
  [432] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_anvil_call, 5, 0, 0),
  [434] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_call, 5, 0, 0),
  [436] = {.entry = {.count = 1, .reusable = true}}, SHIFT(106),
  [438] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [440] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_anvil_call, 6, 0, 0),
  [442] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_call, 6, 0, 0),
  [444] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 6, 0, 0),
  [446] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 6, 0, 0),
  [448] = {.entry = {.count = 1, .reusable = true}}, SHIFT(103),
  [450] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 3, 0, 0),
  [452] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 3, 0, 0),
  [454] = {.entry = {.count = 1, .reusable = true}}, SHIFT(208),
  [456] = {.entry = {.count = 1, .reusable = true}}, SHIFT(142),
  [458] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 1, 0, 0),
  [460] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [462] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 1, 0, 0),
  [464] = {.entry = {.count = 1, .reusable = true}}, SHIFT(138),
  [466] = {.entry = {.count = 1, .reusable = true}}, SHIFT(180),
  [468] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 2, 0, 0),
  [470] = {.entry = {.count = 1, .reusable = true}}, SHIFT(119),
  [472] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 4, 0, 0),
  [474] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 4, 0, 0),
  [476] = {.entry = {.count = 1, .reusable = true}}, SHIFT(183),
  [478] = {.entry = {.count = 1, .reusable = true}}, SHIFT(122),
  [480] = {.entry = {.count = 1, .reusable = true}}, SHIFT(125),
  [482] = {.entry = {.count = 1, .reusable = true}}, SHIFT(211),
  [484] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [486] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_directive, 6, 0, 7),
  [488] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [490] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [492] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_language_tag, 1, 0, 0),
  [494] = {.entry = {.count = 1, .reusable = true}}, SHIFT(182),
  [496] = {.entry = {.count = 1, .reusable = true}}, SHIFT(104),
  [498] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_directive, 5, 0, 7),
  [500] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite_body, 3, 0, 0),
  [502] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument, 3, 0, 4),
  [504] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [506] = {.entry = {.count = 1, .reusable = true}}, SHIFT(192),
  [508] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite, 3, 0, 1),
  [510] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [512] = {.entry = {.count = 1, .reusable = true}}, SHIFT(190),
  [514] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [516] = {.entry = {.count = 1, .reusable = true}}, SHIFT(115),
  [518] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_param, 3, 0, 6),
  [520] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_param, 3, 0, 4),
  [522] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [524] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite_body, 2, 0, 0),
  [526] = {.entry = {.count = 1, .reusable = true}}, SHIFT(116),
  [528] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property_name, 1, 0, 0),
  [530] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [532] = {.entry = {.count = 1, .reusable = true}}, SHIFT(150),
  [534] = {.entry = {.count = 1, .reusable = true}}, SHIFT(109),
  [536] = {.entry = {.count = 1, .reusable = true}}, SHIFT(147),
  [538] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 2, 0, 0),
  [540] = {.entry = {.count = 1, .reusable = true}}, SHIFT(111),
  [542] = {.entry = {.count = 1, .reusable = true}}, SHIFT(191),
  [544] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 4, 0, 0),
  [546] = {.entry = {.count = 1, .reusable = true}}, SHIFT(146),
  [548] = {.entry = {.count = 1, .reusable = true}}, SHIFT(213),
  [550] = {.entry = {.count = 1, .reusable = true}}, SHIFT(188),
  [552] = {.entry = {.count = 1, .reusable = true}}, SHIFT(90),
  [554] = {.entry = {.count = 1, .reusable = true}}, SHIFT(141),
  [556] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 5, 0, 0),
  [558] = {.entry = {.count = 1, .reusable = true}}, SHIFT(85),
  [560] = {.entry = {.count = 1, .reusable = true}}, SHIFT(93),
  [562] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [564] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_args, 3, 0, 0),
  [566] = {.entry = {.count = 1, .reusable = true}}, SHIFT(209),
  [568] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_param_name, 1, 0, 0),
  [570] = {.entry = {.count = 1, .reusable = true}}, SHIFT(149),
  [572] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [574] = {.entry = {.count = 1, .reusable = true}}, SHIFT(67),
  [576] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [578] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_function_name, 1, 0, 0),
  [580] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [582] = {.entry = {.count = 1, .reusable = true}}, SHIFT(186),
  [584] = {.entry = {.count = 1, .reusable = true}}, SHIFT(102),
  [586] = {.entry = {.count = 1, .reusable = true}}, SHIFT(139),
  [588] = {.entry = {.count = 1, .reusable = true}}, SHIFT(105),
  [590] = {.entry = {.count = 1, .reusable = true}}, SHIFT(68),
  [592] = {.entry = {.count = 1, .reusable = true}}, SHIFT(154),
  [594] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 3, 0, 0),
  [596] = {.entry = {.count = 1, .reusable = true}}, SHIFT(169),
  [598] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [600] = {.entry = {.count = 1, .reusable = true}}, SHIFT(127),
  [602] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [604] = {.entry = {.count = 1, .reusable = true}}, SHIFT(203),
  [606] = {.entry = {.count = 1, .reusable = true}}, SHIFT(128),
  [608] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [610] = {.entry = {.count = 1, .reusable = true}}, SHIFT(167),
  [612] = {.entry = {.count = 1, .reusable = true}}, SHIFT(129),
  [614] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [616] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
};

enum ts_external_scanner_symbol_identifiers {
  ts_external_token_embedded_code = 0,
  ts_external_token__embedded_code_start = 1,
};

static const TSSymbol ts_external_scanner_symbol_map[EXTERNAL_TOKEN_COUNT] = {
  [ts_external_token_embedded_code] = sym_embedded_code,
  [ts_external_token__embedded_code_start] = sym__embedded_code_start,
};

static const bool ts_external_scanner_states[3][EXTERNAL_TOKEN_COUNT] = {
  [1] = {
    [ts_external_token_embedded_code] = true,
    [ts_external_token__embedded_code_start] = true,
  },
  [2] = {
    [ts_external_token_embedded_code] = true,
  },
};

#ifdef __cplusplus
extern "C" {
#endif
void *tree_sitter_polybench_external_scanner_create(void);
void tree_sitter_polybench_external_scanner_destroy(void *);
bool tree_sitter_polybench_external_scanner_scan(void *, TSLexer *, const bool *);
unsigned tree_sitter_polybench_external_scanner_serialize(void *, char *);
void tree_sitter_polybench_external_scanner_deserialize(void *, const char *, unsigned);

#ifdef TREE_SITTER_HIDE_SYMBOLS
#define TS_PUBLIC
#elif defined(_WIN32)
#define TS_PUBLIC __declspec(dllexport)
#else
#define TS_PUBLIC __attribute__((visibility("default")))
#endif

TS_PUBLIC const TSLanguage *tree_sitter_polybench(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .field_names = ts_field_names,
    .field_map_slices = ts_field_map_slices,
    .field_map_entries = ts_field_map_entries,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .keyword_lex_fn = ts_lex_keywords,
    .keyword_capture_token = sym_identifier,
    .external_scanner = {
      &ts_external_scanner_states[0][0],
      ts_external_scanner_symbol_map,
      tree_sitter_polybench_external_scanner_create,
      tree_sitter_polybench_external_scanner_destroy,
      tree_sitter_polybench_external_scanner_scan,
      tree_sitter_polybench_external_scanner_serialize,
      tree_sitter_polybench_external_scanner_deserialize,
    },
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
