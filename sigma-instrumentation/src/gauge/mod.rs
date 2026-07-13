//! Gauge geometry for the RPM tach.
//!
//! **Geometry coupling:** layout constants in [`constants`] must match
//! `ui/widgets/rpm_dial.slint`. Sweep range is driven by [`GaugeScale`].

mod constants;
mod geometry;
mod needle;
mod numeral;
mod paths;
mod scale;
mod ticks;

pub use needle::needle_paths;
pub use numeral::{Numeral, numerals};
pub use paths::{
    redline_path, swept_path, track_path, turn_signal_left_path, turn_signal_right_path,
};
pub use scale::{DEFAULT_MAX_RPM, DEFAULT_REDLINE_RPM, GaugeScale};
pub use ticks::{ticks_major, ticks_minor, ticks_redline};

/// Default full-scale RPM (XSR900 GP dial).
pub const MAX_RPM: f32 = DEFAULT_MAX_RPM;
/// Default redline RPM (XSR900 GP dial).
pub const REDLINE: f32 = DEFAULT_REDLINE_RPM;
