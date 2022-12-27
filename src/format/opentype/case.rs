use crate::Result;

pub trait Case {
    fn draw(&self, character: char) -> Result<Option<crate::glyph::Glyph>>;
}
