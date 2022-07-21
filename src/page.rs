// Sprengui
// Copyright © 2020-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
//  - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
//  - MIT License (https://mit-license.org/)
//  - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

pub mod widget;
pub mod window;
pub mod text;

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

