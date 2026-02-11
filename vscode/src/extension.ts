import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';
import { parseBench } from './benchParser';
import { checkGoBlock } from './goChecker';
import { checkTSBlock } from './tsChecker';
import { analyzeSemantics } from './benchSemantic';

/** Default path to Go module root (where go.mod lives), relative to project root. Matches poly-bench CLI layout. */
const GO_MOD_ROOT_RELATIVE = '.polybench/runtime-env/go';
/** Default path to TS runtime env (package.json / node_modules), relative to project root. */
const TS_MODULE_ROOT_RELATIVE = '.polybench/runtime-env/ts';

const POLYBENCH_TOML = 'polybench.toml';

const isBenchFile = (doc: vscode.TextDocument) =>
  doc.uri.scheme === 'file' && doc.fileName.endsWith('.bench');

/** Walk up from the document's directory to find a folder containing polybench.toml. */
function findProjectRoot(doc: vscode.TextDocument): string | null {
  let dir = path.dirname(doc.fileName);
  const root = path.parse(doc.fileName).root;
  while (dir !== root) {
    if (fs.existsSync(path.join(dir, POLYBENCH_TOML))) return dir;
    const parent = path.dirname(dir);
    if (parent === dir) break;
    dir = parent;
  }
  return null;
}

/** Walk up from dir looking for a child .polybench/runtime-env/go with go.mod. */
function findGoModRootByWalking(startDir: string): string | null {
  let dir = startDir;
  const root = path.parse(startDir).root;
  while (dir !== root) {
    const goDir = path.join(dir, GO_MOD_ROOT_RELATIVE);
    if (fs.existsSync(path.join(goDir, 'go.mod'))) return goDir;
    const parent = path.dirname(dir);
    if (parent === dir) break;
    dir = parent;
  }
  return null;
}

function getGoModRoot(doc: vscode.TextDocument): string | null {
  const projectRoot = findProjectRoot(doc);
  const workspaceFolder = vscode.workspace.getWorkspaceFolder(doc.uri);
  const baseDir = projectRoot ?? workspaceFolder?.uri.fsPath ?? path.dirname(doc.fileName);
  const configured = vscode.workspace.getConfiguration('poly-bench').get<string>('goModuleRoot');
  let candidate: string;
  if (configured) {
    candidate = path.isAbsolute(configured) ? configured : path.join(baseDir, configured);
  } else {
    candidate = path.join(baseDir, GO_MOD_ROOT_RELATIVE);
  }
  if (fs.existsSync(path.join(candidate, 'go.mod'))) return candidate;
  const fromWalk = findGoModRootByWalking(path.dirname(doc.fileName));
  if (fromWalk) return fromWalk;
  // Fallback: search workspace folder(s) for any .polybench/runtime-env/go with go.mod (e.g. examples/viem-simple when editing vscode/hash.bench)
  const found = findFirstGoModInWorkspace();
  return found;
}

/** Search workspace folders for any directory that has .polybench/runtime-env/go/go.mod (max depth 5). */
function findFirstGoModInWorkspace(): string | null {
  const folders = vscode.workspace.workspaceFolders;
  if (!folders) return null;
  const maxDepth = 5;
  function search(dir: string, depth: number): string | null {
    if (depth > maxDepth) return null;
    const goDir = path.join(dir, GO_MOD_ROOT_RELATIVE);
    if (fs.existsSync(path.join(goDir, 'go.mod'))) return goDir;
    try {
      const entries = fs.readdirSync(dir, { withFileTypes: true });
      for (const e of entries) {
        if (e.isDirectory() && !e.name.startsWith('.') && e.name !== 'node_modules') {
          const next = path.join(dir, e.name);
          const hit = search(next, depth + 1);
          if (hit) return hit;
        }
      }
    } catch {
      // ignore
    }
    return null;
  }
  for (const f of folders) {
    const hit = search(f.uri.fsPath, 0);
    if (hit) return hit;
  }
  return null;
}

function getTSModuleRoot(doc: vscode.TextDocument): string | null {
  const projectRoot = findProjectRoot(doc);
  const workspaceFolder = vscode.workspace.getWorkspaceFolder(doc.uri);
  const baseDir = projectRoot ?? workspaceFolder?.uri.fsPath ?? path.dirname(doc.fileName);
  const candidate = path.join(baseDir, TS_MODULE_ROOT_RELATIVE);
  const hasPackage = fs.existsSync(path.join(candidate, 'package.json'));
  const hasNodeModules = fs.existsSync(path.join(candidate, 'node_modules'));
  return hasPackage || hasNodeModules ? candidate : null;
}

export function activate(context: vscode.ExtensionContext): void {
  const collection = vscode.languages.createDiagnosticCollection('polybench');
  const output = vscode.window.createOutputChannel('Poly Bench');
  context.subscriptions.push(collection, output);

  function updateDiagnostics(doc: vscode.TextDocument): void {
    if (!isBenchFile(doc)) {
      collection.delete(doc.uri);
      return;
    }
    const diagnostics: vscode.Diagnostic[] = [];
    const text = doc.getText();
    const parseResult = parseBench(text);
    const { errors, embedded } = parseResult;

    for (const e of errors) {
      const line = doc.lineAt(Math.min(e.line, doc.lineCount - 1));
      const range = new vscode.Range(e.line, e.column, e.line, Math.max(e.column, line.range.end.character));
      diagnostics.push(
        new vscode.Diagnostic(range, e.message, vscode.DiagnosticSeverity.Error)
      );
    }

    const goModRoot = getGoModRoot(doc);
    const tsModuleRoot = getTSModuleRoot(doc);
    const goBlocks = embedded.filter((b) => b.lang === 'go');
    if (goBlocks.length > 0) {
      output.appendLine(`Go: checking ${goBlocks.length} block(s), module root: ${goModRoot ?? 'none'}`);
    }
    for (const block of embedded) {
      if (block.lang === 'go') {
        try {
          const goDiags = checkGoBlock(block, goModRoot);
          for (const d of goDiags) {
            const range = new vscode.Range(
              d.startLine,
              d.startColumn,
              d.startLine,
              doc.positionAt(d.endOffset).character
            );
            diagnostics.push(
              new vscode.Diagnostic(
                range,
                d.message,
                d.severity === 'error' ? vscode.DiagnosticSeverity.Error : vscode.DiagnosticSeverity.Warning
              )
            );
          }
          if (goModRoot === null && block.code.trim().length > 0) {
            const firstLine = doc.lineAt(block.startLine - 1);
            diagnostics.push(
              new vscode.Diagnostic(
                new vscode.Range(block.startLine - 1, 0, block.startLine - 1, Math.max(1, firstLine.range.end.character)),
                'Go import checking disabled: no module root. Set poly-bench.goModuleRoot or use a project with .polybench/runtime-env/go.',
                vscode.DiagnosticSeverity.Hint
              )
            );
          }
        } catch (err) {
          const msg = err instanceof Error ? err.message : String(err);
          output.appendLine(`Go check skipped (install Go and ensure \'go\' is on PATH): ${msg}`);
          const firstLine = doc.lineAt(block.startLine - 1);
          diagnostics.push(
            new vscode.Diagnostic(
              new vscode.Range(block.startLine - 1, 0, block.startLine - 1, Math.max(1, firstLine.range.end.character)),
              `Go check skipped: ${msg}`,
              vscode.DiagnosticSeverity.Warning
            )
          );
        }
      } else {
        try {
          for (const d of checkTSBlock(block, tsModuleRoot)) {
            const startPos = doc.positionAt(d.startOffset);
            const endPos = doc.positionAt(d.endOffset);
            diagnostics.push(
              new vscode.Diagnostic(
                new vscode.Range(startPos, endPos),
                d.message,
                d.severity === 'error' ? vscode.DiagnosticSeverity.Error : vscode.DiagnosticSeverity.Warning
              )
            );
          }
        } catch (err) {
          const msg = err instanceof Error ? err.message : String(err);
          output.appendLine(`TypeScript check failed: ${msg}`);
        }
      }
    }

    for (const s of analyzeSemantics(text, parseResult)) {
      const line = Math.min(s.line, doc.lineCount - 1);
      const lineEnd = s.endLine !== undefined ? Math.min(s.endLine, doc.lineCount - 1) : line;
      const range = new vscode.Range(
        line,
        s.column,
        lineEnd,
        s.endColumn !== undefined ? s.endColumn : doc.lineAt(line).range.end.character
      );
      diagnostics.push(
        new vscode.Diagnostic(
          range,
          s.message,
          s.severity === 'error' ? vscode.DiagnosticSeverity.Error : vscode.DiagnosticSeverity.Warning
        )
      );
    }

    collection.set(doc.uri, diagnostics);
  }

  const trigger = (doc: vscode.TextDocument) => {
    if (isBenchFile(doc)) updateDiagnostics(doc);
  };

  for (const doc of vscode.workspace.textDocuments) trigger(doc);
  context.subscriptions.push(
    vscode.workspace.onDidChangeTextDocument((e) => trigger(e.document)),
    vscode.workspace.onDidOpenTextDocument(trigger)
  );
}

export function deactivate(): void {}
