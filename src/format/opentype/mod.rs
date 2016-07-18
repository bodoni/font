extern crate opentype;
extern crate postscript;
extern crate truetype;

use std::path::Path;
use std::rc::Rc;

use {Font, Result};

mod case;
mod mapping;

pub fn open<T: AsRef<Path>>(path: T) -> Result<Vec<Font>> {
    let file = try!(opentype::File::open(path));
    let mut fonts = vec![];
    for mut font in file.fonts {
        let char_mapping = some!(font.char_mapping.take(),
                                 "cannot find the char-to-glyph mapping");
        let font_header = some!(font.font_header.as_ref(),
                                "cannot find the font header");
        let horizontal_header = some!(font.horizontal_header.as_ref(),
                                      "cannot find the horizontal header");
        let mapping = Rc::new(try!(mapping::Mapping::new(char_mapping)));
        match font.compact_font_set.take() {
            Some(font_set) => {
                let font_set = Rc::new(font_set);
                for id in 0..font_set.char_strings.len() {
                    fonts.push(Font {
                        units_per_em: font_header.units_per_em as usize,
                        ascender: horizontal_header.ascender as isize,
                        descender: horizontal_header.descender as isize,
                        case: Box::new(case::PostScript::new(id, font_set.clone(),
                                                             mapping.clone())),
                    });
                }
            },
            _ => raise!("only PostScript outlines are currently supported"),
        }
    }
    Ok(fonts)
}
