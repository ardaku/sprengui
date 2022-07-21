// Sprengui
// Copyright © 2020-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
//  - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
//  - MIT License (https://mit-license.org/)
//  - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use crate::Text;

/// A field for user entry
pub enum Field {
    Text(String),
    Int(i64),
    Number(f64),
}

/// A widget.
///
/// Customized widgets can be built using a `Canvas` (TODO: Example).
pub enum Widget {
    /// Page text
    Text(Vec<Text>),
    /// Page field
    Field(Field),
    /// Page choices (with number of columns)
    Choices(Vec<Widget>, usize),
    /// Page buttons (with number of columns)
    Buttons(Vec<Widget>, usize),
    /// Page canvas (picture, movie, rendered, etc.)
    Canvas(pix::Raster<pix::rgb::SRgba8>),
}
