use crate::glyph::Glyph;
use crate::metrics::Metrics;
use crate::Result;

/// A type handing a font.
pub trait Case {
    /// Draw a character.
    fn draw(&mut self, character: char) -> Result<Option<Glyph>>;

    /// Return metrics.
    fn metrics(&mut self) -> Result<Metrics>;
}
