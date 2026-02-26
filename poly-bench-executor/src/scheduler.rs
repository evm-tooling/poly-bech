//! Benchmark execution scheduler

use super::{AnvilConfig, AnvilService, ProjectRoots};
use crate::comparison::{BenchmarkResult, BenchmarkResults, SuiteResults};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use miette::{miette, Result};
use poly_bench_dsl::{BenchmarkKind, ExecutionOrder, FairnessMode, Lang, SuiteType};
use poly_bench_ir::{BenchmarkIR, BenchmarkSpec, SuiteIR};
use poly_bench_runtime::{
    create_runtimes, extract_generated_snippet, extract_runtime_error_reason, lang_label,
    measurement::Measurement, traits::Runtime, RuntimeConfig,
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
const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

/// Shared state for the multi-run timer
use std::sync::atomic::AtomicU64;

struct TimerState {
    stop_flag: AtomicBool,
    current_run: AtomicU64,
}

#[derive(Debug, Clone, Default)]
pub struct RunOptions {
    pub verbose: bool,
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

async fn run_with_optional_timeout(
    rt: &mut dyn Runtime,
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

/// Format the primary metric for display (time or memory based on suite type)
fn format_primary_metric(m: &Measurement, suite_type: SuiteType) -> String {
    if suite_type == SuiteType::Memory {
        m.bytes_per_op.map(Measurement::format_bytes).unwrap_or_else(|| "-".to_string())
    } else {
        Measurement::format_duration(m.nanos_per_op)
    }
}

/// Get the primary comparison value (lower is better)
fn primary_metric(m: &Measurement, suite_type: SuiteType) -> f64 {
    if suite_type == SuiteType::Memory {
        m.bytes_per_op.map(|b| b as f64).unwrap_or(m.nanos_per_op)
    } else {
        m.nanos_per_op
    }
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
    if error > 0 {
        format!("  {} {}/{} ({:.0}% ok)", "⚠".yellow(), success, error, ok_pct)
    } else {
        format!("  {} {}/{}", "✓".green(), success, error)
    }
}

fn async_error_measurement(err: &str) -> Measurement {
    let mut m = Measurement::timeout_marker();
    m.async_success_count = Some(0);
    m.async_error_count = Some(1);
    m.async_error_samples = Some(vec![err.chars().take(240).collect()]);
    m.estimator_source = Some("async-error".to_string());
    m.timed_out = Some(err.contains("timed out"));
    m
}

fn parse_generated_line(raw: &str) -> Option<usize> {
    // Handles patterns like:
    // - ".../bench.mjs:280"
    // - ".../main.rs:123:45"
    for token in raw.split_whitespace() {
        if !token.contains(':') {
            continue;
        }
        let cleaned = token.trim_matches(|c: char| c == '(' || c == ')' || c == ',' || c == ';');
        let parts: Vec<&str> = cleaned.split(':').collect();
        if parts.len() < 2 {
            continue;
        }
        // Try second-to-last first for file:line:col, then last for file:line.
        if parts.len() >= 3 {
            if let Ok(line) = parts[parts.len() - 2].parse::<usize>() {
                return Some(line);
            }
        }
        if let Ok(line) = parts[parts.len() - 1].parse::<usize>() {
            return Some(line);
        }
    }
    None
}

fn format_runtime_error(
    lang: Lang,
    run_idx: u64,
    spec: &BenchmarkSpec,
    suite_name: &str,
    raw_error: &str,
    verbose: bool,
) -> miette::Report {
    if verbose {
        return miette!(
            "{} run {} failed for {}: {}",
            lang_label(lang),
            run_idx,
            spec.full_name,
            raw_error
        );
    }

    let reason = extract_runtime_error_reason(raw_error);
    let mut lines = Vec::new();
    lines.push(format!("{} run {} failed for {}", lang_label(lang), run_idx, spec.full_name));
    lines.push(format!("suite: {}", suite_name));
    lines.push(format!("reason: {}", reason));

    if let (Some(src), Some(gen_line)) =
        (spec.implementation_sources.get(&lang), parse_generated_line(raw_error))
    {
        // In generated scripts, user impl usually starts after wrapper lines. Approximate with
        // relative offset to provide a useful .bench pointer.
        let bench_line = src.bench_file_line.saturating_add(gen_line.saturating_sub(1));
        lines.push(format!(
            "location: .bench line {} ({} implementation)",
            bench_line,
            lang_label(lang)
        ));
    } else if let Some(src) = spec.implementation_sources.get(&lang) {
        lines.push(format!(
            "location: .bench line {} ({} implementation)",
            src.bench_file_line,
            lang_label(lang)
        ));
    }

    if let Some(snippet) = extract_generated_snippet(raw_error, 1) {
        lines.push("snippet:".to_string());
        for l in snippet.into_iter().take(4) {
            lines.push(format!("  {}", l));
        }
    }

    lines.push("hint: re-run with -v to see raw external runtime trace".to_string());
    miette!("{}", lines.join("\n"))
}

fn colorize_lang_label(label: &str, lang: Lang) -> String {
    use poly_bench_runtime::lang_display;
    match lang_display(lang).terminal_color {
        "green" => label.green().to_string(),
        "cyan" => label.cyan().to_string(),
        "yellow" => label.yellow().to_string(),
        "bright_blue" | "blue" => label.bright_blue().to_string(),
        _ => label.to_string(),
    }
}

fn lang_versus_counterpart(
    measurements: &HashMap<Lang, Measurement>,
    lang: Lang,
    suite_type: SuiteType,
) -> Option<(Lang, f64)> {
    let current = measurements.get(&lang)?;
    let current_val = primary_metric(current, suite_type);
    if current_val <= 0.0 {
        return None;
    }

    let mut best: Option<(Lang, f64)> = None;
    for (peer_lang, peer_m) in measurements {
        if *peer_lang == lang {
            continue;
        }
        let peer_val = primary_metric(peer_m, suite_type);
        if peer_val <= 0.0 {
            continue;
        }
        let ratio =
            if current_val <= peer_val { peer_val / current_val } else { current_val / peer_val };
        if best.as_ref().map(|(_, r)| ratio > *r).unwrap_or(true) {
            best = Some((*peer_lang, ratio));
        }
    }
    best
}

/// Start a background timer that displays elapsed seconds with a spinner
/// Returns a handle to stop the timer
fn start_timer(label: &str) -> Arc<AtomicBool> {
    let stop_flag = Arc::new(AtomicBool::new(false));
    let stop_flag_clone = Arc::clone(&stop_flag);
    let label = label.to_string();

    tokio::spawn(async move {
        let start = Instant::now();
        let mut frame_idx = 0;

        while !stop_flag_clone.load(Ordering::Relaxed) {
            let elapsed = start.elapsed().as_secs_f64();
            let spinner = SPINNER_FRAMES[frame_idx % SPINNER_FRAMES.len()];
            print!("\r    {} {} {:.1}s   ", label.dimmed(), spinner.cyan(), elapsed);
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
    options: &RunOptions,
) -> Result<BenchmarkResults> {
    let mut suite_results = Vec::new();

    // Check if globalSetup has spawnAnvil() and spawn Anvil if needed
    let anvil_service = if let Some(ref anvil_ir) = ir.anvil_config {
        let anvil_spinner = ProgressBar::new_spinner();
        anvil_spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.cyan} {msg}")
                .unwrap()
                .tick_strings(SPINNER_FRAMES),
        );
        anvil_spinner.set_message("Setting up Anvil...");
        anvil_spinner.enable_steady_tick(tokio::time::Duration::from_millis(100));

        // Build config from IR
        let config = AnvilConfig {
            fork_url: anvil_ir.fork_url.clone(),
            fork_block: None,
            use_proxy: anvil_ir.use_proxy,
        };

        match AnvilService::spawn(&config) {
            Ok(service) => {
                anvil_spinner.finish_and_clear();
                println!("{} Anvil is ready", "✓".green().bold());
                Some(service)
            }
            Err(e) => {
                anvil_spinner.finish_and_clear();
                return Err(miette!(
                    "Failed to start Anvil: {}. Ensure Anvil is installed: curl -L https://foundry.paradigm.xyz | bash",
                    e
                ));
            }
        }
    } else {
        None
    };

    // Get the Anvil RPC URL if available
    let anvil_rpc_url = anvil_service.as_ref().map(|s| s.rpc_url.clone());

    for suite in &ir.suites {
        print!("\n{} Suite: {}", "▶".blue().bold(), suite.name.bold());
        if let Some(ref desc) = suite.description {
            print!("  {}", desc.dimmed());
        }
        println!();
        let suite_start = Instant::now();

        let mut benchmark_results = Vec::new();

        // Initialize runtimes via registry
        let config = RuntimeConfig { roots: project_roots.roots.clone() };
        let mut runtimes = create_runtimes(langs, &config)
            .map_err(|e| miette!("Runtime initialization failed: {}", e))?;

        for (_, rt) in runtimes.iter_mut() {
            if let Some(ref url) = anvil_rpc_url {
                rt.set_anvil_rpc_url(url.clone());
            }
            rt.initialize(suite)
                .await
                .map_err(|e| miette!("Runtime initialization failed: {}", e))?;
        }

        // Route to memory or performance path based on suite type.
        // Memory path uses median aggregation for bytes_per_op; performance path uses mean.
        let is_memory_suite = suite.suite_type == SuiteType::Memory;

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

            let mut measurements: HashMap<Lang, Measurement> = HashMap::new();
            let mut precompile_nanos: HashMap<Lang, u64> = HashMap::new();
            let strict_fairness = spec_clone.fairness_mode == FairnessMode::Strict;
            let bench_wall_elapsed: Option<f64>;

            if strict_fairness {
                // Precompile all participating runtimes before timed runs so interleaving does not
                // include compile overhead in any runtime's measured path.
                for lang in langs {
                    if spec.has_lang(*lang) {
                        if let Some(ref mut rt) = runtimes.get_mut(lang) {
                            let pc_start = Instant::now();
                            rt.precompile(&spec_clone, suite).await.map_err(|e| {
                                miette!(
                                    "{} pre-compilation failed ({}): {}",
                                    lang_label(*lang),
                                    spec_clone.full_name,
                                    e
                                )
                            })?;
                            let wall_nanos = pc_start.elapsed().as_nanos() as u64;
                            let rt_nanos = rt.last_precompile_nanos().unwrap_or(0);
                            let nanos = std::cmp::max(wall_nanos, rt_nanos);
                            precompile_nanos.insert(*lang, nanos);
                        }
                    }
                }

                let run_count = spec_clone.count.max(1);
                let mut run_measurements: HashMap<Lang, Vec<Measurement>> = HashMap::new();
                let strict_label = format!("{}:", suite.name);
                let strict_timer = start_multi_run_timer(&strict_label, "cyan", run_count);
                let bench_wall_start = Instant::now();

                // Run-block interleaving: execute run k across all runtimes before run k+1.
                for run_idx in 0..run_count {
                    strict_timer.current_run.store(run_idx + 1, Ordering::Relaxed);
                    let run_langs: Vec<Lang> = langs
                        .iter()
                        .copied()
                        .filter(|l| spec.has_lang(*l) && runtimes.contains_key(l))
                        .collect();
                    let run_langs = strict_run_lang_order(&spec_clone, suite, run_idx, run_langs);

                    for lang in run_langs {
                        if let Some(ref mut rt) = runtimes.get_mut(&lang) {
                            match run_with_optional_timeout(rt.as_mut(), &spec_clone, suite).await {
                                Ok(m) => {
                                    run_measurements.entry(lang).or_default().push(m);
                                }
                                Err(e) => {
                                    let err = format!("{}", e);
                                    let label = &format!("{}:", lang_label(lang));
                                    let colored_label = colorize_lang_label(label, lang);
                                    if spec_clone.kind == BenchmarkKind::Async {
                                        eprintln!(
                                            "\n    {} run {} failed for {} (recorded as async error)",
                                            colored_label,
                                            run_idx + 1,
                                            spec_clone.full_name
                                        );
                                        run_measurements
                                            .entry(lang)
                                            .or_default()
                                            .push(async_error_measurement(&err));
                                    } else {
                                        return Err(format_runtime_error(
                                            lang,
                                            run_idx + 1,
                                            &spec_clone,
                                            &suite.name,
                                            &e.to_string(),
                                            options.verbose,
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
                stop_multi_run_timer(&strict_timer);
                bench_wall_elapsed = Some(bench_wall_start.elapsed().as_secs_f64());

                for (lang, runs) in run_measurements {
                    if runs.is_empty() {
                        continue;
                    }
                    let aggregated = if runs.len() == 1 {
                        runs.into_iter().next().unwrap()
                    } else if is_memory_suite {
                        Measurement::aggregate_runs_memory(runs)
                    } else {
                        Measurement::aggregate_runs(runs)
                    };
                    measurements.insert(lang, aggregated);
                }
            } else {
                // Run each language benchmark (non-strict fairness: sequential per lang)
                let bench_wall_start = Instant::now();
                for lang in langs {
                    if !spec.has_lang(*lang) {
                        continue;
                    }
                    let Some(ref mut rt) = runtimes.get_mut(lang) else {
                        continue;
                    };

                    let precompile_start = Instant::now();
                    rt.precompile(&spec_clone, suite).await.map_err(|e| {
                        miette!(
                            "{} pre-compilation failed ({}): {}",
                            lang_label(*lang),
                            spec_clone.full_name,
                            e
                        )
                    })?;
                    let wall_nanos = precompile_start.elapsed().as_nanos() as u64;
                    let rt_nanos = rt.last_precompile_nanos().unwrap_or(0);
                    let nanos = std::cmp::max(wall_nanos, rt_nanos);
                    precompile_nanos.insert(*lang, nanos);

                    let timer_color = poly_bench_runtime::lang_display(*lang).terminal_color;
                    let lang_label_str = lang_label(*lang);

                    if spec_clone.count > 1 {
                        let label = format!("{} {}:", suite.name, lang_label_str);
                        let timer = start_multi_run_timer(&label, timer_color, spec_clone.count);
                        let mut run_measurements = Vec::new();

                        for run_idx in 0..spec_clone.count {
                            timer.current_run.store(run_idx + 1, Ordering::Relaxed);

                            match run_with_optional_timeout(rt.as_mut(), &spec_clone, suite).await {
                                Ok(m) => run_measurements.push(m),
                                Err(e) => {
                                    let err = format!("{}", e);
                                    if spec_clone.kind == BenchmarkKind::Async {
                                        run_measurements.push(async_error_measurement(&err));
                                    } else {
                                        return Err(format_runtime_error(
                                            *lang,
                                            run_idx + 1,
                                            &spec_clone,
                                            &suite.name,
                                            &e.to_string(),
                                            options.verbose,
                                        ));
                                    }
                                }
                            }
                        }

                        stop_multi_run_timer(&timer);

                        if !run_measurements.is_empty() {
                            let aggregated = if is_memory_suite {
                                Measurement::aggregate_runs_memory(run_measurements)
                            } else {
                                Measurement::aggregate_runs(run_measurements)
                            };
                            let actual_run_secs = aggregated.total_nanos as f64 / 1e9;
                            let ci_str = if let (Some(median), Some(ci_upper)) =
                                (aggregated.median_across_runs, aggregated.ci_95_upper)
                            {
                                format!(" ±{}", Measurement::format_duration(ci_upper - median))
                            } else {
                                String::new()
                            };
                            let colored_label =
                                colorize_lang_label(&format!("{}:", lang_label_str), *lang);
                            print!(
                                "\r    {} {}{}{} ({}x runs, {:.2}s)                    ",
                                colored_label,
                                format_primary_metric(&aggregated, suite.suite_type),
                                ci_str,
                                async_outcome_suffix(spec.kind, &aggregated),
                                spec_clone.count,
                                actual_run_secs
                            );
                            measurements.insert(*lang, aggregated);
                        }
                    } else {
                        let label = format!("{} {}:", suite.name, lang_label_str);
                        let timer = start_timer(&label);
                        let result =
                            run_with_optional_timeout(rt.as_mut(), &spec_clone, suite).await;
                        stop_timer(&timer);

                        match result {
                            Ok(m) => {
                                let actual_run_secs = m.total_nanos as f64 / 1e9;
                                let colored_label =
                                    colorize_lang_label(&format!("{}:", lang_label_str), *lang);
                                print!(
                                    "\r    {} {}{} ({})                    ",
                                    colored_label,
                                    format_primary_metric(&m, suite.suite_type),
                                    async_outcome_suffix(spec.kind, &m),
                                    format!("{:.2}s", actual_run_secs).dimmed()
                                );
                                measurements.insert(*lang, m);
                            }
                            Err(e) => {
                                if spec_clone.kind == BenchmarkKind::Async {
                                    measurements
                                        .insert(*lang, async_error_measurement(&format!("{}", e)));
                                } else {
                                    return Err(format_runtime_error(
                                        *lang,
                                        1,
                                        &spec_clone,
                                        &suite.name,
                                        &e.to_string(),
                                        options.verbose,
                                    ));
                                }
                            }
                        }
                    }
                    println!();
                }
                bench_wall_elapsed = Some(bench_wall_start.elapsed().as_secs_f64());
            }

            // Title: only total elapsed (wall when available, else exec)
            let total_elapsed = bench_wall_elapsed.unwrap_or_else(|| {
                measurements.values().map(|m| m.total_nanos).sum::<u64>() as f64 / 1e9
            });
            println!("  ▸ {} {:.2}s", spec.name.bold(), total_elapsed);

            println!(
                "    {:<8} {:<16} {:<24} {:>12} {:>16} {:>10} {:>12}",
                "lang",
                "ns/op",
                "speedup/baseline",
                "runtime (ms)",
                "warmupTime (ms)",
                "spawn(ms)",
                "precompile"
            );
            for lang in langs {
                if let Some(m) = measurements.get(&lang) {
                    let metric = format_primary_metric(m, suite.suite_type);
                    let rel = lang_versus_counterpart(&measurements, *lang, suite.suite_type)
                        .map(|(peer, ratio)| format!("{:.2}x vs {}", ratio, lang_label(peer)))
                        .unwrap_or_default();
                    let runtime_secs = m.total_nanos as f64 / 1e9;
                    let warmup_str = m
                        .warmup_nanos
                        .map(|n| format!("{:.2}", n as f64 / 1e6))
                        .unwrap_or_else(|| "-".to_string());
                    let spawn_str = m
                        .spawn_nanos
                        .map(|n| format!("{:.2}", n as f64 / 1e6))
                        .unwrap_or_else(|| "-".to_string());
                    let precompile_str = precompile_nanos
                        .get(&lang)
                        .map(|n| {
                            let ms = *n as f64 / 1e6;
                            if ms >= 0.01 {
                                format!("{:.2}", ms)
                            } else if *n > 0 {
                                format!("{:.0}µ", *n as f64 / 1e3)
                            } else {
                                "0".to_string()
                            }
                        })
                        .unwrap_or_else(|| "-".to_string());
                    let padded_label = format!("{:<7}", format!("{}:", lang_label(*lang)));
                    let colored_label = colorize_lang_label(&padded_label, *lang);
                    println!(
                        "    {} {:<16} {:<24} {:>12.2}s {:>16} {:>10} {:>12}",
                        colored_label,
                        metric,
                        rel,
                        runtime_secs,
                        warmup_str,
                        spawn_str,
                        precompile_str
                    );
                }
            }

            // Reconciliation: sum of components vs header total
            let sum_runtime_s =
                measurements.values().map(|m| m.total_nanos).sum::<u64>() as f64 / 1e9;
            let sum_warmup_s =
                measurements.values().filter_map(|m| m.warmup_nanos).sum::<u64>() as f64 / 1e9;
            let sum_spawn_s =
                measurements.values().filter_map(|m| m.spawn_nanos).sum::<u64>() as f64 / 1e9;
            let sum_precompile_s = precompile_nanos.values().sum::<u64>() as f64 / 1e9;
            let sum_all = sum_runtime_s + sum_warmup_s + sum_spawn_s + sum_precompile_s;
            let gap = (total_elapsed - sum_all).abs();
            let eq_sign = if gap < 0.05 { "=" } else { "≈" };
            println!(
                "    {}",
                format!(
                    "sum: runtime {:.2}s + warmup {:.3}s + spawn {:.3}s + precompile {:.3}s {} {:.2}s",
                    sum_runtime_s,
                    sum_warmup_s,
                    sum_spawn_s,
                    sum_precompile_s,
                    eq_sign,
                    total_elapsed,
                )
                .dimmed()
            );
            println!();

            benchmark_results.push(BenchmarkResult::new(
                spec.name.clone(),
                spec.full_name.clone(),
                spec.kind,
                spec.description.clone(),
                measurements,
                suite.suite_type,
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
        for (_, rt) in runtimes.iter_mut() {
            let _ = rt.as_mut().shutdown().await;
        }

        suite_results.push(SuiteResults::new(
            suite.name.clone(),
            suite.description.clone(),
            suite.suite_type,
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
        let mut spec = BenchmarkSpec::new("bench".to_string(), "suite", 100, 10, 0);
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
        use poly_bench_dsl::SuiteType;

        let strict = BenchmarkResult::new(
            "a".to_string(),
            "suite_a".to_string(),
            BenchmarkKind::Sync,
            None,
            HashMap::<Lang, Measurement>::new(),
            SuiteType::Performance,
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
            SuiteType::Performance,
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
