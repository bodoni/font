use std::mem;
use std::ops::Deref;

use Offset;

/// A glyph.
#[derive(Clone, Debug, Default)]
pub struct Glyph {
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
    Line(Offset),
    /// A Bézier curve.
    Curve(Curve),
}

/// A curve.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Curve {
    /// A quadratic Bézier curve.
    Quadratic(Offset, Offset),
    /// A cubic Bézier curve.
    Cubic(Offset, Offset, Offset),
}

#[derive(Default)]
pub struct Builder {
    offset: Offset,
    contour: Contour,
    contours: Vec<Contour>,
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

impl Builder {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    pub fn move_to(&mut self, a: Offset) {
        self.terminate();
        self.offset(a);
        self.contour.offset = self.offset;
    }

    pub fn line_to(&mut self, a: Offset) {
        self.offset(a);
        self.contour.segments.push(Segment::Line(a));
    }

    pub fn quadratic_to(&mut self, a: Offset, b: Option<Offset>) {
        self.offset(a);
        let b = match b {
            Some(b) => b,
            _ => (self.contour.offset.0 - self.offset.0,
                  self.contour.offset.1 - self.offset.1),
        };
        self.offset(b);
        self.contour.segments.push(Segment::Curve(Curve::Quadratic(a, b)));
    }

    pub fn cubic_to(&mut self, a: Offset, b: Offset, c: Offset) {
        self.offset(a);
        self.offset(b);
        self.offset(c);
        self.contour.segments.push(Segment::Curve(Curve::Cubic(a, b, c)));
    }

    #[inline]
    fn offset(&mut self, (x, y): Offset) {
        self.offset.0 += x;
        self.offset.1 += y;
    }

    #[inline]
    fn terminate(&mut self) {
        if !self.contour.is_empty() {
            self.contours.push(mem::replace(&mut self.contour, Default::default()));
        }
    }
}

impl From<Builder> for Glyph {
    #[inline]
    fn from(mut builder: Builder) -> Glyph {
        builder.terminate();
        Glyph { contours: builder.contours }
    }
}
