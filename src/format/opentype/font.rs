use std::cell::RefCell;
use std::rc::Rc;

use opentype::truetype::{NamingTable, WindowsMetrics};
use typeface::Tape;

use super::cache::Cache;
use super::postscript::PostScript;
use super::truetype::TrueType;
use crate::{Number, Result};

pub struct Font<T> {
    cache: Rc<RefCell<Cache<T>>>,
    case: Case,
}

enum Case {
    PostScript(PostScript),
    TrueType(TrueType),
}

impl<T: Tape> Font<T> {
    #[inline]
    pub fn draw(&mut self, character: char) -> Result<Option<crate::glyph::Glyph>> {
        match &self.case {
            Case::PostScript(ref case) => case.draw(character),
            Case::TrueType(ref case) => case.draw(character),
        }
    }

    pub fn flags(&mut self) -> Result<crate::flags::Flags> {
        let mut cache_borrowed = self.cache.borrow_mut();
        let font_header = cache_borrowed.font_header()?.clone();
        let windows_metrics = cache_borrowed.windows_metrics()?.clone();
        macro_rules! get(
            ($($version:ident),+) => (
                match &*windows_metrics {
                    $(WindowsMetrics::$version(ref metrics) => (
                        metrics.selection_flags
                    ),)*
                }
            );
        );
        let machintosh_flags = font_header.macintosh_flags;
        let windows_flags = get!(Version0, Version1, Version2, Version3, Version4, Version5);
        Ok(crate::flags::Flags {
            italic: machintosh_flags.is_italic() || windows_flags.is_italic(),
        })
    }

    pub fn metrics(&mut self) -> Result<crate::metrics::Metrics> {
        let mut cache_borrowed = self.cache.borrow_mut();
        let font_header = cache_borrowed.font_header()?.clone();
        let windows_metrics = cache_borrowed.windows_metrics()?.clone();
        macro_rules! get(
            (@version0 $($version:ident),+) => (
                match &*windows_metrics {
                    $(WindowsMetrics::$version(ref metrics) => (
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
                    $(WindowsMetrics::$version(ref metrics) => (
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
            clipping_ascender,
            ascender,
            cap_height,
            x_height,
            baseline: if font_header.flags.is_baseline_at_0() {
                0.0
            } else {
                Number::NAN
            },
            descender,
            clipping_descender,
            line_gap,
        })
    }

    pub fn names(&mut self) -> Result<Rc<NamingTable>> {
        let mut cache_borrowed = self.cache.borrow_mut();
        Ok(cache_borrowed.naming_table()?.clone())
    }
}

pub fn read<T>(tape: Rc<RefCell<T>>, backend: opentype::Font) -> Result<Vec<Font<T>>>
where
    T: Tape,
{
    let mut fonts = vec![];
    let cache = Rc::new(RefCell::new(Cache::new(tape, backend)));
    let mut cache_borrowed = cache.borrow_mut();
    let metrics = cache_borrowed.metrics()?.clone();
    let mapping = cache_borrowed.mapping()?.clone();
    if let Some(font_set) = cache_borrowed.try_font_set()? {
        for id in 0..font_set.character_strings.len() {
            let case = PostScript::new(id, font_set.clone(), metrics.clone(), mapping.clone());
            fonts.push(Font {
                cache: cache.clone(),
                case: Case::PostScript(case),
            });
        }
    }
    if let Some(glyph_data) = cache_borrowed.try_glyph_data()? {
        let case = TrueType::new(glyph_data.clone(), metrics, mapping);
        fonts.push(Font {
            cache: cache.clone(),
            case: Case::TrueType(case),
        });
    }
    Ok(fonts)
}
