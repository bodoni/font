use std::rc::Rc;

use opentype::truetype::NamingTable;
use typeface::Tape;

use crate::glyph::Glyph;
use crate::metrics::Metrics;
use crate::properties::Properties;
use crate::Result;

/// A font.
pub struct Font {
    case: Box<dyn Case>,
}

pub trait Case {
    fn draw(&mut self, character: char) -> Result<Option<Glyph>>;
    fn metrics(&mut self) -> Result<Metrics>;
    fn names(&mut self) -> Result<Rc<NamingTable>>;
    fn properties(&mut self) -> Result<Properties>;
}

impl Font {
    /// Draw a character.
    #[inline]
    pub fn draw(&mut self, character: char) -> Result<Option<Glyph>> {
        self.case.draw(character)
    }

    /// Return metrics.
    #[inline]
    pub fn metrics(&mut self) -> Result<Metrics> {
        self.case.metrics()
    }

    /// Return names.
    #[inline]
    pub fn names(&mut self) -> Result<Rc<NamingTable>> {
        self.case.names()
    }

    /// Return properties.
    #[inline]
    pub fn properties(&mut self) -> Result<Properties> {
        self.case.properties()
    }
}

pub fn read<T: Tape + 'static>(mut tape: T) -> Result<Vec<Font>> {
    use opentype::truetype::Tag;

    let tag = tape.peek::<Tag>()?;
    if opentype::accept(&tag) {
        Ok(crate::format::opentype::read(tape)?
            .into_iter()
            .map(|font| Font {
                case: Box::new(font),
            })
            .collect())
    } else if webtype::accept(&tag) {
        Ok(crate::format::webtype::read(tape)?
            .into_iter()
            .map(|font| Font {
                case: Box::new(font),
            })
            .collect())
    } else {
        error!("found an unknown file format")
    }
}
