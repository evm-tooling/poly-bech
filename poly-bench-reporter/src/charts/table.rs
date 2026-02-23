//! SVG data table generator with conditional formatting

use poly_bench_dsl::{BenchmarkKind, Lang};
use poly_bench_executor::comparison::BenchmarkResult;
use poly_bench_ir::ChartDirectiveIR;

use super::{
    escape_xml, filter_benchmarks, format_duration, sort_benchmarks, BG_COLOR, BORDER_COLOR,
    GO_COLOR, RUST_COLOR, TEXT_COLOR, TEXT_MUTED, TEXT_SECONDARY, TS_COLOR,
};

const ROW_HEIGHT: i32 = 32;
const HEADER_HEIGHT: i32 = 40;
const CELL_PADDING: i32 = 12;
const MIN_COL_WIDTH: i32 = 100;

/// Generate an SVG data table with conditional formatting
pub fn generate(benchmarks: Vec<&BenchmarkResult>, directive: &ChartDirectiveIR) -> String {
    let mut filtered = filter_benchmarks(benchmarks, directive);
    sort_benchmarks(&mut filtered, directive);

    if filtered.is_empty() {
        return String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"400\" height=\"100\"><text x=\"200\" y=\"50\" text-anchor=\"middle\" font-family=\"sans-serif\">No benchmark data available</text></svg>");
    }

    // Determine which languages have data
    let has_go = filtered.iter().any(|b| b.measurements.contains_key(&Lang::Go));
    let has_ts = filtered.iter().any(|b| b.measurements.contains_key(&Lang::TypeScript));
    let has_rust = filtered.iter().any(|b| b.measurements.contains_key(&Lang::Rust));

    // Build column structure
    let mut columns: Vec<(&str, &str, i32)> = vec![("Benchmark", "name", 150)];
    if has_go {
        columns.push(("Go", "go", MIN_COL_WIDTH));
    }
    if has_ts {
        columns.push(("TypeScript", "ts", MIN_COL_WIDTH));
    }
    if has_rust {
        columns.push(("Rust", "rust", MIN_COL_WIDTH));
    }
    columns.push(("Winner", "winner", 80));

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
    for (label, id, width) in &columns {
        let color = match *id {
            "go" => GO_COLOR,
            "ts" => TS_COLOR,
            "rust" => RUST_COLOR,
            _ => TEXT_COLOR,
        };
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

        // Determine winner for this row
        let go_ns = bench.measurements.get(&Lang::Go).map(|m| m.nanos_per_op);
        let ts_ns = bench.measurements.get(&Lang::TypeScript).map(|m| m.nanos_per_op);
        let rust_ns = bench.measurements.get(&Lang::Rust).map(|m| m.nanos_per_op);

        let mut times: Vec<(Lang, f64)> = vec![];
        if let Some(ns) = go_ns {
            times.push((Lang::Go, ns));
        }
        if let Some(ns) = ts_ns {
            times.push((Lang::TypeScript, ns));
        }
        if let Some(ns) = rust_ns {
            times.push((Lang::Rust, ns));
        }

        let winner = if times.len() >= 2 {
            times.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            let (fastest_lang, fastest_time) = times[0];
            let (_, second_time) = times[1];
            let speedup = second_time / fastest_time;
            if speedup < 1.05 {
                None // Tie
            } else {
                Some(fastest_lang)
            }
        } else {
            None
        };

        // Draw cells
        let mut x = table_x;
        for (_, id, width) in &columns {
            let text_y = row_y + ROW_HEIGHT / 2 + 4;

            match *id {
                "name" => {
                    svg.push_str(&format!(
                        "<text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"11\" font-weight=\"500\" fill=\"{}\">{}</text>\n",
                        x + CELL_PADDING, text_y, TEXT_COLOR, escape_xml(&bench.name)
                    ));
                }
                "go" => {
                    if let Some(m) = bench.measurements.get(&Lang::Go) {
                        let is_winner = winner == Some(Lang::Go);
                        let bg_color = if is_winner { "#DCFCE7" } else { "transparent" };
                        let text_color = if is_winner { "#166534" } else { TEXT_SECONDARY };

                        if is_winner {
                            svg.push_str(&format!(
                                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\"/>\n",
                                x, row_y, width, ROW_HEIGHT, bg_color
                            ));
                        }

                        svg.push_str(&format!(
                            "<text x=\"{}\" y=\"{}\" font-family=\"monospace\" font-size=\"11\" fill=\"{}\">{}</text>\n",
                            x + CELL_PADDING, text_y, text_color, format_duration(m.nanos_per_op)
                        ));
                    }
                }
                "ts" => {
                    if let Some(m) = bench.measurements.get(&Lang::TypeScript) {
                        let is_winner = winner == Some(Lang::TypeScript);
                        let bg_color = if is_winner { "#DBEAFE" } else { "transparent" };
                        let text_color = if is_winner { "#1E40AF" } else { TEXT_SECONDARY };

                        if is_winner {
                            svg.push_str(&format!(
                                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\"/>\n",
                                x, row_y, width, ROW_HEIGHT, bg_color
                            ));
                        }

                        svg.push_str(&format!(
                            "<text x=\"{}\" y=\"{}\" font-family=\"monospace\" font-size=\"11\" fill=\"{}\">{}</text>\n",
                            x + CELL_PADDING, text_y, text_color, format_duration(m.nanos_per_op)
                        ));
                    }
                }
                "rust" => {
                    if let Some(m) = bench.measurements.get(&Lang::Rust) {
                        let is_winner = winner == Some(Lang::Rust);
                        let bg_color = if is_winner { "#FEF3C7" } else { "transparent" };
                        let text_color = if is_winner { "#92400E" } else { TEXT_SECONDARY };

                        if is_winner {
                            svg.push_str(&format!(
                                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\"/>\n",
                                x, row_y, width, ROW_HEIGHT, bg_color
                            ));
                        }

                        svg.push_str(&format!(
                            "<text x=\"{}\" y=\"{}\" font-family=\"monospace\" font-size=\"11\" fill=\"{}\">{}</text>\n",
                            x + CELL_PADDING, text_y, text_color, format_duration(m.nanos_per_op)
                        ));
                    }
                }
                "winner" => {
                    let (label, color) = match winner {
                        Some(Lang::Go) => ("Go", GO_COLOR),
                        Some(Lang::TypeScript) => ("TS", TS_COLOR),
                        Some(Lang::Rust) => ("Rust", RUST_COLOR),
                        _ => ("Tie", TEXT_MUTED),
                    };
                    svg.push_str(&format!(
                        "<text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"11\" font-weight=\"600\" fill=\"{}\">{}</text>\n",
                        x + CELL_PADDING, text_y, color, label
                    ));
                }
                _ => {}
            }

            x += width;
        }
    }

    // Footer metadata for async charts
    let has_async = filtered.iter().any(|b| b.kind == BenchmarkKind::Async);
    if has_async {
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"10\" fill=\"{}\">async-sequential benchmarks present (internal caps: warmup<=5, samples<=50)</text>\n",
            chart_width / 2,
            chart_height - 10,
            TEXT_MUTED
        ));
    }

    svg.push_str("</svg>\n");
    svg
}
