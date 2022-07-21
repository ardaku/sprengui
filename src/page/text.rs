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
    Paragraph,
    /// Code block
    Code,
    /// Quote block begin
    QuoteIn,
    /// Quote block end
    QuoteOut,
    /// H1, usually title of document
    Title,
    /// H2, section 1
    Section,
    /// H3, section 1.1
    Header,
    /// H4, section 1.1.1
    Subheader,
    /// Label for following user entry
    Label,
}

/// Text component
pub enum Text {
    /// Change to new format
    Format(Format),
    /// Strong (bold for written English)
    Strong(bool),
    /// Emphasis (italic for written English)
    Emphasis(bool),
    /// Code (always monospace)
    Code(bool),
    /// Highlight (with a color - except in high-contrast mode, then boxed text)
    Highlight(bool),
    /// Mark (underline, or bracket depending on language)
    Mark(bool),
    /// Render text
    Span(String),
}
