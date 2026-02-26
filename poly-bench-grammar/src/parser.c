#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 251
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 196
#define ALIAS_COUNT 0
#define TOKEN_COUNT 113
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
  anon_sym_rs = 90,
  anon_sym_python = 91,
  anon_sym_py = 92,
  anon_sym_csharp = 93,
  anon_sym_cs = 94,
  sym_inline_code = 95,
  anon_sym_DQUOTE = 96,
  anon_sym_SQUOTE = 97,
  aux_sym_string_content_token1 = 98,
  aux_sym_single_string_content_token1 = 99,
  sym_escape_sequence = 100,
  sym_number = 101,
  sym_float = 102,
  anon_sym_ms = 103,
  anon_sym_s = 104,
  anon_sym_m = 105,
  anon_sym_true = 106,
  anon_sym_false = 107,
  anon_sym_LBRACK = 108,
  anon_sym_RBRACK = 109,
  sym_comment = 110,
  sym_embedded_code = 111,
  sym__embedded_code_start = 112,
  sym_source_file = 113,
  sym_use_statement = 114,
  sym_global_setup = 115,
  sym_global_setup_body = 116,
  sym_global_setup_statement = 117,
  sym_anvil_call = 118,
  sym_anvil_args = 119,
  sym_function_call = 120,
  sym_argument_list = 121,
  sym_argument = 122,
  sym_suite = 123,
  sym_suite_type = 124,
  sym_run_mode = 125,
  sym_suite_body = 126,
  sym__suite_item = 127,
  sym_setup_block = 128,
  sym_setup_body = 129,
  sym__setup_section = 130,
  sym_import_section = 131,
  sym_declare_section = 132,
  sym_init_section = 133,
  sym_helpers_section = 134,
  sym_fixture = 135,
  sym_fixture_params = 136,
  sym_fixture_param = 137,
  sym_fixture_body = 138,
  sym__fixture_item = 139,
  sym_hex_property = 140,
  sym_data_property = 141,
  sym_encoding_property = 142,
  sym_format_property = 143,
  sym_selector_property = 144,
  sym_shape_property = 145,
  sym_file_ref = 146,
  sym_benchmark = 147,
  sym_benchmark_body = 148,
  sym__benchmark_item = 149,
  sym_tags_property = 150,
  sym_skip_hook = 151,
  sym_validate_hook = 152,
  sym_before_hook = 153,
  sym_after_hook = 154,
  sym_each_hook = 155,
  sym_hook_flat = 156,
  sym_hook_grouped = 157,
  sym_after_block = 158,
  sym_after_body = 159,
  sym_chart_directive = 160,
  sym_chart_function_name = 161,
  sym_chart_params = 162,
  sym_chart_param = 163,
  sym_chart_param_name = 164,
  sym__chart_value = 165,
  sym_property = 166,
  sym_property_name = 167,
  sym__value = 168,
  sym_language_implementation = 169,
  sym_language_tag = 170,
  sym__code_or_inline = 171,
  sym_code_block = 172,
  sym_paren_code_block = 173,
  sym_string = 174,
  sym_string_content = 175,
  sym_single_string_content = 176,
  sym_duration = 177,
  sym_duration_unit = 178,
  sym_boolean = 179,
  sym_string_array = 180,
  aux_sym_source_file_repeat1 = 181,
  aux_sym_source_file_repeat2 = 182,
  aux_sym_global_setup_body_repeat1 = 183,
  aux_sym_argument_list_repeat1 = 184,
  aux_sym_suite_body_repeat1 = 185,
  aux_sym_setup_body_repeat1 = 186,
  aux_sym_fixture_params_repeat1 = 187,
  aux_sym_fixture_body_repeat1 = 188,
  aux_sym_benchmark_body_repeat1 = 189,
  aux_sym_hook_grouped_repeat1 = 190,
  aux_sym_after_body_repeat1 = 191,
  aux_sym_chart_params_repeat1 = 192,
  aux_sym_string_content_repeat1 = 193,
  aux_sym_single_string_content_repeat1 = 194,
  aux_sym_string_array_repeat1 = 195,
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
  [anon_sym_rs] = "rs",
  [anon_sym_python] = "python",
  [anon_sym_py] = "py",
  [anon_sym_csharp] = "csharp",
  [anon_sym_cs] = "cs",
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
  [anon_sym_rs] = anon_sym_rs,
  [anon_sym_python] = anon_sym_python,
  [anon_sym_py] = anon_sym_py,
  [anon_sym_csharp] = anon_sym_csharp,
  [anon_sym_cs] = anon_sym_cs,
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
  [anon_sym_rs] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_python] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_py] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_csharp] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_cs] = {
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
  [64] = 64,
  [65] = 65,
  [66] = 65,
  [67] = 67,
  [68] = 68,
  [69] = 69,
  [70] = 70,
  [71] = 71,
  [72] = 72,
  [73] = 73,
  [74] = 65,
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
  [85] = 21,
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
      if (lookahead == 's') ADVANCE(56);
      if (lookahead == 'u') ADVANCE(57);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_s);
      ADVANCE_MAP(
        'a', 58,
        'e', 59,
        'h', 60,
        'i', 61,
        'k', 62,
        'o', 63,
        'p', 64,
        't', 65,
        'u', 66,
      );
      END_STATE();
    case 16:
      if (lookahead == 'a') ADVANCE(67);
      if (lookahead == 'h') ADVANCE(68);
      if (lookahead == 'i') ADVANCE(69);
      if (lookahead == 'r') ADVANCE(70);
      if (lookahead == 's') ADVANCE(71);
      if (lookahead == 'y') ADVANCE(72);
      END_STATE();
    case 17:
      if (lookahead == 's') ADVANCE(73);
      END_STATE();
    case 18:
      if (lookahead == 'a') ADVANCE(74);
      END_STATE();
    case 19:
      if (lookahead == 'a') ADVANCE(75);
      if (lookahead == 'i') ADVANCE(76);
      END_STATE();
    case 20:
      if (lookahead == 'S') ADVANCE(77);
      END_STATE();
    case 21:
      if (lookahead == 't') ADVANCE(78);
      END_STATE();
    case 22:
      if (lookahead == 'v') ADVANCE(79);
      END_STATE();
    case 23:
      if (lookahead == 'y') ADVANCE(80);
      END_STATE();
    case 24:
      if (lookahead == 's') ADVANCE(81);
      END_STATE();
    case 25:
      if (lookahead == 'f') ADVANCE(82);
      if (lookahead == 'n') ADVANCE(83);
      END_STATE();
    case 26:
      if (lookahead == 'a') ADVANCE(84);
      END_STATE();
    case 27:
      if (lookahead == 'u') ADVANCE(85);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(anon_sym_cs);
      if (lookahead == 'h') ADVANCE(86);
      END_STATE();
    case 29:
      if (lookahead == 'T') ADVANCE(87);
      END_STATE();
    case 30:
      if (lookahead == 't') ADVANCE(88);
      END_STATE();
    case 31:
      if (lookahead == 'c') ADVANCE(89);
      if (lookahead == 's') ADVANCE(90);
      END_STATE();
    case 32:
      if (lookahead == 'a') ADVANCE(91);
      END_STATE();
    case 33:
      if (lookahead == 'c') ADVANCE(92);
      END_STATE();
    case 34:
      if (lookahead == 'c') ADVANCE(93);
      END_STATE();
    case 35:
      if (lookahead == 'c') ADVANCE(94);
      END_STATE();
    case 36:
      if (lookahead == 'i') ADVANCE(95);
      if (lookahead == 'l') ADVANCE(96);
      END_STATE();
    case 37:
      if (lookahead == 'l') ADVANCE(97);
      if (lookahead == 'x') ADVANCE(98);
      END_STATE();
    case 38:
      if (lookahead == 'r') ADVANCE(99);
      END_STATE();
    case 39:
      if (lookahead == 'o') ADVANCE(100);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(anon_sym_go);
      END_STATE();
    case 41:
      if (lookahead == 'i') ADVANCE(101);
      if (lookahead == 'l') ADVANCE(102);
      if (lookahead == 'x') ADVANCE(103);
      END_STATE();
    case 42:
      if (lookahead == 'p') ADVANCE(104);
      END_STATE();
    case 43:
      if (lookahead == 'c') ADVANCE(105);
      if (lookahead == 'i') ADVANCE(106);
      END_STATE();
    case 44:
      if (lookahead == 'e') ADVANCE(107);
      END_STATE();
    case 45:
      if (lookahead == 'm') ADVANCE(108);
      END_STATE();
    case 46:
      if (lookahead == 'm') ADVANCE(109);
      END_STATE();
    case 47:
      if (lookahead == 'n') ADVANCE(110);
      END_STATE();
    case 48:
      if (lookahead == 'd') ADVANCE(111);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(anon_sym_ms);
      END_STATE();
    case 50:
      if (lookahead == 'd') ADVANCE(112);
      END_STATE();
    case 51:
      if (lookahead == 't') ADVANCE(113);
      END_STATE();
    case 52:
      if (lookahead == 'r') ADVANCE(114);
      END_STATE();
    case 53:
      ACCEPT_TOKEN(anon_sym_py);
      if (lookahead == 't') ADVANCE(115);
      END_STATE();
    case 54:
      if (lookahead == 'g') ADVANCE(116);
      if (lookahead == 'q') ADVANCE(117);
      END_STATE();
    case 55:
      if (lookahead == 'w') ADVANCE(118);
      END_STATE();
    case 56:
      ACCEPT_TOKEN(anon_sym_rs);
      END_STATE();
    case 57:
      if (lookahead == 's') ADVANCE(119);
      END_STATE();
    case 58:
      if (lookahead == 'm') ADVANCE(120);
      END_STATE();
    case 59:
      if (lookahead == 'l') ADVANCE(121);
      if (lookahead == 't') ADVANCE(122);
      END_STATE();
    case 60:
      if (lookahead == 'a') ADVANCE(123);
      if (lookahead == 'o') ADVANCE(124);
      END_STATE();
    case 61:
      if (lookahead == 'n') ADVANCE(125);
      END_STATE();
    case 62:
      if (lookahead == 'i') ADVANCE(126);
      END_STATE();
    case 63:
      if (lookahead == 'r') ADVANCE(127);
      END_STATE();
    case 64:
      if (lookahead == 'a') ADVANCE(128);
      END_STATE();
    case 65:
      if (lookahead == 'd') ADVANCE(129);
      END_STATE();
    case 66:
      if (lookahead == 'i') ADVANCE(130);
      END_STATE();
    case 67:
      if (lookahead == 'g') ADVANCE(131);
      if (lookahead == 'r') ADVANCE(132);
      END_STATE();
    case 68:
      if (lookahead == 'e') ADVANCE(133);
      END_STATE();
    case 69:
      if (lookahead == 'm') ADVANCE(134);
      if (lookahead == 't') ADVANCE(135);
      END_STATE();
    case 70:
      if (lookahead == 'u') ADVANCE(136);
      END_STATE();
    case 71:
      ACCEPT_TOKEN(anon_sym_ts);
      END_STATE();
    case 72:
      if (lookahead == 'p') ADVANCE(137);
      END_STATE();
    case 73:
      if (lookahead == 'e') ADVANCE(138);
      END_STATE();
    case 74:
      if (lookahead == 'l') ADVANCE(139);
      END_STATE();
    case 75:
      if (lookahead == 'r') ADVANCE(140);
      END_STATE();
    case 76:
      if (lookahead == 'd') ADVANCE(141);
      END_STATE();
    case 77:
      if (lookahead == 'c') ADVANCE(142);
      END_STATE();
    case 78:
      if (lookahead == 'e') ADVANCE(143);
      END_STATE();
    case 79:
      if (lookahead == 'i') ADVANCE(144);
      END_STATE();
    case 80:
      if (lookahead == 'n') ADVANCE(145);
      END_STATE();
    case 81:
      if (lookahead == 'e') ADVANCE(146);
      END_STATE();
    case 82:
      if (lookahead == 'o') ADVANCE(147);
      END_STATE();
    case 83:
      if (lookahead == 'c') ADVANCE(148);
      END_STATE();
    case 84:
      if (lookahead == 'r') ADVANCE(149);
      END_STATE();
    case 85:
      if (lookahead == 'n') ADVANCE(150);
      END_STATE();
    case 86:
      if (lookahead == 'a') ADVANCE(151);
      END_STATE();
    case 87:
      if (lookahead == 'h') ADVANCE(152);
      END_STATE();
    case 88:
      if (lookahead == 'a') ADVANCE(153);
      END_STATE();
    case 89:
      if (lookahead == 'l') ADVANCE(154);
      END_STATE();
    case 90:
      if (lookahead == 'c') ADVANCE(155);
      END_STATE();
    case 91:
      if (lookahead == 'w') ADVANCE(156);
      END_STATE();
    case 92:
      if (lookahead == 'h') ADVANCE(157);
      END_STATE();
    case 93:
      if (lookahead == 'o') ADVANCE(158);
      END_STATE();
    case 94:
      if (lookahead == 'l') ADVANCE(159);
      END_STATE();
    case 95:
      if (lookahead == 'r') ADVANCE(160);
      END_STATE();
    case 96:
      if (lookahead == 's') ADVANCE(161);
      END_STATE();
    case 97:
      if (lookahead == 't') ADVANCE(162);
      END_STATE();
    case 98:
      if (lookahead == 't') ADVANCE(163);
      END_STATE();
    case 99:
      if (lookahead == 'k') ADVANCE(164);
      if (lookahead == 'm') ADVANCE(165);
      END_STATE();
    case 100:
      if (lookahead == 'b') ADVANCE(166);
      END_STATE();
    case 101:
      if (lookahead == 'g') ADVANCE(167);
      END_STATE();
    case 102:
      if (lookahead == 'p') ADVANCE(168);
      END_STATE();
    case 103:
      ACCEPT_TOKEN(anon_sym_hex);
      END_STATE();
    case 104:
      if (lookahead == 'o') ADVANCE(169);
      END_STATE();
    case 105:
      if (lookahead == 'l') ADVANCE(170);
      END_STATE();
    case 106:
      if (lookahead == 't') ADVANCE(171);
      END_STATE();
    case 107:
      if (lookahead == 'r') ADVANCE(172);
      END_STATE();
    case 108:
      if (lookahead == 'i') ADVANCE(173);
      END_STATE();
    case 109:
      if (lookahead == 'o') ADVANCE(174);
      END_STATE();
    case 110:
      if (lookahead == 'S') ADVANCE(175);
      END_STATE();
    case 111:
      if (lookahead == 'e') ADVANCE(176);
      END_STATE();
    case 112:
      if (lookahead == 'e') ADVANCE(177);
      END_STATE();
    case 113:
      if (lookahead == 'l') ADVANCE(178);
      if (lookahead == 'p') ADVANCE(179);
      END_STATE();
    case 114:
      if (lookahead == 'f') ADVANCE(180);
      END_STATE();
    case 115:
      if (lookahead == 'h') ADVANCE(181);
      END_STATE();
    case 116:
      if (lookahead == 'r') ADVANCE(182);
      END_STATE();
    case 117:
      if (lookahead == 'u') ADVANCE(183);
      END_STATE();
    case 118:
      if (lookahead == 'C') ADVANCE(184);
      END_STATE();
    case 119:
      if (lookahead == 't') ADVANCE(185);
      END_STATE();
    case 120:
      if (lookahead == 'e') ADVANCE(186);
      END_STATE();
    case 121:
      if (lookahead == 'e') ADVANCE(187);
      END_STATE();
    case 122:
      if (lookahead == 'u') ADVANCE(188);
      END_STATE();
    case 123:
      if (lookahead == 'p') ADVANCE(189);
      END_STATE();
    case 124:
      if (lookahead == 'w') ADVANCE(190);
      END_STATE();
    case 125:
      if (lookahead == 'k') ADVANCE(191);
      END_STATE();
    case 126:
      if (lookahead == 'p') ADVANCE(192);
      END_STATE();
    case 127:
      if (lookahead == 't') ADVANCE(193);
      END_STATE();
    case 128:
      if (lookahead == 'w') ADVANCE(194);
      END_STATE();
    case 129:
      ACCEPT_TOKEN(anon_sym_std);
      END_STATE();
    case 130:
      if (lookahead == 't') ADVANCE(195);
      END_STATE();
    case 131:
      if (lookahead == 's') ADVANCE(196);
      END_STATE();
    case 132:
      if (lookahead == 'g') ADVANCE(197);
      END_STATE();
    case 133:
      if (lookahead == 'm') ADVANCE(198);
      END_STATE();
    case 134:
      if (lookahead == 'e') ADVANCE(199);
      END_STATE();
    case 135:
      if (lookahead == 'l') ADVANCE(200);
      END_STATE();
    case 136:
      if (lookahead == 'e') ADVANCE(201);
      END_STATE();
    case 137:
      if (lookahead == 'e') ADVANCE(202);
      END_STATE();
    case 138:
      ACCEPT_TOKEN(anon_sym_use);
      END_STATE();
    case 139:
      if (lookahead == 'i') ADVANCE(203);
      END_STATE();
    case 140:
      if (lookahead == 'm') ADVANCE(204);
      END_STATE();
    case 141:
      if (lookahead == 't') ADVANCE(205);
      END_STATE();
    case 142:
      if (lookahead == 'a') ADVANCE(206);
      END_STATE();
    case 143:
      if (lookahead == 'r') ADVANCE(207);
      END_STATE();
    case 144:
      if (lookahead == 'l') ADVANCE(208);
      END_STATE();
    case 145:
      if (lookahead == 'c') ADVANCE(209);
      END_STATE();
    case 146:
      if (lookahead == 'l') ADVANCE(210);
      END_STATE();
    case 147:
      if (lookahead == 'r') ADVANCE(211);
      END_STATE();
    case 148:
      if (lookahead == 'h') ADVANCE(212);
      END_STATE();
    case 149:
      if (lookahead == 't') ADVANCE(213);
      END_STATE();
    case 150:
      if (lookahead == 't') ADVANCE(214);
      END_STATE();
    case 151:
      if (lookahead == 'r') ADVANCE(215);
      END_STATE();
    case 152:
      if (lookahead == 'r') ADVANCE(216);
      END_STATE();
    case 153:
      ACCEPT_TOKEN(anon_sym_data);
      END_STATE();
    case 154:
      if (lookahead == 'a') ADVANCE(217);
      END_STATE();
    case 155:
      if (lookahead == 'r') ADVANCE(218);
      END_STATE();
    case 156:
      if (lookahead == 'B') ADVANCE(219);
      if (lookahead == 'L') ADVANCE(220);
      if (lookahead == 'S') ADVANCE(221);
      if (lookahead == 'T') ADVANCE(222);
      END_STATE();
    case 157:
      ACCEPT_TOKEN(anon_sym_each);
      END_STATE();
    case 158:
      if (lookahead == 'd') ADVANCE(223);
      END_STATE();
    case 159:
      if (lookahead == 'u') ADVANCE(224);
      END_STATE();
    case 160:
      if (lookahead == 'n') ADVANCE(225);
      END_STATE();
    case 161:
      if (lookahead == 'e') ADVANCE(226);
      END_STATE();
    case 162:
      if (lookahead == 'e') ADVANCE(227);
      END_STATE();
    case 163:
      if (lookahead == 'u') ADVANCE(228);
      END_STATE();
    case 164:
      ACCEPT_TOKEN(anon_sym_fork);
      END_STATE();
    case 165:
      if (lookahead == 'a') ADVANCE(229);
      END_STATE();
    case 166:
      if (lookahead == 'a') ADVANCE(230);
      END_STATE();
    case 167:
      if (lookahead == 'h') ADVANCE(231);
      END_STATE();
    case 168:
      if (lookahead == 'e') ADVANCE(232);
      END_STATE();
    case 169:
      if (lookahead == 'r') ADVANCE(233);
      END_STATE();
    case 170:
      if (lookahead == 'u') ADVANCE(234);
      END_STATE();
    case 171:
      ACCEPT_TOKEN(anon_sym_init);
      END_STATE();
    case 172:
      if (lookahead == 'a') ADVANCE(235);
      END_STATE();
    case 173:
      if (lookahead == 't') ADVANCE(236);
      END_STATE();
    case 174:
      if (lookahead == 'r') ADVANCE(237);
      END_STATE();
    case 175:
      if (lookahead == 'p') ADVANCE(238);
      END_STATE();
    case 176:
      ACCEPT_TOKEN(anon_sym_mode);
      END_STATE();
    case 177:
      if (lookahead == 'r') ADVANCE(239);
      END_STATE();
    case 178:
      if (lookahead == 'i') ADVANCE(240);
      END_STATE();
    case 179:
      if (lookahead == 'u') ADVANCE(241);
      END_STATE();
    case 180:
      if (lookahead == 'o') ADVANCE(242);
      END_STATE();
    case 181:
      if (lookahead == 'o') ADVANCE(243);
      END_STATE();
    case 182:
      if (lookahead == 'e') ADVANCE(244);
      END_STATE();
    case 183:
      if (lookahead == 'i') ADVANCE(245);
      END_STATE();
    case 184:
      if (lookahead == 'o') ADVANCE(246);
      END_STATE();
    case 185:
      ACCEPT_TOKEN(anon_sym_rust);
      END_STATE();
    case 186:
      if (lookahead == 'D') ADVANCE(247);
      END_STATE();
    case 187:
      if (lookahead == 'c') ADVANCE(248);
      END_STATE();
    case 188:
      if (lookahead == 'p') ADVANCE(249);
      END_STATE();
    case 189:
      if (lookahead == 'e') ADVANCE(250);
      END_STATE();
    case 190:
      if (lookahead == 'E') ADVANCE(251);
      if (lookahead == 'R') ADVANCE(252);
      if (lookahead == 'S') ADVANCE(253);
      END_STATE();
    case 191:
      ACCEPT_TOKEN(anon_sym_sink);
      END_STATE();
    case 192:
      ACCEPT_TOKEN(anon_sym_skip);
      END_STATE();
    case 193:
      if (lookahead == 'B') ADVANCE(254);
      if (lookahead == 'O') ADVANCE(255);
      END_STATE();
    case 194:
      if (lookahead == 'n') ADVANCE(256);
      END_STATE();
    case 195:
      if (lookahead == 'e') ADVANCE(257);
      END_STATE();
    case 196:
      ACCEPT_TOKEN(anon_sym_tags);
      END_STATE();
    case 197:
      if (lookahead == 'e') ADVANCE(258);
      END_STATE();
    case 198:
      if (lookahead == 'e') ADVANCE(259);
      END_STATE();
    case 199:
      if (lookahead == 'B') ADVANCE(260);
      if (lookahead == 'o') ADVANCE(261);
      END_STATE();
    case 200:
      if (lookahead == 'e') ADVANCE(262);
      END_STATE();
    case 201:
      ACCEPT_TOKEN(anon_sym_true);
      END_STATE();
    case 202:
      if (lookahead == 's') ADVANCE(263);
      END_STATE();
    case 203:
      if (lookahead == 'd') ADVANCE(264);
      END_STATE();
    case 204:
      if (lookahead == 'u') ADVANCE(265);
      END_STATE();
    case 205:
      if (lookahead == 'h') ADVANCE(266);
      END_STATE();
    case 206:
      if (lookahead == 'l') ADVANCE(267);
      END_STATE();
    case 207:
      ACCEPT_TOKEN(anon_sym_after);
      END_STATE();
    case 208:
      ACCEPT_TOKEN(anon_sym_anvil);
      END_STATE();
    case 209:
      ACCEPT_TOKEN(anon_sym_async);
      if (lookahead == 'S') ADVANCE(268);
      if (lookahead == 'W') ADVANCE(269);
      END_STATE();
    case 210:
      if (lookahead == 'i') ADVANCE(270);
      END_STATE();
    case 211:
      if (lookahead == 'e') ADVANCE(271);
      END_STATE();
    case 212:
      ACCEPT_TOKEN(anon_sym_bench);
      if (lookahead == 'A') ADVANCE(272);
      END_STATE();
    case 213:
      if (lookahead == 'i') ADVANCE(273);
      END_STATE();
    case 214:
      ACCEPT_TOKEN(anon_sym_count);
      END_STATE();
    case 215:
      if (lookahead == 'p') ADVANCE(274);
      END_STATE();
    case 216:
      if (lookahead == 'e') ADVANCE(275);
      END_STATE();
    case 217:
      if (lookahead == 'r') ADVANCE(276);
      END_STATE();
    case 218:
      if (lookahead == 'i') ADVANCE(277);
      END_STATE();
    case 219:
      if (lookahead == 'a') ADVANCE(278);
      END_STATE();
    case 220:
      if (lookahead == 'i') ADVANCE(279);
      END_STATE();
    case 221:
      if (lookahead == 'p') ADVANCE(280);
      END_STATE();
    case 222:
      if (lookahead == 'a') ADVANCE(281);
      END_STATE();
    case 223:
      if (lookahead == 'i') ADVANCE(282);
      END_STATE();
    case 224:
      if (lookahead == 'd') ADVANCE(283);
      END_STATE();
    case 225:
      if (lookahead == 'e') ADVANCE(284);
      END_STATE();
    case 226:
      ACCEPT_TOKEN(anon_sym_false);
      END_STATE();
    case 227:
      if (lookahead == 'r') ADVANCE(285);
      END_STATE();
    case 228:
      if (lookahead == 'r') ADVANCE(286);
      END_STATE();
    case 229:
      if (lookahead == 't') ADVANCE(287);
      END_STATE();
    case 230:
      if (lookahead == 'l') ADVANCE(288);
      END_STATE();
    case 231:
      if (lookahead == 't') ADVANCE(289);
      END_STATE();
    case 232:
      if (lookahead == 'r') ADVANCE(290);
      END_STATE();
    case 233:
      if (lookahead == 't') ADVANCE(291);
      END_STATE();
    case 234:
      if (lookahead == 'd') ADVANCE(292);
      END_STATE();
    case 235:
      if (lookahead == 't') ADVANCE(293);
      END_STATE();
    case 236:
      ACCEPT_TOKEN(anon_sym_limit);
      END_STATE();
    case 237:
      if (lookahead == 'y') ADVANCE(294);
      END_STATE();
    case 238:
      if (lookahead == 'e') ADVANCE(295);
      END_STATE();
    case 239:
      ACCEPT_TOKEN(anon_sym_order);
      END_STATE();
    case 240:
      if (lookahead == 'e') ADVANCE(296);
      END_STATE();
    case 241:
      if (lookahead == 't') ADVANCE(297);
      END_STATE();
    case 242:
      if (lookahead == 'r') ADVANCE(298);
      END_STATE();
    case 243:
      if (lookahead == 'n') ADVANCE(299);
      END_STATE();
    case 244:
      if (lookahead == 's') ADVANCE(300);
      END_STATE();
    case 245:
      if (lookahead == 'r') ADVANCE(301);
      END_STATE();
    case 246:
      if (lookahead == 'u') ADVANCE(302);
      END_STATE();
    case 247:
      if (lookahead == 'a') ADVANCE(303);
      END_STATE();
    case 248:
      if (lookahead == 't') ADVANCE(304);
      END_STATE();
    case 249:
      ACCEPT_TOKEN(anon_sym_setup);
      END_STATE();
    case 250:
      ACCEPT_TOKEN(anon_sym_shape);
      END_STATE();
    case 251:
      if (lookahead == 'r') ADVANCE(305);
      END_STATE();
    case 252:
      if (lookahead == 'e') ADVANCE(306);
      END_STATE();
    case 253:
      if (lookahead == 't') ADVANCE(307);
      END_STATE();
    case 254:
      if (lookahead == 'y') ADVANCE(308);
      END_STATE();
    case 255:
      if (lookahead == 'r') ADVANCE(309);
      END_STATE();
    case 256:
      if (lookahead == 'A') ADVANCE(310);
      END_STATE();
    case 257:
      ACCEPT_TOKEN(anon_sym_suite);
      END_STATE();
    case 258:
      if (lookahead == 't') ADVANCE(311);
      END_STATE();
    case 259:
      ACCEPT_TOKEN(anon_sym_theme);
      END_STATE();
    case 260:
      if (lookahead == 'a') ADVANCE(312);
      END_STATE();
    case 261:
      if (lookahead == 'u') ADVANCE(313);
      END_STATE();
    case 262:
      ACCEPT_TOKEN(anon_sym_title);
      END_STATE();
    case 263:
      if (lookahead == 'c') ADVANCE(314);
      END_STATE();
    case 264:
      if (lookahead == 'a') ADVANCE(315);
      END_STATE();
    case 265:
      if (lookahead == 'p') ADVANCE(316);
      END_STATE();
    case 266:
      ACCEPT_TOKEN(anon_sym_width);
      END_STATE();
    case 267:
      if (lookahead == 'e') ADVANCE(317);
      END_STATE();
    case 268:
      if (lookahead == 'a') ADVANCE(318);
      END_STATE();
    case 269:
      if (lookahead == 'a') ADVANCE(319);
      END_STATE();
    case 270:
      if (lookahead == 'n') ADVANCE(320);
      END_STATE();
    case 271:
      ACCEPT_TOKEN(anon_sym_before);
      END_STATE();
    case 272:
      if (lookahead == 's') ADVANCE(321);
      END_STATE();
    case 273:
      if (lookahead == 'n') ADVANCE(322);
      END_STATE();
    case 274:
      ACCEPT_TOKEN(anon_sym_csharp);
      END_STATE();
    case 275:
      if (lookahead == 's') ADVANCE(323);
      END_STATE();
    case 276:
      if (lookahead == 'e') ADVANCE(324);
      END_STATE();
    case 277:
      if (lookahead == 'p') ADVANCE(325);
      END_STATE();
    case 278:
      if (lookahead == 'r') ADVANCE(326);
      END_STATE();
    case 279:
      if (lookahead == 'n') ADVANCE(327);
      END_STATE();
    case 280:
      if (lookahead == 'e') ADVANCE(328);
      END_STATE();
    case 281:
      if (lookahead == 'b') ADVANCE(329);
      END_STATE();
    case 282:
      if (lookahead == 'n') ADVANCE(330);
      END_STATE();
    case 283:
      if (lookahead == 'e') ADVANCE(331);
      END_STATE();
    case 284:
      if (lookahead == 's') ADVANCE(332);
      END_STATE();
    case 285:
      if (lookahead == 'W') ADVANCE(333);
      END_STATE();
    case 286:
      if (lookahead == 'e') ADVANCE(334);
      END_STATE();
    case 287:
      ACCEPT_TOKEN(anon_sym_format);
      END_STATE();
    case 288:
      if (lookahead == 'S') ADVANCE(335);
      END_STATE();
    case 289:
      ACCEPT_TOKEN(anon_sym_height);
      END_STATE();
    case 290:
      if (lookahead == 's') ADVANCE(336);
      END_STATE();
    case 291:
      ACCEPT_TOKEN(anon_sym_import);
      END_STATE();
    case 292:
      if (lookahead == 'e') ADVANCE(337);
      END_STATE();
    case 293:
      if (lookahead == 'i') ADVANCE(338);
      END_STATE();
    case 294:
      ACCEPT_TOKEN(anon_sym_memory);
      END_STATE();
    case 295:
      if (lookahead == 'e') ADVANCE(339);
      END_STATE();
    case 296:
      if (lookahead == 'r') ADVANCE(340);
      END_STATE();
    case 297:
      ACCEPT_TOKEN(anon_sym_output);
      END_STATE();
    case 298:
      if (lookahead == 'm') ADVANCE(341);
      END_STATE();
    case 299:
      ACCEPT_TOKEN(anon_sym_python);
      END_STATE();
    case 300:
      if (lookahead == 's') ADVANCE(342);
      END_STATE();
    case 301:
      if (lookahead == 'e') ADVANCE(343);
      END_STATE();
    case 302:
      if (lookahead == 'n') ADVANCE(344);
      END_STATE();
    case 303:
      if (lookahead == 't') ADVANCE(345);
      END_STATE();
    case 304:
      if (lookahead == 'o') ADVANCE(346);
      END_STATE();
    case 305:
      if (lookahead == 'r') ADVANCE(347);
      END_STATE();
    case 306:
      if (lookahead == 'g') ADVANCE(348);
      END_STATE();
    case 307:
      if (lookahead == 'd') ADVANCE(349);
      END_STATE();
    case 308:
      ACCEPT_TOKEN(anon_sym_sortBy);
      END_STATE();
    case 309:
      if (lookahead == 'd') ADVANCE(350);
      END_STATE();
    case 310:
      if (lookahead == 'n') ADVANCE(351);
      END_STATE();
    case 311:
      if (lookahead == 'T') ADVANCE(352);
      END_STATE();
    case 312:
      if (lookahead == 's') ADVANCE(353);
      END_STATE();
    case 313:
      if (lookahead == 't') ADVANCE(354);
      END_STATE();
    case 314:
      if (lookahead == 'r') ADVANCE(355);
      END_STATE();
    case 315:
      if (lookahead == 't') ADVANCE(356);
      END_STATE();
    case 316:
      ACCEPT_TOKEN(anon_sym_warmup);
      END_STATE();
    case 317:
      ACCEPT_TOKEN(anon_sym_yScale);
      END_STATE();
    case 318:
      if (lookahead == 'm') ADVANCE(357);
      END_STATE();
    case 319:
      if (lookahead == 'r') ADVANCE(358);
      END_STATE();
    case 320:
      if (lookahead == 'e') ADVANCE(359);
      END_STATE();
    case 321:
      if (lookahead == 'y') ADVANCE(360);
      END_STATE();
    case 322:
      if (lookahead == 'g') ADVANCE(361);
      END_STATE();
    case 323:
      if (lookahead == 'h') ADVANCE(362);
      END_STATE();
    case 324:
      ACCEPT_TOKEN(anon_sym_declare);
      END_STATE();
    case 325:
      if (lookahead == 't') ADVANCE(363);
      END_STATE();
    case 326:
      if (lookahead == 'C') ADVANCE(364);
      END_STATE();
    case 327:
      if (lookahead == 'e') ADVANCE(365);
      END_STATE();
    case 328:
      if (lookahead == 'e') ADVANCE(366);
      END_STATE();
    case 329:
      if (lookahead == 'l') ADVANCE(367);
      END_STATE();
    case 330:
      if (lookahead == 'g') ADVANCE(368);
      END_STATE();
    case 331:
      if (lookahead == 'B') ADVANCE(369);
      END_STATE();
    case 332:
      if (lookahead == 's') ADVANCE(370);
      END_STATE();
    case 333:
      if (lookahead == 'i') ADVANCE(371);
      END_STATE();
    case 334:
      ACCEPT_TOKEN(anon_sym_fixture);
      END_STATE();
    case 335:
      if (lookahead == 'e') ADVANCE(372);
      END_STATE();
    case 336:
      ACCEPT_TOKEN(anon_sym_helpers);
      END_STATE();
    case 337:
      if (lookahead == 'B') ADVANCE(373);
      END_STATE();
    case 338:
      if (lookahead == 'o') ADVANCE(374);
      END_STATE();
    case 339:
      if (lookahead == 'd') ADVANCE(375);
      END_STATE();
    case 340:
      if (lookahead == 'D') ADVANCE(376);
      END_STATE();
    case 341:
      if (lookahead == 'a') ADVANCE(377);
      END_STATE();
    case 342:
      if (lookahead == 'i') ADVANCE(378);
      END_STATE();
    case 343:
      if (lookahead == 's') ADVANCE(379);
      END_STATE();
    case 344:
      if (lookahead == 't') ADVANCE(380);
      END_STATE();
    case 345:
      if (lookahead == 'a') ADVANCE(381);
      END_STATE();
    case 346:
      if (lookahead == 'r') ADVANCE(382);
      END_STATE();
    case 347:
      if (lookahead == 'o') ADVANCE(383);
      END_STATE();
    case 348:
      if (lookahead == 'r') ADVANCE(384);
      END_STATE();
    case 349:
      if (lookahead == 'D') ADVANCE(385);
      END_STATE();
    case 350:
      if (lookahead == 'e') ADVANCE(386);
      END_STATE();
    case 351:
      if (lookahead == 'v') ADVANCE(387);
      END_STATE();
    case 352:
      if (lookahead == 'i') ADVANCE(388);
      END_STATE();
    case 353:
      if (lookahead == 'e') ADVANCE(389);
      END_STATE();
    case 354:
      ACCEPT_TOKEN(anon_sym_timeout);
      END_STATE();
    case 355:
      if (lookahead == 'i') ADVANCE(390);
      END_STATE();
    case 356:
      if (lookahead == 'e') ADVANCE(391);
      END_STATE();
    case 357:
      if (lookahead == 'p') ADVANCE(392);
      END_STATE();
    case 358:
      if (lookahead == 'm') ADVANCE(393);
      END_STATE();
    case 359:
      ACCEPT_TOKEN(anon_sym_baseline);
      if (lookahead == 'B') ADVANCE(394);
      END_STATE();
    case 360:
      if (lookahead == 'n') ADVANCE(395);
      END_STATE();
    case 361:
      ACCEPT_TOKEN(anon_sym_charting);
      END_STATE();
    case 362:
      if (lookahead == 'o') ADVANCE(396);
      END_STATE();
    case 363:
      if (lookahead == 'i') ADVANCE(397);
      END_STATE();
    case 364:
      if (lookahead == 'h') ADVANCE(398);
      END_STATE();
    case 365:
      if (lookahead == 'C') ADVANCE(399);
      END_STATE();
    case 366:
      if (lookahead == 'd') ADVANCE(400);
      END_STATE();
    case 367:
      if (lookahead == 'e') ADVANCE(401);
      END_STATE();
    case 368:
      ACCEPT_TOKEN(anon_sym_encoding);
      END_STATE();
    case 369:
      if (lookahead == 'e') ADVANCE(402);
      END_STATE();
    case 370:
      ACCEPT_TOKEN(anon_sym_fairness);
      if (lookahead == 'S') ADVANCE(403);
      END_STATE();
    case 371:
      if (lookahead == 'n') ADVANCE(404);
      END_STATE();
    case 372:
      if (lookahead == 't') ADVANCE(405);
      END_STATE();
    case 373:
      if (lookahead == 'e') ADVANCE(406);
      END_STATE();
    case 374:
      if (lookahead == 'n') ADVANCE(407);
      END_STATE();
    case 375:
      if (lookahead == 'u') ADVANCE(408);
      END_STATE();
    case 376:
      if (lookahead == 'e') ADVANCE(409);
      END_STATE();
    case 377:
      if (lookahead == 'n') ADVANCE(410);
      END_STATE();
    case 378:
      if (lookahead == 'o') ADVANCE(411);
      END_STATE();
    case 379:
      ACCEPT_TOKEN(anon_sym_requires);
      END_STATE();
    case 380:
      ACCEPT_TOKEN(anon_sym_rowCount);
      END_STATE();
    case 381:
      if (lookahead == 's') ADVANCE(412);
      END_STATE();
    case 382:
      ACCEPT_TOKEN(anon_sym_selector);
      END_STATE();
    case 383:
      if (lookahead == 'r') ADVANCE(413);
      END_STATE();
    case 384:
      if (lookahead == 'e') ADVANCE(414);
      END_STATE();
    case 385:
      if (lookahead == 'e') ADVANCE(415);
      END_STATE();
    case 386:
      if (lookahead == 'r') ADVANCE(416);
      END_STATE();
    case 387:
      if (lookahead == 'i') ADVANCE(417);
      END_STATE();
    case 388:
      if (lookahead == 'm') ADVANCE(418);
      END_STATE();
    case 389:
      if (lookahead == 'd') ADVANCE(419);
      END_STATE();
    case 390:
      if (lookahead == 'p') ADVANCE(420);
      END_STATE();
    case 391:
      ACCEPT_TOKEN(anon_sym_validate);
      END_STATE();
    case 392:
      if (lookahead == 'l') ADVANCE(421);
      END_STATE();
    case 393:
      if (lookahead == 'u') ADVANCE(422);
      END_STATE();
    case 394:
      if (lookahead == 'e') ADVANCE(423);
      END_STATE();
    case 395:
      if (lookahead == 'c') ADVANCE(424);
      END_STATE();
    case 396:
      if (lookahead == 'l') ADVANCE(425);
      END_STATE();
    case 397:
      if (lookahead == 'o') ADVANCE(426);
      END_STATE();
    case 398:
      if (lookahead == 'a') ADVANCE(427);
      END_STATE();
    case 399:
      if (lookahead == 'h') ADVANCE(428);
      END_STATE();
    case 400:
      if (lookahead == 'u') ADVANCE(429);
      END_STATE();
    case 401:
      ACCEPT_TOKEN(anon_sym_drawTable);
      END_STATE();
    case 402:
      if (lookahead == 'n') ADVANCE(430);
      END_STATE();
    case 403:
      if (lookahead == 'e') ADVANCE(431);
      END_STATE();
    case 404:
      if (lookahead == 'n') ADVANCE(432);
      END_STATE();
    case 405:
      if (lookahead == 'u') ADVANCE(433);
      END_STATE();
    case 406:
      if (lookahead == 'n') ADVANCE(434);
      END_STATE();
    case 407:
      if (lookahead == 'B') ADVANCE(435);
      if (lookahead == 's') ADVANCE(436);
      END_STATE();
    case 408:
      if (lookahead == 'p') ADVANCE(437);
      END_STATE();
    case 409:
      if (lookahead == 't') ADVANCE(438);
      END_STATE();
    case 410:
      if (lookahead == 'c') ADVANCE(439);
      END_STATE();
    case 411:
      if (lookahead == 'n') ADVANCE(440);
      END_STATE();
    case 412:
      if (lookahead == 'e') ADVANCE(441);
      END_STATE();
    case 413:
      if (lookahead == 'B') ADVANCE(442);
      END_STATE();
    case 414:
      if (lookahead == 's') ADVANCE(443);
      END_STATE();
    case 415:
      if (lookahead == 'v') ADVANCE(444);
      END_STATE();
    case 416:
      ACCEPT_TOKEN(anon_sym_sortOrder);
      END_STATE();
    case 417:
      if (lookahead == 'l') ADVANCE(445);
      END_STATE();
    case 418:
      if (lookahead == 'e') ADVANCE(446);
      END_STATE();
    case 419:
      ACCEPT_TOKEN(anon_sym_timeBased);
      END_STATE();
    case 420:
      if (lookahead == 't') ADVANCE(447);
      END_STATE();
    case 421:
      if (lookahead == 'e') ADVANCE(448);
      if (lookahead == 'i') ADVANCE(449);
      END_STATE();
    case 422:
      if (lookahead == 'p') ADVANCE(450);
      END_STATE();
    case 423:
      if (lookahead == 'n') ADVANCE(451);
      END_STATE();
    case 424:
      ACCEPT_TOKEN(anon_sym_benchAsync);
      END_STATE();
    case 425:
      if (lookahead == 'd') ADVANCE(452);
      END_STATE();
    case 426:
      if (lookahead == 'n') ADVANCE(453);
      END_STATE();
    case 427:
      if (lookahead == 'r') ADVANCE(454);
      END_STATE();
    case 428:
      if (lookahead == 'a') ADVANCE(455);
      END_STATE();
    case 429:
      if (lookahead == 'p') ADVANCE(456);
      END_STATE();
    case 430:
      if (lookahead == 'c') ADVANCE(457);
      END_STATE();
    case 431:
      if (lookahead == 'e') ADVANCE(458);
      END_STATE();
    case 432:
      if (lookahead == 'e') ADVANCE(459);
      END_STATE();
    case 433:
      if (lookahead == 'p') ADVANCE(460);
      END_STATE();
    case 434:
      if (lookahead == 'c') ADVANCE(461);
      END_STATE();
    case 435:
      if (lookahead == 'a') ADVANCE(462);
      END_STATE();
    case 436:
      ACCEPT_TOKEN(anon_sym_iterations);
      END_STATE();
    case 437:
      ACCEPT_TOKEN(anon_sym_minSpeedup);
      END_STATE();
    case 438:
      if (lookahead == 'e') ADVANCE(463);
      END_STATE();
    case 439:
      if (lookahead == 'e') ADVANCE(464);
      END_STATE();
    case 440:
      if (lookahead == 'M') ADVANCE(465);
      END_STATE();
    case 441:
      if (lookahead == 't') ADVANCE(466);
      END_STATE();
    case 442:
      if (lookahead == 'a') ADVANCE(467);
      END_STATE();
    case 443:
      if (lookahead == 's') ADVANCE(468);
      END_STATE();
    case 444:
      ACCEPT_TOKEN(anon_sym_showStdDev);
      END_STATE();
    case 445:
      ACCEPT_TOKEN(anon_sym_spawnAnvil);
      END_STATE();
    case 446:
      ACCEPT_TOKEN(anon_sym_targetTime);
      END_STATE();
    case 447:
      ACCEPT_TOKEN(anon_sym_typescript);
      END_STATE();
    case 448:
      if (lookahead == 'C') ADVANCE(469);
      END_STATE();
    case 449:
      if (lookahead == 'n') ADVANCE(470);
      END_STATE();
    case 450:
      if (lookahead == 'C') ADVANCE(471);
      END_STATE();
    case 451:
      if (lookahead == 'c') ADVANCE(472);
      END_STATE();
    case 452:
      ACCEPT_TOKEN(anon_sym_cvThreshold);
      END_STATE();
    case 453:
      ACCEPT_TOKEN(anon_sym_description);
      END_STATE();
    case 454:
      if (lookahead == 't') ADVANCE(473);
      END_STATE();
    case 455:
      if (lookahead == 'r') ADVANCE(474);
      END_STATE();
    case 456:
      if (lookahead == 'C') ADVANCE(475);
      END_STATE();
    case 457:
      if (lookahead == 'h') ADVANCE(476);
      END_STATE();
    case 458:
      if (lookahead == 'd') ADVANCE(477);
      END_STATE();
    case 459:
      if (lookahead == 'r') ADVANCE(478);
      END_STATE();
    case 460:
      ACCEPT_TOKEN(anon_sym_globalSetup);
      END_STATE();
    case 461:
      if (lookahead == 'h') ADVANCE(479);
      END_STATE();
    case 462:
      if (lookahead == 's') ADVANCE(480);
      END_STATE();
    case 463:
      if (lookahead == 'c') ADVANCE(481);
      END_STATE();
    case 464:
      ACCEPT_TOKEN(anon_sym_performance);
      END_STATE();
    case 465:
      if (lookahead == 'o') ADVANCE(482);
      END_STATE();
    case 466:
      ACCEPT_TOKEN(anon_sym_sameDataset);
      END_STATE();
    case 467:
      if (lookahead == 'r') ADVANCE(483);
      END_STATE();
    case 468:
      if (lookahead == 'i') ADVANCE(484);
      END_STATE();
    case 469:
      if (lookahead == 'a') ADVANCE(485);
      END_STATE();
    case 470:
      if (lookahead == 'g') ADVANCE(486);
      END_STATE();
    case 471:
      if (lookahead == 'a') ADVANCE(487);
      END_STATE();
    case 472:
      if (lookahead == 'h') ADVANCE(488);
      END_STATE();
    case 473:
      ACCEPT_TOKEN(anon_sym_drawBarChart);
      END_STATE();
    case 474:
      if (lookahead == 't') ADVANCE(489);
      END_STATE();
    case 475:
      if (lookahead == 'h') ADVANCE(490);
      END_STATE();
    case 476:
      if (lookahead == 'm') ADVANCE(491);
      END_STATE();
    case 477:
      ACCEPT_TOKEN(anon_sym_fairnessSeed);
      END_STATE();
    case 478:
      ACCEPT_TOKEN(anon_sym_filterWinner);
      END_STATE();
    case 479:
      if (lookahead == 'm') ADVANCE(492);
      END_STATE();
    case 480:
      if (lookahead == 'e') ADVANCE(493);
      END_STATE();
    case 481:
      if (lookahead == 't') ADVANCE(494);
      END_STATE();
    case 482:
      if (lookahead == 'd') ADVANCE(495);
      END_STATE();
    case 483:
      if (lookahead == 's') ADVANCE(496);
      END_STATE();
    case 484:
      if (lookahead == 'o') ADVANCE(497);
      END_STATE();
    case 485:
      if (lookahead == 'p') ADVANCE(498);
      END_STATE();
    case 486:
      if (lookahead == 'P') ADVANCE(499);
      END_STATE();
    case 487:
      if (lookahead == 'p') ADVANCE(500);
      END_STATE();
    case 488:
      if (lookahead == 'm') ADVANCE(501);
      END_STATE();
    case 489:
      ACCEPT_TOKEN(anon_sym_drawLineChart);
      END_STATE();
    case 490:
      if (lookahead == 'a') ADVANCE(502);
      END_STATE();
    case 491:
      if (lookahead == 'a') ADVANCE(503);
      END_STATE();
    case 492:
      if (lookahead == 'a') ADVANCE(504);
      END_STATE();
    case 493:
      if (lookahead == 'd') ADVANCE(505);
      END_STATE();
    case 494:
      if (lookahead == 'i') ADVANCE(506);
      END_STATE();
    case 495:
      if (lookahead == 'e') ADVANCE(507);
      END_STATE();
    case 496:
      ACCEPT_TOKEN(anon_sym_showErrorBars);
      END_STATE();
    case 497:
      if (lookahead == 'n') ADVANCE(508);
      END_STATE();
    case 498:
      ACCEPT_TOKEN(anon_sym_asyncSampleCap);
      END_STATE();
    case 499:
      if (lookahead == 'o') ADVANCE(509);
      END_STATE();
    case 500:
      ACCEPT_TOKEN(anon_sym_asyncWarmupCap);
      END_STATE();
    case 501:
      if (lookahead == 'a') ADVANCE(510);
      END_STATE();
    case 502:
      if (lookahead == 'r') ADVANCE(511);
      END_STATE();
    case 503:
      if (lookahead == 'r') ADVANCE(512);
      END_STATE();
    case 504:
      if (lookahead == 'r') ADVANCE(513);
      END_STATE();
    case 505:
      ACCEPT_TOKEN(anon_sym_iterationBased);
      END_STATE();
    case 506:
      if (lookahead == 'o') ADVANCE(514);
      END_STATE();
    case 507:
      if (lookahead == 'l') ADVANCE(515);
      END_STATE();
    case 508:
      ACCEPT_TOKEN(anon_sym_showRegression);
      END_STATE();
    case 509:
      if (lookahead == 'l') ADVANCE(516);
      END_STATE();
    case 510:
      if (lookahead == 'r') ADVANCE(517);
      END_STATE();
    case 511:
      if (lookahead == 't') ADVANCE(518);
      END_STATE();
    case 512:
      if (lookahead == 'k') ADVANCE(519);
      END_STATE();
    case 513:
      if (lookahead == 'k') ADVANCE(520);
      END_STATE();
    case 514:
      if (lookahead == 'n') ADVANCE(521);
      END_STATE();
    case 515:
      ACCEPT_TOKEN(anon_sym_regressionModel);
      END_STATE();
    case 516:
      if (lookahead == 'i') ADVANCE(522);
      END_STATE();
    case 517:
      if (lookahead == 'k') ADVANCE(523);
      END_STATE();
    case 518:
      ACCEPT_TOKEN(anon_sym_drawSpeedupChart);
      END_STATE();
    case 519:
      if (lookahead == 's') ADVANCE(524);
      END_STATE();
    case 520:
      if (lookahead == 's') ADVANCE(525);
      END_STATE();
    case 521:
      ACCEPT_TOKEN(anon_sym_outlierDetection);
      END_STATE();
    case 522:
      if (lookahead == 'c') ADVANCE(526);
      END_STATE();
    case 523:
      ACCEPT_TOKEN(anon_sym_baselineBenchmark);
      END_STATE();
    case 524:
      ACCEPT_TOKEN(anon_sym_excludeBenchmarks);
      END_STATE();
    case 525:
      ACCEPT_TOKEN(anon_sym_includeBenchmarks);
      END_STATE();
    case 526:
      if (lookahead == 'y') ADVANCE(527);
      END_STATE();
    case 527:
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
  [97] = {.lex_state = 0},
  [98] = {.lex_state = 3},
  [99] = {.lex_state = 0},
  [100] = {.lex_state = 1},
  [101] = {.lex_state = 0},
  [102] = {.lex_state = 0},
  [103] = {.lex_state = 0},
  [104] = {.lex_state = 0},
  [105] = {.lex_state = 0},
  [106] = {.lex_state = 0},
  [107] = {.lex_state = 0},
  [108] = {.lex_state = 0},
  [109] = {.lex_state = 0},
  [110] = {.lex_state = 4},
  [111] = {.lex_state = 0},
  [112] = {.lex_state = 0},
  [113] = {.lex_state = 0},
  [114] = {.lex_state = 1},
  [115] = {.lex_state = 0},
  [116] = {.lex_state = 3},
  [117] = {.lex_state = 3},
  [118] = {.lex_state = 0},
  [119] = {.lex_state = 0},
  [120] = {.lex_state = 0},
  [121] = {.lex_state = 1},
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
  [169] = {.lex_state = 0},
  [170] = {.lex_state = 0},
  [171] = {.lex_state = 0},
  [172] = {.lex_state = 0},
  [173] = {.lex_state = 0},
  [174] = {.lex_state = 0, .external_lex_state = 2},
  [175] = {.lex_state = 0},
  [176] = {.lex_state = 0},
  [177] = {.lex_state = 0, .external_lex_state = 2},
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
  [223] = {.lex_state = 2},
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
    [anon_sym_rs] = ACTIONS(1),
    [anon_sym_python] = ACTIONS(1),
    [anon_sym_py] = ACTIONS(1),
    [anon_sym_csharp] = ACTIONS(1),
    [anon_sym_cs] = ACTIONS(1),
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
    [sym_source_file] = STATE(229),
    [sym_use_statement] = STATE(80),
    [sym_global_setup] = STATE(97),
    [sym_suite] = STATE(99),
    [aux_sym_source_file_repeat1] = STATE(80),
    [aux_sym_source_file_repeat2] = STATE(99),
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
      anon_sym_bench,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(15), 45,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
      anon_sym_RBRACK,
  [57] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(21), 4,
      anon_sym_bench,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(19), 45,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
      anon_sym_RBRACK,
  [114] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(25), 4,
      anon_sym_bench,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(23), 45,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [171] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(29), 4,
      anon_sym_bench,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(27), 44,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [227] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(33), 4,
      anon_sym_bench,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(31), 44,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [283] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(37), 4,
      anon_sym_bench,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(35), 44,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [339] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(41), 4,
      anon_sym_bench,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(39), 44,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [395] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(45), 4,
      anon_sym_bench,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(43), 44,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [451] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(49), 4,
      anon_sym_bench,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(47), 44,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [507] = 15,
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
    STATE(200), 1,
      sym_language_tag,
    STATE(249), 1,
      sym_property_name,
    ACTIONS(71), 2,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(69), 7,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
    STATE(12), 10,
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
  [586] = 15,
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
    ACTIONS(73), 1,
      anon_sym_RBRACE,
    STATE(200), 1,
      sym_language_tag,
    STATE(249), 1,
      sym_property_name,
    ACTIONS(71), 2,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(69), 7,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_rs,
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
  [665] = 15,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(67), 1,
      anon_sym_fairness,
    ACTIONS(75), 1,
      anon_sym_RBRACE,
    ACTIONS(77), 1,
      anon_sym_tags,
    ACTIONS(79), 1,
      anon_sym_skip,
    ACTIONS(81), 1,
      anon_sym_validate,
    ACTIONS(83), 1,
      anon_sym_before,
    ACTIONS(85), 1,
      anon_sym_after,
    ACTIONS(87), 1,
      anon_sym_each,
    STATE(200), 1,
      sym_language_tag,
    STATE(250), 1,
      sym_property_name,
    ACTIONS(71), 2,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(69), 7,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_rs,
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
  [744] = 15,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(67), 1,
      anon_sym_fairness,
    ACTIONS(77), 1,
      anon_sym_tags,
    ACTIONS(79), 1,
      anon_sym_skip,
    ACTIONS(81), 1,
      anon_sym_validate,
    ACTIONS(83), 1,
      anon_sym_before,
    ACTIONS(85), 1,
      anon_sym_after,
    ACTIONS(87), 1,
      anon_sym_each,
    ACTIONS(89), 1,
      anon_sym_RBRACE,
    STATE(200), 1,
      sym_language_tag,
    STATE(250), 1,
      sym_property_name,
    ACTIONS(71), 2,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(69), 7,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_rs,
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
  [823] = 15,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(91), 1,
      anon_sym_RBRACE,
    ACTIONS(96), 1,
      anon_sym_hex,
    ACTIONS(99), 1,
      anon_sym_data,
    ACTIONS(102), 1,
      anon_sym_encoding,
    ACTIONS(105), 1,
      anon_sym_format,
    ACTIONS(108), 1,
      anon_sym_selector,
    ACTIONS(111), 1,
      anon_sym_shape,
    ACTIONS(114), 1,
      anon_sym_fairness,
    STATE(200), 1,
      sym_language_tag,
    STATE(249), 1,
      sym_property_name,
    ACTIONS(120), 2,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(117), 7,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_rs,
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
    ACTIONS(93), 18,
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
  [902] = 15,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(123), 1,
      anon_sym_RBRACE,
    ACTIONS(128), 1,
      anon_sym_tags,
    ACTIONS(131), 1,
      anon_sym_skip,
    ACTIONS(134), 1,
      anon_sym_validate,
    ACTIONS(137), 1,
      anon_sym_before,
    ACTIONS(140), 1,
      anon_sym_after,
    ACTIONS(143), 1,
      anon_sym_each,
    ACTIONS(146), 1,
      anon_sym_fairness,
    STATE(200), 1,
      sym_language_tag,
    STATE(250), 1,
      sym_property_name,
    ACTIONS(152), 2,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(149), 7,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_rs,
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
  [981] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(157), 4,
      anon_sym_bench,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(155), 42,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [1035] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(161), 4,
      anon_sym_async,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(159), 42,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [1089] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(165), 4,
      anon_sym_async,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(163), 42,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [1143] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(169), 3,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(167), 38,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [1192] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(175), 1,
      anon_sym_ms,
    STATE(9), 1,
      sym_duration_unit,
    ACTIONS(177), 2,
      anon_sym_s,
      anon_sym_m,
    ACTIONS(173), 3,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(171), 32,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [1245] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(175), 1,
      anon_sym_ms,
    STATE(9), 1,
      sym_duration_unit,
    ACTIONS(177), 2,
      anon_sym_s,
      anon_sym_m,
    ACTIONS(173), 3,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(171), 32,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [1298] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(181), 3,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(179), 32,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [1341] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(185), 3,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(183), 32,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [1384] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(189), 3,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(187), 32,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [1427] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(193), 3,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(191), 32,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [1470] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(197), 3,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(195), 32,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [1513] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(201), 3,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(199), 32,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [1556] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      anon_sym_globalSetup,
    ACTIONS(67), 1,
      anon_sym_fairness,
    ACTIONS(203), 1,
      anon_sym_RBRACE,
    ACTIONS(205), 1,
      anon_sym_setup,
    ACTIONS(207), 1,
      anon_sym_fixture,
    ACTIONS(209), 1,
      anon_sym_bench,
    ACTIONS(211), 1,
      anon_sym_benchAsync,
    ACTIONS(213), 1,
      anon_sym_after,
    STATE(213), 1,
      sym_property_name,
    STATE(39), 8,
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
  [1617] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(217), 3,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(215), 32,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [1660] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(221), 3,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(219), 32,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [1703] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(225), 3,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(223), 32,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [1746] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(229), 3,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(227), 32,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [1789] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(233), 3,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(231), 32,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [1832] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(237), 3,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(235), 32,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [1875] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(241), 3,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(239), 32,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [1918] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(243), 1,
      anon_sym_globalSetup,
    ACTIONS(246), 1,
      anon_sym_RBRACE,
    ACTIONS(251), 1,
      anon_sym_setup,
    ACTIONS(254), 1,
      anon_sym_fixture,
    ACTIONS(257), 1,
      anon_sym_bench,
    ACTIONS(260), 1,
      anon_sym_benchAsync,
    ACTIONS(263), 1,
      anon_sym_after,
    ACTIONS(266), 1,
      anon_sym_fairness,
    STATE(213), 1,
      sym_property_name,
    STATE(37), 8,
      sym_global_setup,
      sym__suite_item,
      sym_setup_block,
      sym_fixture,
      sym_benchmark,
      sym_after_block,
      sym_property,
      aux_sym_suite_body_repeat1,
    ACTIONS(248), 18,
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
  [1979] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(271), 3,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(269), 32,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [2022] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      anon_sym_globalSetup,
    ACTIONS(67), 1,
      anon_sym_fairness,
    ACTIONS(205), 1,
      anon_sym_setup,
    ACTIONS(207), 1,
      anon_sym_fixture,
    ACTIONS(209), 1,
      anon_sym_bench,
    ACTIONS(211), 1,
      anon_sym_benchAsync,
    ACTIONS(213), 1,
      anon_sym_after,
    ACTIONS(273), 1,
      anon_sym_RBRACE,
    STATE(213), 1,
      sym_property_name,
    STATE(37), 8,
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
  [2083] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(277), 3,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(275), 32,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [2126] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(281), 3,
      anon_sym_fairness,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(279), 32,
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
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [2169] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(175), 1,
      anon_sym_ms,
    STATE(9), 1,
      sym_duration_unit,
    ACTIONS(173), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(177), 2,
      anon_sym_s,
      anon_sym_m,
    ACTIONS(171), 24,
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
  [2213] = 3,
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
  [2250] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(289), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(287), 27,
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
  [2287] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(293), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(291), 27,
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
  [2324] = 3,
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
  [2358] = 3,
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
  [2392] = 3,
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
  [2426] = 3,
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
  [2460] = 3,
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
  [2494] = 3,
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
  [2528] = 3,
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
  [2562] = 3,
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
  [2596] = 3,
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
  [2630] = 3,
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
  [2664] = 3,
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
  [2698] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(341), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(339), 24,
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
  [2732] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(345), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(343), 24,
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
  [2766] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(347), 1,
      anon_sym_RPAREN,
    ACTIONS(351), 1,
      anon_sym_baseline,
    STATE(148), 1,
      sym_chart_param,
    STATE(226), 1,
      sym_chart_params,
    STATE(228), 1,
      sym_chart_param_name,
    ACTIONS(349), 20,
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
  [2807] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_baseline,
    ACTIONS(353), 1,
      anon_sym_RPAREN,
    STATE(175), 1,
      sym_chart_param,
    STATE(228), 1,
      sym_chart_param_name,
    ACTIONS(349), 20,
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
  [2845] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_baseline,
    ACTIONS(355), 1,
      anon_sym_RPAREN,
    STATE(175), 1,
      sym_chart_param,
    STATE(228), 1,
      sym_chart_param_name,
    ACTIONS(349), 20,
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
  [2883] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_baseline,
    STATE(175), 1,
      sym_chart_param,
    STATE(228), 1,
      sym_chart_param_name,
    ACTIONS(349), 20,
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
  [2918] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(357), 1,
      anon_sym_COLON,
    STATE(241), 1,
      sym_language_tag,
    ACTIONS(71), 2,
      anon_sym_py,
      anon_sym_cs,
    STATE(34), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(69), 7,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [2945] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(359), 1,
      anon_sym_RBRACE,
    STATE(200), 1,
      sym_language_tag,
    ACTIONS(71), 2,
      anon_sym_py,
      anon_sym_cs,
    STATE(70), 2,
      sym_language_implementation,
      aux_sym_hook_grouped_repeat1,
    ACTIONS(69), 7,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [2972] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(361), 1,
      sym_identifier,
    ACTIONS(363), 1,
      anon_sym_DQUOTE,
    ACTIONS(365), 1,
      anon_sym_SQUOTE,
    ACTIONS(367), 1,
      sym_number,
    ACTIONS(369), 1,
      sym_float,
    ACTIONS(373), 1,
      anon_sym_LBRACK,
    ACTIONS(371), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(17), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [3005] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(361), 1,
      sym_identifier,
    ACTIONS(363), 1,
      anon_sym_DQUOTE,
    ACTIONS(365), 1,
      anon_sym_SQUOTE,
    ACTIONS(369), 1,
      sym_float,
    ACTIONS(373), 1,
      anon_sym_LBRACK,
    ACTIONS(375), 1,
      sym_number,
    ACTIONS(371), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(17), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [3038] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(357), 1,
      anon_sym_COLON,
    STATE(241), 1,
      sym_language_tag,
    ACTIONS(71), 2,
      anon_sym_py,
      anon_sym_cs,
    STATE(23), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(69), 7,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [3065] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(357), 1,
      anon_sym_COLON,
    STATE(241), 1,
      sym_language_tag,
    ACTIONS(71), 2,
      anon_sym_py,
      anon_sym_cs,
    STATE(32), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(69), 7,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [3092] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(357), 1,
      anon_sym_COLON,
    STATE(241), 1,
      sym_language_tag,
    ACTIONS(71), 2,
      anon_sym_py,
      anon_sym_cs,
    STATE(33), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(69), 7,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [3119] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(377), 1,
      anon_sym_RBRACE,
    STATE(200), 1,
      sym_language_tag,
    ACTIONS(71), 2,
      anon_sym_py,
      anon_sym_cs,
    STATE(71), 2,
      sym_language_implementation,
      aux_sym_hook_grouped_repeat1,
    ACTIONS(69), 7,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [3146] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(379), 1,
      anon_sym_RBRACE,
    STATE(200), 1,
      sym_language_tag,
    ACTIONS(384), 2,
      anon_sym_py,
      anon_sym_cs,
    STATE(71), 2,
      sym_language_implementation,
      aux_sym_hook_grouped_repeat1,
    ACTIONS(381), 7,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [3173] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(357), 1,
      anon_sym_COLON,
    STATE(241), 1,
      sym_language_tag,
    ACTIONS(71), 2,
      anon_sym_py,
      anon_sym_cs,
    STATE(36), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(69), 7,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [3200] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(363), 1,
      anon_sym_DQUOTE,
    ACTIONS(365), 1,
      anon_sym_SQUOTE,
    ACTIONS(373), 1,
      anon_sym_LBRACK,
    ACTIONS(387), 1,
      sym_identifier,
    ACTIONS(389), 1,
      sym_number,
    ACTIONS(391), 1,
      sym_float,
    ACTIONS(371), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(184), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [3233] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(361), 1,
      sym_identifier,
    ACTIONS(363), 1,
      anon_sym_DQUOTE,
    ACTIONS(365), 1,
      anon_sym_SQUOTE,
    ACTIONS(369), 1,
      sym_float,
    ACTIONS(373), 1,
      anon_sym_LBRACK,
    ACTIONS(393), 1,
      sym_number,
    ACTIONS(371), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(17), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [3266] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(395), 1,
      anon_sym_RBRACE,
    ACTIONS(397), 1,
      anon_sym_declare,
    ACTIONS(399), 1,
      anon_sym_import,
    ACTIONS(401), 1,
      anon_sym_async,
    ACTIONS(403), 1,
      anon_sym_init,
    ACTIONS(405), 1,
      anon_sym_helpers,
    STATE(76), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [3296] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(397), 1,
      anon_sym_declare,
    ACTIONS(399), 1,
      anon_sym_import,
    ACTIONS(401), 1,
      anon_sym_async,
    ACTIONS(403), 1,
      anon_sym_init,
    ACTIONS(405), 1,
      anon_sym_helpers,
    ACTIONS(407), 1,
      anon_sym_RBRACE,
    STATE(77), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [3326] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(409), 1,
      anon_sym_RBRACE,
    ACTIONS(411), 1,
      anon_sym_declare,
    ACTIONS(414), 1,
      anon_sym_import,
    ACTIONS(417), 1,
      anon_sym_async,
    ACTIONS(420), 1,
      anon_sym_init,
    ACTIONS(423), 1,
      anon_sym_helpers,
    STATE(77), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [3356] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(363), 1,
      anon_sym_DQUOTE,
    ACTIONS(365), 1,
      anon_sym_SQUOTE,
    ACTIONS(373), 1,
      anon_sym_LBRACK,
    ACTIONS(426), 1,
      sym_number,
    ACTIONS(428), 1,
      sym_float,
    ACTIONS(430), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(170), 4,
      sym__chart_value,
      sym_string,
      sym_boolean,
      sym_string_array,
  [3385] = 4,
    ACTIONS(3), 1,
      sym_comment,
    STATE(168), 1,
      sym_language_tag,
    ACTIONS(71), 2,
      anon_sym_py,
      anon_sym_cs,
    ACTIONS(69), 7,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_rs,
      anon_sym_python,
      anon_sym_csharp,
  [3405] = 9,
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
    ACTIONS(432), 1,
      ts_builtin_sym_end,
    STATE(106), 1,
      sym_global_setup,
    STATE(82), 2,
      sym_use_statement,
      aux_sym_source_file_repeat1,
    STATE(94), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3435] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(434), 1,
      sym_identifier,
    ACTIONS(436), 1,
      anon_sym_RBRACE,
    ACTIONS(438), 1,
      anon_sym_anvil,
    STATE(84), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(129), 2,
      sym_anvil_call,
      sym_function_call,
  [3456] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(442), 1,
      anon_sym_use,
    STATE(82), 2,
      sym_use_statement,
      aux_sym_source_file_repeat1,
    ACTIONS(440), 4,
      ts_builtin_sym_end,
      anon_sym_globalSetup,
      anon_sym_declare,
      anon_sym_suite,
  [3473] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(434), 1,
      sym_identifier,
    ACTIONS(438), 1,
      anon_sym_anvil,
    ACTIONS(445), 1,
      anon_sym_RBRACE,
    STATE(81), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(129), 2,
      sym_anvil_call,
      sym_function_call,
  [3494] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(447), 1,
      sym_identifier,
    ACTIONS(450), 1,
      anon_sym_RBRACE,
    ACTIONS(452), 1,
      anon_sym_anvil,
    STATE(84), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(129), 2,
      sym_anvil_call,
      sym_function_call,
  [3515] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(177), 1,
      anon_sym_m,
    STATE(9), 1,
      sym_duration_unit,
    ACTIONS(171), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
    ACTIONS(175), 2,
      anon_sym_ms,
      anon_sym_s,
  [3533] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(455), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3545] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(457), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3557] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(459), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3569] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(461), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3581] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(463), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3593] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(465), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3605] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(467), 6,
      anon_sym_RBRACE,
      anon_sym_declare,
      anon_sym_import,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3617] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(469), 5,
      ts_builtin_sym_end,
      anon_sym_use,
      anon_sym_globalSetup,
      anon_sym_declare,
      anon_sym_suite,
  [3628] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_declare,
    ACTIONS(13), 1,
      anon_sym_suite,
    ACTIONS(471), 1,
      ts_builtin_sym_end,
    STATE(105), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3645] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(233), 1,
      sym_chart_function_name,
    ACTIONS(473), 4,
      anon_sym_drawSpeedupChart,
      anon_sym_drawTable,
      anon_sym_drawLineChart,
      anon_sym_drawBarChart,
  [3658] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_declare,
    ACTIONS(13), 1,
      anon_sym_suite,
    ACTIONS(475), 1,
      ts_builtin_sym_end,
    STATE(105), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3675] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_declare,
    ACTIONS(13), 1,
      anon_sym_suite,
    ACTIONS(432), 1,
      ts_builtin_sym_end,
    STATE(94), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3692] = 5,
    ACTIONS(477), 1,
      anon_sym_SQUOTE,
    ACTIONS(481), 1,
      sym_comment,
    STATE(116), 1,
      aux_sym_single_string_content_repeat1,
    STATE(217), 1,
      sym_single_string_content,
    ACTIONS(479), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [3709] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_declare,
    ACTIONS(13), 1,
      anon_sym_suite,
    ACTIONS(432), 1,
      ts_builtin_sym_end,
    STATE(105), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3726] = 5,
    ACTIONS(477), 1,
      anon_sym_DQUOTE,
    ACTIONS(481), 1,
      sym_comment,
    STATE(114), 1,
      aux_sym_string_content_repeat1,
    STATE(216), 1,
      sym_string_content,
    ACTIONS(483), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [3743] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(485), 1,
      anon_sym_LBRACE,
    STATE(133), 1,
      sym_suite_type,
    STATE(141), 1,
      sym_suite_body,
    ACTIONS(487), 2,
      anon_sym_performance,
      anon_sym_memory,
  [3760] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(363), 1,
      anon_sym_DQUOTE,
    ACTIONS(365), 1,
      anon_sym_SQUOTE,
    ACTIONS(489), 1,
      anon_sym_ATfile,
    STATE(27), 2,
      sym_file_ref,
      sym_string,
  [3777] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(485), 1,
      anon_sym_LBRACE,
    STATE(151), 1,
      sym_suite_type,
    STATE(163), 1,
      sym_suite_body,
    ACTIONS(487), 2,
      anon_sym_performance,
      anon_sym_memory,
  [3794] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(363), 1,
      anon_sym_DQUOTE,
    ACTIONS(365), 1,
      anon_sym_SQUOTE,
    ACTIONS(489), 1,
      anon_sym_ATfile,
    STATE(28), 2,
      sym_file_ref,
      sym_string,
  [3811] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(491), 1,
      ts_builtin_sym_end,
    ACTIONS(493), 1,
      anon_sym_declare,
    ACTIONS(496), 1,
      anon_sym_suite,
    STATE(105), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3828] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_declare,
    ACTIONS(13), 1,
      anon_sym_suite,
    ACTIONS(471), 1,
      ts_builtin_sym_end,
    STATE(96), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3845] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(499), 1,
      anon_sym_LBRACE,
    ACTIONS(501), 1,
      anon_sym_LPAREN,
    STATE(88), 2,
      sym_code_block,
      sym_paren_code_block,
  [3859] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(363), 1,
      anon_sym_DQUOTE,
    ACTIONS(365), 1,
      anon_sym_SQUOTE,
    ACTIONS(503), 1,
      anon_sym_RBRACK,
    STATE(140), 1,
      sym_string,
  [3875] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(363), 1,
      anon_sym_DQUOTE,
    ACTIONS(365), 1,
      anon_sym_SQUOTE,
    ACTIONS(505), 1,
      anon_sym_RBRACK,
    STATE(188), 1,
      sym_string,
  [3891] = 4,
    ACTIONS(481), 1,
      sym_comment,
    ACTIONS(507), 1,
      anon_sym_LBRACE,
    ACTIONS(509), 1,
      sym_inline_code,
    STATE(35), 2,
      sym__code_or_inline,
      sym_code_block,
  [3905] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(511), 1,
      anon_sym_RBRACE,
    ACTIONS(513), 1,
      anon_sym_charting,
    STATE(124), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [3919] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(515), 1,
      sym_identifier,
    ACTIONS(517), 1,
      anon_sym_RPAREN,
    STATE(161), 1,
      sym_argument,
    STATE(246), 1,
      sym_argument_list,
  [3935] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(515), 1,
      sym_identifier,
    ACTIONS(519), 1,
      anon_sym_RPAREN,
    STATE(161), 1,
      sym_argument,
    STATE(218), 1,
      sym_argument_list,
  [3951] = 4,
    ACTIONS(481), 1,
      sym_comment,
    ACTIONS(521), 1,
      anon_sym_DQUOTE,
    STATE(121), 1,
      aux_sym_string_content_repeat1,
    ACTIONS(523), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [3965] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(363), 1,
      anon_sym_DQUOTE,
    ACTIONS(365), 1,
      anon_sym_SQUOTE,
    ACTIONS(525), 1,
      anon_sym_RBRACK,
    STATE(188), 1,
      sym_string,
  [3981] = 4,
    ACTIONS(481), 1,
      sym_comment,
    ACTIONS(527), 1,
      anon_sym_SQUOTE,
    STATE(117), 1,
      aux_sym_single_string_content_repeat1,
    ACTIONS(529), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [3995] = 4,
    ACTIONS(481), 1,
      sym_comment,
    ACTIONS(531), 1,
      anon_sym_SQUOTE,
    STATE(117), 1,
      aux_sym_single_string_content_repeat1,
    ACTIONS(533), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [4009] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(513), 1,
      anon_sym_charting,
    ACTIONS(536), 1,
      anon_sym_RBRACE,
    STATE(111), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [4023] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(538), 1,
      anon_sym_LBRACE,
    ACTIONS(540), 1,
      anon_sym_LPAREN,
    STATE(52), 1,
      sym_fixture_body,
    STATE(179), 1,
      sym_fixture_params,
  [4039] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(363), 1,
      anon_sym_DQUOTE,
    ACTIONS(365), 1,
      anon_sym_SQUOTE,
    ACTIONS(542), 1,
      sym_identifier,
    STATE(26), 1,
      sym_string,
  [4055] = 4,
    ACTIONS(481), 1,
      sym_comment,
    ACTIONS(544), 1,
      anon_sym_DQUOTE,
    STATE(121), 1,
      aux_sym_string_content_repeat1,
    ACTIONS(546), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [4069] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(363), 1,
      anon_sym_DQUOTE,
    ACTIONS(365), 1,
      anon_sym_SQUOTE,
    ACTIONS(549), 1,
      sym_identifier,
    STATE(25), 1,
      sym_string,
  [4085] = 4,
    ACTIONS(481), 1,
      sym_comment,
    ACTIONS(507), 1,
      anon_sym_LBRACE,
    ACTIONS(551), 1,
      sym_inline_code,
    STATE(20), 2,
      sym__code_or_inline,
      sym_code_block,
  [4099] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(553), 1,
      anon_sym_RBRACE,
    ACTIONS(555), 1,
      anon_sym_charting,
    STATE(124), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [4113] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(363), 1,
      anon_sym_DQUOTE,
    ACTIONS(365), 1,
      anon_sym_SQUOTE,
    STATE(188), 1,
      sym_string,
  [4126] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(558), 1,
      sym_identifier,
    ACTIONS(560), 1,
      anon_sym_RPAREN,
    STATE(190), 1,
      sym_fixture_param,
  [4139] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(562), 1,
      anon_sym_RPAREN,
    ACTIONS(564), 1,
      anon_sym_COMMA,
    STATE(127), 1,
      aux_sym_fixture_params_repeat1,
  [4152] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(567), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [4161] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(571), 1,
      anon_sym_RBRACE,
    ACTIONS(569), 2,
      anon_sym_anvil,
      sym_identifier,
  [4172] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(573), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [4181] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(575), 1,
      anon_sym_RPAREN,
    ACTIONS(577), 1,
      anon_sym_COMMA,
    STATE(154), 1,
      aux_sym_fixture_params_repeat1,
  [4194] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(525), 1,
      anon_sym_RBRACK,
    ACTIONS(579), 1,
      anon_sym_COMMA,
    STATE(138), 1,
      aux_sym_string_array_repeat1,
  [4207] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(212), 1,
      sym_run_mode,
    ACTIONS(581), 2,
      anon_sym_timeBased,
      anon_sym_iterationBased,
  [4218] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(585), 1,
      anon_sym_RBRACE,
    ACTIONS(583), 2,
      anon_sym_anvil,
      sym_identifier,
  [4229] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(178), 1,
      sym_boolean,
    ACTIONS(430), 2,
      anon_sym_true,
      anon_sym_false,
  [4240] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(587), 1,
      anon_sym_RPAREN,
    ACTIONS(589), 1,
      anon_sym_COMMA,
    STATE(136), 1,
      aux_sym_argument_list_repeat1,
  [4253] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(515), 1,
      sym_identifier,
    ACTIONS(592), 1,
      anon_sym_RPAREN,
    STATE(185), 1,
      sym_argument,
  [4266] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(594), 1,
      anon_sym_COMMA,
    ACTIONS(597), 1,
      anon_sym_RBRACK,
    STATE(138), 1,
      aux_sym_string_array_repeat1,
  [4279] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(601), 1,
      anon_sym_RBRACE,
    ACTIONS(599), 2,
      anon_sym_anvil,
      sym_identifier,
  [4290] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(603), 1,
      anon_sym_COMMA,
    ACTIONS(605), 1,
      anon_sym_RBRACK,
    STATE(132), 1,
      aux_sym_string_array_repeat1,
  [4303] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(607), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [4312] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(363), 1,
      anon_sym_DQUOTE,
    ACTIONS(365), 1,
      anon_sym_SQUOTE,
    STATE(215), 1,
      sym_string,
  [4325] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(611), 1,
      anon_sym_RBRACE,
    ACTIONS(609), 2,
      anon_sym_anvil,
      sym_identifier,
  [4336] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(615), 1,
      anon_sym_RBRACE,
    ACTIONS(613), 2,
      anon_sym_anvil,
      sym_identifier,
  [4347] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(363), 1,
      anon_sym_DQUOTE,
    ACTIONS(365), 1,
      anon_sym_SQUOTE,
    STATE(231), 1,
      sym_string,
  [4360] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(619), 1,
      anon_sym_RBRACE,
    ACTIONS(617), 2,
      anon_sym_anvil,
      sym_identifier,
  [4371] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(182), 1,
      sym_boolean,
    ACTIONS(430), 2,
      anon_sym_true,
      anon_sym_false,
  [4382] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(621), 1,
      anon_sym_RPAREN,
    ACTIONS(623), 1,
      anon_sym_COMMA,
    STATE(157), 1,
      aux_sym_chart_params_repeat1,
  [4395] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(558), 1,
      sym_identifier,
    ACTIONS(625), 1,
      anon_sym_RPAREN,
    STATE(190), 1,
      sym_fixture_param,
  [4408] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(558), 1,
      sym_identifier,
    ACTIONS(627), 1,
      anon_sym_RPAREN,
    STATE(131), 1,
      sym_fixture_param,
  [4421] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(245), 1,
      sym_run_mode,
    ACTIONS(581), 2,
      anon_sym_timeBased,
      anon_sym_iterationBased,
  [4432] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(629), 1,
      anon_sym_RPAREN,
    ACTIONS(631), 1,
      anon_sym_COMMA,
    STATE(136), 1,
      aux_sym_argument_list_repeat1,
  [4445] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(515), 1,
      sym_identifier,
    ACTIONS(629), 1,
      anon_sym_RPAREN,
    STATE(185), 1,
      sym_argument,
  [4458] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(625), 1,
      anon_sym_RPAREN,
    ACTIONS(633), 1,
      anon_sym_COMMA,
    STATE(127), 1,
      aux_sym_fixture_params_repeat1,
  [4471] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(637), 1,
      anon_sym_RBRACE,
    ACTIONS(635), 2,
      anon_sym_anvil,
      sym_identifier,
  [4482] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(639), 1,
      anon_sym_RPAREN,
    ACTIONS(641), 1,
      anon_sym_fork,
    STATE(194), 1,
      sym_anvil_args,
  [4495] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(353), 1,
      anon_sym_RPAREN,
    ACTIONS(643), 1,
      anon_sym_COMMA,
    STATE(159), 1,
      aux_sym_chart_params_repeat1,
  [4508] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(645), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [4517] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(647), 1,
      anon_sym_RPAREN,
    ACTIONS(649), 1,
      anon_sym_COMMA,
    STATE(159), 1,
      aux_sym_chart_params_repeat1,
  [4530] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(363), 1,
      anon_sym_DQUOTE,
    ACTIONS(365), 1,
      anon_sym_SQUOTE,
    STATE(40), 1,
      sym_string,
  [4543] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(652), 1,
      anon_sym_RPAREN,
    ACTIONS(654), 1,
      anon_sym_COMMA,
    STATE(152), 1,
      aux_sym_argument_list_repeat1,
  [4556] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(656), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [4565] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(658), 3,
      ts_builtin_sym_end,
      anon_sym_declare,
      anon_sym_suite,
  [4574] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(499), 1,
      anon_sym_LBRACE,
    STATE(87), 1,
      sym_code_block,
  [4584] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(660), 1,
      anon_sym_DOT,
    ACTIONS(662), 1,
      anon_sym_LPAREN,
  [4594] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(664), 2,
      anon_sym_LBRACE,
      anon_sym_COLON,
  [4602] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(499), 1,
      anon_sym_LBRACE,
    STATE(41), 1,
      sym_code_block,
  [4612] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(666), 1,
      anon_sym_LBRACE,
    STATE(55), 1,
      sym_setup_body,
  [4622] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(373), 1,
      anon_sym_LBRACK,
    STATE(24), 1,
      sym_string_array,
  [4632] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(668), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4640] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(670), 1,
      anon_sym_LBRACE,
    STATE(49), 1,
      sym_benchmark_body,
  [4650] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(672), 1,
      anon_sym_LBRACE,
    STATE(45), 1,
      sym_global_setup_body,
  [4660] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(499), 1,
      anon_sym_LBRACE,
    STATE(92), 1,
      sym_code_block,
  [4670] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(674), 1,
      anon_sym_RPAREN,
    ACTIONS(676), 1,
      sym_embedded_code,
  [4680] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(647), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4688] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(678), 2,
      anon_sym_RBRACE,
      anon_sym_charting,
  [4696] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(680), 1,
      anon_sym_RBRACE,
    ACTIONS(682), 1,
      sym_embedded_code,
  [4706] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(485), 1,
      anon_sym_LBRACE,
    STATE(128), 1,
      sym_suite_body,
  [4716] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(538), 1,
      anon_sym_LBRACE,
    STATE(46), 1,
      sym_fixture_body,
  [4726] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(684), 2,
      anon_sym_RBRACE,
      anon_sym_charting,
  [4734] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(515), 1,
      sym_identifier,
    STATE(185), 1,
      sym_argument,
  [4744] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(485), 1,
      anon_sym_LBRACE,
    STATE(130), 1,
      sym_suite_body,
  [4754] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(558), 1,
      sym_identifier,
    STATE(190), 1,
      sym_fixture_param,
  [4764] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(686), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4772] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(587), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4780] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(499), 1,
      anon_sym_LBRACE,
    STATE(89), 1,
      sym_code_block,
  [4790] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(688), 1,
      anon_sym_LBRACE,
    STATE(58), 1,
      sym_after_body,
  [4800] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(597), 2,
      anon_sym_COMMA,
      anon_sym_RBRACK,
  [4808] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(499), 1,
      anon_sym_LBRACE,
    STATE(86), 1,
      sym_code_block,
  [4818] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(562), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4826] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(690), 2,
      anon_sym_timeBased,
      anon_sym_iterationBased,
  [4834] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(692), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4842] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(694), 1,
      anon_sym_COLON,
  [4849] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(696), 1,
      anon_sym_RPAREN,
  [4856] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(698), 1,
      anon_sym_sameDataset,
  [4863] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(700), 1,
      anon_sym_COLON,
  [4870] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(702), 1,
      anon_sym_LPAREN,
  [4877] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(704), 1,
      anon_sym_COLON,
  [4884] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(706), 1,
      anon_sym_LBRACE,
  [4891] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(708), 1,
      anon_sym_COLON,
  [4898] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(710), 1,
      anon_sym_DOT,
  [4905] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(712), 1,
      sym_identifier,
  [4912] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(714), 1,
      anon_sym_COLON,
  [4919] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(716), 1,
      anon_sym_LBRACE,
  [4926] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(718), 1,
      anon_sym_LBRACE,
  [4933] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(720), 1,
      anon_sym_COLON,
  [4940] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(722), 1,
      sym_identifier,
  [4947] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(724), 1,
      anon_sym_COLON,
  [4954] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(726), 1,
      anon_sym_COLON,
  [4961] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(728), 1,
      anon_sym_COLON,
  [4968] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(730), 1,
      anon_sym_RBRACE,
  [4975] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(732), 1,
      anon_sym_sameDataset,
  [4982] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(734), 1,
      anon_sym_COLON,
  [4989] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(736), 1,
      anon_sym_init,
  [4996] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(738), 1,
      anon_sym_RPAREN,
  [5003] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(740), 1,
      anon_sym_DQUOTE,
  [5010] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(740), 1,
      anon_sym_SQUOTE,
  [5017] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(742), 1,
      anon_sym_RPAREN,
  [5024] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(744), 1,
      anon_sym_LBRACE,
  [5031] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(746), 1,
      sym_identifier,
  [5038] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(748), 1,
      sym_identifier,
  [5045] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(750), 1,
      anon_sym_RPAREN,
  [5052] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(752), 1,
      anon_sym_COLON_COLON,
  [5059] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(754), 1,
      sym_identifier,
  [5066] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(756), 1,
      anon_sym_COLON,
  [5073] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(758), 1,
      anon_sym_RPAREN,
  [5080] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(760), 1,
      anon_sym_COLON,
  [5087] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(762), 1,
      anon_sym_COLON,
  [5094] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(764), 1,
      ts_builtin_sym_end,
  [5101] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(766), 1,
      anon_sym_DOT,
  [5108] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(768), 1,
      anon_sym_RPAREN,
  [5115] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(770), 1,
      sym_identifier,
  [5122] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(772), 1,
      anon_sym_LPAREN,
  [5129] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(774), 1,
      anon_sym_LPAREN,
  [5136] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(776), 1,
      anon_sym_suite,
  [5143] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(778), 1,
      anon_sym_COLON,
  [5150] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(780), 1,
      anon_sym_COLON,
  [5157] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(782), 1,
      anon_sym_std,
  [5164] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(784), 1,
      anon_sym_spawnAnvil,
  [5171] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(786), 1,
      sym_identifier,
  [5178] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(788), 1,
      anon_sym_COLON,
  [5185] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(790), 1,
      anon_sym_COLON,
  [5192] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(792), 1,
      anon_sym_LBRACE,
  [5199] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(794), 1,
      anon_sym_LPAREN,
  [5206] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(796), 1,
      anon_sym_sameDataset,
  [5213] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(798), 1,
      anon_sym_RPAREN,
  [5220] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(800), 1,
      anon_sym_COLON,
  [5227] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(802), 1,
      anon_sym_LPAREN,
  [5234] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(804), 1,
      anon_sym_COLON,
  [5241] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(806), 1,
      anon_sym_COLON,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 57,
  [SMALL_STATE(4)] = 114,
  [SMALL_STATE(5)] = 171,
  [SMALL_STATE(6)] = 227,
  [SMALL_STATE(7)] = 283,
  [SMALL_STATE(8)] = 339,
  [SMALL_STATE(9)] = 395,
  [SMALL_STATE(10)] = 451,
  [SMALL_STATE(11)] = 507,
  [SMALL_STATE(12)] = 586,
  [SMALL_STATE(13)] = 665,
  [SMALL_STATE(14)] = 744,
  [SMALL_STATE(15)] = 823,
  [SMALL_STATE(16)] = 902,
  [SMALL_STATE(17)] = 981,
  [SMALL_STATE(18)] = 1035,
  [SMALL_STATE(19)] = 1089,
  [SMALL_STATE(20)] = 1143,
  [SMALL_STATE(21)] = 1192,
  [SMALL_STATE(22)] = 1245,
  [SMALL_STATE(23)] = 1298,
  [SMALL_STATE(24)] = 1341,
  [SMALL_STATE(25)] = 1384,
  [SMALL_STATE(26)] = 1427,
  [SMALL_STATE(27)] = 1470,
  [SMALL_STATE(28)] = 1513,
  [SMALL_STATE(29)] = 1556,
  [SMALL_STATE(30)] = 1617,
  [SMALL_STATE(31)] = 1660,
  [SMALL_STATE(32)] = 1703,
  [SMALL_STATE(33)] = 1746,
  [SMALL_STATE(34)] = 1789,
  [SMALL_STATE(35)] = 1832,
  [SMALL_STATE(36)] = 1875,
  [SMALL_STATE(37)] = 1918,
  [SMALL_STATE(38)] = 1979,
  [SMALL_STATE(39)] = 2022,
  [SMALL_STATE(40)] = 2083,
  [SMALL_STATE(41)] = 2126,
  [SMALL_STATE(42)] = 2169,
  [SMALL_STATE(43)] = 2213,
  [SMALL_STATE(44)] = 2250,
  [SMALL_STATE(45)] = 2287,
  [SMALL_STATE(46)] = 2324,
  [SMALL_STATE(47)] = 2358,
  [SMALL_STATE(48)] = 2392,
  [SMALL_STATE(49)] = 2426,
  [SMALL_STATE(50)] = 2460,
  [SMALL_STATE(51)] = 2494,
  [SMALL_STATE(52)] = 2528,
  [SMALL_STATE(53)] = 2562,
  [SMALL_STATE(54)] = 2596,
  [SMALL_STATE(55)] = 2630,
  [SMALL_STATE(56)] = 2664,
  [SMALL_STATE(57)] = 2698,
  [SMALL_STATE(58)] = 2732,
  [SMALL_STATE(59)] = 2766,
  [SMALL_STATE(60)] = 2807,
  [SMALL_STATE(61)] = 2845,
  [SMALL_STATE(62)] = 2883,
  [SMALL_STATE(63)] = 2918,
  [SMALL_STATE(64)] = 2945,
  [SMALL_STATE(65)] = 2972,
  [SMALL_STATE(66)] = 3005,
  [SMALL_STATE(67)] = 3038,
  [SMALL_STATE(68)] = 3065,
  [SMALL_STATE(69)] = 3092,
  [SMALL_STATE(70)] = 3119,
  [SMALL_STATE(71)] = 3146,
  [SMALL_STATE(72)] = 3173,
  [SMALL_STATE(73)] = 3200,
  [SMALL_STATE(74)] = 3233,
  [SMALL_STATE(75)] = 3266,
  [SMALL_STATE(76)] = 3296,
  [SMALL_STATE(77)] = 3326,
  [SMALL_STATE(78)] = 3356,
  [SMALL_STATE(79)] = 3385,
  [SMALL_STATE(80)] = 3405,
  [SMALL_STATE(81)] = 3435,
  [SMALL_STATE(82)] = 3456,
  [SMALL_STATE(83)] = 3473,
  [SMALL_STATE(84)] = 3494,
  [SMALL_STATE(85)] = 3515,
  [SMALL_STATE(86)] = 3533,
  [SMALL_STATE(87)] = 3545,
  [SMALL_STATE(88)] = 3557,
  [SMALL_STATE(89)] = 3569,
  [SMALL_STATE(90)] = 3581,
  [SMALL_STATE(91)] = 3593,
  [SMALL_STATE(92)] = 3605,
  [SMALL_STATE(93)] = 3617,
  [SMALL_STATE(94)] = 3628,
  [SMALL_STATE(95)] = 3645,
  [SMALL_STATE(96)] = 3658,
  [SMALL_STATE(97)] = 3675,
  [SMALL_STATE(98)] = 3692,
  [SMALL_STATE(99)] = 3709,
  [SMALL_STATE(100)] = 3726,
  [SMALL_STATE(101)] = 3743,
  [SMALL_STATE(102)] = 3760,
  [SMALL_STATE(103)] = 3777,
  [SMALL_STATE(104)] = 3794,
  [SMALL_STATE(105)] = 3811,
  [SMALL_STATE(106)] = 3828,
  [SMALL_STATE(107)] = 3845,
  [SMALL_STATE(108)] = 3859,
  [SMALL_STATE(109)] = 3875,
  [SMALL_STATE(110)] = 3891,
  [SMALL_STATE(111)] = 3905,
  [SMALL_STATE(112)] = 3919,
  [SMALL_STATE(113)] = 3935,
  [SMALL_STATE(114)] = 3951,
  [SMALL_STATE(115)] = 3965,
  [SMALL_STATE(116)] = 3981,
  [SMALL_STATE(117)] = 3995,
  [SMALL_STATE(118)] = 4009,
  [SMALL_STATE(119)] = 4023,
  [SMALL_STATE(120)] = 4039,
  [SMALL_STATE(121)] = 4055,
  [SMALL_STATE(122)] = 4069,
  [SMALL_STATE(123)] = 4085,
  [SMALL_STATE(124)] = 4099,
  [SMALL_STATE(125)] = 4113,
  [SMALL_STATE(126)] = 4126,
  [SMALL_STATE(127)] = 4139,
  [SMALL_STATE(128)] = 4152,
  [SMALL_STATE(129)] = 4161,
  [SMALL_STATE(130)] = 4172,
  [SMALL_STATE(131)] = 4181,
  [SMALL_STATE(132)] = 4194,
  [SMALL_STATE(133)] = 4207,
  [SMALL_STATE(134)] = 4218,
  [SMALL_STATE(135)] = 4229,
  [SMALL_STATE(136)] = 4240,
  [SMALL_STATE(137)] = 4253,
  [SMALL_STATE(138)] = 4266,
  [SMALL_STATE(139)] = 4279,
  [SMALL_STATE(140)] = 4290,
  [SMALL_STATE(141)] = 4303,
  [SMALL_STATE(142)] = 4312,
  [SMALL_STATE(143)] = 4325,
  [SMALL_STATE(144)] = 4336,
  [SMALL_STATE(145)] = 4347,
  [SMALL_STATE(146)] = 4360,
  [SMALL_STATE(147)] = 4371,
  [SMALL_STATE(148)] = 4382,
  [SMALL_STATE(149)] = 4395,
  [SMALL_STATE(150)] = 4408,
  [SMALL_STATE(151)] = 4421,
  [SMALL_STATE(152)] = 4432,
  [SMALL_STATE(153)] = 4445,
  [SMALL_STATE(154)] = 4458,
  [SMALL_STATE(155)] = 4471,
  [SMALL_STATE(156)] = 4482,
  [SMALL_STATE(157)] = 4495,
  [SMALL_STATE(158)] = 4508,
  [SMALL_STATE(159)] = 4517,
  [SMALL_STATE(160)] = 4530,
  [SMALL_STATE(161)] = 4543,
  [SMALL_STATE(162)] = 4556,
  [SMALL_STATE(163)] = 4565,
  [SMALL_STATE(164)] = 4574,
  [SMALL_STATE(165)] = 4584,
  [SMALL_STATE(166)] = 4594,
  [SMALL_STATE(167)] = 4602,
  [SMALL_STATE(168)] = 4612,
  [SMALL_STATE(169)] = 4622,
  [SMALL_STATE(170)] = 4632,
  [SMALL_STATE(171)] = 4640,
  [SMALL_STATE(172)] = 4650,
  [SMALL_STATE(173)] = 4660,
  [SMALL_STATE(174)] = 4670,
  [SMALL_STATE(175)] = 4680,
  [SMALL_STATE(176)] = 4688,
  [SMALL_STATE(177)] = 4696,
  [SMALL_STATE(178)] = 4706,
  [SMALL_STATE(179)] = 4716,
  [SMALL_STATE(180)] = 4726,
  [SMALL_STATE(181)] = 4734,
  [SMALL_STATE(182)] = 4744,
  [SMALL_STATE(183)] = 4754,
  [SMALL_STATE(184)] = 4764,
  [SMALL_STATE(185)] = 4772,
  [SMALL_STATE(186)] = 4780,
  [SMALL_STATE(187)] = 4790,
  [SMALL_STATE(188)] = 4800,
  [SMALL_STATE(189)] = 4808,
  [SMALL_STATE(190)] = 4818,
  [SMALL_STATE(191)] = 4826,
  [SMALL_STATE(192)] = 4834,
  [SMALL_STATE(193)] = 4842,
  [SMALL_STATE(194)] = 4849,
  [SMALL_STATE(195)] = 4856,
  [SMALL_STATE(196)] = 4863,
  [SMALL_STATE(197)] = 4870,
  [SMALL_STATE(198)] = 4877,
  [SMALL_STATE(199)] = 4884,
  [SMALL_STATE(200)] = 4891,
  [SMALL_STATE(201)] = 4898,
  [SMALL_STATE(202)] = 4905,
  [SMALL_STATE(203)] = 4912,
  [SMALL_STATE(204)] = 4919,
  [SMALL_STATE(205)] = 4926,
  [SMALL_STATE(206)] = 4933,
  [SMALL_STATE(207)] = 4940,
  [SMALL_STATE(208)] = 4947,
  [SMALL_STATE(209)] = 4954,
  [SMALL_STATE(210)] = 4961,
  [SMALL_STATE(211)] = 4968,
  [SMALL_STATE(212)] = 4975,
  [SMALL_STATE(213)] = 4982,
  [SMALL_STATE(214)] = 4989,
  [SMALL_STATE(215)] = 4996,
  [SMALL_STATE(216)] = 5003,
  [SMALL_STATE(217)] = 5010,
  [SMALL_STATE(218)] = 5017,
  [SMALL_STATE(219)] = 5024,
  [SMALL_STATE(220)] = 5031,
  [SMALL_STATE(221)] = 5038,
  [SMALL_STATE(222)] = 5045,
  [SMALL_STATE(223)] = 5052,
  [SMALL_STATE(224)] = 5059,
  [SMALL_STATE(225)] = 5066,
  [SMALL_STATE(226)] = 5073,
  [SMALL_STATE(227)] = 5080,
  [SMALL_STATE(228)] = 5087,
  [SMALL_STATE(229)] = 5094,
  [SMALL_STATE(230)] = 5101,
  [SMALL_STATE(231)] = 5108,
  [SMALL_STATE(232)] = 5115,
  [SMALL_STATE(233)] = 5122,
  [SMALL_STATE(234)] = 5129,
  [SMALL_STATE(235)] = 5136,
  [SMALL_STATE(236)] = 5143,
  [SMALL_STATE(237)] = 5150,
  [SMALL_STATE(238)] = 5157,
  [SMALL_STATE(239)] = 5164,
  [SMALL_STATE(240)] = 5171,
  [SMALL_STATE(241)] = 5178,
  [SMALL_STATE(242)] = 5185,
  [SMALL_STATE(243)] = 5192,
  [SMALL_STATE(244)] = 5199,
  [SMALL_STATE(245)] = 5206,
  [SMALL_STATE(246)] = 5213,
  [SMALL_STATE(247)] = 5220,
  [SMALL_STATE(248)] = 5227,
  [SMALL_STATE(249)] = 5234,
  [SMALL_STATE(250)] = 5241,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0, 0, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(238),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(172),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(235),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(232),
  [15] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 3, 0, 0),
  [17] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 3, 0, 0),
  [19] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 2, 0, 0),
  [21] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 2, 0, 0),
  [23] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_boolean, 1, 0, 0),
  [25] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_boolean, 1, 0, 0),
  [27] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_duration_unit, 1, 0, 0),
  [29] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_duration_unit, 1, 0, 0),
  [31] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 5, 0, 0),
  [33] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 5, 0, 0),
  [35] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 4, 0, 0),
  [37] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 4, 0, 0),
  [39] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 3, 0, 0),
  [41] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 3, 0, 0),
  [43] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_duration, 2, 0, 0),
  [45] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_duration, 2, 0, 0),
  [47] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 2, 0, 0),
  [49] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 2, 0, 0),
  [51] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [53] = {.entry = {.count = 1, .reusable = true}}, SHIFT(242),
  [55] = {.entry = {.count = 1, .reusable = true}}, SHIFT(210),
  [57] = {.entry = {.count = 1, .reusable = true}}, SHIFT(209),
  [59] = {.entry = {.count = 1, .reusable = true}}, SHIFT(208),
  [61] = {.entry = {.count = 1, .reusable = true}}, SHIFT(193),
  [63] = {.entry = {.count = 1, .reusable = true}}, SHIFT(206),
  [65] = {.entry = {.count = 1, .reusable = true}}, SHIFT(203),
  [67] = {.entry = {.count = 1, .reusable = false}}, SHIFT(242),
  [69] = {.entry = {.count = 1, .reusable = true}}, SHIFT(166),
  [71] = {.entry = {.count = 1, .reusable = false}}, SHIFT(166),
  [73] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [75] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [77] = {.entry = {.count = 1, .reusable = true}}, SHIFT(196),
  [79] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [81] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [83] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [85] = {.entry = {.count = 1, .reusable = true}}, SHIFT(68),
  [87] = {.entry = {.count = 1, .reusable = true}}, SHIFT(67),
  [89] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [91] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0),
  [93] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(242),
  [96] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(210),
  [99] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(209),
  [102] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(208),
  [105] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(193),
  [108] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(206),
  [111] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(203),
  [114] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(242),
  [117] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(166),
  [120] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(166),
  [123] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0),
  [125] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(242),
  [128] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(196),
  [131] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(72),
  [134] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(63),
  [137] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(69),
  [140] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(68),
  [143] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(67),
  [146] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(242),
  [149] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(166),
  [152] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(166),
  [155] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property, 3, 0, 5),
  [157] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_property, 3, 0, 5),
  [159] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_code_block, 3, 0, 0),
  [161] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_code_block, 3, 0, 0),
  [163] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_code_block, 2, 0, 0),
  [165] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_code_block, 2, 0, 0),
  [167] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_language_implementation, 3, 0, 8),
  [169] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_language_implementation, 3, 0, 8),
  [171] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__value, 1, 0, 0),
  [173] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__value, 1, 0, 0),
  [175] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [177] = {.entry = {.count = 1, .reusable = false}}, SHIFT(5),
  [179] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_each_hook, 2, 0, 0),
  [181] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_each_hook, 2, 0, 0),
  [183] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tags_property, 3, 0, 0),
  [185] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_tags_property, 3, 0, 0),
  [187] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_format_property, 3, 0, 0),
  [189] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_format_property, 3, 0, 0),
  [191] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_encoding_property, 3, 0, 0),
  [193] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_encoding_property, 3, 0, 0),
  [195] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_data_property, 3, 0, 0),
  [197] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_data_property, 3, 0, 0),
  [199] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hex_property, 3, 0, 0),
  [201] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hex_property, 3, 0, 0),
  [203] = {.entry = {.count = 1, .reusable = true}}, SHIFT(162),
  [205] = {.entry = {.count = 1, .reusable = true}}, SHIFT(79),
  [207] = {.entry = {.count = 1, .reusable = true}}, SHIFT(224),
  [209] = {.entry = {.count = 1, .reusable = false}}, SHIFT(220),
  [211] = {.entry = {.count = 1, .reusable = true}}, SHIFT(220),
  [213] = {.entry = {.count = 1, .reusable = true}}, SHIFT(187),
  [215] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_file_ref, 4, 0, 0),
  [217] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_file_ref, 4, 0, 0),
  [219] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_grouped, 4, 0, 0),
  [221] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hook_grouped, 4, 0, 0),
  [223] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_hook, 2, 0, 0),
  [225] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_hook, 2, 0, 0),
  [227] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_before_hook, 2, 0, 0),
  [229] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_before_hook, 2, 0, 0),
  [231] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_validate_hook, 2, 0, 0),
  [233] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_validate_hook, 2, 0, 0),
  [235] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_flat, 3, 0, 8),
  [237] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hook_flat, 3, 0, 8),
  [239] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_skip_hook, 2, 0, 0),
  [241] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_skip_hook, 2, 0, 0),
  [243] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(172),
  [246] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0),
  [248] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(242),
  [251] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(79),
  [254] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(224),
  [257] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(220),
  [260] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(220),
  [263] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(187),
  [266] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(242),
  [269] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_grouped, 3, 0, 0),
  [271] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hook_grouped, 3, 0, 0),
  [273] = {.entry = {.count = 1, .reusable = true}}, SHIFT(158),
  [275] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selector_property, 3, 0, 0),
  [277] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_selector_property, 3, 0, 0),
  [279] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_shape_property, 3, 0, 0),
  [281] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_shape_property, 3, 0, 0),
  [283] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_body, 3, 0, 0),
  [285] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup_body, 3, 0, 0),
  [287] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_body, 2, 0, 0),
  [289] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup_body, 2, 0, 0),
  [291] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup, 2, 0, 0),
  [293] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup, 2, 0, 0),
  [295] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture, 4, 0, 1),
  [297] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture, 4, 0, 1),
  [299] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_body, 3, 0, 0),
  [301] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_setup_body, 3, 0, 0),
  [303] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_body, 3, 0, 0),
  [305] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture_body, 3, 0, 0),
  [307] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark, 3, 0, 1),
  [309] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_benchmark, 3, 0, 1),
  [311] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_body, 2, 0, 0),
  [313] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_body, 2, 0, 0),
  [315] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_body, 2, 0, 0),
  [317] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_setup_body, 2, 0, 0),
  [319] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture, 3, 0, 1),
  [321] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture, 3, 0, 1),
  [323] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark_body, 2, 0, 0),
  [325] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_benchmark_body, 2, 0, 0),
  [327] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_body, 3, 0, 0),
  [329] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_body, 3, 0, 0),
  [331] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_block, 3, 0, 4),
  [333] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_setup_block, 3, 0, 4),
  [335] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark_body, 3, 0, 0),
  [337] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_benchmark_body, 3, 0, 0),
  [339] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_body, 2, 0, 0),
  [341] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture_body, 2, 0, 0),
  [343] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_block, 2, 0, 0),
  [345] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_block, 2, 0, 0),
  [347] = {.entry = {.count = 1, .reusable = true}}, SHIFT(180),
  [349] = {.entry = {.count = 1, .reusable = true}}, SHIFT(225),
  [351] = {.entry = {.count = 1, .reusable = false}}, SHIFT(225),
  [353] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 2, 0, 0),
  [355] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 3, 0, 0),
  [357] = {.entry = {.count = 1, .reusable = true}}, SHIFT(243),
  [359] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [361] = {.entry = {.count = 1, .reusable = false}}, SHIFT(17),
  [363] = {.entry = {.count = 1, .reusable = true}}, SHIFT(100),
  [365] = {.entry = {.count = 1, .reusable = true}}, SHIFT(98),
  [367] = {.entry = {.count = 1, .reusable = false}}, SHIFT(21),
  [369] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [371] = {.entry = {.count = 1, .reusable = false}}, SHIFT(4),
  [373] = {.entry = {.count = 1, .reusable = true}}, SHIFT(108),
  [375] = {.entry = {.count = 1, .reusable = false}}, SHIFT(22),
  [377] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [379] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_hook_grouped_repeat1, 2, 0, 0),
  [381] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_hook_grouped_repeat1, 2, 0, 0), SHIFT_REPEAT(166),
  [384] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_hook_grouped_repeat1, 2, 0, 0), SHIFT_REPEAT(166),
  [387] = {.entry = {.count = 1, .reusable = false}}, SHIFT(184),
  [389] = {.entry = {.count = 1, .reusable = false}}, SHIFT(85),
  [391] = {.entry = {.count = 1, .reusable = true}}, SHIFT(184),
  [393] = {.entry = {.count = 1, .reusable = false}}, SHIFT(42),
  [395] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [397] = {.entry = {.count = 1, .reusable = true}}, SHIFT(186),
  [399] = {.entry = {.count = 1, .reusable = true}}, SHIFT(107),
  [401] = {.entry = {.count = 1, .reusable = true}}, SHIFT(214),
  [403] = {.entry = {.count = 1, .reusable = true}}, SHIFT(164),
  [405] = {.entry = {.count = 1, .reusable = true}}, SHIFT(189),
  [407] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [409] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0),
  [411] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(186),
  [414] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(107),
  [417] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(214),
  [420] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(164),
  [423] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(189),
  [426] = {.entry = {.count = 1, .reusable = false}}, SHIFT(170),
  [428] = {.entry = {.count = 1, .reusable = true}}, SHIFT(170),
  [430] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [432] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, 0, 0),
  [434] = {.entry = {.count = 1, .reusable = false}}, SHIFT(165),
  [436] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [438] = {.entry = {.count = 1, .reusable = false}}, SHIFT(201),
  [440] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0),
  [442] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(238),
  [445] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [447] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(165),
  [450] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0),
  [452] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(201),
  [455] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_helpers_section, 2, 0, 0),
  [457] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_init_section, 2, 0, 0),
  [459] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_section, 2, 0, 0),
  [461] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declare_section, 2, 0, 0),
  [463] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_paren_code_block, 2, 0, 0),
  [465] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_paren_code_block, 3, 0, 0),
  [467] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_init_section, 3, 0, 0),
  [469] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_use_statement, 4, 0, 2),
  [471] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, 0, 0),
  [473] = {.entry = {.count = 1, .reusable = true}}, SHIFT(234),
  [475] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, 0, 0),
  [477] = {.entry = {.count = 1, .reusable = false}}, SHIFT(3),
  [479] = {.entry = {.count = 1, .reusable = false}}, SHIFT(116),
  [481] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [483] = {.entry = {.count = 1, .reusable = false}}, SHIFT(114),
  [485] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [487] = {.entry = {.count = 1, .reusable = true}}, SHIFT(191),
  [489] = {.entry = {.count = 1, .reusable = true}}, SHIFT(197),
  [491] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat2, 2, 0, 0),
  [493] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat2, 2, 0, 0), SHIFT_REPEAT(235),
  [496] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat2, 2, 0, 0), SHIFT_REPEAT(232),
  [499] = {.entry = {.count = 1, .reusable = true}}, SHIFT(177),
  [501] = {.entry = {.count = 1, .reusable = true}}, SHIFT(174),
  [503] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [505] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [507] = {.entry = {.count = 1, .reusable = false}}, SHIFT(177),
  [509] = {.entry = {.count = 1, .reusable = false}}, SHIFT(35),
  [511] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [513] = {.entry = {.count = 1, .reusable = true}}, SHIFT(230),
  [515] = {.entry = {.count = 1, .reusable = true}}, SHIFT(247),
  [517] = {.entry = {.count = 1, .reusable = true}}, SHIFT(134),
  [519] = {.entry = {.count = 1, .reusable = true}}, SHIFT(139),
  [521] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_content, 1, 0, 0),
  [523] = {.entry = {.count = 1, .reusable = false}}, SHIFT(121),
  [525] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [527] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_single_string_content, 1, 0, 0),
  [529] = {.entry = {.count = 1, .reusable = false}}, SHIFT(117),
  [531] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_single_string_content_repeat1, 2, 0, 0),
  [533] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_single_string_content_repeat1, 2, 0, 0), SHIFT_REPEAT(117),
  [536] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [538] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [540] = {.entry = {.count = 1, .reusable = true}}, SHIFT(150),
  [542] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [544] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_string_content_repeat1, 2, 0, 0),
  [546] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_string_content_repeat1, 2, 0, 0), SHIFT_REPEAT(121),
  [549] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [551] = {.entry = {.count = 1, .reusable = false}}, SHIFT(20),
  [553] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_after_body_repeat1, 2, 0, 0),
  [555] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_after_body_repeat1, 2, 0, 0), SHIFT_REPEAT(230),
  [558] = {.entry = {.count = 1, .reusable = true}}, SHIFT(198),
  [560] = {.entry = {.count = 1, .reusable = true}}, SHIFT(219),
  [562] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fixture_params_repeat1, 2, 0, 0),
  [564] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_params_repeat1, 2, 0, 0), SHIFT_REPEAT(183),
  [567] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite, 9, 0, 7),
  [569] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup_statement, 1, 0, 0),
  [571] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_statement, 1, 0, 0),
  [573] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite, 8, 0, 6),
  [575] = {.entry = {.count = 1, .reusable = true}}, SHIFT(205),
  [577] = {.entry = {.count = 1, .reusable = true}}, SHIFT(149),
  [579] = {.entry = {.count = 1, .reusable = true}}, SHIFT(109),
  [581] = {.entry = {.count = 1, .reusable = true}}, SHIFT(195),
  [583] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 3, 0, 0),
  [585] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 3, 0, 0),
  [587] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_argument_list_repeat1, 2, 0, 0),
  [589] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_argument_list_repeat1, 2, 0, 0), SHIFT_REPEAT(181),
  [592] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 3, 0, 0),
  [594] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_string_array_repeat1, 2, 0, 0), SHIFT_REPEAT(125),
  [597] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_string_array_repeat1, 2, 0, 0),
  [599] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 5, 0, 0),
  [601] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 5, 0, 0),
  [603] = {.entry = {.count = 1, .reusable = true}}, SHIFT(115),
  [605] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [607] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite, 3, 0, 1),
  [609] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_anvil_call, 6, 0, 0),
  [611] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_call, 6, 0, 0),
  [613] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 6, 0, 0),
  [615] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 6, 0, 0),
  [617] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_anvil_call, 5, 0, 0),
  [619] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_call, 5, 0, 0),
  [621] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 1, 0, 0),
  [623] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [625] = {.entry = {.count = 1, .reusable = true}}, SHIFT(204),
  [627] = {.entry = {.count = 1, .reusable = true}}, SHIFT(199),
  [629] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 2, 0, 0),
  [631] = {.entry = {.count = 1, .reusable = true}}, SHIFT(137),
  [633] = {.entry = {.count = 1, .reusable = true}}, SHIFT(126),
  [635] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 4, 0, 0),
  [637] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 4, 0, 0),
  [639] = {.entry = {.count = 1, .reusable = true}}, SHIFT(146),
  [641] = {.entry = {.count = 1, .reusable = true}}, SHIFT(227),
  [643] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [645] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite_body, 3, 0, 0),
  [647] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_chart_params_repeat1, 2, 0, 0),
  [649] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_chart_params_repeat1, 2, 0, 0), SHIFT_REPEAT(62),
  [652] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 1, 0, 0),
  [654] = {.entry = {.count = 1, .reusable = true}}, SHIFT(153),
  [656] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite_body, 2, 0, 0),
  [658] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite, 4, 0, 3),
  [660] = {.entry = {.count = 1, .reusable = true}}, SHIFT(240),
  [662] = {.entry = {.count = 1, .reusable = true}}, SHIFT(112),
  [664] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_language_tag, 1, 0, 0),
  [666] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [668] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_param, 3, 0, 5),
  [670] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [672] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [674] = {.entry = {.count = 1, .reusable = true}}, SHIFT(90),
  [676] = {.entry = {.count = 1, .reusable = true}}, SHIFT(222),
  [678] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_directive, 6, 0, 10),
  [680] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [682] = {.entry = {.count = 1, .reusable = true}}, SHIFT(211),
  [684] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_directive, 5, 0, 10),
  [686] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument, 3, 0, 5),
  [688] = {.entry = {.count = 1, .reusable = true}}, SHIFT(118),
  [690] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite_type, 1, 0, 0),
  [692] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_param, 3, 0, 9),
  [694] = {.entry = {.count = 1, .reusable = true}}, SHIFT(122),
  [696] = {.entry = {.count = 1, .reusable = true}}, SHIFT(143),
  [698] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_run_mode, 1, 0, 0),
  [700] = {.entry = {.count = 1, .reusable = true}}, SHIFT(169),
  [702] = {.entry = {.count = 1, .reusable = true}}, SHIFT(145),
  [704] = {.entry = {.count = 1, .reusable = true}}, SHIFT(207),
  [706] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 2, 0, 0),
  [708] = {.entry = {.count = 1, .reusable = true}}, SHIFT(123),
  [710] = {.entry = {.count = 1, .reusable = true}}, SHIFT(239),
  [712] = {.entry = {.count = 1, .reusable = true}}, SHIFT(93),
  [714] = {.entry = {.count = 1, .reusable = true}}, SHIFT(167),
  [716] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 4, 0, 0),
  [718] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 3, 0, 0),
  [720] = {.entry = {.count = 1, .reusable = true}}, SHIFT(160),
  [722] = {.entry = {.count = 1, .reusable = true}}, SHIFT(192),
  [724] = {.entry = {.count = 1, .reusable = true}}, SHIFT(120),
  [726] = {.entry = {.count = 1, .reusable = true}}, SHIFT(102),
  [728] = {.entry = {.count = 1, .reusable = true}}, SHIFT(104),
  [730] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [732] = {.entry = {.count = 1, .reusable = true}}, SHIFT(237),
  [734] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [736] = {.entry = {.count = 1, .reusable = true}}, SHIFT(173),
  [738] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_args, 3, 0, 0),
  [740] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [742] = {.entry = {.count = 1, .reusable = true}}, SHIFT(144),
  [744] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 5, 0, 0),
  [746] = {.entry = {.count = 1, .reusable = true}}, SHIFT(171),
  [748] = {.entry = {.count = 1, .reusable = true}}, SHIFT(103),
  [750] = {.entry = {.count = 1, .reusable = true}}, SHIFT(91),
  [752] = {.entry = {.count = 1, .reusable = true}}, SHIFT(202),
  [754] = {.entry = {.count = 1, .reusable = true}}, SHIFT(119),
  [756] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_param_name, 1, 0, 0),
  [758] = {.entry = {.count = 1, .reusable = true}}, SHIFT(176),
  [760] = {.entry = {.count = 1, .reusable = true}}, SHIFT(142),
  [762] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [764] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [766] = {.entry = {.count = 1, .reusable = true}}, SHIFT(95),
  [768] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [770] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [772] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [774] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_function_name, 1, 0, 0),
  [776] = {.entry = {.count = 1, .reusable = true}}, SHIFT(221),
  [778] = {.entry = {.count = 1, .reusable = true}}, SHIFT(135),
  [780] = {.entry = {.count = 1, .reusable = true}}, SHIFT(147),
  [782] = {.entry = {.count = 1, .reusable = true}}, SHIFT(223),
  [784] = {.entry = {.count = 1, .reusable = true}}, SHIFT(244),
  [786] = {.entry = {.count = 1, .reusable = true}}, SHIFT(248),
  [788] = {.entry = {.count = 1, .reusable = true}}, SHIFT(110),
  [790] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property_name, 1, 0, 0),
  [792] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [794] = {.entry = {.count = 1, .reusable = true}}, SHIFT(156),
  [796] = {.entry = {.count = 1, .reusable = true}}, SHIFT(236),
  [798] = {.entry = {.count = 1, .reusable = true}}, SHIFT(155),
  [800] = {.entry = {.count = 1, .reusable = true}}, SHIFT(73),
  [802] = {.entry = {.count = 1, .reusable = true}}, SHIFT(113),
  [804] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [806] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
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
