use std::cell::RefCell;
use std::rc::Rc;

use typeface::Tape;

use crate::formats::opentype::cache::Cache;
use crate::formats::opentype::postscript::PostScript;
use crate::formats::opentype::truetype::TrueType;
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
    fn axes(&mut self) -> Result<crate::axes::Axes> {
        read_axes(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn characters(&mut self) -> Result<crate::characters::Characters> {
        read_characters(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn metrics(&mut self) -> Result<crate::metrics::Metrics> {
        read_metrics(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn names(&mut self) -> Result<crate::names::Names> {
        read_names(&mut self.cache.borrow_mut())
    }
}

pub fn read<T: Tape>(tape: Rc<RefCell<T>>, backend: opentype::Font) -> Result<Vec<Font<T>>> {
    let mut fonts = vec![];
    let cache = Rc::new(RefCell::new(Cache::new(tape, backend)));
    let mut cache_borrowed = cache.borrow_mut();
    let characters = cache_borrowed.characters()?.clone();
    let metrics = cache_borrowed.metrics()?.clone();
    if let Some(table) = cache_borrowed.try_font_set()? {
        for id in 0..table.character_strings.len() {
            let case = PostScript::new(id, table.clone(), characters.clone(), metrics.clone());
            fonts.push(Font {
                cache: cache.clone(),
                case: Case::PostScript(case),
            });
        }
    }
    if let Some(table) = cache_borrowed.try_glyph_data()? {
        let case = TrueType::new(table.clone(), characters, metrics);
        fonts.push(Font {
            cache: cache.clone(),
            case: Case::TrueType(case),
        });
    }
    Ok(fonts)
}

pub fn read_axes<T: Tape>(cache: &mut Cache<T>) -> Result<crate::axes::Axes> {
    use opentype::truetype::{PostScript, WindowsMetrics};

    use crate::axes::{Type, Value};

    let font_header = cache.font_header()?.clone();
    let machintosh_flags = font_header.macintosh_flags;
    let windows_metrics = cache.windows_metrics()?.clone();
    macro_rules! get(
        ($($version:ident),+) => (
            match &*windows_metrics {
                $(WindowsMetrics::$version(ref table) => (
                    table.weight_class,
                    table.width_class,
                    table.selection_flags,
                ),)*
            }
        );
    );
    let (weight_class, width_class, windows_flags) =
        get!(Version0, Version1, Version2, Version3, Version4, Version5);
    let italic_flag = machintosh_flags.is_italic() || windows_flags.is_italic();

    let postscript = cache.postscript()?.clone();
    macro_rules! get(
        ($($version:ident),+) => (
            match &*postscript {
                $(PostScript::$version(ref table) => table.italic_angle,)*
            }
        );
    );
    let italic_angle = get!(Version1, Version2, Version3);

    let mut axes = crate::axes::Axes::new();
    axes.insert(Type::Italic, Value::from_italic_flag(italic_flag));
    axes.insert(Type::Slant, Value::from_italic_angle(italic_angle));
    axes.insert(Type::Weight, Value::from_weight_class(weight_class));
    axes.insert(Type::Width, Value::from_width_class(width_class));
    if let Some(table) = cache.try_font_variations()? {
        for record in table.axis_records.iter() {
            let r#type = match &*record.tag {
                b"ital" => Type::Italic,
                b"opsz" => Type::OpticalSize,
                b"slnt" => Type::Slant,
                b"wdth" => Type::Width,
                b"wght" => Type::Weight,
                _ => continue,
            };
            axes.insert(
                r#type,
                Value {
                    default: record.default_value.into(),
                    range: Some((record.min_value.into(), record.max_value.into())),
                },
            );
        }
    }
    Ok(axes)
}

pub fn read_characters<T: Tape>(cache: &mut Cache<T>) -> Result<crate::characters::Characters> {
    cache.character_mapping().cloned()
}

pub fn read_metrics<T: Tape>(cache: &mut Cache<T>) -> Result<crate::metrics::Metrics> {
    use opentype::truetype::WindowsMetrics;

    let font_header = cache.font_header()?.clone();
    let windows_metrics = cache.windows_metrics()?.clone();
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
        granularity: font_header.units_per_em.into(),
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

pub fn read_names<T: Tape>(cache: &mut Cache<T>) -> Result<crate::names::Names> {
    Ok(cache.naming_table()?.clone())
}
