use std::cell::RefCell;
use std::rc::Rc;

use opentype::truetype::{NamingTable, WindowsMetrics};
use typeface::Tape;

use crate::format::opentype::cache::Cache;
use crate::format::opentype::postscript::PostScript;
use crate::format::opentype::truetype::TrueType;
use crate::{Number, Result};

pub struct Font<T> {
    cache: Rc<RefCell<Cache<T>>>,
    case: Case,
}

enum Case {
    PostScript(PostScript),
    TrueType(TrueType),
}

impl<T: Tape> crate::font::Case for Font<T> {
    #[inline]
    fn draw(&mut self, character: char) -> Result<Option<crate::glyph::Glyph>> {
        match &self.case {
            Case::PostScript(ref case) => case.draw(character),
            Case::TrueType(ref case) => case.draw(character),
        }
    }

    #[inline]
    fn metrics(&mut self) -> Result<crate::metrics::Metrics> {
        read_metrics(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn names(&mut self) -> Result<Rc<NamingTable>> {
        read_names(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn properties(&mut self) -> Result<crate::properties::Properties> {
        read_properties(&mut self.cache.borrow_mut())
    }
}

pub fn read<T: Tape>(tape: Rc<RefCell<T>>, backend: opentype::Font) -> Result<Vec<Font<T>>> {
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

pub fn read_metrics<T: Tape>(cache: &mut Cache<T>) -> Result<crate::metrics::Metrics> {
    println!("Hi");
    let font_header = cache.font_header()?.clone();
    println!("Hi");
    let windows_metrics = cache.windows_metrics()?.clone();
    println!("Hi");
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

pub fn read_names<T: Tape>(cache: &mut Cache<T>) -> Result<Rc<NamingTable>> {
    Ok(cache.naming_table()?.clone())
}

pub fn read_properties<T: Tape>(cache: &mut Cache<T>) -> Result<crate::properties::Properties> {
    let font_header = cache.font_header()?.clone();
    let windows_metrics = cache.windows_metrics()?.clone();
    macro_rules! get(
        ($($version:ident),+) => (
            match &*windows_metrics {
                $(WindowsMetrics::$version(ref metrics) => (
                    (metrics.vendor_id, metrics.selection_flags)
                ),)*
            }
        );
    );
    let machintosh_flags = font_header.macintosh_flags;
    let (vendor_id, windows_flags) =
        get!(Version0, Version1, Version2, Version3, Version4, Version5);
    let (mut cubic, mut variable) = (false, false);
    for record in cache.offset_table.iter() {
        match &record.tag.0 {
            b"CFF " | b"CFF2" => cubic = true,
            b"fvar" => variable = true,
            _ => {}
        }
    }
    Ok(crate::properties::Properties {
        cubic,
        italic: machintosh_flags.is_italic() || windows_flags.is_italic(),
        variable,
        vendor_id: match String::from_utf8(vendor_id.to_vec()) {
            Ok(value) => value,
            _ => raise!("found a malformed vendor identifier"),
        },
    })
}
