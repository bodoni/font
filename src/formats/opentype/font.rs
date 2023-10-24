use std::cell::RefCell;
use std::io::Result;
use std::rc::Rc;

use typeface::Tape;

use crate::formats::opentype::cache::Cache;
use crate::formats::opentype::postscript::PostScript;
use crate::formats::opentype::truetype::TrueType;
use crate::formats::opentype::{axes, characters, features, metrics, names, tables};

pub struct Font<T> {
    cache: Rc<RefCell<Cache<T>>>,
    case: Case,
}

enum Case {
    PostScript(PostScript),
    TrueType(TrueType),
}

impl<T: Tape> crate::font::Case for Font<T> {
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
    fn tables(&mut self) -> Result<crate::Tables> {
        tables::read(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn draw(&mut self, character: char) -> Result<Option<crate::Glyph>> {
        match &self.case {
            Case::PostScript(ref case) => case.draw(character),
            Case::TrueType(ref case) => case.draw(character),
        }
    }
}

pub fn read<T: Tape>(tape: Rc<RefCell<T>>, backend: opentype::Font) -> Result<Vec<Font<T>>> {
    let mut fonts = vec![];
    let cache = Rc::new(RefCell::new(Cache::new(tape, backend)));
    let mut cache_borrowed = cache.borrow_mut();
    let mapping = cache_borrowed.mapping()?.clone();
    let metrics = cache_borrowed.metrics()?.clone();
    if let Some(table) = cache_borrowed.try_font_set()? {
        for id in 0..table.character_strings.len() {
            let case = PostScript::new(id, table.clone(), mapping.clone(), metrics.clone());
            fonts.push(Font {
                cache: cache.clone(),
                case: Case::PostScript(case),
            });
        }
    }
    if let Some(table) = cache_borrowed.try_glyph_data()? {
        let case = TrueType::new(table.clone(), mapping, metrics);
        fonts.push(Font {
            cache: cache.clone(),
            case: Case::TrueType(case),
        });
    }
    Ok(fonts)
}
