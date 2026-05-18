//! The Web Open Font Format (WOFF).

mod font;

pub use self::font::Font;

use std::io::Result;

use crate::formats::opentype::cache::Reference;

/// Read fonts.
pub fn read<T: crate::Read>(mut tape: T) -> Result<Vec<Font<T>>> {
    let mut fonts = vec![];
    let file = webtype::File::read(&mut tape)?;
    let tape = Reference::new(file.tape);
    for font in file.fonts.into_iter() {
        fonts.extend(self::font::read::<T>(tape.clone(), font)?);
    }
    Ok(fonts)
}
