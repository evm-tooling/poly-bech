//! Benchmark execution scheduler

use super::{AnvilConfig, AnvilService, ProjectRoots};
use crate::comparison::{BenchmarkResult, BenchmarkResults, SuiteResults};
use colored::Colorize;
use miette::Result;
use poly_bench_dsl::{BenchMode, Lang};
use poly_bench_ir::BenchmarkIR;
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
}

/// Stop the multi-run timer
fn stop_multi_run_timer(state: &Arc<TimerState>) {
    state.stop_flag.store(true, Ordering::Relaxed);
    std::thread::sleep(std::time::Duration::from_millis(20));
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
        let config = AnvilConfig { fork_url: anvil_ir.fork_url.clone(), fork_block: None };

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

        // Run each benchmark
        for spec in &suite.benchmarks {
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

            // Concurrency
            if spec.concurrency > 1 {
                args.push(format!("concurrency={}", spec.concurrency));
            }

            // Use sink
            if spec.use_sink {
                args.push(format!("sink=true"));
            }

            println!("  {} {} [{}]", "→".dimmed(), spec.name.bold(), args.join(", ").dimmed());

            let mut measurements: HashMap<Lang, Measurement> = HashMap::new();
            let bench_start = Instant::now();

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

                            match rt.run_benchmark(&spec_clone, suite).await {
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
                                "\r    {} {}{} ({}x runs, {:.2}s)                    ",
                                "Go:".green(),
                                Measurement::format_duration(aggregated.nanos_per_op),
                                ci_str,
                                spec_clone.count,
                                elapsed.as_secs_f64()
                            );
                            measurements.insert(Lang::Go, aggregated);
                        }
                    } else {
                        // Single run with live timer
                        let timer = start_timer("Go:", "green");
                        let result = rt.run_benchmark(&spec_clone, suite).await;
                        stop_timer(&timer);

                        match result {
                            Ok(m) => {
                                let elapsed = lang_start.elapsed();
                                print!(
                                    "\r    {} {} ({})                    ",
                                    "Go:".green(),
                                    Measurement::format_duration(m.nanos_per_op),
                                    format!("{:.2}s", elapsed.as_secs_f64()).dimmed()
                                );
                                measurements.insert(Lang::Go, m);
                            }
                            Err(e) => {
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

                            match rt.run_benchmark(&spec_clone, suite).await {
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
                                "\r    {} {}{} ({}x runs, {:.2}s)                    ",
                                "TS:".cyan(),
                                Measurement::format_duration(aggregated.nanos_per_op),
                                ci_str,
                                spec_clone.count,
                                elapsed.as_secs_f64()
                            );
                            measurements.insert(Lang::TypeScript, aggregated);
                        }
                    } else {
                        // Single run with live timer
                        let timer = start_timer("TS:", "cyan");
                        let result = rt.run_benchmark(&spec_clone, suite).await;
                        stop_timer(&timer);

                        match result {
                            Ok(m) => {
                                let elapsed = lang_start.elapsed();
                                print!(
                                    "\r    {} {} ({})                    ",
                                    "TS:".cyan(),
                                    Measurement::format_duration(m.nanos_per_op),
                                    format!("{:.2}s", elapsed.as_secs_f64()).dimmed()
                                );
                                measurements.insert(Lang::TypeScript, m);
                            }
                            Err(e) => {
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

                            match rt.run_benchmark(&spec_clone, suite).await {
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
                                "\r    {} {}{} ({}x runs, {:.2}s)                    ",
                                "Rust:".yellow(),
                                Measurement::format_duration(aggregated.nanos_per_op),
                                ci_str,
                                spec_clone.count,
                                elapsed.as_secs_f64()
                            );
                            measurements.insert(Lang::Rust, aggregated);
                        }
                    } else {
                        // Single run with live timer
                        let timer = start_timer("Rust:", "yellow");
                        let result = rt.run_benchmark(&spec_clone, suite).await;
                        stop_timer(&timer);

                        match result {
                            Ok(m) => {
                                let elapsed = lang_start.elapsed();
                                print!(
                                    "\r    {} {} ({})                    ",
                                    "Rust:".yellow(),
                                    Measurement::format_duration(m.nanos_per_op),
                                    format!("{:.2}s", elapsed.as_secs_f64()).dimmed()
                                );
                                measurements.insert(Lang::Rust, m);
                            }
                            Err(e) => {
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
                spec.description.clone(),
                measurements,
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
    }

    Ok(BenchmarkResults::new(suite_results))
}
