/**
 * Validates embedded Go code by writing a temporary file and running `go build`.
 * Maps compiler errors back to document offsets.
 */

import * as fs from 'fs';
import * as path from 'path';
import * as os from 'os';
import { execSync } from 'child_process';
import type { EmbeddedBlock } from './benchParser';

export interface GoDiagnostic {
  startOffset: number;
  endOffset: number;
  startLine: number;
  startColumn: number;
  message: string;
  severity: 'error' | 'warning';
}

const GO_ERROR_RE = /^([^:]+):(\d+):(\d+):\s*(.+)$/m;

/** Strip "package command-line-arguments: " prefix; return null if message is only that (so caller can skip). */
function sanitizeGoMessage(msg: string): string | null {
  const out = msg.replace(/^#?\s*package\s+command-line-arguments\s*:\s*/i, '').trim();
  if (!out || /^#?\s*command-line-arguments\s*$/i.test(out)) return null;
  return out;
}

/**
 * @param goModRoot - If set, build is run from this directory (must contain go.mod) so imports resolve.
 */
export function checkGoBlock(block: EmbeddedBlock, goModRoot: string | null = null): GoDiagnostic[] {
  const diagnostics: GoDiagnostic[] = [];
  const code = block.code.trim();
  if (!code) return diagnostics;

  let wrapped: string;
  const isFixture = block.kind === 'fixture';
  const hasPackage = /\bpackage\s+\w+/.test(code);
  const hasFunc = /\bfunc\s+/.test(code);
  if (isFixture) {
    wrapped = `package main\n\nfunc __fixture() []byte {\n${code}\n}`;
  } else if (hasPackage && hasFunc) {
    wrapped = code;
  } else if (hasFunc) {
    wrapped = `package main\n\n${code}`;
  } else {
    wrapped = `package main\n\nfunc main() {\n${code}\n}`;
  }

  const useModRoot = goModRoot != null && goModRoot.length > 0 && fs.existsSync(path.join(goModRoot, 'go.mod'));
  // When using module root: build from a temp subdir inside it so the build dir has exactly one .go file (avoids "no Go files")
  const workDir = useModRoot
    ? path.join(goModRoot, `_polybench_lint_${process.pid}`)
    : fs.mkdtempSync(path.join(os.tmpdir(), 'polybench-go-'));
  if (useModRoot) {
    fs.mkdirSync(workDir, { recursive: true });
  }
  const checkFile = path.join(workDir, 'main.go');
  try {
    fs.writeFileSync(checkFile, wrapped, 'utf8');
    execSync('go build -o /dev/null ./main.go 2>&1', {
      cwd: workDir,
      encoding: 'utf8',
      maxBuffer: 1024 * 1024,
    });
  } catch (err: unknown) {
    const stderr = err && typeof err === 'object' && 'stderr' in err
      ? String((err as { stderr?: unknown }).stderr)
      : err instanceof Error ? err.message : String(err);
    const stdout = err && typeof err === 'object' && 'stdout' in err
      ? String((err as { stdout?: unknown }).stdout)
      : '';
    const output = stderr || stdout;
    if (!useModRoot && /go\.mod file not found|no required module provides package/i.test(output)) {
      return diagnostics;
    }
    const headerLines = wrapped === code ? 0 : isFixture ? 3 : wrapped.includes('func main()') ? 3 : 2;
    const lines = block.code.split(/\r?\n/);
    let match: RegExpExecArray | null;
    const regex = new RegExp(GO_ERROR_RE.source, 'gm');
    while ((match = regex.exec(output)) !== null) {
      const [, , lineStr, colStr, message] = match;
      const tempLine = parseInt(lineStr!, 10) - 1;
      const col = parseInt(colStr!, 10) - 1;
      const snippetLine = tempLine - headerLines;
      if (snippetLine < 0 || snippetLine >= lines.length) continue;
      let snippetOffset = 0;
      for (let i = 0; i < snippetLine; i++) snippetOffset += (lines[i]?.length ?? 0) + 1;
      const lineContent = lines[snippetLine] ?? '';
      const startOffset = block.startOffset + snippetOffset;
      const endOffset = block.startOffset + snippetOffset + Math.min(Math.max(0, col) + 1, lineContent.length);
      const sanitized = sanitizeGoMessage(message!);
      if (sanitized) {
        diagnostics.push({
          startOffset,
          endOffset,
          startLine: block.startLine - 1 + snippetLine,
          startColumn: col,
          message: sanitized,
          severity: 'error',
        });
      }
    }
    // Build failed but no file:line:col lines (e.g. "no required module provides package") – report on first line of block
    if (diagnostics.length === 0 && output.trim().length > 0) {
      const firstLine = lines[0] ?? '';
      const summary = output.split(/\r?\n/).find((l) => l.trim().length > 0)?.trim() ?? output.trim();
      const sanitized = sanitizeGoMessage(summary.length > 120 ? summary.slice(0, 117) + '...' : summary);
      if (sanitized) {
        diagnostics.push({
          startOffset: block.startOffset,
          endOffset: block.startOffset + Math.min(firstLine.length, 1),
          startLine: block.startLine - 1,
          startColumn: 0,
          message: sanitized,
          severity: 'error',
        });
      }
    }
  } finally {
    try {
      fs.rmSync(workDir, { recursive: true });
    } catch {
      // ignore
    }
  }
  return diagnostics;
}
