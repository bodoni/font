use std::io::{Read, Seek};
use std::path::Path;

use crate::{Case, File, Number, Result};

/// A font.
pub struct Font {
    /// The granularity of the coordinate grid.
    pub units_per_em: Number,
    /// The point above which clipping can safely occur.
    pub clipping_ascender: Number,
    /// The typographical ascender relative to the baseline.
    pub ascender: Number,
    /// The cap height relative to the baseline.
    pub cap_height: Number,
    /// The x-height relative to the baseline.
    pub x_height: Number,
    /// The baseline.
    pub baseline: Number,
    /// The typographical descender relative to the baseline.
    pub descender: Number,
    /// The point below which clipping can safely occur.
    pub clipping_descender: Number,
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
            _ => raise!("found a file with multiple fonts, which is not supported yet"),
        }
    }
}

dereference! { Font::case => Box<dyn Case> }
