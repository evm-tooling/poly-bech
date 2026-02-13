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

    // Memory profiling helper (Node.js only)
    const getMemoryUsage = () => {
        if (typeof process !== 'undefined' && process.memoryUsage) {
            return process.memoryUsage().heapUsed;
        }
        return 0;
    };

    // Run a benchmark function (fixed iterations with sink pattern)
    function runBenchmark(fn, iterations, warmup, useSink = true, trackMemory = false) {
        const samples = new Array(iterations);
        
        // Warmup phase
        for (let i = 0; i < warmup; i++) {
            if (useSink) {
                globalThis.__polybench_sink = fn();
            } else {
                fn();
            }
        }
        
        // Memory tracking before
        const memBefore = trackMemory ? getMemoryUsage() : 0;
        
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
        
        // Memory tracking after
        const memAfter = trackMemory ? getMemoryUsage() : 0;
        const bytesPerOp = trackMemory ? Math.max(0, (memAfter - memBefore) / iterations) : undefined;
        
        const nanosPerOp = totalNanos / iterations;
        const opsPerSec = 1e9 / nanosPerOp;
        
        return {
            iterations: iterations,
            totalNanos: totalNanos,
            nanosPerOp: nanosPerOp,
            opsPerSec: opsPerSec,
            bytesPerOp: bytesPerOp,
            samples: samples,
        };
    }

    // Run a benchmark with auto-calibration (time-based, like Go's testing.B)
    // Total benchmark time is approximately targetTimeMs
    function runBenchmarkAuto(fn, targetTimeMs, useSink = true, trackMemory = false) {
        const targetNanos = targetTimeMs * 1e6;
        
        // Brief warmup (fixed 100 iterations to warm JIT)
        for (let i = 0; i < 100; i++) {
            if (useSink) {
                globalThis.__polybench_sink = fn();
            } else {
                fn();
            }
        }
        
        // Memory tracking before
        const memBefore = trackMemory ? getMemoryUsage() : 0;
        
        // Adaptive measurement phase (like Go's testing.B)
        // Run batches, scale up N, stop when total elapsed >= targetTime
        let iterations = 1;
        let totalIterations = 0;
        let totalNanos = 0;
        
        while (totalNanos < targetNanos) {
            // Run batch without per-iteration timing (fast)
            const batchStart = now();
            for (let i = 0; i < iterations; i++) {
                if (useSink) {
                    globalThis.__polybench_sink = fn();
                } else {
                    fn();
                }
            }
            const batchElapsed = now() - batchStart;
            
            totalIterations += iterations;
            totalNanos += batchElapsed;
            
            if (totalNanos >= targetNanos) {
                break;
            }
            
            // Scale up for next batch (matching Go's conservative approach)
            if (batchElapsed > 0) {
                const remainingNanos = targetNanos - totalNanos;
                const predicted = Math.floor(iterations * (remainingNanos / batchElapsed));
                
                let newIters;
                if (remainingNanos < batchElapsed) {
                    // Close to target - use predicted or less
                    newIters = Math.max(1, predicted);
                } else if (remainingNanos < targetNanos / 5) {
                    // Within 20% of target - scale down slightly to avoid overshoot
                    newIters = Math.max(1, Math.floor(predicted * 0.9));
                } else {
                    // Far from target - scale up conservatively
                    newIters = Math.floor(predicted * 1.1);
                    if (newIters <= iterations) {
                        newIters = iterations * 2;
                    }
                    if (newIters > iterations * 10) {
                        newIters = iterations * 10;
                    }
                }
                iterations = Math.max(1, newIters);
            } else {
                iterations *= 10;
            }
        }
        
        // Memory tracking after
        const memAfter = trackMemory ? getMemoryUsage() : 0;
        const bytesPerOp = trackMemory ? Math.max(0, (memAfter - memBefore) / totalIterations) : undefined;
        
        const nanosPerOp = totalNanos / totalIterations;
        const opsPerSec = 1e9 / nanosPerOp;
        
        // Generate representative samples for statistics (sample small subset)
        // Run a few more iterations with per-iteration timing for variance
        const sampleCount = Math.min(1000, totalIterations);
        const samples = new Array(sampleCount);
        for (let i = 0; i < sampleCount; i++) {
            const start = now();
            if (useSink) {
                globalThis.__polybench_sink = fn();
            } else {
                fn();
            }
            samples[i] = now() - start;
        }
        
        return {
            iterations: totalIterations,
            totalNanos: totalNanos,
            nanosPerOp: nanosPerOp,
            opsPerSec: opsPerSec,
            bytesPerOp: bytesPerOp,
            samples: samples,
        };
    }

    // Run an async benchmark function
    async function runBenchmarkAsync(fn, iterations, warmup, useSink = true, trackMemory = false) {
        const samples = new Array(iterations);
        
        // Warmup phase
        for (let i = 0; i < warmup; i++) {
            if (useSink) {
                globalThis.__polybench_sink = await fn();
            } else {
                await fn();
            }
        }
        
        // Memory tracking before
        const memBefore = trackMemory ? getMemoryUsage() : 0;
        
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
        
        // Memory tracking after
        const memAfter = trackMemory ? getMemoryUsage() : 0;
        const bytesPerOp = trackMemory ? Math.max(0, (memAfter - memBefore) / iterations) : undefined;
        
        const nanosPerOp = totalNanos / iterations;
        const opsPerSec = 1e9 / nanosPerOp;
        
        return {
            iterations: iterations,
            totalNanos: totalNanos,
            nanosPerOp: nanosPerOp,
            opsPerSec: opsPerSec,
            bytesPerOp: bytesPerOp,
            samples: samples,
        };
    }

    // Run an async benchmark with auto-calibration (time-based)
    async function runBenchmarkAutoAsync(fn, targetTimeMs, useSink = true, trackMemory = false) {
        const targetNanos = targetTimeMs * 1e6;
        
        // Brief warmup (fixed 100 iterations)
        for (let i = 0; i < 100; i++) {
            if (useSink) {
                globalThis.__polybench_sink = await fn();
            } else {
                await fn();
            }
        }
        
        // Memory tracking before
        const memBefore = trackMemory ? getMemoryUsage() : 0;
        
        // Adaptive measurement phase - no per-iteration timing
        let batchSize = 1;
        let totalIterations = 0;
        let totalNanos = 0;
        
        while (totalNanos < targetNanos) {
            const batchStart = now();
            for (let i = 0; i < batchSize; i++) {
                if (useSink) {
                    globalThis.__polybench_sink = await fn();
                } else {
                    await fn();
                }
            }
            const batchElapsed = now() - batchStart;
            
            totalIterations += batchSize;
            totalNanos += batchElapsed;
            
            if (totalNanos >= targetNanos) {
                break;
            }
            
            // Scale up for next batch (matching Go's conservative approach)
            if (batchElapsed > 0) {
                const remainingNanos = targetNanos - totalNanos;
                const predicted = Math.floor(batchSize * (remainingNanos / batchElapsed));
                
                let newSize;
                if (remainingNanos < batchElapsed) {
                    // Close to target - use predicted or less
                    newSize = Math.max(1, predicted);
                } else if (remainingNanos < targetNanos / 5) {
                    // Within 20% of target - scale down slightly to avoid overshoot
                    newSize = Math.max(1, Math.floor(predicted * 0.9));
                } else {
                    // Far from target - scale up conservatively
                    newSize = Math.floor(predicted * 1.1);
                    if (newSize <= batchSize) {
                        newSize = batchSize * 2;
                    }
                    if (newSize > batchSize * 10) {
                        newSize = batchSize * 10;
                    }
                }
                batchSize = Math.max(1, newSize);
            } else {
                batchSize *= 10;
            }
        }
        
        // Memory tracking after
        const memAfter = trackMemory ? getMemoryUsage() : 0;
        const bytesPerOp = trackMemory ? Math.max(0, (memAfter - memBefore) / totalIterations) : undefined;
        
        const nanosPerOp = totalNanos / totalIterations;
        const opsPerSec = 1e9 / nanosPerOp;
        
        // Collect samples for statistical analysis
        const sampleCount = Math.min(1000, totalIterations);
        const samples = new Array(sampleCount);
        for (let i = 0; i < sampleCount; i++) {
            const start = now();
            if (useSink) {
                globalThis.__polybench_sink = await fn();
            } else {
                await fn();
            }
            samples[i] = now() - start;
        }
        
        return {
            iterations: totalIterations,
            totalNanos: totalNanos,
            nanosPerOp: nanosPerOp,
            opsPerSec: opsPerSec,
            bytesPerOp: bytesPerOp,
            samples: samples,
        };
    }

    // Run a benchmark function with an each-iteration hook
    function runBenchmarkWithHook(fn, eachHook, iterations, warmup, useSink = true, trackMemory = false) {
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
        
        // Memory tracking before
        const memBefore = trackMemory ? getMemoryUsage() : 0;
        
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
        
        // Memory tracking after
        const memAfter = trackMemory ? getMemoryUsage() : 0;
        const bytesPerOp = trackMemory ? Math.max(0, (memAfter - memBefore) / iterations) : undefined;
        
        const nanosPerOp = totalNanos / iterations;
        const opsPerSec = 1e9 / nanosPerOp;
        
        return {
            iterations: iterations,
            totalNanos: totalNanos,
            nanosPerOp: nanosPerOp,
            opsPerSec: opsPerSec,
            bytesPerOp: bytesPerOp,
            samples: samples,
        };
    }

    // Run an async benchmark function with an each-iteration hook
    async function runBenchmarkWithHookAsync(fn, eachHook, iterations, warmup, useSink = true, trackMemory = false) {
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
        
        // Memory tracking before
        const memBefore = trackMemory ? getMemoryUsage() : 0;
        
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
        
        // Memory tracking after
        const memAfter = trackMemory ? getMemoryUsage() : 0;
        const bytesPerOp = trackMemory ? Math.max(0, (memAfter - memBefore) / iterations) : undefined;
        
        const nanosPerOp = totalNanos / iterations;
        const opsPerSec = 1e9 / nanosPerOp;
        
        return {
            iterations: iterations,
            totalNanos: totalNanos,
            nanosPerOp: nanosPerOp,
            opsPerSec: opsPerSec,
            bytesPerOp: bytesPerOp,
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
