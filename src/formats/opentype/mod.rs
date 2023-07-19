pub mod cache;
pub mod font;

mod mapping;
mod metrics;
mod postscript;
mod truetype;

use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

pub use self::font::Font;

use opentype;
use typeface::Tape;

use crate::Result;

pub fn read<T: Tape + 'static>(tape: T) -> Result<Vec<Font<T>>> {
    let tape = Rc::new(RefCell::new(tape));
    let mut fonts = vec![];
    let file = opentype::File::read(tape.borrow_mut().deref_mut())?;
    for font in file.fonts.into_iter() {
        fonts.extend(self::font::read(tape.clone(), font)?);
    }
    Ok(fonts)
}
