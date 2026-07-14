//! One Bluetooth device in the list.

/// A known Bluetooth device as shown in the device list.
#[derive(Debug, Clone, Default)]
pub struct DeviceRow {
    /// D-Bus object path (selection target).
    pub path: String,
    /// Display name.
    pub title: String,
    /// Secondary line (state, battery, …).
    pub detail: String,
    /// Short status badge (e.g. `CONNECTED`).
    pub badge: String,
    /// Currently connected.
    pub connected: bool,
    /// Previously paired.
    pub paired: bool,
}
