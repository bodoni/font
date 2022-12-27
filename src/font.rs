use std::io::{Read, Seek};
use std::path::Path;

use crate::case::Case;
use crate::file::File;
use crate::metrics::Metrics;
use crate::Result;

/// A font.
pub struct Font {
    /// Metrics.
    pub metrics: Metrics,
    /// A collection of glyphs.
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
