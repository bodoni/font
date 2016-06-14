use std::ops::Deref;
use std::path::Path;

use Result;
use font::Font;
use format::opentype;

/// A file.
pub struct File {
    /// The fonts.
    pub fonts: Vec<Font>,
}

impl File {
    /// Open a file.
    pub fn open<T: AsRef<Path>>(path: T) -> Result<Self> {
        let extension = path.as_ref()
                            .extension()
                            .and_then(|e| e.to_str())
                            .map(|e| e.to_lowercase());
        let extension = some!(extension, "unable to detect the file format");
        match &*extension {
            "otc" | "otf" | "ttc" | "ttf" => File::open_opentype(path),
            _ => raise!("encountered an unknown file format"),
        }
    }

    /// Open an OpenType file.
    #[inline]
    pub fn open_opentype<T: AsRef<Path>>(path: T) -> Result<Self> {
        Ok(File { fonts: try!(opentype::open(path)) })
    }
}

impl Deref for File {
    type Target = [Font];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.fonts
    }
}
