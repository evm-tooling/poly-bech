use poly_bench_dsl::Lang;
use poly_bench_executor::comparison::BenchmarkResult;
use poly_bench_ir::ChartDirectiveIR;
use poly_bench_runtime::measurement::Measurement;

use super::{
    compute_ci_bounds, escape_xml, extract_numeric_value, filter_benchmarks, lang_color, regression,
    sort_benchmarks,
};

const MARGIN_LEFT: f64 = 70.0;
const MARGIN_RIGHT: f64 = 30.0;
const MARGIN_TOP: f64 = 60.0;
const MARGIN_BOTTOM: f64 = 80.0;

#[derive(Clone, Copy)]
struct Theme {
    bg: &'static str,
    stroke: &'static str,
    text: &'static str,
    text_secondary: &'static str,
    text_muted: &'static str,
    grid: &'static str,
    plot_bg: &'static str,
    bar_outline: &'static str,
}

impl Theme {
    fn from_name(name: Option<&str>) -> Self {
        match name.map(|s| s.to_lowercase()).as_deref() {
            Some("light") => Self {
                bg: "#FFFFFF",
                stroke: "rgba(0,0,0,0.08)",
                text: "#1A1A1A",
                text_secondary: "rgba(0,0,0,0.7)",
                text_muted: "rgba(0,0,0,0.5)",
                grid: "rgba(0,0,0,0.08)",
                plot_bg: "rgba(0,0,0,0.02)",
                bar_outline: "rgba(0,0,0,0.1)",
            },
            _ => Self {
                bg: "#1E1E20",
                stroke: "rgba(255,255,255,0.12)",
                text: "#FFFFFF",
                text_secondary: "rgba(255,255,255,0.7)",
                text_muted: "rgba(255,255,255,0.45)",
                grid: "rgba(255,255,255,0.10)",
                plot_bg: "rgba(255,255,255,0.02)",
                bar_outline: "rgba(255,255,255,0.15)",
            },
        }
    }
}

pub fn generate(benchmarks: Vec<&BenchmarkResult>, directive: &ChartDirectiveIR) -> String {
    let mut filtered = filter_benchmarks(benchmarks, directive);
    sort_benchmarks(&mut filtered, directive);
    if filtered.is_empty() {
        return empty_chart("No benchmark data available");
    }

    let theme = Theme::from_name(directive.theme.as_deref());
    let width = directive.width.unwrap_or(980).max(520) as f64;
    let height = directive.height.unwrap_or(560).max(360) as f64;
    let plot_w = (width - MARGIN_LEFT - MARGIN_RIGHT).max(120.0);
    let plot_h = (height - MARGIN_TOP - MARGIN_BOTTOM).max(120.0);

    let langs = available_langs(&filtered);
    if langs.is_empty() {
        return empty_chart("No language measurements available");
    }

    let x_values: Vec<f64> = filtered
        .iter()
        .enumerate()
        .map(|(i, b)| extract_numeric_value(&b.name).map(|n| n as f64).unwrap_or((i + 1) as f64))
        .collect();
    let x_min = *x_values.first().unwrap_or(&1.0);
    let x_max = *x_values.last().unwrap_or(&x_min);
    let x_span = if (x_max - x_min).abs() < f64::EPSILON { 1.0 } else { x_max - x_min };

    let mut y_max = 0.0_f64;
    for bench in &filtered {
        for lang in &langs {
            if let Some(m) = bench.measurements.get(lang) {
                y_max = y_max.max(m.nanos_per_op);
                if directive.show_std_dev {
                    if let Some(sd) = m.std_dev_nanos {
                        y_max = y_max.max(m.nanos_per_op + sd);
                    }
                }
                if directive.show_error_bars {
                    if let (_, Some(upper)) =
                        compute_ci_bounds(m.nanos_per_op, m.raw_samples.as_ref(), 95, m.ci_95_lower, m.ci_95_upper)
                    {
                        y_max = y_max.max(upper);
                    }
                }
            }
        }
    }
    if y_max <= 0.0 {
        y_max = 1.0;
    }

    let x_to_px = |x: f64| MARGIN_LEFT + ((x - x_min) / x_span) * plot_w;
    let y_to_px = |y: f64| MARGIN_TOP + (1.0 - (y / y_max).clamp(0.0, 1.0)) * plot_h;

    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{:.0}\" height=\"{:.0}\" viewBox=\"0 0 {:.0} {:.0}\">\n",
        width, height, width, height
    ));
    svg.push_str(&format!(
        "<rect width=\"{:.0}\" height=\"{:.0}\" fill=\"{}\"/>\n",
        width, height, theme.bg
    ));
    svg.push_str(&format!(
        "<rect x=\"0.5\" y=\"0.5\" width=\"{:.0}\" height=\"{:.0}\" fill=\"none\" stroke=\"{}\" rx=\"10\"/>\n",
        width - 1.0,
        height - 1.0,
        theme.stroke
    ));
    svg.push_str(
        "<defs>\n\
<linearGradient id=\"goGrad\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">\n\
  <stop offset=\"0%\" stop-color=\"#00ADD8\" stop-opacity=\"0.95\"/>\n\
  <stop offset=\"100%\" stop-color=\"#0891B2\" stop-opacity=\"0.8\"/>\n\
</linearGradient>\n\
<linearGradient id=\"tsGrad\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">\n\
  <stop offset=\"0%\" stop-color=\"#3178C6\" stop-opacity=\"0.95\"/>\n\
  <stop offset=\"100%\" stop-color=\"#1D4ED8\" stop-opacity=\"0.8\"/>\n\
</linearGradient>\n\
<linearGradient id=\"rustGrad\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">\n\
  <stop offset=\"0%\" stop-color=\"#DEA584\" stop-opacity=\"0.95\"/>\n\
  <stop offset=\"100%\" stop-color=\"#B7410E\" stop-opacity=\"0.8\"/>\n\
</linearGradient>\n\
<filter id=\"barShadow\" x=\"-5%\" y=\"-15%\" width=\"110%\" height=\"140%\">\n\
  <feDropShadow dx=\"0\" dy=\"2\" stdDeviation=\"2\" flood-opacity=\"0.25\"/>\n\
</filter>\n\
</defs>\n",
    );
    svg.push_str(&format!(
        "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"8\" fill=\"{}\"/>\n",
        MARGIN_LEFT,
        MARGIN_TOP,
        plot_w,
        plot_h,
        theme.plot_bg
    ));

    for i in 0..=5 {
        let y = MARGIN_TOP + (i as f64 / 5.0) * plot_h;
        let val = y_max * (1.0 - (i as f64 / 5.0));
        svg.push_str(&format!(
            "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\"/>\n",
            MARGIN_LEFT,
            y,
            MARGIN_LEFT + plot_w,
            y,
            theme.grid
        ));
        svg.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\" font-family=\"sans-serif\" font-size=\"10\" fill=\"{}\">{:.0}</text>\n",
            MARGIN_LEFT - 8.0,
            y + 3.0,
            theme.text_muted,
            val
        ));
    }

    for (idx, bench) in filtered.iter().enumerate() {
        let x = x_to_px(x_values[idx]);
        svg.push_str(&format!(
            "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\"/>\n",
            x,
            MARGIN_TOP,
            x,
            MARGIN_TOP + plot_h,
            theme.grid
        ));
        svg.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"10\" fill=\"{}\">{}</text>\n",
            x,
            MARGIN_TOP + plot_h + 16.0,
            theme.text_muted,
            escape_xml(&bench.name)
        ));
    }

    for lang in &langs {
        let mut points: Vec<(f64, f64)> = Vec::new();
        let mut path = String::new();
        for (idx, bench) in filtered.iter().enumerate() {
            if let Some(m) = bench.measurements.get(lang) {
                let x = x_to_px(x_values[idx]);
                let y = y_to_px(m.nanos_per_op);
                points.push((x_values[idx], m.nanos_per_op));
                if path.is_empty() {
                    path.push_str(&format!("M {:.2} {:.2}", x, y));
                } else {
                    path.push_str(&format!(" L {:.2} {:.2}", x, y));
                }

                if directive.show_error_bars {
                    draw_error_bar(&mut svg, x, m, y_to_px, lang_color(*lang));
                }
                if directive.show_std_dev {
                    draw_std_dev(&mut svg, x, m, y_to_px, lang_color(*lang));
                }
                svg.push_str(&format!(
                    "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"3.5\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1.2\" filter=\"url(#barShadow)\"/>\n",
                    x,
                    y,
                    lang_color(*lang),
                    theme.bar_outline
                ));
            }
        }

        if !path.is_empty() {
            svg.push_str(&format!(
                "<path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"2\"/>\n",
                path,
                lang_color(*lang)
            ));
        }

        if directive.show_regression && points.len() >= 2 {
            if let Some(model) = regression::select_model(&points, Some(directive.regression_model.as_str()))
            {
                let mut reg_path = String::new();
                for step in 0..=80 {
                    let x_val = x_min + (x_span * step as f64 / 80.0);
                    let y_val = model.predict(x_val).max(0.0);
                    let x = x_to_px(x_val);
                    let y = y_to_px(y_val);
                    if reg_path.is_empty() {
                        reg_path.push_str(&format!("M {:.2} {:.2}", x, y));
                    } else {
                        reg_path.push_str(&format!(" L {:.2} {:.2}", x, y));
                    }
                }
                svg.push_str(&format!(
                    "<path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"1.5\" stroke-dasharray=\"4 3\" opacity=\"0.9\"/>\n",
                    reg_path,
                    lang_color(*lang)
                ));
                svg.push_str(&format!(
                    "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\" font-family=\"sans-serif\" font-size=\"9\" fill=\"{}\">{} regression ({:.3} R²)</text>\n",
                    MARGIN_LEFT + plot_w,
                    MARGIN_TOP + 12.0 + (lang_index(*lang) as f64 * 12.0),
                    theme.text_secondary,
                    escape_xml(lang_label(*lang)),
                    model.r_squared
                ));
            }
        }
    }

    svg.push_str(&format!(
        "<text x=\"{:.1}\" y=\"30\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"16\" font-weight=\"700\" fill=\"{}\">{}</text>\n",
        width / 2.0,
        theme.text,
        escape_xml(directive.title.as_deref().unwrap_or("Benchmark Trend Line"))
    ));
    svg.push_str(&format!(
        "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"10\" fill=\"{}\">Benchmark (natural order)</text>\n",
        MARGIN_LEFT + plot_w / 2.0,
        MARGIN_TOP + plot_h + 40.0,
        theme.text_muted
    ));
    svg.push_str(&format!(
        "<text x=\"18\" y=\"{:.1}\" transform=\"rotate(-90 18 {:.1})\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"10\" fill=\"{}\">nanos/op</text>\n",
        MARGIN_TOP + plot_h / 2.0,
        MARGIN_TOP + plot_h / 2.0,
        theme.text_muted
    ));
    svg.push_str(&legend_block(&langs, theme.text, theme.text_muted));
    svg.push_str("</svg>\n");
    svg
}

fn available_langs(benchmarks: &[&BenchmarkResult]) -> Vec<Lang> {
    [Lang::Go, Lang::TypeScript, Lang::Rust]
        .into_iter()
        .filter(|lang| benchmarks.iter().any(|b| b.measurements.contains_key(lang)))
        .collect()
}

fn draw_error_bar(
    svg: &mut String,
    x: f64,
    measurement: &Measurement,
    y_to_px: impl Fn(f64) -> f64,
    color: &str,
) {
    let (lower, upper) = compute_ci_bounds(
        measurement.nanos_per_op,
        measurement.raw_samples.as_ref(),
        95,
        measurement.ci_95_lower,
        measurement.ci_95_upper,
    );
    if let (Some(lo), Some(hi)) = (lower, upper) {
        let y1 = y_to_px(lo.max(0.0));
        let y2 = y_to_px(hi.max(0.0));
        svg.push_str(&format!(
            "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"{}\" stroke-width=\"1\"/>\n",
            x, y1, x, y2, color
        ));
        svg.push_str(&format!(
            "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"{}\" stroke-width=\"1\"/>\n",
            x - 4.0,
            y1,
            x + 4.0,
            y1,
            color
        ));
        svg.push_str(&format!(
            "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"{}\" stroke-width=\"1\"/>\n",
            x - 4.0,
            y2,
            x + 4.0,
            y2,
            color
        ));
    }
}

fn draw_std_dev(
    svg: &mut String,
    x: f64,
    measurement: &Measurement,
    y_to_px: impl Fn(f64) -> f64,
    color: &str,
) {
    if let Some(std_dev) = measurement.std_dev_nanos {
        let lower = (measurement.nanos_per_op - std_dev).max(0.0);
        let upper = measurement.nanos_per_op + std_dev;
        let y1 = y_to_px(lower);
        let y2 = y_to_px(upper);
        svg.push_str(&format!(
            "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"{}\" stroke-opacity=\"0.45\" stroke-width=\"2\"/>\n",
            x, y1, x, y2, color
        ));
    }
}

fn legend_block(langs: &[Lang], text: &str, text_muted: &str) -> String {
    let mut svg = String::new();
    svg.push_str(&format!(
        "<text x=\"70\" y=\"22\" font-family=\"sans-serif\" font-size=\"10\" fill=\"{}\">Legend:</text>\n",
        text_muted
    ));
    for (i, lang) in langs.iter().enumerate() {
        let x = 120.0 + (i as f64 * 120.0);
        svg.push_str(&format!(
            "<circle cx=\"{:.1}\" cy=\"18\" r=\"4\" fill=\"{}\"/>\n",
            x,
            lang_color(*lang)
        ));
        svg.push_str(&format!(
            "<text x=\"{:.1}\" y=\"22\" font-family=\"sans-serif\" font-size=\"10\" fill=\"{}\">{}</text>\n",
            x + 8.0,
            text,
            lang_label(*lang)
        ));
    }
    svg
}

fn lang_label(lang: Lang) -> &'static str {
    match lang {
        Lang::Go => "Go",
        Lang::TypeScript => "TypeScript",
        Lang::Rust => "Rust",
        _ => "Unknown",
    }
}

fn lang_index(lang: Lang) -> usize {
    match lang {
        Lang::Go => 0,
        Lang::TypeScript => 1,
        Lang::Rust => 2,
        _ => 3,
    }
}

fn empty_chart(message: &str) -> String {
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"420\" height=\"120\"><text x=\"210\" y=\"62\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"13\">{}</text></svg>",
        escape_xml(message)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use poly_bench_dsl::BenchmarkKind;
    use poly_bench_ir::ChartDirectiveIR;
    use poly_bench_runtime::measurement::Measurement;
    use std::collections::HashMap;

    fn bench(name: &str, go: f64, ts: f64) -> BenchmarkResult {
        let mut measurements = HashMap::new();
        measurements.insert(
            Lang::Go,
            Measurement {
                iterations: 100,
                total_nanos: (go * 100.0) as u64,
                nanos_per_op: go,
                ops_per_sec: 1_000_000_000.0 / go.max(1.0),
                min_nanos: None,
                max_nanos: None,
                p50_nanos: None,
                p75_nanos: None,
                p99_nanos: None,
                p995_nanos: None,
                rme_percent: None,
                samples: None,
                bytes_per_op: None,
                allocs_per_op: None,
                raw_samples: Some(vec![go as u64 - 10, go as u64, go as u64 + 10]),
                raw_result: None,
                successful_results: None,
                async_success_count: None,
                async_error_count: None,
                async_error_samples: None,
                cv_percent: None,
                outliers_removed: None,
                is_stable: None,
                run_count: Some(3),
                median_across_runs: None,
                ci_95_lower: Some(go - 10.0),
                ci_95_upper: Some(go + 10.0),
                std_dev_nanos: Some(10.0),
                estimator_source: None,
                raw_nanos_per_op: None,
                filtered_nanos_per_op: None,
                timed_out: None,
                run_nanos_per_op: None,
            },
        );
        measurements.insert(
            Lang::TypeScript,
            Measurement {
                iterations: 100,
                total_nanos: (ts * 100.0) as u64,
                nanos_per_op: ts,
                ops_per_sec: 1_000_000_000.0 / ts.max(1.0),
                min_nanos: None,
                max_nanos: None,
                p50_nanos: None,
                p75_nanos: None,
                p99_nanos: None,
                p995_nanos: None,
                rme_percent: None,
                samples: None,
                bytes_per_op: None,
                allocs_per_op: None,
                raw_samples: Some(vec![ts as u64 - 20, ts as u64, ts as u64 + 20]),
                raw_result: None,
                successful_results: None,
                async_success_count: None,
                async_error_count: None,
                async_error_samples: None,
                cv_percent: None,
                outliers_removed: None,
                is_stable: None,
                run_count: Some(3),
                median_across_runs: None,
                ci_95_lower: Some(ts - 20.0),
                ci_95_upper: Some(ts + 20.0),
                std_dev_nanos: Some(20.0),
                estimator_source: None,
                raw_nanos_per_op: None,
                filtered_nanos_per_op: None,
                timed_out: None,
                run_nanos_per_op: None,
            },
        );

        BenchmarkResult::new(
            name.to_string(),
            format!("suite_{}", name),
            BenchmarkKind::Sync,
            None,
            measurements,
            "legacy".to_string(),
            None,
            None,
            None,
            None,
        )
    }

    #[test]
    fn test_generate_line_chart_with_regression() {
        let b1 = bench("n10", 100.0, 140.0);
        let b2 = bench("n100", 500.0, 700.0);
        let b3 = bench("n1000", 2200.0, 3000.0);
        let mut directive = ChartDirectiveIR::new(
            poly_bench_dsl::ChartType::LineChart,
            "line.svg".to_string(),
        );
        directive.show_regression = true;
        directive.show_std_dev = true;
        directive.show_error_bars = true;
        directive.regression_model = "auto".to_string();
        let svg = generate(vec![&b1, &b2, &b3], &directive);
        assert!(svg.contains("<svg"));
        assert!(svg.contains("regression"));
    }
}
