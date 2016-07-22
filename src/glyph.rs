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
    position: Offset,
    offset: Option<Offset>,
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
            position: Offset::from(0.0),
            offset: None,
            contour: Contour::new(0.0),
            contours: vec![],
        }
    }

    pub fn move_to<T: Into<Offset>>(&mut self, a: T) {
        self.flush();
        let a = self.offset(a);
        self.position += a;
        self.contour.offset = a;
    }

    pub fn move_to_origin(&mut self) {
        self.flush();
        self.offset = Some(-self.position);
    }

    pub fn linear_to<T: Into<Offset>>(&mut self, a: T) {
        let a = self.offset(a);
        self.position += a;
        self.contour.segments.push(Segment::Linear(a));
    }

    pub fn quadratic_to<T: Into<Offset>>(&mut self, a: T, b: T) {
        let (a, b) = (self.offset(a), b.into());
        self.position += a;
        self.position += b;
        self.contour.segments.push(Segment::Quadratic(a, b));
    }

    pub fn cubic_to<T: Into<Offset>>(&mut self, a: T, b: T, c: T) {
        let (a, b, c) = (self.offset(a), b.into(), c.into());
        self.position += a;
        self.position += b;
        self.position += c;
        self.contour.segments.push(Segment::Cubic(a, b, c));
    }

    pub fn offset_by<T: Into<Offset>>(&mut self, a: T) {
        match &mut self.offset {
            &mut Some(mut offset) => offset += a,
            offset @ &mut None => *offset = Some(a.into()),
        }
    }

    #[inline]
    pub fn position(&self) -> Offset {
        self.position
    }

    #[inline]
    fn flush(&mut self) {
        let contour = mem::replace(&mut self.contour, Contour::new(0.0));
        if !contour.is_empty() {
            self.contours.push(contour);
        }
    }

    #[inline]
    fn offset<T: Into<Offset>>(&mut self, a: T) -> Offset {
        if let Some(offset) = self.offset.take() { offset + a } else { a.into() }
    }
}

impl From<Builder> for Glyph {
    #[inline]
    fn from(mut builder: Builder) -> Glyph {
        builder.flush();
        Glyph { contours: builder.contours }
    }
}
