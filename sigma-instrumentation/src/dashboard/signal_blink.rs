//! Blink phase for turn / hazard outer-ring indicators.

use slint::ComponentHandle;
use std::time::Duration;

use crate::SigmaDashboard;

const BLINK_MS: u64 = 350;

/// Start a repeating timer that toggles [`SigmaDashboard::get_signal_lit`].
///
/// Keep the returned [`slint::Timer`] alive (e.g. store it or hold it across `ui.run()`).
pub fn start_signal_blink(ui: &SigmaDashboard) -> slint::Timer {
    let ui_weak = ui.as_weak();
    let timer = slint::Timer::default();
    timer.start(
        slint::TimerMode::Repeated,
        Duration::from_millis(BLINK_MS),
        move || {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            if ui.get_turn_left() || ui.get_turn_right() {
                ui.set_signal_lit(!ui.get_signal_lit());
            } else if ui.get_signal_lit() {
                ui.set_signal_lit(false);
            }
        },
    );
    timer
}
