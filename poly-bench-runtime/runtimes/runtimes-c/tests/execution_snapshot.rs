//! Deterministic snapshot tests for C runtime execution.
//! Tests run_benchmark output structure and sanity across bench params.

use poly_bench_dsl::{BenchMode, Lang};
use poly_bench_ir::{BenchmarkSpec, FixtureIR, SuiteIR};
use poly_bench_traits::{Measurement, Runtime};
use runtimes_c::CRuntime;

fn minimal_suite_fixed() -> (BenchmarkSpec, SuiteIR) {
    let mut suite = SuiteIR::new("snap".to_string());
    suite.mode = BenchMode::Fixed;
    suite.default_iterations = 10;
    suite.default_warmup_iterations = 5;

    let fixture = FixtureIR::new("data".to_string(), vec![0xde, 0xad, 0xbe, 0xef]);
    suite.fixtures.push(fixture);

    let mut spec = BenchmarkSpec::new("noop".to_string(), "snap", 10, 5, 0);
    spec.mode = BenchMode::Fixed;
    spec.iterations = 10;
    spec.warmup_iterations = 5;
    spec.fixture_refs.push("data".to_string());
    spec.implementations.insert(Lang::C, "sizeof(data)".to_string());

    (spec, suite)
}

fn assert_measurement_sanity(m: &Measurement, expected_iterations: u64) {
    assert_eq!(m.iterations, expected_iterations, "iterations mismatch");
    assert!(m.total_nanos > 0, "total_nanos should be positive");
    assert!(m.nanos_per_op > 0.0, "nanos_per_op should be positive");
    assert!(m.nanos_per_op < 1e9, "nanos_per_op should be plausible (< 1s)");
    assert!(m.ops_per_sec > 0.0, "ops_per_sec should be positive");
}

#[tokio::test]
#[ignore = "requires clang in PATH"]
async fn test_c_fixed_mode_execution() {
    let (spec, suite) = minimal_suite_fixed();
    let mut rt = CRuntime::new().expect("clang required");
    rt.initialize(&suite).await.expect("init");

    let m = rt.run_benchmark(&spec, &suite).await.expect("run");
    assert_measurement_sanity(&m, 10);
}

#[tokio::test]
#[ignore = "requires clang in PATH"]
async fn test_c_fixed_mode_iterations_100() {
    let mut suite = SuiteIR::new("snap".to_string());
    suite.mode = BenchMode::Fixed;
    suite.fixtures.push(FixtureIR::new("data".to_string(), vec![0xde, 0xad, 0xbe, 0xef]));

    let mut spec = BenchmarkSpec::new("noop".to_string(), "snap", 100, 5, 0);
    spec.mode = BenchMode::Fixed;
    spec.iterations = 100;
    spec.warmup_iterations = 5;
    spec.fixture_refs.push("data".to_string());
    spec.implementations.insert(Lang::C, "sizeof(data)".to_string());

    let mut rt = CRuntime::new().expect("clang required");
    rt.initialize(&suite).await.expect("init");

    let m = rt.run_benchmark(&spec, &suite).await.expect("run");
    assert_measurement_sanity(&m, 100);
}
