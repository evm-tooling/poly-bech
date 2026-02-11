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

const POLYBENCH_SNIPPET_NAME = '_polybench_snippet.ts';

function isUnderRoot(filePath: string, rootDir: string): boolean {
  const normalized = path.isAbsolute(filePath) ? filePath : path.resolve(rootDir, filePath);
  const rel = path.relative(rootDir, normalized);
  return rel !== '' && !rel.startsWith('..') && !path.isAbsolute(rel);
}

/**
 * @param tsModuleRoot - If set, module resolution runs from this dir (e.g. .polybench/runtime-env/ts) so imports like 'viem' resolve from node_modules.
 */
export function checkTSBlock(block: EmbeddedBlock, tsModuleRoot: string | null = null): TSDiagnostic[] {
  const diagnostics: TSDiagnostic[] = [];
  const code = block.code.trim();
  if (!code) return diagnostics;

  const isFixture = block.kind === 'fixture';
  const codeToCheck = isFixture ? FIXTURE_WRAP_PREFIX + code + FIXTURE_WRAP_SUFFIX : code;
  const wrapPrefixLen = isFixture ? FIXTURE_WRAP_PREFIX.length : 0;

  const useModuleRoot = tsModuleRoot != null && tsModuleRoot.length > 0 && fs.existsSync(tsModuleRoot);
  const fileName = useModuleRoot ? path.join(tsModuleRoot, POLYBENCH_SNIPPET_NAME) : 'snippet.ts';
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
    module: ts.ModuleKind.NodeNext,
    moduleResolution: ts.ModuleResolutionKind.NodeNext,
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
      if (useModuleRoot && isUnderRoot(name, tsModuleRoot)) {
        try {
          const full = path.isAbsolute(name) ? name : path.resolve(tsModuleRoot, name);
          if (fs.existsSync(full)) {
            const content = fs.readFileSync(full, 'utf8');
            return ts.createSourceFile(full, content, ts.ScriptTarget.Latest, true);
          }
        } catch {
          // ignore
        }
      }
      return undefined;
    },
    getDefaultLibFileName: () => defaultLibFileName,
    getCurrentDirectory: () => (useModuleRoot ? tsModuleRoot : ''),
    getDirectories: (d) => {
      if (!useModuleRoot) return [];
      const full = path.isAbsolute(d) ? d : path.resolve(tsModuleRoot, d);
      if (!isUnderRoot(full, tsModuleRoot)) return [];
      try {
        return fs.readdirSync(full, { withFileTypes: true })
          .filter((e) => e.isDirectory())
          .map((e) => e.name);
      } catch {
        return [];
      }
    },
    getCanonicalFileName: (f) => path.normalize(f),
    useCaseSensitiveFileNames: () => process.platform !== 'win32',
    getNewLine: () => '\n',
    fileExists: (f) => {
      if (f === fileName) return true;
      if (isLibFile(f)) return true;
      if (useModuleRoot) {
        const full = path.isAbsolute(f) ? f : path.resolve(tsModuleRoot, f);
        if (isUnderRoot(full, tsModuleRoot)) return fs.existsSync(full);
      }
      return false;
    },
    readFile: (f) => {
      if (f === fileName) return codeToCheck;
      if (isLibFile(f)) return readLibFile(f);
      if (useModuleRoot) {
        const full = path.isAbsolute(f) ? f : path.resolve(tsModuleRoot, f);
        if (isUnderRoot(full, tsModuleRoot) && fs.existsSync(full)) {
          try {
            return fs.readFileSync(full, 'utf8');
          } catch {
            // ignore
          }
        }
      }
      return undefined;
    },
    readDirectory: (d) => {
      if (!useModuleRoot) return [];
      const full = path.isAbsolute(d) ? d : path.resolve(tsModuleRoot, d);
      if (!isUnderRoot(full, tsModuleRoot)) return [];
      try {
        return fs.readdirSync(full);
      } catch {
        return [];
      }
    },
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
    const msg = ts.flattenDiagnosticMessageText(d.messageText, '\n');
    if (/Cannot find module 'node:/.test(msg)) continue;
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
      message: msg,
      severity: d.category === ts.DiagnosticCategory.Error ? 'error' : 'warning',
    });
  }
  return diagnostics;
}
