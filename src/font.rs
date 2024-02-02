use std::io::Result;

use crate::metrics::Metrics;
use crate::{Axes, Characters, Features, Glyph, Names, Palettes, Tables};

/// A font.
pub struct Font {
    case: Box<dyn Case>,
}

/// A type that represents a font in a specific format.
pub trait Case {
    /// Return the axes.
    fn axes(&mut self) -> Result<Axes>;

    /// Return the characters.
    fn characters(&mut self) -> Result<Characters>;

    /// Return the features.
    fn features(&mut self) -> Result<Features>;

    /// Return the metrics.
    fn metrics(&mut self) -> Result<Metrics>;

    /// Return the names.
    fn names(&mut self) -> Result<Names>;

    /// Return the palettes.
    fn palettes(&mut self) -> Result<Palettes>;

    /// Return the tables.
    fn tables(&mut self) -> Result<Tables>;

    /// Draw a character.
    fn draw(&mut self, character: char) -> Result<Option<Glyph>>;
}

impl Font {
    /// Return the axes.
    #[inline]
    pub fn axes(&mut self) -> Result<Axes> {
        self.case.axes()
    }

    /// Return the characters.
    #[inline]
    pub fn characters(&mut self) -> Result<Characters> {
        self.case.characters()
    }

    /// Return the features.
    #[inline]
    pub fn features(&mut self) -> Result<Features> {
        self.case.features()
    }

    /// Return the metrics.
    #[inline]
    pub fn metrics(&mut self) -> Result<Metrics> {
        self.case.metrics()
    }

    /// Return the names.
    #[inline]
    pub fn names(&mut self) -> Result<Names> {
        self.case.names()
    }

    /// Return the palettes.
    #[inline]
    pub fn palettes(&mut self) -> Result<Palettes> {
        self.case.palettes()
    }

    /// Return the tables.
    #[inline]
    pub fn tables(&mut self) -> Result<Tables> {
        self.case.tables()
    }

    /// Draw a character.
    #[inline]
    pub fn draw(&mut self, character: char) -> Result<Option<Glyph>> {
        self.case.draw(character)
    }
}

pub fn read<T: typeface::tape::Read + 'static>(mut tape: T) -> Result<Vec<Font>> {
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
