//! Benchmark execution scheduler

use super::{AnvilConfig, AnvilService, ProjectRoots};
use crate::comparison::{BenchmarkResult, BenchmarkResults, SuiteResults};
use colored::Colorize;
use miette::{miette, Result};
use poly_bench_dsl::{BenchMode, BenchmarkKind, ExecutionOrder, FairnessMode, Lang};
use poly_bench_ir::{BenchmarkIR, BenchmarkSpec, SuiteIR};
use poly_bench_runtime::{
    go::GoRuntime, js::JsRuntime, measurement::Measurement, rust::RustRuntime, traits::Runtime,
};
use std::{
    collections::HashMap,
    io::Write,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Instant,
};

/// Spinner frames for the timer
const SPINNER_FRAMES: &[&str] = &["[±]", "[∓]"];

/// Shared state for the multi-run timer
use std::sync::atomic::AtomicU64;

struct TimerState {
    stop_flag: AtomicBool,
    current_run: AtomicU64,
}

/// Simple deterministic RNG (xorshift64*) for reproducible shuffles without extra deps.
struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    fn new(seed: u64) -> Self {
        let state = if seed == 0 { 0x9E37_79B9_7F4A_7C15 } else { seed };
        Self { state }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
}

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
    let mut rng = DeterministicRng::new(seed);
    for i in (1..slice.len()).rev() {
        let j = (rng.next_u64() as usize) % (i + 1);
        slice.swap(i, j);
    }
}

fn strict_run_lang_order(
    spec: &BenchmarkSpec,
    suite: &SuiteIR,
    run_idx: u64,
    mut available_langs: Vec<Lang>,
) -> Vec<Lang> {
    let run_seed = spec
        .fairness_seed
        .unwrap_or_else(|| hash_str_to_u64(&format!("{}:{}", suite.name, spec.full_name))) ^
        (run_idx + 1);
    shuffle_slice(&mut available_langs, run_seed);
    available_langs
}

async fn run_with_optional_timeout<R: Runtime>(
    rt: &mut R,
    spec: &BenchmarkSpec,
    suite: &SuiteIR,
) -> Result<Measurement> {
    if let Some(timeout_ms) = spec.timeout {
        match tokio::time::timeout(
            tokio::time::Duration::from_millis(timeout_ms),
            rt.run_benchmark(spec, suite),
        )
        .await
        {
            Ok(result) => result,
            Err(_) => {
                Err(miette!("benchmark timed out after {}ms ({})", timeout_ms, spec.full_name))
            }
        }
    } else {
        rt.run_benchmark(spec, suite).await
    }
}

fn async_success_error_counts(measurement: &Measurement) -> (u64, u64) {
    let mut success = measurement
        .async_success_count
        .or_else(|| measurement.successful_results.as_ref().map(|v| v.len() as u64))
        .unwrap_or(0);
    let mut error = measurement.async_error_count.unwrap_or(0);
    if measurement.timed_out == Some(true) {
        error = error.saturating_add(1);
    }
    if success.saturating_add(error) == 0 && measurement.iterations > 0 {
        success = measurement.iterations;
    }
    (success, error)
}

fn async_outcome_suffix(kind: BenchmarkKind, measurement: &Measurement) -> String {
    if kind != BenchmarkKind::Async {
        return String::new();
    }
    let (success, error) = async_success_error_counts(measurement);
    let total = success.saturating_add(error);
    if total == 0 {
        return String::new();
    }
    let ok_pct = (success as f64 / total as f64) * 100.0;
    format!(" [ok/err: {}/{} ({:.0}% ok)]", success, error, ok_pct)
}

/// Start a background timer that displays elapsed seconds with a spinner
/// Returns a handle to stop the timer
fn start_timer(label: &str, label_color: &str) -> Arc<AtomicBool> {
    let stop_flag = Arc::new(AtomicBool::new(false));
    let stop_flag_clone = Arc::clone(&stop_flag);
    let label = label.to_string();
    let label_color = label_color.to_string();

    tokio::spawn(async move {
        let start = Instant::now();
        let mut frame_idx = 0;

        while !stop_flag_clone.load(Ordering::Relaxed) {
            let elapsed = start.elapsed().as_secs_f64();
            let spinner = SPINNER_FRAMES[frame_idx % SPINNER_FRAMES.len()];
            let colored_label = match label_color.as_str() {
                "green" => label.green().to_string(),
                "cyan" => label.cyan().to_string(),
                _ => label.clone(),
            };
            print!("\r    {} {} {:.1}s   ", colored_label, spinner.cyan(), elapsed);
            std::io::stdout().flush().ok();
            frame_idx += 1;
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    });

    stop_flag
}

/// Start a background timer for multi-run benchmarks that shows run progress
/// Returns a handle containing stop flag and current run counter
fn start_multi_run_timer(label: &str, label_color: &str, total_runs: u64) -> Arc<TimerState> {
    let state =
        Arc::new(TimerState { stop_flag: AtomicBool::new(false), current_run: AtomicU64::new(1) });
    let state_clone = Arc::clone(&state);
    let label = label.to_string();
    let label_color = label_color.to_string();

    tokio::spawn(async move {
        let start = Instant::now();
        let mut frame_idx = 0;

        while !state_clone.stop_flag.load(Ordering::Relaxed) {
            let elapsed = start.elapsed().as_secs_f64();
            let current = state_clone.current_run.load(Ordering::Relaxed);
            let spinner = SPINNER_FRAMES[frame_idx % SPINNER_FRAMES.len()];
            let colored_label = match label_color.as_str() {
                "green" => label.green().to_string(),
                "cyan" => label.cyan().to_string(),
                _ => label.clone(),
            };
            print!(
                "\r    {} run {}/{} {} {:.1}s   ",
                colored_label,
                current,
                total_runs,
                spinner.cyan(),
                elapsed
            );
            std::io::stdout().flush().ok();
            frame_idx += 1;
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    });

    state
}

/// Stop the timer and clear the line
fn stop_timer(stop_flag: &Arc<AtomicBool>) {
    stop_flag.store(true, Ordering::Relaxed);
    // Small delay to ensure the timer task sees the flag
    std::thread::sleep(std::time::Duration::from_millis(20));
    // Clear spinner line so it doesn't overlap with final output.
    print!("\r\x1b[2K");
    std::io::stdout().flush().ok();
}

/// Stop the multi-run timer
fn stop_multi_run_timer(state: &Arc<TimerState>) {
    state.stop_flag.store(true, Ordering::Relaxed);
    std::thread::sleep(std::time::Duration::from_millis(20));
    // Clear spinner line so it doesn't overlap with final output.
    print!("\r\x1b[2K");
    std::io::stdout().flush().ok();
}

/// Run all benchmarks in the IR
pub async fn run(
    ir: &BenchmarkIR,
    langs: &[Lang],
    iterations_override: Option<u64>,
    project_roots: &ProjectRoots,
) -> Result<BenchmarkResults> {
    let mut suite_results = Vec::new();

    // Check if globalSetup has spawnAnvil() and spawn Anvil if needed
    let anvil_service = if let Some(ref anvil_ir) = ir.anvil_config {
        println!("{} Starting Anvil node...", "⚡".yellow());

        // Build config from IR
        let config = AnvilConfig {
            fork_url: anvil_ir.fork_url.clone(),
            fork_block: None,
            use_proxy: anvil_ir.use_proxy,
        };

        match AnvilService::spawn(&config) {
            Ok(service) => {
                if anvil_ir.fork_url.is_some() {
                    println!("  {} Anvil ready at {} (forking)", "✓".green(), service.rpc_url);
                } else {
                    println!("  {} Anvil ready at {}", "✓".green(), service.rpc_url);
                }
                Some(service)
            }
            Err(e) => {
                eprintln!("  {} Failed to start Anvil: {}", "✗".red(), e);
                eprintln!("  {} Make sure Anvil is installed: curl -L https://foundry.paradigm.xyz | bash", "ℹ".blue());
                None
            }
        }
    } else {
        None
    };

    // Get the Anvil RPC URL if available
    let anvil_rpc_url = anvil_service.as_ref().map(|s| s.rpc_url.clone());

    for suite in &ir.suites {
        println!("\n{} Suite: {}", "▶".blue().bold(), suite.name.bold());
        if let Some(ref desc) = suite.description {
            println!("  {}", desc.dimmed());
        }
        let suite_start = Instant::now();

        let mut benchmark_results = Vec::new();

        // Initialize runtimes
        let mut go_runtime: Option<GoRuntime> = None;
        let mut js_runtime: Option<JsRuntime> = None;
        let mut rust_runtime: Option<RustRuntime> = None;

        if langs.contains(&Lang::Go) {
            let mut rt = GoRuntime::new();
            rt.set_module_root(project_roots.go_root.clone());
            if let Some(ref url) = anvil_rpc_url {
                rt.set_anvil_rpc_url(url.clone());
            }
            if let Err(e) = rt.initialize(suite).await {
                eprintln!("  {} Go runtime initialization failed: {}", "⚠".yellow(), e);
            } else {
                go_runtime = Some(rt);
            }
        }

        if langs.contains(&Lang::TypeScript) {
            match JsRuntime::new() {
                Ok(mut rt) => {
                    rt.set_project_root(project_roots.node_root.clone());
                    if let Some(ref url) = anvil_rpc_url {
                        rt.set_anvil_rpc_url(url.clone());
                    }
                    if let Err(e) = rt.initialize(suite).await {
                        eprintln!("  {} JS runtime initialization failed: {}", "⚠".yellow(), e);
                    } else {
                        js_runtime = Some(rt);
                    }
                }
                Err(e) => {
                    eprintln!("  {} JS runtime not available: {}", "⚠".yellow(), e);
                }
            }
        }

        if langs.contains(&Lang::Rust) {
            let mut rt = RustRuntime::new();
            rt.set_project_root(project_roots.rust_root.clone());
            if let Some(ref url) = anvil_rpc_url {
                rt.set_anvil_rpc_url(url.clone());
            }
            if let Err(e) = rt.initialize(suite).await {
                eprintln!("  {} Rust runtime initialization failed: {}", "⚠".yellow(), e);
            } else {
                rust_runtime = Some(rt);
            }
        }

        // Apply suite-level benchmark ordering
        let mut suite_benchmarks = suite.benchmarks.clone();
        match suite.order {
            ExecutionOrder::Sequential => {}
            ExecutionOrder::Random => {
                let seed = suite
                    .fairness_seed
                    .unwrap_or_else(|| hash_str_to_u64(&format!("suite-order:{}", suite.name)));
                shuffle_slice(&mut suite_benchmarks, seed);
            }
            ExecutionOrder::Parallel => {
                // Runtime-level benchmark parallelism is not yet enabled; preserve deterministic
                // sequential execution to avoid cross-runtime contention skew.
            }
        }

        // Run each benchmark
        for spec in &suite_benchmarks {
            let mut spec_clone = spec.clone();
            if let Some(override_iters) = iterations_override {
                spec_clone.iterations = override_iters;
            }

            // Print benchmark args - show all relevant settings
            let mut args = Vec::new();

            // Mode (auto vs fixed)
            match spec.mode {
                BenchMode::Auto => {
                    args.push(format!("auto"));
                    args.push(format!("targetTime={}ms", spec.target_time_ms));
                }
                BenchMode::Fixed => {
                    args.push(format!("fixed"));
                    args.push(format!("iterations={}", spec.iterations));
                }
            };

            // Count (statistical runs)
            if spec.count > 1 {
                args.push(format!("count={}", spec.count));
            }

            // Warmup (if non-default)
            if spec.warmup > 0 {
                args.push(format!("warmup={}", spec.warmup));
            }

            // Timeout (if set)
            if let Some(timeout) = spec.timeout {
                args.push(format!("timeout={}ms", timeout));
            }

            // Outlier detection
            if spec.outlier_detection {
                args.push(format!("outliers=iqr"));
            }

            // CV threshold (if non-default, default is typically 5.0)
            if spec.cv_threshold != 5.0 && spec.cv_threshold > 0.0 {
                args.push(format!("cvThreshold={}%", spec.cv_threshold));
            }

            // Memory profiling
            if spec.memory {
                args.push(format!("memory=true"));
            }

            // Use sink
            if spec.use_sink {
                args.push(format!("sink=true"));
            }

            println!("  {} {} [{}]", "→".dimmed(), spec.name.bold(), args.join(", ").dimmed());

            let mut measurements: HashMap<Lang, Measurement> = HashMap::new();
            let bench_start = Instant::now();
            let strict_fairness = spec_clone.fairness_mode == FairnessMode::Strict;

            if strict_fairness {
                // Precompile all participating runtimes before timed runs so interleaving does not
                // include compile overhead in any runtime's measured path.
                if spec.has_lang(Lang::Go) {
                    if let Some(ref mut rt) = go_runtime {
                        if let Err(e) = rt.precompile(&spec_clone, suite).await {
                            eprintln!("    {} Pre-compilation failed: {}", "Go:".red(), e);
                        }
                    }
                }
                if spec.has_lang(Lang::TypeScript) {
                    if let Some(ref mut rt) = js_runtime {
                        if let Err(e) = rt.precompile(&spec_clone, suite).await {
                            eprintln!("    {} Pre-compilation failed: {}", "TS:".red(), e);
                        }
                    }
                }
                if spec.has_lang(Lang::Rust) {
                    if let Some(ref mut rt) = rust_runtime {
                        if let Err(e) = rt.precompile(&spec_clone, suite).await {
                            eprintln!("    {} Pre-compilation failed: {}", "Rust:".red(), e);
                        }
                    }
                }

                let run_count = spec_clone.count.max(1);
                let mut run_measurements: HashMap<Lang, Vec<Measurement>> = HashMap::new();
                let strict_timer = start_multi_run_timer("strict:", "cyan", run_count);

                // Run-block interleaving: execute run k across all runtimes before run k+1.
                for run_idx in 0..run_count {
                    strict_timer.current_run.store(run_idx + 1, Ordering::Relaxed);
                    let mut run_langs = Vec::new();
                    if spec.has_lang(Lang::Go) && go_runtime.is_some() {
                        run_langs.push(Lang::Go);
                    }
                    if spec.has_lang(Lang::TypeScript) && js_runtime.is_some() {
                        run_langs.push(Lang::TypeScript);
                    }
                    if spec.has_lang(Lang::Rust) && rust_runtime.is_some() {
                        run_langs.push(Lang::Rust);
                    }

                    let run_langs = strict_run_lang_order(&spec_clone, suite, run_idx, run_langs);

                    for lang in run_langs {
                        match lang {
                            Lang::Go => {
                                if let Some(ref mut rt) = go_runtime {
                                    match run_with_optional_timeout(rt, &spec_clone, suite).await {
                                        Ok(m) => {
                                            run_measurements.entry(Lang::Go).or_default().push(m)
                                        }
                                        Err(e) => eprintln!(
                                            "\n    {} run {} failed: {}",
                                            "Go:".red(),
                                            run_idx + 1,
                                            e
                                        ),
                                    }
                                }
                            }
                            Lang::TypeScript => {
                                if let Some(ref mut rt) = js_runtime {
                                    match run_with_optional_timeout(rt, &spec_clone, suite).await {
                                        Ok(m) => run_measurements
                                            .entry(Lang::TypeScript)
                                            .or_default()
                                            .push(m),
                                        Err(e) => eprintln!(
                                            "\n    {} run {} failed: {}",
                                            "TS:".red(),
                                            run_idx + 1,
                                            e
                                        ),
                                    }
                                }
                            }
                            Lang::Rust => {
                                if let Some(ref mut rt) = rust_runtime {
                                    match run_with_optional_timeout(rt, &spec_clone, suite).await {
                                        Ok(m) => {
                                            run_measurements.entry(Lang::Rust).or_default().push(m)
                                        }
                                        Err(e) => eprintln!(
                                            "\n    {} run {} failed: {}",
                                            "Rust:".red(),
                                            run_idx + 1,
                                            e
                                        ),
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                stop_multi_run_timer(&strict_timer);

                for (lang, runs) in run_measurements {
                    if runs.is_empty() {
                        continue;
                    }
                    let aggregated = if runs.len() == 1 {
                        runs.into_iter().next().unwrap()
                    } else {
                        Measurement::aggregate_runs(runs)
                    };
                    measurements.insert(lang, aggregated);
                }
            } else {
                // Run Go benchmark
                if spec.has_lang(Lang::Go) {
                    if let Some(ref mut rt) = go_runtime {
                        // Pre-compile the benchmark binary before timing starts
                        // This ensures compilation overhead is not included in the wall-clock time
                        let precompile_start = Instant::now();
                        if let Err(e) = rt.precompile(&spec_clone, suite).await {
                            eprintln!("    {} Pre-compilation failed: {}", "Go:".red(), e);
                        }
                        let precompile_elapsed = precompile_start.elapsed();
                        if precompile_elapsed.as_millis() > 100 {
                            print!(
                                "    {} compiled in {:.2}s\n",
                                "Go:".green().dimmed(),
                                precompile_elapsed.as_secs_f64()
                            );
                        }

                        let lang_start = Instant::now();

                        if spec_clone.count > 1 {
                            // Multiple runs for statistical consistency with live timer
                            let timer = start_multi_run_timer("Go:", "green", spec_clone.count);
                            let mut run_measurements = Vec::new();

                            for run_idx in 0..spec_clone.count {
                                timer.current_run.store(run_idx + 1, Ordering::Relaxed);

                                match run_with_optional_timeout(rt, &spec_clone, suite).await {
                                    Ok(m) => run_measurements.push(m),
                                    Err(e) => {
                                        eprintln!(
                                            "\n    {} run {} failed: {}",
                                            "Go:".red(),
                                            run_idx + 1,
                                            e
                                        );
                                    }
                                }
                            }

                            stop_multi_run_timer(&timer);

                            if !run_measurements.is_empty() {
                                let aggregated = Measurement::aggregate_runs(run_measurements);
                                let elapsed = lang_start.elapsed();
                                let ci_str = if let (Some(median), Some(ci_upper)) =
                                    (aggregated.median_across_runs, aggregated.ci_95_upper)
                                {
                                    format!(" ±{}", Measurement::format_duration(ci_upper - median))
                                } else {
                                    String::new()
                                };
                                print!(
                                    "\r    {} {}{}{} ({}x runs, {:.2}s)                    ",
                                    "Go:".green(),
                                    Measurement::format_duration(aggregated.nanos_per_op),
                                    ci_str,
                                    async_outcome_suffix(spec.kind, &aggregated),
                                    spec_clone.count,
                                    elapsed.as_secs_f64()
                                );
                                measurements.insert(Lang::Go, aggregated);
                            }
                        } else {
                            // Single run with live timer
                            let timer = start_timer("Go:", "green");
                            let result = run_with_optional_timeout(rt, &spec_clone, suite).await;
                            stop_timer(&timer);

                            match result {
                                Ok(m) => {
                                    let elapsed = lang_start.elapsed();
                                    print!(
                                        "\r    {} {}{} ({})                    ",
                                        "Go:".green(),
                                        Measurement::format_duration(m.nanos_per_op),
                                        async_outcome_suffix(spec.kind, &m),
                                        format!("{:.2}s", elapsed.as_secs_f64()).dimmed()
                                    );
                                    measurements.insert(Lang::Go, m);
                                }
                                Err(e) => {
                                    if format!("{}", e).contains("timed out") {
                                        measurements
                                            .insert(Lang::Go, Measurement::timeout_marker());
                                    }
                                    print!(
                                        "\r    {} {}                    ",
                                        "Go:".red(),
                                        format!("{}", e).red()
                                    );
                                }
                            }
                        }
                        println!();
                    }
                }

                // Run TypeScript benchmark
                if spec.has_lang(Lang::TypeScript) {
                    if let Some(ref mut rt) = js_runtime {
                        // Pre-compile/prepare the benchmark script before timing starts
                        // This ensures script writing and syntax checking is not included in the
                        // wall-clock time
                        let precompile_start = Instant::now();
                        if let Err(e) = rt.precompile(&spec_clone, suite).await {
                            eprintln!("    {} Pre-compilation failed: {}", "TS:".red(), e);
                        }
                        let precompile_elapsed = precompile_start.elapsed();
                        if precompile_elapsed.as_millis() > 100 {
                            print!(
                                "    {} prepared in {:.2}s\n",
                                "TS:".cyan().dimmed(),
                                precompile_elapsed.as_secs_f64()
                            );
                        }

                        let lang_start = Instant::now();

                        if spec_clone.count > 1 {
                            // Multiple runs for statistical consistency with live timer
                            let timer = start_multi_run_timer("TS:", "cyan", spec_clone.count);
                            let mut run_measurements = Vec::new();

                            for run_idx in 0..spec_clone.count {
                                timer.current_run.store(run_idx + 1, Ordering::Relaxed);

                                match run_with_optional_timeout(rt, &spec_clone, suite).await {
                                    Ok(m) => run_measurements.push(m),
                                    Err(e) => {
                                        eprintln!(
                                            "\n    {} run {} failed: {}",
                                            "TS:".red(),
                                            run_idx + 1,
                                            e
                                        );
                                    }
                                }
                            }

                            stop_multi_run_timer(&timer);

                            if !run_measurements.is_empty() {
                                let aggregated = Measurement::aggregate_runs(run_measurements);
                                let elapsed = lang_start.elapsed();
                                let ci_str = if let (Some(median), Some(ci_upper)) =
                                    (aggregated.median_across_runs, aggregated.ci_95_upper)
                                {
                                    format!(" ±{}", Measurement::format_duration(ci_upper - median))
                                } else {
                                    String::new()
                                };
                                print!(
                                    "\r    {} {}{}{} ({}x runs, {:.2}s)                    ",
                                    "TS:".cyan(),
                                    Measurement::format_duration(aggregated.nanos_per_op),
                                    ci_str,
                                    async_outcome_suffix(spec.kind, &aggregated),
                                    spec_clone.count,
                                    elapsed.as_secs_f64()
                                );
                                measurements.insert(Lang::TypeScript, aggregated);
                            }
                        } else {
                            // Single run with live timer
                            let timer = start_timer("TS:", "cyan");
                            let result = run_with_optional_timeout(rt, &spec_clone, suite).await;
                            stop_timer(&timer);

                            match result {
                                Ok(m) => {
                                    let elapsed = lang_start.elapsed();
                                    print!(
                                        "\r    {} {}{} ({})                    ",
                                        "TS:".cyan(),
                                        Measurement::format_duration(m.nanos_per_op),
                                        async_outcome_suffix(spec.kind, &m),
                                        format!("{:.2}s", elapsed.as_secs_f64()).dimmed()
                                    );
                                    measurements.insert(Lang::TypeScript, m);
                                }
                                Err(e) => {
                                    if format!("{}", e).contains("timed out") {
                                        measurements.insert(
                                            Lang::TypeScript,
                                            Measurement::timeout_marker(),
                                        );
                                    }
                                    print!(
                                        "\r    {} {}                    ",
                                        "TS:".red(),
                                        format!("{}", e).red()
                                    );
                                }
                            }
                        }
                        println!();
                    }
                }

                // Run Rust benchmark
                if spec.has_lang(Lang::Rust) {
                    if let Some(ref mut rt) = rust_runtime {
                        // Pre-compile the benchmark binary before timing starts
                        // This ensures compilation overhead is not included in the wall-clock time
                        let precompile_start = Instant::now();
                        if let Err(e) = rt.precompile(&spec_clone, suite).await {
                            eprintln!("    {} Pre-compilation failed: {}", "Rust:".red(), e);
                        }
                        let precompile_elapsed = precompile_start.elapsed();
                        if precompile_elapsed.as_millis() > 100 {
                            print!(
                                "    {} compiled in {:.2}s\n",
                                "Rust:".yellow().dimmed(),
                                precompile_elapsed.as_secs_f64()
                            );
                        }

                        let lang_start = Instant::now();

                        if spec_clone.count > 1 {
                            // Multiple runs for statistical consistency with live timer
                            let timer = start_multi_run_timer("Rust:", "yellow", spec_clone.count);
                            let mut run_measurements = Vec::new();

                            for run_idx in 0..spec_clone.count {
                                timer.current_run.store(run_idx + 1, Ordering::Relaxed);

                                match run_with_optional_timeout(rt, &spec_clone, suite).await {
                                    Ok(m) => run_measurements.push(m),
                                    Err(e) => {
                                        eprintln!(
                                            "\n    {} run {} failed: {}",
                                            "Rust:".red(),
                                            run_idx + 1,
                                            e
                                        );
                                    }
                                }
                            }

                            stop_multi_run_timer(&timer);

                            if !run_measurements.is_empty() {
                                let aggregated = Measurement::aggregate_runs(run_measurements);
                                let elapsed = lang_start.elapsed();
                                let ci_str = if let (Some(median), Some(ci_upper)) =
                                    (aggregated.median_across_runs, aggregated.ci_95_upper)
                                {
                                    format!(" ±{}", Measurement::format_duration(ci_upper - median))
                                } else {
                                    String::new()
                                };
                                print!(
                                    "\r    {} {}{}{} ({}x runs, {:.2}s)                    ",
                                    "Rust:".yellow(),
                                    Measurement::format_duration(aggregated.nanos_per_op),
                                    ci_str,
                                    async_outcome_suffix(spec.kind, &aggregated),
                                    spec_clone.count,
                                    elapsed.as_secs_f64()
                                );
                                measurements.insert(Lang::Rust, aggregated);
                            }
                        } else {
                            // Single run with live timer
                            let timer = start_timer("Rust:", "yellow");
                            let result = run_with_optional_timeout(rt, &spec_clone, suite).await;
                            stop_timer(&timer);

                            match result {
                                Ok(m) => {
                                    let elapsed = lang_start.elapsed();
                                    print!(
                                        "\r    {} {}{} ({})                    ",
                                        "Rust:".yellow(),
                                        Measurement::format_duration(m.nanos_per_op),
                                        async_outcome_suffix(spec.kind, &m),
                                        format!("{:.2}s", elapsed.as_secs_f64()).dimmed()
                                    );
                                    measurements.insert(Lang::Rust, m);
                                }
                                Err(e) => {
                                    if format!("{}", e).contains("timed out") {
                                        measurements
                                            .insert(Lang::Rust, Measurement::timeout_marker());
                                    }
                                    print!(
                                        "\r    {} {}                    ",
                                        "Rust:".red(),
                                        format!("{}", e).red()
                                    );
                                }
                            }
                        }
                        println!();
                    }
                }
            }

            if strict_fairness {
                if let Some(go_m) = measurements.get(&Lang::Go) {
                    println!(
                        "    {} {}{}",
                        "Go:".green(),
                        Measurement::format_duration(go_m.nanos_per_op),
                        async_outcome_suffix(spec.kind, go_m)
                    );
                }
                if let Some(ts_m) = measurements.get(&Lang::TypeScript) {
                    println!(
                        "    {} {}{}",
                        "TS:".cyan(),
                        Measurement::format_duration(ts_m.nanos_per_op),
                        async_outcome_suffix(spec.kind, ts_m)
                    );
                }
                if let Some(rust_m) = measurements.get(&Lang::Rust) {
                    println!(
                        "    {} {}{}",
                        "Rust:".yellow(),
                        Measurement::format_duration(rust_m.nanos_per_op),
                        async_outcome_suffix(spec.kind, rust_m)
                    );
                }
            }

            let bench_elapsed = bench_start.elapsed();

            // Show comparison summary
            let mut comparison_parts = Vec::new();
            if let (Some(go_m), Some(ts_m)) =
                (measurements.get(&Lang::Go), measurements.get(&Lang::TypeScript))
            {
                let ratio = go_m.nanos_per_op / ts_m.nanos_per_op;
                if (ratio - 1.0).abs() < 0.05 {
                    comparison_parts.push("Go≈TS".dimmed().to_string());
                } else if ratio > 1.0 {
                    comparison_parts
                        .push(format!("TS {}x vs Go", format!("{:.2}", ratio)).cyan().to_string());
                } else {
                    comparison_parts.push(
                        format!("Go {}x vs TS", format!("{:.2}", 1.0 / ratio)).green().to_string(),
                    );
                }
            }
            if let (Some(go_m), Some(rust_m)) =
                (measurements.get(&Lang::Go), measurements.get(&Lang::Rust))
            {
                let ratio = go_m.nanos_per_op / rust_m.nanos_per_op;
                if (ratio - 1.0).abs() < 0.05 {
                    comparison_parts.push("Go≈Rust".dimmed().to_string());
                } else if ratio > 1.0 {
                    comparison_parts.push(
                        format!("Rust {}x vs Go", format!("{:.2}", ratio)).yellow().to_string(),
                    );
                } else {
                    comparison_parts.push(
                        format!("Go {}x vs Rust", format!("{:.2}", 1.0 / ratio))
                            .green()
                            .to_string(),
                    );
                }
            }
            if let (Some(ts_m), Some(rust_m)) =
                (measurements.get(&Lang::TypeScript), measurements.get(&Lang::Rust))
            {
                let ratio = ts_m.nanos_per_op / rust_m.nanos_per_op;
                if (ratio - 1.0).abs() < 0.05 {
                    comparison_parts.push("TS≈Rust".dimmed().to_string());
                } else if ratio > 1.0 {
                    comparison_parts.push(
                        format!("Rust {}x vs TS", format!("{:.2}", ratio)).yellow().to_string(),
                    );
                } else {
                    comparison_parts.push(
                        format!("TS {}x vs Rust", format!("{:.2}", 1.0 / ratio)).cyan().to_string(),
                    );
                }
            }

            if !comparison_parts.is_empty() {
                println!(
                    "    {} [{}]",
                    format!("total: {:.2}s", bench_elapsed.as_secs_f64()).dimmed(),
                    comparison_parts.join(", ")
                );
            } else {
                println!("    {}", format!("total: {:.2}s", bench_elapsed.as_secs_f64()).dimmed());
            }

            // Add visual separation between benchmarks
            println!();

            benchmark_results.push(BenchmarkResult::new(
                spec.name.clone(),
                spec.full_name.clone(),
                spec.kind,
                spec.description.clone(),
                measurements,
                if strict_fairness { "strict".to_string() } else { "legacy".to_string() },
                spec_clone.fairness_seed,
                Some(spec_clone.async_warmup_cap),
                Some(spec_clone.async_sample_cap),
                Some(match spec_clone.async_sampling_policy {
                    poly_bench_dsl::AsyncSamplingPolicy::FixedCap => "fixedCap".to_string(),
                    poly_bench_dsl::AsyncSamplingPolicy::TimeBudgeted => "timeBudgeted".to_string(),
                }),
            ));
        }

        // Shutdown runtimes
        if let Some(mut rt) = go_runtime {
            let _ = rt.shutdown().await;
        }
        if let Some(mut rt) = js_runtime {
            let _ = rt.shutdown().await;
        }
        if let Some(mut rt) = rust_runtime {
            let _ = rt.shutdown().await;
        }

        suite_results.push(SuiteResults::new(
            suite.name.clone(),
            suite.description.clone(),
            benchmark_results,
        ));
        println!(
            "  {}",
            format!("suite total: {:.2}s", suite_start.elapsed().as_secs_f64()).dimmed()
        );
    }

    Ok(BenchmarkResults::new(suite_results))
}

#[cfg(test)]
mod tests {
    use super::*;
    use poly_bench_dsl::BenchmarkKind;
    use poly_bench_runtime::measurement::Measurement;
    use std::collections::HashMap;

    #[test]
    fn test_shuffle_slice_is_deterministic_for_seed() {
        let mut a = vec![1, 2, 3, 4, 5];
        let mut b = vec![1, 2, 3, 4, 5];
        shuffle_slice(&mut a, 12345);
        shuffle_slice(&mut b, 12345);
        assert_eq!(a, b);
    }

    #[test]
    fn test_hash_str_to_u64_is_stable() {
        let x = hash_str_to_u64("suite:bench");
        let y = hash_str_to_u64("suite:bench");
        assert_eq!(x, y);
    }

    #[test]
    fn test_strict_run_lang_order_is_seeded_and_deterministic() {
        let mut suite = SuiteIR::new("suite".to_string());
        let mut spec = BenchmarkSpec::new("bench".to_string(), "suite", 100, 10);
        spec.fairness_seed = Some(42);
        suite.benchmarks.push(spec.clone());

        let langs = vec![Lang::Go, Lang::TypeScript, Lang::Rust];
        let run0_a = strict_run_lang_order(&spec, &suite, 0, langs.clone());
        let run0_b = strict_run_lang_order(&spec, &suite, 0, langs.clone());
        let run1 = strict_run_lang_order(&spec, &suite, 1, langs);

        assert_eq!(run0_a, run0_b);
        assert_ne!(run0_a, run1);
    }

    #[test]
    fn test_benchmark_result_comparison_mode_for_strict_and_legacy() {
        let strict = BenchmarkResult::new(
            "a".to_string(),
            "suite_a".to_string(),
            BenchmarkKind::Sync,
            None,
            HashMap::<Lang, Measurement>::new(),
            "strict".to_string(),
            Some(7),
            None,
            None,
            None,
        );
        let legacy = BenchmarkResult::new(
            "b".to_string(),
            "suite_b".to_string(),
            BenchmarkKind::Sync,
            None,
            HashMap::<Lang, Measurement>::new(),
            "legacy".to_string(),
            None,
            None,
            None,
            None,
        );

        assert_eq!(strict.comparison_mode, "strict");
        assert_eq!(strict.fairness_seed, Some(7));
        assert_eq!(legacy.comparison_mode, "legacy");
        assert_eq!(legacy.fairness_seed, None);
    }
}
