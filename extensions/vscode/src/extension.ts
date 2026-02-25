/**
 * Poly-Bench VS Code Extension
 *
 * This extension provides language support for .bench files by connecting
 * to the poly-bench language server (run via `poly-bench lsp`).
 *
 * Features provided by the language server:
 * - Diagnostics (parse errors, validation, embedded Go/TS checking)
 * - Hover information for keywords and identifiers
 * - Code completion
 * - Semantic tokens for enhanced highlighting
 * - Document formatting
 *
 * The language server uses the current implementation with error-tolerant parsing.
 * Optional Tree-sitter WASM can be used for client-side highlighting.
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

import { languages } from 'vscode';
import {
  createTreeSitterSemanticTokensProvider,
  TREE_SITTER_LEGEND,
} from './treeSitterHighlighting';

let client: LanguageClient | undefined;
/** Resolves when the LSP is ready (so format-on-save can run). */
let clientReady: Promise<void> | null = null;

/** Tree-sitter parser and language (if enabled) */
let treeSitterParser: any = null;
let treeSitterLanguage: any = null;

/** Server launch: command and optional args (e.g. poly-bench with ['lsp']). */
interface LspServerSpec {
  command: string;
  args: string[];
}

/**
 * Check if Tree-sitter WASM highlighting should be used
 */
function shouldUseTreeSitter(): boolean {
  return workspace.getConfiguration('poly-bench').get<boolean>('useTreeSitterHighlighting', true);
}

/**
 * Initialize Tree-sitter WASM parser if enabled
 */
async function initTreeSitter(context: ExtensionContext): Promise<void> {
  if (!shouldUseTreeSitter()) {
    return;
  }

  try {
    const Parser = require('web-tree-sitter');
    await Parser.init();
    treeSitterParser = new Parser();

    const wasmPath = path.join(context.extensionPath, 'tree-sitter', 'tree-sitter-polybench.wasm');
    if (fs.existsSync(wasmPath)) {
      const lang = await Parser.Language.load(wasmPath);
      treeSitterParser.setLanguage(lang);
      treeSitterLanguage = lang;
      console.log('[Poly-Bench] Tree-sitter WASM initialized');
    } else {
      console.log('[Poly-Bench] Tree-sitter WASM not found, using TextMate grammar');
      treeSitterParser = null;
      treeSitterLanguage = null;
    }
  } catch (err) {
    console.error('[Poly-Bench] Failed to initialize Tree-sitter:', err);
    treeSitterParser = null;
  }
}

/**
 * Find the poly-bench LSP server.
 *
 * Search order:
 * 1. User-configured path (poly-bench.lspPath) - must be the poly-bench binary
 * 2. Bundled binary in extension: poly-bench
 * 3. PATH: poly-bench
 */
function findLspServer(context: ExtensionContext): LspServerSpec | null {
  const useLspSubcommand = (cmd: string): LspServerSpec => ({
    command: cmd,
    args: ['lsp'],
  });

  // 1. Check user configuration
  const configured = workspace
    .getConfiguration('poly-bench')
    .get<string>('lspPath');
  if (configured && configured.trim().length > 0) {
    if (fs.existsSync(configured)) {
      const base = path.basename(configured);
      const isMainBinary =
        base === 'poly-bench' || base === 'poly-bench.exe';
      if (isMainBinary) {
        return useLspSubcommand(configured);
      }
      window.showWarningMessage(
        'poly-bench.lspPath must point to the poly-bench binary.'
      );
      return null;
    }
    window.showWarningMessage(
      `Configured poly-bench.lspPath not found: ${configured}`
    );
  }

  // 2. Check bundled binaries
  const bundledPolyBench = [
    path.join(context.extensionPath, 'bin', 'poly-bench'),
    path.join(context.extensionPath, 'bin', 'poly-bench.exe'),
    path.join(context.extensionPath, 'poly-bench'),
    path.join(context.extensionPath, 'poly-bench.exe'),
  ];
  for (const bundled of bundledPolyBench) {
    if (fs.existsSync(bundled)) {
      return useLspSubcommand(bundled);
    }
  }

  // 3. PATH
  return useLspSubcommand('poly-bench');
}

export async function activate(context: ExtensionContext): Promise<void> {
  // Initialize Tree-sitter WASM if enabled
  await initTreeSitter(context);

  const spec = findLspServer(context);

  if (!spec) {
    window.showErrorMessage(
      'poly-bench not found. Install poly-bench or set poly-bench.lspPath.'
    );
    return;
  }

  // Log which LSP we're using so you can verify after reload
  const isAbsolute = path.isAbsolute(spec.command);
  const label = isAbsolute
    ? spec.command.includes('/target/debug/')
      ? 'debug'
      : spec.command.includes('/target/release/')
        ? 'release'
        : 'custom'
    : 'PATH';
  const outputChannel = window.createOutputChannel('Poly-Bench LSP');
  const desc =
    spec.args.length > 0
      ? `${spec.command} ${spec.args.join(' ')}`
      : spec.command;
  outputChannel.appendLine(`[startup] Using Poly-Bench LSP (${label}): ${desc}`);
  console.log(`[Poly-Bench] Using LSP: ${label} → ${desc}`);

  if (treeSitterParser) {
    outputChannel.appendLine('[startup] Tree-sitter WASM highlighting enabled');
  }

  // Register tree-sitter semantic token provider when enabled
  if (treeSitterParser && treeSitterLanguage) {
    const provider = createTreeSitterSemanticTokensProvider(
      context.extensionPath,
      treeSitterParser,
      treeSitterLanguage
    );
    if (provider) {
      context.subscriptions.push(
        languages.registerDocumentSemanticTokensProvider(
          { scheme: 'file', language: 'polybench' },
          provider,
          TREE_SITTER_LEGEND
        )
      );
    }
  }

  // Server options - run poly-bench lsp
  const serverOptions: ServerOptions = {
    run: {
      command: spec.command,
      args: spec.args,
      transport: TransportKind.stdio,
    },
    debug: {
      command: spec.command,
      args: spec.args,
      transport: TransportKind.stdio,
    },
  };

  // When tree-sitter highlighting is on, we provide semantic tokens client-side.
  // Disable LSP semantic tokens to avoid double highlighting.
  const useTreeSitter = treeSitterParser && treeSitterLanguage;
  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: 'file', language: 'polybench' }],
    middleware: useTreeSitter
      ? {
          provideDocumentSemanticTokens: () => undefined,
        }
      : undefined,
    synchronize: {
      // Watch for changes to .bench files and project configuration files
      // This helps the LSP detect when modules are installed/removed
      fileEvents: [
        workspace.createFileSystemWatcher('**/*.bench'),
        workspace.createFileSystemWatcher('**/package.json'),
        workspace.createFileSystemWatcher('**/Cargo.toml'),
        workspace.createFileSystemWatcher('**/go.mod'),
        workspace.createFileSystemWatcher('**/node_modules/.package-lock.json'),
        workspace.createFileSystemWatcher('**/Cargo.lock'),
        workspace.createFileSystemWatcher('**/go.sum'),
        workspace.createFileSystemWatcher('**/requirements.txt'),
      ],
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
