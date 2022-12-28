use std::io::{Read, Seek};
use std::path::Path;

use crate::case::Case;
use crate::file::File;
use crate::Result;

/// A font.
pub struct Font(Box<dyn Case>);

impl Font {
    /// Open a file.
    #[inline]
    pub fn open<T: AsRef<Path>>(path: T) -> Result<Self> {
        Font::read(::std::fs::File::open(path)?)
    }

    /// Read a file.
    pub fn read<T: Read + Seek + 'static>(tape: T) -> Result<Self> {
        let File { mut fonts, .. } = File::read(tape)?;
        match fonts.len() {
            0 => raise!("found an empty file"),
            1 => return Ok(fonts.remove(0)),
            _ => raise!("found a file with multiple fonts, which is not allowed"),
        }
    }
}

dereference! { Font::0 => Box<dyn Case> }

#[inline]
pub fn new(case: Box<dyn Case>) -> Font {
    Font(case)
}
