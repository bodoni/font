use std::path::Path;

use Result;

mod opentype;

pub use self::opentype::OpenType;

/// A file.
pub trait File {
}

impl File {
    /// Open a file.
    pub fn open<T: AsRef<Path>>(path: T) -> Result<Box<File>> {
        let path = path.as_ref();
        let extension = match path.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase()) {
            Some(extension) => extension,
            _ => raise!("unable to detect the file format"),
        };
        match &*extension {
            "otf" => Ok(Box::new(try!(OpenType::open(path)))),
            _ => raise!("encountered an unknown file format"),
        }
    }
}
