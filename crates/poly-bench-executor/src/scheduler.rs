//! Benchmark execution scheduler

use poly_bench_dsl::{Lang, BenchMode};
use poly_bench_ir::BenchmarkIR;
use poly_bench_runtime::go::GoRuntime;
use poly_bench_runtime::js::JsRuntime;
use poly_bench_runtime::measurement::Measurement;
use poly_bench_runtime::traits::Runtime;
use crate::comparison::{BenchmarkResults, SuiteResults, BenchmarkResult};
use super::{ProjectRoots, AnvilService, AnvilConfig};
use colored::Colorize;
use miette::Result;
use std::collections::HashMap;
use std::time::Instant;

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

        let mut benchmark_results = Vec::new();

        // Initialize runtimes
        let mut go_runtime: Option<GoRuntime> = None;
        let mut js_runtime: Option<JsRuntime> = None;

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

        // Run each benchmark
        for spec in &suite.benchmarks {
            let mut spec_clone = spec.clone();
            if let Some(override_iters) = iterations_override {
                spec_clone.iterations = override_iters;
            }

            // Print benchmark args
            let mode_str = match spec.mode {
                BenchMode::Auto => format!("auto, targetTime={}ms", spec.target_time_ms),
                BenchMode::Fixed => format!("fixed, iterations={}", spec.iterations),
            };
            println!("  {} {} [{}]", "→".dimmed(), spec.name.bold(), mode_str.dimmed());

            let mut measurements: HashMap<Lang, Measurement> = HashMap::new();
            let bench_start = Instant::now();

            // Run Go benchmark
            if spec.has_lang(Lang::Go) {
                if let Some(ref mut rt) = go_runtime {
                    let lang_start = Instant::now();
                    match rt.run_benchmark(&spec_clone, suite).await {
                        Ok(m) => {
                            let elapsed = lang_start.elapsed();
                            print!("    {} {} ({})", 
                                "Go:".green(), 
                                Measurement::format_duration(m.nanos_per_op),
                                format!("{:.2}s", elapsed.as_secs_f64()).dimmed()
                            );
                            measurements.insert(Lang::Go, m);
                        }
                        Err(e) => {
                            print!("    {} {}", "Go:".red(), format!("{}", e).red());
                        }
                    }
                    println!();
                }
            }

            // Run TypeScript benchmark
            if spec.has_lang(Lang::TypeScript) {
                if let Some(ref mut rt) = js_runtime {
                    let lang_start = Instant::now();
                    match rt.run_benchmark(&spec_clone, suite).await {
                        Ok(m) => {
                            let elapsed = lang_start.elapsed();
                            print!("    {} {} ({})",
                                "TS:".cyan(),
                                Measurement::format_duration(m.nanos_per_op),
                                format!("{:.2}s", elapsed.as_secs_f64()).dimmed()
                            );
                            measurements.insert(Lang::TypeScript, m);
                        }
                        Err(e) => {
                            print!("    {} {}", "TS:".red(), format!("{}", e).red());
                        }
                    }
                    println!();
                }
            }

            let bench_elapsed = bench_start.elapsed();

            // Show comparison if both ran
            if let (Some(go_m), Some(ts_m)) = (
                measurements.get(&Lang::Go),
                measurements.get(&Lang::TypeScript),
            ) {
                let ratio = go_m.nanos_per_op / ts_m.nanos_per_op;
                let (winner, _speedup) = if (ratio - 1.0).abs() < 0.05 {
                    ("tie".dimmed().to_string(), 1.0)
                } else if ratio > 1.0 {
                    (format!("TS {}x faster", format!("{:.2}", ratio)).cyan().to_string(), ratio)
                } else {
                    (format!("Go {}x faster", format!("{:.2}", 1.0 / ratio)).green().to_string(), 1.0 / ratio)
                };
                println!("    {} [{}]", 
                    format!("total: {:.2}s", bench_elapsed.as_secs_f64()).dimmed(),
                    winner
                );
            } else {
                println!("    {}", format!("total: {:.2}s", bench_elapsed.as_secs_f64()).dimmed());
            }

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

        suite_results.push(SuiteResults::new(
            suite.name.clone(),
            suite.description.clone(),
            benchmark_results,
        ));
    }

    Ok(BenchmarkResults::new(suite_results))
}
