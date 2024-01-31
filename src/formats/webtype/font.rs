use std::cell::RefCell;
use std::io::Result;
use std::rc::Rc;

use crate::formats::opentype::cache::Cache;
use crate::formats::opentype::{axes, characters, features, metrics, names, palettes, tables};

pub struct Font<T> {
    cache: Rc<RefCell<Cache<T>>>,
}

impl<T: typeface::tape::Read> crate::font::Case for Font<T> {
    #[inline]
    fn axes(&mut self) -> Result<crate::Axes> {
        axes::read(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn characters(&mut self) -> Result<crate::Characters> {
        characters::read(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn features(&mut self) -> Result<crate::Features> {
        features::read(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn metrics(&mut self) -> Result<crate::Metrics> {
        metrics::read(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn names(&mut self) -> Result<crate::Names> {
        names::read(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn palettes(&mut self) -> Result<crate::Palettes> {
        palettes::read(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn tables(&mut self) -> Result<crate::Tables> {
        tables::read(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn draw(&mut self, _: char) -> Result<Option<crate::Glyph>> {
        error!("working with glyphs is not supported yet")
    }
}

pub fn read<T>(tape: Rc<RefCell<T>>, backend: webtype::Font) -> Result<Vec<Font<T>>>
where
    T: typeface::tape::Read,
{
    let cache = Rc::new(RefCell::new(Cache::new(tape, backend)));
    Ok(vec![Font { cache }])
}
