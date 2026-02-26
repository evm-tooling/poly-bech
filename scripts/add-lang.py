#!/usr/bin/env python3
"""
Add or regenerate language boilerplate for poly-bench.

Usage:
  ./scripts/add-lang [LANG]   # Regenerate for LANG, or all if omitted
  python scripts/add-lang.py [LANG]
  python scripts/add-lang.py --dry-run   # Show what would be regenerated

Reads scripts/languages.toml as source of truth. Idempotent - re-running
for an existing language regenerates without duplicates.

Regenerated files:
  - poly-bench-dsl: ast.rs, tokens.rs, parser.rs
  - poly-bench-syntax: partial_ast.rs
  - poly-bench-runtime-traits: config.rs
  - poly-bench-grammar: grammar.js, queries/injections.scm
  - poly-bench-project: lib.rs, manifest/build/deps/templates (C# templates)
  - poly-bench-executor: lib.rs, validation.rs, scheduler.rs
  - extensions/vscode: syntaxes/polybench.tmLanguage.json, package.json
  - poly-bench-stdlib: anvil.rs (from scripts/templates/anvil/*.template)

Validation workflow (adding a new language, e.g. after reverting C#):
  1. Run: ./scripts/add-lang csharp
  2. Create runtime crate: runtimes/runtimes-csharp/ (manual)
  3. Register plugin: Add &CSHARP_PLUGIN to poly-bench-runtime/src/registry.rs
  4. Add to workspace: Add "runtimes/runtimes-csharp" to root Cargo.toml members
  5. Add runtime dep: Add runtimes-csharp = { path = "..." } to poly-bench-runtime Cargo.toml
  6. Regenerate grammar: cd poly-bench-grammar && npx tree-sitter generate
  7. Build: cargo build
  8. Test: poly-bench run --lang csharp examples/demo-basic/benchmarks/csharp_ftest.bench
"""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
from pathlib import Path

try:
    import tomllib
except ImportError:
    import tomli as tomllib  # type: ignore

REPO_ROOT = Path(__file__).resolve().parent.parent
SCRIPTS_DIR = REPO_ROOT / "scripts"
LANGUAGES_TOML = SCRIPTS_DIR / "languages.toml"


def load_languages() -> dict:
    """Load languages from languages.toml."""
    with open(LANGUAGES_TOML, "rb") as f:
        data = tomllib.load(f)
    return data.get("languages", {})


def get_config_field(lang_id: str, lang_config: dict) -> str:
    """Get RuntimeConfig field name for a language."""
    return lang_config.get("config_field", f"{lang_id}_root")


def format_aliases_for_from_str(aliases: list[str]) -> str:
    """Format aliases for from_str match arm: "a" | "b" | "c"."""
    return " | ".join(f'"{a}"' for a in aliases)


def regenerate_ast_rs(languages: dict) -> None:
    """Regenerate Lang enum and impl in poly-bench-dsl/src/ast.rs."""
    path = REPO_ROOT / "poly-bench-dsl" / "src" / "ast.rs"
    content = path.read_text()

    # Generate Lang enum variants
    variants = [f"    {cfg['rust_enum']}," for _, cfg in languages.items()]
    enum_body = "\n".join(variants)

    # Generate from_str arms
    from_str_arms = []
    for _, cfg in languages.items():
        aliases = cfg["aliases"]
        if len(aliases) == 1:
            from_str_arms.append(f'            "{aliases[0]}" => Some(Lang::{cfg["rust_enum"]}),')
        else:
            from_str_arms.append(
                f'            {format_aliases_for_from_str(aliases)} => Some(Lang::{cfg["rust_enum"]}),'
            )
    from_str_body = "\n".join(from_str_arms)

    # Generate as_str arms
    as_str_arms = [
        f'            Lang::{cfg["rust_enum"]} => "{cfg["aliases"][0]}",'
        for _, cfg in languages.items()
    ]
    as_str_body = "\n".join(as_str_arms)

    generated = f"""/// Supported programming languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Lang {{
{enum_body}
}}

impl Lang {{
    pub fn from_str(s: &str) -> Option<Self> {{
        match s.to_lowercase().as_str() {{
{from_str_body}
            _ => None,
        }}
    }}

    pub fn as_str(&self) -> &'static str {{
        match self {{
{as_str_body}
        }}
    }}
}}"""

    begin = "// BEGIN-GENERATED: Lang enum (do not edit)\n"
    end = "\n// END-GENERATED: Lang enum"
    pattern = re.compile(
        re.escape(begin) + r".*?" + re.escape(end),
        re.DOTALL,
    )
    if not pattern.search(content):
        raise SystemExit("ast.rs: BEGIN-GENERATED/END-GENERATED markers not found")
    new_content = pattern.sub(begin + generated + end, content)
    path.write_text(new_content)


def regenerate_tokens_rs(languages: dict) -> None:
    """Regenerate TokenKind lang variants and keyword mapping in tokens.rs."""
    path = REPO_ROOT / "poly-bench-dsl" / "src" / "tokens.rs"
    content = path.read_text()

    # Collect all token kinds (may have multiple per lang, e.g. Ts, TypeScript)
    all_tokens = []
    for _, cfg in languages.items():
        all_tokens.extend(cfg["token_kinds"])

    # Generate enum variants (language keywords section)
    variants = [f"    {t}," for t in all_tokens]
    lang_variants = "\n".join(variants)

    # Generate is_lang matches
    is_lang_matches = " |\n                ".join(f"TokenKind::{t}" for t in all_tokens)

    # Generate keyword_from_str language entries
    keyword_entries = []
    for _, cfg in languages.items():
        aliases = cfg["aliases"]
        tokens = cfg["token_kinds"]
        # First alias maps to first token, etc. For single-token langs, all aliases map to that token
        primary_token = tokens[0]
        if len(aliases) == 1:
            keyword_entries.append(f'        "{aliases[0]}" => Some(TokenKind::{primary_token}),')
        else:
            keyword_entries.append(
                f'        {format_aliases_for_from_str(aliases)} => Some(TokenKind::{primary_token}),'
            )
    keyword_body = "\n".join(keyword_entries)

    # Replace language variants (between markers)
    begin_tk = "    // Language keywords (BEGIN-GENERATED: TokenKind lang variants)\n"
    end_tk = "\n    // END-GENERATED: TokenKind lang variants"
    tk_pattern = re.compile(
        re.escape(begin_tk) + r".*?" + re.escape(end_tk),
        re.DOTALL,
    )
    if tk_pattern.search(content):
        content = tk_pattern.sub(begin_tk + lang_variants + end_tk, content)
    else:
        raise SystemExit("tokens.rs: BEGIN-GENERATED/END-GENERATED markers for TokenKind lang variants not found")

    # Replace is_lang matches
    old_is_lang = re.compile(
        r"(TokenKind::Go \|\n\s+TokenKind::Ts \|\n\s+TokenKind::TypeScript \|\n\s+TokenKind::Rust \|\n\s+TokenKind::Python \|\n\s+TokenKind::CSharp)"
    )
    content = old_is_lang.sub(is_lang_matches, content)

    # Replace keyword_from_str language entries (between markers)
    begin_kw = "        // Language keywords (BEGIN-GENERATED: keyword_from_str lang)\n"
    end_kw = "\n        // END-GENERATED: keyword_from_str lang"
    kw_pattern = re.compile(
        re.escape(begin_kw) + r".*?" + re.escape(end_kw),
        re.DOTALL,
    )
    if kw_pattern.search(content):
        content = kw_pattern.sub(begin_kw + keyword_body + end_kw, content)
    else:
        raise SystemExit("tokens.rs: BEGIN-GENERATED/END-GENERATED markers for keyword_from_str not found")

    path.write_text(content)


def regenerate_partial_ast_rs(languages: dict) -> None:
    """Regenerate Lang enum in poly-bench-syntax/src/partial_ast.rs."""
    path = REPO_ROOT / "poly-bench-syntax" / "src" / "partial_ast.rs"
    content = path.read_text()

    variants = [f"    {cfg['rust_enum']}," for _, cfg in languages.items()]
    enum_body = "\n".join(variants)

    from_str_arms = []
    for _, cfg in languages.items():
        aliases = cfg["aliases"]
        if len(aliases) == 1:
            from_str_arms.append(f'            "{aliases[0]}" => Some(Lang::{cfg["rust_enum"]}),')
        else:
            from_str_arms.append(
                f'            {format_aliases_for_from_str(aliases)} => Some(Lang::{cfg["rust_enum"]}),'
            )
    from_str_body = "\n".join(from_str_arms)

    as_str_arms = [
        f'            Lang::{cfg["rust_enum"]} => "{cfg["aliases"][0]}",'
        for _, cfg in languages.items()
    ]
    as_str_body = "\n".join(as_str_arms)

    generated = f"""/// Supported languages for embedded code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lang {{
{enum_body}
}}

impl Lang {{
    pub fn from_str(s: &str) -> Option<Self> {{
        match s.to_lowercase().as_str() {{
{from_str_body}
            _ => None,
        }}
    }}

    pub fn as_str(&self) -> &'static str {{
        match self {{
{as_str_body}
        }}
    }}
}}"""

    old_pattern = r"(/// Supported languages for embedded code\n#\[derive\(Debug, Clone, Copy, PartialEq, Eq, Hash\)\]\npub enum Lang \{\n.*?\n\}\n\nimpl Lang \{\n    pub fn from_str\(s: &str\) -> Option<Self> \{\n        match s\.to_lowercase\(\)\.as_str\(\) \{\n.*?        \}\n    \}\n\n    pub fn as_str\(&self\) -> &'static str \{\n        match self \{\n.*?        \}\n    \}\n\})"
    content = re.sub(old_pattern, generated, content, flags=re.DOTALL)
    path.write_text(content)


def regenerate_config_rs(languages: dict) -> None:
    """Regenerate RuntimeConfig fields in poly-bench-runtime-traits/src/config.rs."""
    path = REPO_ROOT / "poly-bench-runtime-traits" / "src" / "config.rs"
    content = path.read_text()

    fields = []
    for lang_id, cfg in languages.items():
        field_name = get_config_field(lang_id, cfg)
        comment = {
            "go": "Go module root (directory containing go.mod)",
            "typescript": "Node.js project root (directory containing package.json or node_modules)",
            "rust": "Rust project root (directory containing Cargo.toml)",
            "python": "Python project root (directory containing requirements.txt or pyproject.toml)",
            "csharp": "C# project root (directory containing .csproj/.sln)",
        }.get(lang_id, f"{lang_id} project root")
        fields.append(f"    /// {comment}\n    pub {field_name}: Option<PathBuf>,")

    fields_body = "\n".join(fields)

    old_struct = re.compile(
        r"pub struct RuntimeConfig \{\n.*?\n\}",
        re.DOTALL,
    )
    new_struct = f"""pub struct RuntimeConfig {{
{fields_body}
}}"""
    content = old_struct.sub(new_struct, content)
    path.write_text(content)


def regenerate_grammar_js(languages: dict) -> None:
    """Regenerate language_tag choice in poly-bench-grammar/grammar.js."""
    path = REPO_ROOT / "poly-bench-grammar" / "grammar.js"
    content = path.read_text()

    # Flatten all aliases for language_tag
    choices = []
    for _, cfg in languages.items():
        choices.extend(f"'{a}'" for a in cfg["aliases"])
    choices_str = ",\n      ".join(choices)

    old_choice = re.compile(
        r"language_tag: \$ => choice\(\n.*?\n    \),",
        re.DOTALL,
    )
    new_choice = f"""language_tag: $ => choice(
      {choices_str},
    ),"""
    content = old_choice.sub(new_choice, content)
    path.write_text(content)


def generate_injection_predicate(aliases: list[str]) -> str:
    """Generate #eq? or #any-of? predicate for injections."""
    if len(aliases) == 1:
        return f'(#eq? @_lang "{aliases[0]}")'
    quoted = " ".join(f'"{a}"' for a in aliases)
    return f'(#any-of? @_lang {quoted})'


def regenerate_injections_scm(languages: dict) -> None:
    """Regenerate poly-bench-grammar/queries/injections.scm."""
    path = REPO_ROOT / "poly-bench-grammar" / "queries" / "injections.scm"
    lines = [
        "; Injection queries for embedded languages in poly-bench",
        "; These queries identify code blocks and inject the appropriate language parser",
        "",
    ]

    for lang_id, cfg in languages.items():
        aliases = cfg["aliases"]
        injection_lang = cfg["tree_sitter_injection"]
        pred = generate_injection_predicate(aliases)
        # Add #set! when injection lang differs from primary alias (Python, C#)
        need_set = injection_lang != aliases[0]
        set_injection = f'  (#set! injection.language "{injection_lang}")' if need_set else ""

        # Go uses paren_code_block for import; others use code_block
        import_block = "paren_code_block" if cfg.get("import_style") == "paren" else "code_block"

        lines.append(f"; ============================================================")
        lines.append(f"; {cfg['rust_enum']} injections")
        lines.append(f"; ============================================================")
        lines.append("")
        lines.append(f"; Setup block with {cfg['rust_enum']} language")
        lines.append(f"(setup_block")
        lines.append(f"  language: (language_tag) @_lang")
        lines.append(f"  (setup_body")
        lines.append(f"    (import_section")
        lines.append(f"      ({import_block}")
        lines.append(f"        (embedded_code) @injection.content)))")
        lines.append(f"  {pred}")
        if set_injection:
            lines.append(f"  {set_injection}")
        lines.append(")")
        lines.append("")
        lines.append(f"(setup_block")
        lines.append(f"  language: (language_tag) @_lang")
        lines.append(f"  (setup_body")
        lines.append(f"    (declare_section")
        lines.append(f"      (code_block")
        lines.append(f"        (embedded_code) @injection.content)))")
        lines.append(f"  {pred}")
        if set_injection:
            lines.append(f"  {set_injection}")
        lines.append("")
        lines.append(f"(setup_block")
        lines.append(f"  language: (language_tag) @_lang")
        lines.append(f"  (setup_body")
        lines.append(f"    (init_section")
        lines.append(f"      (code_block")
        lines.append(f"        (embedded_code) @injection.content)))")
        lines.append(f"  {pred}")
        if set_injection:
            lines.append(f"  {set_injection}")
        lines.append("")
        lines.append(f"(setup_block")
        lines.append(f"  language: (language_tag) @_lang")
        lines.append(f"  (setup_body")
        lines.append(f"    (helpers_section")
        lines.append(f"      (code_block")
        lines.append(f"        (embedded_code) @injection.content)))")
        lines.append(f"  {pred}")
        if set_injection:
            lines.append(f"  {set_injection}")
        lines.append(")")
        lines.append("")
        lines.append(f"; Language implementation with {cfg['rust_enum']}")
        lines.append(f"(language_implementation")
        lines.append(f"  language: (language_tag) @_lang")
        lines.append(f"  (code_block")
        lines.append(f"    (embedded_code) @injection.content)")
        lines.append(f"  {pred}")
        if set_injection:
            lines.append(f"  {set_injection}")
        lines.append(")")
        lines.append("")
        lines.append(f"; Hook with {cfg['rust_enum']}")
        lines.append(f"(hook_flat")
        lines.append(f"  language: (language_tag) @_lang")
        lines.append(f"  (code_block")
        lines.append(f"    (embedded_code) @injection.content)")
        lines.append(f"  {pred}")
        if set_injection:
            lines.append(f"  {set_injection}")
        lines.append(")")

    # Inline code injections
    lines.append("; ============================================================")
    lines.append("; Inline code injections (single-line expressions)")
    lines.append("; ============================================================")
    lines.append("")

    for lang_id, cfg in languages.items():
        aliases = cfg["aliases"]
        injection_lang = cfg["tree_sitter_injection"]
        pred = generate_injection_predicate(aliases)
        lines.append(f"; {cfg['rust_enum']} inline code")
        lines.append(f"(language_implementation")
        lines.append(f"  language: (language_tag) @_lang")
        lines.append(f"  (inline_code) @injection.content)")
        lines.append(f"  {pred}")
        lines.append(f'  (#set! injection.language "{injection_lang}")')
        lines.append(")")

    path.write_text("\n".join(lines))


def get_embedded_lang(lang_id: str, cfg: dict) -> str:
    """Get VS Code embedded language ID for a language (used for contentName)."""
    return cfg.get("embedded_lang", cfg["vscode_scope"])


def get_grammar_scope(cfg: dict) -> str:
    """Get VS Code grammar scope for include (source.X). Use vscode_scope since that
    matches the actual grammar scope registered by extensions (e.g. C# uses source.cs)."""
    return cfg["vscode_scope"]


def _lang_aliases_regex(aliases: list[str]) -> str:
    """Build regex group for language aliases: (go) or (ts|typescript)."""
    if len(aliases) == 1:
        return f"({re.escape(aliases[0])})"
    return "(" + "|".join(re.escape(a) for a in aliases) + ")"


def _generate_tmlanguage_setup_blocks(lang_id: str, cfg: dict) -> dict:
    """Generate setup-X-block and sub-blocks for a language."""
    aliases = cfg["aliases"]
    lang_regex = _lang_aliases_regex(aliases)
    block_name = cfg["aliases"][0]
    embedded_lang = get_embedded_lang(lang_id, cfg)
    grammar_scope = get_grammar_scope(cfg)
    content_name = f"source.{embedded_lang}.embedded.bench"

    import_style = cfg.get("import_style", "brace")
    has_use_block = lang_id == "rust"

    if import_style == "paren":
        import_block = {
            f"setup-import-{block_name}-block": {
                "name": f"meta.block.import.{block_name}.bench",
                "begin": r'^(\s*)(import)\s*(\()',
                "end": r"^\1\)",
                "beginCaptures": {
                    "2": {"name": "keyword.control.setup.bench"},
                    "3": {"name": "punctuation.definition.block.begin.bench"},
                },
                "endCaptures": {"0": {"name": "punctuation.definition.block.end.bench"}},
                "patterns": [{"include": f"source.{grammar_scope}"}],
            }
        }
    else:
        import_block = {
            f"setup-import-{block_name}-block": {
                "name": f"meta.block.import.{block_name}.bench",
                "contentName": content_name,
                "begin": r'^(\s*)(import)\s*\{',
                "end": r"^\1\}",
                "beginCaptures": {"2": {"name": "keyword.control.setup.bench"}},
                "endCaptures": {"0": {"name": "punctuation.definition.block.end.bench"}},
                "patterns": [{"include": f"source.{grammar_scope}"}],
            }
        }

    sub_blocks = [
        f"#setup-import-{block_name}-block",
        f"#setup-declare-{block_name}-block",
    ]
    if has_use_block:
        sub_blocks.append(f"#setup-use-{block_name}-block")
    sub_blocks.extend([
        f"#setup-init-{block_name}-block",
        f"#setup-helpers-{block_name}-block",
        "#comment",
    ])

    setup_block = {
        f"setup-{block_name}-block": {
            "name": f"meta.block.setup.{block_name}.bench",
            "begin": rf"^(\s*)(setup)\s+{lang_regex}\s*{{\s*$",
            "end": r"^\1\}\s*$",
            "beginCaptures": {
                "2": {"name": "keyword.control.bench"},
                "3": {"name": "entity.name.language.bench"},
            },
            "endCaptures": {"0": {"name": "punctuation.definition.block.end.bench"}},
            "patterns": [{"include": inc} for inc in sub_blocks],
        }
    }

    declare_block = {
        f"setup-declare-{block_name}-block": {
            "name": f"meta.block.declare.{block_name}.bench",
            "contentName": content_name,
            "begin": r'^(\s*)(declare)\s*\{',
            "end": r"^\1\}",
            "beginCaptures": {"2": {"name": "keyword.control.setup.bench"}},
            "endCaptures": {"0": {"name": "punctuation.definition.block.end.bench"}},
            "patterns": [{"include": f"source.{grammar_scope}"}],
        }
    }

    init_block = {
        f"setup-init-{block_name}-block": {
            "name": f"meta.block.init.{block_name}.bench",
            "contentName": content_name,
            "begin": r'^(\s*)(init)\s*\{',
            "end": r"^\1\}",
            "beginCaptures": {"2": {"name": "keyword.control.setup.bench"}},
            "endCaptures": {"0": {"name": "punctuation.definition.block.end.bench"}},
            "patterns": [{"include": f"source.{grammar_scope}"}],
        }
    }

    helpers_block = {
        f"setup-helpers-{block_name}-block": {
            "name": f"meta.block.helpers.{block_name}.bench",
            "contentName": content_name,
            "begin": r'^(\s*)(helpers)\s*\{',
            "end": r"^\1\}",
            "beginCaptures": {"2": {"name": "keyword.control.setup.bench"}},
            "endCaptures": {"0": {"name": "punctuation.definition.block.end.bench"}},
            "patterns": [{"include": f"source.{grammar_scope}"}],
        }
    }

    result = {**import_block, **declare_block, **init_block, **helpers_block}
    if has_use_block:
        use_block = {
            f"setup-use-{block_name}-block": {
                "name": f"meta.block.use.{block_name}.bench",
                "begin": r'^(\s*)(use)\s+',
                "end": ";",
                "beginCaptures": {"2": {"name": "keyword.control.setup.bench"}},
                "patterns": [{"include": f"source.{grammar_scope}"}],
            }
        }
        result.update(use_block)
    result.update(setup_block)
    return result


def _generate_tmlanguage_fixture_bench_blocks(lang_id: str, cfg: dict) -> dict:
    """Generate fixture-X-block, bench-X-block, bench-X-line for a language."""
    aliases = cfg["aliases"]
    lang_regex = _lang_aliases_regex(aliases)
    block_name = cfg["aliases"][0]
    embedded_lang = get_embedded_lang(lang_id, cfg)
    grammar_scope = get_grammar_scope(cfg)
    content_name = f"source.{embedded_lang}.embedded.bench"

    return {
        f"fixture-{block_name}-block": {
            "name": f"meta.block.fixture.{block_name}.bench",
            "begin": rf"\b{lang_regex}\s*:\s*{{\s*$",
            "end": r"^\s{0,12}\}\s*$",
            "beginCaptures": {"1": {"name": "entity.name.language.bench"}},
            "endCaptures": {"0": {"name": "punctuation.definition.block.end.bench"}},
            "contentName": content_name,
            "patterns": [{"include": f"source.{grammar_scope}"}],
        },
        f"bench-{block_name}-block": {
            "name": f"meta.block.bench.{block_name}.bench",
            "begin": rf"\b{lang_regex}\s*:\s*{{\s*$",
            "end": r"^\s{0,12}\}\s*$",
            "beginCaptures": {"1": {"name": "entity.name.language.bench"}},
            "endCaptures": {"0": {"name": "punctuation.definition.block.end.bench"}},
            "contentName": content_name,
            "patterns": [{"include": f"source.{grammar_scope}"}],
        },
        f"bench-{block_name}-line": {
            "name": f"meta.expression.{block_name}.bench",
            "begin": rf"\b{lang_regex}\s*:\s*",
            "end": "$",
            "beginCaptures": {"1": {"name": "entity.name.language.bench"}},
            "contentName": content_name,
            "patterns": [{"include": f"source.{grammar_scope}"}],
        },
    }


def regenerate_tmlanguage(languages: dict) -> None:
    """Regenerate language-specific patterns in polybench.tmLanguage.json.
    Updates include lists and generates setup/fixture/bench block definitions."""
    path = REPO_ROOT / "extensions" / "vscode" / "syntaxes" / "polybench.tmLanguage.json"
    with open(path) as f:
        data = json.load(f)

    repo = data["repository"]

    # Build include lists from config
    setup_includes = [{"include": f"#setup-{cfg['aliases'][0]}-block"} for cfg in languages.values()]
    fixture_includes = [{"include": f"#fixture-{cfg['aliases'][0]}-block"} for cfg in languages.values()]
    bench_block_includes = [{"include": f"#bench-{cfg['aliases'][0]}-block"} for cfg in languages.values()]
    bench_line_includes = [{"include": f"#bench-{cfg['aliases'][0]}-line"} for cfg in languages.values()]

    # Build language alias regex for bench-hooks (skip|validate|before|after|each)
    all_aliases = []
    for cfg in languages.values():
        all_aliases.extend(cfg["aliases"])
    hooks_lang_regex = "|".join(re.escape(a) for a in all_aliases)

    # Add/update block definitions for each language
    for lang_id, cfg in languages.items():
        setup_blocks = _generate_tmlanguage_setup_blocks(lang_id, cfg)
        for k, v in setup_blocks.items():
            repo[k] = v
        fixture_bench = _generate_tmlanguage_fixture_bench_blocks(lang_id, cfg)
        for k, v in fixture_bench.items():
            repo[k] = v

    # Update suite-block: replace setup-X-block includes with generated list
    suite_block = repo.get("suite-block", {})
    if "patterns" in suite_block:
        patterns = suite_block["patterns"]
        new_patterns = []
        skip_setup_lang_blocks = False
        for p in patterns:
            inc = p.get("include", "") if isinstance(p, dict) else ""
            if inc == "#suite-global-setup-block":
                new_patterns.append(p)
                new_patterns.extend(setup_includes)
                skip_setup_lang_blocks = True
                continue
            if skip_setup_lang_blocks and inc.startswith("#setup-") and inc.endswith("-block"):
                continue
            if inc.startswith("#fixture-block") or inc.startswith("#bench-block"):
                skip_setup_lang_blocks = False
            new_patterns.append(p)
        suite_block["patterns"] = new_patterns

    # Update fixture-block: replace fixture-X-block includes with generated list
    fixture_block = repo.get("fixture-block", {})
    if "patterns" in fixture_block:
        patterns = fixture_block["patterns"]
        final = []
        for p in patterns:
            inc = p.get("include", "") if isinstance(p, dict) else ""
            if inc == "#fixture-attributes":
                final.append(p)
                final.extend(fixture_includes)
                continue
            if inc.startswith("#fixture-") and inc.endswith("-block") and inc != "#fixture-block":
                continue
            final.append(p)
        fixture_block["patterns"] = final

    # Update bench-block: replace bench-X-block and bench-X-line includes with generated list
    bench_block = repo.get("bench-block", {})
    if "patterns" in bench_block:
        patterns = bench_block["patterns"]
        final = []
        for p in patterns:
            inc = p.get("include", "") if isinstance(p, dict) else ""
            if inc == "#bench-hooks":
                final.append(p)
                final.extend(bench_block_includes)
                final.extend(bench_line_includes)
                continue
            if (inc.startswith("#bench-") and ("-block" in inc or "-line" in inc) and
                    inc not in ("#bench-block", "#bench-attributes", "#bench-hooks")):
                continue
            final.append(p)
        bench_block["patterns"] = final

    # Update bench-hooks: language patterns for skip|validate and before|after|each
    hooks = repo.get("bench-hooks", {})
    if "patterns" in hooks:
        for p in hooks["patterns"]:
            if isinstance(p, dict) and "match" in p:
                m = p["match"]
                if "skip" in m and "validate" in m and "go" in m:
                    p["match"] = f"\\b(skip|validate)\\s+({hooks_lang_regex})\\s*:"
                elif "before" in m and "after" in m and "each" in m and "go" in m:
                    p["match"] = f"\\b(before|after|each)\\s+({hooks_lang_regex})\\s*:"

    path.write_text(json.dumps(data, indent=2) + "\n")


def regenerate_stdlib_anvil(languages: dict) -> None:
    """Regenerate poly-bench-stdlib anvil.rs from templates.
    Each language gets its anvil code from scripts/templates/anvil/{name}.template.
    Add anvil_template = "ts" to languages.toml if the template name differs from aliases[0].
    Add anvil_imports = ['"os"'] for languages that need extra imports (e.g. Go)."""
    templates_dir = SCRIPTS_DIR / "templates" / "anvil"
    path = REPO_ROOT / "poly-bench-stdlib" / "src" / "anvil.rs"

    consts = []
    get_imports_arms = []
    get_code_arms = []
    test_cases = []

    for lang_id, cfg in languages.items():
        template_name = cfg.get("anvil_template", cfg["aliases"][0])
        template_path = templates_dir / f"{template_name}.template"
        if not template_path.exists():
            raise FileNotFoundError(
                f"Anvil template not found for {lang_id}: {template_path}. "
                f"Create it or set anvil_template in languages.toml."
            )
        code = template_path.read_text().strip()

        rust_enum = cfg["rust_enum"]
        const_name = f"{rust_enum.upper()}_ANVIL"
        consts.append(f'const {const_name}: &str = r#"\n{code}\n"#;')

        imports = cfg.get("anvil_imports", [])
        if imports:
            imports_rust = ", ".join(
                f'"{s.replace(chr(34), chr(92) + chr(34))}"' for s in imports
            )
            get_imports_arms.append(f"        Lang::{rust_enum} => vec![{imports_rust}],")
        else:
            get_imports_arms.append(f"        Lang::{rust_enum} => vec![],")

        get_code_arms.append(f"        Lang::{rust_enum} => {const_name}.to_string(),")

        test_name = f"test_{template_name}_anvil_contains_rpc_url"
        test_check = "os.Getenv" if lang_id == "go" else "process.env" if lang_id == "typescript" else "std::env::var" if lang_id == "rust" else "os.environ" if lang_id == "python" else "Environment.GetEnvironmentVariable"
        test_cases.append(
            f'''    #[test]
    fn {test_name}() {{
        let code = get_code(Lang::{rust_enum});
        assert!(code.contains("ANVIL_RPC_URL"));
        assert!(code.contains("{test_check}"));
    }}'''
        )

    content = f'''//! Anvil module - Anvil RPC URL accessor for EVM benchmarks
//!
//! When `use std::anvil` is specified along with `globalSetup {{ spawnAnvil() }}`,
//! poly-bench spawns a local Anvil Ethereum node and makes the RPC URL available
//! via the `ANVIL_RPC_URL` variable.
//!
//! ## Available Variables
//!
//! - `ANVIL_RPC_URL` - The RPC endpoint URL (e.g., "http://127.0.0.1:8545")
//!
//! ## Usage
//!
//! ```bench
//! use std::anvil
//!
//! globalSetup {{
//!     spawnAnvil()                           // Basic spawn
//!     // spawnAnvil(fork: "https://...")     // With chain forking
//! }}
//!
//! suite evmBench {{
//!     setup go {{
//!         import ("net/http")
//!         
//!         helpers {{
//!             func callRpc() {{
//!                 http.Post(ANVIL_RPC_URL, "application/json", ...)
//!             }}
//!         }}
//!     }}
//!     
//!     bench rpcTest {{
//!         go: callRpc()
//!     }}
//! }}
//! ```

use poly_bench_dsl::Lang;

/// Get the language-specific imports for the anvil module
pub fn get_imports(lang: Lang) -> Vec<&'static str> {{
    match lang {{
{chr(10).join(get_imports_arms)}
    }}
}}

/// Get the language-specific code for the anvil module
pub fn get_code(lang: Lang) -> String {{
    match lang {{
{chr(10).join(get_code_arms)}
    }}
}}

{chr(10).join(consts)}

#[cfg(test)]
mod tests {{
    use super::*;

{chr(10).join(test_cases)}
}}
'''
    path.write_text(content)


def regenerate_package_json(languages: dict) -> None:
    """Regenerate embeddedLanguages in package.json."""
    path = REPO_ROOT / "extensions" / "vscode" / "package.json"
    with open(path) as f:
        data = json.load(f)

    grammars = data.get("contributes", {}).get("grammars", [])
    for g in grammars:
        if g.get("scopeName") == "source.bench" and "embeddedLanguages" in g:
            embedded = {}
            for lang_id, cfg in languages.items():
                lang = get_embedded_lang(lang_id, cfg)
                # Add both vscode_scope and first alias scopes (e.g. cs and csharp for C#)
                for scope_name in [cfg["vscode_scope"], cfg["aliases"][0]]:
                    scope = f"source.{scope_name}.embedded.bench"
                    embedded[scope] = lang
            g["embeddedLanguages"] = embedded
            break

    path.write_text(json.dumps(data, indent=2) + "\n")


def regenerate_project_lib(languages: dict) -> None:
    """Regenerate RUNTIME_ENV constants and runtime_env functions in poly-bench-project."""
    path = REPO_ROOT / "poly-bench-project" / "src" / "lib.rs"
    content = path.read_text()

    comments = {
        "go": "Go runtime env subdir (go.mod, go.sum, generated bench code)",
        "typescript": "TypeScript/Node runtime env subdir (package.json, node_modules, generated bench code)",
        "rust": "Rust runtime env subdir (Cargo.toml, Cargo.lock, generated bench code)",
        "python": "Python runtime env subdir",
        "csharp": "C# runtime env subdir (polybench.csproj, Program.cs, generated bench code)",
    }

    const_lines = []
    fn_lines = []
    for lang_id, cfg in languages.items():
        const_name = f"RUNTIME_ENV_{cfg['rust_enum'].upper()}"
        dir_name = cfg["aliases"][0]
        fn_name = f"runtime_env_{dir_name}"  # e.g. runtime_env_ts for typescript
        comment = comments.get(lang_id, f"{cfg['rust_enum']} runtime env subdir")
        const_lines.append(f"/// {comment}\npub const {const_name}: &str = \"{dir_name}\";")
        fn_lines.append(
            f'''/// Path to the {cfg['rust_enum']} runtime env for a project
pub fn {fn_name}(project_root: &Path) -> PathBuf {{
    project_root.join(RUNTIME_ENV_DIR).join({const_name})
}}'''
        )

    const_body = "\n".join(const_lines)
    fn_body = "\n\n".join(fn_lines)

    for begin, end, body in [
        ("// BEGIN-GENERATED: RUNTIME_ENV constants (do not edit)\n", "\n// END-GENERATED: RUNTIME_ENV constants", const_body),
        ("// BEGIN-GENERATED: runtime_env functions (do not edit)\n", "\n// END-GENERATED: runtime_env functions", fn_body),
    ]:
        pattern = re.compile(re.escape(begin) + r".*?" + re.escape(end), re.DOTALL)
        if pattern.search(content):
            content = pattern.sub(begin + body + end, content)

    path.write_text(content)


def regenerate_executor_lib(languages: dict) -> None:
    """Regenerate ProjectRoots fields in poly-bench-executor."""
    path = REPO_ROOT / "poly-bench-executor" / "src" / "lib.rs"
    content = path.read_text()

    comments = {
        "go": "Go module root (directory containing go.mod)",
        "typescript": "Node.js project root (directory containing package.json or node_modules)",
        "rust": "Rust project root (directory containing Cargo.toml)",
        "python": "Python project root (directory containing requirements.txt or pyproject.toml)",
        "csharp": "C# project root (directory containing .csproj/.sln)",
    }

    fields = []
    for lang_id, cfg in languages.items():
        field_name = get_config_field(lang_id, cfg)
        comment = comments.get(lang_id, f"{cfg['rust_enum']} project root")
        fields.append(f"    /// {comment}\n    pub {field_name}: Option<PathBuf>,")

    fields_body = "\n".join(fields)

    begin = "    // BEGIN-GENERATED: ProjectRoots fields (do not edit)\n"
    end = "\n    // END-GENERATED: ProjectRoots fields"
    pattern = re.compile(re.escape(begin) + r".*?" + re.escape(end), re.DOTALL)
    if not pattern.search(content):
        raise SystemExit("executor lib.rs: BEGIN-GENERATED/END-GENERATED markers for ProjectRoots not found")
    content = pattern.sub(begin + fields_body + end, content)
    path.write_text(content)


def regenerate_executor_validation_scheduler(languages: dict) -> None:
    """Regenerate RuntimeConfig blocks in validation.rs and scheduler.rs."""
    config_lines_8 = []
    config_lines_12 = []
    for lang_id, cfg in languages.items():
        field_name = get_config_field(lang_id, cfg)
        config_lines_8.append(f"        {field_name}: project_roots.{field_name}.clone(),")
        config_lines_12.append(f"            {field_name}: project_roots.{field_name}.clone(),")

    for filename, indent, config_body in [
        ("validation.rs", "        ", "\n".join(config_lines_8)),
        ("scheduler.rs", "            ", "\n".join(config_lines_12)),
    ]:
        path = REPO_ROOT / "poly-bench-executor" / "src" / filename
        content = path.read_text()
        begin = f"{indent}// BEGIN-GENERATED: RuntimeConfig mapping (do not edit)\n"
        end = f"\n{indent}// END-GENERATED: RuntimeConfig mapping"
        pattern = re.compile(re.escape(begin) + r".*?" + re.escape(end), re.DOTALL)
        if pattern.search(content):
            content = pattern.sub(begin + config_body + end, content)
            path.write_text(content)


def regenerate_parser_rs(languages: dict) -> None:
    """Regenerate lang_tokens macro and token_to_lang match arms in parser.rs."""
    path = REPO_ROOT / "poly-bench-dsl" / "src" / "parser.rs"
    content = path.read_text()

    # Generate macro body: TokenKind::Go | TokenKind::Ts | TokenKind::TypeScript | ...
    all_tokens = []
    for _, cfg in languages.items():
        all_tokens.extend(cfg["token_kinds"])
    macro_body = " |\n        ".join(f"TokenKind::{t}" for t in all_tokens)

    macro_generated = f"""macro_rules! lang_tokens {{
    () => {{
        {macro_body}
    }};
}}"""

    # Replace macro section
    begin_macro = "// BEGIN-GENERATED: lang_tokens macro (do not edit)\n"
    end_macro = "\n// END-GENERATED: lang_tokens macro"
    macro_pattern = re.compile(
        re.escape(begin_macro) + r".*?" + re.escape(end_macro),
        re.DOTALL,
    )
    if not macro_pattern.search(content):
        raise SystemExit("parser.rs: BEGIN-GENERATED/END-GENERATED markers for lang_tokens macro not found")
    content = macro_pattern.sub(begin_macro + macro_generated + end_macro, content)

    # Generate token_to_lang arms: TokenKind::X => Some(Lang::Y)
    # Multiple tokens can map to same Lang (e.g. Ts, TypeScript => TypeScript)
    token_arms = []
    for _, cfg in languages.items():
        tokens = cfg["token_kinds"]
        rust_enum = cfg["rust_enum"]
        if len(tokens) == 1:
            token_arms.append(f"            TokenKind::{tokens[0]} => Some(Lang::{rust_enum}),")
        else:
            token_arms.append(
                "            "
                + " | ".join(f"TokenKind::{t}" for t in tokens)
                + f" => Some(Lang::{rust_enum}),"
            )
    token_arms_body = "\n".join(token_arms)

    # Replace token_to_lang section
    begin_t2l = "            // BEGIN-GENERATED: token_to_lang (do not edit)\n"
    end_t2l = "\n            // END-GENERATED: token_to_lang"
    t2l_pattern = re.compile(
        re.escape(begin_t2l) + r".*?" + re.escape(end_t2l),
        re.DOTALL,
    )
    if not t2l_pattern.search(content):
        raise SystemExit("parser.rs: BEGIN-GENERATED/END-GENERATED markers for token_to_lang not found")
    content = t2l_pattern.sub(
        begin_t2l + token_arms_body + end_t2l,
        content,
    )

    path.write_text(content)


TEMPLATES_DIR = SCRIPTS_DIR / "templates"


def _read_template(lang_id: str, name: str) -> str:
    """Read a template file for a language."""
    path = TEMPLATES_DIR / lang_id / f"{name}.template"
    if not path.exists():
        return ""
    return path.read_text()


def _insert_after_if_missing(content: str, pattern: str, to_insert: str, sentinel: str) -> str:
    """Insert to_insert after pattern if sentinel is not already in content."""
    if sentinel in content:
        return content
    idx = content.rfind(pattern)
    if idx == -1:
        return content
    insert_pos = idx + len(pattern)
    return content[:insert_pos] + "\n" + to_insert + content[insert_pos:]


def apply_csharp_templates() -> None:
    """Apply C# templates to project files if not already present."""
    if "csharp" not in load_languages():
        return

    # manifest.rs
    manifest_path = REPO_ROOT / "poly-bench-project" / "src" / "manifest.rs"
    manifest_content = manifest_path.read_text()

    if "CSharpConfig" not in manifest_content:
        # Insert CSharpConfig struct after PythonConfig
        config_tpl = _read_template("csharp", "manifest_config")
        if config_tpl:
            manifest_content = _insert_after_if_missing(
                manifest_content,
                "}\n\nfn default_python_version",
                config_tpl.rstrip() + "\n\n",
                "CSharpConfig",
            )

        # Insert struct field after python field
        field_tpl = _read_template("csharp", "manifest_struct_field")
        if field_tpl:
            manifest_content = _insert_after_if_missing(
                manifest_content,
                "pub python: Option<PythonConfig>,\n\n",
                field_tpl.rstrip(),
                "pub csharp: Option<CSharpConfig>",
            )

        # Insert has_csharp in Manifest::new
        has_check_tpl = _read_template("csharp", "manifest_new_has_check")
        if has_check_tpl:
            manifest_content = _insert_after_if_missing(
                manifest_content,
                "let has_python = languages.iter().any(|l| l == \"python\" || l == \"py\");\n\n",
                has_check_tpl.rstrip(),
                "let has_csharp = ",
            )

        # Insert csharp block in Manifest::new (after python block, before output)
        new_block_tpl = _read_template("csharp", "manifest_new_block")
        if new_block_tpl:
            manifest_content = _insert_after_if_missing(
                manifest_content,
                "            },\n            output: OutputConfig::default()",
                "\n            " + new_block_tpl.rstrip(),
                "csharp: if has_csharp",
            )

        # Insert has_csharp, add/remove methods
        has_method_tpl = _read_template("csharp", "manifest_has_method")
        if has_method_tpl:
            manifest_content = _insert_after_if_missing(
                manifest_content,
                "pub fn has_python(&self) -> bool {\n        self.python.is_some()\n    }\n\n",
                has_method_tpl.rstrip() + "\n\n",
                "pub fn has_csharp",
            )

        add_dep_tpl = _read_template("csharp", "manifest_add_dep")
        if add_dep_tpl:
            manifest_content = _insert_after_if_missing(
                manifest_content,
                "pub fn remove_python_dependency(&mut self, package: &str) -> Result<()> {\n"
                "        let python = self\n"
                "            .python\n"
                "            .as_mut()\n"
                "            .ok_or_else(|| miette::miette!(\"Python is not enabled in this project\"))?;\n"
                "        python.dependencies.remove(package);\n"
                "        Ok(())\n"
                "    }\n\n",
                add_dep_tpl.rstrip() + "\n\n",
                "pub fn add_csharp_dependency",
            )

        remove_dep_tpl = _read_template("csharp", "manifest_remove_dep")
        if remove_dep_tpl:
            manifest_content = _insert_after_if_missing(
                manifest_content,
                "pub fn add_csharp_dependency(&mut self, package: &str, version: &str) -> Result<()> {\n"
                "        let csharp = self\n"
                "            .csharp\n"
                "            .as_mut()\n"
                "            .ok_or_else(|| miette::miette!(\"C# is not enabled in this project\"))?;\n"
                "        csharp.dependencies.insert(package.to_string(), version.to_string());\n"
                "        Ok(())\n"
                "    }\n\n",
                remove_dep_tpl.rstrip() + "\n\n",
                "pub fn remove_csharp_dependency",
            )

        enabled_tpl = _read_template("csharp", "manifest_enabled_langs")
        if enabled_tpl:
            manifest_content = _insert_after_if_missing(
                manifest_content,
                'if self.has_python() {\n            langs.push("python".to_string());\n        }\n\n',
                enabled_tpl.rstrip() + "\n",
                'langs.push("csharp"',
            )

        manifest_path.write_text(manifest_content)

    # build.rs
    build_path = REPO_ROOT / "poly-bench-project" / "src" / "build.rs"
    build_content = build_path.read_text()

    if "build_csharp_env" not in build_content:
        build_call_tpl = _read_template("csharp", "build_project_call")
        build_env_tpl = _read_template("csharp", "build_env")
        if build_call_tpl and build_env_tpl:
            # Insert build_project call after python block
            build_content = _insert_after_if_missing(
                build_content,
                "    }\n\n    println!()",
                "\n    " + build_call_tpl.rstrip() + "\n\n",
                "manifest.has_csharp()",
            )
            # Insert build_csharp_env fn after build_python_env
            build_content = _insert_after_if_missing(
                build_content,
                "    terminal::success_indented(\"Python environment ready\");\n\n    Ok(())\n}\n\nfn ",
                build_env_tpl.rstrip() + "\n\n",
                "fn build_csharp_env",
            )
            build_path.write_text(build_content)

    # templates.rs - csharp_csproj
    templates_path = REPO_ROOT / "poly-bench-project" / "src" / "templates.rs"
    templates_content = templates_path.read_text()

    if "csharp_csproj" not in templates_content:
        csproj_tpl = _read_template("csharp", "csproj")
        if csproj_tpl:
            # Insert csharp_csproj after tsconfig_json (before /// Internal Python deps)
            templates_content = _insert_after_if_missing(
                templates_content,
                '"#\n    .to_string()\n}\n\n/// Internal Python deps',
                csproj_tpl.rstrip() + "\n\n",
                "csharp_csproj",
            )
            templates_path.write_text(templates_content)


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Add or regenerate language boilerplate for poly-bench"
    )
    parser.add_argument(
        "lang",
        nargs="?",
        help="Language to add/regenerate (e.g. csharp). If omitted, regenerates all from languages.toml.",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Print what would be done without making changes",
    )
    args = parser.parse_args()

    if not LANGUAGES_TOML.exists():
        print(f"Error: {LANGUAGES_TOML} not found", file=sys.stderr)
        return 1

    languages = load_languages()
    if not languages:
        print("Error: No languages defined in languages.toml", file=sys.stderr)
        return 1

    if args.lang:
        if args.lang not in languages:
            print(f"Error: Unknown language '{args.lang}'. Known: {list(languages.keys())}", file=sys.stderr)
            return 1
        languages = {args.lang: languages[args.lang]}

    if args.dry_run:
        print(f"Would regenerate for: {list(languages.keys())}")
        return 0

    # Regenerate pure codegen files (from full languages.toml for consistency)
    all_languages = load_languages()
    regenerate_ast_rs(all_languages)
    regenerate_tokens_rs(all_languages)
    regenerate_partial_ast_rs(all_languages)
    regenerate_config_rs(all_languages)
    regenerate_grammar_js(all_languages)
    regenerate_injections_scm(all_languages)
    regenerate_parser_rs(all_languages)
    regenerate_tmlanguage(all_languages)
    regenerate_stdlib_anvil(all_languages)
    regenerate_package_json(all_languages)
    regenerate_project_lib(all_languages)
    regenerate_executor_lib(all_languages)
    regenerate_executor_validation_scheduler(all_languages)
    apply_csharp_templates()

    print(f"Regenerated language boilerplate for: {list(all_languages.keys())}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
