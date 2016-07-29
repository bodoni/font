use opentype;
use std::io::{Read, Seek};
use std::rc::Rc;

use {Font, Number, Result};

mod mapping;
mod metrics;
mod postscript;
mod truetype;

use self::mapping::Mapping;
use self::metrics::Metrics;
use self::postscript::PostScript;
use self::truetype::TrueType;

pub fn read<T: Read + Seek>(tape: &mut T) -> Result<Vec<Font>> {
    let file = try!(opentype::File::read(tape));
    let mut fonts = vec![];
    for mut font in file.fonts {
        let font_header = some!(font.font_header.as_ref(),
                                "cannot find the font header");
        let horizontal_header = some!(font.horizontal_header.take(),
                                      "cannot find the horizontal header");
        let horizontal_metrics = some!(font.horizontal_metrics.take(),
                                       "cannot find the horizontal metrics");
        let char_mapping = some!(font.char_mapping.take(),
                                 "cannot find the char-to-glyph mapping");
        let metrics = Rc::new(try!(Metrics::new(horizontal_header, horizontal_metrics)));
        let mapping = Rc::new(try!(Mapping::new(char_mapping)));
        if let Some(font_set) = font.compact_font_set.take() {
            let font_set = Rc::new(font_set);
            for id in 0..font_set.char_strings.len() {
                fonts.push(Font {
                    units_per_em: font_header.units_per_em as usize,
                    ascender: Number::from(metrics.ascender),
                    descender: Number::from(metrics.descender),
                    case: Box::new(PostScript::new(id, font_set.clone(), metrics.clone(),
                                                   mapping.clone())),
                });
            }
            continue;
        }
        if let Some(glyph_data) = font.glyph_data.take() {
            let glyph_data = Rc::new(glyph_data);
            fonts.push(Font {
                units_per_em: font_header.units_per_em as usize,
                ascender: Number::from(metrics.ascender),
                descender: Number::from(metrics.descender),
                case: Box::new(TrueType::new(glyph_data.clone(), metrics.clone(),
                                             mapping.clone())),
            });
            continue;
        }
        raise!("failed to find glyph outlines");
    }
    Ok(fonts)
}
