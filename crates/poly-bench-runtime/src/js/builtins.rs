//! Built-in JavaScript functions for the benchmark runtime

/// JavaScript harness code that gets injected into the V8 context
pub const BENCHMARK_HARNESS: &str = r#"
// Benchmark harness for poly-bench
(function(globalThis) {
    'use strict';

    // High-resolution timing - use best available timer
    const now = (() => {
        // Node.js: Use process.hrtime.bigint() for nanosecond precision
        if (typeof process !== 'undefined' && process.hrtime && process.hrtime.bigint) {
            return () => Number(process.hrtime.bigint());
        }
        // Deno/Browser: performance.now() in ms, convert to ns
        if (typeof performance !== 'undefined') {
            return () => performance.now() * 1e6;
        }
        // Last resort: Date.now() in ms
        return () => Date.now() * 1e6;
    })();

    // Global sink to prevent dead code elimination
    globalThis.__polybench_sink = undefined;

    // Run a benchmark function (fixed iterations with sink pattern)
    function runBenchmark(fn, iterations, warmup, useSink = true) {
        const samples = new Array(iterations);
        
        // Warmup phase
        for (let i = 0; i < warmup; i++) {
            if (useSink) {
                globalThis.__polybench_sink = fn();
            } else {
                fn();
            }
        }
        
        // Timed phase
        let totalNanos = 0;
        for (let i = 0; i < iterations; i++) {
            const start = now();
            if (useSink) {
                globalThis.__polybench_sink = fn();
            } else {
                fn();
            }
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

    // Run a benchmark with auto-calibration
    function runBenchmarkAuto(fn, targetTimeMs, minIters, maxIters, useSink = true) {
        const targetNanos = targetTimeMs * 1e6;
        
        // Calibration phase: determine optimal iteration count
        let iterations = 1;
        while (iterations < maxIters) {
            const start = now();
            for (let i = 0; i < iterations; i++) {
                if (useSink) {
                    globalThis.__polybench_sink = fn();
                } else {
                    fn();
                }
            }
            const elapsed = now() - start;
            
            if (elapsed >= targetNanos) {
                break;
            }
            
            if (elapsed > 0) {
                // Scale up to reach target time (with 20% buffer)
                let newIters = Math.floor(iterations * (targetNanos / elapsed) * 1.2);
                if (newIters <= iterations) {
                    newIters = iterations * 2;
                }
                iterations = newIters;
            } else {
                iterations *= 10;
            }
            
            if (iterations > maxIters) {
                iterations = maxIters;
            }
        }
        
        if (iterations < minIters) {
            iterations = minIters;
        }
        
        // Warmup phase (10% of calibrated iterations, minimum 10)
        const warmup = Math.max(Math.floor(iterations / 10), 10);
        for (let i = 0; i < warmup; i++) {
            if (useSink) {
                globalThis.__polybench_sink = fn();
            } else {
                fn();
            }
        }
        
        // Timed phase
        const samples = new Array(iterations);
        let totalNanos = 0;
        for (let i = 0; i < iterations; i++) {
            const start = now();
            if (useSink) {
                globalThis.__polybench_sink = fn();
            } else {
                fn();
            }
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
    async function runBenchmarkAsync(fn, iterations, warmup, useSink = true) {
        const samples = new Array(iterations);
        
        // Warmup phase
        for (let i = 0; i < warmup; i++) {
            if (useSink) {
                globalThis.__polybench_sink = await fn();
            } else {
                await fn();
            }
        }
        
        // Timed phase
        let totalNanos = 0;
        for (let i = 0; i < iterations; i++) {
            const start = now();
            if (useSink) {
                globalThis.__polybench_sink = await fn();
            } else {
                await fn();
            }
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

    // Run an async benchmark with auto-calibration
    async function runBenchmarkAutoAsync(fn, targetTimeMs, minIters, maxIters, useSink = true) {
        const targetNanos = targetTimeMs * 1e6;
        
        // Calibration phase
        let iterations = 1;
        while (iterations < maxIters) {
            const start = now();
            for (let i = 0; i < iterations; i++) {
                if (useSink) {
                    globalThis.__polybench_sink = await fn();
                } else {
                    await fn();
                }
            }
            const elapsed = now() - start;
            
            if (elapsed >= targetNanos) {
                break;
            }
            
            if (elapsed > 0) {
                let newIters = Math.floor(iterations * (targetNanos / elapsed) * 1.2);
                if (newIters <= iterations) {
                    newIters = iterations * 2;
                }
                iterations = newIters;
            } else {
                iterations *= 10;
            }
            
            if (iterations > maxIters) {
                iterations = maxIters;
            }
        }
        
        if (iterations < minIters) {
            iterations = minIters;
        }
        
        // Warmup phase
        const warmup = Math.max(Math.floor(iterations / 10), 10);
        for (let i = 0; i < warmup; i++) {
            if (useSink) {
                globalThis.__polybench_sink = await fn();
            } else {
                await fn();
            }
        }
        
        // Timed phase
        const samples = new Array(iterations);
        let totalNanos = 0;
        for (let i = 0; i < iterations; i++) {
            const start = now();
            if (useSink) {
                globalThis.__polybench_sink = await fn();
            } else {
                await fn();
            }
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
    function runBenchmarkWithHook(fn, eachHook, iterations, warmup, useSink = true) {
        const samples = new Array(iterations);
        
        // Warmup phase with hook
        for (let i = 0; i < warmup; i++) {
            eachHook();
            if (useSink) {
                globalThis.__polybench_sink = fn();
            } else {
                fn();
            }
        }
        
        // Timed phase with hook (hook runs outside timing)
        let totalNanos = 0;
        for (let i = 0; i < iterations; i++) {
            eachHook();
            const start = now();
            if (useSink) {
                globalThis.__polybench_sink = fn();
            } else {
                fn();
            }
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
    async function runBenchmarkWithHookAsync(fn, eachHook, iterations, warmup, useSink = true) {
        const samples = new Array(iterations);
        
        // Warmup phase with hook
        for (let i = 0; i < warmup; i++) {
            await eachHook();
            if (useSink) {
                globalThis.__polybench_sink = await fn();
            } else {
                await fn();
            }
        }
        
        // Timed phase with hook (hook runs outside timing)
        let totalNanos = 0;
        for (let i = 0; i < iterations; i++) {
            await eachHook();
            const start = now();
            if (useSink) {
                globalThis.__polybench_sink = await fn();
            } else {
                await fn();
            }
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
        runBenchmarkAuto: runBenchmarkAuto,
        runBenchmarkAsync: runBenchmarkAsync,
        runBenchmarkAutoAsync: runBenchmarkAutoAsync,
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
