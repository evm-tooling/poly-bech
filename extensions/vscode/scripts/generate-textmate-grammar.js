#!/usr/bin/env node
/* eslint-disable no-console */
const fs = require("fs");
const path = require("path");

const ROOT = path.resolve(__dirname, "..");
const GRAMMAR_PATH = path.join(ROOT, "syntaxes", "polybench.tmLanguage.json");

/**
 * Keep this list aligned with DSL Lang metadata.
 * This script generates common TextMate blocks from one canonical shape.
 */
const LANGS = [
  {
    id: "go",
    aliases: ["go"],
    setupRegex: "go",
    blockRegex: "go",
    lineRegex: "go",
    sourceScope: "source.go",
    embeddedScope: "source.go.embedded.bench",
    importParenStyle: true,
    includeContentNameInSetupSections: false,
  },
  {
    id: "ts",
    aliases: ["ts", "typescript"],
    setupRegex: "ts",
    blockRegex: "ts",
    lineRegex: "ts",
    sourceScope: "source.ts",
    embeddedScope: "source.ts.embedded.bench",
    importParenStyle: false,
    includeContentNameInSetupSections: false,
  },
  {
    id: "rust",
    aliases: ["rust", "rs"],
    setupRegex: "rust",
    blockRegex: "rust",
    lineRegex: "rust",
    sourceScope: "source.rust",
    embeddedScope: "source.rust.embedded.bench",
    importParenStyle: false,
    includeContentNameInSetupSections: false,
  },
  {
    id: "python",
    aliases: ["python", "py"],
    setupRegex: "python|py",
    blockRegex: "python|py",
    lineRegex: "python|py",
    sourceScope: "source.python",
    embeddedScope: "source.python.embedded.bench",
    importParenStyle: false,
    includeContentNameInSetupSections: true,
  },
  {
    id: "csharp",
    aliases: ["csharp", "cs"],
    setupRegex: "csharp",
    blockRegex: "csharp",
    lineRegex: "csharp",
    sourceScope: "source.cs",
    embeddedScope: "source.csharp.embedded.bench",
    importParenStyle: false,
    includeContentNameInSetupSections: true,
  },
  // Keep C last so generated ordering stays closest to previous grammar diffs.
  {
    id: "c",
    aliases: ["c"],
    setupRegex: "c",
    blockRegex: "c",
    lineRegex: "c",
    sourceScope: "source.c",
    embeddedScope: "source.c.embedded.bench",
    importParenStyle: false,
    includeContentNameInSetupSections: true,
  },
];

function escapeRegex(text) {
  return text.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function aliasRegex(aliases) {
  if (aliases.length === 1) {
    return aliases[0];
  }
  return `(?:${aliases.map(escapeRegex).join("|")})`;
}

function setupBlock(lang) {
  const patterns = [
    { include: `#setup-import-${lang.id}-block` },
    { include: `#setup-declare-${lang.id}-block` },
    ...(lang.id === "rust" ? [{ include: "#setup-use-rust-block" }] : []),
    { include: `#setup-init-${lang.id}-block` },
    { include: `#setup-helpers-${lang.id}-block` },
    { include: "#comment" },
  ];
  return {
    name: `meta.block.setup.${lang.id}.bench`,
    begin: `^(\\s*)(setup)\\s+(${lang.setupRegex})\\s*\\{\\s*$`,
    end: "^\\1\\}\\s*$",
    beginCaptures: {
      2: { name: "keyword.control.bench" },
      3: { name: "entity.name.language.bench" },
    },
    endCaptures: {
      0: { name: "punctuation.definition.block.end.bench" },
    },
    patterns,
  };
}

function setupSectionBlock(lang, section) {
  const sectionBlock = {
    name: `meta.block.${section}.${lang.id}.bench`,
    begin: `^(\\s*)(${section})\\s*\\{`,
    end: "^\\1\\}",
    beginCaptures: {
      2: { name: "keyword.control.setup.bench" },
    },
    endCaptures: {
      0: { name: "punctuation.definition.block.end.bench" },
    },
    patterns: [{ include: lang.sourceScope }],
  };
  if (lang.includeContentNameInSetupSections) {
    sectionBlock.contentName = lang.embeddedScope;
  }
  return sectionBlock;
}

function setupImportBlock(lang) {
  if (lang.importParenStyle) {
    return {
      name: `meta.block.import.${lang.id}.bench`,
      begin: "^(\\s*)(import)\\s*(\\()",
      end: "^\\1\\)",
      beginCaptures: {
        2: { name: "keyword.control.setup.bench" },
        3: { name: "punctuation.definition.block.begin.bench" },
      },
      endCaptures: {
        0: { name: "punctuation.definition.block.end.bench" },
      },
      patterns: [{ include: lang.sourceScope }],
    };
  }
  return setupSectionBlock(lang, "import");
}

function codeBlock(lang, prefix) {
  return {
    name: `meta.block.${prefix}.${lang.id}.bench`,
    begin: `\\b(${lang.blockRegex})\\s*:\\s*\\{\\s*$`,
    end: "^\\s{0,12}\\}\\s*$",
    beginCaptures: {
      1: { name: "entity.name.language.bench" },
    },
    endCaptures: {
      0: { name: "punctuation.definition.block.end.bench" },
    },
    contentName: lang.embeddedScope,
    patterns: [{ include: lang.sourceScope }],
  };
}

function codeLine(lang, prefix) {
  return {
    name: `meta.expression.${lang.id}.bench`,
    begin: `\\b(${lang.lineRegex})\\s*:\\s*`,
    end: "$",
    beginCaptures: {
      1: { name: "entity.name.language.bench" },
    },
    contentName: lang.embeddedScope,
    patterns: [{ include: lang.sourceScope }],
  };
}

function replaceIncludesStable(patterns, matcher, replacementIncludes) {
  const out = [];
  let inserted = false;
  let sawAny = false;
  for (const p of patterns) {
    if (typeof p.include === "string" && matcher.test(p.include)) {
      sawAny = true;
      if (!inserted) {
        out.push(...replacementIncludes.map((inc) => ({ include: inc })));
        inserted = true;
      }
      continue;
    }
    out.push(p);
  }
  if (!sawAny) {
    out.push(...replacementIncludes.map((inc) => ({ include: inc })));
  }
  return out;
}

function main() {
  const raw = fs.readFileSync(GRAMMAR_PATH, "utf8");
  const grammar = JSON.parse(raw);
  const repo = grammar.repository || {};

  for (const lang of LANGS) {
    repo[`setup-${lang.id}-block`] = setupBlock(lang);
    repo[`setup-import-${lang.id}-block`] = setupImportBlock(lang);
    repo[`setup-declare-${lang.id}-block`] = setupSectionBlock(lang, "declare");
    repo[`setup-init-${lang.id}-block`] = setupSectionBlock(lang, "init");
    repo[`setup-helpers-${lang.id}-block`] = setupSectionBlock(lang, "helpers");
    repo[`fixture-${lang.id}-block`] = codeBlock(lang, "fixture");
    repo[`bench-${lang.id}-block`] = codeBlock(lang, "bench");
    repo[`bench-${lang.id}-line`] = codeLine(lang, "bench");
  }

  // Ensure suite/fixture/bench include lists track LANGS dynamically.
  repo["suite-block"].patterns = replaceIncludesStable(
    repo["suite-block"].patterns,
    /^#setup-[a-z0-9]+-block$/,
    LANGS.map((l) => `#setup-${l.id}-block`),
  );

  repo["fixture-block"].patterns = replaceIncludesStable(
    repo["fixture-block"].patterns,
    /^#fixture-[a-z0-9]+-block$/,
    LANGS.map((l) => `#fixture-${l.id}-block`),
  );

  repo["bench-block"].patterns = replaceIncludesStable(
    repo["bench-block"].patterns,
    /^#bench-[a-z0-9]+-(block|line)$/,
    [
      ...LANGS.map((l) => `#bench-${l.id}-block`),
      ...LANGS.map((l) => `#bench-${l.id}-line`),
    ],
  );

  if (repo["bench-hooks"] && Array.isArray(repo["bench-hooks"].patterns)) {
    const patterns = repo["bench-hooks"].patterns;
    const hookRegex = "(go|ts|rust|python|py|csharp|c)";
    if (patterns[0] && typeof patterns[0].match === "string") {
      patterns[0].match = `\\b(skip|validate)\\s+${hookRegex}\\s*:`;
    }
    if (patterns[1] && typeof patterns[1].match === "string") {
      patterns[1].match = `\\b(before|after|each)\\s+${hookRegex}\\s*:`;
    }
    if (patterns[2] && typeof patterns[2].match === "string") {
      patterns[2].match = "\\b(before|after|each)\\s*:";
    }
  }

  grammar.repository = repo;
  fs.writeFileSync(GRAMMAR_PATH, `${JSON.stringify(grammar, null, 2)}\n`, "utf8");
  console.log(`Generated TextMate grammar at ${GRAMMAR_PATH}`);
}

main();
