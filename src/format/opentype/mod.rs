extern crate opentype;

use std::path::Path;

use {Font, Result};

mod case;

pub fn open<T: AsRef<Path>>(path: T) -> Result<Vec<Font>> {
    let file = try!(opentype::File::open(path));
    let mut fonts = vec![];
    for mut font in file.fonts {
        let header = some!(font.font_header.as_ref(),
                           "cannot find the font header");
        let horizontal_header = some!(font.horizontal_header.as_ref(),
                                      "cannot find the horizontal header");
        match font.postscript_fontset.take() {
            Some(fontset) => fonts.push(Font {
                units_per_em: header.units_per_em as usize,
                ascender: horizontal_header.ascender as isize,
                descender: horizontal_header.descender as isize,
                glyphs: Box::new(case::PostScript::new(fontset)),
            }),
            _ => raise!("only PostScript outlines are currently supported"),
        }
    }
    Ok(fonts)
}
