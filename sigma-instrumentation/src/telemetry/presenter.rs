//! Presenter trait — anything that can drive the dashboard from formatted data.

use crate::SigmaDashboard;
use crate::gauge::GaugeScale;

/// Push formatted telemetry onto the Slint dashboard.
///
/// Implement for [`super::ClusterTelemetry`] (and mocks in tests). Producers
/// convert their domain objects into a presenter and call [`present`](Self::present).
pub trait TelemetryPresenter {
    fn present(&self, ui: &SigmaDashboard, gauge: &GaugeScale);
}
