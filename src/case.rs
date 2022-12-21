use crate::glyph::Glyph;
use crate::Result;

/// A collection of glyphs.
pub trait Case {
    /// Draw a character.
    fn draw(&self, character: char) -> Result<Option<Glyph>>;
}
