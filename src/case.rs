use crate::Result;
use crate::glyph::Glyph;

/// A collection of glyphs.
pub trait Case {
    /// Draw a glyph.
    fn draw(&self, glyph: char) -> Result<Option<Glyph>>;
}
