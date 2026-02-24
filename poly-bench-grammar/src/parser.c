#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 221
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 171
#define ALIAS_COUNT 0
#define TOKEN_COUNT 94
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
  anon_sym_benchAsync = 28,
  anon_sym_tags = 29,
  anon_sym_skip = 30,
  anon_sym_validate = 31,
  anon_sym_before = 32,
  anon_sym_after = 33,
  anon_sym_each = 34,
  anon_sym_charting = 35,
  anon_sym_drawSpeedupChart = 36,
  anon_sym_drawTable = 37,
  anon_sym_title = 38,
  anon_sym_description = 39,
  anon_sym_output = 40,
  anon_sym_sortBy = 41,
  anon_sym_sortOrder = 42,
  anon_sym_baselineBenchmark = 43,
  anon_sym_baseline = 44,
  anon_sym_filterWinner = 45,
  anon_sym_theme = 46,
  anon_sym_width = 47,
  anon_sym_rowCount = 48,
  anon_sym_height = 49,
  anon_sym_limit = 50,
  anon_sym_minSpeedup = 51,
  anon_sym_includeBenchmarks = 52,
  anon_sym_excludeBenchmarks = 53,
  anon_sym_iterations = 54,
  anon_sym_warmup = 55,
  anon_sym_timeout = 56,
  anon_sym_requires = 57,
  anon_sym_order = 58,
  anon_sym_mode = 59,
  anon_sym_targetTime = 60,
  anon_sym_sink = 61,
  anon_sym_outlierDetection = 62,
  anon_sym_cvThreshold = 63,
  anon_sym_count = 64,
  anon_sym_memory = 65,
  anon_sym_fairness = 66,
  anon_sym_fairnessSeed = 67,
  anon_sym_asyncSamplingPolicy = 68,
  anon_sym_asyncWarmupCap = 69,
  anon_sym_asyncSampleCap = 70,
  anon_sym_go = 71,
  anon_sym_ts = 72,
  anon_sym_typescript = 73,
  anon_sym_rust = 74,
  anon_sym_python = 75,
  sym_inline_code = 76,
  anon_sym_DQUOTE = 77,
  anon_sym_SQUOTE = 78,
  aux_sym_string_content_token1 = 79,
  aux_sym_single_string_content_token1 = 80,
  sym_escape_sequence = 81,
  sym_number = 82,
  sym_float = 83,
  anon_sym_ms = 84,
  anon_sym_s = 85,
  anon_sym_m = 86,
  anon_sym_true = 87,
  anon_sym_false = 88,
  anon_sym_LBRACK = 89,
  anon_sym_RBRACK = 90,
  sym_comment = 91,
  sym_embedded_code = 92,
  sym__embedded_code_start = 93,
  sym_source_file = 94,
  sym_use_statement = 95,
  sym_global_setup = 96,
  sym_global_setup_body = 97,
  sym_global_setup_statement = 98,
  sym_anvil_call = 99,
  sym_anvil_args = 100,
  sym_function_call = 101,
  sym_argument_list = 102,
  sym_argument = 103,
  sym_suite = 104,
  sym_suite_body = 105,
  sym__suite_item = 106,
  sym_setup_block = 107,
  sym_setup_body = 108,
  sym__setup_section = 109,
  sym_import_section = 110,
  sym_declare_section = 111,
  sym_init_section = 112,
  sym_helpers_section = 113,
  sym_fixture = 114,
  sym_fixture_params = 115,
  sym_fixture_param = 116,
  sym_fixture_body = 117,
  sym__fixture_item = 118,
  sym_hex_property = 119,
  sym_shape_property = 120,
  sym_file_ref = 121,
  sym_benchmark = 122,
  sym_benchmark_body = 123,
  sym__benchmark_item = 124,
  sym_tags_property = 125,
  sym_skip_hook = 126,
  sym_validate_hook = 127,
  sym_before_hook = 128,
  sym_after_hook = 129,
  sym_each_hook = 130,
  sym_hook_flat = 131,
  sym_hook_grouped = 132,
  sym_after_block = 133,
  sym_after_body = 134,
  sym_chart_directive = 135,
  sym_chart_function_name = 136,
  sym_chart_params = 137,
  sym_chart_param = 138,
  sym_chart_param_name = 139,
  sym__chart_value = 140,
  sym_property = 141,
  sym_property_name = 142,
  sym__value = 143,
  sym_language_implementation = 144,
  sym_language_tag = 145,
  sym__code_or_inline = 146,
  sym_code_block = 147,
  sym_paren_code_block = 148,
  sym_string = 149,
  sym_string_content = 150,
  sym_single_string_content = 151,
  sym_duration = 152,
  sym_duration_unit = 153,
  sym_boolean = 154,
  sym_string_array = 155,
  aux_sym_source_file_repeat1 = 156,
  aux_sym_source_file_repeat2 = 157,
  aux_sym_global_setup_body_repeat1 = 158,
  aux_sym_argument_list_repeat1 = 159,
  aux_sym_suite_body_repeat1 = 160,
  aux_sym_setup_body_repeat1 = 161,
  aux_sym_fixture_params_repeat1 = 162,
  aux_sym_fixture_body_repeat1 = 163,
  aux_sym_benchmark_body_repeat1 = 164,
  aux_sym_hook_grouped_repeat1 = 165,
  aux_sym_after_body_repeat1 = 166,
  aux_sym_chart_params_repeat1 = 167,
  aux_sym_string_content_repeat1 = 168,
  aux_sym_single_string_content_repeat1 = 169,
  aux_sym_string_array_repeat1 = 170,
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
  [anon_sym_memory] = "memory",
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
  [anon_sym_memory] = anon_sym_memory,
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
  [anon_sym_memory] = {
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
  [29] = 23,
  [30] = 30,
  [31] = 31,
  [32] = 32,
  [33] = 33,
  [34] = 34,
  [35] = 23,
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
  [61] = 59,
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
  [79] = 23,
  [80] = 80,
  [81] = 81,
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
  [219] = 197,
  [220] = 197,
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
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0);
      END_STATE();
    case 1:
      if (lookahead == 'f') ADVANCE(20);
      if (lookahead == 'n') ADVANCE(21);
      if (lookahead == 's') ADVANCE(22);
      END_STATE();
    case 2:
      if (lookahead == 'a') ADVANCE(23);
      if (lookahead == 'e') ADVANCE(24);
      END_STATE();
    case 3:
      if (lookahead == 'h') ADVANCE(25);
      if (lookahead == 'o') ADVANCE(26);
      if (lookahead == 'v') ADVANCE(27);
      END_STATE();
    case 4:
      if (lookahead == 'e') ADVANCE(28);
      if (lookahead == 'r') ADVANCE(29);
      END_STATE();
    case 5:
      if (lookahead == 'a') ADVANCE(30);
      if (lookahead == 'x') ADVANCE(31);
      END_STATE();
    case 6:
      if (lookahead == 'a') ADVANCE(32);
      if (lookahead == 'i') ADVANCE(33);
      if (lookahead == 'o') ADVANCE(34);
      END_STATE();
    case 7:
      if (lookahead == 'l') ADVANCE(35);
      if (lookahead == 'o') ADVANCE(36);
      END_STATE();
    case 8:
      if (lookahead == 'e') ADVANCE(37);
      END_STATE();
    case 9:
      if (lookahead == 'm') ADVANCE(38);
      if (lookahead == 'n') ADVANCE(39);
      if (lookahead == 't') ADVANCE(40);
      END_STATE();
    case 10:
      if (lookahead == 'i') ADVANCE(41);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_m);
      if (lookahead == 'e') ADVANCE(42);
      if (lookahead == 'i') ADVANCE(43);
      if (lookahead == 'o') ADVANCE(44);
      if (lookahead == 's') ADVANCE(45);
      END_STATE();
    case 12:
      if (lookahead == 'r') ADVANCE(46);
      if (lookahead == 'u') ADVANCE(47);
      END_STATE();
    case 13:
      if (lookahead == 'y') ADVANCE(48);
      END_STATE();
    case 14:
      if (lookahead == 'e') ADVANCE(49);
      if (lookahead == 'o') ADVANCE(50);
      if (lookahead == 'u') ADVANCE(51);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_s);
      ADVANCE_MAP(
        'e', 52,
        'h', 53,
        'i', 54,
        'k', 55,
        'o', 56,
        'p', 57,
        't', 58,
        'u', 59,
      );
      END_STATE();
    case 16:
      if (lookahead == 'a') ADVANCE(60);
      if (lookahead == 'h') ADVANCE(61);
      if (lookahead == 'i') ADVANCE(62);
      if (lookahead == 'r') ADVANCE(63);
      if (lookahead == 's') ADVANCE(64);
      if (lookahead == 'y') ADVANCE(65);
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
      if (lookahead == 't') ADVANCE(70);
      END_STATE();
    case 21:
      if (lookahead == 'v') ADVANCE(71);
      END_STATE();
    case 22:
      if (lookahead == 'y') ADVANCE(72);
      END_STATE();
    case 23:
      if (lookahead == 's') ADVANCE(73);
      END_STATE();
    case 24:
      if (lookahead == 'f') ADVANCE(74);
      if (lookahead == 'n') ADVANCE(75);
      END_STATE();
    case 25:
      if (lookahead == 'a') ADVANCE(76);
      END_STATE();
    case 26:
      if (lookahead == 'u') ADVANCE(77);
      END_STATE();
    case 27:
      if (lookahead == 'T') ADVANCE(78);
      END_STATE();
    case 28:
      if (lookahead == 'c') ADVANCE(79);
      if (lookahead == 's') ADVANCE(80);
      END_STATE();
    case 29:
      if (lookahead == 'a') ADVANCE(81);
      END_STATE();
    case 30:
      if (lookahead == 'c') ADVANCE(82);
      END_STATE();
    case 31:
      if (lookahead == 'c') ADVANCE(83);
      END_STATE();
    case 32:
      if (lookahead == 'i') ADVANCE(84);
      if (lookahead == 'l') ADVANCE(85);
      END_STATE();
    case 33:
      if (lookahead == 'l') ADVANCE(86);
      if (lookahead == 'x') ADVANCE(87);
      END_STATE();
    case 34:
      if (lookahead == 'r') ADVANCE(88);
      END_STATE();
    case 35:
      if (lookahead == 'o') ADVANCE(89);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(anon_sym_go);
      END_STATE();
    case 37:
      if (lookahead == 'i') ADVANCE(90);
      if (lookahead == 'l') ADVANCE(91);
      if (lookahead == 'x') ADVANCE(92);
      END_STATE();
    case 38:
      if (lookahead == 'p') ADVANCE(93);
      END_STATE();
    case 39:
      if (lookahead == 'c') ADVANCE(94);
      if (lookahead == 'i') ADVANCE(95);
      END_STATE();
    case 40:
      if (lookahead == 'e') ADVANCE(96);
      END_STATE();
    case 41:
      if (lookahead == 'm') ADVANCE(97);
      END_STATE();
    case 42:
      if (lookahead == 'm') ADVANCE(98);
      END_STATE();
    case 43:
      if (lookahead == 'n') ADVANCE(99);
      END_STATE();
    case 44:
      if (lookahead == 'd') ADVANCE(100);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(anon_sym_ms);
      END_STATE();
    case 46:
      if (lookahead == 'd') ADVANCE(101);
      END_STATE();
    case 47:
      if (lookahead == 't') ADVANCE(102);
      END_STATE();
    case 48:
      if (lookahead == 't') ADVANCE(103);
      END_STATE();
    case 49:
      if (lookahead == 'q') ADVANCE(104);
      END_STATE();
    case 50:
      if (lookahead == 'w') ADVANCE(105);
      END_STATE();
    case 51:
      if (lookahead == 's') ADVANCE(106);
      END_STATE();
    case 52:
      if (lookahead == 't') ADVANCE(107);
      END_STATE();
    case 53:
      if (lookahead == 'a') ADVANCE(108);
      END_STATE();
    case 54:
      if (lookahead == 'n') ADVANCE(109);
      END_STATE();
    case 55:
      if (lookahead == 'i') ADVANCE(110);
      END_STATE();
    case 56:
      if (lookahead == 'r') ADVANCE(111);
      END_STATE();
    case 57:
      if (lookahead == 'a') ADVANCE(112);
      END_STATE();
    case 58:
      if (lookahead == 'd') ADVANCE(113);
      END_STATE();
    case 59:
      if (lookahead == 'i') ADVANCE(114);
      END_STATE();
    case 60:
      if (lookahead == 'g') ADVANCE(115);
      if (lookahead == 'r') ADVANCE(116);
      END_STATE();
    case 61:
      if (lookahead == 'e') ADVANCE(117);
      END_STATE();
    case 62:
      if (lookahead == 'm') ADVANCE(118);
      if (lookahead == 't') ADVANCE(119);
      END_STATE();
    case 63:
      if (lookahead == 'u') ADVANCE(120);
      END_STATE();
    case 64:
      ACCEPT_TOKEN(anon_sym_ts);
      END_STATE();
    case 65:
      if (lookahead == 'p') ADVANCE(121);
      END_STATE();
    case 66:
      if (lookahead == 'e') ADVANCE(122);
      END_STATE();
    case 67:
      if (lookahead == 'l') ADVANCE(123);
      END_STATE();
    case 68:
      if (lookahead == 'r') ADVANCE(124);
      END_STATE();
    case 69:
      if (lookahead == 'd') ADVANCE(125);
      END_STATE();
    case 70:
      if (lookahead == 'e') ADVANCE(126);
      END_STATE();
    case 71:
      if (lookahead == 'i') ADVANCE(127);
      END_STATE();
    case 72:
      if (lookahead == 'n') ADVANCE(128);
      END_STATE();
    case 73:
      if (lookahead == 'e') ADVANCE(129);
      END_STATE();
    case 74:
      if (lookahead == 'o') ADVANCE(130);
      END_STATE();
    case 75:
      if (lookahead == 'c') ADVANCE(131);
      END_STATE();
    case 76:
      if (lookahead == 'r') ADVANCE(132);
      END_STATE();
    case 77:
      if (lookahead == 'n') ADVANCE(133);
      END_STATE();
    case 78:
      if (lookahead == 'h') ADVANCE(134);
      END_STATE();
    case 79:
      if (lookahead == 'l') ADVANCE(135);
      END_STATE();
    case 80:
      if (lookahead == 'c') ADVANCE(136);
      END_STATE();
    case 81:
      if (lookahead == 'w') ADVANCE(137);
      END_STATE();
    case 82:
      if (lookahead == 'h') ADVANCE(138);
      END_STATE();
    case 83:
      if (lookahead == 'l') ADVANCE(139);
      END_STATE();
    case 84:
      if (lookahead == 'r') ADVANCE(140);
      END_STATE();
    case 85:
      if (lookahead == 's') ADVANCE(141);
      END_STATE();
    case 86:
      if (lookahead == 't') ADVANCE(142);
      END_STATE();
    case 87:
      if (lookahead == 't') ADVANCE(143);
      END_STATE();
    case 88:
      if (lookahead == 'k') ADVANCE(144);
      END_STATE();
    case 89:
      if (lookahead == 'b') ADVANCE(145);
      END_STATE();
    case 90:
      if (lookahead == 'g') ADVANCE(146);
      END_STATE();
    case 91:
      if (lookahead == 'p') ADVANCE(147);
      END_STATE();
    case 92:
      ACCEPT_TOKEN(anon_sym_hex);
      END_STATE();
    case 93:
      if (lookahead == 'o') ADVANCE(148);
      END_STATE();
    case 94:
      if (lookahead == 'l') ADVANCE(149);
      END_STATE();
    case 95:
      if (lookahead == 't') ADVANCE(150);
      END_STATE();
    case 96:
      if (lookahead == 'r') ADVANCE(151);
      END_STATE();
    case 97:
      if (lookahead == 'i') ADVANCE(152);
      END_STATE();
    case 98:
      if (lookahead == 'o') ADVANCE(153);
      END_STATE();
    case 99:
      if (lookahead == 'S') ADVANCE(154);
      END_STATE();
    case 100:
      if (lookahead == 'e') ADVANCE(155);
      END_STATE();
    case 101:
      if (lookahead == 'e') ADVANCE(156);
      END_STATE();
    case 102:
      if (lookahead == 'l') ADVANCE(157);
      if (lookahead == 'p') ADVANCE(158);
      END_STATE();
    case 103:
      if (lookahead == 'h') ADVANCE(159);
      END_STATE();
    case 104:
      if (lookahead == 'u') ADVANCE(160);
      END_STATE();
    case 105:
      if (lookahead == 'C') ADVANCE(161);
      END_STATE();
    case 106:
      if (lookahead == 't') ADVANCE(162);
      END_STATE();
    case 107:
      if (lookahead == 'u') ADVANCE(163);
      END_STATE();
    case 108:
      if (lookahead == 'p') ADVANCE(164);
      END_STATE();
    case 109:
      if (lookahead == 'k') ADVANCE(165);
      END_STATE();
    case 110:
      if (lookahead == 'p') ADVANCE(166);
      END_STATE();
    case 111:
      if (lookahead == 't') ADVANCE(167);
      END_STATE();
    case 112:
      if (lookahead == 'w') ADVANCE(168);
      END_STATE();
    case 113:
      ACCEPT_TOKEN(anon_sym_std);
      END_STATE();
    case 114:
      if (lookahead == 't') ADVANCE(169);
      END_STATE();
    case 115:
      if (lookahead == 's') ADVANCE(170);
      END_STATE();
    case 116:
      if (lookahead == 'g') ADVANCE(171);
      END_STATE();
    case 117:
      if (lookahead == 'm') ADVANCE(172);
      END_STATE();
    case 118:
      if (lookahead == 'e') ADVANCE(173);
      END_STATE();
    case 119:
      if (lookahead == 'l') ADVANCE(174);
      END_STATE();
    case 120:
      if (lookahead == 'e') ADVANCE(175);
      END_STATE();
    case 121:
      if (lookahead == 'e') ADVANCE(176);
      END_STATE();
    case 122:
      ACCEPT_TOKEN(anon_sym_use);
      END_STATE();
    case 123:
      if (lookahead == 'i') ADVANCE(177);
      END_STATE();
    case 124:
      if (lookahead == 'm') ADVANCE(178);
      END_STATE();
    case 125:
      if (lookahead == 't') ADVANCE(179);
      END_STATE();
    case 126:
      if (lookahead == 'r') ADVANCE(180);
      END_STATE();
    case 127:
      if (lookahead == 'l') ADVANCE(181);
      END_STATE();
    case 128:
      if (lookahead == 'c') ADVANCE(182);
      END_STATE();
    case 129:
      if (lookahead == 'l') ADVANCE(183);
      END_STATE();
    case 130:
      if (lookahead == 'r') ADVANCE(184);
      END_STATE();
    case 131:
      if (lookahead == 'h') ADVANCE(185);
      END_STATE();
    case 132:
      if (lookahead == 't') ADVANCE(186);
      END_STATE();
    case 133:
      if (lookahead == 't') ADVANCE(187);
      END_STATE();
    case 134:
      if (lookahead == 'r') ADVANCE(188);
      END_STATE();
    case 135:
      if (lookahead == 'a') ADVANCE(189);
      END_STATE();
    case 136:
      if (lookahead == 'r') ADVANCE(190);
      END_STATE();
    case 137:
      if (lookahead == 'S') ADVANCE(191);
      if (lookahead == 'T') ADVANCE(192);
      END_STATE();
    case 138:
      ACCEPT_TOKEN(anon_sym_each);
      END_STATE();
    case 139:
      if (lookahead == 'u') ADVANCE(193);
      END_STATE();
    case 140:
      if (lookahead == 'n') ADVANCE(194);
      END_STATE();
    case 141:
      if (lookahead == 'e') ADVANCE(195);
      END_STATE();
    case 142:
      if (lookahead == 'e') ADVANCE(196);
      END_STATE();
    case 143:
      if (lookahead == 'u') ADVANCE(197);
      END_STATE();
    case 144:
      ACCEPT_TOKEN(anon_sym_fork);
      END_STATE();
    case 145:
      if (lookahead == 'a') ADVANCE(198);
      END_STATE();
    case 146:
      if (lookahead == 'h') ADVANCE(199);
      END_STATE();
    case 147:
      if (lookahead == 'e') ADVANCE(200);
      END_STATE();
    case 148:
      if (lookahead == 'r') ADVANCE(201);
      END_STATE();
    case 149:
      if (lookahead == 'u') ADVANCE(202);
      END_STATE();
    case 150:
      ACCEPT_TOKEN(anon_sym_init);
      END_STATE();
    case 151:
      if (lookahead == 'a') ADVANCE(203);
      END_STATE();
    case 152:
      if (lookahead == 't') ADVANCE(204);
      END_STATE();
    case 153:
      if (lookahead == 'r') ADVANCE(205);
      END_STATE();
    case 154:
      if (lookahead == 'p') ADVANCE(206);
      END_STATE();
    case 155:
      ACCEPT_TOKEN(anon_sym_mode);
      END_STATE();
    case 156:
      if (lookahead == 'r') ADVANCE(207);
      END_STATE();
    case 157:
      if (lookahead == 'i') ADVANCE(208);
      END_STATE();
    case 158:
      if (lookahead == 'u') ADVANCE(209);
      END_STATE();
    case 159:
      if (lookahead == 'o') ADVANCE(210);
      END_STATE();
    case 160:
      if (lookahead == 'i') ADVANCE(211);
      END_STATE();
    case 161:
      if (lookahead == 'o') ADVANCE(212);
      END_STATE();
    case 162:
      ACCEPT_TOKEN(anon_sym_rust);
      END_STATE();
    case 163:
      if (lookahead == 'p') ADVANCE(213);
      END_STATE();
    case 164:
      if (lookahead == 'e') ADVANCE(214);
      END_STATE();
    case 165:
      ACCEPT_TOKEN(anon_sym_sink);
      END_STATE();
    case 166:
      ACCEPT_TOKEN(anon_sym_skip);
      END_STATE();
    case 167:
      if (lookahead == 'B') ADVANCE(215);
      if (lookahead == 'O') ADVANCE(216);
      END_STATE();
    case 168:
      if (lookahead == 'n') ADVANCE(217);
      END_STATE();
    case 169:
      if (lookahead == 'e') ADVANCE(218);
      END_STATE();
    case 170:
      ACCEPT_TOKEN(anon_sym_tags);
      END_STATE();
    case 171:
      if (lookahead == 'e') ADVANCE(219);
      END_STATE();
    case 172:
      if (lookahead == 'e') ADVANCE(220);
      END_STATE();
    case 173:
      if (lookahead == 'o') ADVANCE(221);
      END_STATE();
    case 174:
      if (lookahead == 'e') ADVANCE(222);
      END_STATE();
    case 175:
      ACCEPT_TOKEN(anon_sym_true);
      END_STATE();
    case 176:
      if (lookahead == 's') ADVANCE(223);
      END_STATE();
    case 177:
      if (lookahead == 'd') ADVANCE(224);
      END_STATE();
    case 178:
      if (lookahead == 'u') ADVANCE(225);
      END_STATE();
    case 179:
      if (lookahead == 'h') ADVANCE(226);
      END_STATE();
    case 180:
      ACCEPT_TOKEN(anon_sym_after);
      END_STATE();
    case 181:
      ACCEPT_TOKEN(anon_sym_anvil);
      END_STATE();
    case 182:
      ACCEPT_TOKEN(anon_sym_async);
      if (lookahead == 'S') ADVANCE(227);
      if (lookahead == 'W') ADVANCE(228);
      END_STATE();
    case 183:
      if (lookahead == 'i') ADVANCE(229);
      END_STATE();
    case 184:
      if (lookahead == 'e') ADVANCE(230);
      END_STATE();
    case 185:
      ACCEPT_TOKEN(anon_sym_bench);
      if (lookahead == 'A') ADVANCE(231);
      END_STATE();
    case 186:
      if (lookahead == 'i') ADVANCE(232);
      END_STATE();
    case 187:
      ACCEPT_TOKEN(anon_sym_count);
      END_STATE();
    case 188:
      if (lookahead == 'e') ADVANCE(233);
      END_STATE();
    case 189:
      if (lookahead == 'r') ADVANCE(234);
      END_STATE();
    case 190:
      if (lookahead == 'i') ADVANCE(235);
      END_STATE();
    case 191:
      if (lookahead == 'p') ADVANCE(236);
      END_STATE();
    case 192:
      if (lookahead == 'a') ADVANCE(237);
      END_STATE();
    case 193:
      if (lookahead == 'd') ADVANCE(238);
      END_STATE();
    case 194:
      if (lookahead == 'e') ADVANCE(239);
      END_STATE();
    case 195:
      ACCEPT_TOKEN(anon_sym_false);
      END_STATE();
    case 196:
      if (lookahead == 'r') ADVANCE(240);
      END_STATE();
    case 197:
      if (lookahead == 'r') ADVANCE(241);
      END_STATE();
    case 198:
      if (lookahead == 'l') ADVANCE(242);
      END_STATE();
    case 199:
      if (lookahead == 't') ADVANCE(243);
      END_STATE();
    case 200:
      if (lookahead == 'r') ADVANCE(244);
      END_STATE();
    case 201:
      if (lookahead == 't') ADVANCE(245);
      END_STATE();
    case 202:
      if (lookahead == 'd') ADVANCE(246);
      END_STATE();
    case 203:
      if (lookahead == 't') ADVANCE(247);
      END_STATE();
    case 204:
      ACCEPT_TOKEN(anon_sym_limit);
      END_STATE();
    case 205:
      if (lookahead == 'y') ADVANCE(248);
      END_STATE();
    case 206:
      if (lookahead == 'e') ADVANCE(249);
      END_STATE();
    case 207:
      ACCEPT_TOKEN(anon_sym_order);
      END_STATE();
    case 208:
      if (lookahead == 'e') ADVANCE(250);
      END_STATE();
    case 209:
      if (lookahead == 't') ADVANCE(251);
      END_STATE();
    case 210:
      if (lookahead == 'n') ADVANCE(252);
      END_STATE();
    case 211:
      if (lookahead == 'r') ADVANCE(253);
      END_STATE();
    case 212:
      if (lookahead == 'u') ADVANCE(254);
      END_STATE();
    case 213:
      ACCEPT_TOKEN(anon_sym_setup);
      END_STATE();
    case 214:
      ACCEPT_TOKEN(anon_sym_shape);
      END_STATE();
    case 215:
      if (lookahead == 'y') ADVANCE(255);
      END_STATE();
    case 216:
      if (lookahead == 'r') ADVANCE(256);
      END_STATE();
    case 217:
      if (lookahead == 'A') ADVANCE(257);
      END_STATE();
    case 218:
      ACCEPT_TOKEN(anon_sym_suite);
      END_STATE();
    case 219:
      if (lookahead == 't') ADVANCE(258);
      END_STATE();
    case 220:
      ACCEPT_TOKEN(anon_sym_theme);
      END_STATE();
    case 221:
      if (lookahead == 'u') ADVANCE(259);
      END_STATE();
    case 222:
      ACCEPT_TOKEN(anon_sym_title);
      END_STATE();
    case 223:
      if (lookahead == 'c') ADVANCE(260);
      END_STATE();
    case 224:
      if (lookahead == 'a') ADVANCE(261);
      END_STATE();
    case 225:
      if (lookahead == 'p') ADVANCE(262);
      END_STATE();
    case 226:
      ACCEPT_TOKEN(anon_sym_width);
      END_STATE();
    case 227:
      if (lookahead == 'a') ADVANCE(263);
      END_STATE();
    case 228:
      if (lookahead == 'a') ADVANCE(264);
      END_STATE();
    case 229:
      if (lookahead == 'n') ADVANCE(265);
      END_STATE();
    case 230:
      ACCEPT_TOKEN(anon_sym_before);
      END_STATE();
    case 231:
      if (lookahead == 's') ADVANCE(266);
      END_STATE();
    case 232:
      if (lookahead == 'n') ADVANCE(267);
      END_STATE();
    case 233:
      if (lookahead == 's') ADVANCE(268);
      END_STATE();
    case 234:
      if (lookahead == 'e') ADVANCE(269);
      END_STATE();
    case 235:
      if (lookahead == 'p') ADVANCE(270);
      END_STATE();
    case 236:
      if (lookahead == 'e') ADVANCE(271);
      END_STATE();
    case 237:
      if (lookahead == 'b') ADVANCE(272);
      END_STATE();
    case 238:
      if (lookahead == 'e') ADVANCE(273);
      END_STATE();
    case 239:
      if (lookahead == 's') ADVANCE(274);
      END_STATE();
    case 240:
      if (lookahead == 'W') ADVANCE(275);
      END_STATE();
    case 241:
      if (lookahead == 'e') ADVANCE(276);
      END_STATE();
    case 242:
      if (lookahead == 'S') ADVANCE(277);
      END_STATE();
    case 243:
      ACCEPT_TOKEN(anon_sym_height);
      END_STATE();
    case 244:
      if (lookahead == 's') ADVANCE(278);
      END_STATE();
    case 245:
      ACCEPT_TOKEN(anon_sym_import);
      END_STATE();
    case 246:
      if (lookahead == 'e') ADVANCE(279);
      END_STATE();
    case 247:
      if (lookahead == 'i') ADVANCE(280);
      END_STATE();
    case 248:
      ACCEPT_TOKEN(anon_sym_memory);
      END_STATE();
    case 249:
      if (lookahead == 'e') ADVANCE(281);
      END_STATE();
    case 250:
      if (lookahead == 'r') ADVANCE(282);
      END_STATE();
    case 251:
      ACCEPT_TOKEN(anon_sym_output);
      END_STATE();
    case 252:
      ACCEPT_TOKEN(anon_sym_python);
      END_STATE();
    case 253:
      if (lookahead == 'e') ADVANCE(283);
      END_STATE();
    case 254:
      if (lookahead == 'n') ADVANCE(284);
      END_STATE();
    case 255:
      ACCEPT_TOKEN(anon_sym_sortBy);
      END_STATE();
    case 256:
      if (lookahead == 'd') ADVANCE(285);
      END_STATE();
    case 257:
      if (lookahead == 'n') ADVANCE(286);
      END_STATE();
    case 258:
      if (lookahead == 'T') ADVANCE(287);
      END_STATE();
    case 259:
      if (lookahead == 't') ADVANCE(288);
      END_STATE();
    case 260:
      if (lookahead == 'r') ADVANCE(289);
      END_STATE();
    case 261:
      if (lookahead == 't') ADVANCE(290);
      END_STATE();
    case 262:
      ACCEPT_TOKEN(anon_sym_warmup);
      END_STATE();
    case 263:
      if (lookahead == 'm') ADVANCE(291);
      END_STATE();
    case 264:
      if (lookahead == 'r') ADVANCE(292);
      END_STATE();
    case 265:
      if (lookahead == 'e') ADVANCE(293);
      END_STATE();
    case 266:
      if (lookahead == 'y') ADVANCE(294);
      END_STATE();
    case 267:
      if (lookahead == 'g') ADVANCE(295);
      END_STATE();
    case 268:
      if (lookahead == 'h') ADVANCE(296);
      END_STATE();
    case 269:
      ACCEPT_TOKEN(anon_sym_declare);
      END_STATE();
    case 270:
      if (lookahead == 't') ADVANCE(297);
      END_STATE();
    case 271:
      if (lookahead == 'e') ADVANCE(298);
      END_STATE();
    case 272:
      if (lookahead == 'l') ADVANCE(299);
      END_STATE();
    case 273:
      if (lookahead == 'B') ADVANCE(300);
      END_STATE();
    case 274:
      if (lookahead == 's') ADVANCE(301);
      END_STATE();
    case 275:
      if (lookahead == 'i') ADVANCE(302);
      END_STATE();
    case 276:
      ACCEPT_TOKEN(anon_sym_fixture);
      END_STATE();
    case 277:
      if (lookahead == 'e') ADVANCE(303);
      END_STATE();
    case 278:
      ACCEPT_TOKEN(anon_sym_helpers);
      END_STATE();
    case 279:
      if (lookahead == 'B') ADVANCE(304);
      END_STATE();
    case 280:
      if (lookahead == 'o') ADVANCE(305);
      END_STATE();
    case 281:
      if (lookahead == 'd') ADVANCE(306);
      END_STATE();
    case 282:
      if (lookahead == 'D') ADVANCE(307);
      END_STATE();
    case 283:
      if (lookahead == 's') ADVANCE(308);
      END_STATE();
    case 284:
      if (lookahead == 't') ADVANCE(309);
      END_STATE();
    case 285:
      if (lookahead == 'e') ADVANCE(310);
      END_STATE();
    case 286:
      if (lookahead == 'v') ADVANCE(311);
      END_STATE();
    case 287:
      if (lookahead == 'i') ADVANCE(312);
      END_STATE();
    case 288:
      ACCEPT_TOKEN(anon_sym_timeout);
      END_STATE();
    case 289:
      if (lookahead == 'i') ADVANCE(313);
      END_STATE();
    case 290:
      if (lookahead == 'e') ADVANCE(314);
      END_STATE();
    case 291:
      if (lookahead == 'p') ADVANCE(315);
      END_STATE();
    case 292:
      if (lookahead == 'm') ADVANCE(316);
      END_STATE();
    case 293:
      ACCEPT_TOKEN(anon_sym_baseline);
      if (lookahead == 'B') ADVANCE(317);
      END_STATE();
    case 294:
      if (lookahead == 'n') ADVANCE(318);
      END_STATE();
    case 295:
      ACCEPT_TOKEN(anon_sym_charting);
      END_STATE();
    case 296:
      if (lookahead == 'o') ADVANCE(319);
      END_STATE();
    case 297:
      if (lookahead == 'i') ADVANCE(320);
      END_STATE();
    case 298:
      if (lookahead == 'd') ADVANCE(321);
      END_STATE();
    case 299:
      if (lookahead == 'e') ADVANCE(322);
      END_STATE();
    case 300:
      if (lookahead == 'e') ADVANCE(323);
      END_STATE();
    case 301:
      ACCEPT_TOKEN(anon_sym_fairness);
      if (lookahead == 'S') ADVANCE(324);
      END_STATE();
    case 302:
      if (lookahead == 'n') ADVANCE(325);
      END_STATE();
    case 303:
      if (lookahead == 't') ADVANCE(326);
      END_STATE();
    case 304:
      if (lookahead == 'e') ADVANCE(327);
      END_STATE();
    case 305:
      if (lookahead == 'n') ADVANCE(328);
      END_STATE();
    case 306:
      if (lookahead == 'u') ADVANCE(329);
      END_STATE();
    case 307:
      if (lookahead == 'e') ADVANCE(330);
      END_STATE();
    case 308:
      ACCEPT_TOKEN(anon_sym_requires);
      END_STATE();
    case 309:
      ACCEPT_TOKEN(anon_sym_rowCount);
      END_STATE();
    case 310:
      if (lookahead == 'r') ADVANCE(331);
      END_STATE();
    case 311:
      if (lookahead == 'i') ADVANCE(332);
      END_STATE();
    case 312:
      if (lookahead == 'm') ADVANCE(333);
      END_STATE();
    case 313:
      if (lookahead == 'p') ADVANCE(334);
      END_STATE();
    case 314:
      ACCEPT_TOKEN(anon_sym_validate);
      END_STATE();
    case 315:
      if (lookahead == 'l') ADVANCE(335);
      END_STATE();
    case 316:
      if (lookahead == 'u') ADVANCE(336);
      END_STATE();
    case 317:
      if (lookahead == 'e') ADVANCE(337);
      END_STATE();
    case 318:
      if (lookahead == 'c') ADVANCE(338);
      END_STATE();
    case 319:
      if (lookahead == 'l') ADVANCE(339);
      END_STATE();
    case 320:
      if (lookahead == 'o') ADVANCE(340);
      END_STATE();
    case 321:
      if (lookahead == 'u') ADVANCE(341);
      END_STATE();
    case 322:
      ACCEPT_TOKEN(anon_sym_drawTable);
      END_STATE();
    case 323:
      if (lookahead == 'n') ADVANCE(342);
      END_STATE();
    case 324:
      if (lookahead == 'e') ADVANCE(343);
      END_STATE();
    case 325:
      if (lookahead == 'n') ADVANCE(344);
      END_STATE();
    case 326:
      if (lookahead == 'u') ADVANCE(345);
      END_STATE();
    case 327:
      if (lookahead == 'n') ADVANCE(346);
      END_STATE();
    case 328:
      if (lookahead == 's') ADVANCE(347);
      END_STATE();
    case 329:
      if (lookahead == 'p') ADVANCE(348);
      END_STATE();
    case 330:
      if (lookahead == 't') ADVANCE(349);
      END_STATE();
    case 331:
      ACCEPT_TOKEN(anon_sym_sortOrder);
      END_STATE();
    case 332:
      if (lookahead == 'l') ADVANCE(350);
      END_STATE();
    case 333:
      if (lookahead == 'e') ADVANCE(351);
      END_STATE();
    case 334:
      if (lookahead == 't') ADVANCE(352);
      END_STATE();
    case 335:
      if (lookahead == 'e') ADVANCE(353);
      if (lookahead == 'i') ADVANCE(354);
      END_STATE();
    case 336:
      if (lookahead == 'p') ADVANCE(355);
      END_STATE();
    case 337:
      if (lookahead == 'n') ADVANCE(356);
      END_STATE();
    case 338:
      ACCEPT_TOKEN(anon_sym_benchAsync);
      END_STATE();
    case 339:
      if (lookahead == 'd') ADVANCE(357);
      END_STATE();
    case 340:
      if (lookahead == 'n') ADVANCE(358);
      END_STATE();
    case 341:
      if (lookahead == 'p') ADVANCE(359);
      END_STATE();
    case 342:
      if (lookahead == 'c') ADVANCE(360);
      END_STATE();
    case 343:
      if (lookahead == 'e') ADVANCE(361);
      END_STATE();
    case 344:
      if (lookahead == 'e') ADVANCE(362);
      END_STATE();
    case 345:
      if (lookahead == 'p') ADVANCE(363);
      END_STATE();
    case 346:
      if (lookahead == 'c') ADVANCE(364);
      END_STATE();
    case 347:
      ACCEPT_TOKEN(anon_sym_iterations);
      END_STATE();
    case 348:
      ACCEPT_TOKEN(anon_sym_minSpeedup);
      END_STATE();
    case 349:
      if (lookahead == 'e') ADVANCE(365);
      END_STATE();
    case 350:
      ACCEPT_TOKEN(anon_sym_spawnAnvil);
      END_STATE();
    case 351:
      ACCEPT_TOKEN(anon_sym_targetTime);
      END_STATE();
    case 352:
      ACCEPT_TOKEN(anon_sym_typescript);
      END_STATE();
    case 353:
      if (lookahead == 'C') ADVANCE(366);
      END_STATE();
    case 354:
      if (lookahead == 'n') ADVANCE(367);
      END_STATE();
    case 355:
      if (lookahead == 'C') ADVANCE(368);
      END_STATE();
    case 356:
      if (lookahead == 'c') ADVANCE(369);
      END_STATE();
    case 357:
      ACCEPT_TOKEN(anon_sym_cvThreshold);
      END_STATE();
    case 358:
      ACCEPT_TOKEN(anon_sym_description);
      END_STATE();
    case 359:
      if (lookahead == 'C') ADVANCE(370);
      END_STATE();
    case 360:
      if (lookahead == 'h') ADVANCE(371);
      END_STATE();
    case 361:
      if (lookahead == 'd') ADVANCE(372);
      END_STATE();
    case 362:
      if (lookahead == 'r') ADVANCE(373);
      END_STATE();
    case 363:
      ACCEPT_TOKEN(anon_sym_globalSetup);
      END_STATE();
    case 364:
      if (lookahead == 'h') ADVANCE(374);
      END_STATE();
    case 365:
      if (lookahead == 'c') ADVANCE(375);
      END_STATE();
    case 366:
      if (lookahead == 'a') ADVANCE(376);
      END_STATE();
    case 367:
      if (lookahead == 'g') ADVANCE(377);
      END_STATE();
    case 368:
      if (lookahead == 'a') ADVANCE(378);
      END_STATE();
    case 369:
      if (lookahead == 'h') ADVANCE(379);
      END_STATE();
    case 370:
      if (lookahead == 'h') ADVANCE(380);
      END_STATE();
    case 371:
      if (lookahead == 'm') ADVANCE(381);
      END_STATE();
    case 372:
      ACCEPT_TOKEN(anon_sym_fairnessSeed);
      END_STATE();
    case 373:
      ACCEPT_TOKEN(anon_sym_filterWinner);
      END_STATE();
    case 374:
      if (lookahead == 'm') ADVANCE(382);
      END_STATE();
    case 375:
      if (lookahead == 't') ADVANCE(383);
      END_STATE();
    case 376:
      if (lookahead == 'p') ADVANCE(384);
      END_STATE();
    case 377:
      if (lookahead == 'P') ADVANCE(385);
      END_STATE();
    case 378:
      if (lookahead == 'p') ADVANCE(386);
      END_STATE();
    case 379:
      if (lookahead == 'm') ADVANCE(387);
      END_STATE();
    case 380:
      if (lookahead == 'a') ADVANCE(388);
      END_STATE();
    case 381:
      if (lookahead == 'a') ADVANCE(389);
      END_STATE();
    case 382:
      if (lookahead == 'a') ADVANCE(390);
      END_STATE();
    case 383:
      if (lookahead == 'i') ADVANCE(391);
      END_STATE();
    case 384:
      ACCEPT_TOKEN(anon_sym_asyncSampleCap);
      END_STATE();
    case 385:
      if (lookahead == 'o') ADVANCE(392);
      END_STATE();
    case 386:
      ACCEPT_TOKEN(anon_sym_asyncWarmupCap);
      END_STATE();
    case 387:
      if (lookahead == 'a') ADVANCE(393);
      END_STATE();
    case 388:
      if (lookahead == 'r') ADVANCE(394);
      END_STATE();
    case 389:
      if (lookahead == 'r') ADVANCE(395);
      END_STATE();
    case 390:
      if (lookahead == 'r') ADVANCE(396);
      END_STATE();
    case 391:
      if (lookahead == 'o') ADVANCE(397);
      END_STATE();
    case 392:
      if (lookahead == 'l') ADVANCE(398);
      END_STATE();
    case 393:
      if (lookahead == 'r') ADVANCE(399);
      END_STATE();
    case 394:
      if (lookahead == 't') ADVANCE(400);
      END_STATE();
    case 395:
      if (lookahead == 'k') ADVANCE(401);
      END_STATE();
    case 396:
      if (lookahead == 'k') ADVANCE(402);
      END_STATE();
    case 397:
      if (lookahead == 'n') ADVANCE(403);
      END_STATE();
    case 398:
      if (lookahead == 'i') ADVANCE(404);
      END_STATE();
    case 399:
      if (lookahead == 'k') ADVANCE(405);
      END_STATE();
    case 400:
      ACCEPT_TOKEN(anon_sym_drawSpeedupChart);
      END_STATE();
    case 401:
      if (lookahead == 's') ADVANCE(406);
      END_STATE();
    case 402:
      if (lookahead == 's') ADVANCE(407);
      END_STATE();
    case 403:
      ACCEPT_TOKEN(anon_sym_outlierDetection);
      END_STATE();
    case 404:
      if (lookahead == 'c') ADVANCE(408);
      END_STATE();
    case 405:
      ACCEPT_TOKEN(anon_sym_baselineBenchmark);
      END_STATE();
    case 406:
      ACCEPT_TOKEN(anon_sym_excludeBenchmarks);
      END_STATE();
    case 407:
      ACCEPT_TOKEN(anon_sym_includeBenchmarks);
      END_STATE();
    case 408:
      if (lookahead == 'y') ADVANCE(409);
      END_STATE();
    case 409:
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
  [89] = {.lex_state = 1},
  [90] = {.lex_state = 3},
  [91] = {.lex_state = 0},
  [92] = {.lex_state = 0},
  [93] = {.lex_state = 0},
  [94] = {.lex_state = 0},
  [95] = {.lex_state = 0},
  [96] = {.lex_state = 0},
  [97] = {.lex_state = 0},
  [98] = {.lex_state = 0},
  [99] = {.lex_state = 4},
  [100] = {.lex_state = 0},
  [101] = {.lex_state = 0},
  [102] = {.lex_state = 1},
  [103] = {.lex_state = 0},
  [104] = {.lex_state = 3},
  [105] = {.lex_state = 0},
  [106] = {.lex_state = 0},
  [107] = {.lex_state = 0},
  [108] = {.lex_state = 0},
  [109] = {.lex_state = 4},
  [110] = {.lex_state = 0},
  [111] = {.lex_state = 1},
  [112] = {.lex_state = 0},
  [113] = {.lex_state = 0},
  [114] = {.lex_state = 3},
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
  [161] = {.lex_state = 0},
  [162] = {.lex_state = 0},
  [163] = {.lex_state = 0},
  [164] = {.lex_state = 0},
  [165] = {.lex_state = 0},
  [166] = {.lex_state = 0},
  [167] = {.lex_state = 0},
  [168] = {.lex_state = 0},
  [169] = {.lex_state = 0, .external_lex_state = 2},
  [170] = {.lex_state = 0, .external_lex_state = 2},
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
    [anon_sym_memory] = ACTIONS(1),
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
    [sym_source_file] = STATE(216),
    [sym_use_statement] = STATE(73),
    [sym_global_setup] = STATE(94),
    [sym_suite] = STATE(95),
    [aux_sym_source_file_repeat1] = STATE(73),
    [aux_sym_source_file_repeat2] = STATE(95),
    [ts_builtin_sym_end] = ACTIONS(5),
    [anon_sym_use] = ACTIONS(7),
    [anon_sym_globalSetup] = ACTIONS(9),
    [anon_sym_suite] = ACTIONS(11),
    [sym_comment] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 14,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(13), 1,
      anon_sym_RBRACE,
    ACTIONS(15), 1,
      anon_sym_tags,
    ACTIONS(18), 1,
      anon_sym_skip,
    ACTIONS(21), 1,
      anon_sym_validate,
    ACTIONS(24), 1,
      anon_sym_before,
    ACTIONS(27), 1,
      anon_sym_after,
    ACTIONS(30), 1,
      anon_sym_each,
    ACTIONS(36), 1,
      anon_sym_fairness,
    STATE(200), 1,
      sym_language_tag,
    STATE(220), 1,
      sym_property_name,
    ACTIONS(39), 5,
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
    ACTIONS(33), 18,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [73] = 14,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(42), 1,
      anon_sym_RBRACE,
    ACTIONS(44), 1,
      anon_sym_tags,
    ACTIONS(46), 1,
      anon_sym_skip,
    ACTIONS(48), 1,
      anon_sym_validate,
    ACTIONS(50), 1,
      anon_sym_before,
    ACTIONS(52), 1,
      anon_sym_after,
    ACTIONS(54), 1,
      anon_sym_each,
    ACTIONS(58), 1,
      anon_sym_fairness,
    STATE(200), 1,
      sym_language_tag,
    STATE(220), 1,
      sym_property_name,
    ACTIONS(60), 5,
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
    ACTIONS(56), 18,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [146] = 14,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(44), 1,
      anon_sym_tags,
    ACTIONS(46), 1,
      anon_sym_skip,
    ACTIONS(48), 1,
      anon_sym_validate,
    ACTIONS(50), 1,
      anon_sym_before,
    ACTIONS(52), 1,
      anon_sym_after,
    ACTIONS(54), 1,
      anon_sym_each,
    ACTIONS(58), 1,
      anon_sym_fairness,
    ACTIONS(62), 1,
      anon_sym_RBRACE,
    STATE(200), 1,
      sym_language_tag,
    STATE(220), 1,
      sym_property_name,
    ACTIONS(60), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
    STATE(3), 10,
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
    ACTIONS(56), 18,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [219] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(66), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(64), 39,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
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
      anon_sym_memory,
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
  [268] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(70), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(68), 39,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
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
      anon_sym_memory,
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
    ACTIONS(74), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(72), 38,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [365] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(78), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(76), 38,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [413] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(82), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(80), 38,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [461] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(86), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(84), 38,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [509] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(90), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(88), 38,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [557] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(94), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(92), 38,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [605] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(98), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(96), 38,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_COMMA,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [653] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(102), 2,
      anon_sym_async,
      anon_sym_fairness,
    ACTIONS(100), 36,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [699] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(106), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(104), 36,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [745] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(110), 2,
      anon_sym_async,
      anon_sym_fairness,
    ACTIONS(108), 36,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [791] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(58), 1,
      anon_sym_fairness,
    ACTIONS(112), 1,
      anon_sym_RBRACE,
    ACTIONS(114), 1,
      anon_sym_hex,
    ACTIONS(116), 1,
      anon_sym_shape,
    STATE(200), 1,
      sym_language_tag,
    STATE(219), 1,
      sym_property_name,
    ACTIONS(60), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
    STATE(20), 6,
      sym__fixture_item,
      sym_hex_property,
      sym_shape_property,
      sym_property,
      sym_language_implementation,
      aux_sym_fixture_body_repeat1,
    ACTIONS(56), 18,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [848] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(118), 1,
      anon_sym_globalSetup,
    ACTIONS(121), 1,
      anon_sym_RBRACE,
    ACTIONS(123), 1,
      anon_sym_setup,
    ACTIONS(126), 1,
      anon_sym_fixture,
    ACTIONS(129), 1,
      anon_sym_bench,
    ACTIONS(132), 1,
      anon_sym_benchAsync,
    ACTIONS(135), 1,
      anon_sym_after,
    ACTIONS(141), 1,
      anon_sym_fairness,
    STATE(197), 1,
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
    ACTIONS(138), 18,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [909] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(144), 1,
      anon_sym_RBRACE,
    ACTIONS(146), 1,
      anon_sym_hex,
    ACTIONS(149), 1,
      anon_sym_shape,
    ACTIONS(155), 1,
      anon_sym_fairness,
    STATE(200), 1,
      sym_language_tag,
    STATE(219), 1,
      sym_property_name,
    ACTIONS(158), 5,
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
    ACTIONS(152), 18,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [966] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(58), 1,
      anon_sym_fairness,
    ACTIONS(114), 1,
      anon_sym_hex,
    ACTIONS(116), 1,
      anon_sym_shape,
    ACTIONS(161), 1,
      anon_sym_RBRACE,
    STATE(200), 1,
      sym_language_tag,
    STATE(219), 1,
      sym_property_name,
    ACTIONS(60), 5,
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
    ACTIONS(56), 18,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [1023] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      anon_sym_globalSetup,
    ACTIONS(58), 1,
      anon_sym_fairness,
    ACTIONS(163), 1,
      anon_sym_RBRACE,
    ACTIONS(165), 1,
      anon_sym_setup,
    ACTIONS(167), 1,
      anon_sym_fixture,
    ACTIONS(169), 1,
      anon_sym_bench,
    ACTIONS(171), 1,
      anon_sym_benchAsync,
    ACTIONS(173), 1,
      anon_sym_after,
    STATE(197), 1,
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
    ACTIONS(56), 18,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [1084] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      anon_sym_globalSetup,
    ACTIONS(58), 1,
      anon_sym_fairness,
    ACTIONS(165), 1,
      anon_sym_setup,
    ACTIONS(167), 1,
      anon_sym_fixture,
    ACTIONS(169), 1,
      anon_sym_bench,
    ACTIONS(171), 1,
      anon_sym_benchAsync,
    ACTIONS(173), 1,
      anon_sym_after,
    ACTIONS(175), 1,
      anon_sym_RBRACE,
    STATE(197), 1,
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
    ACTIONS(56), 18,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [1145] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(179), 1,
      anon_sym_fairness,
    ACTIONS(181), 1,
      anon_sym_ms,
    STATE(8), 1,
      sym_duration_unit,
    ACTIONS(183), 2,
      anon_sym_s,
      anon_sym_m,
    ACTIONS(177), 30,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1194] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(187), 1,
      anon_sym_fairness,
    ACTIONS(185), 32,
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
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
      anon_sym_timeout,
      anon_sym_requires,
      anon_sym_order,
      anon_sym_mode,
      anon_sym_targetTime,
      anon_sym_sink,
      anon_sym_outlierDetection,
      anon_sym_cvThreshold,
      anon_sym_count,
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1235] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(191), 1,
      anon_sym_fairness,
    ACTIONS(189), 30,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1274] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(195), 1,
      anon_sym_fairness,
    ACTIONS(193), 30,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1313] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(199), 1,
      anon_sym_fairness,
    ACTIONS(197), 30,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1352] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(203), 1,
      anon_sym_fairness,
    ACTIONS(201), 30,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1391] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(179), 1,
      anon_sym_fairness,
    ACTIONS(181), 1,
      anon_sym_ms,
    STATE(8), 1,
      sym_duration_unit,
    ACTIONS(183), 2,
      anon_sym_s,
      anon_sym_m,
    ACTIONS(177), 26,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1436] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(207), 1,
      anon_sym_fairness,
    ACTIONS(205), 30,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1475] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(211), 1,
      anon_sym_fairness,
    ACTIONS(209), 30,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1514] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(215), 1,
      anon_sym_fairness,
    ACTIONS(213), 30,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1553] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(219), 1,
      anon_sym_fairness,
    ACTIONS(217), 30,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1592] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(223), 1,
      anon_sym_fairness,
    ACTIONS(221), 30,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1631] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(181), 1,
      anon_sym_ms,
    STATE(8), 1,
      sym_duration_unit,
    ACTIONS(179), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(183), 2,
      anon_sym_s,
      anon_sym_m,
    ACTIONS(177), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [1675] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(227), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(225), 26,
      ts_builtin_sym_end,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_suite,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [1711] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(231), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(229), 26,
      ts_builtin_sym_end,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_suite,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [1747] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(235), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(233), 26,
      ts_builtin_sym_end,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_suite,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [1783] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(239), 1,
      anon_sym_fairness,
    ACTIONS(237), 26,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1818] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(243), 1,
      anon_sym_fairness,
    ACTIONS(241), 26,
      anon_sym_RBRACE,
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
      anon_sym_memory,
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
    ACTIONS(247), 1,
      anon_sym_fairness,
    ACTIONS(245), 26,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1888] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(251), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(249), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [1922] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(255), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(253), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [1956] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(259), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(257), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [1990] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(263), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(261), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [2024] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(267), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(265), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [2058] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(271), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(269), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [2092] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(275), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(273), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [2126] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(279), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(277), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [2160] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(283), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(281), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [2194] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(287), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(285), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [2228] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(291), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(289), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [2262] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(295), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(293), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [2296] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(299), 2,
      anon_sym_bench,
      anon_sym_fairness,
    ACTIONS(297), 24,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
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
      anon_sym_memory,
      anon_sym_fairnessSeed,
      anon_sym_asyncSamplingPolicy,
      anon_sym_asyncWarmupCap,
      anon_sym_asyncSampleCap,
  [2330] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(301), 1,
      anon_sym_RPAREN,
    ACTIONS(305), 1,
      anon_sym_baseline,
    STATE(115), 1,
      sym_chart_param,
    STATE(196), 1,
      sym_chart_params,
    STATE(198), 1,
      sym_chart_param_name,
    ACTIONS(303), 15,
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
  [2366] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(305), 1,
      anon_sym_baseline,
    ACTIONS(307), 1,
      anon_sym_RPAREN,
    STATE(155), 1,
      sym_chart_param,
    STATE(198), 1,
      sym_chart_param_name,
    ACTIONS(303), 15,
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
  [2399] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(305), 1,
      anon_sym_baseline,
    ACTIONS(309), 1,
      anon_sym_RPAREN,
    STATE(155), 1,
      sym_chart_param,
    STATE(198), 1,
      sym_chart_param_name,
    ACTIONS(303), 15,
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
  [2432] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(305), 1,
      anon_sym_baseline,
    STATE(155), 1,
      sym_chart_param,
    STATE(198), 1,
      sym_chart_param_name,
    ACTIONS(303), 15,
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
  [2462] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(311), 1,
      sym_identifier,
    ACTIONS(313), 1,
      anon_sym_DQUOTE,
    ACTIONS(315), 1,
      anon_sym_SQUOTE,
    ACTIONS(317), 1,
      sym_number,
    ACTIONS(319), 1,
      sym_float,
    ACTIONS(323), 1,
      anon_sym_LBRACK,
    ACTIONS(321), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(15), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2495] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(313), 1,
      anon_sym_DQUOTE,
    ACTIONS(315), 1,
      anon_sym_SQUOTE,
    ACTIONS(323), 1,
      anon_sym_LBRACK,
    ACTIONS(325), 1,
      sym_identifier,
    ACTIONS(327), 1,
      sym_number,
    ACTIONS(329), 1,
      sym_float,
    ACTIONS(321), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(158), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2528] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(311), 1,
      sym_identifier,
    ACTIONS(313), 1,
      anon_sym_DQUOTE,
    ACTIONS(315), 1,
      anon_sym_SQUOTE,
    ACTIONS(319), 1,
      sym_float,
    ACTIONS(323), 1,
      anon_sym_LBRACK,
    ACTIONS(331), 1,
      sym_number,
    ACTIONS(321), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(15), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2561] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(311), 1,
      sym_identifier,
    ACTIONS(313), 1,
      anon_sym_DQUOTE,
    ACTIONS(315), 1,
      anon_sym_SQUOTE,
    ACTIONS(319), 1,
      sym_float,
    ACTIONS(323), 1,
      anon_sym_LBRACK,
    ACTIONS(333), 1,
      sym_number,
    ACTIONS(321), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(15), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2594] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(335), 1,
      anon_sym_RBRACE,
    ACTIONS(337), 1,
      anon_sym_import,
    ACTIONS(339), 1,
      anon_sym_declare,
    ACTIONS(341), 1,
      anon_sym_async,
    ACTIONS(343), 1,
      anon_sym_init,
    ACTIONS(345), 1,
      anon_sym_helpers,
    STATE(65), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [2624] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(347), 1,
      anon_sym_RBRACE,
    ACTIONS(349), 1,
      anon_sym_import,
    ACTIONS(352), 1,
      anon_sym_declare,
    ACTIONS(355), 1,
      anon_sym_async,
    ACTIONS(358), 1,
      anon_sym_init,
    ACTIONS(361), 1,
      anon_sym_helpers,
    STATE(64), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [2654] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(337), 1,
      anon_sym_import,
    ACTIONS(339), 1,
      anon_sym_declare,
    ACTIONS(341), 1,
      anon_sym_async,
    ACTIONS(343), 1,
      anon_sym_init,
    ACTIONS(345), 1,
      anon_sym_helpers,
    ACTIONS(364), 1,
      anon_sym_RBRACE,
    STATE(64), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [2684] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(313), 1,
      anon_sym_DQUOTE,
    ACTIONS(315), 1,
      anon_sym_SQUOTE,
    ACTIONS(323), 1,
      anon_sym_LBRACK,
    ACTIONS(366), 1,
      sym_number,
    ACTIONS(368), 1,
      sym_float,
    ACTIONS(370), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(150), 4,
      sym__chart_value,
      sym_string,
      sym_boolean,
      sym_string_array,
  [2713] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(372), 1,
      anon_sym_COLON,
    STATE(214), 1,
      sym_language_tag,
    STATE(26), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(60), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2734] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(374), 1,
      anon_sym_RBRACE,
    STATE(200), 1,
      sym_language_tag,
    STATE(70), 2,
      sym_language_implementation,
      aux_sym_hook_grouped_repeat1,
    ACTIONS(60), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2755] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(372), 1,
      anon_sym_COLON,
    STATE(214), 1,
      sym_language_tag,
    STATE(31), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(60), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2776] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(376), 1,
      anon_sym_RBRACE,
    STATE(200), 1,
      sym_language_tag,
    STATE(70), 2,
      sym_language_implementation,
      aux_sym_hook_grouped_repeat1,
    ACTIONS(378), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2797] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(372), 1,
      anon_sym_COLON,
    STATE(214), 1,
      sym_language_tag,
    STATE(32), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(60), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2818] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(381), 1,
      anon_sym_RBRACE,
    STATE(200), 1,
      sym_language_tag,
    STATE(68), 2,
      sym_language_implementation,
      aux_sym_hook_grouped_repeat1,
    ACTIONS(60), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2839] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(7), 1,
      anon_sym_use,
    ACTIONS(9), 1,
      anon_sym_globalSetup,
    ACTIONS(11), 1,
      anon_sym_suite,
    ACTIONS(383), 1,
      ts_builtin_sym_end,
    STATE(101), 1,
      sym_global_setup,
    STATE(84), 2,
      sym_use_statement,
      aux_sym_source_file_repeat1,
    STATE(100), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [2866] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(372), 1,
      anon_sym_COLON,
    STATE(214), 1,
      sym_language_tag,
    STATE(33), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(60), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2887] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(372), 1,
      anon_sym_COLON,
    STATE(214), 1,
      sym_language_tag,
    STATE(25), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(60), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2908] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(385), 1,
      sym_identifier,
    ACTIONS(387), 1,
      anon_sym_RBRACE,
    ACTIONS(389), 1,
      anon_sym_anvil,
    STATE(77), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(131), 2,
      sym_anvil_call,
      sym_function_call,
  [2929] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(385), 1,
      sym_identifier,
    ACTIONS(389), 1,
      anon_sym_anvil,
    ACTIONS(391), 1,
      anon_sym_RBRACE,
    STATE(78), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(131), 2,
      sym_anvil_call,
      sym_function_call,
  [2950] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(393), 1,
      sym_identifier,
    ACTIONS(396), 1,
      anon_sym_RBRACE,
    ACTIONS(398), 1,
      anon_sym_anvil,
    STATE(78), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(131), 2,
      sym_anvil_call,
      sym_function_call,
  [2971] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(183), 1,
      anon_sym_m,
    STATE(8), 1,
      sym_duration_unit,
    ACTIONS(177), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
    ACTIONS(181), 2,
      anon_sym_ms,
      anon_sym_s,
  [2989] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(401), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3001] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(403), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3013] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(405), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3025] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(407), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3037] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(411), 1,
      anon_sym_use,
    STATE(84), 2,
      sym_use_statement,
      aux_sym_source_file_repeat1,
    ACTIONS(409), 3,
      ts_builtin_sym_end,
      anon_sym_globalSetup,
      anon_sym_suite,
  [3053] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(414), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3065] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(416), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3077] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(152), 1,
      sym_language_tag,
    ACTIONS(60), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [3091] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(418), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [3103] = 5,
    ACTIONS(420), 1,
      anon_sym_DQUOTE,
    ACTIONS(424), 1,
      sym_comment,
    STATE(111), 1,
      aux_sym_string_content_repeat1,
    STATE(185), 1,
      sym_string_content,
    ACTIONS(422), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [3120] = 5,
    ACTIONS(420), 1,
      anon_sym_SQUOTE,
    ACTIONS(424), 1,
      sym_comment,
    STATE(114), 1,
      aux_sym_single_string_content_repeat1,
    STATE(184), 1,
      sym_single_string_content,
    ACTIONS(426), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [3137] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(313), 1,
      anon_sym_DQUOTE,
    ACTIONS(315), 1,
      anon_sym_SQUOTE,
    ACTIONS(428), 1,
      anon_sym_ATfile,
    STATE(41), 2,
      sym_file_ref,
      sym_string,
  [3154] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(430), 1,
      sym_identifier,
    ACTIONS(432), 1,
      anon_sym_RPAREN,
    STATE(141), 1,
      sym_argument,
    STATE(190), 1,
      sym_argument_list,
  [3170] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(313), 1,
      anon_sym_DQUOTE,
    ACTIONS(315), 1,
      anon_sym_SQUOTE,
    ACTIONS(434), 1,
      anon_sym_RBRACK,
    STATE(166), 1,
      sym_string,
  [3186] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_suite,
    ACTIONS(383), 1,
      ts_builtin_sym_end,
    STATE(100), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3200] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_suite,
    ACTIONS(383), 1,
      ts_builtin_sym_end,
    STATE(103), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3214] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(436), 1,
      anon_sym_RBRACE,
    ACTIONS(438), 1,
      anon_sym_charting,
    STATE(106), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [3228] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(440), 1,
      anon_sym_LBRACE,
    ACTIONS(442), 1,
      anon_sym_LPAREN,
    STATE(48), 1,
      sym_fixture_body,
    STATE(160), 1,
      sym_fixture_params,
  [3244] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(444), 1,
      anon_sym_LBRACE,
    ACTIONS(446), 1,
      anon_sym_LPAREN,
    STATE(81), 2,
      sym_code_block,
      sym_paren_code_block,
  [3258] = 4,
    ACTIONS(424), 1,
      sym_comment,
    ACTIONS(448), 1,
      anon_sym_LBRACE,
    ACTIONS(450), 1,
      sym_inline_code,
    STATE(24), 2,
      sym__code_or_inline,
      sym_code_block,
  [3272] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_suite,
    ACTIONS(452), 1,
      ts_builtin_sym_end,
    STATE(103), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3286] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_suite,
    ACTIONS(452), 1,
      ts_builtin_sym_end,
    STATE(107), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3300] = 4,
    ACTIONS(424), 1,
      sym_comment,
    ACTIONS(454), 1,
      anon_sym_DQUOTE,
    STATE(102), 1,
      aux_sym_string_content_repeat1,
    ACTIONS(456), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [3314] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(459), 1,
      ts_builtin_sym_end,
    ACTIONS(461), 1,
      anon_sym_suite,
    STATE(103), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3328] = 4,
    ACTIONS(424), 1,
      sym_comment,
    ACTIONS(464), 1,
      anon_sym_SQUOTE,
    STATE(104), 1,
      aux_sym_single_string_content_repeat1,
    ACTIONS(466), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [3342] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(313), 1,
      anon_sym_DQUOTE,
    ACTIONS(315), 1,
      anon_sym_SQUOTE,
    ACTIONS(469), 1,
      anon_sym_RBRACK,
    STATE(166), 1,
      sym_string,
  [3358] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(438), 1,
      anon_sym_charting,
    ACTIONS(471), 1,
      anon_sym_RBRACE,
    STATE(113), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [3372] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_suite,
    ACTIONS(473), 1,
      ts_builtin_sym_end,
    STATE(103), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3386] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(313), 1,
      anon_sym_DQUOTE,
    ACTIONS(315), 1,
      anon_sym_SQUOTE,
    ACTIONS(475), 1,
      anon_sym_RBRACK,
    STATE(122), 1,
      sym_string,
  [3402] = 4,
    ACTIONS(424), 1,
      sym_comment,
    ACTIONS(448), 1,
      anon_sym_LBRACE,
    ACTIONS(477), 1,
      sym_inline_code,
    STATE(28), 2,
      sym__code_or_inline,
      sym_code_block,
  [3416] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(479), 4,
      ts_builtin_sym_end,
      anon_sym_use,
      anon_sym_globalSetup,
      anon_sym_suite,
  [3426] = 4,
    ACTIONS(424), 1,
      sym_comment,
    ACTIONS(481), 1,
      anon_sym_DQUOTE,
    STATE(102), 1,
      aux_sym_string_content_repeat1,
    ACTIONS(483), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [3440] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(430), 1,
      sym_identifier,
    ACTIONS(485), 1,
      anon_sym_RPAREN,
    STATE(141), 1,
      sym_argument,
    STATE(212), 1,
      sym_argument_list,
  [3456] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(487), 1,
      anon_sym_RBRACE,
    ACTIONS(489), 1,
      anon_sym_charting,
    STATE(113), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [3470] = 4,
    ACTIONS(424), 1,
      sym_comment,
    ACTIONS(492), 1,
      anon_sym_SQUOTE,
    STATE(104), 1,
      aux_sym_single_string_content_repeat1,
    ACTIONS(494), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [3484] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(496), 1,
      anon_sym_RPAREN,
    ACTIONS(498), 1,
      anon_sym_COMMA,
    STATE(138), 1,
      aux_sym_chart_params_repeat1,
  [3497] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(502), 1,
      anon_sym_RBRACE,
    ACTIONS(500), 2,
      anon_sym_anvil,
      sym_identifier,
  [3508] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(506), 1,
      anon_sym_RBRACE,
    ACTIONS(504), 2,
      anon_sym_anvil,
      sym_identifier,
  [3519] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(508), 1,
      anon_sym_COMMA,
    ACTIONS(511), 1,
      anon_sym_RBRACK,
    STATE(118), 1,
      aux_sym_string_array_repeat1,
  [3532] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(430), 1,
      sym_identifier,
    ACTIONS(513), 1,
      anon_sym_RPAREN,
    STATE(147), 1,
      sym_argument,
  [3545] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(517), 1,
      anon_sym_RBRACE,
    ACTIONS(515), 2,
      anon_sym_anvil,
      sym_identifier,
  [3556] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(313), 1,
      anon_sym_DQUOTE,
    ACTIONS(315), 1,
      anon_sym_SQUOTE,
    STATE(201), 1,
      sym_string,
  [3569] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(519), 1,
      anon_sym_COMMA,
    ACTIONS(521), 1,
      anon_sym_RBRACK,
    STATE(128), 1,
      aux_sym_string_array_repeat1,
  [3582] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(313), 1,
      anon_sym_DQUOTE,
    ACTIONS(315), 1,
      anon_sym_SQUOTE,
    STATE(204), 1,
      sym_string,
  [3595] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(525), 1,
      anon_sym_RBRACE,
    ACTIONS(523), 2,
      anon_sym_anvil,
      sym_identifier,
  [3606] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(529), 1,
      anon_sym_RBRACE,
    ACTIONS(527), 2,
      anon_sym_anvil,
      sym_identifier,
  [3617] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(531), 1,
      anon_sym_RPAREN,
    ACTIONS(533), 1,
      anon_sym_COMMA,
    STATE(126), 1,
      aux_sym_fixture_params_repeat1,
  [3630] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(536), 1,
      sym_identifier,
    ACTIONS(538), 1,
      anon_sym_RPAREN,
    STATE(168), 1,
      sym_fixture_param,
  [3643] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(469), 1,
      anon_sym_RBRACK,
    ACTIONS(540), 1,
      anon_sym_COMMA,
    STATE(118), 1,
      aux_sym_string_array_repeat1,
  [3656] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(542), 1,
      anon_sym_RPAREN,
    ACTIONS(544), 1,
      anon_sym_COMMA,
    STATE(132), 1,
      aux_sym_fixture_params_repeat1,
  [3669] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(209), 1,
      sym_chart_function_name,
    ACTIONS(546), 2,
      anon_sym_drawSpeedupChart,
      anon_sym_drawTable,
  [3680] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(550), 1,
      anon_sym_RBRACE,
    ACTIONS(548), 2,
      anon_sym_anvil,
      sym_identifier,
  [3691] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(552), 1,
      anon_sym_RPAREN,
    ACTIONS(554), 1,
      anon_sym_COMMA,
    STATE(126), 1,
      aux_sym_fixture_params_repeat1,
  [3704] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(536), 1,
      sym_identifier,
    ACTIONS(556), 1,
      anon_sym_RPAREN,
    STATE(129), 1,
      sym_fixture_param,
  [3717] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(558), 1,
      anon_sym_RPAREN,
    ACTIONS(560), 1,
      anon_sym_COMMA,
    STATE(142), 1,
      aux_sym_argument_list_repeat1,
  [3730] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(430), 1,
      sym_identifier,
    ACTIONS(558), 1,
      anon_sym_RPAREN,
    STATE(147), 1,
      sym_argument,
  [3743] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(564), 1,
      anon_sym_RBRACE,
    ACTIONS(562), 2,
      anon_sym_anvil,
      sym_identifier,
  [3754] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(313), 1,
      anon_sym_DQUOTE,
    ACTIONS(315), 1,
      anon_sym_SQUOTE,
    STATE(166), 1,
      sym_string,
  [3767] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(309), 1,
      anon_sym_RPAREN,
    ACTIONS(566), 1,
      anon_sym_COMMA,
    STATE(140), 1,
      aux_sym_chart_params_repeat1,
  [3780] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(568), 1,
      anon_sym_RPAREN,
    ACTIONS(570), 1,
      anon_sym_fork,
    STATE(187), 1,
      sym_anvil_args,
  [3793] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(572), 1,
      anon_sym_RPAREN,
    ACTIONS(574), 1,
      anon_sym_COMMA,
    STATE(140), 1,
      aux_sym_chart_params_repeat1,
  [3806] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(577), 1,
      anon_sym_RPAREN,
    ACTIONS(579), 1,
      anon_sym_COMMA,
    STATE(134), 1,
      aux_sym_argument_list_repeat1,
  [3819] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(581), 1,
      anon_sym_RPAREN,
    ACTIONS(583), 1,
      anon_sym_COMMA,
    STATE(142), 1,
      aux_sym_argument_list_repeat1,
  [3832] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(536), 1,
      sym_identifier,
    ACTIONS(552), 1,
      anon_sym_RPAREN,
    STATE(168), 1,
      sym_fixture_param,
  [3845] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(536), 1,
      sym_identifier,
    STATE(168), 1,
      sym_fixture_param,
  [3855] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(586), 2,
      ts_builtin_sym_end,
      anon_sym_suite,
  [3863] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(588), 1,
      anon_sym_LBRACE,
    STATE(38), 1,
      sym_global_setup_body,
  [3873] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(581), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [3881] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(323), 1,
      anon_sym_LBRACK,
    STATE(34), 1,
      sym_string_array,
  [3891] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(444), 1,
      anon_sym_LBRACE,
    STATE(80), 1,
      sym_code_block,
  [3901] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(590), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [3909] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(592), 1,
      anon_sym_LBRACE,
    STATE(42), 1,
      sym_benchmark_body,
  [3919] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(594), 1,
      anon_sym_LBRACE,
    STATE(54), 1,
      sym_setup_body,
  [3929] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(596), 2,
      anon_sym_LBRACE,
      anon_sym_COLON,
  [3937] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(598), 1,
      anon_sym_LBRACE,
    STATE(165), 1,
      sym_suite_body,
  [3947] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(572), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [3955] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(444), 1,
      anon_sym_LBRACE,
    STATE(82), 1,
      sym_code_block,
  [3965] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(444), 1,
      anon_sym_LBRACE,
    STATE(83), 1,
      sym_code_block,
  [3975] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(600), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [3983] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(444), 1,
      anon_sym_LBRACE,
    STATE(40), 1,
      sym_code_block,
  [3993] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(440), 1,
      anon_sym_LBRACE,
    STATE(46), 1,
      sym_fixture_body,
  [4003] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(444), 1,
      anon_sym_LBRACE,
    STATE(86), 1,
      sym_code_block,
  [4013] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(602), 2,
      anon_sym_RBRACE,
      anon_sym_charting,
  [4021] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(604), 1,
      anon_sym_DOT,
    ACTIONS(606), 1,
      anon_sym_LPAREN,
  [4031] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(608), 2,
      anon_sym_RBRACE,
      anon_sym_charting,
  [4039] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(610), 2,
      ts_builtin_sym_end,
      anon_sym_suite,
  [4047] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(511), 2,
      anon_sym_COMMA,
      anon_sym_RBRACK,
  [4055] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(612), 2,
      ts_builtin_sym_end,
      anon_sym_suite,
  [4063] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(531), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4071] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(614), 1,
      anon_sym_RPAREN,
    ACTIONS(616), 1,
      sym_embedded_code,
  [4081] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(618), 1,
      anon_sym_RBRACE,
    ACTIONS(620), 1,
      sym_embedded_code,
  [4091] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(622), 1,
      anon_sym_LBRACE,
    STATE(49), 1,
      sym_after_body,
  [4101] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(430), 1,
      sym_identifier,
    STATE(147), 1,
      sym_argument,
  [4111] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(624), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4119] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(626), 1,
      sym_identifier,
  [4126] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(628), 1,
      anon_sym_LBRACE,
  [4133] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(630), 1,
      anon_sym_LPAREN,
  [4140] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(632), 1,
      anon_sym_COLON,
  [4147] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(634), 1,
      sym_identifier,
  [4154] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(636), 1,
      anon_sym_LBRACE,
  [4161] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(638), 1,
      sym_identifier,
  [4168] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(640), 1,
      anon_sym_COLON,
  [4175] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(642), 1,
      sym_identifier,
  [4182] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(644), 1,
      anon_sym_LPAREN,
  [4189] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(646), 1,
      anon_sym_SQUOTE,
  [4196] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(646), 1,
      anon_sym_DQUOTE,
  [4203] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(648), 1,
      anon_sym_spawnAnvil,
  [4210] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(650), 1,
      anon_sym_RPAREN,
  [4217] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(652), 1,
      anon_sym_std,
  [4224] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(654), 1,
      anon_sym_LBRACE,
  [4231] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(656), 1,
      anon_sym_RPAREN,
  [4238] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(658), 1,
      anon_sym_COLON,
  [4245] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(660), 1,
      anon_sym_RPAREN,
  [4252] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(662), 1,
      anon_sym_COLON,
  [4259] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(664), 1,
      anon_sym_LPAREN,
  [4266] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(666), 1,
      anon_sym_COLON,
  [4273] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(668), 1,
      anon_sym_RPAREN,
  [4280] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(670), 1,
      anon_sym_COLON,
  [4287] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(672), 1,
      anon_sym_COLON,
  [4294] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(674), 1,
      anon_sym_RBRACE,
  [4301] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(676), 1,
      anon_sym_COLON,
  [4308] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(678), 1,
      anon_sym_RPAREN,
  [4315] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(680), 1,
      anon_sym_DOT,
  [4322] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(682), 1,
      sym_identifier,
  [4329] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(684), 1,
      anon_sym_RPAREN,
  [4336] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(686), 1,
      anon_sym_COLON,
  [4343] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(688), 1,
      anon_sym_COLON,
  [4350] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(690), 1,
      anon_sym_DOT,
  [4357] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(692), 1,
      anon_sym_COLON,
  [4364] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(694), 1,
      anon_sym_LPAREN,
  [4371] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(696), 1,
      anon_sym_LPAREN,
  [4378] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(698), 1,
      anon_sym_init,
  [4385] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(700), 1,
      anon_sym_RPAREN,
  [4392] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(702), 1,
      anon_sym_COLON_COLON,
  [4399] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(704), 1,
      anon_sym_COLON,
  [4406] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(706), 1,
      anon_sym_LBRACE,
  [4413] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(708), 1,
      ts_builtin_sym_end,
  [4420] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(710), 1,
      sym_identifier,
  [4427] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(712), 1,
      anon_sym_LBRACE,
  [4434] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(714), 1,
      anon_sym_COLON,
  [4441] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(716), 1,
      anon_sym_COLON,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 73,
  [SMALL_STATE(4)] = 146,
  [SMALL_STATE(5)] = 219,
  [SMALL_STATE(6)] = 268,
  [SMALL_STATE(7)] = 317,
  [SMALL_STATE(8)] = 365,
  [SMALL_STATE(9)] = 413,
  [SMALL_STATE(10)] = 461,
  [SMALL_STATE(11)] = 509,
  [SMALL_STATE(12)] = 557,
  [SMALL_STATE(13)] = 605,
  [SMALL_STATE(14)] = 653,
  [SMALL_STATE(15)] = 699,
  [SMALL_STATE(16)] = 745,
  [SMALL_STATE(17)] = 791,
  [SMALL_STATE(18)] = 848,
  [SMALL_STATE(19)] = 909,
  [SMALL_STATE(20)] = 966,
  [SMALL_STATE(21)] = 1023,
  [SMALL_STATE(22)] = 1084,
  [SMALL_STATE(23)] = 1145,
  [SMALL_STATE(24)] = 1194,
  [SMALL_STATE(25)] = 1235,
  [SMALL_STATE(26)] = 1274,
  [SMALL_STATE(27)] = 1313,
  [SMALL_STATE(28)] = 1352,
  [SMALL_STATE(29)] = 1391,
  [SMALL_STATE(30)] = 1436,
  [SMALL_STATE(31)] = 1475,
  [SMALL_STATE(32)] = 1514,
  [SMALL_STATE(33)] = 1553,
  [SMALL_STATE(34)] = 1592,
  [SMALL_STATE(35)] = 1631,
  [SMALL_STATE(36)] = 1675,
  [SMALL_STATE(37)] = 1711,
  [SMALL_STATE(38)] = 1747,
  [SMALL_STATE(39)] = 1783,
  [SMALL_STATE(40)] = 1818,
  [SMALL_STATE(41)] = 1853,
  [SMALL_STATE(42)] = 1888,
  [SMALL_STATE(43)] = 1922,
  [SMALL_STATE(44)] = 1956,
  [SMALL_STATE(45)] = 1990,
  [SMALL_STATE(46)] = 2024,
  [SMALL_STATE(47)] = 2058,
  [SMALL_STATE(48)] = 2092,
  [SMALL_STATE(49)] = 2126,
  [SMALL_STATE(50)] = 2160,
  [SMALL_STATE(51)] = 2194,
  [SMALL_STATE(52)] = 2228,
  [SMALL_STATE(53)] = 2262,
  [SMALL_STATE(54)] = 2296,
  [SMALL_STATE(55)] = 2330,
  [SMALL_STATE(56)] = 2366,
  [SMALL_STATE(57)] = 2399,
  [SMALL_STATE(58)] = 2432,
  [SMALL_STATE(59)] = 2462,
  [SMALL_STATE(60)] = 2495,
  [SMALL_STATE(61)] = 2528,
  [SMALL_STATE(62)] = 2561,
  [SMALL_STATE(63)] = 2594,
  [SMALL_STATE(64)] = 2624,
  [SMALL_STATE(65)] = 2654,
  [SMALL_STATE(66)] = 2684,
  [SMALL_STATE(67)] = 2713,
  [SMALL_STATE(68)] = 2734,
  [SMALL_STATE(69)] = 2755,
  [SMALL_STATE(70)] = 2776,
  [SMALL_STATE(71)] = 2797,
  [SMALL_STATE(72)] = 2818,
  [SMALL_STATE(73)] = 2839,
  [SMALL_STATE(74)] = 2866,
  [SMALL_STATE(75)] = 2887,
  [SMALL_STATE(76)] = 2908,
  [SMALL_STATE(77)] = 2929,
  [SMALL_STATE(78)] = 2950,
  [SMALL_STATE(79)] = 2971,
  [SMALL_STATE(80)] = 2989,
  [SMALL_STATE(81)] = 3001,
  [SMALL_STATE(82)] = 3013,
  [SMALL_STATE(83)] = 3025,
  [SMALL_STATE(84)] = 3037,
  [SMALL_STATE(85)] = 3053,
  [SMALL_STATE(86)] = 3065,
  [SMALL_STATE(87)] = 3077,
  [SMALL_STATE(88)] = 3091,
  [SMALL_STATE(89)] = 3103,
  [SMALL_STATE(90)] = 3120,
  [SMALL_STATE(91)] = 3137,
  [SMALL_STATE(92)] = 3154,
  [SMALL_STATE(93)] = 3170,
  [SMALL_STATE(94)] = 3186,
  [SMALL_STATE(95)] = 3200,
  [SMALL_STATE(96)] = 3214,
  [SMALL_STATE(97)] = 3228,
  [SMALL_STATE(98)] = 3244,
  [SMALL_STATE(99)] = 3258,
  [SMALL_STATE(100)] = 3272,
  [SMALL_STATE(101)] = 3286,
  [SMALL_STATE(102)] = 3300,
  [SMALL_STATE(103)] = 3314,
  [SMALL_STATE(104)] = 3328,
  [SMALL_STATE(105)] = 3342,
  [SMALL_STATE(106)] = 3358,
  [SMALL_STATE(107)] = 3372,
  [SMALL_STATE(108)] = 3386,
  [SMALL_STATE(109)] = 3402,
  [SMALL_STATE(110)] = 3416,
  [SMALL_STATE(111)] = 3426,
  [SMALL_STATE(112)] = 3440,
  [SMALL_STATE(113)] = 3456,
  [SMALL_STATE(114)] = 3470,
  [SMALL_STATE(115)] = 3484,
  [SMALL_STATE(116)] = 3497,
  [SMALL_STATE(117)] = 3508,
  [SMALL_STATE(118)] = 3519,
  [SMALL_STATE(119)] = 3532,
  [SMALL_STATE(120)] = 3545,
  [SMALL_STATE(121)] = 3556,
  [SMALL_STATE(122)] = 3569,
  [SMALL_STATE(123)] = 3582,
  [SMALL_STATE(124)] = 3595,
  [SMALL_STATE(125)] = 3606,
  [SMALL_STATE(126)] = 3617,
  [SMALL_STATE(127)] = 3630,
  [SMALL_STATE(128)] = 3643,
  [SMALL_STATE(129)] = 3656,
  [SMALL_STATE(130)] = 3669,
  [SMALL_STATE(131)] = 3680,
  [SMALL_STATE(132)] = 3691,
  [SMALL_STATE(133)] = 3704,
  [SMALL_STATE(134)] = 3717,
  [SMALL_STATE(135)] = 3730,
  [SMALL_STATE(136)] = 3743,
  [SMALL_STATE(137)] = 3754,
  [SMALL_STATE(138)] = 3767,
  [SMALL_STATE(139)] = 3780,
  [SMALL_STATE(140)] = 3793,
  [SMALL_STATE(141)] = 3806,
  [SMALL_STATE(142)] = 3819,
  [SMALL_STATE(143)] = 3832,
  [SMALL_STATE(144)] = 3845,
  [SMALL_STATE(145)] = 3855,
  [SMALL_STATE(146)] = 3863,
  [SMALL_STATE(147)] = 3873,
  [SMALL_STATE(148)] = 3881,
  [SMALL_STATE(149)] = 3891,
  [SMALL_STATE(150)] = 3901,
  [SMALL_STATE(151)] = 3909,
  [SMALL_STATE(152)] = 3919,
  [SMALL_STATE(153)] = 3929,
  [SMALL_STATE(154)] = 3937,
  [SMALL_STATE(155)] = 3947,
  [SMALL_STATE(156)] = 3955,
  [SMALL_STATE(157)] = 3965,
  [SMALL_STATE(158)] = 3975,
  [SMALL_STATE(159)] = 3983,
  [SMALL_STATE(160)] = 3993,
  [SMALL_STATE(161)] = 4003,
  [SMALL_STATE(162)] = 4013,
  [SMALL_STATE(163)] = 4021,
  [SMALL_STATE(164)] = 4031,
  [SMALL_STATE(165)] = 4039,
  [SMALL_STATE(166)] = 4047,
  [SMALL_STATE(167)] = 4055,
  [SMALL_STATE(168)] = 4063,
  [SMALL_STATE(169)] = 4071,
  [SMALL_STATE(170)] = 4081,
  [SMALL_STATE(171)] = 4091,
  [SMALL_STATE(172)] = 4101,
  [SMALL_STATE(173)] = 4111,
  [SMALL_STATE(174)] = 4119,
  [SMALL_STATE(175)] = 4126,
  [SMALL_STATE(176)] = 4133,
  [SMALL_STATE(177)] = 4140,
  [SMALL_STATE(178)] = 4147,
  [SMALL_STATE(179)] = 4154,
  [SMALL_STATE(180)] = 4161,
  [SMALL_STATE(181)] = 4168,
  [SMALL_STATE(182)] = 4175,
  [SMALL_STATE(183)] = 4182,
  [SMALL_STATE(184)] = 4189,
  [SMALL_STATE(185)] = 4196,
  [SMALL_STATE(186)] = 4203,
  [SMALL_STATE(187)] = 4210,
  [SMALL_STATE(188)] = 4217,
  [SMALL_STATE(189)] = 4224,
  [SMALL_STATE(190)] = 4231,
  [SMALL_STATE(191)] = 4238,
  [SMALL_STATE(192)] = 4245,
  [SMALL_STATE(193)] = 4252,
  [SMALL_STATE(194)] = 4259,
  [SMALL_STATE(195)] = 4266,
  [SMALL_STATE(196)] = 4273,
  [SMALL_STATE(197)] = 4280,
  [SMALL_STATE(198)] = 4287,
  [SMALL_STATE(199)] = 4294,
  [SMALL_STATE(200)] = 4301,
  [SMALL_STATE(201)] = 4308,
  [SMALL_STATE(202)] = 4315,
  [SMALL_STATE(203)] = 4322,
  [SMALL_STATE(204)] = 4329,
  [SMALL_STATE(205)] = 4336,
  [SMALL_STATE(206)] = 4343,
  [SMALL_STATE(207)] = 4350,
  [SMALL_STATE(208)] = 4357,
  [SMALL_STATE(209)] = 4364,
  [SMALL_STATE(210)] = 4371,
  [SMALL_STATE(211)] = 4378,
  [SMALL_STATE(212)] = 4385,
  [SMALL_STATE(213)] = 4392,
  [SMALL_STATE(214)] = 4399,
  [SMALL_STATE(215)] = 4406,
  [SMALL_STATE(216)] = 4413,
  [SMALL_STATE(217)] = 4420,
  [SMALL_STATE(218)] = 4427,
  [SMALL_STATE(219)] = 4434,
  [SMALL_STATE(220)] = 4441,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0, 0, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(188),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(146),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(217),
  [13] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0),
  [15] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(191),
  [18] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(69),
  [21] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(71),
  [24] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(74),
  [27] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(75),
  [30] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(67),
  [33] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(177),
  [36] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(177),
  [39] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(153),
  [42] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [44] = {.entry = {.count = 1, .reusable = true}}, SHIFT(191),
  [46] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [48] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [50] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [52] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [54] = {.entry = {.count = 1, .reusable = true}}, SHIFT(67),
  [56] = {.entry = {.count = 1, .reusable = true}}, SHIFT(177),
  [58] = {.entry = {.count = 1, .reusable = false}}, SHIFT(177),
  [60] = {.entry = {.count = 1, .reusable = true}}, SHIFT(153),
  [62] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [64] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 3, 0, 0),
  [66] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 3, 0, 0),
  [68] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 2, 0, 0),
  [70] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 2, 0, 0),
  [72] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_duration_unit, 1, 0, 0),
  [74] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_duration_unit, 1, 0, 0),
  [76] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_duration, 2, 0, 0),
  [78] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_duration, 2, 0, 0),
  [80] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_boolean, 1, 0, 0),
  [82] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_boolean, 1, 0, 0),
  [84] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 2, 0, 0),
  [86] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 2, 0, 0),
  [88] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 3, 0, 0),
  [90] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 3, 0, 0),
  [92] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 4, 0, 0),
  [94] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 4, 0, 0),
  [96] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 5, 0, 0),
  [98] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 5, 0, 0),
  [100] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_code_block, 2, 0, 0),
  [102] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_code_block, 2, 0, 0),
  [104] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property, 3, 0, 4),
  [106] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_property, 3, 0, 4),
  [108] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_code_block, 3, 0, 0),
  [110] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_code_block, 3, 0, 0),
  [112] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [114] = {.entry = {.count = 1, .reusable = true}}, SHIFT(206),
  [116] = {.entry = {.count = 1, .reusable = true}}, SHIFT(205),
  [118] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(146),
  [121] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0),
  [123] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(87),
  [126] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(180),
  [129] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(178),
  [132] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(178),
  [135] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(171),
  [138] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(177),
  [141] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(177),
  [144] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0),
  [146] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(206),
  [149] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(205),
  [152] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(177),
  [155] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(177),
  [158] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(153),
  [161] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [163] = {.entry = {.count = 1, .reusable = true}}, SHIFT(167),
  [165] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [167] = {.entry = {.count = 1, .reusable = true}}, SHIFT(180),
  [169] = {.entry = {.count = 1, .reusable = false}}, SHIFT(178),
  [171] = {.entry = {.count = 1, .reusable = true}}, SHIFT(178),
  [173] = {.entry = {.count = 1, .reusable = true}}, SHIFT(171),
  [175] = {.entry = {.count = 1, .reusable = true}}, SHIFT(145),
  [177] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__value, 1, 0, 0),
  [179] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__value, 1, 0, 0),
  [181] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [183] = {.entry = {.count = 1, .reusable = false}}, SHIFT(7),
  [185] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_language_implementation, 3, 0, 5),
  [187] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_language_implementation, 3, 0, 5),
  [189] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_hook, 2, 0, 0),
  [191] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_hook, 2, 0, 0),
  [193] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_each_hook, 2, 0, 0),
  [195] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_each_hook, 2, 0, 0),
  [197] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_grouped, 4, 0, 0),
  [199] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hook_grouped, 4, 0, 0),
  [201] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_flat, 3, 0, 5),
  [203] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hook_flat, 3, 0, 5),
  [205] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_grouped, 3, 0, 0),
  [207] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hook_grouped, 3, 0, 0),
  [209] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_skip_hook, 2, 0, 0),
  [211] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_skip_hook, 2, 0, 0),
  [213] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_validate_hook, 2, 0, 0),
  [215] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_validate_hook, 2, 0, 0),
  [217] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_before_hook, 2, 0, 0),
  [219] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_before_hook, 2, 0, 0),
  [221] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tags_property, 3, 0, 0),
  [223] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_tags_property, 3, 0, 0),
  [225] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_body, 2, 0, 0),
  [227] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup_body, 2, 0, 0),
  [229] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_body, 3, 0, 0),
  [231] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup_body, 3, 0, 0),
  [233] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup, 2, 0, 0),
  [235] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup, 2, 0, 0),
  [237] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_file_ref, 4, 0, 0),
  [239] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_file_ref, 4, 0, 0),
  [241] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_shape_property, 3, 0, 0),
  [243] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_shape_property, 3, 0, 0),
  [245] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hex_property, 3, 0, 0),
  [247] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hex_property, 3, 0, 0),
  [249] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark, 3, 0, 1),
  [251] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_benchmark, 3, 0, 1),
  [253] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_body, 2, 0, 0),
  [255] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_body, 2, 0, 0),
  [257] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark_body, 3, 0, 0),
  [259] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_benchmark_body, 3, 0, 0),
  [261] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_body, 2, 0, 0),
  [263] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_setup_body, 2, 0, 0),
  [265] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture, 4, 0, 1),
  [267] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture, 4, 0, 1),
  [269] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_body, 3, 0, 0),
  [271] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_body, 3, 0, 0),
  [273] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture, 3, 0, 1),
  [275] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture, 3, 0, 1),
  [277] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_block, 2, 0, 0),
  [279] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_block, 2, 0, 0),
  [281] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_body, 3, 0, 0),
  [283] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture_body, 3, 0, 0),
  [285] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark_body, 2, 0, 0),
  [287] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_benchmark_body, 2, 0, 0),
  [289] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_body, 3, 0, 0),
  [291] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_setup_body, 3, 0, 0),
  [293] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_body, 2, 0, 0),
  [295] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture_body, 2, 0, 0),
  [297] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_block, 3, 0, 3),
  [299] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_setup_block, 3, 0, 3),
  [301] = {.entry = {.count = 1, .reusable = true}}, SHIFT(164),
  [303] = {.entry = {.count = 1, .reusable = true}}, SHIFT(195),
  [305] = {.entry = {.count = 1, .reusable = false}}, SHIFT(195),
  [307] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 3, 0, 0),
  [309] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 2, 0, 0),
  [311] = {.entry = {.count = 1, .reusable = false}}, SHIFT(15),
  [313] = {.entry = {.count = 1, .reusable = true}}, SHIFT(89),
  [315] = {.entry = {.count = 1, .reusable = true}}, SHIFT(90),
  [317] = {.entry = {.count = 1, .reusable = false}}, SHIFT(29),
  [319] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [321] = {.entry = {.count = 1, .reusable = false}}, SHIFT(9),
  [323] = {.entry = {.count = 1, .reusable = true}}, SHIFT(108),
  [325] = {.entry = {.count = 1, .reusable = false}}, SHIFT(158),
  [327] = {.entry = {.count = 1, .reusable = false}}, SHIFT(79),
  [329] = {.entry = {.count = 1, .reusable = true}}, SHIFT(158),
  [331] = {.entry = {.count = 1, .reusable = false}}, SHIFT(35),
  [333] = {.entry = {.count = 1, .reusable = false}}, SHIFT(23),
  [335] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [337] = {.entry = {.count = 1, .reusable = true}}, SHIFT(98),
  [339] = {.entry = {.count = 1, .reusable = true}}, SHIFT(149),
  [341] = {.entry = {.count = 1, .reusable = true}}, SHIFT(211),
  [343] = {.entry = {.count = 1, .reusable = true}}, SHIFT(156),
  [345] = {.entry = {.count = 1, .reusable = true}}, SHIFT(157),
  [347] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0),
  [349] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(98),
  [352] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(149),
  [355] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(211),
  [358] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(156),
  [361] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(157),
  [364] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [366] = {.entry = {.count = 1, .reusable = false}}, SHIFT(150),
  [368] = {.entry = {.count = 1, .reusable = true}}, SHIFT(150),
  [370] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [372] = {.entry = {.count = 1, .reusable = true}}, SHIFT(215),
  [374] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [376] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_hook_grouped_repeat1, 2, 0, 0),
  [378] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_hook_grouped_repeat1, 2, 0, 0), SHIFT_REPEAT(153),
  [381] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [383] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, 0, 0),
  [385] = {.entry = {.count = 1, .reusable = false}}, SHIFT(163),
  [387] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [389] = {.entry = {.count = 1, .reusable = false}}, SHIFT(202),
  [391] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [393] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(163),
  [396] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0),
  [398] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(202),
  [401] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declare_section, 2, 0, 0),
  [403] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_section, 2, 0, 0),
  [405] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_init_section, 2, 0, 0),
  [407] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_helpers_section, 2, 0, 0),
  [409] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0),
  [411] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(188),
  [414] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_paren_code_block, 2, 0, 0),
  [416] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_init_section, 3, 0, 0),
  [418] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_paren_code_block, 3, 0, 0),
  [420] = {.entry = {.count = 1, .reusable = false}}, SHIFT(6),
  [422] = {.entry = {.count = 1, .reusable = false}}, SHIFT(111),
  [424] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [426] = {.entry = {.count = 1, .reusable = false}}, SHIFT(114),
  [428] = {.entry = {.count = 1, .reusable = true}}, SHIFT(176),
  [430] = {.entry = {.count = 1, .reusable = true}}, SHIFT(208),
  [432] = {.entry = {.count = 1, .reusable = true}}, SHIFT(117),
  [434] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [436] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [438] = {.entry = {.count = 1, .reusable = true}}, SHIFT(207),
  [440] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [442] = {.entry = {.count = 1, .reusable = true}}, SHIFT(133),
  [444] = {.entry = {.count = 1, .reusable = true}}, SHIFT(170),
  [446] = {.entry = {.count = 1, .reusable = true}}, SHIFT(169),
  [448] = {.entry = {.count = 1, .reusable = false}}, SHIFT(170),
  [450] = {.entry = {.count = 1, .reusable = false}}, SHIFT(24),
  [452] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, 0, 0),
  [454] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_string_content_repeat1, 2, 0, 0),
  [456] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_string_content_repeat1, 2, 0, 0), SHIFT_REPEAT(102),
  [459] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat2, 2, 0, 0),
  [461] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat2, 2, 0, 0), SHIFT_REPEAT(217),
  [464] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_single_string_content_repeat1, 2, 0, 0),
  [466] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_single_string_content_repeat1, 2, 0, 0), SHIFT_REPEAT(104),
  [469] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [471] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [473] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, 0, 0),
  [475] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [477] = {.entry = {.count = 1, .reusable = false}}, SHIFT(28),
  [479] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_use_statement, 4, 0, 2),
  [481] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_content, 1, 0, 0),
  [483] = {.entry = {.count = 1, .reusable = false}}, SHIFT(102),
  [485] = {.entry = {.count = 1, .reusable = true}}, SHIFT(116),
  [487] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_after_body_repeat1, 2, 0, 0),
  [489] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_after_body_repeat1, 2, 0, 0), SHIFT_REPEAT(207),
  [492] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_single_string_content, 1, 0, 0),
  [494] = {.entry = {.count = 1, .reusable = false}}, SHIFT(104),
  [496] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 1, 0, 0),
  [498] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [500] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 3, 0, 0),
  [502] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 3, 0, 0),
  [504] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 5, 0, 0),
  [506] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 5, 0, 0),
  [508] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_string_array_repeat1, 2, 0, 0), SHIFT_REPEAT(137),
  [511] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_string_array_repeat1, 2, 0, 0),
  [513] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 3, 0, 0),
  [515] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_anvil_call, 5, 0, 0),
  [517] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_call, 5, 0, 0),
  [519] = {.entry = {.count = 1, .reusable = true}}, SHIFT(105),
  [521] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [523] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_anvil_call, 6, 0, 0),
  [525] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_call, 6, 0, 0),
  [527] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 6, 0, 0),
  [529] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 6, 0, 0),
  [531] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fixture_params_repeat1, 2, 0, 0),
  [533] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_params_repeat1, 2, 0, 0), SHIFT_REPEAT(144),
  [536] = {.entry = {.count = 1, .reusable = true}}, SHIFT(193),
  [538] = {.entry = {.count = 1, .reusable = true}}, SHIFT(189),
  [540] = {.entry = {.count = 1, .reusable = true}}, SHIFT(93),
  [542] = {.entry = {.count = 1, .reusable = true}}, SHIFT(218),
  [544] = {.entry = {.count = 1, .reusable = true}}, SHIFT(143),
  [546] = {.entry = {.count = 1, .reusable = true}}, SHIFT(210),
  [548] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup_statement, 1, 0, 0),
  [550] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_statement, 1, 0, 0),
  [552] = {.entry = {.count = 1, .reusable = true}}, SHIFT(179),
  [554] = {.entry = {.count = 1, .reusable = true}}, SHIFT(127),
  [556] = {.entry = {.count = 1, .reusable = true}}, SHIFT(175),
  [558] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 2, 0, 0),
  [560] = {.entry = {.count = 1, .reusable = true}}, SHIFT(119),
  [562] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 4, 0, 0),
  [564] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 4, 0, 0),
  [566] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [568] = {.entry = {.count = 1, .reusable = true}}, SHIFT(120),
  [570] = {.entry = {.count = 1, .reusable = true}}, SHIFT(181),
  [572] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_chart_params_repeat1, 2, 0, 0),
  [574] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_chart_params_repeat1, 2, 0, 0), SHIFT_REPEAT(58),
  [577] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 1, 0, 0),
  [579] = {.entry = {.count = 1, .reusable = true}}, SHIFT(135),
  [581] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_argument_list_repeat1, 2, 0, 0),
  [583] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_argument_list_repeat1, 2, 0, 0), SHIFT_REPEAT(172),
  [586] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite_body, 2, 0, 0),
  [588] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [590] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_param, 3, 0, 4),
  [592] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [594] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [596] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_language_tag, 1, 0, 0),
  [598] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [600] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument, 3, 0, 4),
  [602] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_directive, 6, 0, 7),
  [604] = {.entry = {.count = 1, .reusable = true}}, SHIFT(174),
  [606] = {.entry = {.count = 1, .reusable = true}}, SHIFT(112),
  [608] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_directive, 5, 0, 7),
  [610] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite, 3, 0, 1),
  [612] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite_body, 3, 0, 0),
  [614] = {.entry = {.count = 1, .reusable = true}}, SHIFT(85),
  [616] = {.entry = {.count = 1, .reusable = true}}, SHIFT(192),
  [618] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [620] = {.entry = {.count = 1, .reusable = true}}, SHIFT(199),
  [622] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [624] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_param, 3, 0, 6),
  [626] = {.entry = {.count = 1, .reusable = true}}, SHIFT(194),
  [628] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 2, 0, 0),
  [630] = {.entry = {.count = 1, .reusable = true}}, SHIFT(121),
  [632] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property_name, 1, 0, 0),
  [634] = {.entry = {.count = 1, .reusable = true}}, SHIFT(151),
  [636] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 4, 0, 0),
  [638] = {.entry = {.count = 1, .reusable = true}}, SHIFT(97),
  [640] = {.entry = {.count = 1, .reusable = true}}, SHIFT(123),
  [642] = {.entry = {.count = 1, .reusable = true}}, SHIFT(173),
  [644] = {.entry = {.count = 1, .reusable = true}}, SHIFT(139),
  [646] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [648] = {.entry = {.count = 1, .reusable = true}}, SHIFT(183),
  [650] = {.entry = {.count = 1, .reusable = true}}, SHIFT(124),
  [652] = {.entry = {.count = 1, .reusable = true}}, SHIFT(213),
  [654] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 5, 0, 0),
  [656] = {.entry = {.count = 1, .reusable = true}}, SHIFT(125),
  [658] = {.entry = {.count = 1, .reusable = true}}, SHIFT(148),
  [660] = {.entry = {.count = 1, .reusable = true}}, SHIFT(88),
  [662] = {.entry = {.count = 1, .reusable = true}}, SHIFT(182),
  [664] = {.entry = {.count = 1, .reusable = true}}, SHIFT(92),
  [666] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_param_name, 1, 0, 0),
  [668] = {.entry = {.count = 1, .reusable = true}}, SHIFT(162),
  [670] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [672] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [674] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [676] = {.entry = {.count = 1, .reusable = true}}, SHIFT(99),
  [678] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [680] = {.entry = {.count = 1, .reusable = true}}, SHIFT(186),
  [682] = {.entry = {.count = 1, .reusable = true}}, SHIFT(110),
  [684] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_args, 3, 0, 0),
  [686] = {.entry = {.count = 1, .reusable = true}}, SHIFT(159),
  [688] = {.entry = {.count = 1, .reusable = true}}, SHIFT(91),
  [690] = {.entry = {.count = 1, .reusable = true}}, SHIFT(130),
  [692] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [694] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [696] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_function_name, 1, 0, 0),
  [698] = {.entry = {.count = 1, .reusable = true}}, SHIFT(161),
  [700] = {.entry = {.count = 1, .reusable = true}}, SHIFT(136),
  [702] = {.entry = {.count = 1, .reusable = true}}, SHIFT(203),
  [704] = {.entry = {.count = 1, .reusable = true}}, SHIFT(109),
  [706] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [708] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [710] = {.entry = {.count = 1, .reusable = true}}, SHIFT(154),
  [712] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 3, 0, 0),
  [714] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [716] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
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
