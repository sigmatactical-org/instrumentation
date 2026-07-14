//! Shared menu + snapshot handle.

use super::menu::Menu;
use super::snapshot::Snapshot;
use super::view::apply;
use crate::SigmaDashboard;
use std::rc::Rc;

/// Shared handle used by nav + poll timers.
pub struct Controller {
    /// Focus model.
    pub menu: Menu,
    /// Latest backend snapshot.
    pub snap: Snapshot,
}

impl Controller {
    /// Fresh controller with an unknown-battery snapshot.
    pub fn new() -> Self {
        Self {
            menu: Menu::default(),
            snap: Snapshot {
                bt_battery: -1,
                status: String::new(),
                ..Snapshot::default()
            },
        }
    }

    /// Push the current menu + snapshot onto the dashboard.
    pub fn paint(&self, ui: &SigmaDashboard) {
        apply(ui, &self.menu, &self.snap);
    }
}

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience for tests / callers that want an Rc.
pub type SharedController = Rc<std::cell::RefCell<Controller>>;
