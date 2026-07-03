//! Static gauge artwork bound to a dashboard instance.

use slint::{ModelRc, SharedString, VecModel};
use std::rc::Rc;

use crate::{gauge, SigmaDashboard, Tick};

/// Install computed RPM dial paths, ticks, and numerals on the dashboard.
pub fn init_gauge_art(ui: &SigmaDashboard) {
    ui.set_track_path(gauge::track_path());
    ui.set_redline_path(gauge::redline_path());
    ui.set_ticks_major(gauge::ticks_major());
    ui.set_ticks_minor(gauge::ticks_minor());
    ui.set_ticks_redline(gauge::ticks_redline());
    ui.set_labels(build_numerals());
}

fn build_numerals() -> ModelRc<Tick> {
    let rows: Vec<Tick> = gauge::numerals()
        .into_iter()
        .map(|n| Tick {
            x: n.x,
            y: n.y,
            label: SharedString::from(n.label),
            redline: n.redline,
        })
        .collect();
    ModelRc::new(Rc::new(VecModel::from(rows)))
}
