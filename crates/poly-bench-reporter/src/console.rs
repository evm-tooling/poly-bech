//! Console output reporter

use poly_bench_dsl::Lang;
use poly_bench_executor::{BenchmarkResults, SuiteResults};
use poly_bench_runtime::measurement::Measurement;
use colored::Colorize;
use miette::Result;

/// Generate console report
pub fn report(results: &BenchmarkResults) -> Result<()> {
    println!("\n{}", "═".repeat(80));
    println!("{}", "  BENCHMARK RESULTS".bold());
    println!("{}\n", "═".repeat(80));

    // Overall summary
    println!("{}", "OVERALL SUMMARY".bold().underline());
    println!();
    
    let summary = &results.summary;
    
    // Winner banner
    match summary.winner {
        Some(Lang::Go) => {
            println!("  {} {}", "🏆".green(), summary.winner_description.green().bold());
        }
        Some(Lang::TypeScript) => {
            println!("  {} {}", "🏆".cyan(), summary.winner_description.cyan().bold());
        }
        _ => {
            println!("  {} {}", "🤝", summary.winner_description.dimmed());
        }
    }
    println!();

    // Stats table
    println!("  {:<20} {}", "Total Suites:", summary.total_suites);
    println!("  {:<20} {}", "Total Benchmarks:", summary.total_benchmarks);
    println!("  {:<20} {} ({}%)", 
        "Go Wins:", 
        summary.go_wins,
        (summary.go_wins * 100) / summary.total_benchmarks.max(1)
    );
    println!("  {:<20} {} ({}%)", 
        "TypeScript Wins:", 
        summary.ts_wins,
        (summary.ts_wins * 100) / summary.total_benchmarks.max(1)
    );
    println!("  {:<20} {} ({}%)", 
        "Ties:", 
        summary.ties,
        (summary.ties * 100) / summary.total_benchmarks.max(1)
    );
    println!("  {:<20} {:.2}x", "Geometric Mean:", summary.geo_mean_speedup);
    println!();

    // Suite details
    println!("{}", "SUITE RESULTS".bold().underline());
    println!();

    for suite in &results.suites {
        print_suite(suite);
    }

    // Legend
    println!("{}", "─".repeat(80));
    println!("{}", "LEGEND".dimmed());
    println!("  {} Go faster  |  {} TS faster  |  {} Similar (±5%)",
        "🟢".green(),
        "🔵".cyan(),
        "⚪".dimmed()
    );
    println!("  ns/op = nanoseconds per operation (lower is better)");
    println!();

    Ok(())
}

fn print_suite(suite: &SuiteResults) {
    let icon = match suite.summary.winner {
        Some(Lang::Go) => "🟢",
        Some(Lang::TypeScript) => "🔵",
        _ => "⚪",
    };

    println!("  {} {} ({})", 
        icon,
        suite.name.bold(),
        format!("{:.2}x avg speedup", suite.summary.geo_mean_speedup).dimmed()
    );

    if let Some(ref desc) = suite.description {
        println!("    {}", desc.dimmed());
    }

    // Results table header
    println!();
    println!("    {:<30} {:>15} {:>15} {:>20}",
        "Benchmark".underline(),
        "Go".underline(),
        "TypeScript".underline(),
        "Result".underline()
    );

    for bench in &suite.benchmarks {
        let go_str = bench.measurements.get(&Lang::Go)
            .map(|m| Measurement::format_duration(m.nanos_per_op))
            .unwrap_or_else(|| "-".to_string());

        let ts_str = bench.measurements.get(&Lang::TypeScript)
            .map(|m| Measurement::format_duration(m.nanos_per_op))
            .unwrap_or_else(|| "-".to_string());

        let result_str = if let Some(ref cmp) = bench.comparison {
            cmp.speedup_description()
        } else {
            "-".to_string()
        };

        let result_colored = if let Some(ref cmp) = bench.comparison {
            match cmp.winner {
                poly_bench_runtime::measurement::ComparisonWinner::First => 
                    result_str.green().to_string(),
                poly_bench_runtime::measurement::ComparisonWinner::Second => 
                    result_str.cyan().to_string(),
                poly_bench_runtime::measurement::ComparisonWinner::Tie => 
                    result_str.dimmed().to_string(),
            }
        } else {
            result_str.dimmed().to_string()
        };

        println!("    {:<30} {:>15} {:>15} {:>20}",
            bench.name,
            go_str.green(),
            ts_str.cyan(),
            result_colored
        );
    }

    println!();
}
