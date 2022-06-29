// Sprengui
// Copyright © 2020-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
//  - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
//  - MIT License (https://mit-license.org/)
//  - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::missing_inline_in_public_items)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::exhaustive_structs)]
#![allow(clippy::implicit_return)]

use crate::node::node2d::gui::button::Button;
use crate::node::Node;
use winit::event::{Event, WindowEvent};

/// All scripts implement this trait,
/// so that other traits can use this
/// in there generic arguments e.g.
/// `<T: `[`Script`]`>` covers all
/// of [`NodeScript`], [`ButtonScript`],
/// etc.
pub trait Script {}

/// A script that can be attached to any type of node
pub type NodeScript<T> = fn(&mut dyn Node<T>, &Event<()>);
impl<T> Script for NodeScript<T> {}

/// A script that can be attached to any button node
#[derive(Clone, Copy)]
pub struct ButtonScript<'closures> {
    /// What to do when the button is pressed
    pub on_click: &'closures dyn Fn(&mut dyn Button, &WindowEvent),
    /// What to do when the button is released
    pub on_release: &'closures dyn Fn(&mut dyn Button, &WindowEvent),
    /// What to do when the button is hovered over
    pub on_hover: &'closures dyn Fn(&mut dyn Button),
}
impl Script for ButtonScript<'_> {}

/// A special script for when you don't want a script.
pub type NoScript = ();
impl Script for NoScript {}
