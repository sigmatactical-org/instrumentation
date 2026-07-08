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
    let ts = if s == 0 {
        SharedString::from("0")
    } else if s >= 10 {
        SharedString::from(format!("{t}"))
    } else {
        SharedString::from("")
    };
    let os = if s == 0 {
        SharedString::from("")
    } else {
        SharedString::from(format!("{o}"))
    };
    (hs, ts, os)
}
