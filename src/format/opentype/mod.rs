use opentype;
use postscript::compact::FontSet;
use std::io::{Read, Seek};
use std::rc::Rc;
use truetype::{
    CharMapping,
    FontHeader,
    GlyphData,
    GlyphMapping,
    HorizontalHeader,
    HorizontalMetrics,
    MaximumProfile,
};

use {Case, Font, Result};

mod mapping;
mod metrics;
mod postscript;
mod truetype;

use self::mapping::Mapping;
use self::metrics::Metrics;
use self::postscript::PostScript;
use self::truetype::TrueType;

pub fn read<T>(tape: &mut T) -> Result<Vec<Font>> where T: Read + Seek {
    let mut fonts = vec![];
    for font in &opentype::File::read(tape)?.fonts {
        read_font(tape, &mut fonts, font)?;
    }
    Ok(fonts)
}

fn read_font<T>(tape: &mut T, fonts: &mut Vec<Font>, font: &opentype::Font) -> Result<()>
    where T: Read + Seek
{
    let font_header = some!(font.take::<_, FontHeader>(tape)?, "cannot find the font header");
    let horizontal_header = some!(font.take::<_, HorizontalHeader>(tape)?,
                                  "cannot find the horizontal header");
    let maximum_profile = some!(font.take::<_, MaximumProfile>(tape)?,
                                "cannot find the maximum profile");
    let horizontal_metrics = some!(font.take_given::<_, HorizontalMetrics>(
        tape, (&horizontal_header, &maximum_profile))?, "cannot find the horizontal metrics");
    let char_mapping = some!(font.take::<_, CharMapping>(tape)?,
                             "cannot find the char-to-glyph mapping");
    let metrics = Rc::new(Metrics::new(horizontal_header, horizontal_metrics)?);
    let mapping = Rc::new(Mapping::new(char_mapping)?);
    if let Some(font_set) = font.take::<_, FontSet>(tape)? {
        let font_set = Rc::new(font_set);
        for id in 0..font_set.char_strings.len() {
            let case = PostScript::new(id, font_set.clone(), metrics.clone(), mapping.clone());
            fonts.push(new_font(&font_header, &metrics, Box::new(case)));
        }
        return Ok(());
    }
    if let Some(glyph_mapping) = font.take_given::<_, GlyphMapping>(
        tape, (&font_header, &maximum_profile))? {

        if let Some(glyph_data) = font.take_given::<_, GlyphData>(tape, &glyph_mapping)? {
            let case = TrueType::new(Rc::new(glyph_data), metrics.clone(), mapping.clone());
            fonts.push(new_font(&font_header, &metrics, Box::new(case)));
            return Ok(());
        }
    }
    raise!("failed to find glyph outlines");
}

#[inline]
pub fn new_font(font_header: &FontHeader, metrics: &Metrics, case: Box<Case>) -> Font {
    Font {
        units_per_em: font_header.units_per_em as usize,
        ascender: metrics.ascender as isize,
        descender: metrics.descender as isize,
        case: case,
    }
}
