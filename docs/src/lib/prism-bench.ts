/**
 * Prism.js language definition for poly-bench .bench files.
 * Must use the same Prism instance as prism-react-renderer so Highlight finds the grammar.
 * Register by importing this module before highlighting: import "@/lib/prism-bench"
 */

import { Prism } from 'prism-react-renderer'

const benchKeywords = [
  // Top-level / blocks
  'use',
  'suite',
  'setup',
  'fixture',
  'bench',
  'globalSetup',
  // Setup sections
  'import',
  'declare',
  'init',
  'helpers',
  'async',
  // Hooks
  'before',
  'after',
  'each',
  'skip',
  'validate',
]

const benchProperties = [
  // Core suite + bench config
  'description',
  'iterations',
  'warmup',
  'timeout',
  'requires',
  'order',
  'compare',
  'baseline',
  'mode',
  'targetTime',
  'minIterations',
  'maxIterations',
  'sink',
  'outlierDetection',
  'cvThreshold',
  'count',
  'memory',
  'concurrency',
  // Fixture + hooks
  'hex',
  'shape',
  'tags',
  // Chart params
  'title',
  'output',
  'xlabel',
  'sortBy',
  'sortOrder',
  'timeUnit',
  'legendPosition',
  'regressionStyle',
  'regressionModel',
  'yScale',
  'baselineBenchmark',
  'filterWinner',
  'chartMode',
  'theme',
  'showStats',
  'showConfig',
  'showWinCounts',
  'showGeoMean',
  'showDistribution',
  'showMemory',
  'showTotalTime',
  'showLegend',
  'showGrid',
  'showErrorBars',
  'showRegression',
  'showRegressionLabel',
  'showRSquared',
  'showEquation',
  'showRegressionBand',
  'showMinorGrid',
  'showVerticalGrid',
  'roundTicks',
  'compact',
  'width',
  'height',
  'precision',
  'limit',
  'titleFontSize',
  'subtitleFontSize',
  'axisLabelFontSize',
  'tickLabelFontSize',
  'ciLevel',
  'minSpeedup',
  'axisThickness',
  'yAxisMin',
  'yAxisMax',
  'gridOpacity',
  'minorGridOpacity',
  'errorBarOpacity',
  'errorBarThickness',
  'regressionBandOpacity',
  'symlogThreshold',
  'includeBenchmarks',
  'excludeBenchmarks',
  'fork',
]

const benchValues = [
  'auto',
  'fixed',
  'sequential',
  'parallel',
  'random',
  'performance',
  'throughput',
  'linear',
  'log',
  'symlog',
  'percent',
  'dark',
  'light',
  'asc',
  'desc',
]

const languageTags = ['go', 'ts', 'typescript', 'rust', 'python', 'py']

// Fallback keyword set for embedded code blocks inside bench files.
// This does not fully parse each language, but it provides good visual coverage.
const runtimeKeywords = [
  // Go
  'func',
  'var',
  'const',
  'type',
  'struct',
  'interface',
  'package',
  'defer',
  'go',
  'select',
  'chan',
  'range',
  'map',
  'make',
  'new',
  'if',
  'else',
  'switch',
  'case',
  'default',
  'for',
  'break',
  'continue',
  'return',
  'fallthrough',
  // TypeScript / JavaScript
  'let',
  'const',
  'function',
  'class',
  'extends',
  'implements',
  'interface',
  'type',
  'enum',
  'namespace',
  'import',
  'from',
  'export',
  'new',
  'await',
  'async',
  'yield',
  'try',
  'catch',
  'finally',
  'throw',
  'if',
  'else',
  'switch',
  'case',
  'default',
  'for',
  'while',
  'do',
  'break',
  'continue',
  'return',
  // Rust
  'fn',
  'let',
  'mut',
  'pub',
  'crate',
  'mod',
  'impl',
  'trait',
  'struct',
  'enum',
  'match',
  'use',
  'where',
  'unsafe',
  'async',
  'await',
  'move',
  'if',
  'else',
  'loop',
  'while',
  'for',
  'in',
  'break',
  'continue',
  'return',
  // Python
  'def',
  'class',
  'lambda',
  'if',
  'elif',
  'else',
  'for',
  'while',
  'in',
  'try',
  'except',
  'finally',
  'raise',
  'import',
  'from',
  'as',
  'with',
  'pass',
  'break',
  'continue',
  'return',
  'yield',
]

const joinWords = (words: string[]) => words.join('|')
const runtimeControlWords = [
  'if',
  'else',
  'switch',
  'case',
  'default',
  'for',
  'while',
  'loop',
  'match',
  'return',
  'break',
  'continue',
  'throw',
  'catch',
  'try',
  'finally',
  'new',
]

Prism.languages.bench = {
  comment: {
    pattern: /(?:#|\/\/).*$/m,
    greedy: true,
  },

  string: {
    pattern: /"(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*'/,
    greedy: true,
  },

  'file-ref': {
    pattern: /@file\s*\(/,
    alias: 'function',
  },

  number: {
    pattern: /\b\d+(?:\.\d+)?(?:ms|s|m)?\b/,
    greedy: true,
  },

  boolean: {
    pattern: /\b(?:true|false)\b/,
    greedy: true,
  },

  'use-statement': {
    pattern: /\buse\s+std::[a-zA-Z_][a-zA-Z0-9_]*/,
    inside: {
      keyword: /\buse\b/,
      namespace: /\bstd\b/,
      punctuation: /::/,
      'class-name': /[a-zA-Z_][a-zA-Z0-9_]*$/,
    },
  },

  'language-tag': {
    pattern: new RegExp(`\\b(?:${joinWords(languageTags)})\\b(?=\\s*:)`),
    alias: 'builtin',
  },

  'setup-language': {
    pattern: new RegExp(`\\bsetup\\s+(?:${joinWords(languageTags)})\\b`),
    inside: {
      keyword: /\bsetup\b/,
      builtin: new RegExp(`\\b(?:${joinWords(languageTags)})\\b`),
    },
  },

  'stdlib-call': {
    pattern: /\b(?:anvil|charting|constants)\.[a-zA-Z_][a-zA-Z0-9_]*(?=\s*\()/,
    inside: {
      namespace: /^(?:anvil|charting|constants)/,
      punctuation: /\./,
      function: /[a-zA-Z_][a-zA-Z0-9_]*$/,
    },
  },

  function: {
    pattern: /\b(?:spawnAnvil|stopAnvil|drawTable|drawSpeedupChart)\b(?=\s*\()/,
    greedy: true,
  },

  'function-declaration': {
    pattern: /\b(?:func|fn|function)\s+[a-zA-Z_][a-zA-Z0-9_]*(?=\s*(?:\(|<))/,
    inside: {
      keyword: /\b(?:func|fn|function)\b/,
      function: /[a-zA-Z_][a-zA-Z0-9_]*$/,
    },
  },

  'method-call': {
    pattern: /\b[a-zA-Z_][a-zA-Z0-9_]*\.[a-zA-Z_][a-zA-Z0-9_]*(?=\s*\()/,
    inside: {
      punctuation: /\./,
      function: /[a-zA-Z_][a-zA-Z0-9_]*$/,
    },
  },

  'function-call': {
    pattern: new RegExp(
      `\\b(?!${joinWords(runtimeControlWords)}\\b)[a-zA-Z_][a-zA-Z0-9_]*(?=\\s*\\()`,
    ),
    alias: 'function',
  },

  'type-identifier': {
    pattern: /\b[A-Z][a-zA-Z0-9_]*\b/,
    alias: 'class-name',
  },

  property: {
    pattern: new RegExp(`\\b(?:${joinWords(benchProperties)})\\b\\s*:`),
    greedy: true,
    inside: {
      property: /^[a-zA-Z_][a-zA-Z0-9_]*/,
      punctuation: /:$/,
    },
  },

  keyword: {
    pattern: new RegExp(`\\b(?:${joinWords(benchKeywords)})\\b`),
    greedy: true,
  },

  constant: {
    pattern: new RegExp(`\\b(?:${joinWords(benchValues)})\\b`),
    greedy: true,
  },

  // Runtime fallback coloring for embedded code snippets.
  'runtime-keyword': {
    pattern: new RegExp(`\\b(?:${joinWords(runtimeKeywords)})\\b`),
    alias: 'keyword',
  },

  'runtime-type': {
    pattern:
      /\b(?:string|number|boolean|bigint|any|unknown|void|byte|rune|int|int8|int16|int32|int64|uint|uint8|uint16|uint32|uint64|uintptr|float32|float64|u8|u16|u32|u64|usize|i8|i16|i32|i64|isize|f32|f64|Uint8Array|Buffer|Promise|Context|Client)\b/,
    alias: 'class-name',
  },

  'runtime-constant': {
    pattern: /\b(?:nil|null|undefined|None|Some|Ok|Err|true|false)\b/,
    alias: 'constant',
  },

  operator: {
    pattern: /=>|->|::|[-+*/%!=<>|&^~]+/,
  },

  punctuation: {
    pattern: /[{}()[\].,:;@]/,
  },
}

export function registerPrismBench() {}
