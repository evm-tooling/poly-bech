//! Built-in JavaScript functions for the benchmark runtime

/// JavaScript harness code that gets injected into the V8 context
pub const BENCHMARK_HARNESS: &str = r#"
// Benchmark harness for poly-bench
(function(globalThis) {
    'use strict';

    // High-resolution timing
    const now = typeof performance !== 'undefined' 
        ? () => performance.now() * 1e6  // Convert ms to ns
        : () => Date.now() * 1e6;

    // Run a benchmark function
    function runBenchmark(fn, iterations, warmup) {
        const samples = new Array(iterations);
        
        // Warmup phase
        for (let i = 0; i < warmup; i++) {
            fn();
        }
        
        // Timed phase
        let totalNanos = 0;
        for (let i = 0; i < iterations; i++) {
            const start = now();
            fn();
            const elapsed = now() - start;
            samples[i] = elapsed;
            totalNanos += elapsed;
        }
        
        const nanosPerOp = totalNanos / iterations;
        const opsPerSec = 1e9 / nanosPerOp;
        
        return {
            iterations: iterations,
            totalNanos: totalNanos,
            nanosPerOp: nanosPerOp,
            opsPerSec: opsPerSec,
            samples: samples,
        };
    }

    // Run an async benchmark function
    async function runBenchmarkAsync(fn, iterations, warmup) {
        const samples = new Array(iterations);
        
        // Warmup phase
        for (let i = 0; i < warmup; i++) {
            await fn();
        }
        
        // Timed phase
        let totalNanos = 0;
        for (let i = 0; i < iterations; i++) {
            const start = now();
            await fn();
            const elapsed = now() - start;
            samples[i] = elapsed;
            totalNanos += elapsed;
        }
        
        const nanosPerOp = totalNanos / iterations;
        const opsPerSec = 1e9 / nanosPerOp;
        
        return {
            iterations: iterations,
            totalNanos: totalNanos,
            nanosPerOp: nanosPerOp,
            opsPerSec: opsPerSec,
            samples: samples,
        };
    }

    // Run a benchmark function with an each-iteration hook
    function runBenchmarkWithHook(fn, eachHook, iterations, warmup) {
        const samples = new Array(iterations);
        
        // Warmup phase with hook
        for (let i = 0; i < warmup; i++) {
            eachHook();
            fn();
        }
        
        // Timed phase with hook (hook runs outside timing)
        let totalNanos = 0;
        for (let i = 0; i < iterations; i++) {
            eachHook();
            const start = now();
            fn();
            const elapsed = now() - start;
            samples[i] = elapsed;
            totalNanos += elapsed;
        }
        
        const nanosPerOp = totalNanos / iterations;
        const opsPerSec = 1e9 / nanosPerOp;
        
        return {
            iterations: iterations,
            totalNanos: totalNanos,
            nanosPerOp: nanosPerOp,
            opsPerSec: opsPerSec,
            samples: samples,
        };
    }

    // Run an async benchmark function with an each-iteration hook
    async function runBenchmarkWithHookAsync(fn, eachHook, iterations, warmup) {
        const samples = new Array(iterations);
        
        // Warmup phase with hook
        for (let i = 0; i < warmup; i++) {
            await eachHook();
            await fn();
        }
        
        // Timed phase with hook (hook runs outside timing)
        let totalNanos = 0;
        for (let i = 0; i < iterations; i++) {
            await eachHook();
            const start = now();
            await fn();
            const elapsed = now() - start;
            samples[i] = elapsed;
            totalNanos += elapsed;
        }
        
        const nanosPerOp = totalNanos / iterations;
        const opsPerSec = 1e9 / nanosPerOp;
        
        return {
            iterations: iterations,
            totalNanos: totalNanos,
            nanosPerOp: nanosPerOp,
            opsPerSec: opsPerSec,
            samples: samples,
        };
    }

    // Fixture helpers
    function hexToBytes(hex) {
        hex = hex.replace(/^0x/, '');
        const bytes = new Uint8Array(hex.length / 2);
        for (let i = 0; i < bytes.length; i++) {
            bytes[i] = parseInt(hex.substr(i * 2, 2), 16);
        }
        return bytes;
    }

    function bytesToHex(bytes) {
        return '0x' + Array.from(bytes)
            .map(b => b.toString(16).padStart(2, '0'))
            .join('');
    }

    // Export to global
    globalThis.__polybench = {
        now: now,
        runBenchmark: runBenchmark,
        runBenchmarkAsync: runBenchmarkAsync,
        runBenchmarkWithHook: runBenchmarkWithHook,
        runBenchmarkWithHookAsync: runBenchmarkWithHookAsync,
        hexToBytes: hexToBytes,
        bytesToHex: bytesToHex,
    };
})(globalThis);
"#;

/// Generate fixture injection code
pub fn generate_fixture_code(name: &str, hex_data: &str) -> String {
    format!(
        r#"const {name} = __polybench.hexToBytes("{hex_data}");
const {name}_hex = "{hex_prefix}{hex_data}";
"#,
        name = name,
        hex_data = hex_data.trim_start_matches("0x"),
        hex_prefix = if hex_data.starts_with("0x") { "" } else { "0x" }
    )
}

/// Generate benchmark wrapper code
pub fn generate_benchmark_code(name: &str, impl_code: &str, iterations: u64, warmup: u64) -> String {
    format!(
        r#"
(function() {{
    const result = __polybench.runBenchmark(function() {{
        {impl_code}
    }}, {iterations}, {warmup});
    return JSON.stringify(result);
}})();
"#,
        impl_code = impl_code,
        iterations = iterations,
        warmup = warmup
    )
}
