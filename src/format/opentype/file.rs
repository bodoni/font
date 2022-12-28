use std::io::{Read, Seek};
use std::ops::DerefMut;
use std::{cell::RefCell, rc::Rc};

use crate::Result;

pub struct File<T: Read + Seek>(Rc<RefCell<T>>);

impl<T: Read + Seek + 'static> File<T> {
    #[inline]
    pub fn open(tape: T) -> Self {
        Self(Rc::new(RefCell::new(tape)))
    }

    pub fn read(&mut self) -> Result<Vec<crate::font::Font>> {
        let mut fonts = vec![];
        let file = opentype::File::read(self.0.borrow_mut().deref_mut())?;
        for font in file.fonts.into_iter() {
            for font in crate::format::opentype::font::read(self.0.clone(), font)? {
                fonts.push(crate::font::new(Box::new(font)));
            }
        }
        Ok(fonts)
    }
}
