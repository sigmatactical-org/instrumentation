slint::include_modules!();

use std::cell::Cell;
use std::rc::Rc;
use std::time::{Duration, Instant};

// Yamaha XSR900 GP (890 cc CP3) — red zone 11 250 r/min, ~235 km/h top speed
const IDLE_RPM: f32 = 1_200.0;
const REDLINE_RPM: f32 = 11_250.0;
const REV_LIMIT_RPM: f32 = 11_500.0;
const MAX_SPEED_KMH: f32 = 235.0;
const SHIFT_PAUSE_S: f32 = 0.10;
const TOP_SPEED_HOLD_S: f32 = 1.2;
const SETTLE_S: f32 = 1.5;

/// Upshift targets — stretch 1st, then ride the triple high.
const SHIFT_UP_BY_GEAR: [f32; 7] = [0.0, 11_000.0, 10_500.0, 10_000.0, 9_500.0, 9_000.0, 8_500.0];

/// Downshift when RPM falls below these (higher in upper gears → visible gear stepping on decel).
const SHIFT_DOWN_BY_GEAR: [f32; 7] = [0.0, 3_800.0, 4_200.0, 4_800.0, 5_500.0, 6_500.0, 7_800.0];

const PRIMARY_RATIO: f32 = 1.681;
const FINAL_DRIVE: f32 = 2.813;
const GEARBOX: [f32; 6] = [2.667, 2.000, 1.619, 1.381, 1.190, 1.037];

/// 120/70-17 rear
const WHEEL_CIRC_M: f32 = 1.88;

#[derive(Clone, Copy, PartialEq, Eq)]
enum DemoPhase {
    Launch,
    AccelRun,
    TopSpeedHold,
    DecelRun,
    Settle,
}

struct SimulationState {
    speed_kmh: Cell<f32>,
    rpm: Cell<f32>,
    gear: Cell<i32>,
    phase: Cell<DemoPhase>,
    shift_pause: Cell<f32>,
    hold_timer: Cell<f32>,
    hold_start_speed: Cell<f32>,
    settle_timer: Cell<f32>,
    last_tick: Cell<Option<Instant>>,
}

impl Default for SimulationState {
    fn default() -> Self {
        Self {
            speed_kmh: Cell::new(0.0),
            rpm: Cell::new(IDLE_RPM),
            gear: Cell::new(0),
            phase: Cell::new(DemoPhase::Launch),
            shift_pause: Cell::new(0.0),
            hold_timer: Cell::new(0.0),
            hold_start_speed: Cell::new(0.0),
            settle_timer: Cell::new(0.0),
            last_tick: Cell::new(None),
        }
    }
}

impl SimulationState {
    fn restart_run(&self) {
        self.speed_kmh.set(0.0);
        self.rpm.set(IDLE_RPM);
        self.gear.set(0);
        self.phase.set(DemoPhase::Launch);
        self.shift_pause.set(0.0);
        self.hold_timer.set(0.0);
        self.hold_start_speed.set(0.0);
        self.settle_timer.set(0.0);
    }

    fn begin_top_speed_hold(&self, speed: f32) {
        self.hold_start_speed.set(speed);
        self.hold_timer.set(TOP_SPEED_HOLD_S);
        self.phase.set(DemoPhase::TopSpeedHold);
    }

    fn step(&self, ui: &SigmaDashboard) {
        let now = Instant::now();
        let dt = self
            .last_tick
            .get()
            .map(|t| now.duration_since(t).as_secs_f32())
            .unwrap_or(0.05)
            .clamp(0.0, 0.1);
        self.last_tick.set(Some(now));

        if self.shift_pause.get() > 0.0 {
            let pause = (self.shift_pause.get() - dt).max(0.0);
            self.shift_pause.set(pause);
            self.push_ui(ui);
            return;
        }

        let phase = self.phase.get();
        let mut speed = self.speed_kmh.get();
        let mut gear = self.gear.get();
        let mut rpm = self.rpm.get();

        match phase {
            DemoPhase::Launch => {
                gear = 1;
                rpm = 2_800.0;
                self.phase.set(DemoPhase::AccelRun);
            }
            DemoPhase::AccelRun => {
                if gear == 0 {
                    gear = 1;
                }
                let accel = acceleration_ms2(speed, gear, rpm, true);
                speed = (speed + accel * dt * 3.6).clamp(0.0, MAX_SPEED_KMH);
                rpm = rpm_from_speed(speed, gear);

                if should_upshift(speed, gear, rpm, accel) {
                    gear += 1;
                    rpm = rpm_from_speed(speed, gear);
                    self.shift_pause.set(SHIFT_PAUSE_S);
                }

                if gear >= 6 {
                    let accel_now = acceleration_ms2(speed, gear, rpm, true);
                    if speed >= MAX_SPEED_KMH - 2.0 || accel_now <= 0.05 {
                        self.begin_top_speed_hold(speed);
                    }
                }
            }
            DemoPhase::TopSpeedHold => {
                gear = 6;
                let start = self.hold_start_speed.get();
                let elapsed = TOP_SPEED_HOLD_S - self.hold_timer.get();
                let t = (elapsed / TOP_SPEED_HOLD_S).clamp(0.0, 1.0);
                let ease = t * t * (3.0 - 2.0 * t);
                speed = start + (MAX_SPEED_KMH - start) * ease;
                rpm = rpm_from_speed(speed, gear);
                let mut hold = self.hold_timer.get() - dt;
                if hold <= 0.0 {
                    speed = MAX_SPEED_KMH;
                    rpm = rpm_from_speed(speed, gear);
                    hold = 0.0;
                    self.phase.set(DemoPhase::DecelRun);
                }
                self.hold_timer.set(hold);
            }
            DemoPhase::DecelRun => {
                if gear == 0 {
                    gear = 1;
                }
                let decel = acceleration_ms2(speed, gear, rpm, false);
                speed = (speed + decel * dt * 3.6).max(0.0);
                rpm = rpm_from_speed(speed, gear);

                if should_downshift(gear, rpm) {
                    gear -= 1;
                    if gear == 0 {
                        rpm = IDLE_RPM;
                    } else {
                        rpm = rpm_from_speed(speed, gear);
                    }
                    self.shift_pause.set(SHIFT_PAUSE_S);
                }

                if speed <= 0.05 {
                    speed = 0.0;
                    gear = 0;
                    rpm = IDLE_RPM;
                    self.settle_timer.set(SETTLE_S);
                    self.phase.set(DemoPhase::Settle);
                }
            }
            DemoPhase::Settle => {
                let mut t = self.settle_timer.get() - dt;
                if t <= 0.0 {
                    self.phase.set(DemoPhase::Launch);
                    t = 0.0;
                }
                self.settle_timer.set(t);
                speed = 0.0;
                gear = 0;
                rpm = IDLE_RPM;
            }
        }

        rpm = rpm.clamp(IDLE_RPM, REV_LIMIT_RPM);
        self.speed_kmh.set(speed);
        self.gear.set(gear);
        self.rpm.set(rpm);
        self.push_ui(ui);
    }

    fn push_ui(&self, ui: &SigmaDashboard) {
        ui.set_rpm(self.rpm.get());
        ui.set_speed(self.speed_kmh.get().round() as i32);
        ui.set_gear(self.gear.get().clamp(0, 6));
        ui.set_side_stand(self.speed_kmh.get() == 0.0 && self.gear.get() == 0);
    }
}

fn total_ratio(gear: i32) -> f32 {
    PRIMARY_RATIO * GEARBOX[(gear - 1) as usize] * FINAL_DRIVE
}

fn rpm_from_speed(speed_kmh: f32, gear: i32) -> f32 {
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

fn should_upshift(speed_kmh: f32, gear: i32, rpm: f32, accel: f32) -> bool {
    if gear >= 6 {
        return false;
    }
    if rpm >= shift_up_rpm(gear) {
        return true;
    }
    if accel <= 0.0 && rpm > 6_000.0 {
        return true;
    }
    speed_kmh >= gear_vmax(gear) * 0.90
}

fn should_downshift(gear: i32, rpm: f32) -> bool {
    gear > 1 && rpm < shift_down_rpm(gear)
}

fn torque_factor(rpm: f32) -> f32 {
    let low = (rpm / 3_500.0).clamp(0.0, 1.0);
    let mid = 1.0 - ((rpm - 10_000.0) / 4_000.0).clamp(0.0, 1.0).powi(2) * 0.35;
    let high = 1.0 - ((rpm - 11_000.0) / 350.0).clamp(0.0, 1.0).powi(2);
    (low * mid * high).clamp(0.12, 1.0)
}

fn acceleration_ms2(speed_kmh: f32, gear: i32, rpm: f32, throttle: bool) -> f32 {
    if gear <= 0 {
        return if throttle { 0.0 } else { -0.4 };
    }

    let v = speed_kmh / 3.6;
    let v_max = gear_vmax(gear).max(1.0);
    let v_ratio = (speed_kmh / v_max).clamp(0.0, 1.0);
    let drag = 0.28 + 0.000_42 * v * v;

    if throttle {
        let a_peak = 12.5 / (gear as f32).sqrt();
        let torque = torque_factor(rpm);
        let pull = if gear >= 6 {
            let rpm_room = (1.0 - (rpm / REDLINE_RPM).powf(1.5)).max(0.08);
            a_peak * torque * rpm_room
        } else {
            a_peak * torque * (1.0 - v_ratio.powf(1.45))
        };
        pull - drag
    } else {
        let engine_brake = 3.0 * (gear as f32).powf(0.55) * (rpm / REDLINE_RPM).powf(0.7);
        let aero_brake = 0.000_42 * v * v;
        -(engine_brake + drag * 0.5 + aero_brake)
    }
}

/// Full-screen kiosk when built for Co-Pilot (Wayland + Weston).
fn configure_kiosk(ui: &SigmaDashboard) {
    let kiosk = std::env::var("SLINT_FULLSCREEN")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(cfg!(co_pilot_embedded));

    if kiosk {
        ui.window().set_fullscreen(true);
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = SigmaDashboard::new()?;

    configure_kiosk(&ui);

    let state = Rc::new(SimulationState::default());

    {
        let state = state.clone();
        ui.on_rpm_up(move || state.restart_run());
    }
    {
        let state = state.clone();
        ui.on_rpm_down(move || {
            state.phase.set(DemoPhase::DecelRun);
        });
    }

    let timer = slint::Timer::default();
    let tick_state = state.clone();
    let tick_ui = ui.as_weak();
    timer.start(
        slint::TimerMode::Repeated,
        Duration::from_millis(50),
        move || {
            if let Some(ui) = tick_ui.upgrade() {
                tick_state.step(&ui);
            }
        },
    );

    ui.run()
}
