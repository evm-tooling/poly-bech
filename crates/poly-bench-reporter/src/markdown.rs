//! Markdown report generator

use miette::Result;
use poly_bench_dsl::Lang;
use poly_bench_executor::BenchmarkResults;
use poly_bench_runtime::measurement::Measurement;

/// Generate markdown report
pub fn report(results: &BenchmarkResults) -> Result<String> {
    let mut md = String::new();

    // Title
    md.push_str("# Benchmark Report\n\n");
    md.push_str(&format!("Generated: {}\n\n", chrono_lite()));

    // Overall Summary
    md.push_str("## Overall Summary\n\n");

    let summary = &results.summary;

    match summary.winner {
        Some(Lang::Go) => {
            md.push_str(&format!("**🏆 {}**\n\n", summary.winner_description));
        }
        Some(Lang::TypeScript) => {
            md.push_str(&format!("**🏆 {}**\n\n", summary.winner_description));
        }
        _ => {
            md.push_str(&format!("**🤝 {}**\n\n", summary.winner_description));
        }
    }

    md.push_str("| Metric | Value |\n");
    md.push_str("|--------|-------|\n");
    md.push_str(&format!("| Total Suites | {} |\n", summary.total_suites));
    md.push_str(&format!(
        "| Total Benchmarks | {} |\n",
        summary.total_benchmarks
    ));
    md.push_str(&format!(
        "| Go Wins | {} ({}%) |\n",
        summary.go_wins,
        (summary.go_wins * 100) / summary.total_benchmarks.max(1)
    ));
    md.push_str(&format!(
        "| TypeScript Wins | {} ({}%) |\n",
        summary.ts_wins,
        (summary.ts_wins * 100) / summary.total_benchmarks.max(1)
    ));
    md.push_str(&format!(
        "| Ties | {} ({}%) |\n",
        summary.ties,
        (summary.ties * 100) / summary.total_benchmarks.max(1)
    ));
    md.push_str(&format!(
        "| Geometric Mean Speedup | {:.2}x |\n\n",
        summary.geo_mean_speedup
    ));

    // Suite Results
    md.push_str("## Suite Results\n\n");

    for suite in &results.suites {
        let icon = match suite.summary.winner {
            Some(Lang::Go) => "🟢",
            Some(Lang::TypeScript) => "🔵",
            _ => "⚪",
        };

        md.push_str(&format!(
            "### {} {} ({:.2}x avg)\n\n",
            icon, suite.name, suite.summary.geo_mean_speedup
        ));

        if let Some(ref desc) = suite.description {
            md.push_str(&format!("_{}_\n\n", desc));
        }

        md.push_str("| Benchmark | Go | TypeScript | Result |\n");
        md.push_str("|-----------|-----|------------|--------|\n");

        for bench in &suite.benchmarks {
            let go_str = bench
                .measurements
                .get(&Lang::Go)
                .map(|m| Measurement::format_duration(m.nanos_per_op))
                .unwrap_or_else(|| "-".to_string());

            let ts_str = bench
                .measurements
                .get(&Lang::TypeScript)
                .map(|m| Measurement::format_duration(m.nanos_per_op))
                .unwrap_or_else(|| "-".to_string());

            let result_str = if let Some(ref cmp) = bench.comparison {
                let icon = match cmp.winner {
                    poly_bench_runtime::measurement::ComparisonWinner::First => "🟢",
                    poly_bench_runtime::measurement::ComparisonWinner::Second => "🔵",
                    poly_bench_runtime::measurement::ComparisonWinner::Tie => "⚪",
                };
                format!("{} {}", icon, cmp.speedup_description())
            } else {
                "-".to_string()
            };

            md.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                bench.name, go_str, ts_str, result_str
            ));
        }

        md.push_str("\n");
    }

    // Legend
    md.push_str("## Legend\n\n");
    md.push_str("- 🟢 Go faster\n");
    md.push_str("- 🔵 TypeScript faster\n");
    md.push_str("- ⚪ Similar (within 5%)\n");
    md.push_str("- ns/op = nanoseconds per operation (lower is better)\n");

    Ok(md)
}

/// Simple timestamp without chrono dependency
fn chrono_lite() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();

    let secs = duration.as_secs();

    // Calculate date components (simplified)
    let days = secs / 86400;
    let years = 1970 + days / 365;
    let remaining_days = days % 365;
    let months = remaining_days / 30 + 1;
    let day = remaining_days % 30 + 1;

    let time_secs = secs % 86400;
    let hours = time_secs / 3600;
    let minutes = (time_secs % 3600) / 60;
    let seconds = time_secs % 60;

    format!(
        "{}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        years, months, day, hours, minutes, seconds
    )
}
