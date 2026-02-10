//! Benchmark execution scheduler

use crate::dsl::Lang;
use crate::ir::BenchmarkIR;
use crate::runtime::go::GoRuntime;
use crate::runtime::js::JsRuntime;
use crate::runtime::measurement::Measurement;
use crate::runtime::traits::Runtime;
use crate::executor::comparison::{BenchmarkResults, SuiteResults, BenchmarkResult};
use super::ProjectRoots;
use colored::Colorize;
use miette::Result;
use std::collections::HashMap;

/// Run all benchmarks in the IR
pub async fn run(
    ir: &BenchmarkIR,
    langs: &[Lang],
    iterations_override: Option<u64>,
    project_roots: &ProjectRoots,
) -> Result<BenchmarkResults> {
    let mut suite_results = Vec::new();

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

            print!("  {} {} ", "→".dimmed(), spec.name);

            let mut measurements: HashMap<Lang, Measurement> = HashMap::new();

            // Run Go benchmark
            if spec.has_lang(Lang::Go) {
                if let Some(ref mut rt) = go_runtime {
                    match rt.run_benchmark(&spec_clone, suite).await {
                        Ok(m) => {
                            print!("{} ", format!("Go: {}", Measurement::format_duration(m.nanos_per_op)).green());
                            measurements.insert(Lang::Go, m);
                        }
                        Err(e) => {
                            print!("{} ", format!("Go: {}", e).red());
                        }
                    }
                }
            }

            // Run TypeScript benchmark
            if spec.has_lang(Lang::TypeScript) {
                if let Some(ref mut rt) = js_runtime {
                    match rt.run_benchmark(&spec_clone, suite).await {
                        Ok(m) => {
                            print!("{} ", format!("TS: {}", Measurement::format_duration(m.nanos_per_op)).cyan());
                            measurements.insert(Lang::TypeScript, m);
                        }
                        Err(e) => {
                            print!("{} ", format!("TS: {}", e).red());
                        }
                    }
                }
            }

            // Show comparison if both ran
            if let (Some(go_m), Some(ts_m)) = (
                measurements.get(&Lang::Go),
                measurements.get(&Lang::TypeScript),
            ) {
                let ratio = go_m.nanos_per_op / ts_m.nanos_per_op;
                let (winner, speedup) = if (ratio - 1.0).abs() < 0.05 {
                    ("tie".dimmed().to_string(), 1.0)
                } else if ratio > 1.0 {
                    (format!("TS {}x faster", format!("{:.2}", ratio)).cyan().to_string(), ratio)
                } else {
                    (format!("Go {}x faster", format!("{:.2}", 1.0 / ratio)).green().to_string(), 1.0 / ratio)
                };
                print!("[{}]", winner);
            }

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

        suite_results.push(SuiteResults::new(
            suite.name.clone(),
            suite.description.clone(),
            benchmark_results,
        ));
    }

    Ok(BenchmarkResults::new(suite_results))
}
