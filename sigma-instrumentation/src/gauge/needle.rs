//! Shouldered spear-point needle paths.

use slint::SharedString;

use super::constants::{CX, CY, R};
use super::geometry::{angle_for, deg2rad};

/// Diamond needle → (left bevel, spine, right bevel, outline).
pub fn needle_paths(rpm: f32) -> (SharedString, SharedString, SharedString, SharedString) {
    let a = deg2rad(angle_for(rpm));
    let (dx, dy) = (a.cos(), a.sin());
    let (px, py) = (-a.sin(), a.cos());
    let (rin, rsh, rtip, wb, ws) = (R * 0.30, R * 0.72, R * 0.99, 10.5, 3.5);
    let q = |rad: f32, off: f32| (CX + dx * rad + px * off, CY + dy * rad + py * off);
    let (bl, sl) = (q(rin, wb), q(rsh, wb));
    let (br, sr) = (q(rin, -wb), q(rsh, -wb));
    let (cl, cr) = (q(rin, ws), q(rin, -ws));
    let tp = q(rtip, 0.0);
    let p = |v: &[(f32, f32)]| {
        let mut s = format!("M {:.2} {:.2}", v[0].0, v[0].1);
        for pt in &v[1..] {
            s.push_str(&format!(" L {:.2} {:.2}", pt.0, pt.1));
        }
        s.push_str(" Z");
        s
    };
    (
        p(&[bl, sl, tp, cl]).into(),
        p(&[cl, tp, cr]).into(),
        p(&[br, sr, tp, cr]).into(),
        p(&[bl, sl, tp, sr, br]).into(),
    )
}
