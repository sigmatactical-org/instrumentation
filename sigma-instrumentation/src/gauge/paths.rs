//! Static and dynamic SVG paths for the RPM dial.

use slint::SharedString;

use super::constants::{MAX_RPM, R, REDLINE_ZONE};
use super::geometry::{angle_for, arc, point};

pub fn track_path() -> SharedString {
    arc(0.0, MAX_RPM, R * 0.97).into()
}

pub fn redline_path() -> SharedString {
    arc(REDLINE_ZONE, MAX_RPM, R * 0.90).into()
}

/// Red swept sector from idle to current RPM. Empty near rest.
pub fn swept_path(rpm: f32) -> SharedString {
    let a0 = angle_for(0.0);
    let a1 = angle_for(rpm);
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
