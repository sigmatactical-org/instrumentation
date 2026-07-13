//! Theme initialization from environment or explicit mode.

use crate::SigmaDashboard;

use super::mode::{DisplayMode, parse_mode_from_env};
use super::preset::ThemePreset;

/// Load display mode from the environment and apply. Returns the preset used.
pub fn init_from_env(ui: &SigmaDashboard) -> ThemePreset {
    let preset = ThemePreset::by_mode(parse_mode_from_env());
    preset.apply(ui);
    preset
}

/// Apply a specific display mode (e.g. from a light sensor or user setting).
pub fn apply_mode(ui: &SigmaDashboard, mode: DisplayMode) -> ThemePreset {
    let preset = ThemePreset::by_mode(mode);
    preset.apply(ui);
    preset
}
