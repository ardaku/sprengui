// Sprengui
// Copyright © 2020-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
//  - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
//  - MIT License (https://mit-license.org/)
//  - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use pasts::{Notifier, prelude::*};
use crate::action::Action;
use page::text::Text;
use crate::page::widget::Widget;
use crate::page::window::Window;

#[cfg(test)]
mod tests;

pub mod action;
pub mod page;

enum Icon {
    /// Wand icon for effects
    Wand,
    /// Pencil icon
    Pencil,
}


struct MenuItem {
    icon: Option<Icon>,
    name: String,
    description: String,
    // FIXME: using window crate types
    shortcut: (),
}

/// Heads-up-display menu interface
pub struct Menu {
    id: usize,
    items: Vec<MenuItem>,
}

impl Menu {
    /// Create a menu for this window
    // FIXME: Take menu items as parameters or use builder
    pub fn new(window: &mut Window) -> Self {
        let id = window.menus;
        window.menus += 1;
        let items = Vec::new();
        Self { id, items }
    }
}

