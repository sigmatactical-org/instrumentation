//! Bind [`ClusterTelemetry`] onto Slint dashboard properties.

use slint::SharedString;

use crate::dashboard::{set_needle_paths, set_speed_readout};
use crate::gauge::GaugeScale;
use crate::heading;
use crate::SigmaDashboard;

use super::message::ClusterTelemetry;
use super::presenter::TelemetryPresenter;

/// Push a formatted telemetry message onto the dashboard.
pub fn apply_telemetry(ui: &SigmaDashboard, msg: &ClusterTelemetry, gauge: &GaugeScale) {
    ui.set_rpm(msg.rpm);
    set_speed_readout(ui, msg.speed_kmh.round() as i32);
    ui.set_gear(i32::from(msg.gear));
    ui.set_gear_label(SharedString::from(if msg.gear == 0 {
        "N".to_owned()
    } else {
        msg.gear.to_string()
    }));
    ui.set_at_redline(msg.at_redline);
    ui.set_side_stand(msg.side_stand);
    ui.set_riding_mode(SharedString::from(msg.riding_mode.as_str()));
    ui.set_fuel_pct(msg.fuel_pct / 100.0);
    ui.set_coolant_c(i32::from(msg.coolant_c));
    ui.set_oil_c(i32::from(msg.oil_c));
    ui.set_odometer(msg.odometer.round() as i32);
    ui.set_trip1(msg.trip1);
    ui.set_trip2(msg.trip2);
    ui.set_lean_angle(msg.lean_angle);
    ui.set_gforce(msg.gforce);
    ui.set_battery_v(msg.battery_v);
    ui.set_can_load(i32::from(msg.can_load));
    ui.set_dtc(i32::from(msg.dtc));
    ui.set_abs_active(msg.abs_active);
    ui.set_tc_active(msg.tc_active);
    ui.set_heading(msg.heading);
    ui.set_heading_label(SharedString::from(heading::heading_label(msg.heading)));
    ui.set_elevation(msg.elevation);
    ui.set_telemetry_live(msg.signals_live);

    set_needle_paths(ui, gauge, msg.rpm);
}

impl TelemetryPresenter for ClusterTelemetry {
    fn present(&self, ui: &SigmaDashboard, gauge: &GaugeScale) {
        apply_telemetry(ui, self, gauge);
    }
}
