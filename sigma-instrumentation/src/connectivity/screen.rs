//! Which connectivity screen is showing.

/// Main menu / Bluetooth list / Wi-Fi list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Screen {
    /// Top-level menu (radios + list entries).
    #[default]
    Main = 0,
    /// Bluetooth device list.
    BtList = 1,
    /// Wi-Fi network list.
    WifiList = 2,
}

impl Screen {
    /// Slint-facing discriminant.
    pub fn as_i32(self) -> i32 {
        self as i32
    }
}
