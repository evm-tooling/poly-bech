#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 221
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 166
#define ALIAS_COUNT 0
#define TOKEN_COUNT 89
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
  anon_sym_go = 66,
  anon_sym_ts = 67,
  anon_sym_typescript = 68,
  anon_sym_rust = 69,
  anon_sym_python = 70,
  sym_inline_code = 71,
  anon_sym_DQUOTE = 72,
  anon_sym_SQUOTE = 73,
  aux_sym_string_content_token1 = 74,
  aux_sym_single_string_content_token1 = 75,
  sym_escape_sequence = 76,
  sym_number = 77,
  sym_float = 78,
  anon_sym_ms = 79,
  anon_sym_s = 80,
  anon_sym_m = 81,
  anon_sym_true = 82,
  anon_sym_false = 83,
  anon_sym_LBRACK = 84,
  anon_sym_RBRACK = 85,
  sym_comment = 86,
  sym_embedded_code = 87,
  sym__embedded_code_start = 88,
  sym_source_file = 89,
  sym_use_statement = 90,
  sym_global_setup = 91,
  sym_global_setup_body = 92,
  sym_global_setup_statement = 93,
  sym_anvil_call = 94,
  sym_anvil_args = 95,
  sym_function_call = 96,
  sym_argument_list = 97,
  sym_argument = 98,
  sym_suite = 99,
  sym_suite_body = 100,
  sym__suite_item = 101,
  sym_setup_block = 102,
  sym_setup_body = 103,
  sym__setup_section = 104,
  sym_import_section = 105,
  sym_declare_section = 106,
  sym_init_section = 107,
  sym_helpers_section = 108,
  sym_fixture = 109,
  sym_fixture_params = 110,
  sym_fixture_param = 111,
  sym_fixture_body = 112,
  sym__fixture_item = 113,
  sym_hex_property = 114,
  sym_shape_property = 115,
  sym_file_ref = 116,
  sym_benchmark = 117,
  sym_benchmark_body = 118,
  sym__benchmark_item = 119,
  sym_tags_property = 120,
  sym_skip_hook = 121,
  sym_validate_hook = 122,
  sym_before_hook = 123,
  sym_after_hook = 124,
  sym_each_hook = 125,
  sym_hook_flat = 126,
  sym_hook_grouped = 127,
  sym_after_block = 128,
  sym_after_body = 129,
  sym_chart_directive = 130,
  sym_chart_function_name = 131,
  sym_chart_params = 132,
  sym_chart_param = 133,
  sym_chart_param_name = 134,
  sym__chart_value = 135,
  sym_property = 136,
  sym_property_name = 137,
  sym__value = 138,
  sym_language_implementation = 139,
  sym_language_tag = 140,
  sym__code_or_inline = 141,
  sym_code_block = 142,
  sym_paren_code_block = 143,
  sym_string = 144,
  sym_string_content = 145,
  sym_single_string_content = 146,
  sym_duration = 147,
  sym_duration_unit = 148,
  sym_boolean = 149,
  sym_string_array = 150,
  aux_sym_source_file_repeat1 = 151,
  aux_sym_source_file_repeat2 = 152,
  aux_sym_global_setup_body_repeat1 = 153,
  aux_sym_argument_list_repeat1 = 154,
  aux_sym_suite_body_repeat1 = 155,
  aux_sym_setup_body_repeat1 = 156,
  aux_sym_fixture_params_repeat1 = 157,
  aux_sym_fixture_body_repeat1 = 158,
  aux_sym_benchmark_body_repeat1 = 159,
  aux_sym_hook_grouped_repeat1 = 160,
  aux_sym_after_body_repeat1 = 161,
  aux_sym_chart_params_repeat1 = 162,
  aux_sym_string_content_repeat1 = 163,
  aux_sym_single_string_content_repeat1 = 164,
  aux_sym_string_array_repeat1 = 165,
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
      if (lookahead == 'l') ADVANCE(84);
      END_STATE();
    case 33:
      if (lookahead == 'l') ADVANCE(85);
      if (lookahead == 'x') ADVANCE(86);
      END_STATE();
    case 34:
      if (lookahead == 'r') ADVANCE(87);
      END_STATE();
    case 35:
      if (lookahead == 'o') ADVANCE(88);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(anon_sym_go);
      END_STATE();
    case 37:
      if (lookahead == 'i') ADVANCE(89);
      if (lookahead == 'l') ADVANCE(90);
      if (lookahead == 'x') ADVANCE(91);
      END_STATE();
    case 38:
      if (lookahead == 'p') ADVANCE(92);
      END_STATE();
    case 39:
      if (lookahead == 'c') ADVANCE(93);
      if (lookahead == 'i') ADVANCE(94);
      END_STATE();
    case 40:
      if (lookahead == 'e') ADVANCE(95);
      END_STATE();
    case 41:
      if (lookahead == 'm') ADVANCE(96);
      END_STATE();
    case 42:
      if (lookahead == 'm') ADVANCE(97);
      END_STATE();
    case 43:
      if (lookahead == 'n') ADVANCE(98);
      END_STATE();
    case 44:
      if (lookahead == 'd') ADVANCE(99);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(anon_sym_ms);
      END_STATE();
    case 46:
      if (lookahead == 'd') ADVANCE(100);
      END_STATE();
    case 47:
      if (lookahead == 't') ADVANCE(101);
      END_STATE();
    case 48:
      if (lookahead == 't') ADVANCE(102);
      END_STATE();
    case 49:
      if (lookahead == 'q') ADVANCE(103);
      END_STATE();
    case 50:
      if (lookahead == 'w') ADVANCE(104);
      END_STATE();
    case 51:
      if (lookahead == 's') ADVANCE(105);
      END_STATE();
    case 52:
      if (lookahead == 't') ADVANCE(106);
      END_STATE();
    case 53:
      if (lookahead == 'a') ADVANCE(107);
      END_STATE();
    case 54:
      if (lookahead == 'n') ADVANCE(108);
      END_STATE();
    case 55:
      if (lookahead == 'i') ADVANCE(109);
      END_STATE();
    case 56:
      if (lookahead == 'r') ADVANCE(110);
      END_STATE();
    case 57:
      if (lookahead == 'a') ADVANCE(111);
      END_STATE();
    case 58:
      if (lookahead == 'd') ADVANCE(112);
      END_STATE();
    case 59:
      if (lookahead == 'i') ADVANCE(113);
      END_STATE();
    case 60:
      if (lookahead == 'g') ADVANCE(114);
      if (lookahead == 'r') ADVANCE(115);
      END_STATE();
    case 61:
      if (lookahead == 'e') ADVANCE(116);
      END_STATE();
    case 62:
      if (lookahead == 'm') ADVANCE(117);
      if (lookahead == 't') ADVANCE(118);
      END_STATE();
    case 63:
      if (lookahead == 'u') ADVANCE(119);
      END_STATE();
    case 64:
      ACCEPT_TOKEN(anon_sym_ts);
      END_STATE();
    case 65:
      if (lookahead == 'p') ADVANCE(120);
      END_STATE();
    case 66:
      if (lookahead == 'e') ADVANCE(121);
      END_STATE();
    case 67:
      if (lookahead == 'l') ADVANCE(122);
      END_STATE();
    case 68:
      if (lookahead == 'r') ADVANCE(123);
      END_STATE();
    case 69:
      if (lookahead == 'd') ADVANCE(124);
      END_STATE();
    case 70:
      if (lookahead == 'e') ADVANCE(125);
      END_STATE();
    case 71:
      if (lookahead == 'i') ADVANCE(126);
      END_STATE();
    case 72:
      if (lookahead == 'n') ADVANCE(127);
      END_STATE();
    case 73:
      if (lookahead == 'e') ADVANCE(128);
      END_STATE();
    case 74:
      if (lookahead == 'o') ADVANCE(129);
      END_STATE();
    case 75:
      if (lookahead == 'c') ADVANCE(130);
      END_STATE();
    case 76:
      if (lookahead == 'r') ADVANCE(131);
      END_STATE();
    case 77:
      if (lookahead == 'n') ADVANCE(132);
      END_STATE();
    case 78:
      if (lookahead == 'h') ADVANCE(133);
      END_STATE();
    case 79:
      if (lookahead == 'l') ADVANCE(134);
      END_STATE();
    case 80:
      if (lookahead == 'c') ADVANCE(135);
      END_STATE();
    case 81:
      if (lookahead == 'w') ADVANCE(136);
      END_STATE();
    case 82:
      if (lookahead == 'h') ADVANCE(137);
      END_STATE();
    case 83:
      if (lookahead == 'l') ADVANCE(138);
      END_STATE();
    case 84:
      if (lookahead == 's') ADVANCE(139);
      END_STATE();
    case 85:
      if (lookahead == 't') ADVANCE(140);
      END_STATE();
    case 86:
      if (lookahead == 't') ADVANCE(141);
      END_STATE();
    case 87:
      if (lookahead == 'k') ADVANCE(142);
      END_STATE();
    case 88:
      if (lookahead == 'b') ADVANCE(143);
      END_STATE();
    case 89:
      if (lookahead == 'g') ADVANCE(144);
      END_STATE();
    case 90:
      if (lookahead == 'p') ADVANCE(145);
      END_STATE();
    case 91:
      ACCEPT_TOKEN(anon_sym_hex);
      END_STATE();
    case 92:
      if (lookahead == 'o') ADVANCE(146);
      END_STATE();
    case 93:
      if (lookahead == 'l') ADVANCE(147);
      END_STATE();
    case 94:
      if (lookahead == 't') ADVANCE(148);
      END_STATE();
    case 95:
      if (lookahead == 'r') ADVANCE(149);
      END_STATE();
    case 96:
      if (lookahead == 'i') ADVANCE(150);
      END_STATE();
    case 97:
      if (lookahead == 'o') ADVANCE(151);
      END_STATE();
    case 98:
      if (lookahead == 'S') ADVANCE(152);
      END_STATE();
    case 99:
      if (lookahead == 'e') ADVANCE(153);
      END_STATE();
    case 100:
      if (lookahead == 'e') ADVANCE(154);
      END_STATE();
    case 101:
      if (lookahead == 'l') ADVANCE(155);
      if (lookahead == 'p') ADVANCE(156);
      END_STATE();
    case 102:
      if (lookahead == 'h') ADVANCE(157);
      END_STATE();
    case 103:
      if (lookahead == 'u') ADVANCE(158);
      END_STATE();
    case 104:
      if (lookahead == 'C') ADVANCE(159);
      END_STATE();
    case 105:
      if (lookahead == 't') ADVANCE(160);
      END_STATE();
    case 106:
      if (lookahead == 'u') ADVANCE(161);
      END_STATE();
    case 107:
      if (lookahead == 'p') ADVANCE(162);
      END_STATE();
    case 108:
      if (lookahead == 'k') ADVANCE(163);
      END_STATE();
    case 109:
      if (lookahead == 'p') ADVANCE(164);
      END_STATE();
    case 110:
      if (lookahead == 't') ADVANCE(165);
      END_STATE();
    case 111:
      if (lookahead == 'w') ADVANCE(166);
      END_STATE();
    case 112:
      ACCEPT_TOKEN(anon_sym_std);
      END_STATE();
    case 113:
      if (lookahead == 't') ADVANCE(167);
      END_STATE();
    case 114:
      if (lookahead == 's') ADVANCE(168);
      END_STATE();
    case 115:
      if (lookahead == 'g') ADVANCE(169);
      END_STATE();
    case 116:
      if (lookahead == 'm') ADVANCE(170);
      END_STATE();
    case 117:
      if (lookahead == 'e') ADVANCE(171);
      END_STATE();
    case 118:
      if (lookahead == 'l') ADVANCE(172);
      END_STATE();
    case 119:
      if (lookahead == 'e') ADVANCE(173);
      END_STATE();
    case 120:
      if (lookahead == 'e') ADVANCE(174);
      END_STATE();
    case 121:
      ACCEPT_TOKEN(anon_sym_use);
      END_STATE();
    case 122:
      if (lookahead == 'i') ADVANCE(175);
      END_STATE();
    case 123:
      if (lookahead == 'm') ADVANCE(176);
      END_STATE();
    case 124:
      if (lookahead == 't') ADVANCE(177);
      END_STATE();
    case 125:
      if (lookahead == 'r') ADVANCE(178);
      END_STATE();
    case 126:
      if (lookahead == 'l') ADVANCE(179);
      END_STATE();
    case 127:
      if (lookahead == 'c') ADVANCE(180);
      END_STATE();
    case 128:
      if (lookahead == 'l') ADVANCE(181);
      END_STATE();
    case 129:
      if (lookahead == 'r') ADVANCE(182);
      END_STATE();
    case 130:
      if (lookahead == 'h') ADVANCE(183);
      END_STATE();
    case 131:
      if (lookahead == 't') ADVANCE(184);
      END_STATE();
    case 132:
      if (lookahead == 't') ADVANCE(185);
      END_STATE();
    case 133:
      if (lookahead == 'r') ADVANCE(186);
      END_STATE();
    case 134:
      if (lookahead == 'a') ADVANCE(187);
      END_STATE();
    case 135:
      if (lookahead == 'r') ADVANCE(188);
      END_STATE();
    case 136:
      if (lookahead == 'S') ADVANCE(189);
      if (lookahead == 'T') ADVANCE(190);
      END_STATE();
    case 137:
      ACCEPT_TOKEN(anon_sym_each);
      END_STATE();
    case 138:
      if (lookahead == 'u') ADVANCE(191);
      END_STATE();
    case 139:
      if (lookahead == 'e') ADVANCE(192);
      END_STATE();
    case 140:
      if (lookahead == 'e') ADVANCE(193);
      END_STATE();
    case 141:
      if (lookahead == 'u') ADVANCE(194);
      END_STATE();
    case 142:
      ACCEPT_TOKEN(anon_sym_fork);
      END_STATE();
    case 143:
      if (lookahead == 'a') ADVANCE(195);
      END_STATE();
    case 144:
      if (lookahead == 'h') ADVANCE(196);
      END_STATE();
    case 145:
      if (lookahead == 'e') ADVANCE(197);
      END_STATE();
    case 146:
      if (lookahead == 'r') ADVANCE(198);
      END_STATE();
    case 147:
      if (lookahead == 'u') ADVANCE(199);
      END_STATE();
    case 148:
      ACCEPT_TOKEN(anon_sym_init);
      END_STATE();
    case 149:
      if (lookahead == 'a') ADVANCE(200);
      END_STATE();
    case 150:
      if (lookahead == 't') ADVANCE(201);
      END_STATE();
    case 151:
      if (lookahead == 'r') ADVANCE(202);
      END_STATE();
    case 152:
      if (lookahead == 'p') ADVANCE(203);
      END_STATE();
    case 153:
      ACCEPT_TOKEN(anon_sym_mode);
      END_STATE();
    case 154:
      if (lookahead == 'r') ADVANCE(204);
      END_STATE();
    case 155:
      if (lookahead == 'i') ADVANCE(205);
      END_STATE();
    case 156:
      if (lookahead == 'u') ADVANCE(206);
      END_STATE();
    case 157:
      if (lookahead == 'o') ADVANCE(207);
      END_STATE();
    case 158:
      if (lookahead == 'i') ADVANCE(208);
      END_STATE();
    case 159:
      if (lookahead == 'o') ADVANCE(209);
      END_STATE();
    case 160:
      ACCEPT_TOKEN(anon_sym_rust);
      END_STATE();
    case 161:
      if (lookahead == 'p') ADVANCE(210);
      END_STATE();
    case 162:
      if (lookahead == 'e') ADVANCE(211);
      END_STATE();
    case 163:
      ACCEPT_TOKEN(anon_sym_sink);
      END_STATE();
    case 164:
      ACCEPT_TOKEN(anon_sym_skip);
      END_STATE();
    case 165:
      if (lookahead == 'B') ADVANCE(212);
      if (lookahead == 'O') ADVANCE(213);
      END_STATE();
    case 166:
      if (lookahead == 'n') ADVANCE(214);
      END_STATE();
    case 167:
      if (lookahead == 'e') ADVANCE(215);
      END_STATE();
    case 168:
      ACCEPT_TOKEN(anon_sym_tags);
      END_STATE();
    case 169:
      if (lookahead == 'e') ADVANCE(216);
      END_STATE();
    case 170:
      if (lookahead == 'e') ADVANCE(217);
      END_STATE();
    case 171:
      if (lookahead == 'o') ADVANCE(218);
      END_STATE();
    case 172:
      if (lookahead == 'e') ADVANCE(219);
      END_STATE();
    case 173:
      ACCEPT_TOKEN(anon_sym_true);
      END_STATE();
    case 174:
      if (lookahead == 's') ADVANCE(220);
      END_STATE();
    case 175:
      if (lookahead == 'd') ADVANCE(221);
      END_STATE();
    case 176:
      if (lookahead == 'u') ADVANCE(222);
      END_STATE();
    case 177:
      if (lookahead == 'h') ADVANCE(223);
      END_STATE();
    case 178:
      ACCEPT_TOKEN(anon_sym_after);
      END_STATE();
    case 179:
      ACCEPT_TOKEN(anon_sym_anvil);
      END_STATE();
    case 180:
      ACCEPT_TOKEN(anon_sym_async);
      END_STATE();
    case 181:
      if (lookahead == 'i') ADVANCE(224);
      END_STATE();
    case 182:
      if (lookahead == 'e') ADVANCE(225);
      END_STATE();
    case 183:
      ACCEPT_TOKEN(anon_sym_bench);
      if (lookahead == 'A') ADVANCE(226);
      END_STATE();
    case 184:
      if (lookahead == 'i') ADVANCE(227);
      END_STATE();
    case 185:
      ACCEPT_TOKEN(anon_sym_count);
      END_STATE();
    case 186:
      if (lookahead == 'e') ADVANCE(228);
      END_STATE();
    case 187:
      if (lookahead == 'r') ADVANCE(229);
      END_STATE();
    case 188:
      if (lookahead == 'i') ADVANCE(230);
      END_STATE();
    case 189:
      if (lookahead == 'p') ADVANCE(231);
      END_STATE();
    case 190:
      if (lookahead == 'a') ADVANCE(232);
      END_STATE();
    case 191:
      if (lookahead == 'd') ADVANCE(233);
      END_STATE();
    case 192:
      ACCEPT_TOKEN(anon_sym_false);
      END_STATE();
    case 193:
      if (lookahead == 'r') ADVANCE(234);
      END_STATE();
    case 194:
      if (lookahead == 'r') ADVANCE(235);
      END_STATE();
    case 195:
      if (lookahead == 'l') ADVANCE(236);
      END_STATE();
    case 196:
      if (lookahead == 't') ADVANCE(237);
      END_STATE();
    case 197:
      if (lookahead == 'r') ADVANCE(238);
      END_STATE();
    case 198:
      if (lookahead == 't') ADVANCE(239);
      END_STATE();
    case 199:
      if (lookahead == 'd') ADVANCE(240);
      END_STATE();
    case 200:
      if (lookahead == 't') ADVANCE(241);
      END_STATE();
    case 201:
      ACCEPT_TOKEN(anon_sym_limit);
      END_STATE();
    case 202:
      if (lookahead == 'y') ADVANCE(242);
      END_STATE();
    case 203:
      if (lookahead == 'e') ADVANCE(243);
      END_STATE();
    case 204:
      ACCEPT_TOKEN(anon_sym_order);
      END_STATE();
    case 205:
      if (lookahead == 'e') ADVANCE(244);
      END_STATE();
    case 206:
      if (lookahead == 't') ADVANCE(245);
      END_STATE();
    case 207:
      if (lookahead == 'n') ADVANCE(246);
      END_STATE();
    case 208:
      if (lookahead == 'r') ADVANCE(247);
      END_STATE();
    case 209:
      if (lookahead == 'u') ADVANCE(248);
      END_STATE();
    case 210:
      ACCEPT_TOKEN(anon_sym_setup);
      END_STATE();
    case 211:
      ACCEPT_TOKEN(anon_sym_shape);
      END_STATE();
    case 212:
      if (lookahead == 'y') ADVANCE(249);
      END_STATE();
    case 213:
      if (lookahead == 'r') ADVANCE(250);
      END_STATE();
    case 214:
      if (lookahead == 'A') ADVANCE(251);
      END_STATE();
    case 215:
      ACCEPT_TOKEN(anon_sym_suite);
      END_STATE();
    case 216:
      if (lookahead == 't') ADVANCE(252);
      END_STATE();
    case 217:
      ACCEPT_TOKEN(anon_sym_theme);
      END_STATE();
    case 218:
      if (lookahead == 'u') ADVANCE(253);
      END_STATE();
    case 219:
      ACCEPT_TOKEN(anon_sym_title);
      END_STATE();
    case 220:
      if (lookahead == 'c') ADVANCE(254);
      END_STATE();
    case 221:
      if (lookahead == 'a') ADVANCE(255);
      END_STATE();
    case 222:
      if (lookahead == 'p') ADVANCE(256);
      END_STATE();
    case 223:
      ACCEPT_TOKEN(anon_sym_width);
      END_STATE();
    case 224:
      if (lookahead == 'n') ADVANCE(257);
      END_STATE();
    case 225:
      ACCEPT_TOKEN(anon_sym_before);
      END_STATE();
    case 226:
      if (lookahead == 's') ADVANCE(258);
      END_STATE();
    case 227:
      if (lookahead == 'n') ADVANCE(259);
      END_STATE();
    case 228:
      if (lookahead == 's') ADVANCE(260);
      END_STATE();
    case 229:
      if (lookahead == 'e') ADVANCE(261);
      END_STATE();
    case 230:
      if (lookahead == 'p') ADVANCE(262);
      END_STATE();
    case 231:
      if (lookahead == 'e') ADVANCE(263);
      END_STATE();
    case 232:
      if (lookahead == 'b') ADVANCE(264);
      END_STATE();
    case 233:
      if (lookahead == 'e') ADVANCE(265);
      END_STATE();
    case 234:
      if (lookahead == 'W') ADVANCE(266);
      END_STATE();
    case 235:
      if (lookahead == 'e') ADVANCE(267);
      END_STATE();
    case 236:
      if (lookahead == 'S') ADVANCE(268);
      END_STATE();
    case 237:
      ACCEPT_TOKEN(anon_sym_height);
      END_STATE();
    case 238:
      if (lookahead == 's') ADVANCE(269);
      END_STATE();
    case 239:
      ACCEPT_TOKEN(anon_sym_import);
      END_STATE();
    case 240:
      if (lookahead == 'e') ADVANCE(270);
      END_STATE();
    case 241:
      if (lookahead == 'i') ADVANCE(271);
      END_STATE();
    case 242:
      ACCEPT_TOKEN(anon_sym_memory);
      END_STATE();
    case 243:
      if (lookahead == 'e') ADVANCE(272);
      END_STATE();
    case 244:
      if (lookahead == 'r') ADVANCE(273);
      END_STATE();
    case 245:
      ACCEPT_TOKEN(anon_sym_output);
      END_STATE();
    case 246:
      ACCEPT_TOKEN(anon_sym_python);
      END_STATE();
    case 247:
      if (lookahead == 'e') ADVANCE(274);
      END_STATE();
    case 248:
      if (lookahead == 'n') ADVANCE(275);
      END_STATE();
    case 249:
      ACCEPT_TOKEN(anon_sym_sortBy);
      END_STATE();
    case 250:
      if (lookahead == 'd') ADVANCE(276);
      END_STATE();
    case 251:
      if (lookahead == 'n') ADVANCE(277);
      END_STATE();
    case 252:
      if (lookahead == 'T') ADVANCE(278);
      END_STATE();
    case 253:
      if (lookahead == 't') ADVANCE(279);
      END_STATE();
    case 254:
      if (lookahead == 'r') ADVANCE(280);
      END_STATE();
    case 255:
      if (lookahead == 't') ADVANCE(281);
      END_STATE();
    case 256:
      ACCEPT_TOKEN(anon_sym_warmup);
      END_STATE();
    case 257:
      if (lookahead == 'e') ADVANCE(282);
      END_STATE();
    case 258:
      if (lookahead == 'y') ADVANCE(283);
      END_STATE();
    case 259:
      if (lookahead == 'g') ADVANCE(284);
      END_STATE();
    case 260:
      if (lookahead == 'h') ADVANCE(285);
      END_STATE();
    case 261:
      ACCEPT_TOKEN(anon_sym_declare);
      END_STATE();
    case 262:
      if (lookahead == 't') ADVANCE(286);
      END_STATE();
    case 263:
      if (lookahead == 'e') ADVANCE(287);
      END_STATE();
    case 264:
      if (lookahead == 'l') ADVANCE(288);
      END_STATE();
    case 265:
      if (lookahead == 'B') ADVANCE(289);
      END_STATE();
    case 266:
      if (lookahead == 'i') ADVANCE(290);
      END_STATE();
    case 267:
      ACCEPT_TOKEN(anon_sym_fixture);
      END_STATE();
    case 268:
      if (lookahead == 'e') ADVANCE(291);
      END_STATE();
    case 269:
      ACCEPT_TOKEN(anon_sym_helpers);
      END_STATE();
    case 270:
      if (lookahead == 'B') ADVANCE(292);
      END_STATE();
    case 271:
      if (lookahead == 'o') ADVANCE(293);
      END_STATE();
    case 272:
      if (lookahead == 'd') ADVANCE(294);
      END_STATE();
    case 273:
      if (lookahead == 'D') ADVANCE(295);
      END_STATE();
    case 274:
      if (lookahead == 's') ADVANCE(296);
      END_STATE();
    case 275:
      if (lookahead == 't') ADVANCE(297);
      END_STATE();
    case 276:
      if (lookahead == 'e') ADVANCE(298);
      END_STATE();
    case 277:
      if (lookahead == 'v') ADVANCE(299);
      END_STATE();
    case 278:
      if (lookahead == 'i') ADVANCE(300);
      END_STATE();
    case 279:
      ACCEPT_TOKEN(anon_sym_timeout);
      END_STATE();
    case 280:
      if (lookahead == 'i') ADVANCE(301);
      END_STATE();
    case 281:
      if (lookahead == 'e') ADVANCE(302);
      END_STATE();
    case 282:
      ACCEPT_TOKEN(anon_sym_baseline);
      if (lookahead == 'B') ADVANCE(303);
      END_STATE();
    case 283:
      if (lookahead == 'n') ADVANCE(304);
      END_STATE();
    case 284:
      ACCEPT_TOKEN(anon_sym_charting);
      END_STATE();
    case 285:
      if (lookahead == 'o') ADVANCE(305);
      END_STATE();
    case 286:
      if (lookahead == 'i') ADVANCE(306);
      END_STATE();
    case 287:
      if (lookahead == 'd') ADVANCE(307);
      END_STATE();
    case 288:
      if (lookahead == 'e') ADVANCE(308);
      END_STATE();
    case 289:
      if (lookahead == 'e') ADVANCE(309);
      END_STATE();
    case 290:
      if (lookahead == 'n') ADVANCE(310);
      END_STATE();
    case 291:
      if (lookahead == 't') ADVANCE(311);
      END_STATE();
    case 292:
      if (lookahead == 'e') ADVANCE(312);
      END_STATE();
    case 293:
      if (lookahead == 'n') ADVANCE(313);
      END_STATE();
    case 294:
      if (lookahead == 'u') ADVANCE(314);
      END_STATE();
    case 295:
      if (lookahead == 'e') ADVANCE(315);
      END_STATE();
    case 296:
      ACCEPT_TOKEN(anon_sym_requires);
      END_STATE();
    case 297:
      ACCEPT_TOKEN(anon_sym_rowCount);
      END_STATE();
    case 298:
      if (lookahead == 'r') ADVANCE(316);
      END_STATE();
    case 299:
      if (lookahead == 'i') ADVANCE(317);
      END_STATE();
    case 300:
      if (lookahead == 'm') ADVANCE(318);
      END_STATE();
    case 301:
      if (lookahead == 'p') ADVANCE(319);
      END_STATE();
    case 302:
      ACCEPT_TOKEN(anon_sym_validate);
      END_STATE();
    case 303:
      if (lookahead == 'e') ADVANCE(320);
      END_STATE();
    case 304:
      if (lookahead == 'c') ADVANCE(321);
      END_STATE();
    case 305:
      if (lookahead == 'l') ADVANCE(322);
      END_STATE();
    case 306:
      if (lookahead == 'o') ADVANCE(323);
      END_STATE();
    case 307:
      if (lookahead == 'u') ADVANCE(324);
      END_STATE();
    case 308:
      ACCEPT_TOKEN(anon_sym_drawTable);
      END_STATE();
    case 309:
      if (lookahead == 'n') ADVANCE(325);
      END_STATE();
    case 310:
      if (lookahead == 'n') ADVANCE(326);
      END_STATE();
    case 311:
      if (lookahead == 'u') ADVANCE(327);
      END_STATE();
    case 312:
      if (lookahead == 'n') ADVANCE(328);
      END_STATE();
    case 313:
      if (lookahead == 's') ADVANCE(329);
      END_STATE();
    case 314:
      if (lookahead == 'p') ADVANCE(330);
      END_STATE();
    case 315:
      if (lookahead == 't') ADVANCE(331);
      END_STATE();
    case 316:
      ACCEPT_TOKEN(anon_sym_sortOrder);
      END_STATE();
    case 317:
      if (lookahead == 'l') ADVANCE(332);
      END_STATE();
    case 318:
      if (lookahead == 'e') ADVANCE(333);
      END_STATE();
    case 319:
      if (lookahead == 't') ADVANCE(334);
      END_STATE();
    case 320:
      if (lookahead == 'n') ADVANCE(335);
      END_STATE();
    case 321:
      ACCEPT_TOKEN(anon_sym_benchAsync);
      END_STATE();
    case 322:
      if (lookahead == 'd') ADVANCE(336);
      END_STATE();
    case 323:
      if (lookahead == 'n') ADVANCE(337);
      END_STATE();
    case 324:
      if (lookahead == 'p') ADVANCE(338);
      END_STATE();
    case 325:
      if (lookahead == 'c') ADVANCE(339);
      END_STATE();
    case 326:
      if (lookahead == 'e') ADVANCE(340);
      END_STATE();
    case 327:
      if (lookahead == 'p') ADVANCE(341);
      END_STATE();
    case 328:
      if (lookahead == 'c') ADVANCE(342);
      END_STATE();
    case 329:
      ACCEPT_TOKEN(anon_sym_iterations);
      END_STATE();
    case 330:
      ACCEPT_TOKEN(anon_sym_minSpeedup);
      END_STATE();
    case 331:
      if (lookahead == 'e') ADVANCE(343);
      END_STATE();
    case 332:
      ACCEPT_TOKEN(anon_sym_spawnAnvil);
      END_STATE();
    case 333:
      ACCEPT_TOKEN(anon_sym_targetTime);
      END_STATE();
    case 334:
      ACCEPT_TOKEN(anon_sym_typescript);
      END_STATE();
    case 335:
      if (lookahead == 'c') ADVANCE(344);
      END_STATE();
    case 336:
      ACCEPT_TOKEN(anon_sym_cvThreshold);
      END_STATE();
    case 337:
      ACCEPT_TOKEN(anon_sym_description);
      END_STATE();
    case 338:
      if (lookahead == 'C') ADVANCE(345);
      END_STATE();
    case 339:
      if (lookahead == 'h') ADVANCE(346);
      END_STATE();
    case 340:
      if (lookahead == 'r') ADVANCE(347);
      END_STATE();
    case 341:
      ACCEPT_TOKEN(anon_sym_globalSetup);
      END_STATE();
    case 342:
      if (lookahead == 'h') ADVANCE(348);
      END_STATE();
    case 343:
      if (lookahead == 'c') ADVANCE(349);
      END_STATE();
    case 344:
      if (lookahead == 'h') ADVANCE(350);
      END_STATE();
    case 345:
      if (lookahead == 'h') ADVANCE(351);
      END_STATE();
    case 346:
      if (lookahead == 'm') ADVANCE(352);
      END_STATE();
    case 347:
      ACCEPT_TOKEN(anon_sym_filterWinner);
      END_STATE();
    case 348:
      if (lookahead == 'm') ADVANCE(353);
      END_STATE();
    case 349:
      if (lookahead == 't') ADVANCE(354);
      END_STATE();
    case 350:
      if (lookahead == 'm') ADVANCE(355);
      END_STATE();
    case 351:
      if (lookahead == 'a') ADVANCE(356);
      END_STATE();
    case 352:
      if (lookahead == 'a') ADVANCE(357);
      END_STATE();
    case 353:
      if (lookahead == 'a') ADVANCE(358);
      END_STATE();
    case 354:
      if (lookahead == 'i') ADVANCE(359);
      END_STATE();
    case 355:
      if (lookahead == 'a') ADVANCE(360);
      END_STATE();
    case 356:
      if (lookahead == 'r') ADVANCE(361);
      END_STATE();
    case 357:
      if (lookahead == 'r') ADVANCE(362);
      END_STATE();
    case 358:
      if (lookahead == 'r') ADVANCE(363);
      END_STATE();
    case 359:
      if (lookahead == 'o') ADVANCE(364);
      END_STATE();
    case 360:
      if (lookahead == 'r') ADVANCE(365);
      END_STATE();
    case 361:
      if (lookahead == 't') ADVANCE(366);
      END_STATE();
    case 362:
      if (lookahead == 'k') ADVANCE(367);
      END_STATE();
    case 363:
      if (lookahead == 'k') ADVANCE(368);
      END_STATE();
    case 364:
      if (lookahead == 'n') ADVANCE(369);
      END_STATE();
    case 365:
      if (lookahead == 'k') ADVANCE(370);
      END_STATE();
    case 366:
      ACCEPT_TOKEN(anon_sym_drawSpeedupChart);
      END_STATE();
    case 367:
      if (lookahead == 's') ADVANCE(371);
      END_STATE();
    case 368:
      if (lookahead == 's') ADVANCE(372);
      END_STATE();
    case 369:
      ACCEPT_TOKEN(anon_sym_outlierDetection);
      END_STATE();
    case 370:
      ACCEPT_TOKEN(anon_sym_baselineBenchmark);
      END_STATE();
    case 371:
      ACCEPT_TOKEN(anon_sym_excludeBenchmarks);
      END_STATE();
    case 372:
      ACCEPT_TOKEN(anon_sym_includeBenchmarks);
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
  [0] = 13,
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
    STATE(200), 1,
      sym_language_tag,
    STATE(220), 1,
      sym_property_name,
    ACTIONS(36), 5,
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
    ACTIONS(33), 14,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
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
  [66] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(39), 1,
      anon_sym_RBRACE,
    ACTIONS(41), 1,
      anon_sym_tags,
    ACTIONS(43), 1,
      anon_sym_skip,
    ACTIONS(45), 1,
      anon_sym_validate,
    ACTIONS(47), 1,
      anon_sym_before,
    ACTIONS(49), 1,
      anon_sym_after,
    ACTIONS(51), 1,
      anon_sym_each,
    STATE(200), 1,
      sym_language_tag,
    STATE(220), 1,
      sym_property_name,
    ACTIONS(55), 5,
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
    ACTIONS(53), 14,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
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
  [132] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(41), 1,
      anon_sym_tags,
    ACTIONS(43), 1,
      anon_sym_skip,
    ACTIONS(45), 1,
      anon_sym_validate,
    ACTIONS(47), 1,
      anon_sym_before,
    ACTIONS(49), 1,
      anon_sym_after,
    ACTIONS(51), 1,
      anon_sym_each,
    ACTIONS(57), 1,
      anon_sym_RBRACE,
    STATE(200), 1,
      sym_language_tag,
    STATE(220), 1,
      sym_property_name,
    ACTIONS(55), 5,
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
    ACTIONS(53), 14,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
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
  [198] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(61), 1,
      anon_sym_bench,
    ACTIONS(59), 35,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
      anon_sym_RBRACK,
  [242] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(65), 1,
      anon_sym_bench,
    ACTIONS(63), 35,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
      anon_sym_RBRACK,
  [286] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(69), 1,
      anon_sym_bench,
    ACTIONS(67), 34,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [329] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(73), 1,
      anon_sym_bench,
    ACTIONS(71), 34,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [372] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(77), 1,
      anon_sym_bench,
    ACTIONS(75), 34,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [415] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(81), 1,
      anon_sym_bench,
    ACTIONS(79), 34,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [458] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(85), 1,
      anon_sym_bench,
    ACTIONS(83), 34,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [501] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(89), 1,
      anon_sym_bench,
    ACTIONS(87), 34,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [544] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(93), 1,
      anon_sym_bench,
    ACTIONS(91), 34,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [587] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(95), 33,
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
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [626] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(99), 1,
      anon_sym_bench,
    ACTIONS(97), 32,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [667] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(101), 33,
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
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [706] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(103), 1,
      anon_sym_RBRACE,
    ACTIONS(105), 1,
      anon_sym_hex,
    ACTIONS(107), 1,
      anon_sym_shape,
    STATE(200), 1,
      sym_language_tag,
    STATE(219), 1,
      sym_property_name,
    ACTIONS(55), 5,
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
    ACTIONS(53), 14,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
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
  [756] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(109), 1,
      anon_sym_globalSetup,
    ACTIONS(112), 1,
      anon_sym_RBRACE,
    ACTIONS(114), 1,
      anon_sym_setup,
    ACTIONS(117), 1,
      anon_sym_fixture,
    ACTIONS(120), 1,
      anon_sym_bench,
    ACTIONS(123), 1,
      anon_sym_benchAsync,
    ACTIONS(126), 1,
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
    ACTIONS(129), 14,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
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
  [810] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(132), 1,
      anon_sym_RBRACE,
    ACTIONS(134), 1,
      anon_sym_hex,
    ACTIONS(137), 1,
      anon_sym_shape,
    STATE(200), 1,
      sym_language_tag,
    STATE(219), 1,
      sym_property_name,
    ACTIONS(143), 5,
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
    ACTIONS(140), 14,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
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
  [860] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(105), 1,
      anon_sym_hex,
    ACTIONS(107), 1,
      anon_sym_shape,
    ACTIONS(146), 1,
      anon_sym_RBRACE,
    STATE(200), 1,
      sym_language_tag,
    STATE(219), 1,
      sym_property_name,
    ACTIONS(55), 5,
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
    ACTIONS(53), 14,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
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
  [910] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      anon_sym_globalSetup,
    ACTIONS(148), 1,
      anon_sym_RBRACE,
    ACTIONS(150), 1,
      anon_sym_setup,
    ACTIONS(152), 1,
      anon_sym_fixture,
    ACTIONS(154), 1,
      anon_sym_bench,
    ACTIONS(156), 1,
      anon_sym_benchAsync,
    ACTIONS(158), 1,
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
    ACTIONS(53), 14,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
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
  [964] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      anon_sym_globalSetup,
    ACTIONS(150), 1,
      anon_sym_setup,
    ACTIONS(152), 1,
      anon_sym_fixture,
    ACTIONS(154), 1,
      anon_sym_bench,
    ACTIONS(156), 1,
      anon_sym_benchAsync,
    ACTIONS(158), 1,
      anon_sym_after,
    ACTIONS(160), 1,
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
    ACTIONS(53), 14,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
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
  [1018] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(164), 1,
      anon_sym_ms,
    STATE(8), 1,
      sym_duration_unit,
    ACTIONS(166), 2,
      anon_sym_s,
      anon_sym_m,
    ACTIONS(162), 26,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1060] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(168), 28,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1094] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(170), 26,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1126] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(172), 26,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1158] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(174), 26,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1190] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(176), 26,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1222] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(164), 1,
      anon_sym_ms,
    STATE(8), 1,
      sym_duration_unit,
    ACTIONS(166), 2,
      anon_sym_s,
      anon_sym_m,
    ACTIONS(162), 22,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1260] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(178), 26,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1292] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(180), 26,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1324] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(182), 26,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1356] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(184), 26,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1388] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(186), 26,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1420] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(164), 1,
      anon_sym_ms,
    ACTIONS(188), 1,
      anon_sym_bench,
    STATE(8), 1,
      sym_duration_unit,
    ACTIONS(166), 2,
      anon_sym_s,
      anon_sym_m,
    ACTIONS(162), 20,
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
  [1459] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(192), 1,
      anon_sym_bench,
    ACTIONS(190), 22,
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
  [1490] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(196), 1,
      anon_sym_bench,
    ACTIONS(194), 22,
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
  [1521] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(200), 1,
      anon_sym_bench,
    ACTIONS(198), 22,
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
  [1552] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(202), 22,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1580] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(204), 22,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1608] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(206), 22,
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
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [1636] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(210), 1,
      anon_sym_bench,
    ACTIONS(208), 20,
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
  [1665] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(214), 1,
      anon_sym_bench,
    ACTIONS(212), 20,
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
  [1694] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(218), 1,
      anon_sym_bench,
    ACTIONS(216), 20,
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
  [1723] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(222), 1,
      anon_sym_bench,
    ACTIONS(220), 20,
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
  [1752] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(226), 1,
      anon_sym_bench,
    ACTIONS(224), 20,
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
  [1781] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(230), 1,
      anon_sym_bench,
    ACTIONS(228), 20,
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
  [1810] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(234), 1,
      anon_sym_bench,
    ACTIONS(232), 20,
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
  [1839] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(238), 1,
      anon_sym_bench,
    ACTIONS(236), 20,
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
  [1868] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(242), 1,
      anon_sym_bench,
    ACTIONS(240), 20,
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
  [1897] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(246), 1,
      anon_sym_bench,
    ACTIONS(244), 20,
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
  [1926] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(250), 1,
      anon_sym_bench,
    ACTIONS(248), 20,
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
  [1955] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(254), 1,
      anon_sym_bench,
    ACTIONS(252), 20,
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
  [1984] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(258), 1,
      anon_sym_bench,
    ACTIONS(256), 20,
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
  [2013] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(260), 1,
      anon_sym_RPAREN,
    ACTIONS(264), 1,
      anon_sym_baseline,
    STATE(115), 1,
      sym_chart_param,
    STATE(196), 1,
      sym_chart_params,
    STATE(198), 1,
      sym_chart_param_name,
    ACTIONS(262), 15,
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
  [2049] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(264), 1,
      anon_sym_baseline,
    ACTIONS(266), 1,
      anon_sym_RPAREN,
    STATE(155), 1,
      sym_chart_param,
    STATE(198), 1,
      sym_chart_param_name,
    ACTIONS(262), 15,
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
  [2082] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(264), 1,
      anon_sym_baseline,
    ACTIONS(268), 1,
      anon_sym_RPAREN,
    STATE(155), 1,
      sym_chart_param,
    STATE(198), 1,
      sym_chart_param_name,
    ACTIONS(262), 15,
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
  [2115] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(264), 1,
      anon_sym_baseline,
    STATE(155), 1,
      sym_chart_param,
    STATE(198), 1,
      sym_chart_param_name,
    ACTIONS(262), 15,
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
  [2145] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(270), 1,
      sym_identifier,
    ACTIONS(272), 1,
      anon_sym_DQUOTE,
    ACTIONS(274), 1,
      anon_sym_SQUOTE,
    ACTIONS(276), 1,
      sym_number,
    ACTIONS(278), 1,
      sym_float,
    ACTIONS(282), 1,
      anon_sym_LBRACK,
    ACTIONS(280), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(15), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2178] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(272), 1,
      anon_sym_DQUOTE,
    ACTIONS(274), 1,
      anon_sym_SQUOTE,
    ACTIONS(282), 1,
      anon_sym_LBRACK,
    ACTIONS(284), 1,
      sym_identifier,
    ACTIONS(286), 1,
      sym_number,
    ACTIONS(288), 1,
      sym_float,
    ACTIONS(280), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(158), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2211] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(270), 1,
      sym_identifier,
    ACTIONS(272), 1,
      anon_sym_DQUOTE,
    ACTIONS(274), 1,
      anon_sym_SQUOTE,
    ACTIONS(278), 1,
      sym_float,
    ACTIONS(282), 1,
      anon_sym_LBRACK,
    ACTIONS(290), 1,
      sym_number,
    ACTIONS(280), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(15), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2244] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(270), 1,
      sym_identifier,
    ACTIONS(272), 1,
      anon_sym_DQUOTE,
    ACTIONS(274), 1,
      anon_sym_SQUOTE,
    ACTIONS(278), 1,
      sym_float,
    ACTIONS(282), 1,
      anon_sym_LBRACK,
    ACTIONS(292), 1,
      sym_number,
    ACTIONS(280), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(15), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2277] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(294), 1,
      anon_sym_RBRACE,
    ACTIONS(296), 1,
      anon_sym_import,
    ACTIONS(298), 1,
      anon_sym_declare,
    ACTIONS(300), 1,
      anon_sym_async,
    ACTIONS(302), 1,
      anon_sym_init,
    ACTIONS(304), 1,
      anon_sym_helpers,
    STATE(65), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [2307] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(306), 1,
      anon_sym_RBRACE,
    ACTIONS(308), 1,
      anon_sym_import,
    ACTIONS(311), 1,
      anon_sym_declare,
    ACTIONS(314), 1,
      anon_sym_async,
    ACTIONS(317), 1,
      anon_sym_init,
    ACTIONS(320), 1,
      anon_sym_helpers,
    STATE(64), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [2337] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(296), 1,
      anon_sym_import,
    ACTIONS(298), 1,
      anon_sym_declare,
    ACTIONS(300), 1,
      anon_sym_async,
    ACTIONS(302), 1,
      anon_sym_init,
    ACTIONS(304), 1,
      anon_sym_helpers,
    ACTIONS(323), 1,
      anon_sym_RBRACE,
    STATE(64), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [2367] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(272), 1,
      anon_sym_DQUOTE,
    ACTIONS(274), 1,
      anon_sym_SQUOTE,
    ACTIONS(282), 1,
      anon_sym_LBRACK,
    ACTIONS(325), 1,
      sym_number,
    ACTIONS(327), 1,
      sym_float,
    ACTIONS(329), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(150), 4,
      sym__chart_value,
      sym_string,
      sym_boolean,
      sym_string_array,
  [2396] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(331), 1,
      anon_sym_COLON,
    STATE(214), 1,
      sym_language_tag,
    STATE(26), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(55), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2417] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(333), 1,
      anon_sym_RBRACE,
    STATE(200), 1,
      sym_language_tag,
    STATE(70), 2,
      sym_language_implementation,
      aux_sym_hook_grouped_repeat1,
    ACTIONS(55), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2438] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(331), 1,
      anon_sym_COLON,
    STATE(214), 1,
      sym_language_tag,
    STATE(31), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(55), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2459] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(335), 1,
      anon_sym_RBRACE,
    STATE(200), 1,
      sym_language_tag,
    STATE(70), 2,
      sym_language_implementation,
      aux_sym_hook_grouped_repeat1,
    ACTIONS(337), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2480] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(331), 1,
      anon_sym_COLON,
    STATE(214), 1,
      sym_language_tag,
    STATE(32), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(55), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2501] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(340), 1,
      anon_sym_RBRACE,
    STATE(200), 1,
      sym_language_tag,
    STATE(68), 2,
      sym_language_implementation,
      aux_sym_hook_grouped_repeat1,
    ACTIONS(55), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2522] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(7), 1,
      anon_sym_use,
    ACTIONS(9), 1,
      anon_sym_globalSetup,
    ACTIONS(11), 1,
      anon_sym_suite,
    ACTIONS(342), 1,
      ts_builtin_sym_end,
    STATE(101), 1,
      sym_global_setup,
    STATE(84), 2,
      sym_use_statement,
      aux_sym_source_file_repeat1,
    STATE(100), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [2549] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(331), 1,
      anon_sym_COLON,
    STATE(214), 1,
      sym_language_tag,
    STATE(33), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(55), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2570] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(331), 1,
      anon_sym_COLON,
    STATE(214), 1,
      sym_language_tag,
    STATE(25), 2,
      sym_hook_flat,
      sym_hook_grouped,
    ACTIONS(55), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2591] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(344), 1,
      sym_identifier,
    ACTIONS(346), 1,
      anon_sym_RBRACE,
    ACTIONS(348), 1,
      anon_sym_anvil,
    STATE(77), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(131), 2,
      sym_anvil_call,
      sym_function_call,
  [2612] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(344), 1,
      sym_identifier,
    ACTIONS(348), 1,
      anon_sym_anvil,
    ACTIONS(350), 1,
      anon_sym_RBRACE,
    STATE(78), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(131), 2,
      sym_anvil_call,
      sym_function_call,
  [2633] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(352), 1,
      sym_identifier,
    ACTIONS(355), 1,
      anon_sym_RBRACE,
    ACTIONS(357), 1,
      anon_sym_anvil,
    STATE(78), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(131), 2,
      sym_anvil_call,
      sym_function_call,
  [2654] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(166), 1,
      anon_sym_m,
    STATE(8), 1,
      sym_duration_unit,
    ACTIONS(162), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
    ACTIONS(164), 2,
      anon_sym_ms,
      anon_sym_s,
  [2672] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(360), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [2684] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(362), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [2696] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(364), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [2708] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(366), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [2720] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(370), 1,
      anon_sym_use,
    STATE(84), 2,
      sym_use_statement,
      aux_sym_source_file_repeat1,
    ACTIONS(368), 3,
      ts_builtin_sym_end,
      anon_sym_globalSetup,
      anon_sym_suite,
  [2736] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(373), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [2748] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(375), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [2760] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(152), 1,
      sym_language_tag,
    ACTIONS(55), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2774] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(377), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [2786] = 5,
    ACTIONS(379), 1,
      anon_sym_DQUOTE,
    ACTIONS(383), 1,
      sym_comment,
    STATE(111), 1,
      aux_sym_string_content_repeat1,
    STATE(185), 1,
      sym_string_content,
    ACTIONS(381), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [2803] = 5,
    ACTIONS(379), 1,
      anon_sym_SQUOTE,
    ACTIONS(383), 1,
      sym_comment,
    STATE(114), 1,
      aux_sym_single_string_content_repeat1,
    STATE(184), 1,
      sym_single_string_content,
    ACTIONS(385), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [2820] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(272), 1,
      anon_sym_DQUOTE,
    ACTIONS(274), 1,
      anon_sym_SQUOTE,
    ACTIONS(387), 1,
      anon_sym_ATfile,
    STATE(41), 2,
      sym_file_ref,
      sym_string,
  [2837] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(389), 1,
      sym_identifier,
    ACTIONS(391), 1,
      anon_sym_RPAREN,
    STATE(141), 1,
      sym_argument,
    STATE(190), 1,
      sym_argument_list,
  [2853] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(272), 1,
      anon_sym_DQUOTE,
    ACTIONS(274), 1,
      anon_sym_SQUOTE,
    ACTIONS(393), 1,
      anon_sym_RBRACK,
    STATE(166), 1,
      sym_string,
  [2869] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_suite,
    ACTIONS(342), 1,
      ts_builtin_sym_end,
    STATE(100), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [2883] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_suite,
    ACTIONS(342), 1,
      ts_builtin_sym_end,
    STATE(103), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [2897] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(395), 1,
      anon_sym_RBRACE,
    ACTIONS(397), 1,
      anon_sym_charting,
    STATE(106), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [2911] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(399), 1,
      anon_sym_LBRACE,
    ACTIONS(401), 1,
      anon_sym_LPAREN,
    STATE(48), 1,
      sym_fixture_body,
    STATE(160), 1,
      sym_fixture_params,
  [2927] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(403), 1,
      anon_sym_LBRACE,
    ACTIONS(405), 1,
      anon_sym_LPAREN,
    STATE(81), 2,
      sym_code_block,
      sym_paren_code_block,
  [2941] = 4,
    ACTIONS(383), 1,
      sym_comment,
    ACTIONS(407), 1,
      anon_sym_LBRACE,
    ACTIONS(409), 1,
      sym_inline_code,
    STATE(24), 2,
      sym__code_or_inline,
      sym_code_block,
  [2955] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_suite,
    ACTIONS(411), 1,
      ts_builtin_sym_end,
    STATE(103), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [2969] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_suite,
    ACTIONS(411), 1,
      ts_builtin_sym_end,
    STATE(107), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [2983] = 4,
    ACTIONS(383), 1,
      sym_comment,
    ACTIONS(413), 1,
      anon_sym_DQUOTE,
    STATE(102), 1,
      aux_sym_string_content_repeat1,
    ACTIONS(415), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [2997] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(418), 1,
      ts_builtin_sym_end,
    ACTIONS(420), 1,
      anon_sym_suite,
    STATE(103), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3011] = 4,
    ACTIONS(383), 1,
      sym_comment,
    ACTIONS(423), 1,
      anon_sym_SQUOTE,
    STATE(104), 1,
      aux_sym_single_string_content_repeat1,
    ACTIONS(425), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [3025] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(272), 1,
      anon_sym_DQUOTE,
    ACTIONS(274), 1,
      anon_sym_SQUOTE,
    ACTIONS(428), 1,
      anon_sym_RBRACK,
    STATE(166), 1,
      sym_string,
  [3041] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(397), 1,
      anon_sym_charting,
    ACTIONS(430), 1,
      anon_sym_RBRACE,
    STATE(113), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [3055] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_suite,
    ACTIONS(432), 1,
      ts_builtin_sym_end,
    STATE(103), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [3069] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(272), 1,
      anon_sym_DQUOTE,
    ACTIONS(274), 1,
      anon_sym_SQUOTE,
    ACTIONS(434), 1,
      anon_sym_RBRACK,
    STATE(122), 1,
      sym_string,
  [3085] = 4,
    ACTIONS(383), 1,
      sym_comment,
    ACTIONS(407), 1,
      anon_sym_LBRACE,
    ACTIONS(436), 1,
      sym_inline_code,
    STATE(28), 2,
      sym__code_or_inline,
      sym_code_block,
  [3099] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(438), 4,
      ts_builtin_sym_end,
      anon_sym_use,
      anon_sym_globalSetup,
      anon_sym_suite,
  [3109] = 4,
    ACTIONS(383), 1,
      sym_comment,
    ACTIONS(440), 1,
      anon_sym_DQUOTE,
    STATE(102), 1,
      aux_sym_string_content_repeat1,
    ACTIONS(442), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [3123] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(389), 1,
      sym_identifier,
    ACTIONS(444), 1,
      anon_sym_RPAREN,
    STATE(141), 1,
      sym_argument,
    STATE(212), 1,
      sym_argument_list,
  [3139] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(446), 1,
      anon_sym_RBRACE,
    ACTIONS(448), 1,
      anon_sym_charting,
    STATE(113), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [3153] = 4,
    ACTIONS(383), 1,
      sym_comment,
    ACTIONS(451), 1,
      anon_sym_SQUOTE,
    STATE(104), 1,
      aux_sym_single_string_content_repeat1,
    ACTIONS(453), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [3167] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(455), 1,
      anon_sym_RPAREN,
    ACTIONS(457), 1,
      anon_sym_COMMA,
    STATE(138), 1,
      aux_sym_chart_params_repeat1,
  [3180] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(461), 1,
      anon_sym_RBRACE,
    ACTIONS(459), 2,
      anon_sym_anvil,
      sym_identifier,
  [3191] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(465), 1,
      anon_sym_RBRACE,
    ACTIONS(463), 2,
      anon_sym_anvil,
      sym_identifier,
  [3202] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(467), 1,
      anon_sym_COMMA,
    ACTIONS(470), 1,
      anon_sym_RBRACK,
    STATE(118), 1,
      aux_sym_string_array_repeat1,
  [3215] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(389), 1,
      sym_identifier,
    ACTIONS(472), 1,
      anon_sym_RPAREN,
    STATE(147), 1,
      sym_argument,
  [3228] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(476), 1,
      anon_sym_RBRACE,
    ACTIONS(474), 2,
      anon_sym_anvil,
      sym_identifier,
  [3239] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(272), 1,
      anon_sym_DQUOTE,
    ACTIONS(274), 1,
      anon_sym_SQUOTE,
    STATE(201), 1,
      sym_string,
  [3252] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(478), 1,
      anon_sym_COMMA,
    ACTIONS(480), 1,
      anon_sym_RBRACK,
    STATE(128), 1,
      aux_sym_string_array_repeat1,
  [3265] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(272), 1,
      anon_sym_DQUOTE,
    ACTIONS(274), 1,
      anon_sym_SQUOTE,
    STATE(204), 1,
      sym_string,
  [3278] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(484), 1,
      anon_sym_RBRACE,
    ACTIONS(482), 2,
      anon_sym_anvil,
      sym_identifier,
  [3289] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(488), 1,
      anon_sym_RBRACE,
    ACTIONS(486), 2,
      anon_sym_anvil,
      sym_identifier,
  [3300] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(490), 1,
      anon_sym_RPAREN,
    ACTIONS(492), 1,
      anon_sym_COMMA,
    STATE(126), 1,
      aux_sym_fixture_params_repeat1,
  [3313] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(495), 1,
      sym_identifier,
    ACTIONS(497), 1,
      anon_sym_RPAREN,
    STATE(168), 1,
      sym_fixture_param,
  [3326] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(428), 1,
      anon_sym_RBRACK,
    ACTIONS(499), 1,
      anon_sym_COMMA,
    STATE(118), 1,
      aux_sym_string_array_repeat1,
  [3339] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(501), 1,
      anon_sym_RPAREN,
    ACTIONS(503), 1,
      anon_sym_COMMA,
    STATE(132), 1,
      aux_sym_fixture_params_repeat1,
  [3352] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(209), 1,
      sym_chart_function_name,
    ACTIONS(505), 2,
      anon_sym_drawSpeedupChart,
      anon_sym_drawTable,
  [3363] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(509), 1,
      anon_sym_RBRACE,
    ACTIONS(507), 2,
      anon_sym_anvil,
      sym_identifier,
  [3374] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(511), 1,
      anon_sym_RPAREN,
    ACTIONS(513), 1,
      anon_sym_COMMA,
    STATE(126), 1,
      aux_sym_fixture_params_repeat1,
  [3387] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(495), 1,
      sym_identifier,
    ACTIONS(515), 1,
      anon_sym_RPAREN,
    STATE(129), 1,
      sym_fixture_param,
  [3400] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(517), 1,
      anon_sym_RPAREN,
    ACTIONS(519), 1,
      anon_sym_COMMA,
    STATE(142), 1,
      aux_sym_argument_list_repeat1,
  [3413] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(389), 1,
      sym_identifier,
    ACTIONS(517), 1,
      anon_sym_RPAREN,
    STATE(147), 1,
      sym_argument,
  [3426] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(523), 1,
      anon_sym_RBRACE,
    ACTIONS(521), 2,
      anon_sym_anvil,
      sym_identifier,
  [3437] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(272), 1,
      anon_sym_DQUOTE,
    ACTIONS(274), 1,
      anon_sym_SQUOTE,
    STATE(166), 1,
      sym_string,
  [3450] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(268), 1,
      anon_sym_RPAREN,
    ACTIONS(525), 1,
      anon_sym_COMMA,
    STATE(140), 1,
      aux_sym_chart_params_repeat1,
  [3463] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(527), 1,
      anon_sym_RPAREN,
    ACTIONS(529), 1,
      anon_sym_fork,
    STATE(187), 1,
      sym_anvil_args,
  [3476] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(531), 1,
      anon_sym_RPAREN,
    ACTIONS(533), 1,
      anon_sym_COMMA,
    STATE(140), 1,
      aux_sym_chart_params_repeat1,
  [3489] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(536), 1,
      anon_sym_RPAREN,
    ACTIONS(538), 1,
      anon_sym_COMMA,
    STATE(134), 1,
      aux_sym_argument_list_repeat1,
  [3502] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(540), 1,
      anon_sym_RPAREN,
    ACTIONS(542), 1,
      anon_sym_COMMA,
    STATE(142), 1,
      aux_sym_argument_list_repeat1,
  [3515] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(495), 1,
      sym_identifier,
    ACTIONS(511), 1,
      anon_sym_RPAREN,
    STATE(168), 1,
      sym_fixture_param,
  [3528] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(495), 1,
      sym_identifier,
    STATE(168), 1,
      sym_fixture_param,
  [3538] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(545), 2,
      ts_builtin_sym_end,
      anon_sym_suite,
  [3546] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(547), 1,
      anon_sym_LBRACE,
    STATE(38), 1,
      sym_global_setup_body,
  [3556] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(540), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [3564] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(282), 1,
      anon_sym_LBRACK,
    STATE(34), 1,
      sym_string_array,
  [3574] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(403), 1,
      anon_sym_LBRACE,
    STATE(80), 1,
      sym_code_block,
  [3584] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(549), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [3592] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(551), 1,
      anon_sym_LBRACE,
    STATE(42), 1,
      sym_benchmark_body,
  [3602] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(553), 1,
      anon_sym_LBRACE,
    STATE(54), 1,
      sym_setup_body,
  [3612] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(555), 2,
      anon_sym_LBRACE,
      anon_sym_COLON,
  [3620] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(557), 1,
      anon_sym_LBRACE,
    STATE(165), 1,
      sym_suite_body,
  [3630] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(531), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [3638] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(403), 1,
      anon_sym_LBRACE,
    STATE(82), 1,
      sym_code_block,
  [3648] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(403), 1,
      anon_sym_LBRACE,
    STATE(83), 1,
      sym_code_block,
  [3658] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(559), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [3666] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(403), 1,
      anon_sym_LBRACE,
    STATE(40), 1,
      sym_code_block,
  [3676] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(399), 1,
      anon_sym_LBRACE,
    STATE(46), 1,
      sym_fixture_body,
  [3686] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(403), 1,
      anon_sym_LBRACE,
    STATE(86), 1,
      sym_code_block,
  [3696] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(561), 2,
      anon_sym_RBRACE,
      anon_sym_charting,
  [3704] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(563), 1,
      anon_sym_DOT,
    ACTIONS(565), 1,
      anon_sym_LPAREN,
  [3714] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(567), 2,
      anon_sym_RBRACE,
      anon_sym_charting,
  [3722] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(569), 2,
      ts_builtin_sym_end,
      anon_sym_suite,
  [3730] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(470), 2,
      anon_sym_COMMA,
      anon_sym_RBRACK,
  [3738] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(571), 2,
      ts_builtin_sym_end,
      anon_sym_suite,
  [3746] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(490), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [3754] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(573), 1,
      anon_sym_RPAREN,
    ACTIONS(575), 1,
      sym_embedded_code,
  [3764] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(577), 1,
      anon_sym_RBRACE,
    ACTIONS(579), 1,
      sym_embedded_code,
  [3774] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(581), 1,
      anon_sym_LBRACE,
    STATE(49), 1,
      sym_after_body,
  [3784] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(389), 1,
      sym_identifier,
    STATE(147), 1,
      sym_argument,
  [3794] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(583), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [3802] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(585), 1,
      sym_identifier,
  [3809] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(587), 1,
      anon_sym_LBRACE,
  [3816] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(589), 1,
      anon_sym_LPAREN,
  [3823] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(591), 1,
      anon_sym_COLON,
  [3830] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(593), 1,
      sym_identifier,
  [3837] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(595), 1,
      anon_sym_LBRACE,
  [3844] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(597), 1,
      sym_identifier,
  [3851] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(599), 1,
      anon_sym_COLON,
  [3858] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(601), 1,
      sym_identifier,
  [3865] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(603), 1,
      anon_sym_LPAREN,
  [3872] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(605), 1,
      anon_sym_SQUOTE,
  [3879] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(605), 1,
      anon_sym_DQUOTE,
  [3886] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(607), 1,
      anon_sym_spawnAnvil,
  [3893] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(609), 1,
      anon_sym_RPAREN,
  [3900] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(611), 1,
      anon_sym_std,
  [3907] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(613), 1,
      anon_sym_LBRACE,
  [3914] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(615), 1,
      anon_sym_RPAREN,
  [3921] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(617), 1,
      anon_sym_COLON,
  [3928] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(619), 1,
      anon_sym_RPAREN,
  [3935] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(621), 1,
      anon_sym_COLON,
  [3942] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(623), 1,
      anon_sym_LPAREN,
  [3949] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(625), 1,
      anon_sym_COLON,
  [3956] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(627), 1,
      anon_sym_RPAREN,
  [3963] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(629), 1,
      anon_sym_COLON,
  [3970] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(631), 1,
      anon_sym_COLON,
  [3977] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(633), 1,
      anon_sym_RBRACE,
  [3984] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(635), 1,
      anon_sym_COLON,
  [3991] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(637), 1,
      anon_sym_RPAREN,
  [3998] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(639), 1,
      anon_sym_DOT,
  [4005] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(641), 1,
      sym_identifier,
  [4012] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(643), 1,
      anon_sym_RPAREN,
  [4019] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(645), 1,
      anon_sym_COLON,
  [4026] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(647), 1,
      anon_sym_COLON,
  [4033] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(649), 1,
      anon_sym_DOT,
  [4040] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(651), 1,
      anon_sym_COLON,
  [4047] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(653), 1,
      anon_sym_LPAREN,
  [4054] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(655), 1,
      anon_sym_LPAREN,
  [4061] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(657), 1,
      anon_sym_init,
  [4068] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(659), 1,
      anon_sym_RPAREN,
  [4075] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(661), 1,
      anon_sym_COLON_COLON,
  [4082] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(663), 1,
      anon_sym_COLON,
  [4089] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(665), 1,
      anon_sym_LBRACE,
  [4096] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(667), 1,
      ts_builtin_sym_end,
  [4103] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(669), 1,
      sym_identifier,
  [4110] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(671), 1,
      anon_sym_LBRACE,
  [4117] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(673), 1,
      anon_sym_COLON,
  [4124] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(675), 1,
      anon_sym_COLON,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 66,
  [SMALL_STATE(4)] = 132,
  [SMALL_STATE(5)] = 198,
  [SMALL_STATE(6)] = 242,
  [SMALL_STATE(7)] = 286,
  [SMALL_STATE(8)] = 329,
  [SMALL_STATE(9)] = 372,
  [SMALL_STATE(10)] = 415,
  [SMALL_STATE(11)] = 458,
  [SMALL_STATE(12)] = 501,
  [SMALL_STATE(13)] = 544,
  [SMALL_STATE(14)] = 587,
  [SMALL_STATE(15)] = 626,
  [SMALL_STATE(16)] = 667,
  [SMALL_STATE(17)] = 706,
  [SMALL_STATE(18)] = 756,
  [SMALL_STATE(19)] = 810,
  [SMALL_STATE(20)] = 860,
  [SMALL_STATE(21)] = 910,
  [SMALL_STATE(22)] = 964,
  [SMALL_STATE(23)] = 1018,
  [SMALL_STATE(24)] = 1060,
  [SMALL_STATE(25)] = 1094,
  [SMALL_STATE(26)] = 1126,
  [SMALL_STATE(27)] = 1158,
  [SMALL_STATE(28)] = 1190,
  [SMALL_STATE(29)] = 1222,
  [SMALL_STATE(30)] = 1260,
  [SMALL_STATE(31)] = 1292,
  [SMALL_STATE(32)] = 1324,
  [SMALL_STATE(33)] = 1356,
  [SMALL_STATE(34)] = 1388,
  [SMALL_STATE(35)] = 1420,
  [SMALL_STATE(36)] = 1459,
  [SMALL_STATE(37)] = 1490,
  [SMALL_STATE(38)] = 1521,
  [SMALL_STATE(39)] = 1552,
  [SMALL_STATE(40)] = 1580,
  [SMALL_STATE(41)] = 1608,
  [SMALL_STATE(42)] = 1636,
  [SMALL_STATE(43)] = 1665,
  [SMALL_STATE(44)] = 1694,
  [SMALL_STATE(45)] = 1723,
  [SMALL_STATE(46)] = 1752,
  [SMALL_STATE(47)] = 1781,
  [SMALL_STATE(48)] = 1810,
  [SMALL_STATE(49)] = 1839,
  [SMALL_STATE(50)] = 1868,
  [SMALL_STATE(51)] = 1897,
  [SMALL_STATE(52)] = 1926,
  [SMALL_STATE(53)] = 1955,
  [SMALL_STATE(54)] = 1984,
  [SMALL_STATE(55)] = 2013,
  [SMALL_STATE(56)] = 2049,
  [SMALL_STATE(57)] = 2082,
  [SMALL_STATE(58)] = 2115,
  [SMALL_STATE(59)] = 2145,
  [SMALL_STATE(60)] = 2178,
  [SMALL_STATE(61)] = 2211,
  [SMALL_STATE(62)] = 2244,
  [SMALL_STATE(63)] = 2277,
  [SMALL_STATE(64)] = 2307,
  [SMALL_STATE(65)] = 2337,
  [SMALL_STATE(66)] = 2367,
  [SMALL_STATE(67)] = 2396,
  [SMALL_STATE(68)] = 2417,
  [SMALL_STATE(69)] = 2438,
  [SMALL_STATE(70)] = 2459,
  [SMALL_STATE(71)] = 2480,
  [SMALL_STATE(72)] = 2501,
  [SMALL_STATE(73)] = 2522,
  [SMALL_STATE(74)] = 2549,
  [SMALL_STATE(75)] = 2570,
  [SMALL_STATE(76)] = 2591,
  [SMALL_STATE(77)] = 2612,
  [SMALL_STATE(78)] = 2633,
  [SMALL_STATE(79)] = 2654,
  [SMALL_STATE(80)] = 2672,
  [SMALL_STATE(81)] = 2684,
  [SMALL_STATE(82)] = 2696,
  [SMALL_STATE(83)] = 2708,
  [SMALL_STATE(84)] = 2720,
  [SMALL_STATE(85)] = 2736,
  [SMALL_STATE(86)] = 2748,
  [SMALL_STATE(87)] = 2760,
  [SMALL_STATE(88)] = 2774,
  [SMALL_STATE(89)] = 2786,
  [SMALL_STATE(90)] = 2803,
  [SMALL_STATE(91)] = 2820,
  [SMALL_STATE(92)] = 2837,
  [SMALL_STATE(93)] = 2853,
  [SMALL_STATE(94)] = 2869,
  [SMALL_STATE(95)] = 2883,
  [SMALL_STATE(96)] = 2897,
  [SMALL_STATE(97)] = 2911,
  [SMALL_STATE(98)] = 2927,
  [SMALL_STATE(99)] = 2941,
  [SMALL_STATE(100)] = 2955,
  [SMALL_STATE(101)] = 2969,
  [SMALL_STATE(102)] = 2983,
  [SMALL_STATE(103)] = 2997,
  [SMALL_STATE(104)] = 3011,
  [SMALL_STATE(105)] = 3025,
  [SMALL_STATE(106)] = 3041,
  [SMALL_STATE(107)] = 3055,
  [SMALL_STATE(108)] = 3069,
  [SMALL_STATE(109)] = 3085,
  [SMALL_STATE(110)] = 3099,
  [SMALL_STATE(111)] = 3109,
  [SMALL_STATE(112)] = 3123,
  [SMALL_STATE(113)] = 3139,
  [SMALL_STATE(114)] = 3153,
  [SMALL_STATE(115)] = 3167,
  [SMALL_STATE(116)] = 3180,
  [SMALL_STATE(117)] = 3191,
  [SMALL_STATE(118)] = 3202,
  [SMALL_STATE(119)] = 3215,
  [SMALL_STATE(120)] = 3228,
  [SMALL_STATE(121)] = 3239,
  [SMALL_STATE(122)] = 3252,
  [SMALL_STATE(123)] = 3265,
  [SMALL_STATE(124)] = 3278,
  [SMALL_STATE(125)] = 3289,
  [SMALL_STATE(126)] = 3300,
  [SMALL_STATE(127)] = 3313,
  [SMALL_STATE(128)] = 3326,
  [SMALL_STATE(129)] = 3339,
  [SMALL_STATE(130)] = 3352,
  [SMALL_STATE(131)] = 3363,
  [SMALL_STATE(132)] = 3374,
  [SMALL_STATE(133)] = 3387,
  [SMALL_STATE(134)] = 3400,
  [SMALL_STATE(135)] = 3413,
  [SMALL_STATE(136)] = 3426,
  [SMALL_STATE(137)] = 3437,
  [SMALL_STATE(138)] = 3450,
  [SMALL_STATE(139)] = 3463,
  [SMALL_STATE(140)] = 3476,
  [SMALL_STATE(141)] = 3489,
  [SMALL_STATE(142)] = 3502,
  [SMALL_STATE(143)] = 3515,
  [SMALL_STATE(144)] = 3528,
  [SMALL_STATE(145)] = 3538,
  [SMALL_STATE(146)] = 3546,
  [SMALL_STATE(147)] = 3556,
  [SMALL_STATE(148)] = 3564,
  [SMALL_STATE(149)] = 3574,
  [SMALL_STATE(150)] = 3584,
  [SMALL_STATE(151)] = 3592,
  [SMALL_STATE(152)] = 3602,
  [SMALL_STATE(153)] = 3612,
  [SMALL_STATE(154)] = 3620,
  [SMALL_STATE(155)] = 3630,
  [SMALL_STATE(156)] = 3638,
  [SMALL_STATE(157)] = 3648,
  [SMALL_STATE(158)] = 3658,
  [SMALL_STATE(159)] = 3666,
  [SMALL_STATE(160)] = 3676,
  [SMALL_STATE(161)] = 3686,
  [SMALL_STATE(162)] = 3696,
  [SMALL_STATE(163)] = 3704,
  [SMALL_STATE(164)] = 3714,
  [SMALL_STATE(165)] = 3722,
  [SMALL_STATE(166)] = 3730,
  [SMALL_STATE(167)] = 3738,
  [SMALL_STATE(168)] = 3746,
  [SMALL_STATE(169)] = 3754,
  [SMALL_STATE(170)] = 3764,
  [SMALL_STATE(171)] = 3774,
  [SMALL_STATE(172)] = 3784,
  [SMALL_STATE(173)] = 3794,
  [SMALL_STATE(174)] = 3802,
  [SMALL_STATE(175)] = 3809,
  [SMALL_STATE(176)] = 3816,
  [SMALL_STATE(177)] = 3823,
  [SMALL_STATE(178)] = 3830,
  [SMALL_STATE(179)] = 3837,
  [SMALL_STATE(180)] = 3844,
  [SMALL_STATE(181)] = 3851,
  [SMALL_STATE(182)] = 3858,
  [SMALL_STATE(183)] = 3865,
  [SMALL_STATE(184)] = 3872,
  [SMALL_STATE(185)] = 3879,
  [SMALL_STATE(186)] = 3886,
  [SMALL_STATE(187)] = 3893,
  [SMALL_STATE(188)] = 3900,
  [SMALL_STATE(189)] = 3907,
  [SMALL_STATE(190)] = 3914,
  [SMALL_STATE(191)] = 3921,
  [SMALL_STATE(192)] = 3928,
  [SMALL_STATE(193)] = 3935,
  [SMALL_STATE(194)] = 3942,
  [SMALL_STATE(195)] = 3949,
  [SMALL_STATE(196)] = 3956,
  [SMALL_STATE(197)] = 3963,
  [SMALL_STATE(198)] = 3970,
  [SMALL_STATE(199)] = 3977,
  [SMALL_STATE(200)] = 3984,
  [SMALL_STATE(201)] = 3991,
  [SMALL_STATE(202)] = 3998,
  [SMALL_STATE(203)] = 4005,
  [SMALL_STATE(204)] = 4012,
  [SMALL_STATE(205)] = 4019,
  [SMALL_STATE(206)] = 4026,
  [SMALL_STATE(207)] = 4033,
  [SMALL_STATE(208)] = 4040,
  [SMALL_STATE(209)] = 4047,
  [SMALL_STATE(210)] = 4054,
  [SMALL_STATE(211)] = 4061,
  [SMALL_STATE(212)] = 4068,
  [SMALL_STATE(213)] = 4075,
  [SMALL_STATE(214)] = 4082,
  [SMALL_STATE(215)] = 4089,
  [SMALL_STATE(216)] = 4096,
  [SMALL_STATE(217)] = 4103,
  [SMALL_STATE(218)] = 4110,
  [SMALL_STATE(219)] = 4117,
  [SMALL_STATE(220)] = 4124,
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
  [36] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_benchmark_body_repeat1, 2, 0, 0), SHIFT_REPEAT(153),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(191),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [45] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [47] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [49] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [51] = {.entry = {.count = 1, .reusable = true}}, SHIFT(67),
  [53] = {.entry = {.count = 1, .reusable = true}}, SHIFT(177),
  [55] = {.entry = {.count = 1, .reusable = true}}, SHIFT(153),
  [57] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [59] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 3, 0, 0),
  [61] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 3, 0, 0),
  [63] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 2, 0, 0),
  [65] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 2, 0, 0),
  [67] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_duration_unit, 1, 0, 0),
  [69] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_duration_unit, 1, 0, 0),
  [71] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_duration, 2, 0, 0),
  [73] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_duration, 2, 0, 0),
  [75] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_boolean, 1, 0, 0),
  [77] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_boolean, 1, 0, 0),
  [79] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 2, 0, 0),
  [81] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 2, 0, 0),
  [83] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 3, 0, 0),
  [85] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 3, 0, 0),
  [87] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 4, 0, 0),
  [89] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 4, 0, 0),
  [91] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 5, 0, 0),
  [93] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_array, 5, 0, 0),
  [95] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_code_block, 2, 0, 0),
  [97] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property, 3, 0, 4),
  [99] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_property, 3, 0, 4),
  [101] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_code_block, 3, 0, 0),
  [103] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [105] = {.entry = {.count = 1, .reusable = true}}, SHIFT(206),
  [107] = {.entry = {.count = 1, .reusable = true}}, SHIFT(205),
  [109] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(146),
  [112] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0),
  [114] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(87),
  [117] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(180),
  [120] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(178),
  [123] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(178),
  [126] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(171),
  [129] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(177),
  [132] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0),
  [134] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(206),
  [137] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(205),
  [140] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(177),
  [143] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(153),
  [146] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [148] = {.entry = {.count = 1, .reusable = true}}, SHIFT(167),
  [150] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [152] = {.entry = {.count = 1, .reusable = true}}, SHIFT(180),
  [154] = {.entry = {.count = 1, .reusable = false}}, SHIFT(178),
  [156] = {.entry = {.count = 1, .reusable = true}}, SHIFT(178),
  [158] = {.entry = {.count = 1, .reusable = true}}, SHIFT(171),
  [160] = {.entry = {.count = 1, .reusable = true}}, SHIFT(145),
  [162] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__value, 1, 0, 0),
  [164] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [166] = {.entry = {.count = 1, .reusable = false}}, SHIFT(7),
  [168] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_language_implementation, 3, 0, 5),
  [170] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_hook, 2, 0, 0),
  [172] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_each_hook, 2, 0, 0),
  [174] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_grouped, 4, 0, 0),
  [176] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_flat, 3, 0, 5),
  [178] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_grouped, 3, 0, 0),
  [180] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_skip_hook, 2, 0, 0),
  [182] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_validate_hook, 2, 0, 0),
  [184] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_before_hook, 2, 0, 0),
  [186] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tags_property, 3, 0, 0),
  [188] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__value, 1, 0, 0),
  [190] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_body, 2, 0, 0),
  [192] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup_body, 2, 0, 0),
  [194] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_body, 3, 0, 0),
  [196] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup_body, 3, 0, 0),
  [198] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup, 2, 0, 0),
  [200] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup, 2, 0, 0),
  [202] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_file_ref, 4, 0, 0),
  [204] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_shape_property, 3, 0, 0),
  [206] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hex_property, 3, 0, 0),
  [208] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark, 3, 0, 1),
  [210] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_benchmark, 3, 0, 1),
  [212] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_body, 2, 0, 0),
  [214] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_body, 2, 0, 0),
  [216] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark_body, 3, 0, 0),
  [218] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_benchmark_body, 3, 0, 0),
  [220] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_body, 2, 0, 0),
  [222] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_setup_body, 2, 0, 0),
  [224] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture, 4, 0, 1),
  [226] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture, 4, 0, 1),
  [228] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_body, 3, 0, 0),
  [230] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_body, 3, 0, 0),
  [232] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture, 3, 0, 1),
  [234] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture, 3, 0, 1),
  [236] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_block, 2, 0, 0),
  [238] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_after_block, 2, 0, 0),
  [240] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_body, 3, 0, 0),
  [242] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture_body, 3, 0, 0),
  [244] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark_body, 2, 0, 0),
  [246] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_benchmark_body, 2, 0, 0),
  [248] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_body, 3, 0, 0),
  [250] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_setup_body, 3, 0, 0),
  [252] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_body, 2, 0, 0),
  [254] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fixture_body, 2, 0, 0),
  [256] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_block, 3, 0, 3),
  [258] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_setup_block, 3, 0, 3),
  [260] = {.entry = {.count = 1, .reusable = true}}, SHIFT(164),
  [262] = {.entry = {.count = 1, .reusable = true}}, SHIFT(195),
  [264] = {.entry = {.count = 1, .reusable = false}}, SHIFT(195),
  [266] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 3, 0, 0),
  [268] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 2, 0, 0),
  [270] = {.entry = {.count = 1, .reusable = false}}, SHIFT(15),
  [272] = {.entry = {.count = 1, .reusable = true}}, SHIFT(89),
  [274] = {.entry = {.count = 1, .reusable = true}}, SHIFT(90),
  [276] = {.entry = {.count = 1, .reusable = false}}, SHIFT(29),
  [278] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [280] = {.entry = {.count = 1, .reusable = false}}, SHIFT(9),
  [282] = {.entry = {.count = 1, .reusable = true}}, SHIFT(108),
  [284] = {.entry = {.count = 1, .reusable = false}}, SHIFT(158),
  [286] = {.entry = {.count = 1, .reusable = false}}, SHIFT(79),
  [288] = {.entry = {.count = 1, .reusable = true}}, SHIFT(158),
  [290] = {.entry = {.count = 1, .reusable = false}}, SHIFT(35),
  [292] = {.entry = {.count = 1, .reusable = false}}, SHIFT(23),
  [294] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [296] = {.entry = {.count = 1, .reusable = true}}, SHIFT(98),
  [298] = {.entry = {.count = 1, .reusable = true}}, SHIFT(149),
  [300] = {.entry = {.count = 1, .reusable = true}}, SHIFT(211),
  [302] = {.entry = {.count = 1, .reusable = true}}, SHIFT(156),
  [304] = {.entry = {.count = 1, .reusable = true}}, SHIFT(157),
  [306] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0),
  [308] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(98),
  [311] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(149),
  [314] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(211),
  [317] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(156),
  [320] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(157),
  [323] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [325] = {.entry = {.count = 1, .reusable = false}}, SHIFT(150),
  [327] = {.entry = {.count = 1, .reusable = true}}, SHIFT(150),
  [329] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [331] = {.entry = {.count = 1, .reusable = true}}, SHIFT(215),
  [333] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [335] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_hook_grouped_repeat1, 2, 0, 0),
  [337] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_hook_grouped_repeat1, 2, 0, 0), SHIFT_REPEAT(153),
  [340] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [342] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, 0, 0),
  [344] = {.entry = {.count = 1, .reusable = false}}, SHIFT(163),
  [346] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [348] = {.entry = {.count = 1, .reusable = false}}, SHIFT(202),
  [350] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [352] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(163),
  [355] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0),
  [357] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(202),
  [360] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declare_section, 2, 0, 0),
  [362] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_section, 2, 0, 0),
  [364] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_init_section, 2, 0, 0),
  [366] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_helpers_section, 2, 0, 0),
  [368] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0),
  [370] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(188),
  [373] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_paren_code_block, 2, 0, 0),
  [375] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_init_section, 3, 0, 0),
  [377] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_paren_code_block, 3, 0, 0),
  [379] = {.entry = {.count = 1, .reusable = false}}, SHIFT(6),
  [381] = {.entry = {.count = 1, .reusable = false}}, SHIFT(111),
  [383] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [385] = {.entry = {.count = 1, .reusable = false}}, SHIFT(114),
  [387] = {.entry = {.count = 1, .reusable = true}}, SHIFT(176),
  [389] = {.entry = {.count = 1, .reusable = true}}, SHIFT(208),
  [391] = {.entry = {.count = 1, .reusable = true}}, SHIFT(117),
  [393] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [395] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [397] = {.entry = {.count = 1, .reusable = true}}, SHIFT(207),
  [399] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [401] = {.entry = {.count = 1, .reusable = true}}, SHIFT(133),
  [403] = {.entry = {.count = 1, .reusable = true}}, SHIFT(170),
  [405] = {.entry = {.count = 1, .reusable = true}}, SHIFT(169),
  [407] = {.entry = {.count = 1, .reusable = false}}, SHIFT(170),
  [409] = {.entry = {.count = 1, .reusable = false}}, SHIFT(24),
  [411] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, 0, 0),
  [413] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_string_content_repeat1, 2, 0, 0),
  [415] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_string_content_repeat1, 2, 0, 0), SHIFT_REPEAT(102),
  [418] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat2, 2, 0, 0),
  [420] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat2, 2, 0, 0), SHIFT_REPEAT(217),
  [423] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_single_string_content_repeat1, 2, 0, 0),
  [425] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_single_string_content_repeat1, 2, 0, 0), SHIFT_REPEAT(104),
  [428] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [430] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [432] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, 0, 0),
  [434] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [436] = {.entry = {.count = 1, .reusable = false}}, SHIFT(28),
  [438] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_use_statement, 4, 0, 2),
  [440] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_content, 1, 0, 0),
  [442] = {.entry = {.count = 1, .reusable = false}}, SHIFT(102),
  [444] = {.entry = {.count = 1, .reusable = true}}, SHIFT(116),
  [446] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_after_body_repeat1, 2, 0, 0),
  [448] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_after_body_repeat1, 2, 0, 0), SHIFT_REPEAT(207),
  [451] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_single_string_content, 1, 0, 0),
  [453] = {.entry = {.count = 1, .reusable = false}}, SHIFT(104),
  [455] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 1, 0, 0),
  [457] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [459] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 3, 0, 0),
  [461] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 3, 0, 0),
  [463] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 5, 0, 0),
  [465] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 5, 0, 0),
  [467] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_string_array_repeat1, 2, 0, 0), SHIFT_REPEAT(137),
  [470] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_string_array_repeat1, 2, 0, 0),
  [472] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 3, 0, 0),
  [474] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_anvil_call, 5, 0, 0),
  [476] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_call, 5, 0, 0),
  [478] = {.entry = {.count = 1, .reusable = true}}, SHIFT(105),
  [480] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [482] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_anvil_call, 6, 0, 0),
  [484] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_call, 6, 0, 0),
  [486] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 6, 0, 0),
  [488] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 6, 0, 0),
  [490] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fixture_params_repeat1, 2, 0, 0),
  [492] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_params_repeat1, 2, 0, 0), SHIFT_REPEAT(144),
  [495] = {.entry = {.count = 1, .reusable = true}}, SHIFT(193),
  [497] = {.entry = {.count = 1, .reusable = true}}, SHIFT(189),
  [499] = {.entry = {.count = 1, .reusable = true}}, SHIFT(93),
  [501] = {.entry = {.count = 1, .reusable = true}}, SHIFT(218),
  [503] = {.entry = {.count = 1, .reusable = true}}, SHIFT(143),
  [505] = {.entry = {.count = 1, .reusable = true}}, SHIFT(210),
  [507] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup_statement, 1, 0, 0),
  [509] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_statement, 1, 0, 0),
  [511] = {.entry = {.count = 1, .reusable = true}}, SHIFT(179),
  [513] = {.entry = {.count = 1, .reusable = true}}, SHIFT(127),
  [515] = {.entry = {.count = 1, .reusable = true}}, SHIFT(175),
  [517] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 2, 0, 0),
  [519] = {.entry = {.count = 1, .reusable = true}}, SHIFT(119),
  [521] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 4, 0, 0),
  [523] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 4, 0, 0),
  [525] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [527] = {.entry = {.count = 1, .reusable = true}}, SHIFT(120),
  [529] = {.entry = {.count = 1, .reusable = true}}, SHIFT(181),
  [531] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_chart_params_repeat1, 2, 0, 0),
  [533] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_chart_params_repeat1, 2, 0, 0), SHIFT_REPEAT(58),
  [536] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 1, 0, 0),
  [538] = {.entry = {.count = 1, .reusable = true}}, SHIFT(135),
  [540] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_argument_list_repeat1, 2, 0, 0),
  [542] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_argument_list_repeat1, 2, 0, 0), SHIFT_REPEAT(172),
  [545] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite_body, 2, 0, 0),
  [547] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [549] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_param, 3, 0, 4),
  [551] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [553] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [555] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_language_tag, 1, 0, 0),
  [557] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [559] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument, 3, 0, 4),
  [561] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_directive, 6, 0, 7),
  [563] = {.entry = {.count = 1, .reusable = true}}, SHIFT(174),
  [565] = {.entry = {.count = 1, .reusable = true}}, SHIFT(112),
  [567] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_directive, 5, 0, 7),
  [569] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite, 3, 0, 1),
  [571] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite_body, 3, 0, 0),
  [573] = {.entry = {.count = 1, .reusable = true}}, SHIFT(85),
  [575] = {.entry = {.count = 1, .reusable = true}}, SHIFT(192),
  [577] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [579] = {.entry = {.count = 1, .reusable = true}}, SHIFT(199),
  [581] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [583] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_param, 3, 0, 6),
  [585] = {.entry = {.count = 1, .reusable = true}}, SHIFT(194),
  [587] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 2, 0, 0),
  [589] = {.entry = {.count = 1, .reusable = true}}, SHIFT(121),
  [591] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property_name, 1, 0, 0),
  [593] = {.entry = {.count = 1, .reusable = true}}, SHIFT(151),
  [595] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 4, 0, 0),
  [597] = {.entry = {.count = 1, .reusable = true}}, SHIFT(97),
  [599] = {.entry = {.count = 1, .reusable = true}}, SHIFT(123),
  [601] = {.entry = {.count = 1, .reusable = true}}, SHIFT(173),
  [603] = {.entry = {.count = 1, .reusable = true}}, SHIFT(139),
  [605] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [607] = {.entry = {.count = 1, .reusable = true}}, SHIFT(183),
  [609] = {.entry = {.count = 1, .reusable = true}}, SHIFT(124),
  [611] = {.entry = {.count = 1, .reusable = true}}, SHIFT(213),
  [613] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 5, 0, 0),
  [615] = {.entry = {.count = 1, .reusable = true}}, SHIFT(125),
  [617] = {.entry = {.count = 1, .reusable = true}}, SHIFT(148),
  [619] = {.entry = {.count = 1, .reusable = true}}, SHIFT(88),
  [621] = {.entry = {.count = 1, .reusable = true}}, SHIFT(182),
  [623] = {.entry = {.count = 1, .reusable = true}}, SHIFT(92),
  [625] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_param_name, 1, 0, 0),
  [627] = {.entry = {.count = 1, .reusable = true}}, SHIFT(162),
  [629] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [631] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [633] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [635] = {.entry = {.count = 1, .reusable = true}}, SHIFT(99),
  [637] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [639] = {.entry = {.count = 1, .reusable = true}}, SHIFT(186),
  [641] = {.entry = {.count = 1, .reusable = true}}, SHIFT(110),
  [643] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_args, 3, 0, 0),
  [645] = {.entry = {.count = 1, .reusable = true}}, SHIFT(159),
  [647] = {.entry = {.count = 1, .reusable = true}}, SHIFT(91),
  [649] = {.entry = {.count = 1, .reusable = true}}, SHIFT(130),
  [651] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [653] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [655] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_function_name, 1, 0, 0),
  [657] = {.entry = {.count = 1, .reusable = true}}, SHIFT(161),
  [659] = {.entry = {.count = 1, .reusable = true}}, SHIFT(136),
  [661] = {.entry = {.count = 1, .reusable = true}}, SHIFT(203),
  [663] = {.entry = {.count = 1, .reusable = true}}, SHIFT(109),
  [665] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [667] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [669] = {.entry = {.count = 1, .reusable = true}}, SHIFT(154),
  [671] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 3, 0, 0),
  [673] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [675] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
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
