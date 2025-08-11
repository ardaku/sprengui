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
