//! Focus model for the three connectivity screens.

use super::action::Action;
use super::back_result::BackResult;
use super::screen::Screen;
use super::snapshot::Snapshot;

/// Fixed main-menu rows.
pub const MAIN_BT_POWER: usize = 0;
/// "Devices" row.
pub const MAIN_BT_DEVICES: usize = 1;
/// "Wi-Fi" row.
pub const MAIN_WIFI_POWER: usize = 2;
/// "Networks" row.
pub const MAIN_WIFI_NETWORKS: usize = 3;
/// Number of main-menu rows.
pub const MAIN_COUNT: usize = 4;

/// Current screen plus the focused row on it.
#[derive(Debug, Clone, Default)]
pub struct Menu {
    /// Which screen is showing.
    pub screen: Screen,
    /// Focused row index on that screen.
    pub focus: usize,
}

impl Menu {
    /// Return to the main menu with focus at the top.
    pub fn reset(&mut self) {
        self.screen = Screen::Main;
        self.focus = 0;
    }

    /// Number of focusable rows on the current screen.
    pub fn item_count(&self, snap: &Snapshot) -> usize {
        match self.screen {
            Screen::Main => MAIN_COUNT,
            Screen::BtList => snap.devices.len().saturating_add(1), // + Scan
            Screen::WifiList => snap.networks.len().saturating_add(1), // + Rescan
        }
    }

    /// Move focus by `delta`. Returns `Some(window_delta)` when leaving the window
    /// at the first/last edge on the main menu.
    pub fn move_focus(&mut self, snap: &Snapshot, delta: i32) -> Option<i32> {
        let count = self.item_count(snap).max(1);
        if self.screen == Screen::Main {
            if delta < 0 && self.focus == 0 {
                return Some(-1);
            }
            if delta > 0 && self.focus + 1 >= count {
                return Some(1);
            }
        }
        let next = self.focus as i32 + delta;
        self.focus = next.clamp(0, count as i32 - 1) as usize;
        None
    }

    /// Handle Back: leave a list for the main menu, or leave the window.
    pub fn back(&mut self) -> BackResult {
        match self.screen {
            Screen::Main => BackResult::LeaveWindow,
            Screen::BtList | Screen::WifiList => {
                self.screen = Screen::Main;
                self.focus = 0;
                BackResult::Stay
            }
        }
    }

    /// Activate the focused row, entering lists as a side effect.
    pub fn select(&mut self, snap: &Snapshot) -> Option<Action> {
        match self.screen {
            Screen::Main => match self.focus {
                MAIN_BT_POWER => Some(Action::ToggleBt),
                MAIN_BT_DEVICES => {
                    self.screen = Screen::BtList;
                    self.focus = 0;
                    Some(Action::OpenBtList)
                }
                MAIN_WIFI_POWER => Some(Action::ToggleWifi),
                MAIN_WIFI_NETWORKS => {
                    self.screen = Screen::WifiList;
                    self.focus = 0;
                    Some(Action::OpenWifiList)
                }
                _ => None,
            },
            Screen::BtList => {
                let n = snap.devices.len();
                if self.focus < n {
                    Some(Action::SelectDevice(self.focus))
                } else {
                    Some(Action::BtScan)
                }
            }
            Screen::WifiList => {
                let n = snap.networks.len();
                if self.focus < n {
                    Some(Action::SelectNetwork(self.focus))
                } else {
                    Some(Action::WifiScan)
                }
            }
        }
    }
}
