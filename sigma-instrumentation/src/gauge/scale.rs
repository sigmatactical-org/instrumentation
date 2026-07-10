//! RPM dial scale — max and redline drive tick density and angular mapping.

/// Default full-scale RPM for the XSR900 GP dial (12k sweep).
pub const DEFAULT_MAX_RPM: f32 = 12_000.0;
/// Default redline RPM for the XSR900 GP dial.
pub const DEFAULT_REDLINE_RPM: f32 = 11_250.0;

/// Angular mapping and tick layout for one tach sweep.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GaugeScale {
    pub max_rpm: f32,
    pub redline_rpm: f32,
}

impl GaugeScale {
    pub const DEFAULT: Self = Self {
        max_rpm: DEFAULT_MAX_RPM,
        redline_rpm: DEFAULT_REDLINE_RPM,
    };

    pub fn new(max_rpm: f32, redline_rpm: f32) -> Self {
        Self {
            max_rpm: max_rpm.max(1.0),
            redline_rpm: redline_rpm.clamp(1.0, max_rpm.max(1.0)),
        }
    }

    /// RPM where the red tick arc begins — just below [`redline_rpm`](Self::redline_rpm).
    pub fn redline_zone(&self) -> f32 {
        let margin = (self.max_rpm - self.redline_rpm).max(self.major_step() * 0.5);
        (self.redline_rpm - margin).clamp(0.0, self.max_rpm)
    }

    /// Major numeral / tick spacing (500, 1000, or 2000 RPM).
    pub fn major_step(&self) -> f32 {
        let target = self.max_rpm / 12.0;
        if target <= 625.0 {
            500.0
        } else if target <= 1_250.0 {
            1_000.0
        } else {
            2_000.0
        }
    }

    pub fn minor_step(&self) -> f32 {
        self.major_step() / 2.0
    }

    /// Caption under the dial hub.
    pub fn caption(&self) -> String {
        let step = self.major_step();
        if (step - 1_000.0).abs() < f32::EPSILON {
            "RPM × 1000".into()
        } else {
            format!("RPM × {}", step as i32)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GaugeScale;

    #[test]
    fn default_matches_xsr900_dial() {
        let s = GaugeScale::DEFAULT;
        assert_eq!(s.max_rpm, 12_000.0);
        assert_eq!(s.redline_rpm, 11_250.0);
        assert_eq!(s.major_step(), 1_000.0);
        assert_eq!(s.caption(), "RPM × 1000");
    }

    #[test]
    fn clamps_redline_to_max() {
        let s = GaugeScale::new(8_000.0, 9_000.0);
        assert_eq!(s.redline_rpm, 8_000.0);
    }

    #[test]
    fn picks_five_hundred_step_for_compact_sweep() {
        let s = GaugeScale::new(6_000.0, 5_500.0);
        assert_eq!(s.major_step(), 500.0);
        assert_eq!(s.caption(), "RPM × 500");
    }
}
