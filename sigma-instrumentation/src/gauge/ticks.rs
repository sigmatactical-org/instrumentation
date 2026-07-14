//! Tick mark paths around the RPM dial.

use slint::SharedString;

use super::constants::R;
use super::geometry::{angle_for, point};
use super::scale::GaugeScale;

fn ticks(scale: &GaugeScale, major_wanted: bool, redline_wanted: bool) -> SharedString {
    let _major = scale.major_step();
    let minor = scale.minor_step();
    let count = (scale.max_rpm / minor).round() as i32;
    let zone = scale.redline_zone();
    let mut s = String::new();

    for k in 0..=count {
        let rpm = k as f32 * minor;
        let is_major = k % 2 == 0;
        let in_redline = rpm >= zone;
        if in_redline != redline_wanted {
            continue;
        }
        if !in_redline && is_major != major_wanted {
            continue;
        }
        let ang = angle_for(rpm, scale);
        let inner = if is_major { R * 0.80 } else { R * 0.88 };
        let (xi, yi) = point(ang, inner);
        let (xo, yo) = point(ang, R * 0.97);
        s.push_str(&format!("M {xi:.2} {yi:.2} L {xo:.2} {yo:.2} "));
    }
    s.into()
}

/// SVG path drawing every major tick.
pub fn ticks_major(scale: &GaugeScale) -> SharedString {
    ticks(scale, true, false)
}

/// SVG path drawing every minor tick.
pub fn ticks_minor(scale: &GaugeScale) -> SharedString {
    ticks(scale, false, false)
}

/// SVG path drawing ticks inside the redline band.
pub fn ticks_redline(scale: &GaugeScale) -> SharedString {
    ticks(scale, true, true)
}
