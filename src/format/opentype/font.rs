use std::io::{Read, Seek};
use std::{cell::RefCell, rc::Rc};

use opentype;

use super::cache::Cache;
use super::case::Case;
use crate::{Number, Result};

pub struct Font<T: Read + Seek> {
    cache: Rc<RefCell<Cache<T>>>,
    case: Box<dyn Case>,
}

impl<T: Read + Seek> crate::case::Case for Font<T> {
    #[inline]
    fn draw(&mut self, character: char) -> Result<Option<crate::glyph::Glyph>> {
        self.case.draw(character)
    }

    fn metrics(&mut self) -> Result<crate::metrics::Metrics> {
        let mut cache_borrowed = self.cache.borrow_mut();
        let font_header = cache_borrowed.font_header()?.clone();
        let windows_metrics = cache_borrowed.windows_metrics()?.clone();
        macro_rules! get(
            (@version0 $($version:ident),+) => (
                match &*windows_metrics {
                    $(truetype::WindowsMetrics::$version(ref metrics) => (
                        metrics.windows_ascender.into(),
                        metrics.typographic_ascender.into(),
                        metrics.typographic_descender.into(),
                        -Number::from(metrics.windows_descender),
                        metrics.typographic_line_gap.into(),
                    ),)*
                }
            );
            (@version2 $($version:ident),+) => (
                match &*windows_metrics {
                    $(truetype::WindowsMetrics::$version(ref metrics) => (
                        metrics.cap_height.into(),
                        metrics.x_height.into(),
                    ),)*
                    _ => (
                        Number::NAN,
                        Number::NAN,
                    ),
                }
            );
        );
        let (clipping_ascender, ascender, descender, clipping_descender, line_gap) =
            get!(@version0 Version0, Version1, Version2, Version3, Version4, Version5);
        let (cap_height, x_height) = get!(@version2 Version2, Version3, Version4, Version5);
        Ok(crate::metrics::Metrics {
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
        })
    }
}

pub fn read<T>(tape: Rc<RefCell<T>>, backend: opentype::Font) -> Result<Vec<Font<T>>>
where
    T: Read + Seek,
{
    use super::postscript::PostScript;
    use super::truetype::TrueType;

    let mut fonts = vec![];
    let cache = Rc::new(RefCell::new(Cache::new(tape.clone(), backend)));
    let mut cache_borrowed = cache.borrow_mut();
    let metrics = cache_borrowed.metrics()?.clone();
    let mapping = cache_borrowed.mapping()?.clone();
    if let Some(font_set) = cache_borrowed.try_font_set()? {
        for id in 0..font_set.char_strings.len() {
            let case = PostScript::new(id, font_set.clone(), metrics.clone(), mapping.clone());
            fonts.push(Font {
                cache: cache.clone(),
                case: Box::new(case),
            });
        }
    }
    if let Some(_) = cache_borrowed.try_glyph_mapping()? {
        if let Some(glyph_data) = cache_borrowed.try_glyph_data()? {
            let case = TrueType::new(glyph_data.clone(), metrics.clone(), mapping.clone());
            fonts.push(Font {
                cache: cache.clone(),
                case: Box::new(case),
            });
        }
    }
    Ok(fonts)
}
