//! What the Back button did.

/// Result of [`super::Menu::back`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackResult {
    /// Went up one level inside the window.
    Stay,
    /// Was already at the main menu — leave the window.
    LeaveWindow,
}
