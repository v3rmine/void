use std::fmt::Debug;

use log::{trace, tracing};

use crate::ChartValue;

pub(crate) const SVG_MIN_X: f64 = 0.0;
pub(crate) const SVG_MAX_X: f64 = 1000.0;
pub(crate) const SVG_MIN_Y: f64 = 0.0;
pub(crate) const SVG_MAX_Y: f64 = 300.0;
pub(crate) const SVG_Y_RATIO: f64 = SVG_MAX_Y - SVG_MIN_Y;

pub(crate) const ESTIMATED_ONE_CHAR_SIZE: f64 = 16.0;
pub(crate) const RESERVED_CHARACTERS: f64 = 9.0;
pub(crate) const CHART_MIN_X: f64 = SVG_MIN_X + (ESTIMATED_ONE_CHAR_SIZE * RESERVED_CHARACTERS);
pub(crate) const CHART_MAX_X: f64 = SVG_MAX_X;
pub(crate) const CHART_X_RATIO: f64 = CHART_MAX_X - CHART_MIN_X;
pub(crate) const CHART_MIN_Y: f64 = SVG_MIN_Y + (SVG_Y_RATIO * 0.05);
pub(crate) const CHART_MAX_Y: f64 = SVG_MAX_Y - (SVG_Y_RATIO * 0.05);
pub(crate) const CHART_Y_RATIO: f64 = CHART_MAX_Y - CHART_MIN_Y;

pub(crate) const LABELS_OFFSET: f64 = CHART_MIN_X - (ESTIMATED_ONE_CHAR_SIZE * 0.5);

// Because the viewBox in SVG invert the values (top left corner is 0,0)
#[tracing::instrument(level = "trace")]
pub fn svg_value_invert(value: f64, max: f64, min: f64) -> f64 {
    let result = ((value - min) + (max - min) * -1f64) * -1f64 + min;
    trace!(result);
    result
}

#[tracing::instrument(level = "trace", skip(raw_values))]
pub fn values_to_polyline<T: Debug>(
    raw_values: &[ChartValue<T>],
    (min_value_range, max_value_range): (f64, f64),
) -> Option<String> {
    if raw_values.is_empty() {
        return None;
    };

    let value_ratio = max_value_range - min_value_range;

    let first_date = raw_values.first().map(|(_, date, _)| date).unwrap();
    let last_date = raw_values.last().map(|(_, date, _)| date).unwrap();
    let date_ratio = (last_date - first_date) as f64;
    trace!(
        first_date_timestamp = first_date,
        last_date_timestamp = last_date
    );

    let values = raw_values
        .iter()
        .map(|(val, date, _)| {
            trace!(
                date,
                first_date,
                top = date - first_date,
                bottom = date_ratio * CHART_X_RATIO + CHART_MIN_X
            );
            format!(
                "{},{}",
                ((date - first_date) as f64 / date_ratio * CHART_X_RATIO + CHART_MIN_X).round(),
                svg_value_invert(
                    ((val - min_value_range) / value_ratio * CHART_Y_RATIO + CHART_MIN_Y).round(),
                    CHART_MAX_Y,
                    CHART_MIN_Y
                ),
            )
        })
        .collect::<Vec<_>>()
        .join(" ");
    trace!(svg_values = values);

    Some(values)
}

pub fn round_to_len(value: f64, len: usize) -> f64 {
    (value * 10f64.powi(len as i32)).round() / 10f64.powi(len as i32)
}
