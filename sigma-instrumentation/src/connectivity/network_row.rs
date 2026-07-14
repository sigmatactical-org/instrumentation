//! One Wi-Fi network in the list.

/// A visible Wi-Fi service as shown in the network list.
#[derive(Debug, Clone, Default)]
pub struct NetworkRow {
    /// D-Bus object path (selection target).
    pub path: String,
    /// SSID.
    pub title: String,
    /// Secondary line (state, strength, …).
    pub detail: String,
    /// Short status badge (e.g. `SECURE`).
    pub badge: String,
    /// Currently connected.
    pub connected: bool,
    /// Known/provisioned network.
    pub favorite: bool,
}
