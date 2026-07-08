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
            "day" | "bright" | "daylight" => Self::Day,
            "dusk" | "twilight" | "normal" | "default" | "std" => Self::Dusk,
            _ => Self::Night,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Dusk => "dusk",
            Self::Night => "night",
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
    DisplayMode::Night
}
