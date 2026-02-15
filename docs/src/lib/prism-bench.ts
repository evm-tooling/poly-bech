/**
 * Prism.js language definition for poly-bench .bench files.
 * Must use the same Prism instance as prism-react-renderer so Highlight finds the grammar.
 * Register by importing this module before highlighting: import "@/lib/prism-bench"
 */

import { Prism } from "prism-react-renderer";

Prism.languages.bench = {
  comment: {
    pattern: /#.*$/m,
    greedy: true,
  },

  string: {
    pattern: /"(?:[^"\\]|\\.)*"/,
    greedy: true,
  },

  number: {
    pattern: /\b\d+(?:\.\d+)?(?:ms|s|m)?\b/,
    greedy: true,
  },

  boolean: {
    pattern: /\b(?:true|false)\b/,
    greedy: true,
  },

  constant: {
    pattern: /\b(?:auto|fixed|sequential|let|from|return|parallel|random)\b/,
    greedy: true,
  },

  "use-statement": {
    pattern: /\buse\s+std::[a-zA-Z_][a-zA-Z0-9_]*/,
    inside: {
      keyword: /\buse\b/,
      punctuation: /::/,
      "class-name": /[a-zA-Z_][a-zA-Z0-9_]*$/,
    },
  },

  keyword: {
    pattern: /\b(?:suite|setup|bench|function|fixture|before|after|each|globalSetup|import|init|helpers|declare|async|skip|validate)\b/,
    greedy: true,
  },

  builtin: {
    pattern: /\b(?:go|ts|rust|keccak256Bench|Hasher|Keccak|hasher)\b/,
    greedy: true,
  },

  "stdlib-call": {
    pattern: /\b(?:anvil|charting|constants|math)\.[a-zA-Z_][a-zA-Z0-9_]*/,
    inside: {
      "class-name": /^(?:anvil|charting|constants|math)/,
      punctuation: /^\./,
      function: /[a-zA-Z_][a-zA-Z0-9_]*$/,
    },
  },

  function: {
    pattern: /\b(?:spawnAnvil|keccak256_rust|keccak256Go|keccak256Ts|tiny_keccak|stopAnvil|drawBarChart|drawLineChart|drawPieChart|drawScatterPlot|drawHistogram|drawHeatmap|drawBoxPlot|drawAreaChart)\s*\(?/,
    greedy: true,
  },

  property: {
    pattern: /\b(?:description|iterations|warmup|minIterations|maxIterations|concurrency|count|timeout|targetTime|cvThreshold|requires|tags|order|mode|compare|sink|memory|outlierDetection|baseline|hex|shape|title|xlabel|ylabel|sortBy|sortOrder|timeUnit|showTotalTime|showLegend|showGrid|benchmark)\s*:/,
    greedy: true,
    inside: {
      property: /^[a-zA-Z_][a-zA-Z0-9_]*/,
      punctuation: /:$/,
    },
  },

  punctuation: {
    pattern: /[{}()[\].,:;]/,
  },
};

export function registerPrismBench() {}
