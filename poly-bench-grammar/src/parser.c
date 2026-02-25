#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 251
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 192
#define ALIAS_COUNT 0
#define TOKEN_COUNT 109
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
  anon_sym_go = 86,
  anon_sym_ts = 87,
  anon_sym_typescript = 88,
  anon_sym_rust = 89,
  anon_sym_python = 90,
  sym_inline_code = 91,
  anon_sym_DQUOTE = 92,
  anon_sym_SQUOTE = 93,
  aux_sym_string_content_token1 = 94,
  aux_sym_single_string_content_token1 = 95,
  sym_escape_sequence = 96,
  sym_number = 97,
  sym_float = 98,
  anon_sym_ms = 99,
  anon_sym_s = 100,
  anon_sym_m = 101,
  anon_sym_true = 102,
  anon_sym_false = 103,
  anon_sym_LBRACK = 104,
  anon_sym_RBRACK = 105,
  sym_comment = 106,
  sym_embedded_code = 107,
  sym__embedded_code_start = 108,
  sym_source_file = 109,
  sym_use_statement = 110,
  sym_global_setup = 111,
  sym_global_setup_body = 112,
  sym_global_setup_statement = 113,
  sym_anvil_call = 114,
  sym_anvil_args = 115,
  sym_function_call = 116,
  sym_argument_list = 117,
  sym_argument = 118,
  sym_suite = 119,
  sym_suite_type = 120,
  sym_run_mode = 121,
  sym_suite_body = 122,
  sym__suite_item = 123,
  sym_setup_block = 124,
  sym_setup_body = 125,
  sym__setup_section = 126,
  sym_import_section = 127,
  sym_declare_section = 128,
  sym_init_section = 129,
  sym_helpers_section = 130,
  sym_fixture = 131,
  sym_fixture_params = 132,
  sym_fixture_param = 133,
  sym_fixture_body = 134,
  sym__fixture_item = 135,
  sym_hex_property = 136,
  sym_data_property = 137,
  sym_encoding_property = 138,
  sym_format_property = 139,
  sym_selector_property = 140,
  sym_shape_property = 141,
  sym_file_ref = 142,
  sym_benchmark = 143,
  sym_benchmark_body = 144,
  sym__benchmark_item = 145,
  sym_tags_property = 146,
  sym_skip_hook = 147,
  sym_validate_hook = 148,
  sym_before_hook = 149,
  sym_after_hook = 150,
  sym_each_hook = 151,
  sym_hook_flat = 152,
  sym_hook_grouped = 153,
  sym_after_block = 154,
  sym_after_body = 155,
  sym_chart_directive = 156,
  sym_chart_function_name = 157,
  sym_chart_params = 158,
  sym_chart_param = 159,
  sym_chart_param_name = 160,
  sym__chart_value = 161,
  sym_property = 162,
  sym_property_name = 163,
  sym__value = 164,
  sym_language_implementation = 165,
  sym_language_tag = 166,
  sym__code_or_inline = 167,
  sym_code_block = 168,
  sym_paren_code_block = 169,
  sym_string = 170,
  sym_string_content = 171,
  sym_single_string_content = 172,
  sym_duration = 173,
  sym_duration_unit = 174,
  sym_boolean = 175,
  sym_string_array = 176,
  aux_sym_source_file_repeat1 = 177,
  aux_sym_source_file_repeat2 = 178,
  aux_sym_global_setup_body_repeat1 = 179,
  aux_sym_argument_list_repeat1 = 180,
  aux_sym_suite_body_repeat1 = 181,
  aux_sym_setup_body_repeat1 = 182,
  aux_sym_fixture_params_repeat1 = 183,
  aux_sym_fixture_body_repeat1 = 184,
  aux_sym_benchmark_body_repeat1 = 185,
  aux_sym_hook_grouped_repeat1 = 186,
  aux_sym_after_body_repeat1 = 187,
  aux_sym_chart_params_repeat1 = 188,
  aux_sym_string_content_repeat1 = 189,
  aux_sym_single_string_content_repeat1 = 190,
  aux_sym_string_array_repeat1 = 191,
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
  [22] = 21,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
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
  [42] = 21,
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
  [64] = 63,
  [65] = 65,
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
  [81] = 81,
  [82] = 82,
  [83] = 83,
  [84] = 84,
  [85] = 85,
  [86] = 86,
  [87] = 21,
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
  [249] = 234,
  [250] = 234,
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
      if (lookahead == 'o') ADVANCE(39);
      END_STATE();
    case 8:
      if (lookahead == 'e') ADVANCE(40);
      END_STATE();
    case 9:
      if (lookahead == 'm') ADVANCE(41);
      if (lookahead == 'n') ADVANCE(42);
      if (lookahead == 't') ADVANCE(43);
      END_STATE();
    case 10:
      if (lookahead == 'i') ADVANCE(44);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_m);
      if (lookahead == 'e') ADVANCE(45);
      if (lookahead == 'i') ADVANCE(46);
      if (lookahead == 'o') ADVANCE(47);
      if (lookahead == 's') ADVANCE(48);
      END_STATE();
    case 12:
      if (lookahead == 'r') ADVANCE(49);
      if (lookahead == 'u') ADVANCE(50);
      END_STATE();
    case 13:
      if (lookahead == 'e') ADVANCE(51);
      if (lookahead == 'y') ADVANCE(52);
      END_STATE();
    case 14:
      if (lookahead == 'e') ADVANCE(53);
      if (lookahead == 'o') ADVANCE(54);
      if (lookahead == 'u') ADVANCE(55);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_s);
      ADVANCE_MAP(
        'a', 56,
        'e', 57,
        'h', 58,
        'i', 59,
        'k', 60,
        'o', 61,
        'p', 62,
        't', 63,
        'u', 64,
      );
      END_STATE();
    case 16:
      if (lookahead == 'a') ADVANCE(65);
      if (lookahead == 'h') ADVANCE(66);
      if (lookahead == 'i') ADVANCE(67);
      if (lookahead == 'r') ADVANCE(68);
      if (lookahead == 's') ADVANCE(69);
      if (lookahead == 'y') ADVANCE(70);
      END_STATE();
    case 17:
      if (lookahead == 's') ADVANCE(71);
      END_STATE();
    case 18:
      if (lookahead == 'a') ADVANCE(72);
      END_STATE();
    case 19:
      if (lookahead == 'a') ADVANCE(73);
      if (lookahead == 'i') ADVANCE(74);
      END_STATE();
    case 20:
      if (lookahead == 'S') ADVANCE(75);
      END_STATE();
    case 21:
      if (lookahead == 't') ADVANCE(76);
      END_STATE();
    case 22:
      if (lookahead == 'v') ADVANCE(77);
      END_STATE();
    case 23:
      if (lookahead == 'y') ADVANCE(78);
      END_STATE();
    case 24:
      if (lookahead == 's') ADVANCE(79);
      END_STATE();
    case 25:
      if (lookahead == 'f') ADVANCE(80);
      if (lookahead == 'n') ADVANCE(81);
      END_STATE();
    case 26:
      if (lookahead == 'a') ADVANCE(82);
      END_STATE();
    case 27:
      if (lookahead == 'u') ADVANCE(83);
      END_STATE();
    case 28:
      if (lookahead == 'T') ADVANCE(84);
      END_STATE();
    case 29:
      if (lookahead == 't') ADVANCE(85);
      END_STATE();
    case 30:
      if (lookahead == 'c') ADVANCE(86);
      if (lookahead == 's') ADVANCE(87);
      END_STATE();
    case 31:
      if (lookahead == 'a') ADVANCE(88);
      END_STATE();
    case 32:
      if (lookahead == 'c') ADVANCE(89);
      END_STATE();
    case 33:
      if (lookahead == 'c') ADVANCE(90);
      END_STATE();
    case 34:
      if (lookahead == 'c') ADVANCE(91);
      END_STATE();
    case 35:
      if (lookahead == 'i') ADVANCE(92);
      if (lookahead == 'l') ADVANCE(93);
      END_STATE();
    case 36:
      if (lookahead == 'l') ADVANCE(94);
      if (lookahead == 'x') ADVANCE(95);
      END_STATE();
    case 37:
      if (lookahead == 'r') ADVANCE(96);
      END_STATE();
    case 38:
      if (lookahead == 'o') ADVANCE(97);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(anon_sym_go);
      END_STATE();
    case 40:
      if (lookahead == 'i') ADVANCE(98);
      if (lookahead == 'l') ADVANCE(99);
      if (lookahead == 'x') ADVANCE(100);
      END_STATE();
    case 41:
      if (lookahead == 'p') ADVANCE(101);
      END_STATE();
    case 42:
      if (lookahead == 'c') ADVANCE(102);
      if (lookahead == 'i') ADVANCE(103);
      END_STATE();
    case 43:
      if (lookahead == 'e') ADVANCE(104);
      END_STATE();
    case 44:
      if (lookahead == 'm') ADVANCE(105);
      END_STATE();
    case 45:
      if (lookahead == 'm') ADVANCE(106);
      END_STATE();
    case 46:
      if (lookahead == 'n') ADVANCE(107);
      END_STATE();
    case 47:
      if (lookahead == 'd') ADVANCE(108);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(anon_sym_ms);
      END_STATE();
    case 49:
      if (lookahead == 'd') ADVANCE(109);
      END_STATE();
    case 50:
      if (lookahead == 't') ADVANCE(110);
      END_STATE();
    case 51:
      if (lookahead == 'r') ADVANCE(111);
      END_STATE();
    case 52:
      if (lookahead == 't') ADVANCE(112);
      END_STATE();
    case 53:
      if (lookahead == 'g') ADVANCE(113);
      if (lookahead == 'q') ADVANCE(114);
      END_STATE();
    case 54:
      if (lookahead == 'w') ADVANCE(115);
      END_STATE();
    case 55:
      if (lookahead == 's') ADVANCE(116);
      END_STATE();
    case 56:
      if (lookahead == 'm') ADVANCE(117);
      END_STATE();
    case 57:
      if (lookahead == 'l') ADVANCE(118);
      if (lookahead == 't') ADVANCE(119);
      END_STATE();
    case 58:
      if (lookahead == 'a') ADVANCE(120);
      if (lookahead == 'o') ADVANCE(121);
      END_STATE();
    case 59:
      if (lookahead == 'n') ADVANCE(122);
      END_STATE();
    case 60:
      if (lookahead == 'i') ADVANCE(123);
      END_STATE();
    case 61:
      if (lookahead == 'r') ADVANCE(124);
      END_STATE();
    case 62:
      if (lookahead == 'a') ADVANCE(125);
      END_STATE();
    case 63:
      if (lookahead == 'd') ADVANCE(126);
      END_STATE();
    case 64:
      if (lookahead == 'i') ADVANCE(127);
      END_STATE();
    case 65:
      if (lookahead == 'g') ADVANCE(128);
      if (lookahead == 'r') ADVANCE(129);
      END_STATE();
    case 66:
      if (lookahead == 'e') ADVANCE(130);
      END_STATE();
    case 67:
      if (lookahead == 'm') ADVANCE(131);
      if (lookahead == 't') ADVANCE(132);
      END_STATE();
    case 68:
      if (lookahead == 'u') ADVANCE(133);
      END_STATE();
    case 69:
      ACCEPT_TOKEN(anon_sym_ts);
      END_STATE();
    case 70:
      if (lookahead == 'p') ADVANCE(134);
      END_STATE();
    case 71:
      if (lookahead == 'e') ADVANCE(135);
      END_STATE();
    case 72:
      if (lookahead == 'l') ADVANCE(136);
      END_STATE();
    case 73:
      if (lookahead == 'r') ADVANCE(137);
      END_STATE();
    case 74:
      if (lookahead == 'd') ADVANCE(138);
      END_STATE();
    case 75:
      if (lookahead == 'c') ADVANCE(139);
      END_STATE();
    case 76:
      if (lookahead == 'e') ADVANCE(140);
      END_STATE();
    case 77:
      if (lookahead == 'i') ADVANCE(141);
      END_STATE();
    case 78:
      if (lookahead == 'n') ADVANCE(142);
      END_STATE();
    case 79:
      if (lookahead == 'e') ADVANCE(143);
      END_STATE();
    case 80:
      if (lookahead == 'o') ADVANCE(144);
      END_STATE();
    case 81:
      if (lookahead == 'c') ADVANCE(145);
      END_STATE();
    case 82:
      if (lookahead == 'r') ADVANCE(146);
      END_STATE();
    case 83:
      if (lookahead == 'n') ADVANCE(147);
      END_STATE();
    case 84:
      if (lookahead == 'h') ADVANCE(148);
      END_STATE();
    case 85:
      if (lookahead == 'a') ADVANCE(149);
      END_STATE();
    case 86:
      if (lookahead == 'l') ADVANCE(150);
      END_STATE();
    case 87:
      if (lookahead == 'c') ADVANCE(151);
      END_STATE();
    case 88:
      if (lookahead == 'w') ADVANCE(152);
      END_STATE();
    case 89:
      if (lookahead == 'h') ADVANCE(153);
      END_STATE();
    case 90:
      if (lookahead == 'o') ADVANCE(154);
      END_STATE();
    case 91:
      if (lookahead == 'l') ADVANCE(155);
      END_STATE();
    case 92:
      if (lookahead == 'r') ADVANCE(156);
      END_STATE();
    case 93:
      if (lookahead == 's') ADVANCE(157);
      END_STATE();
    case 94:
      if (lookahead == 't') ADVANCE(158);
      END_STATE();
    case 95:
      if (lookahead == 't') ADVANCE(159);
      END_STATE();
    case 96:
      if (lookahead == 'k') ADVANCE(160);
      if (lookahead == 'm') ADVANCE(161);
      END_STATE();
    case 97:
      if (lookahead == 'b') ADVANCE(162);
      END_STATE();
    case 98:
      if (lookahead == 'g') ADVANCE(163);
      END_STATE();
    case 99:
      if (lookahead == 'p') ADVANCE(164);
      END_STATE();
    case 100:
      ACCEPT_TOKEN(anon_sym_hex);
      END_STATE();
    case 101:
      if (lookahead == 'o') ADVANCE(165);
      END_STATE();
    case 102:
      if (lookahead == 'l') ADVANCE(166);
      END_STATE();
    case 103:
      if (lookahead == 't') ADVANCE(167);
      END_STATE();
    case 104:
      if (lookahead == 'r') ADVANCE(168);
      END_STATE();
    case 105:
      if (lookahead == 'i') ADVANCE(169);
      END_STATE();
    case 106:
      if (lookahead == 'o') ADVANCE(170);
      END_STATE();
    case 107:
      if (lookahead == 'S') ADVANCE(171);
      END_STATE();
    case 108:
      if (lookahead == 'e') ADVANCE(172);
      END_STATE();
    case 109:
      if (lookahead == 'e') ADVANCE(173);
      END_STATE();
    case 110:
      if (lookahead == 'l') ADVANCE(174);
      if (lookahead == 'p') ADVANCE(175);
      END_STATE();
    case 111:
      if (lookahead == 'f') ADVANCE(176);
      END_STATE();
    case 112:
      if (lookahead == 'h') ADVANCE(177);
      END_STATE();
    case 113:
      if (lookahead == 'r') ADVANCE(178);
      END_STATE();
    case 114:
      if (lookahead == 'u') ADVANCE(179);
      END_STATE();
    case 115:
      if (lookahead == 'C') ADVANCE(180);
      END_STATE();
    case 116:
      if (lookahead == 't') ADVANCE(181);
      END_STATE();
    case 117:
      if (lookahead == 'e') ADVANCE(182);
      END_STATE();
    case 118:
      if (lookahead == 'e') ADVANCE(183);
      END_STATE();
    case 119:
      if (lookahead == 'u') ADVANCE(184);
      END_STATE();
    case 120:
      if (lookahead == 'p') ADVANCE(185);
      END_STATE();
    case 121:
      if (lookahead == 'w') ADVANCE(186);
      END_STATE();
    case 122:
      if (lookahead == 'k') ADVANCE(187);
      END_STATE();
    case 123:
      if (lookahead == 'p') ADVANCE(188);
      END_STATE();
    case 124:
      if (lookahead == 't') ADVANCE(189);
      END_STATE();
    case 125:
      if (lookahead == 'w') ADVANCE(190);
      END_STATE();
    case 126:
      ACCEPT_TOKEN(anon_sym_std);
      END_STATE();
    case 127:
      if (lookahead == 't') ADVANCE(191);
      END_STATE();
    case 128:
      if (lookahead == 's') ADVANCE(192);
      END_STATE();
    case 129:
      if (lookahead == 'g') ADVANCE(193);
      END_STATE();
    case 130:
      if (lookahead == 'm') ADVANCE(194);
      END_STATE();
    case 131:
      if (lookahead == 'e') ADVANCE(195);
      END_STATE();
    case 132:
      if (lookahead == 'l') ADVANCE(196);
      END_STATE();
    case 133:
      if (lookahead == 'e') ADVANCE(197);
      END_STATE();
    case 134:
      if (lookahead == 'e') ADVANCE(198);
      END_STATE();
    case 135:
      ACCEPT_TOKEN(anon_sym_use);
      END_STATE();
    case 136:
      if (lookahead == 'i') ADVANCE(199);
      END_STATE();
    case 137:
      if (lookahead == 'm') ADVANCE(200);
      END_STATE();
    case 138:
      if (lookahead == 't') ADVANCE(201);
      END_STATE();
    case 139:
      if (lookahead == 'a') ADVANCE(202);
      END_STATE();
    case 140:
      if (lookahead == 'r') ADVANCE(203);
      END_STATE();
    case 141:
      if (lookahead == 'l') ADVANCE(204);
      END_STATE();
    case 142:
      if (lookahead == 'c') ADVANCE(205);
      END_STATE();
    case 143:
      if (lookahead == 'l') ADVANCE(206);
      END_STATE();
    case 144:
      if (lookahead == 'r') ADVANCE(207);
      END_STATE();
    case 145:
      if (lookahead == 'h') ADVANCE(208);
      END_STATE();
    case 146:
      if (lookahead == 't') ADVANCE(209);
      END_STATE();
    case 147:
      if (lookahead == 't') ADVANCE(210);
      END_STATE();
    case 148:
      if (lookahead == 'r') ADVANCE(211);
      END_STATE();
    case 149:
      ACCEPT_TOKEN(anon_sym_data);
      END_STATE();
    case 150:
      if (lookahead == 'a') ADVANCE(212);
      END_STATE();
    case 151:
      if (lookahead == 'r') ADVANCE(213);
      END_STATE();
    case 152:
      if (lookahead == 'B') ADVANCE(214);
      if (lookahead == 'L') ADVANCE(215);
      if (lookahead == 'S') ADVANCE(216);
      if (lookahead == 'T') ADVANCE(217);
      END_STATE();
    case 153:
      ACCEPT_TOKEN(anon_sym_each);
      END_STATE();
    case 154:
      if (lookahead == 'd') ADVANCE(218);
      END_STATE();
    case 155:
      if (lookahead == 'u') ADVANCE(219);
      END_STATE();
    case 156:
      if (lookahead == 'n') ADVANCE(220);
      END_STATE();
    case 157:
      if (lookahead == 'e') ADVANCE(221);
      END_STATE();
    case 158:
      if (lookahead == 'e') ADVANCE(222);
      END_STATE();
    case 159:
      if (lookahead == 'u') ADVANCE(223);
      END_STATE();
    case 160:
      ACCEPT_TOKEN(anon_sym_fork);
      END_STATE();
    case 161:
      if (lookahead == 'a') ADVANCE(224);
      END_STATE();
    case 162:
      if (lookahead == 'a') ADVANCE(225);
      END_STATE();
    case 163:
      if (lookahead == 'h') ADVANCE(226);
      END_STATE();
    case 164:
      if (lookahead == 'e') ADVANCE(227);
      END_STATE();
    case 165:
      if (lookahead == 'r') ADVANCE(228);
      END_STATE();
    case 166:
      if (lookahead == 'u') ADVANCE(229);
      END_STATE();
    case 167:
      ACCEPT_TOKEN(anon_sym_init);
      END_STATE();
    case 168:
      if (lookahead == 'a') ADVANCE(230);
      END_STATE();
    case 169:
      if (lookahead == 't') ADVANCE(231);
      END_STATE();
    case 170:
      if (lookahead == 'r') ADVANCE(232);
      END_STATE();
    case 171:
      if (lookahead == 'p') ADVANCE(233);
      END_STATE();
    case 172:
      ACCEPT_TOKEN(anon_sym_mode);
      END_STATE();
    case 173:
      if (lookahead == 'r') ADVANCE(234);
      END_STATE();
    case 174:
      if (lookahead == 'i') ADVANCE(235);
      END_STATE();
    case 175:
      if (lookahead == 'u') ADVANCE(236);
      END_STATE();
    case 176:
      if (lookahead == 'o') ADVANCE(237);
      END_STATE();
    case 177:
      if (lookahead == 'o') ADVANCE(238);
      END_STATE();
    case 178:
      if (lookahead == 'e') ADVANCE(239);
      END_STATE();
    case 179:
      if (lookahead == 'i') ADVANCE(240);
      END_STATE();
    case 180:
      if (lookahead == 'o') ADVANCE(241);
      END_STATE();
    case 181:
      ACCEPT_TOKEN(anon_sym_rust);
      END_STATE();
    case 182:
      if (lookahead == 'D') ADVANCE(242);
      END_STATE();
    case 183:
      if (lookahead == 'c') ADVANCE(243);
      END_STATE();
    case 184:
      if (lookahead == 'p') ADVANCE(244);
      END_STATE();
    case 185:
      if (lookahead == 'e') ADVANCE(245);
      END_STATE();
    case 186:
      if (lookahead == 'E') ADVANCE(246);
      if (lookahead == 'R') ADVANCE(247);
      if (lookahead == 'S') ADVANCE(248);
      END_STATE();
    case 187:
      ACCEPT_TOKEN(anon_sym_sink);
      END_STATE();
    case 188:
      ACCEPT_TOKEN(anon_sym_skip);
      END_STATE();
    case 189:
      if (lookahead == 'B') ADVANCE(249);
      if (lookahead == 'O') ADVANCE(250);
      END_STATE();
    case 190:
      if (lookahead == 'n') ADVANCE(251);
      END_STATE();
    case 191:
      if (lookahead == 'e') ADVANCE(252);
      END_STATE();
    case 192:
      ACCEPT_TOKEN(anon_sym_tags);
      END_STATE();
    case 193:
      if (lookahead == 'e') ADVANCE(253);
      END_STATE();
    case 194:
      if (lookahead == 'e') ADVANCE(254);
      END_STATE();
    case 195:
      if (lookahead == 'B') ADVANCE(255);
      if (lookahead == 'o') ADVANCE(256);
      END_STATE();
    case 196:
      if (lookahead == 'e') ADVANCE(257);
      END_STATE();
    case 197:
      ACCEPT_TOKEN(anon_sym_true);
      END_STATE();
    case 198:
      if (lookahead == 's') ADVANCE(258);
      END_STATE();
    case 199:
      if (lookahead == 'd') ADVANCE(259);
      END_STATE();
    case 200:
      if (lookahead == 'u') ADVANCE(260);
      END_STATE();
    case 201:
      if (lookahead == 'h') ADVANCE(261);
      END_STATE();
    case 202:
      if (lookahead == 'l') ADVANCE(262);
      END_STATE();
    case 203:
      ACCEPT_TOKEN(anon_sym_after);
      END_STATE();
    case 204:
      ACCEPT_TOKEN(anon_sym_anvil);
      END_STATE();
    case 205:
      ACCEPT_TOKEN(anon_sym_async);
      if (lookahead == 'S') ADVANCE(263);
      if (lookahead == 'W') ADVANCE(264);
      END_STATE();
    case 206:
      if (lookahead == 'i') ADVANCE(265);
      END_STATE();
    case 207:
      if (lookahead == 'e') ADVANCE(266);
      END_STATE();
    case 208:
      ACCEPT_TOKEN(anon_sym_bench);
      if (lookahead == 'A') ADVANCE(267);
      END_STATE();
    case 209:
      if (lookahead == 'i') ADVANCE(268);
      END_STATE();
    case 210:
      ACCEPT_TOKEN(anon_sym_count);
      END_STATE();
    case 211:
      if (lookahead == 'e') ADVANCE(269);
      END_STATE();
    case 212:
      if (lookahead == 'r') ADVANCE(270);
      END_STATE();
    case 213:
      if (lookahead == 'i') ADVANCE(271);
      END_STATE();
    case 214:
      if (lookahead == 'a') ADVANCE(272);
      END_STATE();
    case 215:
      if (lookahead == 'i') ADVANCE(273);
      END_STATE();
    case 216:
      if (lookahead == 'p') ADVANCE(274);
      END_STATE();
    case 217:
      if (lookahead == 'a') ADVANCE(275);
      END_STATE();
    case 218:
      if (lookahead == 'i') ADVANCE(276);
      END_STATE();
    case 219:
      if (lookahead == 'd') ADVANCE(277);
      END_STATE();
    case 220:
      if (lookahead == 'e') ADVANCE(278);
      END_STATE();
    case 221:
      ACCEPT_TOKEN(anon_sym_false);
      END_STATE();
    case 222:
      if (lookahead == 'r') ADVANCE(279);
      END_STATE();
    case 223:
      if (lookahead == 'r') ADVANCE(280);
      END_STATE();
    case 224:
      if (lookahead == 't') ADVANCE(281);
      END_STATE();
    case 225:
      if (lookahead == 'l') ADVANCE(282);
      END_STATE();
    case 226:
      if (lookahead == 't') ADVANCE(283);
      END_STATE();
    case 227:
      if (lookahead == 'r') ADVANCE(284);
      END_STATE();
    case 228:
      if (lookahead == 't') ADVANCE(285);
      END_STATE();
    case 229:
      if (lookahead == 'd') ADVANCE(286);
      END_STATE();
    case 230:
      if (lookahead == 't') ADVANCE(287);
      END_STATE();
    case 231:
      ACCEPT_TOKEN(anon_sym_limit);
      END_STATE();
    case 232:
      if (lookahead == 'y') ADVANCE(288);
      END_STATE();
    case 233:
      if (lookahead == 'e') ADVANCE(289);
      END_STATE();
    case 234:
      ACCEPT_TOKEN(anon_sym_order);
      END_STATE();
    case 235:
      if (lookahead == 'e') ADVANCE(290);
      END_STATE();
    case 236:
      if (lookahead == 't') ADVANCE(291);
      END_STATE();
    case 237:
      if (lookahead == 'r') ADVANCE(292);
      END_STATE();
    case 238:
      if (lookahead == 'n') ADVANCE(293);
      END_STATE();
    case 239:
      if (lookahead == 's') ADVANCE(294);
      END_STATE();
    case 240:
      if (lookahead == 'r') ADVANCE(295);
      END_STATE();
    case 241:
      if (lookahead == 'u') ADVANCE(296);
      END_STATE();
    case 242:
      if (lookahead == 'a') ADVANCE(297);
      END_STATE();
    case 243:
      if (lookahead == 't') ADVANCE(298);
      END_STATE();
    case 244:
      ACCEPT_TOKEN(anon_sym_setup);
      END_STATE();
    case 245:
      ACCEPT_TOKEN(anon_sym_shape);
      END_STATE();
    case 246:
      if (lookahead == 'r') ADVANCE(299);
      END_STATE();
    case 247:
      if (lookahead == 'e') ADVANCE(300);
      END_STATE();
    case 248:
      if (lookahead == 't') ADVANCE(301);
      END_STATE();
    case 249:
      if (lookahead == 'y') ADVANCE(302);
      END_STATE();
    case 250:
      if (lookahead == 'r') ADVANCE(303);
      END_STATE();
    case 251:
      if (lookahead == 'A') ADVANCE(304);
      END_STATE();
    case 252:
      ACCEPT_TOKEN(anon_sym_suite);
      END_STATE();
    case 253:
      if (lookahead == 't') ADVANCE(305);
      END_STATE();
    case 254:
      ACCEPT_TOKEN(anon_sym_theme);
      END_STATE();
    case 255:
      if (lookahead == 'a') ADVANCE(306);
      END_STATE();
    case 256:
      if (lookahead == 'u') ADVANCE(307);
      END_STATE();
    case 257:
      ACCEPT_TOKEN(anon_sym_title);
      END_STATE();
    case 258:
      if (lookahead == 'c') ADVANCE(308);
      END_STATE();
    case 259:
      if (lookahead == 'a') ADVANCE(309);
      END_STATE();
    case 260:
      if (lookahead == 'p') ADVANCE(310);
      END_STATE();
    case 261:
      ACCEPT_TOKEN(anon_sym_width);
      END_STATE();
    case 262:
      if (lookahead == 'e') ADVANCE(311);
      END_STATE();
    case 263:
      if (lookahead == 'a') ADVANCE(312);
      END_STATE();
    case 264:
      if (lookahead == 'a') ADVANCE(313);
      END_STATE();
    case 265:
      if (lookahead == 'n') ADVANCE(314);
      END_STATE();
    case 266:
      ACCEPT_TOKEN(anon_sym_before);
      END_STATE();
    case 267:
      if (lookahead == 's') ADVANCE(315);
      END_STATE();
    case 268:
      if (lookahead == 'n') ADVANCE(316);
      END_STATE();
    case 269:
      if (lookahead == 's') ADVANCE(317);
      END_STATE();
    case 270:
      if (lookahead == 'e') ADVANCE(318);
      END_STATE();
    case 271:
      if (lookahead == 'p') ADVANCE(319);
      END_STATE();
    case 272:
      if (lookahead == 'r') ADVANCE(320);
      END_STATE();
    case 273:
      if (lookahead == 'n') ADVANCE(321);
      END_STATE();
    case 274:
      if (lookahead == 'e') ADVANCE(322);
      END_STATE();
    case 275:
      if (lookahead == 'b') ADVANCE(323);
      END_STATE();
    case 276:
      if (lookahead == 'n') ADVANCE(324);
      END_STATE();
    case 277:
      if (lookahead == 'e') ADVANCE(325);
      END_STATE();
    case 278:
      if (lookahead == 's') ADVANCE(326);
      END_STATE();
    case 279:
      if (lookahead == 'W') ADVANCE(327);
      END_STATE();
    case 280:
      if (lookahead == 'e') ADVANCE(328);
      END_STATE();
    case 281:
      ACCEPT_TOKEN(anon_sym_format);
      END_STATE();
    case 282:
      if (lookahead == 'S') ADVANCE(329);
      END_STATE();
    case 283:
      ACCEPT_TOKEN(anon_sym_height);
      END_STATE();
    case 284:
      if (lookahead == 's') ADVANCE(330);
      END_STATE();
    case 285:
      ACCEPT_TOKEN(anon_sym_import);
      END_STATE();
    case 286:
      if (lookahead == 'e') ADVANCE(331);
      END_STATE();
    case 287:
      if (lookahead == 'i') ADVANCE(332);
      END_STATE();
    case 288:
      ACCEPT_TOKEN(anon_sym_memory);
      END_STATE();
    case 289:
      if (lookahead == 'e') ADVANCE(333);
      END_STATE();
    case 290:
      if (lookahead == 'r') ADVANCE(334);
      END_STATE();
    case 291:
      ACCEPT_TOKEN(anon_sym_output);
      END_STATE();
    case 292:
      if (lookahead == 'm') ADVANCE(335);
      END_STATE();
    case 293:
      ACCEPT_TOKEN(anon_sym_python);
      END_STATE();
    case 294:
      if (lookahead == 's') ADVANCE(336);
      END_STATE();
    case 295:
      if (lookahead == 'e') ADVANCE(337);
      END_STATE();
    case 296:
      if (lookahead == 'n') ADVANCE(338);
      END_STATE();
    case 297:
      if (lookahead == 't') ADVANCE(339);
      END_STATE();
    case 298:
      if (lookahead == 'o') ADVANCE(340);
      END_STATE();
    case 299:
      if (lookahead == 'r') ADVANCE(341);
      END_STATE();
    case 300:
      if (lookahead == 'g') ADVANCE(342);
      END_STATE();
    case 301:
      if (lookahead == 'd') ADVANCE(343);
      END_STATE();
    case 302:
      ACCEPT_TOKEN(anon_sym_sortBy);
      END_STATE();
    case 303:
      if (lookahead == 'd') ADVANCE(344);
      END_STATE();
    case 304:
      if (lookahead == 'n') ADVANCE(345);
      END_STATE();
    case 305:
      if (lookahead == 'T') ADVANCE(346);
      END_STATE();
    case 306:
      if (lookahead == 's') ADVANCE(347);
      END_STATE();
    case 307:
      if (lookahead == 't') ADVANCE(348);
      END_STATE();
    case 308:
      if (lookahead == 'r') ADVANCE(349);
      END_STATE();
    case 309:
      if (lookahead == 't') ADVANCE(350);
      END_STATE();
    case 310:
      ACCEPT_TOKEN(anon_sym_warmup);
      END_STATE();
    case 311:
      ACCEPT_TOKEN(anon_sym_yScale);
      END_STATE();
    case 312:
      if (lookahead == 'm') ADVANCE(351);
      END_STATE();
    case 313:
      if (lookahead == 'r') ADVANCE(352);
      END_STATE();
    case 314:
      if (lookahead == 'e') ADVANCE(353);
      END_STATE();
    case 315:
      if (lookahead == 'y') ADVANCE(354);
      END_STATE();
    case 316:
      if (lookahead == 'g') ADVANCE(355);
      END_STATE();
    case 317:
      if (lookahead == 'h') ADVANCE(356);
      END_STATE();
    case 318:
      ACCEPT_TOKEN(anon_sym_declare);
      END_STATE();
    case 319:
      if (lookahead == 't') ADVANCE(357);
      END_STATE();
    case 320:
      if (lookahead == 'C') ADVANCE(358);
      END_STATE();
    case 321:
      if (lookahead == 'e') ADVANCE(359);
      END_STATE();
    case 322:
      if (lookahead == 'e') ADVANCE(360);
      END_STATE();
    case 323:
      if (lookahead == 'l') ADVANCE(361);
      END_STATE();
    case 324:
      if (lookahead == 'g') ADVANCE(362);
      END_STATE();
    case 325:
      if (lookahead == 'B') ADVANCE(363);
      END_STATE();
    case 326:
      if (lookahead == 's') ADVANCE(364);
      END_STATE();
    case 327:
      if (lookahead == 'i') ADVANCE(365);
      END_STATE();
    case 328:
      ACCEPT_TOKEN(anon_sym_fixture);
      END_STATE();
    case 329:
      if (lookahead == 'e') ADVANCE(366);
      END_STATE();
    case 330:
      ACCEPT_TOKEN(anon_sym_helpers);
      END_STATE();
    case 331:
      if (lookahead == 'B') ADVANCE(367);
      END_STATE();
    case 332:
      if (lookahead == 'o') ADVANCE(368);
      END_STATE();
    case 333:
      if (lookahead == 'd') ADVANCE(369);
      END_STATE();
    case 334:
      if (lookahead == 'D') ADVANCE(370);
      END_STATE();
    case 335:
      if (lookahead == 'a') ADVANCE(371);
      END_STATE();
    case 336:
      if (lookahead == 'i') ADVANCE(372);
      END_STATE();
    case 337:
      if (lookahead == 's') ADVANCE(373);
      END_STATE();
    case 338:
      if (lookahead == 't') ADVANCE(374);
      END_STATE();
    case 339:
      if (lookahead == 'a') ADVANCE(375);
      END_STATE();
    case 340:
      if (lookahead == 'r') ADVANCE(376);
      END_STATE();
    case 341:
      if (lookahead == 'o') ADVANCE(377);
      END_STATE();
    case 342:
      if (lookahead == 'r') ADVANCE(378);
      END_STATE();
    case 343:
      if (lookahead == 'D') ADVANCE(379);
      END_STATE();
    case 344:
      if (lookahead == 'e') ADVANCE(380);
      END_STATE();
    case 345:
      if (lookahead == 'v') ADVANCE(381);
      END_STATE();
    case 346:
      if (lookahead == 'i') ADVANCE(382);
      END_STATE();
    case 347:
      if (lookahead == 'e') ADVANCE(383);
      END_STATE();
    case 348:
      ACCEPT_TOKEN(anon_sym_timeout);
      END_STATE();
    case 349:
      if (lookahead == 'i') ADVANCE(384);
      END_STATE();
    case 350:
      if (lookahead == 'e') ADVANCE(385);
      END_STATE();
    case 351:
      if (lookahead == 'p') ADVANCE(386);
      END_STATE();
    case 352:
      if (lookahead == 'm') ADVANCE(387);
      END_STATE();
    case 353:
      ACCEPT_TOKEN(anon_sym_baseline);
      if (lookahead == 'B') ADVANCE(388);
      END_STATE();
    case 354:
      if (lookahead == 'n') ADVANCE(389);
      END_STATE();
    case 355:
      ACCEPT_TOKEN(anon_sym_charting);
      END_STATE();
    case 356:
      if (lookahead == 'o') ADVANCE(390);
      END_STATE();
    case 357:
      if (lookahead == 'i') ADVANCE(391);
      END_STATE();
    case 358:
      if (lookahead == 'h') ADVANCE(392);
      END_STATE();
    case 359:
      if (lookahead == 'C') ADVANCE(393);
      END_STATE();
    case 360:
      if (lookahead == 'd') ADVANCE(394);
      END_STATE();
    case 361:
      if (lookahead == 'e') ADVANCE(395);
      END_STATE();
    case 362:
      ACCEPT_TOKEN(anon_sym_encoding);
      END_STATE();
    case 363:
      if (lookahead == 'e') ADVANCE(396);
      END_STATE();
    case 364:
      ACCEPT_TOKEN(anon_sym_fairness);
      if (lookahead == 'S') ADVANCE(397);
      END_STATE();
    case 365:
      if (lookahead == 'n') ADVANCE(398);
      END_STATE();
    case 366:
      if (lookahead == 't') ADVANCE(399);
      END_STATE();
    case 367:
      if (lookahead == 'e') ADVANCE(400);
      END_STATE();
    case 368:
      if (lookahead == 'n') ADVANCE(401);
      END_STATE();
    case 369:
      if (lookahead == 'u') ADVANCE(402);
      END_STATE();
    case 370:
      if (lookahead == 'e') ADVANCE(403);
      END_STATE();
    case 371:
      if (lookahead == 'n') ADVANCE(404);
      END_STATE();
    case 372:
      if (lookahead == 'o') ADVANCE(405);
      END_STATE();
    case 373:
      ACCEPT_TOKEN(anon_sym_requires);
      END_STATE();
    case 374:
      ACCEPT_TOKEN(anon_sym_rowCount);
      END_STATE();
    case 375:
      if (lookahead == 's') ADVANCE(406);
      END_STATE();
    case 376:
      ACCEPT_TOKEN(anon_sym_selector);
      END_STATE();
    case 377:
      if (lookahead == 'r') ADVANCE(407);
      END_STATE();
    case 378:
      if (lookahead == 'e') ADVANCE(408);
      END_STATE();
    case 379:
      if (lookahead == 'e') ADVANCE(409);
      END_STATE();
    case 380:
      if (lookahead == 'r') ADVANCE(410);
      END_STATE();
    case 381:
      if (lookahead == 'i') ADVANCE(411);
      END_STATE();
    case 382:
      if (lookahead == 'm') ADVANCE(412);
      END_STATE();
    case 383:
      if (lookahead == 'd') ADVANCE(413);
      END_STATE();
    case 384:
      if (lookahead == 'p') ADVANCE(414);
      END_STATE();
    case 385:
      ACCEPT_TOKEN(anon_sym_validate);
      END_STATE();
    case 386:
      if (lookahead == 'l') ADVANCE(415);
      END_STATE();
    case 387:
      if (lookahead == 'u') ADVANCE(416);
      END_STATE();
    case 388:
      if (lookahead == 'e') ADVANCE(417);
      END_STATE();
    case 389:
      if (lookahead == 'c') ADVANCE(418);
      END_STATE();
    case 390:
      if (lookahead == 'l') ADVANCE(419);
      END_STATE();
    case 391:
      if (lookahead == 'o') ADVANCE(420);
      END_STATE();
    case 392:
      if (lookahead == 'a') ADVANCE(421);
      END_STATE();
    case 393:
      if (lookahead == 'h') ADVANCE(422);
      END_STATE();
    case 394:
      if (lookahead == 'u') ADVANCE(423);
      END_STATE();
    case 395:
      ACCEPT_TOKEN(anon_sym_drawTable);
      END_STATE();
    case 396:
      if (lookahead == 'n') ADVANCE(424);
      END_STATE();
    case 397:
      if (lookahead == 'e') ADVANCE(425);
      END_STATE();
    case 398:
      if (lookahead == 'n') ADVANCE(426);
      END_STATE();
    case 399:
      if (lookahead == 'u') ADVANCE(427);
      END_STATE();
    case 400:
      if (lookahead == 'n') ADVANCE(428);
      END_STATE();
    case 401:
      if (lookahead == 'B') ADVANCE(429);
      if (lookahead == 's') ADVANCE(430);
      END_STATE();
    case 402:
      if (lookahead == 'p') ADVANCE(431);
      END_STATE();
    case 403:
      if (lookahead == 't') ADVANCE(432);
      END_STATE();
    case 404:
      if (lookahead == 'c') ADVANCE(433);
      END_STATE();
    case 405:
      if (lookahead == 'n') ADVANCE(434);
      END_STATE();
    case 406:
      if (lookahead == 'e') ADVANCE(435);
      END_STATE();
    case 407:
      if (lookahead == 'B') ADVANCE(436);
      END_STATE();
    case 408:
      if (lookahead == 's') ADVANCE(437);
      END_STATE();
    case 409:
      if (lookahead == 'v') ADVANCE(438);
      END_STATE();
    case 410:
      ACCEPT_TOKEN(anon_sym_sortOrder);
      END_STATE();
    case 411:
      if (lookahead == 'l') ADVANCE(439);
      END_STATE();
    case 412:
      if (lookahead == 'e') ADVANCE(440);
      END_STATE();
    case 413:
      ACCEPT_TOKEN(anon_sym_timeBased);
      END_STATE();
    case 414:
      if (lookahead == 't') ADVANCE(441);
      END_STATE();
    case 415:
      if (lookahead == 'e') ADVANCE(442);
      if (lookahead == 'i') ADVANCE(443);
      END_STATE();
    case 416:
      if (lookahead == 'p') ADVANCE(444);
      END_STATE();
    case 417:
      if (lookahead == 'n') ADVANCE(445);
      END_STATE();
    case 418:
      ACCEPT_TOKEN(anon_sym_benchAsync);
      END_STATE();
    case 419:
      if (lookahead == 'd') ADVANCE(446);
      END_STATE();
    case 420:
      if (lookahead == 'n') ADVANCE(447);
      END_STATE();
    case 421:
      if (lookahead == 'r') ADVANCE(448);
      END_STATE();
    case 422:
      if (lookahead == 'a') ADVANCE(449);
      END_STATE();
    case 423:
      if (lookahead == 'p') ADVANCE(450);
      END_STATE();
    case 424:
      if (lookahead == 'c') ADVANCE(451);
      END_STATE();
    case 425:
      if (lookahead == 'e') ADVANCE(452);
      END_STATE();
    case 426:
      if (lookahead == 'e') ADVANCE(453);
      END_STATE();
    case 427:
      if (lookahead == 'p') ADVANCE(454);
      END_STATE();
    case 428:
      if (lookahead == 'c') ADVANCE(455);
      END_STATE();
    case 429:
      if (lookahead == 'a') ADVANCE(456);
      END_STATE();
    case 430:
      ACCEPT_TOKEN(anon_sym_iterations);
      END_STATE();
    case 431:
      ACCEPT_TOKEN(anon_sym_minSpeedup);
      END_STATE();
    case 432:
      if (lookahead == 'e') ADVANCE(457);
      END_STATE();
    case 433:
      if (lookahead == 'e') ADVANCE(458);
      END_STATE();
    case 434:
      if (lookahead == 'M') ADVANCE(459);
      END_STATE();
    case 435:
      if (lookahead == 't') ADVANCE(460);
      END_STATE();
    case 436:
      if (lookahead == 'a') ADVANCE(461);
      END_STATE();
    case 437:
      if (lookahead == 's') ADVANCE(462);
      END_STATE();
    case 438:
      ACCEPT_TOKEN(anon_sym_showStdDev);
      END_STATE();
    case 439:
      ACCEPT_TOKEN(anon_sym_spawnAnvil);
      END_STATE();
    case 440:
      ACCEPT_TOKEN(anon_sym_targetTime);
      END_STATE();
    case 441:
      ACCEPT_TOKEN(anon_sym_typescript);
      END_STATE();
    case 442:
      if (lookahead == 'C') ADVANCE(463);
      END_STATE();
    case 443:
      if (lookahead == 'n') ADVANCE(464);
      END_STATE();
    case 444:
      if (lookahead == 'C') ADVANCE(465);
      END_STATE();
    case 445:
      if (lookahead == 'c') ADVANCE(466);
      END_STATE();
    case 446:
      ACCEPT_TOKEN(anon_sym_cvThreshold);
      END_STATE();
    case 447:
      ACCEPT_TOKEN(anon_sym_description);
      END_STATE();
    case 448:
      if (lookahead == 't') ADVANCE(467);
      END_STATE();
    case 449:
      if (lookahead == 'r') ADVANCE(468);
      END_STATE();
    case 450:
      if (lookahead == 'C') ADVANCE(469);
      END_STATE();
    case 451:
      if (lookahead == 'h') ADVANCE(470);
      END_STATE();
    case 452:
      if (lookahead == 'd') ADVANCE(471);
      END_STATE();
    case 453:
      if (lookahead == 'r') ADVANCE(472);
      END_STATE();
    case 454:
      ACCEPT_TOKEN(anon_sym_globalSetup);
      END_STATE();
    case 455:
      if (lookahead == 'h') ADVANCE(473);
      END_STATE();
    case 456:
      if (lookahead == 's') ADVANCE(474);
      END_STATE();
    case 457:
      if (lookahead == 'c') ADVANCE(475);
      END_STATE();
    case 458:
      ACCEPT_TOKEN(anon_sym_performance);
      END_STATE();
    case 459:
      if (lookahead == 'o') ADVANCE(476);
      END_STATE();
    case 460:
      ACCEPT_TOKEN(anon_sym_sameDataset);
      END_STATE();
    case 461:
      if (lookahead == 'r') ADVANCE(477);
      END_STATE();
    case 462:
      if (lookahead == 'i') ADVANCE(478);
      END_STATE();
    case 463:
      if (lookahead == 'a') ADVANCE(479);
      END_STATE();
    case 464:
      if (lookahead == 'g') ADVANCE(480);
      END_STATE();
    case 465:
      if (lookahead == 'a') ADVANCE(481);
      END_STATE();
    case 466:
      if (lookahead == 'h') ADVANCE(482);
      END_STATE();
    case 467:
      ACCEPT_TOKEN(anon_sym_drawBarChart);
      END_STATE();
    case 468:
      if (lookahead == 't') ADVANCE(483);
      END_STATE();
    case 469:
      if (lookahead == 'h') ADVANCE(484);
      END_STATE();
    case 470:
      if (lookahead == 'm') ADVANCE(485);
      END_STATE();
    case 471:
      ACCEPT_TOKEN(anon_sym_fairnessSeed);
      END_STATE();
    case 472:
      ACCEPT_TOKEN(anon_sym_filterWinner);
      END_STATE();
    case 473:
      if (lookahead == 'm') ADVANCE(486);
      END_STATE();
    case 474:
      if (lookahead == 'e') ADVANCE(487);
      END_STATE();
    case 475:
      if (lookahead == 't') ADVANCE(488);
      END_STATE();
    case 476:
      if (lookahead == 'd') ADVANCE(489);
      END_STATE();
    case 477:
      if (lookahead == 's') ADVANCE(490);
      END_STATE();
    case 478:
      if (lookahead == 'o') ADVANCE(491);
      END_STATE();
    case 479:
      if (lookahead == 'p') ADVANCE(492);
      END_STATE();
    case 480:
      if (lookahead == 'P') ADVANCE(493);
      END_STATE();
    case 481:
      if (lookahead == 'p') ADVANCE(494);
      END_STATE();
    case 482:
      if (lookahead == 'm') ADVANCE(495);
      END_STATE();
    case 483:
      ACCEPT_TOKEN(anon_sym_drawLineChart);
      END_STATE();
    case 484:
      if (lookahead == 'a') ADVANCE(496);
      END_STATE();
    case 485:
      if (lookahead == 'a') ADVANCE(497);
      END_STATE();
    case 486:
      if (lookahead == 'a') ADVANCE(498);
      END_STATE();
    case 487:
      if (lookahead == 'd') ADVANCE(499);
      END_STATE();
    case 488:
      if (lookahead == 'i') ADVANCE(500);
      END_STATE();
    case 489:
      if (lookahead == 'e') ADVANCE(501);
      END_STATE();
    case 490:
      ACCEPT_TOKEN(anon_sym_showErrorBars);
      END_STATE();
    case 491:
      if (lookahead == 'n') ADVANCE(502);
      END_STATE();
    case 492:
      ACCEPT_TOKEN(anon_sym_asyncSampleCap);
      END_STATE();
    case 493:
      if (lookahead == 'o') ADVANCE(503);
      END_STATE();
    case 494:
      ACCEPT_TOKEN(anon_sym_asyncWarmupCap);
      END_STATE();
    case 495:
      if (lookahead == 'a') ADVANCE(504);
      END_STATE();
    case 496:
      if (lookahead == 'r') ADVANCE(505);
      END_STATE();
    case 497:
      if (lookahead == 'r') ADVANCE(506);
      END_STATE();
    case 498:
      if (lookahead == 'r') ADVANCE(507);
      END_STATE();
    case 499:
      ACCEPT_TOKEN(anon_sym_iterationBased);
      END_STATE();
    case 500:
      if (lookahead == 'o') ADVANCE(508);
      END_STATE();
    case 501:
      if (lookahead == 'l') ADVANCE(509);
      END_STATE();
    case 502:
      ACCEPT_TOKEN(anon_sym_showRegression);
      END_STATE();
    case 503:
      if (lookahead == 'l') ADVANCE(510);
      END_STATE();
    case 504:
      if (lookahead == 'r') ADVANCE(511);
      END_STATE();
    case 505:
      if (lookahead == 't') ADVANCE(512);
      END_STATE();
    case 506:
      if (lookahead == 'k') ADVANCE(513);
      END_STATE();
    case 507:
      if (lookahead == 'k') ADVANCE(514);
      END_STATE();
    case 508:
      if (lookahead == 'n') ADVANCE(515);
      END_STATE();
    case 509:
      ACCEPT_TOKEN(anon_sym_regressionModel);
      END_STATE();
    case 510:
      if (lookahead == 'i') ADVANCE(516);
      END_STATE();
    case 511:
      if (lookahead == 'k') ADVANCE(517);
      END_STATE();
    case 512:
      ACCEPT_TOKEN(anon_sym_drawSpeedupChart);
      END_STATE();
    case 513:
      if (lookahead == 's') ADVANCE(518);
      END_STATE();
    case 514:
      if (lookahead == 's') ADVANCE(519);
      END_STATE();
    case 515:
      ACCEPT_TOKEN(anon_sym_outlierDetection);
      END_STATE();
    case 516:
      if (lookahead == 'c') ADVANCE(520);
      END_STATE();
    case 517:
      ACCEPT_TOKEN(anon_sym_baselineBenchmark);
      END_STATE();
    case 518:
      ACCEPT_TOKEN(anon_sym_excludeBenchmarks);
      END_STATE();
    case 519:
      ACCEPT_TOKEN(anon_sym_includeBenchmarks);
      END_STATE();
    case 520:
      if (lookahead == 'y') ADVANCE(521);
      END_STATE();
    case 521:
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
  [106] = {.lex_state = 0},
  [107] = {.lex_state = 1},
  [108] = {.lex_state = 0},
  [109] = {.lex_state = 0},
  [110] = {.lex_state = 1},
  [111] = {.lex_state = 3},
  [112] = {.lex_state = 0},
  [113] = {.lex_state = 4},
  [114] = {.lex_state = 4},
  [115] = {.lex_state = 0},
  [116] = {.lex_state = 3},
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
  [161] = {.lex_state = 0},
  [162] = {.lex_state = 0},
  [163] = {.lex_state = 0},
  [164] = {.lex_state = 0},
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
  [178] = {.lex_state = 0, .external_lex_state = 2},
  [179] = {.lex_state = 0},
  [180] = {.lex_state = 0},
  [181] = {.lex_state = 0},
  [182] = {.lex_state = 0, .external_lex_state = 2},
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
  [237] = {.lex_state = 2},
  [238] = {.lex_state = 0},
  [239] = {.lex_state = 0},
  [240] = {.lex_state = 0},
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
    [sym_source_file] = STATE(238),
    [sym_use_statement] = STATE(71),
    [sym_global_setup] = STATE(98),
    [sym_suite] = STATE(105),
    [aux_sym_source_file_repeat1] = STATE(71),
    [aux_sym_source_file_repeat2] = STATE(105),
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
    ACTIONS(17), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(15), 43,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
      anon_sym_RBRACK,
  [53] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(21), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(19), 43,
      anon_sym_globalSetup,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [106] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(25), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(23), 43,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
      anon_sym_RBRACK,
  [159] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(29), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(27), 42,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [211] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(33), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(31), 42,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [263] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(37), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(35), 42,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [315] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(41), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(39), 42,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [367] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(45), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(43), 42,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [419] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(49), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(47), 42,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [471] = 14,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(51), 1,
      anon_sym_RBRACE,
    ACTIONS(55), 1,
      anon_sym_hex,
    ACTIONS(57), 1,
      anon_sym_data,
    ACTIONS(59), 1,
      anon_sym_encoding,
    ACTIONS(61), 1,
      anon_sym_format,
    ACTIONS(63), 1,
      anon_sym_selector,
    ACTIONS(65), 1,
      anon_sym_shape,
    ACTIONS(67), 1,
      anon_sym_fairness,
    STATE(202), 1,
      sym_language_tag,
    STATE(249), 1,
      sym_property_name,
    ACTIONS(69), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
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
    ACTIONS(53), 18,
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
  [544] = 14,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(67), 1,
      anon_sym_fairness,
    ACTIONS(71), 1,
      anon_sym_RBRACE,
    ACTIONS(73), 1,
      anon_sym_tags,
    ACTIONS(75), 1,
      anon_sym_skip,
    ACTIONS(77), 1,
      anon_sym_validate,
    ACTIONS(79), 1,
      anon_sym_before,
    ACTIONS(81), 1,
      anon_sym_after,
    ACTIONS(83), 1,
      anon_sym_each,
    STATE(202), 1,
      sym_language_tag,
    STATE(250), 1,
      sym_property_name,
    ACTIONS(69), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
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
    ACTIONS(53), 18,
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
  [617] = 14,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(55), 1,
      anon_sym_hex,
    ACTIONS(57), 1,
      anon_sym_data,
    ACTIONS(59), 1,
      anon_sym_encoding,
    ACTIONS(61), 1,
      anon_sym_format,
    ACTIONS(63), 1,
      anon_sym_selector,
    ACTIONS(65), 1,
      anon_sym_shape,
    ACTIONS(67), 1,
      anon_sym_fairness,
    ACTIONS(85), 1,
      anon_sym_RBRACE,
    STATE(202), 1,
      sym_language_tag,
    STATE(249), 1,
      sym_property_name,
    ACTIONS(69), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
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
    ACTIONS(53), 18,
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
  [690] = 14,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(67), 1,
      anon_sym_fairness,
    ACTIONS(73), 1,
      anon_sym_tags,
    ACTIONS(75), 1,
      anon_sym_skip,
    ACTIONS(77), 1,
      anon_sym_validate,
    ACTIONS(79), 1,
      anon_sym_before,
    ACTIONS(81), 1,
      anon_sym_after,
    ACTIONS(83), 1,
      anon_sym_each,
    ACTIONS(87), 1,
      anon_sym_RBRACE,
    STATE(202), 1,
      sym_language_tag,
    STATE(250), 1,
      sym_property_name,
    ACTIONS(69), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
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
    ACTIONS(53), 18,
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
  [763] = 14,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(89), 1,
      anon_sym_RBRACE,
    ACTIONS(94), 1,
      anon_sym_hex,
    ACTIONS(97), 1,
      anon_sym_data,
    ACTIONS(100), 1,
      anon_sym_encoding,
    ACTIONS(103), 1,
      anon_sym_format,
    ACTIONS(106), 1,
      anon_sym_selector,
    ACTIONS(109), 1,
      anon_sym_shape,
    ACTIONS(112), 1,
      anon_sym_fairness,
    STATE(202), 1,
      sym_language_tag,
    STATE(249), 1,
      sym_property_name,
    ACTIONS(115), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
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
    ACTIONS(91), 18,
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
  [836] = 14,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(118), 1,
      anon_sym_RBRACE,
    ACTIONS(123), 1,
      anon_sym_tags,
    ACTIONS(126), 1,
      anon_sym_skip,
    ACTIONS(129), 1,
      anon_sym_validate,
    ACTIONS(132), 1,
      anon_sym_before,
    ACTIONS(135), 1,
      anon_sym_after,
    ACTIONS(138), 1,
      anon_sym_each,
    ACTIONS(141), 1,
      anon_sym_fairness,
    STATE(202), 1,
      sym_language_tag,
    STATE(250), 1,
      sym_property_name,
    ACTIONS(144), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
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
    ACTIONS(120), 18,
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
  [909] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(149), 2,
      anon_sym_async,
      anon_sym_fairness,
    ACTIONS(147), 40,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_memory,
      anon_sym_import,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [959] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(153), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(151), 40,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
      anon_sym_data,
      anon_sym_encoding,
      anon_sym_format,
      anon_sym_selector,
      anon_sym_shape,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1009] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(157), 2,
      anon_sym_async,
      anon_sym_fairness,
    ACTIONS(155), 40,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_memory,
      anon_sym_import,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1059] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(161), 1,
      anon_sym_fairness,
    ACTIONS(159), 36,
      anon_sym_RBRACE,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1104] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(165), 1,
      anon_sym_fairness,
    ACTIONS(167), 1,
      anon_sym_ms,
    STATE(7), 1,
      sym_duration_unit,
    ACTIONS(169), 2,
      anon_sym_s,
      anon_sym_m,
    ACTIONS(163), 30,
      anon_sym_RBRACE,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1153] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(165), 1,
      anon_sym_fairness,
    ACTIONS(167), 1,
      anon_sym_ms,
    STATE(7), 1,
      sym_duration_unit,
    ACTIONS(169), 2,
      anon_sym_s,
      anon_sym_m,
    ACTIONS(163), 30,
      anon_sym_RBRACE,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1202] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      anon_sym_globalSetup,
    ACTIONS(67), 1,
      anon_sym_fairness,
    ACTIONS(171), 1,
      anon_sym_RBRACE,
    ACTIONS(173), 1,
      anon_sym_setup,
    ACTIONS(175), 1,
      anon_sym_fixture,
    ACTIONS(177), 1,
      anon_sym_bench,
    ACTIONS(179), 1,
      anon_sym_benchAsync,
    ACTIONS(181), 1,
      anon_sym_after,
    STATE(234), 1,
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
    ACTIONS(53), 18,
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
  [1263] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      anon_sym_globalSetup,
    ACTIONS(67), 1,
      anon_sym_fairness,
    ACTIONS(173), 1,
      anon_sym_setup,
    ACTIONS(175), 1,
      anon_sym_fixture,
    ACTIONS(177), 1,
      anon_sym_bench,
    ACTIONS(179), 1,
      anon_sym_benchAsync,
    ACTIONS(181), 1,
      anon_sym_after,
    ACTIONS(183), 1,
      anon_sym_RBRACE,
    STATE(234), 1,
      sym_property_name,
    STATE(23), 8,
      sym_global_setup,
      sym__suite_item,
      sym_setup_block,
      sym_fixture,
      sym_benchmark,
      sym_after_block,
      sym_property,
      aux_sym_suite_body_repeat1,
    ACTIONS(53), 18,
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
  [1324] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(185), 1,
      anon_sym_globalSetup,
    ACTIONS(188), 1,
      anon_sym_RBRACE,
    ACTIONS(193), 1,
      anon_sym_setup,
    ACTIONS(196), 1,
      anon_sym_fixture,
    ACTIONS(199), 1,
      anon_sym_bench,
    ACTIONS(202), 1,
      anon_sym_benchAsync,
    ACTIONS(205), 1,
      anon_sym_after,
    ACTIONS(208), 1,
      anon_sym_fairness,
    STATE(234), 1,
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
    ACTIONS(190), 18,
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
  [1385] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(213), 1,
      anon_sym_fairness,
    ACTIONS(211), 30,
      anon_sym_RBRACE,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1424] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(217), 1,
      anon_sym_fairness,
    ACTIONS(215), 30,
      anon_sym_RBRACE,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1463] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(221), 1,
      anon_sym_fairness,
    ACTIONS(219), 30,
      anon_sym_RBRACE,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1502] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(225), 1,
      anon_sym_fairness,
    ACTIONS(223), 30,
      anon_sym_RBRACE,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1541] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(229), 1,
      anon_sym_fairness,
    ACTIONS(227), 30,
      anon_sym_RBRACE,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1580] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(233), 1,
      anon_sym_fairness,
    ACTIONS(231), 30,
      anon_sym_RBRACE,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1619] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(237), 1,
      anon_sym_fairness,
    ACTIONS(235), 30,
      anon_sym_RBRACE,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1658] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(241), 1,
      anon_sym_fairness,
    ACTIONS(239), 30,
      anon_sym_RBRACE,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1697] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(245), 1,
      anon_sym_fairness,
    ACTIONS(243), 30,
      anon_sym_RBRACE,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1736] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(249), 1,
      anon_sym_fairness,
    ACTIONS(247), 30,
      anon_sym_RBRACE,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1775] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(253), 1,
      anon_sym_fairness,
    ACTIONS(251), 30,
      anon_sym_RBRACE,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1814] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(257), 1,
      anon_sym_fairness,
    ACTIONS(255), 30,
      anon_sym_RBRACE,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1853] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(261), 1,
      anon_sym_fairness,
    ACTIONS(259), 30,
      anon_sym_RBRACE,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1892] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(265), 1,
      anon_sym_fairness,
    ACTIONS(263), 30,
      anon_sym_RBRACE,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1931] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(269), 1,
      anon_sym_fairness,
    ACTIONS(267), 30,
      anon_sym_RBRACE,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1970] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(273), 1,
      anon_sym_fairness,
    ACTIONS(271), 30,
      anon_sym_RBRACE,
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
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2009] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(167), 1,
      anon_sym_ms,
    STATE(7), 1,
      sym_duration_unit,
    ACTIONS(165), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(169), 2,
      anon_sym_s,
      anon_sym_m,
    ACTIONS(163), 24,
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
  [2053] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(277), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(275), 27,
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
  [2090] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(281), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(279), 27,
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
  [2127] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(285), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(283), 27,
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
  [2164] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(289), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(287), 24,
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
  [2198] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(293), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(291), 24,
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
  [2232] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(297), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(295), 24,
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
  [2266] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(301), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(299), 24,
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
  [2300] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(305), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(303), 24,
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
  [2334] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(309), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(307), 24,
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
  [2368] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(313), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(311), 24,
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
  [2402] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(317), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(315), 24,
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
  [2436] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(321), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(319), 24,
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
  [2470] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(325), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(323), 24,
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
  [2504] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(329), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(327), 24,
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
  [2538] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(333), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(331), 24,
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
  [2572] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(337), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(335), 24,
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
  [2606] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(339), 1,
      anon_sym_RPAREN,
    ACTIONS(343), 1,
      anon_sym_baseline,
    STATE(146), 1,
      sym_chart_param,
    STATE(226), 1,
      sym_chart_params,
    STATE(228), 1,
      sym_chart_param_name,
    ACTIONS(341), 20,
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
  [2647] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(343), 1,
      anon_sym_baseline,
    ACTIONS(345), 1,
      anon_sym_RPAREN,
    STATE(173), 1,
      sym_chart_param,
    STATE(228), 1,
      sym_chart_param_name,
    ACTIONS(341), 20,
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
  [2685] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(343), 1,
      anon_sym_baseline,
    ACTIONS(347), 1,
      anon_sym_RPAREN,
    STATE(173), 1,
      sym_chart_param,
    STATE(228), 1,
      sym_chart_param_name,
    ACTIONS(341), 20,
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
  [2723] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(343), 1,
      anon_sym_baseline,
    STATE(173), 1,
      sym_chart_param,
    STATE(228), 1,
      sym_chart_param_name,
    ACTIONS(341), 20,
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
  [2758] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(349), 1,
      sym_identifier,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    ACTIONS(355), 1,
      sym_number,
    ACTIONS(357), 1,
      sym_float,
    ACTIONS(361), 1,
      anon_sym_LBRACK,
    ACTIONS(359), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(18), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2791] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(349), 1,
      sym_identifier,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    ACTIONS(357), 1,
      sym_float,
    ACTIONS(361), 1,
      anon_sym_LBRACK,
    ACTIONS(363), 1,
      sym_number,
    ACTIONS(359), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(18), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2824] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    ACTIONS(361), 1,
      anon_sym_LBRACK,
    ACTIONS(365), 1,
      sym_identifier,
    ACTIONS(367), 1,
      sym_number,
    ACTIONS(369), 1,
      sym_float,
    ACTIONS(359), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(180), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2857] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(349), 1,
      sym_identifier,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    ACTIONS(357), 1,
      sym_float,
    ACTIONS(361), 1,
      anon_sym_LBRACK,
    ACTIONS(371), 1,
      sym_number,
    ACTIONS(359), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(18), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2890] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(373), 1,
      anon_sym_RBRACE,
    ACTIONS(375), 1,
      anon_sym_declare,
    ACTIONS(378), 1,
      anon_sym_import,
    ACTIONS(381), 1,
      anon_sym_async,
    ACTIONS(384), 1,
      anon_sym_init,
    ACTIONS(387), 1,
      anon_sym_helpers,
    STATE(67), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [2920] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(390), 1,
      anon_sym_RBRACE,
    ACTIONS(392), 1,
      anon_sym_declare,
    ACTIONS(394), 1,
      anon_sym_import,
    ACTIONS(396), 1,
      anon_sym_async,
    ACTIONS(398), 1,
      anon_sym_init,
    ACTIONS(400), 1,
      anon_sym_helpers,
    STATE(67), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [2950] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(392), 1,
      anon_sym_declare,
    ACTIONS(394), 1,
      anon_sym_import,
    ACTIONS(396), 1,
      anon_sym_async,
    ACTIONS(398), 1,
      anon_sym_init,
    ACTIONS(400), 1,
      anon_sym_helpers,
    ACTIONS(402), 1,
      anon_sym_RBRACE,
    STATE(68), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [2980] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    ACTIONS(361), 1,
      anon_sym_LBRACK,
    ACTIONS(404), 1,
      sym_number,
    ACTIONS(406), 1,
      sym_float,
    ACTIONS(408), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(171), 4,
      sym__chart_value,
      sym_string,
      sym_boolean,
      sym_string_array,
  [3009] = 9,
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
    ACTIONS(410), 1,
      ts_builtin_sym_end,
    STATE(100), 1,
      sym_global_setup,
    STATE(80), 2,
      sym_use_statement,
      aux_sym_source_file_repeat1,
    STATE(97), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3039] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(412), 1,
      anon_sym_COLON,
    STATE(242), 1,
      sym_language_tag,
    STATE(38), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(69), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [3060] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(412), 1,
      anon_sym_COLON,
    STATE(242), 1,
      sym_language_tag,
    STATE(35), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(69), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [3081] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(412), 1,
      anon_sym_COLON,
    STATE(242), 1,
      sym_language_tag,
    STATE(26), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(69), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [3102] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(412), 1,
      anon_sym_COLON,
    STATE(242), 1,
      sym_language_tag,
    STATE(37), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(69), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [3123] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(412), 1,
      anon_sym_COLON,
    STATE(242), 1,
      sym_language_tag,
    STATE(39), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(69), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [3144] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(414), 1,
      anon_sym_RBRACE,
    STATE(202), 1,
      sym_language_tag,
    STATE(77), 2,
      sym_language_implementation,
      aux_sym_hook_grouped_repeat1,
    ACTIONS(416), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [3165] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(419), 1,
      anon_sym_RBRACE,
    STATE(202), 1,
      sym_language_tag,
    STATE(77), 2,
      sym_language_implementation,
      aux_sym_hook_grouped_repeat1,
    ACTIONS(69), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [3186] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(421), 1,
      anon_sym_RBRACE,
    STATE(202), 1,
      sym_language_tag,
    STATE(78), 2,
      sym_language_implementation,
      aux_sym_hook_grouped_repeat1,
    ACTIONS(69), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [3207] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(425), 1,
      anon_sym_use,
    STATE(80), 2,
      sym_use_statement,
      aux_sym_source_file_repeat1,
    ACTIONS(423), 4,
      ts_builtin_sym_end,
      anon_sym_globalSetup,
      anon_sym_declare,
      anon_sym_suite,
  [3224] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(428), 1,
      sym_identifier,
    ACTIONS(430), 1,
      anon_sym_RBRACE,
    ACTIONS(432), 1,
      anon_sym_anvil,
    STATE(83), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(131), 2,
      sym_anvil_call,
      sym_function_call,
  [3245] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(434), 1,
      sym_identifier,
    ACTIONS(437), 1,
      anon_sym_RBRACE,
    ACTIONS(439), 1,
      anon_sym_anvil,
    STATE(82), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(131), 2,
      sym_anvil_call,
      sym_function_call,
  [3266] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(428), 1,
      sym_identifier,
    ACTIONS(432), 1,
      anon_sym_anvil,
    ACTIONS(442), 1,
      anon_sym_RBRACE,
    STATE(82), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(131), 2,
      sym_anvil_call,
      sym_function_call,
  [3287] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(170), 1,
      sym_language_tag,
    ACTIONS(69), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [3301] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(444), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3313] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(446), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3325] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(169), 1,
      anon_sym_m,
    STATE(7), 1,
      sym_duration_unit,
    ACTIONS(163), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
    ACTIONS(167), 2,
      anon_sym_ms,
      anon_sym_s,
  [3343] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(448), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3355] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(450), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3367] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(452), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3379] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(454), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3391] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(456), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3403] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_declare,
    ACTIONS(13), 1,
      anon_sym_suite,
    ACTIONS(458), 1,
      ts_builtin_sym_end,
    STATE(106), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3420] = 5,
    ACTIONS(460), 1,
      anon_sym_DQUOTE,
    ACTIONS(464), 1,
      sym_comment,
    STATE(110), 1,
      aux_sym_string_content_repeat1,
    STATE(196), 1,
      sym_string_content,
    ACTIONS(462), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [3437] = 5,
    ACTIONS(460), 1,
      anon_sym_SQUOTE,
    ACTIONS(464), 1,
      sym_comment,
    STATE(111), 1,
      aux_sym_single_string_content_repeat1,
    STATE(195), 1,
      sym_single_string_content,
    ACTIONS(466), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [3454] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(224), 1,
      sym_chart_function_name,
    ACTIONS(468), 4,
      anon_sym_drawSpeedupChart,
      anon_sym_drawTable,
      anon_sym_drawLineChart,
      anon_sym_drawBarChart,
  [3467] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_declare,
    ACTIONS(13), 1,
      anon_sym_suite,
    ACTIONS(470), 1,
      ts_builtin_sym_end,
    STATE(106), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3484] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_declare,
    ACTIONS(13), 1,
      anon_sym_suite,
    ACTIONS(410), 1,
      ts_builtin_sym_end,
    STATE(97), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3501] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(472), 1,
      anon_sym_LBRACE,
    STATE(126), 1,
      sym_suite_type,
    STATE(127), 1,
      sym_suite_body,
    ACTIONS(474), 2,
      anon_sym_performance,
      anon_sym_memory,
  [3518] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_declare,
    ACTIONS(13), 1,
      anon_sym_suite,
    ACTIONS(470), 1,
      ts_builtin_sym_end,
    STATE(93), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3535] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(472), 1,
      anon_sym_LBRACE,
    STATE(151), 1,
      sym_suite_type,
    STATE(154), 1,
      sym_suite_body,
    ACTIONS(474), 2,
      anon_sym_performance,
      anon_sym_memory,
  [3552] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    ACTIONS(476), 1,
      anon_sym_ATfile,
    STATE(33), 2,
      sym_file_ref,
      sym_string,
  [3569] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(478), 5,
      ts_builtin_sym_end,
      anon_sym_use,
      anon_sym_globalSetup,
      anon_sym_declare,
      anon_sym_suite,
  [3580] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    ACTIONS(476), 1,
      anon_sym_ATfile,
    STATE(32), 2,
      sym_file_ref,
      sym_string,
  [3597] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_declare,
    ACTIONS(13), 1,
      anon_sym_suite,
    ACTIONS(410), 1,
      ts_builtin_sym_end,
    STATE(106), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3614] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(480), 1,
      ts_builtin_sym_end,
    ACTIONS(482), 1,
      anon_sym_declare,
    ACTIONS(485), 1,
      anon_sym_suite,
    STATE(106), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3631] = 4,
    ACTIONS(464), 1,
      sym_comment,
    ACTIONS(488), 1,
      anon_sym_DQUOTE,
    STATE(107), 1,
      aux_sym_string_content_repeat1,
    ACTIONS(490), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [3645] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    ACTIONS(493), 1,
      anon_sym_RBRACK,
    STATE(190), 1,
      sym_string,
  [3661] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(495), 1,
      anon_sym_LBRACE,
    ACTIONS(497), 1,
      anon_sym_LPAREN,
    STATE(91), 2,
      sym_code_block,
      sym_paren_code_block,
  [3675] = 4,
    ACTIONS(464), 1,
      sym_comment,
    ACTIONS(499), 1,
      anon_sym_DQUOTE,
    STATE(107), 1,
      aux_sym_string_content_repeat1,
    ACTIONS(501), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [3689] = 4,
    ACTIONS(464), 1,
      sym_comment,
    ACTIONS(503), 1,
      anon_sym_SQUOTE,
    STATE(116), 1,
      aux_sym_single_string_content_repeat1,
    ACTIONS(505), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [3703] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    ACTIONS(507), 1,
      anon_sym_RBRACK,
    STATE(190), 1,
      sym_string,
  [3719] = 4,
    ACTIONS(464), 1,
      sym_comment,
    ACTIONS(509), 1,
      anon_sym_LBRACE,
    ACTIONS(511), 1,
      sym_inline_code,
    STATE(27), 2,
      sym__code_or_inline,
      sym_code_block,
  [3733] = 4,
    ACTIONS(464), 1,
      sym_comment,
    ACTIONS(509), 1,
      anon_sym_LBRACE,
    ACTIONS(513), 1,
      sym_inline_code,
    STATE(20), 2,
      sym__code_or_inline,
      sym_code_block,
  [3747] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(515), 1,
      anon_sym_RBRACE,
    ACTIONS(517), 1,
      anon_sym_charting,
    STATE(115), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [3761] = 4,
    ACTIONS(464), 1,
      sym_comment,
    ACTIONS(520), 1,
      anon_sym_SQUOTE,
    STATE(116), 1,
      aux_sym_single_string_content_repeat1,
    ACTIONS(522), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [3775] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(525), 1,
      sym_identifier,
    ACTIONS(527), 1,
      anon_sym_RPAREN,
    STATE(129), 1,
      sym_argument,
    STATE(239), 1,
      sym_argument_list,
  [3791] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    ACTIONS(529), 1,
      anon_sym_RBRACK,
    STATE(140), 1,
      sym_string,
  [3807] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(531), 1,
      anon_sym_RBRACE,
    ACTIONS(533), 1,
      anon_sym_charting,
    STATE(115), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [3821] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(525), 1,
      sym_identifier,
    ACTIONS(535), 1,
      anon_sym_RPAREN,
    STATE(129), 1,
      sym_argument,
    STATE(223), 1,
      sym_argument_list,
  [3837] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(533), 1,
      anon_sym_charting,
    ACTIONS(537), 1,
      anon_sym_RBRACE,
    STATE(119), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [3851] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    ACTIONS(539), 1,
      sym_identifier,
    STATE(31), 1,
      sym_string,
  [3867] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(541), 1,
      anon_sym_LBRACE,
    ACTIONS(543), 1,
      anon_sym_LPAREN,
    STATE(56), 1,
      sym_fixture_body,
    STATE(176), 1,
      sym_fixture_params,
  [3883] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    ACTIONS(545), 1,
      sym_identifier,
    STATE(30), 1,
      sym_string,
  [3899] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(547), 1,
      sym_identifier,
    ACTIONS(549), 1,
      anon_sym_RPAREN,
    STATE(164), 1,
      sym_fixture_param,
  [3912] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(216), 1,
      sym_run_mode,
    ACTIONS(551), 2,
      anon_sym_timeBased,
      anon_sym_iterationBased,
  [3923] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(553), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [3932] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(555), 1,
      anon_sym_RPAREN,
    ACTIONS(557), 1,
      anon_sym_COMMA,
    STATE(163), 1,
      aux_sym_fixture_params_repeat1,
  [3945] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(559), 1,
      anon_sym_RPAREN,
    ACTIONS(561), 1,
      anon_sym_COMMA,
    STATE(152), 1,
      aux_sym_argument_list_repeat1,
  [3958] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(565), 1,
      anon_sym_RBRACE,
    ACTIONS(563), 2,
      anon_sym_anvil,
      sym_identifier,
  [3969] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(569), 1,
      anon_sym_RBRACE,
    ACTIONS(567), 2,
      anon_sym_anvil,
      sym_identifier,
  [3980] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(571), 1,
      anon_sym_RPAREN,
    ACTIONS(573), 1,
      anon_sym_COMMA,
    STATE(132), 1,
      aux_sym_fixture_params_repeat1,
  [3993] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(576), 1,
      anon_sym_COMMA,
    ACTIONS(579), 1,
      anon_sym_RBRACK,
    STATE(133), 1,
      aux_sym_string_array_repeat1,
  [4006] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(183), 1,
      sym_boolean,
    ACTIONS(408), 2,
      anon_sym_true,
      anon_sym_false,
  [4017] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(581), 1,
      anon_sym_RPAREN,
    ACTIONS(583), 1,
      anon_sym_COMMA,
    STATE(135), 1,
      aux_sym_argument_list_repeat1,
  [4030] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(525), 1,
      sym_identifier,
    ACTIONS(586), 1,
      anon_sym_RPAREN,
    STATE(181), 1,
      sym_argument,
  [4043] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(588), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [4052] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(590), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [4061] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(493), 1,
      anon_sym_RBRACK,
    ACTIONS(592), 1,
      anon_sym_COMMA,
    STATE(133), 1,
      aux_sym_string_array_repeat1,
  [4074] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(594), 1,
      anon_sym_COMMA,
    ACTIONS(596), 1,
      anon_sym_RBRACK,
    STATE(139), 1,
      aux_sym_string_array_repeat1,
  [4087] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    STATE(231), 1,
      sym_string,
  [4100] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    STATE(218), 1,
      sym_string,
  [4113] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(600), 1,
      anon_sym_RBRACE,
    ACTIONS(598), 2,
      anon_sym_anvil,
      sym_identifier,
  [4124] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(604), 1,
      anon_sym_RBRACE,
    ACTIONS(602), 2,
      anon_sym_anvil,
      sym_identifier,
  [4135] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(608), 1,
      anon_sym_RBRACE,
    ACTIONS(606), 2,
      anon_sym_anvil,
      sym_identifier,
  [4146] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(610), 1,
      anon_sym_RPAREN,
    ACTIONS(612), 1,
      anon_sym_COMMA,
    STATE(159), 1,
      aux_sym_chart_params_repeat1,
  [4159] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(616), 1,
      anon_sym_RBRACE,
    ACTIONS(614), 2,
      anon_sym_anvil,
      sym_identifier,
  [4170] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(186), 1,
      sym_boolean,
    ACTIONS(408), 2,
      anon_sym_true,
      anon_sym_false,
  [4181] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    STATE(190), 1,
      sym_string,
  [4194] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(547), 1,
      sym_identifier,
    ACTIONS(618), 1,
      anon_sym_RPAREN,
    STATE(128), 1,
      sym_fixture_param,
  [4207] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(248), 1,
      sym_run_mode,
    ACTIONS(551), 2,
      anon_sym_timeBased,
      anon_sym_iterationBased,
  [4218] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(620), 1,
      anon_sym_RPAREN,
    ACTIONS(622), 1,
      anon_sym_COMMA,
    STATE(135), 1,
      aux_sym_argument_list_repeat1,
  [4231] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(525), 1,
      sym_identifier,
    ACTIONS(620), 1,
      anon_sym_RPAREN,
    STATE(181), 1,
      sym_argument,
  [4244] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(624), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [4253] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(626), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [4262] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(630), 1,
      anon_sym_RBRACE,
    ACTIONS(628), 2,
      anon_sym_anvil,
      sym_identifier,
  [4273] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(632), 1,
      anon_sym_RPAREN,
    ACTIONS(634), 1,
      anon_sym_fork,
    STATE(227), 1,
      sym_anvil_args,
  [4286] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(636), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [4295] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(347), 1,
      anon_sym_RPAREN,
    ACTIONS(638), 1,
      anon_sym_COMMA,
    STATE(161), 1,
      aux_sym_chart_params_repeat1,
  [4308] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    STATE(29), 1,
      sym_string,
  [4321] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(640), 1,
      anon_sym_RPAREN,
    ACTIONS(642), 1,
      anon_sym_COMMA,
    STATE(161), 1,
      aux_sym_chart_params_repeat1,
  [4334] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(547), 1,
      sym_identifier,
    ACTIONS(645), 1,
      anon_sym_RPAREN,
    STATE(164), 1,
      sym_fixture_param,
  [4347] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(645), 1,
      anon_sym_RPAREN,
    ACTIONS(647), 1,
      anon_sym_COMMA,
    STATE(132), 1,
      aux_sym_fixture_params_repeat1,
  [4360] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(571), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4368] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(649), 2,
      anon_sym_timeBased,
      anon_sym_iterationBased,
  [4376] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(651), 2,
      anon_sym_LBRACE,
      anon_sym_COLON,
  [4384] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(495), 1,
      anon_sym_LBRACE,
    STATE(28), 1,
      sym_code_block,
  [4394] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(653), 1,
      anon_sym_LBRACE,
    STATE(43), 1,
      sym_global_setup_body,
  [4404] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(361), 1,
      anon_sym_LBRACK,
    STATE(40), 1,
      sym_string_array,
  [4414] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(655), 1,
      anon_sym_LBRACE,
    STATE(52), 1,
      sym_setup_body,
  [4424] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(657), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4432] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(659), 1,
      anon_sym_LBRACE,
    STATE(58), 1,
      sym_benchmark_body,
  [4442] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(640), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4450] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(661), 2,
      anon_sym_RBRACE,
      anon_sym_charting,
  [4458] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(663), 1,
      anon_sym_LBRACE,
    STATE(57), 1,
      sym_after_body,
  [4468] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(541), 1,
      anon_sym_LBRACE,
    STATE(48), 1,
      sym_fixture_body,
  [4478] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(495), 1,
      anon_sym_LBRACE,
    STATE(88), 1,
      sym_code_block,
  [4488] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(665), 1,
      anon_sym_RPAREN,
    ACTIONS(667), 1,
      sym_embedded_code,
  [4498] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(669), 2,
      anon_sym_RBRACE,
      anon_sym_charting,
  [4506] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(671), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4514] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(581), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4522] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(673), 1,
      anon_sym_RBRACE,
    ACTIONS(675), 1,
      sym_embedded_code,
  [4532] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(472), 1,
      anon_sym_LBRACE,
    STATE(137), 1,
      sym_suite_body,
  [4542] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(525), 1,
      sym_identifier,
    STATE(181), 1,
      sym_argument,
  [4552] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(547), 1,
      sym_identifier,
    STATE(164), 1,
      sym_fixture_param,
  [4562] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(472), 1,
      anon_sym_LBRACE,
    STATE(138), 1,
      sym_suite_body,
  [4572] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(495), 1,
      anon_sym_LBRACE,
    STATE(92), 1,
      sym_code_block,
  [4582] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(495), 1,
      anon_sym_LBRACE,
    STATE(90), 1,
      sym_code_block,
  [4592] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(495), 1,
      anon_sym_LBRACE,
    STATE(89), 1,
      sym_code_block,
  [4602] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(579), 2,
      anon_sym_COMMA,
      anon_sym_RBRACK,
  [4610] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(677), 1,
      anon_sym_DOT,
    ACTIONS(679), 1,
      anon_sym_LPAREN,
  [4620] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(681), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4628] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(683), 1,
      anon_sym_COLON,
  [4635] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(685), 1,
      sym_identifier,
  [4642] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(687), 1,
      anon_sym_SQUOTE,
  [4649] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(687), 1,
      anon_sym_DQUOTE,
  [4656] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(689), 1,
      anon_sym_LPAREN,
  [4663] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(691), 1,
      anon_sym_LPAREN,
  [4670] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(693), 1,
      anon_sym_COLON,
  [4677] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(695), 1,
      anon_sym_COLON,
  [4684] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(697), 1,
      anon_sym_LBRACE,
  [4691] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(699), 1,
      anon_sym_COLON,
  [4698] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(701), 1,
      anon_sym_COLON,
  [4705] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(703), 1,
      anon_sym_LBRACE,
  [4712] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(705), 1,
      anon_sym_COLON,
  [4719] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(707), 1,
      sym_identifier,
  [4726] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(709), 1,
      anon_sym_COLON,
  [4733] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(711), 1,
      anon_sym_LBRACE,
  [4740] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(713), 1,
      anon_sym_COLON,
  [4747] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(715), 1,
      anon_sym_COLON,
  [4754] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(717), 1,
      anon_sym_DOT,
  [4761] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(719), 1,
      sym_identifier,
  [4768] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(721), 1,
      anon_sym_LPAREN,
  [4775] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(723), 1,
      anon_sym_COLON,
  [4782] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(725), 1,
      anon_sym_RBRACE,
  [4789] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(727), 1,
      anon_sym_sameDataset,
  [4796] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(729), 1,
      anon_sym_init,
  [4803] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(731), 1,
      anon_sym_RPAREN,
  [4810] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(733), 1,
      anon_sym_LBRACE,
  [4817] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(735), 1,
      anon_sym_sameDataset,
  [4824] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(737), 1,
      anon_sym_spawnAnvil,
  [4831] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(739), 1,
      anon_sym_RPAREN,
  [4838] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(741), 1,
      anon_sym_RPAREN,
  [4845] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(743), 1,
      anon_sym_LPAREN,
  [4852] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(745), 1,
      anon_sym_COLON,
  [4859] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(747), 1,
      anon_sym_RPAREN,
  [4866] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(749), 1,
      anon_sym_RPAREN,
  [4873] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(751), 1,
      anon_sym_COLON,
  [4880] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(753), 1,
      anon_sym_COLON,
  [4887] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(755), 1,
      anon_sym_DOT,
  [4894] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(757), 1,
      anon_sym_RPAREN,
  [4901] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(759), 1,
      anon_sym_LPAREN,
  [4908] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(761), 1,
      anon_sym_COLON,
  [4915] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(763), 1,
      anon_sym_COLON,
  [4922] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(765), 1,
      sym_identifier,
  [4929] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(767), 1,
      anon_sym_COLON,
  [4936] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(769), 1,
      anon_sym_COLON_COLON,
  [4943] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(771), 1,
      ts_builtin_sym_end,
  [4950] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(773), 1,
      anon_sym_RPAREN,
  [4957] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(775), 1,
      sym_identifier,
  [4964] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(777), 1,
      anon_sym_COLON,
  [4971] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(779), 1,
      anon_sym_COLON,
  [4978] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(781), 1,
      anon_sym_suite,
  [4985] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(783), 1,
      sym_identifier,
  [4992] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(785), 1,
      anon_sym_std,
  [4999] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(787), 1,
      anon_sym_LBRACE,
  [5006] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(789), 1,
      sym_identifier,
  [5013] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(791), 1,
      anon_sym_sameDataset,
  [5020] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(793), 1,
      anon_sym_COLON,
  [5027] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(795), 1,
      anon_sym_COLON,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 53,
  [SMALL_STATE(4)] = 106,
  [SMALL_STATE(5)] = 159,
  [SMALL_STATE(6)] = 211,
  [SMALL_STATE(7)] = 263,
  [SMALL_STATE(8)] = 315,
  [SMALL_STATE(9)] = 367,
  [SMALL_STATE(10)] = 419,
  [SMALL_STATE(11)] = 471,
  [SMALL_STATE(12)] = 544,
  [SMALL_STATE(13)] = 617,
  [SMALL_STATE(14)] = 690,
  [SMALL_STATE(15)] = 763,
  [SMALL_STATE(16)] = 836,
  [SMALL_STATE(17)] = 909,
  [SMALL_STATE(18)] = 959,
  [SMALL_STATE(19)] = 1009,
  [SMALL_STATE(20)] = 1059,
  [SMALL_STATE(21)] = 1104,
  [SMALL_STATE(22)] = 1153,
  [SMALL_STATE(23)] = 1202,
  [SMALL_STATE(24)] = 1263,
  [SMALL_STATE(25)] = 1324,
  [SMALL_STATE(26)] = 1385,
  [SMALL_STATE(27)] = 1424,
  [SMALL_STATE(28)] = 1463,
  [SMALL_STATE(29)] = 1502,
  [SMALL_STATE(30)] = 1541,
  [SMALL_STATE(31)] = 1580,
  [SMALL_STATE(32)] = 1619,
  [SMALL_STATE(33)] = 1658,
  [SMALL_STATE(34)] = 1697,
  [SMALL_STATE(35)] = 1736,
  [SMALL_STATE(36)] = 1775,
  [SMALL_STATE(37)] = 1814,
  [SMALL_STATE(38)] = 1853,
  [SMALL_STATE(39)] = 1892,
  [SMALL_STATE(40)] = 1931,
  [SMALL_STATE(41)] = 1970,
  [SMALL_STATE(42)] = 2009,
  [SMALL_STATE(43)] = 2053,
  [SMALL_STATE(44)] = 2090,
  [SMALL_STATE(45)] = 2127,
  [SMALL_STATE(46)] = 2164,
  [SMALL_STATE(47)] = 2198,
  [SMALL_STATE(48)] = 2232,
  [SMALL_STATE(49)] = 2266,
  [SMALL_STATE(50)] = 2300,
  [SMALL_STATE(51)] = 2334,
  [SMALL_STATE(52)] = 2368,
  [SMALL_STATE(53)] = 2402,
  [SMALL_STATE(54)] = 2436,
  [SMALL_STATE(55)] = 2470,
  [SMALL_STATE(56)] = 2504,
  [SMALL_STATE(57)] = 2538,
  [SMALL_STATE(58)] = 2572,
  [SMALL_STATE(59)] = 2606,
  [SMALL_STATE(60)] = 2647,
  [SMALL_STATE(61)] = 2685,
  [SMALL_STATE(62)] = 2723,
  [SMALL_STATE(63)] = 2758,
  [SMALL_STATE(64)] = 2791,
  [SMALL_STATE(65)] = 2824,
  [SMALL_STATE(66)] = 2857,
  [SMALL_STATE(67)] = 2890,
  [SMALL_STATE(68)] = 2920,
  [SMALL_STATE(69)] = 2950,
  [SMALL_STATE(70)] = 2980,
  [SMALL_STATE(71)] = 3009,
  [SMALL_STATE(72)] = 3039,
  [SMALL_STATE(73)] = 3060,
  [SMALL_STATE(74)] = 3081,
  [SMALL_STATE(75)] = 3102,
  [SMALL_STATE(76)] = 3123,
  [SMALL_STATE(77)] = 3144,
  [SMALL_STATE(78)] = 3165,
  [SMALL_STATE(79)] = 3186,
  [SMALL_STATE(80)] = 3207,
  [SMALL_STATE(81)] = 3224,
  [SMALL_STATE(82)] = 3245,
  [SMALL_STATE(83)] = 3266,
  [SMALL_STATE(84)] = 3287,
  [SMALL_STATE(85)] = 3301,
  [SMALL_STATE(86)] = 3313,
  [SMALL_STATE(87)] = 3325,
  [SMALL_STATE(88)] = 3343,
  [SMALL_STATE(89)] = 3355,
  [SMALL_STATE(90)] = 3367,
  [SMALL_STATE(91)] = 3379,
  [SMALL_STATE(92)] = 3391,
  [SMALL_STATE(93)] = 3403,
  [SMALL_STATE(94)] = 3420,
  [SMALL_STATE(95)] = 3437,
  [SMALL_STATE(96)] = 3454,
  [SMALL_STATE(97)] = 3467,
  [SMALL_STATE(98)] = 3484,
  [SMALL_STATE(99)] = 3501,
  [SMALL_STATE(100)] = 3518,
  [SMALL_STATE(101)] = 3535,
  [SMALL_STATE(102)] = 3552,
  [SMALL_STATE(103)] = 3569,
  [SMALL_STATE(104)] = 3580,
  [SMALL_STATE(105)] = 3597,
  [SMALL_STATE(106)] = 3614,
  [SMALL_STATE(107)] = 3631,
  [SMALL_STATE(108)] = 3645,
  [SMALL_STATE(109)] = 3661,
  [SMALL_STATE(110)] = 3675,
  [SMALL_STATE(111)] = 3689,
  [SMALL_STATE(112)] = 3703,
  [SMALL_STATE(113)] = 3719,
  [SMALL_STATE(114)] = 3733,
  [SMALL_STATE(115)] = 3747,
  [SMALL_STATE(116)] = 3761,
  [SMALL_STATE(117)] = 3775,
  [SMALL_STATE(118)] = 3791,
  [SMALL_STATE(119)] = 3807,
  [SMALL_STATE(120)] = 3821,
  [SMALL_STATE(121)] = 3837,
  [SMALL_STATE(122)] = 3851,
  [SMALL_STATE(123)] = 3867,
  [SMALL_STATE(124)] = 3883,
  [SMALL_STATE(125)] = 3899,
  [SMALL_STATE(126)] = 3912,
  [SMALL_STATE(127)] = 3923,
  [SMALL_STATE(128)] = 3932,
  [SMALL_STATE(129)] = 3945,
  [SMALL_STATE(130)] = 3958,
  [SMALL_STATE(131)] = 3969,
  [SMALL_STATE(132)] = 3980,
  [SMALL_STATE(133)] = 3993,
  [SMALL_STATE(134)] = 4006,
  [SMALL_STATE(135)] = 4017,
  [SMALL_STATE(136)] = 4030,
  [SMALL_STATE(137)] = 4043,
  [SMALL_STATE(138)] = 4052,
  [SMALL_STATE(139)] = 4061,
  [SMALL_STATE(140)] = 4074,
  [SMALL_STATE(141)] = 4087,
  [SMALL_STATE(142)] = 4100,
  [SMALL_STATE(143)] = 4113,
  [SMALL_STATE(144)] = 4124,
  [SMALL_STATE(145)] = 4135,
  [SMALL_STATE(146)] = 4146,
  [SMALL_STATE(147)] = 4159,
  [SMALL_STATE(148)] = 4170,
  [SMALL_STATE(149)] = 4181,
  [SMALL_STATE(150)] = 4194,
  [SMALL_STATE(151)] = 4207,
  [SMALL_STATE(152)] = 4218,
  [SMALL_STATE(153)] = 4231,
  [SMALL_STATE(154)] = 4244,
  [SMALL_STATE(155)] = 4253,
  [SMALL_STATE(156)] = 4262,
  [SMALL_STATE(157)] = 4273,
  [SMALL_STATE(158)] = 4286,
  [SMALL_STATE(159)] = 4295,
  [SMALL_STATE(160)] = 4308,
  [SMALL_STATE(161)] = 4321,
  [SMALL_STATE(162)] = 4334,
  [SMALL_STATE(163)] = 4347,
  [SMALL_STATE(164)] = 4360,
  [SMALL_STATE(165)] = 4368,
  [SMALL_STATE(166)] = 4376,
  [SMALL_STATE(167)] = 4384,
  [SMALL_STATE(168)] = 4394,
  [SMALL_STATE(169)] = 4404,
  [SMALL_STATE(170)] = 4414,
  [SMALL_STATE(171)] = 4424,
  [SMALL_STATE(172)] = 4432,
  [SMALL_STATE(173)] = 4442,
  [SMALL_STATE(174)] = 4450,
  [SMALL_STATE(175)] = 4458,
  [SMALL_STATE(176)] = 4468,
  [SMALL_STATE(177)] = 4478,
  [SMALL_STATE(178)] = 4488,
  [SMALL_STATE(179)] = 4498,
  [SMALL_STATE(180)] = 4506,
  [SMALL_STATE(181)] = 4514,
  [SMALL_STATE(182)] = 4522,
  [SMALL_STATE(183)] = 4532,
  [SMALL_STATE(184)] = 4542,
  [SMALL_STATE(185)] = 4552,
  [SMALL_STATE(186)] = 4562,
  [SMALL_STATE(187)] = 4572,
  [SMALL_STATE(188)] = 4582,
  [SMALL_STATE(189)] = 4592,
  [SMALL_STATE(190)] = 4602,
  [SMALL_STATE(191)] = 4610,
  [SMALL_STATE(192)] = 4620,
  [SMALL_STATE(193)] = 4628,
  [SMALL_STATE(194)] = 4635,
  [SMALL_STATE(195)] = 4642,
  [SMALL_STATE(196)] = 4649,
  [SMALL_STATE(197)] = 4656,
  [SMALL_STATE(198)] = 4663,
  [SMALL_STATE(199)] = 4670,
  [SMALL_STATE(200)] = 4677,
  [SMALL_STATE(201)] = 4684,
  [SMALL_STATE(202)] = 4691,
  [SMALL_STATE(203)] = 4698,
  [SMALL_STATE(204)] = 4705,
  [SMALL_STATE(205)] = 4712,
  [SMALL_STATE(206)] = 4719,
  [SMALL_STATE(207)] = 4726,
  [SMALL_STATE(208)] = 4733,
  [SMALL_STATE(209)] = 4740,
  [SMALL_STATE(210)] = 4747,
  [SMALL_STATE(211)] = 4754,
  [SMALL_STATE(212)] = 4761,
  [SMALL_STATE(213)] = 4768,
  [SMALL_STATE(214)] = 4775,
  [SMALL_STATE(215)] = 4782,
  [SMALL_STATE(216)] = 4789,
  [SMALL_STATE(217)] = 4796,
  [SMALL_STATE(218)] = 4803,
  [SMALL_STATE(219)] = 4810,
  [SMALL_STATE(220)] = 4817,
  [SMALL_STATE(221)] = 4824,
  [SMALL_STATE(222)] = 4831,
  [SMALL_STATE(223)] = 4838,
  [SMALL_STATE(224)] = 4845,
  [SMALL_STATE(225)] = 4852,
  [SMALL_STATE(226)] = 4859,
  [SMALL_STATE(227)] = 4866,
  [SMALL_STATE(228)] = 4873,
  [SMALL_STATE(229)] = 4880,
  [SMALL_STATE(230)] = 4887,
  [SMALL_STATE(231)] = 4894,
  [SMALL_STATE(232)] = 4901,
  [SMALL_STATE(233)] = 4908,
  [SMALL_STATE(234)] = 4915,
  [SMALL_STATE(235)] = 4922,
  [SMALL_STATE(236)] = 4929,
  [SMALL_STATE(237)] = 4936,
  [SMALL_STATE(238)] = 4943,
  [SMALL_STATE(239)] = 4950,
  [SMALL_STATE(240)] = 4957,
  [SMALL_STATE(241)] = 4964,
  [SMALL_STATE(242)] = 4971,
  [SMALL_STATE(243)] = 4978,
  [SMALL_STATE(244)] = 4985,
  [SMALL_STATE(245)] = 4992,
  [SMALL_STATE(246)] = 4999,
  [SMALL_STATE(247)] = 5006,
  [SMALL_STATE(248)] = 5013,
  [SMALL_STATE(249)] = 5020,
  [SMALL_STATE(250)] = 5027,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0, 0, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(245),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(168),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(243),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(240),
  [15] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 2, 0, 0),
  [17] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 2, 0, 0),
  [19] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_boolean, 1, 0, 0),
  [21] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_boolean, 1, 0, 0),
  [23] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 3, 0, 0),
  [25] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 3, 0, 0),
  [27] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 4, 0, 0),
  [29] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 4, 0, 0),
  [31] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_duration_unit, 1, 0, 0),
  [33] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_duration_unit, 1, 0, 0),
  [35] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_duration, 2, 0, 0),
  [37] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_duration, 2, 0, 0),
  [39] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 5, 0, 0),
  [41] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 5, 0, 0),
  [43] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 2, 0, 0),
  [45] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 2, 0, 0),
  [47] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 3, 0, 0),
  [49] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 3, 0, 0),
  [51] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [53] = {.entry = {.count = 1, .reusable = true}}, SHIFT(241),
  [55] = {.entry = {.count = 1, .reusable = true}}, SHIFT(214),
  [57] = {.entry = {.count = 1, .reusable = true}}, SHIFT(210),
  [59] = {.entry = {.count = 1, .reusable = true}}, SHIFT(209),
  [61] = {.entry = {.count = 1, .reusable = true}}, SHIFT(207),
  [63] = {.entry = {.count = 1, .reusable = true}}, SHIFT(193),
  [65] = {.entry = {.count = 1, .reusable = true}}, SHIFT(203),
  [67] = {.entry = {.count = 1, .reusable = false}}, SHIFT(241),
  [69] = {.entry = {.count = 1, .reusable = true}}, SHIFT(166),
  [71] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [73] = {.entry = {.count = 1, .reusable = true}}, SHIFT(199),
  [75] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [77] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [79] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [81] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [83] = {.entry = {.count = 1, .reusable = true}}, SHIFT(73),
  [85] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [87] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [89] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0),
  [91] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(241),
  [94] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(214),
  [97] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(210),
  [100] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(209),
  [103] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(207),
  [106] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(193),
  [109] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(203),
  [112] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(241),
  [115] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(166),
  [118] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0),
  [120] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(241),
  [123] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(199),
  [126] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(76),
  [129] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(72),
  [132] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(75),
  [135] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(74),
  [138] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(73),
  [141] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(241),
  [144] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(166),
  [147] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_code_block, 3, 0, 0),
  [149] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_code_block, 3, 0, 0),
  [151] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property, 3, 0, 5),
  [153] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_property, 3, 0, 5),
  [155] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_code_block, 2, 0, 0),
  [157] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_code_block, 2, 0, 0),
  [159] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_language_implementation, 3, 0, 8),
  [161] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_language_implementation, 3, 0, 8),
  [163] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__value, 1, 0, 0),
  [165] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__value, 1, 0, 0),
  [167] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [169] = {.entry = {.count = 1, .reusable = false}}, SHIFT(6),
  [171] = {.entry = {.count = 1, .reusable = true}}, SHIFT(158),
  [173] = {.entry = {.count = 1, .reusable = true}}, SHIFT(84),
  [175] = {.entry = {.count = 1, .reusable = true}}, SHIFT(244),
  [177] = {.entry = {.count = 1, .reusable = false}}, SHIFT(247),
  [179] = {.entry = {.count = 1, .reusable = true}}, SHIFT(247),
  [181] = {.entry = {.count = 1, .reusable = true}}, SHIFT(175),
  [183] = {.entry = {.count = 1, .reusable = true}}, SHIFT(155),
  [185] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(168),
  [188] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0),
  [190] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(241),
  [193] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(84),
  [196] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(244),
  [199] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(247),
  [202] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(247),
  [205] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(175),
  [208] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(241),
  [211] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_hook, 2, 0, 0),
  [213] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_hook, 2, 0, 0),
  [215] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_flat, 3, 0, 8),
  [217] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hook_flat, 3, 0, 8),
  [219] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_shape_property, 3, 0, 0),
  [221] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_shape_property, 3, 0, 0),
  [223] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selector_property, 3, 0, 0),
  [225] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_selector_property, 3, 0, 0),
  [227] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_format_property, 3, 0, 0),
  [229] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_format_property, 3, 0, 0),
  [231] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_encoding_property, 3, 0, 0),
  [233] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_encoding_property, 3, 0, 0),
  [235] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_data_property, 3, 0, 0),
  [237] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_data_property, 3, 0, 0),
  [239] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hex_property, 3, 0, 0),
  [241] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hex_property, 3, 0, 0),
  [243] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_file_ref, 4, 0, 0),
  [245] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_file_ref, 4, 0, 0),
  [247] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_each_hook, 2, 0, 0),
  [249] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_each_hook, 2, 0, 0),
  [251] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_grouped, 4, 0, 0),
  [253] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hook_grouped, 4, 0, 0),
  [255] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_before_hook, 2, 0, 0),
  [257] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_before_hook, 2, 0, 0),
  [259] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_validate_hook, 2, 0, 0),
  [261] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_validate_hook, 2, 0, 0),
  [263] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_skip_hook, 2, 0, 0),
  [265] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_skip_hook, 2, 0, 0),
  [267] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tags_property, 3, 0, 0),
  [269] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_tags_property, 3, 0, 0),
  [271] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_grouped, 3, 0, 0),
  [273] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hook_grouped, 3, 0, 0),
  [275] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup, 2, 0, 0),
  [277] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup, 2, 0, 0),
  [279] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_body, 2, 0, 0),
  [281] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup_body, 2, 0, 0),
  [283] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_body, 3, 0, 0),
  [285] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup_body, 3, 0, 0),
  [287] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_body, 2, 0, 0),
  [289] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_body, 2, 0, 0),
  [291] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_body, 3, 0, 0),
  [293] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture_body, 3, 0, 0),
  [295] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture, 4, 0, 1),
  [297] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture, 4, 0, 1),
  [299] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_body, 3, 0, 0),
  [301] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_setup_body, 3, 0, 0),
  [303] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_body, 3, 0, 0),
  [305] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_body, 3, 0, 0),
  [307] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_body, 2, 0, 0),
  [309] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture_body, 2, 0, 0),
  [311] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_block, 3, 0, 4),
  [313] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_setup_block, 3, 0, 4),
  [315] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_body, 2, 0, 0),
  [317] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_setup_body, 2, 0, 0),
  [319] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark_body, 2, 0, 0),
  [321] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_benchmark_body, 2, 0, 0),
  [323] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark_body, 3, 0, 0),
  [325] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_benchmark_body, 3, 0, 0),
  [327] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture, 3, 0, 1),
  [329] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture, 3, 0, 1),
  [331] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_block, 2, 0, 0),
  [333] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_block, 2, 0, 0),
  [335] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark, 3, 0, 1),
  [337] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_benchmark, 3, 0, 1),
  [339] = {.entry = {.count = 1, .reusable = true}}, SHIFT(179),
  [341] = {.entry = {.count = 1, .reusable = true}}, SHIFT(225),
  [343] = {.entry = {.count = 1, .reusable = false}}, SHIFT(225),
  [345] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 3, 0, 0),
  [347] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 2, 0, 0),
  [349] = {.entry = {.count = 1, .reusable = false}}, SHIFT(18),
  [351] = {.entry = {.count = 1, .reusable = true}}, SHIFT(94),
  [353] = {.entry = {.count = 1, .reusable = true}}, SHIFT(95),
  [355] = {.entry = {.count = 1, .reusable = false}}, SHIFT(42),
  [357] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [359] = {.entry = {.count = 1, .reusable = false}}, SHIFT(3),
  [361] = {.entry = {.count = 1, .reusable = true}}, SHIFT(118),
  [363] = {.entry = {.count = 1, .reusable = false}}, SHIFT(22),
  [365] = {.entry = {.count = 1, .reusable = false}}, SHIFT(180),
  [367] = {.entry = {.count = 1, .reusable = false}}, SHIFT(87),
  [369] = {.entry = {.count = 1, .reusable = true}}, SHIFT(180),
  [371] = {.entry = {.count = 1, .reusable = false}}, SHIFT(21),
  [373] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0),
  [375] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(187),
  [378] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(109),
  [381] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(217),
  [384] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(188),
  [387] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(189),
  [390] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [392] = {.entry = {.count = 1, .reusable = true}}, SHIFT(187),
  [394] = {.entry = {.count = 1, .reusable = true}}, SHIFT(109),
  [396] = {.entry = {.count = 1, .reusable = true}}, SHIFT(217),
  [398] = {.entry = {.count = 1, .reusable = true}}, SHIFT(188),
  [400] = {.entry = {.count = 1, .reusable = true}}, SHIFT(189),
  [402] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [404] = {.entry = {.count = 1, .reusable = false}}, SHIFT(171),
  [406] = {.entry = {.count = 1, .reusable = true}}, SHIFT(171),
  [408] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [410] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, 0, 0),
  [412] = {.entry = {.count = 1, .reusable = true}}, SHIFT(246),
  [414] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_hook_grouped_repeat1, 2, 0, 0),
  [416] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_hook_grouped_repeat1, 2, 0, 0), SHIFT_REPEAT(166),
  [419] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [421] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [423] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0),
  [425] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(245),
  [428] = {.entry = {.count = 1, .reusable = false}}, SHIFT(191),
  [430] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [432] = {.entry = {.count = 1, .reusable = false}}, SHIFT(211),
  [434] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(191),
  [437] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0),
  [439] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(211),
  [442] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [444] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_paren_code_block, 3, 0, 0),
  [446] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_paren_code_block, 2, 0, 0),
  [448] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_init_section, 3, 0, 0),
  [450] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_helpers_section, 2, 0, 0),
  [452] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_init_section, 2, 0, 0),
  [454] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_section, 2, 0, 0),
  [456] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declare_section, 2, 0, 0),
  [458] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, 0, 0),
  [460] = {.entry = {.count = 1, .reusable = false}}, SHIFT(2),
  [462] = {.entry = {.count = 1, .reusable = false}}, SHIFT(110),
  [464] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [466] = {.entry = {.count = 1, .reusable = false}}, SHIFT(111),
  [468] = {.entry = {.count = 1, .reusable = true}}, SHIFT(232),
  [470] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, 0, 0),
  [472] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [474] = {.entry = {.count = 1, .reusable = true}}, SHIFT(165),
  [476] = {.entry = {.count = 1, .reusable = true}}, SHIFT(197),
  [478] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_use_statement, 4, 0, 2),
  [480] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat2, 2, 0, 0),
  [482] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat2, 2, 0, 0), SHIFT_REPEAT(243),
  [485] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat2, 2, 0, 0), SHIFT_REPEAT(240),
  [488] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_string_content_repeat1, 2, 0, 0),
  [490] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_string_content_repeat1, 2, 0, 0), SHIFT_REPEAT(107),
  [493] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [495] = {.entry = {.count = 1, .reusable = true}}, SHIFT(182),
  [497] = {.entry = {.count = 1, .reusable = true}}, SHIFT(178),
  [499] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_content, 1, 0, 0),
  [501] = {.entry = {.count = 1, .reusable = false}}, SHIFT(107),
  [503] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_single_string_content, 1, 0, 0),
  [505] = {.entry = {.count = 1, .reusable = false}}, SHIFT(116),
  [507] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [509] = {.entry = {.count = 1, .reusable = false}}, SHIFT(182),
  [511] = {.entry = {.count = 1, .reusable = false}}, SHIFT(27),
  [513] = {.entry = {.count = 1, .reusable = false}}, SHIFT(20),
  [515] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_after_body_repeat1, 2, 0, 0),
  [517] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_after_body_repeat1, 2, 0, 0), SHIFT_REPEAT(230),
  [520] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_single_string_content_repeat1, 2, 0, 0),
  [522] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_single_string_content_repeat1, 2, 0, 0), SHIFT_REPEAT(116),
  [525] = {.entry = {.count = 1, .reusable = true}}, SHIFT(205),
  [527] = {.entry = {.count = 1, .reusable = true}}, SHIFT(130),
  [529] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [531] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [533] = {.entry = {.count = 1, .reusable = true}}, SHIFT(230),
  [535] = {.entry = {.count = 1, .reusable = true}}, SHIFT(145),
  [537] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [539] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [541] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [543] = {.entry = {.count = 1, .reusable = true}}, SHIFT(150),
  [545] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [547] = {.entry = {.count = 1, .reusable = true}}, SHIFT(200),
  [549] = {.entry = {.count = 1, .reusable = true}}, SHIFT(219),
  [551] = {.entry = {.count = 1, .reusable = true}}, SHIFT(220),
  [553] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite, 3, 0, 1),
  [555] = {.entry = {.count = 1, .reusable = true}}, SHIFT(208),
  [557] = {.entry = {.count = 1, .reusable = true}}, SHIFT(162),
  [559] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 1, 0, 0),
  [561] = {.entry = {.count = 1, .reusable = true}}, SHIFT(153),
  [563] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 3, 0, 0),
  [565] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 3, 0, 0),
  [567] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup_statement, 1, 0, 0),
  [569] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_statement, 1, 0, 0),
  [571] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fixture_params_repeat1, 2, 0, 0),
  [573] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_params_repeat1, 2, 0, 0), SHIFT_REPEAT(185),
  [576] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_string_array_repeat1, 2, 0, 0), SHIFT_REPEAT(149),
  [579] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_string_array_repeat1, 2, 0, 0),
  [581] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_argument_list_repeat1, 2, 0, 0),
  [583] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_argument_list_repeat1, 2, 0, 0), SHIFT_REPEAT(184),
  [586] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 3, 0, 0),
  [588] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite, 9, 0, 7),
  [590] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite, 8, 0, 6),
  [592] = {.entry = {.count = 1, .reusable = true}}, SHIFT(112),
  [594] = {.entry = {.count = 1, .reusable = true}}, SHIFT(108),
  [596] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [598] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_anvil_call, 6, 0, 0),
  [600] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_call, 6, 0, 0),
  [602] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 6, 0, 0),
  [604] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 6, 0, 0),
  [606] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 5, 0, 0),
  [608] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 5, 0, 0),
  [610] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 1, 0, 0),
  [612] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [614] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_anvil_call, 5, 0, 0),
  [616] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_call, 5, 0, 0),
  [618] = {.entry = {.count = 1, .reusable = true}}, SHIFT(201),
  [620] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 2, 0, 0),
  [622] = {.entry = {.count = 1, .reusable = true}}, SHIFT(136),
  [624] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite, 4, 0, 3),
  [626] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite_body, 2, 0, 0),
  [628] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 4, 0, 0),
  [630] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 4, 0, 0),
  [632] = {.entry = {.count = 1, .reusable = true}}, SHIFT(147),
  [634] = {.entry = {.count = 1, .reusable = true}}, SHIFT(229),
  [636] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite_body, 3, 0, 0),
  [638] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [640] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_chart_params_repeat1, 2, 0, 0),
  [642] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_chart_params_repeat1, 2, 0, 0), SHIFT_REPEAT(62),
  [645] = {.entry = {.count = 1, .reusable = true}}, SHIFT(204),
  [647] = {.entry = {.count = 1, .reusable = true}}, SHIFT(125),
  [649] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite_type, 1, 0, 0),
  [651] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_language_tag, 1, 0, 0),
  [653] = {.entry = {.count = 1, .reusable = true}}, SHIFT(81),
  [655] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [657] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_param, 3, 0, 5),
  [659] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [661] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_directive, 6, 0, 10),
  [663] = {.entry = {.count = 1, .reusable = true}}, SHIFT(121),
  [665] = {.entry = {.count = 1, .reusable = true}}, SHIFT(86),
  [667] = {.entry = {.count = 1, .reusable = true}}, SHIFT(222),
  [669] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_directive, 5, 0, 10),
  [671] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument, 3, 0, 5),
  [673] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [675] = {.entry = {.count = 1, .reusable = true}}, SHIFT(215),
  [677] = {.entry = {.count = 1, .reusable = true}}, SHIFT(194),
  [679] = {.entry = {.count = 1, .reusable = true}}, SHIFT(117),
  [681] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_param, 3, 0, 9),
  [683] = {.entry = {.count = 1, .reusable = true}}, SHIFT(160),
  [685] = {.entry = {.count = 1, .reusable = true}}, SHIFT(198),
  [687] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [689] = {.entry = {.count = 1, .reusable = true}}, SHIFT(141),
  [691] = {.entry = {.count = 1, .reusable = true}}, SHIFT(120),
  [693] = {.entry = {.count = 1, .reusable = true}}, SHIFT(169),
  [695] = {.entry = {.count = 1, .reusable = true}}, SHIFT(206),
  [697] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 2, 0, 0),
  [699] = {.entry = {.count = 1, .reusable = true}}, SHIFT(114),
  [701] = {.entry = {.count = 1, .reusable = true}}, SHIFT(167),
  [703] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 4, 0, 0),
  [705] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [707] = {.entry = {.count = 1, .reusable = true}}, SHIFT(192),
  [709] = {.entry = {.count = 1, .reusable = true}}, SHIFT(124),
  [711] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 3, 0, 0),
  [713] = {.entry = {.count = 1, .reusable = true}}, SHIFT(122),
  [715] = {.entry = {.count = 1, .reusable = true}}, SHIFT(104),
  [717] = {.entry = {.count = 1, .reusable = true}}, SHIFT(221),
  [719] = {.entry = {.count = 1, .reusable = true}}, SHIFT(103),
  [721] = {.entry = {.count = 1, .reusable = true}}, SHIFT(157),
  [723] = {.entry = {.count = 1, .reusable = true}}, SHIFT(102),
  [725] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [727] = {.entry = {.count = 1, .reusable = true}}, SHIFT(236),
  [729] = {.entry = {.count = 1, .reusable = true}}, SHIFT(177),
  [731] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_args, 3, 0, 0),
  [733] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 5, 0, 0),
  [735] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_run_mode, 1, 0, 0),
  [737] = {.entry = {.count = 1, .reusable = true}}, SHIFT(213),
  [739] = {.entry = {.count = 1, .reusable = true}}, SHIFT(85),
  [741] = {.entry = {.count = 1, .reusable = true}}, SHIFT(144),
  [743] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [745] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_param_name, 1, 0, 0),
  [747] = {.entry = {.count = 1, .reusable = true}}, SHIFT(174),
  [749] = {.entry = {.count = 1, .reusable = true}}, SHIFT(143),
  [751] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [753] = {.entry = {.count = 1, .reusable = true}}, SHIFT(142),
  [755] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [757] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [759] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_function_name, 1, 0, 0),
  [761] = {.entry = {.count = 1, .reusable = true}}, SHIFT(134),
  [763] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [765] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [767] = {.entry = {.count = 1, .reusable = true}}, SHIFT(148),
  [769] = {.entry = {.count = 1, .reusable = true}}, SHIFT(212),
  [771] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [773] = {.entry = {.count = 1, .reusable = true}}, SHIFT(156),
  [775] = {.entry = {.count = 1, .reusable = true}}, SHIFT(99),
  [777] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property_name, 1, 0, 0),
  [779] = {.entry = {.count = 1, .reusable = true}}, SHIFT(113),
  [781] = {.entry = {.count = 1, .reusable = true}}, SHIFT(235),
  [783] = {.entry = {.count = 1, .reusable = true}}, SHIFT(123),
  [785] = {.entry = {.count = 1, .reusable = true}}, SHIFT(237),
  [787] = {.entry = {.count = 1, .reusable = true}}, SHIFT(79),
  [789] = {.entry = {.count = 1, .reusable = true}}, SHIFT(172),
  [791] = {.entry = {.count = 1, .reusable = true}}, SHIFT(233),
  [793] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [795] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
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
