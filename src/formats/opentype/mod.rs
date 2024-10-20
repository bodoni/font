//! The OpenType font format.

pub mod axes;
pub mod characters;
pub mod features;
pub mod names;
pub mod palettes;
pub mod tables;
pub mod timestamps;

pub(crate) mod cache;
pub(crate) mod metrics;

mod font;
mod mapping;
mod postscript;
mod truetype;

pub use self::font::{write, Disposition, Font};

use std::cell::RefCell;
use std::io::Result;
use std::ops::DerefMut;
use std::rc::Rc;

/// Read fonts.
pub fn read<T>(tape: T) -> Result<Vec<Font<T>>>
where
    T: crate::Read,
{
    let tape = Rc::new(RefCell::new(tape));
    let mut fonts = vec![];
    let file = opentype::File::read(tape.borrow_mut().deref_mut())?;
    for font in file.fonts.into_iter() {
        fonts.extend(self::font::read(tape.clone(), font)?);
    }
    Ok(fonts)
}
