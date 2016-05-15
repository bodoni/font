use Result;
use glyph::Glyph;

/// A collection of glyphs.
pub trait Case {
    /// Draw a glyph.
    fn draw(&self, char) -> Result<Option<Glyph>>;
}
