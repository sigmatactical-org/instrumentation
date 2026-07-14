//! Slint binding helpers: render menu + snapshot into the dashboard.

use super::menu::{MAIN_BT_DEVICES, MAIN_BT_POWER, MAIN_WIFI_NETWORKS, MAIN_WIFI_POWER, Menu};
use super::screen::Screen;
use super::snapshot::Snapshot;
use crate::{ConnItem, SigmaDashboard};
use slint::{ModelRc, SharedString, VecModel};

/// Build one focusable row.
fn item(title: &str, detail: &str, badge: &str, focused: bool) -> ConnItem {
    ConnItem {
        title: SharedString::from(title),
        detail: SharedString::from(detail),
        badge: SharedString::from(badge),
        focused,
    }
}

/// Rows for the current screen with the focus flag set.
pub fn build_items(menu: &Menu, snap: &Snapshot) -> Vec<ConnItem> {
    match menu.screen {
        Screen::Main => {
            let bt_badge = if snap.bt_powered { "ON" } else { "OFF" };
            let wifi_badge = if snap.wifi_powered { "ON" } else { "OFF" };
            let bt_detail = if snap.bt_connected {
                snap.bt_device.as_str()
            } else if snap.bt_powered {
                "No headset connected"
            } else {
                "Radio off"
            };
            let wifi_detail = if snap.wifi_connected {
                snap.wifi_ssid.as_str()
            } else if snap.wifi_powered {
                "Not associated"
            } else {
                "Radio off"
            };
            vec![
                item(
                    "Bluetooth",
                    bt_detail,
                    bt_badge,
                    menu.focus == MAIN_BT_POWER,
                ),
                item(
                    "Devices",
                    "Pair or connect headset",
                    "",
                    menu.focus == MAIN_BT_DEVICES,
                ),
                item(
                    "Wi-Fi",
                    wifi_detail,
                    wifi_badge,
                    menu.focus == MAIN_WIFI_POWER,
                ),
                item(
                    "Networks",
                    "Join a saved or open network",
                    "",
                    menu.focus == MAIN_WIFI_NETWORKS,
                ),
            ]
        }
        Screen::BtList => {
            let mut rows: Vec<ConnItem> = snap
                .devices
                .iter()
                .enumerate()
                .map(|(i, d)| item(&d.title, &d.detail, &d.badge, menu.focus == i))
                .collect();
            let scan_i = rows.len();
            rows.push(item(
                "Scan for headsets",
                "Discover nearby Bluetooth devices",
                "",
                menu.focus == scan_i,
            ));
            rows
        }
        Screen::WifiList => {
            let mut rows: Vec<ConnItem> = snap
                .networks
                .iter()
                .enumerate()
                .map(|(i, n)| item(&n.title, &n.detail, &n.badge, menu.focus == i))
                .collect();
            let scan_i = rows.len();
            rows.push(item(
                "Rescan networks",
                "Refresh Wi-Fi scan results",
                "",
                menu.focus == scan_i,
            ));
            rows
        }
    }
}

/// Push snapshot + menu state onto the dashboard.
pub fn apply(ui: &SigmaDashboard, menu: &Menu, snap: &Snapshot) {
    ui.set_conn_screen(menu.screen.as_i32());
    ui.set_conn_busy(snap.busy);
    ui.set_bt_powered(snap.bt_powered);
    ui.set_bt_connected(snap.bt_connected);
    ui.set_bt_device(SharedString::from(if snap.bt_device.is_empty() {
        "—"
    } else {
        snap.bt_device.as_str()
    }));
    ui.set_bt_battery(snap.bt_battery);
    ui.set_wifi_powered(snap.wifi_powered);
    ui.set_wifi_connected(snap.wifi_connected);
    ui.set_wifi_ssid(SharedString::from(if snap.wifi_ssid.is_empty() {
        "—"
    } else {
        snap.wifi_ssid.as_str()
    }));

    let status = if !snap.available && snap.status.is_empty() {
        "BlueZ / connman unavailable — connect radios on the device image."
    } else {
        snap.status.as_str()
    };
    ui.set_conn_status(SharedString::from(status));

    let items = build_items(menu, snap);
    ui.set_conn_items(ModelRc::new(VecModel::from(items)));
}

/// Empty idle state for startup before the first poll.
pub fn apply_idle(ui: &SigmaDashboard) {
    let menu = Menu::default();
    let snap = Snapshot {
        status: "Starting connectivity…".into(),
        bt_battery: -1,
        ..Snapshot::default()
    };
    apply(ui, &menu, &snap);
}
