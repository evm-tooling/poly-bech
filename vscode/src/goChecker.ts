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

export function checkGoBlock(block: EmbeddedBlock): GoDiagnostic[] {
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

  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), 'polybench-go-'));
  const tmpFile = path.join(tmpDir, 'main.go');
  try {
    fs.writeFileSync(tmpFile, wrapped, 'utf8');
    execSync('go build -o /dev/null ./main.go 2>&1', {
      cwd: tmpDir,
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
      diagnostics.push({
        startOffset,
        endOffset,
        startLine: block.startLine - 1 + snippetLine,
        startColumn: col,
        message: message!.trim(),
        severity: 'error',
      });
    }
  } finally {
    try {
      fs.rmSync(tmpDir, { recursive: true });
    } catch {
      // ignore
    }
  }
  return diagnostics;
}
