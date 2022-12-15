use std::io::{Read, Seek};
use std::path::Path;

use crate::format::opentype;
use crate::{Font, Result};

/// A file.
pub struct File {
    /// The fonts.
    pub fonts: Vec<Font>,
}

impl File {
    /// Open a file.
    #[inline]
    pub fn open<T: AsRef<Path>>(path: T) -> Result<Self> {
        File::read(&mut ::std::fs::File::open(path)?)
    }

    /// Read a file.
    #[inline]
    pub fn read<T: Read + Seek>(tape: &mut T) -> Result<Self> {
        Ok(File {
            fonts: opentype::read(tape)?,
        })
    }
}

dereference! { File::fonts => [Font] }
