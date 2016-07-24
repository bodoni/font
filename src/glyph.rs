use std::ops::Deref;

use {Number, Offset};

/// A glyph.
#[derive(Clone, Debug)]
pub struct Glyph {
    /// The left, bottom, right, and top edges.
    pub bounding_box: (Number, Number, Number, Number),
    /// The left and right side bearings.
    pub side_bearings: (Number, Number),
    /// The contours.
    pub contours: Vec<Contour>,
}

/// A contour.
#[derive(Clone, Debug, Default)]
pub struct Contour {
    /// The offset.
    pub offset: Offset,
    /// The segments.
    pub segments: Vec<Segment>,
}

/// A segment.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Segment {
    /// A line.
    Linear(Offset),
    /// A quadratic Bézier curve.
    Quadratic(Offset, Offset),
    /// A cubic Bézier curve.
    Cubic(Offset, Offset, Offset),
}

impl Default for Glyph {
    #[inline]
    fn default() -> Self {
        use std::f32::NAN;
        Glyph { bounding_box: (NAN, NAN, NAN, NAN), side_bearings: (NAN, NAN), contours: vec![] }
    }
}

impl Deref for Glyph {
    type Target = [Contour];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.contours
    }
}

impl Deref for Contour {
    type Target = [Segment];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.segments
    }
}
