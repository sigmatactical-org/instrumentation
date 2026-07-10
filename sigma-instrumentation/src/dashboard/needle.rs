//! RPM-driven dial dynamics — the swept sector and needle paths.

use crate::gauge::GaugeScale;
use crate::{gauge, SigmaDashboard};

/// Update the swept sector and the four needle-path properties for `rpm`.
pub fn set_needle_paths(ui: &SigmaDashboard, scale: &GaugeScale, rpm: f32) {
    ui.set_swept_path(gauge::swept_path(scale, rpm));
    let (left, spine, right, outline) = gauge::needle_paths(scale, rpm);
    ui.set_needle_left(left);
    ui.set_needle_spine(spine);
    ui.set_needle_right(right);
    ui.set_needle_outline(outline);
}
