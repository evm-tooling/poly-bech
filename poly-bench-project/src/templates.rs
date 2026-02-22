//! Template strings for generated project files

/// Generate the example.bench file content
pub fn example_bench(has_go: bool, has_ts: bool, has_rust: bool) -> String {
    let mut content = String::new();

    content.push_str("suite example {\n");
    content.push_str("    description: \"Example benchmark to get you started\"\n");
    content.push_str("    iterations: 50\n");
    content.push_str("    warmup: 100\n");
    // Enable comparison if more than one language
    let lang_count = has_go as i32 + has_ts as i32 + has_rust as i32;
    if lang_count > 1 {
        content.push_str("    compare: true\n");
        if has_go {
            content.push_str("    baseline: \"go\"\n");
        } else if has_rust {
            content.push_str("    baseline: \"rust\"\n");
        }
    }
    content.push('\n');

    // Setup blocks with structured format
    if has_go {
        content.push_str("    setup go {\n");
        content.push_str("        import (\n");
        content.push_str("            \"crypto/sha256\"\n");
        content.push_str("        )\n");
        content.push('\n');
        content.push_str("        init {\n");
        content.push_str("        }\n");
        content.push('\n');
        content.push_str("        helpers {\n");
        content.push_str("            func sha256SumGo(data []byte) [32]byte {\n");
        content.push_str("                return sha256.Sum256(data)\n");
        content.push_str("            }\n");
        content.push_str("        }\n");
        content.push_str("    }\n");
        content.push('\n');
    }

    if has_ts {
        content.push_str("    setup ts {\n");
        content.push_str("        import {\n");
        content.push_str("            import { createHash } from 'node:crypto';\n");
        content.push_str("        }\n");
        content.push('\n');
        content.push_str("        init {\n");
        content.push_str("        }\n");
        content.push('\n');
        content.push_str("        helpers {\n");
        content.push_str("            function sha256SumTs(data: Uint8Array): Buffer {\n");
        content.push_str(
            "                return createHash('sha256').update(Buffer.from(data)).digest()\n",
        );
        content.push_str("            }\n");
        content.push_str("        }\n");
        content.push_str("    }\n");
        content.push('\n');
    }

    if has_rust {
        content.push_str("    setup rust {\n");
        content.push_str("        import {\n");
        content.push_str("            use sha2::{Sha256, Digest};\n");
        content.push_str("        }\n");
        content.push('\n');
        content.push_str("        init {\n");
        content.push_str("        }\n");
        content.push('\n');
        content.push_str("        helpers {\n");
        content.push_str("            fn sha256_sum_rust(data: &[u8]) -> [u8; 32] {\n");
        content.push_str("                let mut hasher = Sha256::new();\n");
        content.push_str("                hasher.update(data);\n");
        content.push_str("                hasher.finalize().into()\n");
        content.push_str("            }\n");
        content.push_str("        }\n");
        content.push_str("    }\n");
        content.push('\n');
    }

    // Fixture
    content.push_str("    fixture data {\n");
    content.push_str("        hex: \"68656c6c6f20776f726c64\"\n");
    content.push_str("    }\n");
    content.push('\n');

    // Benchmark
    content.push_str("    bench sha256Bench {\n");
    if has_go {
        content.push_str("        go: sha256SumGo(data)\n");
    }
    if has_ts {
        content.push_str("        ts: sha256SumTs(data)\n");
    }
    if has_rust {
        content.push_str("        rust: sha256_sum_rust(&data)\n");
    }
    content.push_str("    }\n");
    content.push('\n');

    // Charting example (commented out)
    if lang_count > 1 {
        content.push_str("    # Generate a bar chart after benchmarks complete:\n");
        content.push_str("    # after {\n");
        content.push_str("    #     charting.drawBarChart(\n");
        content.push_str("    #         title: \"SHA256 Performance\",\n");
        content.push_str("    #         xlabel: \"Benchmark\"\n");
        content.push_str("    #     )\n");
        content.push_str("    # }\n");
    }

    content.push_str("}\n");

    content
}

/// Generate a new benchmark file template
pub fn new_bench(name: &str, has_go: bool, has_ts: bool, has_rust: bool) -> String {
    let mut content = String::new();

    content.push_str(&format!("suite {} {{\n", name));
    content.push_str(&format!("    description: \"{} benchmarks\"\n", name));
    content.push_str("    iterations: 50\n");
    content.push_str("    warmup: 100\n");
    let lang_count = has_go as i32 + has_ts as i32 + has_rust as i32;
    if lang_count > 1 {
        content.push_str("    compare: true\n");
        if has_go {
            content.push_str("    baseline: \"go\"\n");
        } else if has_rust {
            content.push_str("    baseline: \"rust\"\n");
        }
    }
    content.push('\n');

    // Setup blocks with structured format
    if has_go {
        content.push_str("    setup go {\n");
        content.push_str("        import (\n");
        content.push_str("            // Add your Go imports here\n");
        content.push_str("        )\n");
        content.push('\n');
        content.push_str("        init {\n");
        content.push_str("            // Initialize variables, parse data, etc.\n");
        content.push_str("        }\n");
        content.push('\n');
        content.push_str("        helpers {\n");
        content.push_str("            // Define helper functions here\n");
        content.push_str("        }\n");
        content.push_str("    }\n");
        content.push('\n');
    }

    if has_ts {
        content.push_str("    setup ts {\n");
        content.push_str("        import {\n");
        content.push_str("            // Add your TypeScript imports here\n");
        content.push_str("        }\n");
        content.push('\n');
        content.push_str("        init {\n");
        content.push_str("            // Initialize variables, parse data, etc.\n");
        content.push_str("        }\n");
        content.push('\n');
        content.push_str("        helpers {\n");
        content.push_str("            // Define helper functions here\n");
        content.push_str("        }\n");
        content.push_str("    }\n");
        content.push('\n');
    }

    if has_rust {
        content.push_str("    setup rust {\n");
        content.push_str("        import {\n");
        content.push_str("            // Add your Rust use statements here\n");
        content.push_str("        }\n");
        content.push('\n');
        content.push_str("        init {\n");
        content.push_str("            // Initialize variables, parse data, etc.\n");
        content.push_str("        }\n");
        content.push('\n');
        content.push_str("        helpers {\n");
        content.push_str("            // Define helper functions here\n");
        content.push_str("        }\n");
        content.push_str("    }\n");
        content.push('\n');
    }

    // Fixture placeholder
    content.push_str("    # Define fixtures with hex data for portability:\n");
    content.push_str("    # fixture myData {\n");
    content.push_str("    #     hex: \"68656c6c6f\"\n");
    content.push_str("    # }\n");
    content.push('\n');

    // Benchmark placeholder
    content.push_str("    # Define benchmarks:\n");
    content.push_str("    # bench myBenchmark {\n");
    if has_go {
        content.push_str("    #     go: myHelperFunction(myData)\n");
    }
    if has_ts {
        content.push_str("    #     ts: myHelperFunction(myData)\n");
    }
    if has_rust {
        content.push_str("    #     rust: my_helper_function(&my_data)\n");
    }
    content.push_str("    # }\n");

    content.push_str("}\n");

    content
}

/// Generate go.mod content
pub fn go_mod(module_name: &str, go_version: Option<&str>) -> String {
    let version = go_version.unwrap_or("1.21");
    format!("module {}\n\ngo {}\n", module_name, version)
}

/// Generate package.json content
pub fn package_json(name: &str) -> String {
    serde_json::json!({
        "name": name,
        "version": "0.0.1",
        "type": "module",
        "private": true,
        "description": "poly-bench benchmarks",
        "scripts": {
            "bench": "poly-bench run"
        },
        "dependencies": {},
        "devDependencies": {
            "@types/node": "^22.0.0",
            "typescript": "^5.0.0"
        }
    })
    .to_string()
}

/// Generate package.json with pretty formatting
pub fn package_json_pretty(name: &str) -> String {
    serde_json::to_string_pretty(&serde_json::json!({
        "name": name,
        "version": "0.0.1",
        "type": "module",
        "private": true,
        "description": "poly-bench benchmarks",
        "scripts": {
            "bench": "poly-bench run"
        },
        "dependencies": {},
        "devDependencies": {
            "@types/node": "^22.0.0",
            "typescript": "^5.0.0"
        }
    }))
    .unwrap_or_else(|_| package_json(name))
}

/// Generate Cargo.toml content for Rust runtime environment
/// Note: Package name is always "polybench_runner" to match the executor's expected binary name
pub fn cargo_toml(_name: &str, edition: &str) -> String {
    format!(
        r#"[package]
name = "polybench_runner"
version = "0.1.0"
edition = "{}"

# Mark this as a standalone workspace to avoid being included in parent workspaces
[workspace]

[dependencies]
serde = {{ version = "1", features = ["derive"] }}
serde_json = "1"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
"#,
        edition
    )
}

/// Generate tsconfig.json content for TypeScript runtime environment
pub fn tsconfig_json() -> String {
    r#"{
  "compilerOptions": {
    "target": "ES2022",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "esModuleInterop": true,
    "strict": true,
    "skipLibCheck": true,
    "noEmit": true,
    "resolveJsonModule": true,
    "allowSyntheticDefaultImports": true,
    "forceConsistentCasingInFileNames": true,
    "types": ["node"]
  },
  "include": ["*.ts", "**/*.ts"],
  "exclude": ["node_modules"]
}
"#
    .to_string()
}

/// Generate .gitignore content
pub fn gitignore() -> &'static str {
    r#"# poly-bench generated files
out/
.polybench/

# Go
*.exe
*.exe~
*.dll
*.so
*.dylib

# Rust
target/
Cargo.lock

# Node.js
node_modules/
npm-debug.log*
yarn-debug.log*
yarn-error.log*

# IDE
.idea/
.vscode/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db
"#
}

/// Generate README.md content
pub fn readme(name: &str, has_go: bool, has_ts: bool, has_rust: bool) -> String {
    let mut content = String::new();

    content.push_str(&format!("# {}\n\n", name));
    content.push_str("A poly-bench project for cross-language benchmarking.\n\n");

    content.push_str("## Getting Started\n\n");
    content.push_str("1. Install dependencies:\n\n");
    content.push_str("```bash\n");
    content.push_str("poly-bench install\n");
    content.push_str("```\n\n");

    content.push_str("2. Run benchmarks:\n\n");
    content.push_str("```bash\n");
    content.push_str("poly-bench run\n");
    content.push_str("```\n\n");

    content.push_str("## Project Structure\n\n");
    content.push_str("```\n");
    content.push_str(&format!("{}/\n", name));
    content.push_str("├── polybench.toml       # Project configuration\n");
    content.push_str("├── benchmarks/          # Benchmark files (.bench)\n");
    content.push_str("│   └── example.bench    # Example benchmark\n");
    content.push_str("├── out/                 # Results and charts (gitignored)\n");
    content.push_str("│   ├── results.json     # Benchmark results\n");
    content.push_str("│   └── *.svg            # Generated charts\n");
    content.push_str("└── .polybench/          # Runtime environment (gitignored)\n");
    content.push_str("    └── runtime-env/      # Per-runtime deps and harness\n");
    if has_go {
        content.push_str("        └── go/           # go.mod, go.sum, generated bench code\n");
    }
    if has_ts {
        content.push_str(
            "        └── ts/           # package.json, node_modules, generated bench code\n",
        );
    }
    if has_rust {
        content.push_str("        └── rust/         # Cargo.toml, target/, generated bench code\n");
    }
    content.push_str("```\n\n");

    content.push_str("## Adding Dependencies\n\n");

    if has_go {
        content.push_str("### Go\n\n");
        content.push_str("```bash\n");
        content.push_str("poly-bench add --go \"github.com/ethereum/go-ethereum@v1.13.0\"\n");
        content.push_str("```\n\n");
    }

    if has_ts {
        content.push_str("### TypeScript\n\n");
        content.push_str("```bash\n");
        content.push_str("poly-bench add --ts \"viem@^2.0.0\"\n");
        content.push_str("```\n\n");
    }

    if has_rust {
        content.push_str("### Rust\n\n");
        content.push_str("```bash\n");
        content.push_str("poly-bench add --rs \"sha2@0.10\"\n");
        content.push_str("```\n\n");
    }

    content.push_str("## Creating New Benchmarks\n\n");
    content.push_str("```bash\n");
    content.push_str("poly-bench new my-benchmark\n");
    content.push_str("```\n\n");

    content.push_str("This creates `benchmarks/my-benchmark.bench` with a template.\n\n");

    content.push_str("## DSL Reference\n\n");
    content.push_str("```bench\n");
    content.push_str("suite my_suite {\n");
    content.push_str("    iterations: 1000\n");
    content.push_str("    warmup: 100\n");
    content.push_str("    description: \"My benchmarks\"\n");
    content.push_str("\n");
    if has_go {
        content.push_str("    setup go {\n");
        content.push_str("        import \"my/package\"\n");
        content.push_str("    }\n");
        content.push_str("\n");
    }
    if has_ts {
        content.push_str("    setup ts {\n");
        content.push_str("        import { myFunc } from 'my-package';\n");
        content.push_str("    }\n");
        content.push_str("\n");
    }
    if has_rust {
        content.push_str("    setup rust {\n");
        content.push_str("        use my_crate::my_func;\n");
        content.push_str("    }\n");
        content.push_str("\n");
    }
    content.push_str("    fixture data {\n");
    content.push_str("        hex: \"68656c6c6f\"  // Binary data as hex\n");
    content.push_str("    }\n");
    content.push_str("\n");
    content.push_str("    bench my_benchmark {\n");
    if has_go {
        content.push_str("        go: myFunc(data)\n");
    }
    if has_ts {
        content.push_str("        ts: myFunc(data)\n");
    }
    if has_rust {
        content.push_str("        rust: my_func(&data)\n");
    }
    content.push_str("    }\n");
    content.push_str("}\n");
    content.push_str("```\n");

    content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_bench_both_languages() {
        let content = example_bench(true, true, false);
        assert!(content.contains("setup go"));
        assert!(content.contains("setup ts"));
        assert!(content.contains("sha256SumGo(data)"));
        assert!(content.contains("sha256SumTs(data)"));
        assert!(content.contains("compare: true"));
        assert!(content.contains("baseline: \"go\""));
        assert!(content.contains("helpers {"));
    }

    #[test]
    fn test_example_bench_go_only() {
        let content = example_bench(true, false, false);
        assert!(content.contains("setup go"));
        assert!(!content.contains("setup ts"));
        assert!(!content.contains("compare: true")); // No comparison with single language
    }

    #[test]
    fn test_example_bench_all_languages() {
        let content = example_bench(true, true, true);
        assert!(content.contains("setup go"));
        assert!(content.contains("setup ts"));
        assert!(content.contains("setup rust"));
        assert!(content.contains("sha256SumGo(data)"));
        assert!(content.contains("sha256SumTs(data)"));
        assert!(content.contains("sha256_sum_rust(&data)"));
        assert!(content.contains("compare: true"));
        assert!(content.contains("baseline: \"go\""));
    }

    #[test]
    fn test_example_bench_rust_only() {
        let content = example_bench(false, false, true);
        assert!(!content.contains("setup go"));
        assert!(!content.contains("setup ts"));
        assert!(content.contains("setup rust"));
        assert!(content.contains("sha256_sum_rust(&data)"));
        assert!(!content.contains("compare: true")); // No comparison with single language
    }

    #[test]
    fn test_go_mod() {
        let content = go_mod("my-project", Some("1.22"));
        assert!(content.contains("module my-project"));
        assert!(content.contains("go 1.22"));
    }

    #[test]
    fn test_package_json() {
        let content = package_json_pretty("my-project");
        assert!(content.contains("\"name\": \"my-project\""));
        assert!(content.contains("\"type\": \"module\""));
        assert!(content.contains("@types/node"));
        assert!(content.contains("typescript"));
        assert!(content.contains("devDependencies"));
    }

    #[test]
    fn test_cargo_toml() {
        let content = cargo_toml("my-project", "2021");
        // Package name is always "polybench_runner" to match executor's expected binary name
        assert!(content.contains("name = \"polybench_runner\""));
        assert!(content.contains("edition = \"2021\""));
        assert!(content.contains("serde"));
        assert!(content.contains("serde_json"));
        // Verify release profile optimizations are included
        assert!(content.contains("[profile.release]"));
        assert!(content.contains("opt-level = 3"));
        assert!(content.contains("lto = true"));
        assert!(content.contains("codegen-units = 1"));
    }

    #[test]
    fn test_tsconfig_json() {
        let content = tsconfig_json();
        assert!(content.contains("\"types\": [\"node\"]"));
        assert!(content.contains("ES2022"));
    }
}
