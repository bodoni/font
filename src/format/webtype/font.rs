use std::cell::RefCell;
use std::rc::Rc;

use opentype::truetype::NamingTable;
use typeface::Tape;

use crate::format::opentype::cache::Cache;
use crate::format::opentype::font::{read_flags, read_metrics, read_names};
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
    fn flags(&mut self) -> Result<crate::flags::Flags> {
        read_flags(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn metrics(&mut self) -> Result<crate::metrics::Metrics> {
        read_metrics(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn names(&mut self) -> Result<Rc<NamingTable>> {
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
