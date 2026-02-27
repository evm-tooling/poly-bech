/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

/**
 * Tree-sitter grammar for the poly-bench DSL
 *
 * This grammar provides robust, incremental parsing with proper error recovery.
 * It handles embedded code blocks (Go, TypeScript, Rust) via a custom scanner
 * that performs brace-counting rather than relying on indentation.
 */

module.exports = grammar({
  name: 'polybench',

  // Tokens that can appear anywhere (whitespace and comments)
  extras: $ => [
    /\s/,
    $.comment,
  ],

  // External scanner handles embedded code blocks
  externals: $ => [
    $.embedded_code,
    $._embedded_code_start,
  ],

  // Word characters for keyword extraction
  word: $ => $.identifier,

  // No conflicts needed - grammar is unambiguous

  rules: {
    // ============================================================
    // Top-level structure
    // ============================================================

    source_file: $ => seq(
      repeat($.use_statement),
      optional($.global_setup),
      repeat($.suite),
    ),

    // use std::module
    use_statement: $ => seq(
      'use',
      'std',
      '::',
      field('module', $.identifier),
    ),

    // globalSetup { ... }
    global_setup: $ => seq(
      'globalSetup',
      $.global_setup_body,
    ),

    global_setup_body: $ => seq(
      '{',
      repeat($.global_setup_statement),
      '}',
    ),

    global_setup_statement: $ => choice(
      $.anvil_call,
      $.function_call,
    ),

    // anvil.spawnAnvil() or anvil.spawnAnvil(fork: "url")
    anvil_call: $ => seq(
      'anvil',
      '.',
      'spawnAnvil',
      '(',
      optional($.anvil_args),
      ')',
    ),

    anvil_args: $ => seq(
      'fork',
      ':',
      $.string,
    ),

    // Generic function call for extensibility
    function_call: $ => seq(
      $.identifier,
      optional(seq('.', $.identifier)),
      '(',
      optional($.argument_list),
      ')',
    ),

    argument_list: $ => seq(
      $.argument,
      repeat(seq(',', $.argument)),
      optional(','),
    ),

    argument: $ => seq(
      field('name', $.identifier),
      ':',
      field('value', $._value),
    ),

    // ============================================================
    // Suite
    // ============================================================

    suite: $ => seq(
      optional('declare'),
      'suite',
      field('name', $.identifier),
      optional(seq(
        field('suite_type', $.suite_type),
        field('run_mode', $.run_mode),
        'sameDataset',
        ':',
        field('same_dataset', $.boolean),
      )),
      $.suite_body,
    ),

    suite_type: $ => choice('performance', 'memory'),

    run_mode: $ => choice('timeBased', 'iterationBased'),

    suite_body: $ => seq(
      '{',
      repeat($._suite_item),
      '}',
    ),

    _suite_item: $ => choice(
      $.property,
      $.global_setup,
      $.setup_block,
      $.fixture,
      $.benchmark,
      $.after_block,
    ),

    // ============================================================
    // Setup blocks
    // ============================================================

    setup_block: $ => seq(
      'setup',
      field('language', $.language_tag),
      $.setup_body,
    ),

    setup_body: $ => seq(
      '{',
      repeat($._setup_section),
      '}',
    ),

    _setup_section: $ => choice(
      $.import_section,
      $.declare_section,
      $.init_section,
      $.helpers_section,
    ),

    // import ( ... ) for Go or import { ... } for TS
    import_section: $ => seq(
      'import',
      choice(
        $.paren_code_block,  // Go: import ( ... )
        $.code_block,        // TS: import { ... }
      ),
    ),

    declare_section: $ => seq(
      'declare',
      $.code_block,
    ),

    init_section: $ => seq(
      optional('async'),
      'init',
      $.code_block,
    ),

    helpers_section: $ => seq(
      'helpers',
      $.code_block,
    ),

    // ============================================================
    // Fixtures
    // ============================================================

    fixture: $ => seq(
      'fixture',
      field('name', $.identifier),
      optional($.fixture_params),
      $.fixture_body,
    ),

    fixture_params: $ => seq(
      '(',
      optional(seq(
        $.fixture_param,
        repeat(seq(',', $.fixture_param)),
        optional(','),
      )),
      ')',
    ),

    fixture_param: $ => seq(
      field('name', $.identifier),
      ':',
      field('type', $.identifier),
    ),

    fixture_body: $ => seq(
      '{',
      repeat($._fixture_item),
      '}',
    ),

    _fixture_item: $ => choice(
      $.property,
      $.hex_property,
      $.data_property,
      $.encoding_property,
      $.format_property,
      $.selector_property,
      $.shape_property,
      $.language_implementation,
    ),

    hex_property: $ => seq(
      'hex',
      ':',
      choice(
        $.string,
        $.file_ref,
      ),
    ),

    data_property: $ => seq(
      'data',
      ':',
      choice(
        $.string,
        $.file_ref,
      ),
    ),

    encoding_property: $ => seq(
      'encoding',
      ':',
      choice($.identifier, $.string),
    ),

    format_property: $ => seq(
      'format',
      ':',
      choice($.identifier, $.string),
    ),

    selector_property: $ => seq(
      'selector',
      ':',
      $.string,
    ),

    shape_property: $ => seq(
      'shape',
      ':',
      $.code_block,
    ),

    file_ref: $ => seq(
      '@file',
      '(',
      $.string,
      ')',
    ),

    // ============================================================
    // Benchmarks
    // ============================================================

    benchmark: $ => seq(
      choice('bench', 'benchAsync'),
      field('name', $.identifier),
      $.benchmark_body,
    ),

    benchmark_body: $ => seq(
      '{',
      repeat($._benchmark_item),
      '}',
    ),

    _benchmark_item: $ => choice(
      $.property,
      $.tags_property,
      $.skip_hook,
      $.validate_hook,
      $.before_hook,
      $.after_hook,
      $.each_hook,
      $.language_implementation,
    ),

    tags_property: $ => seq(
      'tags',
      ':',
      $.string_array,
    ),

    // Hooks can be flat (skip go: CODE) or grouped (skip: { go: CODE })
    skip_hook: $ => seq(
      'skip',
      choice(
        $.hook_flat,
        $.hook_grouped,
      ),
    ),

    validate_hook: $ => seq(
      'validate',
      choice(
        $.hook_flat,
        $.hook_grouped,
      ),
    ),

    before_hook: $ => seq(
      'before',
      choice(
        $.hook_flat,
        $.hook_grouped,
      ),
    ),

    after_hook: $ => seq(
      'after',
      choice(
        $.hook_flat,
        $.hook_grouped,
      ),
    ),

    each_hook: $ => seq(
      'each',
      choice(
        $.hook_flat,
        $.hook_grouped,
      ),
    ),

    // Flat: before go: CODE
    hook_flat: $ => seq(
      field('language', $.language_tag),
      ':',
      $._code_or_inline,
    ),

    // Grouped: before: { go: CODE, ts: CODE }
    hook_grouped: $ => seq(
      ':',
      '{',
      repeat($.language_implementation),
      '}',
    ),

    // ============================================================
    // After block (charting directives)
    // ============================================================

    after_block: $ => seq(
      'after',
      $.after_body,
    ),

    after_body: $ => seq(
      '{',
      repeat($.chart_directive),
      '}',
    ),

    chart_directive: $ => seq(
      'charting',
      '.',
      field('function', $.chart_function_name),
      '(',
      optional($.chart_params),
      ')',
    ),

    chart_function_name: $ => choice(
      'drawSpeedupChart',
      'drawTable',
      'drawLineChart',
      'drawBarChart',
    ),

    chart_params: $ => seq(
      $.chart_param,
      repeat(seq(',', $.chart_param)),
      optional(','),
    ),

    chart_param: $ => seq(
      field('name', $.chart_param_name),
      ':',
      field('value', $._chart_value),
    ),

    chart_param_name: $ => choice(
      'title',
      'description',
      'output',
      'sortBy',
      'sortOrder',
      'baselineBenchmark',
      'baseline',
      'filterWinner',
      'theme',
      'width',
      'rowCount',
      'height',
      'limit',
      'minSpeedup',
      'includeBenchmarks',
      'excludeBenchmarks',
      'showStdDev',
      'showErrorBars',
      'showRegression',
      'regressionModel',
      'yScale',
      'showStatsTable',
    ),

    _chart_value: $ => choice(
      $.string,
      $.number,
      $.float,
      $.boolean,
      $.string_array,
    ),

    // ============================================================
    // Properties (key: value pairs)
    // ============================================================

    property: $ => seq(
      field('name', $.property_name),
      ':',
      field('value', $._value),
    ),

    property_name: $ => choice(
      'description',
      'iterations',
      'warmup',
      'timeout',
      'requires',
      'order',
      'baseline',
      'mode',
      'targetTime',
      'sink',
      'outlierDetection',
      'cvThreshold',
      'count',
      'memory',
      'fairness',
      'fairnessSeed',
      'asyncSamplingPolicy',
      'asyncWarmupCap',
      'asyncSampleCap',
    ),

    _value: $ => choice(
      $.string,
      $.number,
      $.float,
      $.duration,
      $.boolean,
      $.identifier,
      $.string_array,
    ),

    // ============================================================
    // Language implementations
    // ============================================================

    language_implementation: $ => seq(
      field('language', $.language_tag),
      ':',
      $._code_or_inline,
    ),

    // Use identifier so new runtimes work without grammar changes.
    // Validation (Lang::from_str) happens in the convert layer.
    language_tag: $ => $.identifier,

    _code_or_inline: $ => choice(
      $.code_block,
      $.inline_code,
    ),

    // ============================================================
    // Code blocks
    // ============================================================

    // { embedded code with brace counting }
    code_block: $ => seq(
      '{',
      optional($.embedded_code),
      '}',
    ),

    // ( embedded code for Go imports )
    paren_code_block: $ => seq(
      '(',
      optional($.embedded_code),
      ')',
    ),

    // Single-line inline code - matches rest of line
    // Allows braces for things like: go: func() { ... }
    inline_code: $ => /[^\n\r]+/,

    // ============================================================
    // Literals
    // ============================================================

    string: $ => choice(
      seq('"', optional($.string_content), '"'),
      seq("'", optional($.single_string_content), "'"),
    ),

    string_content: $ => repeat1(choice(
      /[^"\\]+/,
      $.escape_sequence,
    )),

    single_string_content: $ => repeat1(choice(
      /[^'\\]+/,
      $.escape_sequence,
    )),

    escape_sequence: $ => /\\[nrt\\"']/,

    number: $ => /\d+/,

    float: $ => /\d+\.\d+/,

    duration: $ => seq(
      $.number,
      $.duration_unit,
    ),

    duration_unit: $ => choice('ms', 's', 'm'),

    boolean: $ => choice('true', 'false'),

    identifier: $ => /[a-zA-Z_][a-zA-Z0-9_]*/,

    string_array: $ => seq(
      '[',
      optional(seq(
        $.string,
        repeat(seq(',', $.string)),
        optional(','),
      )),
      ']',
    ),

    // ============================================================
    // Comments
    // ============================================================

    comment: $ => token(seq('#', /.*/)),
  },
});
