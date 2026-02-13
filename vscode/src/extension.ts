/**
 * Poly-Bench VS Code Extension
 *
 * This extension provides language support for .bench files by connecting
 * to the poly-bench-lsp language server (written in Rust).
 *
 * Features provided by the language server:
 * - Diagnostics (parse errors, validation, embedded Go/TS checking)
 * - Hover information for keywords and identifiers
 * - Code completion
 * - Semantic tokens for enhanced highlighting
 * - Document formatting
 */

import * as path from 'path';
import * as fs from 'fs';
import {
  workspace,
  ExtensionContext,
  window,
  WorkspaceEdit,
  Range,
  TextEdit,
} from 'vscode';

import {
  CloseAction,
  ErrorAction,
  LanguageClient,
  LanguageClientOptions,
  Message,
  ServerOptions,
  TransportKind,
} from 'vscode-languageclient/node';

let client: LanguageClient | undefined;
/** Resolves when the LSP is ready (so format-on-save can run). */
let clientReady: Promise<void> | null = null;

/**
 * Find the poly-bench-lsp binary.
 *
 * Search order:
 * 1. User-configured path (poly-bench.lspPath setting)
 * 2. Bundled binary in extension (if packaged)
 * 3. PATH lookup=
 */
function findLspBinary(context: ExtensionContext): string | null {
  // 1. Check user configuration
  const configured = workspace
    .getConfiguration('poly-bench')
    .get<string>('lspPath');
  if (configured && configured.trim().length > 0) {
    if (fs.existsSync(configured)) {
      return configured;
    }
    window.showWarningMessage(
      `Configured poly-bench.lspPath not found: ${configured}`
    );
  }

  // 2. Check bundled binary in extension
  const bundledPaths = [
    path.join(context.extensionPath, 'bin', 'poly-bench-lsp'),
    path.join(context.extensionPath, 'bin', 'poly-bench-lsp.exe'),
    path.join(context.extensionPath, 'poly-bench-lsp'),
    path.join(context.extensionPath, 'poly-bench-lsp.exe'),
  ];

  for (const bundled of bundledPaths) {
    if (fs.existsSync(bundled)) {
      return bundled;
    }
  }

  // 3. Check if poly-bench-lsp is on PATH
  // We'll just return the name and let the OS find it
  // The server options will handle if it's not found
  return 'poly-bench-lsp';
}

export function activate(context: ExtensionContext): void {
  const serverPath = findLspBinary(context);

  if (!serverPath) {
    window.showErrorMessage(
      'poly-bench-lsp binary not found. Please install it or set poly-bench.lspPath.'
    );
    return;
  }

  // Log which LSP binary we're using so you can verify after reload
  const isAbsolute = path.isAbsolute(serverPath);
  const label = isAbsolute
    ? serverPath.includes('/target/debug/')
      ? 'debug'
      : serverPath.includes('/target/release/')
        ? 'release'
        : 'custom'
    : 'PATH';
  const outputChannel = window.createOutputChannel('Poly-Bench LSP');
  outputChannel.appendLine(`[startup] Using LSP (${label}): ${serverPath}`);
  console.log(`[Poly-Bench] Using LSP: ${label} → ${serverPath}`);

  // Server options - run the LSP binary
  const serverOptions: ServerOptions = {
    run: {
      command: serverPath,
      transport: TransportKind.stdio,
    },
    debug: {
      command: serverPath,
      transport: TransportKind.stdio,
    },
  };

  // Client options - configure which documents to sync and error handling
  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: 'file', language: 'polybench' }],
    synchronize: {
      // Watch for changes to .bench files
      fileEvents: workspace.createFileSystemWatcher('**/*.bench'),
    },
    outputChannel,
    outputChannelName: 'Poly-Bench LSP',
    // Configure error handling for graceful recovery
    errorHandler: {
      error: (error: Error, message: Message | undefined, count: number | undefined) => {
        // Log the error but continue running
        console.error(`[Poly-Bench LSP] Error #${count}: ${error.message}`);
        if (message) {
          console.error(`[Poly-Bench LSP] Message: ${JSON.stringify(message)}`);
        }
        // Continue running instead of shutting down on transient errors
        return { action: ErrorAction.Continue };
      },
      closed: () => {
        // Restart the server when it closes unexpectedly
        console.log('[Poly-Bench LSP] Server closed, attempting restart...');
        return { action: CloseAction.Restart };
      },
    },
    // Increase the restart threshold (default is 5 restarts in 3 minutes)
    connectionOptions: {
      maxRestartCount: 10, // Allow more restarts before giving up
    },
  };

  // Create the language client
  client = new LanguageClient(
    'polyBenchLsp',
    'Poly-Bench Language Server',
    serverOptions,
    clientOptions
  );

  // Start the client and keep a promise so format-on-save can wait for readiness
  clientReady = client.start();

  type LspTextEdit = {
    range: {
      start: { line: number; character: number };
      end: { line: number; character: number };
    };
    newText: string;
  };

  /** Run LSP document formatting and return LSP-style edits. */
  async function doFormat(uri: string): Promise<LspTextEdit[] | null> {
    if (!client) return null;
    return client.sendRequest<LspTextEdit[] | null>('textDocument/formatting', {
      textDocument: { uri },
      options: { tabSize: 4, insertSpaces: true },
    });
  }

  // Format on save: run when poly-bench.formatOnSave is true OR when
  // editor.formatOnSave is true for polybench and our extension is the default formatter
  context.subscriptions.push(
    workspace.onWillSaveTextDocument((e) => {
      const doc = e.document;
      if (doc.languageId !== 'polybench' && !doc.uri.fsPath.endsWith('.bench')) {
        return;
      }

      const polyBenchConfig = workspace.getConfiguration('poly-bench');
      const editorConfig = workspace.getConfiguration('editor', doc.uri);
      const formatOnSavePoly = polyBenchConfig.get<boolean>('formatOnSave');
      const formatOnSaveEditor = editorConfig.get<boolean>('formatOnSave');
      const defaultFormatter = editorConfig.get<string>('defaultFormatter');

      const shouldFormat =
        formatOnSavePoly ||
        (formatOnSaveEditor &&
          (!defaultFormatter || defaultFormatter === 'evm-tooling.poly-bench'));

      if (!shouldFormat || !client || !clientReady) return;

      e.waitUntil(
        clientReady
          .then(() => doFormat(doc.uri.toString()))
          .then((edits) => {
            if (edits && edits.length > 0) {
              const we = new WorkspaceEdit();
              // Replace the entire current document with the formatted text. Do not use the
              // LSP's range: it was computed for the document version the LSP had, and a
              // race can leave trailing content (e.g. an extra "}") if the user edited
              // after the LSP's last did_change.
              const lastLine = Math.max(0, doc.lineCount - 1);
              const fullRange =
                doc.lineCount === 0
                  ? new Range(0, 0, 0, 0)
                  : new Range(
                      0,
                      0,
                      lastLine,
                      doc.lineAt(lastLine).text.length
                    );
              const singleEdit = new TextEdit(fullRange, edits[0].newText);
              we.set(doc.uri, [singleEdit]);
              return workspace.applyEdit(we);
            }
          })
          .catch((err) => {
            console.error('[Poly-Bench] Format on save failed:', err);
            window.showWarningMessage(
              `Poly-Bench format on save failed: ${err instanceof Error ? err.message : String(err)}`
            );
          })
      );
    })
  );

  context.subscriptions.push({
    dispose: () => {
      if (client) {
        client.stop();
      }
    },
  });
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
