//! Display modes — day / dusk / night palettes and opacity tiers.
//!
//! Select via `SIGMA_DISPLAY_MODE` (preferred) or `SIGMA_UI_TONE` (alias).

mod color;
mod init;
mod mode;
mod preset;

pub use init::{apply_mode, init_from_env};
pub use mode::DisplayMode;
pub use preset::ThemePreset;
