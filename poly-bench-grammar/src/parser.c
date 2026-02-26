#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 251
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 193
#define ALIAS_COUNT 0
#define TOKEN_COUNT 110
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
  anon_sym_csharp = 91,
  sym_inline_code = 92,
  anon_sym_DQUOTE = 93,
  anon_sym_SQUOTE = 94,
  aux_sym_string_content_token1 = 95,
  aux_sym_single_string_content_token1 = 96,
  sym_escape_sequence = 97,
  sym_number = 98,
  sym_float = 99,
  anon_sym_ms = 100,
  anon_sym_s = 101,
  anon_sym_m = 102,
  anon_sym_true = 103,
  anon_sym_false = 104,
  anon_sym_LBRACK = 105,
  anon_sym_RBRACK = 106,
  sym_comment = 107,
  sym_embedded_code = 108,
  sym__embedded_code_start = 109,
  sym_source_file = 110,
  sym_use_statement = 111,
  sym_global_setup = 112,
  sym_global_setup_body = 113,
  sym_global_setup_statement = 114,
  sym_anvil_call = 115,
  sym_anvil_args = 116,
  sym_function_call = 117,
  sym_argument_list = 118,
  sym_argument = 119,
  sym_suite = 120,
  sym_suite_type = 121,
  sym_run_mode = 122,
  sym_suite_body = 123,
  sym__suite_item = 124,
  sym_setup_block = 125,
  sym_setup_body = 126,
  sym__setup_section = 127,
  sym_import_section = 128,
  sym_declare_section = 129,
  sym_init_section = 130,
  sym_helpers_section = 131,
  sym_fixture = 132,
  sym_fixture_params = 133,
  sym_fixture_param = 134,
  sym_fixture_body = 135,
  sym__fixture_item = 136,
  sym_hex_property = 137,
  sym_data_property = 138,
  sym_encoding_property = 139,
  sym_format_property = 140,
  sym_selector_property = 141,
  sym_shape_property = 142,
  sym_file_ref = 143,
  sym_benchmark = 144,
  sym_benchmark_body = 145,
  sym__benchmark_item = 146,
  sym_tags_property = 147,
  sym_skip_hook = 148,
  sym_validate_hook = 149,
  sym_before_hook = 150,
  sym_after_hook = 151,
  sym_each_hook = 152,
  sym_hook_flat = 153,
  sym_hook_grouped = 154,
  sym_after_block = 155,
  sym_after_body = 156,
  sym_chart_directive = 157,
  sym_chart_function_name = 158,
  sym_chart_params = 159,
  sym_chart_param = 160,
  sym_chart_param_name = 161,
  sym__chart_value = 162,
  sym_property = 163,
  sym_property_name = 164,
  sym__value = 165,
  sym_language_implementation = 166,
  sym_language_tag = 167,
  sym__code_or_inline = 168,
  sym_code_block = 169,
  sym_paren_code_block = 170,
  sym_string = 171,
  sym_string_content = 172,
  sym_single_string_content = 173,
  sym_duration = 174,
  sym_duration_unit = 175,
  sym_boolean = 176,
  sym_string_array = 177,
  aux_sym_source_file_repeat1 = 178,
  aux_sym_source_file_repeat2 = 179,
  aux_sym_global_setup_body_repeat1 = 180,
  aux_sym_argument_list_repeat1 = 181,
  aux_sym_suite_body_repeat1 = 182,
  aux_sym_setup_body_repeat1 = 183,
  aux_sym_fixture_params_repeat1 = 184,
  aux_sym_fixture_body_repeat1 = 185,
  aux_sym_benchmark_body_repeat1 = 186,
  aux_sym_hook_grouped_repeat1 = 187,
  aux_sym_after_body_repeat1 = 188,
  aux_sym_chart_params_repeat1 = 189,
  aux_sym_string_content_repeat1 = 190,
  aux_sym_single_string_content_repeat1 = 191,
  aux_sym_string_array_repeat1 = 192,
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
  [anon_sym_csharp] = "csharp",
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
  [anon_sym_csharp] = anon_sym_csharp,
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
  [anon_sym_csharp] = {
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
  [86] = 21,
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
  [249] = 218,
  [250] = 218,
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
      if (lookahead == 's') ADVANCE(28);
      if (lookahead == 'v') ADVANCE(29);
      END_STATE();
    case 4:
      if (lookahead == 'a') ADVANCE(30);
      if (lookahead == 'e') ADVANCE(31);
      if (lookahead == 'r') ADVANCE(32);
      END_STATE();
    case 5:
      if (lookahead == 'a') ADVANCE(33);
      if (lookahead == 'n') ADVANCE(34);
      if (lookahead == 'x') ADVANCE(35);
      END_STATE();
    case 6:
      if (lookahead == 'a') ADVANCE(36);
      if (lookahead == 'i') ADVANCE(37);
      if (lookahead == 'o') ADVANCE(38);
      END_STATE();
    case 7:
      if (lookahead == 'l') ADVANCE(39);
      if (lookahead == 'o') ADVANCE(40);
      END_STATE();
    case 8:
      if (lookahead == 'e') ADVANCE(41);
      END_STATE();
    case 9:
      if (lookahead == 'm') ADVANCE(42);
      if (lookahead == 'n') ADVANCE(43);
      if (lookahead == 't') ADVANCE(44);
      END_STATE();
    case 10:
      if (lookahead == 'i') ADVANCE(45);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_m);
      if (lookahead == 'e') ADVANCE(46);
      if (lookahead == 'i') ADVANCE(47);
      if (lookahead == 'o') ADVANCE(48);
      if (lookahead == 's') ADVANCE(49);
      END_STATE();
    case 12:
      if (lookahead == 'r') ADVANCE(50);
      if (lookahead == 'u') ADVANCE(51);
      END_STATE();
    case 13:
      if (lookahead == 'e') ADVANCE(52);
      if (lookahead == 'y') ADVANCE(53);
      END_STATE();
    case 14:
      if (lookahead == 'e') ADVANCE(54);
      if (lookahead == 'o') ADVANCE(55);
      if (lookahead == 'u') ADVANCE(56);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_s);
      ADVANCE_MAP(
        'a', 57,
        'e', 58,
        'h', 59,
        'i', 60,
        'k', 61,
        'o', 62,
        'p', 63,
        't', 64,
        'u', 65,
      );
      END_STATE();
    case 16:
      if (lookahead == 'a') ADVANCE(66);
      if (lookahead == 'h') ADVANCE(67);
      if (lookahead == 'i') ADVANCE(68);
      if (lookahead == 'r') ADVANCE(69);
      if (lookahead == 's') ADVANCE(70);
      if (lookahead == 'y') ADVANCE(71);
      END_STATE();
    case 17:
      if (lookahead == 's') ADVANCE(72);
      END_STATE();
    case 18:
      if (lookahead == 'a') ADVANCE(73);
      END_STATE();
    case 19:
      if (lookahead == 'a') ADVANCE(74);
      if (lookahead == 'i') ADVANCE(75);
      END_STATE();
    case 20:
      if (lookahead == 'S') ADVANCE(76);
      END_STATE();
    case 21:
      if (lookahead == 't') ADVANCE(77);
      END_STATE();
    case 22:
      if (lookahead == 'v') ADVANCE(78);
      END_STATE();
    case 23:
      if (lookahead == 'y') ADVANCE(79);
      END_STATE();
    case 24:
      if (lookahead == 's') ADVANCE(80);
      END_STATE();
    case 25:
      if (lookahead == 'f') ADVANCE(81);
      if (lookahead == 'n') ADVANCE(82);
      END_STATE();
    case 26:
      if (lookahead == 'a') ADVANCE(83);
      END_STATE();
    case 27:
      if (lookahead == 'u') ADVANCE(84);
      END_STATE();
    case 28:
      if (lookahead == 'h') ADVANCE(85);
      END_STATE();
    case 29:
      if (lookahead == 'T') ADVANCE(86);
      END_STATE();
    case 30:
      if (lookahead == 't') ADVANCE(87);
      END_STATE();
    case 31:
      if (lookahead == 'c') ADVANCE(88);
      if (lookahead == 's') ADVANCE(89);
      END_STATE();
    case 32:
      if (lookahead == 'a') ADVANCE(90);
      END_STATE();
    case 33:
      if (lookahead == 'c') ADVANCE(91);
      END_STATE();
    case 34:
      if (lookahead == 'c') ADVANCE(92);
      END_STATE();
    case 35:
      if (lookahead == 'c') ADVANCE(93);
      END_STATE();
    case 36:
      if (lookahead == 'i') ADVANCE(94);
      if (lookahead == 'l') ADVANCE(95);
      END_STATE();
    case 37:
      if (lookahead == 'l') ADVANCE(96);
      if (lookahead == 'x') ADVANCE(97);
      END_STATE();
    case 38:
      if (lookahead == 'r') ADVANCE(98);
      END_STATE();
    case 39:
      if (lookahead == 'o') ADVANCE(99);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(anon_sym_go);
      END_STATE();
    case 41:
      if (lookahead == 'i') ADVANCE(100);
      if (lookahead == 'l') ADVANCE(101);
      if (lookahead == 'x') ADVANCE(102);
      END_STATE();
    case 42:
      if (lookahead == 'p') ADVANCE(103);
      END_STATE();
    case 43:
      if (lookahead == 'c') ADVANCE(104);
      if (lookahead == 'i') ADVANCE(105);
      END_STATE();
    case 44:
      if (lookahead == 'e') ADVANCE(106);
      END_STATE();
    case 45:
      if (lookahead == 'm') ADVANCE(107);
      END_STATE();
    case 46:
      if (lookahead == 'm') ADVANCE(108);
      END_STATE();
    case 47:
      if (lookahead == 'n') ADVANCE(109);
      END_STATE();
    case 48:
      if (lookahead == 'd') ADVANCE(110);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(anon_sym_ms);
      END_STATE();
    case 50:
      if (lookahead == 'd') ADVANCE(111);
      END_STATE();
    case 51:
      if (lookahead == 't') ADVANCE(112);
      END_STATE();
    case 52:
      if (lookahead == 'r') ADVANCE(113);
      END_STATE();
    case 53:
      if (lookahead == 't') ADVANCE(114);
      END_STATE();
    case 54:
      if (lookahead == 'g') ADVANCE(115);
      if (lookahead == 'q') ADVANCE(116);
      END_STATE();
    case 55:
      if (lookahead == 'w') ADVANCE(117);
      END_STATE();
    case 56:
      if (lookahead == 's') ADVANCE(118);
      END_STATE();
    case 57:
      if (lookahead == 'm') ADVANCE(119);
      END_STATE();
    case 58:
      if (lookahead == 'l') ADVANCE(120);
      if (lookahead == 't') ADVANCE(121);
      END_STATE();
    case 59:
      if (lookahead == 'a') ADVANCE(122);
      if (lookahead == 'o') ADVANCE(123);
      END_STATE();
    case 60:
      if (lookahead == 'n') ADVANCE(124);
      END_STATE();
    case 61:
      if (lookahead == 'i') ADVANCE(125);
      END_STATE();
    case 62:
      if (lookahead == 'r') ADVANCE(126);
      END_STATE();
    case 63:
      if (lookahead == 'a') ADVANCE(127);
      END_STATE();
    case 64:
      if (lookahead == 'd') ADVANCE(128);
      END_STATE();
    case 65:
      if (lookahead == 'i') ADVANCE(129);
      END_STATE();
    case 66:
      if (lookahead == 'g') ADVANCE(130);
      if (lookahead == 'r') ADVANCE(131);
      END_STATE();
    case 67:
      if (lookahead == 'e') ADVANCE(132);
      END_STATE();
    case 68:
      if (lookahead == 'm') ADVANCE(133);
      if (lookahead == 't') ADVANCE(134);
      END_STATE();
    case 69:
      if (lookahead == 'u') ADVANCE(135);
      END_STATE();
    case 70:
      ACCEPT_TOKEN(anon_sym_ts);
      END_STATE();
    case 71:
      if (lookahead == 'p') ADVANCE(136);
      END_STATE();
    case 72:
      if (lookahead == 'e') ADVANCE(137);
      END_STATE();
    case 73:
      if (lookahead == 'l') ADVANCE(138);
      END_STATE();
    case 74:
      if (lookahead == 'r') ADVANCE(139);
      END_STATE();
    case 75:
      if (lookahead == 'd') ADVANCE(140);
      END_STATE();
    case 76:
      if (lookahead == 'c') ADVANCE(141);
      END_STATE();
    case 77:
      if (lookahead == 'e') ADVANCE(142);
      END_STATE();
    case 78:
      if (lookahead == 'i') ADVANCE(143);
      END_STATE();
    case 79:
      if (lookahead == 'n') ADVANCE(144);
      END_STATE();
    case 80:
      if (lookahead == 'e') ADVANCE(145);
      END_STATE();
    case 81:
      if (lookahead == 'o') ADVANCE(146);
      END_STATE();
    case 82:
      if (lookahead == 'c') ADVANCE(147);
      END_STATE();
    case 83:
      if (lookahead == 'r') ADVANCE(148);
      END_STATE();
    case 84:
      if (lookahead == 'n') ADVANCE(149);
      END_STATE();
    case 85:
      if (lookahead == 'a') ADVANCE(150);
      END_STATE();
    case 86:
      if (lookahead == 'h') ADVANCE(151);
      END_STATE();
    case 87:
      if (lookahead == 'a') ADVANCE(152);
      END_STATE();
    case 88:
      if (lookahead == 'l') ADVANCE(153);
      END_STATE();
    case 89:
      if (lookahead == 'c') ADVANCE(154);
      END_STATE();
    case 90:
      if (lookahead == 'w') ADVANCE(155);
      END_STATE();
    case 91:
      if (lookahead == 'h') ADVANCE(156);
      END_STATE();
    case 92:
      if (lookahead == 'o') ADVANCE(157);
      END_STATE();
    case 93:
      if (lookahead == 'l') ADVANCE(158);
      END_STATE();
    case 94:
      if (lookahead == 'r') ADVANCE(159);
      END_STATE();
    case 95:
      if (lookahead == 's') ADVANCE(160);
      END_STATE();
    case 96:
      if (lookahead == 't') ADVANCE(161);
      END_STATE();
    case 97:
      if (lookahead == 't') ADVANCE(162);
      END_STATE();
    case 98:
      if (lookahead == 'k') ADVANCE(163);
      if (lookahead == 'm') ADVANCE(164);
      END_STATE();
    case 99:
      if (lookahead == 'b') ADVANCE(165);
      END_STATE();
    case 100:
      if (lookahead == 'g') ADVANCE(166);
      END_STATE();
    case 101:
      if (lookahead == 'p') ADVANCE(167);
      END_STATE();
    case 102:
      ACCEPT_TOKEN(anon_sym_hex);
      END_STATE();
    case 103:
      if (lookahead == 'o') ADVANCE(168);
      END_STATE();
    case 104:
      if (lookahead == 'l') ADVANCE(169);
      END_STATE();
    case 105:
      if (lookahead == 't') ADVANCE(170);
      END_STATE();
    case 106:
      if (lookahead == 'r') ADVANCE(171);
      END_STATE();
    case 107:
      if (lookahead == 'i') ADVANCE(172);
      END_STATE();
    case 108:
      if (lookahead == 'o') ADVANCE(173);
      END_STATE();
    case 109:
      if (lookahead == 'S') ADVANCE(174);
      END_STATE();
    case 110:
      if (lookahead == 'e') ADVANCE(175);
      END_STATE();
    case 111:
      if (lookahead == 'e') ADVANCE(176);
      END_STATE();
    case 112:
      if (lookahead == 'l') ADVANCE(177);
      if (lookahead == 'p') ADVANCE(178);
      END_STATE();
    case 113:
      if (lookahead == 'f') ADVANCE(179);
      END_STATE();
    case 114:
      if (lookahead == 'h') ADVANCE(180);
      END_STATE();
    case 115:
      if (lookahead == 'r') ADVANCE(181);
      END_STATE();
    case 116:
      if (lookahead == 'u') ADVANCE(182);
      END_STATE();
    case 117:
      if (lookahead == 'C') ADVANCE(183);
      END_STATE();
    case 118:
      if (lookahead == 't') ADVANCE(184);
      END_STATE();
    case 119:
      if (lookahead == 'e') ADVANCE(185);
      END_STATE();
    case 120:
      if (lookahead == 'e') ADVANCE(186);
      END_STATE();
    case 121:
      if (lookahead == 'u') ADVANCE(187);
      END_STATE();
    case 122:
      if (lookahead == 'p') ADVANCE(188);
      END_STATE();
    case 123:
      if (lookahead == 'w') ADVANCE(189);
      END_STATE();
    case 124:
      if (lookahead == 'k') ADVANCE(190);
      END_STATE();
    case 125:
      if (lookahead == 'p') ADVANCE(191);
      END_STATE();
    case 126:
      if (lookahead == 't') ADVANCE(192);
      END_STATE();
    case 127:
      if (lookahead == 'w') ADVANCE(193);
      END_STATE();
    case 128:
      ACCEPT_TOKEN(anon_sym_std);
      END_STATE();
    case 129:
      if (lookahead == 't') ADVANCE(194);
      END_STATE();
    case 130:
      if (lookahead == 's') ADVANCE(195);
      END_STATE();
    case 131:
      if (lookahead == 'g') ADVANCE(196);
      END_STATE();
    case 132:
      if (lookahead == 'm') ADVANCE(197);
      END_STATE();
    case 133:
      if (lookahead == 'e') ADVANCE(198);
      END_STATE();
    case 134:
      if (lookahead == 'l') ADVANCE(199);
      END_STATE();
    case 135:
      if (lookahead == 'e') ADVANCE(200);
      END_STATE();
    case 136:
      if (lookahead == 'e') ADVANCE(201);
      END_STATE();
    case 137:
      ACCEPT_TOKEN(anon_sym_use);
      END_STATE();
    case 138:
      if (lookahead == 'i') ADVANCE(202);
      END_STATE();
    case 139:
      if (lookahead == 'm') ADVANCE(203);
      END_STATE();
    case 140:
      if (lookahead == 't') ADVANCE(204);
      END_STATE();
    case 141:
      if (lookahead == 'a') ADVANCE(205);
      END_STATE();
    case 142:
      if (lookahead == 'r') ADVANCE(206);
      END_STATE();
    case 143:
      if (lookahead == 'l') ADVANCE(207);
      END_STATE();
    case 144:
      if (lookahead == 'c') ADVANCE(208);
      END_STATE();
    case 145:
      if (lookahead == 'l') ADVANCE(209);
      END_STATE();
    case 146:
      if (lookahead == 'r') ADVANCE(210);
      END_STATE();
    case 147:
      if (lookahead == 'h') ADVANCE(211);
      END_STATE();
    case 148:
      if (lookahead == 't') ADVANCE(212);
      END_STATE();
    case 149:
      if (lookahead == 't') ADVANCE(213);
      END_STATE();
    case 150:
      if (lookahead == 'r') ADVANCE(214);
      END_STATE();
    case 151:
      if (lookahead == 'r') ADVANCE(215);
      END_STATE();
    case 152:
      ACCEPT_TOKEN(anon_sym_data);
      END_STATE();
    case 153:
      if (lookahead == 'a') ADVANCE(216);
      END_STATE();
    case 154:
      if (lookahead == 'r') ADVANCE(217);
      END_STATE();
    case 155:
      if (lookahead == 'B') ADVANCE(218);
      if (lookahead == 'L') ADVANCE(219);
      if (lookahead == 'S') ADVANCE(220);
      if (lookahead == 'T') ADVANCE(221);
      END_STATE();
    case 156:
      ACCEPT_TOKEN(anon_sym_each);
      END_STATE();
    case 157:
      if (lookahead == 'd') ADVANCE(222);
      END_STATE();
    case 158:
      if (lookahead == 'u') ADVANCE(223);
      END_STATE();
    case 159:
      if (lookahead == 'n') ADVANCE(224);
      END_STATE();
    case 160:
      if (lookahead == 'e') ADVANCE(225);
      END_STATE();
    case 161:
      if (lookahead == 'e') ADVANCE(226);
      END_STATE();
    case 162:
      if (lookahead == 'u') ADVANCE(227);
      END_STATE();
    case 163:
      ACCEPT_TOKEN(anon_sym_fork);
      END_STATE();
    case 164:
      if (lookahead == 'a') ADVANCE(228);
      END_STATE();
    case 165:
      if (lookahead == 'a') ADVANCE(229);
      END_STATE();
    case 166:
      if (lookahead == 'h') ADVANCE(230);
      END_STATE();
    case 167:
      if (lookahead == 'e') ADVANCE(231);
      END_STATE();
    case 168:
      if (lookahead == 'r') ADVANCE(232);
      END_STATE();
    case 169:
      if (lookahead == 'u') ADVANCE(233);
      END_STATE();
    case 170:
      ACCEPT_TOKEN(anon_sym_init);
      END_STATE();
    case 171:
      if (lookahead == 'a') ADVANCE(234);
      END_STATE();
    case 172:
      if (lookahead == 't') ADVANCE(235);
      END_STATE();
    case 173:
      if (lookahead == 'r') ADVANCE(236);
      END_STATE();
    case 174:
      if (lookahead == 'p') ADVANCE(237);
      END_STATE();
    case 175:
      ACCEPT_TOKEN(anon_sym_mode);
      END_STATE();
    case 176:
      if (lookahead == 'r') ADVANCE(238);
      END_STATE();
    case 177:
      if (lookahead == 'i') ADVANCE(239);
      END_STATE();
    case 178:
      if (lookahead == 'u') ADVANCE(240);
      END_STATE();
    case 179:
      if (lookahead == 'o') ADVANCE(241);
      END_STATE();
    case 180:
      if (lookahead == 'o') ADVANCE(242);
      END_STATE();
    case 181:
      if (lookahead == 'e') ADVANCE(243);
      END_STATE();
    case 182:
      if (lookahead == 'i') ADVANCE(244);
      END_STATE();
    case 183:
      if (lookahead == 'o') ADVANCE(245);
      END_STATE();
    case 184:
      ACCEPT_TOKEN(anon_sym_rust);
      END_STATE();
    case 185:
      if (lookahead == 'D') ADVANCE(246);
      END_STATE();
    case 186:
      if (lookahead == 'c') ADVANCE(247);
      END_STATE();
    case 187:
      if (lookahead == 'p') ADVANCE(248);
      END_STATE();
    case 188:
      if (lookahead == 'e') ADVANCE(249);
      END_STATE();
    case 189:
      if (lookahead == 'E') ADVANCE(250);
      if (lookahead == 'R') ADVANCE(251);
      if (lookahead == 'S') ADVANCE(252);
      END_STATE();
    case 190:
      ACCEPT_TOKEN(anon_sym_sink);
      END_STATE();
    case 191:
      ACCEPT_TOKEN(anon_sym_skip);
      END_STATE();
    case 192:
      if (lookahead == 'B') ADVANCE(253);
      if (lookahead == 'O') ADVANCE(254);
      END_STATE();
    case 193:
      if (lookahead == 'n') ADVANCE(255);
      END_STATE();
    case 194:
      if (lookahead == 'e') ADVANCE(256);
      END_STATE();
    case 195:
      ACCEPT_TOKEN(anon_sym_tags);
      END_STATE();
    case 196:
      if (lookahead == 'e') ADVANCE(257);
      END_STATE();
    case 197:
      if (lookahead == 'e') ADVANCE(258);
      END_STATE();
    case 198:
      if (lookahead == 'B') ADVANCE(259);
      if (lookahead == 'o') ADVANCE(260);
      END_STATE();
    case 199:
      if (lookahead == 'e') ADVANCE(261);
      END_STATE();
    case 200:
      ACCEPT_TOKEN(anon_sym_true);
      END_STATE();
    case 201:
      if (lookahead == 's') ADVANCE(262);
      END_STATE();
    case 202:
      if (lookahead == 'd') ADVANCE(263);
      END_STATE();
    case 203:
      if (lookahead == 'u') ADVANCE(264);
      END_STATE();
    case 204:
      if (lookahead == 'h') ADVANCE(265);
      END_STATE();
    case 205:
      if (lookahead == 'l') ADVANCE(266);
      END_STATE();
    case 206:
      ACCEPT_TOKEN(anon_sym_after);
      END_STATE();
    case 207:
      ACCEPT_TOKEN(anon_sym_anvil);
      END_STATE();
    case 208:
      ACCEPT_TOKEN(anon_sym_async);
      if (lookahead == 'S') ADVANCE(267);
      if (lookahead == 'W') ADVANCE(268);
      END_STATE();
    case 209:
      if (lookahead == 'i') ADVANCE(269);
      END_STATE();
    case 210:
      if (lookahead == 'e') ADVANCE(270);
      END_STATE();
    case 211:
      ACCEPT_TOKEN(anon_sym_bench);
      if (lookahead == 'A') ADVANCE(271);
      END_STATE();
    case 212:
      if (lookahead == 'i') ADVANCE(272);
      END_STATE();
    case 213:
      ACCEPT_TOKEN(anon_sym_count);
      END_STATE();
    case 214:
      if (lookahead == 'p') ADVANCE(273);
      END_STATE();
    case 215:
      if (lookahead == 'e') ADVANCE(274);
      END_STATE();
    case 216:
      if (lookahead == 'r') ADVANCE(275);
      END_STATE();
    case 217:
      if (lookahead == 'i') ADVANCE(276);
      END_STATE();
    case 218:
      if (lookahead == 'a') ADVANCE(277);
      END_STATE();
    case 219:
      if (lookahead == 'i') ADVANCE(278);
      END_STATE();
    case 220:
      if (lookahead == 'p') ADVANCE(279);
      END_STATE();
    case 221:
      if (lookahead == 'a') ADVANCE(280);
      END_STATE();
    case 222:
      if (lookahead == 'i') ADVANCE(281);
      END_STATE();
    case 223:
      if (lookahead == 'd') ADVANCE(282);
      END_STATE();
    case 224:
      if (lookahead == 'e') ADVANCE(283);
      END_STATE();
    case 225:
      ACCEPT_TOKEN(anon_sym_false);
      END_STATE();
    case 226:
      if (lookahead == 'r') ADVANCE(284);
      END_STATE();
    case 227:
      if (lookahead == 'r') ADVANCE(285);
      END_STATE();
    case 228:
      if (lookahead == 't') ADVANCE(286);
      END_STATE();
    case 229:
      if (lookahead == 'l') ADVANCE(287);
      END_STATE();
    case 230:
      if (lookahead == 't') ADVANCE(288);
      END_STATE();
    case 231:
      if (lookahead == 'r') ADVANCE(289);
      END_STATE();
    case 232:
      if (lookahead == 't') ADVANCE(290);
      END_STATE();
    case 233:
      if (lookahead == 'd') ADVANCE(291);
      END_STATE();
    case 234:
      if (lookahead == 't') ADVANCE(292);
      END_STATE();
    case 235:
      ACCEPT_TOKEN(anon_sym_limit);
      END_STATE();
    case 236:
      if (lookahead == 'y') ADVANCE(293);
      END_STATE();
    case 237:
      if (lookahead == 'e') ADVANCE(294);
      END_STATE();
    case 238:
      ACCEPT_TOKEN(anon_sym_order);
      END_STATE();
    case 239:
      if (lookahead == 'e') ADVANCE(295);
      END_STATE();
    case 240:
      if (lookahead == 't') ADVANCE(296);
      END_STATE();
    case 241:
      if (lookahead == 'r') ADVANCE(297);
      END_STATE();
    case 242:
      if (lookahead == 'n') ADVANCE(298);
      END_STATE();
    case 243:
      if (lookahead == 's') ADVANCE(299);
      END_STATE();
    case 244:
      if (lookahead == 'r') ADVANCE(300);
      END_STATE();
    case 245:
      if (lookahead == 'u') ADVANCE(301);
      END_STATE();
    case 246:
      if (lookahead == 'a') ADVANCE(302);
      END_STATE();
    case 247:
      if (lookahead == 't') ADVANCE(303);
      END_STATE();
    case 248:
      ACCEPT_TOKEN(anon_sym_setup);
      END_STATE();
    case 249:
      ACCEPT_TOKEN(anon_sym_shape);
      END_STATE();
    case 250:
      if (lookahead == 'r') ADVANCE(304);
      END_STATE();
    case 251:
      if (lookahead == 'e') ADVANCE(305);
      END_STATE();
    case 252:
      if (lookahead == 't') ADVANCE(306);
      END_STATE();
    case 253:
      if (lookahead == 'y') ADVANCE(307);
      END_STATE();
    case 254:
      if (lookahead == 'r') ADVANCE(308);
      END_STATE();
    case 255:
      if (lookahead == 'A') ADVANCE(309);
      END_STATE();
    case 256:
      ACCEPT_TOKEN(anon_sym_suite);
      END_STATE();
    case 257:
      if (lookahead == 't') ADVANCE(310);
      END_STATE();
    case 258:
      ACCEPT_TOKEN(anon_sym_theme);
      END_STATE();
    case 259:
      if (lookahead == 'a') ADVANCE(311);
      END_STATE();
    case 260:
      if (lookahead == 'u') ADVANCE(312);
      END_STATE();
    case 261:
      ACCEPT_TOKEN(anon_sym_title);
      END_STATE();
    case 262:
      if (lookahead == 'c') ADVANCE(313);
      END_STATE();
    case 263:
      if (lookahead == 'a') ADVANCE(314);
      END_STATE();
    case 264:
      if (lookahead == 'p') ADVANCE(315);
      END_STATE();
    case 265:
      ACCEPT_TOKEN(anon_sym_width);
      END_STATE();
    case 266:
      if (lookahead == 'e') ADVANCE(316);
      END_STATE();
    case 267:
      if (lookahead == 'a') ADVANCE(317);
      END_STATE();
    case 268:
      if (lookahead == 'a') ADVANCE(318);
      END_STATE();
    case 269:
      if (lookahead == 'n') ADVANCE(319);
      END_STATE();
    case 270:
      ACCEPT_TOKEN(anon_sym_before);
      END_STATE();
    case 271:
      if (lookahead == 's') ADVANCE(320);
      END_STATE();
    case 272:
      if (lookahead == 'n') ADVANCE(321);
      END_STATE();
    case 273:
      ACCEPT_TOKEN(anon_sym_csharp);
      END_STATE();
    case 274:
      if (lookahead == 's') ADVANCE(322);
      END_STATE();
    case 275:
      if (lookahead == 'e') ADVANCE(323);
      END_STATE();
    case 276:
      if (lookahead == 'p') ADVANCE(324);
      END_STATE();
    case 277:
      if (lookahead == 'r') ADVANCE(325);
      END_STATE();
    case 278:
      if (lookahead == 'n') ADVANCE(326);
      END_STATE();
    case 279:
      if (lookahead == 'e') ADVANCE(327);
      END_STATE();
    case 280:
      if (lookahead == 'b') ADVANCE(328);
      END_STATE();
    case 281:
      if (lookahead == 'n') ADVANCE(329);
      END_STATE();
    case 282:
      if (lookahead == 'e') ADVANCE(330);
      END_STATE();
    case 283:
      if (lookahead == 's') ADVANCE(331);
      END_STATE();
    case 284:
      if (lookahead == 'W') ADVANCE(332);
      END_STATE();
    case 285:
      if (lookahead == 'e') ADVANCE(333);
      END_STATE();
    case 286:
      ACCEPT_TOKEN(anon_sym_format);
      END_STATE();
    case 287:
      if (lookahead == 'S') ADVANCE(334);
      END_STATE();
    case 288:
      ACCEPT_TOKEN(anon_sym_height);
      END_STATE();
    case 289:
      if (lookahead == 's') ADVANCE(335);
      END_STATE();
    case 290:
      ACCEPT_TOKEN(anon_sym_import);
      END_STATE();
    case 291:
      if (lookahead == 'e') ADVANCE(336);
      END_STATE();
    case 292:
      if (lookahead == 'i') ADVANCE(337);
      END_STATE();
    case 293:
      ACCEPT_TOKEN(anon_sym_memory);
      END_STATE();
    case 294:
      if (lookahead == 'e') ADVANCE(338);
      END_STATE();
    case 295:
      if (lookahead == 'r') ADVANCE(339);
      END_STATE();
    case 296:
      ACCEPT_TOKEN(anon_sym_output);
      END_STATE();
    case 297:
      if (lookahead == 'm') ADVANCE(340);
      END_STATE();
    case 298:
      ACCEPT_TOKEN(anon_sym_python);
      END_STATE();
    case 299:
      if (lookahead == 's') ADVANCE(341);
      END_STATE();
    case 300:
      if (lookahead == 'e') ADVANCE(342);
      END_STATE();
    case 301:
      if (lookahead == 'n') ADVANCE(343);
      END_STATE();
    case 302:
      if (lookahead == 't') ADVANCE(344);
      END_STATE();
    case 303:
      if (lookahead == 'o') ADVANCE(345);
      END_STATE();
    case 304:
      if (lookahead == 'r') ADVANCE(346);
      END_STATE();
    case 305:
      if (lookahead == 'g') ADVANCE(347);
      END_STATE();
    case 306:
      if (lookahead == 'd') ADVANCE(348);
      END_STATE();
    case 307:
      ACCEPT_TOKEN(anon_sym_sortBy);
      END_STATE();
    case 308:
      if (lookahead == 'd') ADVANCE(349);
      END_STATE();
    case 309:
      if (lookahead == 'n') ADVANCE(350);
      END_STATE();
    case 310:
      if (lookahead == 'T') ADVANCE(351);
      END_STATE();
    case 311:
      if (lookahead == 's') ADVANCE(352);
      END_STATE();
    case 312:
      if (lookahead == 't') ADVANCE(353);
      END_STATE();
    case 313:
      if (lookahead == 'r') ADVANCE(354);
      END_STATE();
    case 314:
      if (lookahead == 't') ADVANCE(355);
      END_STATE();
    case 315:
      ACCEPT_TOKEN(anon_sym_warmup);
      END_STATE();
    case 316:
      ACCEPT_TOKEN(anon_sym_yScale);
      END_STATE();
    case 317:
      if (lookahead == 'm') ADVANCE(356);
      END_STATE();
    case 318:
      if (lookahead == 'r') ADVANCE(357);
      END_STATE();
    case 319:
      if (lookahead == 'e') ADVANCE(358);
      END_STATE();
    case 320:
      if (lookahead == 'y') ADVANCE(359);
      END_STATE();
    case 321:
      if (lookahead == 'g') ADVANCE(360);
      END_STATE();
    case 322:
      if (lookahead == 'h') ADVANCE(361);
      END_STATE();
    case 323:
      ACCEPT_TOKEN(anon_sym_declare);
      END_STATE();
    case 324:
      if (lookahead == 't') ADVANCE(362);
      END_STATE();
    case 325:
      if (lookahead == 'C') ADVANCE(363);
      END_STATE();
    case 326:
      if (lookahead == 'e') ADVANCE(364);
      END_STATE();
    case 327:
      if (lookahead == 'e') ADVANCE(365);
      END_STATE();
    case 328:
      if (lookahead == 'l') ADVANCE(366);
      END_STATE();
    case 329:
      if (lookahead == 'g') ADVANCE(367);
      END_STATE();
    case 330:
      if (lookahead == 'B') ADVANCE(368);
      END_STATE();
    case 331:
      if (lookahead == 's') ADVANCE(369);
      END_STATE();
    case 332:
      if (lookahead == 'i') ADVANCE(370);
      END_STATE();
    case 333:
      ACCEPT_TOKEN(anon_sym_fixture);
      END_STATE();
    case 334:
      if (lookahead == 'e') ADVANCE(371);
      END_STATE();
    case 335:
      ACCEPT_TOKEN(anon_sym_helpers);
      END_STATE();
    case 336:
      if (lookahead == 'B') ADVANCE(372);
      END_STATE();
    case 337:
      if (lookahead == 'o') ADVANCE(373);
      END_STATE();
    case 338:
      if (lookahead == 'd') ADVANCE(374);
      END_STATE();
    case 339:
      if (lookahead == 'D') ADVANCE(375);
      END_STATE();
    case 340:
      if (lookahead == 'a') ADVANCE(376);
      END_STATE();
    case 341:
      if (lookahead == 'i') ADVANCE(377);
      END_STATE();
    case 342:
      if (lookahead == 's') ADVANCE(378);
      END_STATE();
    case 343:
      if (lookahead == 't') ADVANCE(379);
      END_STATE();
    case 344:
      if (lookahead == 'a') ADVANCE(380);
      END_STATE();
    case 345:
      if (lookahead == 'r') ADVANCE(381);
      END_STATE();
    case 346:
      if (lookahead == 'o') ADVANCE(382);
      END_STATE();
    case 347:
      if (lookahead == 'r') ADVANCE(383);
      END_STATE();
    case 348:
      if (lookahead == 'D') ADVANCE(384);
      END_STATE();
    case 349:
      if (lookahead == 'e') ADVANCE(385);
      END_STATE();
    case 350:
      if (lookahead == 'v') ADVANCE(386);
      END_STATE();
    case 351:
      if (lookahead == 'i') ADVANCE(387);
      END_STATE();
    case 352:
      if (lookahead == 'e') ADVANCE(388);
      END_STATE();
    case 353:
      ACCEPT_TOKEN(anon_sym_timeout);
      END_STATE();
    case 354:
      if (lookahead == 'i') ADVANCE(389);
      END_STATE();
    case 355:
      if (lookahead == 'e') ADVANCE(390);
      END_STATE();
    case 356:
      if (lookahead == 'p') ADVANCE(391);
      END_STATE();
    case 357:
      if (lookahead == 'm') ADVANCE(392);
      END_STATE();
    case 358:
      ACCEPT_TOKEN(anon_sym_baseline);
      if (lookahead == 'B') ADVANCE(393);
      END_STATE();
    case 359:
      if (lookahead == 'n') ADVANCE(394);
      END_STATE();
    case 360:
      ACCEPT_TOKEN(anon_sym_charting);
      END_STATE();
    case 361:
      if (lookahead == 'o') ADVANCE(395);
      END_STATE();
    case 362:
      if (lookahead == 'i') ADVANCE(396);
      END_STATE();
    case 363:
      if (lookahead == 'h') ADVANCE(397);
      END_STATE();
    case 364:
      if (lookahead == 'C') ADVANCE(398);
      END_STATE();
    case 365:
      if (lookahead == 'd') ADVANCE(399);
      END_STATE();
    case 366:
      if (lookahead == 'e') ADVANCE(400);
      END_STATE();
    case 367:
      ACCEPT_TOKEN(anon_sym_encoding);
      END_STATE();
    case 368:
      if (lookahead == 'e') ADVANCE(401);
      END_STATE();
    case 369:
      ACCEPT_TOKEN(anon_sym_fairness);
      if (lookahead == 'S') ADVANCE(402);
      END_STATE();
    case 370:
      if (lookahead == 'n') ADVANCE(403);
      END_STATE();
    case 371:
      if (lookahead == 't') ADVANCE(404);
      END_STATE();
    case 372:
      if (lookahead == 'e') ADVANCE(405);
      END_STATE();
    case 373:
      if (lookahead == 'n') ADVANCE(406);
      END_STATE();
    case 374:
      if (lookahead == 'u') ADVANCE(407);
      END_STATE();
    case 375:
      if (lookahead == 'e') ADVANCE(408);
      END_STATE();
    case 376:
      if (lookahead == 'n') ADVANCE(409);
      END_STATE();
    case 377:
      if (lookahead == 'o') ADVANCE(410);
      END_STATE();
    case 378:
      ACCEPT_TOKEN(anon_sym_requires);
      END_STATE();
    case 379:
      ACCEPT_TOKEN(anon_sym_rowCount);
      END_STATE();
    case 380:
      if (lookahead == 's') ADVANCE(411);
      END_STATE();
    case 381:
      ACCEPT_TOKEN(anon_sym_selector);
      END_STATE();
    case 382:
      if (lookahead == 'r') ADVANCE(412);
      END_STATE();
    case 383:
      if (lookahead == 'e') ADVANCE(413);
      END_STATE();
    case 384:
      if (lookahead == 'e') ADVANCE(414);
      END_STATE();
    case 385:
      if (lookahead == 'r') ADVANCE(415);
      END_STATE();
    case 386:
      if (lookahead == 'i') ADVANCE(416);
      END_STATE();
    case 387:
      if (lookahead == 'm') ADVANCE(417);
      END_STATE();
    case 388:
      if (lookahead == 'd') ADVANCE(418);
      END_STATE();
    case 389:
      if (lookahead == 'p') ADVANCE(419);
      END_STATE();
    case 390:
      ACCEPT_TOKEN(anon_sym_validate);
      END_STATE();
    case 391:
      if (lookahead == 'l') ADVANCE(420);
      END_STATE();
    case 392:
      if (lookahead == 'u') ADVANCE(421);
      END_STATE();
    case 393:
      if (lookahead == 'e') ADVANCE(422);
      END_STATE();
    case 394:
      if (lookahead == 'c') ADVANCE(423);
      END_STATE();
    case 395:
      if (lookahead == 'l') ADVANCE(424);
      END_STATE();
    case 396:
      if (lookahead == 'o') ADVANCE(425);
      END_STATE();
    case 397:
      if (lookahead == 'a') ADVANCE(426);
      END_STATE();
    case 398:
      if (lookahead == 'h') ADVANCE(427);
      END_STATE();
    case 399:
      if (lookahead == 'u') ADVANCE(428);
      END_STATE();
    case 400:
      ACCEPT_TOKEN(anon_sym_drawTable);
      END_STATE();
    case 401:
      if (lookahead == 'n') ADVANCE(429);
      END_STATE();
    case 402:
      if (lookahead == 'e') ADVANCE(430);
      END_STATE();
    case 403:
      if (lookahead == 'n') ADVANCE(431);
      END_STATE();
    case 404:
      if (lookahead == 'u') ADVANCE(432);
      END_STATE();
    case 405:
      if (lookahead == 'n') ADVANCE(433);
      END_STATE();
    case 406:
      if (lookahead == 'B') ADVANCE(434);
      if (lookahead == 's') ADVANCE(435);
      END_STATE();
    case 407:
      if (lookahead == 'p') ADVANCE(436);
      END_STATE();
    case 408:
      if (lookahead == 't') ADVANCE(437);
      END_STATE();
    case 409:
      if (lookahead == 'c') ADVANCE(438);
      END_STATE();
    case 410:
      if (lookahead == 'n') ADVANCE(439);
      END_STATE();
    case 411:
      if (lookahead == 'e') ADVANCE(440);
      END_STATE();
    case 412:
      if (lookahead == 'B') ADVANCE(441);
      END_STATE();
    case 413:
      if (lookahead == 's') ADVANCE(442);
      END_STATE();
    case 414:
      if (lookahead == 'v') ADVANCE(443);
      END_STATE();
    case 415:
      ACCEPT_TOKEN(anon_sym_sortOrder);
      END_STATE();
    case 416:
      if (lookahead == 'l') ADVANCE(444);
      END_STATE();
    case 417:
      if (lookahead == 'e') ADVANCE(445);
      END_STATE();
    case 418:
      ACCEPT_TOKEN(anon_sym_timeBased);
      END_STATE();
    case 419:
      if (lookahead == 't') ADVANCE(446);
      END_STATE();
    case 420:
      if (lookahead == 'e') ADVANCE(447);
      if (lookahead == 'i') ADVANCE(448);
      END_STATE();
    case 421:
      if (lookahead == 'p') ADVANCE(449);
      END_STATE();
    case 422:
      if (lookahead == 'n') ADVANCE(450);
      END_STATE();
    case 423:
      ACCEPT_TOKEN(anon_sym_benchAsync);
      END_STATE();
    case 424:
      if (lookahead == 'd') ADVANCE(451);
      END_STATE();
    case 425:
      if (lookahead == 'n') ADVANCE(452);
      END_STATE();
    case 426:
      if (lookahead == 'r') ADVANCE(453);
      END_STATE();
    case 427:
      if (lookahead == 'a') ADVANCE(454);
      END_STATE();
    case 428:
      if (lookahead == 'p') ADVANCE(455);
      END_STATE();
    case 429:
      if (lookahead == 'c') ADVANCE(456);
      END_STATE();
    case 430:
      if (lookahead == 'e') ADVANCE(457);
      END_STATE();
    case 431:
      if (lookahead == 'e') ADVANCE(458);
      END_STATE();
    case 432:
      if (lookahead == 'p') ADVANCE(459);
      END_STATE();
    case 433:
      if (lookahead == 'c') ADVANCE(460);
      END_STATE();
    case 434:
      if (lookahead == 'a') ADVANCE(461);
      END_STATE();
    case 435:
      ACCEPT_TOKEN(anon_sym_iterations);
      END_STATE();
    case 436:
      ACCEPT_TOKEN(anon_sym_minSpeedup);
      END_STATE();
    case 437:
      if (lookahead == 'e') ADVANCE(462);
      END_STATE();
    case 438:
      if (lookahead == 'e') ADVANCE(463);
      END_STATE();
    case 439:
      if (lookahead == 'M') ADVANCE(464);
      END_STATE();
    case 440:
      if (lookahead == 't') ADVANCE(465);
      END_STATE();
    case 441:
      if (lookahead == 'a') ADVANCE(466);
      END_STATE();
    case 442:
      if (lookahead == 's') ADVANCE(467);
      END_STATE();
    case 443:
      ACCEPT_TOKEN(anon_sym_showStdDev);
      END_STATE();
    case 444:
      ACCEPT_TOKEN(anon_sym_spawnAnvil);
      END_STATE();
    case 445:
      ACCEPT_TOKEN(anon_sym_targetTime);
      END_STATE();
    case 446:
      ACCEPT_TOKEN(anon_sym_typescript);
      END_STATE();
    case 447:
      if (lookahead == 'C') ADVANCE(468);
      END_STATE();
    case 448:
      if (lookahead == 'n') ADVANCE(469);
      END_STATE();
    case 449:
      if (lookahead == 'C') ADVANCE(470);
      END_STATE();
    case 450:
      if (lookahead == 'c') ADVANCE(471);
      END_STATE();
    case 451:
      ACCEPT_TOKEN(anon_sym_cvThreshold);
      END_STATE();
    case 452:
      ACCEPT_TOKEN(anon_sym_description);
      END_STATE();
    case 453:
      if (lookahead == 't') ADVANCE(472);
      END_STATE();
    case 454:
      if (lookahead == 'r') ADVANCE(473);
      END_STATE();
    case 455:
      if (lookahead == 'C') ADVANCE(474);
      END_STATE();
    case 456:
      if (lookahead == 'h') ADVANCE(475);
      END_STATE();
    case 457:
      if (lookahead == 'd') ADVANCE(476);
      END_STATE();
    case 458:
      if (lookahead == 'r') ADVANCE(477);
      END_STATE();
    case 459:
      ACCEPT_TOKEN(anon_sym_globalSetup);
      END_STATE();
    case 460:
      if (lookahead == 'h') ADVANCE(478);
      END_STATE();
    case 461:
      if (lookahead == 's') ADVANCE(479);
      END_STATE();
    case 462:
      if (lookahead == 'c') ADVANCE(480);
      END_STATE();
    case 463:
      ACCEPT_TOKEN(anon_sym_performance);
      END_STATE();
    case 464:
      if (lookahead == 'o') ADVANCE(481);
      END_STATE();
    case 465:
      ACCEPT_TOKEN(anon_sym_sameDataset);
      END_STATE();
    case 466:
      if (lookahead == 'r') ADVANCE(482);
      END_STATE();
    case 467:
      if (lookahead == 'i') ADVANCE(483);
      END_STATE();
    case 468:
      if (lookahead == 'a') ADVANCE(484);
      END_STATE();
    case 469:
      if (lookahead == 'g') ADVANCE(485);
      END_STATE();
    case 470:
      if (lookahead == 'a') ADVANCE(486);
      END_STATE();
    case 471:
      if (lookahead == 'h') ADVANCE(487);
      END_STATE();
    case 472:
      ACCEPT_TOKEN(anon_sym_drawBarChart);
      END_STATE();
    case 473:
      if (lookahead == 't') ADVANCE(488);
      END_STATE();
    case 474:
      if (lookahead == 'h') ADVANCE(489);
      END_STATE();
    case 475:
      if (lookahead == 'm') ADVANCE(490);
      END_STATE();
    case 476:
      ACCEPT_TOKEN(anon_sym_fairnessSeed);
      END_STATE();
    case 477:
      ACCEPT_TOKEN(anon_sym_filterWinner);
      END_STATE();
    case 478:
      if (lookahead == 'm') ADVANCE(491);
      END_STATE();
    case 479:
      if (lookahead == 'e') ADVANCE(492);
      END_STATE();
    case 480:
      if (lookahead == 't') ADVANCE(493);
      END_STATE();
    case 481:
      if (lookahead == 'd') ADVANCE(494);
      END_STATE();
    case 482:
      if (lookahead == 's') ADVANCE(495);
      END_STATE();
    case 483:
      if (lookahead == 'o') ADVANCE(496);
      END_STATE();
    case 484:
      if (lookahead == 'p') ADVANCE(497);
      END_STATE();
    case 485:
      if (lookahead == 'P') ADVANCE(498);
      END_STATE();
    case 486:
      if (lookahead == 'p') ADVANCE(499);
      END_STATE();
    case 487:
      if (lookahead == 'm') ADVANCE(500);
      END_STATE();
    case 488:
      ACCEPT_TOKEN(anon_sym_drawLineChart);
      END_STATE();
    case 489:
      if (lookahead == 'a') ADVANCE(501);
      END_STATE();
    case 490:
      if (lookahead == 'a') ADVANCE(502);
      END_STATE();
    case 491:
      if (lookahead == 'a') ADVANCE(503);
      END_STATE();
    case 492:
      if (lookahead == 'd') ADVANCE(504);
      END_STATE();
    case 493:
      if (lookahead == 'i') ADVANCE(505);
      END_STATE();
    case 494:
      if (lookahead == 'e') ADVANCE(506);
      END_STATE();
    case 495:
      ACCEPT_TOKEN(anon_sym_showErrorBars);
      END_STATE();
    case 496:
      if (lookahead == 'n') ADVANCE(507);
      END_STATE();
    case 497:
      ACCEPT_TOKEN(anon_sym_asyncSampleCap);
      END_STATE();
    case 498:
      if (lookahead == 'o') ADVANCE(508);
      END_STATE();
    case 499:
      ACCEPT_TOKEN(anon_sym_asyncWarmupCap);
      END_STATE();
    case 500:
      if (lookahead == 'a') ADVANCE(509);
      END_STATE();
    case 501:
      if (lookahead == 'r') ADVANCE(510);
      END_STATE();
    case 502:
      if (lookahead == 'r') ADVANCE(511);
      END_STATE();
    case 503:
      if (lookahead == 'r') ADVANCE(512);
      END_STATE();
    case 504:
      ACCEPT_TOKEN(anon_sym_iterationBased);
      END_STATE();
    case 505:
      if (lookahead == 'o') ADVANCE(513);
      END_STATE();
    case 506:
      if (lookahead == 'l') ADVANCE(514);
      END_STATE();
    case 507:
      ACCEPT_TOKEN(anon_sym_showRegression);
      END_STATE();
    case 508:
      if (lookahead == 'l') ADVANCE(515);
      END_STATE();
    case 509:
      if (lookahead == 'r') ADVANCE(516);
      END_STATE();
    case 510:
      if (lookahead == 't') ADVANCE(517);
      END_STATE();
    case 511:
      if (lookahead == 'k') ADVANCE(518);
      END_STATE();
    case 512:
      if (lookahead == 'k') ADVANCE(519);
      END_STATE();
    case 513:
      if (lookahead == 'n') ADVANCE(520);
      END_STATE();
    case 514:
      ACCEPT_TOKEN(anon_sym_regressionModel);
      END_STATE();
    case 515:
      if (lookahead == 'i') ADVANCE(521);
      END_STATE();
    case 516:
      if (lookahead == 'k') ADVANCE(522);
      END_STATE();
    case 517:
      ACCEPT_TOKEN(anon_sym_drawSpeedupChart);
      END_STATE();
    case 518:
      if (lookahead == 's') ADVANCE(523);
      END_STATE();
    case 519:
      if (lookahead == 's') ADVANCE(524);
      END_STATE();
    case 520:
      ACCEPT_TOKEN(anon_sym_outlierDetection);
      END_STATE();
    case 521:
      if (lookahead == 'c') ADVANCE(525);
      END_STATE();
    case 522:
      ACCEPT_TOKEN(anon_sym_baselineBenchmark);
      END_STATE();
    case 523:
      ACCEPT_TOKEN(anon_sym_excludeBenchmarks);
      END_STATE();
    case 524:
      ACCEPT_TOKEN(anon_sym_includeBenchmarks);
      END_STATE();
    case 525:
      if (lookahead == 'y') ADVANCE(526);
      END_STATE();
    case 526:
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
  [94] = {.lex_state = 0},
  [95] = {.lex_state = 3},
  [96] = {.lex_state = 0},
  [97] = {.lex_state = 0},
  [98] = {.lex_state = 0},
  [99] = {.lex_state = 0},
  [100] = {.lex_state = 0},
  [101] = {.lex_state = 1},
  [102] = {.lex_state = 0},
  [103] = {.lex_state = 0},
  [104] = {.lex_state = 0},
  [105] = {.lex_state = 0},
  [106] = {.lex_state = 0},
  [107] = {.lex_state = 1},
  [108] = {.lex_state = 3},
  [109] = {.lex_state = 0},
  [110] = {.lex_state = 0},
  [111] = {.lex_state = 0},
  [112] = {.lex_state = 0},
  [113] = {.lex_state = 4},
  [114] = {.lex_state = 0},
  [115] = {.lex_state = 3},
  [116] = {.lex_state = 0},
  [117] = {.lex_state = 0},
  [118] = {.lex_state = 1},
  [119] = {.lex_state = 0},
  [120] = {.lex_state = 0},
  [121] = {.lex_state = 0},
  [122] = {.lex_state = 0},
  [123] = {.lex_state = 4},
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
  [169] = {.lex_state = 0, .external_lex_state = 2},
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
  [235] = {.lex_state = 2},
  [236] = {.lex_state = 0},
  [237] = {.lex_state = 0},
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
    [anon_sym_csharp] = ACTIONS(1),
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
    [sym_source_file] = STATE(237),
    [sym_use_statement] = STATE(72),
    [sym_global_setup] = STATE(100),
    [sym_suite] = STATE(96),
    [aux_sym_source_file_repeat1] = STATE(72),
    [aux_sym_source_file_repeat2] = STATE(96),
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
    ACTIONS(15), 44,
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
      anon_sym_csharp,
  [54] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(21), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(19), 44,
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
      anon_sym_csharp,
      anon_sym_RBRACK,
  [108] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(25), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(23), 44,
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
      anon_sym_csharp,
      anon_sym_RBRACK,
  [162] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(29), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(27), 43,
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
      anon_sym_csharp,
  [215] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(33), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(31), 43,
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
      anon_sym_csharp,
  [268] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(37), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(35), 43,
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
      anon_sym_csharp,
  [321] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(41), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(39), 43,
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
      anon_sym_csharp,
  [374] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(45), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(43), 43,
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
      anon_sym_csharp,
  [427] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(49), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(47), 43,
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
      anon_sym_csharp,
  [480] = 14,
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
    STATE(204), 1,
      sym_language_tag,
    STATE(249), 1,
      sym_property_name,
    ACTIONS(69), 6,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
      anon_sym_csharp,
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
  [554] = 14,
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
    STATE(204), 1,
      sym_language_tag,
    STATE(250), 1,
      sym_property_name,
    ACTIONS(69), 6,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
      anon_sym_csharp,
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
  [628] = 14,
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
    STATE(204), 1,
      sym_language_tag,
    STATE(249), 1,
      sym_property_name,
    ACTIONS(69), 6,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
      anon_sym_csharp,
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
  [702] = 14,
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
    STATE(204), 1,
      sym_language_tag,
    STATE(250), 1,
      sym_property_name,
    ACTIONS(69), 6,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
      anon_sym_csharp,
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
  [776] = 14,
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
    STATE(204), 1,
      sym_language_tag,
    STATE(249), 1,
      sym_property_name,
    ACTIONS(115), 6,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
      anon_sym_csharp,
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
  [850] = 14,
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
    STATE(204), 1,
      sym_language_tag,
    STATE(250), 1,
      sym_property_name,
    ACTIONS(144), 6,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
      anon_sym_csharp,
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
  [924] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(149), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(147), 41,
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
      anon_sym_csharp,
  [975] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(153), 2,
      anon_sym_async,
      anon_sym_fairness,
    ACTIONS(151), 41,
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
      anon_sym_csharp,
  [1026] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(157), 2,
      anon_sym_async,
      anon_sym_fairness,
    ACTIONS(155), 41,
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
      anon_sym_csharp,
  [1077] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(161), 1,
      anon_sym_fairness,
    ACTIONS(159), 37,
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
      anon_sym_csharp,
  [1123] = 6,
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
    ACTIONS(163), 31,
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
      anon_sym_csharp,
  [1173] = 6,
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
    ACTIONS(163), 31,
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
      anon_sym_csharp,
  [1223] = 12,
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
    STATE(218), 1,
      sym_property_name,
    STATE(24), 8,
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
  [1284] = 12,
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
    STATE(218), 1,
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
  [1345] = 12,
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
    STATE(218), 1,
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
  [1406] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(213), 1,
      anon_sym_fairness,
    ACTIONS(211), 31,
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
      anon_sym_csharp,
  [1446] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(217), 1,
      anon_sym_fairness,
    ACTIONS(215), 31,
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
      anon_sym_csharp,
  [1486] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(221), 1,
      anon_sym_fairness,
    ACTIONS(219), 31,
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
      anon_sym_csharp,
  [1526] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(225), 1,
      anon_sym_fairness,
    ACTIONS(223), 31,
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
      anon_sym_csharp,
  [1566] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(229), 1,
      anon_sym_fairness,
    ACTIONS(227), 31,
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
      anon_sym_csharp,
  [1606] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(233), 1,
      anon_sym_fairness,
    ACTIONS(231), 31,
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
      anon_sym_csharp,
  [1646] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(237), 1,
      anon_sym_fairness,
    ACTIONS(235), 31,
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
      anon_sym_csharp,
  [1686] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(241), 1,
      anon_sym_fairness,
    ACTIONS(239), 31,
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
      anon_sym_csharp,
  [1726] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(245), 1,
      anon_sym_fairness,
    ACTIONS(243), 31,
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
      anon_sym_csharp,
  [1766] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(249), 1,
      anon_sym_fairness,
    ACTIONS(247), 31,
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
      anon_sym_csharp,
  [1806] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(253), 1,
      anon_sym_fairness,
    ACTIONS(251), 31,
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
      anon_sym_csharp,
  [1846] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(257), 1,
      anon_sym_fairness,
    ACTIONS(255), 31,
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
      anon_sym_csharp,
  [1886] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(261), 1,
      anon_sym_fairness,
    ACTIONS(259), 31,
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
      anon_sym_csharp,
  [1926] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(265), 1,
      anon_sym_fairness,
    ACTIONS(263), 31,
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
      anon_sym_csharp,
  [1966] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(269), 1,
      anon_sym_fairness,
    ACTIONS(267), 31,
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
      anon_sym_csharp,
  [2006] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(273), 1,
      anon_sym_fairness,
    ACTIONS(271), 31,
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
      anon_sym_csharp,
  [2046] = 6,
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
  [2090] = 3,
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
  [2127] = 3,
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
  [2164] = 3,
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
  [2201] = 3,
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
  [2235] = 3,
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
  [2269] = 3,
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
  [2303] = 3,
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
  [2337] = 3,
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
  [2371] = 3,
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
  [2405] = 3,
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
  [2439] = 3,
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
  [2473] = 3,
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
  [2507] = 3,
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
  [2541] = 3,
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
  [2575] = 3,
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
  [2609] = 3,
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
  [2643] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(339), 1,
      anon_sym_RPAREN,
    ACTIONS(343), 1,
      anon_sym_baseline,
    STATE(149), 1,
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
  [2684] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(343), 1,
      anon_sym_baseline,
    ACTIONS(345), 1,
      anon_sym_RPAREN,
    STATE(175), 1,
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
  [2722] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(343), 1,
      anon_sym_baseline,
    ACTIONS(347), 1,
      anon_sym_RPAREN,
    STATE(175), 1,
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
  [2760] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(343), 1,
      anon_sym_baseline,
    STATE(175), 1,
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
  [2795] = 9,
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
    STATE(17), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2828] = 9,
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
    STATE(17), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2861] = 9,
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
    STATE(179), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2894] = 9,
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
    STATE(17), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2927] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(373), 1,
      anon_sym_RBRACE,
    ACTIONS(375), 1,
      anon_sym_declare,
    ACTIONS(377), 1,
      anon_sym_import,
    ACTIONS(379), 1,
      anon_sym_async,
    ACTIONS(381), 1,
      anon_sym_init,
    ACTIONS(383), 1,
      anon_sym_helpers,
    STATE(68), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [2957] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(385), 1,
      anon_sym_RBRACE,
    ACTIONS(387), 1,
      anon_sym_declare,
    ACTIONS(390), 1,
      anon_sym_import,
    ACTIONS(393), 1,
      anon_sym_async,
    ACTIONS(396), 1,
      anon_sym_init,
    ACTIONS(399), 1,
      anon_sym_helpers,
    STATE(68), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [2987] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(375), 1,
      anon_sym_declare,
    ACTIONS(377), 1,
      anon_sym_import,
    ACTIONS(379), 1,
      anon_sym_async,
    ACTIONS(381), 1,
      anon_sym_init,
    ACTIONS(383), 1,
      anon_sym_helpers,
    ACTIONS(402), 1,
      anon_sym_RBRACE,
    STATE(67), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [3017] = 8,
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
    STATE(173), 4,
      sym__chart_value,
      sym_string,
      sym_boolean,
      sym_string_array,
  [3046] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(410), 1,
      anon_sym_COLON,
    STATE(241), 1,
      sym_language_tag,
    STATE(38), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(69), 6,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
      anon_sym_csharp,
  [3068] = 9,
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
    ACTIONS(412), 1,
      ts_builtin_sym_end,
    STATE(102), 1,
      sym_global_setup,
    STATE(84), 2,
      sym_use_statement,
      aux_sym_source_file_repeat1,
    STATE(99), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3098] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(410), 1,
      anon_sym_COLON,
    STATE(241), 1,
      sym_language_tag,
    STATE(34), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(69), 6,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
      anon_sym_csharp,
  [3120] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(410), 1,
      anon_sym_COLON,
    STATE(241), 1,
      sym_language_tag,
    STATE(36), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(69), 6,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
      anon_sym_csharp,
  [3142] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(410), 1,
      anon_sym_COLON,
    STATE(241), 1,
      sym_language_tag,
    STATE(37), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(69), 6,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
      anon_sym_csharp,
  [3164] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(410), 1,
      anon_sym_COLON,
    STATE(241), 1,
      sym_language_tag,
    STATE(39), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(69), 6,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
      anon_sym_csharp,
  [3186] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(414), 1,
      anon_sym_RBRACE,
    STATE(204), 1,
      sym_language_tag,
    STATE(78), 2,
      sym_language_implementation,
      aux_sym_hook_grouped_repeat1,
    ACTIONS(69), 6,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
      anon_sym_csharp,
  [3208] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(416), 1,
      anon_sym_RBRACE,
    STATE(204), 1,
      sym_language_tag,
    STATE(78), 2,
      sym_language_implementation,
      aux_sym_hook_grouped_repeat1,
    ACTIONS(418), 6,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
      anon_sym_csharp,
  [3230] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(421), 1,
      anon_sym_RBRACE,
    STATE(204), 1,
      sym_language_tag,
    STATE(77), 2,
      sym_language_implementation,
      aux_sym_hook_grouped_repeat1,
    ACTIONS(69), 6,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
      anon_sym_csharp,
  [3252] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(423), 1,
      sym_identifier,
    ACTIONS(425), 1,
      anon_sym_RBRACE,
    ACTIONS(427), 1,
      anon_sym_anvil,
    STATE(83), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(130), 2,
      sym_anvil_call,
      sym_function_call,
  [3273] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(172), 1,
      sym_language_tag,
    ACTIONS(69), 6,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
      anon_sym_csharp,
  [3288] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(429), 1,
      sym_identifier,
    ACTIONS(432), 1,
      anon_sym_RBRACE,
    ACTIONS(434), 1,
      anon_sym_anvil,
    STATE(82), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(130), 2,
      sym_anvil_call,
      sym_function_call,
  [3309] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(423), 1,
      sym_identifier,
    ACTIONS(427), 1,
      anon_sym_anvil,
    ACTIONS(437), 1,
      anon_sym_RBRACE,
    STATE(82), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(130), 2,
      sym_anvil_call,
      sym_function_call,
  [3330] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(441), 1,
      anon_sym_use,
    STATE(84), 2,
      sym_use_statement,
      aux_sym_source_file_repeat1,
    ACTIONS(439), 4,
      ts_builtin_sym_end,
      anon_sym_globalSetup,
      anon_sym_declare,
      anon_sym_suite,
  [3347] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(444), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3359] = 5,
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
  [3377] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(446), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3389] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(448), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3401] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(450), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3413] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(452), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3425] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(454), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3437] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(456), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3449] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(458), 1,
      ts_builtin_sym_end,
    ACTIONS(460), 1,
      anon_sym_declare,
    ACTIONS(463), 1,
      anon_sym_suite,
    STATE(93), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3466] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    ACTIONS(466), 1,
      anon_sym_ATfile,
    STATE(26), 2,
      sym_file_ref,
      sym_string,
  [3483] = 5,
    ACTIONS(468), 1,
      anon_sym_SQUOTE,
    ACTIONS(472), 1,
      sym_comment,
    STATE(108), 1,
      aux_sym_single_string_content_repeat1,
    STATE(195), 1,
      sym_single_string_content,
    ACTIONS(470), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [3500] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_declare,
    ACTIONS(13), 1,
      anon_sym_suite,
    ACTIONS(412), 1,
      ts_builtin_sym_end,
    STATE(93), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3517] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(474), 1,
      anon_sym_LBRACE,
    STATE(151), 1,
      sym_suite_type,
    STATE(154), 1,
      sym_suite_body,
    ACTIONS(476), 2,
      anon_sym_performance,
      anon_sym_memory,
  [3534] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(221), 1,
      sym_chart_function_name,
    ACTIONS(478), 4,
      anon_sym_drawSpeedupChart,
      anon_sym_drawTable,
      anon_sym_drawLineChart,
      anon_sym_drawBarChart,
  [3547] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_declare,
    ACTIONS(13), 1,
      anon_sym_suite,
    ACTIONS(480), 1,
      ts_builtin_sym_end,
    STATE(93), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3564] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_declare,
    ACTIONS(13), 1,
      anon_sym_suite,
    ACTIONS(412), 1,
      ts_builtin_sym_end,
    STATE(99), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3581] = 5,
    ACTIONS(468), 1,
      anon_sym_DQUOTE,
    ACTIONS(472), 1,
      sym_comment,
    STATE(107), 1,
      aux_sym_string_content_repeat1,
    STATE(197), 1,
      sym_string_content,
    ACTIONS(482), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [3598] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_declare,
    ACTIONS(13), 1,
      anon_sym_suite,
    ACTIONS(480), 1,
      ts_builtin_sym_end,
    STATE(104), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3615] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    ACTIONS(466), 1,
      anon_sym_ATfile,
    STATE(31), 2,
      sym_file_ref,
      sym_string,
  [3632] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_declare,
    ACTIONS(13), 1,
      anon_sym_suite,
    ACTIONS(484), 1,
      ts_builtin_sym_end,
    STATE(93), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3649] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(486), 5,
      ts_builtin_sym_end,
      anon_sym_use,
      anon_sym_globalSetup,
      anon_sym_declare,
      anon_sym_suite,
  [3660] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(474), 1,
      anon_sym_LBRACE,
    STATE(126), 1,
      sym_suite_body,
    STATE(127), 1,
      sym_suite_type,
    ACTIONS(476), 2,
      anon_sym_performance,
      anon_sym_memory,
  [3677] = 4,
    ACTIONS(472), 1,
      sym_comment,
    ACTIONS(488), 1,
      anon_sym_DQUOTE,
    STATE(118), 1,
      aux_sym_string_content_repeat1,
    ACTIONS(490), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [3691] = 4,
    ACTIONS(472), 1,
      sym_comment,
    ACTIONS(492), 1,
      anon_sym_SQUOTE,
    STATE(115), 1,
      aux_sym_single_string_content_repeat1,
    ACTIONS(494), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [3705] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(496), 1,
      anon_sym_LBRACE,
    ACTIONS(498), 1,
      anon_sym_LPAREN,
    STATE(89), 2,
      sym_code_block,
      sym_paren_code_block,
  [3719] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(500), 1,
      anon_sym_RBRACE,
    ACTIONS(502), 1,
      anon_sym_charting,
    STATE(116), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [3733] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    ACTIONS(504), 1,
      anon_sym_RBRACK,
    STATE(189), 1,
      sym_string,
  [3749] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    ACTIONS(506), 1,
      anon_sym_RBRACK,
    STATE(189), 1,
      sym_string,
  [3765] = 4,
    ACTIONS(472), 1,
      sym_comment,
    ACTIONS(508), 1,
      anon_sym_LBRACE,
    ACTIONS(510), 1,
      sym_inline_code,
    STATE(30), 2,
      sym__code_or_inline,
      sym_code_block,
  [3779] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(512), 1,
      sym_identifier,
    ACTIONS(514), 1,
      anon_sym_RPAREN,
    STATE(129), 1,
      sym_argument,
    STATE(194), 1,
      sym_argument_list,
  [3795] = 4,
    ACTIONS(472), 1,
      sym_comment,
    ACTIONS(516), 1,
      anon_sym_SQUOTE,
    STATE(115), 1,
      aux_sym_single_string_content_repeat1,
    ACTIONS(518), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [3809] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(521), 1,
      anon_sym_RBRACE,
    ACTIONS(523), 1,
      anon_sym_charting,
    STATE(116), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [3823] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    ACTIONS(526), 1,
      sym_identifier,
    STATE(29), 1,
      sym_string,
  [3839] = 4,
    ACTIONS(472), 1,
      sym_comment,
    ACTIONS(528), 1,
      anon_sym_DQUOTE,
    STATE(118), 1,
      aux_sym_string_content_repeat1,
    ACTIONS(530), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [3853] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(502), 1,
      anon_sym_charting,
    ACTIONS(533), 1,
      anon_sym_RBRACE,
    STATE(110), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [3867] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(512), 1,
      sym_identifier,
    ACTIONS(535), 1,
      anon_sym_RPAREN,
    STATE(129), 1,
      sym_argument,
    STATE(248), 1,
      sym_argument_list,
  [3883] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    ACTIONS(537), 1,
      sym_identifier,
    STATE(28), 1,
      sym_string,
  [3899] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(539), 1,
      anon_sym_LBRACE,
    ACTIONS(541), 1,
      anon_sym_LPAREN,
    STATE(55), 1,
      sym_fixture_body,
    STATE(177), 1,
      sym_fixture_params,
  [3915] = 4,
    ACTIONS(472), 1,
      sym_comment,
    ACTIONS(508), 1,
      anon_sym_LBRACE,
    ACTIONS(543), 1,
      sym_inline_code,
    STATE(20), 2,
      sym__code_or_inline,
      sym_code_block,
  [3929] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    ACTIONS(545), 1,
      anon_sym_RBRACK,
    STATE(140), 1,
      sym_string,
  [3945] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(549), 1,
      anon_sym_RBRACE,
    ACTIONS(547), 2,
      anon_sym_anvil,
      sym_identifier,
  [3956] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(551), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [3965] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(214), 1,
      sym_run_mode,
    ACTIONS(553), 2,
      anon_sym_timeBased,
      anon_sym_iterationBased,
  [3976] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(555), 1,
      anon_sym_RPAREN,
    ACTIONS(557), 1,
      anon_sym_COMMA,
    STATE(131), 1,
      aux_sym_fixture_params_repeat1,
  [3989] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(559), 1,
      anon_sym_RPAREN,
    ACTIONS(561), 1,
      anon_sym_COMMA,
    STATE(153), 1,
      aux_sym_argument_list_repeat1,
  [4002] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(565), 1,
      anon_sym_RBRACE,
    ACTIONS(563), 2,
      anon_sym_anvil,
      sym_identifier,
  [4013] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(567), 1,
      anon_sym_RPAREN,
    ACTIONS(569), 1,
      anon_sym_COMMA,
    STATE(133), 1,
      aux_sym_fixture_params_repeat1,
  [4026] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(571), 1,
      sym_identifier,
    ACTIONS(573), 1,
      anon_sym_RPAREN,
    STATE(164), 1,
      sym_fixture_param,
  [4039] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(575), 1,
      anon_sym_RPAREN,
    ACTIONS(577), 1,
      anon_sym_COMMA,
    STATE(133), 1,
      aux_sym_fixture_params_repeat1,
  [4052] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(580), 1,
      anon_sym_COMMA,
    ACTIONS(583), 1,
      anon_sym_RBRACK,
    STATE(134), 1,
      aux_sym_string_array_repeat1,
  [4065] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(585), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [4074] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(183), 1,
      sym_boolean,
    ACTIONS(408), 2,
      anon_sym_true,
      anon_sym_false,
  [4085] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(587), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [4094] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(506), 1,
      anon_sym_RBRACK,
    ACTIONS(589), 1,
      anon_sym_COMMA,
    STATE(134), 1,
      aux_sym_string_array_repeat1,
  [4107] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(591), 1,
      anon_sym_RPAREN,
    ACTIONS(593), 1,
      anon_sym_COMMA,
    STATE(139), 1,
      aux_sym_argument_list_repeat1,
  [4120] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(596), 1,
      anon_sym_COMMA,
    ACTIONS(598), 1,
      anon_sym_RBRACK,
    STATE(138), 1,
      aux_sym_string_array_repeat1,
  [4133] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(512), 1,
      sym_identifier,
    ACTIONS(600), 1,
      anon_sym_RPAREN,
    STATE(181), 1,
      sym_argument,
  [4146] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    STATE(215), 1,
      sym_string,
  [4159] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(604), 1,
      anon_sym_RBRACE,
    ACTIONS(602), 2,
      anon_sym_anvil,
      sym_identifier,
  [4170] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(608), 1,
      anon_sym_RBRACE,
    ACTIONS(606), 2,
      anon_sym_anvil,
      sym_identifier,
  [4181] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    STATE(231), 1,
      sym_string,
  [4194] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(612), 1,
      anon_sym_RBRACE,
    ACTIONS(610), 2,
      anon_sym_anvil,
      sym_identifier,
  [4205] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(616), 1,
      anon_sym_RBRACE,
    ACTIONS(614), 2,
      anon_sym_anvil,
      sym_identifier,
  [4216] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(185), 1,
      sym_boolean,
    ACTIONS(408), 2,
      anon_sym_true,
      anon_sym_false,
  [4227] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(618), 1,
      anon_sym_RPAREN,
    ACTIONS(620), 1,
      anon_sym_COMMA,
    STATE(161), 1,
      aux_sym_chart_params_repeat1,
  [4240] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(571), 1,
      sym_identifier,
    ACTIONS(622), 1,
      anon_sym_RPAREN,
    STATE(128), 1,
      sym_fixture_param,
  [4253] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(247), 1,
      sym_run_mode,
    ACTIONS(553), 2,
      anon_sym_timeBased,
      anon_sym_iterationBased,
  [4264] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    STATE(189), 1,
      sym_string,
  [4277] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(624), 1,
      anon_sym_RPAREN,
    ACTIONS(626), 1,
      anon_sym_COMMA,
    STATE(139), 1,
      aux_sym_argument_list_repeat1,
  [4290] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(628), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [4299] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(630), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [4308] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(512), 1,
      sym_identifier,
    ACTIONS(624), 1,
      anon_sym_RPAREN,
    STATE(181), 1,
      sym_argument,
  [4321] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(634), 1,
      anon_sym_RBRACE,
    ACTIONS(632), 2,
      anon_sym_anvil,
      sym_identifier,
  [4332] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(636), 1,
      anon_sym_RPAREN,
    ACTIONS(638), 1,
      anon_sym_fork,
    STATE(223), 1,
      sym_anvil_args,
  [4345] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(640), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [4354] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_DQUOTE,
    ACTIONS(353), 1,
      anon_sym_SQUOTE,
    STATE(27), 1,
      sym_string,
  [4367] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(345), 1,
      anon_sym_RPAREN,
    ACTIONS(642), 1,
      anon_sym_COMMA,
    STATE(162), 1,
      aux_sym_chart_params_repeat1,
  [4380] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(644), 1,
      anon_sym_RPAREN,
    ACTIONS(646), 1,
      anon_sym_COMMA,
    STATE(162), 1,
      aux_sym_chart_params_repeat1,
  [4393] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(567), 1,
      anon_sym_RPAREN,
    ACTIONS(571), 1,
      sym_identifier,
    STATE(164), 1,
      sym_fixture_param,
  [4406] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(575), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4414] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(496), 1,
      anon_sym_LBRACE,
    STATE(33), 1,
      sym_code_block,
  [4424] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(361), 1,
      anon_sym_LBRACK,
    STATE(35), 1,
      sym_string_array,
  [4434] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(649), 1,
      anon_sym_LBRACE,
    STATE(58), 1,
      sym_after_body,
  [4444] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(496), 1,
      anon_sym_LBRACE,
    STATE(90), 1,
      sym_code_block,
  [4454] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(651), 1,
      anon_sym_RPAREN,
    ACTIONS(653), 1,
      sym_embedded_code,
  [4464] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(655), 2,
      anon_sym_LBRACE,
      anon_sym_COLON,
  [4472] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(657), 1,
      anon_sym_LBRACE,
    STATE(43), 1,
      sym_global_setup_body,
  [4482] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(659), 1,
      anon_sym_LBRACE,
    STATE(52), 1,
      sym_setup_body,
  [4492] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(661), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4500] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(663), 1,
      anon_sym_LBRACE,
    STATE(56), 1,
      sym_benchmark_body,
  [4510] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(644), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4518] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(665), 2,
      anon_sym_RBRACE,
      anon_sym_charting,
  [4526] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(539), 1,
      anon_sym_LBRACE,
    STATE(50), 1,
      sym_fixture_body,
  [4536] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(667), 2,
      anon_sym_RBRACE,
      anon_sym_charting,
  [4544] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(669), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4552] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(571), 1,
      sym_identifier,
    STATE(164), 1,
      sym_fixture_param,
  [4562] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(591), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4570] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(671), 1,
      anon_sym_RBRACE,
    ACTIONS(673), 1,
      sym_embedded_code,
  [4580] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(474), 1,
      anon_sym_LBRACE,
    STATE(135), 1,
      sym_suite_body,
  [4590] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(512), 1,
      sym_identifier,
    STATE(181), 1,
      sym_argument,
  [4600] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(474), 1,
      anon_sym_LBRACE,
    STATE(137), 1,
      sym_suite_body,
  [4610] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(496), 1,
      anon_sym_LBRACE,
    STATE(92), 1,
      sym_code_block,
  [4620] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(496), 1,
      anon_sym_LBRACE,
    STATE(88), 1,
      sym_code_block,
  [4630] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(496), 1,
      anon_sym_LBRACE,
    STATE(87), 1,
      sym_code_block,
  [4640] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(583), 2,
      anon_sym_COMMA,
      anon_sym_RBRACK,
  [4648] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(675), 2,
      anon_sym_timeBased,
      anon_sym_iterationBased,
  [4656] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(677), 1,
      anon_sym_DOT,
    ACTIONS(679), 1,
      anon_sym_LPAREN,
  [4666] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(681), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4674] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(683), 1,
      anon_sym_COLON,
  [4681] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(685), 1,
      anon_sym_RPAREN,
  [4688] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(687), 1,
      anon_sym_SQUOTE,
  [4695] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(689), 1,
      anon_sym_RPAREN,
  [4702] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(687), 1,
      anon_sym_DQUOTE,
  [4709] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(691), 1,
      anon_sym_LPAREN,
  [4716] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(693), 1,
      anon_sym_LPAREN,
  [4723] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(695), 1,
      anon_sym_LPAREN,
  [4730] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(697), 1,
      anon_sym_COLON,
  [4737] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(699), 1,
      anon_sym_COLON,
  [4744] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(701), 1,
      anon_sym_LBRACE,
  [4751] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(703), 1,
      anon_sym_COLON,
  [4758] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(705), 1,
      anon_sym_LBRACE,
  [4765] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(707), 1,
      anon_sym_COLON,
  [4772] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(709), 1,
      anon_sym_COLON,
  [4779] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(711), 1,
      anon_sym_DOT,
  [4786] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(713), 1,
      anon_sym_COLON,
  [4793] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(715), 1,
      anon_sym_COLON,
  [4800] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(717), 1,
      sym_identifier,
  [4807] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(719), 1,
      anon_sym_COLON,
  [4814] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(721), 1,
      sym_identifier,
  [4821] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(723), 1,
      anon_sym_sameDataset,
  [4828] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(725), 1,
      anon_sym_RPAREN,
  [4835] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(727), 1,
      anon_sym_init,
  [4842] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(729), 1,
      anon_sym_sameDataset,
  [4849] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(731), 1,
      anon_sym_COLON,
  [4856] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(733), 1,
      anon_sym_LBRACE,
  [4863] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(735), 1,
      anon_sym_spawnAnvil,
  [4870] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(737), 1,
      anon_sym_LPAREN,
  [4877] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(739), 1,
      anon_sym_RBRACE,
  [4884] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(741), 1,
      anon_sym_RPAREN,
  [4891] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(743), 1,
      anon_sym_LPAREN,
  [4898] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(745), 1,
      anon_sym_COLON,
  [4905] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(747), 1,
      anon_sym_RPAREN,
  [4912] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(749), 1,
      anon_sym_COLON,
  [4919] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(751), 1,
      anon_sym_COLON,
  [4926] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(753), 1,
      anon_sym_DOT,
  [4933] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(755), 1,
      sym_identifier,
  [4940] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(757), 1,
      anon_sym_RPAREN,
  [4947] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(759), 1,
      sym_identifier,
  [4954] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(761), 1,
      anon_sym_COLON,
  [4961] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(763), 1,
      sym_identifier,
  [4968] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(765), 1,
      anon_sym_COLON_COLON,
  [4975] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(767), 1,
      anon_sym_COLON,
  [4982] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(769), 1,
      ts_builtin_sym_end,
  [4989] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(771), 1,
      sym_identifier,
  [4996] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(773), 1,
      anon_sym_COLON,
  [5003] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(775), 1,
      anon_sym_suite,
  [5010] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(777), 1,
      anon_sym_COLON,
  [5017] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(779), 1,
      anon_sym_COLON,
  [5024] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(781), 1,
      anon_sym_std,
  [5031] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(783), 1,
      anon_sym_LBRACE,
  [5038] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(785), 1,
      sym_identifier,
  [5045] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(787), 1,
      anon_sym_LBRACE,
  [5052] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(789), 1,
      anon_sym_sameDataset,
  [5059] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(791), 1,
      anon_sym_RPAREN,
  [5066] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(793), 1,
      anon_sym_COLON,
  [5073] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(795), 1,
      anon_sym_COLON,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 54,
  [SMALL_STATE(4)] = 108,
  [SMALL_STATE(5)] = 162,
  [SMALL_STATE(6)] = 215,
  [SMALL_STATE(7)] = 268,
  [SMALL_STATE(8)] = 321,
  [SMALL_STATE(9)] = 374,
  [SMALL_STATE(10)] = 427,
  [SMALL_STATE(11)] = 480,
  [SMALL_STATE(12)] = 554,
  [SMALL_STATE(13)] = 628,
  [SMALL_STATE(14)] = 702,
  [SMALL_STATE(15)] = 776,
  [SMALL_STATE(16)] = 850,
  [SMALL_STATE(17)] = 924,
  [SMALL_STATE(18)] = 975,
  [SMALL_STATE(19)] = 1026,
  [SMALL_STATE(20)] = 1077,
  [SMALL_STATE(21)] = 1123,
  [SMALL_STATE(22)] = 1173,
  [SMALL_STATE(23)] = 1223,
  [SMALL_STATE(24)] = 1284,
  [SMALL_STATE(25)] = 1345,
  [SMALL_STATE(26)] = 1406,
  [SMALL_STATE(27)] = 1446,
  [SMALL_STATE(28)] = 1486,
  [SMALL_STATE(29)] = 1526,
  [SMALL_STATE(30)] = 1566,
  [SMALL_STATE(31)] = 1606,
  [SMALL_STATE(32)] = 1646,
  [SMALL_STATE(33)] = 1686,
  [SMALL_STATE(34)] = 1726,
  [SMALL_STATE(35)] = 1766,
  [SMALL_STATE(36)] = 1806,
  [SMALL_STATE(37)] = 1846,
  [SMALL_STATE(38)] = 1886,
  [SMALL_STATE(39)] = 1926,
  [SMALL_STATE(40)] = 1966,
  [SMALL_STATE(41)] = 2006,
  [SMALL_STATE(42)] = 2046,
  [SMALL_STATE(43)] = 2090,
  [SMALL_STATE(44)] = 2127,
  [SMALL_STATE(45)] = 2164,
  [SMALL_STATE(46)] = 2201,
  [SMALL_STATE(47)] = 2235,
  [SMALL_STATE(48)] = 2269,
  [SMALL_STATE(49)] = 2303,
  [SMALL_STATE(50)] = 2337,
  [SMALL_STATE(51)] = 2371,
  [SMALL_STATE(52)] = 2405,
  [SMALL_STATE(53)] = 2439,
  [SMALL_STATE(54)] = 2473,
  [SMALL_STATE(55)] = 2507,
  [SMALL_STATE(56)] = 2541,
  [SMALL_STATE(57)] = 2575,
  [SMALL_STATE(58)] = 2609,
  [SMALL_STATE(59)] = 2643,
  [SMALL_STATE(60)] = 2684,
  [SMALL_STATE(61)] = 2722,
  [SMALL_STATE(62)] = 2760,
  [SMALL_STATE(63)] = 2795,
  [SMALL_STATE(64)] = 2828,
  [SMALL_STATE(65)] = 2861,
  [SMALL_STATE(66)] = 2894,
  [SMALL_STATE(67)] = 2927,
  [SMALL_STATE(68)] = 2957,
  [SMALL_STATE(69)] = 2987,
  [SMALL_STATE(70)] = 3017,
  [SMALL_STATE(71)] = 3046,
  [SMALL_STATE(72)] = 3068,
  [SMALL_STATE(73)] = 3098,
  [SMALL_STATE(74)] = 3120,
  [SMALL_STATE(75)] = 3142,
  [SMALL_STATE(76)] = 3164,
  [SMALL_STATE(77)] = 3186,
  [SMALL_STATE(78)] = 3208,
  [SMALL_STATE(79)] = 3230,
  [SMALL_STATE(80)] = 3252,
  [SMALL_STATE(81)] = 3273,
  [SMALL_STATE(82)] = 3288,
  [SMALL_STATE(83)] = 3309,
  [SMALL_STATE(84)] = 3330,
  [SMALL_STATE(85)] = 3347,
  [SMALL_STATE(86)] = 3359,
  [SMALL_STATE(87)] = 3377,
  [SMALL_STATE(88)] = 3389,
  [SMALL_STATE(89)] = 3401,
  [SMALL_STATE(90)] = 3413,
  [SMALL_STATE(91)] = 3425,
  [SMALL_STATE(92)] = 3437,
  [SMALL_STATE(93)] = 3449,
  [SMALL_STATE(94)] = 3466,
  [SMALL_STATE(95)] = 3483,
  [SMALL_STATE(96)] = 3500,
  [SMALL_STATE(97)] = 3517,
  [SMALL_STATE(98)] = 3534,
  [SMALL_STATE(99)] = 3547,
  [SMALL_STATE(100)] = 3564,
  [SMALL_STATE(101)] = 3581,
  [SMALL_STATE(102)] = 3598,
  [SMALL_STATE(103)] = 3615,
  [SMALL_STATE(104)] = 3632,
  [SMALL_STATE(105)] = 3649,
  [SMALL_STATE(106)] = 3660,
  [SMALL_STATE(107)] = 3677,
  [SMALL_STATE(108)] = 3691,
  [SMALL_STATE(109)] = 3705,
  [SMALL_STATE(110)] = 3719,
  [SMALL_STATE(111)] = 3733,
  [SMALL_STATE(112)] = 3749,
  [SMALL_STATE(113)] = 3765,
  [SMALL_STATE(114)] = 3779,
  [SMALL_STATE(115)] = 3795,
  [SMALL_STATE(116)] = 3809,
  [SMALL_STATE(117)] = 3823,
  [SMALL_STATE(118)] = 3839,
  [SMALL_STATE(119)] = 3853,
  [SMALL_STATE(120)] = 3867,
  [SMALL_STATE(121)] = 3883,
  [SMALL_STATE(122)] = 3899,
  [SMALL_STATE(123)] = 3915,
  [SMALL_STATE(124)] = 3929,
  [SMALL_STATE(125)] = 3945,
  [SMALL_STATE(126)] = 3956,
  [SMALL_STATE(127)] = 3965,
  [SMALL_STATE(128)] = 3976,
  [SMALL_STATE(129)] = 3989,
  [SMALL_STATE(130)] = 4002,
  [SMALL_STATE(131)] = 4013,
  [SMALL_STATE(132)] = 4026,
  [SMALL_STATE(133)] = 4039,
  [SMALL_STATE(134)] = 4052,
  [SMALL_STATE(135)] = 4065,
  [SMALL_STATE(136)] = 4074,
  [SMALL_STATE(137)] = 4085,
  [SMALL_STATE(138)] = 4094,
  [SMALL_STATE(139)] = 4107,
  [SMALL_STATE(140)] = 4120,
  [SMALL_STATE(141)] = 4133,
  [SMALL_STATE(142)] = 4146,
  [SMALL_STATE(143)] = 4159,
  [SMALL_STATE(144)] = 4170,
  [SMALL_STATE(145)] = 4181,
  [SMALL_STATE(146)] = 4194,
  [SMALL_STATE(147)] = 4205,
  [SMALL_STATE(148)] = 4216,
  [SMALL_STATE(149)] = 4227,
  [SMALL_STATE(150)] = 4240,
  [SMALL_STATE(151)] = 4253,
  [SMALL_STATE(152)] = 4264,
  [SMALL_STATE(153)] = 4277,
  [SMALL_STATE(154)] = 4290,
  [SMALL_STATE(155)] = 4299,
  [SMALL_STATE(156)] = 4308,
  [SMALL_STATE(157)] = 4321,
  [SMALL_STATE(158)] = 4332,
  [SMALL_STATE(159)] = 4345,
  [SMALL_STATE(160)] = 4354,
  [SMALL_STATE(161)] = 4367,
  [SMALL_STATE(162)] = 4380,
  [SMALL_STATE(163)] = 4393,
  [SMALL_STATE(164)] = 4406,
  [SMALL_STATE(165)] = 4414,
  [SMALL_STATE(166)] = 4424,
  [SMALL_STATE(167)] = 4434,
  [SMALL_STATE(168)] = 4444,
  [SMALL_STATE(169)] = 4454,
  [SMALL_STATE(170)] = 4464,
  [SMALL_STATE(171)] = 4472,
  [SMALL_STATE(172)] = 4482,
  [SMALL_STATE(173)] = 4492,
  [SMALL_STATE(174)] = 4500,
  [SMALL_STATE(175)] = 4510,
  [SMALL_STATE(176)] = 4518,
  [SMALL_STATE(177)] = 4526,
  [SMALL_STATE(178)] = 4536,
  [SMALL_STATE(179)] = 4544,
  [SMALL_STATE(180)] = 4552,
  [SMALL_STATE(181)] = 4562,
  [SMALL_STATE(182)] = 4570,
  [SMALL_STATE(183)] = 4580,
  [SMALL_STATE(184)] = 4590,
  [SMALL_STATE(185)] = 4600,
  [SMALL_STATE(186)] = 4610,
  [SMALL_STATE(187)] = 4620,
  [SMALL_STATE(188)] = 4630,
  [SMALL_STATE(189)] = 4640,
  [SMALL_STATE(190)] = 4648,
  [SMALL_STATE(191)] = 4656,
  [SMALL_STATE(192)] = 4666,
  [SMALL_STATE(193)] = 4674,
  [SMALL_STATE(194)] = 4681,
  [SMALL_STATE(195)] = 4688,
  [SMALL_STATE(196)] = 4695,
  [SMALL_STATE(197)] = 4702,
  [SMALL_STATE(198)] = 4709,
  [SMALL_STATE(199)] = 4716,
  [SMALL_STATE(200)] = 4723,
  [SMALL_STATE(201)] = 4730,
  [SMALL_STATE(202)] = 4737,
  [SMALL_STATE(203)] = 4744,
  [SMALL_STATE(204)] = 4751,
  [SMALL_STATE(205)] = 4758,
  [SMALL_STATE(206)] = 4765,
  [SMALL_STATE(207)] = 4772,
  [SMALL_STATE(208)] = 4779,
  [SMALL_STATE(209)] = 4786,
  [SMALL_STATE(210)] = 4793,
  [SMALL_STATE(211)] = 4800,
  [SMALL_STATE(212)] = 4807,
  [SMALL_STATE(213)] = 4814,
  [SMALL_STATE(214)] = 4821,
  [SMALL_STATE(215)] = 4828,
  [SMALL_STATE(216)] = 4835,
  [SMALL_STATE(217)] = 4842,
  [SMALL_STATE(218)] = 4849,
  [SMALL_STATE(219)] = 4856,
  [SMALL_STATE(220)] = 4863,
  [SMALL_STATE(221)] = 4870,
  [SMALL_STATE(222)] = 4877,
  [SMALL_STATE(223)] = 4884,
  [SMALL_STATE(224)] = 4891,
  [SMALL_STATE(225)] = 4898,
  [SMALL_STATE(226)] = 4905,
  [SMALL_STATE(227)] = 4912,
  [SMALL_STATE(228)] = 4919,
  [SMALL_STATE(229)] = 4926,
  [SMALL_STATE(230)] = 4933,
  [SMALL_STATE(231)] = 4940,
  [SMALL_STATE(232)] = 4947,
  [SMALL_STATE(233)] = 4954,
  [SMALL_STATE(234)] = 4961,
  [SMALL_STATE(235)] = 4968,
  [SMALL_STATE(236)] = 4975,
  [SMALL_STATE(237)] = 4982,
  [SMALL_STATE(238)] = 4989,
  [SMALL_STATE(239)] = 4996,
  [SMALL_STATE(240)] = 5003,
  [SMALL_STATE(241)] = 5010,
  [SMALL_STATE(242)] = 5017,
  [SMALL_STATE(243)] = 5024,
  [SMALL_STATE(244)] = 5031,
  [SMALL_STATE(245)] = 5038,
  [SMALL_STATE(246)] = 5045,
  [SMALL_STATE(247)] = 5052,
  [SMALL_STATE(248)] = 5059,
  [SMALL_STATE(249)] = 5066,
  [SMALL_STATE(250)] = 5073,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0, 0, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(243),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(171),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(240),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(238),
  [15] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_boolean, 1, 0, 0),
  [17] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_boolean, 1, 0, 0),
  [19] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 2, 0, 0),
  [21] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 2, 0, 0),
  [23] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 3, 0, 0),
  [25] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 3, 0, 0),
  [27] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 4, 0, 0),
  [29] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 4, 0, 0),
  [31] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_duration_unit, 1, 0, 0),
  [33] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_duration_unit, 1, 0, 0),
  [35] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_duration, 2, 0, 0),
  [37] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_duration, 2, 0, 0),
  [39] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 2, 0, 0),
  [41] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 2, 0, 0),
  [43] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 5, 0, 0),
  [45] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 5, 0, 0),
  [47] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 3, 0, 0),
  [49] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 3, 0, 0),
  [51] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [53] = {.entry = {.count = 1, .reusable = true}}, SHIFT(242),
  [55] = {.entry = {.count = 1, .reusable = true}}, SHIFT(212),
  [57] = {.entry = {.count = 1, .reusable = true}}, SHIFT(210),
  [59] = {.entry = {.count = 1, .reusable = true}}, SHIFT(209),
  [61] = {.entry = {.count = 1, .reusable = true}}, SHIFT(193),
  [63] = {.entry = {.count = 1, .reusable = true}}, SHIFT(207),
  [65] = {.entry = {.count = 1, .reusable = true}}, SHIFT(206),
  [67] = {.entry = {.count = 1, .reusable = false}}, SHIFT(242),
  [69] = {.entry = {.count = 1, .reusable = true}}, SHIFT(170),
  [71] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [73] = {.entry = {.count = 1, .reusable = true}}, SHIFT(201),
  [75] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [77] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [79] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [81] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [83] = {.entry = {.count = 1, .reusable = true}}, SHIFT(73),
  [85] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [87] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [89] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0),
  [91] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(242),
  [94] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(212),
  [97] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(210),
  [100] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(209),
  [103] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(193),
  [106] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(207),
  [109] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(206),
  [112] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(242),
  [115] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(170),
  [118] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0),
  [120] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(242),
  [123] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(201),
  [126] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(76),
  [129] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(71),
  [132] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(75),
  [135] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(74),
  [138] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(73),
  [141] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(242),
  [144] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(170),
  [147] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property, 3, 0, 5),
  [149] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_property, 3, 0, 5),
  [151] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_code_block, 3, 0, 0),
  [153] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_code_block, 3, 0, 0),
  [155] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_code_block, 2, 0, 0),
  [157] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_code_block, 2, 0, 0),
  [159] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_language_implementation, 3, 0, 8),
  [161] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_language_implementation, 3, 0, 8),
  [163] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__value, 1, 0, 0),
  [165] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__value, 1, 0, 0),
  [167] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [169] = {.entry = {.count = 1, .reusable = false}}, SHIFT(6),
  [171] = {.entry = {.count = 1, .reusable = true}}, SHIFT(155),
  [173] = {.entry = {.count = 1, .reusable = true}}, SHIFT(81),
  [175] = {.entry = {.count = 1, .reusable = true}}, SHIFT(245),
  [177] = {.entry = {.count = 1, .reusable = false}}, SHIFT(234),
  [179] = {.entry = {.count = 1, .reusable = true}}, SHIFT(234),
  [181] = {.entry = {.count = 1, .reusable = true}}, SHIFT(167),
  [183] = {.entry = {.count = 1, .reusable = true}}, SHIFT(159),
  [185] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(171),
  [188] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0),
  [190] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(242),
  [193] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(81),
  [196] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(245),
  [199] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(234),
  [202] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(234),
  [205] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(167),
  [208] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(242),
  [211] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_data_property, 3, 0, 0),
  [213] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_data_property, 3, 0, 0),
  [215] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selector_property, 3, 0, 0),
  [217] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_selector_property, 3, 0, 0),
  [219] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_format_property, 3, 0, 0),
  [221] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_format_property, 3, 0, 0),
  [223] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_encoding_property, 3, 0, 0),
  [225] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_encoding_property, 3, 0, 0),
  [227] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_flat, 3, 0, 8),
  [229] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hook_flat, 3, 0, 8),
  [231] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hex_property, 3, 0, 0),
  [233] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hex_property, 3, 0, 0),
  [235] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_grouped, 3, 0, 0),
  [237] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hook_grouped, 3, 0, 0),
  [239] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_shape_property, 3, 0, 0),
  [241] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_shape_property, 3, 0, 0),
  [243] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_each_hook, 2, 0, 0),
  [245] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_each_hook, 2, 0, 0),
  [247] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tags_property, 3, 0, 0),
  [249] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_tags_property, 3, 0, 0),
  [251] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_hook, 2, 0, 0),
  [253] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_hook, 2, 0, 0),
  [255] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_before_hook, 2, 0, 0),
  [257] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_before_hook, 2, 0, 0),
  [259] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_validate_hook, 2, 0, 0),
  [261] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_validate_hook, 2, 0, 0),
  [263] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_skip_hook, 2, 0, 0),
  [265] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_skip_hook, 2, 0, 0),
  [267] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_file_ref, 4, 0, 0),
  [269] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_file_ref, 4, 0, 0),
  [271] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_grouped, 4, 0, 0),
  [273] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hook_grouped, 4, 0, 0),
  [275] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup, 2, 0, 0),
  [277] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup, 2, 0, 0),
  [279] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_body, 2, 0, 0),
  [281] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup_body, 2, 0, 0),
  [283] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_body, 3, 0, 0),
  [285] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup_body, 3, 0, 0),
  [287] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark_body, 3, 0, 0),
  [289] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_benchmark_body, 3, 0, 0),
  [291] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_body, 2, 0, 0),
  [293] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture_body, 2, 0, 0),
  [295] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_body, 3, 0, 0),
  [297] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_body, 3, 0, 0),
  [299] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_body, 3, 0, 0),
  [301] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture_body, 3, 0, 0),
  [303] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture, 4, 0, 1),
  [305] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture, 4, 0, 1),
  [307] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_body, 2, 0, 0),
  [309] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_setup_body, 2, 0, 0),
  [311] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_block, 3, 0, 4),
  [313] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_setup_block, 3, 0, 4),
  [315] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark_body, 2, 0, 0),
  [317] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_benchmark_body, 2, 0, 0),
  [319] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_body, 3, 0, 0),
  [321] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_setup_body, 3, 0, 0),
  [323] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture, 3, 0, 1),
  [325] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture, 3, 0, 1),
  [327] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark, 3, 0, 1),
  [329] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_benchmark, 3, 0, 1),
  [331] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_body, 2, 0, 0),
  [333] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_body, 2, 0, 0),
  [335] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_block, 2, 0, 0),
  [337] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_block, 2, 0, 0),
  [339] = {.entry = {.count = 1, .reusable = true}}, SHIFT(178),
  [341] = {.entry = {.count = 1, .reusable = true}}, SHIFT(225),
  [343] = {.entry = {.count = 1, .reusable = false}}, SHIFT(225),
  [345] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 2, 0, 0),
  [347] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 3, 0, 0),
  [349] = {.entry = {.count = 1, .reusable = false}}, SHIFT(17),
  [351] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [353] = {.entry = {.count = 1, .reusable = true}}, SHIFT(95),
  [355] = {.entry = {.count = 1, .reusable = false}}, SHIFT(22),
  [357] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [359] = {.entry = {.count = 1, .reusable = false}}, SHIFT(2),
  [361] = {.entry = {.count = 1, .reusable = true}}, SHIFT(124),
  [363] = {.entry = {.count = 1, .reusable = false}}, SHIFT(42),
  [365] = {.entry = {.count = 1, .reusable = false}}, SHIFT(179),
  [367] = {.entry = {.count = 1, .reusable = false}}, SHIFT(86),
  [369] = {.entry = {.count = 1, .reusable = true}}, SHIFT(179),
  [371] = {.entry = {.count = 1, .reusable = false}}, SHIFT(21),
  [373] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [375] = {.entry = {.count = 1, .reusable = true}}, SHIFT(186),
  [377] = {.entry = {.count = 1, .reusable = true}}, SHIFT(109),
  [379] = {.entry = {.count = 1, .reusable = true}}, SHIFT(216),
  [381] = {.entry = {.count = 1, .reusable = true}}, SHIFT(187),
  [383] = {.entry = {.count = 1, .reusable = true}}, SHIFT(188),
  [385] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0),
  [387] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(186),
  [390] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(109),
  [393] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(216),
  [396] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(187),
  [399] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(188),
  [402] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [404] = {.entry = {.count = 1, .reusable = false}}, SHIFT(173),
  [406] = {.entry = {.count = 1, .reusable = true}}, SHIFT(173),
  [408] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [410] = {.entry = {.count = 1, .reusable = true}}, SHIFT(244),
  [412] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, 0, 0),
  [414] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [416] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_hook_grouped_repeat1, 2, 0, 0),
  [418] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_hook_grouped_repeat1, 2, 0, 0), SHIFT_REPEAT(170),
  [421] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [423] = {.entry = {.count = 1, .reusable = false}}, SHIFT(191),
  [425] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [427] = {.entry = {.count = 1, .reusable = false}}, SHIFT(208),
  [429] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(191),
  [432] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0),
  [434] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(208),
  [437] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [439] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0),
  [441] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(243),
  [444] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_paren_code_block, 2, 0, 0),
  [446] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_helpers_section, 2, 0, 0),
  [448] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_init_section, 2, 0, 0),
  [450] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_section, 2, 0, 0),
  [452] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_init_section, 3, 0, 0),
  [454] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_paren_code_block, 3, 0, 0),
  [456] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declare_section, 2, 0, 0),
  [458] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat2, 2, 0, 0),
  [460] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat2, 2, 0, 0), SHIFT_REPEAT(240),
  [463] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat2, 2, 0, 0), SHIFT_REPEAT(238),
  [466] = {.entry = {.count = 1, .reusable = true}}, SHIFT(199),
  [468] = {.entry = {.count = 1, .reusable = false}}, SHIFT(3),
  [470] = {.entry = {.count = 1, .reusable = false}}, SHIFT(108),
  [472] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [474] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [476] = {.entry = {.count = 1, .reusable = true}}, SHIFT(190),
  [478] = {.entry = {.count = 1, .reusable = true}}, SHIFT(224),
  [480] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, 0, 0),
  [482] = {.entry = {.count = 1, .reusable = false}}, SHIFT(107),
  [484] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, 0, 0),
  [486] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_use_statement, 4, 0, 2),
  [488] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_content, 1, 0, 0),
  [490] = {.entry = {.count = 1, .reusable = false}}, SHIFT(118),
  [492] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_single_string_content, 1, 0, 0),
  [494] = {.entry = {.count = 1, .reusable = false}}, SHIFT(115),
  [496] = {.entry = {.count = 1, .reusable = true}}, SHIFT(182),
  [498] = {.entry = {.count = 1, .reusable = true}}, SHIFT(169),
  [500] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [502] = {.entry = {.count = 1, .reusable = true}}, SHIFT(229),
  [504] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [506] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [508] = {.entry = {.count = 1, .reusable = false}}, SHIFT(182),
  [510] = {.entry = {.count = 1, .reusable = false}}, SHIFT(30),
  [512] = {.entry = {.count = 1, .reusable = true}}, SHIFT(239),
  [514] = {.entry = {.count = 1, .reusable = true}}, SHIFT(146),
  [516] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_single_string_content_repeat1, 2, 0, 0),
  [518] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_single_string_content_repeat1, 2, 0, 0), SHIFT_REPEAT(115),
  [521] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_after_body_repeat1, 2, 0, 0),
  [523] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_after_body_repeat1, 2, 0, 0), SHIFT_REPEAT(229),
  [526] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [528] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_string_content_repeat1, 2, 0, 0),
  [530] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_string_content_repeat1, 2, 0, 0), SHIFT_REPEAT(118),
  [533] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [535] = {.entry = {.count = 1, .reusable = true}}, SHIFT(125),
  [537] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [539] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [541] = {.entry = {.count = 1, .reusable = true}}, SHIFT(150),
  [543] = {.entry = {.count = 1, .reusable = false}}, SHIFT(20),
  [545] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [547] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 3, 0, 0),
  [549] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 3, 0, 0),
  [551] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite, 3, 0, 1),
  [553] = {.entry = {.count = 1, .reusable = true}}, SHIFT(217),
  [555] = {.entry = {.count = 1, .reusable = true}}, SHIFT(246),
  [557] = {.entry = {.count = 1, .reusable = true}}, SHIFT(163),
  [559] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 1, 0, 0),
  [561] = {.entry = {.count = 1, .reusable = true}}, SHIFT(156),
  [563] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup_statement, 1, 0, 0),
  [565] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_statement, 1, 0, 0),
  [567] = {.entry = {.count = 1, .reusable = true}}, SHIFT(205),
  [569] = {.entry = {.count = 1, .reusable = true}}, SHIFT(132),
  [571] = {.entry = {.count = 1, .reusable = true}}, SHIFT(202),
  [573] = {.entry = {.count = 1, .reusable = true}}, SHIFT(219),
  [575] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fixture_params_repeat1, 2, 0, 0),
  [577] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_params_repeat1, 2, 0, 0), SHIFT_REPEAT(180),
  [580] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_string_array_repeat1, 2, 0, 0), SHIFT_REPEAT(152),
  [583] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_string_array_repeat1, 2, 0, 0),
  [585] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite, 9, 0, 7),
  [587] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite, 8, 0, 6),
  [589] = {.entry = {.count = 1, .reusable = true}}, SHIFT(111),
  [591] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_argument_list_repeat1, 2, 0, 0),
  [593] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_argument_list_repeat1, 2, 0, 0), SHIFT_REPEAT(184),
  [596] = {.entry = {.count = 1, .reusable = true}}, SHIFT(112),
  [598] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [600] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 3, 0, 0),
  [602] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_anvil_call, 6, 0, 0),
  [604] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_call, 6, 0, 0),
  [606] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 6, 0, 0),
  [608] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 6, 0, 0),
  [610] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 5, 0, 0),
  [612] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 5, 0, 0),
  [614] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_anvil_call, 5, 0, 0),
  [616] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_call, 5, 0, 0),
  [618] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 1, 0, 0),
  [620] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [622] = {.entry = {.count = 1, .reusable = true}}, SHIFT(203),
  [624] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 2, 0, 0),
  [626] = {.entry = {.count = 1, .reusable = true}}, SHIFT(141),
  [628] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite, 4, 0, 3),
  [630] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite_body, 2, 0, 0),
  [632] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 4, 0, 0),
  [634] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 4, 0, 0),
  [636] = {.entry = {.count = 1, .reusable = true}}, SHIFT(147),
  [638] = {.entry = {.count = 1, .reusable = true}}, SHIFT(227),
  [640] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite_body, 3, 0, 0),
  [642] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [644] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_chart_params_repeat1, 2, 0, 0),
  [646] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_chart_params_repeat1, 2, 0, 0), SHIFT_REPEAT(62),
  [649] = {.entry = {.count = 1, .reusable = true}}, SHIFT(119),
  [651] = {.entry = {.count = 1, .reusable = true}}, SHIFT(85),
  [653] = {.entry = {.count = 1, .reusable = true}}, SHIFT(196),
  [655] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_language_tag, 1, 0, 0),
  [657] = {.entry = {.count = 1, .reusable = true}}, SHIFT(80),
  [659] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [661] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_param, 3, 0, 5),
  [663] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [665] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_directive, 6, 0, 10),
  [667] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_directive, 5, 0, 10),
  [669] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument, 3, 0, 5),
  [671] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [673] = {.entry = {.count = 1, .reusable = true}}, SHIFT(222),
  [675] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite_type, 1, 0, 0),
  [677] = {.entry = {.count = 1, .reusable = true}}, SHIFT(230),
  [679] = {.entry = {.count = 1, .reusable = true}}, SHIFT(120),
  [681] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_param, 3, 0, 9),
  [683] = {.entry = {.count = 1, .reusable = true}}, SHIFT(121),
  [685] = {.entry = {.count = 1, .reusable = true}}, SHIFT(144),
  [687] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [689] = {.entry = {.count = 1, .reusable = true}}, SHIFT(91),
  [691] = {.entry = {.count = 1, .reusable = true}}, SHIFT(158),
  [693] = {.entry = {.count = 1, .reusable = true}}, SHIFT(145),
  [695] = {.entry = {.count = 1, .reusable = true}}, SHIFT(114),
  [697] = {.entry = {.count = 1, .reusable = true}}, SHIFT(166),
  [699] = {.entry = {.count = 1, .reusable = true}}, SHIFT(213),
  [701] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 2, 0, 0),
  [703] = {.entry = {.count = 1, .reusable = true}}, SHIFT(123),
  [705] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 4, 0, 0),
  [707] = {.entry = {.count = 1, .reusable = true}}, SHIFT(165),
  [709] = {.entry = {.count = 1, .reusable = true}}, SHIFT(160),
  [711] = {.entry = {.count = 1, .reusable = true}}, SHIFT(220),
  [713] = {.entry = {.count = 1, .reusable = true}}, SHIFT(117),
  [715] = {.entry = {.count = 1, .reusable = true}}, SHIFT(94),
  [717] = {.entry = {.count = 1, .reusable = true}}, SHIFT(105),
  [719] = {.entry = {.count = 1, .reusable = true}}, SHIFT(103),
  [721] = {.entry = {.count = 1, .reusable = true}}, SHIFT(192),
  [723] = {.entry = {.count = 1, .reusable = true}}, SHIFT(236),
  [725] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_args, 3, 0, 0),
  [727] = {.entry = {.count = 1, .reusable = true}}, SHIFT(168),
  [729] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_run_mode, 1, 0, 0),
  [731] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [733] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 5, 0, 0),
  [735] = {.entry = {.count = 1, .reusable = true}}, SHIFT(198),
  [737] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [739] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [741] = {.entry = {.count = 1, .reusable = true}}, SHIFT(143),
  [743] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_function_name, 1, 0, 0),
  [745] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_param_name, 1, 0, 0),
  [747] = {.entry = {.count = 1, .reusable = true}}, SHIFT(176),
  [749] = {.entry = {.count = 1, .reusable = true}}, SHIFT(142),
  [751] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [753] = {.entry = {.count = 1, .reusable = true}}, SHIFT(98),
  [755] = {.entry = {.count = 1, .reusable = true}}, SHIFT(200),
  [757] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [759] = {.entry = {.count = 1, .reusable = true}}, SHIFT(97),
  [761] = {.entry = {.count = 1, .reusable = true}}, SHIFT(136),
  [763] = {.entry = {.count = 1, .reusable = true}}, SHIFT(174),
  [765] = {.entry = {.count = 1, .reusable = true}}, SHIFT(211),
  [767] = {.entry = {.count = 1, .reusable = true}}, SHIFT(148),
  [769] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [771] = {.entry = {.count = 1, .reusable = true}}, SHIFT(106),
  [773] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [775] = {.entry = {.count = 1, .reusable = true}}, SHIFT(232),
  [777] = {.entry = {.count = 1, .reusable = true}}, SHIFT(113),
  [779] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property_name, 1, 0, 0),
  [781] = {.entry = {.count = 1, .reusable = true}}, SHIFT(235),
  [783] = {.entry = {.count = 1, .reusable = true}}, SHIFT(79),
  [785] = {.entry = {.count = 1, .reusable = true}}, SHIFT(122),
  [787] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 3, 0, 0),
  [789] = {.entry = {.count = 1, .reusable = true}}, SHIFT(233),
  [791] = {.entry = {.count = 1, .reusable = true}}, SHIFT(157),
  [793] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [795] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
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
