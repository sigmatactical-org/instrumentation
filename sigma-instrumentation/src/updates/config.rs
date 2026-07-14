//! Updates catalog endpoint configuration.

/// Where to poll for updates and what is currently installed.
#[derive(Debug, Clone)]
pub struct UpdatesConfig {
    /// Catalog base URL (no trailing slash).
    pub base_url: String,
    /// Release channel to follow.
    pub channel: String,
    /// Version currently running (drives the "newer" comparison).
    pub current_version: String,
}

impl UpdatesConfig {
    /// Read `SIGMA_UPDATES_URL` / `SIGMA_UPDATES_CHANNEL` / `SIGMA_IMAGE_VERSION`
    /// with lab-friendly defaults.
    pub fn from_env() -> Self {
        Self {
            base_url: std::env::var("SIGMA_UPDATES_URL")
                .unwrap_or_else(|_| "http://updates.sigma.localtest.me:30080".into())
                .trim_end_matches('/')
                .to_owned(),
            channel: std::env::var("SIGMA_UPDATES_CHANNEL").unwrap_or_else(|_| "dev".into()),
            current_version: std::env::var("SIGMA_IMAGE_VERSION")
                .unwrap_or_else(|_| "0.0.0".into()),
        }
    }

    /// Catalog endpoint for the channel's latest release.
    pub fn latest_url(&self) -> String {
        format!("{}/v1/channel/{}/latest", self.base_url, self.channel)
    }
}
