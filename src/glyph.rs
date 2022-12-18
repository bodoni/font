use crate::Offset;

/// A glyph.
#[derive(Clone, Debug, Default)]
pub struct Glyph {
    /// The advance width.
    pub advance_width: usize,
    /// The left, bottom, right, and top edges.
    pub bounding_box: (isize, isize, isize, isize),
    /// The left and right side bearings.
    pub side_bearings: (isize, isize),
    /// The contours.
    pub contours: Vec<Contour>,
}

/// A contour.
#[derive(Clone, Debug, Default)]
pub struct Contour {
    /// The offset.
    pub offset: Offset,
    /// The position.
    pub position: Offset,
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
    pub fn height(&self) -> usize {
        debug_assert!(self.bounding_box.3 >= self.bounding_box.1);
        (self.bounding_box.3 - self.bounding_box.1) as usize
    }

    /// Return the width.
    #[inline]
    pub fn width(&self) -> usize {
        debug_assert!(self.bounding_box.2 >= self.bounding_box.0);
        (self.bounding_box.2 - self.bounding_box.0) as usize
    }
}

dereference! { Glyph::contours => [Contour] }

dereference! { Contour::segments => [Segment] }
