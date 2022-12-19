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

    pub fn describe(&self) -> (Number, Number, Number) {
        macro_rules! get(
            ($($version:ident),+) => (
                match self.windows_metrics {
                    $(
                        WindowsMetrics::$version(ref metrics) => (
                            metrics.typographic_ascender.into(),
                            metrics.typographic_descender.into(),
                            metrics.typographic_line_gap.into(),
                        ),
                    )*
                }
            )
        );
        get!(Version0, Version1, Version2, Version3, Version4, Version5)
    }

    #[inline]
    pub fn get(&self, glyph_index: GlyphID) -> (Number, Number) {
        let (advance_width, left_side_bearing) = self.horizontal_metrics.get(glyph_index);
        (advance_width.into(), left_side_bearing.into())
    }
}

dereference! { Metrics::horizontal_header => HorizontalHeader }
