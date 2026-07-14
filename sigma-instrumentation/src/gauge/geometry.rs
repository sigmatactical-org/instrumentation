//! Polar geometry helpers for the RPM dial.

use super::constants::{CX, CY, S0_DEG, SW_DEG};
use super::scale::GaugeScale;

/// Degrees → radians.
pub fn deg2rad(d: f32) -> f32 {
    d * std::f32::consts::PI / 180.0
}

/// Needle angle for an rpm value on the scale sweep.
pub fn angle_for(rpm: f32, scale: &GaugeScale) -> f32 {
    let f = (rpm / scale.max_rpm).clamp(0.0, 1.0);
    S0_DEG + f * SW_DEG
}

/// Point on the gauge circle at the given angle/radius.
pub fn point(ang_deg: f32, radius: f32) -> (f32, f32) {
    let a = deg2rad(ang_deg);
    (CX + radius * a.cos(), CY + radius * a.sin())
}

/// SVG arc segment between two rpm values.
pub fn arc(rpm_from: f32, rpm_to: f32, radius: f32, scale: &GaugeScale) -> String {
    let a0 = angle_for(rpm_from, scale);
    let a1 = angle_for(rpm_to, scale);
    let (x0, y0) = point(a0, radius);
    let (x1, y1) = point(a1, radius);
    let large = if (a1 - a0).abs() > 180.0 { 1 } else { 0 };
    format!("M {x0:.2} {y0:.2} A {radius:.2} {radius:.2} 0 {large} 1 {x1:.2} {y1:.2}")
}
