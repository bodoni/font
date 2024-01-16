use std::io::Result;
use std::rc::Rc;

use opentype::truetype::tables::HorizontalMetrics;
use opentype::truetype::GlyphID;
use typeface::Tape;

use crate::formats::opentype::cache::Cache;
use crate::Number;

pub struct Metrics {
    horizontal_metrics: Rc<HorizontalMetrics>,
}

impl Metrics {
    #[inline]
    pub fn new(horizontal_metrics: Rc<HorizontalMetrics>) -> Self {
        Metrics { horizontal_metrics }
    }

    #[inline]
    pub fn get(&self, glyph_id: GlyphID) -> (Number, Number) {
        let (advance_width, left_side_bearing) = self.horizontal_metrics.get(glyph_id);
        (advance_width.into(), left_side_bearing.into())
    }
}

pub(crate) fn read<T: Tape>(cache: &mut Cache<T>) -> Result<crate::Metrics> {
    use opentype::truetype::tables::WindowsMetrics;

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
    Ok(crate::Metrics {
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
