//! Yamaha XSR900 GP ride simulation for the testbed.
//!
//! - [`phase`] — the [`DemoPhase`](phase::DemoPhase) state machine
//! - [`physics`] — the drivetrain model (gearing, shift points, acceleration)
//! - [`simulation`] — the [`RideSimulation`] that scripts a run and pushes it
//!   onto the dashboard

mod phase;
mod physics;
mod simulation;

pub use simulation::RideSimulation;
