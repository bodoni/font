use std::cell::RefCell;
use std::io::Result;
use std::ops::DerefMut;
use std::rc::Rc;

use crate::formats::opentype::cache::{Cache, Reference};
use crate::formats::opentype::{axes, characters, features, metrics, names, palettes, tables};

/// A font.
pub struct Font<T> {
    cache: Reference<Cache<T>>,
    index: (bool, usize),
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

    fn glyph(&mut self, character: char) -> Result<Option<crate::Glyph>> {
        let mut cache = self.cache.borrow_mut();
        let mapping = cache.mapping()?.clone();
        let metrics = cache.metrics()?.clone();
        match self.index {
            (true, _) => {
                super::truetype::draw(&cache.glyph_data()?.borrow(), &mapping, &metrics, character)
            }
            (false, id) => super::postscript::draw(
                &cache.font_set()?.borrow(),
                &mapping,
                &metrics,
                id,
                character,
            ),
        }
    }
}

pub fn read<T: typeface::tape::Read>(
    tape: Reference<T>,
    backend: opentype::Font,
) -> Result<Vec<Font<T>>> {
    use opentype::postscript::compact1::FontSet;
    use opentype::truetype::tables::GlyphData;

    let truetype = backend.exists::<GlyphData>();
    let postscript = {
        let mut tape = tape.borrow_mut();
        let tape = tape.deref_mut();
        backend
            .position::<_, FontSet>(tape)?
            .map(|_| FontSet::count(tape))
            .transpose()?
            .unwrap_or(0)
    };
    let cache = Rc::new(RefCell::new(Cache::new(tape, backend)));
    let mut fonts = vec![];
    if truetype {
        fonts.push(Font {
            cache: cache.clone(),
            index: (true, 0),
        });
    }
    for id in 0..postscript {
        fonts.push(Font {
            cache: cache.clone(),
            index: (false, id),
        });
    }
    Ok(fonts)
}
