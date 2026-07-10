//! Map decoded [`VehicleState`] → UI [`ClusterTelemetry`].

use sigma_instrumentation::ClusterTelemetry;
use sigma_racer_telemetry::VehicleState;

pub fn to_cluster(state: &VehicleState) -> ClusterTelemetry {
    ClusterTelemetry {
        speed_kmh: state.speed,
        rpm: state.rpm,
        gear: state.gear,
        at_redline: state.at_redline,
        side_stand: state.side_stand,
        riding_mode: state.riding_mode.clone(),
        fuel_pct: state.fuel_pct,
        coolant_c: state.coolant_c,
        oil_c: state.oil_c,
        odometer: state.odometer,
        trip1: state.trip1,
        trip2: state.trip2,
        lean_angle: state.lean_angle,
        gforce: state.gforce,
        battery_v: state.battery_v,
        can_load: state.can_load,
        dtc: state.dtc,
        abs_active: state.abs_active,
        tc_active: state.tc_active,
        heading: state.heading,
        elevation: state.elevation,
        signals_live: state.signals_live,
    }
}
