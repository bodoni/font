use std::path::Path;

use typeface::Tape;

use crate::Result;

/// A file.
pub struct File<T> {
    /// The fonts.
    pub fonts: Vec<crate::font::Font<T>>,
}

impl<T: Tape + 'static> File<T> {
    /// Read a file.
    #[inline]
    pub fn read(tape: T) -> Result<Self> {
        Ok(Self {
            fonts: crate::font::read(tape)?,
        })
    }
}

impl File<::std::fs::File> {
    /// Open a file.
    #[inline]
    pub fn open<T: AsRef<Path>>(path: T) -> Result<Self> {
        Self::read(::std::fs::File::open(path)?)
    }
}

dereference! { File<T>::fonts => [crate::font::Font<T>] }
