use std::fmt::Debug;

use log::tracing;
use maud::{html, Markup};
use typed_builder::TypedBuilder;

use crate::svg::{
    round_to_len, CHART_MIN_X, LABELS_OFFSET, SVG_MAX_X, SVG_MAX_Y, SVG_MIN_X, SVG_MIN_Y,
};

pub type ChartValue<T> = (f64, i64, T);
pub type ChartLine = (String, Option<String>, String);

#[derive(Debug, Default, Clone, TypedBuilder)]
pub struct ChartContext {
    pub collections: Vec<ChartLine>,
    pub max_value: f64,
    #[builder(default = "%".to_string(), setter(into))]
    pub unit: String,
}

// TODO: Add hover on dates
#[tracing::instrument(level = "debug", skip(ctx), fields(unit = ctx.unit))]
pub fn Chart(ctx: ChartContext) -> Markup {
    if ctx.collections.is_empty() {
        html! {
            p { "No data available." }
        }
    } else {
        let mid_value = round_to_len(ctx.max_value / 2.0, 2);
        html! {
            svg.chart viewBox=(format!("{SVG_MIN_X} {SVG_MIN_Y} {SVG_MAX_X} {SVG_MAX_Y}")) {
                g.grid.x-grid {
                    line x1=(CHART_MIN_X) y1="5%" x2="100%" y2="5%" {}
                    line x1=(CHART_MIN_X) y1="50%" x2="100%" y2="50%" {}
                    line x1=(CHART_MIN_X) y1="95%" x2="100%" y2="95%" {}
                }
                g.labels.x-labels {
                    text x=(LABELS_OFFSET) y="5%" dy="6" { (format!("{}{}", round_to_len(ctx.max_value, 2), ctx.unit)) }
                    text x=(LABELS_OFFSET) y="50%" dy="6" { (format!("{}{}", round_to_len(mid_value, 2), ctx.unit)) }
                    text x=(LABELS_OFFSET) y="95%" dy="6" { (format!("0{}", ctx.unit)) }
                }
                g.lines {
                    @for (color, _label, polyline) in ctx.collections {
                        polyline.dataline fill="none" stroke=(color) stroke-width="2" points=(polyline) {}
                    }
                }
            }
        }
    }
}
