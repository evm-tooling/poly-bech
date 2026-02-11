/**
 * Validates embedded TypeScript/JavaScript code using the TypeScript compiler API
 * (syntax + semantic) and maps diagnostics to document offsets.
 */

import * as fs from 'fs';
import * as path from 'path';
import * as ts from 'typescript';
import type { EmbeddedBlock } from './benchParser';

export interface TSDiagnostic {
  startOffset: number;
  endOffset: number;
  startLine: number;
  startColumn: number;
  message: string;
  severity: 'error' | 'warning';
}

const libCache = new Map<string, string>();
let typescriptLibDir: string | null = null;
function getTypescriptLibDir(): string | null {
  if (typescriptLibDir !== null) return typescriptLibDir;
  try {
    // require.resolve('typescript') -> .../node_modules/typescript/lib/typescript.js
    typescriptLibDir = path.dirname(require.resolve('typescript'));
  } catch {
    typescriptLibDir = '';
  }
  return typescriptLibDir || null;
}
/** Resolve requested name (e.g. "es2020.d.ts") to actual file in typescript/lib (e.g. "lib.es2020.d.ts"). */
function resolveLibFileName(requested: string): string {
  const base = path.basename(requested);
  if (base.startsWith('lib.') && base.endsWith('.d.ts')) return base;
  if (base.endsWith('.d.ts') && (/^es\d{4}/.test(base) || /^esnext/.test(base) || base.startsWith('dom') || base.startsWith('scripthost')))
    return 'lib.' + base;
  if (base.endsWith('.d.ts')) return 'lib.' + base;
  return '';
}
function readLibFile(requestedFileName: string): string {
  const actualName = resolveLibFileName(requestedFileName) || path.basename(requestedFileName);
  let content = libCache.get(actualName);
  if (content !== undefined) return content;
  const libDir = getTypescriptLibDir();
  if (!libDir || !actualName.endsWith('.d.ts')) {
    libCache.set(actualName, '');
    return '';
  }
  try {
    content = fs.readFileSync(path.join(libDir, actualName), 'utf8');
  } catch {
    content = '';
  }
  libCache.set(actualName, content);
  return content;
}
function isLibFile(name: string): boolean {
  const base = path.basename(name);
  if ((base.startsWith('lib.') || base === 'lib.d.ts') && base.endsWith('.d.ts')) return true;
  if (base.endsWith('.d.ts') && (/^es\d{4}/.test(base) || /^esnext/.test(base) || base.startsWith('dom') || base.startsWith('scripthost'))) return true;
  return false;
}

const FIXTURE_WRAP_PREFIX = 'function __fixture(): unknown {\n';
const FIXTURE_WRAP_SUFFIX = '\n}';

export function checkTSBlock(block: EmbeddedBlock): TSDiagnostic[] {
  const diagnostics: TSDiagnostic[] = [];
  const code = block.code.trim();
  if (!code) return diagnostics;

  const isFixture = block.kind === 'fixture';
  const codeToCheck = isFixture ? FIXTURE_WRAP_PREFIX + code + FIXTURE_WRAP_SUFFIX : code;
  const wrapPrefixLen = isFixture ? FIXTURE_WRAP_PREFIX.length : 0;

  const fileName = 'snippet.ts';
  const source = ts.createSourceFile(
    fileName,
    codeToCheck,
    ts.ScriptTarget.Latest,
    true,
    ts.ScriptKind.TS
  );
  const compilerOptions: ts.CompilerOptions = {
    skipLibCheck: true,
    noEmit: true,
    target: ts.ScriptTarget.ES2020,
    lib: ['es2020'],
  };
  const libDir = getTypescriptLibDir();
  const rootFiles = [fileName];
  if (libDir) {
    rootFiles.push(path.join(libDir, 'lib.es5.d.ts'));
    rootFiles.push(path.join(libDir, 'lib.es2020.d.ts'));
  }
  const defaultLibFileName = ts.getDefaultLibFileName(compilerOptions);
  const host: ts.CompilerHost = {
    getSourceFile: (name) => {
      if (name === fileName) return source;
      if (isLibFile(name)) {
        const content = readLibFile(name);
        if (!content) return undefined;
        return ts.createSourceFile(name, content, ts.ScriptTarget.Latest, true);
      }
      return undefined;
    },
    getDefaultLibFileName: () => defaultLibFileName,
    getCurrentDirectory: () => '',
    getDirectories: () => [],
    getCanonicalFileName: (f) => f,
    useCaseSensitiveFileNames: () => true,
    getNewLine: () => '\n',
    fileExists: (f) => f === fileName || isLibFile(f),
    readFile: (f) => (isLibFile(f) ? readLibFile(f) : undefined),
    readDirectory: () => [],
    writeFile: () => {},
  };
  const program = ts.createProgram(rootFiles, compilerOptions, host);
  const allDiagnostics = [
    ...program.getSyntacticDiagnostics(source),
    ...program.getSemanticDiagnostics(source),
  ];

  const sourceLines = codeToCheck.split(/\r?\n/);
  function offsetToLineColInSource(offset: number): { line: number; column: number } {
    let o = 0;
    for (let i = 0; i < sourceLines.length; i++) {
      const lineLen = sourceLines[i].length + 1;
      if (o + lineLen > offset) return { line: i, column: offset - o };
      o += lineLen;
    }
    return { line: sourceLines.length - 1, column: sourceLines[sourceLines.length - 1]?.length ?? 0 };
  }

  for (const d of allDiagnostics) {
    if (d.file !== source) continue;
    const start = d.start ?? 0;
    const length = d.length ?? 0;
    const { line: relLine, column: relCol } = offsetToLineColInSource(start);
    const snippetLine = isFixture && relLine >= 1 ? relLine - 1 : relLine;
    const startInSnippet = Math.max(0, start - wrapPrefixLen);
    const endInSnippet = Math.min(code.length, Math.max(0, start + length - wrapPrefixLen));
    const docLine = block.startLine - 1 + snippetLine;
    const startOffset = block.startOffset + startInSnippet;
    const endOffset = block.startOffset + endInSnippet;
    diagnostics.push({
      startOffset,
      endOffset,
      startLine: docLine,
      startColumn: relCol,
      message: ts.flattenDiagnosticMessageText(d.messageText, '\n'),
      severity: d.category === ts.DiagnosticCategory.Error ? 'error' : 'warning',
    });
  }
  return diagnostics;
}
