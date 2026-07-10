//! Static gauge artwork and readout helpers bound to a dashboard instance.

mod gauge_art;
mod needle;
mod signal_blink;
mod speed;

pub use gauge_art::init_gauge_art;
pub use needle::set_needle_paths;
pub use signal_blink::start_signal_blink;
pub use speed::{set_speed_readout, speed_digits};
