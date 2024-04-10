use std::io::Result;
use std::path::Path;

/// A file.
pub struct File<T> {
    /// The fonts.
    pub fonts: Vec<crate::font::Font<T>>,
}

impl<T> File<T>
where
    T: crate::Read,
{
    /// Read a file.
    #[inline]
    pub fn read(tape: T) -> Result<Self> {
        Ok(Self {
            fonts: crate::font::read(tape)?,
        })
    }
}

impl File<std::fs::File> {
    /// Open a file.
    #[inline]
    pub fn open<T: AsRef<Path>>(path: T) -> Result<Self> {
        Self::read(std::fs::File::open(path)?)
    }
}

dereference! { File<T>::fonts => [crate::font::Font<T>] }

#[cfg(test)]
mod tests {
    macro_rules! ok(($result:expr) => ($result.unwrap()));

    #[test]
    fn cursor() {
        use std::io::Read;

        let path = "tests/fixtures/SourceSerifPro-Regular.otf";
        let mut file = ok!(std::fs::File::open(path));
        let mut data = Vec::new();
        ok!(file.read_to_end(&mut data));

        let data = std::io::Cursor::new(data);
        let _ = ok!(super::File::read(data));
    }
}
