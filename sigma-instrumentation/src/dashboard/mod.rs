//! Static gauge artwork and readout helpers bound to a dashboard instance.

mod gauge_art;
mod speed;

pub use gauge_art::init_gauge_art;
pub use speed::{set_speed_readout, speed_digits};
