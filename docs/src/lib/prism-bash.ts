/**
 * Prism.js language definition for shell/Bash snippets.
 * Uses common Prism token aliases so existing code theme colors apply cleanly.
 */

import { Prism } from 'prism-react-renderer'

const shellBuiltins = [
  'alias',
  'bg',
  'bind',
  'break',
  'builtin',
  'cd',
  'command',
  'continue',
  'declare',
  'dirs',
  'disown',
  'echo',
  'eval',
  'exec',
  'exit',
  'export',
  'false',
  'fc',
  'fg',
  'getopts',
  'hash',
  'help',
  'history',
  'jobs',
  'kill',
  'let',
  'local',
  'mapfile',
  'popd',
  'printf',
  'pushd',
  'pwd',
  'read',
  'readonly',
  'return',
  'set',
  'shift',
  'source',
  'test',
  'times',
  'trap',
  'true',
  'type',
  'typeset',
  'ulimit',
  'umask',
  'unalias',
  'unset',
  'wait',
]

const shellKeywords = [
  'if',
  'then',
  'else',
  'elif',
  'fi',
  'case',
  'in',
  'esac',
  'for',
  'select',
  'while',
  'until',
  'do',
  'done',
  'function',
  'time',
  'coproc',
]

const join = (values: string[]) => values.join('|')

Prism.languages.bash = {
  comment: {
    pattern: /(^|[^\\])#.*/m,
    lookbehind: true,
    greedy: true,
  },

  string: {
    pattern: /"(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*'/,
    greedy: true,
  },

  variable: [
    {
      // ${VAR}, ${VAR:-default}, etc.
      pattern: /\$\{[^}]+\}/,
      greedy: true,
    },
    {
      // $VAR, $1, $?, $@
      pattern: /\$(?:[a-zA-Z_]\w*|\d+|[@*#?$!-])/,
      greedy: true,
    },
  ],

  option: {
    // --long-flag or -f
    pattern: /(^|\s)--?[a-zA-Z][\w-]*/,
    lookbehind: true,
    alias: 'attr-name',
  },

  path: {
    // ./foo, ../foo, /usr/bin, ~/dev/project
    pattern: /(^|\s)(?:~|\/|\.\.?(?=\/))(?:[^\s'"`|;&()\\]|\\.)+/,
    lookbehind: true,
    alias: 'url',
  },

  'command-substitution': {
    pattern: /\$\((?:[^)(]+|\([^)(]*\))*\)/,
    inside: {
      punctuation: /[()$]/,
    },
  },

  builtin: {
    pattern: new RegExp(`\\b(?:${join(shellBuiltins)})\\b`),
  },

  keyword: {
    pattern: new RegExp(`\\b(?:${join(shellKeywords)})\\b`),
  },

  command: {
    // First command word after control operators or line start.
    pattern: /(^|[;&|]\s*|\s)([a-zA-Z_][\w.-]*)(?=\s|$)/,
    lookbehind: true,
    alias: 'function',
  },

  number: {
    pattern: /\b\d+(?:\.\d+)?\b/,
  },

  operator: {
    pattern: /&&|\|\||\||;|<<-?|>>?|[<>]=?|=|!|\b(?:-eq|-ne|-lt|-le|-gt|-ge)\b/,
  },

  punctuation: {
    pattern: /[{}[\](),]/,
  },
}

// Share same grammar for shell aliases used in docs.
Prism.languages.shell = Prism.languages.bash
Prism.languages.sh = Prism.languages.bash
Prism.languages.zsh = Prism.languages.bash

export function registerPrismBash() {}
