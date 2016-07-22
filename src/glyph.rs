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
    start: Offset,
    position: Offset,
    contour: Contour,
    contours: Vec<Contour>,
    compensation: Option<Offset>,
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
    fn new<T: Into<Offset>>(offset: T) -> Self {
        Contour { offset: offset.into(), segments: vec![] }
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
        Builder {
            start: Offset::zero(),
            position: Offset::zero(),
            contour: Contour::new(0.0),
            contours: vec![],
            compensation: None,
        }
    }

    pub fn move_by<T: Into<Offset>>(&mut self, a: T) {
        self.flush();
        let a = self.compensate(a);
        self.position += a;
        self.start = self.position;
        self.contour.offset = a;
    }

    pub fn move_to_origin(&mut self) {
        self.flush();
        self.compensation = Some(-self.position);
    }

    pub fn linear_by<T: Into<Offset>>(&mut self, a: T) {
        let a = self.compensate(a);
        self.position += a;
        self.contour.segments.push(Segment::Linear(a));
    }

    pub fn quadratic_by<T: Into<Offset>>(&mut self, a: T, b: T) {
        let (a, b) = (self.compensate(a), b.into());
        self.position += a;
        self.position += b;
        self.contour.segments.push(Segment::Quadratic(a, b));
    }

    pub fn cubic_by<T: Into<Offset>>(&mut self, a: T, b: T, c: T) {
        let (a, b, c) = (self.compensate(a), b.into(), c.into());
        self.position += a;
        self.position += b;
        self.position += c;
        self.contour.segments.push(Segment::Cubic(a, b, c));
    }

    pub fn compensate_by<T: Into<Offset>>(&mut self, a: T) {
        match &mut self.compensation {
            &mut Some(mut compensation) => compensation += a,
            compensation @ &mut None => *compensation = Some(a.into()),
        }
    }

    #[inline]
    pub fn offset(&self) -> Offset {
        self.start - self.position
    }

    #[inline]
    pub fn position(&self) -> Offset {
        self.position
    }

    #[inline]
    fn compensate<T: Into<Offset>>(&mut self, a: T) -> Offset {
        if let Some(compensation) = self.compensation.take() { compensation + a } else { a.into() }
    }

    #[inline]
    fn flush(&mut self) {
        if self.contour.is_empty() {
            return;
        }
        let offset = self.start - self.position;
        if !offset.is_zero() {
            self.linear_by(offset);
        }
        self.contours.push(mem::replace(&mut self.contour, Contour::new(0.0)));
    }
}

impl From<Builder> for Glyph {
    #[inline]
    fn from(mut builder: Builder) -> Glyph {
        builder.flush();
        Glyph { contours: builder.contours }
    }
}
