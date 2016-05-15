use std::rc::Rc;
use super::postscript::compact::FontSet;
use super::postscript::type2::Program;

use Result;
use glyph::Glyph;
use super::mapping::Mapping;

pub struct PostScript {
    id: usize,
    fontset: Rc<FontSet>,
    mapping: Rc<Mapping>,
}

impl PostScript {
    #[inline]
    pub fn new(id: usize, fontset: Rc<FontSet>, mapping: Rc<Mapping>) -> PostScript {
        PostScript { id: id, fontset: fontset, mapping: mapping }
    }
}

impl ::case::Case for PostScript {
    fn draw(&self, glyph: char) -> Result<Option<Glyph>> {
        let id = match self.mapping.find(glyph) {
            Some(id) => id,
            _ => return Ok(None),
        };
        let _ = Program::new(&self.fontset.char_strings[self.id][id],
                             &self.fontset.global_subroutines,
                             &self.fontset.local_subroutines[self.id]);
        Ok(None)
    }
}
