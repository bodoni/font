pub mod axes;
pub mod cache;
pub mod features;
pub mod font;

mod characters;
mod metrics;
mod postscript;
mod truetype;

pub use self::font::Font;

use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

use opentype::truetype::NamingTable;
use typeface::Tape;

use crate::Result;

/// Names.
pub type Names = Rc<NamingTable>;

pub fn read<T: Tape + 'static>(tape: T) -> Result<Vec<Font<T>>> {
    let tape = Rc::new(RefCell::new(tape));
    let mut fonts = vec![];
    let file = opentype::File::read(tape.borrow_mut().deref_mut())?;
    for font in file.fonts.into_iter() {
        fonts.extend(self::font::read(tape.clone(), font)?);
    }
    Ok(fonts)
}
