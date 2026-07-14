//! One release entry from the updates catalog.

use serde::Deserialize;

/// The latest release published on a catalog channel.
#[derive(Debug, Clone, Deserialize)]
pub struct ChannelRelease {
    /// Channel name (`dev`, `stable`, …).
    pub channel: String,
    /// Release version string.
    pub version: String,
    /// Human-readable release notes.
    #[serde(default)]
    pub notes: String,
    /// Optional install hint from the catalog.
    #[serde(default)]
    pub install: String,
    /// Direct RAUC bundle URL; derived from the catalog layout when empty.
    #[serde(default)]
    pub bundle_url: String,
}
