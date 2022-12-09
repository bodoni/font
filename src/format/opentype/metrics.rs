use truetype::{GlyphID, HorizontalHeader, HorizontalMetrics, WindowsMetrics};

use crate::{Number, Result};

pub struct Metrics {
    horizontal_header: HorizontalHeader,
    horizontal_metrics: HorizontalMetrics,
    windows_metrics: WindowsMetrics,
}

impl Metrics {
    #[inline]
    pub fn new(
        horizontal_header: HorizontalHeader,
        horizontal_metrics: HorizontalMetrics,
        windows_metrics: WindowsMetrics,
    ) -> Result<Self> {
        Ok(Metrics {
            horizontal_header,
            horizontal_metrics,
            windows_metrics,
        })
    }

    pub fn describe(&self) -> (isize, isize) {
        macro_rules! get(
            ($($version:ident),+) => (
                match self.windows_metrics {
                    $(WindowsMetrics::$version(ref metrics) => (metrics.typographic_ascender, metrics.typographic_descender),)*
                }
            )
        );
        let (ascender, descender) =
            get!(Version0, Version1, Version2, Version3, Version4, Version5);
        (ascender as isize, descender as isize)
    }

    #[inline]
    pub fn get(&self, glyph_index: GlyphID) -> (Number, Number) {
        let (advance_width, left_side_bearing) = self.horizontal_metrics.get(glyph_index);
        (Number::from(advance_width), Number::from(left_side_bearing))
    }
}

deref! { Metrics::horizontal_header => HorizontalHeader }
