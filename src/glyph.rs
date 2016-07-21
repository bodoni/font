use std::mem;
use std::ops::Deref;

use Offset;

/// A glyph.
#[derive(Clone, Debug)]
pub struct Glyph {
    /// The contours.
    pub contours: Vec<Contour>,
}

/// A contour.
#[derive(Clone, Debug)]
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

impl Contour {
    #[inline]
    fn new(offset: Offset) -> Self {
        Contour { offset: offset, segments: vec![] }
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
        Builder { offset: Offset::zero(), contour: Contour::new(Offset::zero()), contours: vec![] }
    }

    pub fn move_to(&mut self, a: Offset) {
        self.offset += a;
        let contour = mem::replace(&mut self.contour, Contour::new(a));
        if !contour.is_empty() {
            self.contours.push(contour);
        }
    }

    pub fn line_to(&mut self, a: Offset) {
        self.offset += a;
        self.contour.segments.push(Segment::Linear(a));
    }

    pub fn quadratic_curve_to(&mut self, a: Offset, b: Offset) {
        self.offset += a;
        self.offset += b;
        self.contour.segments.push(Segment::Quadratic(a, b));
    }

    pub fn cubic_curve_to(&mut self, a: Offset, b: Offset, c: Offset) {
        self.offset += a;
        self.offset += b;
        self.offset += c;
        self.contour.segments.push(Segment::Cubic(a, b, c));
    }

    #[inline]
    pub fn offset(&self) -> Offset {
        self.offset
    }
}

impl From<Builder> for Glyph {
    #[inline]
    fn from(Builder { contour, mut contours, .. }: Builder) -> Glyph {
        if !contour.is_empty() {
            contours.push(contour);
        }
        Glyph { contours: contours }
    }
}
