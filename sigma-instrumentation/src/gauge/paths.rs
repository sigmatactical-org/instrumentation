//! Static and dynamic SVG paths for the RPM dial.

use slint::SharedString;

use super::constants::R;
use super::geometry::{angle_for, arc, point};
use super::scale::GaugeScale;

/// Outer-ring radii for turn / hazard signal sectors (matches dial bezel).
const SIGNAL_R_IN: f32 = 200.0;
const SIGNAL_R_OUT: f32 = 216.0;
const SIGNAL_SPAN_DEG: f32 = 30.0;

pub fn track_path(scale: &GaugeScale) -> SharedString {
    arc(0.0, scale.max_rpm, R * 0.97, scale).into()
}

pub fn redline_path(scale: &GaugeScale) -> SharedString {
    arc(scale.redline_zone(), scale.max_rpm, R * 0.90, scale).into()
}

/// Red swept sector from idle to current RPM. Empty near rest.
pub fn swept_path(scale: &GaugeScale, rpm: f32) -> SharedString {
    let a0 = angle_for(0.0, scale);
    let a1 = angle_for(rpm, scale);
    if (a1 - a0) < 0.5 {
        return SharedString::from("");
    }
    let (rin, rout) = (R * 0.34, R * 0.90);
    let (osx, osy) = point(a0, rout);
    let (oex, oey) = point(a1, rout);
    let (iex, iey) = point(a1, rin);
    let (isx, isy) = point(a0, rin);
    let large = if (a1 - a0) > 180.0 { 1 } else { 0 };
    format!(
        "M {osx:.2} {osy:.2} A {rout:.2} {rout:.2} 0 {large} 1 {oex:.2} {oey:.2} \
         L {iex:.2} {iey:.2} A {rin:.2} {rin:.2} 0 {large} 0 {isx:.2} {isy:.2} Z"
    )
    .into()
}

fn annular_sector(a0_deg: f32, a1_deg: f32, rin: f32, rout: f32) -> String {
    let (osx, osy) = point(a0_deg, rout);
    let (oex, oey) = point(a1_deg, rout);
    let (iex, iey) = point(a1_deg, rin);
    let (isx, isy) = point(a0_deg, rin);
    let large = if (a1_deg - a0_deg).abs() > 180.0 {
        1
    } else {
        0
    };
    format!(
        "M {osx:.2} {osy:.2} A {rout:.2} {rout:.2} 0 {large} 1 {oex:.2} {oey:.2} \
         L {iex:.2} {iey:.2} A {rin:.2} {rin:.2} 0 {large} 0 {isx:.2} {isy:.2} Z"
    )
}

/// 30° outer-ring sector centered on the left (9 o'clock) — turn / hazard.
pub fn turn_signal_left_path() -> SharedString {
    let half = SIGNAL_SPAN_DEG / 2.0;
    annular_sector(180.0 - half, 180.0 + half, SIGNAL_R_IN, SIGNAL_R_OUT).into()
}

/// 30° outer-ring sector centered on the right (3 o'clock) — turn / hazard.
pub fn turn_signal_right_path() -> SharedString {
    let half = SIGNAL_SPAN_DEG / 2.0;
    annular_sector(0.0 - half, 0.0 + half, SIGNAL_R_IN, SIGNAL_R_OUT).into()
}
