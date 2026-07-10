//! Formatted telemetry → dashboard binding (CAN/IPC stay outside this crate).

mod apply;
mod message;
mod presenter;

pub use apply::apply_telemetry;
pub use message::ClusterTelemetry;
pub use presenter::TelemetryPresenter;
