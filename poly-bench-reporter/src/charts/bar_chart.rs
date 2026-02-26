use poly_bench_dsl::{Lang, SuiteType};
use poly_bench_executor::comparison::BenchmarkResult;
use poly_bench_ir::ChartDirectiveIR;
use poly_bench_runtime::measurement::Measurement;

use super::{
    axis_label_for_scale, compute_ci_bounds, derive_y_scale_params, escape_xml,
    extract_numeric_value, filter_benchmarks, generate_y_ticks, inverse_transform_y, lang_color,
    make_y_to_px,
    regression::{self, SelectedModel},
    sort_benchmarks, split_gap_bounds, y_upper_with_headroom, YAxisScale,
};

const MARGIN_LEFT: f64 = 90.0;
const MARGIN_RIGHT: f64 = 40.0;
const MARGIN_TOP: f64 = 84.0;
const MARGIN_BOTTOM: f64 = 24.0;
const X_AXIS_LABEL_OFFSET: f64 = 46.0;
const STATS_TOP_GAP: f64 = 18.0;
const STATS_HEADER_HEIGHT: f64 = 22.0;
const STATS_ROW_HEIGHT: f64 = 22.0;
const TARGET_BAR_WIDTH: f64 = 20.0;

#[derive(Clone, Copy)]
struct Theme {
    bg: &'static str,
    stroke: &'static str,
    text: &'static str,
    text_secondary: &'static str,
    text_muted: &'static str,
    grid: &'static str,
    plot_bg: &'static str,
    row_border: &'static str,
    bar_outline: &'static str,
    detail_box_fill: &'static str,
    detail_box_stroke: &'static str,
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
                row_border: "rgba(0,0,0,0.35)",
                bar_outline: "rgba(0,0,0,0.5)",
                detail_box_fill: "rgba(0,0,0,0.015)",
                detail_box_stroke: "rgba(0,0,0,0.14)",
            },
            _ => Self {
                bg: "#1E1E20",
                stroke: "rgba(255,255,255,0.12)",
                text: "#FFFFFF",
                text_secondary: "rgba(255,255,255,0.7)",
                text_muted: "rgba(255,255,255,0.45)",
                grid: "rgba(255,255,255,0.10)",
                plot_bg: "rgba(255,255,255,0.02)",
                bar_outline: "rgba(0,0,0,0.9)",
                row_border: "rgba(255,255,255,0.35)",
                detail_box_fill: "rgba(255,255,255,0.015)",
                detail_box_stroke: "rgba(255,255,255,0.14)",
            },
        }
    }
}

struct LangStats {
    lang: Lang,
    mean: f64,
    min: f64,
    max: f64,
    samples: usize,
    regression: Option<SelectedModel>,
}

pub fn generate(
    benchmarks: Vec<&BenchmarkResult>,
    directive: &ChartDirectiveIR,
    suite_type: SuiteType,
) -> String {
    let mut filtered = filter_benchmarks(benchmarks, directive);
    sort_benchmarks(&mut filtered, directive);
    if filtered.is_empty() {
        return empty_chart("No benchmark data available");
    }

    let langs = available_langs(&filtered);
    if langs.is_empty() {
        return empty_chart("No language measurements available");
    }
    let group_count = filtered.len().max(1) as f64;
    let lang_count = langs.len().max(1) as f64;
    let target_inter_gap = TARGET_BAR_WIDTH * 0.15;
    let target_bars_w =
        (lang_count * TARGET_BAR_WIDTH) + ((lang_count - 1.0).max(0.0) * target_inter_gap);
    let target_group_gap = (target_bars_w * 0.34).max(18.0);
    let min_plot_w = group_count * (target_bars_w + target_group_gap);
    let min_chart_w = (MARGIN_LEFT + MARGIN_RIGHT + min_plot_w).ceil();

    let theme = Theme::from_name(directive.theme.as_deref());
    let width = (directive.width.unwrap_or(980) as f64).max(560.0).max(min_chart_w);
    let height = directive.height.unwrap_or(680).max(620) as f64;
    let plot_w = (width - MARGIN_LEFT - MARGIN_RIGHT).max(120.0);
    let (_, _, stats_box_height) = stats_layout(langs.len(), plot_w);
    let plot_h = (height -
        MARGIN_TOP -
        MARGIN_BOTTOM -
        X_AXIS_LABEL_OFFSET -
        STATS_TOP_GAP -
        stats_box_height)
        .max(120.0);

    let is_memory = suite_type == SuiteType::Memory;
    let primary_value = |m: &Measurement| -> Option<f64> {
        if is_memory {
            m.bytes_per_op.map(|b| b as f64)
        } else {
            Some(m.nanos_per_op)
        }
    };

    let mut y_max = 0.0_f64;
    for bench in &filtered {
        for lang in &langs {
            if let Some(m) = bench.measurements.get(lang) {
                if let Some(v) = primary_value(m) {
                    if v.is_finite() && v < f64::MAX {
                        y_max = y_max.max(v);
                        if !is_memory && directive.show_std_dev {
                            if let Some(sd) = m.std_dev_nanos {
                                y_max = y_max.max(m.nanos_per_op + sd);
                            }
                        }
                        if !is_memory && directive.show_error_bars {
                            if let (_, Some(upper)) = compute_ci_bounds(
                                m.nanos_per_op,
                                m.raw_samples.as_ref(),
                                95,
                                m.ci_95_lower,
                                m.ci_95_upper,
                            ) {
                                y_max = y_max.max(upper);
                            }
                        }
                    }
                }
            }
        }
    }
    let scale = YAxisScale::from_str(Some(directive.y_scale.as_str()));
    if y_max <= 0.0 {
        y_max = 1.0;
    }
    let y_upper = y_upper_with_headroom(y_max, scale);
    let all_values: Vec<f64> = filtered
        .iter()
        .flat_map(|b| b.measurements.values().filter_map(|m| primary_value(m)))
        .filter(|v| v.is_finite() && *v < f64::MAX)
        .collect();
    let scale_params = derive_y_scale_params(&all_values, scale);
    let y_to_px = make_y_to_px(scale, 0.0, y_upper, MARGIN_TOP, plot_h, scale_params);

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
<linearGradient id=\"pythonGrad\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">\n\
  <stop offset=\"0%\" stop-color=\"#D8BD4A\" stop-opacity=\"1\"/>\n\
  <stop offset=\"100%\" stop-color=\"#EEDB7A\" stop-opacity=\"1\"/>\n\
</linearGradient>\n\
<linearGradient id=\"csharpGrad\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">\n\
  <stop offset=\"0%\" stop-color=\"#512BD4\" stop-opacity=\"1\"/>\n\
  <stop offset=\"100%\" stop-color=\"#7C3AED\" stop-opacity=\"1\"/>\n\
</linearGradient>\n\
<filter id=\"barShadow\" x=\"-5%\" y=\"-15%\" width=\"110%\" height=\"140%\">\n\
  <feDropShadow dx=\"0\" dy=\"2\" stdDeviation=\"2\" flood-opacity=\"0.25\"/>\n\
</filter>\n\
</defs>\n",
    );
    svg.push_str(&format!(
        "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"8\" fill=\"{}\"/>\n",
        MARGIN_LEFT, MARGIN_TOP, plot_w, plot_h, theme.plot_bg
    ));

    let ticks = generate_y_ticks(scale, 0.0, y_upper, true, scale_params);
    for (idx, (val, label)) in ticks.iter().enumerate() {
        let y = y_to_px(*val);
        svg.push_str(&format!(
            "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\"/>\n",
            MARGIN_LEFT,
            y,
            MARGIN_LEFT + plot_w,
            y,
            theme.grid
        ));
        svg.push_str(&format!(
            "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-width=\"1\"/>\n",
            MARGIN_LEFT - 6.0,
            y,
            MARGIN_LEFT,
            y,
            theme.text_muted
        ));
        let tick_label = if matches!(scale, YAxisScale::Linear) {
            if label.is_empty() {
                format!("{:.0}", val)
            } else {
                label.clone()
            }
        } else if should_show_non_linear_tick_label(idx, ticks.len(), label) {
            if label.is_empty() {
                format_axis_tick(*val)
            } else {
                label.clone()
            }
        } else {
            String::new()
        };
        if tick_label.is_empty() {
            continue;
        }
        svg.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\" font-family=\"sans-serif\" font-size=\"13\" fill=\"{}\">{}</text>\n",
            MARGIN_LEFT - 8.0,
            y + 4.0,
            theme.text_muted,
            escape_xml(&tick_label)
        ));
    }

    let group_slot_w = (plot_w / group_count.max(1.0)).max(36.0);
    let group_gap = (group_slot_w * 0.34).max(18.0);
    let group_inner_w = (group_slot_w - group_gap).max(18.0);
    let compact_denom = lang_count + 1.0 + ((lang_count - 1.0).max(0.0) * 0.22);
    let mut bar_w = ((group_inner_w / compact_denom) * 1.3).max(10.0);
    let mut inter_bar_gap = (bar_w * 0.22).max(2.0);
    let mut bars_total_w = (lang_count * bar_w) + ((lang_count - 1.0).max(0.0) * inter_bar_gap);
    if bars_total_w > group_inner_w {
        let scale = group_inner_w / bars_total_w;
        bar_w *= scale;
        inter_bar_gap *= scale;
        bars_total_w = (lang_count * bar_w) + ((lang_count - 1.0).max(0.0) * inter_bar_gap);
    }

    for (i, bench) in filtered.iter().enumerate() {
        let group_x = MARGIN_LEFT + i as f64 * group_slot_w;
        let group_inner_x = group_x + group_gap / 2.0;
        let bar_start_x = group_inner_x + ((group_inner_w - bars_total_w) / 2.0).max(0.0);
        for (j, lang) in langs.iter().enumerate() {
            if let Some(m) = bench.measurements.get(lang) {
                let Some(v) = primary_value(m) else { continue };
                if !v.is_finite() || v >= f64::MAX {
                    continue;
                }
                let x = bar_start_x + j as f64 * (bar_w + inter_bar_gap);
                let y = y_to_px(v);
                let h = (MARGIN_TOP + plot_h - y).max(1.0);
                let gradient = poly_bench_runtime::lang_gradient_id(*lang);
                svg.push_str(&format!(
                    "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"none\" stroke=\"{}\" stroke-width=\"1\"/>\n",
                    x,
                    y,
                    bar_w,
                    h,
                    theme.row_border
                ));
                svg.push_str(&format!(
                    "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"url(#{})\" stroke=\"{}\" stroke-width=\"1.5\" filter=\"url(#barShadow)\"/>\n",
                    x,
                    y,
                    bar_w,
                    h,
                    gradient,
                    theme.bar_outline
                ));

                let cx = x + (bar_w * 0.5);
                if !is_memory && directive.show_error_bars {
                    draw_error_bar(&mut svg, cx, m, &y_to_px, series_color(*lang));
                }
                if !is_memory && directive.show_std_dev {
                    draw_std_dev(&mut svg, cx, m, &y_to_px, series_color(*lang));
                }
            }
        }
        svg.push_str(&format!(
            "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"{}\" stroke-width=\"1\"/>\n",
            group_x + group_slot_w / 2.0,
            MARGIN_TOP + plot_h,
            group_x + group_slot_w / 2.0,
            MARGIN_TOP + plot_h + 6.0,
            theme.text_muted
        ));
        svg.push_str(&format!(
            "<text x=\"{:.2}\" y=\"{:.2}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"13\" fill=\"{}\">{}</text>\n",
            group_x + group_slot_w / 2.0,
            MARGIN_TOP + plot_h + 20.0,
            theme.text_muted,
            escape_xml(&bench.name)
        ));
    }

    let x_values: Vec<f64> = filtered
        .iter()
        .enumerate()
        .map(|(i, b)| extract_numeric_value(&b.name).map(|n| n as f64).unwrap_or((i + 1) as f64))
        .collect();
    let mut stats: Vec<LangStats> = Vec::new();
    if directive.show_regression {
        let mut used_regression_label_ys: Vec<f64> = Vec::new();
        for lang in &langs {
            let points: Vec<(f64, f64)> = filtered
                .iter()
                .enumerate()
                .filter_map(|(idx, bench)| {
                    bench.measurements.get(lang).and_then(|m| {
                        primary_value(m)
                            .filter(|v| v.is_finite() && *v < f64::MAX)
                            .map(|v| (x_values[idx], v))
                    })
                })
                .collect();
            if points.len() < 2 {
                if !points.is_empty() {
                    let ys: Vec<f64> = points.iter().map(|(_, y)| *y).collect();
                    let sum = ys.iter().sum::<f64>();
                    let mean = sum / ys.len() as f64;
                    let min = ys.iter().copied().fold(f64::INFINITY, f64::min);
                    let max = ys.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                    stats.push(LangStats {
                        lang: *lang,
                        mean,
                        min,
                        max,
                        samples: ys.len(),
                        regression: None,
                    });
                }
                continue;
            }
            let model =
                regression::select_model(&points, Some(directive.regression_model.as_str()));
            if let Some(model) = &model {
                let x_min = *x_values.first().unwrap_or(&1.0);
                let x_max = *x_values.last().unwrap_or(&x_min);
                let x_span = if (x_max - x_min).abs() < f64::EPSILON { 1.0 } else { x_max - x_min };
                let x_to_px = |x: f64| MARGIN_LEFT + ((x - x_min) / x_span) * plot_w;
                let mut reg_segments: Vec<Vec<(f64, f64)>> = Vec::new();
                let mut current_reg_points: Vec<(f64, f64)> = Vec::new();
                let split_gap = split_gap_bounds(scale, scale_params);
                for step in 0..=80 {
                    let x_val = x_min + (x_span * step as f64 / 80.0);
                    let y_val = model.predict(x_val).max(0.0);
                    let in_split_gap =
                        split_gap.map(|(low, high)| y_val > low && y_val < high).unwrap_or(false);
                    if in_split_gap {
                        if !current_reg_points.is_empty() {
                            reg_segments.push(std::mem::take(&mut current_reg_points));
                        }
                        continue;
                    }
                    let x = x_to_px(x_val);
                    let y = y_to_px(y_val);
                    current_reg_points.push((x, y));
                }
                if !current_reg_points.is_empty() {
                    reg_segments.push(current_reg_points);
                }
                let regression_color = series_color(*lang);
                for reg_points in &reg_segments {
                    let reg_path = path_from_points(reg_points);
                    svg.push_str(&format!(
                        "<path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"1.1\" stroke-dasharray=\"4 3\" opacity=\"0.62\"/>\n",
                        reg_path, regression_color
                    ));
                }
                if reg_segments.len() > 1 {
                    for seg_idx in 0..(reg_segments.len() - 1) {
                        if let Some((x1, y1)) = reg_segments[seg_idx].last().copied() {
                            draw_split_gap_marker(&mut svg, x1, y1, regression_color, theme.bg);
                        }
                        if let Some((x2, y2)) = reg_segments[seg_idx + 1].first().copied() {
                            draw_split_gap_marker(&mut svg, x2, y2, regression_color, theme.bg);
                        }
                    }
                }
                let label_x = MARGIN_LEFT + plot_w - 10.0;
                let anchor_y = y_to_px(model.predict(x_max).max(0.0));
                let label_y = choose_regression_label_y(
                    anchor_y,
                    MARGIN_TOP,
                    plot_h,
                    &used_regression_label_ys,
                );
                used_regression_label_ys.push(label_y);
                let reg_label = model_label(&model).to_string();
                svg.push_str(&format!(
                    "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\" font-family=\"sans-serif\" font-size=\"13\" font-weight=\"700\" fill=\"{}\" paint-order=\"stroke\" stroke=\"{}\" stroke-width=\"3\">{}</text>\n",
                    label_x,
                    label_y,
                    regression_color,
                    theme.bg,
                    escape_xml(&reg_label)
                ));
            }
            let ys: Vec<f64> = points.iter().map(|(_, y)| *y).collect();
            let sum = ys.iter().sum::<f64>();
            let mean = sum / ys.len() as f64;
            let min = ys.iter().copied().fold(f64::INFINITY, f64::min);
            let max = ys.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            let sample_count = filtered
                .iter()
                .filter_map(|bench| bench.measurements.get(lang).and_then(|m| m.run_count))
                .max()
                .unwrap_or(ys.len() as u64) as usize;
            stats.push(LangStats {
                lang: *lang,
                mean,
                min,
                max,
                samples: sample_count,
                regression: model,
            });
        }
    } else {
        for lang in &langs {
            let ys: Vec<f64> = filtered
                .iter()
                .filter_map(|bench| {
                    bench
                        .measurements
                        .get(lang)
                        .and_then(|m| primary_value(m).filter(|v| v.is_finite() && *v < f64::MAX))
                })
                .collect();
            if ys.is_empty() {
                continue;
            }
            let sum = ys.iter().sum::<f64>();
            let mean = sum / ys.len() as f64;
            let min = ys.iter().copied().fold(f64::INFINITY, f64::min);
            let max = ys.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            let sample_count = filtered
                .iter()
                .filter_map(|bench| bench.measurements.get(lang).and_then(|m| m.run_count))
                .max()
                .unwrap_or(ys.len() as u64) as usize;
            stats.push(LangStats {
                lang: *lang,
                mean,
                min,
                max,
                samples: sample_count,
                regression: None,
            });
        }
    }

    let title = directive.title.as_deref().unwrap_or("Benchmark Trend Bars");
    let description = directive.description.as_deref().unwrap_or("Benchmark trend comparison");
    svg.push_str(&format!(
        "<text x=\"{:.1}\" y=\"30\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"16\" font-weight=\"700\" fill=\"{}\">{}</text>\n",
        width / 2.0,
        theme.text,
        escape_xml(title)
    ));
    svg.push_str(&format!(
        "<text x=\"{:.1}\" y=\"50\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"13\" fill=\"{}\">{}</text>\n",
        width / 2.0,
        theme.text_muted,
        escape_xml(description)
    ));
    svg.push_str(&format!(
        "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"14\" font-weight=\"700\" fill=\"{}\">Benchmark (natural order)</text>\n",
        MARGIN_LEFT + plot_w / 2.0,
        MARGIN_TOP + plot_h + X_AXIS_LABEL_OFFSET,
        theme.text_muted
    ));
    let axis_title_x = 14.0;
    svg.push_str(&format!(
        "<text x=\"{:.1}\" y=\"{:.1}\" transform=\"rotate(-90 {:.1} {:.1})\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"14\" font-weight=\"700\" fill=\"{}\">{}</text>\n",
        axis_title_x,
        MARGIN_TOP + plot_h / 2.0 + 4.0,
        axis_title_x,
        MARGIN_TOP + plot_h / 2.0 + 4.0,
        theme.text_muted,
        escape_xml(&axis_label_for_scale(
            if is_memory { "bytes/op" } else { "nanos/op" },
            scale,
        ))
    ));
    if matches!(scale, YAxisScale::Split) {
        let split_t = 0.5;
        let split_y_raw = inverse_transform_y(split_t, scale, scale_params);
        let split_y = y_to_px(split_y_raw);
        svg.push_str(&format!(
            "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-dasharray=\"3 3\" stroke-width=\"1.2\"/>\n",
            MARGIN_LEFT,
            split_y,
            MARGIN_LEFT + plot_w,
            split_y,
            theme.text_secondary
        ));
    }
    svg.push_str(&legend_block(
        &langs,
        MARGIN_LEFT + 10.0,
        MARGIN_TOP + 10.0,
        theme.text,
        theme.text_muted,
        theme.detail_box_fill,
        theme.detail_box_stroke,
        theme.bg,
    ));
    svg.push_str(&stats_panel(
        &stats,
        MARGIN_LEFT,
        MARGIN_TOP + plot_h + X_AXIS_LABEL_OFFSET + STATS_TOP_GAP,
        plot_w,
        stats_box_height,
        is_memory,
        theme,
    ));
    svg.push_str(&format!(
        "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-width=\"1.6\"/>\n",
        MARGIN_LEFT,
        MARGIN_TOP + plot_h,
        MARGIN_LEFT + plot_w,
        MARGIN_TOP + plot_h,
        theme.text
    ));
    svg.push_str(&format!(
        "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-width=\"1.6\"/>\n",
        MARGIN_LEFT, MARGIN_TOP, MARGIN_LEFT, MARGIN_TOP + plot_h, theme.text
    ));
    svg.push_str("</svg>\n");
    svg
}

fn available_langs(benchmarks: &[&BenchmarkResult]) -> Vec<Lang> {
    poly_bench_runtime::supported_languages()
        .iter()
        .copied()
        .filter(|lang| benchmarks.iter().any(|b| b.measurements.contains_key(lang)))
        .collect()
}

fn draw_error_bar(
    svg: &mut String,
    x: f64,
    measurement: &Measurement,
    y_to_px: &impl Fn(f64) -> f64,
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
    y_to_px: &impl Fn(f64) -> f64,
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

fn legend_block(
    langs: &[Lang],
    x: f64,
    y: f64,
    text: &str,
    text_muted: &str,
    box_fill: &str,
    box_stroke: &str,
    opaque_fill: &str,
) -> String {
    let mut svg = String::new();
    let box_h = 32.0 + (langs.len() as f64 * 22.0);
    svg.push_str(&format!(
        "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"118\" height=\"{:.1}\" rx=\"8\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>\n",
        x, y, box_h, opaque_fill, box_stroke
    ));
    svg.push_str(&format!(
        "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"118\" height=\"{:.1}\" rx=\"8\" fill=\"{}\" fill-opacity=\"0.15\"/>\n",
        x, y, box_h, box_fill
    ));
    svg.push_str(&format!(
        "<text x=\"{:.1}\" y=\"{:.1}\" font-family=\"sans-serif\" font-size=\"11\" font-weight=\"600\" fill=\"{}\">Legend</text>\n",
        x + 10.0,
        y + 16.0,
        text_muted
    ));
    for (i, lang) in langs.iter().enumerate() {
        let item_y = y + 38.0 + (i as f64 * 22.0);
        svg.push_str(&format!(
            "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4.5\" fill=\"{}\"/>\n",
            x + 12.0,
            item_y - 3.0,
            badge_color(*lang)
        ));
        svg.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" font-family=\"sans-serif\" font-size=\"11\" fill=\"{}\">{}</text>\n",
            x + 22.0,
            item_y,
            text,
            lang_label(*lang)
        ));
    }
    svg
}

fn stats_panel(
    stats: &[LangStats],
    x: f64,
    y: f64,
    width: f64,
    _height: f64,
    is_memory: bool,
    theme: Theme,
) -> String {
    let mut svg = String::new();

    if stats.is_empty() {
        return svg;
    }
    let pad_x = 10.0;
    let pad_y = 10.0;
    let table_x = x + pad_x;
    let table_y = y + pad_y;
    let table_w = width - pad_x * 2.0;
    let lang_w = table_w * 0.18;
    let mean_w = table_w * 0.18;
    let range_w = table_w * 0.24;
    let sample_w = table_w * 0.18;
    let eq_w = (table_w - lang_w - mean_w - range_w - sample_w).max(90.0);
    let cols = [lang_w, mean_w, range_w, sample_w, eq_w];
    let headers = ["Language", "Mean", "Min / Max", "Samples / R2", "Regression"];
    let header_y = table_y + STATS_HEADER_HEIGHT;
    let table_h = STATS_HEADER_HEIGHT + stats.len() as f64 * STATS_ROW_HEIGHT;
    svg.push_str(&format!(
        "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"6\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>\n",
        table_x,
        table_y,
        table_w,
        table_h,
        theme.plot_bg,
        theme.detail_box_stroke
    ));
    svg.push_str(&format!(
        "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-width=\"1\"/>\n",
        table_x,
        header_y,
        table_x + table_w,
        header_y,
        theme.detail_box_stroke
    ));
    let mut col_x = table_x;
    for (idx, col_w) in cols.iter().enumerate() {
        if idx < headers.len() {
            svg.push_str(&format!(
                "<text x=\"{:.1}\" y=\"{:.1}\" font-family=\"sans-serif\" font-size=\"10\" font-weight=\"700\" fill=\"{}\">{}</text>\n",
                col_x + 8.0,
                table_y + 15.0,
                theme.text_secondary,
                headers[idx]
            ));
        }
        col_x += *col_w;
        if idx < cols.len() - 1 {
            svg.push_str(&format!(
                "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-width=\"1\"/>\n",
                col_x,
                table_y,
                col_x,
                table_y + table_h,
                theme.detail_box_stroke
            ));
        }
    }

    for (idx, stat) in stats.iter().enumerate() {
        let row_top = header_y + idx as f64 * STATS_ROW_HEIGHT;
        if idx % 2 == 1 {
            svg.push_str(&format!(
                "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{}\" fill-opacity=\"0.35\"/>\n",
                table_x,
                row_top,
                table_w,
                STATS_ROW_HEIGHT,
                theme.detail_box_fill
            ));
        }
        if idx < stats.len() - 1 {
            svg.push_str(&format!(
                "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-width=\"1\"/>\n",
                table_x,
                row_top + STATS_ROW_HEIGHT,
                table_x + table_w,
                row_top + STATS_ROW_HEIGHT,
                theme.detail_box_stroke
            ));
        }
        let baseline_y = row_top + 15.0;
        let r2 = stat
            .regression
            .as_ref()
            .map(|m| format!("{:.3}", m.r_squared))
            .unwrap_or_else(|| "n/a".to_string());
        let eq = stat
            .regression
            .as_ref()
            .map(|m| truncate_text(&m.format_equation(), 34))
            .unwrap_or_else(|| "n/a".to_string());
        let (mean_fmt, range_fmt) = if is_memory {
            (
                Measurement::format_bytes(stat.mean as u64),
                format!(
                    "{} / {}",
                    Measurement::format_bytes(stat.min as u64),
                    Measurement::format_bytes(stat.max as u64)
                ),
            )
        } else {
            (format!("{:.0} ns/op", stat.mean), format!("{:.0} / {:.0}", stat.min, stat.max))
        };
        let mut x_cursor = table_x;
        let lang_badge = badge_color(stat.lang);
        svg.push_str(&format!(
            "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{}\"/>\n",
            x_cursor + 9.0,
            baseline_y - 3.0,
            lang_badge
        ));
        svg.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" font-family=\"sans-serif\" font-size=\"10.5\" font-weight=\"600\" fill=\"{}\">{}</text>\n",
            x_cursor + 17.0,
            baseline_y,
            theme.text,
            lang_label(stat.lang)
        ));
        x_cursor += cols[0];
        svg.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" font-family=\"sans-serif\" font-size=\"10.5\" fill=\"{}\">{}</text>\n",
            x_cursor + 8.0,
            baseline_y,
            theme.text_muted,
            mean_fmt
        ));
        x_cursor += cols[1];
        svg.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" font-family=\"sans-serif\" font-size=\"10.5\" fill=\"{}\">{}</text>\n",
            x_cursor + 8.0,
            baseline_y,
            theme.text_muted,
            range_fmt
        ));
        x_cursor += cols[2];
        svg.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" font-family=\"sans-serif\" font-size=\"10.5\" fill=\"{}\">{} / {}</text>\n",
            x_cursor + 8.0,
            baseline_y,
            theme.text_muted,
            stat.samples,
            r2
        ));
        x_cursor += cols[3];
        svg.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" font-family=\"sans-serif\" font-size=\"10.5\" fill=\"{}\">{}</text>\n",
            x_cursor + 8.0,
            baseline_y,
            theme.text_muted,
            escape_xml(&eq)
        ));
    }
    svg
}

fn lang_label(lang: Lang) -> &'static str {
    poly_bench_runtime::lang_full_name(lang)
}

fn series_color(lang: Lang) -> &'static str {
    match lang {
        Lang::Python => "#D8BD4A",
        _ => lang_color(lang),
    }
}

fn badge_color(lang: Lang) -> &'static str {
    match lang {
        Lang::Python => "#C6A73B",
        Lang::CSharp => "#6B32D6",
        _ => series_color(lang),
    }
}

fn stats_layout(count: usize, width: f64) -> (usize, usize, f64) {
    if count == 0 {
        return (1, 0, 0.0);
    }
    let _ = width;
    let rows = count;
    let height = 20.0 + STATS_HEADER_HEIGHT + (rows as f64 * STATS_ROW_HEIGHT) + 12.0;
    (1, rows, height)
}

fn model_label(model: &SelectedModel) -> &'static str {
    match model.model_type {
        regression::ModelType::Constant => "O(1)",
        regression::ModelType::Logarithmic => "O(log n)",
        regression::ModelType::Linear => "O(n)",
        regression::ModelType::Linearithmic => "O(n log n)",
        regression::ModelType::Quadratic => "O(n^2)",
        regression::ModelType::Mixed => "mixed",
        regression::ModelType::Cubic => "O(n^3)",
        regression::ModelType::PowerLaw => "power",
    }
}

fn truncate_text(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    let mut out = String::new();
    for (idx, ch) in text.chars().enumerate() {
        if idx >= max_chars.saturating_sub(1) {
            break;
        }
        out.push(ch);
    }
    out.push('…');
    out
}

fn should_show_non_linear_tick_label(idx: usize, total: usize, source_label: &str) -> bool {
    if !source_label.is_empty() {
        return true;
    }
    if total <= 4 {
        return true;
    }
    idx == 0 || idx == total - 1 || idx == total / 2 || idx == total / 3 || idx == (2 * total) / 3
}

fn format_axis_tick(value: f64) -> String {
    if value >= 1_000_000_000.0 {
        format!("{:.1}B", value / 1_000_000_000.0)
    } else if value >= 1_000_000.0 {
        format!("{:.1}M", value / 1_000_000.0)
    } else if value >= 1_000.0 {
        format!("{:.1}K", value / 1_000.0)
    } else if value >= 10.0 {
        format!("{:.0}", value)
    } else {
        format!("{:.2}", value)
    }
}

fn choose_regression_label_y(
    anchor_y: f64,
    plot_top: f64,
    plot_height: f64,
    occupied: &[f64],
) -> f64 {
    let min_y = plot_top + 16.0;
    let max_y = plot_top + plot_height - 8.0;
    let candidates = [0.0, -24.0, 24.0, -40.0, 40.0, -56.0, 56.0];
    let mut best_y = anchor_y.clamp(min_y, max_y);
    let mut best_score = f64::NEG_INFINITY;
    for offset in candidates {
        let y = (anchor_y + offset).clamp(min_y, max_y);
        let min_sep = occupied.iter().map(|other| (y - other).abs()).fold(100.0, f64::min);
        let score = min_sep - offset.abs() * 0.05;
        if score > best_score {
            best_score = score;
            best_y = y;
        }
    }
    best_y
}

fn path_from_points(points: &[(f64, f64)]) -> String {
    let mut path = String::new();
    for (idx, (x, y)) in points.iter().enumerate() {
        if idx == 0 {
            path.push_str(&format!("M {:.2} {:.2}", x, y));
        } else {
            path.push_str(&format!(" L {:.2} {:.2}", x, y));
        }
    }
    path
}

fn draw_split_gap_marker(svg: &mut String, x: f64, y: f64, color: &str, bg: &str) {
    svg.push_str(&format!(
        "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"2.8\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1.2\"/>\n",
        x, y, bg, color
    ));
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
            poly_bench_dsl::SuiteType::Performance,
            "legacy".to_string(),
            None,
            None,
            None,
            None,
        )
    }

    #[test]
    fn test_generate_bar_chart_with_regression() {
        let b1 = bench("n10", 100.0, 140.0);
        let b2 = bench("n100", 500.0, 700.0);
        let b3 = bench("n1000", 2200.0, 3000.0);
        let mut directive =
            ChartDirectiveIR::new(poly_bench_dsl::ChartType::BarChart, "bar.svg".to_string());
        directive.show_regression = true;
        directive.show_std_dev = true;
        directive.show_error_bars = true;
        directive.regression_model = "auto".to_string();
        let svg = generate(vec![&b1, &b2, &b3], &directive, poly_bench_dsl::SuiteType::Performance);
        assert!(svg.contains("<svg"));
        assert!(svg.contains("stroke-dasharray"));
    }

    #[test]
    fn test_generate_bar_chart_all_y_scales() {
        let b1 = bench("n10", 100.0, 140.0);
        let b2 = bench("n100", 5_000.0, 7_000.0);
        let b3 = bench("n1000", 220_000.0, 300_000.0);
        for scale in ["linear", "log10", "symlog", "split"] {
            let mut directive =
                ChartDirectiveIR::new(poly_bench_dsl::ChartType::BarChart, "bar.svg".to_string());
            directive.description = Some("desc".to_string());
            directive.y_scale = scale.to_string();
            let svg =
                generate(vec![&b1, &b2, &b3], &directive, poly_bench_dsl::SuiteType::Performance);
            assert!(svg.contains("<svg"));
            assert!(svg.contains("nanos/op"));
        }
    }
}
