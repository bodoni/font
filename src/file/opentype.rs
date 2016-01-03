use opentype;
use std::path::Path;

use Result;
use super::File;

/// An OpenType file.
pub struct OpenType(opentype::File);

impl OpenType {
    /// Open a file.
    #[inline]
    pub fn open<T: AsRef<Path>>(path: T) -> Result<OpenType> {
        Ok(OpenType(try!(opentype::File::open(path))))
    }
}

impl File for OpenType {
}
