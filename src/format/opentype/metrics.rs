use truetype::{GlyphID, HorizontalHeader, HorizontalMetrics, WindowsMetrics};

use crate::Result;

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

    pub fn describe(&self) -> (isize, isize, isize) {
        macro_rules! get(
            ($($version:ident),+) => (
                match self.windows_metrics {
                    $(
                        WindowsMetrics::$version(ref metrics) => (
                            metrics.typographic_ascender,
                            metrics.typographic_descender,
                            metrics.typographic_line_gap,
                        ),
                    )*
                }
            )
        );
        let (ascender, descender, line_gap) =
            get!(Version0, Version1, Version2, Version3, Version4, Version5);
        (ascender as isize, descender as isize, line_gap as isize)
    }

    #[inline]
    pub fn get(&self, glyph_index: GlyphID) -> (usize, isize) {
        let (advance_width, left_side_bearing) = self.horizontal_metrics.get(glyph_index);
        (advance_width as usize, left_side_bearing as isize)
    }
}

dereference! { Metrics::horizontal_header => HorizontalHeader }
