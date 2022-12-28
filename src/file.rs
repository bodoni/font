use std::path::Path;

use typeface::Tape;

use crate::font::Font;
use crate::Result;

/// A file.
pub struct File {
    /// The fonts.
    pub fonts: Vec<Font>,
}

impl File {
    /// Open a file.
    #[inline]
    pub fn open<T: AsRef<Path>>(path: T) -> Result<Self> {
        File::read(::std::fs::File::open(path)?)
    }

    /// Read a file.
    #[inline]
    pub fn read<T: Tape + 'static>(tape: T) -> Result<Self> {
        Ok(File {
            fonts: crate::format::opentype::read(tape)?,
        })
    }
}

dereference! { File::fonts => [Font] }
