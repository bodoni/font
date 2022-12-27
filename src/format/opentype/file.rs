use std::io::{Read, Seek};

use crate::font::Font;
use crate::format::opentype::font;
use crate::Result;

pub struct File<T: Read + Seek>(T);

impl<T: Read + Seek> File<T> {
    #[inline]
    pub fn open(tape: T) -> Self {
        Self(tape)
    }

    pub fn read(&mut self) -> Result<Vec<Font>> {
        let mut fonts = vec![];
        for font in &opentype::File::read(&mut self.0)?.fonts {
            font::read(&mut self.0, &mut fonts, font)?;
        }
        Ok(fonts)
    }
}
