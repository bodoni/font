use std::rc::Rc;

use opentype::truetype::NamingTable;
use typeface::Tape;

use crate::axes::Axes;
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

    fn axes(&mut self) -> Result<Axes>;
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

    /// Return the axes.
    #[inline]
    pub fn axes(&mut self) -> Result<Axes> {
        self.case.axes()
    }

    /// Return the metrics.
    #[inline]
    pub fn metrics(&mut self) -> Result<Metrics> {
        self.case.metrics()
    }

    /// Return the names.
    #[inline]
    pub fn names(&mut self) -> Result<Rc<NamingTable>> {
        self.case.names()
    }

    /// Return the properties.
    #[inline]
    pub fn properties(&mut self) -> Result<Properties> {
        self.case.properties()
    }
}

pub fn read<T: Tape + 'static>(mut tape: T) -> Result<Vec<Font>> {
    use opentype::truetype::Tag;

    let tag = tape.peek::<Tag>()?;
    if opentype::accept(&tag) {
        Ok(crate::formats::opentype::read(tape)?
            .into_iter()
            .map(|font| Font {
                case: Box::new(font),
            })
            .collect())
    } else if webtype::accept(&tag) {
        Ok(crate::formats::webtype::read(tape)?
            .into_iter()
            .map(|font| Font {
                case: Box::new(font),
            })
            .collect())
    } else {
        error!("found an unknown file format")
    }
}
