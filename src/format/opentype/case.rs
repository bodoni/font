extern crate postscript;

use self::postscript::compact::FontSet;

pub struct PostScript {
    #[allow(dead_code)]
    fontset: FontSet,
}

impl PostScript {
    #[inline]
    pub fn new(fontset: FontSet) -> PostScript {
        PostScript { fontset: fontset }
    }
}

impl ::case::Case for PostScript {
}
