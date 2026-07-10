//! Day / dusk / night display modes.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisplayMode {
    Day,
    Dusk,
    Night,
}

impl DisplayMode {
    pub fn parse(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "dusk" | "twilight" => Self::Dusk,
            "night" | "dark" => Self::Night,
            // day / bright / daylight / default / unset aliases
            _ => Self::Day,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Dusk => "dusk",
            Self::Night => "night",
        }
    }

    /// Normal ambient cycle: day → dusk → night → day.
    pub fn cycle(self) -> Self {
        match self {
            Self::Day => Self::Dusk,
            Self::Dusk => Self::Night,
            Self::Night => Self::Day,
        }
    }
}

pub fn parse_mode_from_env() -> DisplayMode {
    if let Ok(v) = std::env::var("SIGMA_DISPLAY_MODE") {
        return DisplayMode::parse(&v);
    }
    if let Ok(v) = std::env::var("SIGMA_UI_TONE") {
        return DisplayMode::parse(&v);
    }
    DisplayMode::Day
}
