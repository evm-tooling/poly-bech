#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 251
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 187
#define ALIAS_COUNT 0
#define TOKEN_COUNT 104
#define EXTERNAL_TOKEN_COUNT 2
#define FIELD_COUNT 9
#define MAX_ALIAS_SEQUENCE_LENGTH 9
#define PRODUCTION_ID_COUNT 11

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
  anon_sym_declare = 16,
  anon_sym_suite = 17,
  anon_sym_sameDataset = 18,
  anon_sym_performance = 19,
  anon_sym_memory = 20,
  anon_sym_timeBased = 21,
  anon_sym_iterationBased = 22,
  anon_sym_setup = 23,
  anon_sym_import = 24,
  anon_sym_async = 25,
  anon_sym_init = 26,
  anon_sym_helpers = 27,
  anon_sym_fixture = 28,
  anon_sym_hex = 29,
  anon_sym_data = 30,
  anon_sym_encoding = 31,
  anon_sym_format = 32,
  anon_sym_selector = 33,
  anon_sym_shape = 34,
  anon_sym_ATfile = 35,
  anon_sym_bench = 36,
  anon_sym_benchAsync = 37,
  anon_sym_tags = 38,
  anon_sym_skip = 39,
  anon_sym_validate = 40,
  anon_sym_before = 41,
  anon_sym_after = 42,
  anon_sym_each = 43,
  anon_sym_charting = 44,
  anon_sym_drawSpeedupChart = 45,
  anon_sym_drawTable = 46,
  anon_sym_drawLineChart = 47,
  anon_sym_drawBarChart = 48,
  anon_sym_title = 49,
  anon_sym_description = 50,
  anon_sym_output = 51,
  anon_sym_sortBy = 52,
  anon_sym_sortOrder = 53,
  anon_sym_baselineBenchmark = 54,
  anon_sym_baseline = 55,
  anon_sym_filterWinner = 56,
  anon_sym_theme = 57,
  anon_sym_width = 58,
  anon_sym_rowCount = 59,
  anon_sym_height = 60,
  anon_sym_limit = 61,
  anon_sym_minSpeedup = 62,
  anon_sym_includeBenchmarks = 63,
  anon_sym_excludeBenchmarks = 64,
  anon_sym_showStdDev = 65,
  anon_sym_showErrorBars = 66,
  anon_sym_showRegression = 67,
  anon_sym_regressionModel = 68,
  anon_sym_yScale = 69,
  anon_sym_iterations = 70,
  anon_sym_warmup = 71,
  anon_sym_timeout = 72,
  anon_sym_requires = 73,
  anon_sym_order = 74,
  anon_sym_mode = 75,
  anon_sym_targetTime = 76,
  anon_sym_sink = 77,
  anon_sym_outlierDetection = 78,
  anon_sym_cvThreshold = 79,
  anon_sym_count = 80,
  anon_sym_fairness = 81,
  anon_sym_fairnessSeed = 82,
  anon_sym_asyncSamplingPolicy = 83,
  anon_sym_asyncWarmupCap = 84,
  anon_sym_asyncSampleCap = 85,
  sym_inline_code = 86,
  anon_sym_DQUOTE = 87,
  anon_sym_SQUOTE = 88,
  aux_sym_string_content_token1 = 89,
  aux_sym_single_string_content_token1 = 90,
  sym_escape_sequence = 91,
  sym_number = 92,
  sym_float = 93,
  anon_sym_ms = 94,
  anon_sym_s = 95,
  anon_sym_m = 96,
  anon_sym_true = 97,
  anon_sym_false = 98,
  anon_sym_LBRACK = 99,
  anon_sym_RBRACK = 100,
  sym_comment = 101,
  sym_embedded_code = 102,
  sym__embedded_code_start = 103,
  sym_source_file = 104,
  sym_use_statement = 105,
  sym_global_setup = 106,
  sym_global_setup_body = 107,
  sym_global_setup_statement = 108,
  sym_anvil_call = 109,
  sym_anvil_args = 110,
  sym_function_call = 111,
  sym_argument_list = 112,
  sym_argument = 113,
  sym_suite = 114,
  sym_suite_type = 115,
  sym_run_mode = 116,
  sym_suite_body = 117,
  sym__suite_item = 118,
  sym_setup_block = 119,
  sym_setup_body = 120,
  sym__setup_section = 121,
  sym_import_section = 122,
  sym_declare_section = 123,
  sym_init_section = 124,
  sym_helpers_section = 125,
  sym_fixture = 126,
  sym_fixture_params = 127,
  sym_fixture_param = 128,
  sym_fixture_body = 129,
  sym__fixture_item = 130,
  sym_hex_property = 131,
  sym_data_property = 132,
  sym_encoding_property = 133,
  sym_format_property = 134,
  sym_selector_property = 135,
  sym_shape_property = 136,
  sym_file_ref = 137,
  sym_benchmark = 138,
  sym_benchmark_body = 139,
  sym__benchmark_item = 140,
  sym_tags_property = 141,
  sym_skip_hook = 142,
  sym_validate_hook = 143,
  sym_before_hook = 144,
  sym_after_hook = 145,
  sym_each_hook = 146,
  sym_hook_flat = 147,
  sym_hook_grouped = 148,
  sym_after_block = 149,
  sym_after_body = 150,
  sym_chart_directive = 151,
  sym_chart_function_name = 152,
  sym_chart_params = 153,
  sym_chart_param = 154,
  sym_chart_param_name = 155,
  sym__chart_value = 156,
  sym_property = 157,
  sym_property_name = 158,
  sym__value = 159,
  sym_language_implementation = 160,
  sym_language_tag = 161,
  sym__code_or_inline = 162,
  sym_code_block = 163,
  sym_paren_code_block = 164,
  sym_string = 165,
  sym_string_content = 166,
  sym_single_string_content = 167,
  sym_duration = 168,
  sym_duration_unit = 169,
  sym_boolean = 170,
  sym_string_array = 171,
  aux_sym_source_file_repeat1 = 172,
  aux_sym_source_file_repeat2 = 173,
  aux_sym_global_setup_body_repeat1 = 174,
  aux_sym_argument_list_repeat1 = 175,
  aux_sym_suite_body_repeat1 = 176,
  aux_sym_setup_body_repeat1 = 177,
  aux_sym_fixture_params_repeat1 = 178,
  aux_sym_fixture_body_repeat1 = 179,
  aux_sym_benchmark_body_repeat1 = 180,
  aux_sym_hook_grouped_repeat1 = 181,
  aux_sym_after_body_repeat1 = 182,
  aux_sym_chart_params_repeat1 = 183,
  aux_sym_string_content_repeat1 = 184,
  aux_sym_single_string_content_repeat1 = 185,
  aux_sym_string_array_repeat1 = 186,
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
  [anon_sym_declare] = "declare",
  [anon_sym_suite] = "suite",
  [anon_sym_sameDataset] = "sameDataset",
  [anon_sym_performance] = "performance",
  [anon_sym_memory] = "memory",
  [anon_sym_timeBased] = "timeBased",
  [anon_sym_iterationBased] = "iterationBased",
  [anon_sym_setup] = "setup",
  [anon_sym_import] = "import",
  [anon_sym_async] = "async",
  [anon_sym_init] = "init",
  [anon_sym_helpers] = "helpers",
  [anon_sym_fixture] = "fixture",
  [anon_sym_hex] = "hex",
  [anon_sym_data] = "data",
  [anon_sym_encoding] = "encoding",
  [anon_sym_format] = "format",
  [anon_sym_selector] = "selector",
  [anon_sym_shape] = "shape",
  [anon_sym_ATfile] = "@file",
  [anon_sym_bench] = "bench",
  [anon_sym_benchAsync] = "benchAsync",
  [anon_sym_tags] = "tags",
  [anon_sym_skip] = "skip",
  [anon_sym_validate] = "validate",
  [anon_sym_before] = "before",
  [anon_sym_after] = "after",
  [anon_sym_each] = "each",
  [anon_sym_charting] = "charting",
  [anon_sym_drawSpeedupChart] = "drawSpeedupChart",
  [anon_sym_drawTable] = "drawTable",
  [anon_sym_drawLineChart] = "drawLineChart",
  [anon_sym_drawBarChart] = "drawBarChart",
  [anon_sym_title] = "title",
  [anon_sym_description] = "description",
  [anon_sym_output] = "output",
  [anon_sym_sortBy] = "sortBy",
  [anon_sym_sortOrder] = "sortOrder",
  [anon_sym_baselineBenchmark] = "baselineBenchmark",
  [anon_sym_baseline] = "baseline",
  [anon_sym_filterWinner] = "filterWinner",
  [anon_sym_theme] = "theme",
  [anon_sym_width] = "width",
  [anon_sym_rowCount] = "rowCount",
  [anon_sym_height] = "height",
  [anon_sym_limit] = "limit",
  [anon_sym_minSpeedup] = "minSpeedup",
  [anon_sym_includeBenchmarks] = "includeBenchmarks",
  [anon_sym_excludeBenchmarks] = "excludeBenchmarks",
  [anon_sym_showStdDev] = "showStdDev",
  [anon_sym_showErrorBars] = "showErrorBars",
  [anon_sym_showRegression] = "showRegression",
  [anon_sym_regressionModel] = "regressionModel",
  [anon_sym_yScale] = "yScale",
  [anon_sym_iterations] = "iterations",
  [anon_sym_warmup] = "warmup",
  [anon_sym_timeout] = "timeout",
  [anon_sym_requires] = "requires",
  [anon_sym_order] = "order",
  [anon_sym_mode] = "mode",
  [anon_sym_targetTime] = "targetTime",
  [anon_sym_sink] = "sink",
  [anon_sym_outlierDetection] = "outlierDetection",
  [anon_sym_cvThreshold] = "cvThreshold",
  [anon_sym_count] = "count",
  [anon_sym_fairness] = "fairness",
  [anon_sym_fairnessSeed] = "fairnessSeed",
  [anon_sym_asyncSamplingPolicy] = "asyncSamplingPolicy",
  [anon_sym_asyncWarmupCap] = "asyncWarmupCap",
  [anon_sym_asyncSampleCap] = "asyncSampleCap",
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
  [sym_suite_type] = "suite_type",
  [sym_run_mode] = "run_mode",
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
  [sym_data_property] = "data_property",
  [sym_encoding_property] = "encoding_property",
  [sym_format_property] = "format_property",
  [sym_selector_property] = "selector_property",
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
  [anon_sym_declare] = anon_sym_declare,
  [anon_sym_suite] = anon_sym_suite,
  [anon_sym_sameDataset] = anon_sym_sameDataset,
  [anon_sym_performance] = anon_sym_performance,
  [anon_sym_memory] = anon_sym_memory,
  [anon_sym_timeBased] = anon_sym_timeBased,
  [anon_sym_iterationBased] = anon_sym_iterationBased,
  [anon_sym_setup] = anon_sym_setup,
  [anon_sym_import] = anon_sym_import,
  [anon_sym_async] = anon_sym_async,
  [anon_sym_init] = anon_sym_init,
  [anon_sym_helpers] = anon_sym_helpers,
  [anon_sym_fixture] = anon_sym_fixture,
  [anon_sym_hex] = anon_sym_hex,
  [anon_sym_data] = anon_sym_data,
  [anon_sym_encoding] = anon_sym_encoding,
  [anon_sym_format] = anon_sym_format,
  [anon_sym_selector] = anon_sym_selector,
  [anon_sym_shape] = anon_sym_shape,
  [anon_sym_ATfile] = anon_sym_ATfile,
  [anon_sym_bench] = anon_sym_bench,
  [anon_sym_benchAsync] = anon_sym_benchAsync,
  [anon_sym_tags] = anon_sym_tags,
  [anon_sym_skip] = anon_sym_skip,
  [anon_sym_validate] = anon_sym_validate,
  [anon_sym_before] = anon_sym_before,
  [anon_sym_after] = anon_sym_after,
  [anon_sym_each] = anon_sym_each,
  [anon_sym_charting] = anon_sym_charting,
  [anon_sym_drawSpeedupChart] = anon_sym_drawSpeedupChart,
  [anon_sym_drawTable] = anon_sym_drawTable,
  [anon_sym_drawLineChart] = anon_sym_drawLineChart,
  [anon_sym_drawBarChart] = anon_sym_drawBarChart,
  [anon_sym_title] = anon_sym_title,
  [anon_sym_description] = anon_sym_description,
  [anon_sym_output] = anon_sym_output,
  [anon_sym_sortBy] = anon_sym_sortBy,
  [anon_sym_sortOrder] = anon_sym_sortOrder,
  [anon_sym_baselineBenchmark] = anon_sym_baselineBenchmark,
  [anon_sym_baseline] = anon_sym_baseline,
  [anon_sym_filterWinner] = anon_sym_filterWinner,
  [anon_sym_theme] = anon_sym_theme,
  [anon_sym_width] = anon_sym_width,
  [anon_sym_rowCount] = anon_sym_rowCount,
  [anon_sym_height] = anon_sym_height,
  [anon_sym_limit] = anon_sym_limit,
  [anon_sym_minSpeedup] = anon_sym_minSpeedup,
  [anon_sym_includeBenchmarks] = anon_sym_includeBenchmarks,
  [anon_sym_excludeBenchmarks] = anon_sym_excludeBenchmarks,
  [anon_sym_showStdDev] = anon_sym_showStdDev,
  [anon_sym_showErrorBars] = anon_sym_showErrorBars,
  [anon_sym_showRegression] = anon_sym_showRegression,
  [anon_sym_regressionModel] = anon_sym_regressionModel,
  [anon_sym_yScale] = anon_sym_yScale,
  [anon_sym_iterations] = anon_sym_iterations,
  [anon_sym_warmup] = anon_sym_warmup,
  [anon_sym_timeout] = anon_sym_timeout,
  [anon_sym_requires] = anon_sym_requires,
  [anon_sym_order] = anon_sym_order,
  [anon_sym_mode] = anon_sym_mode,
  [anon_sym_targetTime] = anon_sym_targetTime,
  [anon_sym_sink] = anon_sym_sink,
  [anon_sym_outlierDetection] = anon_sym_outlierDetection,
  [anon_sym_cvThreshold] = anon_sym_cvThreshold,
  [anon_sym_count] = anon_sym_count,
  [anon_sym_fairness] = anon_sym_fairness,
  [anon_sym_fairnessSeed] = anon_sym_fairnessSeed,
  [anon_sym_asyncSamplingPolicy] = anon_sym_asyncSamplingPolicy,
  [anon_sym_asyncWarmupCap] = anon_sym_asyncWarmupCap,
  [anon_sym_asyncSampleCap] = anon_sym_asyncSampleCap,
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
  [sym_suite_type] = sym_suite_type,
  [sym_run_mode] = sym_run_mode,
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
  [sym_data_property] = sym_data_property,
  [sym_encoding_property] = sym_encoding_property,
  [sym_format_property] = sym_format_property,
  [sym_selector_property] = sym_selector_property,
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
  [anon_sym_declare] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_suite] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_sameDataset] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_performance] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_memory] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_timeBased] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_iterationBased] = {
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
  [anon_sym_data] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_encoding] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_format] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_selector] = {
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
  [anon_sym_benchAsync] = {
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
  [anon_sym_drawSpeedupChart] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_drawTable] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_drawLineChart] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_drawBarChart] = {
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
  [anon_sym_baselineBenchmark] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_baseline] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_filterWinner] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_theme] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_width] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_rowCount] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_height] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_limit] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_minSpeedup] = {
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
  [anon_sym_showStdDev] = {
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
  [anon_sym_regressionModel] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_yScale] = {
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
  [anon_sym_mode] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_targetTime] = {
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
  [anon_sym_fairness] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_fairnessSeed] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_asyncSamplingPolicy] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_asyncWarmupCap] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_asyncSampleCap] = {
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
  [sym_suite_type] = {
    .visible = true,
    .named = true,
  },
  [sym_run_mode] = {
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
  [sym_data_property] = {
    .visible = true,
    .named = true,
  },
  [sym_encoding_property] = {
    .visible = true,
    .named = true,
  },
  [sym_format_property] = {
    .visible = true,
    .named = true,
  },
  [sym_selector_property] = {
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
  field_run_mode = 5,
  field_same_dataset = 6,
  field_suite_type = 7,
  field_type = 8,
  field_value = 9,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_function] = "function",
  [field_language] = "language",
  [field_module] = "module",
  [field_name] = "name",
  [field_run_mode] = "run_mode",
  [field_same_dataset] = "same_dataset",
  [field_suite_type] = "suite_type",
  [field_type] = "type",
  [field_value] = "value",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 1},
  [3] = {.index = 2, .length = 1},
  [4] = {.index = 3, .length = 1},
  [5] = {.index = 4, .length = 2},
  [6] = {.index = 6, .length = 4},
  [7] = {.index = 10, .length = 4},
  [8] = {.index = 14, .length = 1},
  [9] = {.index = 15, .length = 2},
  [10] = {.index = 17, .length = 1},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_name, 1},
  [1] =
    {field_module, 3},
  [2] =
    {field_name, 2},
  [3] =
    {field_language, 1},
  [4] =
    {field_name, 0},
    {field_value, 2},
  [6] =
    {field_name, 1},
    {field_run_mode, 3},
    {field_same_dataset, 6},
    {field_suite_type, 2},
  [10] =
    {field_name, 2},
    {field_run_mode, 4},
    {field_same_dataset, 7},
    {field_suite_type, 3},
  [14] =
    {field_language, 0},
  [15] =
    {field_name, 0},
    {field_type, 2},
  [17] =
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
  [25] = 24,
  [26] = 24,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 32,
  [33] = 33,
  [34] = 34,
  [35] = 35,
  [36] = 36,
  [37] = 37,
  [38] = 38,
  [39] = 39,
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
  [60] = 60,
  [61] = 61,
  [62] = 62,
  [63] = 63,
  [64] = 64,
  [65] = 63,
  [66] = 63,
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
  [81] = 24,
  [82] = 82,
  [83] = 83,
  [84] = 84,
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
  [219] = 219,
  [220] = 220,
  [221] = 221,
  [222] = 222,
  [223] = 223,
  [224] = 224,
  [225] = 225,
  [226] = 226,
  [227] = 227,
  [228] = 228,
  [229] = 229,
  [230] = 230,
  [231] = 231,
  [232] = 232,
  [233] = 233,
  [234] = 234,
  [235] = 235,
  [236] = 236,
  [237] = 237,
  [238] = 238,
  [239] = 239,
  [240] = 240,
  [241] = 241,
  [242] = 242,
  [243] = 243,
  [244] = 244,
  [245] = 245,
  [246] = 246,
  [247] = 247,
  [248] = 248,
  [249] = 213,
  [250] = 213,
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
        'y', 20,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0);
      END_STATE();
    case 1:
      if (lookahead == 'f') ADVANCE(21);
      if (lookahead == 'n') ADVANCE(22);
      if (lookahead == 's') ADVANCE(23);
      END_STATE();
    case 2:
      if (lookahead == 'a') ADVANCE(24);
      if (lookahead == 'e') ADVANCE(25);
      END_STATE();
    case 3:
      if (lookahead == 'h') ADVANCE(26);
      if (lookahead == 'o') ADVANCE(27);
      if (lookahead == 'v') ADVANCE(28);
      END_STATE();
    case 4:
      if (lookahead == 'a') ADVANCE(29);
      if (lookahead == 'e') ADVANCE(30);
      if (lookahead == 'r') ADVANCE(31);
      END_STATE();
    case 5:
      if (lookahead == 'a') ADVANCE(32);
      if (lookahead == 'n') ADVANCE(33);
      if (lookahead == 'x') ADVANCE(34);
      END_STATE();
    case 6:
      if (lookahead == 'a') ADVANCE(35);
      if (lookahead == 'i') ADVANCE(36);
      if (lookahead == 'o') ADVANCE(37);
      END_STATE();
    case 7:
      if (lookahead == 'l') ADVANCE(38);
      END_STATE();
    case 8:
      if (lookahead == 'e') ADVANCE(39);
      END_STATE();
    case 9:
      if (lookahead == 'm') ADVANCE(40);
      if (lookahead == 'n') ADVANCE(41);
      if (lookahead == 't') ADVANCE(42);
      END_STATE();
    case 10:
      if (lookahead == 'i') ADVANCE(43);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_m);
      if (lookahead == 'e') ADVANCE(44);
      if (lookahead == 'i') ADVANCE(45);
      if (lookahead == 'o') ADVANCE(46);
      if (lookahead == 's') ADVANCE(47);
      END_STATE();
    case 12:
      if (lookahead == 'r') ADVANCE(48);
      if (lookahead == 'u') ADVANCE(49);
      END_STATE();
    case 13:
      if (lookahead == 'e') ADVANCE(50);
      END_STATE();
    case 14:
      if (lookahead == 'e') ADVANCE(51);
      if (lookahead == 'o') ADVANCE(52);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_s);
      ADVANCE_MAP(
        'a', 53,
        'e', 54,
        'h', 55,
        'i', 56,
        'k', 57,
        'o', 58,
        'p', 59,
        't', 60,
        'u', 61,
      );
      END_STATE();
    case 16:
      if (lookahead == 'a') ADVANCE(62);
      if (lookahead == 'h') ADVANCE(63);
      if (lookahead == 'i') ADVANCE(64);
      if (lookahead == 'r') ADVANCE(65);
      END_STATE();
    case 17:
      if (lookahead == 's') ADVANCE(66);
      END_STATE();
    case 18:
      if (lookahead == 'a') ADVANCE(67);
      END_STATE();
    case 19:
      if (lookahead == 'a') ADVANCE(68);
      if (lookahead == 'i') ADVANCE(69);
      END_STATE();
    case 20:
      if (lookahead == 'S') ADVANCE(70);
      END_STATE();
    case 21:
      if (lookahead == 't') ADVANCE(71);
      END_STATE();
    case 22:
      if (lookahead == 'v') ADVANCE(72);
      END_STATE();
    case 23:
      if (lookahead == 'y') ADVANCE(73);
      END_STATE();
    case 24:
      if (lookahead == 's') ADVANCE(74);
      END_STATE();
    case 25:
      if (lookahead == 'f') ADVANCE(75);
      if (lookahead == 'n') ADVANCE(76);
      END_STATE();
    case 26:
      if (lookahead == 'a') ADVANCE(77);
      END_STATE();
    case 27:
      if (lookahead == 'u') ADVANCE(78);
      END_STATE();
    case 28:
      if (lookahead == 'T') ADVANCE(79);
      END_STATE();
    case 29:
      if (lookahead == 't') ADVANCE(80);
      END_STATE();
    case 30:
      if (lookahead == 'c') ADVANCE(81);
      if (lookahead == 's') ADVANCE(82);
      END_STATE();
    case 31:
      if (lookahead == 'a') ADVANCE(83);
      END_STATE();
    case 32:
      if (lookahead == 'c') ADVANCE(84);
      END_STATE();
    case 33:
      if (lookahead == 'c') ADVANCE(85);
      END_STATE();
    case 34:
      if (lookahead == 'c') ADVANCE(86);
      END_STATE();
    case 35:
      if (lookahead == 'i') ADVANCE(87);
      if (lookahead == 'l') ADVANCE(88);
      END_STATE();
    case 36:
      if (lookahead == 'l') ADVANCE(89);
      if (lookahead == 'x') ADVANCE(90);
      END_STATE();
    case 37:
      if (lookahead == 'r') ADVANCE(91);
      END_STATE();
    case 38:
      if (lookahead == 'o') ADVANCE(92);
      END_STATE();
    case 39:
      if (lookahead == 'i') ADVANCE(93);
      if (lookahead == 'l') ADVANCE(94);
      if (lookahead == 'x') ADVANCE(95);
      END_STATE();
    case 40:
      if (lookahead == 'p') ADVANCE(96);
      END_STATE();
    case 41:
      if (lookahead == 'c') ADVANCE(97);
      if (lookahead == 'i') ADVANCE(98);
      END_STATE();
    case 42:
      if (lookahead == 'e') ADVANCE(99);
      END_STATE();
    case 43:
      if (lookahead == 'm') ADVANCE(100);
      END_STATE();
    case 44:
      if (lookahead == 'm') ADVANCE(101);
      END_STATE();
    case 45:
      if (lookahead == 'n') ADVANCE(102);
      END_STATE();
    case 46:
      if (lookahead == 'd') ADVANCE(103);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(anon_sym_ms);
      END_STATE();
    case 48:
      if (lookahead == 'd') ADVANCE(104);
      END_STATE();
    case 49:
      if (lookahead == 't') ADVANCE(105);
      END_STATE();
    case 50:
      if (lookahead == 'r') ADVANCE(106);
      END_STATE();
    case 51:
      if (lookahead == 'g') ADVANCE(107);
      if (lookahead == 'q') ADVANCE(108);
      END_STATE();
    case 52:
      if (lookahead == 'w') ADVANCE(109);
      END_STATE();
    case 53:
      if (lookahead == 'm') ADVANCE(110);
      END_STATE();
    case 54:
      if (lookahead == 'l') ADVANCE(111);
      if (lookahead == 't') ADVANCE(112);
      END_STATE();
    case 55:
      if (lookahead == 'a') ADVANCE(113);
      if (lookahead == 'o') ADVANCE(114);
      END_STATE();
    case 56:
      if (lookahead == 'n') ADVANCE(115);
      END_STATE();
    case 57:
      if (lookahead == 'i') ADVANCE(116);
      END_STATE();
    case 58:
      if (lookahead == 'r') ADVANCE(117);
      END_STATE();
    case 59:
      if (lookahead == 'a') ADVANCE(118);
      END_STATE();
    case 60:
      if (lookahead == 'd') ADVANCE(119);
      END_STATE();
    case 61:
      if (lookahead == 'i') ADVANCE(120);
      END_STATE();
    case 62:
      if (lookahead == 'g') ADVANCE(121);
      if (lookahead == 'r') ADVANCE(122);
      END_STATE();
    case 63:
      if (lookahead == 'e') ADVANCE(123);
      END_STATE();
    case 64:
      if (lookahead == 'm') ADVANCE(124);
      if (lookahead == 't') ADVANCE(125);
      END_STATE();
    case 65:
      if (lookahead == 'u') ADVANCE(126);
      END_STATE();
    case 66:
      if (lookahead == 'e') ADVANCE(127);
      END_STATE();
    case 67:
      if (lookahead == 'l') ADVANCE(128);
      END_STATE();
    case 68:
      if (lookahead == 'r') ADVANCE(129);
      END_STATE();
    case 69:
      if (lookahead == 'd') ADVANCE(130);
      END_STATE();
    case 70:
      if (lookahead == 'c') ADVANCE(131);
      END_STATE();
    case 71:
      if (lookahead == 'e') ADVANCE(132);
      END_STATE();
    case 72:
      if (lookahead == 'i') ADVANCE(133);
      END_STATE();
    case 73:
      if (lookahead == 'n') ADVANCE(134);
      END_STATE();
    case 74:
      if (lookahead == 'e') ADVANCE(135);
      END_STATE();
    case 75:
      if (lookahead == 'o') ADVANCE(136);
      END_STATE();
    case 76:
      if (lookahead == 'c') ADVANCE(137);
      END_STATE();
    case 77:
      if (lookahead == 'r') ADVANCE(138);
      END_STATE();
    case 78:
      if (lookahead == 'n') ADVANCE(139);
      END_STATE();
    case 79:
      if (lookahead == 'h') ADVANCE(140);
      END_STATE();
    case 80:
      if (lookahead == 'a') ADVANCE(141);
      END_STATE();
    case 81:
      if (lookahead == 'l') ADVANCE(142);
      END_STATE();
    case 82:
      if (lookahead == 'c') ADVANCE(143);
      END_STATE();
    case 83:
      if (lookahead == 'w') ADVANCE(144);
      END_STATE();
    case 84:
      if (lookahead == 'h') ADVANCE(145);
      END_STATE();
    case 85:
      if (lookahead == 'o') ADVANCE(146);
      END_STATE();
    case 86:
      if (lookahead == 'l') ADVANCE(147);
      END_STATE();
    case 87:
      if (lookahead == 'r') ADVANCE(148);
      END_STATE();
    case 88:
      if (lookahead == 's') ADVANCE(149);
      END_STATE();
    case 89:
      if (lookahead == 't') ADVANCE(150);
      END_STATE();
    case 90:
      if (lookahead == 't') ADVANCE(151);
      END_STATE();
    case 91:
      if (lookahead == 'k') ADVANCE(152);
      if (lookahead == 'm') ADVANCE(153);
      END_STATE();
    case 92:
      if (lookahead == 'b') ADVANCE(154);
      END_STATE();
    case 93:
      if (lookahead == 'g') ADVANCE(155);
      END_STATE();
    case 94:
      if (lookahead == 'p') ADVANCE(156);
      END_STATE();
    case 95:
      ACCEPT_TOKEN(anon_sym_hex);
      END_STATE();
    case 96:
      if (lookahead == 'o') ADVANCE(157);
      END_STATE();
    case 97:
      if (lookahead == 'l') ADVANCE(158);
      END_STATE();
    case 98:
      if (lookahead == 't') ADVANCE(159);
      END_STATE();
    case 99:
      if (lookahead == 'r') ADVANCE(160);
      END_STATE();
    case 100:
      if (lookahead == 'i') ADVANCE(161);
      END_STATE();
    case 101:
      if (lookahead == 'o') ADVANCE(162);
      END_STATE();
    case 102:
      if (lookahead == 'S') ADVANCE(163);
      END_STATE();
    case 103:
      if (lookahead == 'e') ADVANCE(164);
      END_STATE();
    case 104:
      if (lookahead == 'e') ADVANCE(165);
      END_STATE();
    case 105:
      if (lookahead == 'l') ADVANCE(166);
      if (lookahead == 'p') ADVANCE(167);
      END_STATE();
    case 106:
      if (lookahead == 'f') ADVANCE(168);
      END_STATE();
    case 107:
      if (lookahead == 'r') ADVANCE(169);
      END_STATE();
    case 108:
      if (lookahead == 'u') ADVANCE(170);
      END_STATE();
    case 109:
      if (lookahead == 'C') ADVANCE(171);
      END_STATE();
    case 110:
      if (lookahead == 'e') ADVANCE(172);
      END_STATE();
    case 111:
      if (lookahead == 'e') ADVANCE(173);
      END_STATE();
    case 112:
      if (lookahead == 'u') ADVANCE(174);
      END_STATE();
    case 113:
      if (lookahead == 'p') ADVANCE(175);
      END_STATE();
    case 114:
      if (lookahead == 'w') ADVANCE(176);
      END_STATE();
    case 115:
      if (lookahead == 'k') ADVANCE(177);
      END_STATE();
    case 116:
      if (lookahead == 'p') ADVANCE(178);
      END_STATE();
    case 117:
      if (lookahead == 't') ADVANCE(179);
      END_STATE();
    case 118:
      if (lookahead == 'w') ADVANCE(180);
      END_STATE();
    case 119:
      ACCEPT_TOKEN(anon_sym_std);
      END_STATE();
    case 120:
      if (lookahead == 't') ADVANCE(181);
      END_STATE();
    case 121:
      if (lookahead == 's') ADVANCE(182);
      END_STATE();
    case 122:
      if (lookahead == 'g') ADVANCE(183);
      END_STATE();
    case 123:
      if (lookahead == 'm') ADVANCE(184);
      END_STATE();
    case 124:
      if (lookahead == 'e') ADVANCE(185);
      END_STATE();
    case 125:
      if (lookahead == 'l') ADVANCE(186);
      END_STATE();
    case 126:
      if (lookahead == 'e') ADVANCE(187);
      END_STATE();
    case 127:
      ACCEPT_TOKEN(anon_sym_use);
      END_STATE();
    case 128:
      if (lookahead == 'i') ADVANCE(188);
      END_STATE();
    case 129:
      if (lookahead == 'm') ADVANCE(189);
      END_STATE();
    case 130:
      if (lookahead == 't') ADVANCE(190);
      END_STATE();
    case 131:
      if (lookahead == 'a') ADVANCE(191);
      END_STATE();
    case 132:
      if (lookahead == 'r') ADVANCE(192);
      END_STATE();
    case 133:
      if (lookahead == 'l') ADVANCE(193);
      END_STATE();
    case 134:
      if (lookahead == 'c') ADVANCE(194);
      END_STATE();
    case 135:
      if (lookahead == 'l') ADVANCE(195);
      END_STATE();
    case 136:
      if (lookahead == 'r') ADVANCE(196);
      END_STATE();
    case 137:
      if (lookahead == 'h') ADVANCE(197);
      END_STATE();
    case 138:
      if (lookahead == 't') ADVANCE(198);
      END_STATE();
    case 139:
      if (lookahead == 't') ADVANCE(199);
      END_STATE();
    case 140:
      if (lookahead == 'r') ADVANCE(200);
      END_STATE();
    case 141:
      ACCEPT_TOKEN(anon_sym_data);
      END_STATE();
    case 142:
      if (lookahead == 'a') ADVANCE(201);
      END_STATE();
    case 143:
      if (lookahead == 'r') ADVANCE(202);
      END_STATE();
    case 144:
      if (lookahead == 'B') ADVANCE(203);
      if (lookahead == 'L') ADVANCE(204);
      if (lookahead == 'S') ADVANCE(205);
      if (lookahead == 'T') ADVANCE(206);
      END_STATE();
    case 145:
      ACCEPT_TOKEN(anon_sym_each);
      END_STATE();
    case 146:
      if (lookahead == 'd') ADVANCE(207);
      END_STATE();
    case 147:
      if (lookahead == 'u') ADVANCE(208);
      END_STATE();
    case 148:
      if (lookahead == 'n') ADVANCE(209);
      END_STATE();
    case 149:
      if (lookahead == 'e') ADVANCE(210);
      END_STATE();
    case 150:
      if (lookahead == 'e') ADVANCE(211);
      END_STATE();
    case 151:
      if (lookahead == 'u') ADVANCE(212);
      END_STATE();
    case 152:
      ACCEPT_TOKEN(anon_sym_fork);
      END_STATE();
    case 153:
      if (lookahead == 'a') ADVANCE(213);
      END_STATE();
    case 154:
      if (lookahead == 'a') ADVANCE(214);
      END_STATE();
    case 155:
      if (lookahead == 'h') ADVANCE(215);
      END_STATE();
    case 156:
      if (lookahead == 'e') ADVANCE(216);
      END_STATE();
    case 157:
      if (lookahead == 'r') ADVANCE(217);
      END_STATE();
    case 158:
      if (lookahead == 'u') ADVANCE(218);
      END_STATE();
    case 159:
      ACCEPT_TOKEN(anon_sym_init);
      END_STATE();
    case 160:
      if (lookahead == 'a') ADVANCE(219);
      END_STATE();
    case 161:
      if (lookahead == 't') ADVANCE(220);
      END_STATE();
    case 162:
      if (lookahead == 'r') ADVANCE(221);
      END_STATE();
    case 163:
      if (lookahead == 'p') ADVANCE(222);
      END_STATE();
    case 164:
      ACCEPT_TOKEN(anon_sym_mode);
      END_STATE();
    case 165:
      if (lookahead == 'r') ADVANCE(223);
      END_STATE();
    case 166:
      if (lookahead == 'i') ADVANCE(224);
      END_STATE();
    case 167:
      if (lookahead == 'u') ADVANCE(225);
      END_STATE();
    case 168:
      if (lookahead == 'o') ADVANCE(226);
      END_STATE();
    case 169:
      if (lookahead == 'e') ADVANCE(227);
      END_STATE();
    case 170:
      if (lookahead == 'i') ADVANCE(228);
      END_STATE();
    case 171:
      if (lookahead == 'o') ADVANCE(229);
      END_STATE();
    case 172:
      if (lookahead == 'D') ADVANCE(230);
      END_STATE();
    case 173:
      if (lookahead == 'c') ADVANCE(231);
      END_STATE();
    case 174:
      if (lookahead == 'p') ADVANCE(232);
      END_STATE();
    case 175:
      if (lookahead == 'e') ADVANCE(233);
      END_STATE();
    case 176:
      if (lookahead == 'E') ADVANCE(234);
      if (lookahead == 'R') ADVANCE(235);
      if (lookahead == 'S') ADVANCE(236);
      END_STATE();
    case 177:
      ACCEPT_TOKEN(anon_sym_sink);
      END_STATE();
    case 178:
      ACCEPT_TOKEN(anon_sym_skip);
      END_STATE();
    case 179:
      if (lookahead == 'B') ADVANCE(237);
      if (lookahead == 'O') ADVANCE(238);
      END_STATE();
    case 180:
      if (lookahead == 'n') ADVANCE(239);
      END_STATE();
    case 181:
      if (lookahead == 'e') ADVANCE(240);
      END_STATE();
    case 182:
      ACCEPT_TOKEN(anon_sym_tags);
      END_STATE();
    case 183:
      if (lookahead == 'e') ADVANCE(241);
      END_STATE();
    case 184:
      if (lookahead == 'e') ADVANCE(242);
      END_STATE();
    case 185:
      if (lookahead == 'B') ADVANCE(243);
      if (lookahead == 'o') ADVANCE(244);
      END_STATE();
    case 186:
      if (lookahead == 'e') ADVANCE(245);
      END_STATE();
    case 187:
      ACCEPT_TOKEN(anon_sym_true);
      END_STATE();
    case 188:
      if (lookahead == 'd') ADVANCE(246);
      END_STATE();
    case 189:
      if (lookahead == 'u') ADVANCE(247);
      END_STATE();
    case 190:
      if (lookahead == 'h') ADVANCE(248);
      END_STATE();
    case 191:
      if (lookahead == 'l') ADVANCE(249);
      END_STATE();
    case 192:
      ACCEPT_TOKEN(anon_sym_after);
      END_STATE();
    case 193:
      ACCEPT_TOKEN(anon_sym_anvil);
      END_STATE();
    case 194:
      ACCEPT_TOKEN(anon_sym_async);
      if (lookahead == 'S') ADVANCE(250);
      if (lookahead == 'W') ADVANCE(251);
      END_STATE();
    case 195:
      if (lookahead == 'i') ADVANCE(252);
      END_STATE();
    case 196:
      if (lookahead == 'e') ADVANCE(253);
      END_STATE();
    case 197:
      ACCEPT_TOKEN(anon_sym_bench);
      if (lookahead == 'A') ADVANCE(254);
      END_STATE();
    case 198:
      if (lookahead == 'i') ADVANCE(255);
      END_STATE();
    case 199:
      ACCEPT_TOKEN(anon_sym_count);
      END_STATE();
    case 200:
      if (lookahead == 'e') ADVANCE(256);
      END_STATE();
    case 201:
      if (lookahead == 'r') ADVANCE(257);
      END_STATE();
    case 202:
      if (lookahead == 'i') ADVANCE(258);
      END_STATE();
    case 203:
      if (lookahead == 'a') ADVANCE(259);
      END_STATE();
    case 204:
      if (lookahead == 'i') ADVANCE(260);
      END_STATE();
    case 205:
      if (lookahead == 'p') ADVANCE(261);
      END_STATE();
    case 206:
      if (lookahead == 'a') ADVANCE(262);
      END_STATE();
    case 207:
      if (lookahead == 'i') ADVANCE(263);
      END_STATE();
    case 208:
      if (lookahead == 'd') ADVANCE(264);
      END_STATE();
    case 209:
      if (lookahead == 'e') ADVANCE(265);
      END_STATE();
    case 210:
      ACCEPT_TOKEN(anon_sym_false);
      END_STATE();
    case 211:
      if (lookahead == 'r') ADVANCE(266);
      END_STATE();
    case 212:
      if (lookahead == 'r') ADVANCE(267);
      END_STATE();
    case 213:
      if (lookahead == 't') ADVANCE(268);
      END_STATE();
    case 214:
      if (lookahead == 'l') ADVANCE(269);
      END_STATE();
    case 215:
      if (lookahead == 't') ADVANCE(270);
      END_STATE();
    case 216:
      if (lookahead == 'r') ADVANCE(271);
      END_STATE();
    case 217:
      if (lookahead == 't') ADVANCE(272);
      END_STATE();
    case 218:
      if (lookahead == 'd') ADVANCE(273);
      END_STATE();
    case 219:
      if (lookahead == 't') ADVANCE(274);
      END_STATE();
    case 220:
      ACCEPT_TOKEN(anon_sym_limit);
      END_STATE();
    case 221:
      if (lookahead == 'y') ADVANCE(275);
      END_STATE();
    case 222:
      if (lookahead == 'e') ADVANCE(276);
      END_STATE();
    case 223:
      ACCEPT_TOKEN(anon_sym_order);
      END_STATE();
    case 224:
      if (lookahead == 'e') ADVANCE(277);
      END_STATE();
    case 225:
      if (lookahead == 't') ADVANCE(278);
      END_STATE();
    case 226:
      if (lookahead == 'r') ADVANCE(279);
      END_STATE();
    case 227:
      if (lookahead == 's') ADVANCE(280);
      END_STATE();
    case 228:
      if (lookahead == 'r') ADVANCE(281);
      END_STATE();
    case 229:
      if (lookahead == 'u') ADVANCE(282);
      END_STATE();
    case 230:
      if (lookahead == 'a') ADVANCE(283);
      END_STATE();
    case 231:
      if (lookahead == 't') ADVANCE(284);
      END_STATE();
    case 232:
      ACCEPT_TOKEN(anon_sym_setup);
      END_STATE();
    case 233:
      ACCEPT_TOKEN(anon_sym_shape);
      END_STATE();
    case 234:
      if (lookahead == 'r') ADVANCE(285);
      END_STATE();
    case 235:
      if (lookahead == 'e') ADVANCE(286);
      END_STATE();
    case 236:
      if (lookahead == 't') ADVANCE(287);
      END_STATE();
    case 237:
      if (lookahead == 'y') ADVANCE(288);
      END_STATE();
    case 238:
      if (lookahead == 'r') ADVANCE(289);
      END_STATE();
    case 239:
      if (lookahead == 'A') ADVANCE(290);
      END_STATE();
    case 240:
      ACCEPT_TOKEN(anon_sym_suite);
      END_STATE();
    case 241:
      if (lookahead == 't') ADVANCE(291);
      END_STATE();
    case 242:
      ACCEPT_TOKEN(anon_sym_theme);
      END_STATE();
    case 243:
      if (lookahead == 'a') ADVANCE(292);
      END_STATE();
    case 244:
      if (lookahead == 'u') ADVANCE(293);
      END_STATE();
    case 245:
      ACCEPT_TOKEN(anon_sym_title);
      END_STATE();
    case 246:
      if (lookahead == 'a') ADVANCE(294);
      END_STATE();
    case 247:
      if (lookahead == 'p') ADVANCE(295);
      END_STATE();
    case 248:
      ACCEPT_TOKEN(anon_sym_width);
      END_STATE();
    case 249:
      if (lookahead == 'e') ADVANCE(296);
      END_STATE();
    case 250:
      if (lookahead == 'a') ADVANCE(297);
      END_STATE();
    case 251:
      if (lookahead == 'a') ADVANCE(298);
      END_STATE();
    case 252:
      if (lookahead == 'n') ADVANCE(299);
      END_STATE();
    case 253:
      ACCEPT_TOKEN(anon_sym_before);
      END_STATE();
    case 254:
      if (lookahead == 's') ADVANCE(300);
      END_STATE();
    case 255:
      if (lookahead == 'n') ADVANCE(301);
      END_STATE();
    case 256:
      if (lookahead == 's') ADVANCE(302);
      END_STATE();
    case 257:
      if (lookahead == 'e') ADVANCE(303);
      END_STATE();
    case 258:
      if (lookahead == 'p') ADVANCE(304);
      END_STATE();
    case 259:
      if (lookahead == 'r') ADVANCE(305);
      END_STATE();
    case 260:
      if (lookahead == 'n') ADVANCE(306);
      END_STATE();
    case 261:
      if (lookahead == 'e') ADVANCE(307);
      END_STATE();
    case 262:
      if (lookahead == 'b') ADVANCE(308);
      END_STATE();
    case 263:
      if (lookahead == 'n') ADVANCE(309);
      END_STATE();
    case 264:
      if (lookahead == 'e') ADVANCE(310);
      END_STATE();
    case 265:
      if (lookahead == 's') ADVANCE(311);
      END_STATE();
    case 266:
      if (lookahead == 'W') ADVANCE(312);
      END_STATE();
    case 267:
      if (lookahead == 'e') ADVANCE(313);
      END_STATE();
    case 268:
      ACCEPT_TOKEN(anon_sym_format);
      END_STATE();
    case 269:
      if (lookahead == 'S') ADVANCE(314);
      END_STATE();
    case 270:
      ACCEPT_TOKEN(anon_sym_height);
      END_STATE();
    case 271:
      if (lookahead == 's') ADVANCE(315);
      END_STATE();
    case 272:
      ACCEPT_TOKEN(anon_sym_import);
      END_STATE();
    case 273:
      if (lookahead == 'e') ADVANCE(316);
      END_STATE();
    case 274:
      if (lookahead == 'i') ADVANCE(317);
      END_STATE();
    case 275:
      ACCEPT_TOKEN(anon_sym_memory);
      END_STATE();
    case 276:
      if (lookahead == 'e') ADVANCE(318);
      END_STATE();
    case 277:
      if (lookahead == 'r') ADVANCE(319);
      END_STATE();
    case 278:
      ACCEPT_TOKEN(anon_sym_output);
      END_STATE();
    case 279:
      if (lookahead == 'm') ADVANCE(320);
      END_STATE();
    case 280:
      if (lookahead == 's') ADVANCE(321);
      END_STATE();
    case 281:
      if (lookahead == 'e') ADVANCE(322);
      END_STATE();
    case 282:
      if (lookahead == 'n') ADVANCE(323);
      END_STATE();
    case 283:
      if (lookahead == 't') ADVANCE(324);
      END_STATE();
    case 284:
      if (lookahead == 'o') ADVANCE(325);
      END_STATE();
    case 285:
      if (lookahead == 'r') ADVANCE(326);
      END_STATE();
    case 286:
      if (lookahead == 'g') ADVANCE(327);
      END_STATE();
    case 287:
      if (lookahead == 'd') ADVANCE(328);
      END_STATE();
    case 288:
      ACCEPT_TOKEN(anon_sym_sortBy);
      END_STATE();
    case 289:
      if (lookahead == 'd') ADVANCE(329);
      END_STATE();
    case 290:
      if (lookahead == 'n') ADVANCE(330);
      END_STATE();
    case 291:
      if (lookahead == 'T') ADVANCE(331);
      END_STATE();
    case 292:
      if (lookahead == 's') ADVANCE(332);
      END_STATE();
    case 293:
      if (lookahead == 't') ADVANCE(333);
      END_STATE();
    case 294:
      if (lookahead == 't') ADVANCE(334);
      END_STATE();
    case 295:
      ACCEPT_TOKEN(anon_sym_warmup);
      END_STATE();
    case 296:
      ACCEPT_TOKEN(anon_sym_yScale);
      END_STATE();
    case 297:
      if (lookahead == 'm') ADVANCE(335);
      END_STATE();
    case 298:
      if (lookahead == 'r') ADVANCE(336);
      END_STATE();
    case 299:
      if (lookahead == 'e') ADVANCE(337);
      END_STATE();
    case 300:
      if (lookahead == 'y') ADVANCE(338);
      END_STATE();
    case 301:
      if (lookahead == 'g') ADVANCE(339);
      END_STATE();
    case 302:
      if (lookahead == 'h') ADVANCE(340);
      END_STATE();
    case 303:
      ACCEPT_TOKEN(anon_sym_declare);
      END_STATE();
    case 304:
      if (lookahead == 't') ADVANCE(341);
      END_STATE();
    case 305:
      if (lookahead == 'C') ADVANCE(342);
      END_STATE();
    case 306:
      if (lookahead == 'e') ADVANCE(343);
      END_STATE();
    case 307:
      if (lookahead == 'e') ADVANCE(344);
      END_STATE();
    case 308:
      if (lookahead == 'l') ADVANCE(345);
      END_STATE();
    case 309:
      if (lookahead == 'g') ADVANCE(346);
      END_STATE();
    case 310:
      if (lookahead == 'B') ADVANCE(347);
      END_STATE();
    case 311:
      if (lookahead == 's') ADVANCE(348);
      END_STATE();
    case 312:
      if (lookahead == 'i') ADVANCE(349);
      END_STATE();
    case 313:
      ACCEPT_TOKEN(anon_sym_fixture);
      END_STATE();
    case 314:
      if (lookahead == 'e') ADVANCE(350);
      END_STATE();
    case 315:
      ACCEPT_TOKEN(anon_sym_helpers);
      END_STATE();
    case 316:
      if (lookahead == 'B') ADVANCE(351);
      END_STATE();
    case 317:
      if (lookahead == 'o') ADVANCE(352);
      END_STATE();
    case 318:
      if (lookahead == 'd') ADVANCE(353);
      END_STATE();
    case 319:
      if (lookahead == 'D') ADVANCE(354);
      END_STATE();
    case 320:
      if (lookahead == 'a') ADVANCE(355);
      END_STATE();
    case 321:
      if (lookahead == 'i') ADVANCE(356);
      END_STATE();
    case 322:
      if (lookahead == 's') ADVANCE(357);
      END_STATE();
    case 323:
      if (lookahead == 't') ADVANCE(358);
      END_STATE();
    case 324:
      if (lookahead == 'a') ADVANCE(359);
      END_STATE();
    case 325:
      if (lookahead == 'r') ADVANCE(360);
      END_STATE();
    case 326:
      if (lookahead == 'o') ADVANCE(361);
      END_STATE();
    case 327:
      if (lookahead == 'r') ADVANCE(362);
      END_STATE();
    case 328:
      if (lookahead == 'D') ADVANCE(363);
      END_STATE();
    case 329:
      if (lookahead == 'e') ADVANCE(364);
      END_STATE();
    case 330:
      if (lookahead == 'v') ADVANCE(365);
      END_STATE();
    case 331:
      if (lookahead == 'i') ADVANCE(366);
      END_STATE();
    case 332:
      if (lookahead == 'e') ADVANCE(367);
      END_STATE();
    case 333:
      ACCEPT_TOKEN(anon_sym_timeout);
      END_STATE();
    case 334:
      if (lookahead == 'e') ADVANCE(368);
      END_STATE();
    case 335:
      if (lookahead == 'p') ADVANCE(369);
      END_STATE();
    case 336:
      if (lookahead == 'm') ADVANCE(370);
      END_STATE();
    case 337:
      ACCEPT_TOKEN(anon_sym_baseline);
      if (lookahead == 'B') ADVANCE(371);
      END_STATE();
    case 338:
      if (lookahead == 'n') ADVANCE(372);
      END_STATE();
    case 339:
      ACCEPT_TOKEN(anon_sym_charting);
      END_STATE();
    case 340:
      if (lookahead == 'o') ADVANCE(373);
      END_STATE();
    case 341:
      if (lookahead == 'i') ADVANCE(374);
      END_STATE();
    case 342:
      if (lookahead == 'h') ADVANCE(375);
      END_STATE();
    case 343:
      if (lookahead == 'C') ADVANCE(376);
      END_STATE();
    case 344:
      if (lookahead == 'd') ADVANCE(377);
      END_STATE();
    case 345:
      if (lookahead == 'e') ADVANCE(378);
      END_STATE();
    case 346:
      ACCEPT_TOKEN(anon_sym_encoding);
      END_STATE();
    case 347:
      if (lookahead == 'e') ADVANCE(379);
      END_STATE();
    case 348:
      ACCEPT_TOKEN(anon_sym_fairness);
      if (lookahead == 'S') ADVANCE(380);
      END_STATE();
    case 349:
      if (lookahead == 'n') ADVANCE(381);
      END_STATE();
    case 350:
      if (lookahead == 't') ADVANCE(382);
      END_STATE();
    case 351:
      if (lookahead == 'e') ADVANCE(383);
      END_STATE();
    case 352:
      if (lookahead == 'n') ADVANCE(384);
      END_STATE();
    case 353:
      if (lookahead == 'u') ADVANCE(385);
      END_STATE();
    case 354:
      if (lookahead == 'e') ADVANCE(386);
      END_STATE();
    case 355:
      if (lookahead == 'n') ADVANCE(387);
      END_STATE();
    case 356:
      if (lookahead == 'o') ADVANCE(388);
      END_STATE();
    case 357:
      ACCEPT_TOKEN(anon_sym_requires);
      END_STATE();
    case 358:
      ACCEPT_TOKEN(anon_sym_rowCount);
      END_STATE();
    case 359:
      if (lookahead == 's') ADVANCE(389);
      END_STATE();
    case 360:
      ACCEPT_TOKEN(anon_sym_selector);
      END_STATE();
    case 361:
      if (lookahead == 'r') ADVANCE(390);
      END_STATE();
    case 362:
      if (lookahead == 'e') ADVANCE(391);
      END_STATE();
    case 363:
      if (lookahead == 'e') ADVANCE(392);
      END_STATE();
    case 364:
      if (lookahead == 'r') ADVANCE(393);
      END_STATE();
    case 365:
      if (lookahead == 'i') ADVANCE(394);
      END_STATE();
    case 366:
      if (lookahead == 'm') ADVANCE(395);
      END_STATE();
    case 367:
      if (lookahead == 'd') ADVANCE(396);
      END_STATE();
    case 368:
      ACCEPT_TOKEN(anon_sym_validate);
      END_STATE();
    case 369:
      if (lookahead == 'l') ADVANCE(397);
      END_STATE();
    case 370:
      if (lookahead == 'u') ADVANCE(398);
      END_STATE();
    case 371:
      if (lookahead == 'e') ADVANCE(399);
      END_STATE();
    case 372:
      if (lookahead == 'c') ADVANCE(400);
      END_STATE();
    case 373:
      if (lookahead == 'l') ADVANCE(401);
      END_STATE();
    case 374:
      if (lookahead == 'o') ADVANCE(402);
      END_STATE();
    case 375:
      if (lookahead == 'a') ADVANCE(403);
      END_STATE();
    case 376:
      if (lookahead == 'h') ADVANCE(404);
      END_STATE();
    case 377:
      if (lookahead == 'u') ADVANCE(405);
      END_STATE();
    case 378:
      ACCEPT_TOKEN(anon_sym_drawTable);
      END_STATE();
    case 379:
      if (lookahead == 'n') ADVANCE(406);
      END_STATE();
    case 380:
      if (lookahead == 'e') ADVANCE(407);
      END_STATE();
    case 381:
      if (lookahead == 'n') ADVANCE(408);
      END_STATE();
    case 382:
      if (lookahead == 'u') ADVANCE(409);
      END_STATE();
    case 383:
      if (lookahead == 'n') ADVANCE(410);
      END_STATE();
    case 384:
      if (lookahead == 'B') ADVANCE(411);
      if (lookahead == 's') ADVANCE(412);
      END_STATE();
    case 385:
      if (lookahead == 'p') ADVANCE(413);
      END_STATE();
    case 386:
      if (lookahead == 't') ADVANCE(414);
      END_STATE();
    case 387:
      if (lookahead == 'c') ADVANCE(415);
      END_STATE();
    case 388:
      if (lookahead == 'n') ADVANCE(416);
      END_STATE();
    case 389:
      if (lookahead == 'e') ADVANCE(417);
      END_STATE();
    case 390:
      if (lookahead == 'B') ADVANCE(418);
      END_STATE();
    case 391:
      if (lookahead == 's') ADVANCE(419);
      END_STATE();
    case 392:
      if (lookahead == 'v') ADVANCE(420);
      END_STATE();
    case 393:
      ACCEPT_TOKEN(anon_sym_sortOrder);
      END_STATE();
    case 394:
      if (lookahead == 'l') ADVANCE(421);
      END_STATE();
    case 395:
      if (lookahead == 'e') ADVANCE(422);
      END_STATE();
    case 396:
      ACCEPT_TOKEN(anon_sym_timeBased);
      END_STATE();
    case 397:
      if (lookahead == 'e') ADVANCE(423);
      if (lookahead == 'i') ADVANCE(424);
      END_STATE();
    case 398:
      if (lookahead == 'p') ADVANCE(425);
      END_STATE();
    case 399:
      if (lookahead == 'n') ADVANCE(426);
      END_STATE();
    case 400:
      ACCEPT_TOKEN(anon_sym_benchAsync);
      END_STATE();
    case 401:
      if (lookahead == 'd') ADVANCE(427);
      END_STATE();
    case 402:
      if (lookahead == 'n') ADVANCE(428);
      END_STATE();
    case 403:
      if (lookahead == 'r') ADVANCE(429);
      END_STATE();
    case 404:
      if (lookahead == 'a') ADVANCE(430);
      END_STATE();
    case 405:
      if (lookahead == 'p') ADVANCE(431);
      END_STATE();
    case 406:
      if (lookahead == 'c') ADVANCE(432);
      END_STATE();
    case 407:
      if (lookahead == 'e') ADVANCE(433);
      END_STATE();
    case 408:
      if (lookahead == 'e') ADVANCE(434);
      END_STATE();
    case 409:
      if (lookahead == 'p') ADVANCE(435);
      END_STATE();
    case 410:
      if (lookahead == 'c') ADVANCE(436);
      END_STATE();
    case 411:
      if (lookahead == 'a') ADVANCE(437);
      END_STATE();
    case 412:
      ACCEPT_TOKEN(anon_sym_iterations);
      END_STATE();
    case 413:
      ACCEPT_TOKEN(anon_sym_minSpeedup);
      END_STATE();
    case 414:
      if (lookahead == 'e') ADVANCE(438);
      END_STATE();
    case 415:
      if (lookahead == 'e') ADVANCE(439);
      END_STATE();
    case 416:
      if (lookahead == 'M') ADVANCE(440);
      END_STATE();
    case 417:
      if (lookahead == 't') ADVANCE(441);
      END_STATE();
    case 418:
      if (lookahead == 'a') ADVANCE(442);
      END_STATE();
    case 419:
      if (lookahead == 's') ADVANCE(443);
      END_STATE();
    case 420:
      ACCEPT_TOKEN(anon_sym_showStdDev);
      END_STATE();
    case 421:
      ACCEPT_TOKEN(anon_sym_spawnAnvil);
      END_STATE();
    case 422:
      ACCEPT_TOKEN(anon_sym_targetTime);
      END_STATE();
    case 423:
      if (lookahead == 'C') ADVANCE(444);
      END_STATE();
    case 424:
      if (lookahead == 'n') ADVANCE(445);
      END_STATE();
    case 425:
      if (lookahead == 'C') ADVANCE(446);
      END_STATE();
    case 426:
      if (lookahead == 'c') ADVANCE(447);
      END_STATE();
    case 427:
      ACCEPT_TOKEN(anon_sym_cvThreshold);
      END_STATE();
    case 428:
      ACCEPT_TOKEN(anon_sym_description);
      END_STATE();
    case 429:
      if (lookahead == 't') ADVANCE(448);
      END_STATE();
    case 430:
      if (lookahead == 'r') ADVANCE(449);
      END_STATE();
    case 431:
      if (lookahead == 'C') ADVANCE(450);
      END_STATE();
    case 432:
      if (lookahead == 'h') ADVANCE(451);
      END_STATE();
    case 433:
      if (lookahead == 'd') ADVANCE(452);
      END_STATE();
    case 434:
      if (lookahead == 'r') ADVANCE(453);
      END_STATE();
    case 435:
      ACCEPT_TOKEN(anon_sym_globalSetup);
      END_STATE();
    case 436:
      if (lookahead == 'h') ADVANCE(454);
      END_STATE();
    case 437:
      if (lookahead == 's') ADVANCE(455);
      END_STATE();
    case 438:
      if (lookahead == 'c') ADVANCE(456);
      END_STATE();
    case 439:
      ACCEPT_TOKEN(anon_sym_performance);
      END_STATE();
    case 440:
      if (lookahead == 'o') ADVANCE(457);
      END_STATE();
    case 441:
      ACCEPT_TOKEN(anon_sym_sameDataset);
      END_STATE();
    case 442:
      if (lookahead == 'r') ADVANCE(458);
      END_STATE();
    case 443:
      if (lookahead == 'i') ADVANCE(459);
      END_STATE();
    case 444:
      if (lookahead == 'a') ADVANCE(460);
      END_STATE();
    case 445:
      if (lookahead == 'g') ADVANCE(461);
      END_STATE();
    case 446:
      if (lookahead == 'a') ADVANCE(462);
      END_STATE();
    case 447:
      if (lookahead == 'h') ADVANCE(463);
      END_STATE();
    case 448:
      ACCEPT_TOKEN(anon_sym_drawBarChart);
      END_STATE();
    case 449:
      if (lookahead == 't') ADVANCE(464);
      END_STATE();
    case 450:
      if (lookahead == 'h') ADVANCE(465);
      END_STATE();
    case 451:
      if (lookahead == 'm') ADVANCE(466);
      END_STATE();
    case 452:
      ACCEPT_TOKEN(anon_sym_fairnessSeed);
      END_STATE();
    case 453:
      ACCEPT_TOKEN(anon_sym_filterWinner);
      END_STATE();
    case 454:
      if (lookahead == 'm') ADVANCE(467);
      END_STATE();
    case 455:
      if (lookahead == 'e') ADVANCE(468);
      END_STATE();
    case 456:
      if (lookahead == 't') ADVANCE(469);
      END_STATE();
    case 457:
      if (lookahead == 'd') ADVANCE(470);
      END_STATE();
    case 458:
      if (lookahead == 's') ADVANCE(471);
      END_STATE();
    case 459:
      if (lookahead == 'o') ADVANCE(472);
      END_STATE();
    case 460:
      if (lookahead == 'p') ADVANCE(473);
      END_STATE();
    case 461:
      if (lookahead == 'P') ADVANCE(474);
      END_STATE();
    case 462:
      if (lookahead == 'p') ADVANCE(475);
      END_STATE();
    case 463:
      if (lookahead == 'm') ADVANCE(476);
      END_STATE();
    case 464:
      ACCEPT_TOKEN(anon_sym_drawLineChart);
      END_STATE();
    case 465:
      if (lookahead == 'a') ADVANCE(477);
      END_STATE();
    case 466:
      if (lookahead == 'a') ADVANCE(478);
      END_STATE();
    case 467:
      if (lookahead == 'a') ADVANCE(479);
      END_STATE();
    case 468:
      if (lookahead == 'd') ADVANCE(480);
      END_STATE();
    case 469:
      if (lookahead == 'i') ADVANCE(481);
      END_STATE();
    case 470:
      if (lookahead == 'e') ADVANCE(482);
      END_STATE();
    case 471:
      ACCEPT_TOKEN(anon_sym_showErrorBars);
      END_STATE();
    case 472:
      if (lookahead == 'n') ADVANCE(483);
      END_STATE();
    case 473:
      ACCEPT_TOKEN(anon_sym_asyncSampleCap);
      END_STATE();
    case 474:
      if (lookahead == 'o') ADVANCE(484);
      END_STATE();
    case 475:
      ACCEPT_TOKEN(anon_sym_asyncWarmupCap);
      END_STATE();
    case 476:
      if (lookahead == 'a') ADVANCE(485);
      END_STATE();
    case 477:
      if (lookahead == 'r') ADVANCE(486);
      END_STATE();
    case 478:
      if (lookahead == 'r') ADVANCE(487);
      END_STATE();
    case 479:
      if (lookahead == 'r') ADVANCE(488);
      END_STATE();
    case 480:
      ACCEPT_TOKEN(anon_sym_iterationBased);
      END_STATE();
    case 481:
      if (lookahead == 'o') ADVANCE(489);
      END_STATE();
    case 482:
      if (lookahead == 'l') ADVANCE(490);
      END_STATE();
    case 483:
      ACCEPT_TOKEN(anon_sym_showRegression);
      END_STATE();
    case 484:
      if (lookahead == 'l') ADVANCE(491);
      END_STATE();
    case 485:
      if (lookahead == 'r') ADVANCE(492);
      END_STATE();
    case 486:
      if (lookahead == 't') ADVANCE(493);
      END_STATE();
    case 487:
      if (lookahead == 'k') ADVANCE(494);
      END_STATE();
    case 488:
      if (lookahead == 'k') ADVANCE(495);
      END_STATE();
    case 489:
      if (lookahead == 'n') ADVANCE(496);
      END_STATE();
    case 490:
      ACCEPT_TOKEN(anon_sym_regressionModel);
      END_STATE();
    case 491:
      if (lookahead == 'i') ADVANCE(497);
      END_STATE();
    case 492:
      if (lookahead == 'k') ADVANCE(498);
      END_STATE();
    case 493:
      ACCEPT_TOKEN(anon_sym_drawSpeedupChart);
      END_STATE();
    case 494:
      if (lookahead == 's') ADVANCE(499);
      END_STATE();
    case 495:
      if (lookahead == 's') ADVANCE(500);
      END_STATE();
    case 496:
      ACCEPT_TOKEN(anon_sym_outlierDetection);
      END_STATE();
    case 497:
      if (lookahead == 'c') ADVANCE(501);
      END_STATE();
    case 498:
      ACCEPT_TOKEN(anon_sym_baselineBenchmark);
      END_STATE();
    case 499:
      ACCEPT_TOKEN(anon_sym_excludeBenchmarks);
      END_STATE();
    case 500:
      ACCEPT_TOKEN(anon_sym_includeBenchmarks);
      END_STATE();
    case 501:
      if (lookahead == 'y') ADVANCE(502);
      END_STATE();
    case 502:
      ACCEPT_TOKEN(anon_sym_asyncSamplingPolicy);
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
  [91] = {.lex_state = 0},
  [92] = {.lex_state = 0},
  [93] = {.lex_state = 0},
  [94] = {.lex_state = 1},
  [95] = {.lex_state = 3},
  [96] = {.lex_state = 0},
  [97] = {.lex_state = 0},
  [98] = {.lex_state = 0},
  [99] = {.lex_state = 0},
  [100] = {.lex_state = 0},
  [101] = {.lex_state = 0},
  [102] = {.lex_state = 0},
  [103] = {.lex_state = 0},
  [104] = {.lex_state = 0},
  [105] = {.lex_state = 0},
  [106] = {.lex_state = 1},
  [107] = {.lex_state = 0},
  [108] = {.lex_state = 3},
  [109] = {.lex_state = 0},
  [110] = {.lex_state = 0},
  [111] = {.lex_state = 0},
  [112] = {.lex_state = 4},
  [113] = {.lex_state = 0},
  [114] = {.lex_state = 0},
  [115] = {.lex_state = 0},
  [116] = {.lex_state = 0},
  [117] = {.lex_state = 0},
  [118] = {.lex_state = 1},
  [119] = {.lex_state = 0},
  [120] = {.lex_state = 3},
  [121] = {.lex_state = 4},
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
  [161] = {.lex_state = 0},
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
  [189] = {.lex_state = 0, .external_lex_state = 2},
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
  [213] = {.lex_state = 0},
  [214] = {.lex_state = 0},
  [215] = {.lex_state = 0},
  [216] = {.lex_state = 0},
  [217] = {.lex_state = 0},
  [218] = {.lex_state = 0},
  [219] = {.lex_state = 0},
  [220] = {.lex_state = 0},
  [221] = {.lex_state = 0},
  [222] = {.lex_state = 0},
  [223] = {.lex_state = 0},
  [224] = {.lex_state = 0},
  [225] = {.lex_state = 0},
  [226] = {.lex_state = 0},
  [227] = {.lex_state = 0},
  [228] = {.lex_state = 0},
  [229] = {.lex_state = 0},
  [230] = {.lex_state = 0},
  [231] = {.lex_state = 0},
  [232] = {.lex_state = 0},
  [233] = {.lex_state = 0},
  [234] = {.lex_state = 0},
  [235] = {.lex_state = 0},
  [236] = {.lex_state = 0},
  [237] = {.lex_state = 0},
  [238] = {.lex_state = 0},
  [239] = {.lex_state = 0},
  [240] = {.lex_state = 2},
  [241] = {.lex_state = 0},
  [242] = {.lex_state = 0},
  [243] = {.lex_state = 0},
  [244] = {.lex_state = 0},
  [245] = {.lex_state = 0},
  [246] = {.lex_state = 0},
  [247] = {.lex_state = 0},
  [248] = {.lex_state = 0},
  [249] = {.lex_state = 0},
  [250] = {.lex_state = 0},
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
    [anon_sym_declare] = ACTIONS(1),
    [anon_sym_suite] = ACTIONS(1),
    [anon_sym_sameDataset] = ACTIONS(1),
    [anon_sym_performance] = ACTIONS(1),
    [anon_sym_memory] = ACTIONS(1),
    [anon_sym_timeBased] = ACTIONS(1),
    [anon_sym_iterationBased] = ACTIONS(1),
    [anon_sym_setup] = ACTIONS(1),
    [anon_sym_import] = ACTIONS(1),
    [anon_sym_async] = ACTIONS(1),
    [anon_sym_init] = ACTIONS(1),
    [anon_sym_helpers] = ACTIONS(1),
    [anon_sym_fixture] = ACTIONS(1),
    [anon_sym_hex] = ACTIONS(1),
    [anon_sym_data] = ACTIONS(1),
    [anon_sym_encoding] = ACTIONS(1),
    [anon_sym_format] = ACTIONS(1),
    [anon_sym_selector] = ACTIONS(1),
    [anon_sym_shape] = ACTIONS(1),
    [anon_sym_ATfile] = ACTIONS(1),
    [anon_sym_bench] = ACTIONS(1),
    [anon_sym_benchAsync] = ACTIONS(1),
    [anon_sym_tags] = ACTIONS(1),
    [anon_sym_skip] = ACTIONS(1),
    [anon_sym_validate] = ACTIONS(1),
    [anon_sym_before] = ACTIONS(1),
    [anon_sym_after] = ACTIONS(1),
    [anon_sym_each] = ACTIONS(1),
    [anon_sym_charting] = ACTIONS(1),
    [anon_sym_drawSpeedupChart] = ACTIONS(1),
    [anon_sym_drawTable] = ACTIONS(1),
    [anon_sym_drawLineChart] = ACTIONS(1),
    [anon_sym_drawBarChart] = ACTIONS(1),
    [anon_sym_title] = ACTIONS(1),
    [anon_sym_description] = ACTIONS(1),
    [anon_sym_output] = ACTIONS(1),
    [anon_sym_sortBy] = ACTIONS(1),
    [anon_sym_sortOrder] = ACTIONS(1),
    [anon_sym_baselineBenchmark] = ACTIONS(1),
    [anon_sym_baseline] = ACTIONS(1),
    [anon_sym_filterWinner] = ACTIONS(1),
    [anon_sym_theme] = ACTIONS(1),
    [anon_sym_width] = ACTIONS(1),
    [anon_sym_rowCount] = ACTIONS(1),
    [anon_sym_height] = ACTIONS(1),
    [anon_sym_limit] = ACTIONS(1),
    [anon_sym_minSpeedup] = ACTIONS(1),
    [anon_sym_includeBenchmarks] = ACTIONS(1),
    [anon_sym_excludeBenchmarks] = ACTIONS(1),
    [anon_sym_showStdDev] = ACTIONS(1),
    [anon_sym_showErrorBars] = ACTIONS(1),
    [anon_sym_showRegression] = ACTIONS(1),
    [anon_sym_regressionModel] = ACTIONS(1),
    [anon_sym_yScale] = ACTIONS(1),
    [anon_sym_iterations] = ACTIONS(1),
    [anon_sym_warmup] = ACTIONS(1),
    [anon_sym_timeout] = ACTIONS(1),
    [anon_sym_requires] = ACTIONS(1),
    [anon_sym_order] = ACTIONS(1),
    [anon_sym_mode] = ACTIONS(1),
    [anon_sym_targetTime] = ACTIONS(1),
    [anon_sym_sink] = ACTIONS(1),
    [anon_sym_outlierDetection] = ACTIONS(1),
    [anon_sym_cvThreshold] = ACTIONS(1),
    [anon_sym_count] = ACTIONS(1),
    [anon_sym_fairness] = ACTIONS(1),
    [anon_sym_fairnessSeed] = ACTIONS(1),
    [anon_sym_asyncSamplingPolicy] = ACTIONS(1),
    [anon_sym_asyncWarmupCap] = ACTIONS(1),
    [anon_sym_asyncSampleCap] = ACTIONS(1),
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
    [sym_source_file] = STATE(243),
    [sym_use_statement] = STATE(71),
    [sym_global_setup] = STATE(85),
    [sym_suite] = STATE(86),
    [aux_sym_source_file_repeat1] = STATE(71),
    [aux_sym_source_file_repeat2] = STATE(86),
    [ts_builtin_sym_end] = ACTIONS(5),
    [anon_sym_use] = ACTIONS(7),
    [anon_sym_globalSetup] = ACTIONS(9),
    [anon_sym_declare] = ACTIONS(11),
    [anon_sym_suite] = ACTIONS(13),
    [sym_comment] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(17), 4,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
    ACTIONS(15), 37,
      anon_sym_globalSetup,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
      anon_sym_bench,
      anon_sym_benchAsync,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [49] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(21), 4,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
    ACTIONS(19), 37,
      anon_sym_globalSetup,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
      anon_sym_bench,
      anon_sym_benchAsync,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [98] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(25), 4,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
    ACTIONS(23), 37,
      anon_sym_globalSetup,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
      anon_sym_bench,
      anon_sym_benchAsync,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [147] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(29), 3,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
    ACTIONS(27), 37,
      anon_sym_globalSetup,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
      anon_sym_bench,
      anon_sym_benchAsync,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [195] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(33), 3,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
    ACTIONS(31), 37,
      anon_sym_globalSetup,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
      anon_sym_bench,
      anon_sym_benchAsync,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [243] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(37), 3,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
    ACTIONS(35), 37,
      anon_sym_globalSetup,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
      anon_sym_bench,
      anon_sym_benchAsync,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [291] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(41), 3,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
    ACTIONS(39), 37,
      anon_sym_globalSetup,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
      anon_sym_bench,
      anon_sym_benchAsync,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [339] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(45), 3,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
    ACTIONS(43), 37,
      anon_sym_globalSetup,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
      anon_sym_bench,
      anon_sym_benchAsync,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [387] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(49), 3,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
    ACTIONS(47), 37,
      anon_sym_globalSetup,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
      anon_sym_bench,
      anon_sym_benchAsync,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [435] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(51), 1,
      sym_identifier,
    ACTIONS(53), 1,
      anon_sym_RBRACE,
    ACTIONS(57), 1,
      anon_sym_hex,
    ACTIONS(59), 1,
      anon_sym_data,
    ACTIONS(61), 1,
      anon_sym_encoding,
    ACTIONS(63), 1,
      anon_sym_format,
    ACTIONS(65), 1,
      anon_sym_selector,
    ACTIONS(67), 1,
      anon_sym_shape,
    STATE(218), 1,
      sym_language_tag,
    STATE(249), 1,
      sym_property_name,
    STATE(15), 10,
      sym__fixture_item,
      sym_hex_property,
      sym_data_property,
      sym_encoding_property,
      sym_format_property,
      sym_selector_property,
      sym_shape_property,
      sym_property,
      sym_language_implementation,
      aux_sym_fixture_body_repeat1,
    ACTIONS(55), 19,
      anon_sym_memory,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [502] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(51), 1,
      sym_identifier,
    ACTIONS(69), 1,
      anon_sym_RBRACE,
    ACTIONS(71), 1,
      anon_sym_tags,
    ACTIONS(73), 1,
      anon_sym_skip,
    ACTIONS(75), 1,
      anon_sym_validate,
    ACTIONS(77), 1,
      anon_sym_before,
    ACTIONS(79), 1,
      anon_sym_after,
    ACTIONS(81), 1,
      anon_sym_each,
    STATE(218), 1,
      sym_language_tag,
    STATE(250), 1,
      sym_property_name,
    STATE(14), 10,
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
    ACTIONS(55), 19,
      anon_sym_memory,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [569] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(51), 1,
      sym_identifier,
    ACTIONS(57), 1,
      anon_sym_hex,
    ACTIONS(59), 1,
      anon_sym_data,
    ACTIONS(61), 1,
      anon_sym_encoding,
    ACTIONS(63), 1,
      anon_sym_format,
    ACTIONS(65), 1,
      anon_sym_selector,
    ACTIONS(67), 1,
      anon_sym_shape,
    ACTIONS(83), 1,
      anon_sym_RBRACE,
    STATE(218), 1,
      sym_language_tag,
    STATE(249), 1,
      sym_property_name,
    STATE(11), 10,
      sym__fixture_item,
      sym_hex_property,
      sym_data_property,
      sym_encoding_property,
      sym_format_property,
      sym_selector_property,
      sym_shape_property,
      sym_property,
      sym_language_implementation,
      aux_sym_fixture_body_repeat1,
    ACTIONS(55), 19,
      anon_sym_memory,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [636] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(51), 1,
      sym_identifier,
    ACTIONS(71), 1,
      anon_sym_tags,
    ACTIONS(73), 1,
      anon_sym_skip,
    ACTIONS(75), 1,
      anon_sym_validate,
    ACTIONS(77), 1,
      anon_sym_before,
    ACTIONS(79), 1,
      anon_sym_after,
    ACTIONS(81), 1,
      anon_sym_each,
    ACTIONS(85), 1,
      anon_sym_RBRACE,
    STATE(218), 1,
      sym_language_tag,
    STATE(250), 1,
      sym_property_name,
    STATE(16), 10,
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
    ACTIONS(55), 19,
      anon_sym_memory,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [703] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(87), 1,
      sym_identifier,
    ACTIONS(90), 1,
      anon_sym_RBRACE,
    ACTIONS(95), 1,
      anon_sym_hex,
    ACTIONS(98), 1,
      anon_sym_data,
    ACTIONS(101), 1,
      anon_sym_encoding,
    ACTIONS(104), 1,
      anon_sym_format,
    ACTIONS(107), 1,
      anon_sym_selector,
    ACTIONS(110), 1,
      anon_sym_shape,
    STATE(218), 1,
      sym_language_tag,
    STATE(249), 1,
      sym_property_name,
    STATE(15), 10,
      sym__fixture_item,
      sym_hex_property,
      sym_data_property,
      sym_encoding_property,
      sym_format_property,
      sym_selector_property,
      sym_shape_property,
      sym_property,
      sym_language_implementation,
      aux_sym_fixture_body_repeat1,
    ACTIONS(92), 19,
      anon_sym_memory,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [770] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(113), 1,
      sym_identifier,
    ACTIONS(116), 1,
      anon_sym_RBRACE,
    ACTIONS(121), 1,
      anon_sym_tags,
    ACTIONS(124), 1,
      anon_sym_skip,
    ACTIONS(127), 1,
      anon_sym_validate,
    ACTIONS(130), 1,
      anon_sym_before,
    ACTIONS(133), 1,
      anon_sym_after,
    ACTIONS(136), 1,
      anon_sym_each,
    STATE(218), 1,
      sym_language_tag,
    STATE(250), 1,
      sym_property_name,
    STATE(16), 10,
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
    ACTIONS(118), 19,
      anon_sym_memory,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [837] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(141), 1,
      anon_sym_RBRACE,
    ACTIONS(139), 37,
      anon_sym_globalSetup,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
      anon_sym_bench,
      anon_sym_benchAsync,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [883] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(145), 1,
      anon_sym_RBRACE,
    ACTIONS(143), 37,
      anon_sym_declare,
      anon_sym_memory,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [929] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(149), 1,
      anon_sym_RBRACE,
    ACTIONS(147), 37,
      anon_sym_declare,
      anon_sym_memory,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [975] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(151), 1,
      anon_sym_globalSetup,
    ACTIONS(154), 1,
      anon_sym_RBRACE,
    ACTIONS(159), 1,
      anon_sym_setup,
    ACTIONS(162), 1,
      anon_sym_fixture,
    ACTIONS(165), 1,
      anon_sym_bench,
    ACTIONS(168), 1,
      anon_sym_benchAsync,
    ACTIONS(171), 1,
      anon_sym_after,
    ACTIONS(174), 1,
      anon_sym_fairness,
    STATE(213), 1,
      sym_property_name,
    STATE(20), 8,
      sym_global_setup,
      sym__suite_item,
      sym_setup_block,
      sym_fixture,
      sym_benchmark,
      sym_after_block,
      sym_property,
      aux_sym_suite_body_repeat1,
    ACTIONS(156), 18,
      anon_sym_memory,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [1036] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      anon_sym_globalSetup,
    ACTIONS(55), 1,
      anon_sym_fairness,
    ACTIONS(177), 1,
      anon_sym_RBRACE,
    ACTIONS(181), 1,
      anon_sym_setup,
    ACTIONS(183), 1,
      anon_sym_fixture,
    ACTIONS(185), 1,
      anon_sym_bench,
    ACTIONS(187), 1,
      anon_sym_benchAsync,
    ACTIONS(189), 1,
      anon_sym_after,
    STATE(213), 1,
      sym_property_name,
    STATE(20), 8,
      sym_global_setup,
      sym__suite_item,
      sym_setup_block,
      sym_fixture,
      sym_benchmark,
      sym_after_block,
      sym_property,
      aux_sym_suite_body_repeat1,
    ACTIONS(179), 18,
      anon_sym_memory,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [1097] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      anon_sym_globalSetup,
    ACTIONS(55), 1,
      anon_sym_fairness,
    ACTIONS(181), 1,
      anon_sym_setup,
    ACTIONS(183), 1,
      anon_sym_fixture,
    ACTIONS(185), 1,
      anon_sym_bench,
    ACTIONS(187), 1,
      anon_sym_benchAsync,
    ACTIONS(189), 1,
      anon_sym_after,
    ACTIONS(191), 1,
      anon_sym_RBRACE,
    STATE(213), 1,
      sym_property_name,
    STATE(21), 8,
      sym_global_setup,
      sym__suite_item,
      sym_setup_block,
      sym_fixture,
      sym_benchmark,
      sym_after_block,
      sym_property,
      aux_sym_suite_body_repeat1,
    ACTIONS(179), 18,
      anon_sym_memory,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [1158] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(195), 1,
      anon_sym_RBRACE,
    ACTIONS(193), 32,
      anon_sym_memory,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [1199] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(199), 1,
      anon_sym_RBRACE,
    STATE(7), 1,
      sym_duration_unit,
    ACTIONS(201), 3,
      anon_sym_ms,
      anon_sym_s,
      anon_sym_m,
    ACTIONS(197), 26,
      anon_sym_memory,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [1242] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(199), 1,
      anon_sym_RBRACE,
    STATE(7), 1,
      sym_duration_unit,
    ACTIONS(201), 3,
      anon_sym_ms,
      anon_sym_s,
      anon_sym_m,
    ACTIONS(197), 26,
      anon_sym_memory,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [1285] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(203), 1,
      anon_sym_ms,
    STATE(7), 1,
      sym_duration_unit,
    ACTIONS(197), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(201), 2,
      anon_sym_s,
      anon_sym_m,
    ACTIONS(199), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_benchAsync,
      anon_sym_after,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [1329] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(207), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(205), 27,
      ts_builtin_sym_end,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_suite,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_benchAsync,
      anon_sym_after,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [1366] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(211), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(209), 27,
      ts_builtin_sym_end,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_suite,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_benchAsync,
      anon_sym_after,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [1403] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(215), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(213), 27,
      ts_builtin_sym_end,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_suite,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_benchAsync,
      anon_sym_after,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [1440] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(219), 1,
      anon_sym_RBRACE,
    ACTIONS(217), 26,
      anon_sym_memory,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [1475] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(223), 1,
      anon_sym_RBRACE,
    ACTIONS(221), 26,
      anon_sym_memory,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [1510] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(227), 1,
      anon_sym_RBRACE,
    ACTIONS(225), 26,
      anon_sym_memory,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [1545] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(231), 1,
      anon_sym_RBRACE,
    ACTIONS(229), 26,
      anon_sym_memory,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [1580] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(235), 1,
      anon_sym_RBRACE,
    ACTIONS(233), 26,
      anon_sym_memory,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [1615] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(239), 1,
      anon_sym_RBRACE,
    ACTIONS(237), 26,
      anon_sym_memory,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [1650] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(243), 1,
      anon_sym_RBRACE,
    ACTIONS(241), 26,
      anon_sym_memory,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [1685] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(247), 1,
      anon_sym_RBRACE,
    ACTIONS(245), 26,
      anon_sym_memory,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [1720] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(251), 1,
      anon_sym_RBRACE,
    ACTIONS(249), 26,
      anon_sym_memory,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [1755] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(255), 1,
      anon_sym_RBRACE,
    ACTIONS(253), 26,
      anon_sym_memory,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [1790] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(259), 1,
      anon_sym_RBRACE,
    ACTIONS(257), 26,
      anon_sym_memory,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [1825] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(263), 1,
      anon_sym_RBRACE,
    ACTIONS(261), 26,
      anon_sym_memory,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [1860] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(267), 1,
      anon_sym_RBRACE,
    ACTIONS(265), 26,
      anon_sym_memory,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [1895] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(271), 1,
      anon_sym_RBRACE,
    ACTIONS(269), 26,
      anon_sym_memory,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [1930] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(275), 1,
      anon_sym_RBRACE,
    ACTIONS(273), 26,
      anon_sym_memory,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [1965] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(279), 1,
      anon_sym_RBRACE,
    ACTIONS(277), 26,
      anon_sym_memory,
      anon_sym_tags,
      anon_sym_skip,
      anon_sym_validate,
      anon_sym_before,
      anon_sym_after,
      anon_sym_each,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairness,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      sym_identifier,
  [2000] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(283), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(281), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_benchAsync,
      anon_sym_after,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [2034] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(287), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(285), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_benchAsync,
      anon_sym_after,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [2068] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(291), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(289), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_benchAsync,
      anon_sym_after,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [2102] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(295), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(293), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_benchAsync,
      anon_sym_after,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [2136] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(299), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(297), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_benchAsync,
      anon_sym_after,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [2170] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(303), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(301), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_benchAsync,
      anon_sym_after,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [2204] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(307), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(305), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_benchAsync,
      anon_sym_after,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [2238] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(311), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(309), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_benchAsync,
      anon_sym_after,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [2272] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(315), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(313), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_benchAsync,
      anon_sym_after,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [2306] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(319), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(317), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_benchAsync,
      anon_sym_after,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [2340] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(323), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(321), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_benchAsync,
      anon_sym_after,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [2374] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(327), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(325), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_benchAsync,
      anon_sym_after,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [2408] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(331), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(329), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_benchAsync,
      anon_sym_after,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [2442] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(333), 1,
      anon_sym_RPAREN,
    ACTIONS(337), 1,
      anon_sym_baseline,
    STATE(132), 1,
      sym_chart_param,
    STATE(226), 1,
      sym_chart_params,
    STATE(228), 1,
      sym_chart_param_name,
    ACTIONS(335), 20,
      anon_sym_title,
      anon_sym_description,
      anon_sym_output,
      anon_sym_sortBy,
      anon_sym_sortOrder,
      anon_sym_baselineBenchmark,
      anon_sym_filterWinner,
      anon_sym_theme,
      anon_sym_width,
      anon_sym_rowCount,
      anon_sym_height,
      anon_sym_limit,
      anon_sym_minSpeedup,
      anon_sym_includeBenchmarks,
      anon_sym_excludeBenchmarks,
      anon_sym_showStdDev,
      anon_sym_showErrorBars,
      anon_sym_showRegression,
      anon_sym_regressionModel,
      anon_sym_yScale,
  [2483] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(337), 1,
      anon_sym_baseline,
    ACTIONS(339), 1,
      anon_sym_RPAREN,
    STATE(172), 1,
      sym_chart_param,
    STATE(228), 1,
      sym_chart_param_name,
    ACTIONS(335), 20,
      anon_sym_title,
      anon_sym_description,
      anon_sym_output,
      anon_sym_sortBy,
      anon_sym_sortOrder,
      anon_sym_baselineBenchmark,
      anon_sym_filterWinner,
      anon_sym_theme,
      anon_sym_width,
      anon_sym_rowCount,
      anon_sym_height,
      anon_sym_limit,
      anon_sym_minSpeedup,
      anon_sym_includeBenchmarks,
      anon_sym_excludeBenchmarks,
      anon_sym_showStdDev,
      anon_sym_showErrorBars,
      anon_sym_showRegression,
      anon_sym_regressionModel,
      anon_sym_yScale,
  [2521] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(337), 1,
      anon_sym_baseline,
    ACTIONS(341), 1,
      anon_sym_RPAREN,
    STATE(172), 1,
      sym_chart_param,
    STATE(228), 1,
      sym_chart_param_name,
    ACTIONS(335), 20,
      anon_sym_title,
      anon_sym_description,
      anon_sym_output,
      anon_sym_sortBy,
      anon_sym_sortOrder,
      anon_sym_baselineBenchmark,
      anon_sym_filterWinner,
      anon_sym_theme,
      anon_sym_width,
      anon_sym_rowCount,
      anon_sym_height,
      anon_sym_limit,
      anon_sym_minSpeedup,
      anon_sym_includeBenchmarks,
      anon_sym_excludeBenchmarks,
      anon_sym_showStdDev,
      anon_sym_showErrorBars,
      anon_sym_showRegression,
      anon_sym_regressionModel,
      anon_sym_yScale,
  [2559] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(337), 1,
      anon_sym_baseline,
    STATE(172), 1,
      sym_chart_param,
    STATE(228), 1,
      sym_chart_param_name,
    ACTIONS(335), 20,
      anon_sym_title,
      anon_sym_description,
      anon_sym_output,
      anon_sym_sortBy,
      anon_sym_sortOrder,
      anon_sym_baselineBenchmark,
      anon_sym_filterWinner,
      anon_sym_theme,
      anon_sym_width,
      anon_sym_rowCount,
      anon_sym_height,
      anon_sym_limit,
      anon_sym_minSpeedup,
      anon_sym_includeBenchmarks,
      anon_sym_excludeBenchmarks,
      anon_sym_showStdDev,
      anon_sym_showErrorBars,
      anon_sym_showRegression,
      anon_sym_regressionModel,
      anon_sym_yScale,
  [2594] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(343), 1,
      sym_identifier,
    ACTIONS(345), 1,
      anon_sym_DQUOTE,
    ACTIONS(347), 1,
      anon_sym_SQUOTE,
    ACTIONS(349), 1,
      sym_number,
    ACTIONS(351), 1,
      sym_float,
    ACTIONS(355), 1,
      anon_sym_LBRACK,
    ACTIONS(353), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(17), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2627] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(345), 1,
      anon_sym_DQUOTE,
    ACTIONS(347), 1,
      anon_sym_SQUOTE,
    ACTIONS(355), 1,
      anon_sym_LBRACK,
    ACTIONS(357), 1,
      sym_identifier,
    ACTIONS(359), 1,
      sym_number,
    ACTIONS(361), 1,
      sym_float,
    ACTIONS(353), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(191), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2660] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(343), 1,
      sym_identifier,
    ACTIONS(345), 1,
      anon_sym_DQUOTE,
    ACTIONS(347), 1,
      anon_sym_SQUOTE,
    ACTIONS(351), 1,
      sym_float,
    ACTIONS(355), 1,
      anon_sym_LBRACK,
    ACTIONS(363), 1,
      sym_number,
    ACTIONS(353), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(17), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2693] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(343), 1,
      sym_identifier,
    ACTIONS(345), 1,
      anon_sym_DQUOTE,
    ACTIONS(347), 1,
      anon_sym_SQUOTE,
    ACTIONS(351), 1,
      sym_float,
    ACTIONS(355), 1,
      anon_sym_LBRACK,
    ACTIONS(365), 1,
      sym_number,
    ACTIONS(353), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(17), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2726] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(367), 1,
      anon_sym_RBRACE,
    ACTIONS(369), 1,
      anon_sym_declare,
    ACTIONS(372), 1,
      anon_sym_import,
    ACTIONS(375), 1,
      anon_sym_async,
    ACTIONS(378), 1,
      anon_sym_init,
    ACTIONS(381), 1,
      anon_sym_helpers,
    STATE(67), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [2756] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(384), 1,
      anon_sym_RBRACE,
    ACTIONS(386), 1,
      anon_sym_declare,
    ACTIONS(388), 1,
      anon_sym_import,
    ACTIONS(390), 1,
      anon_sym_async,
    ACTIONS(392), 1,
      anon_sym_init,
    ACTIONS(394), 1,
      anon_sym_helpers,
    STATE(67), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [2786] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(386), 1,
      anon_sym_declare,
    ACTIONS(388), 1,
      anon_sym_import,
    ACTIONS(390), 1,
      anon_sym_async,
    ACTIONS(392), 1,
      anon_sym_init,
    ACTIONS(394), 1,
      anon_sym_helpers,
    ACTIONS(396), 1,
      anon_sym_RBRACE,
    STATE(68), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [2816] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(345), 1,
      anon_sym_DQUOTE,
    ACTIONS(347), 1,
      anon_sym_SQUOTE,
    ACTIONS(355), 1,
      anon_sym_LBRACK,
    ACTIONS(398), 1,
      sym_number,
    ACTIONS(400), 1,
      sym_float,
    ACTIONS(402), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(170), 4,
      sym__chart_value,
      sym_string,
      sym_boolean,
      sym_string_array,
  [2845] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(7), 1,
      anon_sym_use,
    ACTIONS(9), 1,
      anon_sym_globalSetup,
    ACTIONS(11), 1,
      anon_sym_declare,
    ACTIONS(13), 1,
      anon_sym_suite,
    ACTIONS(404), 1,
      ts_builtin_sym_end,
    STATE(90), 1,
      sym_global_setup,
    STATE(73), 2,
      sym_use_statement,
      aux_sym_source_file_repeat1,
    STATE(88), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [2875] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(406), 1,
      sym_identifier,
    ACTIONS(408), 1,
      anon_sym_RBRACE,
    ACTIONS(410), 1,
      anon_sym_anvil,
    STATE(75), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(135), 2,
      sym_anvil_call,
      sym_function_call,
  [2896] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(414), 1,
      anon_sym_use,
    STATE(73), 2,
      sym_use_statement,
      aux_sym_source_file_repeat1,
    ACTIONS(412), 4,
      ts_builtin_sym_end,
      anon_sym_globalSetup,
      anon_sym_declare,
      anon_sym_suite,
  [2913] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(417), 1,
      sym_identifier,
    ACTIONS(420), 1,
      anon_sym_RBRACE,
    ACTIONS(422), 1,
      anon_sym_anvil,
    STATE(74), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(135), 2,
      sym_anvil_call,
      sym_function_call,
  [2934] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(406), 1,
      sym_identifier,
    ACTIONS(410), 1,
      anon_sym_anvil,
    ACTIONS(425), 1,
      anon_sym_RBRACE,
    STATE(74), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(135), 2,
      sym_anvil_call,
      sym_function_call,
  [2955] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(427), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [2967] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(429), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [2979] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(431), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [2991] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(433), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3003] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(435), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3015] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(201), 1,
      anon_sym_m,
    STATE(7), 1,
      sym_duration_unit,
    ACTIONS(199), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
    ACTIONS(203), 2,
      anon_sym_ms,
      anon_sym_s,
  [3033] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(437), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3045] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(439), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3057] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(441), 1,
      sym_identifier,
    ACTIONS(443), 1,
      anon_sym_COLON,
    STATE(232), 1,
      sym_language_tag,
    STATE(43), 2,
      sym_hook_flat,
      sym_hook_grouped,
  [3074] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_declare,
    ACTIONS(13), 1,
      anon_sym_suite,
    ACTIONS(404), 1,
      ts_builtin_sym_end,
    STATE(88), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3091] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_declare,
    ACTIONS(13), 1,
      anon_sym_suite,
    ACTIONS(404), 1,
      ts_builtin_sym_end,
    STATE(91), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3108] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(445), 1,
      anon_sym_LBRACE,
    STATE(141), 1,
      sym_suite_type,
    STATE(145), 1,
      sym_suite_body,
    ACTIONS(447), 2,
      anon_sym_performance,
      anon_sym_memory,
  [3125] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_declare,
    ACTIONS(13), 1,
      anon_sym_suite,
    ACTIONS(449), 1,
      ts_builtin_sym_end,
    STATE(91), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3142] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(451), 1,
      sym_identifier,
    ACTIONS(454), 1,
      anon_sym_RBRACE,
    STATE(218), 1,
      sym_language_tag,
    STATE(89), 2,
      sym_language_implementation,
      aux_sym_hook_grouped_repeat1,
  [3159] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_declare,
    ACTIONS(13), 1,
      anon_sym_suite,
    ACTIONS(449), 1,
      ts_builtin_sym_end,
    STATE(101), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3176] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(456), 1,
      ts_builtin_sym_end,
    ACTIONS(458), 1,
      anon_sym_declare,
    ACTIONS(461), 1,
      anon_sym_suite,
    STATE(91), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3193] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(441), 1,
      sym_identifier,
    ACTIONS(464), 1,
      anon_sym_RBRACE,
    STATE(218), 1,
      sym_language_tag,
    STATE(89), 2,
      sym_language_implementation,
      aux_sym_hook_grouped_repeat1,
  [3210] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(345), 1,
      anon_sym_DQUOTE,
    ACTIONS(347), 1,
      anon_sym_SQUOTE,
    ACTIONS(466), 1,
      anon_sym_ATfile,
    STATE(37), 2,
      sym_file_ref,
      sym_string,
  [3227] = 5,
    ACTIONS(468), 1,
      anon_sym_DQUOTE,
    ACTIONS(472), 1,
      sym_comment,
    STATE(118), 1,
      aux_sym_string_content_repeat1,
    STATE(208), 1,
      sym_string_content,
    ACTIONS(470), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [3244] = 5,
    ACTIONS(468), 1,
      anon_sym_SQUOTE,
    ACTIONS(472), 1,
      sym_comment,
    STATE(120), 1,
      aux_sym_single_string_content_repeat1,
    STATE(207), 1,
      sym_single_string_content,
    ACTIONS(474), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [3261] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(345), 1,
      anon_sym_DQUOTE,
    ACTIONS(347), 1,
      anon_sym_SQUOTE,
    ACTIONS(466), 1,
      anon_sym_ATfile,
    STATE(36), 2,
      sym_file_ref,
      sym_string,
  [3278] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(193), 1,
      sym_chart_function_name,
    ACTIONS(476), 4,
      anon_sym_drawSpeedupChart,
      anon_sym_drawTable,
      anon_sym_drawLineChart,
      anon_sym_drawBarChart,
  [3291] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(445), 1,
      anon_sym_LBRACE,
    STATE(147), 1,
      sym_suite_type,
    STATE(151), 1,
      sym_suite_body,
    ACTIONS(447), 2,
      anon_sym_performance,
      anon_sym_memory,
  [3308] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(441), 1,
      sym_identifier,
    ACTIONS(443), 1,
      anon_sym_COLON,
    STATE(232), 1,
      sym_language_tag,
    STATE(41), 2,
      sym_hook_flat,
      sym_hook_grouped,
  [3325] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(441), 1,
      sym_identifier,
    ACTIONS(478), 1,
      anon_sym_RBRACE,
    STATE(218), 1,
      sym_language_tag,
    STATE(92), 2,
      sym_language_implementation,
      aux_sym_hook_grouped_repeat1,
  [3342] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_declare,
    ACTIONS(13), 1,
      anon_sym_suite,
    ACTIONS(480), 1,
      ts_builtin_sym_end,
    STATE(91), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3359] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(482), 5,
      ts_builtin_sym_end,
      anon_sym_use,
      anon_sym_globalSetup,
      anon_sym_declare,
      anon_sym_suite,
  [3370] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(441), 1,
      sym_identifier,
    ACTIONS(443), 1,
      anon_sym_COLON,
    STATE(232), 1,
      sym_language_tag,
    STATE(44), 2,
      sym_hook_flat,
      sym_hook_grouped,
  [3387] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(441), 1,
      sym_identifier,
    ACTIONS(443), 1,
      anon_sym_COLON,
    STATE(232), 1,
      sym_language_tag,
    STATE(45), 2,
      sym_hook_flat,
      sym_hook_grouped,
  [3404] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(441), 1,
      sym_identifier,
    ACTIONS(443), 1,
      anon_sym_COLON,
    STATE(232), 1,
      sym_language_tag,
    STATE(42), 2,
      sym_hook_flat,
      sym_hook_grouped,
  [3421] = 4,
    ACTIONS(472), 1,
      sym_comment,
    ACTIONS(484), 1,
      anon_sym_DQUOTE,
    STATE(106), 1,
      aux_sym_string_content_repeat1,
    ACTIONS(486), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [3435] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(489), 1,
      sym_identifier,
    ACTIONS(491), 1,
      anon_sym_RPAREN,
    STATE(159), 1,
      sym_argument,
    STATE(205), 1,
      sym_argument_list,
  [3451] = 4,
    ACTIONS(472), 1,
      sym_comment,
    ACTIONS(493), 1,
      anon_sym_SQUOTE,
    STATE(108), 1,
      aux_sym_single_string_content_repeat1,
    ACTIONS(495), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [3465] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(498), 1,
      anon_sym_LBRACE,
    ACTIONS(500), 1,
      anon_sym_LPAREN,
    STATE(76), 2,
      sym_code_block,
      sym_paren_code_block,
  [3479] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(345), 1,
      anon_sym_DQUOTE,
    ACTIONS(347), 1,
      anon_sym_SQUOTE,
    ACTIONS(502), 1,
      anon_sym_RBRACK,
    STATE(179), 1,
      sym_string,
  [3495] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(489), 1,
      sym_identifier,
    ACTIONS(504), 1,
      anon_sym_RPAREN,
    STATE(159), 1,
      sym_argument,
    STATE(248), 1,
      sym_argument_list,
  [3511] = 4,
    ACTIONS(472), 1,
      sym_comment,
    ACTIONS(506), 1,
      anon_sym_LBRACE,
    ACTIONS(508), 1,
      sym_inline_code,
    STATE(33), 2,
      sym__code_or_inline,
      sym_code_block,
  [3525] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(345), 1,
      anon_sym_DQUOTE,
    ACTIONS(347), 1,
      anon_sym_SQUOTE,
    ACTIONS(510), 1,
      anon_sym_RBRACK,
    STATE(179), 1,
      sym_string,
  [3541] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(345), 1,
      anon_sym_DQUOTE,
    ACTIONS(347), 1,
      anon_sym_SQUOTE,
    ACTIONS(512), 1,
      anon_sym_RBRACK,
    STATE(146), 1,
      sym_string,
  [3557] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(514), 1,
      anon_sym_RBRACE,
    ACTIONS(516), 1,
      anon_sym_charting,
    STATE(115), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [3571] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(345), 1,
      anon_sym_DQUOTE,
    ACTIONS(347), 1,
      anon_sym_SQUOTE,
    ACTIONS(519), 1,
      sym_identifier,
    STATE(35), 1,
      sym_string,
  [3587] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(521), 1,
      anon_sym_LBRACE,
    ACTIONS(523), 1,
      anon_sym_LPAREN,
    STATE(52), 1,
      sym_fixture_body,
    STATE(175), 1,
      sym_fixture_params,
  [3603] = 4,
    ACTIONS(472), 1,
      sym_comment,
    ACTIONS(525), 1,
      anon_sym_DQUOTE,
    STATE(106), 1,
      aux_sym_string_content_repeat1,
    ACTIONS(527), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [3617] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(345), 1,
      anon_sym_DQUOTE,
    ACTIONS(347), 1,
      anon_sym_SQUOTE,
    ACTIONS(529), 1,
      sym_identifier,
    STATE(34), 1,
      sym_string,
  [3633] = 4,
    ACTIONS(472), 1,
      sym_comment,
    ACTIONS(531), 1,
      anon_sym_SQUOTE,
    STATE(108), 1,
      aux_sym_single_string_content_repeat1,
    ACTIONS(533), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [3647] = 4,
    ACTIONS(472), 1,
      sym_comment,
    ACTIONS(506), 1,
      anon_sym_LBRACE,
    ACTIONS(535), 1,
      sym_inline_code,
    STATE(23), 2,
      sym__code_or_inline,
      sym_code_block,
  [3661] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(537), 1,
      anon_sym_RBRACE,
    ACTIONS(539), 1,
      anon_sym_charting,
    STATE(123), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [3675] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(539), 1,
      anon_sym_charting,
    ACTIONS(541), 1,
      anon_sym_RBRACE,
    STATE(115), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [3689] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(543), 1,
      anon_sym_RPAREN,
    ACTIONS(545), 1,
      anon_sym_COMMA,
    STATE(142), 1,
      aux_sym_fixture_params_repeat1,
  [3702] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(182), 1,
      sym_boolean,
    ACTIONS(402), 2,
      anon_sym_true,
      anon_sym_false,
  [3713] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(489), 1,
      sym_identifier,
    ACTIONS(547), 1,
      anon_sym_RPAREN,
    STATE(187), 1,
      sym_argument,
  [3726] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(549), 1,
      anon_sym_RPAREN,
    ACTIONS(551), 1,
      anon_sym_COMMA,
    STATE(127), 1,
      aux_sym_chart_params_repeat1,
  [3739] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(547), 1,
      anon_sym_RPAREN,
    ACTIONS(554), 1,
      anon_sym_COMMA,
    STATE(156), 1,
      aux_sym_argument_list_repeat1,
  [3752] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(341), 1,
      anon_sym_RPAREN,
    ACTIONS(556), 1,
      anon_sym_COMMA,
    STATE(127), 1,
      aux_sym_chart_params_repeat1,
  [3765] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(558), 1,
      sym_identifier,
    ACTIONS(560), 1,
      anon_sym_RPAREN,
    STATE(138), 1,
      sym_fixture_param,
  [3778] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(345), 1,
      anon_sym_DQUOTE,
    ACTIONS(347), 1,
      anon_sym_SQUOTE,
    STATE(179), 1,
      sym_string,
  [3791] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(562), 1,
      anon_sym_RPAREN,
    ACTIONS(564), 1,
      anon_sym_COMMA,
    STATE(129), 1,
      aux_sym_chart_params_repeat1,
  [3804] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(183), 1,
      sym_boolean,
    ACTIONS(402), 2,
      anon_sym_true,
      anon_sym_false,
  [3815] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(345), 1,
      anon_sym_DQUOTE,
    ACTIONS(347), 1,
      anon_sym_SQUOTE,
    STATE(231), 1,
      sym_string,
  [3828] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(568), 1,
      anon_sym_RBRACE,
    ACTIONS(566), 2,
      anon_sym_anvil,
      sym_identifier,
  [3839] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(570), 1,
      anon_sym_COMMA,
    ACTIONS(573), 1,
      anon_sym_RBRACK,
    STATE(136), 1,
      aux_sym_string_array_repeat1,
  [3852] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(577), 1,
      anon_sym_RBRACE,
    ACTIONS(575), 2,
      anon_sym_anvil,
      sym_identifier,
  [3863] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(579), 1,
      anon_sym_RPAREN,
    ACTIONS(581), 1,
      anon_sym_COMMA,
    STATE(124), 1,
      aux_sym_fixture_params_repeat1,
  [3876] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(585), 1,
      anon_sym_RBRACE,
    ACTIONS(583), 2,
      anon_sym_anvil,
      sym_identifier,
  [3887] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(587), 1,
      anon_sym_RPAREN,
    ACTIONS(589), 1,
      anon_sym_fork,
    STATE(210), 1,
      sym_anvil_args,
  [3900] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(220), 1,
      sym_run_mode,
    ACTIONS(591), 2,
      anon_sym_timeBased,
      anon_sym_iterationBased,
  [3911] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(593), 1,
      anon_sym_RPAREN,
    ACTIONS(595), 1,
      anon_sym_COMMA,
    STATE(142), 1,
      aux_sym_fixture_params_repeat1,
  [3924] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(558), 1,
      sym_identifier,
    ACTIONS(598), 1,
      anon_sym_RPAREN,
    STATE(192), 1,
      sym_fixture_param,
  [3937] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(600), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [3946] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(602), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [3955] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(604), 1,
      anon_sym_COMMA,
    ACTIONS(606), 1,
      anon_sym_RBRACK,
    STATE(162), 1,
      aux_sym_string_array_repeat1,
  [3968] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(211), 1,
      sym_run_mode,
    ACTIONS(591), 2,
      anon_sym_timeBased,
      anon_sym_iterationBased,
  [3979] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(345), 1,
      anon_sym_DQUOTE,
    ACTIONS(347), 1,
      anon_sym_SQUOTE,
    STATE(198), 1,
      sym_string,
  [3992] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(610), 1,
      anon_sym_RBRACE,
    ACTIONS(608), 2,
      anon_sym_anvil,
      sym_identifier,
  [4003] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(614), 1,
      anon_sym_RBRACE,
    ACTIONS(612), 2,
      anon_sym_anvil,
      sym_identifier,
  [4014] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(616), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [4023] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(618), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [4032] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(620), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [4041] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(622), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [4050] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(626), 1,
      anon_sym_RBRACE,
    ACTIONS(624), 2,
      anon_sym_anvil,
      sym_identifier,
  [4061] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(628), 1,
      anon_sym_RPAREN,
    ACTIONS(630), 1,
      anon_sym_COMMA,
    STATE(156), 1,
      aux_sym_argument_list_repeat1,
  [4074] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(489), 1,
      sym_identifier,
    ACTIONS(633), 1,
      anon_sym_RPAREN,
    STATE(187), 1,
      sym_argument,
  [4087] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(543), 1,
      anon_sym_RPAREN,
    ACTIONS(558), 1,
      sym_identifier,
    STATE(192), 1,
      sym_fixture_param,
  [4100] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(635), 1,
      anon_sym_RPAREN,
    ACTIONS(637), 1,
      anon_sym_COMMA,
    STATE(128), 1,
      aux_sym_argument_list_repeat1,
  [4113] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(641), 1,
      anon_sym_RBRACE,
    ACTIONS(639), 2,
      anon_sym_anvil,
      sym_identifier,
  [4124] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(345), 1,
      anon_sym_DQUOTE,
    ACTIONS(347), 1,
      anon_sym_SQUOTE,
    STATE(30), 1,
      sym_string,
  [4137] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(502), 1,
      anon_sym_RBRACK,
    ACTIONS(643), 1,
      anon_sym_COMMA,
    STATE(136), 1,
      aux_sym_string_array_repeat1,
  [4150] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(498), 1,
      anon_sym_LBRACE,
    STATE(79), 1,
      sym_code_block,
  [4160] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(645), 1,
      anon_sym_RBRACE,
    ACTIONS(647), 1,
      sym_embedded_code,
  [4170] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(498), 1,
      anon_sym_LBRACE,
    STATE(83), 1,
      sym_code_block,
  [4180] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(649), 2,
      anon_sym_LBRACE,
      anon_sym_COLON,
  [4188] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(498), 1,
      anon_sym_LBRACE,
    STATE(80), 1,
      sym_code_block,
  [4198] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(651), 1,
      anon_sym_LBRACE,
    STATE(27), 1,
      sym_global_setup_body,
  [4208] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(498), 1,
      anon_sym_LBRACE,
    STATE(32), 1,
      sym_code_block,
  [4218] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(653), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4226] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(498), 1,
      anon_sym_LBRACE,
    STATE(78), 1,
      sym_code_block,
  [4236] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(549), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4244] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(355), 1,
      anon_sym_LBRACK,
    STATE(31), 1,
      sym_string_array,
  [4254] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(655), 2,
      anon_sym_RBRACE,
      anon_sym_charting,
  [4262] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(521), 1,
      anon_sym_LBRACE,
    STATE(55), 1,
      sym_fixture_body,
  [4272] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(657), 2,
      anon_sym_RBRACE,
      anon_sym_charting,
  [4280] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(558), 1,
      sym_identifier,
    STATE(192), 1,
      sym_fixture_param,
  [4290] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(659), 1,
      anon_sym_DOT,
    ACTIONS(661), 1,
      anon_sym_LPAREN,
  [4300] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(573), 2,
      anon_sym_COMMA,
      anon_sym_RBRACK,
  [4308] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(663), 2,
      anon_sym_timeBased,
      anon_sym_iterationBased,
  [4316] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(665), 1,
      anon_sym_LBRACE,
    STATE(58), 1,
      sym_after_body,
  [4326] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(445), 1,
      anon_sym_LBRACE,
    STATE(153), 1,
      sym_suite_body,
  [4336] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(445), 1,
      anon_sym_LBRACE,
    STATE(154), 1,
      sym_suite_body,
  [4346] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(667), 1,
      anon_sym_LBRACE,
    STATE(47), 1,
      sym_setup_body,
  [4356] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(489), 1,
      sym_identifier,
    STATE(187), 1,
      sym_argument,
  [4366] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(441), 1,
      sym_identifier,
    STATE(184), 1,
      sym_language_tag,
  [4376] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(628), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4384] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(669), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4392] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(671), 1,
      anon_sym_RPAREN,
    ACTIONS(673), 1,
      sym_embedded_code,
  [4402] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(675), 1,
      anon_sym_LBRACE,
    STATE(50), 1,
      sym_benchmark_body,
  [4412] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(677), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4420] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(593), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4428] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(679), 1,
      anon_sym_LPAREN,
  [4435] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(681), 1,
      anon_sym_COLON,
  [4442] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(683), 1,
      anon_sym_LPAREN,
  [4449] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(685), 1,
      sym_identifier,
  [4456] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(687), 1,
      anon_sym_init,
  [4463] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(689), 1,
      anon_sym_RPAREN,
  [4470] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(691), 1,
      anon_sym_RBRACE,
  [4477] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(693), 1,
      anon_sym_COLON,
  [4484] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(695), 1,
      anon_sym_RPAREN,
  [4491] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(697), 1,
      anon_sym_LPAREN,
  [4498] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(699), 1,
      sym_identifier,
  [4505] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(701), 1,
      anon_sym_spawnAnvil,
  [4512] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(703), 1,
      anon_sym_RPAREN,
  [4519] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(705), 1,
      anon_sym_LBRACE,
  [4526] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(707), 1,
      anon_sym_SQUOTE,
  [4533] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(707), 1,
      anon_sym_DQUOTE,
  [4540] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(709), 1,
      anon_sym_COLON,
  [4547] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(711), 1,
      anon_sym_RPAREN,
  [4554] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(713), 1,
      anon_sym_sameDataset,
  [4561] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(715), 1,
      anon_sym_COLON,
  [4568] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(717), 1,
      anon_sym_COLON,
  [4575] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(719), 1,
      anon_sym_COLON,
  [4582] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(721), 1,
      anon_sym_COLON,
  [4589] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(723), 1,
      anon_sym_LBRACE,
  [4596] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(725), 1,
      anon_sym_sameDataset,
  [4603] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(727), 1,
      anon_sym_COLON,
  [4610] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(729), 1,
      anon_sym_LBRACE,
  [4617] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(731), 1,
      anon_sym_sameDataset,
  [4624] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(733), 1,
      anon_sym_DOT,
  [4631] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(735), 1,
      sym_identifier,
  [4638] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(737), 1,
      sym_identifier,
  [4645] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(739), 1,
      anon_sym_LPAREN,
  [4652] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(741), 1,
      anon_sym_COLON,
  [4659] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(743), 1,
      anon_sym_RPAREN,
  [4666] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(745), 1,
      anon_sym_COLON,
  [4673] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(747), 1,
      anon_sym_COLON,
  [4680] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(749), 1,
      anon_sym_DOT,
  [4687] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(751), 1,
      anon_sym_COLON,
  [4694] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(753), 1,
      anon_sym_RPAREN,
  [4701] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(755), 1,
      anon_sym_COLON,
  [4708] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(757), 1,
      anon_sym_COLON,
  [4715] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(759), 1,
      anon_sym_LPAREN,
  [4722] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(761), 1,
      anon_sym_COLON,
  [4729] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(763), 1,
      anon_sym_COLON,
  [4736] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(765), 1,
      anon_sym_COLON,
  [4743] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(767), 1,
      sym_identifier,
  [4750] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(769), 1,
      anon_sym_LBRACE,
  [4757] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(771), 1,
      anon_sym_COLON_COLON,
  [4764] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(773), 1,
      anon_sym_LBRACE,
  [4771] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(775), 1,
      sym_identifier,
  [4778] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(777), 1,
      ts_builtin_sym_end,
  [4785] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(779), 1,
      sym_identifier,
  [4792] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(781), 1,
      anon_sym_suite,
  [4799] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(783), 1,
      anon_sym_std,
  [4806] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(785), 1,
      anon_sym_COLON,
  [4813] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(787), 1,
      anon_sym_RPAREN,
  [4820] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(789), 1,
      anon_sym_COLON,
  [4827] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(791), 1,
      anon_sym_COLON,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 49,
  [SMALL_STATE(4)] = 98,
  [SMALL_STATE(5)] = 147,
  [SMALL_STATE(6)] = 195,
  [SMALL_STATE(7)] = 243,
  [SMALL_STATE(8)] = 291,
  [SMALL_STATE(9)] = 339,
  [SMALL_STATE(10)] = 387,
  [SMALL_STATE(11)] = 435,
  [SMALL_STATE(12)] = 502,
  [SMALL_STATE(13)] = 569,
  [SMALL_STATE(14)] = 636,
  [SMALL_STATE(15)] = 703,
  [SMALL_STATE(16)] = 770,
  [SMALL_STATE(17)] = 837,
  [SMALL_STATE(18)] = 883,
  [SMALL_STATE(19)] = 929,
  [SMALL_STATE(20)] = 975,
  [SMALL_STATE(21)] = 1036,
  [SMALL_STATE(22)] = 1097,
  [SMALL_STATE(23)] = 1158,
  [SMALL_STATE(24)] = 1199,
  [SMALL_STATE(25)] = 1242,
  [SMALL_STATE(26)] = 1285,
  [SMALL_STATE(27)] = 1329,
  [SMALL_STATE(28)] = 1366,
  [SMALL_STATE(29)] = 1403,
  [SMALL_STATE(30)] = 1440,
  [SMALL_STATE(31)] = 1475,
  [SMALL_STATE(32)] = 1510,
  [SMALL_STATE(33)] = 1545,
  [SMALL_STATE(34)] = 1580,
  [SMALL_STATE(35)] = 1615,
  [SMALL_STATE(36)] = 1650,
  [SMALL_STATE(37)] = 1685,
  [SMALL_STATE(38)] = 1720,
  [SMALL_STATE(39)] = 1755,
  [SMALL_STATE(40)] = 1790,
  [SMALL_STATE(41)] = 1825,
  [SMALL_STATE(42)] = 1860,
  [SMALL_STATE(43)] = 1895,
  [SMALL_STATE(44)] = 1930,
  [SMALL_STATE(45)] = 1965,
  [SMALL_STATE(46)] = 2000,
  [SMALL_STATE(47)] = 2034,
  [SMALL_STATE(48)] = 2068,
  [SMALL_STATE(49)] = 2102,
  [SMALL_STATE(50)] = 2136,
  [SMALL_STATE(51)] = 2170,
  [SMALL_STATE(52)] = 2204,
  [SMALL_STATE(53)] = 2238,
  [SMALL_STATE(54)] = 2272,
  [SMALL_STATE(55)] = 2306,
  [SMALL_STATE(56)] = 2340,
  [SMALL_STATE(57)] = 2374,
  [SMALL_STATE(58)] = 2408,
  [SMALL_STATE(59)] = 2442,
  [SMALL_STATE(60)] = 2483,
  [SMALL_STATE(61)] = 2521,
  [SMALL_STATE(62)] = 2559,
  [SMALL_STATE(63)] = 2594,
  [SMALL_STATE(64)] = 2627,
  [SMALL_STATE(65)] = 2660,
  [SMALL_STATE(66)] = 2693,
  [SMALL_STATE(67)] = 2726,
  [SMALL_STATE(68)] = 2756,
  [SMALL_STATE(69)] = 2786,
  [SMALL_STATE(70)] = 2816,
  [SMALL_STATE(71)] = 2845,
  [SMALL_STATE(72)] = 2875,
  [SMALL_STATE(73)] = 2896,
  [SMALL_STATE(74)] = 2913,
  [SMALL_STATE(75)] = 2934,
  [SMALL_STATE(76)] = 2955,
  [SMALL_STATE(77)] = 2967,
  [SMALL_STATE(78)] = 2979,
  [SMALL_STATE(79)] = 2991,
  [SMALL_STATE(80)] = 3003,
  [SMALL_STATE(81)] = 3015,
  [SMALL_STATE(82)] = 3033,
  [SMALL_STATE(83)] = 3045,
  [SMALL_STATE(84)] = 3057,
  [SMALL_STATE(85)] = 3074,
  [SMALL_STATE(86)] = 3091,
  [SMALL_STATE(87)] = 3108,
  [SMALL_STATE(88)] = 3125,
  [SMALL_STATE(89)] = 3142,
  [SMALL_STATE(90)] = 3159,
  [SMALL_STATE(91)] = 3176,
  [SMALL_STATE(92)] = 3193,
  [SMALL_STATE(93)] = 3210,
  [SMALL_STATE(94)] = 3227,
  [SMALL_STATE(95)] = 3244,
  [SMALL_STATE(96)] = 3261,
  [SMALL_STATE(97)] = 3278,
  [SMALL_STATE(98)] = 3291,
  [SMALL_STATE(99)] = 3308,
  [SMALL_STATE(100)] = 3325,
  [SMALL_STATE(101)] = 3342,
  [SMALL_STATE(102)] = 3359,
  [SMALL_STATE(103)] = 3370,
  [SMALL_STATE(104)] = 3387,
  [SMALL_STATE(105)] = 3404,
  [SMALL_STATE(106)] = 3421,
  [SMALL_STATE(107)] = 3435,
  [SMALL_STATE(108)] = 3451,
  [SMALL_STATE(109)] = 3465,
  [SMALL_STATE(110)] = 3479,
  [SMALL_STATE(111)] = 3495,
  [SMALL_STATE(112)] = 3511,
  [SMALL_STATE(113)] = 3525,
  [SMALL_STATE(114)] = 3541,
  [SMALL_STATE(115)] = 3557,
  [SMALL_STATE(116)] = 3571,
  [SMALL_STATE(117)] = 3587,
  [SMALL_STATE(118)] = 3603,
  [SMALL_STATE(119)] = 3617,
  [SMALL_STATE(120)] = 3633,
  [SMALL_STATE(121)] = 3647,
  [SMALL_STATE(122)] = 3661,
  [SMALL_STATE(123)] = 3675,
  [SMALL_STATE(124)] = 3689,
  [SMALL_STATE(125)] = 3702,
  [SMALL_STATE(126)] = 3713,
  [SMALL_STATE(127)] = 3726,
  [SMALL_STATE(128)] = 3739,
  [SMALL_STATE(129)] = 3752,
  [SMALL_STATE(130)] = 3765,
  [SMALL_STATE(131)] = 3778,
  [SMALL_STATE(132)] = 3791,
  [SMALL_STATE(133)] = 3804,
  [SMALL_STATE(134)] = 3815,
  [SMALL_STATE(135)] = 3828,
  [SMALL_STATE(136)] = 3839,
  [SMALL_STATE(137)] = 3852,
  [SMALL_STATE(138)] = 3863,
  [SMALL_STATE(139)] = 3876,
  [SMALL_STATE(140)] = 3887,
  [SMALL_STATE(141)] = 3900,
  [SMALL_STATE(142)] = 3911,
  [SMALL_STATE(143)] = 3924,
  [SMALL_STATE(144)] = 3937,
  [SMALL_STATE(145)] = 3946,
  [SMALL_STATE(146)] = 3955,
  [SMALL_STATE(147)] = 3968,
  [SMALL_STATE(148)] = 3979,
  [SMALL_STATE(149)] = 3992,
  [SMALL_STATE(150)] = 4003,
  [SMALL_STATE(151)] = 4014,
  [SMALL_STATE(152)] = 4023,
  [SMALL_STATE(153)] = 4032,
  [SMALL_STATE(154)] = 4041,
  [SMALL_STATE(155)] = 4050,
  [SMALL_STATE(156)] = 4061,
  [SMALL_STATE(157)] = 4074,
  [SMALL_STATE(158)] = 4087,
  [SMALL_STATE(159)] = 4100,
  [SMALL_STATE(160)] = 4113,
  [SMALL_STATE(161)] = 4124,
  [SMALL_STATE(162)] = 4137,
  [SMALL_STATE(163)] = 4150,
  [SMALL_STATE(164)] = 4160,
  [SMALL_STATE(165)] = 4170,
  [SMALL_STATE(166)] = 4180,
  [SMALL_STATE(167)] = 4188,
  [SMALL_STATE(168)] = 4198,
  [SMALL_STATE(169)] = 4208,
  [SMALL_STATE(170)] = 4218,
  [SMALL_STATE(171)] = 4226,
  [SMALL_STATE(172)] = 4236,
  [SMALL_STATE(173)] = 4244,
  [SMALL_STATE(174)] = 4254,
  [SMALL_STATE(175)] = 4262,
  [SMALL_STATE(176)] = 4272,
  [SMALL_STATE(177)] = 4280,
  [SMALL_STATE(178)] = 4290,
  [SMALL_STATE(179)] = 4300,
  [SMALL_STATE(180)] = 4308,
  [SMALL_STATE(181)] = 4316,
  [SMALL_STATE(182)] = 4326,
  [SMALL_STATE(183)] = 4336,
  [SMALL_STATE(184)] = 4346,
  [SMALL_STATE(185)] = 4356,
  [SMALL_STATE(186)] = 4366,
  [SMALL_STATE(187)] = 4376,
  [SMALL_STATE(188)] = 4384,
  [SMALL_STATE(189)] = 4392,
  [SMALL_STATE(190)] = 4402,
  [SMALL_STATE(191)] = 4412,
  [SMALL_STATE(192)] = 4420,
  [SMALL_STATE(193)] = 4428,
  [SMALL_STATE(194)] = 4435,
  [SMALL_STATE(195)] = 4442,
  [SMALL_STATE(196)] = 4449,
  [SMALL_STATE(197)] = 4456,
  [SMALL_STATE(198)] = 4463,
  [SMALL_STATE(199)] = 4470,
  [SMALL_STATE(200)] = 4477,
  [SMALL_STATE(201)] = 4484,
  [SMALL_STATE(202)] = 4491,
  [SMALL_STATE(203)] = 4498,
  [SMALL_STATE(204)] = 4505,
  [SMALL_STATE(205)] = 4512,
  [SMALL_STATE(206)] = 4519,
  [SMALL_STATE(207)] = 4526,
  [SMALL_STATE(208)] = 4533,
  [SMALL_STATE(209)] = 4540,
  [SMALL_STATE(210)] = 4547,
  [SMALL_STATE(211)] = 4554,
  [SMALL_STATE(212)] = 4561,
  [SMALL_STATE(213)] = 4568,
  [SMALL_STATE(214)] = 4575,
  [SMALL_STATE(215)] = 4582,
  [SMALL_STATE(216)] = 4589,
  [SMALL_STATE(217)] = 4596,
  [SMALL_STATE(218)] = 4603,
  [SMALL_STATE(219)] = 4610,
  [SMALL_STATE(220)] = 4617,
  [SMALL_STATE(221)] = 4624,
  [SMALL_STATE(222)] = 4631,
  [SMALL_STATE(223)] = 4638,
  [SMALL_STATE(224)] = 4645,
  [SMALL_STATE(225)] = 4652,
  [SMALL_STATE(226)] = 4659,
  [SMALL_STATE(227)] = 4666,
  [SMALL_STATE(228)] = 4673,
  [SMALL_STATE(229)] = 4680,
  [SMALL_STATE(230)] = 4687,
  [SMALL_STATE(231)] = 4694,
  [SMALL_STATE(232)] = 4701,
  [SMALL_STATE(233)] = 4708,
  [SMALL_STATE(234)] = 4715,
  [SMALL_STATE(235)] = 4722,
  [SMALL_STATE(236)] = 4729,
  [SMALL_STATE(237)] = 4736,
  [SMALL_STATE(238)] = 4743,
  [SMALL_STATE(239)] = 4750,
  [SMALL_STATE(240)] = 4757,
  [SMALL_STATE(241)] = 4764,
  [SMALL_STATE(242)] = 4771,
  [SMALL_STATE(243)] = 4778,
  [SMALL_STATE(244)] = 4785,
  [SMALL_STATE(245)] = 4792,
  [SMALL_STATE(246)] = 4799,
  [SMALL_STATE(247)] = 4806,
  [SMALL_STATE(248)] = 4813,
  [SMALL_STATE(249)] = 4820,
  [SMALL_STATE(250)] = 4827,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0, 0, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(246),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(168),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(245),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(244),
  [15] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_boolean, 1, 0, 0),
  [17] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_boolean, 1, 0, 0),
  [19] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 2, 0, 0),
  [21] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 2, 0, 0),
  [23] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 3, 0, 0),
  [25] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 3, 0, 0),
  [27] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 3, 0, 0),
  [29] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 3, 0, 0),
  [31] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_duration_unit, 1, 0, 0),
  [33] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_duration_unit, 1, 0, 0),
  [35] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_duration, 2, 0, 0),
  [37] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_duration, 2, 0, 0),
  [39] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 2, 0, 0),
  [41] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 2, 0, 0),
  [43] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 5, 0, 0),
  [45] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 5, 0, 0),
  [47] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 4, 0, 0),
  [49] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 4, 0, 0),
  [51] = {.entry = {.count = 1, .reusable = false}}, SHIFT(166),
  [53] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [55] = {.entry = {.count = 1, .reusable = false}}, SHIFT(200),
  [57] = {.entry = {.count = 1, .reusable = false}}, SHIFT(237),
  [59] = {.entry = {.count = 1, .reusable = false}}, SHIFT(236),
  [61] = {.entry = {.count = 1, .reusable = false}}, SHIFT(233),
  [63] = {.entry = {.count = 1, .reusable = false}}, SHIFT(230),
  [65] = {.entry = {.count = 1, .reusable = false}}, SHIFT(227),
  [67] = {.entry = {.count = 1, .reusable = false}}, SHIFT(194),
  [69] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [71] = {.entry = {.count = 1, .reusable = false}}, SHIFT(214),
  [73] = {.entry = {.count = 1, .reusable = false}}, SHIFT(99),
  [75] = {.entry = {.count = 1, .reusable = false}}, SHIFT(84),
  [77] = {.entry = {.count = 1, .reusable = false}}, SHIFT(105),
  [79] = {.entry = {.count = 1, .reusable = false}}, SHIFT(104),
  [81] = {.entry = {.count = 1, .reusable = false}}, SHIFT(103),
  [83] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [85] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [87] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(166),
  [90] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0),
  [92] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(200),
  [95] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(237),
  [98] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(236),
  [101] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(233),
  [104] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(230),
  [107] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(227),
  [110] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(194),
  [113] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(166),
  [116] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0),
  [118] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(200),
  [121] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(214),
  [124] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(99),
  [127] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(84),
  [130] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(105),
  [133] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(104),
  [136] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(103),
  [139] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_property, 3, 0, 5),
  [141] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property, 3, 0, 5),
  [143] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_code_block, 3, 0, 0),
  [145] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_code_block, 3, 0, 0),
  [147] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_code_block, 2, 0, 0),
  [149] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_code_block, 2, 0, 0),
  [151] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(168),
  [154] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0),
  [156] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(200),
  [159] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(186),
  [162] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(196),
  [165] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(222),
  [168] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(222),
  [171] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(181),
  [174] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(200),
  [177] = {.entry = {.count = 1, .reusable = true}}, SHIFT(144),
  [179] = {.entry = {.count = 1, .reusable = true}}, SHIFT(200),
  [181] = {.entry = {.count = 1, .reusable = true}}, SHIFT(186),
  [183] = {.entry = {.count = 1, .reusable = true}}, SHIFT(196),
  [185] = {.entry = {.count = 1, .reusable = false}}, SHIFT(222),
  [187] = {.entry = {.count = 1, .reusable = true}}, SHIFT(222),
  [189] = {.entry = {.count = 1, .reusable = true}}, SHIFT(181),
  [191] = {.entry = {.count = 1, .reusable = true}}, SHIFT(152),
  [193] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_language_implementation, 3, 0, 8),
  [195] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_language_implementation, 3, 0, 8),
  [197] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__value, 1, 0, 0),
  [199] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__value, 1, 0, 0),
  [201] = {.entry = {.count = 1, .reusable = false}}, SHIFT(6),
  [203] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [205] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup, 2, 0, 0),
  [207] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup, 2, 0, 0),
  [209] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_body, 3, 0, 0),
  [211] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup_body, 3, 0, 0),
  [213] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_body, 2, 0, 0),
  [215] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup_body, 2, 0, 0),
  [217] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_selector_property, 3, 0, 0),
  [219] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selector_property, 3, 0, 0),
  [221] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_tags_property, 3, 0, 0),
  [223] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tags_property, 3, 0, 0),
  [225] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_shape_property, 3, 0, 0),
  [227] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_shape_property, 3, 0, 0),
  [229] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hook_flat, 3, 0, 8),
  [231] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_flat, 3, 0, 8),
  [233] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_format_property, 3, 0, 0),
  [235] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_format_property, 3, 0, 0),
  [237] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_encoding_property, 3, 0, 0),
  [239] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_encoding_property, 3, 0, 0),
  [241] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_data_property, 3, 0, 0),
  [243] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_data_property, 3, 0, 0),
  [245] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hex_property, 3, 0, 0),
  [247] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hex_property, 3, 0, 0),
  [249] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hook_grouped, 4, 0, 0),
  [251] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_grouped, 4, 0, 0),
  [253] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_file_ref, 4, 0, 0),
  [255] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_file_ref, 4, 0, 0),
  [257] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hook_grouped, 3, 0, 0),
  [259] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_grouped, 3, 0, 0),
  [261] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_skip_hook, 2, 0, 0),
  [263] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_skip_hook, 2, 0, 0),
  [265] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_before_hook, 2, 0, 0),
  [267] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_before_hook, 2, 0, 0),
  [269] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_validate_hook, 2, 0, 0),
  [271] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_validate_hook, 2, 0, 0),
  [273] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_each_hook, 2, 0, 0),
  [275] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_each_hook, 2, 0, 0),
  [277] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_hook, 2, 0, 0),
  [279] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_hook, 2, 0, 0),
  [281] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_body, 2, 0, 0),
  [283] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture_body, 2, 0, 0),
  [285] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_block, 3, 0, 4),
  [287] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_setup_block, 3, 0, 4),
  [289] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark_body, 3, 0, 0),
  [291] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_benchmark_body, 3, 0, 0),
  [293] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_body, 2, 0, 0),
  [295] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_body, 2, 0, 0),
  [297] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark, 3, 0, 1),
  [299] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_benchmark, 3, 0, 1),
  [301] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_body, 3, 0, 0),
  [303] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture_body, 3, 0, 0),
  [305] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture, 3, 0, 1),
  [307] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture, 3, 0, 1),
  [309] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_body, 3, 0, 0),
  [311] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_setup_body, 3, 0, 0),
  [313] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark_body, 2, 0, 0),
  [315] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_benchmark_body, 2, 0, 0),
  [317] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture, 4, 0, 1),
  [319] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture, 4, 0, 1),
  [321] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_body, 3, 0, 0),
  [323] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_body, 3, 0, 0),
  [325] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_body, 2, 0, 0),
  [327] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_setup_body, 2, 0, 0),
  [329] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_block, 2, 0, 0),
  [331] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_block, 2, 0, 0),
  [333] = {.entry = {.count = 1, .reusable = true}}, SHIFT(176),
  [335] = {.entry = {.count = 1, .reusable = true}}, SHIFT(225),
  [337] = {.entry = {.count = 1, .reusable = false}}, SHIFT(225),
  [339] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 3, 0, 0),
  [341] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 2, 0, 0),
  [343] = {.entry = {.count = 1, .reusable = false}}, SHIFT(17),
  [345] = {.entry = {.count = 1, .reusable = true}}, SHIFT(94),
  [347] = {.entry = {.count = 1, .reusable = true}}, SHIFT(95),
  [349] = {.entry = {.count = 1, .reusable = false}}, SHIFT(26),
  [351] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [353] = {.entry = {.count = 1, .reusable = false}}, SHIFT(2),
  [355] = {.entry = {.count = 1, .reusable = true}}, SHIFT(114),
  [357] = {.entry = {.count = 1, .reusable = false}}, SHIFT(191),
  [359] = {.entry = {.count = 1, .reusable = false}}, SHIFT(81),
  [361] = {.entry = {.count = 1, .reusable = true}}, SHIFT(191),
  [363] = {.entry = {.count = 1, .reusable = false}}, SHIFT(24),
  [365] = {.entry = {.count = 1, .reusable = false}}, SHIFT(25),
  [367] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0),
  [369] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(165),
  [372] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(109),
  [375] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(197),
  [378] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(167),
  [381] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(171),
  [384] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [386] = {.entry = {.count = 1, .reusable = true}}, SHIFT(165),
  [388] = {.entry = {.count = 1, .reusable = true}}, SHIFT(109),
  [390] = {.entry = {.count = 1, .reusable = true}}, SHIFT(197),
  [392] = {.entry = {.count = 1, .reusable = true}}, SHIFT(167),
  [394] = {.entry = {.count = 1, .reusable = true}}, SHIFT(171),
  [396] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [398] = {.entry = {.count = 1, .reusable = false}}, SHIFT(170),
  [400] = {.entry = {.count = 1, .reusable = true}}, SHIFT(170),
  [402] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [404] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, 0, 0),
  [406] = {.entry = {.count = 1, .reusable = false}}, SHIFT(178),
  [408] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [410] = {.entry = {.count = 1, .reusable = false}}, SHIFT(221),
  [412] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0),
  [414] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(246),
  [417] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(178),
  [420] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0),
  [422] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(221),
  [425] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [427] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_section, 2, 0, 0),
  [429] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_paren_code_block, 3, 0, 0),
  [431] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_helpers_section, 2, 0, 0),
  [433] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_init_section, 3, 0, 0),
  [435] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_init_section, 2, 0, 0),
  [437] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_paren_code_block, 2, 0, 0),
  [439] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declare_section, 2, 0, 0),
  [441] = {.entry = {.count = 1, .reusable = true}}, SHIFT(166),
  [443] = {.entry = {.count = 1, .reusable = true}}, SHIFT(239),
  [445] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [447] = {.entry = {.count = 1, .reusable = true}}, SHIFT(180),
  [449] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, 0, 0),
  [451] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_hook_grouped_repeat1, 2, 0, 0), SHIFT_REPEAT(166),
  [454] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_hook_grouped_repeat1, 2, 0, 0),
  [456] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat2, 2, 0, 0),
  [458] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat2, 2, 0, 0), SHIFT_REPEAT(245),
  [461] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat2, 2, 0, 0), SHIFT_REPEAT(244),
  [464] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [466] = {.entry = {.count = 1, .reusable = true}}, SHIFT(202),
  [468] = {.entry = {.count = 1, .reusable = false}}, SHIFT(3),
  [470] = {.entry = {.count = 1, .reusable = false}}, SHIFT(118),
  [472] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [474] = {.entry = {.count = 1, .reusable = false}}, SHIFT(120),
  [476] = {.entry = {.count = 1, .reusable = true}}, SHIFT(195),
  [478] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [480] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, 0, 0),
  [482] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_use_statement, 4, 0, 2),
  [484] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_string_content_repeat1, 2, 0, 0),
  [486] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_string_content_repeat1, 2, 0, 0), SHIFT_REPEAT(106),
  [489] = {.entry = {.count = 1, .reusable = true}}, SHIFT(247),
  [491] = {.entry = {.count = 1, .reusable = true}}, SHIFT(139),
  [493] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_single_string_content_repeat1, 2, 0, 0),
  [495] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_single_string_content_repeat1, 2, 0, 0), SHIFT_REPEAT(108),
  [498] = {.entry = {.count = 1, .reusable = true}}, SHIFT(164),
  [500] = {.entry = {.count = 1, .reusable = true}}, SHIFT(189),
  [502] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [504] = {.entry = {.count = 1, .reusable = true}}, SHIFT(155),
  [506] = {.entry = {.count = 1, .reusable = false}}, SHIFT(164),
  [508] = {.entry = {.count = 1, .reusable = false}}, SHIFT(33),
  [510] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [512] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [514] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_after_body_repeat1, 2, 0, 0),
  [516] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_after_body_repeat1, 2, 0, 0), SHIFT_REPEAT(229),
  [519] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [521] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [523] = {.entry = {.count = 1, .reusable = true}}, SHIFT(130),
  [525] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_content, 1, 0, 0),
  [527] = {.entry = {.count = 1, .reusable = false}}, SHIFT(106),
  [529] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [531] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_single_string_content, 1, 0, 0),
  [533] = {.entry = {.count = 1, .reusable = false}}, SHIFT(108),
  [535] = {.entry = {.count = 1, .reusable = false}}, SHIFT(23),
  [537] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [539] = {.entry = {.count = 1, .reusable = true}}, SHIFT(229),
  [541] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [543] = {.entry = {.count = 1, .reusable = true}}, SHIFT(206),
  [545] = {.entry = {.count = 1, .reusable = true}}, SHIFT(143),
  [547] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 2, 0, 0),
  [549] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_chart_params_repeat1, 2, 0, 0),
  [551] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_chart_params_repeat1, 2, 0, 0), SHIFT_REPEAT(62),
  [554] = {.entry = {.count = 1, .reusable = true}}, SHIFT(157),
  [556] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [558] = {.entry = {.count = 1, .reusable = true}}, SHIFT(215),
  [560] = {.entry = {.count = 1, .reusable = true}}, SHIFT(216),
  [562] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 1, 0, 0),
  [564] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [566] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup_statement, 1, 0, 0),
  [568] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_statement, 1, 0, 0),
  [570] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_string_array_repeat1, 2, 0, 0), SHIFT_REPEAT(131),
  [573] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_string_array_repeat1, 2, 0, 0),
  [575] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_anvil_call, 5, 0, 0),
  [577] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_call, 5, 0, 0),
  [579] = {.entry = {.count = 1, .reusable = true}}, SHIFT(241),
  [581] = {.entry = {.count = 1, .reusable = true}}, SHIFT(158),
  [583] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 5, 0, 0),
  [585] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 5, 0, 0),
  [587] = {.entry = {.count = 1, .reusable = true}}, SHIFT(137),
  [589] = {.entry = {.count = 1, .reusable = true}}, SHIFT(212),
  [591] = {.entry = {.count = 1, .reusable = true}}, SHIFT(217),
  [593] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fixture_params_repeat1, 2, 0, 0),
  [595] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_params_repeat1, 2, 0, 0), SHIFT_REPEAT(177),
  [598] = {.entry = {.count = 1, .reusable = true}}, SHIFT(219),
  [600] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite_body, 3, 0, 0),
  [602] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite, 3, 0, 1),
  [604] = {.entry = {.count = 1, .reusable = true}}, SHIFT(110),
  [606] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [608] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_anvil_call, 6, 0, 0),
  [610] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_call, 6, 0, 0),
  [612] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 6, 0, 0),
  [614] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 6, 0, 0),
  [616] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite, 4, 0, 3),
  [618] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite_body, 2, 0, 0),
  [620] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite, 9, 0, 7),
  [622] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite, 8, 0, 6),
  [624] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 3, 0, 0),
  [626] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 3, 0, 0),
  [628] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_argument_list_repeat1, 2, 0, 0),
  [630] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_argument_list_repeat1, 2, 0, 0), SHIFT_REPEAT(185),
  [633] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 3, 0, 0),
  [635] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 1, 0, 0),
  [637] = {.entry = {.count = 1, .reusable = true}}, SHIFT(126),
  [639] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 4, 0, 0),
  [641] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 4, 0, 0),
  [643] = {.entry = {.count = 1, .reusable = true}}, SHIFT(113),
  [645] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [647] = {.entry = {.count = 1, .reusable = true}}, SHIFT(199),
  [649] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_language_tag, 1, 0, 0),
  [651] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [653] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_param, 3, 0, 5),
  [655] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_directive, 6, 0, 10),
  [657] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_directive, 5, 0, 10),
  [659] = {.entry = {.count = 1, .reusable = true}}, SHIFT(203),
  [661] = {.entry = {.count = 1, .reusable = true}}, SHIFT(111),
  [663] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite_type, 1, 0, 0),
  [665] = {.entry = {.count = 1, .reusable = true}}, SHIFT(122),
  [667] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [669] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_param, 3, 0, 9),
  [671] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [673] = {.entry = {.count = 1, .reusable = true}}, SHIFT(201),
  [675] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [677] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument, 3, 0, 5),
  [679] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [681] = {.entry = {.count = 1, .reusable = true}}, SHIFT(169),
  [683] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_function_name, 1, 0, 0),
  [685] = {.entry = {.count = 1, .reusable = true}}, SHIFT(117),
  [687] = {.entry = {.count = 1, .reusable = true}}, SHIFT(163),
  [689] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_args, 3, 0, 0),
  [691] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [693] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property_name, 1, 0, 0),
  [695] = {.entry = {.count = 1, .reusable = true}}, SHIFT(77),
  [697] = {.entry = {.count = 1, .reusable = true}}, SHIFT(134),
  [699] = {.entry = {.count = 1, .reusable = true}}, SHIFT(234),
  [701] = {.entry = {.count = 1, .reusable = true}}, SHIFT(224),
  [703] = {.entry = {.count = 1, .reusable = true}}, SHIFT(150),
  [705] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 4, 0, 0),
  [707] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [709] = {.entry = {.count = 1, .reusable = true}}, SHIFT(133),
  [711] = {.entry = {.count = 1, .reusable = true}}, SHIFT(149),
  [713] = {.entry = {.count = 1, .reusable = true}}, SHIFT(235),
  [715] = {.entry = {.count = 1, .reusable = true}}, SHIFT(148),
  [717] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [719] = {.entry = {.count = 1, .reusable = true}}, SHIFT(173),
  [721] = {.entry = {.count = 1, .reusable = true}}, SHIFT(242),
  [723] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 2, 0, 0),
  [725] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_run_mode, 1, 0, 0),
  [727] = {.entry = {.count = 1, .reusable = true}}, SHIFT(121),
  [729] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 5, 0, 0),
  [731] = {.entry = {.count = 1, .reusable = true}}, SHIFT(209),
  [733] = {.entry = {.count = 1, .reusable = true}}, SHIFT(204),
  [735] = {.entry = {.count = 1, .reusable = true}}, SHIFT(190),
  [737] = {.entry = {.count = 1, .reusable = true}}, SHIFT(102),
  [739] = {.entry = {.count = 1, .reusable = true}}, SHIFT(140),
  [741] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_param_name, 1, 0, 0),
  [743] = {.entry = {.count = 1, .reusable = true}}, SHIFT(174),
  [745] = {.entry = {.count = 1, .reusable = true}}, SHIFT(161),
  [747] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [749] = {.entry = {.count = 1, .reusable = true}}, SHIFT(97),
  [751] = {.entry = {.count = 1, .reusable = true}}, SHIFT(119),
  [753] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [755] = {.entry = {.count = 1, .reusable = true}}, SHIFT(112),
  [757] = {.entry = {.count = 1, .reusable = true}}, SHIFT(116),
  [759] = {.entry = {.count = 1, .reusable = true}}, SHIFT(107),
  [761] = {.entry = {.count = 1, .reusable = true}}, SHIFT(125),
  [763] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [765] = {.entry = {.count = 1, .reusable = true}}, SHIFT(93),
  [767] = {.entry = {.count = 1, .reusable = true}}, SHIFT(98),
  [769] = {.entry = {.count = 1, .reusable = true}}, SHIFT(100),
  [771] = {.entry = {.count = 1, .reusable = true}}, SHIFT(223),
  [773] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 3, 0, 0),
  [775] = {.entry = {.count = 1, .reusable = true}}, SHIFT(188),
  [777] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [779] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [781] = {.entry = {.count = 1, .reusable = true}}, SHIFT(238),
  [783] = {.entry = {.count = 1, .reusable = true}}, SHIFT(240),
  [785] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [787] = {.entry = {.count = 1, .reusable = true}}, SHIFT(160),
  [789] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [791] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
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
