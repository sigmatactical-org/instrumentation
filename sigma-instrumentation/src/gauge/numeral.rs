//! RPM scale numerals around the dial.

use super::constants::R;
use super::geometry::{angle_for, point};
use super::scale::GaugeScale;

pub struct Numeral {
    pub x: f32,
    pub y: f32,
    pub label: String,
    pub redline: bool,
}

pub fn numerals(scale: &GaugeScale) -> Vec<Numeral> {
    let step = scale.major_step();
    let count = (scale.max_rpm / step).round() as i32;
    let zone = scale.redline_zone();

    (0..=count)
        .map(|k| {
            let rpm = k as f32 * step;
            let (x, y) = point(angle_for(rpm, scale), R * 0.72);
            let label_thousands = (rpm / 1_000.0).round() as i32;
            let label = if step >= 1_000.0 {
                format!("{label_thousands}")
            } else {
                format!("{:.1}", rpm / 1_000.0)
            };
            Numeral {
                x,
                y,
                label,
                redline: rpm >= zone,
            }
        })
        .collect()
}
