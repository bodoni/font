mod font;

pub use self::font::Font;

use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

use typeface::Tape;

use crate::Result;

pub fn read<T: Tape + 'static>(tape: T) -> Result<Vec<Font<T>>> {
    let tape = Rc::new(RefCell::new(tape));
    let mut fonts = vec![];
    let file = webtype::File::read(tape.borrow_mut().deref_mut())?;
    for font in file.fonts.into_iter() {
        fonts.extend(font::read(tape.clone(), font)?);
    }
    Ok(fonts)
}
