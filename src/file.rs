use std::io::Result;
use std::path::Path;

use typeface::Tape;

/// A file.
pub struct File {
    /// The fonts.
    pub fonts: Vec<crate::font::Font>,
}

impl File {
    /// Open a file.
    #[inline]
    pub fn open<T: AsRef<Path>>(path: T) -> Result<Self> {
        Self::read(std::fs::File::open(path)?)
    }

    /// Read a file.
    #[inline]
    pub fn read<T: Tape + 'static>(tape: T) -> Result<Self> {
        Ok(Self {
            fonts: crate::font::read(tape)?,
        })
    }
}

dereference! { File::fonts => [crate::font::Font] }
