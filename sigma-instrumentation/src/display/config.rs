//! Display sizing presets.

const PANEL_WIDTH: u32 = 800;
const PANEL_HEIGHT: u32 = 480;

/// How the dashboard window should be presented on startup.
#[derive(Clone, Copy, Debug)]
pub struct DisplayConfig {
    /// Fixed window size (e.g. 800×480 virt panel). When set, fullscreen is skipped.
    pub fixed_size: Option<(u32, u32)>,
    /// Default kiosk/fullscreen when `SLINT_FULLSCREEN` is unset.
    pub default_kiosk: bool,
}

impl DisplayConfig {
    pub const fn virt_panel() -> Self {
        Self {
            fixed_size: Some((PANEL_WIDTH, PANEL_HEIGHT)),
            default_kiosk: false,
        }
    }

    pub const fn desktop() -> Self {
        Self {
            fixed_size: None,
            default_kiosk: false,
        }
    }

    pub const fn embedded(default_kiosk: bool) -> Self {
        Self {
            fixed_size: None,
            default_kiosk,
        }
    }
}
