mod cache;
mod case;
mod font;
mod mapping;
mod metrics;
mod postscript;
mod truetype;

use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

use typeface::Tape;

use crate::Result;

pub fn read<T: Tape + 'static>(tape: T) -> Result<Vec<crate::font::Font>> {
    let tape = Rc::new(RefCell::new(tape));
    let mut fonts = vec![];
    let file = opentype::File::read(tape.borrow_mut().deref_mut())?;
    for font in file.fonts.into_iter() {
        for font in crate::format::opentype::font::read(tape.clone(), font)? {
            fonts.push(crate::font::new(Box::new(font)));
        }
    }
    Ok(fonts)
}
