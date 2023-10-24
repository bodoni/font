use crate::Number;

/// Metrics.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Metrics {
    /// The granularity of the coordinate grid.
    pub granularity: Number,
    /// The point above which clipping can safely occur.
    pub clipping_ascender: Number,
    /// The typographical ascender relative to the baseline.
    pub ascender: Number,
    /// The cap height relative to the baseline.
    pub cap_height: Number,
    /// The x-height relative to the baseline.
    pub x_height: Number,
    /// The baseline.
    pub baseline: Number,
    /// The typographical descender relative to the baseline.
    pub descender: Number,
    /// The point below which clipping can safely occur.
    pub clipping_descender: Number,
    /// The typographical line gap.
    pub line_gap: Number,
}
