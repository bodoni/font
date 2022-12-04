use std::io::{Read, Seek};
use std::ops::Deref;
use std::path::Path;

use crate::{Case, File, Result};

/// A font.
pub struct Font {
    /// The number of units per em.
    pub units_per_em: usize,
    /// The ascender line relative to the base line.
    pub ascender: isize,
    /// The descender line relative to the base line.
    pub descender: isize,
    /// The collection of glyphs.
    pub case: Box<dyn Case>,
}

impl Font {
    /// Open a file containing a single font.
    #[inline]
    pub fn open<T: AsRef<Path>>(path: T) -> Result<Self> {
        Font::read(&mut ::std::fs::File::open(path)?)
    }

    /// Read a file containing a single font.
    pub fn read<T: Read + Seek>(tape: &mut T) -> Result<Self> {
        let File { mut fonts, .. } = File::read(tape)?;
        match fonts.len() {
            0 => raise!("found an empty file"),
            1 => return Ok(fonts.remove(0)),
            _ => raise!("files with multiple fonts are not supported yet"),
        }
    }
}

impl Deref for Font {
    type Target = Box<dyn Case>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.case
    }
}
