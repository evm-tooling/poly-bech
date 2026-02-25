//! SVG data table generator with conditional formatting

use poly_bench_dsl::{BenchmarkKind, Lang, SuiteType};
use poly_bench_executor::comparison::BenchmarkResult;
use poly_bench_ir::ChartDirectiveIR;
use poly_bench_runtime::{lang_color, lang_label, measurement::Measurement, supported_languages};

use super::{
    escape_xml, filter_benchmarks, format_duration, sort_benchmarks, BG_COLOR, BORDER_COLOR,
    TEXT_COLOR, TEXT_MUTED, TEXT_SECONDARY,
};

const ROW_HEIGHT: i32 = 32;
const HEADER_HEIGHT: i32 = 40;
const CELL_PADDING: i32 = 12;
const MIN_COL_WIDTH: i32 = 100;

/// Generate an SVG data table with conditional formatting
pub fn generate(
    benchmarks: Vec<&BenchmarkResult>,
    directive: &ChartDirectiveIR,
    suite_type: SuiteType,
) -> String {
    let mut filtered = filter_benchmarks(benchmarks, directive);
    sort_benchmarks(&mut filtered, directive);

    if filtered.is_empty() {
        return String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"400\" height=\"100\"><text x=\"200\" y=\"50\" text-anchor=\"middle\" font-family=\"sans-serif\">No benchmark data available</text></svg>");
    }

    // Build column structure: (label, lang_opt, width) - lang_opt is Some(lang) for lang columns
    let mut columns: Vec<(&'static str, Option<Lang>, i32)> = vec![("Benchmark", None, 150)];
    for lang in supported_languages() {
        if filtered.iter().any(|b| b.measurements.contains_key(lang)) {
            columns.push((lang_label(*lang), Some(*lang), MIN_COL_WIDTH));
        }
    }
    columns.push(("Winner", None, 80));

    let _num_cols = columns.len();
    let total_width: i32 = columns.iter().map(|(_, _, w)| w).sum();
    let chart_width = directive.width.unwrap_or(total_width + 40);
    let num_rows = filtered.len();
    let chart_height = directive.height.unwrap_or(
        HEADER_HEIGHT + (num_rows as i32 * ROW_HEIGHT) + 60, // Extra for title
    );

    let mut svg = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n",
        chart_width, chart_height, chart_width, chart_height
    );

    // Background
    svg.push_str(&format!(
        "<rect width=\"{}\" height=\"{}\" fill=\"{}\" rx=\"8\"/>\n",
        chart_width, chart_height, BG_COLOR
    ));
    svg.push_str(&format!(
        "<rect x=\".5\" y=\".5\" width=\"{}\" height=\"{}\" fill=\"none\" stroke=\"{}\" rx=\"8\"/>\n",
        chart_width - 1, chart_height - 1, BORDER_COLOR
    ));

    // Title
    let title = directive.title.as_deref().unwrap_or("Benchmark Results");
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"25\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"14\" font-weight=\"700\" fill=\"{}\">{}</text>\n",
        chart_width / 2, TEXT_COLOR, escape_xml(title)
    ));

    let table_y = 40;
    let table_x = 20;

    // Draw header row
    svg.push_str(&format!(
        "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#F3F4F6\" rx=\"4\"/>\n",
        table_x, table_y, total_width, HEADER_HEIGHT
    ));

    let mut x = table_x;
    for (label, lang_opt, width) in &columns {
        let color = lang_opt.map_or(TEXT_COLOR, |l| lang_color(l));
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"12\" font-weight=\"600\" fill=\"{}\">{}</text>\n",
            x + CELL_PADDING, table_y + HEADER_HEIGHT / 2 + 4, color, escape_xml(label)
        ));
        x += width;
    }

    // Draw data rows
    for (row_idx, bench) in filtered.iter().enumerate() {
        let row_y = table_y + HEADER_HEIGHT + (row_idx as i32 * ROW_HEIGHT);

        // Alternating row background
        if row_idx % 2 == 1 {
            svg.push_str(&format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#F9FAFB\"/>\n",
                table_x, row_y, total_width, ROW_HEIGHT
            ));
        }

        // Row border
        svg.push_str(&format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-opacity=\"0.3\"/>\n",
            table_x, row_y + ROW_HEIGHT, table_x + total_width, row_y + ROW_HEIGHT, BORDER_COLOR
        ));

        // Determine winner for this row (lower is better)
        let primary_value = |m: &Measurement| -> Option<f64> {
            if suite_type == SuiteType::Memory {
                m.bytes_per_op.map(|b| b as f64)
            } else {
                Some(m.nanos_per_op)
            }
        };

        let mut values: Vec<(Lang, f64)> = Vec::new();
        for lang in supported_languages() {
            if let Some(v) = bench.measurements.get(lang).and_then(primary_value) {
                values.push((*lang, v));
            }
        }

        let winner = if values.len() >= 2 {
            values.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            let (best_lang, best_val) = values[0];
            let (_, second_val) = values[1];
            let ratio = second_val / best_val.max(1e-9);
            if ratio < 1.05 {
                None // Tie
            } else {
                Some(best_lang)
            }
        } else {
            None
        };

        // Draw cells
        let mut x = table_x;
        for (col_label, lang_opt, width) in &columns {
            let text_y = row_y + ROW_HEIGHT / 2 + 4;

            match lang_opt {
                None => {
                    if *col_label == "Benchmark" {
                        svg.push_str(&format!(
                            "<text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"11\" font-weight=\"500\" fill=\"{}\">{}</text>\n",
                            x + CELL_PADDING, text_y, TEXT_COLOR, escape_xml(&bench.name)
                        ));
                    } else {
                        let (label, color) = winner
                            .map(|l| (lang_label(l), lang_color(l)))
                            .unwrap_or(("Tie", TEXT_MUTED));
                        svg.push_str(&format!(
                            "<text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"11\" font-weight=\"600\" fill=\"{}\">{}</text>\n",
                            x + CELL_PADDING, text_y, color, label
                        ));
                    }
                }
                Some(lang) => {
                    if let Some(m) = bench.measurements.get(lang) {
                        let is_winner = winner == Some(*lang);
                        let bg_color = if is_winner { "#DCFCE7" } else { "transparent" };
                        let text_color = if is_winner { "#166534" } else { TEXT_SECONDARY };

                        if is_winner {
                            svg.push_str(&format!(
                                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\"/>\n",
                                x, row_y, width, ROW_HEIGHT, bg_color
                            ));
                        }

                        let cell_text = if suite_type == SuiteType::Memory {
                            m.bytes_per_op
                                .map(|b| Measurement::format_bytes(b))
                                .unwrap_or_else(|| "—".to_string())
                        } else {
                            format_duration(m.nanos_per_op)
                        };
                        svg.push_str(&format!(
                            "<text x=\"{}\" y=\"{}\" font-family=\"monospace\" font-size=\"11\" fill=\"{}\">{}</text>\n",
                            x + CELL_PADDING, text_y, text_color, cell_text
                        ));
                    }
                }
            }

            x += width;
        }
    }

    // Footer metadata for async charts
    let has_async = filtered.iter().any(|b| b.kind == BenchmarkKind::Async);
    if has_async {
        let mut warmup_cap = None;
        let mut sample_cap = None;
        let mut sampling_policy = None;
        for bench in &filtered {
            if let Some(details) = &bench.async_details {
                warmup_cap.get_or_insert(details.warmup_cap);
                sample_cap.get_or_insert(details.sample_cap);
                sampling_policy.get_or_insert(details.sampling_policy.as_str());
            }
        }
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"10\" fill=\"{}\">async-sequential benchmarks present (policy: {}, warmup<={}, samples<={})</text>\n",
            chart_width / 2,
            chart_height - 10,
            TEXT_MUTED,
            sampling_policy.unwrap_or("timeBudgeted"),
            warmup_cap.unwrap_or(5),
            sample_cap.unwrap_or(50)
        ));
    }

    svg.push_str("</svg>\n");
    svg
}
