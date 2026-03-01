//! Built-in JavaScript functions for the benchmark runtime

/// Performance harness: no memory instrumentation. Zero overhead for performance benchmarks.
pub const BENCH_HARNESS_PERF: &str = r#"
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

    // Convert benchmark return values into JSON-safe data for reporting.
    function normalizeRawResult(value) {
        if (value === undefined) return null;
        if (typeof value === 'bigint') return value.toString();
        try {
            return JSON.parse(JSON.stringify(value, (_, v) => typeof v === 'bigint' ? v.toString() : v));
        } catch (_) {
            return String(value);
        }
    }

    function normalizeErrorResult(error) {
        if (error === undefined || error === null) return 'Unknown async error';
        if (error && typeof error === 'object') {
            if (typeof error.stack === 'string' && error.stack.length > 0) return error.stack;
            if (typeof error.message === 'string' && error.message.length > 0) return error.message;
        }
        try {
            return String(error);
        } catch (_) {
            return 'Unserializable async error';
        }
    }

    // Warmup helper: warmupTimeMs takes precedence over warmupIterations. Both 0 = skip.
    // Returns warmup duration in nanoseconds.
    function doWarmupSync(fn, warmupIterations, warmupTimeMs, useSink) {
        let warmupNanos = 0;
        if (warmupTimeMs > 0) {
            const start = now();
            const limitNs = warmupTimeMs * 1e6;
            while ((now() - start) < limitNs) {
                if (useSink) { globalThis.__polybench_sink = fn(); } else { fn(); }
            }
            warmupNanos = now() - start;
        } else if (warmupIterations > 0) {
            const start = now();
            for (let i = 0; i < warmupIterations; i++) {
                if (useSink) { globalThis.__polybench_sink = fn(); } else { fn(); }
            }
            warmupNanos = now() - start;
        }
        return warmupNanos;
    }

    // Run a benchmark function (fixed iterations with sink pattern)
    // trackMemory param accepted for API compatibility but ignored - no memory overhead
    function runBenchmark(fn, iterations, warmupIterations, warmupTimeMs, useSink = true, trackMemory = false) {
        const samples = new Array(iterations);
        let lastResult;
        
        // Warmup phase (warmupTimeMs takes precedence)
        const warmupNanos = doWarmupSync(fn, warmupIterations, warmupTimeMs || 0, useSink);
        if (useSink) lastResult = globalThis.__polybench_sink;
        
        // Timed phase (no memory tracking)
        let totalNanos = 0;
        for (let i = 0; i < iterations; i++) {
            const start = now();
            if (useSink) {
                lastResult = fn();
                globalThis.__polybench_sink = lastResult;
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
            warmupNanos: warmupNanos,
            nanosPerOp: nanosPerOp,
            opsPerSec: opsPerSec,
            bytesPerOp: undefined,
            samples: samples,
            rawResult: normalizeRawResult(lastResult),
        };
    }

    // Run a benchmark with auto-calibration (time-based, like Go's testing.B)
    // Total benchmark time is approximately targetTimeMs
    // Uses batch timing only; no samples (aligns with Go/Rust - aggregate for nanos_per_op).
    function runBenchmarkAuto(fn, targetTimeMs, useSink = true, trackMemory = false, warmupIterations = 0, warmupTimeMs = 0) {
        const targetNanos = targetTimeMs * 1e6;
        let lastResult;
        const warmupNanos = doWarmupSync(fn, warmupIterations, warmupTimeMs, useSink);
        if (useSink) lastResult = globalThis.__polybench_sink;
        let iterations = 1;
        let totalIterations = 0;
        let totalNanos = 0;
        while (totalNanos < targetNanos) {
            const batchStart = now();
            for (let i = 0; i < iterations; i++) {
                if (useSink) {
                    lastResult = fn();
                    globalThis.__polybench_sink = lastResult;
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
            if (batchElapsed > 0) {
                const remainingNanos = targetNanos - totalNanos;
                const predicted = Math.floor(iterations * (remainingNanos / batchElapsed));
                let newIters;
                if (remainingNanos < batchElapsed) {
                    newIters = Math.max(1, predicted);
                } else if (remainingNanos < targetNanos / 5) {
                    newIters = Math.max(1, Math.floor(predicted * 0.9));
                } else {
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
        const nanosPerOp = totalNanos / totalIterations;
        const opsPerSec = 1e9 / nanosPerOp;
        return {
            iterations: totalIterations,
            totalNanos: totalNanos,
            warmupNanos: warmupNanos,
            nanosPerOp: nanosPerOp,
            opsPerSec: opsPerSec,
            bytesPerOp: undefined,
            samples: [],
            rawResult: normalizeRawResult(lastResult),
        };
    }

    function normalizeAsyncSamplingPolicy(policy) {
        if (typeof policy !== 'string') return 'timeBudgeted';
        const p = policy.toLowerCase();
        if (p === 'fixedcap' || p === 'fixed_cap' || p === 'fixed-cap') return 'fixedCap';
        return 'timeBudgeted';
    }

    function reservoirSamplePush(samples, sampleCap, value, seenCount) {
        const nextSeenCount = seenCount + 1;
        if (sampleCap <= 0) {
            return nextSeenCount;
        }
        if (samples.length < sampleCap) {
            samples.push(value);
            return nextSeenCount;
        }
        const replaceIdx = Math.floor(Math.random() * nextSeenCount);
        if (replaceIdx < sampleCap) {
            samples[replaceIdx] = value;
        }
        return nextSeenCount;
    }

    // Warmup helper for async: warmupTimeMs takes precedence. When using iterations, cap by warmupCap.
    // Returns warmup duration in nanoseconds.
    async function doWarmupAsync(fn, warmupIterations, warmupTimeMs, warmupCap, useSink) {
        let warmupNanos = 0;
        if (warmupTimeMs > 0) {
            const start = now();
            const limitNs = warmupTimeMs * 1e6;
            while ((now() - start) < limitNs) {
                try {
                    if (useSink) { globalThis.__polybench_sink = await fn(); } else { await fn(); }
                } catch (_) {}
            }
            warmupNanos = now() - start;
        } else if (warmupIterations > 0) {
            const effective = Math.min(warmupIterations, warmupCap);
            const start = now();
            for (let i = 0; i < effective; i++) {
                try {
                    if (useSink) { globalThis.__polybench_sink = await fn(); } else { await fn(); }
                } catch (_) {}
            }
            warmupNanos = now() - start;
        }
        return warmupNanos;
    }

    // Run an async benchmark function
    async function runBenchmarkAsync(fn, iterations, warmupIterations, warmupTimeMs, useSink = true, trackMemory = false, sampleCap = 50, warmupCap = 5, samplingPolicy = 'timeBudgeted') {
        const effectiveSampleCount = Math.min(sampleCap, iterations);
        const samples = [];
        let sampleSeenCount = 0;
        let lastResult;
        const successfulResults = [];
        const errorSamples = [];
        let successfulCount = 0;
        let errorCount = 0;
        const policy = normalizeAsyncSamplingPolicy(samplingPolicy);
        
        // Warmup phase (warmupTimeMs takes precedence)
        const warmupNanos = await doWarmupAsync(fn, warmupIterations || 0, warmupTimeMs || 0, warmupCap, useSink);
        lastResult = useSink ? globalThis.__polybench_sink : undefined;
        
        // Timed phase
        let totalNanos = 0;
        for (let i = 0; i < iterations; i++) {
            const start = now();
            try {
                if (useSink) {
                    lastResult = await fn();
                    globalThis.__polybench_sink = lastResult;
                } else {
                    lastResult = await fn();
                }
                successfulCount += 1;
                if (policy === 'fixedCap') {
                    if (successfulResults.length < effectiveSampleCount) {
                        successfulResults.push(normalizeRawResult(lastResult));
                    }
                } else {
                    successfulResults.push(normalizeRawResult(lastResult));
                }
            } catch (error) {
                errorCount += 1;
                if (errorSamples.length < effectiveSampleCount) {
                    errorSamples.push(normalizeErrorResult(error));
                }
            }
            const elapsed = now() - start;
            sampleSeenCount = reservoirSamplePush(samples, effectiveSampleCount, elapsed, sampleSeenCount);
            totalNanos += elapsed;
        }
        
        const nanosPerOp = totalNanos / iterations;
        const opsPerSec = 1e9 / nanosPerOp;

        return {
            iterations: iterations,
            totalNanos: totalNanos,
            warmupNanos: warmupNanos,
            nanosPerOp: nanosPerOp,
            opsPerSec: opsPerSec,
            bytesPerOp: undefined,
            samples: samples,
            rawResult: normalizeRawResult(lastResult),
            successfulResults: successfulResults,
            successfulCount: successfulCount,
            errorCount: errorCount,
            errorSamples: errorSamples,
        };
    }

    // Run an async benchmark with auto-calibration (time-based)
    async function runBenchmarkAutoAsync(fn, targetTimeMs, useSink = true, trackMemory = false, warmupIterations = 0, warmupTimeMs = 0, sampleCap = 50, warmupCap = 5, samplingPolicy = 'timeBudgeted') {
        const targetNanos = targetTimeMs * 1e6;
        const effectiveSampleCount = Math.max(0, sampleCap);
        const policy = normalizeAsyncSamplingPolicy(samplingPolicy);
        let lastResult;
        const successfulResults = [];
        const errorSamples = [];
        let successfulCount = 0;
        let errorCount = 0;
        const samples = [];
        let sampleSeenCount = 0;
        
        // Warmup (warmupTimeMs takes precedence)
        const warmupNanos = await doWarmupAsync(fn, warmupIterations, warmupTimeMs, warmupCap, useSink);
        lastResult = useSink ? globalThis.__polybench_sink : undefined;
        
        let totalIterations = 0;
        let totalNanos = 0;

        if (policy === 'fixedCap') {
            // Fixed-cap policy: execute a bounded number of async iterations.
            const fixedIterations = Math.max(1, sampleCap);
            for (let i = 0; i < fixedIterations; i++) {
                const start = now();
                try {
                    if (useSink) {
                        lastResult = await fn();
                        globalThis.__polybench_sink = lastResult;
                    } else {
                        lastResult = await fn();
                    }
                    successfulCount += 1;
                    successfulResults.push(normalizeRawResult(lastResult));
                } catch (error) {
                    errorCount += 1;
                    if (errorSamples.length < sampleCap) {
                        errorSamples.push(normalizeErrorResult(error));
                    }
                }
                const elapsed = now() - start;
                totalIterations += 1;
                totalNanos += elapsed;
                sampleSeenCount = reservoirSamplePush(samples, effectiveSampleCount, elapsed, sampleSeenCount);
            }
        } else {
            // Time-budgeted policy: execute until measured async time reaches target budget.
            while (totalNanos < targetNanos) {
                const start = now();
                try {
                    if (useSink) {
                        lastResult = await fn();
                        globalThis.__polybench_sink = lastResult;
                    } else {
                        lastResult = await fn();
                    }
                    successfulCount += 1;
                    successfulResults.push(normalizeRawResult(lastResult));
                } catch (error) {
                    errorCount += 1;
                    if (errorSamples.length < sampleCap) {
                        errorSamples.push(normalizeErrorResult(error));
                    }
                }
                const elapsed = now() - start;
                totalIterations += 1;
                totalNanos += elapsed;
                sampleSeenCount = reservoirSamplePush(samples, effectiveSampleCount, elapsed, sampleSeenCount);
            }
        }
        
        const nanosPerOp = totalNanos / (totalIterations > 0 ? totalIterations : 1);
        const opsPerSec = 1e9 / nanosPerOp;
        
        return {
            iterations: totalIterations,
            totalNanos: totalNanos,
            warmupNanos: warmupNanos,
            nanosPerOp: nanosPerOp,
            opsPerSec: opsPerSec,
            bytesPerOp: undefined,
            samples: samples,
            rawResult: normalizeRawResult(lastResult),
            successfulResults: successfulResults,
            successfulCount: successfulCount,
            errorCount: errorCount,
            errorSamples: errorSamples,
        };
    }

    // Warmup with hook (sync). Returns warmup duration in nanoseconds.
    function doWarmupSyncWithHook(fn, eachHook, warmupIterations, warmupTimeMs, useSink) {
        let warmupNanos = 0;
        if (warmupTimeMs > 0) {
            const start = now();
            const limitNs = warmupTimeMs * 1e6;
            while ((now() - start) < limitNs) {
                eachHook();
                if (useSink) { globalThis.__polybench_sink = fn(); } else { fn(); }
            }
            warmupNanos = now() - start;
        } else if (warmupIterations > 0) {
            const start = now();
            for (let i = 0; i < warmupIterations; i++) {
                eachHook();
                if (useSink) { globalThis.__polybench_sink = fn(); } else { fn(); }
            }
            warmupNanos = now() - start;
        }
        return warmupNanos;
    }

    // Run a benchmark function with an each-iteration hook
    function runBenchmarkWithHook(fn, eachHook, iterations, warmupIterations, warmupTimeMs, useSink = true, trackMemory = false) {
        const samples = new Array(iterations);
        let lastResult;
        
        // Warmup phase with hook (warmupTimeMs takes precedence)
        const warmupNanos = doWarmupSyncWithHook(fn, eachHook, warmupIterations || 0, warmupTimeMs || 0, useSink);
        lastResult = useSink ? globalThis.__polybench_sink : undefined;
        
        // Timed phase with hook (hook runs outside timing)
        let totalNanos = 0;
        for (let i = 0; i < iterations; i++) {
            eachHook();
            const start = now();
            if (useSink) {
                lastResult = fn();
                globalThis.__polybench_sink = lastResult;
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
            warmupNanos: warmupNanos,
            nanosPerOp: nanosPerOp,
            opsPerSec: opsPerSec,
            bytesPerOp: undefined,
            samples: samples,
            rawResult: normalizeRawResult(lastResult),
        };
    }

    // Warmup with hook (async). Returns warmup duration in nanoseconds.
    async function doWarmupAsyncWithHook(fn, eachHook, warmupIterations, warmupTimeMs, warmupCap, useSink) {
        let warmupNanos = 0;
        if (warmupTimeMs > 0) {
            const start = now();
            const limitNs = warmupTimeMs * 1e6;
            while ((now() - start) < limitNs) {
                try {
                    await eachHook();
                    if (useSink) { globalThis.__polybench_sink = await fn(); } else { await fn(); }
                } catch (_) {}
            }
            warmupNanos = now() - start;
        } else if (warmupIterations > 0) {
            const effective = Math.min(warmupIterations, warmupCap);
            const start = now();
            for (let i = 0; i < effective; i++) {
                try {
                    await eachHook();
                    if (useSink) { globalThis.__polybench_sink = await fn(); } else { await fn(); }
                } catch (_) {}
            }
            warmupNanos = now() - start;
        }
        return warmupNanos;
    }

    // Run an async benchmark function with an each-iteration hook
    async function runBenchmarkWithHookAsync(fn, eachHook, iterations, warmupIterations, warmupTimeMs, useSink = true, trackMemory = false, sampleCap = 50, warmupCap = 5, samplingPolicy = 'timeBudgeted') {
        const samples = new Array(iterations);
        let lastResult;
        const successfulResults = [];
        const errorSamples = [];
        let successfulCount = 0;
        let errorCount = 0;
        const policy = normalizeAsyncSamplingPolicy(samplingPolicy);
        
        // Warmup phase with hook (warmupTimeMs takes precedence)
        const warmupNanos = await doWarmupAsyncWithHook(fn, eachHook, warmupIterations || 0, warmupTimeMs || 0, warmupCap, useSink);
        lastResult = useSink ? globalThis.__polybench_sink : undefined;
        
        // Timed phase with hook (hook runs outside timing)
        let totalNanos = 0;
        for (let i = 0; i < iterations; i++) {
            let elapsed = 0;
            try {
                await eachHook();
                const start = now();
                if (useSink) {
                    lastResult = await fn();
                    globalThis.__polybench_sink = lastResult;
                } else {
                    lastResult = await fn();
                }
                elapsed = now() - start;
                successfulCount += 1;
                if (policy === 'fixedCap' && successfulResults.length < sampleCap) {
                    successfulResults.push(normalizeRawResult(lastResult));
                } else if (policy !== 'fixedCap') {
                    successfulResults.push(normalizeRawResult(lastResult));
                }
            } catch (error) {
                errorCount += 1;
                if (errorSamples.length < sampleCap) {
                    errorSamples.push(normalizeErrorResult(error));
                }
            }
            samples[i] = elapsed;
            totalNanos += elapsed;
        }
        
        const nanosPerOp = totalNanos / iterations;
        const opsPerSec = 1e9 / nanosPerOp;
        
        return {
            iterations: iterations,
            totalNanos: totalNanos,
            warmupNanos: warmupNanos,
            nanosPerOp: nanosPerOp,
            opsPerSec: opsPerSec,
            bytesPerOp: undefined,
            samples: samples,
            rawResult: normalizeRawResult(lastResult),
            successfulResults: successfulResults,
            successfulCount: successfulCount,
            errorCount: errorCount,
            errorSamples: errorSamples,
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

/// Memory harness: uses V8 total_allocated_bytes (GC-insensitive) and forces GC before measurement.
/// Use with Node --expose-gc for memory benchmarks.
pub const BENCH_HARNESS_MEMORY: &str = r#"
// Benchmark harness for poly-bench (memory path)
(function(globalThis) {
    'use strict';

    const v8 = (typeof require !== 'undefined') ? require('node:v8') : null;

    // High-resolution timing - use best available timer
    const now = (() => {
        if (typeof process !== 'undefined' && process.hrtime && process.hrtime.bigint) {
            return () => Number(process.hrtime.bigint());
        }
        if (typeof performance !== 'undefined') {
            return () => performance.now() * 1e6;
        }
        return () => Date.now() * 1e6;
    })();

    globalThis.__polybench_sink = undefined;

    // GC before measurement when available (Node --expose-gc)
    const forceGC = () => { if (typeof global.gc === 'function') global.gc(); };
    // Use V8 total_heap_size_executable or malloced_memory as cumulative metric
    // total_heap_size grows with allocations and doesn't shrink immediately after GC
    const getMemorySnapshot = () => {
        const stats = v8 && v8.getHeapStatistics ? v8.getHeapStatistics() : null;
        // malloced_memory tracks cumulative C++ allocations; total_heap_size tracks JS heap
        // Both are more stable than heapUsed for tracking allocations within benchmark loops
        const totalHeap = stats ? stats.total_heap_size : 0;
        const malloced = stats ? (stats.malloced_memory || 0) : 0;
        const heapUsed = (typeof process !== 'undefined' && process.memoryUsage) ? process.memoryUsage().heapUsed : 0;
        return { totalHeap, malloced, heapUsed };
    };
    const bytesPerOpFromSnapshots = (before, after, iters) => {
        // Try malloced_memory first (cumulative C++ allocations)
        if (before.malloced > 0 && after.malloced > 0) {
            const delta = (after.malloced - before.malloced) / iters;
            if (delta > 0) return Math.round(delta);
        }
        // Fallback to total_heap_size (cumulative JS heap growth)
        if (before.totalHeap > 0 && after.totalHeap > 0) {
            const delta = (after.totalHeap - before.totalHeap) / iters;
            if (delta > 0) return Math.round(delta);
        }
        // Last resort: heapUsed delta (can be negative after GC, so clamp)
        const heapDelta = Math.max(0, (after.heapUsed - before.heapUsed)) / iters;
        return heapDelta > 0 ? Math.round(heapDelta) : 0;
    };

    function normalizeRawResult(value) {
        if (value === undefined) return null;
        if (typeof value === 'bigint') return value.toString();
        try {
            return JSON.parse(JSON.stringify(value, (_, v) => typeof v === 'bigint' ? v.toString() : v));
        } catch (_) {
            return String(value);
        }
    }

    function normalizeErrorResult(error) {
        if (error === undefined || error === null) return 'Unknown async error';
        if (error && typeof error === 'object') {
            if (typeof error.stack === 'string' && error.stack.length > 0) return error.stack;
            if (typeof error.message === 'string' && error.message.length > 0) return error.message;
        }
        try {
            return String(error);
        } catch (_) {
            return 'Unserializable async error';
        }
    }

    function runBenchmark(fn, iterations, warmupIterations, warmupTimeMs, useSink = true, trackMemory = true) {
        const samples = new Array(iterations);
        let lastResult;
        if (warmupTimeMs > 0) {
            const start = now();
            const limitNs = warmupTimeMs * 1e6;
            while ((now() - start) < limitNs) {
                if (useSink) { lastResult = fn(); globalThis.__polybench_sink = lastResult; } else { fn(); }
            }
        } else if (warmupIterations > 0) {
            for (let i = 0; i < warmupIterations; i++) {
                if (useSink) { lastResult = fn(); globalThis.__polybench_sink = lastResult; } else { fn(); }
            }
        }
        lastResult = useSink ? globalThis.__polybench_sink : undefined;
        const memBefore = getMemorySnapshot();
        let totalNanos = 0;
        for (let i = 0; i < iterations; i++) {
            const start = now();
            if (useSink) { lastResult = fn(); globalThis.__polybench_sink = lastResult; } else { fn(); }
            samples[i] = now() - start;
            totalNanos += samples[i];
        }
        const memAfter = getMemorySnapshot();
        const bytesPerOp = bytesPerOpFromSnapshots(memBefore, memAfter, iterations);
        return { iterations, totalNanos, nanosPerOp: totalNanos / iterations, opsPerSec: 1e9 / (totalNanos / iterations), bytesPerOp, samples, rawResult: normalizeRawResult(lastResult) };
    }

    function runBenchmarkAuto(fn, targetTimeMs, useSink = true, trackMemory = true, warmupIterations = 0, warmupTimeMs = 0) {
        const targetNanos = targetTimeMs * 1e6;
        let lastResult;
        if (warmupTimeMs > 0) {
            const start = now();
            const limitNs = warmupTimeMs * 1e6;
            while ((now() - start) < limitNs) {
                if (useSink) { lastResult = fn(); globalThis.__polybench_sink = lastResult; } else { fn(); }
            }
        } else if (warmupIterations > 0) {
            for (let i = 0; i < warmupIterations; i++) {
                if (useSink) { lastResult = fn(); globalThis.__polybench_sink = lastResult; } else { fn(); }
            }
        }
        lastResult = useSink ? globalThis.__polybench_sink : undefined;
        const memBefore = getMemorySnapshot();
        let iterations = 1, totalIterations = 0, totalNanos = 0;
        const samples = [];
        while (totalNanos < targetNanos) {
            const batchStart = now();
            for (let i = 0; i < iterations; i++) {
                if (useSink) { lastResult = fn(); globalThis.__polybench_sink = lastResult; } else { fn(); }
            }
            const batchElapsed = now() - batchStart;
            totalIterations += iterations;
            totalNanos += batchElapsed;
            samples.push(batchElapsed / (iterations || 1));
            if (totalNanos >= targetNanos) break;
            if (batchElapsed > 0) {
                const remainingNanos = targetNanos - totalNanos;
                const predicted = Math.floor(iterations * (remainingNanos / batchElapsed));
                let newIters = remainingNanos < batchElapsed ? Math.max(1, predicted) : remainingNanos < targetNanos / 5 ? Math.max(1, Math.floor(predicted * 0.9)) : Math.min(iterations * 10, Math.max(iterations * 2, Math.floor(predicted * 1.1)));
                iterations = Math.max(1, newIters);
            } else { iterations *= 10; }
        }
        const memAfter = getMemorySnapshot();
        const bytesPerOp = bytesPerOpFromSnapshots(memBefore, memAfter, totalIterations);
        const nanosPerOp = totalNanos / totalIterations;
        return { iterations: totalIterations, totalNanos, nanosPerOp, opsPerSec: 1e9 / nanosPerOp, bytesPerOp, samples, rawResult: normalizeRawResult(lastResult) };
    }

    function normalizeAsyncSamplingPolicy(policy) {
        if (typeof policy !== 'string') return 'timeBudgeted';
        return (policy.toLowerCase() === 'fixedcap' || policy.toLowerCase() === 'fixed_cap' || policy.toLowerCase() === 'fixed-cap') ? 'fixedCap' : 'timeBudgeted';
    }

    function reservoirSamplePush(samples, sampleCap, value, seenCount) {
        const nextSeenCount = seenCount + 1;
        if (sampleCap <= 0) return nextSeenCount;
        if (samples.length < sampleCap) { samples.push(value); return nextSeenCount; }
        const replaceIdx = Math.floor(Math.random() * nextSeenCount);
        if (replaceIdx < sampleCap) samples[replaceIdx] = value;
        return nextSeenCount;
    }

    async function runBenchmarkAsync(fn, iterations, warmupIterations, warmupTimeMs, useSink = true, trackMemory = true, sampleCap = 50, warmupCap = 5, samplingPolicy = 'timeBudgeted') {
        const effectiveSampleCount = Math.min(sampleCap, iterations);
        const samples = []; let sampleSeenCount = 0, lastResult;
        const successfulResults = [], errorSamples = [];
        let successfulCount = 0, errorCount = 0;
        const policy = normalizeAsyncSamplingPolicy(samplingPolicy);
        if (warmupTimeMs > 0) {
            const start = now();
            const limitNs = warmupTimeMs * 1e6;
            while ((now() - start) < limitNs) {
                try { if (useSink) { lastResult = await fn(); globalThis.__polybench_sink = lastResult; } else { lastResult = await fn(); } } catch (_) {}
            }
        } else if (warmupIterations > 0) {
            const effective = Math.min(warmupIterations, warmupCap);
            for (let i = 0; i < effective; i++) {
                try { if (useSink) { lastResult = await fn(); globalThis.__polybench_sink = lastResult; } else { lastResult = await fn(); } } catch (_) {}
            }
        }
        lastResult = useSink ? globalThis.__polybench_sink : undefined;
        const memBefore = getMemorySnapshot();
        let totalNanos = 0;
        for (let i = 0; i < iterations; i++) {
            const start = now();
            try {
                if (useSink) { lastResult = await fn(); globalThis.__polybench_sink = lastResult; } else { lastResult = await fn(); }
                successfulCount++;
                if (policy === 'fixedCap') { if (successfulResults.length < effectiveSampleCount) successfulResults.push(normalizeRawResult(lastResult)); }
                else { successfulResults.push(normalizeRawResult(lastResult)); }
            } catch (error) { errorCount++; if (errorSamples.length < effectiveSampleCount) errorSamples.push(normalizeErrorResult(error)); }
            const elapsed = now() - start;
            sampleSeenCount = reservoirSamplePush(samples, effectiveSampleCount, elapsed, sampleSeenCount);
            totalNanos += elapsed;
        }
        const memAfter = getMemorySnapshot();
        const bytesPerOp = bytesPerOpFromSnapshots(memBefore, memAfter, iterations);
        return { iterations, totalNanos, nanosPerOp: totalNanos / iterations, opsPerSec: 1e9 / (totalNanos / iterations), bytesPerOp, samples, rawResult: normalizeRawResult(lastResult), successfulResults, successfulCount, errorCount, errorSamples };
    }

    async function runBenchmarkAutoAsync(fn, targetTimeMs, useSink = true, trackMemory = true, warmupIterations = 0, warmupTimeMs = 0, sampleCap = 50, warmupCap = 5, samplingPolicy = 'timeBudgeted') {
        const targetNanos = targetTimeMs * 1e6;
        const effectiveSampleCount = Math.max(0, sampleCap);
        const policy = normalizeAsyncSamplingPolicy(samplingPolicy);
        let lastResult;
        const successfulResults = [], errorSamples = [];
        let successfulCount = 0, errorCount = 0;
        const samples = []; let sampleSeenCount = 0;
        if (warmupTimeMs > 0) {
            const start = now();
            const limitNs = warmupTimeMs * 1e6;
            while ((now() - start) < limitNs) {
                try { if (useSink) { lastResult = await fn(); globalThis.__polybench_sink = lastResult; } else { lastResult = await fn(); } } catch (_) {}
            }
        } else if (warmupIterations > 0) {
            const effective = Math.min(warmupIterations, warmupCap);
            for (let i = 0; i < effective; i++) {
                try { if (useSink) { lastResult = await fn(); globalThis.__polybench_sink = lastResult; } else { lastResult = await fn(); } } catch (_) {}
            }
        }
        lastResult = useSink ? globalThis.__polybench_sink : undefined;
        const memBefore = getMemorySnapshot();
        let totalIterations = 0, totalNanos = 0;
        if (policy === 'fixedCap') {
            const fixedIterations = Math.max(1, sampleCap);
            for (let i = 0; i < fixedIterations; i++) {
                const start = now();
                try { if (useSink) { lastResult = await fn(); globalThis.__polybench_sink = lastResult; } else { lastResult = await fn(); } successfulCount++; successfulResults.push(normalizeRawResult(lastResult)); }
                catch (error) { errorCount++; if (errorSamples.length < sampleCap) errorSamples.push(normalizeErrorResult(error)); }
                const elapsed = now() - start;
                totalIterations++; totalNanos += elapsed;
                sampleSeenCount = reservoirSamplePush(samples, effectiveSampleCount, elapsed, sampleSeenCount);
            }
        } else {
            while (totalNanos < targetNanos) {
                const start = now();
                try { if (useSink) { lastResult = await fn(); globalThis.__polybench_sink = lastResult; } else { lastResult = await fn(); } successfulCount++; successfulResults.push(normalizeRawResult(lastResult)); }
                catch (error) { errorCount++; if (errorSamples.length < sampleCap) errorSamples.push(normalizeErrorResult(error)); }
                const elapsed = now() - start;
                totalIterations++; totalNanos += elapsed;
                sampleSeenCount = reservoirSamplePush(samples, effectiveSampleCount, elapsed, sampleSeenCount);
            }
        }
        const memAfter = getMemorySnapshot();
        const denomIterations = totalIterations > 0 ? totalIterations : 1;
        const bytesPerOp = bytesPerOpFromSnapshots(memBefore, memAfter, denomIterations);
        return { iterations: totalIterations, totalNanos, nanosPerOp: totalNanos / denomIterations, opsPerSec: 1e9 / (totalNanos / denomIterations), bytesPerOp, samples, rawResult: normalizeRawResult(lastResult), successfulResults, successfulCount, errorCount, errorSamples };
    }

    function runBenchmarkWithHook(fn, eachHook, iterations, warmupIterations, warmupTimeMs, useSink = true, trackMemory = true) {
        const samples = new Array(iterations);
        let lastResult;
        if (warmupTimeMs > 0) {
            const start = now();
            const limitNs = warmupTimeMs * 1e6;
            while ((now() - start) < limitNs) { eachHook(); if (useSink) { lastResult = fn(); globalThis.__polybench_sink = lastResult; } else { fn(); } }
        } else if (warmupIterations > 0) {
            for (let i = 0; i < warmupIterations; i++) { eachHook(); if (useSink) { lastResult = fn(); globalThis.__polybench_sink = lastResult; } else { fn(); } }
        }
        lastResult = useSink ? globalThis.__polybench_sink : undefined;
        const memBefore = getMemorySnapshot();
        let totalNanos = 0;
        for (let i = 0; i < iterations; i++) {
            eachHook();
            const start = now();
            if (useSink) { lastResult = fn(); globalThis.__polybench_sink = lastResult; } else { fn(); }
            samples[i] = now() - start;
            totalNanos += samples[i];
        }
        const memAfter = getMemorySnapshot();
        const bytesPerOp = bytesPerOpFromSnapshots(memBefore, memAfter, iterations);
        return { iterations, totalNanos, nanosPerOp: totalNanos / iterations, opsPerSec: 1e9 / (totalNanos / iterations), bytesPerOp, samples, rawResult: normalizeRawResult(lastResult) };
    }

    async function runBenchmarkWithHookAsync(fn, eachHook, iterations, warmupIterations, warmupTimeMs, useSink = true, trackMemory = true, sampleCap = 50, warmupCap = 5, samplingPolicy = 'timeBudgeted') {
        const samples = new Array(iterations);
        let lastResult;
        const successfulResults = [], errorSamples = [];
        let successfulCount = 0, errorCount = 0;
        const policy = normalizeAsyncSamplingPolicy(samplingPolicy);
        if (warmupTimeMs > 0) {
            const start = now();
            const limitNs = warmupTimeMs * 1e6;
            while ((now() - start) < limitNs) {
                try { await eachHook(); if (useSink) { lastResult = await fn(); globalThis.__polybench_sink = lastResult; } else { lastResult = await fn(); } } catch (_) {}
            }
        } else if (warmupIterations > 0) {
            const effective = Math.min(warmupIterations, warmupCap);
            for (let i = 0; i < effective; i++) {
                try { await eachHook(); if (useSink) { lastResult = await fn(); globalThis.__polybench_sink = lastResult; } else { lastResult = await fn(); } } catch (_) {}
            }
        }
        lastResult = useSink ? globalThis.__polybench_sink : undefined;
        const memBefore = getMemorySnapshot();
        let totalNanos = 0;
        for (let i = 0; i < iterations; i++) {
            let elapsed = 0;
            try {
                await eachHook();
                const start = now();
                if (useSink) { lastResult = await fn(); globalThis.__polybench_sink = lastResult; } else { lastResult = await fn(); }
                elapsed = now() - start;
                successfulCount++;
                if (policy === 'fixedCap' && successfulResults.length < sampleCap) successfulResults.push(normalizeRawResult(lastResult));
                else if (policy !== 'fixedCap') successfulResults.push(normalizeRawResult(lastResult));
            } catch (error) { errorCount++; if (errorSamples.length < sampleCap) errorSamples.push(normalizeErrorResult(error)); }
            samples[i] = elapsed;
            totalNanos += elapsed;
        }
        const memAfter = getMemorySnapshot();
        const bytesPerOp = bytesPerOpFromSnapshots(memBefore, memAfter, iterations);
        return { iterations, totalNanos, nanosPerOp: totalNanos / iterations, opsPerSec: 1e9 / (totalNanos / iterations), bytesPerOp, samples, rawResult: normalizeRawResult(lastResult), successfulResults, successfulCount, errorCount, errorSamples };
    }

    function hexToBytes(hex) {
        hex = hex.replace(/^0x/, '');
        const bytes = new Uint8Array(hex.length / 2);
        for (let i = 0; i < bytes.length; i++) bytes[i] = parseInt(hex.substr(i * 2, 2), 16);
        return bytes;
    }

    function bytesToHex(bytes) {
        return '0x' + Array.from(bytes).map(b => b.toString(16).padStart(2, '0')).join('');
    }

    globalThis.__polybench = {
        now, runBenchmark, runBenchmarkAuto, runBenchmarkAsync, runBenchmarkAutoAsync,
        runBenchmarkWithHook, runBenchmarkWithHookAsync, hexToBytes, bytesToHex,
    };
})(globalThis);
"#;

/// Select harness based on memory mode. Performance path has zero memory overhead.
pub fn get_bench_harness(track_memory: bool) -> &'static str {
    if track_memory {
        BENCH_HARNESS_MEMORY
    } else {
        BENCH_HARNESS_PERF
    }
}

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
pub fn generate_benchmark_code(
    _name: &str,
    impl_code: &str,
    iterations: u64,
    warmup: u64,
) -> String {
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

#[cfg(test)]
mod tests {
    use super::{BENCH_HARNESS_MEMORY, BENCH_HARNESS_PERF};

    #[test]
    fn test_harness_contains_async_sampling_policy_switch() {
        assert!(BENCH_HARNESS_PERF.contains("normalizeAsyncSamplingPolicy"));
        assert!(BENCH_HARNESS_PERF.contains("policy === 'fixedCap'"));
        assert!(BENCH_HARNESS_PERF.contains("policy !== 'fixedCap'"));
    }

    #[test]
    fn test_harness_contains_async_error_capture_fields() {
        assert!(BENCH_HARNESS_PERF.contains("successfulCount"));
        assert!(BENCH_HARNESS_PERF.contains("errorCount"));
        assert!(BENCH_HARNESS_PERF.contains("errorSamples"));
        assert!(BENCH_HARNESS_PERF.contains("normalizeErrorResult"));
    }

    #[test]
    fn test_harness_uses_reservoir_sampling_for_async_samples() {
        assert!(BENCH_HARNESS_PERF.contains("function reservoirSamplePush"));
        assert!(BENCH_HARNESS_PERF.contains("sampleSeenCount = reservoirSamplePush"));
        assert!(!BENCH_HARNESS_PERF.contains("samples.slice(0, effectiveSampleCount)"));
    }

    #[test]
    fn test_memory_harness_uses_total_allocated_bytes() {
        assert!(BENCH_HARNESS_MEMORY.contains("total_allocated_bytes"));
        assert!(BENCH_HARNESS_MEMORY.contains("global.gc"));
        assert!(BENCH_HARNESS_MEMORY.contains("node:v8"));
    }
}
