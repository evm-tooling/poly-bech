//! Hover provider for the LSP v2
//!
//! This module provides hover information for keywords and identifiers
//! in poly-bench files, including embedded Go code via gopls,
//! TypeScript code via typescript-language-server, Rust code via rust-analyzer,
//! Python code via pyright/pylsp, and C# via OmniSharp.
use poly_bench_dsl::Lang;
use poly_bench_runtime::lang_full_name;
use tower_lsp::lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind, Position, Url};

use crate::{
    document::Document,
    embedded::{extract_embedded_blocks, find_block_at_offset, EmbeddedConfig},
    hover_cache::{cache_hover, get_cached_hover},
    virtual_files::VirtualFileManagers,
};

/// Get hover information at a position with full embedded language support
pub fn get_hover(
    doc: &Document,
    position: Position,
    config: &EmbeddedConfig,
    uri: &Url,
    managers: &VirtualFileManagers,
) -> Option<Hover> {
    let source = doc.source_text();
    let offset = doc.position_to_byte(position.line as usize, position.character as usize);

    // Extract embedded blocks from the partial AST
    let blocks = extract_embedded_blocks(&doc.partial_ast);

    // Check if we're in an embedded code block
    if let Some(block) = find_block_at_offset(&blocks, offset) {
        tracing::trace!(
            "[hover] embedded block lang={:?} uri={} at {}:{}",
            block.lang,
            uri,
            position.line,
            position.character
        );
        // Check cache first
        if let Some(cached) = get_cached_hover(uri, position) {
            tracing::trace!("[hover] cache hit");
            return cached;
        }

        let hover = if let Some(module_root) = config.module_root(block.lang) {
            tracing::trace!("[hover] module_root found: {}", module_root);
            let bench_uri = uri.as_str();
            let bench_path = uri.to_file_path().ok().map(|p| p.to_string_lossy().to_string());

            bench_path.as_ref().and_then(|bp| {
                poly_bench_runtime::get_embedded_hover_provider(
                    poly_bench_lsp_traits::syntax_lang_to_dsl(block.lang),
                )
                .and_then(|p| {
                    let ctx = crate::embedded_hover_context::LspEmbeddedHoverContext {
                        doc,
                        bench_uri,
                        bench_path: bp.as_str(),
                        blocks: &blocks,
                        module_root,
                        bench_offset: offset,
                        managers,
                    };
                    p.get_hover(&ctx)
                })
            })
        } else {
            tracing::warn!("[hover] no module_root for embedded lang {:?}", block.lang);
            None
        };

        // Cache the result
        cache_hover(uri, position, hover.clone());

        if hover.is_some() {
            return hover;
        }

        // Fallback to stdlib symbols
        if let Some(hover) = get_stdlib_symbol_hover(doc, position) {
            return Some(hover);
        }
    }

    // Try DSL hover (keywords, AST nodes)
    tracing::trace!("[hover] falling back to DSL hover");
    get_dsl_hover(doc, position, &source)
}

/// Get hover for DSL keywords and AST nodes
fn get_dsl_hover(doc: &Document, position: Position, source: &str) -> Option<Hover> {
    let point = tree_sitter::Point::new(position.line as usize, position.character as usize);

    let node = doc.tree.root_node().descendant_for_point_range(point, point)?;
    let kind = node.kind();
    let text = node.utf8_text(source.as_bytes()).ok()?;

    // Check for specific node types first
    let content = match kind {
        "suite" | "suite_keyword" => {
            let name = if kind == "suite" {
                node.child_by_field_name("name")
                    .and_then(|n| n.utf8_text(source.as_bytes()).ok())
                    .unwrap_or("<unnamed>")
            } else {
                "<unnamed>"
            };
            format!("**Suite**: `{}`\n\n{}", name, keyword_docs("suite").unwrap_or(""))
        }
        "benchmark" | "bench_keyword" | "bench" | "benchAsync" => {
            let name = if kind == "benchmark" {
                node.child_by_field_name("name")
                    .and_then(|n| n.utf8_text(source.as_bytes()).ok())
                    .unwrap_or("<unnamed>")
            } else {
                "<unnamed>"
            };
            let bench_keyword = if kind == "benchmark" {
                if let Ok(raw) = node.utf8_text(source.as_bytes()) {
                    if raw.trim_start().starts_with("benchAsync ") {
                        "benchAsync"
                    } else {
                        "bench"
                    }
                } else {
                    "bench"
                }
            } else if text == "benchAsync" || kind == "benchAsync" {
                "benchAsync"
            } else {
                "bench"
            };
            format!("**Benchmark**: `{}`\n\n{}", name, keyword_docs(bench_keyword).unwrap_or(""))
        }
        "fixture" | "fixture_keyword" => {
            let name = if kind == "fixture" {
                node.child_by_field_name("name")
                    .and_then(|n| n.utf8_text(source.as_bytes()).ok())
                    .unwrap_or("<unnamed>")
            } else {
                "<unnamed>"
            };
            format!("**Fixture**: `{}`\n\n{}", name, keyword_docs("fixture").unwrap_or(""))
        }
        "property_name" => get_property_documentation(text),
        "chart_function_name" => get_chart_function_documentation(text),
        "language_tag" => {
            let display = Lang::from_str(text).map(lang_full_name).unwrap_or(text);
            format!("**Language**: `{}`\n\nEmbedded {} code block", text, display)
        }
        "identifier" => {
            // Check if it's a keyword
            if let Some(docs) = keyword_docs(text) {
                docs.to_string()
            } else if let Some(docs) = stdlib_symbol_docs(text) {
                docs.to_string()
            } else if let Some(docs) = stdlib_module_docs(text) {
                docs.to_string()
            } else {
                return None;
            }
        }
        _ => {
            // Try keyword lookup for the text
            if let Some(docs) = keyword_docs(text) {
                docs.to_string()
            } else if let Some(docs) = stdlib_symbol_docs(text) {
                docs.to_string()
            } else {
                return None;
            }
        }
    };

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: content,
        }),
        range: None,
    })
}

/// Get hover for stdlib symbols in embedded code
fn get_stdlib_symbol_hover(doc: &Document, position: Position) -> Option<Hover> {
    let source = doc.source_text();
    let point = tree_sitter::Point::new(position.line as usize, position.character as usize);

    let node = doc.tree.root_node().descendant_for_point_range(point, point)?;
    let text = node.utf8_text(source.as_bytes()).ok()?;

    if let Some(docs) = stdlib_symbol_docs(text) {
        return Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: docs.to_string(),
            }),
            range: None,
        });
    }

    None
}

/// Get documentation for a property name
fn get_property_documentation(name: &str) -> String {
    match name {
        "description" => {
            "**description**: `string`\n\nA human-readable description of the suite or benchmark."
                .to_string()
        }
        "iterations" => {
            "**iterations**: `number`\n\nNumber of benchmark iterations to run.".to_string()
        }
        "warmup" => {
            "**warmup**: `number`\n\nNumber of warmup iterations before measurement.".to_string()
        }
        "timeout" => {
            "**timeout**: `duration`\n\nMaximum time allowed for the benchmark (e.g., `30s`, `5000ms`)."
                .to_string()
        }
        "order" => {
            "**order**: `sequential | random`\n\nOrder in which benchmarks are executed."
                .to_string()
        }
        "baseline" => {
            "**baseline**: `string`\n\nLanguage to use as the baseline for comparison.".to_string()
        }
        "sameDataset" => {
            "**sameDataset**: `boolean`\n\nSuite declaration header field for dataset relationship.".to_string()
        }
        "targetTime" => {
            "**targetTime**: `duration`\n\nTarget duration when declaration run mode is `timeBased`."
                .to_string()
        }
        "sink" => {
            "**sink**: `boolean`\n\nWhether to sink (consume) the result to prevent optimization."
                .to_string()
        }
        "memory" => {
            "**memory** (removed)\n\nUse `suiteType: memory` in the suite declaration instead."
                .to_string()
        }
        "outlierDetection" => {
            "**outlierDetection**: `boolean`\n\nEnable IQR-based outlier detection and removal."
                .to_string()
        }
        "cvThreshold" => {
            "**cvThreshold**: `number`\n\nCoefficient of variation threshold (%) for stability warnings."
                .to_string()
        }
        "fairness" => {
            "**fairness**: `\"legacy\" | \"strict\"`\n\nRuntime scheduling fairness mode.\n- `legacy`: previous grouped execution order\n- `strict`: interleaved per-run ordering across runtimes (fairness-first)"
                .to_string()
        }
        "fairnessSeed" => {
            "**fairnessSeed**: `number`\n\nDeterministic seed used to randomize strict fairness runtime order reproducibly."
                .to_string()
        }
        "asyncSamplingPolicy" => {
            "**asyncSamplingPolicy**: `\"timeBudgeted\" | \"fixedCap\"`\n\nControls async auto-mode sampling strategy.\n- `timeBudgeted`: sample until target time budget\n- `fixedCap`: sample a fixed number of async iterations"
                .to_string()
        }
        "asyncWarmupCap" => {
            "**asyncWarmupCap**: `number`\n\nUpper bound for async warmup iterations in auto mode.".to_string()
        }
        "asyncSampleCap" => {
            "**asyncSampleCap**: `number`\n\nUpper bound for stored async samples per run.".to_string()
        }
        "tags" => "**tags**: `string[]`\n\nLabels for filtering and grouping benchmarks.".to_string(),
        "requires" => {
            "**requires**: `string[]`\n\nLanguages that must have implementations.".to_string()
        }
        _ => format!("**{}**", name),
    }
}

/// Get documentation for a chart function
fn get_chart_function_documentation(name: &str) -> String {
    match name {
        "drawSpeedupChart" => {
            "**drawSpeedupChart**\n\nDraw a chart showing relative speedup compared to baseline."
                .to_string()
        }
        "drawTable" => "**drawTable**\n\nGenerate a table of benchmark results.".to_string(),
        "drawLineChart" => {
            "**drawLineChart**\n\nGenerate a multi-series line chart with stats overlays."
                .to_string()
        }
        "drawBarChart" => {
            "**drawBarChart**\n\nGenerate grouped bars with stats overlays.".to_string()
        }
        _ => format!("**{}**", name),
    }
}

/// Get documentation for a keyword
fn keyword_docs(word: &str) -> Option<&'static str> {
    match word {
        "suite" => Some(
            "**suite** `name { ... }`\n\n\
            Top-level benchmark suite container.\n\n\
            Contains setup blocks, fixtures, and benchmark definitions.",
        ),
        "bench" => Some(
            "**bench** `name { ... }`\n\n\
            Benchmark definition with language implementations.\n\n\
            ```\nbench encode {\n    go: encodeData(input)\n    ts: encodeData(input)\n}\n```",
        ),
        "benchAsync" => Some(
            "**benchAsync** `name { ... }`\n\n\
            Async-sequential benchmark definition.\n\n\
            Each iteration awaits completion before the next starts (no framework-managed concurrency).\n\n\
            ```\nbenchAsync fetchBlock {\n    ts: await getBlockNumber()\n}\n```",
        ),
        "setup" => Some(
            "**setup** `<lang> { ... }`\n\n\
            Per-language setup block with sections:\n\
            - `import` - Import statements\n\
            - `declare` - Package-level declarations\n\
            - `init` - Initialization code (runs once)\n\
            - `helpers` - Reusable helper functions",
        ),
        "fixture" => Some(
            "**fixture** `name { ... }`\n\n\
            Shared test data fixture.\n\n\
            Can contain:\n\
            - `hex:` - Hex-encoded binary data\n\
            - `go:` / `ts:` - Language-specific implementations\n\
            - `shape:` - JSON-like shape annotation",
        ),
        "import" => Some(
            "**import** `{ ... }` or `( ... )`\n\n\
            Import statements for the language.\n\n\
            For Go, use grouped imports:\n\
            ```\nimport (\n    \"fmt\"\n    \"encoding/json\"\n)\n```",
        ),
        "declare" => Some(
            "**declare** `{ ... }`\n\n\
            Package-level declarations (vars, types, consts).\n\n\
            Placed at package scope, outside any function.",
        ),
        "init" => Some(
            "**init** `{ ... }`\n\n\
            Initialization code that runs once before benchmarks.\n\n\
            Use for expensive setup that shouldn't be timed.",
        ),
        "helpers" => Some(
            "**helpers** `{ ... }`\n\n\
            Reusable helper functions available to all benchmarks.\n\n\
            Define functions that can be called from benchmark implementations.",
        ),
        "async" => Some(
            "**async** `init { ... }`\n\n\
            Mark init block as async (TypeScript only).\n\n\
            Allows using `await` in initialization code.",
        ),
        "description" => Some(
            "**description:** `\"...\"`\n\n\
            Human-readable description for the suite, fixture, or benchmark.",
        ),
        "iterations" => Some(
            "**iterations:** `<number>`\n\n\
            Number of benchmark iterations.\n\n\
            Default: 1000",
        ),
        "warmup" => Some(
            "**warmup:** `<number>`\n\n\
            Number of warmup iterations before measuring.\n\n\
            Default: 100",
        ),
        "timeout" => Some(
            "**timeout:** `<duration>`\n\n\
            Maximum execution time per benchmark.\n\n\
            Examples: `30s`, `500ms`, `1m`",
        ),
        "tags" => Some(
            "**tags:** `[\"tag1\", \"tag2\"]`\n\n\
            Labels for filtering and grouping benchmarks.",
        ),
        "requires" => Some(
            "**requires:** `[\"go\", \"ts\"]`\n\n\
            Languages that must have implementations.\n\n\
            Benchmarks missing required languages will error.",
        ),
        "order" => Some(
            "**order:** `sequential | parallel | random`\n\n\
            Execution order for benchmarks in the suite.\n\n\
            - `sequential` - Run in definition order (default)\n\
            - `parallel` - Run concurrently where supported\n\
            - `random` - Randomize order",
        ),
        "baseline" => Some(
            "**baseline:** `\"go\" | \"ts\"`\n\n\
            Baseline language for comparison ratios.\n\n\
            Other languages are compared against this baseline.",
        ),
        "sameDataset" => Some(
            "**sameDataset:** `true | false`\n\n\
            Suite-level flag in the declaration header describing whether benchmarks share the same dataset.\n\n\
            This metadata is used for chart-policy gating.",
        ),
        "targetTime" => Some(
            "**targetTime:** `<duration>`\n\n\
            Target duration when declaration run mode is `timeBased`.\n\n\
            Examples: `3000ms`, `10s`, `1m`\n\n\
            Default: `3000ms` (3 seconds)",
        ),
        "sink" => Some(
            "**sink:** `true | false`\n\n\
            Use sink/black-box pattern to prevent dead code elimination.\n\n\
            When enabled, the result of the benchmark expression is passed\n\
            to a sink function that prevents compiler optimizations from\n\
            eliminating the benchmarked code.\n\n\
            Default: `true`",
        ),
        "outlierDetection" => Some(
            "**outlierDetection:** `true | false`\n\n\
            Enable IQR-based outlier detection and removal.\n\n\
            When enabled, statistical outliers are identified using the\n\
            interquartile range (IQR) method and excluded from results.\n\
            This improves measurement stability.\n\n\
            Default: `true`",
        ),
        "cvThreshold" => Some(
            "**cvThreshold:** `<number>`\n\n\
            Coefficient of variation threshold (%) for stability warnings.\n\n\
            If the CV of measurements exceeds this threshold, a warning\n\
            is shown indicating the results may be unstable.\n\n\
            Default: `5` (5%)",
        ),
        "memory" => Some(
            "**memory** (removed)\n\n\
            Use `suiteType: memory` in the suite declaration instead.\n\n\
            Example: `declare suite mySuite memory timeBased sameDataset: true`",
        ),
        "fairness" => Some(
            "**fairness:** `\"legacy\" | \"strict\"`\n\n\
            Runtime scheduling fairness mode.\n\n\
            - `legacy` keeps previous grouped execution behavior\n\
            - `strict` interleaves runtime execution per run to reduce temporal bias\n\n\
            Default: `legacy`",
        ),
        "fairnessSeed" => Some(
            "**fairnessSeed:** `<number>`\n\n\
            Seed used for deterministic strict-fairness ordering.\n\n\
            Use a fixed value to reproduce runtime ordering and results.",
        ),
        "asyncSamplingPolicy" => Some(
            "**asyncSamplingPolicy:** `\"timeBudgeted\" | \"fixedCap\"`\n\n\
            Sampling strategy for async auto benchmarks.\n\n\
            - `timeBudgeted` samples until `targetTime` budget is reached\n\
            - `fixedCap` samples up to `asyncSampleCap` completed iterations\n\n\
            Default: `timeBudgeted`",
        ),
        "asyncWarmupCap" => Some(
            "**asyncWarmupCap:** `<number>`\n\n\
            Cap on async warmup iterations in auto mode.\n\n\
            Effective warmup is `min(warmup, asyncWarmupCap)`.",
        ),
        "asyncSampleCap" => Some(
            "**asyncSampleCap:** `<number>`\n\n\
            Cap on collected async samples per run.\n\n\
            Helps bound sample storage and run-time variance.",
        ),
        "legacy" => Some(
            "**legacy**\n\n\
            Fairness mode that preserves prior grouped runtime execution behavior.",
        ),
        "strict" => Some(
            "**strict**\n\n\
            Fairness mode that interleaves runtimes per run using deterministic ordering.",
        ),
        "timeBudgeted" => Some(
            "**timeBudgeted**\n\n\
            Async sampling policy that continues collecting iterations until target time is reached.",
        ),
        "fixedCap" => Some(
            "**fixedCap**\n\n\
            Async sampling policy that limits sampling to `asyncSampleCap` iterations.",
        ),
        "skip" => Some(
            "**skip** `<lang>:` `<condition>`\n\n\
            Skip benchmark if condition is true.\n\n\
            ```\nskip go: runtime.GOOS == \"windows\"\n```",
        ),
        "validate" => Some(
            "**validate** `<lang>:` `<expression>`\n\n\
            Validate benchmark result.\n\n\
            Expression should return a boolean.",
        ),
        "before" => Some(
            "**before** `<lang>:` `{ ... }`\n\n\
            Hook that runs once before all iterations.",
        ),
        "after" => Some(
            "**after** `<lang>:` `{ ... }` or `{ ... }`\n\n\
            Hook that runs once after all iterations.\n\n\
            **Benchmark-level hook:**\n\
            ```\nbench test {\n    after go: { cleanup() }\n}\n```\n\n\
            **Suite-level charting block:**\n\
            ```\nafter {\n    charting.drawSpeedupChart(title: \"Results\")\n}\n```",
        ),
        "each" => Some(
            "**each** `<lang>:` `{ ... }`\n\n\
            Hook that runs before each iteration.\n\n\
            Executed outside timing measurement.",
        ),
        "hex" => Some(
            "**hex:** `\"0x...\"` or `@file(\"path\")`\n\n\
            Hex-encoded binary data.\n\n\
            Portable format that works across all languages.\n\n\
            **Inline:** `hex: \"deadbeef\"`\n\
            **From file:** `hex: @file(\"testdata/input.hex\")`",
        ),
        "@file" | "file" => Some(
            "**@file** `(\"path/to/file\")`\n\n\
            Load hex data from an external file.\n\n\
            The file should contain hex-encoded binary data.\n\n\
            **Example:**\n\
            ```\nfixture largeData {\n    hex: @file(\"testdata/large_input.hex\")\n}\n```\n\n\
            Paths are relative to the .bench file location.",
        ),
        "shape" => Some(
            "**shape:** `\"type\"`\n\n\
            JSON-like type annotation for documentation.\n\n\
            Example: `shape: \"{ id: number, name: string }\"`",
        ),
        "sequential" => Some(
            "**sequential**\n\n\
            Run benchmarks in definition order.\n\n\
            This is the default execution order.",
        ),
        "parallel" => Some(
            "**parallel**\n\n\
            Run benchmarks concurrently where supported.\n\n\
            May improve total execution time.",
        ),
        "random" => Some(
            "**random**\n\n\
            Randomize benchmark execution order.\n\n\
            Helps detect order-dependent issues.",
        ),
        "use" => Some(
            "**use** `std::module`\n\n\
            Import a module from the poly-bench standard library.\n\n\
            Available modules:\n\
            - `anvil` - Anvil node integration (ANVIL_RPC_URL)\n\
            - `charting` - Chart generation (drawSpeedupChart, drawTable, drawLineChart, drawBarChart)\n\
            - `constants` - Mathematical constants (std_PI, std_E)",
        ),
        "globalSetup" => Some(
            "**globalSetup** `{ ... }`\n\n\
            Global setup block for suite-level initialization.\n\n\
            Can be placed inside a suite or at file level (for all suites).\n\n\
            **Available functions (with std::anvil):**\n\
            - `anvil.spawnAnvil()` - Spawn a local Anvil Ethereum node\n\
            - `anvil.spawnAnvil(fork: \"url\")` - Spawn with chain forking",
        ),
        "go" => Some(
            "**Go** language\n\n\
            Implementations are compiled and executed via Go plugin system.",
        ),
        "ts" | "typescript" => Some(
            "**TypeScript** language\n\n\
            Implementations are transpiled and executed via embedded V8 runtime.",
        ),
        "rust" | "rs" => Some(
            "**Rust** language\n\n\
            Native Rust benchmark support.",
        ),
        "python" | "py" => Some(
            "**Python** language\n\n\
            Python benchmark support.",
        ),
        "csharp" => Some(
            "**C#** language\n\n\
            C# benchmark support.",
        ),
        _ => None,
    }
}

/// Get documentation for a stdlib module
fn stdlib_module_docs(module: &str) -> Option<&'static str> {
    match module {
        "anvil" => Some(
            "**std::anvil**\n\n\
            Anvil Ethereum node integration from the poly-bench standard library.\n\n\
            When imported, poly-bench automatically spawns a local Anvil node before\n\
            running benchmarks and makes the RPC URL available via `ANVIL_RPC_URL`.\n\n\
            **Provided variables:**\n\
            - `ANVIL_RPC_URL` - The RPC endpoint URL (e.g., http://127.0.0.1:8545)\n\n\
            **Requirements:** Anvil must be installed (part of Foundry toolchain)",
        ),
        "charting" => Some(
            "**std::charting**\n\n\
            Chart generation from benchmark results.\n\n\
            Use in a suite-level `after { }` block to generate charts after benchmarks complete.\n\n\
            **Provided functions:**\n\
            - `charting.drawSpeedupChart()` - Generate a speedup comparison chart\n\
            - `charting.drawTable()` - Generate a data table\n\
            - `charting.drawLineChart()` - Generate a line chart with overlays\n\
            - `charting.drawBarChart()` - Generate a grouped bar chart with overlays",
        ),
        "constants" => Some(
            "**std::constants**\n\n\
            Mathematical constants from the poly-bench standard library.\n\n\
            **Provided constants:**\n\
            - `std_PI` - Pi (π ≈ 3.14159265358979323846)\n\
            - `std_E` - Euler's number (e ≈ 2.71828182845904523536)",
        ),
        "std" => Some(
            "**std**\n\n\
            Poly-bench standard library namespace.\n\n\
            Use `use std::module` to import a standard library module.\n\n\
            Available modules:\n\
            - `anvil` - Local Ethereum node management\n\
            - `charting` - Chart generation from benchmark results\n\
            - `constants` - Mathematical constants",
        ),
        _ => None,
    }
}

/// Get documentation for stdlib symbols
fn stdlib_symbol_docs(symbol: &str) -> Option<&'static str> {
    match symbol {
        "spawnAnvil" => Some(
            "**anvil.spawnAnvil** `()` or `(fork: \"url\")`\n\n\
            Spawn a local Anvil Ethereum node.\n\n\
            Anvil is started before benchmarks and stopped after.\n\
            The RPC URL is available as `anvil.ANVIL_RPC_URL` in benchmark code.\n\n\
            **Options:**\n\
            - `fork: \"url\"` - Fork from an existing chain",
        ),
        "ANVIL_RPC_URL" => Some(
            "```go\nvar ANVIL_RPC_URL string\n```\n\n\
            **anvil.ANVIL_RPC_URL** - Anvil RPC endpoint URL.\n\n\
            When `use std::anvil` is specified with `anvil.spawnAnvil()`,\n\
            poly-bench automatically starts an Anvil node and sets this variable\n\
            to its RPC URL (e.g., `http://127.0.0.1:8545`).\n\n\
            *From `std::anvil`*",
        ),
        "drawSpeedupChart" => Some(
            "**charting.drawSpeedupChart** `(...params)`\n\n\
            Generate a chart showing relative speedup compared to baseline.\n\n\
            **Basic Parameters:**\n\
            - `title` - Chart title (string)\n\
            - `baselineBenchmark` - Benchmark to use as baseline\n\
            - `rowCount` - Number of benchmark cards per row in combined charts\n\n\
            *From `std::charting`*",
        ),
        "drawTable" => Some(
            "**charting.drawTable** `(...params)`\n\n\
            Generate a table of benchmark results.\n\n\
            **Basic Parameters:**\n\
            - `title` - Table title (string)\n\n\
            *From `std::charting`*",
        ),
        "drawLineChart" => Some(
            "**charting.drawLineChart** `(...params)`\n\n\
            Generate a line chart across related benchmarks.\n\n\
            **Default-on overlays:**\n\
            - `showStdDev: true`\n\
            - `showErrorBars: true`\n\
            - `showRegression: true`\n\
            - `regressionModel: \"auto\"`\n\n\
            *From `std::charting`*",
        ),
        "drawBarChart" => Some(
            "**charting.drawBarChart** `(...params)`\n\n\
            Generate a grouped bar chart across related benchmarks.\n\n\
            **Default-on overlays:**\n\
            - `showStdDev: true`\n\
            - `showErrorBars: true`\n\
            - `showRegression: true`\n\
            - `regressionModel: \"auto\"`\n\n\
            *From `std::charting`*",
        ),
        "PI" | "std_PI" => Some(
            "```go\nconst PI float64 = 3.14159265358979323846\n```\n\n\
            **Pi (π)** - The ratio of a circle's circumference to its diameter.\n\n\
            *From `std::constants`*",
        ),
        "E" | "std_E" => Some(
            "```go\nconst E float64 = 2.71828182845904523536\n```\n\n\
            **Euler's number (e)** - The base of natural logarithms.\n\n\
            *From `std::constants`*",
        ),
        _ => None,
    }
}
