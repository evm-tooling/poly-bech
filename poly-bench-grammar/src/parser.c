#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 221
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 165
#define ALIAS_COUNT 0
#define TOKEN_COUNT 88
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
  anon_sym_drawSpeedupChart = 35,
  anon_sym_drawTable = 36,
  anon_sym_title = 37,
  anon_sym_description = 38,
  anon_sym_output = 39,
  anon_sym_sortBy = 40,
  anon_sym_sortOrder = 41,
  anon_sym_baselineBenchmark = 42,
  anon_sym_baseline = 43,
  anon_sym_filterWinner = 44,
  anon_sym_theme = 45,
  anon_sym_width = 46,
  anon_sym_rowCount = 47,
  anon_sym_height = 48,
  anon_sym_limit = 49,
  anon_sym_minSpeedup = 50,
  anon_sym_includeBenchmarks = 51,
  anon_sym_excludeBenchmarks = 52,
  anon_sym_iterations = 53,
  anon_sym_warmup = 54,
  anon_sym_timeout = 55,
  anon_sym_requires = 56,
  anon_sym_order = 57,
  anon_sym_mode = 58,
  anon_sym_targetTime = 59,
  anon_sym_sink = 60,
  anon_sym_outlierDetection = 61,
  anon_sym_cvThreshold = 62,
  anon_sym_count = 63,
  anon_sym_memory = 64,
  anon_sym_go = 65,
  anon_sym_ts = 66,
  anon_sym_typescript = 67,
  anon_sym_rust = 68,
  anon_sym_python = 69,
  sym_inline_code = 70,
  anon_sym_DQUOTE = 71,
  anon_sym_SQUOTE = 72,
  aux_sym_string_content_token1 = 73,
  aux_sym_single_string_content_token1 = 74,
  sym_escape_sequence = 75,
  sym_number = 76,
  sym_float = 77,
  anon_sym_ms = 78,
  anon_sym_s = 79,
  anon_sym_m = 80,
  anon_sym_true = 81,
  anon_sym_false = 82,
  anon_sym_LBRACK = 83,
  anon_sym_RBRACK = 84,
  sym_comment = 85,
  sym_embedded_code = 86,
  sym__embedded_code_start = 87,
  sym_source_file = 88,
  sym_use_statement = 89,
  sym_global_setup = 90,
  sym_global_setup_body = 91,
  sym_global_setup_statement = 92,
  sym_anvil_call = 93,
  sym_anvil_args = 94,
  sym_function_call = 95,
  sym_argument_list = 96,
  sym_argument = 97,
  sym_suite = 98,
  sym_suite_body = 99,
  sym__suite_item = 100,
  sym_setup_block = 101,
  sym_setup_body = 102,
  sym__setup_section = 103,
  sym_import_section = 104,
  sym_declare_section = 105,
  sym_init_section = 106,
  sym_helpers_section = 107,
  sym_fixture = 108,
  sym_fixture_params = 109,
  sym_fixture_param = 110,
  sym_fixture_body = 111,
  sym__fixture_item = 112,
  sym_hex_property = 113,
  sym_shape_property = 114,
  sym_file_ref = 115,
  sym_benchmark = 116,
  sym_benchmark_body = 117,
  sym__benchmark_item = 118,
  sym_tags_property = 119,
  sym_skip_hook = 120,
  sym_validate_hook = 121,
  sym_before_hook = 122,
  sym_after_hook = 123,
  sym_each_hook = 124,
  sym_hook_flat = 125,
  sym_hook_grouped = 126,
  sym_after_block = 127,
  sym_after_body = 128,
  sym_chart_directive = 129,
  sym_chart_function_name = 130,
  sym_chart_params = 131,
  sym_chart_param = 132,
  sym_chart_param_name = 133,
  sym__chart_value = 134,
  sym_property = 135,
  sym_property_name = 136,
  sym__value = 137,
  sym_language_implementation = 138,
  sym_language_tag = 139,
  sym__code_or_inline = 140,
  sym_code_block = 141,
  sym_paren_code_block = 142,
  sym_string = 143,
  sym_string_content = 144,
  sym_single_string_content = 145,
  sym_duration = 146,
  sym_duration_unit = 147,
  sym_boolean = 148,
  sym_string_array = 149,
  aux_sym_source_file_repeat1 = 150,
  aux_sym_source_file_repeat2 = 151,
  aux_sym_global_setup_body_repeat1 = 152,
  aux_sym_argument_list_repeat1 = 153,
  aux_sym_suite_body_repeat1 = 154,
  aux_sym_setup_body_repeat1 = 155,
  aux_sym_fixture_params_repeat1 = 156,
  aux_sym_fixture_body_repeat1 = 157,
  aux_sym_benchmark_body_repeat1 = 158,
  aux_sym_hook_grouped_repeat1 = 159,
  aux_sym_after_body_repeat1 = 160,
  aux_sym_chart_params_repeat1 = 161,
  aux_sym_string_content_repeat1 = 162,
  aux_sym_single_string_content_repeat1 = 163,
  aux_sym_string_array_repeat1 = 164,
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
  [29] = 20,
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
  [79] = 20,
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
      END_STATE();
    case 184:
      if (lookahead == 'i') ADVANCE(226);
      END_STATE();
    case 185:
      ACCEPT_TOKEN(anon_sym_count);
      END_STATE();
    case 186:
      if (lookahead == 'e') ADVANCE(227);
      END_STATE();
    case 187:
      if (lookahead == 'r') ADVANCE(228);
      END_STATE();
    case 188:
      if (lookahead == 'i') ADVANCE(229);
      END_STATE();
    case 189:
      if (lookahead == 'p') ADVANCE(230);
      END_STATE();
    case 190:
      if (lookahead == 'a') ADVANCE(231);
      END_STATE();
    case 191:
      if (lookahead == 'd') ADVANCE(232);
      END_STATE();
    case 192:
      ACCEPT_TOKEN(anon_sym_false);
      END_STATE();
    case 193:
      if (lookahead == 'r') ADVANCE(233);
      END_STATE();
    case 194:
      if (lookahead == 'r') ADVANCE(234);
      END_STATE();
    case 195:
      if (lookahead == 'l') ADVANCE(235);
      END_STATE();
    case 196:
      if (lookahead == 't') ADVANCE(236);
      END_STATE();
    case 197:
      if (lookahead == 'r') ADVANCE(237);
      END_STATE();
    case 198:
      if (lookahead == 't') ADVANCE(238);
      END_STATE();
    case 199:
      if (lookahead == 'd') ADVANCE(239);
      END_STATE();
    case 200:
      if (lookahead == 't') ADVANCE(240);
      END_STATE();
    case 201:
      ACCEPT_TOKEN(anon_sym_limit);
      END_STATE();
    case 202:
      if (lookahead == 'y') ADVANCE(241);
      END_STATE();
    case 203:
      if (lookahead == 'e') ADVANCE(242);
      END_STATE();
    case 204:
      ACCEPT_TOKEN(anon_sym_order);
      END_STATE();
    case 205:
      if (lookahead == 'e') ADVANCE(243);
      END_STATE();
    case 206:
      if (lookahead == 't') ADVANCE(244);
      END_STATE();
    case 207:
      if (lookahead == 'n') ADVANCE(245);
      END_STATE();
    case 208:
      if (lookahead == 'r') ADVANCE(246);
      END_STATE();
    case 209:
      if (lookahead == 'u') ADVANCE(247);
      END_STATE();
    case 210:
      ACCEPT_TOKEN(anon_sym_setup);
      END_STATE();
    case 211:
      ACCEPT_TOKEN(anon_sym_shape);
      END_STATE();
    case 212:
      if (lookahead == 'y') ADVANCE(248);
      END_STATE();
    case 213:
      if (lookahead == 'r') ADVANCE(249);
      END_STATE();
    case 214:
      if (lookahead == 'A') ADVANCE(250);
      END_STATE();
    case 215:
      ACCEPT_TOKEN(anon_sym_suite);
      END_STATE();
    case 216:
      if (lookahead == 't') ADVANCE(251);
      END_STATE();
    case 217:
      ACCEPT_TOKEN(anon_sym_theme);
      END_STATE();
    case 218:
      if (lookahead == 'u') ADVANCE(252);
      END_STATE();
    case 219:
      ACCEPT_TOKEN(anon_sym_title);
      END_STATE();
    case 220:
      if (lookahead == 'c') ADVANCE(253);
      END_STATE();
    case 221:
      if (lookahead == 'a') ADVANCE(254);
      END_STATE();
    case 222:
      if (lookahead == 'p') ADVANCE(255);
      END_STATE();
    case 223:
      ACCEPT_TOKEN(anon_sym_width);
      END_STATE();
    case 224:
      if (lookahead == 'n') ADVANCE(256);
      END_STATE();
    case 225:
      ACCEPT_TOKEN(anon_sym_before);
      END_STATE();
    case 226:
      if (lookahead == 'n') ADVANCE(257);
      END_STATE();
    case 227:
      if (lookahead == 's') ADVANCE(258);
      END_STATE();
    case 228:
      if (lookahead == 'e') ADVANCE(259);
      END_STATE();
    case 229:
      if (lookahead == 'p') ADVANCE(260);
      END_STATE();
    case 230:
      if (lookahead == 'e') ADVANCE(261);
      END_STATE();
    case 231:
      if (lookahead == 'b') ADVANCE(262);
      END_STATE();
    case 232:
      if (lookahead == 'e') ADVANCE(263);
      END_STATE();
    case 233:
      if (lookahead == 'W') ADVANCE(264);
      END_STATE();
    case 234:
      if (lookahead == 'e') ADVANCE(265);
      END_STATE();
    case 235:
      if (lookahead == 'S') ADVANCE(266);
      END_STATE();
    case 236:
      ACCEPT_TOKEN(anon_sym_height);
      END_STATE();
    case 237:
      if (lookahead == 's') ADVANCE(267);
      END_STATE();
    case 238:
      ACCEPT_TOKEN(anon_sym_import);
      END_STATE();
    case 239:
      if (lookahead == 'e') ADVANCE(268);
      END_STATE();
    case 240:
      if (lookahead == 'i') ADVANCE(269);
      END_STATE();
    case 241:
      ACCEPT_TOKEN(anon_sym_memory);
      END_STATE();
    case 242:
      if (lookahead == 'e') ADVANCE(270);
      END_STATE();
    case 243:
      if (lookahead == 'r') ADVANCE(271);
      END_STATE();
    case 244:
      ACCEPT_TOKEN(anon_sym_output);
      END_STATE();
    case 245:
      ACCEPT_TOKEN(anon_sym_python);
      END_STATE();
    case 246:
      if (lookahead == 'e') ADVANCE(272);
      END_STATE();
    case 247:
      if (lookahead == 'n') ADVANCE(273);
      END_STATE();
    case 248:
      ACCEPT_TOKEN(anon_sym_sortBy);
      END_STATE();
    case 249:
      if (lookahead == 'd') ADVANCE(274);
      END_STATE();
    case 250:
      if (lookahead == 'n') ADVANCE(275);
      END_STATE();
    case 251:
      if (lookahead == 'T') ADVANCE(276);
      END_STATE();
    case 252:
      if (lookahead == 't') ADVANCE(277);
      END_STATE();
    case 253:
      if (lookahead == 'r') ADVANCE(278);
      END_STATE();
    case 254:
      if (lookahead == 't') ADVANCE(279);
      END_STATE();
    case 255:
      ACCEPT_TOKEN(anon_sym_warmup);
      END_STATE();
    case 256:
      if (lookahead == 'e') ADVANCE(280);
      END_STATE();
    case 257:
      if (lookahead == 'g') ADVANCE(281);
      END_STATE();
    case 258:
      if (lookahead == 'h') ADVANCE(282);
      END_STATE();
    case 259:
      ACCEPT_TOKEN(anon_sym_declare);
      END_STATE();
    case 260:
      if (lookahead == 't') ADVANCE(283);
      END_STATE();
    case 261:
      if (lookahead == 'e') ADVANCE(284);
      END_STATE();
    case 262:
      if (lookahead == 'l') ADVANCE(285);
      END_STATE();
    case 263:
      if (lookahead == 'B') ADVANCE(286);
      END_STATE();
    case 264:
      if (lookahead == 'i') ADVANCE(287);
      END_STATE();
    case 265:
      ACCEPT_TOKEN(anon_sym_fixture);
      END_STATE();
    case 266:
      if (lookahead == 'e') ADVANCE(288);
      END_STATE();
    case 267:
      ACCEPT_TOKEN(anon_sym_helpers);
      END_STATE();
    case 268:
      if (lookahead == 'B') ADVANCE(289);
      END_STATE();
    case 269:
      if (lookahead == 'o') ADVANCE(290);
      END_STATE();
    case 270:
      if (lookahead == 'd') ADVANCE(291);
      END_STATE();
    case 271:
      if (lookahead == 'D') ADVANCE(292);
      END_STATE();
    case 272:
      if (lookahead == 's') ADVANCE(293);
      END_STATE();
    case 273:
      if (lookahead == 't') ADVANCE(294);
      END_STATE();
    case 274:
      if (lookahead == 'e') ADVANCE(295);
      END_STATE();
    case 275:
      if (lookahead == 'v') ADVANCE(296);
      END_STATE();
    case 276:
      if (lookahead == 'i') ADVANCE(297);
      END_STATE();
    case 277:
      ACCEPT_TOKEN(anon_sym_timeout);
      END_STATE();
    case 278:
      if (lookahead == 'i') ADVANCE(298);
      END_STATE();
    case 279:
      if (lookahead == 'e') ADVANCE(299);
      END_STATE();
    case 280:
      ACCEPT_TOKEN(anon_sym_baseline);
      if (lookahead == 'B') ADVANCE(300);
      END_STATE();
    case 281:
      ACCEPT_TOKEN(anon_sym_charting);
      END_STATE();
    case 282:
      if (lookahead == 'o') ADVANCE(301);
      END_STATE();
    case 283:
      if (lookahead == 'i') ADVANCE(302);
      END_STATE();
    case 284:
      if (lookahead == 'd') ADVANCE(303);
      END_STATE();
    case 285:
      if (lookahead == 'e') ADVANCE(304);
      END_STATE();
    case 286:
      if (lookahead == 'e') ADVANCE(305);
      END_STATE();
    case 287:
      if (lookahead == 'n') ADVANCE(306);
      END_STATE();
    case 288:
      if (lookahead == 't') ADVANCE(307);
      END_STATE();
    case 289:
      if (lookahead == 'e') ADVANCE(308);
      END_STATE();
    case 290:
      if (lookahead == 'n') ADVANCE(309);
      END_STATE();
    case 291:
      if (lookahead == 'u') ADVANCE(310);
      END_STATE();
    case 292:
      if (lookahead == 'e') ADVANCE(311);
      END_STATE();
    case 293:
      ACCEPT_TOKEN(anon_sym_requires);
      END_STATE();
    case 294:
      ACCEPT_TOKEN(anon_sym_rowCount);
      END_STATE();
    case 295:
      if (lookahead == 'r') ADVANCE(312);
      END_STATE();
    case 296:
      if (lookahead == 'i') ADVANCE(313);
      END_STATE();
    case 297:
      if (lookahead == 'm') ADVANCE(314);
      END_STATE();
    case 298:
      if (lookahead == 'p') ADVANCE(315);
      END_STATE();
    case 299:
      ACCEPT_TOKEN(anon_sym_validate);
      END_STATE();
    case 300:
      if (lookahead == 'e') ADVANCE(316);
      END_STATE();
    case 301:
      if (lookahead == 'l') ADVANCE(317);
      END_STATE();
    case 302:
      if (lookahead == 'o') ADVANCE(318);
      END_STATE();
    case 303:
      if (lookahead == 'u') ADVANCE(319);
      END_STATE();
    case 304:
      ACCEPT_TOKEN(anon_sym_drawTable);
      END_STATE();
    case 305:
      if (lookahead == 'n') ADVANCE(320);
      END_STATE();
    case 306:
      if (lookahead == 'n') ADVANCE(321);
      END_STATE();
    case 307:
      if (lookahead == 'u') ADVANCE(322);
      END_STATE();
    case 308:
      if (lookahead == 'n') ADVANCE(323);
      END_STATE();
    case 309:
      if (lookahead == 's') ADVANCE(324);
      END_STATE();
    case 310:
      if (lookahead == 'p') ADVANCE(325);
      END_STATE();
    case 311:
      if (lookahead == 't') ADVANCE(326);
      END_STATE();
    case 312:
      ACCEPT_TOKEN(anon_sym_sortOrder);
      END_STATE();
    case 313:
      if (lookahead == 'l') ADVANCE(327);
      END_STATE();
    case 314:
      if (lookahead == 'e') ADVANCE(328);
      END_STATE();
    case 315:
      if (lookahead == 't') ADVANCE(329);
      END_STATE();
    case 316:
      if (lookahead == 'n') ADVANCE(330);
      END_STATE();
    case 317:
      if (lookahead == 'd') ADVANCE(331);
      END_STATE();
    case 318:
      if (lookahead == 'n') ADVANCE(332);
      END_STATE();
    case 319:
      if (lookahead == 'p') ADVANCE(333);
      END_STATE();
    case 320:
      if (lookahead == 'c') ADVANCE(334);
      END_STATE();
    case 321:
      if (lookahead == 'e') ADVANCE(335);
      END_STATE();
    case 322:
      if (lookahead == 'p') ADVANCE(336);
      END_STATE();
    case 323:
      if (lookahead == 'c') ADVANCE(337);
      END_STATE();
    case 324:
      ACCEPT_TOKEN(anon_sym_iterations);
      END_STATE();
    case 325:
      ACCEPT_TOKEN(anon_sym_minSpeedup);
      END_STATE();
    case 326:
      if (lookahead == 'e') ADVANCE(338);
      END_STATE();
    case 327:
      ACCEPT_TOKEN(anon_sym_spawnAnvil);
      END_STATE();
    case 328:
      ACCEPT_TOKEN(anon_sym_targetTime);
      END_STATE();
    case 329:
      ACCEPT_TOKEN(anon_sym_typescript);
      END_STATE();
    case 330:
      if (lookahead == 'c') ADVANCE(339);
      END_STATE();
    case 331:
      ACCEPT_TOKEN(anon_sym_cvThreshold);
      END_STATE();
    case 332:
      ACCEPT_TOKEN(anon_sym_description);
      END_STATE();
    case 333:
      if (lookahead == 'C') ADVANCE(340);
      END_STATE();
    case 334:
      if (lookahead == 'h') ADVANCE(341);
      END_STATE();
    case 335:
      if (lookahead == 'r') ADVANCE(342);
      END_STATE();
    case 336:
      ACCEPT_TOKEN(anon_sym_globalSetup);
      END_STATE();
    case 337:
      if (lookahead == 'h') ADVANCE(343);
      END_STATE();
    case 338:
      if (lookahead == 'c') ADVANCE(344);
      END_STATE();
    case 339:
      if (lookahead == 'h') ADVANCE(345);
      END_STATE();
    case 340:
      if (lookahead == 'h') ADVANCE(346);
      END_STATE();
    case 341:
      if (lookahead == 'm') ADVANCE(347);
      END_STATE();
    case 342:
      ACCEPT_TOKEN(anon_sym_filterWinner);
      END_STATE();
    case 343:
      if (lookahead == 'm') ADVANCE(348);
      END_STATE();
    case 344:
      if (lookahead == 't') ADVANCE(349);
      END_STATE();
    case 345:
      if (lookahead == 'm') ADVANCE(350);
      END_STATE();
    case 346:
      if (lookahead == 'a') ADVANCE(351);
      END_STATE();
    case 347:
      if (lookahead == 'a') ADVANCE(352);
      END_STATE();
    case 348:
      if (lookahead == 'a') ADVANCE(353);
      END_STATE();
    case 349:
      if (lookahead == 'i') ADVANCE(354);
      END_STATE();
    case 350:
      if (lookahead == 'a') ADVANCE(355);
      END_STATE();
    case 351:
      if (lookahead == 'r') ADVANCE(356);
      END_STATE();
    case 352:
      if (lookahead == 'r') ADVANCE(357);
      END_STATE();
    case 353:
      if (lookahead == 'r') ADVANCE(358);
      END_STATE();
    case 354:
      if (lookahead == 'o') ADVANCE(359);
      END_STATE();
    case 355:
      if (lookahead == 'r') ADVANCE(360);
      END_STATE();
    case 356:
      if (lookahead == 't') ADVANCE(361);
      END_STATE();
    case 357:
      if (lookahead == 'k') ADVANCE(362);
      END_STATE();
    case 358:
      if (lookahead == 'k') ADVANCE(363);
      END_STATE();
    case 359:
      if (lookahead == 'n') ADVANCE(364);
      END_STATE();
    case 360:
      if (lookahead == 'k') ADVANCE(365);
      END_STATE();
    case 361:
      ACCEPT_TOKEN(anon_sym_drawSpeedupChart);
      END_STATE();
    case 362:
      if (lookahead == 's') ADVANCE(366);
      END_STATE();
    case 363:
      if (lookahead == 's') ADVANCE(367);
      END_STATE();
    case 364:
      ACCEPT_TOKEN(anon_sym_outlierDetection);
      END_STATE();
    case 365:
      ACCEPT_TOKEN(anon_sym_baselineBenchmark);
      END_STATE();
    case 366:
      ACCEPT_TOKEN(anon_sym_excludeBenchmarks);
      END_STATE();
    case 367:
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
  [198] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(59), 35,
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
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
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
  [239] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(61), 35,
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
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
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
  [280] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(63), 34,
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
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
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
  [320] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(65), 34,
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
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
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
  [360] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(67), 34,
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
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
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
  [400] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(69), 34,
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
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
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
  [440] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(71), 34,
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
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
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
  [480] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(73), 34,
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
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
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
  [520] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(75), 34,
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
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
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
  [560] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(77), 33,
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
  [599] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(79), 33,
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
  [638] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(81), 32,
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
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
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
  [676] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(83), 1,
      anon_sym_RBRACE,
    ACTIONS(85), 1,
      anon_sym_hex,
    ACTIONS(87), 1,
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
  [726] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(89), 1,
      anon_sym_RBRACE,
    ACTIONS(91), 1,
      anon_sym_hex,
    ACTIONS(94), 1,
      anon_sym_shape,
    STATE(200), 1,
      sym_language_tag,
    STATE(219), 1,
      sym_property_name,
    ACTIONS(100), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
    STATE(18), 6,
      sym__fixture_item,
      sym_hex_property,
      sym_shape_property,
      sym_property,
      sym_language_implementation,
      aux_sym_fixture_body_repeat1,
    ACTIONS(97), 14,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
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
  [776] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(85), 1,
      anon_sym_hex,
    ACTIONS(87), 1,
      anon_sym_shape,
    ACTIONS(103), 1,
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
    STATE(18), 6,
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
  [826] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(107), 1,
      anon_sym_ms,
    STATE(8), 1,
      sym_duration_unit,
    ACTIONS(109), 2,
      anon_sym_s,
      anon_sym_m,
    ACTIONS(105), 26,
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
  [868] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(111), 1,
      anon_sym_globalSetup,
    ACTIONS(114), 1,
      anon_sym_RBRACE,
    ACTIONS(116), 1,
      anon_sym_setup,
    ACTIONS(119), 1,
      anon_sym_fixture,
    ACTIONS(122), 1,
      anon_sym_bench,
    ACTIONS(125), 1,
      anon_sym_after,
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
    ACTIONS(128), 14,
      anon_sym_description,
      anon_sym_baseline,
      anon_sym_iterations,
      anon_sym_warmup,
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
  [919] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      anon_sym_globalSetup,
    ACTIONS(131), 1,
      anon_sym_RBRACE,
    ACTIONS(133), 1,
      anon_sym_setup,
    ACTIONS(135), 1,
      anon_sym_fixture,
    ACTIONS(137), 1,
      anon_sym_bench,
    ACTIONS(139), 1,
      anon_sym_after,
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
  [970] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      anon_sym_globalSetup,
    ACTIONS(133), 1,
      anon_sym_setup,
    ACTIONS(135), 1,
      anon_sym_fixture,
    ACTIONS(137), 1,
      anon_sym_bench,
    ACTIONS(139), 1,
      anon_sym_after,
    ACTIONS(141), 1,
      anon_sym_RBRACE,
    STATE(197), 1,
      sym_property_name,
    STATE(22), 8,
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
  [1021] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(143), 28,
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
  [1055] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(145), 26,
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
  [1087] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(147), 26,
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
  [1119] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(149), 26,
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
  [1151] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(151), 26,
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
  [1183] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(107), 1,
      anon_sym_ms,
    STATE(8), 1,
      sym_duration_unit,
    ACTIONS(109), 2,
      anon_sym_s,
      anon_sym_m,
    ACTIONS(105), 22,
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
  [1221] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(153), 26,
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
  [1253] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(155), 26,
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
  [1285] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(157), 26,
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
  [1317] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(159), 26,
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
  [1349] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(161), 26,
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
  [1381] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(107), 1,
      anon_sym_ms,
    STATE(8), 1,
      sym_duration_unit,
    ACTIONS(109), 2,
      anon_sym_s,
      anon_sym_m,
    ACTIONS(105), 20,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
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
  [1417] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(163), 22,
      ts_builtin_sym_end,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_suite,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
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
  [1445] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(165), 22,
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
  [1473] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(167), 22,
      ts_builtin_sym_end,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_suite,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
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
  [1501] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(169), 22,
      ts_builtin_sym_end,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_suite,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
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
  [1529] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(171), 22,
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
  [1557] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(173), 22,
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
  [1585] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(175), 20,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
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
  [1611] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(177), 20,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
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
  [1637] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(179), 20,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
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
  [1663] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(181), 20,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
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
  [1689] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(183), 20,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
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
  [1715] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(185), 20,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
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
  [1741] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(187), 1,
      anon_sym_RPAREN,
    ACTIONS(191), 1,
      anon_sym_baseline,
    STATE(115), 1,
      sym_chart_param,
    STATE(196), 1,
      sym_chart_params,
    STATE(198), 1,
      sym_chart_param_name,
    ACTIONS(189), 15,
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
  [1777] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(193), 20,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
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
  [1803] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(195), 20,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
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
  [1829] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(197), 20,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
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
  [1855] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(199), 20,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
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
  [1881] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(201), 20,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
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
  [1907] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(203), 20,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
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
  [1933] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(205), 20,
      anon_sym_globalSetup,
      anon_sym_RBRACE,
      anon_sym_setup,
      anon_sym_fixture,
      anon_sym_bench,
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
  [1959] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(191), 1,
      anon_sym_baseline,
    ACTIONS(207), 1,
      anon_sym_RPAREN,
    STATE(155), 1,
      sym_chart_param,
    STATE(198), 1,
      sym_chart_param_name,
    ACTIONS(189), 15,
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
  [1992] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(191), 1,
      anon_sym_baseline,
    ACTIONS(209), 1,
      anon_sym_RPAREN,
    STATE(155), 1,
      sym_chart_param,
    STATE(198), 1,
      sym_chart_param_name,
    ACTIONS(189), 15,
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
  [2025] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(191), 1,
      anon_sym_baseline,
    STATE(155), 1,
      sym_chart_param,
    STATE(198), 1,
      sym_chart_param_name,
    ACTIONS(189), 15,
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
  [2055] = 9,
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
    STATE(16), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2088] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(213), 1,
      anon_sym_DQUOTE,
    ACTIONS(215), 1,
      anon_sym_SQUOTE,
    ACTIONS(223), 1,
      anon_sym_LBRACK,
    ACTIONS(225), 1,
      sym_identifier,
    ACTIONS(227), 1,
      sym_number,
    ACTIONS(229), 1,
      sym_float,
    ACTIONS(221), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(158), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2121] = 9,
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
    ACTIONS(231), 1,
      sym_number,
    ACTIONS(221), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(16), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2154] = 9,
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
    STATE(16), 5,
      sym__value,
      sym_string,
      sym_duration,
      sym_boolean,
      sym_string_array,
  [2187] = 8,
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
    STATE(65), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [2217] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(247), 1,
      anon_sym_RBRACE,
    ACTIONS(249), 1,
      anon_sym_import,
    ACTIONS(252), 1,
      anon_sym_declare,
    ACTIONS(255), 1,
      anon_sym_async,
    ACTIONS(258), 1,
      anon_sym_init,
    ACTIONS(261), 1,
      anon_sym_helpers,
    STATE(64), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [2247] = 8,
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
    ACTIONS(264), 1,
      anon_sym_RBRACE,
    STATE(64), 6,
      sym__setup_section,
      sym_import_section,
      sym_declare_section,
      sym_init_section,
      sym_helpers_section,
      aux_sym_setup_body_repeat1,
  [2277] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(213), 1,
      anon_sym_DQUOTE,
    ACTIONS(215), 1,
      anon_sym_SQUOTE,
    ACTIONS(223), 1,
      anon_sym_LBRACK,
    ACTIONS(266), 1,
      sym_number,
    ACTIONS(268), 1,
      sym_float,
    ACTIONS(270), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(150), 4,
      sym__chart_value,
      sym_string,
      sym_boolean,
      sym_string_array,
  [2306] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(272), 1,
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
  [2327] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(274), 1,
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
  [2348] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(272), 1,
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
  [2369] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(276), 1,
      anon_sym_RBRACE,
    STATE(200), 1,
      sym_language_tag,
    STATE(70), 2,
      sym_language_implementation,
      aux_sym_hook_grouped_repeat1,
    ACTIONS(278), 5,
      anon_sym_go,
      anon_sym_ts,
      anon_sym_typescript,
      anon_sym_rust,
      anon_sym_python,
  [2390] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(272), 1,
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
  [2411] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(281), 1,
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
  [2432] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(7), 1,
      anon_sym_use,
    ACTIONS(9), 1,
      anon_sym_globalSetup,
    ACTIONS(11), 1,
      anon_sym_suite,
    ACTIONS(283), 1,
      ts_builtin_sym_end,
    STATE(101), 1,
      sym_global_setup,
    STATE(84), 2,
      sym_use_statement,
      aux_sym_source_file_repeat1,
    STATE(100), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [2459] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(272), 1,
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
  [2480] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(272), 1,
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
  [2501] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(285), 1,
      sym_identifier,
    ACTIONS(287), 1,
      anon_sym_RBRACE,
    ACTIONS(289), 1,
      anon_sym_anvil,
    STATE(77), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(131), 2,
      sym_anvil_call,
      sym_function_call,
  [2522] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(285), 1,
      sym_identifier,
    ACTIONS(289), 1,
      anon_sym_anvil,
    ACTIONS(291), 1,
      anon_sym_RBRACE,
    STATE(78), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(131), 2,
      sym_anvil_call,
      sym_function_call,
  [2543] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(293), 1,
      sym_identifier,
    ACTIONS(296), 1,
      anon_sym_RBRACE,
    ACTIONS(298), 1,
      anon_sym_anvil,
    STATE(78), 2,
      sym_global_setup_statement,
      aux_sym_global_setup_body_repeat1,
    STATE(131), 2,
      sym_anvil_call,
      sym_function_call,
  [2564] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(109), 1,
      anon_sym_m,
    STATE(8), 1,
      sym_duration_unit,
    ACTIONS(105), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
    ACTIONS(107), 2,
      anon_sym_ms,
      anon_sym_s,
  [2582] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(301), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [2594] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(303), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [2606] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(305), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [2618] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(307), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [2630] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(311), 1,
      anon_sym_use,
    STATE(84), 2,
      sym_use_statement,
      aux_sym_source_file_repeat1,
    ACTIONS(309), 3,
      ts_builtin_sym_end,
      anon_sym_globalSetup,
      anon_sym_suite,
  [2646] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(314), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [2658] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(316), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [2670] = 3,
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
  [2684] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(318), 6,
      anon_sym_RBRACE,
      anon_sym_import,
      anon_sym_declare,
      anon_sym_async,
      anon_sym_init,
      anon_sym_helpers,
  [2696] = 5,
    ACTIONS(320), 1,
      anon_sym_DQUOTE,
    ACTIONS(324), 1,
      sym_comment,
    STATE(111), 1,
      aux_sym_string_content_repeat1,
    STATE(185), 1,
      sym_string_content,
    ACTIONS(322), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [2713] = 5,
    ACTIONS(320), 1,
      anon_sym_SQUOTE,
    ACTIONS(324), 1,
      sym_comment,
    STATE(114), 1,
      aux_sym_single_string_content_repeat1,
    STATE(184), 1,
      sym_single_string_content,
    ACTIONS(326), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [2730] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(213), 1,
      anon_sym_DQUOTE,
    ACTIONS(215), 1,
      anon_sym_SQUOTE,
    ACTIONS(328), 1,
      anon_sym_ATfile,
    STATE(41), 2,
      sym_file_ref,
      sym_string,
  [2747] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(330), 1,
      sym_identifier,
    ACTIONS(332), 1,
      anon_sym_RPAREN,
    STATE(141), 1,
      sym_argument,
    STATE(190), 1,
      sym_argument_list,
  [2763] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(213), 1,
      anon_sym_DQUOTE,
    ACTIONS(215), 1,
      anon_sym_SQUOTE,
    ACTIONS(334), 1,
      anon_sym_RBRACK,
    STATE(166), 1,
      sym_string,
  [2779] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_suite,
    ACTIONS(283), 1,
      ts_builtin_sym_end,
    STATE(100), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [2793] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_suite,
    ACTIONS(283), 1,
      ts_builtin_sym_end,
    STATE(103), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [2807] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(336), 1,
      anon_sym_RBRACE,
    ACTIONS(338), 1,
      anon_sym_charting,
    STATE(106), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [2821] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(340), 1,
      anon_sym_LBRACE,
    ACTIONS(342), 1,
      anon_sym_LPAREN,
    STATE(45), 1,
      sym_fixture_body,
    STATE(160), 1,
      sym_fixture_params,
  [2837] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(344), 1,
      anon_sym_LBRACE,
    ACTIONS(346), 1,
      anon_sym_LPAREN,
    STATE(81), 2,
      sym_code_block,
      sym_paren_code_block,
  [2851] = 4,
    ACTIONS(324), 1,
      sym_comment,
    ACTIONS(348), 1,
      anon_sym_LBRACE,
    ACTIONS(350), 1,
      sym_inline_code,
    STATE(24), 2,
      sym__code_or_inline,
      sym_code_block,
  [2865] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_suite,
    ACTIONS(352), 1,
      ts_builtin_sym_end,
    STATE(103), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [2879] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_suite,
    ACTIONS(352), 1,
      ts_builtin_sym_end,
    STATE(107), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [2893] = 4,
    ACTIONS(324), 1,
      sym_comment,
    ACTIONS(354), 1,
      anon_sym_DQUOTE,
    STATE(102), 1,
      aux_sym_string_content_repeat1,
    ACTIONS(356), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [2907] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(359), 1,
      ts_builtin_sym_end,
    ACTIONS(361), 1,
      anon_sym_suite,
    STATE(103), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [2921] = 4,
    ACTIONS(324), 1,
      sym_comment,
    ACTIONS(364), 1,
      anon_sym_SQUOTE,
    STATE(104), 1,
      aux_sym_single_string_content_repeat1,
    ACTIONS(366), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [2935] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(213), 1,
      anon_sym_DQUOTE,
    ACTIONS(215), 1,
      anon_sym_SQUOTE,
    ACTIONS(369), 1,
      anon_sym_RBRACK,
    STATE(166), 1,
      sym_string,
  [2951] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(338), 1,
      anon_sym_charting,
    ACTIONS(371), 1,
      anon_sym_RBRACE,
    STATE(113), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [2965] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_suite,
    ACTIONS(373), 1,
      ts_builtin_sym_end,
    STATE(103), 2,
      sym_suite,
      aux_sym_source_file_repeat2,
  [2979] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(213), 1,
      anon_sym_DQUOTE,
    ACTIONS(215), 1,
      anon_sym_SQUOTE,
    ACTIONS(375), 1,
      anon_sym_RBRACK,
    STATE(122), 1,
      sym_string,
  [2995] = 4,
    ACTIONS(324), 1,
      sym_comment,
    ACTIONS(348), 1,
      anon_sym_LBRACE,
    ACTIONS(377), 1,
      sym_inline_code,
    STATE(28), 2,
      sym__code_or_inline,
      sym_code_block,
  [3009] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(379), 4,
      ts_builtin_sym_end,
      anon_sym_use,
      anon_sym_globalSetup,
      anon_sym_suite,
  [3019] = 4,
    ACTIONS(324), 1,
      sym_comment,
    ACTIONS(381), 1,
      anon_sym_DQUOTE,
    STATE(102), 1,
      aux_sym_string_content_repeat1,
    ACTIONS(383), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [3033] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(330), 1,
      sym_identifier,
    ACTIONS(385), 1,
      anon_sym_RPAREN,
    STATE(141), 1,
      sym_argument,
    STATE(212), 1,
      sym_argument_list,
  [3049] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(387), 1,
      anon_sym_RBRACE,
    ACTIONS(389), 1,
      anon_sym_charting,
    STATE(113), 2,
      sym_chart_directive,
      aux_sym_after_body_repeat1,
  [3063] = 4,
    ACTIONS(324), 1,
      sym_comment,
    ACTIONS(392), 1,
      anon_sym_SQUOTE,
    STATE(104), 1,
      aux_sym_single_string_content_repeat1,
    ACTIONS(394), 2,
      aux_sym_single_string_content_token1,
      sym_escape_sequence,
  [3077] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(396), 1,
      anon_sym_RPAREN,
    ACTIONS(398), 1,
      anon_sym_COMMA,
    STATE(138), 1,
      aux_sym_chart_params_repeat1,
  [3090] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(402), 1,
      anon_sym_RBRACE,
    ACTIONS(400), 2,
      anon_sym_anvil,
      sym_identifier,
  [3101] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(406), 1,
      anon_sym_RBRACE,
    ACTIONS(404), 2,
      anon_sym_anvil,
      sym_identifier,
  [3112] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(408), 1,
      anon_sym_COMMA,
    ACTIONS(411), 1,
      anon_sym_RBRACK,
    STATE(118), 1,
      aux_sym_string_array_repeat1,
  [3125] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(330), 1,
      sym_identifier,
    ACTIONS(413), 1,
      anon_sym_RPAREN,
    STATE(147), 1,
      sym_argument,
  [3138] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(417), 1,
      anon_sym_RBRACE,
    ACTIONS(415), 2,
      anon_sym_anvil,
      sym_identifier,
  [3149] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(213), 1,
      anon_sym_DQUOTE,
    ACTIONS(215), 1,
      anon_sym_SQUOTE,
    STATE(201), 1,
      sym_string,
  [3162] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(419), 1,
      anon_sym_COMMA,
    ACTIONS(421), 1,
      anon_sym_RBRACK,
    STATE(128), 1,
      aux_sym_string_array_repeat1,
  [3175] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(213), 1,
      anon_sym_DQUOTE,
    ACTIONS(215), 1,
      anon_sym_SQUOTE,
    STATE(204), 1,
      sym_string,
  [3188] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(425), 1,
      anon_sym_RBRACE,
    ACTIONS(423), 2,
      anon_sym_anvil,
      sym_identifier,
  [3199] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(429), 1,
      anon_sym_RBRACE,
    ACTIONS(427), 2,
      anon_sym_anvil,
      sym_identifier,
  [3210] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(431), 1,
      anon_sym_RPAREN,
    ACTIONS(433), 1,
      anon_sym_COMMA,
    STATE(126), 1,
      aux_sym_fixture_params_repeat1,
  [3223] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(436), 1,
      sym_identifier,
    ACTIONS(438), 1,
      anon_sym_RPAREN,
    STATE(168), 1,
      sym_fixture_param,
  [3236] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(369), 1,
      anon_sym_RBRACK,
    ACTIONS(440), 1,
      anon_sym_COMMA,
    STATE(118), 1,
      aux_sym_string_array_repeat1,
  [3249] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(442), 1,
      anon_sym_RPAREN,
    ACTIONS(444), 1,
      anon_sym_COMMA,
    STATE(132), 1,
      aux_sym_fixture_params_repeat1,
  [3262] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(209), 1,
      sym_chart_function_name,
    ACTIONS(446), 2,
      anon_sym_drawSpeedupChart,
      anon_sym_drawTable,
  [3273] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(450), 1,
      anon_sym_RBRACE,
    ACTIONS(448), 2,
      anon_sym_anvil,
      sym_identifier,
  [3284] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(452), 1,
      anon_sym_RPAREN,
    ACTIONS(454), 1,
      anon_sym_COMMA,
    STATE(126), 1,
      aux_sym_fixture_params_repeat1,
  [3297] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(436), 1,
      sym_identifier,
    ACTIONS(456), 1,
      anon_sym_RPAREN,
    STATE(129), 1,
      sym_fixture_param,
  [3310] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(458), 1,
      anon_sym_RPAREN,
    ACTIONS(460), 1,
      anon_sym_COMMA,
    STATE(142), 1,
      aux_sym_argument_list_repeat1,
  [3323] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(330), 1,
      sym_identifier,
    ACTIONS(458), 1,
      anon_sym_RPAREN,
    STATE(147), 1,
      sym_argument,
  [3336] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(464), 1,
      anon_sym_RBRACE,
    ACTIONS(462), 2,
      anon_sym_anvil,
      sym_identifier,
  [3347] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(213), 1,
      anon_sym_DQUOTE,
    ACTIONS(215), 1,
      anon_sym_SQUOTE,
    STATE(166), 1,
      sym_string,
  [3360] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(209), 1,
      anon_sym_RPAREN,
    ACTIONS(466), 1,
      anon_sym_COMMA,
    STATE(140), 1,
      aux_sym_chart_params_repeat1,
  [3373] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(468), 1,
      anon_sym_RPAREN,
    ACTIONS(470), 1,
      anon_sym_fork,
    STATE(187), 1,
      sym_anvil_args,
  [3386] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(472), 1,
      anon_sym_RPAREN,
    ACTIONS(474), 1,
      anon_sym_COMMA,
    STATE(140), 1,
      aux_sym_chart_params_repeat1,
  [3399] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(477), 1,
      anon_sym_RPAREN,
    ACTIONS(479), 1,
      anon_sym_COMMA,
    STATE(134), 1,
      aux_sym_argument_list_repeat1,
  [3412] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(481), 1,
      anon_sym_RPAREN,
    ACTIONS(483), 1,
      anon_sym_COMMA,
    STATE(142), 1,
      aux_sym_argument_list_repeat1,
  [3425] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(436), 1,
      sym_identifier,
    ACTIONS(452), 1,
      anon_sym_RPAREN,
    STATE(168), 1,
      sym_fixture_param,
  [3438] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(436), 1,
      sym_identifier,
    STATE(168), 1,
      sym_fixture_param,
  [3448] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(486), 2,
      ts_builtin_sym_end,
      anon_sym_suite,
  [3456] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(488), 1,
      anon_sym_LBRACE,
    STATE(39), 1,
      sym_global_setup_body,
  [3466] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(481), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [3474] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(223), 1,
      anon_sym_LBRACK,
    STATE(34), 1,
      sym_string_array,
  [3484] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(344), 1,
      anon_sym_LBRACE,
    STATE(80), 1,
      sym_code_block,
  [3494] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(490), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [3502] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(492), 1,
      anon_sym_LBRACE,
    STATE(49), 1,
      sym_benchmark_body,
  [3512] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(494), 1,
      anon_sym_LBRACE,
    STATE(55), 1,
      sym_setup_body,
  [3522] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(496), 2,
      anon_sym_LBRACE,
      anon_sym_COLON,
  [3530] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(498), 1,
      anon_sym_LBRACE,
    STATE(165), 1,
      sym_suite_body,
  [3540] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(472), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [3548] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(344), 1,
      anon_sym_LBRACE,
    STATE(82), 1,
      sym_code_block,
  [3558] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(344), 1,
      anon_sym_LBRACE,
    STATE(83), 1,
      sym_code_block,
  [3568] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(500), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [3576] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(344), 1,
      anon_sym_LBRACE,
    STATE(40), 1,
      sym_code_block,
  [3586] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(340), 1,
      anon_sym_LBRACE,
    STATE(46), 1,
      sym_fixture_body,
  [3596] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(344), 1,
      anon_sym_LBRACE,
    STATE(86), 1,
      sym_code_block,
  [3606] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(502), 2,
      anon_sym_RBRACE,
      anon_sym_charting,
  [3614] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(504), 1,
      anon_sym_DOT,
    ACTIONS(506), 1,
      anon_sym_LPAREN,
  [3624] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(508), 2,
      anon_sym_RBRACE,
      anon_sym_charting,
  [3632] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(510), 2,
      ts_builtin_sym_end,
      anon_sym_suite,
  [3640] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(411), 2,
      anon_sym_COMMA,
      anon_sym_RBRACK,
  [3648] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(512), 2,
      ts_builtin_sym_end,
      anon_sym_suite,
  [3656] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(431), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [3664] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(514), 1,
      anon_sym_RPAREN,
    ACTIONS(516), 1,
      sym_embedded_code,
  [3674] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(518), 1,
      anon_sym_RBRACE,
    ACTIONS(520), 1,
      sym_embedded_code,
  [3684] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(522), 1,
      anon_sym_LBRACE,
    STATE(50), 1,
      sym_after_body,
  [3694] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(330), 1,
      sym_identifier,
    STATE(147), 1,
      sym_argument,
  [3704] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(524), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [3712] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(526), 1,
      sym_identifier,
  [3719] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(528), 1,
      anon_sym_LBRACE,
  [3726] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(530), 1,
      anon_sym_LPAREN,
  [3733] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(532), 1,
      anon_sym_COLON,
  [3740] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(534), 1,
      sym_identifier,
  [3747] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(536), 1,
      anon_sym_LBRACE,
  [3754] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(538), 1,
      sym_identifier,
  [3761] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(540), 1,
      anon_sym_COLON,
  [3768] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(542), 1,
      sym_identifier,
  [3775] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(544), 1,
      anon_sym_LPAREN,
  [3782] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(546), 1,
      anon_sym_SQUOTE,
  [3789] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(546), 1,
      anon_sym_DQUOTE,
  [3796] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(548), 1,
      anon_sym_spawnAnvil,
  [3803] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(550), 1,
      anon_sym_RPAREN,
  [3810] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(552), 1,
      anon_sym_std,
  [3817] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(554), 1,
      anon_sym_LBRACE,
  [3824] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(556), 1,
      anon_sym_RPAREN,
  [3831] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(558), 1,
      anon_sym_COLON,
  [3838] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(560), 1,
      anon_sym_RPAREN,
  [3845] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(562), 1,
      anon_sym_COLON,
  [3852] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(564), 1,
      anon_sym_LPAREN,
  [3859] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(566), 1,
      anon_sym_COLON,
  [3866] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(568), 1,
      anon_sym_RPAREN,
  [3873] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(570), 1,
      anon_sym_COLON,
  [3880] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(572), 1,
      anon_sym_COLON,
  [3887] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(574), 1,
      anon_sym_RBRACE,
  [3894] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(576), 1,
      anon_sym_COLON,
  [3901] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(578), 1,
      anon_sym_RPAREN,
  [3908] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(580), 1,
      anon_sym_DOT,
  [3915] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(582), 1,
      sym_identifier,
  [3922] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(584), 1,
      anon_sym_RPAREN,
  [3929] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(586), 1,
      anon_sym_COLON,
  [3936] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(588), 1,
      anon_sym_COLON,
  [3943] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(590), 1,
      anon_sym_DOT,
  [3950] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(592), 1,
      anon_sym_COLON,
  [3957] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(594), 1,
      anon_sym_LPAREN,
  [3964] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(596), 1,
      anon_sym_LPAREN,
  [3971] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(598), 1,
      anon_sym_init,
  [3978] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(600), 1,
      anon_sym_RPAREN,
  [3985] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(602), 1,
      anon_sym_COLON_COLON,
  [3992] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(604), 1,
      anon_sym_COLON,
  [3999] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(606), 1,
      anon_sym_LBRACE,
  [4006] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(608), 1,
      ts_builtin_sym_end,
  [4013] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(610), 1,
      sym_identifier,
  [4020] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(612), 1,
      anon_sym_LBRACE,
  [4027] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(614), 1,
      anon_sym_COLON,
  [4034] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(616), 1,
      anon_sym_COLON,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 66,
  [SMALL_STATE(4)] = 132,
  [SMALL_STATE(5)] = 198,
  [SMALL_STATE(6)] = 239,
  [SMALL_STATE(7)] = 280,
  [SMALL_STATE(8)] = 320,
  [SMALL_STATE(9)] = 360,
  [SMALL_STATE(10)] = 400,
  [SMALL_STATE(11)] = 440,
  [SMALL_STATE(12)] = 480,
  [SMALL_STATE(13)] = 520,
  [SMALL_STATE(14)] = 560,
  [SMALL_STATE(15)] = 599,
  [SMALL_STATE(16)] = 638,
  [SMALL_STATE(17)] = 676,
  [SMALL_STATE(18)] = 726,
  [SMALL_STATE(19)] = 776,
  [SMALL_STATE(20)] = 826,
  [SMALL_STATE(21)] = 868,
  [SMALL_STATE(22)] = 919,
  [SMALL_STATE(23)] = 970,
  [SMALL_STATE(24)] = 1021,
  [SMALL_STATE(25)] = 1055,
  [SMALL_STATE(26)] = 1087,
  [SMALL_STATE(27)] = 1119,
  [SMALL_STATE(28)] = 1151,
  [SMALL_STATE(29)] = 1183,
  [SMALL_STATE(30)] = 1221,
  [SMALL_STATE(31)] = 1253,
  [SMALL_STATE(32)] = 1285,
  [SMALL_STATE(33)] = 1317,
  [SMALL_STATE(34)] = 1349,
  [SMALL_STATE(35)] = 1381,
  [SMALL_STATE(36)] = 1417,
  [SMALL_STATE(37)] = 1445,
  [SMALL_STATE(38)] = 1473,
  [SMALL_STATE(39)] = 1501,
  [SMALL_STATE(40)] = 1529,
  [SMALL_STATE(41)] = 1557,
  [SMALL_STATE(42)] = 1585,
  [SMALL_STATE(43)] = 1611,
  [SMALL_STATE(44)] = 1637,
  [SMALL_STATE(45)] = 1663,
  [SMALL_STATE(46)] = 1689,
  [SMALL_STATE(47)] = 1715,
  [SMALL_STATE(48)] = 1741,
  [SMALL_STATE(49)] = 1777,
  [SMALL_STATE(50)] = 1803,
  [SMALL_STATE(51)] = 1829,
  [SMALL_STATE(52)] = 1855,
  [SMALL_STATE(53)] = 1881,
  [SMALL_STATE(54)] = 1907,
  [SMALL_STATE(55)] = 1933,
  [SMALL_STATE(56)] = 1959,
  [SMALL_STATE(57)] = 1992,
  [SMALL_STATE(58)] = 2025,
  [SMALL_STATE(59)] = 2055,
  [SMALL_STATE(60)] = 2088,
  [SMALL_STATE(61)] = 2121,
  [SMALL_STATE(62)] = 2154,
  [SMALL_STATE(63)] = 2187,
  [SMALL_STATE(64)] = 2217,
  [SMALL_STATE(65)] = 2247,
  [SMALL_STATE(66)] = 2277,
  [SMALL_STATE(67)] = 2306,
  [SMALL_STATE(68)] = 2327,
  [SMALL_STATE(69)] = 2348,
  [SMALL_STATE(70)] = 2369,
  [SMALL_STATE(71)] = 2390,
  [SMALL_STATE(72)] = 2411,
  [SMALL_STATE(73)] = 2432,
  [SMALL_STATE(74)] = 2459,
  [SMALL_STATE(75)] = 2480,
  [SMALL_STATE(76)] = 2501,
  [SMALL_STATE(77)] = 2522,
  [SMALL_STATE(78)] = 2543,
  [SMALL_STATE(79)] = 2564,
  [SMALL_STATE(80)] = 2582,
  [SMALL_STATE(81)] = 2594,
  [SMALL_STATE(82)] = 2606,
  [SMALL_STATE(83)] = 2618,
  [SMALL_STATE(84)] = 2630,
  [SMALL_STATE(85)] = 2646,
  [SMALL_STATE(86)] = 2658,
  [SMALL_STATE(87)] = 2670,
  [SMALL_STATE(88)] = 2684,
  [SMALL_STATE(89)] = 2696,
  [SMALL_STATE(90)] = 2713,
  [SMALL_STATE(91)] = 2730,
  [SMALL_STATE(92)] = 2747,
  [SMALL_STATE(93)] = 2763,
  [SMALL_STATE(94)] = 2779,
  [SMALL_STATE(95)] = 2793,
  [SMALL_STATE(96)] = 2807,
  [SMALL_STATE(97)] = 2821,
  [SMALL_STATE(98)] = 2837,
  [SMALL_STATE(99)] = 2851,
  [SMALL_STATE(100)] = 2865,
  [SMALL_STATE(101)] = 2879,
  [SMALL_STATE(102)] = 2893,
  [SMALL_STATE(103)] = 2907,
  [SMALL_STATE(104)] = 2921,
  [SMALL_STATE(105)] = 2935,
  [SMALL_STATE(106)] = 2951,
  [SMALL_STATE(107)] = 2965,
  [SMALL_STATE(108)] = 2979,
  [SMALL_STATE(109)] = 2995,
  [SMALL_STATE(110)] = 3009,
  [SMALL_STATE(111)] = 3019,
  [SMALL_STATE(112)] = 3033,
  [SMALL_STATE(113)] = 3049,
  [SMALL_STATE(114)] = 3063,
  [SMALL_STATE(115)] = 3077,
  [SMALL_STATE(116)] = 3090,
  [SMALL_STATE(117)] = 3101,
  [SMALL_STATE(118)] = 3112,
  [SMALL_STATE(119)] = 3125,
  [SMALL_STATE(120)] = 3138,
  [SMALL_STATE(121)] = 3149,
  [SMALL_STATE(122)] = 3162,
  [SMALL_STATE(123)] = 3175,
  [SMALL_STATE(124)] = 3188,
  [SMALL_STATE(125)] = 3199,
  [SMALL_STATE(126)] = 3210,
  [SMALL_STATE(127)] = 3223,
  [SMALL_STATE(128)] = 3236,
  [SMALL_STATE(129)] = 3249,
  [SMALL_STATE(130)] = 3262,
  [SMALL_STATE(131)] = 3273,
  [SMALL_STATE(132)] = 3284,
  [SMALL_STATE(133)] = 3297,
  [SMALL_STATE(134)] = 3310,
  [SMALL_STATE(135)] = 3323,
  [SMALL_STATE(136)] = 3336,
  [SMALL_STATE(137)] = 3347,
  [SMALL_STATE(138)] = 3360,
  [SMALL_STATE(139)] = 3373,
  [SMALL_STATE(140)] = 3386,
  [SMALL_STATE(141)] = 3399,
  [SMALL_STATE(142)] = 3412,
  [SMALL_STATE(143)] = 3425,
  [SMALL_STATE(144)] = 3438,
  [SMALL_STATE(145)] = 3448,
  [SMALL_STATE(146)] = 3456,
  [SMALL_STATE(147)] = 3466,
  [SMALL_STATE(148)] = 3474,
  [SMALL_STATE(149)] = 3484,
  [SMALL_STATE(150)] = 3494,
  [SMALL_STATE(151)] = 3502,
  [SMALL_STATE(152)] = 3512,
  [SMALL_STATE(153)] = 3522,
  [SMALL_STATE(154)] = 3530,
  [SMALL_STATE(155)] = 3540,
  [SMALL_STATE(156)] = 3548,
  [SMALL_STATE(157)] = 3558,
  [SMALL_STATE(158)] = 3568,
  [SMALL_STATE(159)] = 3576,
  [SMALL_STATE(160)] = 3586,
  [SMALL_STATE(161)] = 3596,
  [SMALL_STATE(162)] = 3606,
  [SMALL_STATE(163)] = 3614,
  [SMALL_STATE(164)] = 3624,
  [SMALL_STATE(165)] = 3632,
  [SMALL_STATE(166)] = 3640,
  [SMALL_STATE(167)] = 3648,
  [SMALL_STATE(168)] = 3656,
  [SMALL_STATE(169)] = 3664,
  [SMALL_STATE(170)] = 3674,
  [SMALL_STATE(171)] = 3684,
  [SMALL_STATE(172)] = 3694,
  [SMALL_STATE(173)] = 3704,
  [SMALL_STATE(174)] = 3712,
  [SMALL_STATE(175)] = 3719,
  [SMALL_STATE(176)] = 3726,
  [SMALL_STATE(177)] = 3733,
  [SMALL_STATE(178)] = 3740,
  [SMALL_STATE(179)] = 3747,
  [SMALL_STATE(180)] = 3754,
  [SMALL_STATE(181)] = 3761,
  [SMALL_STATE(182)] = 3768,
  [SMALL_STATE(183)] = 3775,
  [SMALL_STATE(184)] = 3782,
  [SMALL_STATE(185)] = 3789,
  [SMALL_STATE(186)] = 3796,
  [SMALL_STATE(187)] = 3803,
  [SMALL_STATE(188)] = 3810,
  [SMALL_STATE(189)] = 3817,
  [SMALL_STATE(190)] = 3824,
  [SMALL_STATE(191)] = 3831,
  [SMALL_STATE(192)] = 3838,
  [SMALL_STATE(193)] = 3845,
  [SMALL_STATE(194)] = 3852,
  [SMALL_STATE(195)] = 3859,
  [SMALL_STATE(196)] = 3866,
  [SMALL_STATE(197)] = 3873,
  [SMALL_STATE(198)] = 3880,
  [SMALL_STATE(199)] = 3887,
  [SMALL_STATE(200)] = 3894,
  [SMALL_STATE(201)] = 3901,
  [SMALL_STATE(202)] = 3908,
  [SMALL_STATE(203)] = 3915,
  [SMALL_STATE(204)] = 3922,
  [SMALL_STATE(205)] = 3929,
  [SMALL_STATE(206)] = 3936,
  [SMALL_STATE(207)] = 3943,
  [SMALL_STATE(208)] = 3950,
  [SMALL_STATE(209)] = 3957,
  [SMALL_STATE(210)] = 3964,
  [SMALL_STATE(211)] = 3971,
  [SMALL_STATE(212)] = 3978,
  [SMALL_STATE(213)] = 3985,
  [SMALL_STATE(214)] = 3992,
  [SMALL_STATE(215)] = 3999,
  [SMALL_STATE(216)] = 4006,
  [SMALL_STATE(217)] = 4013,
  [SMALL_STATE(218)] = 4020,
  [SMALL_STATE(219)] = 4027,
  [SMALL_STATE(220)] = 4034,
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
  [57] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [59] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 3, 0, 0),
  [61] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 2, 0, 0),
  [63] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_duration_unit, 1, 0, 0),
  [65] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_duration, 2, 0, 0),
  [67] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_boolean, 1, 0, 0),
  [69] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 2, 0, 0),
  [71] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 3, 0, 0),
  [73] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 4, 0, 0),
  [75] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_array, 5, 0, 0),
  [77] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_code_block, 2, 0, 0),
  [79] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_code_block, 3, 0, 0),
  [81] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property, 3, 0, 4),
  [83] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [85] = {.entry = {.count = 1, .reusable = true}}, SHIFT(206),
  [87] = {.entry = {.count = 1, .reusable = true}}, SHIFT(205),
  [89] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0),
  [91] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(206),
  [94] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(205),
  [97] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(177),
  [100] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_body_repeat1, 2, 0, 0), SHIFT_REPEAT(153),
  [103] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [105] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__value, 1, 0, 0),
  [107] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [109] = {.entry = {.count = 1, .reusable = false}}, SHIFT(7),
  [111] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(146),
  [114] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0),
  [116] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(87),
  [119] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(180),
  [122] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(178),
  [125] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(171),
  [128] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_suite_body_repeat1, 2, 0, 0), SHIFT_REPEAT(177),
  [131] = {.entry = {.count = 1, .reusable = true}}, SHIFT(167),
  [133] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [135] = {.entry = {.count = 1, .reusable = true}}, SHIFT(180),
  [137] = {.entry = {.count = 1, .reusable = true}}, SHIFT(178),
  [139] = {.entry = {.count = 1, .reusable = true}}, SHIFT(171),
  [141] = {.entry = {.count = 1, .reusable = true}}, SHIFT(145),
  [143] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_language_implementation, 3, 0, 5),
  [145] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_hook, 2, 0, 0),
  [147] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_each_hook, 2, 0, 0),
  [149] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_grouped, 4, 0, 0),
  [151] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_flat, 3, 0, 5),
  [153] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hook_grouped, 3, 0, 0),
  [155] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_skip_hook, 2, 0, 0),
  [157] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_validate_hook, 2, 0, 0),
  [159] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_before_hook, 2, 0, 0),
  [161] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tags_property, 3, 0, 0),
  [163] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_body, 2, 0, 0),
  [165] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_file_ref, 4, 0, 0),
  [167] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_body, 3, 0, 0),
  [169] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup, 2, 0, 0),
  [171] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_shape_property, 3, 0, 0),
  [173] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hex_property, 3, 0, 0),
  [175] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_body, 2, 0, 0),
  [177] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_body, 2, 0, 0),
  [179] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark_body, 3, 0, 0),
  [181] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture, 3, 0, 1),
  [183] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture, 4, 0, 1),
  [185] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_body, 3, 0, 0),
  [187] = {.entry = {.count = 1, .reusable = true}}, SHIFT(164),
  [189] = {.entry = {.count = 1, .reusable = true}}, SHIFT(195),
  [191] = {.entry = {.count = 1, .reusable = false}}, SHIFT(195),
  [193] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark, 3, 0, 1),
  [195] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_after_block, 2, 0, 0),
  [197] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_body, 3, 0, 0),
  [199] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_benchmark_body, 2, 0, 0),
  [201] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_body, 3, 0, 0),
  [203] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_body, 2, 0, 0),
  [205] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_setup_block, 3, 0, 3),
  [207] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 3, 0, 0),
  [209] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 2, 0, 0),
  [211] = {.entry = {.count = 1, .reusable = false}}, SHIFT(16),
  [213] = {.entry = {.count = 1, .reusable = true}}, SHIFT(89),
  [215] = {.entry = {.count = 1, .reusable = true}}, SHIFT(90),
  [217] = {.entry = {.count = 1, .reusable = false}}, SHIFT(29),
  [219] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [221] = {.entry = {.count = 1, .reusable = false}}, SHIFT(9),
  [223] = {.entry = {.count = 1, .reusable = true}}, SHIFT(108),
  [225] = {.entry = {.count = 1, .reusable = false}}, SHIFT(158),
  [227] = {.entry = {.count = 1, .reusable = false}}, SHIFT(79),
  [229] = {.entry = {.count = 1, .reusable = true}}, SHIFT(158),
  [231] = {.entry = {.count = 1, .reusable = false}}, SHIFT(35),
  [233] = {.entry = {.count = 1, .reusable = false}}, SHIFT(20),
  [235] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [237] = {.entry = {.count = 1, .reusable = true}}, SHIFT(98),
  [239] = {.entry = {.count = 1, .reusable = true}}, SHIFT(149),
  [241] = {.entry = {.count = 1, .reusable = true}}, SHIFT(211),
  [243] = {.entry = {.count = 1, .reusable = true}}, SHIFT(156),
  [245] = {.entry = {.count = 1, .reusable = true}}, SHIFT(157),
  [247] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0),
  [249] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(98),
  [252] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(149),
  [255] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(211),
  [258] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(156),
  [261] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(157),
  [264] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [266] = {.entry = {.count = 1, .reusable = false}}, SHIFT(150),
  [268] = {.entry = {.count = 1, .reusable = true}}, SHIFT(150),
  [270] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [272] = {.entry = {.count = 1, .reusable = true}}, SHIFT(215),
  [274] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [276] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_hook_grouped_repeat1, 2, 0, 0),
  [278] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_hook_grouped_repeat1, 2, 0, 0), SHIFT_REPEAT(153),
  [281] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [283] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, 0, 0),
  [285] = {.entry = {.count = 1, .reusable = false}}, SHIFT(163),
  [287] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [289] = {.entry = {.count = 1, .reusable = false}}, SHIFT(202),
  [291] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [293] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(163),
  [296] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0),
  [298] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_global_setup_body_repeat1, 2, 0, 0), SHIFT_REPEAT(202),
  [301] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declare_section, 2, 0, 0),
  [303] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_section, 2, 0, 0),
  [305] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_init_section, 2, 0, 0),
  [307] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_helpers_section, 2, 0, 0),
  [309] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0),
  [311] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(188),
  [314] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_paren_code_block, 2, 0, 0),
  [316] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_init_section, 3, 0, 0),
  [318] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_paren_code_block, 3, 0, 0),
  [320] = {.entry = {.count = 1, .reusable = false}}, SHIFT(6),
  [322] = {.entry = {.count = 1, .reusable = false}}, SHIFT(111),
  [324] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [326] = {.entry = {.count = 1, .reusable = false}}, SHIFT(114),
  [328] = {.entry = {.count = 1, .reusable = true}}, SHIFT(176),
  [330] = {.entry = {.count = 1, .reusable = true}}, SHIFT(208),
  [332] = {.entry = {.count = 1, .reusable = true}}, SHIFT(117),
  [334] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [336] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [338] = {.entry = {.count = 1, .reusable = true}}, SHIFT(207),
  [340] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [342] = {.entry = {.count = 1, .reusable = true}}, SHIFT(133),
  [344] = {.entry = {.count = 1, .reusable = true}}, SHIFT(170),
  [346] = {.entry = {.count = 1, .reusable = true}}, SHIFT(169),
  [348] = {.entry = {.count = 1, .reusable = false}}, SHIFT(170),
  [350] = {.entry = {.count = 1, .reusable = false}}, SHIFT(24),
  [352] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, 0, 0),
  [354] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_string_content_repeat1, 2, 0, 0),
  [356] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_string_content_repeat1, 2, 0, 0), SHIFT_REPEAT(102),
  [359] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat2, 2, 0, 0),
  [361] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat2, 2, 0, 0), SHIFT_REPEAT(217),
  [364] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_single_string_content_repeat1, 2, 0, 0),
  [366] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_single_string_content_repeat1, 2, 0, 0), SHIFT_REPEAT(104),
  [369] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [371] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [373] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, 0, 0),
  [375] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [377] = {.entry = {.count = 1, .reusable = false}}, SHIFT(28),
  [379] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_use_statement, 4, 0, 2),
  [381] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_content, 1, 0, 0),
  [383] = {.entry = {.count = 1, .reusable = false}}, SHIFT(102),
  [385] = {.entry = {.count = 1, .reusable = true}}, SHIFT(116),
  [387] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_after_body_repeat1, 2, 0, 0),
  [389] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_after_body_repeat1, 2, 0, 0), SHIFT_REPEAT(207),
  [392] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_single_string_content, 1, 0, 0),
  [394] = {.entry = {.count = 1, .reusable = false}}, SHIFT(104),
  [396] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_params, 1, 0, 0),
  [398] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [400] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 3, 0, 0),
  [402] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 3, 0, 0),
  [404] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 5, 0, 0),
  [406] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 5, 0, 0),
  [408] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_string_array_repeat1, 2, 0, 0), SHIFT_REPEAT(137),
  [411] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_string_array_repeat1, 2, 0, 0),
  [413] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 3, 0, 0),
  [415] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_anvil_call, 5, 0, 0),
  [417] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_call, 5, 0, 0),
  [419] = {.entry = {.count = 1, .reusable = true}}, SHIFT(105),
  [421] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [423] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_anvil_call, 6, 0, 0),
  [425] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_call, 6, 0, 0),
  [427] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 6, 0, 0),
  [429] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 6, 0, 0),
  [431] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fixture_params_repeat1, 2, 0, 0),
  [433] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fixture_params_repeat1, 2, 0, 0), SHIFT_REPEAT(144),
  [436] = {.entry = {.count = 1, .reusable = true}}, SHIFT(193),
  [438] = {.entry = {.count = 1, .reusable = true}}, SHIFT(189),
  [440] = {.entry = {.count = 1, .reusable = true}}, SHIFT(93),
  [442] = {.entry = {.count = 1, .reusable = true}}, SHIFT(218),
  [444] = {.entry = {.count = 1, .reusable = true}}, SHIFT(143),
  [446] = {.entry = {.count = 1, .reusable = true}}, SHIFT(210),
  [448] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_setup_statement, 1, 0, 0),
  [450] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_setup_statement, 1, 0, 0),
  [452] = {.entry = {.count = 1, .reusable = true}}, SHIFT(179),
  [454] = {.entry = {.count = 1, .reusable = true}}, SHIFT(127),
  [456] = {.entry = {.count = 1, .reusable = true}}, SHIFT(175),
  [458] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 2, 0, 0),
  [460] = {.entry = {.count = 1, .reusable = true}}, SHIFT(119),
  [462] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 4, 0, 0),
  [464] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 4, 0, 0),
  [466] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [468] = {.entry = {.count = 1, .reusable = true}}, SHIFT(120),
  [470] = {.entry = {.count = 1, .reusable = true}}, SHIFT(181),
  [472] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_chart_params_repeat1, 2, 0, 0),
  [474] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_chart_params_repeat1, 2, 0, 0), SHIFT_REPEAT(58),
  [477] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 1, 0, 0),
  [479] = {.entry = {.count = 1, .reusable = true}}, SHIFT(135),
  [481] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_argument_list_repeat1, 2, 0, 0),
  [483] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_argument_list_repeat1, 2, 0, 0), SHIFT_REPEAT(172),
  [486] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite_body, 2, 0, 0),
  [488] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [490] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_param, 3, 0, 4),
  [492] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [494] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [496] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_language_tag, 1, 0, 0),
  [498] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [500] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument, 3, 0, 4),
  [502] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_directive, 6, 0, 7),
  [504] = {.entry = {.count = 1, .reusable = true}}, SHIFT(174),
  [506] = {.entry = {.count = 1, .reusable = true}}, SHIFT(112),
  [508] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_directive, 5, 0, 7),
  [510] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite, 3, 0, 1),
  [512] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_suite_body, 3, 0, 0),
  [514] = {.entry = {.count = 1, .reusable = true}}, SHIFT(85),
  [516] = {.entry = {.count = 1, .reusable = true}}, SHIFT(192),
  [518] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [520] = {.entry = {.count = 1, .reusable = true}}, SHIFT(199),
  [522] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [524] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_param, 3, 0, 6),
  [526] = {.entry = {.count = 1, .reusable = true}}, SHIFT(194),
  [528] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 2, 0, 0),
  [530] = {.entry = {.count = 1, .reusable = true}}, SHIFT(121),
  [532] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property_name, 1, 0, 0),
  [534] = {.entry = {.count = 1, .reusable = true}}, SHIFT(151),
  [536] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 4, 0, 0),
  [538] = {.entry = {.count = 1, .reusable = true}}, SHIFT(97),
  [540] = {.entry = {.count = 1, .reusable = true}}, SHIFT(123),
  [542] = {.entry = {.count = 1, .reusable = true}}, SHIFT(173),
  [544] = {.entry = {.count = 1, .reusable = true}}, SHIFT(139),
  [546] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [548] = {.entry = {.count = 1, .reusable = true}}, SHIFT(183),
  [550] = {.entry = {.count = 1, .reusable = true}}, SHIFT(124),
  [552] = {.entry = {.count = 1, .reusable = true}}, SHIFT(213),
  [554] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 5, 0, 0),
  [556] = {.entry = {.count = 1, .reusable = true}}, SHIFT(125),
  [558] = {.entry = {.count = 1, .reusable = true}}, SHIFT(148),
  [560] = {.entry = {.count = 1, .reusable = true}}, SHIFT(88),
  [562] = {.entry = {.count = 1, .reusable = true}}, SHIFT(182),
  [564] = {.entry = {.count = 1, .reusable = true}}, SHIFT(92),
  [566] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_param_name, 1, 0, 0),
  [568] = {.entry = {.count = 1, .reusable = true}}, SHIFT(162),
  [570] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [572] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [574] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [576] = {.entry = {.count = 1, .reusable = true}}, SHIFT(99),
  [578] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [580] = {.entry = {.count = 1, .reusable = true}}, SHIFT(186),
  [582] = {.entry = {.count = 1, .reusable = true}}, SHIFT(110),
  [584] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anvil_args, 3, 0, 0),
  [586] = {.entry = {.count = 1, .reusable = true}}, SHIFT(159),
  [588] = {.entry = {.count = 1, .reusable = true}}, SHIFT(91),
  [590] = {.entry = {.count = 1, .reusable = true}}, SHIFT(130),
  [592] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [594] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [596] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chart_function_name, 1, 0, 0),
  [598] = {.entry = {.count = 1, .reusable = true}}, SHIFT(161),
  [600] = {.entry = {.count = 1, .reusable = true}}, SHIFT(136),
  [602] = {.entry = {.count = 1, .reusable = true}}, SHIFT(203),
  [604] = {.entry = {.count = 1, .reusable = true}}, SHIFT(109),
  [606] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [608] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [610] = {.entry = {.count = 1, .reusable = true}}, SHIFT(154),
  [612] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fixture_params, 3, 0, 0),
  [614] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
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
