//! Optional framebuffer dump for UI debugging (`SIGMA_SNAPSHOT=/tmp/out.png`).

use slint::ComponentHandle;

use crate::SigmaDashboard;

pub fn dump(ui: &SigmaDashboard, path: &str) -> bool {
    match ui.window().take_snapshot() {
        Ok(buf) => {
            if let Err(err) = image::save_buffer(
                path,
                buf.as_bytes(),
                buf.width(),
                buf.height(),
                image::ColorType::Rgba8,
            ) {
                eprintln!("sigma snapshot: write {path}: {err}");
                return false;
            }
            eprintln!(
                "sigma snapshot: wrote {path} ({}×{}) scale={}",
                buf.width(),
                buf.height(),
                ui.window().scale_factor()
            );
            true
        }
        Err(err) => {
            eprintln!("sigma snapshot: take_snapshot failed: {err}");
            false
        }
    }
}
