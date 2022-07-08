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

/// The root of the node tree
pub mod node;
/// Everything related to scripts
pub mod script;

pub struct Sprengui2D {
    /// The node tree
    pub tree: node::NodeTree<
        node::node2d::Node2D,
        script::NodeScript<node::node2d::Node2D>,
    >,
}

pub trait RealEq {
    fn eq(&self, other: &Self) -> bool;
}
