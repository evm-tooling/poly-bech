import * as vscode from 'vscode';
import { parseBench } from './benchParser';
import { checkGoBlock } from './goChecker';
import { checkTSBlock } from './tsChecker';
import { analyzeSemantics } from './benchSemantic';

export function activate(context: vscode.ExtensionContext): void {
  const collection = vscode.languages.createDiagnosticCollection('polybench');
  context.subscriptions.push(collection);

  function updateDiagnostics(doc: vscode.TextDocument): void {
    if (doc.languageId !== 'polybench') {
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

    for (const block of embedded) {
      if (block.lang === 'go') {
        try {
          for (const d of checkGoBlock(block)) {
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
        } catch {
          // Go not installed or other runtime error; skip Go diagnostics
        }
      } else {
        for (const d of checkTSBlock(block)) {
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
    if (doc.uri.scheme === 'file' && doc.fileName.endsWith('.bench')) updateDiagnostics(doc);
  };

  for (const doc of vscode.workspace.textDocuments) trigger(doc);
  context.subscriptions.push(
    vscode.workspace.onDidChangeTextDocument((e) => { if (e.document.uri.scheme === 'file') trigger(e.document); }),
    vscode.workspace.onDidOpenTextDocument(trigger)
  );
}

export function deactivate(): void {}
