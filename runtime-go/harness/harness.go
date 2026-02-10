// Package harness provides the benchmark harness interface for poly-bench.
//
// This package defines the types and utilities used by generated benchmark
// plugins to communicate results back to the poly-bench runtime.
package harness

import (
	"encoding/json"
	"time"
)

// BenchResult holds the results of a benchmark run
type BenchResult struct {
	// Number of iterations executed
	Iterations uint64 `json:"iterations"`
	// Total time in nanoseconds
	TotalNanos uint64 `json:"total_nanos"`
	// Nanoseconds per operation
	NanosPerOp float64 `json:"nanos_per_op"`
	// Operations per second
	OpsPerSec float64 `json:"ops_per_sec"`
	// Bytes allocated per operation
	BytesPerOp uint64 `json:"bytes_per_op"`
	// Allocations per operation
	AllocsPerOp uint64 `json:"allocs_per_op"`
	// Individual sample times in nanoseconds
	Samples []uint64 `json:"samples,omitempty"`
}

// ToJSON serializes the result to JSON
func (r BenchResult) ToJSON() string {
	bytes, _ := json.Marshal(r)
	return string(bytes)
}

// BenchFunc is the signature for a benchmark function
type BenchFunc func() interface{}

// RunBenchmark executes a benchmark function and returns results
func RunBenchmark(fn BenchFunc, iterations, warmup int) BenchResult {
	samples := make([]uint64, iterations)

	// Warmup phase
	for i := 0; i < warmup; i++ {
		_ = fn()
	}

	// Timed phase
	var totalNanos uint64
	for i := 0; i < iterations; i++ {
		start := time.Now()
		_ = fn()
		elapsed := time.Since(start).Nanoseconds()
		samples[i] = uint64(elapsed)
		totalNanos += uint64(elapsed)
	}

	nanosPerOp := float64(totalNanos) / float64(iterations)
	opsPerSec := 1e9 / nanosPerOp

	return BenchResult{
		Iterations: uint64(iterations),
		TotalNanos: totalNanos,
		NanosPerOp: nanosPerOp,
		OpsPerSec:  opsPerSec,
		Samples:    samples,
	}
}

// MeasureOnce measures a single execution
func MeasureOnce(fn BenchFunc) time.Duration {
	start := time.Now()
	_ = fn()
	return time.Since(start)
}
