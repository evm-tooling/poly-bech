/**
 * Simple parser for .bench files: structure validation and extraction of
 * embedded Go/TS block ranges for downstream diagnostics.
 */

export interface BenchSyntaxError {
  line: number;   // 0-based
  column: number;
  message: string;
}

export interface EmbeddedBlock {
  lang: 'go' | 'ts';
  kind: 'setup' | 'fixture';
  startLine: number;
  endLine: number;
  startOffset: number;
  endOffset: number;
  code: string;
}

export interface BenchParseResult {
  errors: BenchSyntaxError[];
  embedded: EmbeddedBlock[];
}

const SUITE_RE = /^\s*(suite)\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\{\s*$/;
const SETUP_GO_RE = /^\s*(setup)\s+(go)\s*\{\s*$/;
const SETUP_TS_RE = /^\s*(setup)\s+(ts)\s*\{\s*$/;
const FIXTURE_RE = /^\s*(fixture)\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\{\s*$/;
const BENCH_RE = /^\s*(bench)\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\{\s*$/;
const GO_BLOCK_RE = /^\s*(go)\s*:\s*\{\s*$/;
const TS_BLOCK_RE = /^\s*(ts)\s*:\s*\{\s*$/;
/** True when line is only a closing } with indent exactly equal to the block's opening indent. */
function isBlockClosingLine(line: string, blockIndent: string): boolean {
  if (!/^\s*\}\s*$/.test(line)) return false;
  const leading = line.slice(0, line.length - line.trimStart().length);
  return leading === blockIndent;
}

function findUnescapedBraces(line: string): { open: number[]; close: number[] } {
  const open: number[] = [];
  const close: number[] = [];
  let inString: '"' | "'" | null = null;
  let i = 0;
  while (i < line.length) {
    const c = line[i];
    if (inString) {
      if (c === '\\' && i + 1 < line.length) i++;
      else if (c === inString) inString = null;
      i++;
      continue;
    }
    if (c === '"' || c === "'") inString = c;
    else if (c === '{') open.push(i);
    else if (c === '}') close.push(i);
    i++;
  }
  return { open, close };
}

export function parseBench(content: string): BenchParseResult {
  const errors: BenchSyntaxError[] = [];
  const embedded: EmbeddedBlock[] = [];
  const lines = content.split(/\r?\n/);
  const stack: { kind: string; line: number; indent: string }[] = [];
  let suiteStartLine: number | null = null;

  function docOffset(lineIndex: number, col: number): number {
    let o = 0;
    for (let i = 0; i < lineIndex && i < lines.length; i++) o += lines[i].length + 1;
    return o + Math.min(col, lines[lineIndex]?.length ?? 0);
  }

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    const trimmed = line.trimStart();
    const leadingSpaces = line.length - trimmed.length;
    const indent = line.slice(0, leadingSpaces);
    const { open: openBraces, close: closeBraces } = findUnescapedBraces(line);

    // Closing brace(s) – pop stack and possibly end embedded block
    for (let k = 0; k < closeBraces.length; k++) {
      if (stack.length === 0) {
        errors.push({ line: i, column: closeBraces[k], message: 'Unexpected }' });
        continue;
      }
      const top = stack[stack.length - 1];
      if (top.kind === 'suite') {
        if (trimmed === '}' || /^\s*\}\s*$/.test(line)) {
          stack.pop();
          if (stack.length === 0) suiteStartLine = null;
        }
        continue;
      }
      if (top.kind === 'setup-go' || top.kind === 'setup-ts') {
        if (isBlockClosingLine(line, top.indent)) {
          const startLine = top.line;
          const startOffset = docOffset(startLine, 0);
          const endOffset = docOffset(i, 0);
          const code = content.slice(startOffset, endOffset).replace(/\s+$/, '');
          embedded.push({
            lang: top.kind === 'setup-go' ? 'go' : 'ts',
            kind: 'setup',
            startLine: startLine + 1,
            endLine: i,
            startOffset,
            endOffset,
            code,
          });
          stack.pop();
        }
        continue;
      }
      if (top.kind === 'fixture-go' || top.kind === 'fixture-ts') {
        if (isBlockClosingLine(line, top.indent)) {
          const startLine = top.line;
          const startOffset = docOffset(startLine, 0);
          const endOffset = docOffset(i, 0);
          const code = content.slice(startOffset, endOffset).replace(/\s+$/, '');
          embedded.push({
            lang: top.kind === 'fixture-go' ? 'go' : 'ts',
            kind: 'fixture',
            startLine: startLine + 1,
            endLine: i,
            startOffset,
            endOffset,
            code,
          });
          stack.pop();
        }
        continue;
      }
      // Inner block of bench (ts: { ... } or go: { ... }) closes with same indent as opening
      if (top.kind === 'bench-go' || top.kind === 'bench-ts') {
        if (isBlockClosingLine(line, top.indent)) {
          stack.pop();
        }
        continue;
      }
      if (top.kind === 'fixture' || top.kind === 'bench') {
        if (trimmed === '}' || /^\s*\}\s*$/.test(line)) stack.pop();
      }
    }

    // Skip if we're inside an embedded block (stack top is go/ts block)
    const insideEmbedded = stack.length > 0 && (
      stack[stack.length - 1].kind === 'setup-go' ||
      stack[stack.length - 1].kind === 'setup-ts' ||
      stack[stack.length - 1].kind === 'fixture-go' ||
      stack[stack.length - 1].kind === 'fixture-ts'
    );
    if (insideEmbedded) continue;

    // Opening patterns
    if (SUITE_RE.test(line)) {
      if (stack.length > 0)
        errors.push({ line: i, column: 0, message: "Nested 'suite' not allowed" });
      else suiteStartLine = i;
      stack.push({ kind: 'suite', line: i, indent });
      continue;
    }
    if (SETUP_GO_RE.test(line)) {
      if (stack.length === 0 || stack[stack.length - 1].kind !== 'suite')
        errors.push({ line: i, column: 0, message: "'setup go' must be inside a suite" });
      stack.push({ kind: 'setup-go', line: i + 1, indent }); // content starts next line
      continue;
    }
    if (SETUP_TS_RE.test(line)) {
      if (stack.length === 0 || stack[stack.length - 1].kind !== 'suite')
        errors.push({ line: i, column: 0, message: "'setup ts' must be inside a suite" });
      stack.push({ kind: 'setup-ts', line: i + 1, indent });
      continue;
    }
    if (FIXTURE_RE.test(line)) {
      if (stack.length === 0 || stack[stack.length - 1].kind !== 'suite')
        errors.push({ line: i, column: 0, message: "'fixture' must be inside a suite" });
      stack.push({ kind: 'fixture', line: i, indent });
      continue;
    }
    if (BENCH_RE.test(line)) {
      if (stack.length === 0 || stack[stack.length - 1].kind !== 'suite')
        errors.push({ line: i, column: 0, message: "'bench' must be inside a suite" });
      stack.push({ kind: 'bench', line: i, indent });
      continue;
    }
    if (GO_BLOCK_RE.test(line)) {
      if (stack.length > 0 && stack[stack.length - 1].kind === 'fixture')
        stack.push({ kind: 'fixture-go', line: i + 1, indent });
      else if (stack.length > 0 && stack[stack.length - 1].kind === 'bench')
        stack.push({ kind: 'bench-go', line: i + 1, indent });
      continue;
    }
    if (TS_BLOCK_RE.test(line)) {
      if (stack.length > 0 && stack[stack.length - 1].kind === 'fixture')
        stack.push({ kind: 'fixture-ts', line: i + 1, indent });
      else if (stack.length > 0 && stack[stack.length - 1].kind === 'bench')
        stack.push({ kind: 'bench-ts', line: i + 1, indent });
      continue;
    }
  }

  if (stack.length > 0) {
    const top = stack[stack.length - 1];
    errors.push({
      line: top.line,
      column: 0,
      message: `Unclosed ${top.kind.replace('-', ' ')} block`,
    });
  }

  return { errors, embedded };
}
