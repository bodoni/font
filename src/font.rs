use std::rc::Rc;

use opentype::truetype::NamingTable;
use typeface::Tape;

use crate::flags::Flags;
use crate::glyph::Glyph;
use crate::metrics::Metrics;
use crate::Result;

/// A font.
pub struct Font<T> {
    case: Case<T>,
}

pub enum Case<T> {
    OpenType(crate::format::opentype::Font<T>),
    WebType(crate::format::webtype::Font<T>),
}

impl<T: Tape> Font<T> {
    /// Draw a character.
    #[inline]
    pub fn draw(&mut self, character: char) -> Result<Option<Glyph>> {
        match &mut self.case {
            Case::OpenType(ref mut case) => case.draw(character),
            Case::WebType(ref mut case) => case.draw(character),
        }
    }

    /// Return flags.
    #[inline]
    pub fn flags(&mut self) -> Result<Flags> {
        match &mut self.case {
            Case::OpenType(ref mut case) => case.flags(),
            Case::WebType(ref mut case) => case.flags(),
        }
    }

    /// Return metrics.
    #[inline]
    pub fn metrics(&mut self) -> Result<Metrics> {
        match &mut self.case {
            Case::OpenType(ref mut case) => case.metrics(),
            Case::WebType(ref mut case) => case.metrics(),
        }
    }

    /// Return names.
    #[inline]
    pub fn names(&mut self) -> Result<Rc<NamingTable>> {
        match &mut self.case {
            Case::OpenType(ref mut case) => case.names(),
            Case::WebType(ref mut case) => case.names(),
        }
    }
}

pub fn read<T: Tape + 'static>(mut tape: T) -> Result<Vec<Font<T>>> {
    use opentype::truetype::Tag;

    let tag = tape.peek::<Tag>()?;
    if opentype::accept(&tag) {
        Ok(crate::format::opentype::read(tape)?
            .into_iter()
            .map(|font| Font {
                case: Case::OpenType(font),
            })
            .collect())
    } else if webtype::accept(&tag) {
        Ok(crate::format::webtype::read(tape)?
            .into_iter()
            .map(|font| Font {
                case: Case::WebType(font),
            })
            .collect())
    } else {
        error!("found an unknown file format")
    }
}
