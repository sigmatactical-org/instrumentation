//! The demo ride's state machine phases.

/// Stages of the scripted XSR900 GP demo run.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DemoPhase {
    Launch,
    AccelRun,
    TopSpeedHold,
    DecelRun,
    Settle,
}
