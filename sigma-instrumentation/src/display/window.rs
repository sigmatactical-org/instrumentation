//! Window size and fullscreen policy.

use slint::ComponentHandle;

use crate::SigmaDashboard;

use super::config::DisplayConfig;

fn panel_size_from_env() -> Option<(u32, u32)> {
    let (w, h) = (
        std::env::var("SIGMA_PANEL_WIDTH").ok()?,
        std::env::var("SIGMA_PANEL_HEIGHT").ok()?,
    );
    let (w, h) = (w.parse().ok()?, h.parse().ok()?);
    (w > 0 && h > 0).then_some((w, h))
}

fn pin_panel_window(ui: &SigmaDashboard, w: u32, h: u32) {
    // Prefer logical size so 800×480 design coords match the window on HiDPI /
    // Wayland. Fall back to preferred size only (no pin) if unset.
    ui.window()
        .set_size(slint::LogicalSize::new(w as f32, h as f32));
}

/// Apply window size / fullscreen policy from config and environment.
pub fn configure_window(ui: &SigmaDashboard, config: DisplayConfig) {
    // Desktop testbed: let preferred-width/height drive the window — do not
    // fight the compositor scale with an early set_size (clips the dial).
    if config.fixed_size.is_some()
        && std::env::var("SIGMA_PIN_WINDOW").ok().as_deref() != Some("1")
        && panel_size_from_env().is_none()
        && !config.default_kiosk
    {
        return;
    }

    if let Some((w, h)) = panel_size_from_env().or(config.fixed_size) {
        pin_panel_window(ui, w, h);
        return;
    }

    let kiosk = std::env::var("SLINT_FULLSCREEN")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(config.default_kiosk);

    if kiosk {
        ui.window().set_fullscreen(true);
        ui.window()
            .set_position(slint::PhysicalPosition::new(0, 0));
    }
}
