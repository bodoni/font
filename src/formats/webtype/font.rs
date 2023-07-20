use std::cell::RefCell;
use std::rc::Rc;

use typeface::Tape;

use crate::formats::opentype::cache::Cache;
use crate::formats::opentype::font::{read_axes, read_characters, read_metrics, read_names};
use crate::Result;

pub struct Font<T> {
    cache: Rc<RefCell<Cache<T>>>,
}

impl<T: Tape> crate::font::Case for Font<T> {
    #[inline]
    fn draw(&mut self, _: char) -> Result<Option<crate::glyph::Glyph>> {
        error!("working with glyphs is not supported yet")
    }

    #[inline]
    fn axes(&mut self) -> Result<crate::axes::Axes> {
        read_axes(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn characters(&mut self) -> Result<crate::characters::Characters> {
        read_characters(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn metrics(&mut self) -> Result<crate::metrics::Metrics> {
        read_metrics(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn names(&mut self) -> Result<crate::names::Names> {
        read_names(&mut self.cache.borrow_mut())
    }
}

pub fn read<T>(tape: Rc<RefCell<T>>, backend: webtype::Font) -> Result<Vec<Font<T>>>
where
    T: Tape,
{
    let cache = Rc::new(RefCell::new(Cache::new(tape, backend)));
    Ok(vec![Font { cache }])
}