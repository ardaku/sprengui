pub mod text;
pub mod widget;
pub mod window;

use crate::{Widget, Window};

/// A GUI page
pub struct Page {
    widgets: Vec<Widget>,
}

impl Page {
    pub fn new(_window: &mut Window) -> Self {
        Self {
            widgets: Vec::new(),
        }
    }

    /// Set the widgets
    pub fn set<const N: usize>(&mut self, widgets: [Widget; N]) {
        self.widgets.clear();
        self.widgets.extend(widgets);
    }

    /// Get the number of widgets in this window
    pub fn len(&self) -> usize {
        self.widgets.len()
    }

    /// Get whether or not there are no widgets in the window
    pub fn is_empty(&self) -> bool {
        self.widgets.is_empty()
    }

    /// Add a widget to the window
    pub fn push(&mut self, widget: Widget) {
        self.widgets.push(widget);
    }
}
