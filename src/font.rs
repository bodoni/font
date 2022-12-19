use std::io::{Read, Seek};
use std::path::Path;

use crate::{Case, File, Number, Result};

/// A font.
pub struct Font {
    /// The number of units per em.
    pub units_per_em: Number,
    /// The typographical ascender.
    pub ascender: Number,
    /// The cap height.
    pub cap_height: Number,
    /// The x-height.
    pub x_height: Number,
    /// The baseline.
    pub baseline: Number,
    /// The typographical descender.
    pub descender: Number,
    /// The typographical line gap.
    pub line_gap: Number,
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

dereference! { Font::case => Box<dyn Case> }
