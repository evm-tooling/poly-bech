/**
 * Tree-sitter based semantic token provider for poly-bench.
 * When useTreeSitterHighlighting is true, this provides client-side highlighting
 * from the tree-sitter parse tree and highlights.scm queries.
 */

import * as path from 'path';
import * as fs from 'fs';
import {
  DocumentSemanticTokensProvider,
  Range,
  SemanticTokensBuilder,
  SemanticTokensLegend,
  TextDocument,
} from 'vscode';

/** Map tree-sitter capture names to semantic token type indices */
const CAPTURE_TO_TYPE: Record<string, number> = {
  keyword: 0,
  'keyword.import': 0,
  'keyword.coroutine': 0,
  type: 1,
  'type.definition': 1,
  function: 2,
  'function.definition': 2,
  'function.builtin': 2,
  variable: 3,
  'variable.definition': 3,
  'variable.parameter': 4,
  string: 5,
  'string.escape': 5,
  'string.special': 5,
  number: 6,
  'number.float': 6,
  comment: 7,
  property: 8,
  module: 9,
  'constant.builtin': 3,
  'punctuation.delimiter': 10,
  'punctuation.bracket': 10,
  error: 11,
};

/** Token types for the legend (must match indices above) */
export const TREE_SITTER_TOKEN_TYPES = [
  'keyword',
  'type',
  'function',
  'variable',
  'parameter',
  'string',
  'number',
  'comment',
  'property',
  'namespace',
  'operator',
  'variable', // error - use variable as fallback
];

export const TREE_SITTER_TOKEN_MODIFIERS: string[] = ['definition', 'declaration'];

export const TREE_SITTER_LEGEND: SemanticTokensLegend = {
  tokenTypes: TREE_SITTER_TOKEN_TYPES,
  tokenModifiers: TREE_SITTER_TOKEN_MODIFIERS,
};

export function createTreeSitterSemanticTokensProvider(
  extensionPath: string,
  parser: any,
  lang: any
): DocumentSemanticTokensProvider | null {
  if (!parser || !lang) return null;

  const queriesPath = path.join(extensionPath, 'tree-sitter', 'queries', 'highlights.scm');
  if (!fs.existsSync(queriesPath)) {
    console.warn('[Poly-Bench] highlights.scm not found, tree-sitter highlighting disabled');
    return null;
  }

  let query: any;
  try {
    const querySource = fs.readFileSync(queriesPath, 'utf8');
    query = lang.query(querySource);
  } catch (err) {
    console.error('[Poly-Bench] Failed to load highlights query:', err);
    return null;
  }

  const provider: DocumentSemanticTokensProvider = {
    provideDocumentSemanticTokens(document: TextDocument) {
      const text = document.getText();
      if (!text) return new SemanticTokensBuilder(TREE_SITTER_LEGEND).build();

      let tree;
      try {
        tree = parser.parse(text);
      } catch (err) {
        console.error('[Poly-Bench] Tree-sitter parse failed:', err);
        return new SemanticTokensBuilder(TREE_SITTER_LEGEND).build();
      }

      if (!tree) return new SemanticTokensBuilder(TREE_SITTER_LEGEND).build();

      const root = tree.rootNode;
      const builder = new SemanticTokensBuilder(TREE_SITTER_LEGEND);

      try {
        const captures = query.captures(root);
        captures.sort((a: any, b: any) => {
          const aNode = a.node;
          const bNode = b.node;
          if (aNode.startPosition.row !== bNode.startPosition.row) {
            return aNode.startPosition.row - bNode.startPosition.row;
          }
          return aNode.startPosition.column - bNode.startPosition.column;
        });
        for (const cap of captures) {
          const typeIdx = CAPTURE_TO_TYPE[cap.name];
          if (typeIdx === undefined) continue;

          const node = cap.node;
          const startPos = node.startPosition;
          const endPos = node.endPosition;

          const startLine = startPos.row;
          const startChar = startPos.column;
          const endLine = endPos.row;
          const endChar = endPos.column;

          const range = new Range(startLine, startChar, endLine, endChar);
          const length = document.getText(range).length;

          let modifiers = 0;
          if (cap.name.includes('definition') || cap.name.includes('declaration')) {
            modifiers = 1; // definition
          }

          builder.push(startLine, startChar, length, typeIdx, modifiers);
        }
      } finally {
        tree.delete();
      }

      return builder.build();
    },
  };

  return provider;
}
