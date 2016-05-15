extern crate opentype;

use std::path::Path;
use std::rc::Rc;

use {Font, Result};

mod case;

pub fn open<T: AsRef<Path>>(path: T) -> Result<Vec<Font>> {
    let file = try!(opentype::File::open(path));
    let mut fonts = vec![];
    for mut font in file.fonts {
        let font_header = some!(font.font_header.as_ref(),
                           "cannot find the font header");
        let char_mapping = Rc::new(some!(font.char_mapping.take(),
                                         "cannot find the char-to-glyph mapping"));
        let horizontal_header = some!(font.horizontal_header.as_ref(),
                                      "cannot find the horizontal header");
        match font.postscript_fontset.take() {
            Some(fontset) => {
                let fontset = Rc::new(fontset);
                for id in 0..fontset.char_strings.len() {
                    fonts.push(Font {
                        units_per_em: font_header.units_per_em as usize,
                        ascender: horizontal_header.ascender as isize,
                        descender: horizontal_header.descender as isize,
                        case: Box::new(case::PostScript::new(id, fontset.clone(),
                                                             char_mapping.clone())),
                    });
                }
            },
            _ => raise!("only PostScript outlines are currently supported"),
        }
    }
    Ok(fonts)
}
