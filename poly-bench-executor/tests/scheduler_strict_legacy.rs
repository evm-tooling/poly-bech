use poly_bench_dsl::{BenchmarkKind, FairnessMode, Lang};
use poly_bench_executor::{comparison::BenchmarkResult, run, ProjectRoots, RunOptions};
use poly_bench_ir::{BenchmarkIR, BenchmarkSpec, SuiteIR};
use poly_bench_runtime::measurement::Measurement;
use std::collections::HashMap;

fn strict_order(
    spec: &BenchmarkSpec,
    suite: &SuiteIR,
    run_idx: u64,
    mut langs: Vec<Lang>,
) -> Vec<Lang> {
    fn hash_str_to_u64(s: &str) -> u64 {
        use std::{
            collections::hash_map::DefaultHasher,
            hash::{Hash, Hasher},
        };
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    }
    fn shuffle_slice<T>(slice: &mut [T], seed: u64) {
        if slice.len() < 2 {
            return;
        }
        let mut state = if seed == 0 { 0x9E37_79B9_7F4A_7C15 } else { seed };
        for i in (1..slice.len()).rev() {
            state ^= state >> 12;
            state ^= state << 25;
            state ^= state >> 27;
            let r = state.wrapping_mul(0x2545_F491_4F6C_DD1D) as usize;
            let j = r % (i + 1);
            slice.swap(i, j);
        }
    }

    let seed = spec
        .fairness_seed
        .unwrap_or_else(|| hash_str_to_u64(&format!("{}:{}", suite.name, spec.full_name))) ^
        (run_idx + 1);
    shuffle_slice(&mut langs, seed);
    langs
}

#[test]
fn test_strict_mode_interleaving_order_is_per_run() {
    let suite = SuiteIR::new("suite".to_string());
    let mut spec = BenchmarkSpec::new("bench".to_string(), "suite", 100, 10, 0);
    spec.fairness_mode = FairnessMode::Strict;
    spec.fairness_seed = Some(99);

    let langs = vec![Lang::Go, Lang::TypeScript, Lang::Rust];
    let run0 = strict_order(&spec, &suite, 0, langs.clone());
    let run1 = strict_order(&spec, &suite, 1, langs);

    assert_eq!(run0.len(), 3);
    assert_eq!(run1.len(), 3);
    assert_ne!(run0, run1);
}

#[test]
fn test_legacy_mode_metadata_is_legacy() {
    let result = BenchmarkResult::new(
        "bench".to_string(),
        "suite_bench".to_string(),
        poly_bench_dsl::BenchmarkKind::Sync,
        None,
        HashMap::<Lang, Measurement>::new(),
        poly_bench_dsl::SuiteType::Performance,
        "legacy".to_string(),
        None,
        None,
        None,
        None,
    );
    assert_eq!(result.comparison_mode, "legacy");
    assert_eq!(result.fairness_seed, None);
}

#[tokio::test]
async fn test_scheduler_emits_strict_and_legacy_modes() {
    let mut strict_suite = SuiteIR::new("strict_suite".to_string());
    let mut strict_bench = BenchmarkSpec::new("bench".to_string(), "strict_suite", 1, 0, 0);
    strict_bench.kind = BenchmarkKind::Sync;
    strict_bench.fairness_mode = FairnessMode::Strict;
    strict_bench.fairness_seed = Some(5);
    strict_suite.benchmarks.push(strict_bench);

    let mut legacy_suite = SuiteIR::new("legacy_suite".to_string());
    let mut legacy_bench = BenchmarkSpec::new("bench".to_string(), "legacy_suite", 1, 0, 0);
    legacy_bench.kind = BenchmarkKind::Sync;
    legacy_bench.fairness_mode = FairnessMode::Legacy;
    legacy_suite.benchmarks.push(legacy_bench);

    let opts = RunOptions::default();
    let strict_results =
        run(&BenchmarkIR::new(vec![strict_suite]), &[], None, &ProjectRoots::default(), &opts)
            .await
            .unwrap();
    let legacy_results =
        run(&BenchmarkIR::new(vec![legacy_suite]), &[], None, &ProjectRoots::default(), &opts)
            .await
            .unwrap();

    let strict_mode = &strict_results.suites[0].benchmarks[0].comparison_mode;
    let strict_seed = strict_results.suites[0].benchmarks[0].fairness_seed;
    let legacy_mode = &legacy_results.suites[0].benchmarks[0].comparison_mode;

    assert_eq!(strict_mode, "strict");
    assert_eq!(strict_seed, Some(5));
    assert_eq!(legacy_mode, "legacy");
}
