//! Tick mark paths around the RPM dial.

use slint::SharedString;

use super::constants::{R, REDLINE_ZONE};
use super::geometry::{angle_for, point};

fn ticks(major_wanted: bool, redline_wanted: bool) -> SharedString {
    let mut s = String::new();
    for k in 0..=24 {
        let rpm = k as f32 * 500.0;
        let major = k % 2 == 0;
        let redline = rpm >= REDLINE_ZONE;
        if redline != redline_wanted {
            continue;
        }
        if !redline && major != major_wanted {
            continue;
        }
        let ang = angle_for(rpm);
        let inner = if major { R * 0.80 } else { R * 0.88 };
        let (xi, yi) = point(ang, inner);
        let (xo, yo) = point(ang, R * 0.97);
        s.push_str(&format!("M {xi:.2} {yi:.2} L {xo:.2} {yo:.2} "));
    }
    s.into()
}

pub fn ticks_major() -> SharedString {
    ticks(true, false)
}

pub fn ticks_minor() -> SharedString {
    ticks(false, false)
}

pub fn ticks_redline() -> SharedString {
    ticks(true, true)
}
