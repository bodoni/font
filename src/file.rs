use std::path::Path;

use Result;
use font::Font;
use format::opentype;

/// A file.
pub struct File {
    /// Fonts.
    pub fonts: Vec<Font>,
}

impl File {
    /// Open a file.
    pub fn open<T: AsRef<Path>>(path: T) -> Result<File> {
        let path = path.as_ref();
        let extension = match path.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase()) {
            Some(extension) => extension,
            _ => raise!("unable to detect the file format"),
        };
        match &*extension {
            "otf" => Ok(File { fonts: try!(opentype::open(path)) }),
            _ => raise!("encountered an unknown file format"),
        }
    }
}
