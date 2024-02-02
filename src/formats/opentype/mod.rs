//! The OpenType font format.

pub mod axes;
pub mod characters;
pub mod features;
pub mod metrics;
pub mod names;
pub mod palettes;
pub mod tables;

pub(crate) mod cache;

mod font;
mod postscript;
mod truetype;

pub use self::font::Font;

use std::cell::RefCell;
use std::io::Result;
use std::ops::DerefMut;
use std::rc::Rc;

/// Read fonts.
pub fn read<T: typeface::tape::Read + 'static>(tape: T) -> Result<Vec<Font<T>>> {
    let tape = Rc::new(RefCell::new(tape));
    let mut fonts = vec![];
    let file = opentype::File::read(tape.borrow_mut().deref_mut())?;
    for font in file.fonts.into_iter() {
        fonts.extend(self::font::read(tape.clone(), font)?);
    }
    Ok(fonts)
}
