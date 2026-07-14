//! Point-in-time view of both radios.

use super::device_row::DeviceRow;
use super::network_row::NetworkRow;

/// Everything the Connectivity window renders, refreshed by the poll timer.
#[derive(Debug, Clone, Default)]
pub struct Snapshot {
    /// Bluetooth adapter powered.
    pub bt_powered: bool,
    /// A headset is connected.
    pub bt_connected: bool,
    /// Connected headset name.
    pub bt_device: String,
    /// Headset battery percent, `-1` when unknown.
    pub bt_battery: i32,
    /// Wi-Fi radio powered.
    pub wifi_powered: bool,
    /// Associated with a network.
    pub wifi_connected: bool,
    /// Associated SSID.
    pub wifi_ssid: String,
    /// Bluetooth device list.
    pub devices: Vec<DeviceRow>,
    /// Wi-Fi network list.
    pub networks: Vec<NetworkRow>,
    /// Status line (action results, backend errors).
    pub status: String,
    /// An action is in flight.
    pub busy: bool,
    /// At least one backend is reachable.
    pub available: bool,
}
