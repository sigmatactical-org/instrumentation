//! Gauge geometry for the RPM tach.
//!
//! **Geometry coupling:** constants in [`constants`] must match
//! `ui/widgets/rpm_dial.slint`. Change both together.

mod constants;
mod geometry;
mod needle;
mod numeral;
mod paths;
mod ticks;

pub use constants::{MAX_RPM, REDLINE};
pub use needle::needle_paths;
pub use numeral::{numerals, Numeral};
pub use paths::{redline_path, swept_path, track_path};
pub use ticks::{ticks_major, ticks_minor, ticks_redline};
