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
    fn axes(&mut self) -> Result<crate::Axes> {
        read_axes(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn characters(&mut self) -> Result<crate::Characters> {
        read_characters(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn features(&mut self) -> Result<crate::Features> {
        read_features(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn metrics(&mut self) -> Result<crate::Metrics> {
        read_metrics(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn names(&mut self) -> Result<crate::Names> {
        read_names(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn draw(&mut self, character: char) -> Result<Option<crate::Glyph>> {
        match &self.case {
            Case::PostScript(ref case) => case.draw(character),
            Case::TrueType(ref case) => case.draw(character),
        }
    }
}

pub fn read<T: Tape>(tape: Rc<RefCell<T>>, backend: opentype::Font) -> Result<Vec<Font<T>>> {
    let mut fonts = vec![];
    let cache = Rc::new(RefCell::new(Cache::new(tape, backend)));
    let mut cache_borrowed = cache.borrow_mut();
    let mapping = cache_borrowed.mapping()?.clone();
    let metrics = cache_borrowed.metrics()?.clone();
    if let Some(table) = cache_borrowed.try_font_set()? {
        for id in 0..table.character_strings.len() {
            let case = PostScript::new(id, table.clone(), mapping.clone(), metrics.clone());
            fonts.push(Font {
                cache: cache.clone(),
                case: Case::PostScript(case),
            });
        }
    }
    if let Some(table) = cache_borrowed.try_glyph_data()? {
        let case = TrueType::new(table.clone(), mapping, metrics);
        fonts.push(Font {
            cache: cache.clone(),
            case: Case::TrueType(case),
        });
    }
    Ok(fonts)
}

pub fn read_axes<T: Tape>(cache: &mut Cache<T>) -> Result<crate::Axes> {
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
            if let Some(r#type) = crate::axes::Type::from_tag(&record.tag) {
                axes.insert(
                    r#type,
                    Value {
                        default: record.default_value.into(),
                        range: Some((record.min_value.into(), record.max_value.into())),
                    },
                );
            }
        }
    }
    Ok(axes)
}

pub fn read_characters<T: Tape>(cache: &mut Cache<T>) -> Result<crate::Characters> {
    use crate::formats::opentype::characters::Characters;

    Ok(Characters::new(cache.character_mapping()?)?
        .0
        .into_iter()
        .map(|(lower, upper)| lower..=upper)
        .collect())
}

pub fn read_features<T: Tape>(_: &mut Cache<T>) -> Result<crate::Features> {
    unimplemented!()
}

pub fn read_metrics<T: Tape>(cache: &mut Cache<T>) -> Result<crate::Metrics> {
    use opentype::truetype::WindowsMetrics;

    let font_header = cache.font_header()?.clone();
    let windows_metrics = cache.windows_metrics()?.clone();
    macro_rules! get(
        (@version0 $($version:ident),+) => (
            match &*windows_metrics {
                $(WindowsMetrics::$version(ref table) => (
                    table.windows_ascender.into(),
                    table.typographic_ascender.into(),
                    table.typographic_descender.into(),
                    -Number::from(table.windows_descender),
                    table.typographic_line_gap.into(),
                ),)*
            }
        );
        (@version2 $($version:ident),+) => (
            match &*windows_metrics {
                $(WindowsMetrics::$version(ref table) => (
                    table.cap_height.into(),
                    table.x_height.into(),
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

pub fn read_names<T: Tape>(cache: &mut Cache<T>) -> Result<crate::Names> {
    Ok(cache.naming_table()?.clone())
}
