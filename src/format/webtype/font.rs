use std::cell::RefCell;
use std::io::Result;
use std::rc::Rc;

use typeface::Tape;

use crate::format::opentype::cache::Cache;
use crate::format::opentype::{axis, character, feature, metric, name};

pub struct Font<T> {
    cache: Rc<RefCell<Cache<T>>>,
}

impl<T: Tape> crate::font::Case for Font<T> {
    #[inline]
    fn axes(&mut self) -> Result<crate::Axes> {
        axis::read(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn characters(&mut self) -> Result<crate::Characters> {
        character::read(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn features(&mut self) -> Result<crate::Features> {
        feature::read(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn metrics(&mut self) -> Result<crate::Metrics> {
        metric::read(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn names(&mut self) -> Result<crate::Names> {
        name::read(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn draw(&mut self, _: char) -> Result<Option<crate::Glyph>> {
        error!("working with glyphs is not supported yet")
    }
}

pub fn read<T>(tape: Rc<RefCell<T>>, backend: webtype::Font) -> Result<Vec<Font<T>>>
where
    T: Tape,
{
    let cache = Rc::new(RefCell::new(Cache::new(tape, backend)));
    Ok(vec![Font { cache }])
}
