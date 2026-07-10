//! Speed readout digit helpers.

use slint::SharedString;

use crate::SigmaDashboard;

pub fn set_speed_readout(ui: &SigmaDashboard, speed: i32) {
    let (h, t, o) = speed_digits(speed);
    ui.set_speed(speed);
    ui.set_d_hundreds(h);
    ui.set_d_tens(t);
    ui.set_d_ones(o);
}

/// Split speed into hundreds / tens / ones digit cells (right-aligned, 0–999 km/h).
pub fn speed_digits(speed: i32) -> (SharedString, SharedString, SharedString) {
    let s = speed.clamp(0, 999);
    let h = s / 100;
    let t = (s / 10) % 10;
    let o = s % 10;
    let hs = if s >= 100 {
        SharedString::from(format!("{h}"))
    } else {
        SharedString::from("")
    };
    let ts = if s >= 10 {
        SharedString::from(format!("{t}"))
    } else {
        SharedString::from("")
    };
    // Always render the ones column so 0–9 km/h and 0 at rest stay visible.
    let os = SharedString::from(format!("{o}"));
    (hs, ts, os)
}

#[cfg(test)]
mod tests {
    use super::speed_digits;

    #[test]
    fn zero_shows_in_ones_column() {
        let (h, t, o) = speed_digits(0);
        assert_eq!(h.as_str(), "");
        assert_eq!(t.as_str(), "");
        assert_eq!(o.as_str(), "0");
    }

    #[test]
    fn single_digit_speed() {
        let (h, t, o) = speed_digits(7);
        assert_eq!(h.as_str(), "");
        assert_eq!(t.as_str(), "");
        assert_eq!(o.as_str(), "7");
    }

    #[test]
    fn three_digit_speed() {
        let (h, t, o) = speed_digits(235);
        assert_eq!(h.as_str(), "2");
        assert_eq!(t.as_str(), "3");
        assert_eq!(o.as_str(), "5");
    }
}
