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
 */

import * as path from 'path';
import * as fs from 'fs';
import { workspace, ExtensionContext, window } from 'vscode';

import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from 'vscode-languageclient/node';

let client: LanguageClient | undefined;

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

  // Client options - configure which documents to sync
  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: 'file', language: 'polybench' }],
    synchronize: {
      // Watch for changes to .bench files
      fileEvents: workspace.createFileSystemWatcher('**/*.bench'),
    },
    outputChannelName: 'Poly-Bench LSP',
  };

  // Create the language client
  client = new LanguageClient(
    'polyBenchLsp',
    'Poly-Bench Language Server',
    serverOptions,
    clientOptions
  );

  // Start the client (this also starts the server)
  client.start();

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
