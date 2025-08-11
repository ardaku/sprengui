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
