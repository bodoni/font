use crate::{Number, Offset};

/// A glyph.
#[derive(Clone, Debug)]
pub struct Glyph {
    /// The advance width.
    pub advance_width: Number,
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

impl Glyph {
    /// Return the height.
    #[inline]
    pub fn height(&self) -> Number {
        self.bounding_box.3 - self.bounding_box.1
    }

    /// Return the width.
    #[inline]
    pub fn width(&self) -> Number {
        self.bounding_box.2 - self.bounding_box.0
    }
}

impl Default for Glyph {
    #[inline]
    fn default() -> Self {
        use std::f32::NAN;
        Glyph {
            advance_width: NAN,
            bounding_box: (NAN, NAN, NAN, NAN),
            side_bearings: (NAN, NAN),
            contours: vec![],
        }
    }
}

deref! { Glyph::contours => [Contour] }

deref! { Contour::segments => [Segment] }
