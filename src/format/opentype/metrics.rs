use std::ops::Deref;
use truetype::{HorizontalHeader, HorizontalMetrics};

use {Number, Result};

pub struct Metrics {
    header: HorizontalHeader,
    metrics: HorizontalMetrics,
}

impl Metrics {
    #[inline]
    pub fn new(header: HorizontalHeader, metrics: HorizontalMetrics) -> Result<Self> {
        Ok(Metrics { header: header, metrics: metrics })
    }

    #[inline]
    pub fn get(&self, index: usize) -> (Number, Number) {
        let metrics = self.metrics.get(index);
        (Number::from(metrics.0), Number::from(metrics.1))
    }
}

impl Deref for Metrics {
    type Target = HorizontalHeader;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.header
    }
}
