//! Yamaha XSR900 GP drivetrain model — gearing, shift points, and the
//! longitudinal acceleration used to drive the demo ride.

use sigma_instrumentation::gauge;

// Yamaha XSR900 GP (890 cc CP3) — red zone 11 250 r/min, ~235 km/h top speed
pub const IDLE_RPM: f32 = 1_200.0;
pub const REV_LIMIT_RPM: f32 = 11_500.0;
pub const MAX_SPEED_KMH: f32 = 235.0;

const DRAG_PLATEAU_ACCEL: f32 = 0.08;
const VMAX_UPSHIFT_RATIO: f32 = 0.87;

const SHIFT_UP_BY_GEAR: [f32; 7] = [0.0, 11_300.0, 11_100.0, 10_350.0, 10_100.0, 9_400.0, 8_500.0];
const SHIFT_DOWN_BY_GEAR: [f32; 7] = [0.0, 3_800.0, 4_200.0, 4_800.0, 5_500.0, 6_500.0, 7_800.0];

const PRIMARY_RATIO: f32 = 1.681;
const FINAL_DRIVE: f32 = 2.813;
const GEARBOX: [f32; 6] = [2.667, 2.000, 1.619, 1.381, 1.190, 1.037];
const WHEEL_CIRC_M: f32 = 1.88;

fn total_ratio(gear: i32) -> f32 {
    debug_assert!(
        (1..=6).contains(&gear),
        "total_ratio: gear must be 1..=6, got {gear}"
    );
    if !(1..=6).contains(&gear) {
        return 1.0;
    }
    PRIMARY_RATIO * GEARBOX[(gear - 1) as usize] * FINAL_DRIVE
}

pub fn rpm_from_speed(speed_kmh: f32, gear: i32) -> f32 {
    if gear <= 0 {
        return IDLE_RPM;
    }
    let wheel_rpm = speed_kmh / 3.6 / WHEEL_CIRC_M * 60.0;
    (wheel_rpm * total_ratio(gear)).clamp(IDLE_RPM, REV_LIMIT_RPM)
}

fn gear_vmax(gear: i32) -> f32 {
    if gear <= 0 {
        return 0.0;
    }
    REV_LIMIT_RPM / total_ratio(gear) * WHEEL_CIRC_M / 60.0 * 3.6
}

fn shift_up_rpm(gear: i32) -> f32 {
    SHIFT_UP_BY_GEAR[gear.clamp(0, 6) as usize]
}

fn shift_down_rpm(gear: i32) -> f32 {
    SHIFT_DOWN_BY_GEAR[gear.clamp(0, 6) as usize]
}

pub fn should_upshift(speed_kmh: f32, gear: i32, rpm: f32, accel: f32) -> bool {
    if gear >= 6 {
        return false;
    }
    let target = shift_up_rpm(gear);
    if rpm >= target {
        return true;
    }
    if accel <= DRAG_PLATEAU_ACCEL && rpm > 6_000.0 {
        return true;
    }
    if rpm >= target * 0.96 && accel < 0.25 {
        return true;
    }
    speed_kmh >= gear_vmax(gear) * VMAX_UPSHIFT_RATIO
}

pub fn should_downshift(gear: i32, rpm: f32) -> bool {
    gear > 1 && rpm < shift_down_rpm(gear)
}

fn torque_factor(rpm: f32) -> f32 {
    let low = (rpm / 3_500.0).clamp(0.0, 1.0);
    let mid = 1.0 - ((rpm - 10_000.0) / 4_000.0).clamp(0.0, 1.0).powi(2) * 0.35;
    let high = 1.0 - ((rpm - 11_000.0) / 350.0).clamp(0.0, 1.0).powi(2);
    (low * mid * high).clamp(0.12, 1.0)
}

pub fn acceleration_ms2(speed_kmh: f32, gear: i32, rpm: f32, throttle: bool) -> f32 {
    if gear <= 0 {
        return if throttle { 0.0 } else { -0.4 };
    }

    let v = speed_kmh / 3.6;
    let v_max = gear_vmax(gear).max(1.0);
    let v_ratio = (speed_kmh / v_max).clamp(0.0, 1.0);
    let drag = 0.28 + 0.000_42 * v * v;
    let redline = gauge::REDLINE;

    if throttle {
        let a_peak = 12.5 / (gear as f32).sqrt();
        let torque = torque_factor(rpm);
        let pull = if gear >= 6 {
            let rpm_room = (1.0 - (rpm / redline).powf(1.5)).max(0.08);
            a_peak * torque * rpm_room
        } else {
            a_peak * torque * (1.0 - v_ratio.powf(1.45))
        };
        pull - drag
    } else {
        let engine_brake = 3.0 * (gear as f32).powf(0.55) * (rpm / redline).powf(0.7);
        let aero_brake = 0.000_42 * v * v;
        -(engine_brake + drag * 0.5 + aero_brake)
    }
}
