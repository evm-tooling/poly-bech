#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 239
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 184
#define ALIAS_COUNT 0
#define TOKEN_COUNT 105
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
  anon_sym_shape = 30,
  anon_sym_ATfile = 31,
  anon_sym_bench = 32,
  anon_sym_benchAsync = 33,
  anon_sym_tags = 34,
  anon_sym_skip = 35,
  anon_sym_validate = 36,
  anon_sym_before = 37,
  anon_sym_after = 38,
  anon_sym_each = 39,
  anon_sym_charting = 40,
  anon_sym_drawSpeedupChart = 41,
  anon_sym_drawTable = 42,
  anon_sym_drawLineChart = 43,
  anon_sym_drawBarChart = 44,
  anon_sym_title = 45,
  anon_sym_description = 46,
  anon_sym_output = 47,
  anon_sym_sortBy = 48,
  anon_sym_sortOrder = 49,
  anon_sym_baselineBenchmark = 50,
  anon_sym_baseline = 51,
  anon_sym_filterWinner = 52,
  anon_sym_theme = 53,
  anon_sym_width = 54,
  anon_sym_rowCount = 55,
  anon_sym_height = 56,
  anon_sym_limit = 57,
  anon_sym_minSpeedup = 58,
  anon_sym_includeBenchmarks = 59,
  anon_sym_excludeBenchmarks = 60,
  anon_sym_showStdDev = 61,
  anon_sym_showErrorBars = 62,
  anon_sym_showRegression = 63,
  anon_sym_regressionModel = 64,
  anon_sym_yScale = 65,
  anon_sym_iterations = 66,
  anon_sym_warmup = 67,
  anon_sym_timeout = 68,
  anon_sym_requires = 69,
  anon_sym_order = 70,
  anon_sym_mode = 71,
  anon_sym_targetTime = 72,
  anon_sym_sink = 73,
  anon_sym_outlierDetection = 74,
  anon_sym_cvThreshold = 75,
  anon_sym_count = 76,
  anon_sym_fairness = 77,
  anon_sym_fairnessSeed = 78,
  anon_sym_asyncSamplingPolicy = 79,
  anon_sym_asyncWarmupCap = 80,
  anon_sym_asyncSampleCap = 81,
  anon_sym_go = 82,
  anon_sym_ts = 83,
  anon_sym_typescript = 84,
  anon_sym_rust = 85,
  anon_sym_python = 86,
  sym_inline_code = 87,
  anon_sym_DQUOTE = 88,
  anon_sym_SQUOTE = 89,
  aux_sym_string_content_token1 = 90,
  aux_sym_single_string_content_token1 = 91,
  sym_escape_sequence = 92,
  sym_number = 93,
  sym_float = 94,
  anon_sym_ms = 95,
  anon_sym_s = 96,
  anon_sym_m = 97,
  anon_sym_true = 98,
  anon_sym_false = 99,
  anon_sym_LBRACK = 100,
  anon_sym_RBRACK = 101,
  sym_comment = 102,
  sym_embedded_code = 103,
  sym__embedded_code_start = 104,
  sym_source_file = 105,
  sym_use_statement = 106,
  sym_global_setup = 107,
  sym_global_setup_body = 108,
  sym_global_setup_statement = 109,
  sym_anvil_call = 110,
  sym_anvil_args = 111,
  sym_function_call = 112,
  sym_argument_list = 113,
  sym_argument = 114,
  sym_suite = 115,
  sym_suite_type = 116,
  sym_run_mode = 117,
  sym_suite_body = 118,
  sym__suite_item = 119,
  sym_setup_block = 120,
  sym_setup_body = 121,
  sym__setup_section = 122,
  sym_import_section = 123,
  sym_declare_section = 124,
  sym_init_section = 125,
  sym_helpers_section = 126,
  sym_fixture = 127,
  sym_fixture_params = 128,
  sym_fixture_param = 129,
  sym_fixture_body = 130,
  sym__fixture_item = 131,
  sym_hex_property = 132,
  sym_shape_property = 133,
  sym_file_ref = 134,
  sym_benchmark = 135,
  sym_benchmark_body = 136,
  sym__benchmark_item = 137,
  sym_tags_property = 138,
  sym_skip_hook = 139,
  sym_validate_hook = 140,
  sym_before_hook = 141,
  sym_after_hook = 142,
  sym_each_hook = 143,
  sym_hook_flat = 144,
  sym_hook_grouped = 145,
  sym_after_block = 146,
  sym_after_body = 147,
  sym_chart_directive = 148,
  sym_chart_function_name = 149,
  sym_chart_params = 150,
  sym_chart_param = 151,
  sym_chart_param_name = 152,
  sym__chart_value = 153,
  sym_property = 154,
  sym_property_name = 155,
  sym__value = 156,
  sym_language_implementation = 157,
  sym_language_tag = 158,
  sym__code_or_inline = 159,
  sym_code_block = 160,
  sym_paren_code_block = 161,
  sym_string = 162,
  sym_string_content = 163,
  sym_single_string_content = 164,
  sym_duration = 165,
  sym_duration_unit = 166,
  sym_boolean = 167,
  sym_string_array = 168,
  aux_sym_source_file_repeat1 = 169,
  aux_sym_source_file_repeat2 = 170,
  aux_sym_global_setup_body_repeat1 = 171,
  aux_sym_argument_list_repeat1 = 172,
  aux_sym_suite_body_repeat1 = 173,
  aux_sym_setup_body_repeat1 = 174,
  aux_sym_fixture_params_repeat1 = 175,
  aux_sym_fixture_body_repeat1 = 176,
  aux_sym_benchmark_body_repeat1 = 177,
  aux_sym_hook_grouped_repeat1 = 178,
  aux_sym_after_body_repeat1 = 179,
  aux_sym_chart_params_repeat1 = 180,
  aux_sym_string_content_repeat1 = 181,
  aux_sym_single_string_content_repeat1 = 182,
  aux_sym_string_array_repeat1 = 183,
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
  [25] = 25,
  [26] = 20,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 32,
  [33] = 33,
  [34] = 34,
  [35] = 20,
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
  [61] = 60,
  [62] = 60,
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
  [83] = 20,
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
  [237] = 194,
  [238] = 194,
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
      if (lookahead == 'e') ADVANCE(29);
      if (lookahead == 'r') ADVANCE(30);
      END_STATE();
    case 5:
      if (lookahead == 'a') ADVANCE(31);
      if (lookahead == 'x') ADVANCE(32);
      END_STATE();
    case 6:
      if (lookahead == 'a') ADVANCE(33);
      if (lookahead == 'i') ADVANCE(34);
      if (lookahead == 'o') ADVANCE(35);
      END_STATE();
    case 7:
      if (lookahead == 'l') ADVANCE(36);
      if (lookahead == 'o') ADVANCE(37);
      END_STATE();
    case 8:
      if (lookahead == 'e') ADVANCE(38);
      END_STATE();
    case 9:
      if (lookahead == 'm') ADVANCE(39);
      if (lookahead == 'n') ADVANCE(40);
      if (lookahead == 't') ADVANCE(41);
      END_STATE();
    case 10:
      if (lookahead == 'i') ADVANCE(42);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_m);
      if (lookahead == 'e') ADVANCE(43);
      if (lookahead == 'i') ADVANCE(44);
      if (lookahead == 'o') ADVANCE(45);
      if (lookahead == 's') ADVANCE(46);
      END_STATE();
    case 12:
      if (lookahead == 'r') ADVANCE(47);
      if (lookahead == 'u') ADVANCE(48);
      END_STATE();
    case 13:
      if (lookahead == 'e') ADVANCE(49);
      if (lookahead == 'y') ADVANCE(50);
      END_STATE();
    case 14:
      if (lookahead == 'e') ADVANCE(51);
      if (lookahead == 'o') ADVANCE(52);
      if (lookahead == 'u') ADVANCE(53);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_s);
      ADVANCE_MAP(
        'a', 54,
        'e', 55,
        'h', 56,
        'i', 57,
        'k', 58,
        'o', 59,
        'p', 60,
        't', 61,
        'u', 62,
      );
      END_STATE();
    case 16:
      if (lookahead == 'a') ADVANCE(63);
      if (lookahead == 'h') ADVANCE(64);
      if (lookahead == 'i') ADVANCE(65);
      if (lookahead == 'r') ADVANCE(66);
      if (lookahead == 's') ADVANCE(67);
      if (lookahead == 'y') ADVANCE(68);
      END_STATE();
    case 17:
      if (lookahead == 's') ADVANCE(69);
      END_STATE();
    case 18:
      if (lookahead == 'a') ADVANCE(70);
      END_STATE();
    case 19:
      if (lookahead == 'a') ADVANCE(71);
      if (lookahead == 'i') ADVANCE(72);
      END_STATE();
    case 20:
      if (lookahead == 'S') ADVANCE(73);
      END_STATE();
    case 21:
      if (lookahead == 't') ADVANCE(74);
      END_STATE();
    case 22:
      if (lookahead == 'v') ADVANCE(75);
      END_STATE();
    case 23:
      if (lookahead == 'y') ADVANCE(76);
      END_STATE();
    case 24:
      if (lookahead == 's') ADVANCE(77);
      END_STATE();
    case 25:
      if (lookahead == 'f') ADVANCE(78);
      if (lookahead == 'n') ADVANCE(79);
      END_STATE();
    case 26:
      if (lookahead == 'a') ADVANCE(80);
      END_STATE();
    case 27:
      if (lookahead == 'u') ADVANCE(81);
      END_STATE();
    case 28:
      if (lookahead == 'T') ADVANCE(82);
      END_STATE();
    case 29:
      if (lookahead == 'c') ADVANCE(83);
      if (lookahead == 's') ADVANCE(84);
      END_STATE();
    case 30:
      if (lookahead == 'a') ADVANCE(85);
      END_STATE();
    case 31:
      if (lookahead == 'c') ADVANCE(86);
      END_STATE();
    case 32:
      if (lookahead == 'c') ADVANCE(87);
      END_STATE();
    case 33:
      if (lookahead == 'i') ADVANCE(88);
      if (lookahead == 'l') ADVANCE(89);
      END_STATE();
    case 34:
      if (lookahead == 'l') ADVANCE(90);
      if (lookahead == 'x') ADVANCE(91);
      END_STATE();
    case 35:
      if (lookahead == 'r') ADVANCE(92);
      END_STATE();
    case 36:
      if (lookahead == 'o') ADVANCE(93);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(anon_sym_go);
      END_STATE();
    case 38:
      if (lookahead == 'i') ADVANCE(94);
      if (lookahead == 'l') ADVANCE(95);
      if (lookahead == 'x') ADVANCE(96);
      END_STATE();
    case 39:
      if (lookahead == 'p') ADVANCE(97);
      END_STATE();
    case 40:
      if (lookahead == 'c') ADVANCE(98);
      if (lookahead == 'i') ADVANCE(99);
      END_STATE();
    case 41:
      if (lookahead == 'e') ADVANCE(100);
      END_STATE();
    case 42:
      if (lookahead == 'm') ADVANCE(101);
      END_STATE();
    case 43:
      if (lookahead == 'm') ADVANCE(102);
      END_STATE();
    case 44:
      if (lookahead == 'n') ADVANCE(103);
      END_STATE();
    case 45:
      if (lookahead == 'd') ADVANCE(104);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(anon_sym_ms);
      END_STATE();
    case 47:
      if (lookahead == 'd') ADVANCE(105);
      END_STATE();
    case 48:
      if (lookahead == 't') ADVANCE(106);
      END_STATE();
    case 49:
      if (lookahead == 'r') ADVANCE(107);
      END_STATE();
    case 50:
      if (lookahead == 't') ADVANCE(108);
      END_STATE();
    case 51:
      if (lookahead == 'g') ADVANCE(109);
      if (lookahead == 'q') ADVANCE(110);
      END_STATE();
    case 52:
      if (lookahead == 'w') ADVANCE(111);
      END_STATE();
    case 53:
      if (lookahead == 's') ADVANCE(112);
      END_STATE();
    case 54:
      if (lookahead == 'm') ADVANCE(113);
      END_STATE();
    case 55:
      if (lookahead == 't') ADVANCE(114);
      END_STATE();
    case 56:
      if (lookahead == 'a') ADVANCE(115);
      if (lookahead == 'o') ADVANCE(116);
      END_STATE();
    case 57:
      if (lookahead == 'n') ADVANCE(117);
      END_STATE();
    case 58:
      if (lookahead == 'i') ADVANCE(118);
      END_STATE();
    case 59:
      if (lookahead == 'r') ADVANCE(119);
      END_STATE();
    case 60:
      if (lookahead == 'a') ADVANCE(120);
      END_STATE();
    case 61:
      if (lookahead == 'd') ADVANCE(121);
      END_STATE();
    case 62:
      if (lookahead == 'i') ADVANCE(122);
      END_STATE();
    case 63:
      if (lookahead == 'g') ADVANCE(123);
      if (lookahead == 'r') ADVANCE(124);
      END_STATE();
    case 64:
      if (lookahead == 'e') ADVANCE(125);
      END_STATE();
    case 65:
      if (lookahead == 'm') ADVANCE(126);
      if (lookahead == 't') ADVANCE(127);
      END_STATE();
    case 66:
      if (lookahead == 'u') ADVANCE(128);
      END_STATE();
    case 67:
      ACCEPT_TOKEN(anon_sym_ts);
      END_STATE();
    case 68:
      if (lookahead == 'p') ADVANCE(129);
      END_STATE();
    case 69:
      if (lookahead == 'e') ADVANCE(130);
      END_STATE();
    case 70:
      if (lookahead == 'l') ADVANCE(131);
      END_STATE();
    case 71:
      if (lookahead == 'r') ADVANCE(132);
      END_STATE();
    case 72:
      if (lookahead == 'd') ADVANCE(133);
      END_STATE();
    case 73:
      if (lookahead == 'c') ADVANCE(134);
      END_STATE();
    case 74:
      if (lookahead == 'e') ADVANCE(135);
      END_STATE();
    case 75:
      if (lookahead == 'i') ADVANCE(136);
      END_STATE();
    case 76:
      if (lookahead == 'n') ADVANCE(137);
      END_STATE();
    case 77:
      if (lookahead == 'e') ADVANCE(138);
      END_STATE();
    case 78:
      if (lookahead == 'o') ADVANCE(139);
      END_STATE();
    case 79:
      if (lookahead == 'c') ADVANCE(140);
      END_STATE();
    case 80:
      if (lookahead == 'r') ADVANCE(141);
      END_STATE();
    case 81:
      if (lookahead == 'n') ADVANCE(142);
      END_STATE();
    case 82:
      if (lookahead == 'h') ADVANCE(143);
      END_STATE();
    case 83:
      if (lookahead == 'l') ADVANCE(144);
      END_STATE();
    case 84:
      if (lookahead == 'c') ADVANCE(145);
      END_STATE();
    case 85:
      if (lookahead == 'w') ADVANCE(146);
      END_STATE();
    case 86:
      if (lookahead == 'h') ADVANCE(147);
      END_STATE();
    case 87:
      if (lookahead == 'l') ADVANCE(148);
      END_STATE();
    case 88:
      if (lookahead == 'r') ADVANCE(149);
      END_STATE();
    case 89:
      if (lookahead == 's') ADVANCE(150);
      END_STATE();
    case 90:
      if (lookahead == 't') ADVANCE(151);
      END_STATE();
    case 91:
      if (lookahead == 't') ADVANCE(152);
      END_STATE();
    case 92:
      if (lookahead == 'k') ADVANCE(153);
      END_STATE();
    case 93:
      if (lookahead == 'b') ADVANCE(154);
      END_STATE();
    case 94:
      if (lookahead == 'g') ADVANCE(155);
      END_STATE();
    case 95:
      if (lookahead == 'p') ADVANCE(156);
      END_STATE();
    case 96:
      ACCEPT_TOKEN(anon_sym_hex);
      END_STATE();
    case 97:
      if (lookahead == 'o') ADVANCE(157);
      END_STATE();
    case 98:
      if (lookahead == 'l') ADVANCE(158);
      END_STATE();
    case 99:
      if (lookahead == 't') ADVANCE(159);
      END_STATE();
    case 100:
      if (lookahead == 'r') ADVANCE(160);
      END_STATE();
    case 101:
      if (lookahead == 'i') ADVANCE(161);
      END_STATE();
    case 102:
      if (lookahead == 'o') ADVANCE(162);
      END_STATE();
    case 103:
      if (lookahead == 'S') ADVANCE(163);
      END_STATE();
    case 104:
      if (lookahead == 'e') ADVANCE(164);
      END_STATE();
    case 105:
      if (lookahead == 'e') ADVANCE(165);
      END_STATE();
    case 106:
      if (lookahead == 'l') ADVANCE(166);
      if (lookahead == 'p') ADVANCE(167);
      END_STATE();
    case 107:
      if (lookahead == 'f') ADVANCE(168);
      END_STATE();
    case 108:
      if (lookahead == 'h') ADVANCE(169);
      END_STATE();
    case 109:
      if (lookahead == 'r') ADVANCE(170);
      END_STATE();
    case 110:
      if (lookahead == 'u') ADVANCE(171);
      END_STATE();
    case 111:
      if (lookahead == 'C') ADVANCE(172);
      END_STATE();
    case 112:
      if (lookahead == 't') ADVANCE(173);
      END_STATE();
    case 113:
      if (lookahead == 'e') ADVANCE(174);
      END_STATE();
    case 114:
      if (lookahead == 'u') ADVANCE(175);
      END_STATE();
    case 115:
      if (lookahead == 'p') ADVANCE(176);
      END_STATE();
    case 116:
      if (lookahead == 'w') ADVANCE(177);
      END_STATE();
    case 117:
      if (lookahead == 'k') ADVANCE(178);
      END_STATE();
    case 118:
      if (lookahead == 'p') ADVANCE(179);
      END_STATE();
    case 119:
      if (lookahead == 't') ADVANCE(180);
      END_STATE();
    case 120:
      if (lookahead == 'w') ADVANCE(181);
      END_STATE();
    case 121:
      ACCEPT_TOKEN(anon_sym_std);
      END_STATE();
    case 122:
      if (lookahead == 't') ADVANCE(182);
      END_STATE();
    case 123:
      if (lookahead == 's') ADVANCE(183);
      END_STATE();
    case 124:
      if (lookahead == 'g') ADVANCE(184);
      END_STATE();
    case 125:
      if (lookahead == 'm') ADVANCE(185);
      END_STATE();
    case 126:
      if (lookahead == 'e') ADVANCE(186);
      END_STATE();
    case 127:
      if (lookahead == 'l') ADVANCE(187);
      END_STATE();
    case 128:
      if (lookahead == 'e') ADVANCE(188);
      END_STATE();
    case 129:
      if (lookahead == 'e') ADVANCE(189);
      END_STATE();
    case 130:
      ACCEPT_TOKEN(anon_sym_use);
      END_STATE();
    case 131:
      if (lookahead == 'i') ADVANCE(190);
      END_STATE();
    case 132:
      if (lookahead == 'm') ADVANCE(191);
      END_STATE();
    case 133:
      if (lookahead == 't') ADVANCE(192);
      END_STATE();
    case 134:
      if (lookahead == 'a') ADVANCE(193);
      END_STATE();
    case 135:
      if (lookahead == 'r') ADVANCE(194);
      END_STATE();
    case 136:
      if (lookahead == 'l') ADVANCE(195);
      END_STATE();
    case 137:
      if (lookahead == 'c') ADVANCE(196);
      END_STATE();
    case 138:
      if (lookahead == 'l') ADVANCE(197);
      END_STATE();
    case 139:
      if (lookahead == 'r') ADVANCE(198);
      END_STATE();
    case 140:
      if (lookahead == 'h') ADVANCE(199);
      END_STATE();
    case 141:
      if (lookahead == 't') ADVANCE(200);
      END_STATE();
    case 142:
      if (lookahead == 't') ADVANCE(201);
      END_STATE();
    case 143:
      if (lookahead == 'r') ADVANCE(202);
      END_STATE();
    case 144:
      if (lookahead == 'a') ADVANCE(203);
      END_STATE();
    case 145:
      if (lookahead == 'r') ADVANCE(204);
      END_STATE();
    case 146:
      if (lookahead == 'B') ADVANCE(205);
      if (lookahead == 'L') ADVANCE(206);
      if (lookahead == 'S') ADVANCE(207);
      if (lookahead == 'T') ADVANCE(208);
      END_STATE();
    case 147:
      ACCEPT_TOKEN(anon_sym_each);
      END_STATE();
    case 148:
      if (lookahead == 'u') ADVANCE(209);
      END_STATE();
    case 149:
      if (lookahead == 'n') ADVANCE(210);
      END_STATE();
    case 150:
      if (lookahead == 'e') ADVANCE(211);
      END_STATE();
    case 151:
      if (lookahead == 'e') ADVANCE(212);
      END_STATE();
    case 152:
      if (lookahead == 'u') ADVANCE(213);
      END_STATE();
    case 153:
      ACCEPT_TOKEN(anon_sym_fork);
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
      if (lookahead == 'o') ADVANCE(227);
      END_STATE();
    case 170:
      if (lookahead == 'e') ADVANCE(228);
      END_STATE();
    case 171:
      if (lookahead == 'i') ADVANCE(229);
      END_STATE();
    case 172:
      if (lookahead == 'o') ADVANCE(230);
      END_STATE();
    case 173:
      ACCEPT_TOKEN(anon_sym_rust);
      END_STATE();
    case 174:
      if (lookahead == 'D') ADVANCE(231);
      END_STATE();
    case 175:
      if (lookahead == 'p') ADVANCE(232);
      END_STATE();
    case 176:
      if (lookahead == 'e') ADVANCE(233);
      END_STATE();
    case 177:
      if (lookahead == 'E') ADVANCE(234);
      if (lookahead == 'R') ADVANCE(235);
      if (lookahead == 'S') ADVANCE(236);
      END_STATE();
    case 178:
      ACCEPT_TOKEN(anon_sym_sink);
      END_STATE();
    case 179:
      ACCEPT_TOKEN(anon_sym_skip);
      END_STATE();
    case 180:
      if (lookahead == 'B') ADVANCE(237);
      if (lookahead == 'O') ADVANCE(238);
      END_STATE();
    case 181:
      if (lookahead == 'n') ADVANCE(239);
      END_STATE();
    case 182:
      if (lookahead == 'e') ADVANCE(240);
      END_STATE();
    case 183:
      ACCEPT_TOKEN(anon_sym_tags);
      END_STATE();
    case 184:
      if (lookahead == 'e') ADVANCE(241);
      END_STATE();
    case 185:
      if (lookahead == 'e') ADVANCE(242);
      END_STATE();
    case 186:
      if (lookahead == 'B') ADVANCE(243);
      if (lookahead == 'o') ADVANCE(244);
      END_STATE();
    case 187:
      if (lookahead == 'e') ADVANCE(245);
      END_STATE();
    case 188:
      ACCEPT_TOKEN(anon_sym_true);
      END_STATE();
    case 189:
      if (lookahead == 's') ADVANCE(246);
      END_STATE();
    case 190:
      if (lookahead == 'd') ADVANCE(247);
      END_STATE();
    case 191:
      if (lookahead == 'u') ADVANCE(248);
      END_STATE();
    case 192:
      if (lookahead == 'h') ADVANCE(249);
      END_STATE();
    case 193:
      if (lookahead == 'l') ADVANCE(250);
      END_STATE();
    case 194:
      ACCEPT_TOKEN(anon_sym_after);
      END_STATE();
    case 195:
      ACCEPT_TOKEN(anon_sym_anvil);
      END_STATE();
    case 196:
      ACCEPT_TOKEN(anon_sym_async);
      if (lookahead == 'S') ADVANCE(251);
      if (lookahead == 'W') ADVANCE(252);
      END_STATE();
    case 197:
      if (lookahead == 'i') ADVANCE(253);
      END_STATE();
    case 198:
      if (lookahead == 'e') ADVANCE(254);
      END_STATE();
    case 199:
      ACCEPT_TOKEN(anon_sym_bench);
      if (lookahead == 'A') ADVANCE(255);
      END_STATE();
    case 200:
      if (lookahead == 'i') ADVANCE(256);
      END_STATE();
    case 201:
      ACCEPT_TOKEN(anon_sym_count);
      END_STATE();
    case 202:
      if (lookahead == 'e') ADVANCE(257);
      END_STATE();
    case 203:
      if (lookahead == 'r') ADVANCE(258);
      END_STATE();
    case 204:
      if (lookahead == 'i') ADVANCE(259);
      END_STATE();
    case 205:
      if (lookahead == 'a') ADVANCE(260);
      END_STATE();
    case 206:
      if (lookahead == 'i') ADVANCE(261);
      END_STATE();
    case 207:
      if (lookahead == 'p') ADVANCE(262);
      END_STATE();
    case 208:
      if (lookahead == 'a') ADVANCE(263);
      END_STATE();
    case 209:
      if (lookahead == 'd') ADVANCE(264);
      END_STATE();
    case 210:
      if (lookahead == 'e') ADVANCE(265);
      END_STATE();
    case 211:
      ACCEPT_TOKEN(anon_sym_false);
      END_STATE();
    case 212:
      if (lookahead == 'r') ADVANCE(266);
      END_STATE();
    case 213:
      if (lookahead == 'r') ADVANCE(267);
      END_STATE();
    case 214:
      if (lookahead == 'l') ADVANCE(268);
      END_STATE();
    case 215:
      if (lookahead == 't') ADVANCE(269);
      END_STATE();
    case 216:
      if (lookahead == 'r') ADVANCE(270);
      END_STATE();
    case 217:
      if (lookahead == 't') ADVANCE(271);
      END_STATE();
    case 218:
      if (lookahead == 'd') ADVANCE(272);
      END_STATE();
    case 219:
      if (lookahead == 't') ADVANCE(273);
      END_STATE();
    case 220:
      ACCEPT_TOKEN(anon_sym_limit);
      END_STATE();
    case 221:
      if (lookahead == 'y') ADVANCE(274);
      END_STATE();
    case 222:
      if (lookahead == 'e') ADVANCE(275);
      END_STATE();
    case 223:
      ACCEPT_TOKEN(anon_sym_order);
      END_STATE();
    case 224:
      if (lookahead == 'e') ADVANCE(276);
      END_STATE();
    case 225:
      if (lookahead == 't') ADVANCE(277);
      END_STATE();
    case 226:
      if (lookahead == 'r') ADVANCE(278);
      END_STATE();
    case 227:
      if (lookahead == 'n') ADVANCE(279);
      END_STATE();
    case 228:
      if (lookahead == 's') ADVANCE(280);
      END_STATE();
    case 229:
      if (lookahead == 'r') ADVANCE(281);
      END_STATE();
    case 230:
      if (lookahead == 'u') ADVANCE(282);
      END_STATE();
    case 231:
      if (lookahead == 'a') ADVANCE(283);
      END_STATE();
    case 232:
      ACCEPT_TOKEN(anon_sym_setup);
      END_STATE();
    case 233:
      ACCEPT_TOKEN(anon_sym_shape);
      END_STATE();
    case 234:
      if (lookahead == 'r') ADVANCE(284);
      END_STATE();
    case 235:
      if (lookahead == 'e') ADVANCE(285);
      END_STATE();
    case 236:
      if (lookahead == 't') ADVANCE(286);
      END_STATE();
    case 237:
      if (lookahead == 'y') ADVANCE(287);
      END_STATE();
    case 238:
      if (lookahead == 'r') ADVANCE(288);
      END_STATE();
    case 239:
      if (lookahead == 'A') ADVANCE(289);
      END_STATE();
    case 240:
      ACCEPT_TOKEN(anon_sym_suite);
      END_STATE();
    case 241:
      if (lookahead == 't') ADVANCE(290);
      END_STATE();
    case 242:
      ACCEPT_TOKEN(anon_sym_theme);
      END_STATE();
    case 243:
      if (lookahead == 'a') ADVANCE(291);
      END_STATE();
    case 244:
      if (lookahead == 'u') ADVANCE(292);
      END_STATE();
    case 245:
      ACCEPT_TOKEN(anon_sym_title);
      END_STATE();
    case 246:
      if (lookahead == 'c') ADVANCE(293);
      END_STATE();
    case 247:
      if (lookahead == 'a') ADVANCE(294);
      END_STATE();
    case 248:
      if (lookahead == 'p') ADVANCE(295);
      END_STATE();
    case 249:
      ACCEPT_TOKEN(anon_sym_width);
      END_STATE();
    case 250:
      if (lookahead == 'e') ADVANCE(296);
      END_STATE();
    case 251:
      if (lookahead == 'a') ADVANCE(297);
      END_STATE();
    case 252:
      if (lookahead == 'a') ADVANCE(298);
      END_STATE();
    case 253:
      if (lookahead == 'n') ADVANCE(299);
      END_STATE();
    case 254:
      ACCEPT_TOKEN(anon_sym_before);
      END_STATE();
    case 255:
      if (lookahead == 's') ADVANCE(300);
      END_STATE();
    case 256:
      if (lookahead == 'n') ADVANCE(301);
      END_STATE();
    case 257:
      if (lookahead == 's') ADVANCE(302);
      END_STATE();
    case 258:
      if (lookahead == 'e') ADVANCE(303);
      END_STATE();
    case 259:
      if (lookahead == 'p') ADVANCE(304);
      END_STATE();
    case 260:
      if (lookahead == 'r') ADVANCE(305);
      END_STATE();
    case 261:
      if (lookahead == 'n') ADVANCE(306);
      END_STATE();
    case 262:
      if (lookahead == 'e') ADVANCE(307);
      END_STATE();
    case 263:
      if (lookahead == 'b') ADVANCE(308);
      END_STATE();
    case 264:
      if (lookahead == 'e') ADVANCE(309);
      END_STATE();
    case 265:
      if (lookahead == 's') ADVANCE(310);
      END_STATE();
    case 266:
      if (lookahead == 'W') ADVANCE(311);
      END_STATE();
    case 267:
      if (lookahead == 'e') ADVANCE(312);
      END_STATE();
    case 268:
      if (lookahead == 'S') ADVANCE(313);
      END_STATE();
    case 269:
      ACCEPT_TOKEN(anon_sym_height);
      END_STATE();
    case 270:
      if (lookahead == 's') ADVANCE(314);
      END_STATE();
    case 271:
      ACCEPT_TOKEN(anon_sym_import);
      END_STATE();
    case 272:
      if (lookahead == 'e') ADVANCE(315);
      END_STATE();
    case 273:
      if (lookahead == 'i') ADVANCE(316);
      END_STATE();
    case 274:
      ACCEPT_TOKEN(anon_sym_memory);
      END_STATE();
    case 275:
      if (lookahead == 'e') ADVANCE(317);
      END_STATE();
    case 276:
      if (lookahead == 'r') ADVANCE(318);
      END_STATE();
    case 277:
      ACCEPT_TOKEN(anon_sym_output);
      END_STATE();
    case 278:
      if (lookahead == 'm') ADVANCE(319);
      END_STATE();
    case 279:
      ACCEPT_TOKEN(anon_sym_python);
      END_STATE();
    case 280:
      if (lookahead == 's') ADVANCE(320);
      END_STATE();
    case 281:
      if (lookahead == 'e') ADVANCE(321);
      END_STATE();
    case 282:
      if (lookahead == 'n') ADVANCE(322);
      END_STATE();
    case 283:
      if (lookahead == 't') ADVANCE(323);
      END_STATE();
    case 284:
      if (lookahead == 'r') ADVANCE(324);
      END_STATE();
    case 285:
      if (lookahead == 'g') ADVANCE(325);
      END_STATE();
    case 286:
      if (lookahead == 'd') ADVANCE(326);
      END_STATE();
    case 287:
      ACCEPT_TOKEN(anon_sym_sortBy);
      END_STATE();
    case 288:
      if (lookahead == 'd') ADVANCE(327);
      END_STATE();
    case 289:
      if (lookahead == 'n') ADVANCE(328);
      END_STATE();
    case 290:
      if (lookahead == 'T') ADVANCE(329);
      END_STATE();
    case 291:
      if (lookahead == 's') ADVANCE(330);
      END_STATE();
    case 292:
      if (lookahead == 't') ADVANCE(331);
      END_STATE();
    case 293:
      if (lookahead == 'r') ADVANCE(332);
      END_STATE();
    case 294:
      if (lookahead == 't') ADVANCE(333);
      END_STATE();
    case 295:
      ACCEPT_TOKEN(anon_sym_warmup);
      END_STATE();
    case 296:
      ACCEPT_TOKEN(anon_sym_yScale);
      END_STATE();
    case 297:
      if (lookahead == 'm') ADVANCE(334);
      END_STATE();
    case 298:
      if (lookahead == 'r') ADVANCE(335);
      END_STATE();
    case 299:
      if (lookahead == 'e') ADVANCE(336);
      END_STATE();
    case 300:
      if (lookahead == 'y') ADVANCE(337);
      END_STATE();
    case 301:
      if (lookahead == 'g') ADVANCE(338);
      END_STATE();
    case 302:
      if (lookahead == 'h') ADVANCE(339);
      END_STATE();
    case 303:
      ACCEPT_TOKEN(anon_sym_declare);
      END_STATE();
    case 304:
      if (lookahead == 't') ADVANCE(340);
      END_STATE();
    case 305:
      if (lookahead == 'C') ADVANCE(341);
      END_STATE();
    case 306:
      if (lookahead == 'e') ADVANCE(342);
      END_STATE();
    case 307:
      if (lookahead == 'e') ADVANCE(343);
      END_STATE();
    case 308:
      if (lookahead == 'l') ADVANCE(344);
      END_STATE();
    case 309:
      if (lookahead == 'B') ADVANCE(345);
      END_STATE();
    case 310:
      if (lookahead == 's') ADVANCE(346);
      END_STATE();
    case 311:
      if (lookahead == 'i') ADVANCE(347);
      END_STATE();
    case 312:
      ACCEPT_TOKEN(anon_sym_fixture);
      END_STATE();
    case 313:
      if (lookahead == 'e') ADVANCE(348);
      END_STATE();
    case 314:
      ACCEPT_TOKEN(anon_sym_helpers);
      END_STATE();
    case 315:
      if (lookahead == 'B') ADVANCE(349);
      END_STATE();
    case 316:
      if (lookahead == 'o') ADVANCE(350);
      END_STATE();
    case 317:
      if (lookahead == 'd') ADVANCE(351);
      END_STATE();
    case 318:
      if (lookahead == 'D') ADVANCE(352);
      END_STATE();
    case 319:
      if (lookahead == 'a') ADVANCE(353);
      END_STATE();
    case 320:
      if (lookahead == 'i') ADVANCE(354);
      END_STATE();
    case 321:
      if (lookahead == 's') ADVANCE(355);
      END_STATE();
    case 322:
      if (lookahead == 't') ADVANCE(356);
      END_STATE();
    case 323:
      if (lookahead == 'a') ADVANCE(357);
      END_STATE();
    case 324:
      if (lookahead == 'o') ADVANCE(358);
      END_STATE();
    case 325:
      if (lookahead == 'r') ADVANCE(359);
      END_STATE();
    case 326:
      if (lookahead == 'D') ADVANCE(360);
      END_STATE();
    case 327:
      if (lookahead == 'e') ADVANCE(361);
      END_STATE();
    case 328:
      if (lookahead == 'v') ADVANCE(362);
      END_STATE();
    case 329:
      if (lookahead == 'i') ADVANCE(363);
      END_STATE();
    case 330:
      if (lookahead == 'e') ADVANCE(364);
      END_STATE();
    case 331:
      ACCEPT_TOKEN(anon_sym_timeout);
      END_STATE();
    case 332:
      if (lookahead == 'i') ADVANCE(365);
      END_STATE();
    case 333:
      if (lookahead == 'e') ADVANCE(366);
      END_STATE();
    case 334:
      if (lookahead == 'p') ADVANCE(367);
      END_STATE();
    case 335:
      if (lookahead == 'm') ADVANCE(368);
      END_STATE();
    case 336:
      ACCEPT_TOKEN(anon_sym_baseline);
      if (lookahead == 'B') ADVANCE(369);
      END_STATE();
    case 337:
      if (lookahead == 'n') ADVANCE(370);
      END_STATE();
    case 338:
      ACCEPT_TOKEN(anon_sym_charting);
      END_STATE();
    case 339:
      if (lookahead == 'o') ADVANCE(371);
      END_STATE();
    case 340:
      if (lookahead == 'i') ADVANCE(372);
      END_STATE();
    case 341:
      if (lookahead == 'h') ADVANCE(373);
      END_STATE();
    case 342:
      if (lookahead == 'C') ADVANCE(374);
      END_STATE();
    case 343:
      if (lookahead == 'd') ADVANCE(375);
      END_STATE();
    case 344:
      if (lookahead == 'e') ADVANCE(376);
      END_STATE();
    case 345:
      if (lookahead == 'e') ADVANCE(377);
      END_STATE();
    case 346:
      ACCEPT_TOKEN(anon_sym_fairness);
      if (lookahead == 'S') ADVANCE(378);
      END_STATE();
    case 347:
      if (lookahead == 'n') ADVANCE(379);
      END_STATE();
    case 348:
      if (lookahead == 't') ADVANCE(380);
      END_STATE();
    case 349:
      if (lookahead == 'e') ADVANCE(381);
      END_STATE();
    case 350:
      if (lookahead == 'n') ADVANCE(382);
      END_STATE();
    case 351:
      if (lookahead == 'u') ADVANCE(383);
      END_STATE();
    case 352:
      if (lookahead == 'e') ADVANCE(384);
      END_STATE();
    case 353:
      if (lookahead == 'n') ADVANCE(385);
      END_STATE();
    case 354:
      if (lookahead == 'o') ADVANCE(386);
      END_STATE();
    case 355:
      ACCEPT_TOKEN(anon_sym_requires);
      END_STATE();
    case 356:
      ACCEPT_TOKEN(anon_sym_rowCount);
      END_STATE();
    case 357:
      if (lookahead == 's') ADVANCE(387);
      END_STATE();
    case 358:
      if (lookahead == 'r') ADVANCE(388);
      END_STATE();
    case 359:
      if (lookahead == 'e') ADVANCE(389);
      END_STATE();
    case 360:
      if (lookahead == 'e') ADVANCE(390);
      END_STATE();
    case 361:
      if (lookahead == 'r') ADVANCE(391);
      END_STATE();
    case 362:
      if (lookahead == 'i') ADVANCE(392);
      END_STATE();
    case 363:
      if (lookahead == 'm') ADVANCE(393);
      END_STATE();
    case 364:
      if (lookahead == 'd') ADVANCE(394);
      END_STATE();
    case 365:
      if (lookahead == 'p') ADVANCE(395);
      END_STATE();
    case 366:
      ACCEPT_TOKEN(anon_sym_validate);
      END_STATE();
    case 367:
      if (lookahead == 'l') ADVANCE(396);
      END_STATE();
    case 368:
      if (lookahead == 'u') ADVANCE(397);
      END_STATE();
    case 369:
      if (lookahead == 'e') ADVANCE(398);
      END_STATE();
    case 370:
      if (lookahead == 'c') ADVANCE(399);
      END_STATE();
    case 371:
      if (lookahead == 'l') ADVANCE(400);
      END_STATE();
    case 372:
      if (lookahead == 'o') ADVANCE(401);
      END_STATE();
    case 373:
      if (lookahead == 'a') ADVANCE(402);
      END_STATE();
    case 374:
      if (lookahead == 'h') ADVANCE(403);
      END_STATE();
    case 375:
      if (lookahead == 'u') ADVANCE(404);
      END_STATE();
    case 376:
      ACCEPT_TOKEN(anon_sym_drawTable);
      END_STATE();
    case 377:
      if (lookahead == 'n') ADVANCE(405);
      END_STATE();
    case 378:
      if (lookahead == 'e') ADVANCE(406);
      END_STATE();
    case 379:
      if (lookahead == 'n') ADVANCE(407);
      END_STATE();
    case 380:
      if (lookahead == 'u') ADVANCE(408);
      END_STATE();
    case 381:
      if (lookahead == 'n') ADVANCE(409);
      END_STATE();
    case 382:
      if (lookahead == 'B') ADVANCE(410);
      if (lookahead == 's') ADVANCE(411);
      END_STATE();
    case 383:
      if (lookahead == 'p') ADVANCE(412);
      END_STATE();
    case 384:
      if (lookahead == 't') ADVANCE(413);
      END_STATE();
    case 385:
      if (lookahead == 'c') ADVANCE(414);
      END_STATE();
    case 386:
      if (lookahead == 'n') ADVANCE(415);
      END_STATE();
    case 387:
      if (lookahead == 'e') ADVANCE(416);
      END_STATE();
    case 388:
      if (lookahead == 'B') ADVANCE(417);
      END_STATE();
    case 389:
      if (lookahead == 's') ADVANCE(418);
      END_STATE();
    case 390:
      if (lookahead == 'v') ADVANCE(419);
      END_STATE();
    case 391:
      ACCEPT_TOKEN(anon_sym_sortOrder);
      END_STATE();
    case 392:
      if (lookahead == 'l') ADVANCE(420);
      END_STATE();
    case 393:
      if (lookahead == 'e') ADVANCE(421);
      END_STATE();
    case 394:
      ACCEPT_TOKEN(anon_sym_timeBased);
      END_STATE();
    case 395:
      if (lookahead == 't') ADVANCE(422);
      END_STATE();
    case 396:
      if (lookahead == 'e') ADVANCE(423);
      if (lookahead == 'i') ADVANCE(424);
      END_STATE();
    case 397:
      if (lookahead == 'p') ADVANCE(425);
      END_STATE();
    case 398:
      if (lookahead == 'n') ADVANCE(426);
      END_STATE();
    case 399:
      ACCEPT_TOKEN(anon_sym_benchAsync);
      END_STATE();
    case 400:
      if (lookahead == 'd') ADVANCE(427);
      END_STATE();
    case 401:
      if (lookahead == 'n') ADVANCE(428);
      END_STATE();
    case 402:
      if (lookahead == 'r') ADVANCE(429);
      END_STATE();
    case 403:
      if (lookahead == 'a') ADVANCE(430);
      END_STATE();
    case 404:
      if (lookahead == 'p') ADVANCE(431);
      END_STATE();
    case 405:
      if (lookahead == 'c') ADVANCE(432);
      END_STATE();
    case 406:
      if (lookahead == 'e') ADVANCE(433);
      END_STATE();
    case 407:
      if (lookahead == 'e') ADVANCE(434);
      END_STATE();
    case 408:
      if (lookahead == 'p') ADVANCE(435);
      END_STATE();
    case 409:
      if (lookahead == 'c') ADVANCE(436);
      END_STATE();
    case 410:
      if (lookahead == 'a') ADVANCE(437);
      END_STATE();
    case 411:
      ACCEPT_TOKEN(anon_sym_iterations);
      END_STATE();
    case 412:
      ACCEPT_TOKEN(anon_sym_minSpeedup);
      END_STATE();
    case 413:
      if (lookahead == 'e') ADVANCE(438);
      END_STATE();
    case 414:
      if (lookahead == 'e') ADVANCE(439);
      END_STATE();
    case 415:
      if (lookahead == 'M') ADVANCE(440);
      END_STATE();
    case 416:
      if (lookahead == 't') ADVANCE(441);
      END_STATE();
    case 417:
      if (lookahead == 'a') ADVANCE(442);
      END_STATE();
    case 418:
      if (lookahead == 's') ADVANCE(443);
      END_STATE();
    case 419:
      ACCEPT_TOKEN(anon_sym_showStdDev);
      END_STATE();
    case 420:
      ACCEPT_TOKEN(anon_sym_spawnAnvil);
      END_STATE();
    case 421:
      ACCEPT_TOKEN(anon_sym_targetTime);
      END_STATE();
    case 422:
      ACCEPT_TOKEN(anon_sym_typescript);
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
  [94] = {.lex_state = 0},
  [95] = {.lex_state = 0},
  [96] = {.lex_state = 0},
  [97] = {.lex_state = 3},
  [98] = {.lex_state = 1},
  [99] = {.lex_state = 0},
  [100] = {.lex_state = 0},
  [101] = {.lex_state = 0},
  [102] = {.lex_state = 0},
  [103] = {.lex_state = 3},
  [104] = {.lex_state = 0},
  [105] = {.lex_state = 0},
  [106] = {.lex_state = 4},
  [107] = {.lex_state = 0},
  [108] = {.lex_state = 0},
  [109] = {.lex_state = 3},
  [110] = {.lex_state = 1},
  [111] = {.lex_state = 0},
  [112] = {.lex_state = 0},
  [113] = {.lex_state = 0},
  [114] = {.lex_state = 0},
  [115] = {.lex_state = 4},
  [116] = {.lex_state = 1},
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
  [178] = {.lex_state = 0},
  [179] = {.lex_state = 0, .external_lex_state = 2},
  [180] = {.lex_state = 0},
  [181] = {.lex_state = 0, .external_lex_state = 2},
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
  [232] = {.lex_state = 2},
  [233] = {.lex_state = 0},
  [234] = {.lex_state = 0},
  [235] = {.lex_state = 0},
  [236] = {.lex_state = 0},
  [237] = {.lex_state = 0},
  [238] = {.lex_state = 0},
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
    [sym_source_file] = STATE(233),
    [sym_use_statement] = STATE(67),
    [sym_global_setup] = STATE(99),
    [sym_suite] = STATE(92),
    [aux_sym_source_file_repeat1] = STATE(67),
    [aux_sym_source_file_repeat2] = STATE(92),
    [ts_builtin_sym_end] = ACTIONS(5),
    [anon_sym_use] = ACTIONS(7),
    [anon_sym_globalSetup] = ACTIONS(9),
    [anon_sym_declare] = ACTIONS(11),
    [anon_sym_suite] = ACTIONS(13),
    [sym_comment] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 14,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(15), 1,
      anon_sym_RBRACE,
    ACTIONS(19), 1,
      anon_sym_tags,
    ACTIONS(21), 1,
      anon_sym_skip,
    ACTIONS(23), 1,
      anon_sym_validate,
    ACTIONS(25), 1,
      anon_sym_before,
    ACTIONS(27), 1,
      anon_sym_after,
    ACTIONS(29), 1,
      anon_sym_each,
    ACTIONS(31), 1,
      anon_sym_fairness,
    STATE(229), 1,
      sym_language_tag,
    STATE(238), 1,
      sym_property_name,
    ACTIONS(33), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
    STATE(4), 10,
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
    ACTIONS(17), 18,
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
  [73] = 14,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(19), 1,
      anon_sym_tags,
    ACTIONS(21), 1,
      anon_sym_skip,
    ACTIONS(23), 1,
      anon_sym_validate,
    ACTIONS(25), 1,
      anon_sym_before,
    ACTIONS(27), 1,
      anon_sym_after,
    ACTIONS(29), 1,
      anon_sym_each,
    ACTIONS(31), 1,
      anon_sym_fairness,
    ACTIONS(35), 1,
      anon_sym_RBRACE,
    STATE(229), 1,
      sym_language_tag,
    STATE(238), 1,
      sym_property_name,
    ACTIONS(33), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
    STATE(2), 10,
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
    ACTIONS(17), 18,
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
  [146] = 14,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(37), 1,
      anon_sym_RBRACE,
    ACTIONS(42), 1,
      anon_sym_tags,
    ACTIONS(45), 1,
      anon_sym_skip,
    ACTIONS(48), 1,
      anon_sym_validate,
    ACTIONS(51), 1,
      anon_sym_before,
    ACTIONS(54), 1,
      anon_sym_after,
    ACTIONS(57), 1,
      anon_sym_each,
    ACTIONS(60), 1,
      anon_sym_fairness,
    STATE(229), 1,
      sym_language_tag,
    STATE(238), 1,
      sym_property_name,
    ACTIONS(63), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
    STATE(4), 10,
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
    ACTIONS(39), 18,
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
  [219] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(68), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(66), 39,
      anon_sym_globalSetup,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
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
  [268] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(72), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(70), 39,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
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
  [317] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(76), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(74), 39,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
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
  [366] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(80), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(78), 38,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
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
  [414] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(84), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(82), 38,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
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
  [462] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(88), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(86), 38,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
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
  [510] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(92), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(90), 38,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
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
  [558] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(96), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(94), 38,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
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
  [606] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(100), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(98), 38,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
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
  [654] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(104), 2,
      anon_sym_async,
      anon_sym_fairness,
    ACTIONS(102), 36,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_memory,
      anon_sym_import,
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
  [700] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(108), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(106), 36,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_memory,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_hex,
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
  [746] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(112), 2,
      anon_sym_async,
      anon_sym_fairness,
    ACTIONS(110), 36,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_memory,
      anon_sym_import,
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
  [792] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(31), 1,
      anon_sym_fairness,
    ACTIONS(114), 1,
      anon_sym_RBRACE,
    ACTIONS(116), 1,
      anon_sym_hex,
    ACTIONS(118), 1,
      anon_sym_shape,
    STATE(229), 1,
      sym_language_tag,
    STATE(237), 1,
      sym_property_name,
    ACTIONS(33), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
    STATE(19), 6,
      sym__fixture_item,
      sym_hex_property,
      sym_shape_property,
      sym_property,
      sym_language_implementation,
      aux_sym_fixture_body_repeat1,
    ACTIONS(17), 18,
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
  [849] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(120), 1,
      anon_sym_globalSetup,
    ACTIONS(123), 1,
      anon_sym_RBRACE,
    ACTIONS(128), 1,
      anon_sym_setup,
    ACTIONS(131), 1,
      anon_sym_fixture,
    ACTIONS(134), 1,
      anon_sym_bench,
    ACTIONS(137), 1,
      anon_sym_benchAsync,
    ACTIONS(140), 1,
      anon_sym_after,
    ACTIONS(143), 1,
      anon_sym_fairness,
    STATE(194), 1,
      sym_property_name,
    STATE(18), 8,
      sym_global_setup,
      sym__suite_item,
      sym_setup_block,
      sym_fixture,
      sym_benchmark,
      sym_after_block,
      sym_property,
      aux_sym_suite_body_repeat1,
    ACTIONS(125), 18,
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
  [910] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(31), 1,
      anon_sym_fairness,
    ACTIONS(116), 1,
      anon_sym_hex,
    ACTIONS(118), 1,
      anon_sym_shape,
    ACTIONS(146), 1,
      anon_sym_RBRACE,
    STATE(229), 1,
      sym_language_tag,
    STATE(237), 1,
      sym_property_name,
    ACTIONS(33), 5,
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
    ACTIONS(17), 18,
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
  [967] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(150), 1,
      anon_sym_fairness,
    ACTIONS(152), 1,
      anon_sym_ms,
    STATE(9), 1,
      sym_duration_unit,
    ACTIONS(154), 2,
      anon_sym_s,
      anon_sym_m,
    ACTIONS(148), 30,
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
  [1016] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      anon_sym_globalSetup,
    ACTIONS(31), 1,
      anon_sym_fairness,
    ACTIONS(156), 1,
      anon_sym_RBRACE,
    ACTIONS(158), 1,
      anon_sym_setup,
    ACTIONS(160), 1,
      anon_sym_fixture,
    ACTIONS(162), 1,
      anon_sym_bench,
    ACTIONS(164), 1,
      anon_sym_benchAsync,
    ACTIONS(166), 1,
      anon_sym_after,
    STATE(194), 1,
      sym_property_name,
    STATE(18), 8,
      sym_global_setup,
      sym__suite_item,
      sym_setup_block,
      sym_fixture,
      sym_benchmark,
      sym_after_block,
      sym_property,
      aux_sym_suite_body_repeat1,
    ACTIONS(17), 18,
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
  [1077] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      anon_sym_globalSetup,
    ACTIONS(31), 1,
      anon_sym_fairness,
    ACTIONS(158), 1,
      anon_sym_setup,
    ACTIONS(160), 1,
      anon_sym_fixture,
    ACTIONS(162), 1,
      anon_sym_bench,
    ACTIONS(164), 1,
      anon_sym_benchAsync,
    ACTIONS(166), 1,
      anon_sym_after,
    ACTIONS(168), 1,
      anon_sym_RBRACE,
    STATE(194), 1,
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
    ACTIONS(17), 18,
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
  [1138] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(170), 1,
      anon_sym_RBRACE,
    ACTIONS(175), 1,
      anon_sym_hex,
    ACTIONS(178), 1,
      anon_sym_shape,
    ACTIONS(181), 1,
      anon_sym_fairness,
    STATE(229), 1,
      sym_language_tag,
    STATE(237), 1,
      sym_property_name,
    ACTIONS(184), 5,
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
    ACTIONS(172), 18,
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
  [1195] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(189), 1,
      anon_sym_fairness,
    ACTIONS(187), 32,
      anon_sym_RBRACE,
      anon_sym_memory,
      anon_sym_hex,
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
  [1236] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(193), 1,
      anon_sym_fairness,
    ACTIONS(191), 30,
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
  [1275] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(150), 1,
      anon_sym_fairness,
    ACTIONS(152), 1,
      anon_sym_ms,
    STATE(9), 1,
      sym_duration_unit,
    ACTIONS(154), 2,
      anon_sym_s,
      anon_sym_m,
    ACTIONS(148), 26,
      anon_sym_RBRACE,
      anon_sym_memory,
      anon_sym_hex,
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
  [1320] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(197), 1,
      anon_sym_fairness,
    ACTIONS(195), 30,
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
  [1359] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(201), 1,
      anon_sym_fairness,
    ACTIONS(199), 30,
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
  [1398] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(205), 1,
      anon_sym_fairness,
    ACTIONS(203), 30,
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
  [1437] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(209), 1,
      anon_sym_fairness,
    ACTIONS(207), 30,
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
  [1476] = 3,
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
  [1515] = 3,
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
  [1554] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(221), 1,
      anon_sym_fairness,
    ACTIONS(219), 30,
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
  [1593] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(225), 1,
      anon_sym_fairness,
    ACTIONS(223), 30,
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
  [1632] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(152), 1,
      anon_sym_ms,
    STATE(9), 1,
      sym_duration_unit,
    ACTIONS(150), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(154), 2,
      anon_sym_s,
      anon_sym_m,
    ACTIONS(148), 24,
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
  [1676] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(229), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(227), 27,
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
  [1713] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(233), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(231), 27,
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
  [1750] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(237), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(235), 27,
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
  [1787] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(241), 1,
      anon_sym_fairness,
    ACTIONS(239), 26,
      anon_sym_RBRACE,
      anon_sym_memory,
      anon_sym_hex,
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
  [1822] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(245), 1,
      anon_sym_fairness,
    ACTIONS(243), 26,
      anon_sym_RBRACE,
      anon_sym_memory,
      anon_sym_hex,
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
  [1857] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(249), 1,
      anon_sym_fairness,
    ACTIONS(247), 26,
      anon_sym_RBRACE,
      anon_sym_memory,
      anon_sym_hex,
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
  [1892] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(253), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(251), 24,
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
  [1926] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(257), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(255), 24,
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
  [1960] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(261), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(259), 24,
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
  [1994] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(265), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(263), 24,
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
  [2028] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(269), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(267), 24,
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
  [2062] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(273), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(271), 24,
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
  [2096] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(277), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(275), 24,
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
  [2130] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(281), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(279), 24,
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
  [2164] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(285), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(283), 24,
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
  [2232] = 3,
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
  [2266] = 3,
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
  [2300] = 3,
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
  [2334] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(303), 1,
      anon_sym_RPAREN,
    ACTIONS(307), 1,
      anon_sym_baseline,
    STATE(127), 1,
      sym_chart_param,
    STATE(214), 1,
      sym_chart_params,
    STATE(216), 1,
      sym_chart_param_name,
    ACTIONS(305), 20,
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
  [2375] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(307), 1,
      anon_sym_baseline,
    ACTIONS(309), 1,
      anon_sym_RPAREN,
    STATE(162), 1,
      sym_chart_param,
    STATE(216), 1,
      sym_chart_param_name,
    ACTIONS(305), 20,
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
  [2413] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(307), 1,
      anon_sym_baseline,
    ACTIONS(311), 1,
      anon_sym_RPAREN,
    STATE(162), 1,
      sym_chart_param,
    STATE(216), 1,
      sym_chart_param_name,
    ACTIONS(305), 20,
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
  [2451] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(307), 1,
      anon_sym_baseline,
    STATE(162), 1,
      sym_chart_param,
    STATE(216), 1,
      sym_chart_param_name,
    ACTIONS(305), 20,
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
  [2486] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(313), 1,
      sym_identifier,
    ACTIONS(315), 1,
      anon_sym_DQUOTE,
    ACTIONS(317), 1,
      anon_sym_SQUOTE,
    ACTIONS(319), 1,
      sym_number,
    ACTIONS(321), 1,
      sym_float,
    ACTIONS(325), 1,
      anon_sym_LBRACK,
    ACTIONS(323), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(170), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2519] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(315), 1,
      anon_sym_DQUOTE,
    ACTIONS(317), 1,
      anon_sym_SQUOTE,
    ACTIONS(325), 1,
      anon_sym_LBRACK,
    ACTIONS(327), 1,
      sym_identifier,
    ACTIONS(329), 1,
      sym_number,
    ACTIONS(331), 1,
      sym_float,
    ACTIONS(323), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(15), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2552] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(315), 1,
      anon_sym_DQUOTE,
    ACTIONS(317), 1,
      anon_sym_SQUOTE,
    ACTIONS(325), 1,
      anon_sym_LBRACK,
    ACTIONS(327), 1,
      sym_identifier,
    ACTIONS(331), 1,
      sym_float,
    ACTIONS(333), 1,
      sym_number,
    ACTIONS(323), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(15), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2585] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(315), 1,
      anon_sym_DQUOTE,
    ACTIONS(317), 1,
      anon_sym_SQUOTE,
    ACTIONS(325), 1,
      anon_sym_LBRACK,
    ACTIONS(327), 1,
      sym_identifier,
    ACTIONS(331), 1,
      sym_float,
    ACTIONS(335), 1,
      sym_number,
    ACTIONS(323), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(15), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2618] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(337), 1,
      anon_sym_RBRACE,
    ACTIONS(339), 1,
      anon_sym_declare,
    ACTIONS(341), 1,
      anon_sym_import,
    ACTIONS(343), 1,
      anon_sym_async,
    ACTIONS(345), 1,
      anon_sym_init,
    ACTIONS(347), 1,
      anon_sym_helpers,
    STATE(65), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [2648] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(349), 1,
      anon_sym_RBRACE,
    ACTIONS(351), 1,
      anon_sym_declare,
    ACTIONS(354), 1,
      anon_sym_import,
    ACTIONS(357), 1,
      anon_sym_async,
    ACTIONS(360), 1,
      anon_sym_init,
    ACTIONS(363), 1,
      anon_sym_helpers,
    STATE(64), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [2678] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(339), 1,
      anon_sym_declare,
    ACTIONS(341), 1,
      anon_sym_import,
    ACTIONS(343), 1,
      anon_sym_async,
    ACTIONS(345), 1,
      anon_sym_init,
    ACTIONS(347), 1,
      anon_sym_helpers,
    ACTIONS(366), 1,
      anon_sym_RBRACE,
    STATE(64), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [2708] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(315), 1,
      anon_sym_DQUOTE,
    ACTIONS(317), 1,
      anon_sym_SQUOTE,
    ACTIONS(325), 1,
      anon_sym_LBRACK,
    ACTIONS(368), 1,
      sym_number,
    ACTIONS(370), 1,
      sym_float,
    ACTIONS(372), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(161), 4,
      sym__chart_value,
      sym_string,
      sym_boolean,
      sym_string_array,
  [2737] = 9,
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
    ACTIONS(374), 1,
      ts_builtin_sym_end,
    STATE(94), 1,
      sym_global_setup,
    STATE(79), 2,
      sym_use_statement,
      aux_sym_source_file_repeat1,
    STATE(96), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [2767] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(376), 1,
      anon_sym_RBRACE,
    STATE(229), 1,
      sym_language_tag,
    STATE(68), 2,
      sym_language_implementation,
      aux_sym_hook_grouped_repeat1,
    ACTIONS(378), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2788] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(381), 1,
      anon_sym_COLON,
    STATE(225), 1,
      sym_language_tag,
    STATE(25), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(33), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2809] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(383), 1,
      anon_sym_RBRACE,
    STATE(229), 1,
      sym_language_tag,
    STATE(75), 2,
      sym_language_implementation,
      aux_sym_hook_grouped_repeat1,
    ACTIONS(33), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2830] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(381), 1,
      anon_sym_COLON,
    STATE(225), 1,
      sym_language_tag,
    STATE(33), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(33), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2851] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(381), 1,
      anon_sym_COLON,
    STATE(225), 1,
      sym_language_tag,
    STATE(29), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(33), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2872] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(381), 1,
      anon_sym_COLON,
    STATE(225), 1,
      sym_language_tag,
    STATE(32), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(33), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2893] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(381), 1,
      anon_sym_COLON,
    STATE(225), 1,
      sym_language_tag,
    STATE(34), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(33), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2914] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(385), 1,
      anon_sym_RBRACE,
    STATE(229), 1,
      sym_language_tag,
    STATE(68), 2,
      sym_language_implementation,
      aux_sym_hook_grouped_repeat1,
    ACTIONS(33), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2935] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(387), 1,
      sym_identifier,
    ACTIONS(389), 1,
      anon_sym_RBRACE,
    ACTIONS(391), 1,
      anon_sym_anvil,
    STATE(77), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(122), 2,
      sym_anvil_call,
      sym_function_call,
  [2956] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(387), 1,
      sym_identifier,
    ACTIONS(391), 1,
      anon_sym_anvil,
    ACTIONS(393), 1,
      anon_sym_RBRACE,
    STATE(78), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(122), 2,
      sym_anvil_call,
      sym_function_call,
  [2977] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(395), 1,
      sym_identifier,
    ACTIONS(398), 1,
      anon_sym_RBRACE,
    ACTIONS(400), 1,
      anon_sym_anvil,
    STATE(78), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(122), 2,
      sym_anvil_call,
      sym_function_call,
  [2998] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(405), 1,
      anon_sym_use,
    STATE(79), 2,
      sym_use_statement,
      aux_sym_source_file_repeat1,
    ACTIONS(403), 4,
      ts_builtin_sym_end,
      anon_sym_globalSetup,
      anon_sym_declare,
      anon_sym_suite,
  [3015] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(408), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3027] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(410), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3039] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(412), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3051] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(154), 1,
      anon_sym_m,
    STATE(9), 1,
      sym_duration_unit,
    ACTIONS(148), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
    ACTIONS(152), 2,
      anon_sym_ms,
      anon_sym_s,
  [3069] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(167), 1,
      sym_language_tag,
    ACTIONS(33), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [3083] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(414), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3095] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(416), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3107] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(418), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3119] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(420), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3131] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(422), 1,
      ts_builtin_sym_end,
    ACTIONS(424), 1,
      anon_sym_declare,
    ACTIONS(427), 1,
      anon_sym_suite,
    STATE(89), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3148] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(186), 1,
      sym_chart_function_name,
    ACTIONS(430), 4,
      anon_sym_drawSpeedupChart,
      anon_sym_drawTable,
      anon_sym_drawLineChart,
      anon_sym_drawBarChart,
  [3161] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(432), 1,
      anon_sym_LBRACE,
    STATE(118), 1,
      sym_suite_type,
    STATE(134), 1,
      sym_suite_body,
    ACTIONS(434), 2,
      anon_sym_performance,
      anon_sym_memory,
  [3178] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_declare,
    ACTIONS(13), 1,
      anon_sym_suite,
    ACTIONS(374), 1,
      ts_builtin_sym_end,
    STATE(89), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3195] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(315), 1,
      anon_sym_DQUOTE,
    ACTIONS(317), 1,
      anon_sym_SQUOTE,
    ACTIONS(436), 1,
      anon_sym_ATfile,
    STATE(40), 2,
      sym_file_ref,
      sym_string,
  [3212] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_declare,
    ACTIONS(13), 1,
      anon_sym_suite,
    ACTIONS(438), 1,
      ts_builtin_sym_end,
    STATE(101), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3229] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(432), 1,
      anon_sym_LBRACE,
    STATE(123), 1,
      sym_suite_type,
    STATE(126), 1,
      sym_suite_body,
    ACTIONS(434), 2,
      anon_sym_performance,
      anon_sym_memory,
  [3246] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_declare,
    ACTIONS(13), 1,
      anon_sym_suite,
    ACTIONS(438), 1,
      ts_builtin_sym_end,
    STATE(89), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3263] = 5,
    ACTIONS(440), 1,
      anon_sym_SQUOTE,
    ACTIONS(444), 1,
      sym_comment,
    STATE(103), 1,
      aux_sym_single_string_content_repeat1,
    STATE(204), 1,
      sym_single_string_content,
    ACTIONS(442), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [3280] = 5,
    ACTIONS(440), 1,
      anon_sym_DQUOTE,
    ACTIONS(444), 1,
      sym_comment,
    STATE(116), 1,
      aux_sym_string_content_repeat1,
    STATE(206), 1,
      sym_string_content,
    ACTIONS(446), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [3297] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_declare,
    ACTIONS(13), 1,
      anon_sym_suite,
    ACTIONS(374), 1,
      ts_builtin_sym_end,
    STATE(96), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3314] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(448), 5,
      ts_builtin_sym_end,
      anon_sym_use,
      anon_sym_globalSetup,
      anon_sym_declare,
      anon_sym_suite,
  [3325] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_declare,
    ACTIONS(13), 1,
      anon_sym_suite,
    ACTIONS(450), 1,
      ts_builtin_sym_end,
    STATE(89), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3342] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(452), 1,
      anon_sym_RBRACE,
    ACTIONS(454), 1,
      anon_sym_charting,
    STATE(112), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [3356] = 4,
    ACTIONS(444), 1,
      sym_comment,
    ACTIONS(456), 1,
      anon_sym_SQUOTE,
    STATE(109), 1,
      aux_sym_single_string_content_repeat1,
    ACTIONS(458), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [3370] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(460), 1,
      sym_identifier,
    ACTIONS(462), 1,
      anon_sym_RPAREN,
    STATE(149), 1,
      sym_argument,
    STATE(221), 1,
      sym_argument_list,
  [3386] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(315), 1,
      anon_sym_DQUOTE,
    ACTIONS(317), 1,
      anon_sym_SQUOTE,
    ACTIONS(464), 1,
      anon_sym_RBRACK,
    STATE(175), 1,
      sym_string,
  [3402] = 4,
    ACTIONS(444), 1,
      sym_comment,
    ACTIONS(466), 1,
      anon_sym_LBRACE,
    ACTIONS(468), 1,
      sym_inline_code,
    STATE(28), 2,
      sym__code_or_inline,
      sym_code_block,
  [3416] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(315), 1,
      anon_sym_DQUOTE,
    ACTIONS(317), 1,
      anon_sym_SQUOTE,
    ACTIONS(470), 1,
      anon_sym_RBRACK,
    STATE(175), 1,
      sym_string,
  [3432] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(472), 1,
      anon_sym_LBRACE,
    ACTIONS(474), 1,
      anon_sym_LPAREN,
    STATE(80), 2,
      sym_code_block,
      sym_paren_code_block,
  [3446] = 4,
    ACTIONS(444), 1,
      sym_comment,
    ACTIONS(476), 1,
      anon_sym_SQUOTE,
    STATE(109), 1,
      aux_sym_single_string_content_repeat1,
    ACTIONS(478), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [3460] = 4,
    ACTIONS(444), 1,
      sym_comment,
    ACTIONS(481), 1,
      anon_sym_DQUOTE,
    STATE(110), 1,
      aux_sym_string_content_repeat1,
    ACTIONS(483), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [3474] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(486), 1,
      anon_sym_LBRACE,
    ACTIONS(488), 1,
      anon_sym_LPAREN,
    STATE(48), 1,
      sym_fixture_body,
    STATE(173), 1,
      sym_fixture_params,
  [3490] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(454), 1,
      anon_sym_charting,
    ACTIONS(490), 1,
      anon_sym_RBRACE,
    STATE(113), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [3504] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(492), 1,
      anon_sym_RBRACE,
    ACTIONS(494), 1,
      anon_sym_charting,
    STATE(113), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [3518] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(315), 1,
      anon_sym_DQUOTE,
    ACTIONS(317), 1,
      anon_sym_SQUOTE,
    ACTIONS(497), 1,
      anon_sym_RBRACK,
    STATE(138), 1,
      sym_string,
  [3534] = 4,
    ACTIONS(444), 1,
      sym_comment,
    ACTIONS(466), 1,
      anon_sym_LBRACE,
    ACTIONS(499), 1,
      sym_inline_code,
    STATE(24), 2,
      sym__code_or_inline,
      sym_code_block,
  [3548] = 4,
    ACTIONS(444), 1,
      sym_comment,
    ACTIONS(501), 1,
      anon_sym_DQUOTE,
    STATE(110), 1,
      aux_sym_string_content_repeat1,
    ACTIONS(503), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [3562] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(460), 1,
      sym_identifier,
    ACTIONS(505), 1,
      anon_sym_RPAREN,
    STATE(149), 1,
      sym_argument,
    STATE(215), 1,
      sym_argument_list,
  [3578] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(222), 1,
      sym_run_mode,
    ACTIONS(507), 2,
      anon_sym_timeBased,
      anon_sym_iterationBased,
  [3589] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(509), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [3598] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(511), 1,
      anon_sym_RPAREN,
    ACTIONS(513), 1,
      anon_sym_COMMA,
    STATE(120), 1,
      aux_sym_chart_params_repeat1,
  [3611] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(311), 1,
      anon_sym_RPAREN,
    ACTIONS(516), 1,
      anon_sym_COMMA,
    STATE(120), 1,
      aux_sym_chart_params_repeat1,
  [3624] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(520), 1,
      anon_sym_RBRACE,
    ACTIONS(518), 2,
      anon_sym_anvil,
      sym_identifier,
  [3635] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(192), 1,
      sym_run_mode,
    ACTIONS(507), 2,
      anon_sym_timeBased,
      anon_sym_iterationBased,
  [3646] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(522), 1,
      anon_sym_RPAREN,
    ACTIONS(524), 1,
      anon_sym_COMMA,
    STATE(155), 1,
      aux_sym_fixture_params_repeat1,
  [3659] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(315), 1,
      anon_sym_DQUOTE,
    ACTIONS(317), 1,
      anon_sym_SQUOTE,
    STATE(175), 1,
      sym_string,
  [3672] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(526), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [3681] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(528), 1,
      anon_sym_RPAREN,
    ACTIONS(530), 1,
      anon_sym_COMMA,
    STATE(121), 1,
      aux_sym_chart_params_repeat1,
  [3694] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(532), 1,
      anon_sym_RPAREN,
    ACTIONS(534), 1,
      anon_sym_COMMA,
    STATE(128), 1,
      aux_sym_argument_list_repeat1,
  [3707] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(539), 1,
      anon_sym_RBRACE,
    ACTIONS(537), 2,
      anon_sym_anvil,
      sym_identifier,
  [3718] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(470), 1,
      anon_sym_RBRACK,
    ACTIONS(541), 1,
      anon_sym_COMMA,
    STATE(135), 1,
      aux_sym_string_array_repeat1,
  [3731] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(315), 1,
      anon_sym_DQUOTE,
    ACTIONS(317), 1,
      anon_sym_SQUOTE,
    STATE(219), 1,
      sym_string,
  [3744] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(545), 1,
      anon_sym_RBRACE,
    ACTIONS(543), 2,
      anon_sym_anvil,
      sym_identifier,
  [3755] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(176), 1,
      sym_boolean,
    ACTIONS(372), 2,
      anon_sym_true,
      anon_sym_false,
  [3766] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(547), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [3775] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(549), 1,
      anon_sym_COMMA,
    ACTIONS(552), 1,
      anon_sym_RBRACK,
    STATE(135), 1,
      aux_sym_string_array_repeat1,
  [3788] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(183), 1,
      sym_boolean,
    ACTIONS(372), 2,
      anon_sym_true,
      anon_sym_false,
  [3799] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(554), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [3808] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(556), 1,
      anon_sym_COMMA,
    ACTIONS(558), 1,
      anon_sym_RBRACK,
    STATE(130), 1,
      aux_sym_string_array_repeat1,
  [3821] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(560), 1,
      anon_sym_RPAREN,
    ACTIONS(562), 1,
      anon_sym_COMMA,
    STATE(139), 1,
      aux_sym_fixture_params_repeat1,
  [3834] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(315), 1,
      anon_sym_DQUOTE,
    ACTIONS(317), 1,
      anon_sym_SQUOTE,
    STATE(202), 1,
      sym_string,
  [3847] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(567), 1,
      anon_sym_RBRACE,
    ACTIONS(565), 2,
      anon_sym_anvil,
      sym_identifier,
  [3858] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(571), 1,
      anon_sym_RBRACE,
    ACTIONS(569), 2,
      anon_sym_anvil,
      sym_identifier,
  [3869] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(573), 1,
      sym_identifier,
    ACTIONS(575), 1,
      anon_sym_RPAREN,
    STATE(182), 1,
      sym_fixture_param,
  [3882] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(577), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [3891] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(579), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [3900] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(573), 1,
      sym_identifier,
    ACTIONS(581), 1,
      anon_sym_RPAREN,
    STATE(124), 1,
      sym_fixture_param,
  [3913] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(585), 1,
      anon_sym_RBRACE,
    ACTIONS(583), 2,
      anon_sym_anvil,
      sym_identifier,
  [3924] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(460), 1,
      sym_identifier,
    ACTIONS(587), 1,
      anon_sym_RPAREN,
    STATE(172), 1,
      sym_argument,
  [3937] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(589), 1,
      anon_sym_RPAREN,
    ACTIONS(591), 1,
      anon_sym_COMMA,
    STATE(150), 1,
      aux_sym_argument_list_repeat1,
  [3950] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(593), 1,
      anon_sym_RPAREN,
    ACTIONS(595), 1,
      anon_sym_COMMA,
    STATE(128), 1,
      aux_sym_argument_list_repeat1,
  [3963] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(460), 1,
      sym_identifier,
    ACTIONS(593), 1,
      anon_sym_RPAREN,
    STATE(172), 1,
      sym_argument,
  [3976] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(599), 1,
      anon_sym_RBRACE,
    ACTIONS(597), 2,
      anon_sym_anvil,
      sym_identifier,
  [3987] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(601), 1,
      anon_sym_RPAREN,
    ACTIONS(603), 1,
      anon_sym_fork,
    STATE(209), 1,
      sym_anvil_args,
  [4000] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(573), 1,
      sym_identifier,
    ACTIONS(605), 1,
      anon_sym_RPAREN,
    STATE(182), 1,
      sym_fixture_param,
  [4013] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(605), 1,
      anon_sym_RPAREN,
    ACTIONS(607), 1,
      anon_sym_COMMA,
    STATE(139), 1,
      aux_sym_fixture_params_repeat1,
  [4026] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(573), 1,
      sym_identifier,
    STATE(182), 1,
      sym_fixture_param,
  [4036] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(609), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4044] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(472), 1,
      anon_sym_LBRACE,
    STATE(85), 1,
      sym_code_block,
  [4054] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(472), 1,
      anon_sym_LBRACE,
    STATE(39), 1,
      sym_code_block,
  [4064] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(611), 1,
      anon_sym_LBRACE,
    STATE(38), 1,
      sym_global_setup_body,
  [4074] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(613), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4082] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(511), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4090] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(325), 1,
      anon_sym_LBRACK,
    STATE(31), 1,
      sym_string_array,
  [4100] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(615), 1,
      anon_sym_DOT,
    ACTIONS(617), 1,
      anon_sym_LPAREN,
  [4110] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(619), 1,
      anon_sym_LBRACE,
    STATE(47), 1,
      sym_benchmark_body,
  [4120] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(621), 2,
      anon_sym_RBRACE,
      anon_sym_charting,
  [4128] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(623), 1,
      anon_sym_LBRACE,
    STATE(49), 1,
      sym_setup_body,
  [4138] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(625), 2,
      anon_sym_LBRACE,
      anon_sym_COLON,
  [4146] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(627), 2,
      anon_sym_timeBased,
      anon_sym_iterationBased,
  [4154] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(629), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4162] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(631), 2,
      anon_sym_RBRACE,
      anon_sym_charting,
  [4170] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(532), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4178] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(486), 1,
      anon_sym_LBRACE,
    STATE(45), 1,
      sym_fixture_body,
  [4188] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(472), 1,
      anon_sym_LBRACE,
    STATE(86), 1,
      sym_code_block,
  [4198] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(552), 2,
      anon_sym_COMMA,
      anon_sym_RBRACK,
  [4206] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(432), 1,
      anon_sym_LBRACE,
    STATE(144), 1,
      sym_suite_body,
  [4216] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(472), 1,
      anon_sym_LBRACE,
    STATE(88), 1,
      sym_code_block,
  [4226] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(460), 1,
      sym_identifier,
    STATE(172), 1,
      sym_argument,
  [4236] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(633), 1,
      anon_sym_RBRACE,
    ACTIONS(635), 1,
      sym_embedded_code,
  [4246] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(472), 1,
      anon_sym_LBRACE,
    STATE(87), 1,
      sym_code_block,
  [4256] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(637), 1,
      anon_sym_RPAREN,
    ACTIONS(639), 1,
      sym_embedded_code,
  [4266] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(560), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4274] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(432), 1,
      anon_sym_LBRACE,
    STATE(145), 1,
      sym_suite_body,
  [4284] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(641), 1,
      anon_sym_LBRACE,
    STATE(43), 1,
      sym_after_body,
  [4294] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(643), 1,
      sym_identifier,
  [4301] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(645), 1,
      anon_sym_LPAREN,
  [4308] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(647), 1,
      anon_sym_LPAREN,
  [4315] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(649), 1,
      anon_sym_RBRACE,
  [4322] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(651), 1,
      anon_sym_LPAREN,
  [4329] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(653), 1,
      anon_sym_RPAREN,
  [4336] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(655), 1,
      anon_sym_LPAREN,
  [4343] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(657), 1,
      anon_sym_sameDataset,
  [4350] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(659), 1,
      anon_sym_sameDataset,
  [4357] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(661), 1,
      anon_sym_COLON,
  [4364] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(663), 1,
      anon_sym_LBRACE,
  [4371] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(665), 1,
      anon_sym_COLON,
  [4378] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(667), 1,
      sym_identifier,
  [4385] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(669), 1,
      anon_sym_COLON,
  [4392] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(671), 1,
      anon_sym_COLON,
  [4399] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(673), 1,
      anon_sym_COLON,
  [4406] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(675), 1,
      anon_sym_init,
  [4413] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(677), 1,
      anon_sym_RPAREN,
  [4420] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(679), 1,
      anon_sym_DOT,
  [4427] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(681), 1,
      anon_sym_SQUOTE,
  [4434] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(683), 1,
      anon_sym_COLON,
  [4441] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(681), 1,
      anon_sym_DQUOTE,
  [4448] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(685), 1,
      anon_sym_LBRACE,
  [4455] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(687), 1,
      anon_sym_COLON,
  [4462] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(689), 1,
      anon_sym_RPAREN,
  [4469] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(691), 1,
      sym_identifier,
  [4476] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(693), 1,
      anon_sym_spawnAnvil,
  [4483] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(695), 1,
      anon_sym_COLON,
  [4490] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(697), 1,
      anon_sym_COLON,
  [4497] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(699), 1,
      anon_sym_RPAREN,
  [4504] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(701), 1,
      anon_sym_RPAREN,
  [4511] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(703), 1,
      anon_sym_COLON,
  [4518] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(705), 1,
      anon_sym_LPAREN,
  [4525] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(707), 1,
      anon_sym_std,
  [4532] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(709), 1,
      anon_sym_RPAREN,
  [4539] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(711), 1,
      anon_sym_COLON,
  [4546] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(713), 1,
      anon_sym_RPAREN,
  [4553] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(715), 1,
      anon_sym_sameDataset,
  [4560] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(717), 1,
      anon_sym_COLON,
  [4567] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(719), 1,
      anon_sym_LBRACE,
  [4574] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(721), 1,
      anon_sym_COLON,
  [4581] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(723), 1,
      anon_sym_DOT,
  [4588] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(725), 1,
      anon_sym_LBRACE,
  [4595] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(727), 1,
      sym_identifier,
  [4602] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(729), 1,
      anon_sym_COLON,
  [4609] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(731), 1,
      anon_sym_LBRACE,
  [4616] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(733), 1,
      sym_identifier,
  [4623] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(735), 1,
      anon_sym_COLON_COLON,
  [4630] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(737), 1,
      ts_builtin_sym_end,
  [4637] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(739), 1,
      sym_identifier,
  [4644] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(741), 1,
      anon_sym_suite,
  [4651] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(743), 1,
      sym_identifier,
  [4658] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(745), 1,
      anon_sym_COLON,
  [4665] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(747), 1,
      anon_sym_COLON,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 73,
  [SMALL_STATE(4)] = 146,
  [SMALL_STATE(5)] = 219,
  [SMALL_STATE(6)] = 268,
  [SMALL_STATE(7)] = 317,
  [SMALL_STATE(8)] = 366,
  [SMALL_STATE(9)] = 414,
  [SMALL_STATE(10)] = 462,
  [SMALL_STATE(11)] = 510,
  [SMALL_STATE(12)] = 558,
  [SMALL_STATE(13)] = 606,
  [SMALL_STATE(14)] = 654,
  [SMALL_STATE(15)] = 700,
  [SMALL_STATE(16)] = 746,
  [SMALL_STATE(17)] = 792,
  [SMALL_STATE(18)] = 849,
  [SMALL_STATE(19)] = 910,
  [SMALL_STATE(20)] = 967,
  [SMALL_STATE(21)] = 1016,
  [SMALL_STATE(22)] = 1077,
  [SMALL_STATE(23)] = 1138,
  [SMALL_STATE(24)] = 1195,
  [SMALL_STATE(25)] = 1236,
  [SMALL_STATE(26)] = 1275,
  [SMALL_STATE(27)] = 1320,
  [SMALL_STATE(28)] = 1359,
  [SMALL_STATE(29)] = 1398,
  [SMALL_STATE(30)] = 1437,
  [SMALL_STATE(31)] = 1476,
  [SMALL_STATE(32)] = 1515,
  [SMALL_STATE(33)] = 1554,
  [SMALL_STATE(34)] = 1593,
  [SMALL_STATE(35)] = 1632,
  [SMALL_STATE(36)] = 1676,
  [SMALL_STATE(37)] = 1713,
  [SMALL_STATE(38)] = 1750,
  [SMALL_STATE(39)] = 1787,
  [SMALL_STATE(40)] = 1822,
  [SMALL_STATE(41)] = 1857,
  [SMALL_STATE(42)] = 1892,
  [SMALL_STATE(43)] = 1926,
  [SMALL_STATE(44)] = 1960,
  [SMALL_STATE(45)] = 1994,
  [SMALL_STATE(46)] = 2028,
  [SMALL_STATE(47)] = 2062,
  [SMALL_STATE(48)] = 2096,
  [SMALL_STATE(49)] = 2130,
  [SMALL_STATE(50)] = 2164,
  [SMALL_STATE(51)] = 2198,
  [SMALL_STATE(52)] = 2232,
  [SMALL_STATE(53)] = 2266,
  [SMALL_STATE(54)] = 2300,
  [SMALL_STATE(55)] = 2334,
  [SMALL_STATE(56)] = 2375,
  [SMALL_STATE(57)] = 2413,
  [SMALL_STATE(58)] = 2451,
  [SMALL_STATE(59)] = 2486,
  [SMALL_STATE(60)] = 2519,
  [SMALL_STATE(61)] = 2552,
  [SMALL_STATE(62)] = 2585,
  [SMALL_STATE(63)] = 2618,
  [SMALL_STATE(64)] = 2648,
  [SMALL_STATE(65)] = 2678,
  [SMALL_STATE(66)] = 2708,
  [SMALL_STATE(67)] = 2737,
  [SMALL_STATE(68)] = 2767,
  [SMALL_STATE(69)] = 2788,
  [SMALL_STATE(70)] = 2809,
  [SMALL_STATE(71)] = 2830,
  [SMALL_STATE(72)] = 2851,
  [SMALL_STATE(73)] = 2872,
  [SMALL_STATE(74)] = 2893,
  [SMALL_STATE(75)] = 2914,
  [SMALL_STATE(76)] = 2935,
  [SMALL_STATE(77)] = 2956,
  [SMALL_STATE(78)] = 2977,
  [SMALL_STATE(79)] = 2998,
  [SMALL_STATE(80)] = 3015,
  [SMALL_STATE(81)] = 3027,
  [SMALL_STATE(82)] = 3039,
  [SMALL_STATE(83)] = 3051,
  [SMALL_STATE(84)] = 3069,
  [SMALL_STATE(85)] = 3083,
  [SMALL_STATE(86)] = 3095,
  [SMALL_STATE(87)] = 3107,
  [SMALL_STATE(88)] = 3119,
  [SMALL_STATE(89)] = 3131,
  [SMALL_STATE(90)] = 3148,
  [SMALL_STATE(91)] = 3161,
  [SMALL_STATE(92)] = 3178,
  [SMALL_STATE(93)] = 3195,
  [SMALL_STATE(94)] = 3212,
  [SMALL_STATE(95)] = 3229,
  [SMALL_STATE(96)] = 3246,
  [SMALL_STATE(97)] = 3263,
  [SMALL_STATE(98)] = 3280,
  [SMALL_STATE(99)] = 3297,
  [SMALL_STATE(100)] = 3314,
  [SMALL_STATE(101)] = 3325,
  [SMALL_STATE(102)] = 3342,
  [SMALL_STATE(103)] = 3356,
  [SMALL_STATE(104)] = 3370,
  [SMALL_STATE(105)] = 3386,
  [SMALL_STATE(106)] = 3402,
  [SMALL_STATE(107)] = 3416,
  [SMALL_STATE(108)] = 3432,
  [SMALL_STATE(109)] = 3446,
  [SMALL_STATE(110)] = 3460,
  [SMALL_STATE(111)] = 3474,
  [SMALL_STATE(112)] = 3490,
  [SMALL_STATE(113)] = 3504,
  [SMALL_STATE(114)] = 3518,
  [SMALL_STATE(115)] = 3534,
  [SMALL_STATE(116)] = 3548,
  [SMALL_STATE(117)] = 3562,
  [SMALL_STATE(118)] = 3578,
  [SMALL_STATE(119)] = 3589,
  [SMALL_STATE(120)] = 3598,
  [SMALL_STATE(121)] = 3611,
  [SMALL_STATE(122)] = 3624,
  [SMALL_STATE(123)] = 3635,
  [SMALL_STATE(124)] = 3646,
  [SMALL_STATE(125)] = 3659,
  [SMALL_STATE(126)] = 3672,
  [SMALL_STATE(127)] = 3681,
  [SMALL_STATE(128)] = 3694,
  [SMALL_STATE(129)] = 3707,
  [SMALL_STATE(130)] = 3718,
  [SMALL_STATE(131)] = 3731,
  [SMALL_STATE(132)] = 3744,
  [SMALL_STATE(133)] = 3755,
  [SMALL_STATE(134)] = 3766,
  [SMALL_STATE(135)] = 3775,
  [SMALL_STATE(136)] = 3788,
  [SMALL_STATE(137)] = 3799,
  [SMALL_STATE(138)] = 3808,
  [SMALL_STATE(139)] = 3821,
  [SMALL_STATE(140)] = 3834,
  [SMALL_STATE(141)] = 3847,
  [SMALL_STATE(142)] = 3858,
  [SMALL_STATE(143)] = 3869,
  [SMALL_STATE(144)] = 3882,
  [SMALL_STATE(145)] = 3891,
  [SMALL_STATE(146)] = 3900,
  [SMALL_STATE(147)] = 3913,
  [SMALL_STATE(148)] = 3924,
  [SMALL_STATE(149)] = 3937,
  [SMALL_STATE(150)] = 3950,
  [SMALL_STATE(151)] = 3963,
  [SMALL_STATE(152)] = 3976,
  [SMALL_STATE(153)] = 3987,
  [SMALL_STATE(154)] = 4000,
  [SMALL_STATE(155)] = 4013,
  [SMALL_STATE(156)] = 4026,
  [SMALL_STATE(157)] = 4036,
  [SMALL_STATE(158)] = 4044,
  [SMALL_STATE(159)] = 4054,
  [SMALL_STATE(160)] = 4064,
  [SMALL_STATE(161)] = 4074,
  [SMALL_STATE(162)] = 4082,
  [SMALL_STATE(163)] = 4090,
  [SMALL_STATE(164)] = 4100,
  [SMALL_STATE(165)] = 4110,
  [SMALL_STATE(166)] = 4120,
  [SMALL_STATE(167)] = 4128,
  [SMALL_STATE(168)] = 4138,
  [SMALL_STATE(169)] = 4146,
  [SMALL_STATE(170)] = 4154,
  [SMALL_STATE(171)] = 4162,
  [SMALL_STATE(172)] = 4170,
  [SMALL_STATE(173)] = 4178,
  [SMALL_STATE(174)] = 4188,
  [SMALL_STATE(175)] = 4198,
  [SMALL_STATE(176)] = 4206,
  [SMALL_STATE(177)] = 4216,
  [SMALL_STATE(178)] = 4226,
  [SMALL_STATE(179)] = 4236,
  [SMALL_STATE(180)] = 4246,
  [SMALL_STATE(181)] = 4256,
  [SMALL_STATE(182)] = 4266,
  [SMALL_STATE(183)] = 4274,
  [SMALL_STATE(184)] = 4284,
  [SMALL_STATE(185)] = 4294,
  [SMALL_STATE(186)] = 4301,
  [SMALL_STATE(187)] = 4308,
  [SMALL_STATE(188)] = 4315,
  [SMALL_STATE(189)] = 4322,
  [SMALL_STATE(190)] = 4329,
  [SMALL_STATE(191)] = 4336,
  [SMALL_STATE(192)] = 4343,
  [SMALL_STATE(193)] = 4350,
  [SMALL_STATE(194)] = 4357,
  [SMALL_STATE(195)] = 4364,
  [SMALL_STATE(196)] = 4371,
  [SMALL_STATE(197)] = 4378,
  [SMALL_STATE(198)] = 4385,
  [SMALL_STATE(199)] = 4392,
  [SMALL_STATE(200)] = 4399,
  [SMALL_STATE(201)] = 4406,
  [SMALL_STATE(202)] = 4413,
  [SMALL_STATE(203)] = 4420,
  [SMALL_STATE(204)] = 4427,
  [SMALL_STATE(205)] = 4434,
  [SMALL_STATE(206)] = 4441,
  [SMALL_STATE(207)] = 4448,
  [SMALL_STATE(208)] = 4455,
  [SMALL_STATE(209)] = 4462,
  [SMALL_STATE(210)] = 4469,
  [SMALL_STATE(211)] = 4476,
  [SMALL_STATE(212)] = 4483,
  [SMALL_STATE(213)] = 4490,
  [SMALL_STATE(214)] = 4497,
  [SMALL_STATE(215)] = 4504,
  [SMALL_STATE(216)] = 4511,
  [SMALL_STATE(217)] = 4518,
  [SMALL_STATE(218)] = 4525,
  [SMALL_STATE(219)] = 4532,
  [SMALL_STATE(220)] = 4539,
  [SMALL_STATE(221)] = 4546,
  [SMALL_STATE(222)] = 4553,
  [SMALL_STATE(223)] = 4560,
  [SMALL_STATE(224)] = 4567,
  [SMALL_STATE(225)] = 4574,
  [SMALL_STATE(226)] = 4581,
  [SMALL_STATE(227)] = 4588,
  [SMALL_STATE(228)] = 4595,
  [SMALL_STATE(229)] = 4602,
  [SMALL_STATE(230)] = 4609,
  [SMALL_STATE(231)] = 4616,
  [SMALL_STATE(232)] = 4623,
  [SMALL_STATE(233)] = 4630,
  [SMALL_STATE(234)] = 4637,
  [SMALL_STATE(235)] = 4644,
  [SMALL_STATE(236)] = 4651,
  [SMALL_STATE(237)] = 4658,
  [SMALL_STATE(238)] = 4665,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0, 0, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(218),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(160),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(235),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(234),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(199),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(220),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(73),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [31] = {.entry = {.count = 1, .reusable = false}}, SHIFT(199),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(168),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [37] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0),
  [39] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(199),
  [42] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(220),
  [45] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(72),
  [48] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(74),
  [51] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(71),
  [54] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(73),
  [57] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(69),
  [60] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(199),
  [63] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(168),
  [66] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_boolean, 1, 0, 0),
  [68] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_boolean, 1, 0, 0),
  [70] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 2, 0, 0),
  [72] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 2, 0, 0),
  [74] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 3, 0, 0),
  [76] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 3, 0, 0),
  [78] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_duration_unit, 1, 0, 0),
  [80] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_duration_unit, 1, 0, 0),
  [82] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_duration, 2, 0, 0),
  [84] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_duration, 2, 0, 0),
  [86] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 2, 0, 0),
  [88] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 2, 0, 0),
  [90] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 3, 0, 0),
  [92] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 3, 0, 0),
  [94] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 4, 0, 0),
  [96] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 4, 0, 0),
  [98] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 5, 0, 0),
  [100] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 5, 0, 0),
  [102] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_code_block, 3, 0, 0),
  [104] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_code_block, 3, 0, 0),
  [106] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property, 3, 0, 5),
  [108] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_property, 3, 0, 5),
  [110] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_code_block, 2, 0, 0),
  [112] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_code_block, 2, 0, 0),
  [114] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [116] = {.entry = {.count = 1, .reusable = true}}, SHIFT(212),
  [118] = {.entry = {.count = 1, .reusable = true}}, SHIFT(200),
  [120] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(160),
  [123] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0),
  [125] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(199),
  [128] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(84),
  [131] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(197),
  [134] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(185),
  [137] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(185),
  [140] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(184),
  [143] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(199),
  [146] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [148] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__value, 1, 0, 0),
  [150] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__value, 1, 0, 0),
  [152] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [154] = {.entry = {.count = 1, .reusable = false}}, SHIFT(8),
  [156] = {.entry = {.count = 1, .reusable = true}}, SHIFT(119),
  [158] = {.entry = {.count = 1, .reusable = true}}, SHIFT(84),
  [160] = {.entry = {.count = 1, .reusable = true}}, SHIFT(197),
  [162] = {.entry = {.count = 1, .reusable = false}}, SHIFT(185),
  [164] = {.entry = {.count = 1, .reusable = true}}, SHIFT(185),
  [166] = {.entry = {.count = 1, .reusable = true}}, SHIFT(184),
  [168] = {.entry = {.count = 1, .reusable = true}}, SHIFT(137),
  [170] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0),
  [172] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(199),
  [175] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(212),
  [178] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(200),
  [181] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(199),
  [184] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(168),
  [187] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_language_implementation, 3, 0, 8),
  [189] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_language_implementation, 3, 0, 8),
  [191] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_each_hook, 2, 0, 0),
  [193] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_each_hook, 2, 0, 0),
  [195] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_grouped, 4, 0, 0),
  [197] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hook_grouped, 4, 0, 0),
  [199] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_flat, 3, 0, 8),
  [201] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hook_flat, 3, 0, 8),
  [203] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_skip_hook, 2, 0, 0),
  [205] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_skip_hook, 2, 0, 0),
  [207] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_grouped, 3, 0, 0),
  [209] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hook_grouped, 3, 0, 0),
  [211] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tags_property, 3, 0, 0),
  [213] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_tags_property, 3, 0, 0),
  [215] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_hook, 2, 0, 0),
  [217] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_hook, 2, 0, 0),
  [219] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_before_hook, 2, 0, 0),
  [221] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_before_hook, 2, 0, 0),
  [223] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_validate_hook, 2, 0, 0),
  [225] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_validate_hook, 2, 0, 0),
  [227] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_body, 2, 0, 0),
  [229] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup_body, 2, 0, 0),
  [231] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_body, 3, 0, 0),
  [233] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup_body, 3, 0, 0),
  [235] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup, 2, 0, 0),
  [237] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup, 2, 0, 0),
  [239] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_shape_property, 3, 0, 0),
  [241] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_shape_property, 3, 0, 0),
  [243] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hex_property, 3, 0, 0),
  [245] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hex_property, 3, 0, 0),
  [247] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_file_ref, 4, 0, 0),
  [249] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_file_ref, 4, 0, 0),
  [251] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_body, 2, 0, 0),
  [253] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_body, 2, 0, 0),
  [255] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_block, 2, 0, 0),
  [257] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_block, 2, 0, 0),
  [259] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_body, 2, 0, 0),
  [261] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_setup_body, 2, 0, 0),
  [263] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture, 4, 0, 1),
  [265] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture, 4, 0, 1),
  [267] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark_body, 2, 0, 0),
  [269] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_benchmark_body, 2, 0, 0),
  [271] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark, 3, 0, 1),
  [273] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_benchmark, 3, 0, 1),
  [275] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture, 3, 0, 1),
  [277] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture, 3, 0, 1),
  [279] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_block, 3, 0, 4),
  [281] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_setup_block, 3, 0, 4),
  [283] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_body, 3, 0, 0),
  [285] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_body, 3, 0, 0),
  [287] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_body, 3, 0, 0),
  [289] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_setup_body, 3, 0, 0),
  [291] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_body, 2, 0, 0),
  [293] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture_body, 2, 0, 0),
  [295] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_body, 3, 0, 0),
  [297] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture_body, 3, 0, 0),
  [299] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark_body, 3, 0, 0),
  [301] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_benchmark_body, 3, 0, 0),
  [303] = {.entry = {.count = 1, .reusable = true}}, SHIFT(171),
  [305] = {.entry = {.count = 1, .reusable = true}}, SHIFT(213),
  [307] = {.entry = {.count = 1, .reusable = false}}, SHIFT(213),
  [309] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 3, 0, 0),
  [311] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 2, 0, 0),
  [313] = {.entry = {.count = 1, .reusable = false}}, SHIFT(170),
  [315] = {.entry = {.count = 1, .reusable = true}}, SHIFT(98),
  [317] = {.entry = {.count = 1, .reusable = true}}, SHIFT(97),
  [319] = {.entry = {.count = 1, .reusable = false}}, SHIFT(83),
  [321] = {.entry = {.count = 1, .reusable = true}}, SHIFT(170),
  [323] = {.entry = {.count = 1, .reusable = false}}, SHIFT(5),
  [325] = {.entry = {.count = 1, .reusable = true}}, SHIFT(114),
  [327] = {.entry = {.count = 1, .reusable = false}}, SHIFT(15),
  [329] = {.entry = {.count = 1, .reusable = false}}, SHIFT(20),
  [331] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [333] = {.entry = {.count = 1, .reusable = false}}, SHIFT(26),
  [335] = {.entry = {.count = 1, .reusable = false}}, SHIFT(35),
  [337] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [339] = {.entry = {.count = 1, .reusable = true}}, SHIFT(177),
  [341] = {.entry = {.count = 1, .reusable = true}}, SHIFT(108),
  [343] = {.entry = {.count = 1, .reusable = true}}, SHIFT(201),
  [345] = {.entry = {.count = 1, .reusable = true}}, SHIFT(180),
  [347] = {.entry = {.count = 1, .reusable = true}}, SHIFT(174),
  [349] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0),
  [351] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(177),
  [354] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(108),
  [357] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(201),
  [360] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(180),
  [363] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(174),
  [366] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [368] = {.entry = {.count = 1, .reusable = false}}, SHIFT(161),
  [370] = {.entry = {.count = 1, .reusable = true}}, SHIFT(161),
  [372] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [374] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, 0, 0),
  [376] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_hook_grouped_repeat1, 2, 0, 0),
  [378] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_hook_grouped_repeat1, 2, 0, 0), SHIFT_REPEAT(168),
  [381] = {.entry = {.count = 1, .reusable = true}}, SHIFT(227),
  [383] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [385] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [387] = {.entry = {.count = 1, .reusable = false}}, SHIFT(164),
  [389] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [391] = {.entry = {.count = 1, .reusable = false}}, SHIFT(226),
  [393] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [395] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(164),
  [398] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0),
  [400] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(226),
  [403] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0),
  [405] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(218),
  [408] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_section, 2, 0, 0),
  [410] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_paren_code_block, 2, 0, 0),
  [412] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_paren_code_block, 3, 0, 0),
  [414] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_init_section, 3, 0, 0),
  [416] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_helpers_section, 2, 0, 0),
  [418] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_init_section, 2, 0, 0),
  [420] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declare_section, 2, 0, 0),
  [422] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat2, 2, 0, 0),
  [424] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat2, 2, 0, 0), SHIFT_REPEAT(235),
  [427] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat2, 2, 0, 0), SHIFT_REPEAT(234),
  [430] = {.entry = {.count = 1, .reusable = true}}, SHIFT(217),
  [432] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [434] = {.entry = {.count = 1, .reusable = true}}, SHIFT(169),
  [436] = {.entry = {.count = 1, .reusable = true}}, SHIFT(187),
  [438] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, 0, 0),
  [440] = {.entry = {.count = 1, .reusable = false}}, SHIFT(6),
  [442] = {.entry = {.count = 1, .reusable = false}}, SHIFT(103),
  [444] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [446] = {.entry = {.count = 1, .reusable = false}}, SHIFT(116),
  [448] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_use_statement, 4, 0, 2),
  [450] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, 0, 0),
  [452] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [454] = {.entry = {.count = 1, .reusable = true}}, SHIFT(203),
  [456] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_single_string_content, 1, 0, 0),
  [458] = {.entry = {.count = 1, .reusable = false}}, SHIFT(109),
  [460] = {.entry = {.count = 1, .reusable = true}}, SHIFT(205),
  [462] = {.entry = {.count = 1, .reusable = true}}, SHIFT(147),
  [464] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [466] = {.entry = {.count = 1, .reusable = false}}, SHIFT(179),
  [468] = {.entry = {.count = 1, .reusable = false}}, SHIFT(28),
  [470] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [472] = {.entry = {.count = 1, .reusable = true}}, SHIFT(179),
  [474] = {.entry = {.count = 1, .reusable = true}}, SHIFT(181),
  [476] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_single_string_content_repeat1, 2, 0, 0),
  [478] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_single_string_content_repeat1, 2, 0, 0), SHIFT_REPEAT(109),
  [481] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_string_content_repeat1, 2, 0, 0),
  [483] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_string_content_repeat1, 2, 0, 0), SHIFT_REPEAT(110),
  [486] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [488] = {.entry = {.count = 1, .reusable = true}}, SHIFT(146),
  [490] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [492] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_after_body_repeat1, 2, 0, 0),
  [494] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_after_body_repeat1, 2, 0, 0), SHIFT_REPEAT(203),
  [497] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [499] = {.entry = {.count = 1, .reusable = false}}, SHIFT(24),
  [501] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_content, 1, 0, 0),
  [503] = {.entry = {.count = 1, .reusable = false}}, SHIFT(110),
  [505] = {.entry = {.count = 1, .reusable = true}}, SHIFT(129),
  [507] = {.entry = {.count = 1, .reusable = true}}, SHIFT(193),
  [509] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite_body, 3, 0, 0),
  [511] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_chart_params_repeat1, 2, 0, 0),
  [513] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_chart_params_repeat1, 2, 0, 0), SHIFT_REPEAT(58),
  [516] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [518] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup_statement, 1, 0, 0),
  [520] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_statement, 1, 0, 0),
  [522] = {.entry = {.count = 1, .reusable = true}}, SHIFT(230),
  [524] = {.entry = {.count = 1, .reusable = true}}, SHIFT(154),
  [526] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite, 3, 0, 1),
  [528] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 1, 0, 0),
  [530] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [532] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_argument_list_repeat1, 2, 0, 0),
  [534] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_argument_list_repeat1, 2, 0, 0), SHIFT_REPEAT(178),
  [537] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 5, 0, 0),
  [539] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 5, 0, 0),
  [541] = {.entry = {.count = 1, .reusable = true}}, SHIFT(105),
  [543] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_anvil_call, 5, 0, 0),
  [545] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_call, 5, 0, 0),
  [547] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite, 4, 0, 3),
  [549] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_string_array_repeat1, 2, 0, 0), SHIFT_REPEAT(125),
  [552] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_string_array_repeat1, 2, 0, 0),
  [554] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite_body, 2, 0, 0),
  [556] = {.entry = {.count = 1, .reusable = true}}, SHIFT(107),
  [558] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [560] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fixture_params_repeat1, 2, 0, 0),
  [562] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_params_repeat1, 2, 0, 0), SHIFT_REPEAT(156),
  [565] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_anvil_call, 6, 0, 0),
  [567] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_call, 6, 0, 0),
  [569] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 6, 0, 0),
  [571] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 6, 0, 0),
  [573] = {.entry = {.count = 1, .reusable = true}}, SHIFT(223),
  [575] = {.entry = {.count = 1, .reusable = true}}, SHIFT(207),
  [577] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite, 8, 0, 6),
  [579] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite, 9, 0, 7),
  [581] = {.entry = {.count = 1, .reusable = true}}, SHIFT(224),
  [583] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 3, 0, 0),
  [585] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 3, 0, 0),
  [587] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 3, 0, 0),
  [589] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 1, 0, 0),
  [591] = {.entry = {.count = 1, .reusable = true}}, SHIFT(151),
  [593] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 2, 0, 0),
  [595] = {.entry = {.count = 1, .reusable = true}}, SHIFT(148),
  [597] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 4, 0, 0),
  [599] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 4, 0, 0),
  [601] = {.entry = {.count = 1, .reusable = true}}, SHIFT(132),
  [603] = {.entry = {.count = 1, .reusable = true}}, SHIFT(208),
  [605] = {.entry = {.count = 1, .reusable = true}}, SHIFT(195),
  [607] = {.entry = {.count = 1, .reusable = true}}, SHIFT(143),
  [609] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_param, 3, 0, 9),
  [611] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [613] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_param, 3, 0, 5),
  [615] = {.entry = {.count = 1, .reusable = true}}, SHIFT(210),
  [617] = {.entry = {.count = 1, .reusable = true}}, SHIFT(104),
  [619] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [621] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_directive, 6, 0, 10),
  [623] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [625] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_language_tag, 1, 0, 0),
  [627] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite_type, 1, 0, 0),
  [629] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument, 3, 0, 5),
  [631] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_directive, 5, 0, 10),
  [633] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [635] = {.entry = {.count = 1, .reusable = true}}, SHIFT(188),
  [637] = {.entry = {.count = 1, .reusable = true}}, SHIFT(81),
  [639] = {.entry = {.count = 1, .reusable = true}}, SHIFT(190),
  [641] = {.entry = {.count = 1, .reusable = true}}, SHIFT(102),
  [643] = {.entry = {.count = 1, .reusable = true}}, SHIFT(165),
  [645] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [647] = {.entry = {.count = 1, .reusable = true}}, SHIFT(131),
  [649] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [651] = {.entry = {.count = 1, .reusable = true}}, SHIFT(117),
  [653] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [655] = {.entry = {.count = 1, .reusable = true}}, SHIFT(153),
  [657] = {.entry = {.count = 1, .reusable = true}}, SHIFT(198),
  [659] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_run_mode, 1, 0, 0),
  [661] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
  [663] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 4, 0, 0),
  [665] = {.entry = {.count = 1, .reusable = true}}, SHIFT(136),
  [667] = {.entry = {.count = 1, .reusable = true}}, SHIFT(111),
  [669] = {.entry = {.count = 1, .reusable = true}}, SHIFT(133),
  [671] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property_name, 1, 0, 0),
  [673] = {.entry = {.count = 1, .reusable = true}}, SHIFT(159),
  [675] = {.entry = {.count = 1, .reusable = true}}, SHIFT(158),
  [677] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_args, 3, 0, 0),
  [679] = {.entry = {.count = 1, .reusable = true}}, SHIFT(90),
  [681] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [683] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [685] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 5, 0, 0),
  [687] = {.entry = {.count = 1, .reusable = true}}, SHIFT(140),
  [689] = {.entry = {.count = 1, .reusable = true}}, SHIFT(141),
  [691] = {.entry = {.count = 1, .reusable = true}}, SHIFT(189),
  [693] = {.entry = {.count = 1, .reusable = true}}, SHIFT(191),
  [695] = {.entry = {.count = 1, .reusable = true}}, SHIFT(93),
  [697] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_param_name, 1, 0, 0),
  [699] = {.entry = {.count = 1, .reusable = true}}, SHIFT(166),
  [701] = {.entry = {.count = 1, .reusable = true}}, SHIFT(142),
  [703] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [705] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_function_name, 1, 0, 0),
  [707] = {.entry = {.count = 1, .reusable = true}}, SHIFT(232),
  [709] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [711] = {.entry = {.count = 1, .reusable = true}}, SHIFT(163),
  [713] = {.entry = {.count = 1, .reusable = true}}, SHIFT(152),
  [715] = {.entry = {.count = 1, .reusable = true}}, SHIFT(196),
  [717] = {.entry = {.count = 1, .reusable = true}}, SHIFT(236),
  [719] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 2, 0, 0),
  [721] = {.entry = {.count = 1, .reusable = true}}, SHIFT(106),
  [723] = {.entry = {.count = 1, .reusable = true}}, SHIFT(211),
  [725] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [727] = {.entry = {.count = 1, .reusable = true}}, SHIFT(100),
  [729] = {.entry = {.count = 1, .reusable = true}}, SHIFT(115),
  [731] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 3, 0, 0),
  [733] = {.entry = {.count = 1, .reusable = true}}, SHIFT(91),
  [735] = {.entry = {.count = 1, .reusable = true}}, SHIFT(228),
  [737] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [739] = {.entry = {.count = 1, .reusable = true}}, SHIFT(95),
  [741] = {.entry = {.count = 1, .reusable = true}}, SHIFT(231),
  [743] = {.entry = {.count = 1, .reusable = true}}, SHIFT(157),
  [745] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [747] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
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
