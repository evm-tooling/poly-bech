//! Markdown report generator

use miette::Result;
use poly_bench_dsl::Lang;
use poly_bench_executor::BenchmarkResults;
use poly_bench_runtime::{
    lang_full_name, lang_icon, measurement::Measurement, supported_languages,
};

/// Generate markdown report
pub fn report(results: &BenchmarkResults) -> Result<String> {
    let mut md = String::new();

    // Title
    md.push_str("# Benchmark Report\n\n");
    md.push_str(&format!("Generated: {}\n\n", chrono_lite()));

    // Overall Summary
    md.push_str("## Overall Summary\n\n");

    let summary = &results.summary;

    if summary.winner.is_some() {
        md.push_str(&format!("**🏆 {}**\n\n", summary.winner_description));
    } else {
        md.push_str(&format!("**🤝 {}**\n\n", summary.winner_description));
    }

    md.push_str("| Metric | Value |\n");
    md.push_str("|--------|-------|\n");
    md.push_str(&format!("| Total Suites | {} |\n", summary.total_suites));
    md.push_str(&format!("| Total Benchmarks | {} |\n", summary.total_benchmarks));
    for lang in supported_languages() {
        let wins = summary.lang_wins.get(lang).copied().unwrap_or(0) as usize;
        md.push_str(&format!(
            "| {} Wins | {} ({}%) |\n",
            lang_full_name(*lang),
            wins,
            (wins * 100) / summary.total_benchmarks.max(1)
        ));
    }
    md.push_str(&format!(
        "| Ties | {} ({}%) |\n",
        summary.ties,
        (summary.ties * 100) / summary.total_benchmarks.max(1)
    ));
    md.push_str(&format!("| Geometric Mean Speedup | {:.2}x |\n\n", summary.geo_mean_speedup));

    // Suite Results
    md.push_str("## Suite Results\n\n");

    for suite in &results.suites {
        let icon = suite.summary.winner.map(lang_icon).unwrap_or("⚪");

        md.push_str(&format!(
            "### {} {} ({:.2}x avg)\n\n",
            icon, suite.name, suite.summary.geo_mean_speedup
        ));

        if let Some(ref desc) = suite.description {
            md.push_str(&format!("_{}_\n\n", desc));
        }

        // Determine which languages are present in this suite
        let present_langs: Vec<Lang> = supported_languages()
            .iter()
            .copied()
            .filter(|lang| suite.benchmarks.iter().any(|b| b.measurements.contains_key(lang)))
            .collect();

        // Build header based on present languages
        let mut header = "| Benchmark |".to_string();
        for lang in &present_langs {
            header.push_str(&format!(" {} |", lang_full_name(*lang)));
        }
        header.push_str(" Result |\n");
        md.push_str(&header);

        let mut sep = "|-----------|".to_string();
        for _ in &present_langs {
            sep.push_str("------------|");
        }
        sep.push_str("--------|\n");
        md.push_str(&sep);

        for bench in &suite.benchmarks {
            let mut cells: Vec<String> = vec![bench.name.clone()];
            for lang in &present_langs {
                let cell = bench
                    .measurements
                    .get(lang)
                    .map(|m| Measurement::format_duration(m.nanos_per_op))
                    .unwrap_or_else(|| "-".to_string());
                cells.push(cell);
            }

            // Determine winner from all measurements (lower is better)
            let mut times: Vec<(Lang, f64)> = bench
                .measurements
                .iter()
                .filter_map(|(lang, m)| Some((*lang, m.nanos_per_op)))
                .collect();
            times.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            let result_str = if times.len() >= 2 {
                let (best_lang, best_time) = times[0];
                let (_, second_time) = times[1];
                let speedup = second_time / best_time.max(1e-9);
                let icon = lang_icon(best_lang);
                if speedup < 1.05 {
                    "⚪ Similar".to_string()
                } else {
                    format!("{} {} {:.2}x faster", icon, lang_full_name(best_lang), speedup)
                }
            } else if let Some(ref cmp) = bench.comparison {
                let icon = match cmp.winner {
                    poly_bench_runtime::measurement::ComparisonWinner::First => "🟢",
                    poly_bench_runtime::measurement::ComparisonWinner::Second => "🔵",
                    poly_bench_runtime::measurement::ComparisonWinner::Tie => "⚪",
                };
                format!("{} {}", icon, cmp.speedup_description())
            } else {
                "-".to_string()
            };
            cells.push(result_str);

            let row: String = cells.iter().map(|c| format!(" {} |", c)).collect();
            md.push_str(&format!("|{}\n", row));
        }

        md.push_str("\n");
    }

    // Legend
    md.push_str("## Legend\n\n");
    for lang in supported_languages() {
        md.push_str(&format!("- {} {} faster\n", lang_icon(*lang), lang_full_name(*lang)));
    }
    md.push_str("- ⚪ Similar (within 5%)\n");
    md.push_str("- ns/op = nanoseconds per operation (lower is better)\n");

    Ok(md)
}

/// Simple timestamp without chrono dependency
fn chrono_lite() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();

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

    format!("{}-{:02}-{:02}T{:02}:{:02}:{:02}Z", years, months, day, hours, minutes, seconds)
}
