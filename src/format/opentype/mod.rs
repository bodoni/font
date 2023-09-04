pub mod axis;
pub mod cache;
pub mod character;
pub mod feature;
pub mod font;
pub mod metric;
pub mod name;
pub mod table;

mod postscript;
mod truetype;

pub use self::font::Font;

use std::cell::RefCell;
use std::io::Result;
use std::ops::DerefMut;
use std::rc::Rc;

use typeface::Tape;

pub fn read<T: Tape + 'static>(tape: T) -> Result<Vec<Font<T>>> {
    let tape = Rc::new(RefCell::new(tape));
    let mut fonts = vec![];
    let file = opentype::File::read(tape.borrow_mut().deref_mut())?;
    for font in file.fonts.into_iter() {
        fonts.extend(self::font::read(tape.clone(), font)?);
    }
    Ok(fonts)
}
