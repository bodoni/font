//! The Web Open Font Format (WOFF).

mod font;

pub use self::font::Font;

use std::cell::RefCell;
use std::io::{Cursor, Result};
use std::rc::Rc;

/// Read fonts.
pub fn read<T: typeface::tape::Read>(mut tape: T) -> Result<Vec<Font<Cursor<Vec<u8>>>>> {
    let mut fonts = vec![];
    let file = webtype::File::read(&mut tape)?;
    let tape = Rc::new(RefCell::new(file.tape));
    for font in file.fonts.into_iter() {
        fonts.extend(self::font::read(tape.clone(), font)?);
    }
    Ok(fonts)
}
