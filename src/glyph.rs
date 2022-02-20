use crate::{Number, Offset};

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
    /// The offset of the previous point of the Glyph
    pub offset: Offset,
    /// The segments.
    pub segments: Vec<Segment>,
}

/// A segment.
/// Each Offset is an Offset from the previous field.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Segment {
    /// A line.
    Linear(Offset),
    /// A quadratic Bézier curve.
    Quadratic(Offset, Offset),
    /// A cubic Bézier curve.
    Cubic(Offset, Offset, Offset),
}

impl Segment {
    /// Helper function to interpolate the current Segment
    pub fn interpolate(&self, t: f32) -> Offset {
        assert!(t >= 0.0 && t <= 1.0);

        let ts = t * t; // t ** 2
        let tss = ts * t; // t ** 3
        let it = 1.0 - t; // 1.0 - t
        let its = it * it; // (1.0 - t) ** 2
        let itss = its * it; // (1.0 - t) ** 3

        match self {
            Segment::Linear(p1) => *p1 * t,
            Segment::Quadratic(x, y) => {
                let p1 = *x;
                let p2 = p1 + *y;
                p1 * t * it * 2 + p2 * ts
            }
            Segment::Cubic(x, y, z) => {
                let p1 = *x;
                let p2 = p1 + *y;
                let p3 = p2 + *z;
                p1 * t * itss * 3.0 + p2 * ts * it * 3 + p3 * tss
            }
        }
    }
}

impl Glyph {
    /// Return the width including the side bearings.
    #[inline]
    pub fn advance_width(&self) -> Number {
        self.side_bearings.0 + self.width() + self.side_bearings.1
    }

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
            bounding_box: (NAN, NAN, NAN, NAN),
            side_bearings: (NAN, NAN),
            contours: vec![],
        }
    }
}

deref! { Glyph::contours => [Contour] }

deref! { Contour::segments => [Segment] }
