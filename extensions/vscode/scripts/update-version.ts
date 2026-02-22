#!/usr/bin/env node
/**
 * Simple script to update the version in package.json without building or publishing.
 * 
 * Usage:
 *   npx ts-node ./scripts/update-version.ts 0.0.34
 *   VERSION=0.0.34 npx ts-node ./scripts/update-version.ts
 */
import { readFile, writeFile } from "node:fs/promises";
import path from "node:path";

function parseVersion(argv: string[]): string | undefined {
  // supports: VERSION=0.0.1
  const kv = argv.find((a) => a.startsWith("VERSION="));
  if (kv) return kv.slice("VERSION=".length);

  // supports: --version 0.0.1
  const idx = argv.indexOf("--version");
  if (idx !== -1 && argv[idx + 1]) return argv[idx + 1];

  // supports: positional: 0.0.1
  const pos = argv.find((a) => /^\d+\.\d+\.\d+(-[0-9A-Za-z.-]+)?$/.test(a));
  if (pos) return pos;

  return undefined;
}

async function main() {
  const argv = process.argv.slice(2);
  const version = parseVersion(argv) ?? process.env.VERSION;

  if (!version) {
    console.error("Usage:");
    console.error("  npx ts-node ./scripts/update-version.ts 0.0.34");
    console.error("  VERSION=0.0.34 npx ts-node ./scripts/update-version.ts");
    process.exit(1);
  }

  if (!/^\d+\.\d+\.\d+(-[0-9A-Za-z.-]+)?$/.test(version)) {
    console.error(`Invalid version: "${version}"`);
    process.exit(1);
  }

  const root = process.cwd();
  const pkgPath = path.join(root, "package.json");

  const pkgRaw = await readFile(pkgPath, "utf8");
  const pkg = JSON.parse(pkgRaw) as { version?: string };

  const oldVersion = pkg.version ?? "(missing)";
  pkg.version = version;

  await writeFile(pkgPath, JSON.stringify(pkg, null, 2) + "\n", "utf8");
  console.log(`✅ package.json version: ${oldVersion} → ${version}`);
}

main().catch((err) => {
  console.error("❌ Failed:", err instanceof Error ? err.message : err);
  process.exit(1);
});
