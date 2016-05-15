extern crate postscript;

use self::postscript::compact::FontSet;
use self::postscript::type2::Program;
use std::rc::Rc;

use Result;
use glyph::Glyph;

pub struct PostScript {
    fontset: Rc<FontSet>,
    id: usize,
}

impl PostScript {
    #[inline]
    pub fn new(fontset: Rc<FontSet>, id: usize) -> PostScript {
        PostScript { fontset: fontset, id: id }
    }
}

impl ::case::Case for PostScript {
    fn draw(&self, _: char) -> Result<Option<Glyph>> {
        let _ = Program::new(&self.fontset.char_strings[self.id][0],
                             &self.fontset.global_subroutines,
                             &self.fontset.local_subroutines[self.id]);
        Ok(None)
    }
}
