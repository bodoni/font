pub mod axes;
pub mod cache;
pub mod characters;
pub mod features;
pub mod font;
pub mod metrics;
pub mod names;
pub mod palettes;
pub mod tables;

mod postscript;
mod truetype;

pub use self::font::Font;

use std::cell::RefCell;
use std::io::Result;
use std::ops::DerefMut;
use std::rc::Rc;

pub fn read<T: typeface::tape::Read + 'static>(tape: T) -> Result<Vec<Font<T>>> {
    let tape = Rc::new(RefCell::new(tape));
    let mut fonts = vec![];
    let file = opentype::File::read(tape.borrow_mut().deref_mut())?;
    for font in file.fonts.into_iter() {
        fonts.extend(self::font::read(tape.clone(), font)?);
    }
    Ok(fonts)
}
