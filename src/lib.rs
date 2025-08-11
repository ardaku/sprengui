#![allow(dead_code)]

use crate::action::Action;
use crate::page::widget::Widget;
use crate::page::window::Window;
use page::text::Text;
use pasts::prelude::*;

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
