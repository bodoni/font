use crate::glyph::Glyph;
use crate::Result;

/// A collection of glyphs.
pub trait Case {
    /// Draw a glyph.
    fn draw(&self, glyph: char) -> Result<Option<Glyph>>;
}
