//! Connectivity window focus model and Slint binding helpers.
//!
//! Face buttons on window 5 (Connectivity):
//! - Previous / Next move the focus highlight (edge Prev/Next leave the window)
//! - Select activates the focused row
//! - Back leaves a list to the main menu, or returns home from the main menu

mod action;
mod back_result;
mod controller;
mod device_row;
mod menu;
mod network_row;
mod screen;
mod snapshot;
mod view;

pub use action::Action;
pub use back_result::BackResult;
pub use controller::{Controller, SharedController};
pub use device_row::DeviceRow;
pub use menu::{
    MAIN_BT_DEVICES, MAIN_BT_POWER, MAIN_COUNT, MAIN_WIFI_NETWORKS, MAIN_WIFI_POWER, Menu,
};
pub use network_row::NetworkRow;
pub use screen::Screen;
pub use snapshot::Snapshot;
pub use view::{apply, apply_idle, build_items};

/// Window index for Connectivity (keep in sync with [`crate::windows`]).
pub const WINDOW: i32 = 5;
