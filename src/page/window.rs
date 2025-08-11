use std::{pin::Pin, task::Poll};

use pasts::Notifier;

use crate::{Action, Exec, Menu, Widget};

/// Graphical window.
pub struct Window {
    /* window::Window, */
    widgets: Vec<Widget>,
    pub(crate) menus: usize,
}

impl Window {
    /// Open a window
    pub fn new(self) -> Window {
        Window {
            widgets: Vec::new(),
            menus: 0,
        }
    }

    /// Create a canvas for the window
    ///
    /// The canvas will be created the same size as the window.
    pub fn canvas(&mut self) {
        todo!("Tried creating a canvas");
    }

    /// Set the window label
    ///
    /// Usually this is set to the name of the file that is currently open.
    pub fn label(&mut self, label: &str) {
        todo!("Tried setting label to {label}");
    }

    /// Close current tab (APP+W)
    pub fn close(&mut self) {
        todo!();
    }

    /// Switch tab (OPT+W), also collapse/expand tab list
    pub fn switch(&mut self) {
        todo!();
    }

    /// Open a new tab using the OS file selector (APP+E)
    pub fn open(&mut self) {
        todo!();
    }

    /// Select tab (OPT+E)
    pub fn select(&mut self) {
        todo!();
    }

    /// Group tab (APP+R)
    pub fn group(&mut self) {
        todo!();
    }

    /// Ungroup tab (OPT+R)
    pub fn ungroup(&mut self) {
        todo!();
    }

    /// Open a new blank tab (APP+T)
    pub fn tab(&mut self) {
        todo!();
    }

    /// Re-open a tab (OPT+T)
    pub fn reopen(&mut self) {
        todo!();
    }

    /// Set the HUD menu for this window.
    pub fn menu(&mut self, _menu: &Menu) {
        todo!()
    }

    /// Searching
    pub fn search(&mut self) {
        todo!()
    }

    /// Headerbar navigation
    pub fn nav(&mut self) {
        todo!()
    }
}

impl Notifier for Window {
    type Event = Action;

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut Exec<'_>,
    ) -> Poll<Self::Event> {
        Poll::Pending
    }
}
