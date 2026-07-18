//! Window index table — keep in sync with `road_dashboard.slint` and `app.slint`.
//!
//! Face buttons (see [`crate::buttons`]): Previous / Next / Back / Select.
//!
//! On Connectivity (index 5) and Updates (index 9), Previous/Next move
//! in-window focus; Select activates; Back leaves a list or returns home.
//! Edge Previous/Next leave the focused window.
//!
//! | Index | Window        | Tier        |
//! |-------|---------------|-------------|
//! | 0     | Systems       | left panel  |
//! | 1     | Navigation    | left panel  |
//! | 2     | Compass/GPS   | left panel  |
//! | 3     | Diagnostics   | left panel  |
//! | 4     | Camera        | left panel  |
//! | 5     | Alerts        | left panel  |
//! | 6     | Connectivity  | full-screen |
//! | 7     | Maintenance   | full-screen |
//! | 8     | Fuel          | full-screen |
//! | 9     | Security      | full-screen |
//! | 10    | Updates       | full-screen |

/// Last index of glanceable left-panel windows (while moving).
pub const PANEL_MAX: i32 = 5;

/// Streaming anomaly alerts window (Select acknowledges latched alerts).
pub const ALERTS: i32 = 5;

/// Total window count.
pub const COUNT: i32 = 11;
