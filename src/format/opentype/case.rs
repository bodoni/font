extern crate postscript;
extern crate truetype;

use self::postscript::compact::FontSet;
use self::postscript::type2::Program;
use self::truetype::CharMapping;
use std::rc::Rc;

use Result;
use glyph::Glyph;

pub struct PostScript {
    id: usize,
    fontset: Rc<FontSet>,
    #[allow(dead_code)]
    mapping: Rc<CharMapping>,
}

impl PostScript {
    #[inline]
    pub fn new(id: usize, fontset: Rc<FontSet>, mapping: Rc<CharMapping>) -> PostScript {
        PostScript { id: id, fontset: fontset, mapping: mapping }
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
