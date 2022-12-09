use truetype::{HorizontalHeader, HorizontalMetrics};

use crate::{Number, Result};

pub struct Metrics {
    header: HorizontalHeader,
    metrics: HorizontalMetrics,
}

impl Metrics {
    #[inline]
    pub fn new(header: HorizontalHeader, metrics: HorizontalMetrics) -> Result<Self> {
        Ok(Metrics {
            header: header,
            metrics: metrics,
        })
    }

    #[inline]
    pub fn get(&self, index: usize) -> (Number, Number) {
        let (advance_width, left_side_bearing) = self.metrics.get(index);
        (Number::from(advance_width), Number::from(left_side_bearing))
    }
}

deref! { Metrics::header => HorizontalHeader }
