// Sprengui
// Copyright © 2020-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
//  - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
//  - MIT License (https://mit-license.org/)
//  - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

/// Text formatting
pub enum Format {
    /// Paragraph
    Paragraph(Box<Self>),
    /// Code block
    Code(Box<Self>),
    /// Quote block begin
    QuoteIn(Box<Self>),
    /// Quote block end
    QuoteOut(Box<Self>),
    /// H1, usually title of document
    Title(Box<Self>),
    /// H2, section 1
    Section(Box<Self>),
    /// H3, section 1.1
    Header(Box<Self>),
    /// H4, section 1.1.1
    Subheader(Box<Self>),
    /// Label for following user entry
    Label(Box<Self>),
    /// End the chain of formatting
    ///
    /// Should not be used on it's own.
    None,
}

/// Text component
pub enum Text {
    /// Change to new format
    Format(Format, Box<Self>),
    /// Strong (bold for written English)
    Strong(bool, Box<Self>),
    /// Emphasis (italic for written English)
    Emphasis(bool, Box<Self>),
    /// Code (always monospace)
    Code(bool, Box<Self>),
    /// Highlight (with a color - except in high-contrast mode, then boxed text)
    Highlight(bool, Box<Self>),
    /// Mark (underline, or bracket depending on language)
    Mark(bool, Box<Self>),
    /// Render text
    Span(String),
}
