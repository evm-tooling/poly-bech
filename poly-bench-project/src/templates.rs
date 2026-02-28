//! Template strings for generated project files
use miette::Result;
use poly_bench_dsl::Lang;
use std::path::Path;

fn has_lang(enabled_langs: &[Lang], lang: Lang) -> bool {
    enabled_langs.contains(&lang)
}

/// Embedded sort fixture hex files for the bubble benchmark
mod bubble_fixtures {
    macro_rules! include_sort_hex {
        ($name:ident, $file:literal) => {
            pub static $name: &[u8] = include_bytes!($file);
        };
    }
    include_sort_hex!(SORT_100, "../fixtures/sort/sort_100.hex");
    include_sort_hex!(SORT_200, "../fixtures/sort/sort_200.hex");
    include_sort_hex!(SORT_300, "../fixtures/sort/sort_300.hex");
    include_sort_hex!(SORT_400, "../fixtures/sort/sort_400.hex");
    include_sort_hex!(SORT_500, "../fixtures/sort/sort_500.hex");
    include_sort_hex!(SORT_600, "../fixtures/sort/sort_600.hex");
    include_sort_hex!(SORT_700, "../fixtures/sort/sort_700.hex");
    include_sort_hex!(SORT_800, "../fixtures/sort/sort_800.hex");
    include_sort_hex!(SORT_900, "../fixtures/sort/sort_900.hex");
    include_sort_hex!(SORT_1000, "../fixtures/sort/sort_1000.hex");
}

/// Write bubble sort fixture hex files to benchmarks_dir/fixtures/sort/
pub fn write_bubble_fixtures(benchmarks_dir: &Path) -> Result<()> {
    let fixtures_dir = benchmarks_dir.join("fixtures").join("sort");
    std::fs::create_dir_all(&fixtures_dir)
        .map_err(|e| miette::miette!("Failed to create fixtures/sort: {}", e))?;
    let files = [
        ("sort_100.hex", bubble_fixtures::SORT_100),
        ("sort_200.hex", bubble_fixtures::SORT_200),
        ("sort_300.hex", bubble_fixtures::SORT_300),
        ("sort_400.hex", bubble_fixtures::SORT_400),
        ("sort_500.hex", bubble_fixtures::SORT_500),
        ("sort_600.hex", bubble_fixtures::SORT_600),
        ("sort_700.hex", bubble_fixtures::SORT_700),
        ("sort_800.hex", bubble_fixtures::SORT_800),
        ("sort_900.hex", bubble_fixtures::SORT_900),
        ("sort_1000.hex", bubble_fixtures::SORT_1000),
    ];
    for (name, data) in files {
        std::fs::write(fixtures_dir.join(name), data)
            .map_err(|e| miette::miette!("Failed to write {}: {}", name, e))?;
    }
    Ok(())
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

/// Generate the example.bench file content (bubble sort benchmark)
/// Uses bubble sort on int32 arrays with @file fixtures
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

    content.push_str("declare suite bubbleN performance timeBased sameDataset: true {\n");
    content.push_str("    description: \"O(n^2) bubble sort on int32 array\"\n");
    if lang_count > 1 && has_go {
        content.push_str("    baseline: \"go\"\n");
    } else if lang_count > 1 && has_rust {
        content.push_str("    baseline: \"rust\"\n");
    }
    content.push_str("    warmup: 100\n");
    content.push_str("    targetTime: 100ms\n");
    content.push_str("    fairness: \"strict\"\n");
    content.push_str("    cvThreshold: 5\n");
    content.push_str("    count: 1\n");
    content.push('\n');

    // Setup blocks
    if has_go {
        content.push_str("    setup go {\n");
        content.push_str("        import (\n");
        content.push_str("            \"encoding/binary\"\n");
        content.push_str("        )\n\n");
        content.push_str("        helpers {\n");
        content.push_str("            func bubbleGo(data []byte) []byte {\n");
        content.push_str("                n := len(data) / 4\n");
        content.push_str("                arr := make([]int32, n)\n");
        content.push_str("                for i := 0; i < n; i++ {\n");
        content.push_str(
            "                    arr[i] = int32(binary.BigEndian.Uint32(data[i*4 : (i+1)*4]))\n",
        );
        content.push_str("                }\n");
        content.push_str("                for i := 0; i < n; i++ {\n");
        content.push_str("                    for j := 0; j < n-1-i; j++ {\n");
        content.push_str("                        if arr[j] > arr[j+1] {\n");
        content.push_str("                            tmp := arr[j]\n");
        content.push_str("                            arr[j] = arr[j+1]\n");
        content.push_str("                            arr[j+1] = tmp\n");
        content.push_str("                        }\n");
        content.push_str("                    }\n");
        content.push_str("                }\n");
        content.push_str("                out := make([]byte, len(data))\n");
        content.push_str("                for i := 0; i < n; i++ {\n");
        content.push_str(
            "                    binary.BigEndian.PutUint32(out[i*4:(i+1)*4], uint32(arr[i]))\n",
        );
        content.push_str("                }\n");
        content.push_str("                return out\n");
        content.push_str("            }\n");
        content.push_str("        }\n");
        content.push_str("    }\n\n");
    }

    if has_ts {
        content.push_str("    setup ts {\n");
        content.push_str("        helpers {\n");
        content.push_str("            function bubbleTs(data: Uint8Array): Uint8Array {\n");
        content.push_str("                const n = data.length / 4\n");
        content.push_str("                const arr = new Array(n)\n");
        content.push_str("                for (let i = 0; i < n; i++) {\n");
        content.push_str("                    arr[i] = (data[i*4]<<24) | (data[i*4+1]<<16) | (data[i*4+2]<<8) | data[i*4+3]\n");
        content.push_str("                }\n");
        content.push_str("                for (let i = 0; i < n; i++) {\n");
        content.push_str("                    for (let j = 0; j < n - 1 - i; j++) {\n");
        content.push_str("                        if (arr[j] > arr[j+1]) {\n");
        content.push_str("                            const tmp = arr[j]\n");
        content.push_str("                            arr[j] = arr[j+1]\n");
        content.push_str("                            arr[j+1] = tmp\n");
        content.push_str("                        }\n");
        content.push_str("                    }\n");
        content.push_str("                }\n");
        content.push_str("                const out = new Uint8Array(data.length)\n");
        content.push_str("                const view = new DataView(out.buffer)\n");
        content.push_str("                for (let i = 0; i < n; i++) {\n");
        content.push_str("                    view.setInt32(i * 4, arr[i], false)\n");
        content.push_str("                }\n");
        content.push_str("                return out\n");
        content.push_str("            }\n");
        content.push_str("        }\n");
        content.push_str("    }\n\n");
    }

    if has_python {
        content.push_str("    setup python {\n");
        content.push_str("        import {\n");
        content.push_str("            import struct\n");
        content.push_str("        }\n\n");
        content.push_str("        helpers {\n");
        content.push_str("            def bubble_python(data):\n");
        content.push_str("                n = len(data) // 4\n");
        content.push_str("                fmt = \">\" + str(n) + \"i\"\n");
        content.push_str("                arr = list(struct.unpack(fmt, data))\n");
        content.push_str("                for i in range(n):\n");
        content.push_str("                    for j in range(n - 1 - i):\n");
        content.push_str("                        if arr[j] > arr[j + 1]:\n");
        content.push_str("                            tmp = arr[j]\n");
        content.push_str("                            arr[j] = arr[j + 1]\n");
        content.push_str("                            arr[j + 1] = tmp\n");
        content.push_str("                out = struct.pack(fmt, *arr)\n");
        content.push_str("                return out\n");
        content.push_str("        }\n");
        content.push_str("    }\n\n");
    }

    if has_rust {
        content.push_str("    setup rust {\n");
        content.push_str("        helpers {\n");
        content.push_str("            fn bubble_rust(data: &[u8]) -> Vec<u8> {\n");
        content.push_str("                let n = data.len() / 4;\n");
        content.push_str("                let mut arr: Vec<i32> = (0..n).map(|i| {\n");
        content.push_str("                    let j = i * 4;\n");
        content.push_str(
            "                    i32::from_be_bytes([data[j], data[j+1], data[j+2], data[j+3]])\n",
        );
        content.push_str("                }).collect();\n");
        content.push_str("                for i in 0..n {\n");
        content.push_str("                    for j in 0..n-1-i {\n");
        content.push_str("                        if arr[j] > arr[j+1] {\n");
        content.push_str("                            let tmp = arr[j];\n");
        content.push_str("                            arr[j] = arr[j+1];\n");
        content.push_str("                            arr[j+1] = tmp;\n");
        content.push_str("                        }\n");
        content.push_str("                    }\n");
        content.push_str("                }\n");
        content.push_str("                let mut out = vec![0u8; data.len()];\n");
        content.push_str("                for i in 0..n {\n");
        content.push_str(
            "                    out[i*4..(i+1)*4].copy_from_slice(&arr[i].to_be_bytes());\n",
        );
        content.push_str("                }\n");
        content.push_str("                out\n");
        content.push_str("            }\n");
        content.push_str("        }\n");
        content.push_str("    }\n\n");
    }

    if has_csharp {
        content.push_str("    setup csharp {\n");
        content.push_str("        import {\n");
        content.push_str("            using System;\n");
        content.push_str("            using System.Buffers.Binary;\n");
        content.push_str("        }\n\n");
        content.push_str("        helpers {\n");
        content.push_str("            static byte[] BubbleCsharp(byte[] data) {\n");
        content.push_str("                int n = data.Length / 4;\n");
        content.push_str("                int[] arr = new int[n];\n");
        content.push_str("                for (int i = 0; i < n; i++) {\n");
        content.push_str("                    int j = i * 4;\n");
        content.push_str("                    arr[i] = BinaryPrimitives.ReadInt32BigEndian(new ReadOnlySpan<byte>(data, j, 4));\n");
        content.push_str("                }\n");
        content.push_str("                for (int i = 0; i < n; i++) {\n");
        content.push_str("                    for (int j = 0; j < n - 1 - i; j++) {\n");
        content.push_str("                        if (arr[j] > arr[j + 1]) {\n");
        content.push_str("                            int tmp = arr[j];\n");
        content.push_str("                            arr[j] = arr[j + 1];\n");
        content.push_str("                            arr[j + 1] = tmp;\n");
        content.push_str("                        }\n");
        content.push_str("                    }\n");
        content.push_str("                }\n");
        content.push_str("                byte[] outBytes = new byte[data.Length];\n");
        content.push_str("                for (int i = 0; i < n; i++) {\n");
        content.push_str("                    int j = i * 4;\n");
        content.push_str("                    BinaryPrimitives.WriteInt32BigEndian(new Span<byte>(outBytes, j, 4), arr[i]);\n");
        content.push_str("                }\n");
        content.push_str("                return outBytes;\n");
        content.push_str("            }\n");
        content.push_str("        }\n");
        content.push_str("    }\n\n");
    }

    if has_zig {
        content.push_str("    setup zig {\n");
        content.push_str("        helpers {\n");
        content.push_str("            fn bubbleZig(data: []const u8) []u8 {\n");
        content.push_str("                const n = data.len / 4;\n");
        content.push_str("                const allocator = std.heap.page_allocator;\n");
        content.push_str(
            "                const arr = allocator.alloc(i32, n) catch @panic(\"alloc failed\");\n",
        );
        content.push_str("                defer allocator.free(arr);\n");
        content.push_str("                for (0..n) |i| {\n");
        content.push_str("                    const offset = i * 4;\n");
        content.push_str("                    arr[i] = std.mem.readInt(i32, @ptrCast(data[offset..].ptr), .big);\n");
        content.push_str("                }\n");
        content.push_str("                for (0..n) |i| {\n");
        content.push_str("                    for (0..n - 1 - i) |j| {\n");
        content.push_str("                        if (arr[j] > arr[j + 1]) {\n");
        content.push_str("                            const tmp = arr[j];\n");
        content.push_str("                            arr[j] = arr[j + 1];\n");
        content.push_str("                            arr[j + 1] = tmp;\n");
        content.push_str("                        }\n");
        content.push_str("                    }\n");
        content.push_str("                }\n");
        content.push_str("                const out = allocator.alloc(u8, data.len) catch @panic(\"alloc failed\");\n");
        content.push_str("                errdefer allocator.free(out);\n");
        content.push_str("                for (0..n) |i| {\n");
        content.push_str("                    const offset = i * 4;\n");
        content.push_str("                    std.mem.writeInt(i32, @ptrCast(out[offset..].ptr), arr[i], .big);\n");
        content.push_str("                }\n");
        content.push_str("                return out;\n");
        content.push_str("            }\n\n");
        content.push_str("            fn bubbleZigAndFree(data: []const u8) void {\n");
        content.push_str("                const allocator = std.heap.page_allocator;\n");
        content.push_str("                const result = bubbleZig(data);\n");
        content.push_str("                allocator.free(result);\n");
        content.push_str("            }\n");
        content.push_str("        }\n");
        content.push_str("    }\n\n");
    }

    if has_c {
        content.push_str("    setup c {\n");
        content.push_str("        helpers {\n");
        content.push_str("            static int32_t read_i32_be(const unsigned char* p) {\n");
        content.push_str("                return (int32_t)(\n");
        content.push_str("                    ((uint32_t)p[0] << 24) |\n");
        content.push_str("                    ((uint32_t)p[1] << 16) |\n");
        content.push_str("                    ((uint32_t)p[2] << 8) |\n");
        content.push_str("                    (uint32_t)p[3]\n");
        content.push_str("                );\n");
        content.push_str("            }\n\n");
        content.push_str("            static void write_i32_be(unsigned char* p, int32_t v) {\n");
        content.push_str("                uint32_t u = (uint32_t)v;\n");
        content.push_str("                p[0] = (unsigned char)((u >> 24) & 0xFF);\n");
        content.push_str("                p[1] = (unsigned char)((u >> 16) & 0xFF);\n");
        content.push_str("                p[2] = (unsigned char)((u >> 8) & 0xFF);\n");
        content.push_str("                p[3] = (unsigned char)(u & 0xFF);\n");
        content.push_str("            }\n\n");
        content.push_str("            static void bubble_c(unsigned char* data, size_t len) {\n");
        content.push_str("                size_t n = len / 4;\n");
        content.push_str("                int32_t* arr = (int32_t*)malloc(n * sizeof(int32_t));\n");
        content.push_str("                if (!arr) return;\n");
        content.push_str("                for (size_t i = 0; i < n; i++) {\n");
        content.push_str("                    arr[i] = read_i32_be(data + i * 4);\n");
        content.push_str("                }\n");
        content.push_str("                for (size_t i = 0; i < n; i++) {\n");
        content.push_str("                    for (size_t j = 0; j < n - 1 - i; j++) {\n");
        content.push_str("                        if (arr[j] > arr[j + 1]) {\n");
        content.push_str("                            int32_t tmp = arr[j];\n");
        content.push_str("                            arr[j] = arr[j + 1];\n");
        content.push_str("                            arr[j + 1] = tmp;\n");
        content.push_str("                        }\n");
        content.push_str("                    }\n");
        content.push_str("                }\n");
        content.push_str("                unsigned char* out = (unsigned char*)malloc(len);\n");
        content.push_str("                if (!out) { free(arr); return; }\n");
        content.push_str("                for (size_t i = 0; i < n; i++) {\n");
        content.push_str("                    write_i32_be(out + i * 4, arr[i]);\n");
        content.push_str("                }\n");
        content.push_str("                memcpy(data, out, len);\n");
        content.push_str("                free(arr);\n");
        content.push_str("                free(out);\n");
        content.push_str("            }\n");
        content.push_str("        }\n");
        content.push_str("    }\n\n");
    }

    // Fixtures
    for (name, file) in [
        ("s100", "sort_100.hex"),
        ("s200", "sort_200.hex"),
        ("s300", "sort_300.hex"),
        ("s400", "sort_400.hex"),
        ("s500", "sort_500.hex"),
        ("s600", "sort_600.hex"),
        ("s700", "sort_700.hex"),
        ("s800", "sort_800.hex"),
        ("s900", "sort_900.hex"),
        ("s1000", "sort_1000.hex"),
    ] {
        content.push_str(&format!("    fixture {} {{\n", name));
        content.push_str(&format!("        hex: @file(\"fixtures/sort/{}\")\n", file));
        content.push_str("    }\n\n");
    }

    // Bench blocks
    fn bench_block(
        content: &mut String,
        n: &str,
        s: &str,
        has_go: bool,
        has_ts: bool,
        has_rust: bool,
        has_python: bool,
        has_c: bool,
        has_csharp: bool,
        has_zig: bool,
    ) {
        content.push_str(&format!("    bench {} {{\n", n));
        if has_go {
            content.push_str(&format!("        go: bubbleGo({})\n", s));
        }
        if has_ts {
            content.push_str(&format!("        ts: bubbleTs({})\n", s));
        }
        if has_rust {
            content.push_str(&format!("        rust: bubble_rust(&{})\n", s));
        }
        if has_python {
            content.push_str(&format!("        python: bubble_python({})\n", s));
        }
        if has_csharp {
            content.push_str(&format!("        csharp: BubbleCsharp({});\n", s));
        }
        if has_zig {
            content.push_str(&format!("        zig: bubbleZigAndFree(&{});\n", s));
        }
        if has_c {
            content.push_str(&format!(
                "        c: {{\n            unsigned char __buf[sizeof({})];\n            memcpy(__buf, {}, sizeof(__buf));\n            bubble_c(__buf, sizeof(__buf));\n        }}\n",
                s, s
            ));
        }
        content.push_str("    }\n\n");
    }

    for (n, s) in [
        ("n100", "s100"),
        ("n200", "s200"),
        ("n300", "s300"),
        ("n400", "s400"),
        ("n500", "s500"),
        ("n600", "s600"),
        ("n700", "s700"),
        ("n800", "s800"),
        ("n900", "s900"),
        ("n1000", "s1000"),
    ] {
        bench_block(
            &mut content,
            n,
            s,
            has_go,
            has_ts,
            has_rust,
            has_python,
            has_c,
            has_csharp,
            has_zig,
        );
    }

    // After block
    if lang_count > 1 {
        content.push_str("    after {\n");
        content.push_str("        charting.drawLineChart(\n");
        content.push_str("            title: \"Bubble Sort - O(n²)\",\n");
        content.push_str("            description: \"Classic bubble sort - quadratic trend with regression + error overlays\",\n");
        content.push_str("            output: \"bubble-line-linear.svg\",\n");
        content.push_str("            yScale: \"linear\",\n");
        content.push_str("            showStdDev: true,\n");
        content.push_str("            showErrorBars: true,\n");
        content.push_str("            showRegression: true,\n");
        content.push_str("            regressionModel: \"auto\",\n");
        content.push_str("        )\n");
        content.push_str("        charting.drawSpeedupChart(\n");
        content.push_str("            title: \"Bubble Sort - O(n²)\",\n");
        content.push_str("            description: \"Relative performance vs baseline\",\n");
        content.push_str("            output: \"bubble-bar-speed.svg\",\n");
        content.push_str("        )\n");
        content.push_str("        charting.drawTable(\n");
        content.push_str("            title: \"Sort Performance Comparison\",\n");
        content.push_str(
            "            description: \"Vertical grouped bars - Go vs TypeScript vs Rust\",\n",
        );
        content.push_str("            output: \"bubble-table-curr.svg\",\n");
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
        content.push_str("poly-bench add --zig \"git+https://github.com/Hejsil/zig-bench#main\"\n");
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
        assert!(content.contains("bubbleGo(s100)"));
        assert!(content.contains("bubbleTs(s100)"));
        assert!(content.contains("baseline: \"go\""));
        assert!(content.contains("helpers {"));
        assert!(content.contains("charting.drawTable"));
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
        assert!(content.contains("bubbleGo(s100)"));
        assert!(content.contains("bubbleTs(s100)"));
        assert!(content.contains("bubble_rust(&s100)"));
        assert!(content.contains("bubbleGo(s700)"));
        assert!(content.contains("bubbleTs(s700)"));
        assert!(content.contains("bubble_rust(&s700)"));
        assert!(content.contains("baseline: \"go\""));
        assert!(content.contains("use std::charting"));
    }

    #[test]
    fn test_example_bench_rust_only() {
        let content = example_bench(false, false, true, false, false, false, false);
        assert!(!content.contains("setup go"));
        assert!(!content.contains("setup ts"));
        assert!(content.contains("setup rust"));
        assert!(content.contains("bubble_rust(&s100)"));
        assert!(!content.contains("baseline: \"go\"")); // No baseline with single language
    }

    #[test]
    fn test_example_bench_no_external_deps() {
        // Verify no external dependencies are required
        let content = example_bench(true, true, true, true, false, false, false);
        assert!(!content.contains("sha2"));
        assert!(!content.contains("crypto/sha256"));
        assert!(!content.contains("node:crypto"));
        assert!(!content.contains("fibGo")); // Bubble uses bubbleGo
        assert!(content.contains("targetTime: 100ms"));
        assert!(content.contains("declare suite bubbleN"));
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
