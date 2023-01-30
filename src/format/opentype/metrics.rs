use std::rc::Rc;

use truetype::{GlyphID, HorizontalMetrics};

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
