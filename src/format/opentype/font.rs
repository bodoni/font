use std::io::{Read, Seek};
use std::rc::Rc;

use ::postscript::compact1::FontSet;
use ::truetype::{
    CharacterMapping, FontHeader, GlyphData, GlyphMapping, HorizontalHeader, HorizontalMetrics,
    MaximumProfile, WindowsMetrics,
};
use opentype;

use crate::case::Case;
use crate::font;
use crate::format::opentype::mapping::Mapping;
use crate::format::opentype::metrics::Metrics;
use crate::format::opentype::postscript::PostScript;
use crate::format::opentype::truetype::TrueType;
use crate::{Number, Result};

pub fn read<T>(tape: &mut T, fonts: &mut Vec<font::Font>, font: &opentype::Font) -> Result<()>
where
    T: Read + Seek,
{
    macro_rules! get(
        ($option:expr, $message:expr,) => (
            match $option {
                Some(value) => value,
                _ => raise!($message),
            }
        );
    );
    let font_header = get!(
        font.take::<_, FontHeader>(tape)?,
        "cannot find the font header",
    );
    let horizontal_header = get!(
        font.take::<_, HorizontalHeader>(tape)?,
        "cannot find the horizontal header",
    );
    let maximum_profile = get!(
        font.take::<_, MaximumProfile>(tape)?,
        "cannot find the maximum profile",
    );
    let horizontal_metrics = get!(
        font.take_given::<_, HorizontalMetrics>(tape, (&horizontal_header, &maximum_profile))?,
        "cannot find the horizontal metrics",
    );
    let windows_metrics = get!(
        font.take::<_, WindowsMetrics>(tape)?,
        "cannot find the OS/2 and Windows metrics",
    );
    let character_mapping = get!(
        font.take::<_, CharacterMapping>(tape)?,
        "cannot find the character-to-glyph mapping",
    );
    let metrics = Rc::new(Metrics::new(
        horizontal_header,
        horizontal_metrics,
        windows_metrics,
    )?);
    let mapping = Rc::new(Mapping::new(character_mapping)?);
    if let Some(font_set) = font.take::<_, FontSet>(tape)? {
        let font_set = Rc::new(font_set);
        for id in 0..font_set.char_strings.len() {
            let case = PostScript::new(id, font_set.clone(), metrics.clone(), mapping.clone());
            fonts.push(build(&font_header, &metrics, Box::new(case)));
        }
        return Ok(());
    }
    if let Some(glyph_mapping) =
        font.take_given::<_, GlyphMapping>(tape, (&font_header, &maximum_profile))?
    {
        if let Some(glyph_data) = font.take_given::<_, GlyphData>(tape, &glyph_mapping)? {
            let case = TrueType::new(Rc::new(glyph_data), metrics.clone(), mapping.clone());
            fonts.push(build(&font_header, &metrics, Box::new(case)));
            return Ok(());
        }
    }
    raise!("found no glyph outlines");
}

fn build(font_header: &FontHeader, metrics: &Metrics, case: Box<dyn Case>) -> font::Font {
    let (
        clipping_ascender,
        ascender,
        cap_height,
        x_height,
        descender,
        clipping_descender,
        line_gap,
    ) = metrics.describe();
    font::Font {
        metrics: crate::metrics::Metrics {
            units_per_em: font_header.units_per_em.into(),
            clipping_ascender: clipping_ascender,
            ascender: ascender,
            cap_height: cap_height,
            x_height: x_height,
            baseline: if font_header.flags.is_baseline_at_0() {
                0.0
            } else {
                Number::NAN
            },
            descender: descender,
            clipping_descender: clipping_descender,
            line_gap: line_gap,
        },
        case: case,
    }
}
