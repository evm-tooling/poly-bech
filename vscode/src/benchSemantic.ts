/**
 * Semantic analysis for .bench files: cross-reference setup functions and
 * fixture names with bench expressions, flag unknown/unused symbols.
 */

import type { BenchParseResult, EmbeddedBlock } from './benchParser';

export interface SemanticDiagnostic {
  line: number;
  column: number;
  endLine?: number;
  endColumn?: number;
  message: string;
  severity: 'error' | 'warning';
}

interface FixtureInfo {
  name: string;
  hasHex: boolean;
  line: number;
}

interface BenchCall {
  benchName: string;
  lang: 'go' | 'ts';
  expr: string;
  line: number;
  exprStartColumn: number;
  callee: string;
  calleeColumn: number;
  args: string[];
}

const SUITE_RE = /^\s*suite\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\{\s*$/;
const SETUP_GO_RE = /^\s*setup\s+go\s*\{\s*$/;
const SETUP_TS_RE = /^\s*setup\s+ts\s*\{\s*$/;
const FIXTURE_RE = /^\s*fixture\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\{\s*$/;
const FIXTURE_HEX_RE = /^\s*hex\s*:/;
const BENCH_RE = /^\s*bench\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\{\s*$/;
const GO_LINE_RE = /^\s*go\s*:\s*(.+)$/;
const TS_LINE_RE = /^\s*ts\s*:\s*(.+)$/;
const KEY_RE = /^\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*:\s*/;

const SUITE_KEYS = new Set(['description', 'iterations', 'warmup']);
const FIXTURE_KEYS = new Set(['description', 'hex', 'go', 'ts']);
const BENCH_KEYS = new Set(['description', 'iterations', 'go', 'ts']);

function closestKey(typo: string, valid: Set<string>): string | undefined {
  const list = [...valid];
  if (list.length === 0) return undefined;
  let best = list[0];
  let bestScore = Infinity;
  const a = typo.toLowerCase();
  for (const b of list) {
    const bLower = b.toLowerCase();
    if (a === bLower) return b;
    let score = Math.abs(a.length - bLower.length);
    const len = Math.min(a.length, bLower.length);
    for (let i = 0; i < len; i++) if (a[i] !== bLower[i]) score++;
    score += Math.max(0, bLower.length - len) + Math.max(0, a.length - len);
    if (score < bestScore) {
      bestScore = score;
      best = b;
    }
  }
  return bestScore <= 5 ? best : undefined;
}
const GO_FUNC_RE = /\bfunc\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(/g;
const TS_IMPORT_RE = /import\s+\{\s*([^}]+)\}\s+from\s+['"]/g;
const TS_IMPORT_RE_DEFAULT = /import\s+(\w+)\s+from\s+['"]/g;

function extractGoFunctionNames(code: string): Set<string> {
  const names = new Set<string>();
  let m: RegExpExecArray | null;
  while ((m = GO_FUNC_RE.exec(code)) !== null) names.add(m[1]);
  return names;
}

function extractTsNames(code: string): Set<string> {
  const names = new Set<string>();
  let m: RegExpExecArray | null;
  const re1 = new RegExp(TS_IMPORT_RE.source, 'g');
  while ((m = re1.exec(code)) !== null) {
    m[1].split(',').forEach((s) => {
      const t = s.trim().split(/\s+as\s+/)[0].trim();
      if (t) names.add(t);
    });
  }
  const re2 = new RegExp(TS_IMPORT_RE_DEFAULT.source, 'g');
  while ((m = re2.exec(code)) !== null) names.add(m[1]);
  return names;
}

function parseCallExpr(expr: string): { callee: string; args: string[] } | null {
  const trimmed = expr.trim();
  const paren = trimmed.indexOf('(');
  if (paren < 0 || !trimmed.endsWith(')')) return null;
  const callee = trimmed.slice(0, paren).trim();
  const argsStr = trimmed.slice(paren + 1, trimmed.length - 1).trim();
  if (!/^[a-zA-Z_][a-zA-Z0-9_]*$/.test(callee)) return null;
  const args = argsStr ? argsStr.split(',').map((a) => a.trim()).filter(Boolean) : [];
  return { callee, args };
}

function collectFixturesAndBenchCalls(content: string): { fixtures: FixtureInfo[]; calls: BenchCall[] } {
  const lines = content.split(/\r?\n/);
  const fixtures: FixtureInfo[] = [];
  const calls: BenchCall[] = [];
  let inBench = false;
  let benchName = '';
  let inFixture = false;
  let fixtureHasHex = false;
  let fixtureStack = 0;
  let braceDepth = 0;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    const trimmed = line.trimStart();
    const indent = line.length - trimmed.length;

    const fixMatch = line.match(FIXTURE_RE);
    if (fixMatch) {
      inFixture = true;
      fixtureHasHex = false;
      fixtureStack = 1;
      fixtures.push({ name: fixMatch[1], hasHex: false, line: i });
      continue;
    }
    if (inFixture) {
      if (FIXTURE_HEX_RE.test(line)) {
        fixtureHasHex = true;
        const last = fixtures[fixtures.length - 1];
        if (last) last.hasHex = true;
      }
      for (const c of line) {
        if (c === '{') fixtureStack++;
        else if (c === '}') fixtureStack--;
      }
      if (fixtureStack <= 0) inFixture = false;
      continue;
    }

    const benchMatch = line.match(BENCH_RE);
    if (benchMatch) {
      inBench = true;
      benchName = benchMatch[1];
      braceDepth = 1;
      continue;
    }
    if (inBench) {
      for (const c of line) {
        if (c === '{') braceDepth++;
        else if (c === '}') braceDepth--;
      }
      const goMatch = line.match(GO_LINE_RE);
      if (goMatch) {
        const expr = goMatch[1].trim();
        const parsed = parseCallExpr(expr);
        if (parsed) {
          const goCol = line.indexOf('go:');
          const afterLabel = line.slice(goCol + 3);
          const rest = afterLabel.trimStart();
          const exprStart = goCol + 3 + (afterLabel.length - rest.length);
          const calleeOffset = rest.indexOf(parsed.callee);
          calls.push({
            benchName,
            lang: 'go',
            expr,
            line: i,
            exprStartColumn: exprStart,
            callee: parsed.callee,
            calleeColumn: exprStart + (calleeOffset >= 0 ? calleeOffset : 0),
            args: parsed.args,
          });
        }
      }
      const tsMatch = line.match(TS_LINE_RE);
      if (tsMatch) {
        const expr = tsMatch[1].trim();
        const parsed = parseCallExpr(expr);
        if (parsed) {
          const tsCol = line.indexOf('ts:');
          const afterLabel = line.slice(tsCol + 3);
          const rest = afterLabel.trimStart();
          const exprStart = tsCol + 3 + (afterLabel.length - rest.length);
          const calleeOffset = rest.indexOf(parsed.callee);
          calls.push({
            benchName,
            lang: 'ts',
            expr,
            line: i,
            exprStartColumn: exprStart,
            callee: parsed.callee,
            calleeColumn: exprStart + (calleeOffset >= 0 ? calleeOffset : 0),
            args: parsed.args,
          });
        }
      }
      if (braceDepth <= 0) inBench = false;
    }
  }
  return { fixtures, calls };
}

type BlockContext = 'suite' | 'setup-go' | 'setup-ts' | 'fixture' | 'fixture-go' | 'fixture-ts' | 'bench';

function collectKeyDiagnostics(content: string): SemanticDiagnostic[] {
  const diagnostics: SemanticDiagnostic[] = [];
  const lines = content.split(/\r?\n/);
  const stack: BlockContext[] = [];
  let fixtureBraceDepth = 0;
  let benchBraceDepth = 0;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    const trimmed = line.trimStart();
    const keyMatch = line.match(KEY_RE);

    if (SUITE_RE.test(line)) {
      stack.push('suite');
      continue;
    }
    if (SETUP_GO_RE.test(line)) {
      stack.push('setup-go');
      continue;
    }
    if (SETUP_TS_RE.test(line)) {
      stack.push('setup-ts');
      continue;
    }
    if (FIXTURE_RE.test(line)) {
      stack.push('fixture');
      fixtureBraceDepth = 1;
      continue;
    }
    if (BENCH_RE.test(line)) {
      stack.push('bench');
      benchBraceDepth = 1;
      continue;
    }

    if (stack.length > 0) {
      const top = stack[stack.length - 1];
      if (top === 'fixture') {
        for (const c of line) {
          if (c === '{') fixtureBraceDepth++;
          else if (c === '}') fixtureBraceDepth--;
        }
        if (/^\s*go\s*:\s*\{\s*$/.test(line)) {
          stack.push('fixture-go');
          continue;
        }
        if (/^\s*ts\s*:\s*\{\s*$/.test(line)) {
          stack.push('fixture-ts');
          continue;
        }
      }
      if (top === 'bench') {
        for (const c of line) {
          if (c === '{') benchBraceDepth++;
          else if (c === '}') benchBraceDepth--;
        }
      }
      if (top === 'setup-go' || top === 'setup-ts') {
        if (/^\s{2,4}\}\s*$/.test(line)) stack.pop();
        continue;
      }
      if (top === 'fixture-go' || top === 'fixture-ts') {
        if (/^\s{4,8}\}\s*$/.test(line)) {
          stack.pop();
          fixtureBraceDepth--;
        }
        continue;
      }
      if (top === 'fixture' && fixtureBraceDepth <= 0) stack.pop();
      if (top === 'bench' && benchBraceDepth <= 0) stack.pop();
      if (top === 'suite' && /^\s*\}\s*$/.test(line)) stack.pop();
    }

    const inEmbedded = stack.length > 0 && (
      stack[stack.length - 1] === 'setup-go' ||
      stack[stack.length - 1] === 'setup-ts' ||
      stack[stack.length - 1] === 'fixture-go' ||
      stack[stack.length - 1] === 'fixture-ts'
    );
    if (inEmbedded || !keyMatch) continue;

    const key = keyMatch[1];
    const keyCol = (keyMatch.index ?? 0) + (keyMatch[0].indexOf(key));
    const keyEndCol = keyCol + key.length;
    const ctx = stack[stack.length - 1];
    const valid = ctx === 'suite' ? SUITE_KEYS : ctx === 'fixture' ? FIXTURE_KEYS : ctx === 'bench' ? BENCH_KEYS : null;
    if (!valid) continue;
    if (valid.has(key)) continue;

    const suggestion = closestKey(key, valid);
    const msg = suggestion
      ? `Unknown key '${key}'. Did you mean '${suggestion}'?`
      : `Unknown key '${key}'. Valid keys here: ${[...valid].sort().join(', ')}.`;
    diagnostics.push({
      line: i,
      column: keyCol,
      endColumn: keyEndCol,
      message: msg,
      severity: 'error',
    });
  }
  return diagnostics;
}

export function analyzeSemantics(content: string, parseResult: BenchParseResult): SemanticDiagnostic[] {
  const diagnostics: SemanticDiagnostic[] = [];
  const { embedded } = parseResult;
  const { fixtures, calls } = collectFixturesAndBenchCalls(content);

  diagnostics.push(...collectKeyDiagnostics(content));

  const fixtureNames = new Set(fixtures.map((f) => f.name));
  const fixtureHexNames = new Set<string>();
  for (const f of fixtures) {
    if (f.hasHex) fixtureHexNames.add(f.name + '_hex');
  }
  const validGoArgs = new Set([...fixtureNames]);
  const validTsArgs = new Set([...fixtureNames, ...fixtureHexNames]);

  const goSetupFuncs = new Set<string>();
  const tsSetupNames = new Set<string>();
  for (const block of embedded) {
    if (block.kind !== 'setup') continue;
    if (block.lang === 'go') extractGoFunctionNames(block.code).forEach((n) => goSetupFuncs.add(n));
    else extractTsNames(block.code).forEach((n) => tsSetupNames.add(n));
  }

  const usedGoFuncs = new Set<string>();
  const usedTsNames = new Set<string>();
  const usedFixtures = new Set<string>();

  for (const call of calls) {
    const calleeLen = call.callee.length;
    if (call.lang === 'go') {
      if (!goSetupFuncs.has(call.callee)) {
        diagnostics.push({
          line: call.line,
          column: call.calleeColumn,
          endColumn: call.calleeColumn + calleeLen,
          message: `Unknown function '${call.callee}'. Did you mean one of: ${[...goSetupFuncs].join(', ') || 'none defined in setup go'}`,
          severity: 'error',
        });
      } else usedGoFuncs.add(call.callee);
      for (const arg of call.args) {
        if (!validGoArgs.has(arg)) {
          const argOffset = call.expr.indexOf(arg);
          const argCol = argOffset >= 0 ? call.exprStartColumn + argOffset : call.calleeColumn;
          diagnostics.push({
            line: call.line,
            column: argCol,
            endColumn: argCol + arg.length,
            message: `Unknown fixture or variable '${arg}'. Available: ${[...validGoArgs].join(', ') || 'none'}`,
            severity: 'error',
          });
        } else usedFixtures.add(arg);
      }
    } else {
      if (!tsSetupNames.has(call.callee)) {
        const suggestion = [...tsSetupNames].find((n) => n.startsWith(call.callee.slice(0, 2)) || call.callee.startsWith(n.slice(0, 2)));
        const hint = suggestion ? ` Did you mean '${suggestion}'?` : tsSetupNames.size ? ` Available: ${[...tsSetupNames].join(', ')}.` : ' No imports in setup ts.';
        diagnostics.push({
          line: call.line,
          column: call.calleeColumn,
          endColumn: call.calleeColumn + calleeLen,
          message: `Unknown function or identifier '${call.callee}'.${hint}`,
          severity: 'error',
        });
      } else usedTsNames.add(call.callee);
      for (const arg of call.args) {
        if (!validTsArgs.has(arg)) {
          const argOffset = call.expr.indexOf(arg);
          const argCol = argOffset >= 0 ? call.exprStartColumn + argOffset : call.calleeColumn;
          diagnostics.push({
            line: call.line,
            column: argCol,
            endColumn: argCol + arg.length,
            message: `Unknown fixture or variable '${arg}'. Available: ${[...validTsArgs].join(', ') || 'none'}`,
            severity: 'error',
          });
        }
        if (arg.endsWith('_hex')) usedFixtures.add(arg.replace(/_hex$/, ''));
        else usedFixtures.add(arg);
      }
    }
  }

  for (const f of goSetupFuncs) {
    if (!usedGoFuncs.has(f)) {
      diagnostics.push({
        line: 0,
        column: 0,
        message: `Function '${f}' is defined in setup go but never used in any bench`,
        severity: 'warning',
      });
    }
  }
  for (const f of tsSetupNames) {
    if (!usedTsNames.has(f)) {
      diagnostics.push({
        line: 0,
        column: 0,
        message: `'${f}' is imported in setup ts but never used in any bench`,
        severity: 'warning',
      });
    }
  }
  for (const f of fixtures) {
    if (!usedFixtures.has(f.name)) {
      diagnostics.push({
        line: f.line,
        column: 0,
        message: `Fixture '${f.name}' is never referenced in any bench`,
        severity: 'warning',
      });
    }
  }

  return diagnostics;
}
