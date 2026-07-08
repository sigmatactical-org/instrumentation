//! RPM scale numerals around the dial.

use super::constants::{R, REDLINE_ZONE};
use super::geometry::{angle_for, point};

pub struct Numeral {
    pub x: f32,
    pub y: f32,
    pub label: String,
    pub redline: bool,
}

pub fn numerals() -> Vec<Numeral> {
    (0..=12)
        .map(|k| {
            let rpm = k as f32 * 1000.0;
            let (x, y) = point(angle_for(rpm), R * 0.72);
            Numeral {
                x,
                y,
                label: format!("{k}"),
                redline: rpm >= REDLINE_ZONE,
            }
        })
        .collect()
}
