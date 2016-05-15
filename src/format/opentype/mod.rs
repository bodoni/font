extern crate opentype;

use std::path::Path;

use {Font, Result};

pub fn open<T: AsRef<Path>>(path: T) -> Result<Vec<Font>> {
    let file = try!(opentype::File::open(path));
    let mut fonts = vec![];
    for font in file.fonts {
        let header = some!(font.font_header.as_ref(), "the font header is missing");
        let horizontal_header = some!(font.horizontal_header.as_ref(),
                                      "the horizontal header is missing");
        fonts.push(Font {
            units_per_em: header.units_per_em as usize,
            ascender: horizontal_header.ascender as isize,
            descender: horizontal_header.descender as isize,
        });
    }
    Ok(fonts)
}
