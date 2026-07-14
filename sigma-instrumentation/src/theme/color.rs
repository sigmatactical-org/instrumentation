//! Slint color helper.

use slint::Color;

/// Opaque color from 8-bit RGB components.
pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgb_u8(r, g, b)
}
