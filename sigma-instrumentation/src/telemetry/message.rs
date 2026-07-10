//! Formatted vehicle telemetry for the dashboard (no CAN / IPC types).

/// UI-facing snapshot of what the cluster needs to render.
///
/// Producers (SocketCAN, IPC, candump replay, mocks) decode upstream and push
/// this message. The dashboard never sees raw frames.
#[derive(Debug, Clone, PartialEq)]
pub struct ClusterTelemetry {
    pub speed_kmh: f32,
    pub rpm: f32,
    pub gear: i8,
    pub at_redline: bool,
    pub side_stand: bool,
    pub riding_mode: String,
    pub fuel_pct: f32,
    pub coolant_c: i16,
    pub oil_c: i16,
    pub odometer: f32,
    pub trip1: f32,
    pub trip2: f32,
    pub lean_angle: f32,
    pub gforce: f32,
    pub battery_v: f32,
    pub can_load: u8,
    pub dtc: u8,
    pub abs_active: bool,
    pub tc_active: bool,
    pub heading: f32,
    pub elevation: i32,
    /// True while the producer is actively updating.
    pub signals_live: bool,
}

impl ClusterTelemetry {
    /// Parked / no-bus defaults (matches a cold XSR900-style idle).
    pub fn idle() -> Self {
        Self {
            speed_kmh: 0.0,
            rpm: 1_200.0,
            gear: 0,
            at_redline: false,
            side_stand: true,
            riding_mode: "SPORT".into(),
            fuel_pct: 62.0,
            coolant_c: 42,
            oil_c: 52,
            odometer: 1_245.0,
            trip1: 137.4,
            trip2: 42.1,
            lean_angle: 0.0,
            gforce: 0.0,
            battery_v: 13.1,
            can_load: 8,
            dtc: 0,
            abs_active: false,
            tc_active: false,
            heading: 0.0,
            elevation: 667,
            signals_live: false,
        }
    }
}
