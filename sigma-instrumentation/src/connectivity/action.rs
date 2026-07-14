//! Actions the menu can request from the backends.

/// A user-selected operation, executed by the platform backends.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Toggle Bluetooth power.
    ToggleBt,
    /// Enter the device list.
    OpenBtList,
    /// Toggle Wi-Fi power.
    ToggleWifi,
    /// Enter the network list.
    OpenWifiList,
    /// Discover Bluetooth devices.
    BtScan,
    /// Rescan Wi-Fi networks.
    WifiScan,
    /// Index into [`super::Snapshot::devices`].
    SelectDevice(usize),
    /// Index into [`super::Snapshot::networks`].
    SelectNetwork(usize),
}
