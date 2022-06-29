// Sprengui
// Copyright © 2020-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
//  - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
//  - MIT License (https://mit-license.org/)
//  - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

//! # sprengui
//!
//! A simple, yet powerful, GUI library for Rust.

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

use winit::event::Event;

/// The root of the node tree
pub mod node;
/// Everything related to scripts
pub mod script;

pub trait EventListener {
    fn on_event(&mut self, event: &Event<()>);
    fn disable(&mut self);
    fn enable(&mut self);
    fn is_enabled(&self) -> bool;
    fn is_disabled(&self) -> bool {
        !self.is_enabled()
    }
    fn set_enabled(&mut self, enabled: bool);
    fn set_disabled(&mut self, disabled: bool) {
        self.set_enabled(!disabled);
    }
    fn toggle_enabled(&mut self) {
        self.set_enabled(!self.is_enabled());
    }
    fn is_visible(&self) -> bool;
    fn is_hidden(&self) -> bool {
        !self.is_visible()
    }
    fn set_visible(&mut self, visible: bool);
    fn set_hidden(&mut self, hidden: bool) {
        self.set_visible(!hidden);
    }
    fn toggle_visible(&mut self) {
        self.set_visible(!self.is_visible());
    }
}
