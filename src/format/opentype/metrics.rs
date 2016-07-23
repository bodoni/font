use std::ops::Deref;
use truetype::{HorizontalHeader, HorizontalMetrics};

use Result;

pub struct Metrics {
    header: HorizontalHeader,
    #[allow(dead_code)]
    metrics: HorizontalMetrics,
}

impl Metrics {
    pub fn new(header: HorizontalHeader, metrics: HorizontalMetrics) -> Result<Self> {
        Ok(Metrics { header: header, metrics: metrics })
    }
}

impl Deref for Metrics {
    type Target = HorizontalHeader;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.header
    }
}
