use std::rc::Rc;

use truetype::NamingTable;
use typeface::Tape;

use crate::glyph::Glyph;
use crate::metrics::Metrics;
use crate::Result;

/// A font.
pub struct Font<T> {
    case: Case<T>,
}

pub enum Case<T> {
    OpenType(crate::format::opentype::Font<T>),
}

impl<T: Tape> Font<T> {
    /// Draw a character.
    #[inline]
    pub fn draw(&mut self, character: char) -> Result<Option<Glyph>> {
        match &mut self.case {
            Case::OpenType(ref mut case) => case.draw(character),
        }
    }

    /// Return metrics.
    #[inline]
    pub fn metrics(&mut self) -> Result<Metrics> {
        match &mut self.case {
            Case::OpenType(ref mut case) => case.metrics(),
        }
    }

    /// Return names.
    #[inline]
    pub fn names(&mut self) -> Result<Rc<NamingTable>> {
        match &mut self.case {
            Case::OpenType(ref mut case) => case.names(),
        }
    }
}

pub fn read<T: Tape + 'static>(tape: T) -> Result<Vec<Font<T>>> {
    Ok(crate::format::opentype::read(tape)?
        .into_iter()
        .map(|font| Font {
            case: Case::OpenType(font),
        })
        .collect())
}
