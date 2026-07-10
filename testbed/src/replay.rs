//! Candump replay that emits formatted [`ClusterTelemetry`] for the dashboard.

use chrono::{Local, Timelike};
use sigma_instrumentation::{
    apply_telemetry, windows, ClusterTelemetry, GaugeScale, SigmaDashboard, TelemetryPresenter,
};
use sigma_racer_telemetry::can::{decode_frame, parse_candump, CandumpFrame};
use sigma_racer_telemetry::VehicleState;
use slint::SharedString;
use std::cell::{Cell, RefCell};
use std::path::{Path, PathBuf};
use std::time::Instant;

use crate::map::to_cluster;

const GAUGE: GaugeScale = GaugeScale::DEFAULT;
const SAMPLE_LOG: &str =
    include_str!("../../../sigma-racer-cluster/testdata/sample-ride.log");

pub struct CandumpReplay {
    frames: RefCell<Vec<CandumpFrame>>,
    path_label: RefCell<String>,
    state: RefCell<VehicleState>,
    cursor: Cell<usize>,
    last_tick: Cell<Option<Instant>>,
    /// Simulated seconds into the log (advances by dt × rate).
    sim_t: Cell<f64>,
    rate: Cell<f32>,
    window: Cell<i32>,
    last_clock_min: Cell<i32>,
}

impl Default for CandumpReplay {
    fn default() -> Self {
        let frames = parse_candump(SAMPLE_LOG);
        Self {
            frames: RefCell::new(frames),
            path_label: RefCell::new("(baked sample)".into()),
            state: RefCell::new(VehicleState::idle()),
            cursor: Cell::new(0),
            last_tick: Cell::new(None),
            sim_t: Cell::new(0.0),
            rate: Cell::new(1.0),
            window: Cell::new(0),
            last_clock_min: Cell::new(-1),
        }
    }
}

impl CandumpReplay {
    pub fn path_label(&self) -> String {
        self.path_label.borrow().clone()
    }

    pub fn rate(&self) -> f32 {
        self.rate.get()
    }

    pub fn set_rate(&self, rate: f32) {
        self.rate.set(rate.clamp(0.25, 4.0));
    }

    pub fn load_path(&self, path: &Path) -> Result<(), String> {
        let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let frames = parse_candump(&text);
        if frames.is_empty() {
            return Err("no usable candump frames".into());
        }
        let label = path
            .file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.to_owned())
            .unwrap_or_else(|| path.display().to_string());
        self.replace_frames(frames, label);
        Ok(())
    }

    pub fn restart(&self) {
        let label = self.path_label.borrow().clone();
        let frames = self.frames.borrow().clone();
        self.replace_frames(frames, label);
    }

    fn replace_frames(&self, frames: Vec<CandumpFrame>, label: String) {
        *self.frames.borrow_mut() = frames;
        *self.path_label.borrow_mut() = label;
        *self.state.borrow_mut() = VehicleState::idle();
        self.cursor.set(0);
        self.sim_t.set(0.0);
        self.last_tick.set(None);
    }

    fn stopped(&self) -> bool {
        self.state.borrow().speed < 0.5
    }

    pub fn nav_next(&self) {
        let cur = self.window.get();
        let next = if self.stopped() {
            (cur + 1).rem_euclid(windows::COUNT)
        } else {
            (cur.clamp(0, windows::PANEL_MAX) + 1).rem_euclid(windows::PANEL_MAX + 1)
        };
        self.window.set(next);
    }

    pub fn nav_prev(&self) {
        let cur = self.window.get();
        let prev = if self.stopped() {
            (cur - 1).rem_euclid(windows::COUNT)
        } else {
            (cur.clamp(0, windows::PANEL_MAX) - 1).rem_euclid(windows::PANEL_MAX + 1)
        };
        self.window.set(prev);
    }

    pub fn nav_home(&self) {
        self.window.set(0);
    }

    pub fn nav_select(&self, idx: i32) {
        if !(0..windows::COUNT).contains(&idx) {
            return;
        }
        if idx > windows::PANEL_MAX && !self.stopped() {
            return;
        }
        self.window.set(idx);
    }

    /// Advance replay by one UI tick and present formatted telemetry.
    pub fn step(&self, ui: &SigmaDashboard) {
        let now = Instant::now();
        let dt = match self.last_tick.get() {
            Some(prev) => now.duration_since(prev).as_secs_f64().min(0.05),
            None => 0.0,
        };
        self.last_tick.set(Some(now));

        let rate = f64::from(self.rate.get());
        let mut t = self.sim_t.get() + dt * rate;

        let frames = self.frames.borrow();
        if frames.is_empty() {
            let mut idle = ClusterTelemetry::idle();
            idle.signals_live = false;
            idle.present(ui, &GAUGE);
            return;
        }

        let mut cursor = self.cursor.get();
        let mut state = self.state.borrow_mut();

        if cursor >= frames.len() {
            cursor = 0;
            t = 0.0;
            *state = VehicleState::idle();
        }

        while cursor < frames.len() && frames[cursor].at <= t {
            let f = &frames[cursor];
            decode_frame(f.id, &f.data, &mut state);
            cursor += 1;
        }
        state.refresh_derived();
        state.signals_live = true;

        self.cursor.set(cursor);
        self.sim_t.set(t);

        let mut msg = to_cluster(&state);
        msg.signals_live = true;
        apply_telemetry(ui, &msg, &GAUGE);

        ui.set_current_window(self.window.get());

        let clock = Local::now();
        let minute = clock.minute() as i32;
        if minute != self.last_clock_min.get() {
            self.last_clock_min.set(minute);
            ui.set_clock(SharedString::from(clock.format("%H:%M").to_string()));
        }
    }
}

/// Open a native file dialog and return the chosen path.
pub fn pick_candump_file() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("candump log", &["log", "txt", "candump"])
        .add_filter("All", &["*"])
        .set_title("Select candump -L log")
        .pick_file()
}
