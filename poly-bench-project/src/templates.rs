//! Template strings for generated project files
use poly_bench_dsl::Lang;

fn has_lang(enabled_langs: &[Lang], lang: Lang) -> bool {
    enabled_langs.contains(&lang)
}

/// Generate the example.bench file content from a language list.
pub fn example_bench_for_langs(enabled_langs: &[Lang]) -> String {
    example_bench(
        has_lang(enabled_langs, Lang::Go),
        has_lang(enabled_langs, Lang::TypeScript),
        has_lang(enabled_langs, Lang::Rust),
        has_lang(enabled_langs, Lang::Python),
        has_lang(enabled_langs, Lang::C),
        has_lang(enabled_langs, Lang::CSharp),
        has_lang(enabled_langs, Lang::Zig),
    )
}

/// Generate the example.bench file content
/// Uses only standard library features - no external dependencies required
pub fn example_bench(
    has_go: bool,
    has_ts: bool,
    has_rust: bool,
    has_python: bool,
    has_c: bool,
    has_csharp: bool,
    has_zig: bool,
) -> String {
    let mut content = String::new();

    // Add charting import for the after block
    let lang_count = has_go as i32 +
        has_ts as i32 +
        has_rust as i32 +
        has_python as i32 +
        has_c as i32 +
        has_csharp as i32 +
        has_zig as i32;
    if lang_count > 1 {
        content.push_str("use std::charting\n\n");
    }

    content.push_str("suite example {\n");
    content.push_str("    description: \"Fibonacci benchmark - no external dependencies\"\n");
    content.push_str("    warmup: 10\n");

    // Set baseline if more than one language
    if lang_count > 1 {
        if has_go {
            content.push_str("    baseline: \"go\"\n");
        } else if has_rust {
            content.push_str("    baseline: \"rust\"\n");
        }
    }
    content.push_str("    targetTime: 2000ms\n");
    content.push_str("    mode: \"auto\"\n");
    content.push_str("    count: 2\n");
    content.push_str("    cvThreshold: 5\n");
    content.push('\n');

    // Setup blocks - no imports needed, just helper functions
    if has_go {
        content.push_str("    setup go {\n");
        content.push_str("        helpers {\n");
        content.push_str("            func fibGo(data []byte) []byte {\n");
        content.push_str("                n := int(data[0])\n");
        content.push_str("                if n <= 1 {\n");
        content.push_str("                    return []byte{byte(n)}\n");
        content.push_str("                }\n");
        content.push_str("                a, b := 0, 1\n");
        content.push_str("                for i := 2; i <= n; i++ {\n");
        content.push_str("                    a, b = b, a+b\n");
        content.push_str("                }\n");
        content.push_str("                return []byte{byte(b & 0xFF)}\n");
        content.push_str("            }\n");
        content.push_str("        }\n");
        content.push_str("    }\n");
        content.push('\n');
    }

    if has_ts {
        content.push_str("    setup ts {\n");
        content.push_str("        helpers {\n");
        content.push_str("            function fibTs(data: Uint8Array): Uint8Array {\n");
        content.push_str("                const n = data[0]\n");
        content.push_str("                if (n <= 1) return new Uint8Array([n])\n");
        content.push_str("                let a = 0, b = 1\n");
        content.push_str("                for (let i = 2; i <= n; i++) {\n");
        content.push_str("                    [a, b] = [b, a + b]\n");
        content.push_str("                }\n");
        content.push_str("                return new Uint8Array([b & 0xFF])\n");
        content.push_str("            }\n");
        content.push_str("        }\n");
        content.push_str("    }\n");
        content.push('\n');
    }

    if has_rust {
        content.push_str("    setup rust {\n");
        content.push_str("        helpers {\n");
        content.push_str("            fn fib_rust(data: &[u8]) -> Vec<u8> {\n");
        content.push_str("                let n = data[0] as usize;\n");
        content.push_str("                if n <= 1 {\n");
        content.push_str("                    return vec![n as u8];\n");
        content.push_str("                }\n");
        content.push_str("                let (mut a, mut b) = (0u64, 1u64);\n");
        content.push_str("                for _ in 2..=n {\n");
        content.push_str("                    let tmp = a + b;\n");
        content.push_str("                    a = b;\n");
        content.push_str("                    b = tmp;\n");
        content.push_str("                }\n");
        content.push_str("                vec![(b & 0xFF) as u8]\n");
        content.push_str("            }\n");
        content.push_str("        }\n");
        content.push_str("    }\n");
        content.push('\n');
    }

    if has_python {
        content.push_str("    setup python {\n");
        content.push_str("        helpers {\n");
        content.push_str("            def fib_python(data: bytes) -> bytes:\n");
        content.push_str("                n = data[0]\n");
        content.push_str("                if n <= 1:\n");
        content.push_str("                    return bytes([n])\n");
        content.push_str("                a, b = 0, 1\n");
        content.push_str("                for _ in range(2, n + 1):\n");
        content.push_str("                    a, b = b, a + b\n");
        content.push_str("                return bytes([b & 0xFF])\n");
        content.push_str("        }\n");
        content.push_str("    }\n");
        content.push('\n');
    }

    if has_c {
        content.push_str("    setup c {\n");
        content.push_str("        import {\n");
        content.push_str("            #include <stdint.h>\n");
        content.push_str("        }\n");
        content.push_str("        helpers {\n");
        content.push_str("            static unsigned char* fib_c(unsigned char* data) {\n");
        content.push_str("                int n = data[0];\n");
        content.push_str("                if (n <= 1) {\n");
        content.push_str("                    data[0] = (unsigned char)n;\n");
        content.push_str("                    return data;\n");
        content.push_str("                }\n");
        content.push_str("                int a = 0, b = 1;\n");
        content.push_str("                for (int i = 2; i <= n; i++) {\n");
        content.push_str("                    int next = a + b;\n");
        content.push_str("                    a = b;\n");
        content.push_str("                    b = next;\n");
        content.push_str("                }\n");
        content.push_str("                data[0] = (unsigned char)(b & 0xFF);\n");
        content.push_str("                return data;\n");
        content.push_str("            }\n");
        content.push_str("        }\n");
        content.push_str("    }\n");
        content.push('\n');
    }

    if has_csharp {
        content.push_str("    setup csharp {\n");
        content.push_str("        import {\n");
        content.push_str("            using System;\n");
        content.push_str("        }\n");
        content.push_str("        helpers {\n");
        content.push_str("            static byte[] fib_csharp(byte[] data) {\n");
        content.push_str("                int n = data[0];\n");
        content.push_str("                if (n <= 1) return new byte[] { (byte)n };\n");
        content.push_str("                int a = 0, b = 1;\n");
        content.push_str("                for (int i = 2; i <= n; i++) {\n");
        content.push_str("                    int next = a + b;\n");
        content.push_str("                    a = b;\n");
        content.push_str("                    b = next;\n");
        content.push_str("                }\n");
        content.push_str("                return new byte[] { (byte)(b & 0xFF) };\n");
        content.push_str("            }\n");
        content.push_str("        }\n");
        content.push_str("    }\n");
        content.push('\n');
    }

    if has_zig {
        content.push_str("    setup zig {\n");
        content.push_str("        helpers {\n");
        content.push_str("            fn fibZig(data: []const u8) u8 {\n");
        content.push_str("                const n = data[0];\n");
        content.push_str("                if (n <= 1) return @intCast(n);\n");
        content.push_str("                var a: u32 = 0;\n");
        content.push_str("                var b: u32 = 1;\n");
        content.push_str("                for (2..n + 1) |_| {\n");
        content.push_str("                    const next = a + b;\n");
        content.push_str("                    a = b;\n");
        content.push_str("                    b = next;\n");
        content.push_str("                }\n");
        content.push_str("                return @intCast(b & 0xFF);\n");
        content.push_str("            }\n");
        content.push_str("        }\n");
        content.push_str("    }\n");
        content.push('\n');
    }

    // Fixtures with different input sizes (n values encoded as single bytes)
    content.push_str("    fixture n20 {\n");
    content.push_str("        hex: \"14\"\n");
    content.push_str("    }\n");
    content.push('\n');
    content.push_str("    fixture n30 {\n");
    content.push_str("        hex: \"1e\"\n");
    content.push_str("    }\n");
    content.push('\n');
    content.push_str("    fixture n40 {\n");
    content.push_str("        hex: \"28\"\n");
    content.push_str("    }\n");
    content.push('\n');
    content.push_str("    fixture n50 {\n");
    content.push_str("        hex: \"32\"\n");
    content.push_str("    }\n");
    content.push('\n');
    content.push_str("    fixture n60 {\n");
    content.push_str("        hex: \"3C\"\n");
    content.push_str("    }\n");
    content.push('\n');
    content.push_str("    fixture n70 {\n");
    content.push_str("        hex: \"46\"\n");
    content.push_str("    }\n");
    content.push('\n');

    // Benchmarks for each input size
    content.push_str("    bench fib20 {\n");
    if has_go {
        content.push_str("        go: fibGo(n20)\n");
    }
    if has_ts {
        content.push_str("        ts: fibTs(n20)\n");
    }
    if has_rust {
        content.push_str("        rust: fib_rust(&n20)\n");
    }
    if has_python {
        content.push_str("        python: fib_python(n20)\n");
    }
    if has_c {
        content.push_str("        c: fib_c(n20)\n");
    }
    if has_csharp {
        content.push_str("        csharp: fib_csharp(n20)\n");
    }
    if has_zig {
        content.push_str("        zig: fibZig(n20[0..])\n");
    }
    content.push_str("    }\n");
    content.push('\n');

    content.push_str("    bench fib30 {\n");
    if has_go {
        content.push_str("        go: fibGo(n30)\n");
    }
    if has_ts {
        content.push_str("        ts: fibTs(n30)\n");
    }
    if has_rust {
        content.push_str("        rust: fib_rust(&n30)\n");
    }
    if has_python {
        content.push_str("        python: fib_python(n30)\n");
    }
    if has_c {
        content.push_str("        c: fib_c(n30)\n");
    }
    if has_csharp {
        content.push_str("        csharp: fib_csharp(n30)\n");
    }
    if has_zig {
        content.push_str("        zig: fibZig(n30[0..])\n");
    }
    content.push_str("    }\n");
    content.push('\n');

    content.push_str("    bench fib40 {\n");
    if has_go {
        content.push_str("        go: fibGo(n40)\n");
    }
    if has_ts {
        content.push_str("        ts: fibTs(n40)\n");
    }
    if has_rust {
        content.push_str("        rust: fib_rust(&n40)\n");
    }
    if has_python {
        content.push_str("        python: fib_python(n40)\n");
    }
    if has_c {
        content.push_str("        c: fib_c(n40)\n");
    }
    if has_csharp {
        content.push_str("        csharp: fib_csharp(n40)\n");
    }
    if has_zig {
        content.push_str("        zig: fibZig(n40[0..])\n");
    }
    content.push_str("    }\n");
    content.push('\n');

    content.push_str("    bench fib50 {\n");
    if has_go {
        content.push_str("        go: fibGo(n50)\n");
    }
    if has_ts {
        content.push_str("        ts: fibTs(n50)\n");
    }
    if has_rust {
        content.push_str("        rust: fib_rust(&n50)\n");
    }
    if has_python {
        content.push_str("        python: fib_python(n50)\n");
    }
    if has_c {
        content.push_str("        c: fib_c(n50)\n");
    }
    if has_csharp {
        content.push_str("        csharp: fib_csharp(n50)\n");
    }
    if has_zig {
        content.push_str("        zig: fibZig(n50[0..])\n");
    }
    content.push_str("    }\n");
    content.push('\n');

    content.push_str("    bench fib60 {\n");
    if has_go {
        content.push_str("        go: fibGo(n60)\n");
    }
    if has_ts {
        content.push_str("        ts: fibTs(n60)\n");
    }
    if has_rust {
        content.push_str("        rust: fib_rust(&n60)\n");
    }
    if has_python {
        content.push_str("        python: fib_python(n60)\n");
    }
    if has_c {
        content.push_str("        c: fib_c(n60)\n");
    }
    if has_csharp {
        content.push_str("        csharp: fib_csharp(n60)\n");
    }
    if has_zig {
        content.push_str("        zig: fibZig(n60[0..])\n");
    }
    content.push_str("    }\n");
    content.push('\n');

    content.push_str("    bench fib70 {\n");
    if has_go {
        content.push_str("        go: fibGo(n70)\n");
    }
    if has_ts {
        content.push_str("        ts: fibTs(n70)\n");
    }
    if has_rust {
        content.push_str("        rust: fib_rust(&n70)\n");
    }
    if has_python {
        content.push_str("        python: fib_python(n70)\n");
    }
    if has_c {
        content.push_str("        c: fib_c(n70)\n");
    }
    if has_csharp {
        content.push_str("        csharp: fib_csharp(n70)\n");
    }
    if has_zig {
        content.push_str("        zig: fibZig(n70[0..])\n");
    }
    content.push_str("    }\n");

    // Charting block with all chart types
    if lang_count > 1 {
        content.push('\n');
        content.push_str("    after {\n");
        content.push_str("        charting.drawSpeedupChart(\n");
        content.push_str("            title: \"Fibonacci Speedup\",\n");
        content.push_str("            description: \"Relative performance vs baseline\",\n");
        content.push_str("            output: \"fib-speedup.svg\",\n");
        content.push_str("            baseline: \"go\",\n");
        content.push_str("            sortBy: \"name\",\n");
        content.push_str("            sortOrder: \"asc\",\n");
        content.push_str("            legendPosition: \"top-left\"\n");
        content.push_str("        )\n");
        content.push('\n');
        content.push_str("        charting.drawTable(\n");
        content.push_str("            title: \"Fibonacci Results Table\",\n");
        content.push_str("            description: \"Raw timings and winners\",\n");
        content.push_str("            output: \"fib-table.svg\",\n");
        content.push_str("            sortBy: \"name\",\n");
        content.push_str("            sortOrder: \"asc\"\n");
        content.push_str("        )\n");
        content.push_str("    }\n");
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
            "typescript": "^5.0.0",
            "typescript-language-server": "^3.0.0"
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
            "typescript": "^5.0.0",
            "typescript-language-server": "^3.0.0"
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

/// Generate NuGet.config for C# runtime env. Adds vs-impl feed so dotnet tool install
/// and restore can resolve roslyn-language-server (fixes DotnetToolSettings.xml packaging issues).
pub fn csharp_nuget_config() -> &'static str {
    r#"<?xml version="1.0" encoding="utf-8"?>
<configuration>
  <packageSources>
    <add key="vs-impl" value="https://pkgs.dev.azure.com/azure-public/vside/_packaging/vs-impl/nuget/v3/index.json" />
  </packageSources>
</configuration>
"#
}

/// Generate C# project file content
pub fn csharp_csproj(target_framework: &str) -> String {
    format!(
        r#"<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>{}</TargetFramework>
    <ImplicitUsings>enable</ImplicitUsings>
    <Nullable>enable</Nullable>
    <UseAppHost>false</UseAppHost>
  </PropertyGroup>
</Project>
"#,
        target_framework
    )
}

/// Generate build.zig for Zig runtime env
pub fn build_zig() -> String {
    r#"const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "polybench-runner",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    b.installArtifact(exe);
}
"#
    .to_string()
}

/// Generate build.zig.zon for Zig runtime env
pub fn build_zig_zon() -> String {
    r#".{
    .name = "polybench",
    .version = "0.0.1",
}
"#
    .to_string()
}

/// Generate src/main.zig for Zig runtime env
pub fn main_zig() -> String {
    "pub fn main() void {}\n".to_string()
}

/// Internal Python deps always included in .polybench runtime-env (e.g. pyright for LSP).
/// These are never in polybench.toml; install/remove must not strip them from requirements.txt.
const PYTHON_INTERNAL_DEPS: &[(&str, &str)] = &[("pyright[nodejs]", "latest")];

/// Generate requirements.txt for Python runtime env.
/// When writing to .polybench/runtime-env/python/, use `requirements_txt_for_runtime_env` to
/// preserve internal deps (pyright) that must not be removed by install/remove.
pub fn requirements_txt(deps: &[(String, String)]) -> String {
    format_requirements_txt(deps)
}

/// Generate requirements.txt for .polybench runtime-env, always including internal deps (pyright).
/// Use this when writing to .polybench/runtime-env/python/ so install/remove never strips pyright.
pub fn requirements_txt_for_runtime_env(manifest_deps: &[(String, String)]) -> String {
    let mut all: Vec<(String, String)> = manifest_deps.to_vec();
    for (k, v) in PYTHON_INTERNAL_DEPS {
        if !all.iter().any(|(d, _)| d == *k || d == "pyright") {
            all.push((k.to_string(), v.to_string()));
        }
    }
    format_requirements_txt(&all)
}

fn format_requirements_txt(deps: &[(String, String)]) -> String {
    if deps.is_empty() {
        "# Python dependencies for poly-bench runtime\n# Add packages with: poly-bench add --py \"package==version\"\n".to_string()
    } else {
        let mut lines = vec!["# Python dependencies for poly-bench runtime".to_string()];
        for (pkg, ver) in deps {
            let line = if ver.eq_ignore_ascii_case("latest") {
                pkg.clone()
            } else {
                format!("{}=={}", pkg, ver)
            };
            lines.push(line);
        }
        lines.join("\n") + "\n"
    }
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
pub fn readme_for_langs(name: &str, enabled_langs: &[Lang]) -> String {
    readme(
        name,
        has_lang(enabled_langs, Lang::Go),
        has_lang(enabled_langs, Lang::TypeScript),
        has_lang(enabled_langs, Lang::Rust),
        has_lang(enabled_langs, Lang::Python),
        has_lang(enabled_langs, Lang::C),
        has_lang(enabled_langs, Lang::CSharp),
        has_lang(enabled_langs, Lang::Zig),
    )
}

/// Generate README.md content
pub fn readme(
    name: &str,
    has_go: bool,
    has_ts: bool,
    has_rust: bool,
    has_python: bool,
    has_c: bool,
    has_csharp: bool,
    has_zig: bool,
) -> String {
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
    if has_python {
        content.push_str("        └── python/       # requirements.txt, generated bench code\n");
    }
    if has_c {
        content.push_str("        └── c/            # Makefile, bench.c, generated bench code\n");
    }
    if has_csharp {
        content.push_str(
            "        └── csharp/       # polybench.csproj, Program.cs, generated bench code\n",
        );
    }
    if has_zig {
        content.push_str(
            "        └── zig/          # build.zig, build.zig.zon, src/main.zig, generated bench code\n",
        );
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

    if has_python {
        content.push_str("### Python\n\n");
        content.push_str("```bash\n");
        content.push_str("poly-bench add --py \"numpy==1.0\"\n");
        content.push_str("```\n\n");
    }

    if has_c {
        content.push_str("### C\n\n");
        content.push_str("```bash\n");
        content.push_str("poly-bench add --c \"openssl@3.2\"\n");
        content.push_str("```\n\n");
    }

    if has_csharp {
        content.push_str("### C#\n\n");
        content.push_str("```bash\n");
        content.push_str("poly-bench add --cs \"Newtonsoft.Json@13.0.3\"\n");
        content.push_str("```\n\n");
    }

    if has_zig {
        content.push_str("### Zig\n\n");
        content.push_str("```bash\n");
        content.push_str("poly-bench add --zig \"package@0.1.0\"\n");
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
        let content = example_bench(true, true, false, false, false, false, false);
        assert!(content.contains("setup go"));
        assert!(content.contains("setup ts"));
        assert!(content.contains("fibGo(n20)"));
        assert!(content.contains("fibTs(n20)"));
        assert!(content.contains("baseline: \"go\""));
        assert!(content.contains("helpers {"));
        assert!(content.contains("charting.drawTable"));
        assert!(content.contains("charting.drawSpeedupChart"));
        assert!(content.contains("charting.drawSpeedupChart"));
    }

    #[test]
    fn test_example_bench_go_only() {
        let content = example_bench(true, false, false, false, false, false, false);
        assert!(content.contains("setup go"));
        assert!(!content.contains("setup ts"));
        assert!(!content.contains("baseline: \"go\"")); // No baseline with single language
        assert!(!content.contains("charting")); // No charting with single language
    }

    #[test]
    fn test_example_bench_all_languages() {
        let content = example_bench(true, true, true, true, false, false, false);
        assert!(content.contains("setup go"));
        assert!(content.contains("setup ts"));
        assert!(content.contains("setup rust"));
        assert!(content.contains("fibGo(n20)"));
        assert!(content.contains("fibTs(n20)"));
        assert!(content.contains("fib_rust(&n20)"));
        assert!(content.contains("fibGo(n70)"));
        assert!(content.contains("fibTs(n70)"));
        assert!(content.contains("fib_rust(&n70)"));
        assert!(content.contains("baseline: \"go\""));
        assert!(content.contains("use std::charting"));
    }

    #[test]
    fn test_example_bench_rust_only() {
        let content = example_bench(false, false, true, false, false, false, false);
        assert!(!content.contains("setup go"));
        assert!(!content.contains("setup ts"));
        assert!(content.contains("setup rust"));
        assert!(content.contains("fib_rust(&n20)"));
        assert!(!content.contains("baseline: \"go\"")); // No baseline with single language
    }

    #[test]
    fn test_example_bench_no_external_deps() {
        // Verify no external dependencies are required
        let content = example_bench(true, true, true, true, false, false, false);
        assert!(!content.contains("sha2"));
        assert!(!content.contains("crypto/sha256"));
        assert!(!content.contains("node:crypto"));
        assert!(!content.contains("n10"));
        assert!(content.contains("targetTime: 2000ms"));
        assert!(content.contains("mode: \"auto\""));
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
