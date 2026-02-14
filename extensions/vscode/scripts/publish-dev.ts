#!/usr/bin/env node
import { readFile, writeFile, readdir, unlink } from "node:fs/promises";
import path from "node:path";
import { spawn } from "node:child_process";

function run(cmd: string, args: string[], cwd: string): Promise<void> {
  return new Promise((resolve, reject) => {
    const p = spawn(cmd, args, {
      cwd,
      stdio: "inherit",
      shell: process.platform === "win32",
    });
    p.on("error", reject);
    p.on("exit", (code) => {
      if (code === 0) resolve();
      else reject(new Error(`${cmd} ${args.join(" ")} failed with code ${code}`));
    });
  });
}

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
  console.log("🚀 publish:dev starting…");

  const argv = process.argv.slice(2);
  const version = parseVersion(argv) ?? process.env.VERSION;

  if (!version) {
    console.error("Usage:");
    console.error("  npm run publish:dev -- VERSION=0.0.22");
    console.error("  npm run publish:dev -- --version 0.0.22");
    console.error("  VERSION=0.0.22 npm run publish:dev");
    process.exit(1);
  }

  if (!/^\d+\.\d+\.\d+(-[0-9A-Za-z.-]+)?$/.test(version)) {
    console.error(`Invalid version: "${version}"`);
    process.exit(1);
  }

  const root = process.cwd(); // run from extensions/vscode/ folder
  const pkgPath = path.join(root, "package.json");

  // 1) Update package.json
  const pkgRaw = await readFile(pkgPath, "utf8");
  const pkg = JSON.parse(pkgRaw) as { version?: string };

  const oldVersion = pkg.version ?? "(missing)";
  pkg.version = version;

  await writeFile(pkgPath, JSON.stringify(pkg, null, 2) + "\n", "utf8");
  console.log(`✅ package.json version: ${oldVersion} → ${version}`);

  // 2) Delete old vsix
  const files = await readdir(root);
  const vsixFiles = files.filter((f) => f.endsWith(".vsix"));
  for (const f of vsixFiles) {
    await unlink(path.join(root, f));
    console.log(`🗑️  deleted ${f}`);
  }

  // 3) npm install
  console.log("📦 npm install…");
  await run("npm", ["install"], root);

  // 4) compile
  console.log("🔨 npm run compile…");
  await run("npm", ["run", "compile"], root);

  // 5) vsce package
  console.log("📦 vsce package…");
  await run("npx", ["--yes", "@vscode/vsce", "package"], root);

  console.log("✅ Done.");
}

main().catch((err) => {
  console.error("❌ publish:dev failed:", err instanceof Error ? err.stack ?? err.message : err);
  process.exit(1);
});
